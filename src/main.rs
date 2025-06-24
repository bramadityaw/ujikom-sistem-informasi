use askama::Template;
use serde::{Deserialize, Serialize};
use serde_with::rust::string_empty_as_none;
use sqlx::{SqlitePool, prelude::FromRow};

use axum::{
    Router,
    extract::{Query, State},
    response::{Html, IntoResponse},
    routing::get,
};

const DB_URL: &str = "sqlite://db/sqlite.db";

#[derive(Clone)]
struct AppState {
    pool: SqlitePool,
}

impl AppState {
    pub async fn new() -> anyhow::Result<Self> {
        let pool = SqlitePool::connect(DB_URL).await?;
        Ok(AppState { pool })
    }
}

#[derive(Debug, Deserialize)]
struct Params {
    #[serde(default, with = "string_empty_as_none")]
    cari: Option<i64>,
}

#[derive(FromRow, Serialize, Deserialize)]
struct Pegawai {
    nip: String,
    nama: String,
    // TODO: parse two fields below to specialized types
    alamat: String,
    tanggal_lahir: String,
    // TODO: make this an enum
    kode_divisi: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct HomepageTemplate {
    pegawai: Option<Pegawai>,
}

async fn pegawai_nip(params: Query<Params>, State(state): State<AppState>) -> impl IntoResponse {
    match params.0.cari {
        None => {
            let template = HomepageTemplate { pegawai: None };
            Html(template.render().unwrap())
        }
        Some(nip) => {
            match sqlx::query_as::<_, Pegawai>(
                "select nip, nama, alamat, tanggal_lahir, kode_divisi from pegawai where nip=?;",
            )
            .bind(nip)
            .fetch_one(&state.pool)
            .await
            {
                Ok(pegawai) => {
                    let template = HomepageTemplate {
                        pegawai: Some(pegawai),
                    };
                    Html(template.render().unwrap())
                }
                Err(_) => {
                    let template = HomepageTemplate { pegawai: None };
                    Html(template.render().unwrap())
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let state = AppState::new().await?;
    let app = Router::new().route("/", get(pegawai_nip)).with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    eprintln!("Listening in http://0.0.0.0:3000");
    axum::serve(listener, app).await?;
    Ok(())
}
