use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use tracing::info;

use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::user::{User, UserBmc};

type Db = Pool<Postgres>;

// NOTE: Hardcode to prevent deployed system db update.
//  POSTGRES
const PG_DEV_POSTGRES_URL: &str = "postgres://postgres:japama@127.0.0.1/postgres";
const PG_DEV_APP_URL: &str = "postgres://postgres:japama@127.0.0.1/teacherinator";

// sql files
const SQL_RECREATE_DB_FILE_NAME: &str = "00-recreate-db.sql";
const SQL_DIR: &str = "sql/dev_initial";

const DEMO_PWD: &str = "welcome";

pub async fn init_dev_db() -> Result<(), Box<dyn std::error::Error>> {
    info!("{:<12} - init_dev_db()", "FOR-DEV-ONLY");

    // -- Get the sql_dir
    // Note: This is because cargo test and cargo run won't give the same
    //       current_dir given the worspace layout.
    let current_dir = std::env::current_dir().unwrap();
    let v: Vec<_> = current_dir.components().collect();
    let path_comp = v.get(v.len().wrapping_sub(3));
    let base_dir = if Some(true) == path_comp.map(|c| c.as_os_str() == "crates") {
        v[..v.len() - 3].iter().collect::<PathBuf>()
    } else {
        current_dir.clone()
    };
    let sql_dir = base_dir.join(SQL_DIR);

    // -- Create the app_db/app_user with the postgres user.
    {
        let sql_recreate_db_file = sql_dir.join(SQL_RECREATE_DB_FILE_NAME);
        let root_db = new_db_pool(PG_DEV_POSTGRES_URL).await?;
        pexec(&root_db, &sql_recreate_db_file).await?;

    }

    // -- Get sql files.
    let mut paths: Vec<PathBuf> = fs::read_dir(sql_dir)?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect();
    paths.sort();

    // -- SQL Execute each file.
    let app_db = new_db_pool(PG_DEV_APP_URL).await?;

    for path in paths {
        let path_str = path.to_string_lossy();

        if path_str.ends_with(".sql") && !path_str.ends_with(SQL_RECREATE_DB_FILE_NAME) {
            pexec(&app_db, &path).await?;
        }
    }

    // -- Init model layer.
    let mm = ModelManager::new().await?;
    let ctx = Ctx::root_ctx();

    // -- Set admin pwd
    let admin_user: User = UserBmc::first_by_username(&ctx, &mm, "admin")
        .await?
        .unwrap();
    UserBmc::update_pwd(&ctx, &mm, admin_user.id, DEMO_PWD).await?;

    let admin_user: User = UserBmc::first_by_username(&ctx, &mm, "profesor1")
        .await?
        .unwrap();
    UserBmc::update_pwd(&ctx, &mm, admin_user.id, DEMO_PWD).await?;
    
    info!("{:<12} - init_dev_db - set admin pwd", "FOR-DEV-ONLY");

    Ok(())
}

async fn pexec(db: &Db, file: &Path) -> Result<(), sqlx::Error> {
    info!("{:<12} - pexec: {file:?}", "FOR-DEV-ONLY");

    // -- Read the file.
    let content = fs::read_to_string(file)?;

    // FIXME: Make the split more sql proof.
    let sqls: Vec<&str> = content.split(';').collect();

    for sql in sqls {
        sqlx::query(sql).execute(db).await?;
    }

    Ok(())
}

async fn new_db_pool(db_con_url: &str) -> Result<Db, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_millis(500))
        .connect(db_con_url)
        .await
}
