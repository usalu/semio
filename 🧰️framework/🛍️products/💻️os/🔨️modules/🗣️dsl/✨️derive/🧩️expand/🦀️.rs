//! 🧩️ Expansion library of the DSL derives: the implementation owner `../🦀️.rs` compiled as an ordinary library, so its units are testable; the proc-macro entry points live in `../📦️packages/🦀️rust/🦀️.rs`.

extern crate proc_macro;

use semio_framework_native_build_compiler_resources as compiler_resources;

#[path = "../🦀️.rs"]
mod component;
pub use component::document_diff_and_composite_expansion::{expand_derive_composite_mutation, expand_diff_binary, expand_diff_text, expand_dsl_document};
pub use component::mutation_leaf_attrs_and_schema_index::expand_mutation_leaf;
pub use component::mutations_expansion::expand_derive_mutations;

/// 🪝️ Test hook of the companion crate `semio-framework-os-kernel-dsl-derive-tests`: the private expansion units, public only behind the `testkit` feature.
#[cfg(feature = "testkit")]
#[doc(hidden)]
pub mod testkit {
    pub mod component {
        pub use crate::component::source_authority::{MUTATION_AUTHORITY_LOCATOR, MUTATION_AUTHORITY_SCHEMA, MUTATION_SCHEMA_INDEX_LOCATOR, MutationSourceAuthority, mutation_aggregate_source_authority, mutation_authority_aggregate_sources, mutation_authority_common, mutation_authority_domain_owners, mutation_authority_locator, mutation_authority_relative, mutation_authority_segment, mutation_authority_workspace_root, mutation_source_authority};
        pub use crate::component::mutation_leaf_descriptor::{emit_mutation_leaf_descriptor, parse_mutation_leaf_descriptor};
        pub use crate::component::mutation_leaf_attrs_and_schema_index::{MutationLeafAttrs, expand_mutation_leaf, mutation_authority_workspace_token, mutation_leaf_payload_schema_path, mutation_leaf_portable_path, mutation_leaf_withdraw_only, mutation_leaf_workspace_token, mutation_schema_document_index, parse_mutation_leaf_attrs};
        pub use crate::component::document_diff_and_composite_expansion::{diff_binary_tokens, diff_text_tokens, expand_composite_mutation, expand_derive_composite_mutation, expand_diff_binary, expand_diff_text, expand_dsl_document, parse_composite_attrs};
        pub use crate::component::mutations_expansion::{expand_derive_mutations, expand_mutations, parse_mutations_attrs};
    }
}
