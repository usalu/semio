//! 🧮️ Demo and retained-text snapshots, their diffs and decode counters.
//!
//! 🧭️ Moved from the former store unit test module; the items are `pub` because the test crates of the Store use them through the public API only.

use crate::prelude::*;
use semio_framework_os_spr::ColdMutationDiff;
use crate::*;
use crate::demo::{AddN, AssignN, DeleteN, DemoMutation, SetN};
use crate::lossy::{LossyMutation, SetN as LossySetN};
use crate::severity::{SetErrorN, SetFatalN, SetN as SeveritySetN, SetWarningN, SeverityMutation};
use crate::timestamped::{SetN as TimestampedSetN, TimestampedMutation};
use crate::validated::{AssignN as ValidatedAssignN, SetN as ValidatedSetN, ValidatedMutation};
use pack::snapshot::ArtifactSqliteSnapshot as _;
use semio_framework_os_store_snapshot_clone::RetainedCloneEdit;
use semio_framework_os_store_snapshot_clone::RetainedCloneEditCursor;
use semio_framework_os_store_snapshot_clone::RetainedCloneEditStep;
use semio_framework_os_store_snapshot_clone::RetainedClonePreparationFactory;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef};
use semio_framework_pack_json_retained_schema::testing::{OWNED_SCHEMA_TEST_FIELDS, owned_schema_test_cursor, drive_owned_schema, schema_test_close_grant};
use semio_framework_value::testing::physical_test_close_grant;

pub fn assert_fixture_descriptor<T: semio_framework_os_spr::MutationLeaf>(descriptor: &str) {
    assert_eq!(serde_json::Value::from(T::DESCRIPTOR.to_value()), serde_json::from_str::<serde_json::Value>(descriptor).unwrap());
    assert!(T::DESCRIPTOR.validate().is_ok());
}

/// ➕️ L3 for one fixture leaf: the concrete inverse's diffs sum to the negative of the forward diff on every listed demo document (`None` is the deleted value).
pub async fn assert_fixture_sum_law<Op: semio_framework_os_spr::Mutation<DemoSnapshot> + semio_framework_os_spr::ColdMutation<DemoSnapshot, ReferenceDiff = DemoDiff>>(mutation: &Op, bases: &[Option<i32>]) {
    for n in bases {
        semio_framework_os_spr::protocol_laws::assert_mutation_inverse_sum_law(mutation, &DemoSnapshot { n: *n }).await;
    }
}


#[derive(semio_framework_dsl_record_derive::DslRecord, Debug, PartialEq, Serialize, ToValue, Deserialize, FromValue, semio_framework_os_dsl::DslArtifact, semio_framework_value_derive::RetainedClone, semio_framework_value_derive::RetireOwned)]
#[artifact(id = "demo.doc", extension = "demo")]
pub struct DemoSnapshot {
    pub n: Option<i32>,
}

std::thread_local! { pub static DEMO_PROJECTION_CLONES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; }

impl Clone for DemoSnapshot {
    fn clone(&self) -> Self { DEMO_PROJECTION_CLONES.with(|count| count.set(count.get() + 1)); Self { n: self.n } }
}

impl semio_framework_schema_composition::ArtifactCompositionFields for DemoSnapshot {
    fn visit_child_refs<'a, V: semio_framework_schema_composition::ChildRefVisitor<'a>>(&'a self, _visitor: &mut V) -> Result<(), V::Error> {
        Ok(())
    }
}

impl semio_framework_schema_composition::ArtifactChildReadAdmission for DemoSnapshot {
 fn admit_child_reads<R:semio_framework_schema_composition::ChildReadSource>(&self,source:&mut R)->Result<(),R::Error>{source.step()}
}

impl Default for DemoSnapshot {
    fn default() -> Self {
        Self { n: Some(0) }
    }
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6).

impl semio_framework_os_store_text_io::ArtifactSqliteSnapshotHost for DemoSnapshot {
    
