//! ⚙️ Block 2D artifact — headless compute over the `Block2dSnapshot` projection (constitutional:
//! engine).
//!
//! 🧭️ Placement rule for helpers: anything here takes ONLY document-side types (`Block2dSnapshot`/
//! its nested records). Helpers that also need the ◻2d app's view state (`crate::apps::block2d::config::
//! Block2dConfig`) stay at app level — an artifact must never depend on an app.

use crate::artifacts::block2d::{Block2dSnapshot, BLOCK_2D_SCHEMA};
use serde_json::{json, Value};

//#region 🔖️Register
/// 🗂️ Registers `Block2dSnapshot`'s pack↔dsl codec under `BLOCK_2D_SCHEMA`. Called from the plugin
/// root's `semio_plugin!{ setup: … }`.
pub fn register() {
    crate::artifacts::block2d::composer::register();

    register_pilot_languages();
    register_artifact_schema();
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<crate::apps::block2d::Block2dPlayApp>(BLOCK_2D_SCHEMA);
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "block.block2d",
        extension: Some("block2d"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::block2d::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::block2d::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::block2d::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::block2d::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("block.block2d"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "block.block2d.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::block2d::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::block2d::op::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::block2d::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::block2d::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("block.block2d.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "block.block2d.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::block2d::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::block2d::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("block.block2d.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "2d.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::block2d::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::block2d::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("2d.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "2d.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::block2d::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::block2d::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("2d.spr"),
    });
}

//#endregion 🔖️Register

//#region 🔖️DocumentHelpers
pub fn empty_block2d_snapshot() -> Block2dSnapshot {
    Block2dSnapshot::default()
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
/// 🌉️ Maps this `NodeKind` definition into the `s/plugin/puzzle` 2d manifest shape (`portKinds`/
/// `wireKinds`/`edgeKinds`/`nodeKinds`/`kindCompatibility` — see
/// `s/plugin/puzzle/app/2d/manifest/🛂️manifest.jsonconcrete-forest.manifest.json`), the seam puzzle imports through
/// its `Kit×Type` media port. Block owns no wire/edge-kind rows (`AGENTS.md`: referenced by
/// `default_wire_kind` only), so those arrays stay empty here — a merge keeps the puzzle manifest's
/// existing rows.
pub fn puzzle2d_manifest_fragment(definition: &Block2dSnapshot) -> Value {
    let port_kinds: Vec<Value> = definition.handle_kinds.iter().map(|kind| json!({ "id": kind.id, "name": kind.name, "presentation": { "color": kind.color, "defaultWireKind": kind.default_wire_kind } })).collect();
    let handles: Vec<Value> = definition.handles.iter().map(|handle| json!({ "handleKind": handle.handle_kind, "angle": handle.angle, "radius": handle.radius })).collect();
    let node_kind = json!({
        "id": definition.node_kind.id,
        "name": definition.node_kind.name,
        "presentation": {
            "meshUrl": Value::Null,
            "handles": handles,
        },
    });
    let kind_compatibility: Vec<Value> = definition.compatibility.iter().map(|rule| json!({ "bidirectional": rule.bidirectional, "specificity": "handle", "source": rule.source, "target": rule.target })).collect();
    json!({
        "schema": "manifest",
        "id": definition.node_kind.id,
        "name": definition.node_kind.name,
        "axes": { "portModel": "ported", "directedness": "directed" },
        "portKinds": port_kinds,
        "wireKinds": Vec::<Value>::new(),
        "edgeKinds": Vec::<Value>::new(),
        "nodeKinds": [node_kind],
        "kindCompatibility": kind_compatibility,
    })
}
//#endregion 🔖️PuzzleCatalogFragment

//#region 🔖️Io
/// 🔌️ `Block2dPlayApp`'s typed media I/O surface (`AppDefinition.io`) — the implicit document ports
/// (`Kit×Type`, matching the `"2d.block"` artifact kind) plus a `"catalog:out"` port giving
/// `puzzle2d_manifest_fragment` a real caller (see `crate::apps::block2d`'s `export_media`).
pub fn block2d_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo::from_document(
        BLOCK_2D_SCHEMA,
        semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Kit, form: semio_framework_plugin::MediaForm::Type },
        semio_framework_plugin::ArtifactPresentation { id: "2d.block".into(), name: "Node Kind".into(), dimension: "2d".into(), component_kind: "block2d".into() },
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
    use crate::artifacts::block2d::{Block2dHandleKind, Block2dHandleTemplate};
    use crate::BlockKindIdentity;

    #[test]
    fn empty_definition_matches_default() {
        assert_eq!(empty_block2d_snapshot(), Block2dSnapshot::default());
    }

    #[test]
    fn next_id_skips_existing() {
        let existing = ["h0", "h1"];
        assert_eq!(next_id(existing.into_iter(), "h"), "h2");
        assert_eq!(next_id(std::iter::empty(), "h"), "h0");
    }

    #[test]
    fn puzzle2d_manifest_fragment_maps_kind_identity_and_handles() {
        let mut definition = Block2dSnapshot { schema: BLOCK_2D_SCHEMA.into(), node_kind: BlockKindIdentity { id: "left".into(), name: "left".into(), label: "Left".into(), ..Default::default() }, ..Block2dSnapshot::default() };
        definition.handle_kinds.push(Block2dHandleKind { id: "b-l".into(), name: "b-l".into(), label: "b-l".into(), color: "hsl(206 52% 48%)".into(), default_wire_kind: "cable.link".into() });
        definition.handles.push(Block2dHandleTemplate { id: "h0".into(), handle_kind: "b-l".into(), angle: -1.57, radius: 0.36 });
        let fragment = puzzle2d_manifest_fragment(&definition);
        assert_eq!(fragment["nodeKinds"][0]["id"], "left");
        assert_eq!(fragment["nodeKinds"][0]["presentation"]["handles"][0]["handleKind"], "b-l");
        assert_eq!(fragment["portKinds"][0]["id"], "b-l");
    }

    #[test]
    fn block2d_io_declares_the_catalog_out_port() {
        let io = block2d_io();
        assert_eq!(io.document_schema, BLOCK_2D_SCHEMA);
        let ports = io.all_ports();
        let catalog = ports.iter().find(|port| port.id == "catalog:out").expect("catalog:out port declared");
        assert_eq!(catalog.kind_id.as_deref(), Some("kit.catalog"));
        assert_eq!(catalog.direction, semio_framework_plugin::MediaPortDirection::Out);
    }
}
//#endregion 🧪️Tests



//#region 🔖️ArtifactSchemaRegistry
/// 🧬️ Registers `block2d` fifteen-leaf artifact schema descriptor once.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::block2d::schema::block2d_artifact_schema_descriptor());
}
//#endregion 🔖️ArtifactSchemaRegistry

//#region 🔖️ArtifactEngine
/// ⚙️ UI-independent block2d artifact engine — owns the full artifact; `snapshot()` is its persisted subset.
pub struct Block2dEngine {
    artifact: crate::artifacts::block2d::schema::Block2dArtifact,
    snapshot: Block2dSnapshot,
}

impl Block2dEngine {
    pub fn new(snapshot: Block2dSnapshot) -> Self {
        let artifact = crate::artifacts::block2d::schema::Block2dArtifact::from_snapshot(snapshot.clone());
        Self { artifact, snapshot }
    }
}
//#endregion 🔖️ArtifactEngine
