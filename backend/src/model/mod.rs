use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct IdRes { pub id: uuid::Uuid }

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Users { pub users: Vec<UserRes> }


#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct User {
    pub user_id:        uuid::Uuid,
    pub user_name:      String,
    pub passhash:       String,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct UserRes {
    pub user_id:        uuid::Uuid,
    pub user_name:      String,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Cells {
    pub cells:          Vec<CellExtracted>,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct CellReq {
    pub cell_id:        uuid::Uuid,
    pub user_id:        uuid::Uuid,
    pub device_id:      String,
    pub text:           String,
    pub is_open:        bool,
    pub fileprops:      Vec<FileProp>,
    pub parent_ids:     Vec<uuid::Uuid>,
    pub child_ids:      Vec<uuid::Uuid>,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct CellExtracted {
    pub cell_id:        uuid::Uuid,
    pub user_id:        uuid::Uuid,
    pub device_id:      String,
    pub text:           String,
    pub is_open:        bool,
    pub fileprops:      Vec<FileProp>,
    pub parents:        Vec<CellExtracted>,
    pub children:       Vec<CellExtracted>,
}

impl CellExtracted {
    pub fn from_cell_row(
        cell_row: CellRow,
        parent_cells: Vec<CellExtracted>,
        child_cells: Vec<CellExtracted>,
    ) -> CellExtracted {
        CellExtracted {
            cell_id: cell_row.cell_id,
            user_id: cell_row.user_id,
            device_id: cell_row.device_id,
            text: cell_row.text,
            is_open: cell_row.is_open,
            fileprops: cell_row.fileprops.0,
            parents: parent_cells,
            children: child_cells,
        }
    }
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct CellRow {
    pub cell_id:        uuid::Uuid,
    pub user_id:        uuid::Uuid,
    pub device_id:      String,
    pub text:           String,
    pub is_open:        bool,
    pub fileprops:      sqlx::types::Json<Vec<FileProp>>,
}
#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct CellFilter {
    pub user_id:        uuid::Uuid,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct FileProp {
    pub path:           String,
    pub url:            String,
    pub completed:      bool,
}

/*
#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Parent {
    pub parent_id:    uuid::Uuid,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Child {
    pub child_id:    uuid::Uuid,
}
*/