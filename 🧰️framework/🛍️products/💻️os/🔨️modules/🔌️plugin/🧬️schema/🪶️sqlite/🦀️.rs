//! 🪶️ Exact semantic snapshot transport declared by the codec WIT interface.
use semio_framework::sqlite_snapshot::SqliteDatabaseLimits;
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize, ToValue, FromValue)]
#[serde(deny_unknown_fields)]
pub struct SnapshotLimits {
    pub max_file_bytes: u64,
    pub max_value_bytes: u64,
    pub max_allocation_bytes: u64,
    pub max_schema_bytes: u64,
    pub max_rows: u64,
    pub max_columns: u64,
    pub max_tables: u64,
    pub max_pages: u64,
}

impl From<SqliteDatabaseLimits> for SnapshotLimits {
    fn from(value: SqliteDatabaseLimits) -> Self {
        Self {
            max_file_bytes: value.max_file_bytes as u64,
            max_value_bytes: value.max_value_bytes as u64,
            max_allocation_bytes: value.max_allocation_bytes as u64,
            max_schema_bytes: value.max_schema_bytes as u64,
            max_rows: value.max_rows as u64,
            max_columns: value.max_columns as u64,
            max_tables: value.max_tables as u64,
            max_pages: value.max_pages as u64,
        }
    }
}

impl SnapshotLimits {
    pub fn native(self) -> Result<SqliteDatabaseLimits, String> {
        let count = |value| usize::try_from(value).map_err(|_| "SQLite resource bound exceeds platform address space".to_string());
        Ok(SqliteDatabaseLimits {
            max_file_bytes: count(self.max_file_bytes)?,
            max_value_bytes: count(self.max_value_bytes)?,
            max_allocation_bytes: count(self.max_allocation_bytes)?,
            max_schema_bytes: count(self.max_schema_bytes)?,
            max_rows: count(self.max_rows)?,
            max_columns: count(self.max_columns)?,
            max_tables: count(self.max_tables)?,
            max_pages: count(self.max_pages)?,
        })
    }
}

/// 🎟️ Carries independent original native ownership axes across the component boundary.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, ToValue, FromValue)]
#[serde(deny_unknown_fields)]
pub struct SnapshotGrant {
    pub maximum_items: u64,
    pub maximum_copy_bytes: u64,
    pub maximum_capacity_bytes: u64,
    pub maximum_release_bytes: u64,
    pub maximum_depth: u64,
}
impl From<semio_framework_value::RetainedCloneGrant> for SnapshotGrant {
    fn from(value: semio_framework_value::RetainedCloneGrant) -> Self {
        Self { maximum_items:value.maximum_items as u64, maximum_copy_bytes:value.maximum_copy_bytes as u64, maximum_capacity_bytes:value.maximum_capacity_bytes as u64, maximum_release_bytes:value.maximum_release_bytes as u64, maximum_depth:value.maximum_depth as u64 }
    }
}
impl SnapshotGrant {
    /// 📏️ Refuses platform truncation while preserving every zero axis.
    pub fn native(self) -> Result<semio_framework_value::RetainedCloneGrant, semio_framework_value::ValueError> {
        let count=|value|usize::try_from(value).map_err(|_|semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"native component grant exceeds platform address space"));
        Ok(semio_framework_value::RetainedCloneGrant { maximum_items:count(self.maximum_items)?, maximum_copy_bytes:count(self.maximum_copy_bytes)?, maximum_capacity_bytes:count(self.maximum_capacity_bytes)?, maximum_release_bytes:count(self.maximum_release_bytes)?, maximum_depth:count(self.maximum_depth)? })
    }
}

#[derive(Deserialize, Serialize, ToValue, FromValue)]
#[serde(deny_unknown_fields)]
pub struct SnapshotInput {
    pub dialect: String,
    pub encoding: String,
    pub payload: Vec<u8>,
    pub limits: SnapshotLimits,
    pub native: SnapshotGrant,
}

/// 🧭️ Native snapshot encoding has no owned text backing at the component handoff.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize, ToValue, FromValue)]
#[serde(rename_all="lowercase")]
pub enum SnapshotEncoding { Binary, Text }
impl SnapshotEncoding { pub fn as_str(self)->&'static str { match self {Self::Binary=>"binary",Self::Text=>"text"} } }
semio_framework_value::artifact_retire_leaf!(SnapshotEncoding);

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToValue, FromValue, semio_framework_value::RetireOwned)]
pub struct SnapshotPayload {
    pub encoding: SnapshotEncoding,
    pub bytes: Vec<u8>,
    pub diagnostics: Vec<u8>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToValue, FromValue, semio_framework_value::RetireOwned)]
pub struct SnapshotFile {
    pub bytes: Vec<u8>,
    pub diagnostics: Vec<u8>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToValue, FromValue, semio_framework_value::RetireOwned)]
#[serde(deny_unknown_fields)]
pub struct SnapshotRejection {
    pub kind: semio_framework_value::ValueRefusalKind,
    pub message: String,
    pub diagnostics: Vec<u8>,
}

