//! 🧬️ `PdfVtMutation` — the ISO 16612-2 (PDF/VT-1) CONFORMANCE-CLASS vocabulary of `stdio.pdf` 1.7. Every
//! variant's `diff()` is handcrafted directly against `base` through a named transform, and every
//! variant's `inverse()` is handcrafted, reading whatever pre-state it needs out of the base.
//!
//! **Why this subset needs a vocabulary of its own.** `✳️any` owns the DOCUMENT vocabulary —
//! `insert-page`, `remove-page`, `move-page`, the media/crop box kinds, page content, `/Info` as
//! authoring metadata, and the raw object/dict/trailer edit primitives. Not one of those mutations
//! can move a document between conformance classes, because a conformance class is a property of the
//! retained object GRAPH and of no page at all. This enum is one variant per axis of this subset's
//! own `check_vt_conformance` (`../../🦀️component.rs`), which reads every axis `check_x_conformance` reads, plus two of its own: `/Root/DPartRoot` (hard) and a `/DPM` metadata dictionary on every `/DPart` node reachable from it (soft).
//!
//! This is the one place in this artifact where a vocabulary is a strict SUPERSET of a sibling's, and it is so by the subset's own code rather than by copying: `check_vt_conformance`'s first statement is literally `let mut out = check_x_conformance(snapshot);`, because ISO 16612-2 is defined ON TOP of ISO 15930 — a PDF/VT file is a PDF/X file with a document-part hierarchy. The sixteen inherited kinds are therefore not duplicated prose but a stated inheritance, and the four that are this subset's own — the `/DPartRoot` pair and the `/DPM` pair — are the variable-data partitioning mechanism no other conformance class in this standard has any concept of. The implementation is shared through the named `document::pdf_conformance` engine, never copied: what differs between `✳️x` and `✳️vt` is the declared axis list and the declared vocabulary, which is what a subset is.
//!
//! The two vocabularies are disjoint by construction: no `✳️any` mutation moves an axis this enum
//! addresses, and no variant here touches page content.
//!
//! `Diff` is `PdfDiff`, the SAME diff type `✳️any` uses — the two subsets share one snapshot type, so
//! they share its diff. What differs is the vocabulary that produces it, which is what a subset is.
//! `ArtifactBuilder::Mutation` on this subset's builder still names `✳️any`'s document vocabulary: a
//! builder has exactly one associated mutation type, and a conformant document still needs its pages
//! edited. Unifying the two behind one type is a deliberate open seam, recorded rather than guessed at.
//!
//! The object-graph primitives every one of the six conformance vocabularies needs live in the ONE
//! named module `✳️any::schema::mutations::conformance_support`, reached by name here rather than
//! copied — six copies of the same graph surgery is exactly what a shared module exists to prevent.
//!
//! @see ../../🧪️oracle/🔣️component.json — the mutation catalog `KINDS` is measured against.
//! @see ../🦀️component.rs — this subset's conformance check, one axis per variant below.

use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::diff::PdfDiff;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::mutations::conformance_support as support;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::{ObjRef, PdfObject, PdfSnapshot};
use protocol::command::DiffAlgebra;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Class
/// 🏳️ The OutputIntent marker this subset's own checker demands, and whether that intent must also
/// carry a `/DestOutputProfile`. Both are read straight off `check_vt_conformance`.
pub const OUTPUT_INTENT_SUBTYPE: &str = "GTS_PDFX";
pub const OUTPUT_INTENT_DEST_PROFILE: bool = true;

/// 📇️ The metadata a class stamp writes when this subset polices document metadata at all.
pub const CONFORMANT_TITLE: &str = "A PDF/VT-1 conformant document";
pub const CONFORMANT_AUTHOR: &str = "semio stdio conformance stamp";
//#endregion 🔖️Class

