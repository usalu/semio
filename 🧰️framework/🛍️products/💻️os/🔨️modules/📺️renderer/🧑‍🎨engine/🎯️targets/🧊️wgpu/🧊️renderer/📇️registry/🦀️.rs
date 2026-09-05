//! 📇️ Renderer macro registration.

#[cfg(not(target_os = "wasi"))]
#[macro_export]
macro_rules! action_args_json {
    ($($tt:tt)*) => {
        semio_framework::optional_json_to_dsl(Some(serde_json::json!($($tt)*)))
    };
}

