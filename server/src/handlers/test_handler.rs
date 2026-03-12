use crate::auth::auth_middleware::AuthUser;
use crate::entity::test::{ActiveModel, Entity as Test};
use axum::{Extension, Json, extract::{
    State,
    Path
}};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTest {
    pub name: String,
    pub questions: String,
    pub id: Option<i32>,
}

pub async fn update_and_create_test(
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<AuthUser>,
    Json(create_test): Json<CreateTest>,
) -> Json<serde_json::Value> {
    println!("Request to /test comed");
    let test = if let Some(id) = create_test.id {
        Test::find_by_id(id).one(&db).await.ok().flatten()
    } else {
        None
    };
    println!("Incoming test: {:?}", create_test.name);
    println!("Questions: {:?}", create_test.questions);
    println!("User: {:?}", user.user_id);
    if let Some(gtest) = test {
        let mut test_active: ActiveModel = gtest.into();

        test_active.text = Set(create_test.name);
        test_active.questions = Set(create_test.questions.to_string());

        let _ = test_active.update(&db).await.unwrap();

        Json(serde_json::json!({
            "success": true,
            "action": "updated"
        }))
    } else {
        let ctest = ActiveModel {
            text: Set(create_test.name),
            questions: Set(create_test.questions.to_string()),
            author_id: Set(user.user_id.parse::<i32>().unwrap_or(0)),
            ..Default::default()
        };

        if let Err(e) = ctest.insert(&db).await {
            println!("DB ERROR: {:?}", e);
            Json(serde_json::json!({
                "success": false,

            }))
        } else {
            Json(serde_json::json!({
                "success": true,
                "action": "created"
            }))
        }
    }
}

pub async fn get_test_by_id(Path(test_id): Path<i32>, State(db): State<DatabaseConnection>) -> Json<serde_json::Value>{
    let test = Test::find_by_id(test_id)
    .one(&db)
    .await.ok().flatten();
    
    if let Some(gtest) = test {
        Json(serde_json::json!({
            "name": gtest.text,
            "questions": gtest.questions,
            "id": gtest.id
        }))
    } else {
        Json(serde_json::json!({
            "success": false,
            "message": "Not found test by id"
        }))
    }
}