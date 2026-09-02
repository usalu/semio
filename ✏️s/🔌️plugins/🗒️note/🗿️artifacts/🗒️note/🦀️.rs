//! 📝️ Note artifact — the document entity this plugin's app edits: an infinite-canvas block tree
//! (text/image/table/math/ink/group blocks).

use semio_framework_plugin::{ArtifactKindSpec, Dialect, MediaClass, MediaForm, MediaType, OsMediaCapability, StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::text::schema::snapshot::{SemioTextMark, SemioTextMarkKind, SemioTextRun, SemioTextSnapshot, STDIO_SEMIOTEXT_DOCUMENT_SCHEMA};
use serde::{Deserialize, Serialize};
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
pub async fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    let rows: &[(&str, &str, &str, &[(&str, &str)], Option<(&str, &str)>)] = &[
        ("s.note.standard.v1", "standard", "1", &[], None),
        ("s.note.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.note.schema.artifact", "schema", "s.note.note", &[("schema", "s.note.note")], None),
        ("s.note.inference.artifact", "inference", "s.note.note.inference", &[("schema", "s.note.note.inference")], None),
        // 🐛️ Pre-existing gap (found while verifying ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-
        // PLUGIN-RUNTIME E2-builder-descriptor's proof migration: `Plugin::builder("note")...
        // try_build()` was silently failing assembly — `PluginAssemblyError{code:"artifact-
        // definition.runtime-capability", message:"no declared composer capability owns the
        // runtime claims"}` — never surfaced before because nothing checked `try_build()`'s
        // `Result` against a real assertion). `io_registry::entries()` registers SEVEN composer
        // rows (`crate::artifacts::note::…🚪️io/🦀️.rs`'s `entries()`), not six: the six
        // `EXPORT_*` rows below plus `composer_entry_of::<NoteAnyComposer>()`, whose `writes` is
        // this artifact's OWN dialect (`NoteComposer::DIALECT`, `s.note@1/*`) — the native "compose
        // my own snapshot from various format sources" composer. `PluginBuilder::declare(…)
        // .composers(entries)` requires a declared "composer" capability whose `dialect` claim
        // matches EVERY entry's `writes` coordinate; only the six STDIO-format rows existed, so the
        // native self-composer's claim never matched anything. Added here rather than removing the
        // native composer from `entries()`: the self-composer is real, load-bearing behaviour
        // (`rebuild_native_snapshot` is the shared decode path every `EXPORT_*` composer calls).
        ("s.note.composer.note", "composer", "s.note@1/*", &[("dialect", "s.note@1/*")], None),
        ("s.note.composer.svg", "composer", "s.stdio.svg@1.1/*", &[("dialect", "s.stdio.svg@1.1/*")], None),
        ("s.note.composer.pdf", "composer", "s.stdio.pdf@1.4/*", &[("dialect", "s.stdio.pdf@1.4/*")], None),
        ("s.note.composer.png", "composer", "s.stdio.png@1.2/*", &[("dialect", "s.stdio.png@1.2/*")], None),
        ("s.note.composer.json", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.note.composer.dwg", "composer", "s.stdio.dwg@ac1018/*", &[("dialect", "s.stdio.dwg@ac1018/*")], None),
        ("s.note.composer.dxf", "composer", "s.stdio.dxf@r12/*", &[("dialect", "s.stdio.dxf@r12/*")], None),
        ("s.note.grammar.document", "grammar", "note.document", &[("grammar", "note.document")], None),
        ("s.note.grammar.op", "grammar", "note.op", &[("grammar", "note.op")], None),
        ("s.note.grammar.diff", "grammar", "note.diff", &[("grammar", "note.diff")], None),
        ("s.note.grammar.pack", "grammar", "note.pack", &[("grammar", "note.pack")], None),
        ("s.note.grammar.spr", "grammar", "note.spr", &[("grammar", "note.spr")], None),
        ("s.note.codec.document.v1", "codec", "note.document:note", &[("codec", "note.document"), ("extension", "note")], None),
        ("s.note.localization.en", "localization", "Note", &[], Some(("en", "Note"))),
        ("s.note.localization.de", "localization", "Notiz", &[], Some(("de", "Notiz"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.note")?);
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
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.note.note").expect("canonical note kind"), localization: &[], standards: vec![crate::artifacts::note::standards::v1::standard()] }
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
        export_stdio_kinds: crate::artifacts::note::io::export_stdio_kinds().to_vec(),
        import_stdio_kinds: crate::artifacts::note::io::import_stdio_kinds().to_vec(),
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct NoteCamera {
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
    #[serde(default = "default_zoom")]
    pub zoom: f64,
}

impl Default for NoteCamera {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

pub async fn default_zoom() -> f64 {
    1.0
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum NoteBlockNode {
    #[serde(rename = "text", rename_all = "camelCase")]
    Text {
        id: String,
        name: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        #[serde(default)]
        rotation: f64,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default)]
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
    #[serde(rename = "image", rename_all = "camelCase")]
    Image {
        id: String,
        name: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        #[serde(default)]
        rotation: f64,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default)]
        locked: bool,
        image_key: String,
    },
    #[serde(rename = "table", rename_all = "camelCase")]
    Table {
        id: String,
        name: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        #[serde(default)]
        rotation: f64,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default)]
        locked: bool,
        columns: Vec<String>,
        rows: Vec<Vec<NoteTableCell>>,
    },
    #[serde(rename = "math", rename_all = "camelCase")]
    Math {
        id: String,
        name: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        #[serde(default)]
        rotation: f64,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default)]
        locked: bool,
        #[dsl(lang = "tex")]
        tex: String,
        display_mode: bool,
    },
    #[serde(rename = "stroke", rename_all = "camelCase")]
    #[dsl(key = "stroke")]
    Ink {
        id: String,
        name: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        #[serde(default)]
        rotation: f64,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default)]
        locked: bool,
        points: Vec<[f64; 2]>,
        stroke_width: f64,
        color: [f64; 4],
    },
    #[serde(rename = "group", rename_all = "camelCase")]
    Group {
        id: String,
        name: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        #[serde(default)]
        rotation: f64,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default)]
        locked: bool,
        #[dsl(statements, block)]
        children: Vec<NoteBlockNode>,
    },
}

//#region 🔖️ComposedTypes
/// 🕸️ Snapshot-owned text child record. The handle preserves composition identity while the bounded
/// paragraph records are durable authority that survives reopen and worker migration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct NoteTextChild {
    pub handle: store::ArtifactChild<SemioTextSnapshot>,
    #[serde(default)]
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
pub async fn text_snapshot_from_paragraphs(paragraphs: &[NoteTextParagraph]) -> SemioTextSnapshot {
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
pub async fn paragraphs_from_text_snapshot(snapshot: &SemioTextSnapshot) -> Vec<NoteTextParagraph> {
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
pub async fn note_text_child_handle(block_id: &str, paragraphs: &[NoteTextParagraph]) -> NoteTextChild {
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
pub async fn note_block_text(handle: &NoteTextChild) -> Vec<NoteTextParagraph> {
    handle.paragraphs.clone()
}

/// 🏗️ Mints a new content-addressed handle and stores its paragraphs in the same snapshot-owned
/// record used by mutation-diff, fixture, and converter builders.
pub async fn note_text_child_record(block_id: &str, paragraphs: &[NoteTextParagraph]) -> NoteTextChild {
    note_text_child_handle(block_id, paragraphs)
}
//#endregion 🔖️TextChildren

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "r")]
pub struct NoteTextRun {
    #[dsl(positional)]
    pub text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub underline: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "p")]
pub struct NoteTextParagraph {
    pub runs: Vec<NoteTextRun>,
}

pub async fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct NoteTableCell {
    #[dsl(positional)]
    pub content: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct NoteImageAsset {
    pub mime: String,
    pub data: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
}

pub use crate::artifacts::note::schema::diff::NoteDiff;
pub use crate::artifacts::note::schema::mutations::NoteMutation;
pub use crate::artifacts::note::schema::snapshot::NoteSnapshot;

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
        let json_text = serde_json::to_string(&document).unwrap();
        let parsed: NoteSnapshot = serde_json::from_str(&json_text).unwrap();
        assert_eq!(parsed.assets.get("asset-1").unwrap().mime, "image/png");
        assert_eq!(parsed.grid_subdivisions, Some(6.0));
        assert_eq!(parsed.grid_opacity, Some(0.5));
    }
}
//#endregion 🧪️Tests
