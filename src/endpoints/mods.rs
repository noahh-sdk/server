use std::io::Write;

use actix_web::{get, web, Responder, post};
use futures::StreamExt;

use crate::{AppData, Error};
use crate::types::models::Mod;

#[get("/v1/mods")]
pub async fn index(data: web::Data<AppData>) -> Result<impl Responder, Error> {
    let mut pool = data.db.acquire().await.or(Err(Error::DbAcquireError))?;
    let mods = sqlx::query_as!(Mod, "SELECT * FROM mods")
        .fetch_all(&mut *pool)
        .await.or(Err(Error::DbError))?;
    Ok(web::Json(mods))
}

#[get("/v1/mods/{id}")]
pub async fn get(id: String, data: web::Data<AppData>) -> Result<impl Responder, Error> {
    let mut pool = data.db.acquire().await.or(Err(Error::DbAcquireError))?;
    let res = sqlx::query_as!(Mod, r#"SELECT * FROM mods WHERE id = ?"#, id)
        .fetch_one(&mut *pool)
        .await.or(Err(Error::DbError))?;

    Ok(web::Json(res))
}

#[post("/v1/mods/{id}")]
pub async fn create(id: String, data: web::Data<AppData>, mut noahh_file: web::Payload) -> Result<impl Responder, Error> {
    // todo: authenticate
    let mut file = std::fs::File::open(format!("db/temp_{id}.noahh")).or(Err(Error::FsError))?;
    //                                                   ^ todo: sanitize
    let mut written = 0usize;
    while let Some(chunk) = noahh_file.next().await {
        let chunk = chunk.map_err(|e| Error::UploadError(e.to_string()))?;
        written += chunk.len();
        if written > 262_144 {
            return Err(Error::UploadError("file too large".to_string()));
        }
        file.write_all(&chunk).or(Err(Error::FsError))?;
    }

    // todo: load info from noahh file and add to database

    Ok(web::Json(None::<()>))
}