//#region 🔖️Mutations
/// 📐️ Typed conformance-class mutation for `stdio.pdf` 1.7 under ISO 16612-2 (PDF/VT-1). Every variant
/// addresses ONE axis of the class; none addresses page content.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum PdfVtMutation {
    /// 🚫️ The identity element of the vocabulary.
    #[default]
    NoMutation,
    /// 🔄️ Replaces the whole document. A conformance class is a whole-graph property, so this is
    /// the class stamp in its total form — every axis at once. Build the target with
    /// [`stamp_conformance`].
    SetSnapshot {
        snapshot: PdfSnapshot,
    },
    /// 🔒️ Adds a Standard Security Handler dictionary object declaring `/V version /R revision`.
    InsertEncryptionDictionary {
        version: i64,
        revision: i64,
    },
    /// 🔓️ Drops the Standard Security Handler dictionary declaring `/V version /R revision`.
    RemoveEncryptionDictionary {
        version: i64,
        revision: i64,
    },
    /// 🏳️ Installs `/Root/OutputIntents` with one intent carrying this subset's own marker.
    SetOutputIntent {
        identifier: String,
    },
    /// 🏳️ Drops `/Root/OutputIntents` entirely.
    RemoveOutputIntent,
    /// 📄️ Sets page `page_index`'s `/TrimBox` — the per-page geometry ISO 15930-7 requires on
    /// every page and no other conformance class in this standard reads at all.
    SetTrimBox {
        page_index: usize,
        trim_box: [f64; 4],
    },
    /// 📄️ Drops page `page_index`'s `/TrimBox`.
    RemoveTrimBox {
        page_index: usize,
    },
    /// 🔤️ Points the `descriptor_ordinal`-th `/FontDescriptor` at the font program object
    /// `program` through `key` (`/FontFile`, `/FontFile2` or `/FontFile3`).
    EmbedFontFile {
        descriptor_ordinal: usize,
        key: String,
        program: ObjRef,
    },
    /// 🔤️ Drops whichever of the three font-program keys the `descriptor_ordinal`-th
    /// `/FontDescriptor` carries.
    RemoveFontFile {
        descriptor_ordinal: usize,
    },
    /// 📜️ Adds an `/S /JavaScript` action carrying `script` in its `/JS`.
    InsertJavaScriptAction {
        script: String,
    },
    /// 📜️ Drops the `/S /JavaScript` action carrying `script`.
    RemoveJavaScriptAction {
        script: String,
    },
    /// 🚀️ Adds an `/S /Launch` action targeting `target` in its `/F`.
    InsertLaunchAction {
        target: String,
    },
    /// 🚀️ Drops the `/S /Launch` action targeting `target`.
    RemoveLaunchAction {
        target: String,
    },
    /// 🎬️ Adds a `/Subtype /Movie` or `/Subtype /Sound` annotation titled `title`. `/Subtype /3D`
    /// is a different name and is deliberately not reachable here.
    InsertMediaAnnotation {
        subtype: String,
        title: String,
    },
    /// 🎬️ Drops the `/Subtype /Movie` or `/Subtype /Sound` annotation titled `title`.
    RemoveMediaAnnotation {
        subtype: String,
        title: String,
    },
    /// 🗂️ Installs `/Root/DPartRoot` over one `/Type /DPart` node carrying `/DPM << /Job … >>`.
    SetDpartRoot {
        job: String,
    },
    /// 🗂️ Drops `/Root/DPartRoot`.
    RemoveDpartRoot,
    /// 🗂️ Rewrites the root `/DPart` node's `/DPM` metadata.
    SetDpartMetadata {
        job: String,
    },
    /// 🗂️ Drops the root `/DPart` node's `/DPM` metadata.
    RemoveDpartMetadata,
}

/// 🧾️ Kebab-case spelling of every `PdfVtMutation` variant, in declaration order — the exhaustive
/// mutation catalog `pdf-1-7-vt` (`../../🧪️oracle/🔣️component.json`) is measured against this
/// exact list. `kinds_match_enum_and_catalog` proves it never drifts from either side.
pub const KINDS: &[&str] = &["no-mutation", "set-snapshot", "insert-encryption-dictionary", "remove-encryption-dictionary", "set-output-intent", "remove-output-intent", "set-trim-box", "remove-trim-box", "embed-font-file", "remove-font-file", "insert-javascript-action", "remove-javascript-action", "insert-launch-action", "remove-launch-action", "insert-media-annotation", "remove-media-annotation", "set-dpart-root", "remove-dpart-root", "set-dpart-metadata", "remove-dpart-metadata"];
//#endregion 🔖️Mutations

