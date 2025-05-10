use std::fs;
use std::io::{BufReader, Read};
// use sha2::{Sha256, Digest};
use serde::Deserialize;
use crate::model::{Dir};

use reqwest;
use std::fs;
use std::io::prelude::*;
use std::path::Path;
use std::error::Error;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Debug, Deserialize)]
struct Config {
  web_url: Option<String>,
  db_path: Option<String>,
}

// Ref: https://cipepser.hatenablog.com/entry/rust-toml
fn read_file(path: String) -> Result<String, String> {
  let mut file_content = String::new();
  let mut fr = fs::File::open(path)
      .map(|f| BufReader::new(f))
      .map_err(|e| e.to_string())?;
  fr.read_to_string(&mut file_content)
      .map_err(|e| e.to_string())?;
  Ok(file_content)
}

fn get_config() -> Config {
  let path = "./flowfs.toml";
  let s = match read_file(path.to_owned()) {
    Ok(s) => s,
    Err(e) => panic!("fail to read file: {}", e),
  };
  let config: Result<Config, toml::de::Error> = toml::from_str(&s);
  match config {
    Ok(c) => return c,
    Err(e) => panic!("fail to parse toml: {}", e),
  };
}

pub fn get_url() -> String {
  get_config()
    .web_url
    .unwrap_or("127.0.0.1:8080".to_string())
}

pub fn get_db_path() -> String {
  get_config()
    .db_path
    .unwrap_or("flowfs".to_string())
}

// Function to download a file from a URL
async fn download_file(url: &str, output_path: &Path) -> Result<(), Box<dyn Error>> {
  let response = reqwest::get(url).await?.bytes().await?;
  let mut file = fs::File::create(output_path)?;
  file.write_all(&response)?;
  Ok(())
}

// Function to create directories recursively
fn create_directories(path: &Path) -> std::io::Result<()> {
  if !path.exists() {
      fs::create_dir_all(path)?;
  }
  Ok(())
}

// Function to recursively download files and create directories
async fn download_dir(dir: &Dir, base_path: &Path) -> Result<(), Box<dyn Error>> {
  let current_path = base_path.join(&dir.name);
  create_directories(&current_path)?;

  // Download files in the current directory
  for fileprop in &dir.fileprops {
      let file_path = current_path.join(&fileprop.name);
      download_file(&fileprop.url, &file_path).await?;
  }

  // Recurse into subdirectories
  for sub_dir in &dir.dirs {
      download_dir(sub_dir, &current_path).await?;
  }

  Ok(())
}

/*
pub fn download_tree(rootdir: Dir) {
  if rootdir.name == "/" {
    {
      std::fs::create_dir("root_dir").unwrap();
        for fileprop in rootdir.fileprops {
                /*li {
                    a { svg_icon::general_file {} {fileprop.name} }
                }
                */
            }
            for subdir in rootdir.dirs {
                /*li {
                    FileTree { rootdir: subdir }
                }
                */
            }
    }
  } else {
    {
        details { open: "false", 
            summary { svg_icon::directory {} {rootdir.name} }
            ul {
                for fileprop in rootdir.fileprops {
                    /*
                    li {
                        a { svg_icon::general_file {} {fileprop.name} }
                    }
                    */
                }
                for subdir in rootdir.dirs {
                    /*
                    li {
                        FileTree { rootdir: subdir }
                    }
                    */
                }
            }
        }
    }
  }
}
*/