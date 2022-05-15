use std::fs;
use std::path::{Path, PathBuf};
use crate::error::AppError;
use crate::exe_directory;

pub mod app_state;
pub mod tabs;
pub mod tab;
pub mod new_tab;
pub mod tab_content;
pub mod config;

const DIR : &str = "./docs";

fn docs_path() -> Result<PathBuf, AppError> {
    Ok(exe_directory()?.join(DIR))
}

fn docs(path: &Path) -> Result<Vec<PathBuf>, AppError> {
    let mut docs = Vec::new();
    for entry_result in fs::read_dir(path)? {
        if let Ok(entry) = entry_result {
            docs.push(entry.path());
        }
    }

    Ok(docs)
}