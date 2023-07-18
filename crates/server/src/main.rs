use std::path::PathBuf;

use axum::{routing::get, Router, extract::State};
use shuttle_persist::PersistInstance;
use tower_http::services::ServeDir;

static CLICKS_KEY: &str = "clicks";

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn get_clicks(state: State<AppState>) -> String {
    let persist = &state.persist;
    let clicks = persist.load::<u32>(CLICKS_KEY).unwrap_or_default();
    clicks.to_string()
}

async fn add_clicks(state: State<AppState>) -> String {
    let persist = &state.persist;
    let clicks = persist.load::<u32>(CLICKS_KEY).unwrap_or_default();
    let new_value = clicks + 1;
    let _persist_result = persist.save(CLICKS_KEY, new_value);
    new_value.to_string()
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
