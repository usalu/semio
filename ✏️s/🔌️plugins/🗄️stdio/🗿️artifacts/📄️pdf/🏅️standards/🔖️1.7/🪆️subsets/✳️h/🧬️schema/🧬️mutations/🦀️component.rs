//! 🧬️ `PdfHMutation` — the the PDF Healthcare Best Practices Guide (PDF/H) CONFORMANCE-CLASS vocabulary of `stdio.pdf` 1.7. Every
//! variant's `diff()` is handcrafted directly against `base` through a named transform, and every
//! variant's `inverse()` is handcrafted, reading whatever pre-state it needs out of the base.
//!
//! **Why this subset needs a vocabulary of its own.** `✳️any` owns the DOCUMENT vocabulary —
//! `insert-page`, `remove-page`, `move-page`, the media/crop box kinds, page content, `/Info` as
//! authoring metadata, and the raw object/dict/trailer edit primitives. Not one of those mutations
//! can move a document between conformance classes, because a conformance class is a property of the
//! retained object GRAPH and of no page at all. This enum is one variant per axis of this subset's
//! own `check_h_conformance` (`../../🦀️component.rs`), which reads five axes: a populated `Info.title` AND `Info.author`, any `/S /JavaScript` action or bare `/JS` key, any `/S /Launch` action, an `/AcroForm` field with `/FT /Sig`, and an embedded program on every font's `/FontDescriptor`.
//!
//! PDF/H is the one subset in this standard whose checker raises NOTHING harder than a warning — `check_h_conformance` calls `soft()` for every finding and never `hard()`, because the PDF Healthcare Best Practices Guide is guidance, not a normative ISO conformance class. Its vocabulary follows from that: it is the only one here with a SIGNATURE-FIELD pair (a signature flow is recommended, never required) and the only one that treats document metadata as a conformance axis at all, which is why `set-info-title` and `set-info-author` appear here and in no sibling but `✳️ua`, whose own title axis has a different reason. There is no encryption axis: the Guide's checker does not scan for `/Encrypt`, and inventing one to match PDF/A would be fabricating a rule this subset does not have.
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
/// carry a `/DestOutputProfile`. Both are read straight off `check_h_conformance`.
pub const OUTPUT_INTENT_SUBTYPE: &str = "GTS_PDFA1";
pub const OUTPUT_INTENT_DEST_PROFILE: bool = false;

/// 📇️ The metadata a class stamp writes when this subset polices document metadata at all.
pub const CONFORMANT_TITLE: &str = "A PDF/H conformant document";
pub const CONFORMANT_AUTHOR: &str = "semio stdio conformance stamp";
//#endregion 🔖️Class

