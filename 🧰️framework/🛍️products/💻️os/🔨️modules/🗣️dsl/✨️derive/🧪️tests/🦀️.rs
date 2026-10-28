//! 🌆️ In-crate laws of the DSL derive expansion: macro export roster, mutation source authority, leaf and aggregate derives, composite attributes and diff codecs.

use super::*;
use super::document_diff_and_composite_expansion::{diff_binary_tokens, diff_text_tokens, expand_composite_mutation, expand_derive_composite_mutation, expand_diff_binary, expand_diff_text, expand_dsl_document, parse_composite_attrs};
use super::mutation_leaf_attrs_and_schema_index::{MutationLeafAttrs, expand_mutation_leaf, mutation_leaf_payload_schema_path, mutation_leaf_portable_path, mutation_leaf_withdraw_only, mutation_leaf_workspace_token, mutation_schema_document_index, parse_mutation_leaf_attrs};
use super::mutations_expansion::{expand_derive_mutations, expand_mutations, parse_mutations_attrs};
use super::source_authority::{MUTATION_AUTHORITY_LOCATOR, MUTATION_AUTHORITY_SCHEMA, mutation_authority_aggregate_sources, mutation_authority_common, mutation_authority_domain_owners, mutation_authority_segment};

/// 🔍️ The derive resolves `$id` documents from the generated index and ignores schema files the index does not list.
#[test]
fn schema_document_index_reads_one_file_and_ignores_unlisted_schemas() {
    let parent = std::path::PathBuf::from(std::env::var("SEMIO_TEST_ARTIFACT_DIR").expect("ticket-owned schema index output required"));
    let workspace = parent.join(format!("dsl-schema-index-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&workspace);
    std::fs::create_dir_all(&workspace).unwrap();
    std::fs::write(workspace.join("package.json"), b"{\"workspaces\":[]}\n").unwrap();
    std::fs::write(workspace.join("📋️project.json"), b"{}\n").unwrap();
    let index_path = workspace.join(MUTATION_SCHEMA_INDEX_LOCATOR);
    std::fs::create_dir_all(index_path.parent().unwrap()).unwrap();
    let schema_dir = "🧬️schema";
    let kept = workspace.join("plugin").join(schema_dir).join("kept.json");
    let extra = workspace.join("plugin").join(schema_dir).join("extra.json");
    std::fs::create_dir_all(kept.parent().unwrap()).unwrap();
    std::fs::write(&kept, r#"{"$id":"urn:semio:schema-index:kept"}"#).unwrap();
    std::fs::write(&extra, r#"{"$id":"urn:semio:schema-index:extra"}"#).unwrap();
    let relative = format!("plugin/{schema_dir}/kept.json");
    std::fs::write(&index_path, format!(r#"{{"schema":"semio.dsl.mutation-schema-document-index/v1","roots":{{"plugin":{{"urn:semio:schema-index:kept":"{relative}"}}}}}}"#)).unwrap();
    let indexed = mutation_schema_document_index(&workspace.join("plugin")).unwrap();
    assert_eq!(indexed.keys().cloned().collect::<Vec<_>>(), vec!["urn:semio:schema-index:kept".to_string()]);
    assert_eq!(indexed["urn:semio:schema-index:kept"], kept);
    let collection = workspace.join("owner").join("🧫️fixtures");
    std::fs::create_dir_all(&collection).unwrap();
    assert!(mutation_schema_document_index(&collection).unwrap().is_empty());
    assert!(mutation_schema_document_index(&workspace.join("absent")).is_err());
    std::fs::remove_dir_all(&workspace).unwrap();
}

#[path = "🔬️mutation-source-authority/🦀️.rs"]
mod mutation_source_authority_tests;

#[path = "🔬️mutation-aggregate-source-authority/🦀️.rs"]
mod mutation_aggregate_source_authority_tests;

#[path = "🔬️mutation-leaf-json/🦀️.rs"]
mod mutation_leaf_json_tests;

#[path = "🔬️mutation-leaf-derive/🦀️.rs"]
mod mutation_leaf_derive_tests;

#[path = "🔬️mutation-attrs/🦀️.rs"]
mod mutation_attrs_tests;

#[path = "🔬️mandatory-mutations/🦀️.rs"]
mod mandatory_mutations_tests;

#[path = "🔬️composite-attrs/🦀️.rs"]
mod composite_attrs_tests;

#[path = "🔬️composite-timestamp/🦀️.rs"]
mod composite_timestamp_tests;

#[path = "🚪️diff-codecs/🦀️.rs"]
mod diff_codec_tests;

#[path = "❄️cold-dispatch/🦀️.rs"]
mod cold_dispatch_tests;
