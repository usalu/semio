//! 🧬️ `PdfUaMutation` — the ISO 14289-1 (PDF/UA-1) CONFORMANCE-CLASS vocabulary of `stdio.pdf` 1.7. Every
//! variant's `diff()` is handcrafted directly against `base` through a named transform, and every
//! variant's `inverse()` is handcrafted, reading whatever pre-state it needs out of the base.
//!
//! **Why this subset needs a vocabulary of its own.** `✳️any` owns the DOCUMENT vocabulary —
//! `insert-page`, `remove-page`, `move-page`, the media/crop box kinds, page content, `/Info` as
//! authoring metadata, and the raw object/dict/trailer edit primitives. Not one of those mutations
//! can move a document between conformance classes, because a conformance class is a property of the
//! retained object GRAPH and of no page at all. This enum is one variant per axis of this subset's
//! own `check_ua_conformance` (`../../🦀️component.rs`), which reads six axes, all of them keys of the document CATALOG or of `/Info`: `/Root/MarkInfo` with `/Marked true`, `/Root/StructTreeRoot`, a non-empty `/Root/Lang`, `/Root/ViewerPreferences` with `/DisplayDocTitle true`, a non-empty `Info.title`, and an embedded program on every font's `/FontDescriptor`.
//!
//! PDF/UA is the only conformance class in this standard that is about ACCESSIBILITY rather than about reproduction or archiving, and its vocabulary shows it: not one variant here addresses an object that could appear in a print or archival profile. `set-mark-info` and `set-struct-tree-root` are its two HARD axes — `check_ua_conformance` is the only checker in the sextet that calls `hard()` for a MISSING key rather than for a forbidden one — and `set-lang`/`set-display-doc-title` are its two soft ones. It shares `set-info-title` with `✳️h` and nothing else with anybody: no encryption axis, no action axes, no output intent, because `check_ua_conformance` reads none of them.
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
/// carry a `/DestOutputProfile`. Both are read straight off `check_ua_conformance`.
pub const OUTPUT_INTENT_SUBTYPE: &str = "GTS_PDFA1";
pub const OUTPUT_INTENT_DEST_PROFILE: bool = false;

/// 📇️ The metadata a class stamp writes when this subset polices document metadata at all.
pub const CONFORMANT_TITLE: &str = "A PDF/UA-1 conformant document";
pub const CONFORMANT_AUTHOR: &str = "semio stdio conformance stamp";
//#endregion 🔖️Class

