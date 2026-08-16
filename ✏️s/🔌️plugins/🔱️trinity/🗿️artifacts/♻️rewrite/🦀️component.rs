//! ♻️ `trinity.rewrite.rule` artifact — document entities (constitutional: general).

pub use crate::artifacts::rewrite::schema::diff::RewriteDiff;
pub use crate::artifacts::rewrite::schema::mutations::RewriteRuleMutation;

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

use crate::artifacts::jack::PropertyValue;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region ⚠️ Errors
/// ⚠️ Trinity rewrite-engine errors.
#[derive(Debug, thiserror::Error)]
pub enum TrinityRewriteError {
    /// 🧩️ Trinity graph fixture load/validation/mutation failure.
    #[error(transparent)]
    Graph(#[from] crate::artifacts::jack::TrinityRamError),
    /// 🧭️ VCS store/dispatch failure.
    #[error(transparent)]
    Vcs(#[from] vcs::VcsError),
    /// 🧬️ JSON (de)serialization failure.
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    /// 🔤️ Jack query parse/execute failure (the shared `🫀️core` jack-query kernel's own API is not
    /// yet thiserror-migrated).
    #[error("{0}")]
    Jack(String),
    /// 📐️ Force-directed layout failure (`infinite_board_port_directed`'s own API is not yet
    /// thiserror-migrated).
    #[error("{0}")]
    Layout(String),
    /// 🎨️ Canvas theme merge failure (`infinite_board_port_directed`'s own API is not yet
    /// thiserror-migrated).
    #[error("{0}")]
    CanvasTheme(String),
    #[error("force layout fixture missing nodes")]
    ForceLayoutFixtureMissingNodes,
}
//#endregion ⚠️ Errors

//#region 🔖️Types
/// 📍️ Local `{x, y}` twin for a bare `(f64, f64)` tuple — the DSL engine's `DslField` binding has no
/// impl for raw Rust tuples (only named `DslRecord`/`DslScalar` types can bind), so `rule_layout`'s
/// value type is this named record instead, with `From`/`Into` conversions at this crate's own
/// remaining `(f64, f64)` call sites (tests only — no production logic reads `rule_layout` today).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct LayoutPoint {
    pub x: f64,
    pub y: f64,
}

impl From<(f64, f64)> for LayoutPoint {
    fn from((x, y): (f64, f64)) -> Self {
        Self { x, y }
    }
}

impl From<LayoutPoint> for (f64, f64) {
    fn from(point: LayoutPoint) -> Self {
        (point.x, point.y)
    }
}

/// 📸️ Persisted rewrite snapshot — defined in `snapshot::schema`.
pub use super::snapshot::schema::RewriteSnapshot;

pub const REWRITE_RULE_SCHEMA: &str = "trinity.rewrite.rule";
//#endregion 🔖️Types

// 📜️ `RewriteSnapshot`/`RewriteRuleMutation` derive their `store::ArtifactDsl`/`protocol::OpText`
// impls directly (see `#[derive(dsl::DslRecord)]` above and `#[derive(dsl::DslEnum)]` in `🔧️op`) —
// every field already binds through the `dsl::` engine with no foreign types, so no hand-written
// parser/printer or twin type is needed anywhere in this artifact (unlike `jack`'s `JackSnapshot`).

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — Text × Document per owner-table (`text.♻️rewrite`).
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "text.♻️rewrite".into(),
        name: "Trinity Rewrite Rule".into(),
        source_format: REWRITE_RULE_SCHEMA.into(),
        component_kind: "trinity".into(),
        dimension: "text".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Text, form: MediaForm::Document },
        schema: REWRITE_RULE_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.docx", "stdio.json", "stdio.md", "stdio.pdf", "stdio.txt"],
        import_stdio_kinds: vec!["stdio.docx", "stdio.json", "stdio.md", "stdio.pdf", "stdio.txt"],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🔖️Register
