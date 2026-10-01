//! 🪶️ Exact semantic snapshot transport declared by the codec WIT interface.
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};
use semio_framework::sqlite_snapshot::SqliteDatabaseLimits;

#[derive(Clone, Copy, Debug, Deserialize, Serialize, ToValue, FromValue)]
#[serde(deny_unknown_fields)]
pub struct SnapshotLimits {
    pub max_file_bytes: u64,
    pub max_value_bytes: u64,
    pub max_schema_bytes: u64,
    pub max_rows: u64,
    pub max_columns: u64,
    pub max_tables: u64,
    pub max_pages: u64,
}

impl From<SqliteDatabaseLimits> for SnapshotLimits {
    fn from(value: SqliteDatabaseLimits) -> Self { Self { max_file_bytes: value.max_file_bytes as u64, max_value_bytes: value.max_value_bytes as u64, max_schema_bytes: value.max_schema_bytes as u64, max_rows: value.max_rows as u64, max_columns: value.max_columns as u64, max_tables: value.max_tables as u64, max_pages: value.max_pages as u64 } }
}

impl SnapshotLimits {
    pub fn native(self) -> Result<SqliteDatabaseLimits, String> {
        let count = |value| usize::try_from(value).map_err(|_| "SQLite resource bound exceeds platform address space".to_string());
        Ok(SqliteDatabaseLimits { max_file_bytes: count(self.max_file_bytes)?, max_value_bytes: count(self.max_value_bytes)?, max_schema_bytes: count(self.max_schema_bytes)?, max_rows: count(self.max_rows)?, max_columns: count(self.max_columns)?, max_tables: count(self.max_tables)?, max_pages: count(self.max_pages)? })
    }
}

#[derive(Deserialize, Serialize, ToValue, FromValue)]
#[serde(deny_unknown_fields)]
pub struct SnapshotInput {
    pub dialect: String,
    pub encoding: String,
    pub payload: Vec<u8>,
    pub limits: SnapshotLimits,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToValue, FromValue)]
pub struct SnapshotPayload {
    pub encoding: String,
    pub bytes: Vec<u8>,
    pub diagnostics: Vec<u8>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToValue, FromValue)]
pub struct SnapshotFile { pub bytes: Vec<u8>, pub diagnostics: Vec<u8> }

#[derive(Clone, Debug, Deserialize, Serialize, ToValue, FromValue)]
pub struct SnapshotRejection { pub message: String, pub diagnostics: Vec<u8> }

#[derive(Clone, Debug, Deserialize, Serialize, ToValue, FromValue)]
pub enum SnapshotFileResult { Done(SnapshotFile), Rejected(SnapshotRejection) }

#[derive(Clone, Debug, Deserialize, Serialize, ToValue, FromValue)]
pub enum SnapshotPayloadResult { Done(SnapshotPayload), Rejected(SnapshotRejection) }

pub fn encode_diagnostics(diagnostics: &Vec<semio_framework::Diagnostic>) -> Vec<u8> {
    semio_framework_os_kernel::pack_rt::encode_wire_value(&semio_framework_os_kernel::ToValue::to_value(diagnostics))
}

pub fn decode_diagnostics(bytes: &[u8]) -> Result<Vec<semio_framework::Diagnostic>, String> {
    let value = semio_framework_os_kernel::pack_rt::decode_wire_value(bytes).map_err(|error| error.to_string())?;
    <Vec<semio_framework::Diagnostic> as semio_framework_os_kernel::FromValue>::from_value(value).map_err(|error| error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sqlite_snapshot_guest_wire_preserves_diagnostics_and_limits() {
        let mut warning = semio_framework::Diagnostic::error("snapshot.guest.fixture", semio_framework::TextSpan::at(2, 3), "semantic fixture warning");
        warning.severity = semio_framework::Severity::Warning;
        let diagnostics = vec![warning];
        assert_eq!(decode_diagnostics(&encode_diagnostics(&diagnostics)).unwrap(), diagnostics);
        assert!(decode_diagnostics(&[0xff]).is_err());
        let fixture: SnapshotInput = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
        assert_eq!(fixture.dialect, "s.testkit.w1c-fixture@1/strict");
        assert_eq!(fixture.limits.native().unwrap().max_rows, 4096);
        let limits = SqliteDatabaseLimits::default();
        let transported = SnapshotLimits::from(limits).native().unwrap();
        assert_eq!(transported.max_rows, limits.max_rows);
        assert_eq!(transported.max_file_bytes, limits.max_file_bytes);
        let result = SnapshotFileResult::Rejected(SnapshotRejection { message: "invalid exact subset".into(), diagnostics: encode_diagnostics(&diagnostics) });
        let packed = semio_framework_os_kernel::pack_rt::encode_wire_value(&semio_framework_os_kernel::ToValue::to_value(&result));
        let value = semio_framework_os_kernel::pack_rt::decode_wire_value(&packed).unwrap();
        let restored = <SnapshotFileResult as semio_framework_os_kernel::FromValue>::from_value(value).unwrap();
        let SnapshotFileResult::Rejected(restored) = restored else { panic!("rejection changed kind") };
        assert_eq!(restored.message, "invalid exact subset");
        assert_eq!(decode_diagnostics(&restored.diagnostics).unwrap(), diagnostics);
    }
}
