use log::error;
use crate::model::*;

/*
use axum::debug_handler;
use axum::{
    extract::{Path, State},
    response::Json,
    http::StatusCode,
};
*/

use sqlx::Sqlite;
use sqlx::pool::Pool;

pub async fn list_users(
    pool: Pool<Sqlite>,
) -> Result<Users, sqlx::Error> {
    match sqlx::query_as("SELECT user_id, user_name FROM users")
        .fetch_all(&pool)
        .await {
            Ok(users) => Ok(Users{users}),
            Err(e) => {
                error!("{}", e);
                Err(e)
            }
        }
} 

pub async fn create_user(
    pool: Pool<Sqlite>,
    payload: User
) -> Result<IdRes, sqlx::Error> {
    match sqlx::query("INSERT INTO users (user_id, user_name, passhash) VALUES (?, ?, ?)")
      .bind(&payload.user_id)
      .bind(&payload.user_name)
      .bind(&payload.passhash)
      .fetch_all(&pool)
      .await {
        Ok(_) => Ok(IdRes{id: payload.user_id}),
        Err(e) => {
            error!("{}", e);
            Err(e)
        }
    }
}

pub async fn show_user(
    user_id: &str,
    pool: Pool<Sqlite>,
) -> Result<UserRes, sqlx::Error> {
    match sqlx::query_as("SELECT user_id, user_name FROM users WHERE user_id=?")
        .bind(user_id)
        .fetch_one(&pool)
        .await {
            Ok(user) => Ok(user),
            Err(e) => {
                error!("{}", e);
                Err(e)
            }
        }
}

pub async fn delete_user(
    user_id: &str,
    pool: Pool<Sqlite>,
) -> Result<IdRes, sqlx::Error> {
    match sqlx::query("DELETE FROM users WHERE user_id=?")
        .bind(&user_id)
        .execute(&pool)
        .await {
            Err(e) => {
                error!("{}", e);
                Err(e)
            }
            Ok(_) => Ok(IdRes{id: user_id.to_string()}),
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
