//! 👯️ Block 5D artifact — the document entity the 🖐️5d app edits (constitutional: general). Edits
//! exactly one `PartKind`: its identity, both 2d/3d presentations, its representations, and the
//! `GripKind` templates placed on it in both projections (keep each grip's 2d/3d halves as flat scalar
//! fields — see `s/plugin/puzzle/app/5d/dsl/rs/lib.rs:62` for the known pack table-column bug this
//! dodges).


use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use serde::{Deserialize, Serialize};

pub const BLOCK_5D_SCHEMA: &str = "block.5d";

/// 🎯️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET: the one `Dialect` coordinate every
/// surface (`✏️editor`, `👁️viewer`) of the `✳️any` subset binds `ArtifactEditor::DIALECT`/
/// `ArtifactViewer::DIALECT` to — `"s.block.block5d"` matches this file's own `definition()`'s
/// `"s.block5d.schema.artifact"` row descriptor, standard `"1"` and subset `"*"` match this file's
/// own `🏅️standards/🔖️1/🪆️subsets/✳️any` location. Lives at the ARTIFACT level (not under
/// `editor`/`viewer`) so a viewer file can read it without ever importing through the sibling editor
/// module.
pub const BLOCK5D_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.block.block5d", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };

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
/// `crate::editor::block5d::create_block5d_app`.
pub async fn artifact_kind() -> ArtifactKindSpec {
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
    async fn artifact_kind_declares_the_5d_block_interchange_kind() {
        let kind = artifact_kind();
        assert_eq!(kind.id, "5d.block");
        assert_eq!(kind.schema, BLOCK_5D_SCHEMA);
        assert_eq!(kind.component_kind, "block5d");
    }
}
//#endregion 🧪️Tests
//#region 🪪️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`, which called four different global registries directly from
/// a plugin `.setup()` callback. `Block5dPlayApp`'s CONFIG/PRESENCE schema — an app-scope concern
/// `ArtifactDeclaration` deliberately has no field for (see that struct's own doc) — now registers via
/// `ArtifactEditor::app_schema()` returning `crate::editor::block5d::config::schema::app_schema_descriptor()`
/// (ticket W1c), so `.setup()` is gone from `🧱️block/🦀️component.rs` entirely.
pub async fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    let rows: &[(&str, &str, &str, &[(&str, &str)], Option<(&str, &str)>)] = &[
        ("s.block5d.standard.v1", "standard", "1", &[], None),
        ("s.block5d.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.block5d.schema.artifact", "schema", "s.block.block5d", &[("schema", "s.block.block5d")], None),
        ("s.block5d.inference.artifact", "inference", "s.block.block5d.inference", &[("schema", "s.block.block5d.inference")], None),
        ("s.block5d.composer.native", "composer", "s.block5d@1/*", &[("dialect", "s.block5d@1/*")], None),
        ("s.block5d.composer.format-1", "composer", "s.stdio.zip@2.0/*", &[("dialect", "s.stdio.zip@2.0/*")], None),
        ("s.block5d.composer.format-2", "composer", "s.stdio.png@1.2/*", &[("dialect", "s.stdio.png@1.2/*")], None),
        ("s.block5d.composer.format-3", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.block5d.composer.format-4", "composer", "s.stdio.stl@ascii/*", &[("dialect", "s.stdio.stl@ascii/*")], None),
        ("s.block5d.composer.format-5", "composer", "s.stdio.obj@3.0/*", &[("dialect", "s.stdio.obj@3.0/*")], None),
        ("s.block5d.grammar.1", "grammar", "block.block5d", &[("grammar", "block.block5d")], None),
        ("s.block5d.grammar.2", "grammar", "block.block5d.op", &[("grammar", "block.block5d.op")], None),
        ("s.block5d.grammar.3", "grammar", "block.block5d.diff", &[("grammar", "block.block5d.diff")], None),
        ("s.block5d.grammar.4", "grammar", "5d.pack", &[("grammar", "5d.pack")], None),
        ("s.block5d.grammar.5", "grammar", "5d.spr", &[("grammar", "5d.spr")], None),
        // 🐛️ D2-capability-claim-repairs: `.document_codec::<EditorApp<Block5dPlayApp>>()` derives
        // its extension claim from `<Block5dSnapshot as store::ArtifactDsl>::EXTENSION`
        // (`…/🧬️schema/📸️snapshot/🦀️component.rs`), which is `"block5d"`, not `"block"`.
        ("s.block5d.codec.document-1", "codec", "block.5d:block5d", &[("codec", "block.5d"), ("extension", "block5d")], None),
        ("s.block5d.localization.en", "localization", "5D Block", &[], Some(("en", "5D Block"))),
        ("s.block5d.localization.de", "localization", "5D-Baustein", &[], Some(("de", "5D-Baustein"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.block5d")?);
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

pub async fn declaration() -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition()?)
        .schema(crate::artifacts::block5d::schema::block5d_artifact_schema_descriptor())
        .inferences([crate::artifacts::block5d::standards::v1::subsets::any::schema::inferences::block5d_artifact_inference_descriptor()])
        .composers(crate::artifacts::block5d::standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<semio_framework_plugin::app::EditorApp<crate::editor::block5d::Block5dPlayApp>>()
        .try_build()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`.
async fn pilot_languages() -> &'static [dsl::LanguageSpec] {
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
