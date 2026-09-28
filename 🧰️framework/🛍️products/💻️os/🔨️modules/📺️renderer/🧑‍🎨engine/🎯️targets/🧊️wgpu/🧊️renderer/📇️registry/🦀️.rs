//! 📇️ Renderer macro registration.

#[cfg(not(target_os = "wasi"))]
#[macro_export]
macro_rules! action_args_json {
    ($($tt:tt)*) => {
        Some(semio_framework::dsl_value!($($tt)*))
    };
}

