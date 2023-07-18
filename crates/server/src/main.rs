use std::path::PathBuf;

use axum::{routing::get, Router, extract::State, Json};
use chrono::{Utc, Duration};
use shared::models::clicks::Clicks;
use shuttle_persist::PersistInstance;
use tower_http::services::ServeDir;

static CLICKS_KEY: &str = "clicks";

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn get_clicks(state: State<AppState>) -> Json<Clicks> {
    let persist = &state.persist;
    let clicks = persist.load::<Clicks>(CLICKS_KEY).unwrap_or_default();
    Json(clicks)
}

async fn add_clicks(state: State<AppState>) -> Result<Json<Clicks>, String> {
    let persist = &state.persist;
    let clicks = persist.load::<Clicks>(CLICKS_KEY).unwrap_or_default();
    let now = Utc::now();
    if now > clicks.last_click_ts + Duration::seconds(5) {
        let clicks = Clicks {
            amount: clicks.amount + 1,
            last_click_ts: now,
        };
        let _ = persist.save(CLICKS_KEY, &clicks);
        Ok(Json(clicks))
    }
    else {
        Err("Clicks need to wait 5s".to_string())
    }
}

#[derive(Clone)]
struct AppState {
    persist: PersistInstance,
}

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_static_folder::StaticFolder(folder = "public")] public_folder: PathBuf,
    #[shuttle_persist::Persist] persist: PersistInstance,
) -> shuttle_axum::ShuttleAxum {
    let state = AppState {
        persist,
    };
    let router = Router::new()
        .nest_service("/", ServeDir::new(public_folder))
        .route("/api/hello", get(hello_world))
        .route("/api/clicks", get(get_clicks).post(add_clicks))
        .with_state(state);

    Ok(router.into())
}
