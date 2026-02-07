use serde::Serialize;
use sqlx::{PgConnection, Row};
use crate::{types::{models::mod_version::ModVersion, api::PaginatedData}, Error};

#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct Mod {
    pub id: String,
    pub repository: String,
    pub latest_version: String,
    pub validated: bool,
    pub versions: Vec<ModVersion>
}

impl Mod {
    pub async fn get_index(pool: &mut PgConnection, page: i64, per_page: i64, query: String) -> Result<PaginatedData<Mod>, Error> {

        #[derive(Debug)]
        struct ModRecord {
            id: String,
            repository: String,
            latest_version: String,
            validated: bool,
        }

        let limit = per_page;
        let offset = (page - 1) * per_page;

        let mut query_string = "%".to_owned();
        query_string.push_str(&query);
        query_string.push('%');

        // 🔥 RUNTIME VERSION — NO MACRO
        let records: Vec<ModRecord> = sqlx::query_as::<_, ModRecord>(
            "SELECT id, repository, latest_version, validated 
             FROM mods 
             WHERE id LIKE $1 
             LIMIT $2 OFFSET $3"
        )
            .bind(&query_string)
            .bind(limit)
            .bind(offset)
            .fetch_all(&mut *pool)
            .await
            .map_err(|_| Error::DbError)?;

        // 🔥 RUNTIME VERSION — NO MACRO
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM mods"
        )
            .fetch_one(&mut *pool)
            .await
            .map_err(|_| Error::DbError)?
            .unwrap_or(0);

        let ids = records.iter().map(|x| x.id.as_str()).collect();
        let versions = ModVersion::get_versions_for_mods(pool, ids).await?;

        let ret = records.into_iter().map(|x| {
            let version_vec = versions.get(&x.id).cloned().unwrap_or_default();
            Mod {
                id: x.id,
                repository: x.repository,
                latest_version: x.latest_version,
                validated: x.validated,
                versions: version_vec
            }
        }).collect();

        Ok(PaginatedData { data: ret, count })
    }
}
