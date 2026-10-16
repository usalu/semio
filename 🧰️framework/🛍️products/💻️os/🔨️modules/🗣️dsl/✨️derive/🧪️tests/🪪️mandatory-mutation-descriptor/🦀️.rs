//! 🧪️ Independent `syn` facts for the base mutation declaration contract.

use serde::{Deserialize, Serialize};
use syn::{Item, TraitItem};

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct MutationTraitFacts {
    descriptors_has_initializer: bool,
    descriptor_has_default_body: bool,
    sentinel_present: bool,
}

#[derive(Deserialize)]
struct NeutralFacts {
    cases: Vec<NeutralCase>,
}

#[derive(Deserialize)]
struct NeutralCase {
    id: String,
    subject: String,
    category: String,
    facts: MutationTraitFacts,
}

fn mutation_trait_facts(source: &str) -> MutationTraitFacts {
    let file = syn::parse_file(source).expect("actual mutation source must parse");
    let mutations: Vec<_> = file.items.iter().filter_map(|item| match item {
        Item::Trait(item) if matches!(&item.vis, syn::Visibility::Public(_)) && item.ident == "Mutation" => Some(item),
        _ => None,
    }).collect();
    assert_eq!(mutations.len(), 1, "actual source must contain exactly one public Mutation trait");
    let mutation = mutations[0];
    let descriptors: Vec<_> = mutation.items.iter().filter_map(|item| match item {
        TraitItem::Const(item) if item.ident == "DESCRIPTORS" => Some(item),
        _ => None,
    }).collect();
    assert_eq!(descriptors.len(), 1, "Mutation must contain exactly one DESCRIPTORS associated constant");
    let descriptors_has_initializer = descriptors[0].default.is_some();
    let descriptor: Vec<_> = mutation.items.iter().filter_map(|item| match item {
        TraitItem::Fn(item) if item.sig.ident == "descriptor" => Some(item),
        _ => None,
    }).collect();
    assert_eq!(descriptor.len(), 1, "Mutation must contain exactly one descriptor associated method");
    let descriptor_has_default_body = descriptor[0].default.is_some();
    let sentinels = file.items.iter().filter(|item| matches!(item, Item::Const(item) if item.ident == "UNDECLARED_MUTATION_LEAF")).count();
    MutationTraitFacts { descriptors_has_initializer, descriptor_has_default_body, sentinel_present: sentinels != 0 }
}

#[test]
fn mutation_trait_matches_adjacent_required_declaration_facts() {
    let neutral: NeutralFacts = serde_json::from_str(include_str!("../🧫️fixtures/🪪️mandatory-mutation-descriptor/🔣️.json")).expect("adjacent neutral vector must parse");
    let expected = neutral.cases.into_iter().find(|case| case.id == "required-items-accepted").expect("neutral vector must contain required-items-accepted");
    assert_eq!(expected.subject, "mutation-trait-facts");
    assert_eq!(expected.category, "required-associated-items");
    let actual = mutation_trait_facts(std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../../../🔨️modules/📡️replication/🎮️mutation/🦀️.rs")).expect("fixture").as_str());
    println!("actual={} expected={}", serde_json::to_string(&actual).expect("actual facts serialize"), serde_json::to_string(&expected.facts).expect("expected facts serialize"));
    assert_eq!(actual, expected.facts);
}

/// 🔍️ The derive resolves `$id` documents from the generated index and ignores schema files the index does not list.
#[test]
fn schema_document_index_reads_one_file_and_ignores_unlisted_schemas() {
    let parent = std::path::PathBuf::from(std::env::var("SEMIO_TEST_ARTIFACT_DIR").expect("ticket-owned schema index output required"));
    let workspace = parent.join(format!("dsl-schema-index-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&workspace);
    std::fs::create_dir_all(&workspace).unwrap();
    std::fs::write(workspace.join("package.json"), b"{\"workspaces\":[]}\n").unwrap();
    std::fs::write(workspace.join("📋️project.json"), b"{}\n").unwrap();
    let index_path = workspace.join(super::MUTATION_SCHEMA_INDEX_LOCATOR);
    std::fs::create_dir_all(index_path.parent().unwrap()).unwrap();
    let schema_dir = "🧬️schema";
    let kept = workspace.join("plugin").join(schema_dir).join("kept.json");
    let extra = workspace.join("plugin").join(schema_dir).join("extra.json");
    std::fs::create_dir_all(kept.parent().unwrap()).unwrap();
    std::fs::write(&kept, r#"{"$id":"urn:semio:schema-index:kept"}"#).unwrap();
    std::fs::write(&extra, r#"{"$id":"urn:semio:schema-index:extra"}"#).unwrap();
    let relative = format!("plugin/{schema_dir}/kept.json");
    std::fs::write(&index_path, format!(r#"{{"schema":"semio.dsl.mutation-schema-document-index/v1","roots":{{"plugin":{{"urn:semio:schema-index:kept":"{relative}"}}}}}}"#)).unwrap();
    let indexed = super::mutation_schema_document_index(&workspace.join("plugin")).unwrap();
    assert_eq!(indexed.keys().cloned().collect::<Vec<_>>(), vec!["urn:semio:schema-index:kept".to_string()]);
    assert_eq!(indexed["urn:semio:schema-index:kept"], kept);
    let collection = workspace.join("owner").join("🧫️fixtures");
    std::fs::create_dir_all(&collection).unwrap();
    assert!(super::mutation_schema_document_index(&collection).unwrap().is_empty());
    assert!(super::mutation_schema_document_index(&workspace.join("absent")).is_err());
    println!("[DEBUG] schema document index tracked one file and ignored the unlisted schema");
    std::fs::remove_dir_all(&workspace).unwrap();
}

