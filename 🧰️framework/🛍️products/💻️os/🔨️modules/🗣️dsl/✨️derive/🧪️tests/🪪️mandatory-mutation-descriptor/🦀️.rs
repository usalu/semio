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
    let neutral: NeutralFacts = serde_json::from_str(include_str!("🔣️.json")).expect("adjacent neutral vector must parse");
    let expected = neutral.cases.into_iter().find(|case| case.id == "required-items-accepted").expect("neutral vector must contain required-items-accepted");
    assert_eq!(expected.subject, "mutation-trait-facts");
    assert_eq!(expected.category, "required-associated-items");
    let actual = mutation_trait_facts(include_str!("../../../../../../../🔨️modules/📡️replication/🎮️mutation/🦀️.rs"));
    println!("[DEBUG] actual={} expected={}", serde_json::to_string(&actual).expect("actual facts serialize"), serde_json::to_string(&expected.facts).expect("expected facts serialize"));
    assert_eq!(actual, expected.facts);
}
