/// wraps on top of `web_sys::console.log_1`, use it like:
/// ```ignore
/// util::log!("a is {}", a);
/// ```
#[macro_export]
macro_rules! log {
  ($($t:tt)*) => {{
    web_sys::console::log_1(&format!($($t)*).into());
  }};
}
