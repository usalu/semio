//! 🩻️ Block 2D artifact — the document entity the ◻2d app edits (constitutional: general). Edits
//! exactly one `NodeKind`: its identity, rim presentation, and the `HandleKind` templates placed on
//! that rim.


pub use crate::artifacts::block2d::schema::snapshot::Block2dSnapshot;
pub use crate::artifacts::block2d::schema::mutations::Block2dMutation;
pub use crate::artifacts::block2d::schema::diff::Block2dDiff;

use crate::{BlockAttribute, BlockAuthor, BlockCamera2d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta};
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use serde::{Deserialize, Serialize};



pub const BLOCK_2D_SCHEMA: &str = "block.2d";

// #region 🔖️Document
/// 🔵️ The node's own rim presentation — mirrors `Puzzle2dNode`'s shape fields, minus placement (a
/// kind definition has no x/y — those belong to the puzzle assembly, not the definition).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Block2dPresentation {
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

/// 🔘️ One handle-kind catalog row this node kind ships with.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Block2dHandleKind {
    #[dsl(defines = "handle_kind")]
    pub id: String,
    pub name: String,
    pub label: String,
    pub color: String,
    pub default_wire_kind: String,
}

/// 🌱️ One rim-handle template — where a handle of `handle_kind` sits on the node's rim.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Block2dHandleTemplate {
    pub id: String,
    #[dsl(refs = "handle_kind")]
    pub handle_kind: String,
    #[dsl(angle = "rad")]
    pub angle: f64,
    pub radius: f64,
}

//#region 🔖️Snapshot
//#endregion 🔖️Snapshot

// #endregion 🔖️Document

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — the canonical `2d.block` declaration, stitched into
/// `crate::apps::block2d::create_block2d_app`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "2d.block".into(),
        name: "Node Kind".into(),
        source_format: BLOCK_2D_SCHEMA.into(),
        component_kind: "block2d".into(),
        dimension: "2d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
        schema: BLOCK_2D_SCHEMA.into(),
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
    fn artifact_kind_declares_the_2d_block_interchange_kind() {
        let kind = artifact_kind();
        assert_eq!(kind.id, "2d.block");
        assert_eq!(kind.schema, BLOCK_2D_SCHEMA);
        assert_eq!(kind.component_kind, "block2d");
    }
}
//#endregion 🧪️Tests
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
    use crate::artifacts::block2d::standards::v1::subsets::any::io::io_registry as v1;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("Block2dComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v1::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry

//#region 🪪️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `crate::apps::block2d::register()`, which called four different global
/// registries directly, and the plugin root's `.setup(crate::register_block_exports)` escape hatch
/// that invoked it. `crate::apps::block2d::config::schema::app_schema_descriptor()` is handed to
/// `ArtifactApp::app_schema` instead (ticket W1c), not declared here: an app-scope concern
/// `ArtifactDeclaration` deliberately has no field for (see that struct's own doc).
pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration {
    semio_framework_plugin::ArtifactDeclaration::builder("s.block2d")
        .schema(crate::artifacts::block2d::standards::v1::subsets::any::schema::block2d_artifact_schema_descriptor())
        .inferences([crate::artifacts::block2d::standards::v1::subsets::any::schema::inferences::block2d_artifact_inference_descriptor()])
        .composers(crate::artifacts::block2d::standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<crate::apps::block2d::Block2dPlayApp>()
        .build()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring the
/// `OnceLock`-backed `io_registry::entries()` convention already used above.
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "block.block2d",
                    extension: Some("block2d"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::block2d::dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::block2d::dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::block2d::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::block2d::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("block.block2d"),
                },
                dsl::LanguageSpec {
                    id: "block.block2d.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::block2d::op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::block2d::op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::block2d::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::block2d::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("block.block2d.op"),
                },
                dsl::LanguageSpec {
                    id: "block.block2d.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::block2d::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::block2d::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("block.block2d.diff"),
                },
                dsl::LanguageSpec {
                    id: "2d.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::block2d::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::block2d::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("2d.pack"),
                },
                dsl::LanguageSpec {
                    id: "2d.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::block2d::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::block2d::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("2d.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🪪️Declaration