    fn preflight_sqlite_snapshot_encoding(&self, _: semio_framework_io_sqlite_snapshot::SnapshotEncoding, control: &mut semio_framework_io_sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<(), semio_framework_value::ValueError> {
        let mut bound = semio_framework_io_sqlite_snapshot::artifact::NativeEncodingBound::new(control)?;
        bound.add(4096)?;
        bound.finish()
    }
    
    
}

impl pack::snapshot::ArtifactSqliteSnapshot for DemoSnapshot {
const SQLITE_SCHEMA: &'static str = include_str!("../../🧪️tests/🔬️unit/🗄️.sql");
fn to_sqlite_database(&self, control: &mut semio_framework_io_sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<semio_framework_io_sqlite_snapshot::SqliteDatabase, semio_framework_value::ValueError> {
        use semio_framework_io_sqlite_snapshot::SqliteDatabase; use semio_framework_io_sqlite_snapshot::SqliteRow; use semio_framework_io_sqlite_snapshot::SqliteSnapshotPhase; use semio_framework_io_sqlite_snapshot::SqliteValue;
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 1)?;
        let mut database = SqliteDatabase::from_schema(Self::SQLITE_SCHEMA)?;
        database.table_mut("demo_state")?.rows.push(SqliteRow { rowid: 1, values: vec![SqliteValue::Integer(1), self.n.map(|value| SqliteValue::Integer(i64::from(value))).unwrap_or(SqliteValue::Null)] });
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 1, 1)?;
        Ok(database)
    }
fn from_sqlite_database(database: &semio_framework_io_sqlite_snapshot::SqliteDatabase, control: &mut semio_framework_io_sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<Self, semio_framework_value::ValueError> {
        use semio_framework_io_sqlite_snapshot::SqliteSnapshotPhase; use semio_framework_io_sqlite_snapshot::SqliteValue;
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 0, 1)?;
        let rows = &database.table("demo_state")?.rows;
        if rows.len() != 1 || rows[0].rowid != 1 || rows[0].integer(0)? != 1 {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "demo SQLite snapshot requires one state row"));
        }
        let snapshot = Self {
            n: match rows[0].values.get(1) {
                Some(SqliteValue::Null) => None,
                Some(SqliteValue::Integer(value)) => Some(i32::try_from(*value).map_err(|_| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "demo state n exceeds i32"))?),
                _ => return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "demo state n must be integer or null")),
            },
        };
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 1, 1)?;
        Ok(snapshot)
    }
}

