pub fn get_host() -> String {
    let window = web_sys::window().expect("no global `window` exists");
    let location = window.location();
    let host = location.host().expect("should have a host");
    let protocol = location.protocol().expect("should have a protocol");
    let endpoint = format!("{protocol}//{host}");
    endpoint
}