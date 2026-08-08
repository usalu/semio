//! ⚙️ Block 3D artifact — headless compute over the `Block3dDefinition` projection (constitutional:
//! engine).
//!
//! 🧭️ Placement rule for helpers: anything here takes ONLY document-side types (`Block3dDefinition`/
//! `BlockRepresentation`/…). Helpers that also need the 🧊️3d app's view state
//! (`crate::apps::block3d::config::Block3dConfig`/`Block3dWindowView`) stay at app level (see
//! `crate::apps::block3d::world`) — an artifact must never depend on an app.

use crate::artifacts::block3d::{Block3dDefinition, BLOCK_3D_SCHEMA};
use serde_json::{json, Value};

//#region 🔖️Register
/// 🗂️ Registers `Block3dDefinition`'s pack↔dsl codec under `BLOCK_3D_SCHEMA`. Called from the plugin
/// root's `semio_plugin!{ setup: … }`.
pub fn register() {
    register_pilot_languages();
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<crate::apps::block3d::Block3dPlayApp>(BLOCK_3D_SCHEMA);
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "block.block3d",
        extension: Some("block3d"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::block3d::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::block3d::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::block3d::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::block3d::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("block.block3d"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "block.block3d.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::block3d::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::block3d::op::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::block3d::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::block3d::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("block.block3d.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "block.block3d.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::block3d::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::block3d::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("block.block3d.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "3d.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::block3d::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::block3d::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("3d.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "3d.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::block3d::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::block3d::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("3d.spr"),
    });
}

//#endregion 🔖️Register

//#region 🔖️DocumentHelpers
pub fn empty_block3d_definition() -> Block3dDefinition {
    Block3dDefinition::default()
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

/// 🌐️ Resolves the active representation's mesh url — the first representation whose `tags` all
/// appear in `wanted_tags`, or the first representation overall, or `None` for an empty catalog.
pub fn resolve_active_mesh_url<'a>(definition: &'a Block3dDefinition, wanted_tags: &[&str]) -> Option<&'a str> {
    definition
        .representations
        .iter()
        .find(|representation| wanted_tags.iter().all(|tag| representation.tags.iter().any(|other| other == tag)))
        .or_else(|| definition.representations.first())
        .and_then(|representation| representation.mesh_url.as_deref())
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️PuzzleCatalogFragment
/// 🌉️ Maps this `ObjectKind` definition into the `s/plugin/puzzle` 3d catalog shape (`objectKinds`/
/// `vortexKinds`/`cableKinds`/`attractionKinds` — see `Puzzle3dKindCatalogs`), the seam puzzle imports
/// through its `Kit×Type` media port. The active representation's mesh (first row, or the first
/// matching `wanted_tags`) becomes the catalog row's `meshUrl`.
pub fn puzzle3d_catalog_fragment(definition: &Block3dDefinition, wanted_tags: &[&str]) -> Value {
    let vortices: Vec<Value> = definition.vortices.iter().map(|vortex| json!({ "id": vortex.id, "vortexKind": vortex.vortex_kind, "position": vortex.position, "direction": vortex.direction, "radius": vortex.radius })).collect();
    let object_kind = json!({
        "id": definition.object_kind.id,
        "name": definition.object_kind.name,
        "label": definition.object_kind.label,
        "meshUrl": resolve_active_mesh_url(definition, wanted_tags),
        "vortices": vortices,
    });
    let vortex_kinds: Vec<Value> = definition.vortex_kinds.iter().map(|kind| json!({ "id": kind.id, "name": kind.name, "label": kind.label, "color": kind.color, "defaultCableKind": kind.default_cable_kind })).collect();
    let kind_compatibility: Vec<Value> = definition.compatibility.iter().map(|rule| json!({ "source": rule.source, "target": rule.target, "bidirectional": rule.bidirectional })).collect();
    json!({
        "schema": "manifest",
        "objectKinds": [object_kind],
        "vortexKinds": vortex_kinds,
        "cableKinds": Vec::<Value>::new(),
        "attractionKinds": Vec::<Value>::new(),
        "kindCompatibility": kind_compatibility,
    })
}
//#endregion 🔖️PuzzleCatalogFragment

