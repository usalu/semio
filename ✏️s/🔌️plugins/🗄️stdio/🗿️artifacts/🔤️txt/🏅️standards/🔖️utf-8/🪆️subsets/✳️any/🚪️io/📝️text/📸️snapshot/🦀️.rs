//! 📝️ Text representation codec surface for `stdio.txt` (snapshot).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type TxtSnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v_utf_8::subsets::any::schema::snapshot::*;
use crate::STDIO_TXT_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;

/// 🧬️ CARRIER LAW (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3):
/// `s.stdio.txt@utf-8/*` is `CARRIER_TEXT` — its native `Text` `IoPayload` IS the raw external
/// file text, verbatim. The previous impl prepended a `semio stdio.txt.dsl v1` preamble line
/// (`wrap_text`) and stripped it back off on parse, which made every exported `.txt` file carry
/// a foreign header line instead of the honest raw text. Fixed here (the codec, not the test):
/// `parse_dsl`/`print_dsl` are now `TxtSnapshot::from_body`/`to_body` directly, no preamble.
/// Proven by `carrier_native_is_raw` in `🚪️io/🦀️.rs`.
impl store::ArtifactDsl for TxtSnapshot {
    const EXTENSION: &'static str = "txt";
    fn envelope_id() -> &'static str {
        "stdio.txt"
    }

    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        Ok(Self::from_body(text))
    }
    fn print_dsl(&self) -> String {
        self.to_body()
    }
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::standards::v_utf_8::subsets::any::schema::*;
use crate::schema::snapshot::LineEnding;
use crate::TxtSnapshot;
use framework_schema::ArtifactSchema;

/// 🌱 Empty persisted snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn empty_txt_snapshot() -> TxtSnapshot {
    TxtSnapshot::default()
}

/// 📄️ The `demo` example, parsed once from `examples::demo::PRIMARY_TEXT` — the single source
/// of truth `🗣️.dsl.semio` is genuinely `print_dsl` of (P2-P3 `fixture_honesty_law`),
/// same pattern as `note::semio_example_snapshot`/`csv::demo_csv_snapshot`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_txt_snapshot() -> TxtSnapshot {
    <TxtSnapshot as store::ArtifactDsl>::parse_dsl(crate::examples::demo::PRIMARY_TEXT).unwrap_or_else(|_| empty_txt_snapshot())
}
}
pub use snapshot_wire_codec::*;
