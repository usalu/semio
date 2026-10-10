//! 🧬️ CsvSnapshot schema — persistent fields + the real RFC4180 codec (dissolved out of the
//! former `⚙️engine`, ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES — kept beside the
//! `ArtifactDsl`/`ArtifactPack` impls that call it directly, mirroring `json`'s own already-
//! established `parse_json_text`/`write_json_text` placement in its `📸️snapshot/🦀️.rs`).

use crate::STDIO_CSV_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;




fn default_true() -> bool {
    true
}

//#region 🔖️Field
/// 🔤 One RFC 4180 field value plus whether the source quoted it — rfc4180's own optional
/// quoting means whether a field WAS quoted is real information worth preserving losslessly,
/// so re-serializing can reproduce the exact source bytes rather than a lossy normal form
/// (https://www.rfc-editor.org/rfc/rfc4180#section-2, rule 5).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct CsvField {
    #[value(default)]
    pub value: String,
    #[value(default)]
    pub quoted: bool,
}
//#endregion 🔖️Field

//#region 🔖️Record
/// 📄 One RFC 4180 record (row) — a strong-like entity, index-keyed within
/// `CsvSnapshot::records`. Field COUNT is real, per-record information (rfc4180 is a
/// loosely-typed grid on the wire even though most producers keep it rectangular).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct CsvRecord {
    #[value(default)]
    pub fields: Vec<CsvField>,
}
//#endregion 🔖️Record

//#region 🔖️Snapshot
/// 📸️ Persisted `stdio.csv` snapshot (RFC 4180 table, with a header-row option). The
/// header row (when present) is `records[0]` — RFC 4180 draws no structural distinction
/// between a header record and a data record, only a convention of which one comes first.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.csv")]
pub struct CsvSnapshot {
    #[state(artifact)]
    pub schema: String,
    /// 📑 Whether the first record is a header row (RFC 4180's own optional convention).
    #[state(artifact)]
    #[value(default = "default_true")]
    pub has_header: bool,
    #[state(artifact)]
    #[value(default)]
    pub records: Vec<CsvRecord>,
}

impl Default for CsvSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_CSV_DOCUMENT_SCHEMA.into(), has_header: true, records: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️Codec
//#region 🔖️Tokenizer





//#endregion 🔖️Tokenizer

//#region 🔖️SnapshotCodec







//#endregion 🔖️SnapshotCodec
//#endregion 🔖️Codec

//#region 🔖️DocumentHelpers



//#endregion 🔖️DocumentHelpers

//#region 🔖️HandcraftedArtifactCodecs





//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
