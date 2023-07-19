use dioxus::prelude::*;
  
pub(crate) fn home(cx: Scope) -> Element {
    cx.render(rsx! (
        h1 { "Home" }
        p { "These are all the strange things made with Dioxus." }
    ))
}