//#region 🔖️Io
/// 🔌️ `Block3dPlayApp`'s typed media I/O surface (`AppDefinition.io`) — the implicit document ports
/// (`Kit×Type`, matching the `"3d.block"` artifact kind) plus the `"catalog:out"` port: the puzzle3d
/// seam that gives `puzzle3d_catalog_fragment` a real caller (see `crate::apps::block3d`'s
/// `export_media`).
pub fn block3d_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo::from_document(
        BLOCK_3D_SCHEMA,
        semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Kit, form: semio_framework_plugin::MediaForm::Type },
        semio_framework_plugin::ArtifactPresentation { id: "3d.block".into(), name: "Object Kind".into(), dimension: "3d".into(), component_kind: "block3d".into() },
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
    use crate::artifacts::block3d::{Block3dVortexTemplate, BLOCK_3D_SCHEMA};
    use crate::{BlockKindIdentity, BlockRepresentation};

    #[test]
    fn empty_definition_matches_default() {
        assert_eq!(empty_block3d_definition(), Block3dDefinition::default());
    }

    #[test]
    fn resolve_active_mesh_url_prefers_matching_tags() {
        let mut definition = Block3dDefinition::default();
        definition.representations.push(BlockRepresentation { id: "r0".into(), name: "1:500".into(), mesh_url: Some("/mesh/low.glb".into()), tags: vec!["1to500".into()], lod: None, description: String::new(), attributes: Vec::new() });
        definition.representations.push(BlockRepresentation { id: "r1".into(), name: "full".into(), mesh_url: Some("/mesh/full.glb".into()), tags: vec!["full".into()], lod: None, description: String::new(), attributes: Vec::new() });
        assert_eq!(resolve_active_mesh_url(&definition, &["full"]), Some("/mesh/full.glb"));
        assert_eq!(resolve_active_mesh_url(&definition, &["missing"]), Some("/mesh/low.glb"));
    }

    #[test]
    fn puzzle3d_catalog_fragment_maps_vortices() {
        let mut definition = Block3dDefinition { schema: BLOCK_3D_SCHEMA.into(), object_kind: BlockKindIdentity { id: "capsule".into(), name: "capsule".into(), label: "Capsule".into(), ..Default::default() }, ..Block3dDefinition::default() };
        definition.vortices.push(Block3dVortexTemplate { id: "v0".into(), vortex_kind: "door".into(), position: [0.0, 0.0, 0.0], direction: [0.0, 1.0, 0.0], radius: 0.3, label: None });
        let fragment = puzzle3d_catalog_fragment(&definition, &[]);
        assert_eq!(fragment["objectKinds"][0]["id"], "capsule");
        assert_eq!(fragment["objectKinds"][0]["vortices"][0]["vortexKind"], "door");
    }

    #[test]
    fn block3d_io_declares_the_catalog_out_port() {
        let io = block3d_io();
        assert_eq!(io.document_schema, BLOCK_3D_SCHEMA);
        let ports = io.all_ports();
        assert!(ports.iter().any(|port| port.id == "document:in"));
        assert!(ports.iter().any(|port| port.id == "document:out"));
        let catalog = ports.iter().find(|port| port.id == "catalog:out").expect("catalog:out port declared");
        assert_eq!(catalog.kind_id.as_deref(), Some("kit.catalog"));
        assert_eq!(catalog.direction, semio_framework_plugin::MediaPortDirection::Out);
        assert_eq!(catalog.multiplicity, semio_framework_plugin::PortMultiplicity::Many);
    }
}
//#endregion 🧪️Tests


//#region 🔖️ArtifactEngine
pub struct Block3dEngine {
    projection: crate::artifacts::block3d::Block3dDefinition,
}

impl Block3dEngine {
    pub fn new(projection: crate::artifacts::block3d::Block3dDefinition) -> Self {
        Self { projection }
    }
}

impl protocol::ArtifactEngine for Block3dEngine {
    type Projection = crate::artifacts::block3d::Block3dDefinition;
    type Mutation = crate::artifacts::block3d::mutations::Block3dMutation;
    type Diff = crate::artifacts::block3d::diff::Block3dDiff;

    fn projection(&self) -> &Self::Projection {
        &self.projection
    }

    fn apply(&mut self, mutation: &Self::Mutation) -> Result<Self::Diff, protocol::EngineFault> {
        let diff = <Self::Mutation as protocol::Mutation<Self::Projection>>::diff(mutation, &self.projection);
        crate::artifacts::block3d::mutations::apply_block3d_mutation(&mut self.projection, mutation);
        Ok(diff)
    }

    fn inverse(&self, mutation: &Self::Mutation) -> Vec<Self::Mutation> {
        <Self::Mutation as protocol::Mutation<Self::Projection>>::inverse(mutation, &self.projection)
    }
}
//#endregion 🔖️ArtifactEngine
