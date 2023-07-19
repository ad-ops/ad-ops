use dioxus::prelude::*;
use log::LevelFilter;
use shared::models::clicks::Clicks;

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    log::info!("starting app");
    dioxus_web::launch(app);
}

fn get_host() -> String {
    let window = web_sys::window().expect("no global `window` exists");
    let location = window.location();
    let host = location.host().expect("should have a host");
    let protocol = location.protocol().expect("should have a protocol");
    let endpoint = format!("{protocol}//{host}");
    endpoint
}

fn app(cx: Scope) -> Element {
    let clicks = use_state(cx, || 0);
    let get_clicks = move || {
        cx.spawn({
            let clicks = clicks.to_owned();
            async move {
                let request = reqwest::get(format!("{}/api/clicks", &get_host())).await;
                if let Ok(response) = request {
                    if let Ok(parsed) = response.json::<Clicks>().await {
                        clicks.set(parsed.amount);
                        return;
                    }
                }
                clicks.set(0);
            }
        });
    };
    let add_clicks = move |_| {
        cx.spawn({
            let clicks = clicks.to_owned();
            async move {
                let request = reqwest::Client::new()
                    .post(format!("{}/api/clicks", &get_host()))
                    .send()
                    .await;
                if let Ok(response) = request {
                    if let Ok(parsed) = response.json::<Clicks>().await {
                        clicks.set(parsed.amount);
                        return;
                    }
                }
                clicks.set(0);
            }
        });
    };

    // get_clicks();
    cx.render(rsx! (
        div {
            style: "text-align: center;",
            h1 { "🖱️ Clicker 🚀" }
            h3 { "Please help increase the Clicks!"}
            p { "Collect more clicks with together with others!" }
            p { "A click can only be added once every 5 second." }
            strong { "Total Clicks: {clicks}" }
            p {}
            button {
                onclick: add_clicks,
                "Click!"
            }
        }
    ))
}
