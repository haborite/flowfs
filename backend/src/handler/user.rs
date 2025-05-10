use log::error;
use crate::model::*;

use axum::debug_handler;
use axum::{
    extract::{Path, State},
    response::Json,
    http::StatusCode,
};

use sqlx::Postgres;
use sqlx::pool::Pool;

#[debug_handler]
pub async fn list_users(
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<Users>, StatusCode> {
    match sqlx::query_as("SELECT user_id, user_name FROM users")
        .fetch_all(&pool)
        .await {
            Ok(users) => Ok(Json(Users{users})),
            Err(e) => {
                error!("{}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
} 

#[debug_handler]
pub async fn create_user(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<User>
) -> Result<Json<IdRes>, StatusCode> {
    match sqlx::query("INSERT INTO users (user_id, user_name, passhash) VALUES ($1, $2, $3)")
      .bind(payload.user_id)
      .bind(payload.user_name)
      .bind(payload.passhash)
      .fetch_all(&pool)
      .await {
        Ok(_) => Ok(Json(IdRes{id: payload.user_id})),
        Err(e) => {
            error!("{}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

#[debug_handler]
pub async fn show_user(
    Path(user_id): Path<uuid::Uuid>,
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<UserRes>, StatusCode> {
    match sqlx::query_as("SELECT user_id, user_name FROM users WHERE user_id=$1")
        .bind(user_id)
        .fetch_one(&pool)
        .await {
            Ok(user) => Ok(Json(user)),
            Err(e) => {
                error!("{}", e);
                Err(StatusCode::NOT_FOUND)
            }
        }
}

#[debug_handler]
pub async fn delete_user(
    Path(user_id): Path<uuid::Uuid>,
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<IdRes>, StatusCode> {
    match sqlx::query("DELETE FROM users WHERE user_id=$1")
        .bind(user_id)
        .execute(&pool)
        .await {
            Err(e) => {
                error!("{}", e);
                Err(StatusCode::NOT_FOUND)
            }
            Ok(_) => Ok(Json(IdRes{id: user_id})),
        }
}

/*
#[debug_handler]
pub async fn update_user(
    Path(id): Path<ObjectId>,
    State(users): State<Collection<User>>,
    Json(payload): Json<UpdateUserReq>, 
) -> Result<Json<User>, StatusCode> {
    let filter = doc! {"_id": id};
    let opts = FindOneAndUpdateOptions::builder()
        .return_document(ReturnDocument::After)
        .build();
    let mut update = doc! {};
    if let Some(name) = payload.name {
        update.insert("$set", doc!{"name": name});
    }
    if let Some(passhash) = payload.passhash {
        update.insert("$set", doc!{"passhash": passhash});
    }
    match users.find_one_and_update(filter, update, opts).await {
        Err(e) => {
            error!("{}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
        Ok(None) => {
            error!("{}", "User not found");
            return Err(StatusCode::NOT_FOUND)
        },
        Ok(Some(user)) => {
            Ok(Json(user))
        }
    }
}
*/
