//! 🧬️ `PdfEMutation` — the ISO 24517-1:2008 (PDF/E-1) CONFORMANCE-CLASS vocabulary of `stdio.pdf` 1.7. Every
//! variant's `diff()` is handcrafted directly against `base` through a named transform, and every
//! variant's `inverse()` is handcrafted, reading whatever pre-state it needs out of the base.
//!
//! **Why this subset needs a vocabulary of its own.** `✳️any` owns the DOCUMENT vocabulary —
//! `insert-page`, `remove-page`, `move-page`, the media/crop box kinds, page content, `/Info` as
//! authoring metadata, and the raw object/dict/trailer edit primitives. Not one of those mutations
//! can move a document between conformance classes, because a conformance class is a property of the
//! retained object GRAPH and of no page at all. This enum is one variant per axis of this subset's
//! own `check_e_conformance` (`../../🦀️component.rs`), which reads six axes: any Standard Security Handler `/Encrypt` dictionary object, any `/S /JavaScript` action or bare `/JS` key, any `/S /Launch` action, any `/Subtype /Movie` or `/Subtype /Sound` annotation, a NON-EMPTY `/Root/OutputIntents` array whatever its `/S` marker, and an embedded program on every font's `/FontDescriptor`.
//!
//! Two things separate this vocabulary from PDF/A's and PDF/X's. First, the MEDIA-ANNOTATION pair: `check_e_conformance`'s `movie_or_sound_annotations` matches `/Subtype /Movie` and `/Subtype /Sound` and deliberately never matches `/Subtype /3D`, which ISO 24517-1 explicitly ALLOWS — an engineering-document format that forbids video but permits embedded 3D geometry. Second, the OutputIntent axis is satisfied by ANY intent (`has_any_output_intent` tests only that the array is non-empty), unlike PDF/A's `/GTS_PDFA1` and PDF/X's `/GTS_PDFX` + `/DestOutputProfile`. The engine still has to write SOME `/S` marker, and it writes `/GTS_PDFX` — one of the two subtypes ISO 32000-1 Table 365 actually registers — rather than inventing a PDF/E name the standard does not define.
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
/// carry a `/DestOutputProfile`. Both are read straight off `check_e_conformance`.
pub const OUTPUT_INTENT_SUBTYPE: &str = "GTS_PDFX";
pub const OUTPUT_INTENT_DEST_PROFILE: bool = false;

/// 📇️ The metadata a class stamp writes when this subset polices document metadata at all.
pub const CONFORMANT_TITLE: &str = "A PDF/E-1 conformant document";
pub const CONFORMANT_AUTHOR: &str = "semio stdio conformance stamp";
//#endregion 🔖️Class

//#region 🔖️Mutations
/// 📐️ Typed conformance-class mutation for `stdio.pdf` 1.7 under ISO 24517-1:2008 (PDF/E-1). Every variant
/// addresses ONE axis of the class; none addresses page content.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum PdfEMutation {
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
    /// 🏳️ Installs `/Root/OutputIntents` with one intent carrying this subset's own marker.
    SetOutputIntent {
        identifier: String,
    },
    /// 🏳️ Drops `/Root/OutputIntents` entirely.
    RemoveOutputIntent,
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
}

/// 🧾️ Kebab-case spelling of every `PdfEMutation` variant, in declaration order — the exhaustive
/// mutation catalog `pdf-1-7-e` (`../../🧪️oracle/🔣️component.json`) is measured against this
/// exact list. `kinds_match_enum_and_catalog` proves it never drifts from either side.
pub const KINDS: &[&str] = &["no-mutation", "set-snapshot", "insert-encryption-dictionary", "remove-encryption-dictionary", "insert-javascript-action", "remove-javascript-action", "insert-launch-action", "remove-launch-action", "insert-media-annotation", "remove-media-annotation", "set-output-intent", "remove-output-intent", "embed-font-file", "remove-font-file"];
//#endregion 🔖️Mutations

