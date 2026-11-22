//! 🧱️ Reserves the WebAssembly shadow stack for Remodel's actor lifecycle.

/// 📏️ Fits the measured 15.5 MB of active prototype viewer-opening frames with room for the remaining calls.
fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    if std::env::var("TARGET").as_deref() == Ok("wasm32-wasip2") {
        println!("cargo::rustc-link-arg-cdylib=-zstack-size=33554432");
    }
}
