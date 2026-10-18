//! 🧵️ Operation text, binary and canonical JSON images of the fixture mutation families.
//!
//! 🧭️ Moved from the former store unit test module; the items are `pub` because the test crates of the Store use them through the public API only.

use crate::prelude::*;
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

/// 🧾️ The canonical JSON image of one fixture operation: its name, an optional integer payload under its field name and an optional physical clock, in the exact serde order.
#[derive(Clone, Copy)]
pub struct FixtureOperationJson {
    pub operation: &'static str,
    pub field: &'static str,
    pub value: Option<Option<i64>>,
    pub physical_ms: Option<u64>,
}

impl FixtureOperationJson {
    pub fn length(&self) -> usize {
        1 + usize::from(self.value.is_some()) + usize::from(self.physical_ms.is_some())
    }

    pub fn node(&self, path: &[usize]) -> Result<ArtifactCanonicalJsonNode<'static>, String> {
        use ArtifactCanonicalJsonNode as N;
        Ok(match (path, self.value, self.physical_ms) {
            ([], _, _) => N::Object(self.length()),
            ([0], _, _) => N::String(self.operation),
            ([1], Some(value), _) => value.map_or(N::Null, N::I64),
            ([2], Some(_), Some(milliseconds)) => N::U64(milliseconds),
            _ => return Err("canonical-edit.invalid-typed-path".into()),
        })
    }

    pub fn key(&self, path: &[usize], index: usize) -> Result<ArtifactCanonicalJsonText<'static>, String> {
        match (path, index) {
            ([], 0) => Ok("operation".into()),
            ([], 1) if self.value.is_some() => Ok(self.field.into()),
            ([], 2) if self.physical_ms.is_some() => Ok("physicalMs".into()),
            _ => Err("canonical-edit.invalid-typed-path".into()),
        }
    }

    pub fn borrowed<'a>(self) -> ArtifactCanonicalJsonValue<'a> {
        ArtifactCanonicalJsonValue::Object(ArtifactCanonicalJsonObject::new((0..self.length()).map(move |index| (self.key(&[], index).expect("fixed fixture key"), ArtifactCanonicalJsonValue::Scalar(self.node(&[index]).expect("fixed fixture node"))))))
    }
}

/// 🧬️ Names the fixture operation image of one mutation family.
pub trait FixtureOperation {
    fn fixture_operation(&self) -> FixtureOperationJson;
}

impl FixtureOperation for DemoMutation {
    fn fixture_operation(&self) -> FixtureOperationJson {
        match self {
            DemoMutation::SetN(value) => FixtureOperationJson { operation: "setN", field: "n", value: Some(Some(i64::from(value.n))), physical_ms: None },
            DemoMutation::DeleteN(_) => FixtureOperationJson { operation: "deleteN", field: "", value: None, physical_ms: None },
            DemoMutation::AddN(value) => FixtureOperationJson { operation: "addN", field: "delta", value: Some(Some(i64::from(value.delta))), physical_ms: None },
            DemoMutation::AssignN(value) => FixtureOperationJson { operation: "assignN", field: "n", value: Some(value.n.map(i64::from)), physical_ms: None },
        }
    }
}

impl FixtureOperation for SeverityMutation {
    fn fixture_operation(&self) -> FixtureOperationJson {
        let integer = |operation, n: i32| FixtureOperationJson { operation, field: "n", value: Some(Some(i64::from(n))), physical_ms: None };
        match self {
            SeverityMutation::SetN(value) => integer("setN", value.n),
            SeverityMutation::SetWarningN(value) => integer("setWarningN", value.n),
            SeverityMutation::SetErrorN(value) => integer("setErrorN", value.n),
            SeverityMutation::SetFatalN(value) => integer("setFatalN", value.n),
            SeverityMutation::AssignN(value) => FixtureOperationJson { operation: "assignN", field: "n", value: Some(value.n.map(i64::from)), physical_ms: None },
        }
    }
}

impl FixtureOperation for ValidatedMutation {
    fn fixture_operation(&self) -> FixtureOperationJson {
        match self {
            ValidatedMutation::SetN(value) => FixtureOperationJson { operation: "setN", field: "n", value: Some(Some(i64::from(value.n))), physical_ms: None },
            ValidatedMutation::AssignN(value) => FixtureOperationJson { operation: "assignN", field: "n", value: Some(value.n.map(i64::from)), physical_ms: None },
        }
    }
}

impl FixtureOperation for TimestampedMutation {
    fn fixture_operation(&self) -> FixtureOperationJson {
        match self {
            TimestampedMutation::SetN(value) => FixtureOperationJson { operation: "setN", field: "n", value: Some(Some(i64::from(value.n))), physical_ms: Some(value.physical_ms) },
            TimestampedMutation::AssignN(value) => FixtureOperationJson { operation: "assignN", field: "n", value: Some(value.n.map(i64::from)), physical_ms: Some(value.physical_ms) },
        }
    }
}

