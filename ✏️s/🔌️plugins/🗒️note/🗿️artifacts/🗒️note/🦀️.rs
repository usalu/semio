//! 📝️ Note artifact — the document entity this plugin's app edits: an infinite-canvas block tree
//! (text/image/table/math/ink/group blocks).

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<NoteMutation, NoteConfigMutation>, Fault>`, the exact signature `ArtifactApp::handle`
// and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing it
// here would diverge from the trait it must satisfy, and the lint does not fire on the trait impl itself
// (only on the free functions the taxonomy split creates), so this is a pure artefact of decomposition.
#[allow(clippy::result_large_err)]
extern crate self as semio_s_artifact_note_note;

use semio_framework_plugin::{ArtifactKindSpec, Dialect, MediaClass, MediaForm, MediaType, OsMediaCapability, StandardId, SubsetId};
use semio_s_artifact_stdio_semio::standards::v1::subsets::text::schema::snapshot::{SemioTextMark, SemioTextMarkKind, SemioTextRun, SemioTextSnapshot, STDIO_SEMIOTEXT_DOCUMENT_SCHEMA};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};
#[cfg(test)]
use std::collections::BTreeMap;

//#region 🔖️Register
/// 🔖️ This artifact's OLD-channel definition (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1).
/// KEPT unread by the new declaration tree (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM,
/// debt D1 — deleted repo-wide only once every plugin has migrated, not this pass): the real en/de
/// localized names (`"Note"`/`"Notiz"`) still live only on these `ArtifactCapability` rows, and
/// `crate::editor::note::config::schema::register_app_schema()` (still called from this file's own
/// `.setup()`) registers the `NotePlayApp` CONFIG/PRESENCE schema, an app-scope concern neither the
/// old nor the new declaration type has a field for. The `io_registry::entries()`/`NoteComposer`
/// machinery this comment block's own `"composer"` rows once cross-checked against is deleted
/// (`🚪️io/🦀️.rs`'s `io()` replaces it); the capability rows themselves are inert now, kept
/// only because nothing on this pass's boundary reads or removes `definition()`'s callers.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    let rows: &[semio_framework_plugin::ArtifactCapabilityRow<'_>] = &[
        ("s.note.note.standard.v1", "standard", "1", &[], None),
        ("s.note.note.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.note.note.schema.artifact", "schema", "s.note.note", &[("schema", "s.note.note")], None),
        ("s.note.note.inference.artifact", "inference", "s.note.note.inference", &[("schema", "s.note.note.inference")], None),
        // 🐛️ Pre-existing gap (found while verifying ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-
        // PLUGIN-RUNTIME E2-builder-descriptor's proof migration: `Plugin::builder("note")...
        // try_build()` was silently failing assembly — `PluginAssemblyError{code:"artifact-
        // definition.runtime-capability", message:"no declared composer capability owns the
        // runtime claims"}` — never surfaced before because nothing checked `try_build()`'s
        // `Result` against a real assertion). `io_registry::entries()` registers SEVEN composer
        // rows (`crate::…🚪️io/🦀️.rs`'s `entries()`), not six: the six
        // `EXPORT_*` rows below plus `composer_entry_of::<NoteAnyComposer>()`, whose `writes` is
        // this artifact's OWN dialect (`NoteComposer::DIALECT`, `s.note@1/*`) — the native "compose
        // my own snapshot from various format sources" composer. `PluginBuilder::declare(…)
        // .composers(entries)` requires a declared "composer" capability whose `dialect` claim
        // matches EVERY entry's `writes` coordinate; only the six STDIO-format rows existed, so the
        // native self-composer's claim never matched anything. Added here rather than removing the
        // native composer from `entries()`: the self-composer is real, load-bearing behaviour
        // (`rebuild_native_snapshot` is the shared decode path every `EXPORT_*` composer calls).
        ("s.note.note.composer.note", "composer", "s.note.note@1/*", &[("dialect", "s.note.note@1/*")], None),
        ("s.note.note.composer.svg", "composer", "s.stdio.svg@1.1/*", &[("dialect", "s.stdio.svg@1.1/*")], None),
        ("s.note.note.composer.pdf", "composer", "s.stdio.pdf@1.4/*", &[("dialect", "s.stdio.pdf@1.4/*")], None),
        ("s.note.note.composer.png", "composer", "s.stdio.png@1.2/*", &[("dialect", "s.stdio.png@1.2/*")], None),
        ("s.note.note.composer.json", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.note.note.composer.dwg", "composer", "s.stdio.dwg@ac1018/*", &[("dialect", "s.stdio.dwg@ac1018/*")], None),
        ("s.note.note.composer.dxf", "composer", "s.stdio.dxf@r12/*", &[("dialect", "s.stdio.dxf@r12/*")], None),
        ("s.note.note.grammar.document", "grammar", "note.document", &[("grammar", "note.document")], None),
        ("s.note.note.grammar.op", "grammar", "note.op", &[("grammar", "note.op")], None),
        ("s.note.note.grammar.diff", "grammar", "note.diff", &[("grammar", "note.diff")], None),
        ("s.note.note.grammar.pack", "grammar", "note.pack", &[("grammar", "note.pack")], None),
        ("s.note.note.grammar.spr", "grammar", "note.spr", &[("grammar", "note.spr")], None),
        ("s.note.note.codec.document.v1", "codec", "note.document:note", &[("codec", "note.document"), ("codec-extension", "13:note.document:note")], None),
        ("s.note.note.localization.en", "localization", "Note", &[], Some(("en", "Note"))),
        ("s.note.note.localization.de", "localization", "Notiz", &[], Some(("de", "Notiz"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.note.note")?);
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

/// 🗿️ New declaration tree (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §2)
/// — replaces the OLD `declaration()`/`pilot_languages()` pair outright (atomic cutover, no dual
/// registration channel). `localization: &[]` is a documented shortfall, not an oversight: the real
/// en/de localized names (`"Note"`/`"Notiz"`) still live on `definition()`'s kept
/// `ArtifactCapability` rows (debt D1) — wiring them into this field is real follow-up work, not
/// required for this pass (`📓️recipe-subset.md` §4c, matches the stdio pilot's identical deviation).
pub fn artifact() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<crate::NoteApps> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use store::os_io::ArtifactKindId;
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.note.note").expect("canonical note kind"), localization: &[], standards: vec![crate::standards::v1::standard()] }
}
//#endregion 🔖️Register

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::editor::note::create_note_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "2d.note".into(),
        name: "2D Note".into(),
        source_format: "note.document".into(),
        component_kind: "note".into(),
        dimension: "2d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Document },
        schema: "note.document".into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: crate::io::export_stdio_kinds().iter().map(|kind| (*kind).to_owned()).collect(),
        import_stdio_kinds: crate::io::import_stdio_kinds().iter().map(|kind| (*kind).to_owned()).collect(),
    }
}
//#endregion 🔖️ArtifactKind

//#region 🔖️Dialect
/// 👁️✏️ C1's canonical surface id grammar (`<artifact_kind>@<standard>/<subset>#<role>`) — lives at
/// the ARTIFACT level, not under `editor`/`viewer`, so a viewer file can read it without ever
/// importing through the sibling `editor` module. `artifact_kind` matches this file's own
/// `definition()` `"s.note.schema.artifact"` capability row's descriptor (`"s.note.note"`, the same
/// schema id `NoteSnapshot`'s `#[artifact_schema(id = "s.note.note")]` already keys off);
/// `standard`/`subset` match this file's own `🏅️standards/🔖️1/🪆️subsets/✳️any` location — i.e. the
/// canonical surface ids are `s.note.note@1/*#editor` / `s.note.note@1/*#viewer`.
pub const NOTE_DIALECT: Dialect = Dialect { artifact_kind: "s.note.note", standard: StandardId("1"), subset: SubsetId::ANY };
//#endregion 🔖️Dialect

//#region 🔖️Domain
pub const NOTE_DOCUMENT_SCHEMA: &str = "note.document";

/// 🎥️ Camera pose — ephemeral view state that lives in `crate::editor::note::config::NoteConfig`, never in
/// `NoteSnapshot`, so it stays out of undo history and off the operation channel.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct NoteCamera {
    #[serde(default)]
    #[value(default)]
    pub x: f64,
    #[serde(default)]
    #[value(default)]
    pub y: f64,
    #[serde(default = "default_zoom")]
    #[value(default = "default_zoom")]
    pub zoom: f64,
}

impl Default for NoteCamera {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

pub fn default_zoom() -> f64 {
    1.0
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslEnum)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum NoteBlockNode {
    #[value(rename = "text", rename_all = "camelCase")]
    Text {
        id: String,
        name: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        #[value(default)]
        rotation: f64,
        #[value(default = "default_true")]
        visible: bool,
        #[value(default)]
        locked: bool,
        /// ✏️ Composed content — ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`note→C:text`):
        /// replaces the former inline `paragraphs: Vec<NoteTextParagraph>` (which duplicated stdio's
        /// `s.stdio.semio.text` run/mark shape) with a content-addressed handle onto that composed
        /// subset. See this file's `🔖️TextBridge`/`🔖️TextChildren` regions for the
        /// converter and durable child-record accessor.
        content: NoteTextChild,
        font_size: f64,
        font_weight: String,
        align: String,
    },
    #[value(rename = "image", rename_all = "camelCase")]
    Image {
        id: String,
        name: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        #[value(default)]
        rotation: f64,
        #[value(default = "default_true")]
        visible: bool,
        #[value(default)]
        locked: bool,
        image_key: String,
    },
    #[value(rename = "table", rename_all = "camelCase")]
    Table {
        id: String,
        name: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        #[value(default)]
        rotation: f64,
        #[value(default = "default_true")]
        visible: bool,
        #[value(default)]
        locked: bool,
        columns: Vec<String>,
        rows: Vec<Vec<NoteTableCell>>,
    },
    #[value(rename = "math", rename_all = "camelCase")]
    Math {
        id: String,
        name: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        #[value(default)]
        rotation: f64,
        #[value(default = "default_true")]
        visible: bool,
        #[value(default)]
        locked: bool,
        #[dsl(lang = "tex")]
        tex: String,
        display_mode: bool,
    },
    #[value(rename = "stroke", rename_all = "camelCase")]
    #[dsl(key = "stroke")]
    Ink {
        id: String,
        name: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        #[value(default)]
        rotation: f64,
        #[value(default = "default_true")]
        visible: bool,
        #[value(default)]
        locked: bool,
        points: Vec<[f64; 2]>,
        stroke_width: f64,
        color: [f64; 4],
    },
    #[value(rename = "group", rename_all = "camelCase")]
    Group {
        id: String,
        name: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        #[value(default)]
        rotation: f64,
        #[value(default = "default_true")]
        visible: bool,
        #[value(default)]
        locked: bool,
        #[dsl(statements, block)]
        children: Vec<NoteBlockNode>,
    },
}

//#region 🔖️ComposedTypes
/// 🕸️ Snapshot-owned text child record. The handle preserves composition identity while the bounded
/// paragraph records are durable authority that survives reopen and worker migration.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct NoteTextChild {
    pub handle: store::ArtifactChild<SemioTextSnapshot>,
    #[value(default)]
    pub paragraphs: Vec<NoteTextParagraph>,
}
//#endregion 🔖️ComposedTypes

//#region 🔖️TextBridge
/// 🌉 REAL bidirectional converter between one `NoteBlockNode::Text`'s paragraph/run working
/// representation and the composed child's own `SemioTextSnapshot` (a flat run list — no paragraph
/// grouping). Paragraph boundaries are honestly lossy in one edge case, documented here rather than
/// silently dropped: a boundary is represented as a marks-free, language-free run whose `content` is
/// exactly `"\n"`; this collapses "zero paragraphs" and "one paragraph containing zero runs" onto the
/// same single empty paragraph on the way back (both are legitimate "no text yet" states,
/// indistinguishable once flattened — [`paragraphs_from_text_snapshot`] always emits a trailing
/// paragraph for whatever ran since the last separator, even an empty one). `NoteTextRun.underline`
/// has no equivalent mark in stdio's closed
/// bold/italic/code/link vocabulary and is dropped on the way in — real, honestly-lossy, not fabricated.
pub fn text_snapshot_from_paragraphs(paragraphs: &[NoteTextParagraph]) -> SemioTextSnapshot {
    let mut runs = Vec::new();
    for (index, paragraph) in paragraphs.iter().enumerate() {
        if index > 0 {
            runs.push(SemioTextRun { language: String::new(), content: "\n".into(), marks: Vec::new() });
        }
        for run in &paragraph.runs {
            let mut marks = Vec::new();
            if run.bold == Some(true) {
                marks.push(SemioTextMark { kind: SemioTextMarkKind::Bold, href: String::new() });
            }
            if run.italic == Some(true) {
                marks.push(SemioTextMark { kind: SemioTextMarkKind::Italic, href: String::new() });
            }
            if let Some(href) = &run.link {
                marks.push(SemioTextMark { kind: SemioTextMarkKind::Link, href: href.clone() });
            }
            runs.push(SemioTextRun { language: String::new(), content: run.text.clone(), marks });
        }
    }
    SemioTextSnapshot { schema: STDIO_SEMIOTEXT_DOCUMENT_SCHEMA.into(), runs }
}

/// 🌉 Inverse of [`text_snapshot_from_paragraphs`] — splits the flat run list back into paragraphs on
/// every marks-free/language-free `"\n"` separator run. See that function's doc comment for the one
/// honestly-lossy edge case (empty paragraph list vs. one paragraph with zero runs).
pub fn paragraphs_from_text_snapshot(snapshot: &SemioTextSnapshot) -> Vec<NoteTextParagraph> {
    let mut paragraphs = Vec::new();
    let mut current = Vec::new();
    for run in &snapshot.runs {
        if run.content == "\n" && run.marks.is_empty() && run.language.is_empty() {
            paragraphs.push(NoteTextParagraph { runs: std::mem::take(&mut current) });
            continue;
        }
        let bold = run.marks.iter().any(|mark| mark.kind == SemioTextMarkKind::Bold).then_some(true);
        let italic = run.marks.iter().any(|mark| mark.kind == SemioTextMarkKind::Italic).then_some(true);
        let link = run.marks.iter().find(|mark| mark.kind == SemioTextMarkKind::Link).map(|mark| mark.href.clone());
        current.push(NoteTextRun { text: run.content.clone(), bold, italic, underline: None, link });
    }
    paragraphs.push(NoteTextParagraph { runs: current });
    paragraphs
}

/// 🕸️ Deterministic content-addressed CHILD handle for one text block's composed content — same
/// `(child_id, target)` for identical `(block_id, paragraphs)`, a different pair once either changes;
/// mirrors writer's `document_child_handle`/cad's `cad_model_child_handle`, keyed by `block_id` (not
/// content alone) so two distinct blocks never collide on the same child slot.
pub fn note_text_child_handle(block_id: &str, paragraphs: &[NoteTextParagraph]) -> NoteTextChild {
    use std::hash::{Hash, Hasher};
    let content_json = serde_json::to_string(paragraphs).unwrap_or_default();
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    block_id.hash(&mut hasher);
    content_json.hash(&mut hasher);
    let content_hash = hasher.finish();
    let child_id = format!("note-text-{content_hash:016x}");
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "text".into() };
    let target = store::os_io::ArtifactRef { artifact_id: format!("{block_id}-text"), dialect };
    NoteTextChild { handle: store::ArtifactChild::new(child_id, target), paragraphs: paragraphs.to_vec() }
}
//#endregion 🔖️TextBridge

//#region 🔖️TextChildren
/// 🔎 Reads the durable paragraphs owned by the text-child record.
pub fn note_block_text(handle: &NoteTextChild) -> Vec<NoteTextParagraph> {
    handle.paragraphs.clone()
}

/// 🏗️ Mints a new content-addressed handle and stores its paragraphs in the same snapshot-owned
/// record used by mutation-diff, fixture, and converter builders.
pub fn note_text_child_record(block_id: &str, paragraphs: &[NoteTextParagraph]) -> NoteTextChild {
    note_text_child_handle(block_id, paragraphs)
}
//#endregion 🔖️TextChildren

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "r")]
pub struct NoteTextRun {
    #[dsl(positional)]
    pub text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub underline: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "p")]
pub struct NoteTextParagraph {
    pub runs: Vec<NoteTextRun>,
}

pub fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord)]
pub struct NoteTableCell {
    #[dsl(positional)]
    pub content: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct NoteImageAsset {
    pub mime: String,
    pub data: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
}

pub use crate::schema::diff::NoteDiff;
pub use crate::schema::mutations::NoteMutation;
pub use crate::schema::snapshot::NoteSnapshot;

//#endregion 🔖️Domain

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 🗂️ The manifest-facing `ArtifactKindSpec.schema` and `NOTE_DOCUMENT_SCHEMA` are deliberately the
    /// same string here (note has no separate "fixture" store schema, unlike shooting) — pinned so a
    /// future edit can't silently diverge them without noticing.
    #[semio_framework_async_macros::async_test]
    async fn artifact_kind_schema_matches_the_store_schema() {
        assert_eq!(artifact_kind().schema, NOTE_DOCUMENT_SCHEMA);
    }

    //#region 🔖️TextBridgeTests
    /// 🧪️ Real round trip for the paragraph <-> `SemioTextSnapshot` converter: multiple paragraphs,
    /// multiple runs, every mark (bold/italic/link) the converter maps.
    #[semio_framework_async_macros::async_test]
    async fn text_bridge_round_trips_paragraphs_through_semio_text_snapshot() {
        let paragraphs = vec![
            NoteTextParagraph { runs: vec![NoteTextRun { text: "plain ".into(), bold: None, italic: None, underline: None, link: None }, NoteTextRun { text: "bold".into(), bold: Some(true), italic: None, underline: None, link: None }] },
            NoteTextParagraph { runs: vec![NoteTextRun { text: "second para".into(), bold: None, italic: Some(true), underline: None, link: Some("https://semio.tech".into()) }] },
        ];
        let snapshot = text_snapshot_from_paragraphs(&paragraphs);
        assert_eq!(snapshot.runs.len(), 4, "2 content runs + 1 paragraph separator, but bold/link split across marks not runs: {snapshot:?}");
        let restored = paragraphs_from_text_snapshot(&snapshot);
        assert_eq!(restored, paragraphs);
    }

    /// 🧪️ Documents the one honest lossy edge case: an empty paragraph list and a single paragraph
    /// with zero runs both flatten to zero runs, and [`paragraphs_from_text_snapshot`] always emits a
    /// trailing paragraph for whatever ran since the last separator (even if that's none) — so both
    /// restore as the SAME single empty paragraph, never as an empty paragraph list.
    #[semio_framework_async_macros::async_test]
    async fn text_bridge_collapses_empty_paragraph_shapes() {
        let one_empty_paragraph = vec![NoteTextParagraph { runs: Vec::new() }];
        assert_eq!(paragraphs_from_text_snapshot(&text_snapshot_from_paragraphs(&[])), one_empty_paragraph);
        assert_eq!(paragraphs_from_text_snapshot(&text_snapshot_from_paragraphs(&one_empty_paragraph)), one_empty_paragraph);
    }

    /// 🧪️ `underline` has no equivalent mark in stdio's text subset and is honestly dropped, never
    /// fabricated back on the way out.
    #[semio_framework_async_macros::async_test]
    async fn text_bridge_drops_underline_honestly() {
        let paragraphs = vec![NoteTextParagraph { runs: vec![NoteTextRun { text: "u".into(), bold: None, italic: None, underline: Some(true), link: None }] }];
        let restored = paragraphs_from_text_snapshot(&text_snapshot_from_paragraphs(&paragraphs));
        assert_eq!(restored[0].runs[0].underline, None);
    }

    /// 🧪️ Each minted text-child record owns its paragraphs, and two distinct block ids never
    /// collide even with identical paragraph content.
    #[semio_framework_async_macros::async_test]
    async fn text_child_records_are_owned_and_block_ids_never_collide() {
        let paragraphs = vec![NoteTextParagraph { runs: vec![NoteTextRun { text: "hi".into(), bold: None, italic: None, underline: None, link: None }] }];
        let a = note_text_child_record("block-a", &paragraphs);
        let b = note_text_child_record("block-b", &paragraphs);
        assert_ne!(a.handle.child_id, b.handle.child_id, "identical content on distinct block ids must not share a child slot");
        assert_eq!(note_block_text(&a), paragraphs);
        assert_eq!(note_block_text(&b), paragraphs);
    }

    /// 🧪️ An explicitly empty durable child record reads back an empty paragraph list.
    #[semio_framework_async_macros::async_test]
    async fn note_block_text_reads_an_empty_owned_record() {
        let handle = note_text_child_handle("empty-owned-record", &[]);
        assert_eq!(note_block_text(&handle), Vec::<NoteTextParagraph>::new());
    }
    //#endregion 🔖️TextBridgeTests

    #[semio_framework_async_macros::async_test]
    async fn note_document_round_trips_assets_and_grid_settings() {
        let mut document = NoteSnapshot {
            schema: NOTE_DOCUMENT_SCHEMA.into(),
            id: "empty".into(),
            title: None,
            blocks: Vec::new(),
            grid_visible: Some(true),
            grid_spacing: Some(32.0),
            grid_subdivisions: Some(4.0),
            grid_opacity: Some(0.35),
            snap_enabled: Some(false),
            snap_grid_spacing: Some(8.0),
            pencil_width: Some(3.0),
            eraser_radius: Some(12.0),
            assets: BTreeMap::new(),
            linked_artifact: None,
        };
        document.assets.insert("asset-1".into(), NoteImageAsset { mime: "image/png".into(), data: "data:image/png;base64,abc".into(), width: Some(10.0), height: Some(20.0) });
        document.grid_subdivisions = Some(6.0);
        document.grid_opacity = Some(0.5);
        let json_text = dsl::os_pack::to_json_string(&document);
        let parsed: NoteSnapshot = dsl::os_pack::from_json_str(&json_text).unwrap();
        assert_eq!(parsed.assets.get("asset-1").unwrap().mime, "image/png");
        assert_eq!(parsed.grid_subdivisions, Some(6.0));
        assert_eq!(parsed.grid_opacity, Some(0.5));
    }
}
//#endregion 🧪️Tests

/// 🖼️ Encodes persisted Note fields and the local camera for an ink-canvas scene.
pub fn note_canvas_document_json(document: &NoteSnapshot, camera: &NoteCamera) -> String {
    let mut value = dsl::os_pack::json_from_dsl_value(&dsl::ToValue::to_value(document));
    value.as_object_mut().expect("NoteSnapshot is a record").insert("camera", dsl::os_pack::json_from_dsl_value(&dsl::ToValue::to_value(camera)));
    dsl::os_pack::json_to_string(&value)
}

#[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "."]
                                pub mod outline {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧾outline/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod svg {
                                            #[path = "."]
                                            pub mod v1_1 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod pdf {
                                            #[path = "."]
                                            pub mod v1_4 {
                                                #[path = "."]
                                                pub mod base {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📖️pdf/🔖️1.4/🧱️base/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod png {
                                            #[path = "."]
                                            pub mod v1_2 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod json {
                                            #[path = "."]
                                            pub mod v_rfc8259 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod dwg {
                                            #[path = "."]
                                            pub mod v_ac1018 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod dxf {
                                            #[path = "."]
                                            pub mod v_r12 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📐️dxf/🔖️r12/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod svg {
                                            #[path = "."]
                                            pub mod v1_1 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod pdf {
                                            #[path = "."]
                                            pub mod v1_4 {
                                                #[path = "."]
                                                pub mod base {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.4/🧱️base/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod png {
                                            #[path = "."]
                                            pub mod v1_2 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod json {
                                            #[path = "."]
                                            pub mod v_rfc8259 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod dwg {
                                            #[path = "."]
                                            pub mod v_ac1018 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod dxf {
                                            #[path = "."]
                                            pub mod v_r12 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📐️dxf/🔖️r12/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        #[path = "."]
                        pub mod examples {
                            #[path = "."]
                            pub mod demo {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                        }
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod ink {
                        #[path = "."]
                        pub mod schema {
                            #[path = "."]
                            pub mod mutations {
                                #[path = "."]
                                pub mod change_pencil_width {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🖋️ink/🧬️schema/🧬️mutations/✏️change-pencil-width/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🖋️ink/🧬️schema/🧬️mutations/✏️change-pencil-width/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🖋️ink/🧬️schema/🧬️mutations/✏️change-pencil-width/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🖋️ink/🧬️schema/🧬️mutations/✏️change-pencil-width/🧪️tests/✏️thickens-pencil/🦀️.rs"]
                                    mod tests_thickens_pencil;
                                }
                                #[path = "."]
                                pub mod change_eraser_radius {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🖋️ink/🧬️schema/🧬️mutations/🧽️change-eraser-radius/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🖋️ink/🧬️schema/🧬️mutations/🧽️change-eraser-radius/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🖋️ink/🧬️schema/🧬️mutations/🧽️change-eraser-radius/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🖋️ink/🧬️schema/🧬️mutations/🧽️change-eraser-radius/🧪️tests/🧽️enlarges-eraser/🦀️.rs"]
                                    mod tests_enlarges_eraser;
                                }
                                #[path = "."]
                                pub mod change_block_ink_width {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🖋️ink/🧬️schema/🧬️mutations/🖊️change-block-ink-width/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🖋️ink/🧬️schema/🧬️mutations/🖊️change-block-ink-width/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🖋️ink/🧬️schema/🧬️mutations/🖊️change-block-ink-width/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🖋️ink/🧬️schema/🧬️mutations/🖊️change-block-ink-width/🧪️tests/🖊️thickens-the-0f080f/🦀️.rs"]
                                    mod tests_thickens_the_sketch_stroke;
                                }
                                #[path = "."]
                                pub mod edit_block_ink_stroke {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🖋️ink/🧬️schema/🧬️mutations/🎨️edit-block-ink-stroke/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🖋️ink/🧬️schema/🧬️mutations/🎨️edit-block-ink-stroke/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🖋️ink/🧬️schema/🧬️mutations/🎨️edit-block-ink-stroke/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🖋️ink/🧬️schema/🧬️mutations/🎨️edit-block-ink-stroke/🧪️tests/🎨️redraws-the-1f3ccf/🦀️.rs"]
                                    mod tests_redraws_the_sketch_polyline;
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod schema {
                            #[path = "."]
                            pub mod mutations {
                                #[path = "."]
                                pub mod edit_block_text {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/📝️text/🧬️schema/🧬️mutations/📝️edit-block-text/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/📝️text/🧬️schema/🧬️mutations/📝️edit-block-text/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/📝️text/🧬️schema/🧬️mutations/📝️edit-block-text/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/📝️text/🧬️schema/🧬️mutations/📝️edit-block-text/🧪️tests/📝️replaces-the-431dc0/🦀️.rs"]
                                    mod tests_replaces_the_intro_paragraphs;
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod math {
                        #[path = "."]
                        pub mod schema {
                            #[path = "."]
                            pub mod mutations {
                                #[path = "."]
                                pub mod edit_block_math {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧮️math/🧬️schema/🧬️mutations/🧮️edit-block-math/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧮️math/🧬️schema/🧬️mutations/🧮️edit-block-math/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧮️math/🧬️schema/🧬️mutations/🧮️edit-block-math/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧮️math/🧬️schema/🧬️mutations/🧮️edit-block-math/🧪️tests/📐️replaces-the-tex-d63e6c/🦀️.rs"]
                                    mod tests_replaces_the_tex_with_pythagoras;
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod table {
                        #[path = "."]
                        pub mod schema {
                            #[path = "."]
                            pub mod mutations {
                                #[path = "."]
                                pub mod insert_table_row {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/⬇️insert-table-row/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/⬇️insert-table-row/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/⬇️insert-table-row/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/⬇️insert-table-row/🧪️tests/⬇️appends-a-blank-third-row/🦀️.rs"]
                                    mod tests_appends_a_blank_third_row;
                                }
                                #[path = "."]
                                pub mod remove_table_row {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/⬆️remove-table-row/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/⬆️remove-table-row/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/⬆️remove-table-row/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/⬆️remove-table-row/🧪️tests/⬆️drops-the-2d8001/🦀️.rs"]
                                    mod tests_drops_the_trailing_blank_row;
                                }
                                #[path = "."]
                                pub mod insert_table_column {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/➡️insert-table-column/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/➡️insert-table-column/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/➡️insert-table-column/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/➡️insert-table-column/🧪️tests/➡️appends-the-dfe3a4/🦀️.rs"]
                                    mod tests_appends_the_lettered_column_c;
                                }
                                #[path = "."]
                                pub mod remove_table_column {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/⬅️remove-table-column/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/⬅️remove-table-column/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/⬅️remove-table-column/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/⬅️remove-table-column/🧪️tests/⬅️drops-the-a7e80e/🦀️.rs"]
                                    mod tests_drops_the_trailing_column_b;
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod asset {
                        #[path = "."]
                        pub mod schema {
                            #[path = "."]
                            pub mod mutations {
                                #[path = "."]
                                pub mod create_asset {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🖼️asset/🧬️schema/🧬️mutations/🆕️create-asset/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🖼️asset/🧬️schema/🧬️mutations/🆕️create-asset/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🖼️asset/🧬️schema/🧬️mutations/🆕️create-asset/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🖼️asset/🧬️schema/🧬️mutations/🆕️create-asset/🧪️tests/➕️adds-a-second-image-asset/🦀️.rs"]
                                    mod tests_adds_a_second_image_asset;
                                }
                                #[path = "."]
                                pub mod replace_asset_payload {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🖼️asset/🧬️schema/🧬️mutations/🔁️replace-asset-payload/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🖼️asset/🧬️schema/🧬️mutations/🔁️replace-asset-payload/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🖼️asset/🧬️schema/🧬️mutations/🔁️replace-asset-payload/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🖼️asset/🧬️schema/🧬️mutations/🔁️replace-asset-payload/🧪️tests/🔁️swaps-logo-2808bd/🦀️.rs"]
                                    mod tests_swaps_logo_payload_for_svg;
                                }
                                #[path = "."]
                                pub mod delete_asset {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🖼️asset/🧬️schema/🧬️mutations/🗑️delete-asset/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🖼️asset/🧬️schema/🧬️mutations/🗑️delete-asset/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🖼️asset/🧬️schema/🧬️mutations/🗑️delete-asset/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🖼️asset/🧬️schema/🧬️mutations/🗑️delete-asset/🧪️tests/🗑️removes-the-logo-asset/🦀️.rs"]
                                    mod tests_removes_the_logo_asset;
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod block {
                        #[path = "."]
                        pub mod schema {
                            #[path = "."]
                            pub mod mutations {
                                #[path = "."]
                                pub mod create_block {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/➕️create-block/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/➕️create-block/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/➕️create-block/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/➕️create-block/🧪️tests/📷️inserts-a-photo-418575/🦀️.rs"]
                                    mod tests_inserts_a_photo_block_at_root_index_2;
                                }
                                #[path = "."]
                                pub mod delete_block {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/❌️delete-block/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/❌️delete-block/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/❌️delete-block/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/❌️delete-block/🧪️tests/➖️removes-the-math-block/🦀️.rs"]
                                    mod tests_removes_the_math_block;
                                }
                                #[path = "."]
                                pub mod delete_blocks {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🧹️delete-blocks/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🧹️delete-blocks/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🧹️delete-blocks/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🧹️delete-blocks/🧪️tests/🗑️removes-the-ink-5664a5/🦀️.rs"]
                                    mod tests_removes_the_ink_and_image_blocks;
                                }
                                #[path = "."]
                                pub mod duplicate_block {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/📋️duplicate-block/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/📋️duplicate-block/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/📋️duplicate-block/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/📋️duplicate-block/🧪️tests/📋️copies-the-math-6630c6/🦀️.rs"]
                                    mod tests_copies_the_math_block_right_after_its_source;
                                }
                                #[path = "."]
                                pub mod duplicate_blocks {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/👥️duplicate-blocks/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/👥️duplicate-blocks/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/👥️duplicate-blocks/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/👥️duplicate-blocks/🧪️tests/👥️copies-ink-and-7ddb0d/🦀️.rs"]
                                    mod tests_copies_ink_and_table_with_shifting_indices;
                                }
                                #[path = "."]
                                pub mod move_block_to_container {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🚚️move-block-to-container/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🚚️move-block-to-container/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🚚️move-block-to-container/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🚚️move-block-to-container/🧪️tests/📥️reparents-ink-e7faab/🦀️.rs"]
                                    mod tests_reparents_ink_into_the_callout_group;
                                }
                                #[path = "."]
                                pub mod drag_blocks {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🤏️drag-blocks/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🤏️drag-blocks/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🤏️drag-blocks/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🤏️drag-blocks/🧪️tests/🤏️nudges-ink-and-the-d88d16/🦀️.rs"]
                                    mod tests_nudges_ink_and_the_whole_group_subtree;
                                }
                                #[path = "."]
                                pub mod rename_block {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🔖️rename-block/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🔖️rename-block/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🔖️rename-block/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🔖️rename-block/🧪️tests/🏷️renames-the-table-block/🦀️.rs"]
                                    mod tests_renames_the_table_block;
                                }
                                #[path = "."]
                                pub mod change_block_visible {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/👀️change-block-visible/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/👀️change-block-visible/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/👀️change-block-visible/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/👀️change-block-visible/🧪️tests/🙈️hides-the-image-ec51f1/🦀️.rs"]
                                    mod tests_hides_the_image_block;
                                }
                                #[path = "."]
                                pub mod change_block_locked {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🔒️change-block-locked/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🔒️change-block-locked/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🔒️change-block-locked/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🔒️change-block-locked/🧪️tests/🔒️locks-the-d97c6f/🦀️.rs"]
                                    mod tests_locks_the_callout_group;
                                }
                                #[path = "."]
                                pub mod move_block {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/📍️move-block/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/📍️move-block/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/📍️move-block/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/📍️move-block/🧪️tests/📍️repositions-the-math-block/🦀️.rs"]
                                    mod tests_repositions_the_math_block;
                                }
                                #[path = "."]
                                pub mod resize_block {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/↔️resize-block/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/↔️resize-block/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/↔️resize-block/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/↔️resize-block/🧪️tests/📐️enlarges-the-image-block/🦀️.rs"]
                                    mod tests_enlarges_the_image_block;
                                }
                                #[path = "."]
                                pub mod change_block_font_size {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🔤️change-block-font-size/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🔤️change-block-font-size/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🔤️change-block-font-size/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🔤️change-block-font-size/🧪️tests/🔤️enlarges-the-f1d8c5/🦀️.rs"]
                                    mod tests_enlarges_the_intro_font;
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod canvas {
                        #[path = "."]
                        pub mod schema {
                            #[path = "."]
                            pub mod mutations {
                                #[path = "."]
                                pub mod change_grid_visible {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/👁️change-grid-visible/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/👁️change-grid-visible/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/👁️change-grid-visible/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/👁️change-grid-visible/🧪️tests/🙈️hides-the-grid/🦀️.rs"]
                                    mod tests_hides_the_grid;
                                }
                                #[path = "."]
                                pub mod change_grid_spacing {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/📏️change-grid-spacing/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/📏️change-grid-spacing/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/📏️change-grid-spacing/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/📏️change-grid-spacing/🧪️tests/📏️widens-grid-spacing/🦀️.rs"]
                                    mod tests_widens_grid_spacing;
                                }
                                #[path = "."]
                                pub mod change_grid_subdivisions {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/🔢️change-grid-subdivisions/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/🔢️change-grid-subdivisions/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/🔢️change-grid-subdivisions/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/🔢️change-grid-subdivisions/🧪️tests/🔢️doubles-grid-fa3b25/🦀️.rs"]
                                    mod tests_doubles_grid_subdivisions;
                                }
                                #[path = "."]
                                pub mod change_grid_opacity {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/🌫️change-grid-opacity/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/🌫️change-grid-opacity/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/🌫️change-grid-opacity/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/🌫️change-grid-opacity/🧪️tests/🌫️raises-grid-opacity/🦀️.rs"]
                                    mod tests_raises_grid_opacity;
                                }
                                #[path = "."]
                                pub mod change_snap_enabled {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/🧲️change-snap-enabled/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/🧲️change-snap-enabled/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/🧲️change-snap-enabled/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/🧲️change-snap-enabled/🧪️tests/🧲️enables-snap/🦀️.rs"]
                                    mod tests_enables_snap;
                                }
                                #[path = "."]
                                pub mod change_snap_grid_spacing {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/📐️change-snap-grid-spacing/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/📐️change-snap-grid-spacing/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/📐️change-snap-grid-spacing/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/📐️change-snap-grid-spacing/🧪️tests/📐️halves-snap-grid-bad026/🦀️.rs"]
                                    mod tests_halves_snap_grid_spacing;
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod document {
                        #[path = "."]
                        pub mod schema {
                            #[path = "."]
                            pub mod mutations {
                                #[path = "."]
                                pub mod rename_note {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/📜️document/🧬️schema/🧬️mutations/🏷️rename-note/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/📜️document/🧬️schema/🧬️mutations/🏷️rename-note/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/📜️document/🧬️schema/🧬️mutations/🏷️rename-note/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/📜️document/🧬️schema/🧬️mutations/🏷️rename-note/🧪️tests/🏷️retitles-the-document/🦀️.rs"]
                                    mod tests_retitles_the_document;
                                }
                            }
                        }
                    }
                }
                #[path = "🏅️standards/🔖️1/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op {
            pub use crate::standards::v1::subsets::any::io::mutations::text::*;
            pub use crate::standards::v1::subsets::any::schema::mutations::NoteMutation;
        }
        pub mod dsl {
            pub use crate::standards::v1::subsets::any::io::snapshot::text::*;
        }
        pub mod spr {
            pub use crate::standards::v1::subsets::any::io::mutations::binary::*;
        }
        pub mod diff {
            pub use crate::standards::v1::subsets::any::schema::diff::*;
            pub mod schema {
                pub use crate::standards::v1::subsets::any::schema::diff::*;
            }
            pub mod text {
                pub use crate::standards::v1::subsets::any::io::diff::text::*;
            }
        }
        pub mod mutations {
            pub use crate::standards::v1::subsets::any::schema::mutations::*;
        }
        pub mod snapshot {
            pub mod schema {
                pub use crate::standards::v1::subsets::any::schema::snapshot::*;
            }
            pub mod pack {
                pub use crate::standards::v1::subsets::any::io::snapshot::binary::*;
            }
        }
        pub mod examples {
            pub use super::standards::v1::subsets::any::examples::*;
        }

/// ✏️ The mutation-capable surface (contract §2.1/§2.4) — every leaf `#[path]`-mounted from the real
/// subset dir under `🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/…`.
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod note {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️.rs"]
        pub mod retained;

        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs"]
        pub mod terminology;

        #[path = "."]
        pub mod commands {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➕️add-block/🦀️.rs"]
            pub mod add_block;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗑️delete-block/🦀️.rs"]
            pub mod delete_block;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚫️delete-selection/🦀️.rs"]
            pub mod delete_selection;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️duplicate-block/🦀️.rs"]
            pub mod duplicate_block;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🪞️duplicate-selection/🦀️.rs"]
            pub mod duplicate_selection;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/💬️engagement-input/🦀️.rs"]
            pub mod engagement_input;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📤️engagement-submit/🦀️.rs"]
            pub mod engagement_submit;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖊️ink-apply-events/🦀️.rs"]
            pub mod ink_apply_events;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️load-request/🦀️.rs"]
            pub mod load_request;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚚️move-block/🦀️.rs"]
            pub mod move_block;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧭️navigator-engagement-input/🦀️.rs"]
            pub mod navigator_engagement_input;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🕹️nudge-selection/🦀️.rs"]
            pub mod nudge_selection;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⬇️nudge-selection-down/🦀️.rs"]
            pub mod nudge_selection_down;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⏬️nudge-selection-down-fast/🦀️.rs"]
            pub mod nudge_selection_down_fast;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⬅️nudge-selection-left/🦀️.rs"]
            pub mod nudge_selection_left;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⏪️nudge-selection-left-fast/🦀️.rs"]
            pub mod nudge_selection_left_fast;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➡️nudge-selection-right/🦀️.rs"]
            pub mod nudge_selection_right;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⏩️nudge-selection-right-fast/🦀️.rs"]
            pub mod nudge_selection_right_fast;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⬆️nudge-selection-up/🦀️.rs"]
            pub mod nudge_selection_up;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⏫️nudge-selection-up-fast/🦀️.rs"]
            pub mod nudge_selection_up_fast;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🩹️patch-blocks/🦀️.rs"]
            pub mod patch_blocks;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/💾️save-download/🦀️.rs"]
            pub mod save_download;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗃️set-active-example/🦀️.rs"]
            pub mod set_active_example;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧰️set-active-utility/🦀️.rs"]
            pub mod set_active_utility;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎥️set-camera/🦀️.rs"]
            pub mod set_camera;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔭️set-camera-zoom/🦀️.rs"]
            pub mod set_camera_zoom;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧽️set-eraser-radius/🦀️.rs"]
            pub mod set_eraser_radius;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧫️set-fixture-json/🦀️.rs"]
            pub mod set_fixture_json;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔲️set-grid-opacity/🦀️.rs"]
            pub mod set_grid_opacity;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📏️set-grid-spacing/🦀️.rs"]
            pub mod set_grid_spacing;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔢️set-grid-subdivisions/🦀️.rs"]
            pub mod set_grid_subdivisions;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️set-grid-visible/🦀️.rs"]
            pub mod set_grid_visible;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗣️set-locale/🦀️.rs"]
            pub mod set_locale;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️set-pencil-width/🦀️.rs"]
            pub mod set_pencil_width;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧲️set-snap-enabled/🦀️.rs"]
            pub mod set_snap_enabled;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📐️set-snap-grid-spacing/🦀️.rs"]
            pub mod set_snap_grid_spacing;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod composite {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️composite/🦀️.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod options {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️composite/☑️options/🎥️camera/🦀️.rs"]
                            pub mod camera;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️composite/☑️options/🧹️eraser-point/🦀️.rs"]
                            pub mod eraser_point;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️composite/☑️options/🧽️eraser-stroke/🦀️.rs"]
                            pub mod eraser_stroke;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️composite/☑️options/🔲️grid/🦀️.rs"]
                            pub mod grid;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️composite/☑️options/✏️pencil/🦀️.rs"]
                            pub mod pencil;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️composite/☑️options/🧲️snap/🦀️.rs"]
                            pub mod snap;
                        }
                    }

                    #[path = "."]
                    pub mod navigator {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧭️navigator/🦀️.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod options {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧭️navigator/☑️options/🔲️grid-visible/🦀️.rs"]
                            pub mod grid_visible;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧭️navigator/☑️options/🔍️zoom/🦀️.rs"]
                            pub mod zoom;
                        }
                    }
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️.rs"]
            pub mod catalogue;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🦀️.rs"]
            pub mod document;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs"]
            pub mod inspection;
        }
    }
}

/// 👁️ The read-only surface (contract §2.2/§2.6) — a genuinely independent module tree from
/// `editor` above, never `#[path]`-mounting anything under `✏️editor/`: that would let
/// `policyViewerPurityBreaches`' substring check on the sibling editor module catch a real
/// dependency, but the deeper reason is architectural — the viewer must stay constructible without
/// ever touching the editor's mutation-capable types.
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod note {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🖼️composite/🦀️.rs"]
                    pub mod composite;
                }
            }
        }
    }
}