impl ArtifactDsl for DemoSnapshot {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, TextError> {
        let body = match semio_framework_os_semio::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let record = self.__dsl_to_record();
        let body = semio_framework_dsl_record::print(&record, &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = semio_framework_os_semio::SemioEnvelope::from_envelope_id(<Self as ArtifactDsl>::envelope_id(), semio_framework_os_semio::Component::Dsl, 1).expect("valid envelope_id");
        semio_framework_os_semio::wrap_text(&envelope, &body)
    }
}

std::thread_local! {
    pub static MEMBER_SNAPSHOT_DECODE_COUNT: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// 📦️ Handcrafted ArtifactPack (P6).
impl ArtifactPack for DemoSnapshot {
    /// 🪶️ Publishes this owner's actual relational snapshot capability.
    

    fn encode_pack_with(&self, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        let inner = pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = semio_framework_os_semio::SemioEnvelope::from_envelope_id(<Self as ArtifactDsl>::envelope_id(), semio_framework_os_semio::Component::Pack, 1).map_err(|error| PackError::from(error.into_value_error()))?;
        Ok(semio_framework_os_semio::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &PackDecodeOptions) -> Result<Self, PackError> {
        MEMBER_SNAPSHOT_DECODE_COUNT.with(|count| count.set(count.get() + 1));
        let (envelope, inner) = semio_framework_os_semio::unwrap_binary(bytes).map_err(|error| PackError::from(error.into_value_error()))?;
        if !envelope.matches_identity(<Self as ArtifactDsl>::envelope_id(), semio_framework_os_semio::Component::Pack, 1) {
            return Err(PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

impl semio_framework_os_store_text_io::ArtifactStoreSnapshot for DemoSnapshot {
fn sqlite_snapshot_codec() -> Option<semio_framework_os_store_text_io::ArtifactSqliteSnapshotCodec> {
        Some(<Self as semio_framework_os_io::sqlite_snapshot_operations::ArtifactSqliteSnapshotOperations>::sqlite_codec())
    }
}


// `impl pack::snapshot::ArtifactPack for DemoSnapshot` is now generated automatically by
// `#[derive(crate::os_dsl::DslArtifact)]` above (see dsl/derive/rs/lib.rs's `🔖️DslArtifact` region) —
// same seam as its `impl pack::snapshot::ArtifactDsl for DemoSnapshot` sibling.

#[derive(Clone, Debug, Default, PartialEq, Serialize, ToValue, Deserialize, FromValue)]
pub struct DemoDiff {
    pub n: Option<DemoFieldChange>,
}

#[derive(Clone, Debug, PartialEq, Serialize, ToValue, Deserialize, FromValue)]
pub struct DemoFieldChange {
    pub value: Option<i32>,
}

impl DemoDiff {
    pub fn value(value: Option<i32>) -> Self {
        Self { n: Some(DemoFieldChange { value }) }
    }
}

impl semio_framework_os_spr::DiffAlgebra<DemoSnapshot> for DemoDiff {

    fn is_empty(&self) -> bool {
        self.n.is_none()
    }
}

impl semio_framework_os_spr::ColdDiffAlgebra<DemoSnapshot> for DemoDiff {
    fn inverse(&self, base: &DemoSnapshot) -> Self {
        Self { n: self.n.as_ref().map(|_| DemoFieldChange { value: base.n }) }
    }
}

impl MutationDiff<DemoSnapshot> for DemoDiff {



}

impl ColdMutationDiff<DemoSnapshot> for DemoDiff {
    fn absorb(&mut self, other: Self) {
        if other.n.is_some() {
            self.n = other.n;
        }
    }

    fn apply(&self, snapshot: &mut DemoSnapshot, _capability: semio_framework_os_spr::ApplyCapability) -> semio_framework_os_spr::MutationApplyResult<()> {
                if let Some(change) = &self.n { snapshot.n = change.value; }
        Ok(())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
pub struct LossyDiff {}

impl semio_framework_os_spr::DiffAlgebra<DemoSnapshot> for LossyDiff {

    fn is_empty(&self) -> bool {
        true
    }
}

impl semio_framework_os_spr::ColdDiffAlgebra<DemoSnapshot> for LossyDiff {
    fn inverse(&self, _base: &DemoSnapshot) -> Self {
        Self {}
    }
}

impl MutationDiff<DemoSnapshot> for LossyDiff {


}

impl ColdMutationDiff<DemoSnapshot> for LossyDiff {
    fn absorb(&mut self, _other: Self) {}

    fn apply(&self, _unpublished: &mut DemoSnapshot, _capability: semio_framework_os_spr::ApplyCapability) -> semio_framework_os_spr::MutationApplyResult<()> {
        Ok(())
    }
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_value_derive::RetainedClone, semio_framework_value_derive::RetireOwned)]
pub struct RetainedTextSnapshot {
    pub text: String,
}

impl semio_framework_schema_composition::ArtifactCompositionFields for RetainedTextSnapshot {
    fn visit_child_refs<'a, V: semio_framework_schema_composition::ChildRefVisitor<'a>>(&'a self, _visitor: &mut V) -> Result<(), V::Error> {
        Ok(())
    }
}

impl semio_framework_schema_composition::ArtifactChildReadAdmission for RetainedTextSnapshot {
 fn admit_child_reads<R:semio_framework_schema_composition::ChildReadSource>(&self,source:&mut R)->Result<(),R::Error>{source.step()}
}

impl ArtifactPack for RetainedTextSnapshot {
    fn encode_pack_with(&self, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        <semio_framework_value::DslValue as ArtifactPack>::encode_pack_with(&self.to_value(), options)
    }

    fn decode_pack_with(bytes: &[u8], options: &PackDecodeOptions) -> Result<Self, PackError> {
        Self::from_value(<semio_framework_value::DslValue as ArtifactPack>::decode_pack_with(bytes, options)?).map_err(PackError::from)
    }
}

impl semio_framework_os_store_text_io::ArtifactStoreSnapshot for RetainedTextSnapshot {

}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
pub struct RetainedTextDiff {
    pub text: Option<String>,
}

impl semio_framework_os_spr::DiffAlgebra<RetainedTextSnapshot> for RetainedTextDiff {

    fn is_empty(&self) -> bool {
        self.text.is_none()
    }
}

impl semio_framework_os_spr::ColdDiffAlgebra<RetainedTextSnapshot> for RetainedTextDiff {
    fn inverse(&self, base: &RetainedTextSnapshot) -> Self {
        Self { text: self.text.as_ref().map(|_| base.text.clone()) }
    }
}

impl MutationDiff<RetainedTextSnapshot> for RetainedTextDiff {



}

impl ColdMutationDiff<RetainedTextSnapshot> for RetainedTextDiff {
    fn absorb(&mut self, other: Self) {
        if other.text.is_some() {
            self.text = other.text;
        }
    }

    fn apply(&self, base: &mut RetainedTextSnapshot, _capability: semio_framework_os_spr::ApplyCapability) -> semio_framework_os_spr::MutationApplyResult<()> {
                if let Some(value) = &self.text { base.text = value.clone(); }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_value_derive::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct SetRetainedText {
    pub text: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_value_derive::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[serde(tag = "operation", rename_all = "camelCase", deny_unknown_fields)]
#[value(tag = "operation", rename_all = "camelCase", deny_unknown_fields)]
#[canonical_json(owner=semio_framework_pack_json)]
pub enum RetainedTextMutation {
    SetRetainedText(SetRetainedText),
}

impl semio_framework_os_spr::Mutation<RetainedTextSnapshot> for RetainedTextMutation {
    type Diff = RetainedTextDiff;
    const DESCRIPTORS: &'static [semio_framework_os_spr::MutationLeafDescriptor] = <DemoMutation as semio_framework_os_spr::Mutation<DemoSnapshot>>::DESCRIPTORS;

    fn descriptor(&self) -> &'static semio_framework_os_spr::MutationLeafDescriptor {
        &<Self as semio_framework_os_spr::Mutation<RetainedTextSnapshot>>::DESCRIPTORS[0]
    }





    fn conflict_target(&self) -> Vec<String> {
        vec!["text".into()]
    }
}

impl semio_framework_os_spr::ColdMutation<RetainedTextSnapshot> for RetainedTextMutation {
    type ReferenceDiff = RetainedTextDiff;
    fn diff(&self, _base: &RetainedTextSnapshot) -> semio_framework_os_spr::MutationOutcome<RetainedTextDiff> {
        let Self::SetRetainedText(value) = self;
        semio_framework_os_spr::MutationOutcome::new(RetainedTextDiff { text: Some(value.text.clone()) })
    }

    fn inverse(&self, base: &RetainedTextSnapshot) -> Result<Vec<Self>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Self::SetRetainedText(SetRetainedText { text: base.text.clone() })]
    
    })())
}
}

impl OpText for RetainedTextMutation {
    fn parse_op(line: &str) -> Result<Self, TextError> {
        serde_json::from_str(line).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(error.to_string()).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
    }

    fn print_op(&self) -> String {
        serde_json::to_string(self).expect("retained text fixture mutation serializes")
    }
}

impl OpBinary for RetainedTextMutation {
    fn encode_op(&self) -> Result<Vec<u8>, semio_framework_os_spr::ProtocolError> {
        Ok(self.print_op().into_bytes())
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, semio_framework_os_spr::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| semio_framework_os_spr::ProtocolError::Malformed { what: "retained text mutation", offset: error.valid_up_to() as u64, detail: error.to_string() })?;
        Self::parse_op(text).map_err(|error| semio_framework_os_spr::ProtocolError::Malformed { what: "retained text mutation", offset: 0, detail: error.to_string() })
    }
}

impl ArtifactCanonicalJson for RetainedTextMutation {
    fn canonical_json_borrowed_root(&self) -> Result<Option<ArtifactCanonicalJsonValue<'_>>, semio_framework_value::ValueError> {
        let Self::SetRetainedText(value) = self;
        Ok(Some(ArtifactCanonicalJsonValue::Object(ArtifactCanonicalJsonObject::new(
            [("operation", ArtifactCanonicalJsonValue::Scalar(ArtifactCanonicalJsonNode::String("setRetainedText"))), ("text", ArtifactCanonicalJsonValue::Scalar(ArtifactCanonicalJsonNode::String(&value.text)))].into_iter(),
        ))))
    }
}

impl ArtifactDsl for RetainedTextSnapshot {
    const EXTENSION: &'static str = "retained-text";
    fn parse_dsl(text: &str) -> Result<Self, TextError> {
        Ok(Self { text: text.to_string() })
    }
    fn print_dsl(&self) -> String {
        self.text.clone()
    }
}
//#endregion 🔖️ArtifactCodec
