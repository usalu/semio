//! 🏭️ Production mutation bridge for the outward Stdio composition.
//!
//! `test inventory` runs this and compares what it prints against each owner manifest and its claimed test
//! catalog. Every row is read out of a production aggregate's `DESCRIPTORS`, which the `dsl::Mutations` derive
//! generates from the mutation leaves themselves; a subset's inventory is the descriptors whose leaf `owner` lies
//! inside that subset's owner, so a verb dispatched in production and absent from the manifest is a breach.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🏭️inventory/📋️orchestration/🟦️.ts — the caller.

extern crate semio_framework_os_kernel as protocol;

use protocol::{Mutation, MutationLeafDescriptor};

/// 🧬️ One aggregate's descriptors, for the snapshot its `#[mutations(snapshot = …)]` names.
fn descriptors<S, M: Mutation<S>>() -> &'static [MutationLeafDescriptor] {
    M::DESCRIPTORS
}

/// 🧭️ Every mutation aggregate the artifact crates below mount, with its type name.
include!("../🤖️generated/🧩️mutations/🦀️.rs");

/// 🗺️ Manifest coordinate (artifact, standard, subset, state-lane surface or `""` for the document) → the owner
/// directory whose leaves it measures, and, where several subsets share one owner directory, the aggregate type-name
/// prefix that is that subset's dispatch.


/// 🎚️ The subset surface directories whose state lanes (config, presence, transient) are never document dispatch.
const SURFACE_DIRS: &[&str] = &["👁️viewer", "✏️editor"];

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (command, artifact, standard, subset, surface) = match args.as_slice() {
        [command, artifact, standard, subset] => (command, artifact, standard, subset, ""),
        [command, artifact, standard, subset, surface] => (command, artifact, standard, subset, surface.as_str()),
        _ => {
            eprintln!("usage: list-mutations <artifact> <standard> <subset> [<surface>]");
            std::process::exit(2);
        }
    };
    let Some((_, _, _, _, owner, prefix)) = COORDINATES.iter().find(|(a, s, u, f, _, _)| command == "list-mutations" && a == artifact && s == standard && u == subset && *f == surface) else {
        eprintln!("this bridge does not answer {command} {artifact} {standard} {subset} {surface}");
        std::process::exit(2);
    };
    let owner_prefix = format!("{owner}/");
    let in_state_lane = |leaf: &str| surface.is_empty() && leaf.split('/').any(|segment| SURFACE_DIRS.contains(&segment));
    let mut rows: Vec<semio_framework_pack_json::Value> = Vec::new();
    let mut seen: Vec<&str> = Vec::new();
    for descriptor in AGGREGATES.iter().filter(|(name, _)| name.starts_with(prefix)).flat_map(|(_, aggregate)| aggregate().iter()) {
        if !descriptor.owner.starts_with(&owner_prefix) || in_state_lane(&descriptor.owner[owner_prefix.len()..]) || seen.contains(&descriptor.semantic_kind) {
            continue;
        }
        seen.push(descriptor.semantic_kind);
        rows.push(semio_framework_pack_json::object([
            ("id".to_string(), semio_framework_pack_json::Value::from(descriptor.semantic_kind)),
            ("variant".to_string(), semio_framework_pack_json::Value::from(descriptor.aggregate_variant)),
            ("outcomes".to_string(), semio_framework_pack_json::array(descriptor.outcome_classes.iter().map(|class| semio_framework_pack_json::Value::from(class.as_str())))),
        ]));
    }
    let mut fields = vec![
        ("schema".to_string(), semio_framework_pack_json::Value::from("semio.repository-test.runtime-inventory/v2")),
        ("artifact".to_string(), semio_framework_pack_json::Value::from(artifact.as_str())),
        ("standard".to_string(), semio_framework_pack_json::Value::from(standard.as_str())),
        ("subset".to_string(), semio_framework_pack_json::Value::from(subset.as_str())),
    ];
    if !surface.is_empty() {
        fields.push(("surface".to_string(), semio_framework_pack_json::Value::from(surface)));
    }
    fields.extend([
        ("bridgeVersion".to_string(), semio_framework_pack_json::Value::from(1_i64)),
        ("producedBy".to_string(), semio_framework_pack_json::Value::from("semio-hub-stdio-mutation-bridge")),
        ("mutations".to_string(), semio_framework_pack_json::array(rows)),
    ]);
    let out = semio_framework_pack_json::object(fields);
    println!("{}", semio_framework_pack_json::to_string(&out));
}
