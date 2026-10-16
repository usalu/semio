use std::env;

fn main() {
    if env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        let stack = if env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc") { "/STACK:8388608" } else { "-Wl,--stack,8388608" };
        println!("cargo:rustc-link-arg-bin=semio-wgpu-native={stack}");
    }
}
