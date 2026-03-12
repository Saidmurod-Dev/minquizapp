use crate::auth::auth_middleware::AuthUser;
use crate::auth::jwt;
use crate::entity::user;
use crate::hash;
use crate::{
    entity::user::{Column as UserColumn, Entity as User},
    hash::hash_password,
};
use axum::Extension;
use axum::{Json, extract::State, http::StatusCode};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: String,
}
#[derive(Serialize)]
pub struct StatusMsg {
    success: bool,
    message: String,
    data: String,
}

pub async fn register_user(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<CreateUser>,
) -> Json<StatusMsg> {
    let hashed_password = hash_password(&payload.password);

    let user = user::ActiveModel {
        name: Set(payload.name),
        email: Set(payload.email),
        password: Set(hashed_password),
        ..Default::default()
    };

    match user.insert(&db).await {
        Ok(_) => Json(StatusMsg {
            success: true,
            message: "User created".to_string(),
            data: "".to_string(),
        }),
        Err(e) => {
            println!("DB ERROR: {:?}", e);

            Json(StatusMsg {
                success: false,
                message: "Database error".to_string(),
                data: "".to_string(),
            })
        }
    }
}
#[derive(Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}
pub async fn login_user(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<LoginUser>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let user = User::find()
        .filter(UserColumn::Email.eq(payload.email))
        .one(&db)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB error".to_string()))?
        .ok_or((StatusCode::UNAUTHORIZED, "Email xato".to_string()))?;
    println!("Payload password: {:?}", payload.password);
    println!("Stored hash: {:?}", user.password);
    let valid = hash::verify_password(&payload.password, &user.password);
    println!("Password valid? {}", valid);

    if !valid {
        return Err((StatusCode::UNAUTHORIZED, "password xato".to_string()));
    }

    // 3️⃣ JWT yaratish
    let token = jwt::create_jwt(
        user.id.to_string(),
        &env::var("JWT_SECRET").expect("flashinglights"),
    );

    Ok(Json(serde_json::json!({ "token": token })))
}

pub async fn get_user_data(
    Extension(huser): Extension<AuthUser>,
    State(db): State<DatabaseConnection>,
) -> Json<serde_json::Value> {
    let fuser: Option<user::Model> = User::find_by_id(huser.user_id.parse::<i32>().unwrap())
        .one(&db)
        .await
        .unwrap();
    if let Some(guser) = fuser {
        Json(serde_json::json!({
            "name": guser.name,
            "email": guser.email
        }))
    } else {
        Json(serde_json::json!({
            "error": "User not found"
        }))
    }
}
