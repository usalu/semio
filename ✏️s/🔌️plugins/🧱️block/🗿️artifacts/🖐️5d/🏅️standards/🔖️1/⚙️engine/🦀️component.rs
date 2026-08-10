//! ⚙️ Block 5D artifact — headless compute over the `Block5dSnapshot` projection (constitutional:
//! engine).
//!
//! 🧭️ Placement rule for helpers: anything here takes ONLY document-side types
//! (`Block5dSnapshot`/…). Helpers that also need the 🖐️5d app's view state
//! (`crate::apps::block5d::config::Block5dConfig`) stay at app level — an artifact must never depend on
//! an app.

use crate::artifacts::block5d::{Block5dSnapshot, BLOCK_5D_SCHEMA};
use serde_json::{json, Value};

//#region 🔖️Register
/// 🗂️ Registers `Block5dSnapshot`'s pack↔dsl codec under `BLOCK_5D_SCHEMA`. Called from the plugin
/// root's `semio_plugin!{ setup: … }`.
pub fn register() {
    crate::artifacts::block5d::composer::register();

    register_pilot_languages();
    register_artifact_schema();
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<crate::apps::block5d::Block5dPlayApp>(BLOCK_5D_SCHEMA);
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "block.block5d",
        extension: Some("block5d"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::block5d::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::block5d::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::block5d::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::block5d::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("block.block5d"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "block.block5d.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::block5d::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::block5d::op::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::block5d::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::block5d::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("block.block5d.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "block.block5d.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::block5d::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::block5d::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("block.block5d.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "5d.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::block5d::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::block5d::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("5d.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "5d.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::block5d::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::block5d::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("5d.spr"),
    });
}

//#endregion 🔖️Register

//#region 🔖️DocumentHelpers
pub fn empty_block5d_snapshot() -> Block5dSnapshot {
    Block5dSnapshot::default()
}