//#region 🔖️Mutations
/// 📐️ Typed conformance-class mutation for `stdio.pdf` 1.7 under ISO 14289-1 (PDF/UA-1). Every variant
/// addresses ONE axis of the class; none addresses page content.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum PdfUaMutation {
    /// 🚫️ The identity element of the vocabulary.
    #[default]
    NoMutation,
    /// 🔄️ Replaces the whole document. A conformance class is a whole-graph property, so this is
    /// the class stamp in its total form — every axis at once. Build the target with
    /// [`stamp_conformance`].
    SetSnapshot {
        snapshot: PdfSnapshot,
    },
    /// 🏷️ Sets `/Root/MarkInfo << /Marked … >>` — one of PDF/UA's two HARD axes.
    SetMarkInfo {
        marked: bool,
    },
    /// 🏷️ Drops `/Root/MarkInfo`.
    RemoveMarkInfo,
    /// 🌲️ Installs `/Root/StructTreeRoot` over a real `/Type /StructTreeRoot` object — PDF/UA's
    /// other HARD axis.
    SetStructTreeRoot,
    /// 🌲️ Drops `/Root/StructTreeRoot`.
    RemoveStructTreeRoot,
    /// 🗣️ Sets `/Root/Lang` — the document language PDF/UA expects.
    SetLang {
        lang: String,
    },
    /// 🗣️ Drops `/Root/Lang`.
    RemoveLang,
    /// 🪧️ Sets `/Root/ViewerPreferences << /DisplayDocTitle … >>` — show the title, not the file name.
    SetDisplayDocTitle {
        display: bool,
    },
    /// 🪧️ Drops `/Root/ViewerPreferences`.
    RemoveDisplayDocTitle,
    /// 📇️ Sets the document `Info.title`. A conformance axis here, not authoring metadata: this
    /// subset's checker reads it and flags an absent-or-EMPTY value, which is why the inverse can
    /// legitimately restore an empty string rather than an absence.
    SetInfoTitle {
        title: String,
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

/// 🧾️ Kebab-case spelling of every `PdfUaMutation` variant, in declaration order — the exhaustive
/// mutation catalog `pdf-1-7-ua` (`../../🧪️oracle/🔣️component.json`) is measured against this
/// exact list. `kinds_match_enum_and_catalog` proves it never drifts from either side.
pub const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-mark-info", "remove-mark-info", "set-struct-tree-root", "remove-struct-tree-root", "set-lang", "remove-lang", "set-display-doc-title", "remove-display-doc-title", "set-info-title", "embed-font-file", "remove-font-file"];
//#endregion 🔖️Mutations

//#region 🔖️Stamp
/// 🏅️ Stamps every axis this subset OWNS into (or out of) its conformant state — the whole-document
/// target `SetSnapshot` carries. Only axes whose conformant state is the PRESENCE of something are
/// stamped: an axis whose conformant state is the ABSENCE of a forbidden construct (fontPrograms) is
/// already conformant on a document that does not carry it, and adding one in order to remove it
/// again would be theatre rather than a stamp.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn stamp_conformance(base: PdfSnapshot, stamped: bool) -> PdfSnapshot {
    let mut next = base;
    if stamped {
            support::set_catalog_entry(&mut next, "MarkInfo", support::single_entry_dict("Marked", PdfObject::Bool(true)));
            let root = support::insert_object(&mut next, support::struct_tree_root_object());
            support::set_catalog_entry(&mut next, "StructTreeRoot", PdfObject::Ref(root));
            support::set_catalog_entry(&mut next, "Lang", support::literal("en-GB"));
            support::set_catalog_entry(&mut next, "ViewerPreferences", support::single_entry_dict("DisplayDocTitle", PdfObject::Bool(true)));
            next.info.title = Some(CONFORMANT_TITLE.to_string());
    } else {
            support::remove_catalog_entry(&mut next, "MarkInfo");
            support::remove_catalog_entry(&mut next, "StructTreeRoot");
            support::remove_catalog_entry(&mut next, "Lang");
            support::remove_catalog_entry(&mut next, "ViewerPreferences");
            next.info.title = Some(String::new());
    }
    next
}
//#endregion 🔖️Stamp

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot` through its own diff — the diff is the single semantics
/// source, never a separate imperative apply path.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_ua_conformance_mutation(snapshot: &mut PdfSnapshot, mutation: &PdfUaMutation) -> protocol::MutationOutcome<PdfDiff> {
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
fn transform(mutation: &PdfUaMutation, base: &PdfSnapshot) -> PdfSnapshot {
    let mut next = base.clone();
    match mutation {
            Self::NoMutation => {},
            Self::SetSnapshot { snapshot } => next = snapshot.clone(),
            Self::SetMarkInfo { marked } => { support::set_catalog_entry(&mut next, "MarkInfo", support::single_entry_dict("Marked", PdfObject::Bool(*marked))); },
            Self::RemoveMarkInfo => { support::remove_catalog_entry(&mut next, "MarkInfo"); },
            Self::SetStructTreeRoot => { let id = support::insert_object(&mut next, support::struct_tree_root_object()); support::set_catalog_entry(&mut next, "StructTreeRoot", PdfObject::Ref(id)); },
            Self::RemoveStructTreeRoot => { support::remove_catalog_entry(&mut next, "StructTreeRoot"); },
            Self::SetLang { lang } => { support::set_catalog_entry(&mut next, "Lang", support::literal(lang)); },
            Self::RemoveLang => { support::remove_catalog_entry(&mut next, "Lang"); },
            Self::SetDisplayDocTitle { display } => { support::set_catalog_entry(&mut next, "ViewerPreferences", support::single_entry_dict("DisplayDocTitle", PdfObject::Bool(*display))); },
            Self::RemoveDisplayDocTitle => { support::remove_catalog_entry(&mut next, "ViewerPreferences"); },
            Self::SetInfoTitle { title } => { next.info.title = Some(title.clone()); },
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
//#endregion 🔖️Transform

//#region 🔖️MutationTrait
impl Mutation<PdfSnapshot> for PdfUaMutation {
    type Diff = PdfDiff;

    fn diff(&self, base: &PdfSnapshot) -> protocol::MutationOutcome<Self::Diff> {
        protocol::MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &transform(self, base)))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<Self> {
        vec![match self {
            Self::NoMutation => Self::NoMutation,
            Self::SetSnapshot { snapshot } => Self::SetSnapshot { snapshot: base.clone() },
            Self::SetMarkInfo { marked } => match support::catalog_flag(base, "MarkInfo", "Marked") {
                Some(previous) => Self::SetMarkInfo { marked: previous },
                None => Self::RemoveMarkInfo,
            },
            Self::RemoveMarkInfo => match support::catalog_flag(base, "MarkInfo", "Marked") {
                Some(previous) => Self::SetMarkInfo { marked: previous },
                None => Self::NoMutation,
            },
            Self::SetStructTreeRoot => Self::RemoveStructTreeRoot,
            Self::RemoveStructTreeRoot => Self::SetStructTreeRoot,
            Self::SetLang { lang } => match support::catalog_entry(base, "Lang").and_then(|value| match value { PdfObject::Str(bytes) => Some(String::from_utf8_lossy(bytes).into_owned()), _ => None }) {
                Some(previous) => Self::SetLang { lang: previous },
                None => Self::RemoveLang,
            },
            Self::RemoveLang => match support::catalog_entry(base, "Lang").and_then(|value| match value { PdfObject::Str(bytes) => Some(String::from_utf8_lossy(bytes).into_owned()), _ => None }) {
                Some(previous) => Self::SetLang { lang: previous },
                None => Self::NoMutation,
            },
            Self::SetDisplayDocTitle { display } => match support::catalog_flag(base, "ViewerPreferences", "DisplayDocTitle") {
                Some(previous) => Self::SetDisplayDocTitle { display: previous },
                None => Self::RemoveDisplayDocTitle,
            },
            Self::RemoveDisplayDocTitle => match support::catalog_flag(base, "ViewerPreferences", "DisplayDocTitle") {
                Some(previous) => Self::SetDisplayDocTitle { display: previous },
                None => Self::NoMutation,
            },
            Self::SetInfoTitle { title } => Self::SetInfoTitle { title: base.info.title.clone().unwrap_or_default() },
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
    /// variant is added to `PdfUaMutation` without a matching kebab-case spelling here, which is what
    /// keeps `KINDS` honest against the enum. The second half reads the sibling oracle manifest's
    /// `kinds` array as text (the framework never parses Rust, so this is the only side that can
    /// prove the manifest matches) and asserts the same list, in the same order.
    #[test]
    fn kinds_match_enum_and_catalog() {
        fn kind_of(mutation: &PdfUaMutation) -> &'static str {
            match mutation {
                PdfUaMutation::NoMutation => "no-mutation",
                PdfUaMutation::SetSnapshot { .. } => "set-snapshot",
                PdfUaMutation::SetMarkInfo { .. } => "set-mark-info",
                PdfUaMutation::RemoveMarkInfo => "remove-mark-info",
                PdfUaMutation::SetStructTreeRoot => "set-struct-tree-root",
                PdfUaMutation::RemoveStructTreeRoot => "remove-struct-tree-root",
                PdfUaMutation::SetLang { .. } => "set-lang",
                PdfUaMutation::RemoveLang => "remove-lang",
                PdfUaMutation::SetDisplayDocTitle { .. } => "set-display-doc-title",
                PdfUaMutation::RemoveDisplayDocTitle => "remove-display-doc-title",
                PdfUaMutation::SetInfoTitle { .. } => "set-info-title",
                PdfUaMutation::EmbedFontFile { .. } => "embed-font-file",
                PdfUaMutation::RemoveFontFile { .. } => "remove-font-file",
            }
        }
        let samples = [
            PdfUaMutation::NoMutation,
            PdfUaMutation::SetSnapshot { snapshot: PdfSnapshot::default() },
            PdfUaMutation::SetMarkInfo { marked: true },
            PdfUaMutation::RemoveMarkInfo,
            PdfUaMutation::SetStructTreeRoot,
            PdfUaMutation::RemoveStructTreeRoot,
            PdfUaMutation::SetLang { lang: String::new() },
            PdfUaMutation::RemoveLang,
            PdfUaMutation::SetDisplayDocTitle { display: true },
            PdfUaMutation::RemoveDisplayDocTitle,
            PdfUaMutation::SetInfoTitle { title: String::new() },
            PdfUaMutation::EmbedFontFile { descriptor_ordinal: 0, key: String::new(), program: ObjRef::default() },
            PdfUaMutation::RemoveFontFile { descriptor_ordinal: 0 },
        ];
        let from_enum: Vec<&'static str> = samples.iter().map(kind_of).collect();
        assert_eq!(from_enum, KINDS, "KINDS must list every PdfUaMutation variant, in declaration order");

        let manifest = include_str!("../../🧪️oracle/🔣️component.json");
        let needle = "\"kinds\": [";
        let start = manifest.find(needle).expect("manifest declares a kinds array") + needle.len();
        let end = start + manifest[start..].find(']').expect("kinds array is closed");
        let declared: Vec<String> = manifest[start..end].split(',').map(|entry| entry.trim().trim_matches('"').trim().trim_matches('"').to_string()).filter(|entry| !entry.is_empty()).collect();
        assert_eq!(declared, KINDS, "the oracle manifest's kinds must match PdfUaMutation exactly");
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
            apply_ua_conformance_mutation(&mut state, &mutation);
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
    fn exhaustive(base: &PdfSnapshot) -> Vec<PdfUaMutation> {
        let program = support::font_descriptors(base).first().copied().and_then(|id| support::font_program(base, id)).map(|(_, id)| id).unwrap_or(ObjRef { num: 4, gen: 0 });
        let _ = program;
        vec![
            PdfUaMutation::NoMutation,
            PdfUaMutation::SetSnapshot { snapshot: stamp_conformance(base.clone(), true) },
            PdfUaMutation::SetMarkInfo { marked: true },
            PdfUaMutation::RemoveMarkInfo,
            PdfUaMutation::SetStructTreeRoot,
            PdfUaMutation::RemoveStructTreeRoot,
            PdfUaMutation::SetLang { lang: "en-GB".to_string() },
            PdfUaMutation::RemoveLang,
            PdfUaMutation::SetDisplayDocTitle { display: true },
            PdfUaMutation::RemoveDisplayDocTitle,
            PdfUaMutation::SetInfoTitle { title: "Reuse of load-bearing timber components".to_string() },
            PdfUaMutation::EmbedFontFile { descriptor_ordinal: 0, key: "FontFile2".to_string(), program: program },
            PdfUaMutation::RemoveFontFile { descriptor_ordinal: 0 },
        ]
    }
    //#endregion 🔖️Fixture
}
//#endregion 🧪️Tests
