//! 📥️ Items of `native` that belong to `os-store-one-item-batch` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🦀️.rs`.

use semio_framework_value::DslValue;
fn retire_projection(value: DslValue) {
    let mut retirement = semio_framework_value::IntrinsicRetirement::new(value);
    while !retirement.terminal_is_empty() { retirement.close_step(256); }
}
