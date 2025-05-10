use std::env;
use env_logger;

mod utils;
mod model;
mod handler;

use handler::{
    // user::{list_users, create_user, show_user, update_user, delete_user},
    user::{list_users, create_user, show_user, delete_user},
    // cell::{create_cell, show_cell, update_cell, delete_cell},
    cell::{list_cells, create_cell, show_cell, delete_cell},
};

use axum::{
    routing::get,
    Router,
};

use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {

    // Initialize logger
    env_logger::init();

    // Set web URL
    let listen_url = utils::get_url();
    let addr = env::args().nth(1).unwrap_or_else(|| listen_url);

    // Setup PostgreSQL client
    let db_url = utils::get_db_path();
    let pool = PgPoolOptions::new().max_connections(16).connect(&db_url).await.unwrap();
    let _r = init_db(&pool).await.unwrap();

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/users", get(list_users).post(create_user))
        .route("/users/:user_id", get(show_user).delete(delete_user))
        // .route("/users/:user_id", get(show_user).put(update_user).delete(delete_user))
        .route("/cells", get(list_cells).post(create_cell))
        .route("/cells/:cell_id", get(show_cell).delete(delete_cell))
        // .route("/users/:user_id/cells/:cell_id", get(show_cell).put(update_cell).delete(delete_cell))
        .with_state(pool);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

}


async fn init_db(db: &sqlx::pool::Pool<sqlx::Postgres>) -> Result<(), sqlx::Error> {

    let _r = sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            user_id           UUID NOT NULL PRIMARY KEY
            , user_name       TEXT NOT NULL
            , passhash        TEXT NOT NULL
            , created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(db)
    .await?;

    println!("{:?}", _r);

    let _r = sqlx::query(
        "CREATE TABLE IF NOT EXISTS cells (
            cell_id         UUID NOT NULL PRIMARY KEY
            , user_id       UUID NOT NULL REFERENCES users(user_id)
            , device_id     TEXT NOT NULL
            , text          TEXT NOT NULL
            , fileprops     jsonb NOT NULL
            , is_open       BOOLEAN NOT NULL
            , created_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(db)
    .await?;

    println!("{:?}", _r);

    let _r = sqlx::query(
        "CREATE TABLE IF NOT EXISTS family_tree (
            child_id        UUID NOT NULL REFERENCES cells(cell_id)
            , parent_id     UUID NOT NULL REFERENCES cells(cell_id)
        )"
    )
    .execute(db)
    .await?;

    println!("{:?}", _r);

    Ok(())
}



/*

        match cmd {

            4 => {
                let req: FileProp = serde_json::from_slice(body).unwrap();
                let file_props = db.collection("file_props");
                let parent_id_wrap = req.parent_id;
                let file_prop_id = file_props.insert_one(req, None).await.unwrap().inserted_id;
                let file_prop_id = file_prop_id.as_object_id().unwrap();
                println!("FileProp created with ID: {}", &file_prop_id);
                if let Some(parent_id) = parent_id_wrap {
                    file_props.update_one(
                        doc! { "_id": parent_id },
                        doc! { "$push": {"files": file_prop_id} },
                        None,
                    ).await.unwrap();
                }
                response = serde_json::to_string(&CreateFilePropRes{file_prop_id}).unwrap();
            }
            5 => {
                let req: FileBlob = serde_json::from_slice(body).unwrap();
                let bucket = db.gridfs_bucket(None);
                let blob = req.blob;
                let mut upload_stream = bucket.open_upload_stream(req.file_name, None);
                upload_stream.write_all(&blob[..]).await.unwrap();
                let uploaded_id = upload_stream.id();
                let uploaded_id = uploaded_id.as_object_id().unwrap();
                upload_stream.close().await.unwrap();
                println!("File uploaded with ID: {}", &uploaded_id);
                response = serde_json::to_string(&UploadFileRes{uploaded_id}).unwrap();
            }

*/
