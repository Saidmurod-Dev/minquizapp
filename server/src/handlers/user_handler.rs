use crate::{entity::user, hash::hash_password};
use axum::{Json, extract::State};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::{Deserialize, Serialize};

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
        }),
        Err(e) => {
            println!("DB ERROR: {:?}", e);

            Json(StatusMsg {
                success: false,
                message: "Database error".to_string(),
            })
        }
    }
}

pub async fn login_user() {

}