macro_rules! fixture_canonical_json {
    ($($mutation:ty),*) => {$(
        impl ArtifactCanonicalJson for $mutation {
            fn canonical_json_node(&self, path: &[usize]) -> Result<ArtifactCanonicalJsonNode<'_>, semio_framework_value::ValueError> {
                self.fixture_operation().node(path).map_err(|reason| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, reason))
            }
            fn canonical_json_key(&self, path: &[usize], index: usize) -> Result<ArtifactCanonicalJsonText<'_>, semio_framework_value::ValueError> {
                self.fixture_operation().key(path, index).map_err(|reason| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, reason))
            }
            fn canonical_json_borrowed_root(&self) -> Result<Option<ArtifactCanonicalJsonValue<'_>>, semio_framework_value::ValueError> {
                Ok(Some(self.fixture_operation().borrowed()))
            }
        }
    )*};
}

fixture_canonical_json!(SeverityMutation, ValidatedMutation, TimestampedMutation);


//#region 🔖️OpCodec
/// 🎞️ Handcrafted OpText (P6).
impl OpText for DemoMutation {
    fn parse_op(line: &str) -> Result<Self, TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown operation line '{line}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl OpBinary for DemoMutation {
    fn encode_op(&self) -> Result<Vec<u8>, semio_framework_os_spr::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(semio_framework_os_spr::ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword {keyword:?} is not a declared variant") })?;
        let spec = (variants[ordinal].1.ordinary)();
        let body = semio_framework_os_pack::record::encode_record_body(&spec, &record, &PackEncodeOptions::default()).map_err(semio_framework_os_spr::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        semio_framework_os_pack::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, semio_framework_os_spr::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = semio_framework_os_pack::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(semio_framework_os_spr::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(semio_framework_os_spr::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
        let spec = (spec_fn.ordinary)();
        let body = &bytes[reader.position()..];
        let (record, _report) = semio_framework_os_pack::record::decode_record_body(body, &spec, &PackDecodeOptions::default()).map_err(semio_framework_os_spr::ProtocolError::from)?;
        let record_offset = reader.position() as u64;
        <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record).map_err(|error| semio_framework_os_spr::ProtocolError::Malformed { what: "op record", offset: record_offset, detail: error.to_string() })
    }
}


impl ArtifactCanonicalJson for DemoMutation {
    fn canonical_json_node(&self, path: &[usize]) -> Result<ArtifactCanonicalJsonNode<'_>, semio_framework_value::ValueError> {
        self.fixture_operation().node(path).map_err(|reason| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, reason))
    }
    fn canonical_json_key(&self, path: &[usize], index: usize) -> Result<ArtifactCanonicalJsonText<'_>, semio_framework_value::ValueError> {
        self.fixture_operation().key(path, index).map_err(|reason| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, reason))
    }
    fn canonical_json_borrowed_root(&self) -> Result<Option<ArtifactCanonicalJsonValue<'_>>, semio_framework_value::ValueError> {
        let value = match self {
            DemoMutation::SetN(value) => ArtifactCanonicalJsonValue::Object(ArtifactCanonicalJsonObject::new(
                [("operation", ArtifactCanonicalJsonValue::Scalar(ArtifactCanonicalJsonNode::String("setN"))), ("n", ArtifactCanonicalJsonValue::Scalar(ArtifactCanonicalJsonNode::I64(i64::from(value.n))))].into_iter(),
            )),
            DemoMutation::DeleteN(_) => ArtifactCanonicalJsonValue::Object(ArtifactCanonicalJsonObject::new([("operation", ArtifactCanonicalJsonValue::Scalar(ArtifactCanonicalJsonNode::String("deleteN")))].into_iter())),
            DemoMutation::AddN(value) => ArtifactCanonicalJsonValue::Object(ArtifactCanonicalJsonObject::new(
                [("operation", ArtifactCanonicalJsonValue::Scalar(ArtifactCanonicalJsonNode::String("addN"))), ("delta", ArtifactCanonicalJsonValue::Scalar(ArtifactCanonicalJsonNode::I64(i64::from(value.delta))))].into_iter(),
            )),
            DemoMutation::AssignN(value) => ArtifactCanonicalJsonValue::Object(ArtifactCanonicalJsonObject::new(
                [
                    ("operation", ArtifactCanonicalJsonValue::Scalar(ArtifactCanonicalJsonNode::String("assignN"))),
                    ("n", value.n.map_or(ArtifactCanonicalJsonValue::Scalar(ArtifactCanonicalJsonNode::Null), |n| ArtifactCanonicalJsonValue::Scalar(ArtifactCanonicalJsonNode::I64(i64::from(n))))),
                ]
                .into_iter(),
            )),
        };
        Ok(Some(value))
    }
}


