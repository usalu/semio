//! 🩻️ Block 2D artifact — the document entity the ◻2d app edits (constitutional: general). Edits
//! exactly one `NodeKind`: its identity, rim presentation, and the `HandleKind` templates placed on
//! that rim.

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
/// 🎯️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET: the one `Dialect` coordinate every
/// surface (`✏️editor`, `👁️viewer`) of the `✳️any` subset binds `ArtifactEditor::DIALECT`/
/// `ArtifactViewer::DIALECT` to — `"s.block.block2d"` matches this file's own `definition()` row
/// (`"s.block2d.schema.artifact"`, descriptor `"s.block.block2d"`), standard `"1"` and subset `"*"`
/// match this file's own `🏅️standards/🔖️1/🪆️subsets/✳️any` location. Lives at the artifact level
/// (not under `editor`/`viewer`) so `policyViewerPurityBreaches` never sees a viewer file importing
/// through the sibling editor module just to read this constant.
pub const BLOCK2D_DIALECT: semio_framework_plugin::Dialect = semio_framework_plugin::Dialect { artifact_kind: "s.block.block2d", standard: semio_framework_plugin::StandardId("1"), subset: semio_framework_plugin::SubsetId::ANY };

/// 🗂️ This artifact's `ArtifactKindSpec` — the canonical `2d.block` declaration, stitched into
/// `crate::editor::block2d::create_block2d_app`.
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

    #[semio_framework_async_macros::async_test]
    async fn artifact_kind_declares_the_2d_block_interchange_kind() {
        let kind = artifact_kind();
        assert_eq!(kind.id, "2d.block");
        assert_eq!(kind.schema, BLOCK_2D_SCHEMA);
        assert_eq!(kind.component_kind, "block2d");
    }
}
//#endregion 🧪️Tests
//#region 🪪️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()` this app used to carry, which called four different global
/// registries directly, and the plugin root's `.setup(crate::register_block_exports)` escape hatch
/// that invoked it. `crate::editor::block2d::config::schema::app_schema_descriptor()` is handed to
/// `ArtifactEditor::app_schema` instead (ticket W1c), not declared here: an app-scope concern
/// `ArtifactDeclaration` deliberately has no field for (see that struct's own doc).
pub async fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    let rows: &[(&str, &str, &str, &[(&str, &str)], Option<(&str, &str)>)] = &[
        ("s.block2d.standard.v1", "standard", "1", &[], None),
        ("s.block2d.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.block2d.schema.artifact", "schema", "s.block.block2d", &[("schema", "s.block.block2d")], None),
        ("s.block2d.inference.artifact", "inference", "s.block.block2d.inference", &[("schema", "s.block.block2d.inference")], None),
        ("s.block2d.composer.native", "composer", "s.block2d@1/*", &[("dialect", "s.block2d@1/*")], None),
        ("s.block2d.composer.format-1", "composer", "s.stdio.zip@2.0/*", &[("dialect", "s.stdio.zip@2.0/*")], None),
        ("s.block2d.composer.format-2", "composer", "s.stdio.png@1.2/*", &[("dialect", "s.stdio.png@1.2/*")], None),
        ("s.block2d.composer.format-3", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.block2d.composer.format-4", "composer", "s.stdio.stl@ascii/*", &[("dialect", "s.stdio.stl@ascii/*")], None),
        ("s.block2d.composer.format-5", "composer", "s.stdio.obj@3.0/*", &[("dialect", "s.stdio.obj@3.0/*")], None),
        ("s.block2d.grammar.1", "grammar", "block.block2d", &[("grammar", "block.block2d")], None),
        ("s.block2d.grammar.2", "grammar", "block.block2d.op", &[("grammar", "block.block2d.op")], None),
        ("s.block2d.grammar.3", "grammar", "block.block2d.diff", &[("grammar", "block.block2d.diff")], None),
        ("s.block2d.grammar.4", "grammar", "2d.pack", &[("grammar", "2d.pack")], None),
        ("s.block2d.grammar.5", "grammar", "2d.spr", &[("grammar", "2d.spr")], None),
        // 🐛️ D2-capability-claim-repairs: `.document_codec::<EditorApp<Block2dPlayApp>>()` derives
        // its extension claim from `<Block2dSnapshot as store::ArtifactDsl>::EXTENSION`
        // (`…/🧬️schema/📸️snapshot/🦀️.rs`), which is `"block2d"`, not `"block"`.
        ("s.block2d.codec.document-1", "codec", "block.2d:block2d", &[("codec", "block.2d"), ("extension", "block2d")], None),
        ("s.block2d.localization.en", "localization", "2D Block", &[], Some(("en", "2D Block"))),
        ("s.block2d.localization.de", "localization", "2D-Baustein", &[], Some(("de", "2D-Baustein"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.block2d")?);
    for (identity, kind, descriptor, claims, localization) in rows {
        let mut capability = ArtifactCapability::new(ArtifactIdentity::parse(*identity)?, ArtifactCapabilityKind::parse(*kind)?).descriptor(descriptor.as_bytes())?;
        for (namespace, value) in *claims {
            capability = capability.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::parse(*namespace)?, *value)?)?;
        }
        if let Some((locale, text)) = localization {
            capability = capability.localization(ArtifactLocalization::new(ArtifactLocale::parse(*locale)?, *text)?)?;
        }
        definition = definition.capability(capability)?;
    }
    Ok(definition)
}

/// 🔖️ New declaration channel (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME,
/// `descriptor-prep`): replaces `declaration()`/`ArtifactDeclaration::builder(...).try_build()` — the
/// old channel's `require_declared_capability_or_record` runs an exact sorted-claims equality check
/// between `definition()`'s hand-authored capability rows and the runtime registration, which is why
/// `.artifact(block2d::declaration())` failed assembly. This channel never runs that check; the real
/// data (schema/inference descriptors, editor/viewer, native codec) is read from
/// `standards::v1::subsets::any::subset()` instead. Mirrors `🗒️note`/`🖍️draw`/`🔱️trinity`'s own
/// migration exactly.
pub fn artifact() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<crate::BlockApps> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use store::os_io::ArtifactKindId;
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.block.block2d").expect("canonical block2d kind"), localization: &[], standards: vec![crate::artifacts::block2d::standards::v1::standard()] }
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`. `pub` (was
/// private): the new `🪆️subsets/✳️any/🦀️.rs` reads it to build `io_declaration()`'s native
/// codec pairs, the same way trinity's own migration needed its `pilot_languages()` made `pub`.
pub fn pilot_languages() -> &'static [dsl::LanguageSpec] {
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
