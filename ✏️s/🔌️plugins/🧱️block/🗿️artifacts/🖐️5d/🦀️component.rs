//! 👯️ Block 5D artifact — the document entity the 🖐️5d app edits (constitutional: general). Edits
//! exactly one `PartKind`: its identity, both 2d/3d presentations, its representations, and the
//! `GripKind` templates placed on it in both projections (keep each grip's 2d/3d halves as flat scalar
//! fields — see `s/plugin/puzzle/app/5d/dsl/rs/lib.rs:62` for the known pack table-column bug this
//! dodges).


pub use crate::artifacts::block5d::schema::snapshot::Block5dSnapshot;
pub use crate::artifacts::block5d::schema::mutations::Block5dMutation;
pub use crate::artifacts::block5d::schema::diff::Block5dDiff;

use crate::{BlockAttribute, BlockAuthor, BlockCamera2d, BlockCamera3d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta, BlockRepresentation};
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use serde::{Deserialize, Serialize};



pub const BLOCK_5D_SCHEMA: &str = "block.5d";

// #region 🔖️Document
/// 🔵️ The part's 2D-projection presentation (board node).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Block5dPart2d {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shape: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_kind: Option<String>,
}

/// 🧱️ The part's 3D-projection presentation (world object) — pose defaults only; the mesh itself
/// comes from `representations`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Block5dPart3d {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orientation: Option<[f64; 4]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<[f64; 3]>,
}

/// 🔘️ One grip-kind catalog row this part kind ships with.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Block5dGripKind {
    #[dsl(defines = "grip_kind")]
    pub id: String,
    pub name: String,
    pub label: String,
    pub color: String,
    pub default_rope_kind: String,
}

/// 🌱️ One rim-grip template, unified across both projections — flat scalar fields (no nested 2d/3d
/// sub-records) to dodge the pack table-column bug noted on `Block5dPart3d` above.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Block5dGripTemplate {
    pub id: String,
    #[dsl(refs = "grip_kind")]
    pub grip_kind: String,
    #[serde(default)]
    #[dsl(angle = "rad")]
    pub angle: f64,
    #[serde(default)]
    pub radius_2d: f64,
    #[serde(default)]
    #[dsl(coord)]
    pub position: [f64; 3],
    #[serde(default)]
    #[dsl(dir)]
    pub direction: [f64; 3],
    #[serde(default)]
    pub radius_3d: f64,
}

//#region 🔖️Snapshot
//#endregion 🔖️Snapshot

// #endregion 🔖️Document

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — the canonical `5d.block` declaration, stitched into
/// `crate::apps::block5d::create_block5d_app`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "5d.block".into(),
        name: "Part Kind".into(),
        source_format: BLOCK_5D_SCHEMA.into(),
        component_kind: "block5d".into(),
        dimension: "5d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
        schema: BLOCK_5D_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec!["stdio.json", "stdio.obj", "stdio.png", "stdio.stl", "stdio.zip"],
        import_stdio_kinds: vec!["stdio.json", "stdio.obj", "stdio.png", "stdio.stl", "stdio.zip"],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn artifact_kind_declares_the_5d_block_interchange_kind() {
        let kind = artifact_kind();
        assert_eq!(kind.id, "5d.block");
        assert_eq!(kind.schema, BLOCK_5D_SCHEMA);
        assert_eq!(kind.component_kind, "block5d");
    }
}
//#endregion 🧪️Tests
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
    use crate::artifacts::block5d::standards::v1::engine::io_registry as v1;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("Block5dComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v1::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry

//#region 🪪️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`, which called four different global registries directly from
/// a plugin `.setup()` callback. `Block5dPlayApp`'s CONFIG/PRESENCE schema — an app-scope concern
/// `ArtifactDeclaration` deliberately has no field for (see that struct's own doc) — now registers via
/// `ArtifactApp::app_schema()` returning `crate::apps::block5d::config::schema::app_schema_descriptor()`
/// (ticket W1c), so `.setup()` is gone from `🧱️block/🦀️component.rs` entirely.
pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration {
    semio_framework_plugin::ArtifactDeclaration::builder("s.block5d")
        .schema(crate::artifacts::block5d::schema::block5d_artifact_schema_descriptor())
        .inferences([crate::artifacts::block5d::standards::v1::subsets::any::schema::inferences::block5d_artifact_inference_descriptor()])
        .composers(crate::artifacts::block5d::standards::v1::engine::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<crate::apps::block5d::Block5dPlayApp>()
        .build()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring the
/// `OnceLock`-backed `io_registry::entries()` convention already used below.
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "block.block5d",
                    extension: Some("block5d"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::block5d::dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::block5d::dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::block5d::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::block5d::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("block.block5d"),
                },
                dsl::LanguageSpec {
                    id: "block.block5d.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::block5d::op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::block5d::op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::block5d::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::block5d::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("block.block5d.op"),
                },
                dsl::LanguageSpec {
                    id: "block.block5d.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::block5d::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::block5d::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("block.block5d.diff"),
                },
                dsl::LanguageSpec {
                    id: "5d.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::block5d::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::block5d::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("5d.pack"),
                },
                dsl::LanguageSpec {
                    id: "5d.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::block5d::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::block5d::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("5d.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🪪️Declaration
