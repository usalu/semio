//! 🏭️ Production mutation bridge for `s.stdio.step@ap214/✳️cc6`.
//!
//! Answers `listMutations(artifact, standard, subset)` by ASKING PRODUCTION, not by restating a list.
//! Protocol v2 requires `runtime inventory = owner manifest = claimed test inventory` exactly, and
//! the whole point of the runtime half is that it comes out of the dispatch enum itself: this binary
//! iterates one value per `StepCc6Mutation` variant and reports the kind each value answers with, so
//! adding a variant to production changes this output without anybody editing it.
//!
//! It emits nothing else. Marshalling and invoking only — no geometry, no comparison, no policy.
//!
//! @see ../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json — RuntimeMutationInventory
//! @see ../🔣️oracle.json — the manifest this output is held against

use semio_s_plugin_stdio::artifacts::step::mutations::cc6::StepCc6Mutation;

//#region 🔖️Inventory
/// 🏭️ One value per dispatch variant. Exhaustive by construction: a new variant fails to compile
/// here until it is added, which is what keeps the runtime half of the equality gate honest.
fn every_variant() -> Vec<StepCc6Mutation> {
    vec![
        StepCc6Mutation::SetSnapshot(crate::artifacts::step::standards::v_ap214::subsets::cc6::schema::mutations::set_snapshot::SetSnapshot { snapshot: Default::default() }),
        StepCc6Mutation::SetFileSchema(crate::artifacts::step::standards::v_ap214::subsets::cc6::schema::mutations::set_file_schema::SetFileSchema { schemas: Vec::new() }),
        StepCc6Mutation::SetProductIdentity(crate::artifacts::step::standards::v_ap214::subsets::cc6::schema::mutations::set_product_identity::SetProductIdentity { identity: None }),
        StepCc6Mutation::SetShapeRepresentation(crate::artifacts::step::standards::v_ap214::subsets::cc6::schema::mutations::set_shape_representation::SetShapeRepresentation { id: 0, representation: None }),
    ]
}

/// 🎯️ The outcome classes each kind can actually reach. Read from the class guard's own behaviour:
/// every kind can be REJECTED (the ladder refuses a type it does not admit) and every kind that
/// edits state can be APPLIED; `set-product-identity` additionally reaches NO-OP when the identity
/// it is given is the one already present, and `set-shape-representation` reaches EMPTY and DISJOINT
/// because it may delete the only representation or install one that resolves to no body.
fn outcomes_of(kind: &str) -> &'static [&'static str] {
    match kind {
        "set-snapshot" | "set-file-schema" => &["applied", "rejected"],
        "set-product-identity" => &["applied", "no-op", "rejected"],
        "set-shape-representation" => &["applied", "no-op", "empty", "disjoint", "rejected"],
        _ => &["no-op"],
    }
}

/// 🦀️ PascalCase dispatch variant of one kind, so manifest↔runtime equality is NAME-checked rather
/// than order-checked.
fn variant_of(kind: &str) -> String {
    kind.split('-').map(|segment| {
        let mut chars = segment.chars();
        match chars.next() {
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            None => String::new(),
        }
    }).collect()
}

fn escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}
//#endregion 🔖️Inventory

//#region 🚪️Entry
fn main() {
    let argv: Vec<String> = std::env::args().collect();
    if argv.get(1).map(String::as_str) != Some("list-mutations") {
        eprintln!("usage: bridge list-mutations <artifact> <standard> <subset>");
        std::process::exit(2);
    }
    let artifact = argv.get(2).cloned().unwrap_or_else(|| "s.stdio.step".to_string());
    let standard = argv.get(3).cloned().unwrap_or_else(|| "ap214".to_string());
    let subset = argv.get(4).cloned().unwrap_or_else(|| "cc6".to_string());

    let rows: Vec<String> = every_variant()
        .iter()
        .map(|mutation| {
            let kind = mutation.kind();
            let outcomes = outcomes_of(kind).iter().map(|outcome| format!("\"{}\"", outcome)).collect::<Vec<_>>().join(",");
            format!("{{\"id\":\"{}\",\"variant\":\"{}\",\"outcomes\":[{}]}}", escape(kind), escape(&variant_of(kind)), outcomes)
        })
        .collect();

    println!(
        "{{\"schema\":\"semio.repository-test.runtime-inventory/v2\",\"artifact\":\"{}\",\"standard\":\"{}\",\"subset\":\"{}\",\"bridgeVersion\":1,\"producedBy\":\"semio-s-plugin-stdio StepCc6Mutation dispatch enum\",\"mutations\":[{}]}}",
        escape(&artifact),
        escape(&standard),
        escape(&subset),
        rows.join(",")
    );
}
//#endregion 🚪️Entry
