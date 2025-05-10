#![allow(non_snake_case)]

// use core::ffi;

use dioxus::prelude::*;
use model::{CellExtracted, CellFilter, CellReq, Cells, FileProp};
use tracing::Level;

// use futures::future::join_all;
use sqlx::sqlite::SqlitePoolOptions;

// pub static BASE_API_URL: &str = "127.0.0.1:8080";
const MY_UUID: uuid::Uuid = uuid::Uuid::nil();

const L_SIDEBAR_W: u32 = 48;
const R_SIDEBAR_W: u32 = 48;
const NAVBAR_H: u32 = 16;

mod handler;
mod model;
mod svg_icon;
mod tree;
mod utils;

#[derive(PartialEq, Clone, Props)]
pub struct ButtonWrapProps {
    on_update: EventHandler<MouseEvent>,
}

pub async fn get_cells(cell_filter: CellFilter) -> Result<Cells, sqlx::Error> {
    let db_url = utils::get_db_path();
    let pool = SqlitePoolOptions::new()
        .max_connections(16)
        .connect(&db_url)
        .await
        .unwrap();
    let _r = init_db(&pool).await.unwrap();
    handler::cell::list_cells(pool, cell_filter).await
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

#[component]
fn App() -> Element {
    let force_reload: Signal<i32> = use_signal(|| 0);

    let cells_future = use_resource(use_reactive!(|(force_reload,)| async move {
        println!("Loading cells with {}", force_reload);
        let cell_filter = CellFilter {
            user_id: MY_UUID.to_string(),
        };
        get_cells(cell_filter).await
    }));

    match cells_future.read_unchecked().as_ref() {
        Some(Ok(cells)) => {
            println!("Cell future has Something");
            rsx! {
                link { rel: "stylesheet", href: "tailwind.css" }
                Column { cells: cells.clone(), force_reload }
            }
        }
        Some(Err(err)) => {
            println!("{:?}", err);
            rsx! {
                link { rel: "stylesheet", href: "tailwind.css" }
            }
        }
        None => {
            println!("Cell future is None");
            rsx! {
                link { rel: "stylesheet", href: "tailwind.css" }
            }
        }
    }
}

#[component]
fn Column(cells: Cells, force_reload: Signal<i32>) -> Element {
    rsx! {
        div { class: "flex min-h-screen",
            aside { class: "sticky top-0 h-[calc(100vh-theme(spacing.0))] w-{L_SIDEBAR_W} overflow-y-auto bg-green-200",
                DrawerLeft { force_reload }
            }
            main { class: "flex-1 mt-{NAVBAR_H} left-{L_SIDEBAR_W} right-{R_SIDEBAR_W} bg-yellow-200",
                Cells { cells, force_reload }
                nav { class: "fixed h-{NAVBAR_H} w-full top-0 bg-blue-200",
                    "Nav"
                }
            }
            aside { class: "sticky top-0 h-[calc(100vh-theme(spacing.0))] w-{R_SIDEBAR_W} right-0 overflow-y-auto",
                DrawerRight {}
            }

        }
    }
}

#[component]
fn DrawerLeft(force_reload: Signal<i32>) -> Element {
    rsx! {
        ul { class: "menu w-full min-h-full bg-white text-base-content",
            li {
                a { "Sidebar Item 1" }
            }
            li {
                a { "Sidebar Item 2" }
            }
            CellPostForm { force_reload }
        }
    }
}

#[component]
fn DrawerRight() -> Element {
    rsx! {
        ul { class: "menu p-4 right-0 w-48 min-h-full bg-white text-base-content",
            li {
                a { "Note" }
            }
            li {
                a { "Menu Item B" }
            }
        }
    }
}

#[component]
fn CellPostForm(force_reload: Signal<i32>) -> Element {
    let cell_id = uuid::Uuid::new_v4().to_string();
    let user_id = MY_UUID.to_string();
    let device_id = "Dev0".to_string();
    let mut text = use_signal::<String>(|| "".to_string());
    let is_open = use_signal::<bool>(|| false);
    let rootdir = model::Dir {
        name: "/".to_string(),
        dirs: vec![model::Dir {
            name: "test".to_string(),
            dirs: vec![model::Dir {
                name: "test2".to_string(),
                dirs: Vec::new(),
                fileprops: vec![FileProp {
                    name: "photo.png".to_string(),
                    url: "https://example.com/phto.png".to_string(),
                    completed: true,
                }]
            }],
            fileprops: vec![FileProp {
                name: "photo3.docx".to_string(),
                url: "https://example.com/phto3.docx".to_string(),
                completed: true,
            }],
        }],
        fileprops: vec![FileProp {
            name: "photo2.png".to_string(),
            url: "https://example.com/phto2.png".to_string(),
            completed: true,
        },FileProp {
            name: "marine.txt".to_string(),
            url: "https://example.com/marine.txt".to_string(),
            completed: true,
        },]
    };
    let parent_ids: Vec<String> = Vec::new();
    let child_ids: Vec<String> = Vec::new();

    let mut new_cell_req: Signal<Option<model::CellReq>> = use_signal(|| None);

    let _new_cell_future = use_resource(move || async move {
        let db_url = utils::get_db_path();
        let pool = SqlitePoolOptions::new()
            .max_connections(16)
            .connect(&db_url)
            .await
            .unwrap();
        if let Some(req) = &*new_cell_req.read() {
            println!("I will post new cell.");
            println!("{:?}", req);
            let res = handler::create_cell(pool, req.clone()).await;
            match res {
                Ok(res) => {
                    println!("Successfully posted: {:?}", res);
                    force_reload += 1;
                    println!("Restarted......, right?");
                }
                Err(e) => {
                    println!("Failed: {:?}", e);
                }
            }
        }
    });

    rsx! {
        textarea {
            value: "{text}",
            placeholder: "Bio2",
            oninput: move |event| {
                text.set(event.value().clone());
            },
            // prevent_default: "onchange",
            onchange: move |event| {},
            class: "textarea textarea-bordered textarea-md w-full max-w-xs"
        }
        button {
            class: "ml-2 px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500",
            onclick: move |event| {
                println!("Clicked! Event: {event:?}");
                println!("Current text: {text}");
                let cell_req = CellReq {
                    cell_id: cell_id.clone(),
                    user_id: user_id.clone(),
                    device_id: device_id.clone(),
                    text: text.read().to_string(),
                    is_open: *is_open.read(),
                    rootdir: rootdir.clone(),
                    parent_ids: parent_ids.clone(),
                    child_ids: child_ids.clone(),
                };
                new_cell_req.set(Some(cell_req));
            },
            "Send"
        }
    }
}

#[component]
fn Cells(cells: Cells, force_reload: Signal<i32>) -> Element {
    rsx! {
        div { class: "flex flex-col p-6 items-center bg-base-200",
            for cell in cells.cells.iter() {
                Cell {
                    cell: cell.clone(),
                    force_reload,
                }
            }
        }
    }
}

#[component]
fn Cell(cell: CellExtracted, force_reload: Signal<i32>) -> Element {
    let mut is_trancated: Signal<bool> = use_signal(|| true);

    // let window = web_sys::window().unwrap();
    // let document = window.document().unwrap();
    // let element = document.get_element_by_id("my_element").unwrap();
    // let mut is_menu_open = false;
    rsx! {
        div { class: "w-2/3 my-2 px-6 py-2 rounded-lg bg-white shadow",
            div { class: "text-gray-400",
                "2024/05/25 23:41"
            }
            div { class: "flex divide-x",
                div { class: "flex flex-col w-full h-full items-end mr-4",
                    div { class: "w-full items-start",
                        p { class: if is_trancated() { "line-clamp-4 break-words" } else { "break-words" },
                            "{cell.text}"
                        }
                        button {
                            // id: cell.cell_id.clone(),
                            onclick: move |_evt| is_trancated.toggle(),
                            if is_trancated() { "Read more..." } else { "Read less..." }
                        }
                    }
                    div { class: "flex flex-row mt-4",
                        // DotsMenu {cell_id: cell.cell_id, force_reload}
                        button {
                            class: "ml-4",
                            onclick: move |evt| {
                                println!("push download: {evt:?}");
                                // std::fs::create_dir("t").unwrap();
                                utils::download_dir();
                            },
                            svg_icon::download{},
                        }
                        button {
                            class: "ml-4",
                            onclick: move |evt| {
                                println!("push reply: {evt:?}");
                            },
                            svg_icon::reply{},
                        }
                        button {
                            class: "ml-4",
                            onclick: move |evt| {
                                // del_cell_sig.set(false);
                                // is_menu_open.toggle();
                                println!("push delete: {evt:?}");
                                let cell_id = cell.cell_id.clone();
                                let _del_cell_future = use_resource(move ||
                                    {
                                        let cell_id = cell_id.clone();
                                        async move {
                                            let db_url = utils::get_db_path();
                                            let pool = SqlitePoolOptions::new()
                                                .max_connections(16)
                                                .connect(&db_url)
                                                .await
                                                .unwrap();
                                            println!("I will delete the cell.");
                                            let res = handler::delete_cell(&cell_id, pool).await;
                                            match res {
                                                Ok(res) => {
                                                    println!("Successfully posted: {:?}", res);
                                                    force_reload += 1;
                                                    println!("Restarted......, right?");
                                                }
                                                Err(e) => {
                                                    println!("Failed: {:?}", e);
                                                }
                                            }
                                        }
                                    }
                                );
                            },
                            svg_icon::trash{},
                        } 
                        button {class: "ml-4", svg_icon::dotsmenu{}} 
                    }
                }
                div { class: "min-w-60",
                    tree::FileTree {rootdir: cell.rootdir}
                }
            }
        }
    }
}

#[component]
fn DotsMenu(cell_id: String, force_reload: Signal<i32>) -> Element {

    let mut is_menu_open: Signal<bool> = use_signal(||false);
    // let close_menu = |evt: Event<MouseData>| {
    //     let elem = evt.data.target();
    //     elem.set_focus(false);
    // };

    rsx! {
            ul { class: "menu lg:menu-horizontal rounded-box lg:mb-64",
            li {
                a { "Item 1" }
            }
            li {
                a { "Item 3" }
            }
            li {
                details { open: is_menu_open, 
                    summary { svg_icon::dotsmenu {} }
                    ul {
                        li {
                            onclick: move |evt| {
                                println!("{:?}", is_menu_open);
                                // is_menu_open.toggle();
                                is_menu_open.toggle();
                                println!("{:?}", is_menu_open);
                                // is_menu_open.set(true);            
                            },
                            // onfocus: move |evt| {
                            //     println!("{:?}", is_menu_open);
                            // },
                            a { "Submenu 1" }
                        }
                        li {
                            onclick: move |evt| {
                                // del_cell_sig.set(false);
                                is_menu_open.toggle();
                                println!("push delete: {evt:?}");
                                let cell_id = cell_id.clone();
                                let _del_cell_future = use_resource(move ||
                                    {
                                        let cell_id = cell_id.clone();
                                        async move {
                                            let db_url = utils::get_db_path();
                                            let pool = SqlitePoolOptions::new()
                                                .max_connections(16)
                                                .connect(&db_url)
                                                .await
                                                .unwrap();
                                            println!("I will delete the cell.");
                                            let res = handler::delete_cell(&cell_id, pool).await;
                                            match res {
                                                Ok(res) => {
                                                    println!("Successfully posted: {:?}", res);
                                                    force_reload += 1;
                                                    println!("Restarted......, right?");
                                                }
                                                Err(e) => {
                                                    println!("Failed: {:?}", e);
                                                }
                                            }
                                        }
                                    }
                                );
                            },
                            a { svg_icon::trash {} "削除" },
                        }
                        li {
                            details { open: "false",
                                summary { "Parent" }
                                ul {
                                    li {
                                        a { "item 1" }
                                    }
                                    li {
                                        a { "item 2" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

async fn init_db(db: &sqlx::pool::Pool<sqlx::Sqlite>) -> Result<(), sqlx::Error> {
    let _r = sqlx::query("PRAGMA foreign_keys = ON;").execute(db).await?;

    let _r = sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            user_id     TEXT PRIMARY KEY,
            user_name   TEXT NOT NULL,
            passhash    TEXT NOT NULL,
            created_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
        )",
    )
    .execute(db)
    .await?;

    println!("{:?}", _r);

    let _r = sqlx::query(
        "CREATE TABLE IF NOT EXISTS cells (
            cell_id     TEXT PRIMARY KEY,
            user_id     TEXT NOT NULL,
            device_id   TEXT NOT NULL,
            text        TEXT NOT NULL,
            rootdir     TEXT NOT NULL,
            is_open     INTEGER NOT NULL CHECK (is_open IN (0, 1)),
            created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
    )
    .execute(db)
    .await?;

    println!("{:?}", _r);

    let _r = sqlx::query(
        "CREATE TABLE IF NOT EXISTS family_tree (
            child_id    TEXT NOT NULL,
            parent_id   TEXT NOT NULL,
            FOREIGN KEY (child_id) REFERENCES cells(cell_id),
            FOREIGN KEY (parent_id) REFERENCES cells(cell_id)
        )",
    )
    .execute(db)
    .await?;

    println!("{:?}", _r);

    Ok(())
}
