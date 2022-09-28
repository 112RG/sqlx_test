use chrono::{DateTime, NaiveDateTime, Utc};
use std::time::SystemTime;
use anyhow::{Ok, Result};
use walkdir::WalkDir;

macro_rules! skip_fail {
    ($res:expr) => {
        match $res {
            std::result::Result::Ok(val) => val,
            Err(e) => {
                println!("An error occured: {}; skipped.", e);
                continue;
            }
        }
    };
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");

    Ok(())
}

pub async fn walk_full(db: &DatabaseConnection) -> Result<()> {
    println!("Starting scan");
    //let dirs: Vec<entity::directories::Model> = entity::directories::Entity::find().all(db).await?;
    let current_dir: &str = "/mnt/end/aa";
    for entry in WalkDir::new(current_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path: String = entry.path().to_string_lossy().to_string();
        if entry.file_type().is_dir() {
            let fmtime: SystemTime = entry.metadata().unwrap().modified().unwrap();
            let mtime: DateTime<Utc> = fmtime.into();
            insert_directory(&path, &mtime, db).await?;
        }
        let f_name = entry.file_name().to_string_lossy();
        if f_name.ends_with(".flac") {
            let metadata = skip_fail!(tag_helper::get_metadata(path.to_owned()));
            skip_fail!(services::song::create_song(db, metadata).await);
        }
        if f_name.contains("cover.") {
            println!("Found cover for {:?}", path);
            services::album::update_cover_for_path(
                db,
                path,
                entry.path().parent().unwrap().to_string_lossy().to_string(),
            )
            .await?;
        }
    }

    Ok(())
}

async fn create_song()
