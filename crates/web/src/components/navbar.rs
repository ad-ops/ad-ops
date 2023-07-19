use dioxus::prelude::*;
use dioxus_router::Link;

pub(crate) fn navbar(cx: Scope) -> Element {
    cx.render(rsx! (
      nav {
          ul {
              id: "nav-bar",
              li {
                  Link { to: "/", "Home" }
              }
              li {
                  Link { to: "/clicky", "Clicky" }
              }
          }
      }
    ))
}
