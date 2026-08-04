//! 🏙️ Block 3D app — headless compute (constitutional: engine).

use block_3d::{Block3dDefinition, BLOCK_3D_SCHEMA};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

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

//#region 🔖️Config
/// 🧮️ `Block3dPlayApp`'s real `DocumentApp::Config` — B1 pure-trait conversion. Absorbs the former
/// `Block3dPlayApp` `RefCell` runtime fields (`selected_ids`/`active_representation_id`) plus the
/// locale this app now resolves itself (mirrors `shooting_engine::ShootingConfig::locale`). `wanted_tags`
/// is new — the tag filter `export_media("catalog:out", ..)` (see `🖱️ui`) is supposed to source
/// `puzzle3d_catalog_fragment`'s `wanted_tags` argument from; `DocumentApp::export_media`'s landed
/// signature (`🧰️framework/…/🔌️plugin`) doesn't thread `ConfigView` through yet (only `doc`), so
/// today's `export_media` always calls the fragment builder with an empty (all-tags) filter — this
/// field stays here, ready for whenever a later wave threads `cfg` into `export_media`, or for any
/// other in-process reader of the config record.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "block3dcfg")]
#[dsl(layout = "lines")]
pub struct Block3dConfig {
    /// 👁️ Multi-selected row ids in the document tree — was `Block3dPlayApp::selected_ids`.
    pub selected_ids: Vec<String>,
    /// 👁️ The representation shown in the inspector's representation select — was
    /// `Block3dPlayApp::active_representation_id`.
    pub active_representation_id: Option<String>,
    /// 🏷️ Tag filter for `puzzle3d_catalog_fragment`'s active-representation resolution — see the
    /// struct doc above for why `export_media` can't read it yet. Empty means "all tags".
    pub wanted_tags: Vec<String>,
    /// 🗣️ BCP-47 locale tag — was read off the deleted `ViewState.locale`.
    pub locale: String,
}

impl Default for Block3dConfig {
    fn default() -> Self {
        Self { selected_ids: Vec::new(), active_representation_id: None, wanted_tags: Vec::new(), locale: "en-US".into() }
    }
}

store::impl_whole_record_config!(Block3dConfig);

//#endregion 🔖️Config

//#region 🔖️Io
/// 🔌️ `Block3dPlayApp`'s typed media I/O surface (`AppDefinition.io`) — the implicit document ports
/// (`Kit×Type`, matching the `"3d.block"` artifact kind) plus the flagship `"catalog:out"` port this
/// ticket adds: the puzzle3d seam that finally gives `puzzle3d_catalog_fragment` a real caller (see
/// `🖱️ui`'s `Block3dPlayApp::export_media`).
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
    use block_3d::{Block3dVortexTemplate, BLOCK_3D_SCHEMA};
    use block_shared::{BlockKindIdentity, BlockRepresentation};

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
    fn block3d_config_default_has_no_selection_and_all_tags() {
        let config = Block3dConfig::default();
        assert!(config.selected_ids.is_empty());
        assert!(config.active_representation_id.is_none());
        assert!(config.wanted_tags.is_empty());
        assert_eq!(config.locale, "en-US");
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
