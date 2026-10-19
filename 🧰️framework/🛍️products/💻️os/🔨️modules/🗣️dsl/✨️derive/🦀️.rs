//! 🏭️ Product artifact envelopes, OS diff codecs, and source-authorized mutation derives.

use proc_macro::TokenStream;
use quote::quote;
use std::{collections::{BTreeMap, HashSet}, fs, path::{Component, Path, PathBuf}};
use syn::{Data, DeriveInput, Fields, Type, parse_macro_input};

#[path = "🔖️mutation-source-authority/🦀️.rs"]
pub(crate) mod source_authority;
use source_authority::{MUTATION_SCHEMA_INDEX_LOCATOR, MUTATION_SCHEMA_INDEX_SCHEMA, MutationAggregateSourceAuthority, MutationSourceAuthority, mutation_aggregate_source_authority, mutation_authority_locator, mutation_authority_no_follow, mutation_authority_relative, mutation_authority_workspace_root, mutation_source_authority};

#[path = "🧾️mutation-leaf-descriptor/🦀️.rs"]
pub(crate) mod mutation_leaf_descriptor;
use mutation_leaf_descriptor::{MutationLeafJson, emit_mutation_leaf_descriptor, mutation_leaf_kebab, parse_mutation_leaf_descriptor};

#[path = "🪪️mutation-leaf-attrs-and-schema-index/🦀️.rs"]
pub(crate) mod mutation_leaf_attrs_and_schema_index;
use mutation_leaf_attrs_and_schema_index::{mutation_authority_canonical, mutation_authority_workspace_token, mutation_leaf_include_path};

#[path = "🏗️document-diff-and-composite-expansion/🦀️.rs"]
pub(crate) mod document_diff_and_composite_expansion;
use document_diff_and_composite_expansion::to_kebab;

#[path = "🔁️mutations-expansion/🦀️.rs"]
pub(crate) mod mutations_expansion;


#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