/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring
/// `io_registry::entries()`'s own `OnceLock` convention.
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "rewrite.document",
                    extension: Some("rewrite"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::rewrite::dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::rewrite::dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::rewrite::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::rewrite::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("rewrite.document"),
                },
                dsl::LanguageSpec {
                    id: "rewrite.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::rewrite::op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::rewrite::op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::rewrite::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::rewrite::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("rewrite.op"),
                },
                dsl::LanguageSpec {
                    id: "rewrite.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::rewrite::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::rewrite::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("rewrite.diff"),
                },
                dsl::LanguageSpec {
                    id: "rewrite.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::rewrite::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::rewrite::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("rewrite.pack"),
                },
                dsl::LanguageSpec {
                    id: "rewrite.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::rewrite::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::rewrite::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("rewrite.spr"),
                },
            ]
        })
        .as_slice()
}

/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`, which called four different global registries directly from
/// a plugin `.setup()` callback. `crate::apps::rewrite::config::schema::register_app_schema()` is the
/// one exception, kept alive via the plugin root's own narrowed `.setup()`: it registers the
/// `TrinityRewritePlayApp` CONFIG/PRESENCE schema, an app-scope concern `ArtifactDeclaration`
/// deliberately has no field for (see that struct's own doc).
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    let rows: &[(&str, &str, &str, &[(&str, &str)], Option<(&str, &str)>)] = &[
        ("s.rewrite.standard.v1", "standard", "1", &[], None),
        ("s.rewrite.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.rewrite.schema.artifact", "schema", "s.trinity.rewrite", &[("schema", "s.trinity.rewrite")], None),
        ("s.rewrite.inference.artifact", "inference", "s.trinity.rewrite.inference", &[("schema", "s.trinity.rewrite.inference")], None),
        ("s.rewrite.composer.native", "composer", "s.rewrite@1/*", &[("dialect", "s.rewrite@1/*")], None),
        ("s.rewrite.composer.format-1", "composer", "s.stdio.txt@utf-8/*", &[("dialect", "s.stdio.txt@utf-8/*")], None),
        ("s.rewrite.composer.format-2", "composer", "s.stdio.pdf@1.4/*", &[("dialect", "s.stdio.pdf@1.4/*")], None),
        ("s.rewrite.composer.format-3", "composer", "s.stdio.docx@ecma-376/*", &[("dialect", "s.stdio.docx@ecma-376/*")], None),
        ("s.rewrite.composer.format-4", "composer", "s.stdio.md@commonmark/*", &[("dialect", "s.stdio.md@commonmark/*")], None),
        ("s.rewrite.composer.format-5", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.rewrite.grammar.1", "grammar", "rewrite.document", &[("grammar", "rewrite.document")], None),
        ("s.rewrite.grammar.2", "grammar", "rewrite.op", &[("grammar", "rewrite.op")], None),
        ("s.rewrite.grammar.3", "grammar", "rewrite.diff", &[("grammar", "rewrite.diff")], None),
        ("s.rewrite.grammar.4", "grammar", "rewrite.pack", &[("grammar", "rewrite.pack")], None),
        ("s.rewrite.grammar.5", "grammar", "rewrite.spr", &[("grammar", "rewrite.spr")], None),
        ("s.rewrite.codec.document-1", "codec", "trinity.rewrite.rule:rewrite", &[("codec", "trinity.rewrite.rule"), ("extension", "rewrite")], None),
        ("s.rewrite.localization.en", "localization", "Rewrite", &[], Some(("en", "Rewrite"))),
        ("s.rewrite.localization.de", "localization", "Umschreiben", &[], Some(("de", "Umschreiben"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.rewrite")?);
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

pub fn declaration() -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition()?)
        .schema(crate::artifacts::rewrite::schema::rewrite_artifact_schema_descriptor())
        .inferences([crate::artifacts::rewrite::standards::v1::subsets::any::schema::inferences::rewrite_artifact_inference_descriptor()])
        .composers(crate::artifacts::rewrite::standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<crate::apps::rewrite::TrinityRewritePlayApp>()
        .try_build()
}
//#endregion 🔖️Register

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::artifacts::rewrite::standards::v1::subsets::any::io::io_registry as v1;
    use semio_framework_plugin::{register_composer_entries, ComposeError, ComposedArtifact, ComposerEntry, Dialect, ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("RewriteComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v1::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
