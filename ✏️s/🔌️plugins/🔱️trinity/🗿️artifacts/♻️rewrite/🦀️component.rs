//! ♻️ `trinity.rewrite.rule` artifact — document entities (constitutional: general).

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

use serde::{Deserialize, Serialize};

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

pub const REWRITE_RULE_SCHEMA: &str = "trinity.rewrite.rule";

/// 🎯️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET: the one `Dialect` coordinate every
/// surface (editor AND viewer) of this artifact shares — lives at the ARTIFACT level, not under
/// `editor`, so a viewer file can read it without ever importing through the sibling `editor` module.
/// `artifact_kind = "s.trinity.rewrite"` matches `#[artifact_schema(id = "s.trinity.rewrite")]` in
/// this subset's own `🧬️schema/🦀️component.rs`; `standard`/`subset` match this file's own
/// `🏅️standards/🔖️1/🪆️subsets/✳️any` location — the canonical surface id is
/// `s.trinity.rewrite@1/*#editor` / `s.trinity.rewrite@1/*#viewer` (contract §1 grammar). NOT to be
/// confused with the unrelated, pre-existing `const DIALECT` inside
/// `derived_analysis::RewriteAnalyzerAnalysis` in this subset's `🧬️schema/🦀️component.rs` — a
/// different trait (`ArtifactAnalysis`), a different string (`"s.rewrite"`), out of scope here.
pub const TRINITY_REWRITE_DIALECT: semio_framework_plugin::Dialect = semio_framework_plugin::Dialect { artifact_kind: "s.trinity.rewrite", standard: semio_framework_plugin::StandardId("1"), subset: semio_framework_plugin::SubsetId::ANY };
//#endregion 🔖️Types

// 📜️ `RewriteSnapshot`/`RewriteRuleMutation` derive their `store::ArtifactDsl`/`protocol::OpText`
// impls directly (see `#[derive(dsl::DslRecord)]` above and `#[derive(dsl::DslEnum)]` in `🔧️op`) —
// every field already binds through the `dsl::` engine with no foreign types, so no hand-written
// parser/printer or twin type is needed anywhere in this artifact (unlike `jack`'s `JackSnapshot`).

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — Text × Document per owner-table (`text.♻️rewrite`).
pub async fn artifact_kind() -> ArtifactKindSpec {
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
/// `pub` (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME, fleet-trinity-recipe): the new
/// declaration tree's `🪆️subsets/✳️any/🦀️component.rs` reads these same five `LanguageSpec`s to build
/// its `NativeCodecs` `LanguagePair`s (see that file's own doc for why it does not delegate to a
/// sibling `io::io()` the way `🗒️note`/`🖍️draw` do).
pub async fn pilot_languages() -> &'static [dsl::LanguageSpec] {
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

/// 🔖️ This artifact's OLD-channel definition (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1).
/// KEPT unread by the new declaration tree (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM,
/// debt D1 — deleted repo-wide only once every plugin has migrated, not this pass — `🗒️note`/`🖍️draw`
/// precedent, `📓️terra-fleet-trinity-recipe-report.md`): the real en/de localized names
/// (`"Rewrite"`/`"Umschreiben"`) still live only on these `ArtifactCapability` rows.
/// `crate::editor::rewrite::config::schema::register_app_schema()` is the one exception, kept alive
/// via the plugin root's own narrowed `.setup()`: it registers the `TrinityRewritePlayApp`
/// CONFIG/PRESENCE schema, an app-scope concern neither the old nor the new declaration type has a
/// field for.
pub async fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
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

/// 🌳️ This artifact's declaration tree root (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-
/// MECHANISM design.md §2, fleet-trinity-recipe) — replaces the old `declaration()`
/// (`ArtifactDeclaration::builder(...).schema(...).inferences(...).composers(...).languages(...)
/// .document_codec(...)` chain, deleted outright, no dual channel) as the ONLY registration channel
/// for schema/io/viewer/editor rows. `definition()` (old `ArtifactDefinition`/capability rows, above)
/// is kept per debt D1, and `artifact_kind()` is kept because this crate's own plugin-root
/// `.activation(...)` still reads `artifact_kind().id`; neither has any caller left in this function.
pub async fn artifact() -> semio_framework_plugin::app::declarations::ArtifactDeclaration {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use store::os_io::ArtifactKindId;
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.trinity.rewrite").expect("canonical rewrite kind"), localization: &[], standards: vec![crate::artifacts::rewrite::standards::v1::standard()] }
}
//#endregion 🔖️Register