//#region 🔖️Mutations
/// 📐️ Typed conformance-class mutation for `stdio.pdf` 1.7 under the PDF Healthcare Best Practices Guide (PDF/H). Every variant
/// addresses ONE axis of the class; none addresses page content.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum PdfHMutation {
    /// 🚫️ The identity element of the vocabulary.
    #[default]
    NoMutation,
    /// 🔄️ Replaces the whole document. A conformance class is a whole-graph property, so this is
    /// the class stamp in its total form — every axis at once. Build the target with
    /// [`stamp_conformance`].
    SetSnapshot {
        snapshot: PdfSnapshot,
    },
    /// 📇️ Sets the document `Info.title`. A conformance axis here, not authoring metadata: this
    /// subset's checker reads it and flags an absent-or-EMPTY value, which is why the inverse can
    /// legitimately restore an empty string rather than an absence.
    SetInfoTitle {
        title: String,
    },
    /// 📇️ Sets the document `Info.author`, on the same footing as [`Self::SetInfoTitle`].
    SetInfoAuthor {
        author: String,
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
    /// ✍️ Adds an `/AcroForm` field with `/FT /Sig` titled `name`.
    InsertSignatureField {
        name: String,
    },
    /// ✍️ Drops the `/FT /Sig` field titled `name`, and the whole `/AcroForm` with it when it was
    /// the last one.
    RemoveSignatureField {
        name: String,
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
}

/// 🧾️ Kebab-case spelling of every `PdfHMutation` variant, in declaration order — the exhaustive
/// mutation catalog `pdf-1-7-h` (`../../🧪️oracle/🔣️component.json`) is measured against this
/// exact list. `kinds_match_enum_and_catalog` proves it never drifts from either side.
pub const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-info-title", "set-info-author", "insert-javascript-action", "remove-javascript-action", "insert-launch-action", "remove-launch-action", "insert-signature-field", "remove-signature-field", "embed-font-file", "remove-font-file"];
//#endregion 🔖️Mutations

//#region 🔖️Stamp
/// 🏅️ Stamps every axis this subset OWNS into (or out of) its conformant state — the whole-document
/// target `SetSnapshot` carries. Only axes whose conformant state is the PRESENCE of something are
/// stamped: an axis whose conformant state is the ABSENCE of a forbidden construct (javaScriptActions, launchActions, fontPrograms) is
/// already conformant on a document that does not carry it, and adding one in order to remove it
/// again would be theatre rather than a stamp.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn stamp_conformance(base: PdfSnapshot, stamped: bool) -> PdfSnapshot {
    let mut next = base;
    if stamped {
            next.info.title = Some(CONFORMANT_TITLE.to_string());
            next.info.author = Some(CONFORMANT_AUTHOR.to_string());
            support::insert_signature_field(&mut next, "Signature1");
    } else {
            next.info.title = Some(String::new());
            next.info.author = Some(String::new());
            support::remove_signature_field(&mut next, "Signature1");
    }
    next
}
//#endregion 🔖️Stamp

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot` through its own diff — the diff is the single semantics
/// source, never a separate imperative apply path.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_h_conformance_mutation(snapshot: &mut PdfSnapshot, mutation: &PdfHMutation) -> protocol::MutationOutcome<PdfDiff> {
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
impl PdfHMutation {
    fn transform(mutation: &PdfHMutation, base: &PdfSnapshot) -> PdfSnapshot {
    let mut next = base.clone();
    match mutation {
            Self::NoMutation => {},
            Self::SetSnapshot { snapshot } => next = snapshot.clone(),
            Self::SetInfoTitle { title } => { next.info.title = Some(title.clone()); },
            Self::SetInfoAuthor { author } => { next.info.author = Some(author.clone()); },
            Self::InsertJavaScriptAction { script } => { support::insert_object(&mut next, support::action_object("JavaScript", "JS", script)); },
            Self::RemoveJavaScriptAction { script } => { if let Some(id) = support::action_with(&next, "JavaScript", "JS", script) { support::remove_object(&mut next, id); } },
            Self::InsertLaunchAction { target } => { support::insert_object(&mut next, support::action_object("Launch", "F", target)); },
            Self::RemoveLaunchAction { target } => { if let Some(id) = support::action_with(&next, "Launch", "F", target) { support::remove_object(&mut next, id); } },
            Self::InsertSignatureField { name } => { support::insert_signature_field(&mut next, name); },
            Self::RemoveSignatureField { name } => { support::remove_signature_field(&mut next, name); },
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
impl Mutation<PdfSnapshot> for PdfHMutation {
    type Diff = PdfDiff;

    fn diff(&self, base: &PdfSnapshot) -> protocol::MutationOutcome<Self::Diff> {
        protocol::MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &Self::transform(self, base)))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<Self> {
        vec![match self {
            Self::NoMutation => Self::NoMutation,
            Self::SetSnapshot { snapshot } => Self::SetSnapshot { snapshot: base.clone() },
            Self::SetInfoTitle { title } => Self::SetInfoTitle { title: base.info.title.clone().unwrap_or_default() },
            Self::SetInfoAuthor { author } => Self::SetInfoAuthor { author: base.info.author.clone().unwrap_or_default() },
            Self::InsertJavaScriptAction { script } => Self::RemoveJavaScriptAction { script: script.clone() },
            Self::RemoveJavaScriptAction { script } => Self::InsertJavaScriptAction { script: script.clone() },
            Self::InsertLaunchAction { target } => Self::RemoveLaunchAction { target: target.clone() },
            Self::RemoveLaunchAction { target } => Self::InsertLaunchAction { target: target.clone() },
            Self::InsertSignatureField { name } => Self::RemoveSignatureField { name: name.clone() },
            Self::RemoveSignatureField { name } => Self::InsertSignatureField { name: name.clone() },
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
    /// variant is added to `PdfHMutation` without a matching kebab-case spelling here, which is what
    /// keeps `KINDS` honest against the enum. The second half reads the sibling oracle manifest's
    /// `kinds` array as text (the framework never parses Rust, so this is the only side that can
    /// prove the manifest matches) and asserts the same list, in the same order.
    #[test]
    fn kinds_match_enum_and_catalog() {
        fn kind_of(mutation: &PdfHMutation) -> &'static str {
            match mutation {
                PdfHMutation::NoMutation => "no-mutation",
                PdfHMutation::SetSnapshot { .. } => "set-snapshot",
                PdfHMutation::SetInfoTitle { .. } => "set-info-title",
                PdfHMutation::SetInfoAuthor { .. } => "set-info-author",
                PdfHMutation::InsertJavaScriptAction { .. } => "insert-javascript-action",
                PdfHMutation::RemoveJavaScriptAction { .. } => "remove-javascript-action",
                PdfHMutation::InsertLaunchAction { .. } => "insert-launch-action",
                PdfHMutation::RemoveLaunchAction { .. } => "remove-launch-action",
                PdfHMutation::InsertSignatureField { .. } => "insert-signature-field",
                PdfHMutation::RemoveSignatureField { .. } => "remove-signature-field",
                PdfHMutation::EmbedFontFile { .. } => "embed-font-file",
                PdfHMutation::RemoveFontFile { .. } => "remove-font-file",
            }
        }
        let samples = [
            PdfHMutation::NoMutation,
            PdfHMutation::SetSnapshot { snapshot: PdfSnapshot::default() },
            PdfHMutation::SetInfoTitle { title: String::new() },
            PdfHMutation::SetInfoAuthor { author: String::new() },
            PdfHMutation::InsertJavaScriptAction { script: String::new() },
            PdfHMutation::RemoveJavaScriptAction { script: String::new() },
            PdfHMutation::InsertLaunchAction { target: String::new() },
            PdfHMutation::RemoveLaunchAction { target: String::new() },
            PdfHMutation::InsertSignatureField { name: String::new() },
            PdfHMutation::RemoveSignatureField { name: String::new() },
            PdfHMutation::EmbedFontFile { descriptor_ordinal: 0, key: String::new(), program: ObjRef::default() },
            PdfHMutation::RemoveFontFile { descriptor_ordinal: 0 },
        ];
        let from_enum: Vec<&'static str> = samples.iter().map(kind_of).collect();
        assert_eq!(from_enum, KINDS, "KINDS must list every PdfHMutation variant, in declaration order");

        let manifest = include_str!("../../🧪️oracle/🔣️component.json");
        let needle = "\"kinds\": [";
        let start = manifest.find(needle).expect("manifest declares a kinds array") + needle.len();
        let end = start + manifest[start..].find(']').expect("kinds array is closed");
        let declared: Vec<String> = manifest[start..end].split(',').map(|entry| entry.trim().trim_matches('"').trim().trim_matches('"').to_string()).filter(|entry| !entry.is_empty()).collect();
        assert_eq!(declared, KINDS, "the oracle manifest's kinds must match PdfHMutation exactly");
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
            apply_h_conformance_mutation(&mut state, &mutation);
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
    fn exhaustive(base: &PdfSnapshot) -> Vec<PdfHMutation> {
        let program = support::font_descriptors(base).first().copied().and_then(|id| support::font_program(base, id)).map(|(_, id)| id).unwrap_or(ObjRef { num: 4, gen: 0 });
        let _ = program;
        vec![
            PdfHMutation::NoMutation,
            PdfHMutation::SetSnapshot { snapshot: stamp_conformance(base.clone(), true) },
            PdfHMutation::SetInfoTitle { title: "Reuse of load-bearing timber components".to_string() },
            PdfHMutation::SetInfoAuthor { author: "a real author".to_string() },
            PdfHMutation::InsertJavaScriptAction { script: "app.alert('audit');".to_string() },
            PdfHMutation::RemoveJavaScriptAction { script: "app.alert('audit');".to_string() },
            PdfHMutation::InsertLaunchAction { target: "render-plots.bat".to_string() },
            PdfHMutation::RemoveLaunchAction { target: "render-plots.bat".to_string() },
            PdfHMutation::InsertSignatureField { name: "Signature1".to_string() },
            PdfHMutation::RemoveSignatureField { name: "Signature1".to_string() },
            PdfHMutation::EmbedFontFile { descriptor_ordinal: 0, key: "FontFile2".to_string(), program: program },
            PdfHMutation::RemoveFontFile { descriptor_ordinal: 0 },
        ]
    }
    //#endregion 🔖️Fixture
}
//#endregion 🧪️Tests