//#region 🏛️SpaceTests
/// ⏱️ Like `DemoMutation` but with an explicit, test-controlled `timestamp()` override, so
/// undo-ordering-by-HLT tests don't depend on real wall-clock resolution.

//#region 🔖️OpCodec
/// 🎞️ Handcrafted OpText (P6).
impl OpText for TimestampedMutation {
    fn parse_op(line: &str) -> Result<Self, TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown operation line '{line}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl OpBinary for TimestampedMutation {
    fn encode_op(&self) -> Result<Vec<u8>, semio_framework_os_spr::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(semio_framework_os_spr::ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword {keyword:?} is not a declared variant") })?;
        let spec = (variants[ordinal].1.ordinary)();
        let body = semio_framework_os_pack::record::encode_record_body(&spec, &record, &PackEncodeOptions::default()).map_err(semio_framework_os_spr::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        semio_framework_os_pack::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, semio_framework_os_spr::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = semio_framework_os_pack::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(semio_framework_os_spr::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(semio_framework_os_spr::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
        let spec = (spec_fn.ordinary)();
        let body = &bytes[reader.position()..];
        let (record, _report) = semio_framework_os_pack::record::decode_record_body(body, &spec, &PackDecodeOptions::default()).map_err(semio_framework_os_spr::ProtocolError::from)?;
        let record_offset = reader.position() as u64;
        <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record).map_err(|error| semio_framework_os_spr::ProtocolError::Malformed { what: "op record", offset: record_offset, detail: error.to_string() })
    }
}



//#region 🔖️PreviewWireTests
/// 🧪️ Fixture producing one message per non-clean variant, each using one of the frozen
/// nine `mutation.*` codes (`📋️contract-freeze.md` §C2's table) — `preview_wire`'s and
/// `CompositionCoordinator` phase 1's shared dry-run fixture. `WarnN` ⇒ `mutation.clamped`
/// (Warning, non-empty diff), `ErrorN` ⇒ `mutation.target-missing` (Error, empty diff — LAW 2),
/// `FatalN` ⇒ `mutation.invariant` (Fatal, empty diff — LAW 1), `CleanN` ⇒ silent.

/// 🎞️ Handcrafted OpText (P6).
impl OpText for SeverityMutation {
    fn parse_op(line: &str) -> Result<Self, TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown operation line '{line}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl OpBinary for SeverityMutation {
    fn encode_op(&self) -> Result<Vec<u8>, semio_framework_os_spr::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(semio_framework_os_spr::ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword {keyword:?} is not a declared variant") })?;
        let spec = (variants[ordinal].1.ordinary)();
        let body = semio_framework_os_pack::record::encode_record_body(&spec, &record, &PackEncodeOptions::default()).map_err(semio_framework_os_spr::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        semio_framework_os_pack::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, semio_framework_os_spr::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = semio_framework_os_pack::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(semio_framework_os_spr::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(semio_framework_os_spr::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
        let spec = (spec_fn.ordinary)();
        let body = &bytes[reader.position()..];
        let (record, _report) = semio_framework_os_pack::record::decode_record_body(body, &spec, &PackDecodeOptions::default()).map_err(semio_framework_os_spr::ProtocolError::from)?;
        let record_offset = reader.position() as u64;
        <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record).map_err(|error| semio_framework_os_spr::ProtocolError::Malformed { what: "op record", offset: record_offset, detail: error.to_string() })
    }
}



//#region 🔖️CompositionTests

impl OpText for ValidatedMutation {
    fn parse_op(line: &str) -> Result<Self, TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown operation line '{line}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}

impl OpBinary for ValidatedMutation {
    fn encode_op(&self) -> Result<Vec<u8>, semio_framework_os_spr::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(semio_framework_os_spr::ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword {keyword:?} is not a declared variant") })?;
        let spec = (variants[ordinal].1.ordinary)();
        let body = semio_framework_os_pack::record::encode_record_body(&spec, &record, &PackEncodeOptions::default()).map_err(semio_framework_os_spr::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        semio_framework_os_pack::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, semio_framework_os_spr::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = semio_framework_os_pack::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(semio_framework_os_spr::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(semio_framework_os_spr::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
        let spec = (spec_fn.ordinary)();
        let body = &bytes[reader.position()..];
        let (record, _report) = semio_framework_os_pack::record::decode_record_body(body, &spec, &PackDecodeOptions::default()).map_err(semio_framework_os_spr::ProtocolError::from)?;
        let record_offset = reader.position() as u64;
        <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record).map_err(|error| semio_framework_os_spr::ProtocolError::Malformed { what: "op record", offset: record_offset, detail: error.to_string() })
    }
}
//#endregion 🔖️CompositionTests
//#endregion 🔖️PreviewWireTests
//#endregion 🔖️OpCodec
//#endregion 🏛️SpaceTests
//#endregion 🔖️OpCodec
