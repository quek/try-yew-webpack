pub fn path(name: &'static str) -> std::string::String {
    (js! { return window.assets(@{name}); }).into_string().unwrap()
}
