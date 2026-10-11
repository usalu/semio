//! 🧬️ SemioTextSnapshot — the neutral interchange shape for plain and inline-marked text: a
//! sequence of language-tagged runs, each carrying content plus inline marks (bold/italic/code/
//! link). LEAF subset (no child slots, no link slots) per the master plan's stdio target
//! vocabulary — `document`/`drawing`/`presentation`/etc. compose this for their textual content
//! (a later wave). Absorbs the duplicated `LocalizedText` types that currently exist twice inside
//! the norm plugin (ticket UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM, W2a/text).
//!
//! Modeled on `🖼️image`'s hand-rolled `ArtifactDsl`/`ArtifactPack` convention (real hex/bracket
//! text codec + real varint-length-prefixed binary codec, both wrapped in the shared
//! `store::semio_format` envelope) — `📑️document`'s `DocRun`/`DocStyle` is the structural cousin
//! (run + inline marks), but `text` owns runs standalone rather than nested inside block
//! structure, per this ticket's brief.



use framework_schema::ArtifactSchema;

//#region 🔖️Ids
/// 🏷️ Document schema / DSL envelope id AND `ArtifactSchema` descriptor id — same literal for
/// both, per the master plan's "Schema descriptor ids `s.stdio.semio` + `s.stdio.semio.<subset>`"
/// convention, one per subset.
pub const STDIO_SEMIOTEXT_DOCUMENT_SCHEMA: &str = "s.stdio.semio.text";
//#endregion 🔖️Ids

//#region 🔖️MarkKind
/// 🖊️ The closed inline-mark vocabulary this leaf carries — bold/italic/code (flag-only) and link
/// (carries an `href`). `href` on a non-`Link` mark is always the empty string.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, Default, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum SemioTextMarkKind {
    #[default]
    Bold,
    Italic,
    Code,
    Link,
}
//#endregion 🔖️MarkKind

//#region 🔖️Mark
/// 🔖️ One inline mark applied to a run. Strong entity, index-addressed within its owning run's
/// `marks` (an intrinsically ordered, anonymous collection — see `➕add-mark`/`➖remove-mark`).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, Default, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct SemioTextMark {
    pub kind: SemioTextMarkKind,
    /// 🔗️ Populated only when `kind == Link`; empty string otherwise.
    #[value(default)]
    pub href: String,
}
//#endregion 🔖️Mark

//#region 🔖️Run
/// 🏃️ One run of text: a BCP-47 `language` tag (`""` = unspecified, inherits from context), the
/// authored `content`, and its ordered `marks`. Runs themselves are index-addressed (no stable
/// id — an intrinsically ordered, anonymous collection, `📓️taxonomy.md` addressing rule #3), the
/// same shape `insert-run`/`remove-run`/`reorder-runs` operate on.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, Default, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct SemioTextRun {
    #[value(default)]
    pub language: String,
    #[value(default)]
    pub content: String,
    #[value(default)]
    pub marks: Vec<SemioTextMark>,
}
//#endregion 🔖️Run

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.text")]
pub struct SemioTextSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub runs: Vec<SemioTextRun>,
}

impl Default for SemioTextSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_SEMIOTEXT_DOCUMENT_SCHEMA.into(), runs: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️TextPrimitives

















//#endregion 🔖️TextPrimitives

//#region 🔖️BinaryPrimitives














//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🌉️ExternalCodecBridge



//#endregion 🌉️ExternalCodecBridge

//#region 🔖️Wire







//#endregion 🔖️Wire

//#region 🔖️Demo
/// 🌱 The demo `s.stdio.semio.text` document — three runs (plain, bold, and a link mark) across
/// two languages, exercising every leaf/collection shape at least once. Single source of truth for
/// `📚️examples/…/🖼️assets/🗣️.dsl.semio`/`🎒️.pack.semio` and for the conformance-law
/// tests in `🚪️io/🦀️.rs`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_text_snapshot() -> SemioTextSnapshot {
    SemioTextSnapshot {
        schema: STDIO_SEMIOTEXT_DOCUMENT_SCHEMA.into(),
        runs: vec![
            SemioTextRun { language: "en".into(), content: "Hello, ".into(), marks: vec![] },
            SemioTextRun { language: "en".into(), content: "world".into(), marks: vec![SemioTextMark { kind: SemioTextMarkKind::Bold, href: String::new() }] },
            SemioTextRun { language: "de".into(), content: "semio.tech".into(), marks: vec![SemioTextMark { kind: SemioTextMarkKind::Link, href: "https://semio.tech".into() }] },
        ],
    }
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;




//#endregion 🔖️Tests


