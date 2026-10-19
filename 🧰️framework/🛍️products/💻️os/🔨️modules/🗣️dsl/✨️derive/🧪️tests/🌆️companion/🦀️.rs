//! 🌆️ Companion tests of the DSL derive expansion library through source and public facts only: the proc-macro export roster, the product macro owner and the base mutation declaration contract.
//!
//! 🧭️ The tests parse the production tree with `syn` against language-neutral fixtures and reach no private item; the laws that do live inside the expansion crate.
#![cfg(test)]

#[path = "../📤️macro-exports/🦀️.rs"]
mod macro_export_tests;

#[path = "../🪪️mandatory-mutation-descriptor/🦀️.rs"]
mod mandatory_mutation_descriptor_tests;

#[path = "../🪆️record-owner/🦀️.rs"]
mod canonical_product_macro_tests;