/// 👓️ Scalar demands describe retained owners without granting their next turn.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, ToValue, FromValue)]
pub struct SnapshotDemands { pub items:u64, pub copy_bytes:u64, pub capacity_bytes:u64, pub release_bytes:u64, pub depth:u64 }
/// 🎟️ Identifies actual pending custody in the original component instance.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, ToValue, FromValue)]
pub struct SnapshotRetirement { pub ticket:u64, pub owned_bytes:u64, pub refusal:u32, pub native_pending:bool, pub output_pending:bool, pub terminal:bool, pub demands:SnapshotDemands }
semio_framework_value::artifact_retire_leaf!(SnapshotDemands,SnapshotRetirement);
/// ♻️ Carries the actual step receipt without an owned transport envelope.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, ToValue, FromValue)]
pub struct SnapshotCloseInput {pub ticket:u64,pub native:SnapshotGrant}
#[derive(Clone, Copy, Debug, Deserialize, Serialize, ToValue, FromValue)]
pub struct SnapshotCloseReceipt {pub retirement:SnapshotRetirement,pub progress:SnapshotProgress,pub refusal:u32}
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, ToValue, FromValue)]
pub struct SnapshotProgress {pub copied_items:u64,pub copied_bytes:u64,pub retained_capacity_bytes:u64,pub released_bytes:u64}
impl From<semio_framework_value::RetainedCloneProgress> for SnapshotProgress {fn from(value:semio_framework_value::RetainedCloneProgress)->Self{Self{copied_items:value.copied_items as u64,copied_bytes:value.copied_bytes as u64,retained_capacity_bytes:value.retained_capacity_bytes as u64,released_bytes:value.released_bytes as u64}}}
impl SnapshotProgress {
 /// 📏️ Refuses foreign receipt truncation and preserves every original grant axis.
 pub fn native(self,grant:semio_framework_value::RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneProgress,semio_framework_value::ValueError>{let count=|value|usize::try_from(value).map_err(|_|semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"snapshot close receipt exceeds receiving address space"));let progress=semio_framework_value::RetainedCloneProgress{copied_items:count(self.copied_items)?,copied_bytes:count(self.copied_bytes)?,retained_capacity_bytes:count(self.retained_capacity_bytes)?,released_bytes:count(self.released_bytes)?};if !progress.fits(grant){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"snapshot close receipt exceeds original grant"))}Ok(progress)}
}
semio_framework_value::artifact_retire_leaf!(SnapshotProgress,SnapshotCloseReceipt);

#[derive(Clone, Debug, Deserialize, Serialize, ToValue, FromValue, semio_framework_value::RetireOwned)]
pub enum SnapshotFileResult {
    Done(SnapshotFile),
    Rejected(SnapshotRejection),
    Pending(SnapshotRetirement),
}

#[derive(Clone, Debug, Deserialize, Serialize, ToValue, FromValue, semio_framework_value::RetireOwned)]
pub enum SnapshotPayloadResult {
    Done(SnapshotPayload),
    Rejected(SnapshotRejection),
    Pending(SnapshotRetirement),
}

pub fn encode_diagnostics(diagnostics: &Vec<semio_framework::Diagnostic>) -> Vec<u8> {
    semio_framework_os_kernel::pack_rt::encode_wire_value(&semio_framework_value::ToValue::to_value(diagnostics))
}

pub fn decode_diagnostics(bytes: &[u8]) -> Result<Vec<semio_framework::Diagnostic>, semio_framework_value::ValueError> {
    let value = semio_framework_os_kernel::pack_rt::decode_wire_value(bytes).map_err(|error| error.into_value_error())?;
    <Vec<semio_framework::Diagnostic> as semio_framework_value::FromValue>::from_value(value)
}

impl SnapshotRejection {
    /// 🚨️ Restores the provider cause and its packed diagnostics without classifying prose.
    pub fn into_io_error(self) -> Result<semio_framework::io_schema::IoError, semio_framework_value::ValueError> {
        Ok(semio_framework::io_schema::IoError {
            cause: semio_framework_value::ValueError::new(self.kind, self.message),
            diagnostics: decode_diagnostics(&self.diagnostics)?,
        })
    }
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
        let grant=fixture.native.native().unwrap();
        assert_eq!(grant.maximum_copy_bytes,8192);
        assert_eq!(grant.maximum_release_bytes,16384);
        assert_eq!(grant.maximum_capacity_bytes,1048576);
        let limits = SqliteDatabaseLimits::default();
        let transported = SnapshotLimits::from(limits).native().unwrap();
        assert_eq!(transported.max_rows, limits.max_rows);
        assert_eq!(transported.max_file_bytes, limits.max_file_bytes);
        let result = SnapshotFileResult::Rejected(SnapshotRejection { kind: semio_framework_value::ValueRefusalKind::InvalidValue, message: "invalid exact subset".into(), diagnostics: encode_diagnostics(&diagnostics) });
        let packed = semio_framework_os_kernel::pack_rt::encode_wire_value(&semio_framework_value::ToValue::to_value(&result));
        let value = semio_framework_os_kernel::pack_rt::decode_wire_value(&packed).unwrap();
        let restored = <SnapshotFileResult as semio_framework_value::FromValue>::from_value(value).unwrap();
        let SnapshotFileResult::Rejected(restored) = restored else { panic!("rejection changed kind") };
        assert_eq!(restored.message, "invalid exact subset");
        assert_eq!(decode_diagnostics(&restored.diagnostics).unwrap(), diagnostics);
    }
}