//#region 🔖️Stamp
/// 🏅️ Stamps every axis this subset OWNS into (or out of) its conformant state — the whole-document
/// target `SetSnapshot` carries. Only axes whose conformant state is the PRESENCE of something are
/// stamped: an axis whose conformant state is the ABSENCE of a forbidden construct (encryptionDictionaries, javaScriptActions, launchActions, mediaAnnotations, fontPrograms) is
/// already conformant on a document that does not carry it, and adding one in order to remove it
/// again would be theatre rather than a stamp.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn stamp_conformance(base: PdfSnapshot, stamped: bool) -> PdfSnapshot {
    let mut next = base;
    if stamped {
            support::set_output_intent(&mut next, OUTPUT_INTENT_SUBTYPE, "sRGB IEC61966-2.1", OUTPUT_INTENT_DEST_PROFILE);
    } else {
            support::remove_catalog_entry(&mut next, "OutputIntents");
    }
    next
}
//#endregion 🔖️Stamp

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot` through its own diff — the diff is the single semantics
/// source, never a separate imperative apply path.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_e_conformance_mutation(snapshot: &mut PdfSnapshot, mutation: &PdfEMutation) -> protocol::MutationOutcome<PdfDiff> {
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
impl PdfEMutation {
    fn transform(mutation: &PdfEMutation, base: &PdfSnapshot) -> PdfSnapshot {
    let mut next = base.clone();
    match mutation {
            Self::NoMutation => {},
            Self::SetSnapshot { snapshot } => next = snapshot.clone(),
            Self::InsertEncryptionDictionary { version, revision } => { support::insert_object(&mut next, support::encryption_dictionary(*version, *revision)); },
            Self::RemoveEncryptionDictionary { version, revision } => { if let Some(id) = support::encryption_dictionary_with(&next, *version, *revision) { support::remove_object(&mut next, id); } },
            Self::InsertJavaScriptAction { script } => { support::insert_object(&mut next, support::action_object("JavaScript", "JS", script)); },
            Self::RemoveJavaScriptAction { script } => { if let Some(id) = support::action_with(&next, "JavaScript", "JS", script) { support::remove_object(&mut next, id); } },
            Self::InsertLaunchAction { target } => { support::insert_object(&mut next, support::action_object("Launch", "F", target)); },
            Self::RemoveLaunchAction { target } => { if let Some(id) = support::action_with(&next, "Launch", "F", target) { support::remove_object(&mut next, id); } },
            Self::InsertMediaAnnotation { subtype, title } => { support::insert_object(&mut next, support::media_annotation_object(subtype, title)); },
            Self::RemoveMediaAnnotation { subtype, title } => { if let Some(id) = support::media_annotation(&next, subtype, title) { support::remove_object(&mut next, id); } },
            Self::SetOutputIntent { identifier } => { support::set_output_intent(&mut next, OUTPUT_INTENT_SUBTYPE, identifier, OUTPUT_INTENT_DEST_PROFILE); },
            Self::RemoveOutputIntent => { support::remove_catalog_entry(&mut next, "OutputIntents"); },
            Self::EmbedFontFile { descriptor_ordinal, key, program } => { if let Some(id) = support::font_descriptors(&next).get(*descriptor_ordinal).copied() { support::set_entry(&mut next, id, key, PdfObject::Ref(*program)); } },
            Self::RemoveFontFile { descriptor_ordinal } => {
                if let Some(id) = support::font_descriptors(&next).get(*descriptor_ordinal).copied() {
                    if let Some((key, _)) = support::font_program(&next, id) {
                        support::remove_entry(&mut next, id, &key);
                    }
                }
            },
    }
    next
    }
}
//#endregion 🔖️Transform

//#region 🔖️MutationTrait
impl Mutation<PdfSnapshot> for PdfEMutation {
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
            Self::InsertJavaScriptAction { script } => Self::RemoveJavaScriptAction { script: script.clone() },
            Self::RemoveJavaScriptAction { script } => Self::InsertJavaScriptAction { script: script.clone() },
            Self::InsertLaunchAction { target } => Self::RemoveLaunchAction { target: target.clone() },
            Self::RemoveLaunchAction { target } => Self::InsertLaunchAction { target: target.clone() },
            Self::InsertMediaAnnotation { subtype, title } => Self::RemoveMediaAnnotation { subtype: subtype.clone(), title: title.clone() },
            Self::RemoveMediaAnnotation { subtype, title } => Self::InsertMediaAnnotation { subtype: subtype.clone(), title: title.clone() },
            Self::SetOutputIntent { identifier } => Self::RemoveOutputIntent,
            Self::RemoveOutputIntent => match support::output_intent_identifier(base) {
                Some(identifier) => Self::SetOutputIntent { identifier },
                None => Self::NoMutation,
            },
            Self::EmbedFontFile { descriptor_ordinal, key, program } => Self::RemoveFontFile { descriptor_ordinal: *descriptor_ordinal },
            Self::RemoveFontFile { descriptor_ordinal } => match support::font_descriptors(base).get(*descriptor_ordinal).copied().and_then(|id| support::font_program(base, id)) {
                Some((key, program)) => Self::EmbedFontFile { descriptor_ordinal: *descriptor_ordinal, key, program },
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
    /// variant is added to `PdfEMutation` without a matching kebab-case spelling here, which is what
    /// keeps `KINDS` honest against the enum. The second half reads the sibling oracle manifest's
    /// `kinds` array as text (the framework never parses Rust, so this is the only side that can
    /// prove the manifest matches) and asserts the same list, in the same order.
    #[test]
    fn kinds_match_enum_and_catalog() {
        fn kind_of(mutation: &PdfEMutation) -> &'static str {
            match mutation {
                PdfEMutation::NoMutation => "no-mutation",
                PdfEMutation::SetSnapshot { .. } => "set-snapshot",
                PdfEMutation::InsertEncryptionDictionary { .. } => "insert-encryption-dictionary",
                PdfEMutation::RemoveEncryptionDictionary { .. } => "remove-encryption-dictionary",
                PdfEMutation::InsertJavaScriptAction { .. } => "insert-javascript-action",
                PdfEMutation::RemoveJavaScriptAction { .. } => "remove-javascript-action",
                PdfEMutation::InsertLaunchAction { .. } => "insert-launch-action",
                PdfEMutation::RemoveLaunchAction { .. } => "remove-launch-action",
                PdfEMutation::InsertMediaAnnotation { .. } => "insert-media-annotation",
                PdfEMutation::RemoveMediaAnnotation { .. } => "remove-media-annotation",
                PdfEMutation::SetOutputIntent { .. } => "set-output-intent",
                PdfEMutation::RemoveOutputIntent => "remove-output-intent",
                PdfEMutation::EmbedFontFile { .. } => "embed-font-file",
                PdfEMutation::RemoveFontFile { .. } => "remove-font-file",
            }
        }
        let samples = [
            PdfEMutation::NoMutation,
            PdfEMutation::SetSnapshot { snapshot: PdfSnapshot::default() },
            PdfEMutation::InsertEncryptionDictionary { version: 2, revision: 3 },
            PdfEMutation::RemoveEncryptionDictionary { version: 2, revision: 3 },
            PdfEMutation::InsertJavaScriptAction { script: String::new() },
            PdfEMutation::RemoveJavaScriptAction { script: String::new() },
            PdfEMutation::InsertLaunchAction { target: String::new() },
            PdfEMutation::RemoveLaunchAction { target: String::new() },
            PdfEMutation::InsertMediaAnnotation { subtype: String::new(), title: String::new() },
            PdfEMutation::RemoveMediaAnnotation { subtype: String::new(), title: String::new() },
            PdfEMutation::SetOutputIntent { identifier: String::new() },
            PdfEMutation::RemoveOutputIntent,
            PdfEMutation::EmbedFontFile { descriptor_ordinal: 0, key: String::new(), program: ObjRef::default() },
            PdfEMutation::RemoveFontFile { descriptor_ordinal: 0 },
        ];
        let from_enum: Vec<&'static str> = samples.iter().map(kind_of).collect();
        assert_eq!(from_enum, KINDS, "KINDS must list every PdfEMutation variant, in declaration order");

        let manifest = include_str!("../../🧪️oracle/🔣️component.json");
        let needle = "\"kinds\": [";
        let start = manifest.find(needle).expect("manifest declares a kinds array") + needle.len();
        let end = start + manifest[start..].find(']').expect("kinds array is closed");
        let declared: Vec<String> = manifest[start..end].split(',').map(|entry| entry.trim().trim_matches('"').trim().trim_matches('"').to_string()).filter(|entry| !entry.is_empty()).collect();
        assert_eq!(declared, KINDS, "the oracle manifest's kinds must match PdfEMutation exactly");
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
            apply_e_conformance_mutation(&mut state, &mutation);
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
    fn exhaustive(base: &PdfSnapshot) -> Vec<PdfEMutation> {
        let program = support::font_descriptors(base).first().copied().and_then(|id| support::font_program(base, id)).map(|(_, id)| id).unwrap_or(ObjRef { num: 4, gen: 0 });
        let _ = program;
        vec![
            PdfEMutation::NoMutation,
            PdfEMutation::SetSnapshot { snapshot: stamp_conformance(base.clone(), true) },
            PdfEMutation::InsertEncryptionDictionary { version: 2, revision: 3 },
            PdfEMutation::RemoveEncryptionDictionary { version: 2, revision: 3 },
            PdfEMutation::InsertJavaScriptAction { script: "app.alert('audit');".to_string() },
            PdfEMutation::RemoveJavaScriptAction { script: "app.alert('audit');".to_string() },
            PdfEMutation::InsertLaunchAction { target: "render-plots.bat".to_string() },
            PdfEMutation::RemoveLaunchAction { target: "render-plots.bat".to_string() },
            PdfEMutation::InsertMediaAnnotation { subtype: "Movie".to_string(), title: "site walkthrough".to_string() },
            PdfEMutation::RemoveMediaAnnotation { subtype: "Movie".to_string(), title: "site walkthrough".to_string() },
            PdfEMutation::SetOutputIntent { identifier: "sRGB IEC61966-2.1".to_string() },
            PdfEMutation::RemoveOutputIntent,
            PdfEMutation::EmbedFontFile { descriptor_ordinal: 0, key: "FontFile2".to_string(), program: program },
            PdfEMutation::RemoveFontFile { descriptor_ordinal: 0 },
        ]
    }
    //#endregion 🔖️Fixture
}
//#endregion 🧪️Tests
