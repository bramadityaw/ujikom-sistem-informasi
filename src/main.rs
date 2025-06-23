use serde::{Deserialize, Serialize};
use sqlx::{
    Sqlite, SqlitePool,
    migrate::{MigrateDatabase, Migrator},
    prelude::FromRow,
};

use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};

const DB_URL: &str = "sqlite://db/sqlite.db";

#[derive(Clone)]
struct AppState {
    pool: SqlitePool,
}

impl AppState {
    async fn create_db() -> anyhow::Result<()> {
        if !Sqlite::database_exists(DB_URL).await? {
            println!("Creating database {}", DB_URL);
            match Sqlite::create_database(DB_URL).await {
                Ok(_) => println!("Create db success"),
                Err(error) => {
                    eprintln!("error: {}", error);
                    return Ok(());
                }
            }
        } else {
            println!("Database already exists");
        }

        Ok(())
    }
    pub async fn new() -> anyhow::Result<Self> {
        Self::create_db().await?;
        let pool = SqlitePool::connect(DB_URL).await?;
        let crate_dir = std::env::var("CARGO_MANIFEST_DIR")?;
        let migrations = std::path::Path::new(&crate_dir).join("./migrations");

        match Migrator::new(migrations).await?.run(&pool).await {
            Ok(_) => println!("Migration success"),
            Err(error) => {
                eprintln!("error: {}", error);
                return Err(error.into());
            }
        }

        Ok(AppState { pool })
    }
}

async fn hello_world() -> &'static str {
    "Hello Axum!"
}

#[derive(FromRow, Serialize, Deserialize)]
struct Pegawai {
    nip: i64,
    nama: String,
    // TODO: parse two fields below to specialized types
    alamat: String,
    tanggal_lahir: String,
    // TODO: make this an enum
    kode_divisi: String,
}

async fn semua_pegawai(State(state): State<AppState>) -> Json<Vec<Pegawai>> {
    let pegawai =
        match sqlx::query_as("select nip, nama, alamat, tanggal_lahir, kode_divisi from pegawai")
            .fetch_all(&state.pool)
            .await
        {
            Ok(ps) => ps,
            Err(e) => {
                eprintln!("error: {}", e);
                Vec::new()
            }
        };

    Json(pegawai)
}

async fn pegawai_nip(Path(nip): Path<i64>, State(state): State<AppState>) -> Json<Vec<Pegawai>> {
    let pegawai = match sqlx::query_as(
        "select nip, nama, alamat, tanggal_lahir, kode_divisi from pegawai where nip=?",
    )
    .bind(nip)
    .fetch_all(&state.pool)
    .await
    {
        Ok(ps) => ps,
        Err(e) => {
            eprintln!("error: {}", e);
            Vec::new()
        }
    };

    Json(pegawai)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let state = AppState::new().await?;
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/pegawai", get(semua_pegawai))
        .route("/pegawai/{nip}", get(pegawai_nip))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
