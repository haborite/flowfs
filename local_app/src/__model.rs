use serde::{Deserialize, Serialize};
use dioxus::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct IdRes { pub id: uuid::Uuid }

#[derive(Serialize, Deserialize, Debug)]
pub struct Users { pub users: Vec<UserRes> }


#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub user_id:        uuid::Uuid,
    pub user_name:      String,
    pub passhash:       String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserRes {
    pub user_id:        uuid::Uuid,
    pub user_name:      String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cells {
    pub cells:          Vec<CellExtracted>,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct CellExtracted {
    pub cell_id:        uuid::Uuid,
    pub user_id:        uuid::Uuid,
    pub device_id:      String,
    pub text:           String,
    pub is_open:        bool,
    // pub fileprops:      Vec<FileProp>,
    pub rootdir:        Dir,
    pub parents:        Vec<CellExtracted>,
    pub children:       Vec<CellExtracted>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CellFilter {
    pub user_id:        uuid::Uuid,
}

#[derive(Props, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Dir {
    pub name:           String,
    pub dirs:           Vec<Dir>,
    pub fileprops:      Vec<FileProp>,
}

#[derive(Props, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct FileProp {
    pub name:           String,
    pub url:            String,
    pub completed:      bool,
}
