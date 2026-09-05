//! 🧪️ Acceptance test 4: the zero-variant case, `dyn_enum_close! { pub enum NoMembers: Trait {} }` —
//! the default type parameter for the many plugins that compose nothing. Must generate `impl Trait for
//! NoMembers` with `match *self {}` bodies (verified against real rustc — matching a REFERENCE to an
//! uninhabited type with zero arms is rejected as "non-exhaustive .. references are always considered
//! inhabited"; matching the DEREF'd place is accepted — `📓️terra-dyn-enum-macro-report.md`).
#![allow(async_fn_in_trait)] // R7 — never resolved by `+ Send` or by making a method sync.

use semio_framework_dispatch_macros::{dyn_enum, dyn_enum_close};

// `&mut self` is deliberately NOT mixed with `self: Arc<Self>` here — `#[dyn_enum]` rejects that
// combination on the SAME trait (an `Arc<Self>` method needs shared `Arc<Concrete>` variant storage,
// which `&mut self` cannot safely reach through; see `analyze_rejects_arc_self_mixed_with_mut_self` in
// `🦀️.rs`). `&mut self`'s own uninhabited case is covered separately by `✍️mutable-receiver.rs`.
#[dyn_enum]
pub trait Widget {
    async fn render(&self) -> String;
    fn destroy(self) -> u32;
    fn share(self: std::sync::Arc<Self>) -> u32;
}

dyn_enum_close! {
    pub enum NoWidgets: Widget {}
}

/// 🕳️ `NoWidgets` has zero variants — a value of this type can never exist. This function only needs
/// to TYPE-CHECK, not run; its existence proves `impl Widget for NoWidgets` compiles for every receiver
/// kind used above (`&self`, `self`, `self: Arc<Self>`) with zero match arms.
#[allow(dead_code, unreachable_code)]
fn assert_widget_impl_compiles(never: NoWidgets, arc_never: std::sync::Arc<NoWidgets>) {
    let _ = never.destroy();
    let _ = arc_never.share();
}

#[test]
fn no_widgets_type_checks_and_has_a_well_defined_size() {
    // 🕳️ The real proof is `assert_widget_impl_compiles` above — this just confirms the type exists
    // and is usable in ordinary generic/runtime positions (e.g. inside an `Option<NoWidgets>`) even
    // though no value of it can ever be constructed.
    let _ = std::mem::size_of::<NoWidgets>();
    let absent: Option<NoWidgets> = None;
    assert!(absent.is_none());
}
