//! 🌆️ Companion tests of the DSL derive expansion library: macro export roster, mutation source authority, leaf and aggregate derives, composite attributes and diff codecs.
//!
//! 🧭️ The tests are mounted unchanged from the production tree and reach the production crate through its public API only.
#![cfg(test)]

pub use semio_framework_os_kernel_dsl_derive_expand::*;
extern crate proc_macro;

#[path = "."]
pub mod component {
    pub use semio_framework_os_kernel_dsl_derive_expand::testkit::component::*;
    use proc_macro::TokenStream;
    use quote::quote;
    use std::fs;
    use std::path::{Path, PathBuf};
    use syn::{Data, DeriveInput, Fields, Type, parse_macro_input};

    #[path = "../📤️macro-exports/🦀️.rs"]
    mod macro_export_tests;

    #[path = "../🪪️mandatory-mutation-descriptor/🦀️.rs"]
    mod mandatory_mutation_descriptor_tests;

    #[path = "../🔬️mutation-source-authority/🦀️.rs"]
    mod mutation_source_authority_tests;

    #[path = "../🔬️mutation-aggregate-source-authority/🦀️.rs"]
    mod mutation_aggregate_source_authority_tests;

    #[path = "../🔬️mutation-leaf-json/🦀️.rs"]
    mod mutation_leaf_json_tests;

    #[path = "../🔬️mutation-leaf-derive/🦀️.rs"]
    mod mutation_leaf_derive_tests;

    #[path = "../🔬️mutation-attrs/🦀️.rs"]
    mod mutation_attrs_tests;

    #[path = "../🔬️mandatory-mutations/🦀️.rs"]
    mod mandatory_mutations_tests;

    #[path = "../🔬️composite-attrs/🦀️.rs"]
    mod composite_attrs_tests;

    #[path = "../🔬️composite-timestamp/🦀️.rs"]
    mod composite_timestamp_tests;

    #[path = "../🪆️record-owner/🦀️.rs"]
    mod canonical_product_macro_tests;

    #[path = "../🚪️diff-codecs/🦀️.rs"]
    mod diff_codec_tests;
}
