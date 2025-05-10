  
  
#[debug_handler]
pub async fn create_fileprops(
    Path(user_id): Path<uuid::Uuid>,
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<CellReq>
) -> Result<(), StatusCode> {
    // Prepare the query
    let fileprops: Vec<uuid::Uuid> = payload.fileprop_ids;
    let mut query = "INSERT INTO fileprops (name, file_url, completed) VALUES ".to_owned();
  
    // Prepare the values placeholder string and parameter values
    let mut params = Vec::new();
    for (i, fileprop) in fileprops.iter().enumerate() {
        if i > 0 {
            query.push_str(", ");
        }
        query.push_str(&format!("(${}, ${}, ${})", i * 3 + 1, i * 3 + 2, i * 3 + 3));
        params.push(fileprop.name.as_str());
        params.push(fileprop.file_url.as_str());
        params.push(if fileprop.completed {"1"} else {"0"});
    }

    // Execute the query with parameters
    if let Err(e) = sqlx::query(&query)
    .bind(params)
    .execute(&pool)
    .await {
        error!("{}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR)
    };

}  