/// 🪪️ Finds the smallest `"{prefix}{n}"` id not already present in `existing`.
pub fn next_id<'a>(existing: impl Iterator<Item = &'a str>, prefix: &str) -> String {
    let ids: std::collections::HashSet<&str> = existing.collect();
    let mut i = ids.len();
    loop {
        let candidate = format!("{prefix}{i}");
        if !ids.iter().any(|id| *id == candidate) {
            return candidate;
        }
        i += 1;
    }
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️PuzzleCatalogFragment
/// 🌉️ Maps this `PartKind` definition into the `s/plugin/puzzle` 5d catalog shape
/// (`Puzzle5dKindCatalogs`: `parts`/`grips`/`fasteners`/`ropes`), the seam puzzle imports through its
/// `Kit×Type` media port. Block owns no fastener/rope-kind rows, so those arrays stay empty here.
pub fn puzzle5d_catalog_fragment(definition: &Block5dSnapshot) -> Value {
    let grips: Vec<Value> = definition
        .grips
        .iter()
        .map(|grip| {
            json!({
                "gripKind": grip.grip_kind,
                "2d": { "angle": grip.angle, "gripKind": grip.grip_kind, "radius": grip.radius_2d },
                "3d": { "position": grip.position, "direction": grip.direction, "radius": grip.radius_3d },
            })
        })
        .collect();
    let mesh_url = definition.representations.first().and_then(|representation| representation.mesh_url.clone());
    let part = json!({
        "id": definition.part_kind.id,
        "name": definition.part_kind.name,
        "label": definition.part_kind.label,
        "meshUrl": mesh_url,
        "grips": grips,
    });
    let grip_kinds: Vec<Value> = definition.grip_kinds.iter().map(|kind| json!({ "id": kind.id, "name": kind.name, "label": kind.label, "color": kind.color, "defaultRopeKind": kind.default_rope_kind })).collect();
    json!({
        "schema": "manifest",
        "parts": [part],
        "grips": grip_kinds,
        "fasteners": Vec::<Value>::new(),
        "ropes": Vec::<Value>::new(),
        "kindCompatibility": definition.compatibility.iter().map(|rule| json!({ "source": rule.source, "target": rule.target, "bidirectional": rule.bidirectional })).collect::<Vec<_>>(),
    })
}
//#endregion 🔖️PuzzleCatalogFragment

//#region 🔖️Io
/// 🔌️ `Block5dPlayApp`'s typed media I/O surface (`AppDefinition.io`) — the implicit document ports
/// (`Kit×Type`, matching the `"5d.block"` artifact kind) plus a `"catalog:out"` port giving
/// `puzzle5d_catalog_fragment` a real caller (see `crate::apps::block5d`'s `export_media`).
pub fn block5d_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo::from_document(
        BLOCK_5D_SCHEMA,
        semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Kit, form: semio_framework_plugin::MediaForm::Type },
        semio_framework_plugin::ArtifactPresentation { id: "5d.block".into(), name: "Part Kind".into(), dimension: "5d".into(), component_kind: "block5d".into() },
    )
    .with_ports(vec![semio_framework_plugin::MediaPortSpec {
        id: "catalog:out".into(),
        label: "Kit Catalog".into(),
        direction: semio_framework_plugin::MediaPortDirection::Out,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Kit, form: semio_framework_plugin::MediaForm::Type },
        kind_id: Some("kit.catalog".into()),
        required: false,
        multiplicity: semio_framework_plugin::PortMultiplicity::Many,
    }])
}
//#endregion 🔖️Io

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::block5d::Block5dGripTemplate;
    use crate::BlockKindIdentity;

    #[test]
    fn empty_definition_matches_default() {
        assert_eq!(empty_block5d_snapshot(), Block5dSnapshot::default());
    }

    #[test]
    fn puzzle5d_catalog_fragment_maps_grips() {
        let mut definition = Block5dSnapshot { schema: BLOCK_5D_SCHEMA.into(), part_kind: BlockKindIdentity { id: "left".into(), name: "left".into(), label: "Left".into(), ..Default::default() }, ..Block5dSnapshot::default() };
        definition.grips.push(Block5dGripTemplate { id: "g0".into(), grip_kind: "b-l".into(), angle: -1.57, radius_2d: 0.36, position: [4.05, 4.68, 3.0], direction: [0.0, 1.0, 0.0], radius_3d: 0.36 });
        let fragment = puzzle5d_catalog_fragment(&definition);
        assert_eq!(fragment["parts"][0]["id"], "left");
        assert_eq!(fragment["parts"][0]["grips"][0]["gripKind"], "b-l");
    }

    #[test]
    fn block5d_io_declares_the_catalog_out_port() {
        let io = block5d_io();
        assert_eq!(io.document_schema, BLOCK_5D_SCHEMA);
        let ports = io.all_ports();
        let catalog = ports.iter().find(|port| port.id == "catalog:out").expect("catalog:out port declared");
        assert_eq!(catalog.kind_id.as_deref(), Some("kit.catalog"));
        assert_eq!(catalog.direction, semio_framework_plugin::MediaPortDirection::Out);
    }
}
//#endregion 🧪️Tests



//#region 🔖️ArtifactSchemaRegistry
/// 🧬️ Registers `block5d` fifteen-leaf artifact schema descriptor once.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::block5d::schema::block5d_artifact_schema_descriptor());
}
//#endregion 🔖️ArtifactSchemaRegistry

//#region 🔖️ArtifactEngine
/// ⚙️ UI-independent block5d artifact engine — owns the full artifact; `snapshot()` is its persisted subset.
pub struct Block5dEngine {
    artifact: crate::artifacts::block5d::schema::Block5dArtifact,
    snapshot: Block5dSnapshot,
}

impl Block5dEngine {
    pub fn new(snapshot: Block5dSnapshot) -> Self {
        let artifact = crate::artifacts::block5d::schema::Block5dArtifact::from_snapshot(snapshot.clone());
        Self { artifact, snapshot }
    }
}
//#endregion 🔖️ArtifactEngine
