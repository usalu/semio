use std::env;

/// 🪒️ Norm component link adapter: strips the symbol-name section from every norm wasm component, so a prototype component
/// (which keeps `strip = "none"` for its rlibs) stays under the 512 MiB component admission limit; only the final cdylib relinks.
fn main() {
    if env::var("CARGO_CFG_TARGET_ARCH").as_deref() == Ok("wasm32") {
        println!("cargo:rustc-link-arg-cdylib=--strip-all");
    }
}
