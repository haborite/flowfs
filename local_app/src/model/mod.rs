use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use dioxus::prelude::*;

#[derive(FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct IdRes { pub id: String }

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Users { pub users: Vec<UserRes> }


#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct User {
    pub user_id:        String,
    pub user_name:      String,
    pub passhash:       String,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct UserRes {
    pub user_id:        String,
    pub user_name:      String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Props)]
pub struct Cells {
    pub cells:          Vec<CellExtracted>,
}

impl Cells {
    pub fn new() -> Self {
        Cells {
            cells: Vec::new(),
        }
    }
}

#[derive(FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct CellReq {
    pub cell_id:        String,
    pub user_id:        String,
    pub device_id:      String,
    pub text:           String,
    pub is_open:        bool,
    pub rootdir:        Dir,
    pub parent_ids:     Vec<String>,
    pub child_ids:      Vec<String>,
}

impl CellReq {
    pub fn set(cellreq: &CellReq) -> Self {
        cellreq.clone()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Props)]
pub struct CellExtracted {
    pub cell_id:        String,
    pub user_id:        String,
    pub device_id:      String,
    pub text:           String,
    pub is_open:        bool,
    pub rootdir:        Dir,
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
            rootdir: cell_row.rootdir.0,
            parents: parent_cells,
            children: child_cells,
        }
    }
}

#[derive(PartialEq, Props, Clone, Debug)]
pub struct CellProps {
    pub cell_id:        String,
    pub user_id:        String,
    pub device_id:      String,
    pub text:           String,
    pub is_open:        bool,
    pub rootdir:        Dir,
    pub parents:        Vec<CellProps>,
    pub children:       Vec<CellProps>,
}

impl From<CellExtracted> for CellProps {
    fn from(cell_extracted: CellExtracted) -> Self {
        CellProps {
            cell_id: cell_extracted.cell_id,
            user_id: cell_extracted.user_id,
            device_id: cell_extracted.device_id,
            text: cell_extracted.text,
            is_open: cell_extracted.is_open,
            rootdir: cell_extracted.rootdir,
            parents: cell_extracted.parents.into_iter().map(CellProps::from).collect(),
            children: cell_extracted.children.into_iter().map(CellProps::from).collect(),
        }        
    }
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct CellRow {
    pub cell_id:        String,
    pub user_id:        String,
    pub device_id:      String,
    pub text:           String,
    pub is_open:        bool,
    pub rootdir:        sqlx::types::Json<Dir>,
}
#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct CellFilter {
    pub user_id:        String,
}

/*
#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct FileProp {
    pub path:           Vec<String>,
    pub url:            String,
    pub completed:      bool,
}
*/

#[derive(Props, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Dir {
    pub name:           String,
    pub dirs:           Vec<Dir>,
    pub fileprops:      Vec<FileProp>,
}

impl Dir {
    pub fn new() -> Self {
        Dir {
            name: "/".to_string(),
            dirs: Vec::new(),
            fileprops: Vec::new(),
        }
    }
}

#[derive(Props, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct FileProp {
    pub name:           String,
    pub url:            String,
    pub completed:      bool,
}