//#region 🔖️Stamp
/// 🏅️ Stamps every axis this subset OWNS into (or out of) its conformant state — the whole-document
/// target `SetSnapshot` carries. Only axes whose conformant state is the PRESENCE of something are
/// stamped: an axis whose conformant state is the ABSENCE of a forbidden construct (encryptionDictionaries, fontPrograms, javaScriptActions, launchActions, mediaAnnotations) is
/// already conformant on a document that does not carry it, and adding one in order to remove it
/// again would be theatre rather than a stamp.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn stamp_conformance(base: PdfSnapshot, stamped: bool) -> PdfSnapshot {
    let mut next = base;
    if stamped {
            support::set_output_intent(&mut next, OUTPUT_INTENT_SUBTYPE, "sRGB IEC61966-2.1", OUTPUT_INTENT_DEST_PROFILE);
            for page in support::page_objects(&next) {
                let media = support::page_box(&next, page, "MediaBox").unwrap_or([0.0, 0.0, 612.0, 792.0]);
                support::set_entry(&mut next, page, "TrimBox", support::box_object(media));
            }
            support::set_dpart_root(&mut next, "variable-data job 1");
    } else {
            support::remove_catalog_entry(&mut next, "OutputIntents");
            for page in support::page_objects(&next) {
                support::remove_entry(&mut next, page, "TrimBox");
            }
            support::remove_catalog_entry(&mut next, "DPartRoot");
    }
    next
}
//#endregion 🔖️Stamp

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot` through its own diff — the diff is the single semantics
/// source, never a separate imperative apply path.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_vt_conformance_mutation(snapshot: &mut PdfSnapshot, mutation: &PdfVtMutation) -> protocol::MutationOutcome<PdfDiff> {
    let outcome = Mutation::diff(mutation, snapshot);
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}
//#endregion 🔖️Apply

//#region 🔖️Transform
/// 🔧️ The handcrafted per-variant graph surgery `diff()` measures. Written against a clone of
/// `base` so `diff()` stays a STATE DELTA computed from two real snapshots rather than an
/// apply-and-capture of a mutable pipeline — `PdfDiff::between` is the same primitive
/// `diff_set_snapshot` is built on, and it is exact on `objects` (keyed by `ObjRef`), on `trailer`
/// (keyed by dict key) and on `pages` (keyed by index).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
impl PdfVtMutation {
    fn transform(mutation: &PdfVtMutation, base: &PdfSnapshot) -> PdfSnapshot {
    let mut next = base.clone();
    match mutation {
            Self::NoMutation => {},
            Self::SetSnapshot { snapshot } => next = snapshot.clone(),
            Self::InsertEncryptionDictionary { version, revision } => { support::insert_object(&mut next, support::encryption_dictionary(*version, *revision)); },
            Self::RemoveEncryptionDictionary { version, revision } => { if let Some(id) = support::encryption_dictionary_with(&next, *version, *revision) { support::remove_object(&mut next, id); } },
            Self::SetOutputIntent { identifier } => { support::set_output_intent(&mut next, OUTPUT_INTENT_SUBTYPE, identifier, OUTPUT_INTENT_DEST_PROFILE); },
            Self::RemoveOutputIntent => { support::remove_catalog_entry(&mut next, "OutputIntents"); },
            Self::SetTrimBox { page_index, trim_box } => { if let Some(page) = support::page_objects(&next).get(*page_index).copied() { support::set_entry(&mut next, page, "TrimBox", support::box_object(*trim_box)); } },
            Self::RemoveTrimBox { page_index } => { if let Some(page) = support::page_objects(&next).get(*page_index).copied() { support::remove_entry(&mut next, page, "TrimBox"); } },
            Self::EmbedFontFile { descriptor_ordinal, key, program } => { if let Some(id) = support::font_descriptors(&next).get(*descriptor_ordinal).copied() { support::set_entry(&mut next, id, key, PdfObject::Ref(*program)); } },
            Self::RemoveFontFile { descriptor_ordinal } => {
                if let Some(id) = support::font_descriptors(&next).get(*descriptor_ordinal).copied() {
                    if let Some((key, _)) = support::font_program(&next, id) {
                        support::remove_entry(&mut next, id, &key);
                    }
                }
            },
            Self::InsertJavaScriptAction { script } => { support::insert_object(&mut next, support::action_object("JavaScript", "JS", script)); },
            Self::RemoveJavaScriptAction { script } => { if let Some(id) = support::action_with(&next, "JavaScript", "JS", script) { support::remove_object(&mut next, id); } },
            Self::InsertLaunchAction { target } => { support::insert_object(&mut next, support::action_object("Launch", "F", target)); },
            Self::RemoveLaunchAction { target } => { if let Some(id) = support::action_with(&next, "Launch", "F", target) { support::remove_object(&mut next, id); } },
            Self::InsertMediaAnnotation { subtype, title } => { support::insert_object(&mut next, support::media_annotation_object(subtype, title)); },
            Self::RemoveMediaAnnotation { subtype, title } => { if let Some(id) = support::media_annotation(&next, subtype, title) { support::remove_object(&mut next, id); } },
            Self::SetDpartRoot { job } => { support::set_dpart_root(&mut next, job); },
            Self::RemoveDpartRoot => { support::remove_catalog_entry(&mut next, "DPartRoot"); },
            Self::SetDpartMetadata { job } => { support::set_dpart_job(&mut next, Some(job)); },
            Self::RemoveDpartMetadata => { support::set_dpart_job(&mut next, None); },
    }
    next
    }
}
//#endregion 🔖️Transform

//#region 🔖️MutationTrait
impl Mutation<PdfSnapshot> for PdfVtMutation {
    type Diff = PdfDiff;

    fn diff(&self, base: &PdfSnapshot) -> protocol::MutationOutcome<Self::Diff> {
        protocol::MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &Self::transform(self, base)))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<Self> {
        vec![match self {
            Self::NoMutation => Self::NoMutation,
            Self::SetSnapshot { snapshot } => Self::SetSnapshot { snapshot: base.clone() },
            Self::InsertEncryptionDictionary { version, revision } => Self::RemoveEncryptionDictionary { version: *version, revision: *revision },
            Self::RemoveEncryptionDictionary { version, revision } => Self::InsertEncryptionDictionary { version: *version, revision: *revision },
            Self::SetOutputIntent { identifier } => Self::RemoveOutputIntent,
            Self::RemoveOutputIntent => match support::output_intent_identifier(base) {
                Some(identifier) => Self::SetOutputIntent { identifier },
                None => Self::NoMutation,
            },
            Self::SetTrimBox { page_index, trim_box } => match support::page_objects(base).get(*page_index).copied().and_then(|page| support::page_box(base, page, "TrimBox")) {
                Some(previous) => Self::SetTrimBox { page_index: *page_index, trim_box: previous },
                None => Self::RemoveTrimBox { page_index: *page_index },
            },
            Self::RemoveTrimBox { page_index } => match support::page_objects(base).get(*page_index).copied().and_then(|page| support::page_box(base, page, "TrimBox")) {
                Some(previous) => Self::SetTrimBox { page_index: *page_index, trim_box: previous },
                None => Self::NoMutation,
            },
            Self::EmbedFontFile { descriptor_ordinal, key, program } => Self::RemoveFontFile { descriptor_ordinal: *descriptor_ordinal },
            Self::RemoveFontFile { descriptor_ordinal } => match support::font_descriptors(base).get(*descriptor_ordinal).copied().and_then(|id| support::font_program(base, id)) {
                Some((key, program)) => Self::EmbedFontFile { descriptor_ordinal: *descriptor_ordinal, key, program },
                None => Self::NoMutation,
            },
            Self::InsertJavaScriptAction { script } => Self::RemoveJavaScriptAction { script: script.clone() },
            Self::RemoveJavaScriptAction { script } => Self::InsertJavaScriptAction { script: script.clone() },
            Self::InsertLaunchAction { target } => Self::RemoveLaunchAction { target: target.clone() },
            Self::RemoveLaunchAction { target } => Self::InsertLaunchAction { target: target.clone() },
            Self::InsertMediaAnnotation { subtype, title } => Self::RemoveMediaAnnotation { subtype: subtype.clone(), title: title.clone() },
            Self::RemoveMediaAnnotation { subtype, title } => Self::InsertMediaAnnotation { subtype: subtype.clone(), title: title.clone() },
            Self::SetDpartRoot { job } => Self::RemoveDpartRoot,
            Self::RemoveDpartRoot => Self::SetDpartRoot { job: support::dpart_job(base).unwrap_or_default() },
            Self::SetDpartMetadata { job } => match support::dpart_job(base) {
                Some(previous) => Self::SetDpartMetadata { job: previous },
                None => Self::RemoveDpartMetadata,
            },
            Self::RemoveDpartMetadata => match support::dpart_job(base) {
                Some(previous) => Self::SetDpartMetadata { job: previous },
                None => Self::NoMutation,
            },
        }]
    }
}
//#endregion 🔖️MutationTrait

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationDiff;

    //#region 🔖️KindsConformanceLaw
    /// 🧭️ `kind_of` is an EXHAUSTIVE match (no wildcard arm) — the compiler refuses this file if a
    /// variant is added to `PdfVtMutation` without a matching kebab-case spelling here, which is what
    /// keeps `KINDS` honest against the enum. The second half reads the sibling oracle manifest's
    /// `kinds` array as text (the framework never parses Rust, so this is the only side that can
    /// prove the manifest matches) and asserts the same list, in the same order.
    #[test]
    fn kinds_match_enum_and_catalog() {
        fn kind_of(mutation: &PdfVtMutation) -> &'static str {
            match mutation {
                PdfVtMutation::NoMutation => "no-mutation",
                PdfVtMutation::SetSnapshot { .. } => "set-snapshot",
                PdfVtMutation::InsertEncryptionDictionary { .. } => "insert-encryption-dictionary",
                PdfVtMutation::RemoveEncryptionDictionary { .. } => "remove-encryption-dictionary",
                PdfVtMutation::SetOutputIntent { .. } => "set-output-intent",
                PdfVtMutation::RemoveOutputIntent => "remove-output-intent",
                PdfVtMutation::SetTrimBox { .. } => "set-trim-box",
                PdfVtMutation::RemoveTrimBox { .. } => "remove-trim-box",
                PdfVtMutation::EmbedFontFile { .. } => "embed-font-file",
                PdfVtMutation::RemoveFontFile { .. } => "remove-font-file",
                PdfVtMutation::InsertJavaScriptAction { .. } => "insert-javascript-action",
                PdfVtMutation::RemoveJavaScriptAction { .. } => "remove-javascript-action",
                PdfVtMutation::InsertLaunchAction { .. } => "insert-launch-action",
                PdfVtMutation::RemoveLaunchAction { .. } => "remove-launch-action",
                PdfVtMutation::InsertMediaAnnotation { .. } => "insert-media-annotation",
                PdfVtMutation::RemoveMediaAnnotation { .. } => "remove-media-annotation",
                PdfVtMutation::SetDpartRoot { .. } => "set-dpart-root",
                PdfVtMutation::RemoveDpartRoot => "remove-dpart-root",
                PdfVtMutation::SetDpartMetadata { .. } => "set-dpart-metadata",
                PdfVtMutation::RemoveDpartMetadata => "remove-dpart-metadata",
            }
        }
        let samples = [
            PdfVtMutation::NoMutation,
            PdfVtMutation::SetSnapshot { snapshot: PdfSnapshot::default() },
            PdfVtMutation::InsertEncryptionDictionary { version: 2, revision: 3 },
            PdfVtMutation::RemoveEncryptionDictionary { version: 2, revision: 3 },
            PdfVtMutation::SetOutputIntent { identifier: String::new() },
            PdfVtMutation::RemoveOutputIntent,
            PdfVtMutation::SetTrimBox { page_index: 0, trim_box: [0.0; 4] },
            PdfVtMutation::RemoveTrimBox { page_index: 0 },
            PdfVtMutation::EmbedFontFile { descriptor_ordinal: 0, key: String::new(), program: ObjRef::default() },
            PdfVtMutation::RemoveFontFile { descriptor_ordinal: 0 },
            PdfVtMutation::InsertJavaScriptAction { script: String::new() },
            PdfVtMutation::RemoveJavaScriptAction { script: String::new() },
            PdfVtMutation::InsertLaunchAction { target: String::new() },
            PdfVtMutation::RemoveLaunchAction { target: String::new() },
            PdfVtMutation::InsertMediaAnnotation { subtype: String::new(), title: String::new() },
            PdfVtMutation::RemoveMediaAnnotation { subtype: String::new(), title: String::new() },
            PdfVtMutation::SetDpartRoot { job: String::new() },
            PdfVtMutation::RemoveDpartRoot,
            PdfVtMutation::SetDpartMetadata { job: String::new() },
            PdfVtMutation::RemoveDpartMetadata,
        ];
        let from_enum: Vec<&'static str> = samples.iter().map(kind_of).collect();
        assert_eq!(from_enum, KINDS, "KINDS must list every PdfVtMutation variant, in declaration order");

        let manifest = include_str!("../../🧪️oracle/🔣️component.json");
        let needle = "\"kinds\": [";
        let start = manifest.find(needle).expect("manifest declares a kinds array") + needle.len();
        let end = start + manifest[start..].find(']').expect("kinds array is closed");
        let declared: Vec<String> = manifest[start..end].split(',').map(|entry| entry.trim().trim_matches('"').trim().trim_matches('"').to_string()).filter(|entry| !entry.is_empty()).collect();
        assert_eq!(declared, KINDS, "the oracle manifest's kinds must match PdfVtMutation exactly");
    }
    //#endregion 🔖️KindsConformanceLaw

    //#region 🔖️InverseLaw
    /// ⚖️ `apply(inverse(m), apply(m, base))` must recover `base` — proven at the Rust-model level on
    /// a real, hand-built object graph rather than asserted. The base carries a catalog, a page, a
    /// font descriptor with an embedded program and a Filespec, so every axis this vocabulary
    /// addresses has something real to move.
    #[test]
    fn mutation_apply_inverse_round_trips_every_variant() {
        let base = fixture();
        for mutation in exhaustive(&base) {
            let mut state = base.clone();
            apply_vt_conformance_mutation(&mut state, &mutation);
            for undo in mutation.inverse(&base) {
                let diff = undo.diff(&state);
                state = diff.diff().apply(&state).expect("the inverse diff applies");
            }
            assert_eq!(state, base, "apply(inverse(m), apply(m, base)) must recover base for {mutation:?}");
        }
    }
    //#endregion 🔖️InverseLaw

    //#region 🔖️StampLaw
    /// 🏅️ The class stamp is bijective on a document that carries none of the keys it installs —
    /// which is exactly the corpus this subset's case runs on — so `SetSnapshot` is exactly
    /// invertible on every axis the stamp touches.
    #[test]
    fn stamping_a_class_and_stripping_it_again_is_the_identity() {
        let base = fixture();
        assert_eq!(stamp_conformance(stamp_conformance(base.clone(), true), false), stamp_conformance(base, false));
    }
    //#endregion 🔖️StampLaw

    //#region 🔖️Fixture
    /// 🧫️ A small but real object graph: catalog, page tree, one page, one font descriptor with an
    /// embedded program, and one Filespec with an attached stream.
    fn fixture() -> PdfSnapshot {
        use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::{PdfDictEntry, PdfIndirectObject, PdfPage};
        let object = |num: u32, value: PdfObject| PdfIndirectObject { id: ObjRef { num, gen: 0 }, value };
        let entry = |key: &str, value: PdfObject| PdfDictEntry { key: key.to_string(), value };
        PdfSnapshot {
            schema: crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::STDIO_PDF17_DOCUMENT_SCHEMA.to_string(),
            declared_version: "1.7".to_string(),
            pages: vec![PdfPage::new(595.276, 841.89)],
            info: Default::default(),
            objects: vec![
                object(1, PdfObject::Dict(vec![entry("Type", PdfObject::Name("Catalog".to_string())), entry("Pages", PdfObject::Ref(ObjRef { num: 2, gen: 0 }))])),
                object(2, PdfObject::Dict(vec![entry("Type", PdfObject::Name("Pages".to_string())), entry("Kids", PdfObject::Array(vec![PdfObject::Ref(ObjRef { num: 3, gen: 0 })])), entry("Count", PdfObject::Int(1))])),
                object(3, PdfObject::Dict(vec![entry("Type", PdfObject::Name("Page".to_string())), entry("Parent", PdfObject::Ref(ObjRef { num: 2, gen: 0 }))])),
                object(4, PdfObject::Stream { dict: vec![entry("Length1", PdfObject::Int(12))], data: b"font program".to_vec(), filters: Vec::new() }),
                object(5, PdfObject::Dict(vec![entry("Type", PdfObject::Name("FontDescriptor".to_string())), entry("FontFile2", PdfObject::Ref(ObjRef { num: 4, gen: 0 }))])),
                object(6, PdfObject::Stream { dict: Vec::new(), data: b"attached payload".to_vec(), filters: Vec::new() }),
                object(
                    7,
                    PdfObject::Dict(vec![
                        entry("Type", PdfObject::Name("Filespec".to_string())),
                        entry("F", PdfObject::Str(b"measurements.csv".to_vec())),
                        entry("EF", PdfObject::Dict(vec![entry("F", PdfObject::Ref(ObjRef { num: 6, gen: 0 }))])),
                    ]),
                ),
            ],
            trailer: vec![entry("Root", PdfObject::Ref(ObjRef { num: 1, gen: 0 }))],
        }
    }

    /// 🧾️ One real instance of every variant, built against `base` so an ordinal or a reference it
    /// carries names something that is genuinely there.
    fn exhaustive(base: &PdfSnapshot) -> Vec<PdfVtMutation> {
        let program = support::font_descriptors(base).first().copied().and_then(|id| support::font_program(base, id)).map(|(_, id)| id).unwrap_or(ObjRef { num: 4, gen: 0 });
        let _ = program;
        vec![
            PdfVtMutation::NoMutation,
            PdfVtMutation::SetSnapshot { snapshot: stamp_conformance(base.clone(), true) },
            PdfVtMutation::InsertEncryptionDictionary { version: 2, revision: 3 },
            PdfVtMutation::RemoveEncryptionDictionary { version: 2, revision: 3 },
            PdfVtMutation::SetOutputIntent { identifier: "sRGB IEC61966-2.1".to_string() },
            PdfVtMutation::RemoveOutputIntent,
            PdfVtMutation::SetTrimBox { page_index: 0, trim_box: [8.5, 8.5, 586.776, 833.39] },
            PdfVtMutation::RemoveTrimBox { page_index: 0 },
            PdfVtMutation::EmbedFontFile { descriptor_ordinal: 0, key: "FontFile2".to_string(), program: program },
            PdfVtMutation::RemoveFontFile { descriptor_ordinal: 0 },
            PdfVtMutation::InsertJavaScriptAction { script: "app.alert('audit');".to_string() },
            PdfVtMutation::RemoveJavaScriptAction { script: "app.alert('audit');".to_string() },
            PdfVtMutation::InsertLaunchAction { target: "render-plots.bat".to_string() },
            PdfVtMutation::RemoveLaunchAction { target: "render-plots.bat".to_string() },
            PdfVtMutation::InsertMediaAnnotation { subtype: "Movie".to_string(), title: "site walkthrough".to_string() },
            PdfVtMutation::RemoveMediaAnnotation { subtype: "Movie".to_string(), title: "site walkthrough".to_string() },
            PdfVtMutation::SetDpartRoot { job: "run 4711".to_string() },
            PdfVtMutation::RemoveDpartRoot,
            PdfVtMutation::SetDpartMetadata { job: "run 4711".to_string() },
            PdfVtMutation::RemoveDpartMetadata,
        ]
    }
    //#endregion 🔖️Fixture
}
//#endregion 🧪️Tests
