//! 🗿️ Artifact identity: the artifact state record and its conversions, the schema id, the dialect coordinate and the two artifact kinds.

use super::*;

//#region 🔖️Artifact
/// 🧬️ puzzle3d document artifact state.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.puzzle.puzzle3d")]
pub struct Puzzle3dArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub domain: String,
    #[state(artifact)]
    pub meta: Puzzle3dMeta,
    #[state(artifact)]
    pub objects: Vec<Puzzle3dObject>,
    #[state(artifact)]
    pub attractions: Vec<Puzzle3dAttraction>,
    #[state(artifact)]
    pub target_volumes: Vec<Puzzle3dTargetVolume>,
    #[state(artifact)]
    pub references: Vec<Puzzle3dReference>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for Puzzle3dArtifact {
    fn default() -> Self {
        Self::from_snapshot(Puzzle3dSnapshot::default())
    }
}

impl Puzzle3dArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> Puzzle3dSnapshot {
        Puzzle3dSnapshot {
            schema: self.schema.clone(),
            domain: self.domain.clone(),
            meta: self.meta.clone(),
            objects: self.objects.clone(),
            attractions: self.attractions.clone(),
            target_volumes: self.target_volumes.clone(),
            references: self.references.clone(),
        }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: Puzzle3dSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            domain: snapshot.domain,
            meta: snapshot.meta,
            objects: snapshot.objects,
            attractions: snapshot.attractions,
            target_volumes: snapshot.target_volumes,
            references: snapshot.references,
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: Puzzle3dSnapshot) {
        self.schema = snapshot.schema;
        self.domain = snapshot.domain;
        self.meta = snapshot.meta;
        self.objects = snapshot.objects;
        self.attractions = snapshot.attractions;
        self.target_volumes = snapshot.target_volumes;
        self.references = snapshot.references;
    }
}
//#endregion 🔖️Conversions

pub const PUZZLE_3D_SCHEMA: &str = "puzzle.3d";

/// 🎯️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET: the one `Dialect` coordinate every
/// surface (`✏️editor`, `👁️viewer`) of the `✳️any` subset binds `ArtifactEditor::DIALECT`/
/// `ArtifactViewer::DIALECT` to — `"s.puzzle.puzzle3d"` matches the artifact-kind id this subset's
/// own mutation/inference descriptors already key off (see `definition()`'s own
/// `"s.puzzle3d.schema.artifact"` row), standard `"1"` and subset `"*"` match this file's own
/// `🏅️standards/🔖️1/🪆️subsets/✳️any` location. Lives at the artifact level (not under the two
/// surfaces) so a viewer file can read it without ever importing through the sibling editor module.
pub const PUZZLE3D_DIALECT: semio_framework_artifact_reference::Dialect = semio_framework_artifact_reference::Dialect { artifact_kind: "s.puzzle.puzzle3d", standard: semio_framework_artifact_reference::StandardId("1"), subset: semio_framework_artifact_reference::SubsetId::ANY };

/// 🗿️ The `3d.puzzle` artifact kind — lifted out of the pre-consolidation manifest builder chain so
/// the artifact, not the app, owns its own identity.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    semio_framework_plugin::ArtifactKindSpec {
        id: "3d.puzzle".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("3D Puzzle", "3D-Puzzle"),
        source_format: "puzzle.3d".into(),
        component_kind: "puzzle3d".into(),
        dimension: "3d".into(),
        media_capability: semio_framework_plugin::OsMediaCapability::MeshOnly,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::ThreeD, form: semio_framework_plugin::MediaForm::Design },
        schema: PUZZLE_3D_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.json".into()],
        import_stdio_kinds: vec!["stdio.json".into()],
    }
}

/// 🔌️ The `kit.catalog` artifact kind puzzle3d's `kit:in` port CONSUMES — kept as a spec puzzle can
/// read, but no longer registered as a declaration.
///
/// 🐛️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (registrar, 2026-08-18): the old doc here called the
/// duplicate declaration "harmless if a producer, e.g. block3d, declares an identical spec". It was
/// not harmless — it was the last thing blocking `🎪️demonstrator`, which bundles six plugins and
/// therefore registered `kit.catalog` twice, and the definition registry correctly rejected it.
/// An artifact kind is a SCHEMA: N declarations are N sources of truth for one contract, and the
/// first divergence between them would be undetectable. Ownership resolved to `🧱️block`, the only
/// plugin that actually PRODUCES the kind (`catalog:out`, `KIT_CATALOG_ARTIFACT_ID` across its 3d
/// and 5d artifact trees); `🧩️puzzle` and `🪵️sourcing` are consumers and reference it by `kind_id`
/// on their ports, which is all a consumer ever needed. `🗄️stdio`'s own docstrings describe
/// absorbing this kind into the shared vocabulary later — that move stays open and is not
/// pre-empted here.
pub fn kit_catalog_artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    semio_framework_plugin::ArtifactKindSpec {
        id: "kit.catalog".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Kit Catalog", "Bausatzkatalog"),
        source_format: "kit.catalog".into(),
        component_kind: "kit".into(),
        dimension: "3d".into(),
        media_capability: semio_framework_plugin::OsMediaCapability::MeshOnly,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Kit, form: semio_framework_plugin::MediaForm::Type },
        schema: "kit.catalog".into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.dwg".into(), "stdio.gltf".into(), "stdio.json".into(), "stdio.las".into(), "stdio.obj".into(), "stdio.ply".into(), "stdio.png".into(), "stdio.stl".into()],
        import_stdio_kinds: vec!["stdio.dwg".into(), "stdio.gltf".into(), "stdio.json".into(), "stdio.las".into(), "stdio.obj".into(), "stdio.ply".into(), "stdio.png".into(), "stdio.stl".into()],
    }
}
