use dioxus::prelude::*;
use shared::models::clicks::Clicks;
use web::get_host;

pub(crate) fn clicky(cx: Scope) -> Element {
    let clicks = use_state(cx, || 0);
    let _get_clicks = move || {
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
            }
        });
    };

    // get_clicks();
    cx.render(rsx! (
        div {
            id: "count-container",
            h1 { "🖱️ Clicky 🚀" }
            h3 { "Join the Click Revolution!"}
            p {
                "Why should you click this button, you ask? Because each click you make is a commitment to innovation and progress! It's a stand against the ordinary, a declaration that you too, are part of the driving force pushing the boundaries of possibility. The counter may just be a number, but each increment represents a leap forward, a step into the future. Don't just stand by the sidelines. Join us, make your click count, and be part of this amazing journey!"
            }
            p { 
                i { "A click can only be added once every 5 second." }
            }
            p { 
                id: "counter",
                "Total Clicks: {clicks}"
            }
            button {
                onclick: add_clicks,
                id: "increment-button",
                "Click me!"
            }
        }
    ))
}
