//! 🧩️ Expansion library of the DSL derives: the implementation owner `../🦀️.rs` compiled as an ordinary library, so its units are testable; the proc-macro entry points live in `../📦️packages/🦀️rust/🦀️.rs`.

extern crate proc_macro;

use semio_framework_native_build_compiler_resources as compiler_resources;

#[path = "../🦀️.rs"]
mod component;
pub use component::document_diff_and_composite_expansion::{expand_derive_composite_mutation, expand_diff_binary, expand_diff_text, expand_dsl_document};
pub use component::mutation_leaf_attrs_and_schema_index::expand_mutation_leaf;
pub use component::mutations_expansion::expand_derive_mutations;
