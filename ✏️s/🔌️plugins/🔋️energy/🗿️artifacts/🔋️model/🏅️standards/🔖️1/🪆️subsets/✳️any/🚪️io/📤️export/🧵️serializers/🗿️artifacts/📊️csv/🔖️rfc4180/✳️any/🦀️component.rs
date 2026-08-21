//! model -> csv
use crate::artifacts::model::EnergyModelSnapshot;
use semio_s_plugin_stdio::artifacts::csv::schema::snapshot::{CsvField, CsvRecord};
use semio_s_plugin_stdio::artifacts::csv::{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

pub async fn register() {}

/// 🌉 One header record (`payload`) + one data record carrying the DSL-printed snapshot as a
/// single quoted field — `CsvSnapshot::records[0]` IS the header row (RFC 4180 draws no
/// structural distinction; see csv's own snapshot module).
pub async fn serialize(snapshot: &EnergyModelSnapshot) -> Result<CsvSnapshot, store::TextError> {
    let field = |value: String| CsvField { value, quoted: true };
    Ok(CsvSnapshot {
        schema: STDIO_CSV_DOCUMENT_SCHEMA.into(),
        has_header: true,
        records: vec![CsvRecord { fields: vec![field("payload".into())] }, CsvRecord { fields: vec![field(<EnergyModelSnapshot as store::ArtifactDsl>::print_dsl(snapshot))] }],
    })
}

pub async fn serialize_bytes(snapshot: &EnergyModelSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<CsvSnapshot as store::ArtifactPack>::encode_pack(&serialize(snapshot)?))
}
