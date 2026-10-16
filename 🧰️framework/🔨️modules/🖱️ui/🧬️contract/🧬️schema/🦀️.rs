//! 🧬️ Actual document scope registration and optional projection wiring.
pub use semio_framework_ui_contract_presentation::schema_metadata::{register_scope_exports,DECLARED_FORMATS};
#[cfg(feature="typegen")]
pub use semio_framework_ui_contract_typegen::{SchemaMetadata,TYPES,validate,render_typescript};
