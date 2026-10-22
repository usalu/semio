//! 📃️ Canonical flat semantic documents, exact resident custody, assembly and paged patches.
pub use semio_framework_ui_contract_values::*;
pub use semio_framework_ui_contract_actions::*;
pub use semio_framework_ui_contract_presentation::*;
pub use semio_framework_ui_contract_components::*;
pub use semio_framework_ui_contract_operations::*;
pub use semio_framework_ui_contract_builder::*;
pub use semio_framework_ui_contract_text_edit::*;
pub use semio_framework_ui_contract_identity::*;
#[path="📃️document/🦀️.rs"]
mod document;
pub use document::*;
#[path="🧾️fields/🦀️.rs"]
mod fields;
#[path="♻️retirement/🌳️typed/📋️fields/🦀️.rs"]
mod typed_retirement;
#[path="🧾️copy/🦀️.rs"]
mod typed_copy;
#[path="🧾️compare/🦀️.rs"]
mod typed_compare;
#[path="♻️retirement/👷️worker/🦀️.rs"]
mod worker_retirement;
pub use worker_retirement::{ui_worker_retirement_admission,ui_worker_retirement_permits,UI_WORKER_RETIREMENT_POLICY};
#[path="🧬️schema/🦀️.rs"]
pub mod schema_metadata;
#[path="♿️accessibility/🦀️.rs"]
mod accessibility_projection;
pub use accessibility_projection::*;

#[cfg(test)]
/// 🚧️ Serializes the tests that open, assemble or compare through the process-wide retained arena and resident ledger.
pub(crate) fn arena_test_gate() -> std::sync::MutexGuard<'static, ()> {
    static GATE: std::sync::Mutex<()> = std::sync::Mutex::new(());
    GATE.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}
