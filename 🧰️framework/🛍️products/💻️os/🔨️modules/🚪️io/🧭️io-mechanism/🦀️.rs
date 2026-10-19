//! 🧭️ IO mechanism: the typed serializer and deserializer vtable registry, routing, execution and identification.
//!
//! This file is the body of the `io_mechanism` module; the root mounts it as a module and keeps its documentation.

pub use semio_framework_io_control::{IoRunControl, IoNativeDirection};
use semio_framework_artifact_reference::io::text::artifact_reference::DialectCoordinateText as _;
use semio_framework_artifact_reference::{ArtifactDialect, Dialect};
use semio_framework_io_schema::{Confidence, IoEntryDescriptor, IoError, IoFidelity, IoOutcome, IoPayload, IoResult, IoRoute, CARRIER_BINARY, CARRIER_TEXT, SQLITE_SNAPSHOT};
use semio_framework_io_sqlite_snapshot::{SnapshotEncoding, SqliteDatabase, SqliteDatabaseLimits, SqliteRow, SqliteTable, SqliteValue, SqliteSnapshotProgress, SqliteSnapshotControl, SqliteSnapshotPhase, export_sqlite_database, import_sqlite_database, export_sqlite_database_controlled, import_sqlite_database_controlled};
use semio_framework_diagnostic::Diagnostic;
use semio_framework_os_store_space_history::io::sqlite::snapshot::settlement::TypedFileSettlement;
use semio_framework_value::{ValueError, ValueRefusalKind};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::RwLock;

#[path="../📤️serialization/📦️owned/🦀️.rs"]
pub mod owned_serialization;
pub use owned_serialization::{io_begin_owned_serialization, OwnedSerializerAdmission, OwnedSerializerChildren, OwnedSerializerFactory, OwnedSerializerJob, OwnedSerializerOutput, OwnedSerializerOutputHandoff, OwnedSerializerFaultHandoff, OwnedSerializerRefusal, OwnedSerializerRequest};
use owned_serialization::same_owned_serializer;

//#region 🔖️Traits
/// 🎹️ A typed native-value → foreign-payload encoder. `INTO`/`FIDELITY` are the foreign
/// dialect and the strongest fidelity this serializer achieves. `children` are the owned members of a composed native (design
/// §20.15: composed content is read on read, never through the parent's local owner) — the empty view for any other native.
pub trait Serializer<S> {
    const INTO: Dialect;
    const FIDELITY: IoFidelity;
    /// 🏭️ Declares the native retained-owner factory a serializer registers, or none when it only serializes through the borrowed path.
    const OWNED_FACTORY: Option<OwnedSerializerFactory> = None;
    fn serialize(from: &S, children: &ArchiveChildren, control: &mut IoRunControl<'_ , '_>) -> impl std::future::Future<Output = IoResult<IoPayload>>;
}

/// 🎹️ A typed foreign-payload → native-value decoder. `FROM`/`FIDELITY` are the foreign
/// dialect and the strongest fidelity this deserializer achieves. `CONFORMANCE` is D5's
/// `SubsetValidator` replacement, run by `deserializer_entry`/`deserializer_entry_text` right
/// after a successful `deserialize` — an addition beyond the ticket's literal trait sketch: an
/// `IoEntry.run`'s bare-`fn`-pointer field cannot capture a runtime `conformance` CONSTRUCTOR
/// PARAMETER (a closure that closes over any value, even a `Copy` fn pointer, cannot coerce to
/// a bare `fn` pointer), so a compile-time-resolvable associated const is the only channel that
/// keeps `IoEntry.run` a true, non-capturing function pointer.
pub trait Deserializer<S> {
    const FROM: Dialect;
    const FIDELITY: IoFidelity;
    const CONFORMANCE: Option<fn(&S) -> Vec<Diagnostic>> = None;
    fn sniff(_payload: &IoPayload) -> impl std::future::Future<Output = Confidence> + Send {
        std::future::ready(Confidence::None)
    }
    fn deserialize(payload: &IoPayload, control: &mut IoRunControl<'_ , '_>) -> impl std::future::Future<Output = IoResult<S>>;
}
//#endregion 🔖️Traits

//#region 🔖️ArchiveChildren
/// 🪆️ Borrows the original admitted composed members without cloning their backing or bypassing their decoder.
#[derive(Debug,Default,PartialEq,Eq)]
pub struct ArchiveChildren{members:Vec<semio_framework_os_channel::OwnedDocumentMemberPackEntry>}
impl ArchiveChildren{
    /// 🈳️ Provides a view with no owned child backing.
    pub const fn empty()->Self{Self{members:Vec::new()}}
    /// 📋️ Borrows each exact slot and child identity in original archive order.
    pub fn slots(&self)->impl Iterator<Item=(&str,&str)>{self.members.iter().map(|member|(member.owner.slot.as_str(),member.owner.child_id.as_str()))}
    /// 🪪️ Borrows the child's complete original archived dialect without materializing it.
    pub fn reference(&self,slot:&str,child_id:&str)->Option<&semio_framework_os_channel::DocumentArchiveArtifactRef>{self.find(slot,child_id).map(|member|&member.reference)}
    /// 📨️ Borrows the original envelope for the selected registered controlled child receiver.
    pub fn envelope(&self,slot:&str,child_id:&str)->Option<&[u8]>{self.find(slot,child_id).map(|member|member.envelope_pack.as_slice())}
    fn find(&self,slot:&str,child_id:&str)->Option<&semio_framework_os_channel::OwnedDocumentMemberPackEntry>{self.members.iter().find(|member|member.owner.slot==slot&&member.owner.child_id==child_id)}
}

/// 📨️ A binary native payload split into the parent's native bytes and its owned children: a composed artifact's head carrier
/// (`semio_framework_os_channel::encode_document_archive_bytes` of `{parent HEAD native, empty parent history, members}`, first byte
/// [`semio_framework_os_channel::DOCUMENT_ARCHIVE_VERSION`]) yields its parent bytes and members; any other payload is the parent alone.
fn native_carrier<'a>(bytes: &'a [u8],control:&mut semio_framework_io_control::NativeSnapshotDecodeOwner<'_, '_>) -> Result<(std::borrow::Cow<'a, [u8]>, ArchiveChildren), IoError> {
    if bytes.first() != Some(&semio_framework_os_channel::DOCUMENT_ARCHIVE_VERSION) {
        return Ok((std::borrow::Cow::Borrowed(bytes), ArchiveChildren::empty()));
    }
    let archive = semio_framework_os_channel::decode_document_archive_head_controlled(bytes,control).map_err(IoError::from_value_error)?;
    Ok((std::borrow::Cow::Owned(archive.parent_pack), ArchiveChildren { members: archive.members }))
}
//#endregion 🔖️ArchiveChildren

//#region 🔖️DslTxtCarrier
/// 🗒️ Serialize any `ArtifactDsl` document as UTF-8 carrier text (stdio.txt body law).
pub fn serialize_dsl_txt<S: pack::snapshot::ArtifactDsl>(from: &S) -> IoResult<IoPayload> {
    Ok(IoOutcome::clean(IoPayload::Text(pack::snapshot::ArtifactDsl::print_dsl(from))))
}

/// 📃️ Parse carrier text (or UTF-8 binary) back into an `ArtifactDsl` document.
pub fn deserialize_dsl_txt<S: pack::snapshot::ArtifactDsl>(payload: &IoPayload) -> IoResult<S> {
    let text = match payload {
        IoPayload::Text(text) => text.as_str(),
        IoPayload::Binary(bytes) => std::str::from_utf8(bytes).map_err(|error| IoError::from_value_error(ValueError::new(ValueRefusalKind::InvalidValue, error.to_string())))?,
    };
    S::parse_dsl(text).map(IoOutcome::clean).map_err(text_refusal)
}

fn refusal(kind: ValueRefusalKind, message: &'static str) -> IoError { IoError::from_value_error(ValueError::literal(kind, message)) }

fn text_refusal(error: semio_framework_diagnostic::TextError) -> IoError {
    let mut callback = |_| true;
    let mut control = semio_framework_value::NativeEncodeControl::new(isize::MAX as usize, &mut callback);
    IoError::from_text_error_controlled(error, &mut control).unwrap_or_else(IoError::from_value_error)
}
//#endregion 🔖️DslTxtCarrier

//#region 🔖️Entry
/// 🧲️ The side of an entry that is the registering artifact's native dialect, declared by the constructor and never inferred: an
/// `Export` entry serializes out of it (`from` is native), an `Import` entry deserializes into it (`into` is native).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum IoEntryDirection {
    Export,
    Import,
}

/// 🛤️ One registered entry seen from its native side — the shape a package descriptor lists as an import/export row.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct IoNativeRoute {
    pub native: ArtifactDialect,
    pub foreign: ArtifactDialect,
    pub direction: IoEntryDirection,
}

/// 🧾️ Type-erased io vtable row: one directed hop `from -> into` at a declared `fidelity`, its native side `direction`, an
/// optional `sniff` (carrier-dialect identification), and the erased `run` this hop executes.
pub struct IoEntry {
    pub owned_serializer: Option<OwnedSerializerFactory>,
    pub from: Dialect,
    pub into: Dialect,
    pub fidelity: IoFidelity,
    pub direction: IoEntryDirection,
    pub sniff: Option<fn(&IoPayload) -> Confidence>,
    pub run: fn(&IoPayload, &mut IoRunControl<'_, '_>) -> IoResult<IoPayload>,
}

fn same_io_entry(left: &IoEntry, right: &IoEntry) -> bool {
    let same_sniff = match (left.sniff, right.sniff) {
        (Some(a), Some(b)) => std::ptr::fn_addr_eq(a, b),
        (None, None) => true,
        _ => false,
    };
    ArtifactDialect::from(left.from) == ArtifactDialect::from(right.from) && ArtifactDialect::from(left.into) == ArtifactDialect::from(right.into) && left.fidelity == right.fidelity && left.direction == right.direction && same_sniff && std::ptr::fn_addr_eq(left.run, right.run) && same_owned_serializer(left.owned_serializer,right.owned_serializer)
}

fn descriptor_of(entry: &IoEntry) -> IoEntryDescriptor {
    IoEntryDescriptor { from: ArtifactDialect::from(entry.from), into: ArtifactDialect::from(entry.into), fidelity: entry.fidelity, sniffs: entry.sniff.is_some() }
}
//#endregion 🔖️Entry

//#region 🔖️Registry
type EntryKey = (ArtifactDialect, ArtifactDialect);
type EntryMap = BTreeMap<EntryKey, &'static IoEntry>;

static IO_MECHANISM_REGISTRY: std::sync::OnceLock<RwLock<EntryMap>> = std::sync::OnceLock::new();

fn io_mechanism_registry() -> &'static RwLock<EntryMap> {
    IO_MECHANISM_REGISTRY.get_or_init(|| RwLock::new(BTreeMap::new()))
}

/// 🧷️ Exact domain identity and semantic snapshot provider published by artifact assembly.
#[derive(Clone)]
pub struct NativeSnapshotRegistration {
    pub dialect: ArtifactDialect,
    pub codec: semio_framework_os_store_text_io::ArtifactCodec,
}

impl NativeSnapshotRegistration {
    /// 🪶️ Publishes a snapshot registration only for an owner-declared relational capability.
    pub fn from_capability(dialect: ArtifactDialect, codec: semio_framework_os_store_text_io::ArtifactCodec) -> Option<Self> {
        codec.snapshot_sqlite.as_ref()?;
        Some(Self { dialect, codec })
    }
}

pub(super) type NativeSnapshotMap = BTreeMap<ArtifactDialect, semio_framework_os_store_text_io::ArtifactCodec>;
static NATIVE_SNAPSHOT_REGISTRY: std::sync::OnceLock<RwLock<NativeSnapshotMap>> = std::sync::OnceLock::new();

pub(super) fn native_snapshot_registry() -> &'static RwLock<NativeSnapshotMap> {
    NATIVE_SNAPSHOT_REGISTRY.get_or_init(|| RwLock::new(BTreeMap::new()))
}

pub(super) fn propose_native_snapshots(existing: &NativeSnapshotMap, registrations: &[NativeSnapshotRegistration]) -> Result<NativeSnapshotMap, IoRegistryError> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

    let mut proposed = NativeSnapshotMap::new();
    for registration in registrations {
        let dialect = &registration.dialect;
        let coordinate = dialect.to_coordinate();
        if dialect == &ArtifactDialect::from(SQLITE_SNAPSHOT) || ArtifactDialect::parse_coordinate(&coordinate).as_ref() != Ok(dialect) || coordinate.chars().any(char::is_control) || registration.codec.snapshot_sqlite.is_none() {
            return Err(IoRegistryError::InvalidSnapshotDialect(dialect.clone()));
        }
        let provider = registration.codec.snapshot_sqlite.as_ref().expect("checked explicit SQLite provider");
        if !registration.codec.native_identity.validates_provider(&registration.codec.schema, Some(provider)) {
            return Err(IoRegistryError::InvalidSnapshotSchema { dialect: dialect.clone(), message: "relational capability disagrees with original native snapshot identity".to_string() });
        }
        let schema = SqliteDatabase::from_schema(&provider.schema).map_err(|error| IoRegistryError::InvalidSnapshotSchema { dialect: dialect.clone(), message: error.to_string() })?;
        if schema.tables.is_empty() || schema.tables.iter().any(|table| table.name.eq_ignore_ascii_case("semio_snapshot")) {
            return Err(IoRegistryError::InvalidSnapshotSchema { dialect: dialect.clone(), message: "artifact SQLite schema must contain domain tables and cannot claim reserved snapshot metadata".to_string() });
        }
        for current in [existing.get(dialect), proposed.get(dialect)].into_iter().flatten() {
            if current.schema != registration.codec.schema || current.native_identity != registration.codec.native_identity || current.extension != registration.codec.extension || current.pack_schema_hash != registration.codec.pack_schema_hash || !current.snapshot_sqlite.as_ref().expect("registered relational provider").identical_to(&registration.codec.snapshot_sqlite.as_ref().expect("validated relational provider")) {
                return Err(IoRegistryError::Duplicate { from: Box::new(dialect.clone()), into: Box::new(ArtifactDialect::from(SQLITE_SNAPSHOT)) });
            }
        }
        proposed.insert(dialect.clone(), registration.codec.clone());
    }
    Ok(proposed)
}

/// 🧭️ Publishes original native owners, their document codecs, and exact binding rows atomically.
pub fn register_native_snapshots_in_assembly(assembly: &semio_framework_schema_registry::assembly::Transaction, registrations: &[NativeSnapshotRegistration]) -> Result<(), crate::lifted_composer_registry::ArtifactAssemblyRegistryError> {
    let document_codecs=registrations.iter().map(|registration|registration.codec.clone()).collect();
    let document_bindings=registrations.iter().map(|registration|{
        let contributor=match &registration.codec.native_identity{semio_framework_os_store_text_io::ArtifactNativeSnapshotIdentity::Typed{owner,..}=>owner.to_string(),semio_framework_os_store_text_io::ArtifactNativeSnapshotIdentity::Guest{plugin_id,..}=>plugin_id.clone()};
        semio_framework_os_store_codec_registry::composer_registry::bindings::ArtifactCodecBinding::from_codec(semio_framework_os_store_codec_registry::composer_registry::bindings::ArtifactCodecBindingChannel::DirectNative,contributor,Some(registration.dialect.artifact_kind.clone()),Vec::new(),Some(registration.dialect.clone()),None,&registration.codec)
    }).collect();
    crate::lifted_composer_registry::commit_artifact_assembly_registry_plan(assembly,crate::lifted_composer_registry::ArtifactAssemblyRegistryPlan{document_codecs,document_bindings,native_snapshots:registrations.to_vec(),..Default::default()})
}

/// 🔎️ Checks snapshot registration conflicts before any assembly publication.
pub fn preflight_native_snapshots(registrations: &[NativeSnapshotRegistration]) -> Result<(), IoRegistryError> {
    let registry = native_snapshot_registry().read().map_err(|_| IoRegistryError::Unavailable)?;
    propose_native_snapshots(&registry, registrations).map(|_| ())
}

/// 🏛️ Resolves the exact registered dialect's handwritten semantic SQL schema.
pub fn native_snapshot_sqlite_schema(dialect: &ArtifactDialect) -> Result<String, IoRegistryError> {
    let registry = native_snapshot_registry().read().map_err(|_| IoRegistryError::Unavailable)?;
    let codec = registry.get(dialect).ok_or_else(|| IoRegistryError::InvalidSnapshotDialect(dialect.clone()))?;
    let provider = codec.snapshot_sqlite.as_ref().ok_or_else(|| IoRegistryError::InvalidSnapshotDialect(dialect.clone()))?;
    Ok(provider.schema.to_string())
}

fn snapshot_descriptor(from: ArtifactDialect, into: ArtifactDialect) -> IoEntryDescriptor {
    IoEntryDescriptor { from, into, fidelity: IoFidelity::Exact, sniffs: false }
}

fn is_sqlite_snapshot(dialect:&ArtifactDialect)->bool{dialect.artifact_kind==SQLITE_SNAPSHOT.artifact_kind&&dialect.standard.as_str()==SQLITE_SNAPSHOT.standard.0&&dialect.subset.as_str()==SQLITE_SNAPSHOT.subset.0}

fn supplemental_snapshot_entry(snapshots: &NativeSnapshotMap, from: &ArtifactDialect, into: &ArtifactDialect) -> Option<IoEntryDescriptor> {
    if (is_sqlite_snapshot(into) && snapshots.contains_key(from)) || (is_sqlite_snapshot(from) && snapshots.contains_key(into)) {
        Some(snapshot_descriptor(from.clone(), into.clone()))
    } else {
        None
    }
}

const SNAPSHOT_METADATA_SQL: &str = include_str!("../../../../../🔨️modules/🚪️io/🪶️sqlite-snapshot/🧬️schema/🗄️.sql");

fn validate_snapshot_schema(database: &SqliteDatabase, schema: &str, phase: SqliteSnapshotPhase, control: &mut SqliteSnapshotControl<'_>) -> Result<(), ValueError> {
    semio_framework_io_sqlite_snapshot::validate_sqlite_database_schema_controlled(database, schema, phase, control)
}

fn validate_snapshot_coordinate(kind: &str, standard: &str, subset: &str, control: &mut SqliteSnapshotControl<'_>, phase: SqliteSnapshotPhase) -> Result<(), ValueError> {
    if kind.is_empty() || standard.is_empty() || subset.is_empty() { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "snapshot SQLite metadata dialect is invalid")); }
    for (index, text) in [kind, standard, subset].iter().enumerate() {
        let mut position = 0usize;
        let mut characters = 0usize;
        for character in text.chars() {
            if character.is_control() || index == 0 && character == '@' || index == 2 && character == '/' { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "snapshot SQLite metadata dialect is invalid")); }
            position += character.len_utf8();
            characters += 1;
            if characters % 256 == 0 { control.checkpoint(phase, position, text.len())?; }
        }
        control.checkpoint(phase, position, text.len())?;
    }
    Ok(())
}

fn validate_snapshot_metadata<'a>(table: &'a SqliteTable, control: &mut SqliteSnapshotControl<'_>) -> Result<(&'a str, &'a str, &'a str, SnapshotEncoding), ValueError> {
    semio_framework_io_sqlite_snapshot::validate_sqlite_table_schema_controlled(table, SNAPSHOT_METADATA_SQL, SqliteSnapshotPhase::ReconstructSnapshot, control)?;
    if table.rows.len() != 1 || table.rows[0].rowid != 1 { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "snapshot SQLite metadata schema or row count is invalid")); }
    let [SqliteValue::Integer(1), SqliteValue::Text(kind), SqliteValue::Text(standard), SqliteValue::Text(subset), SqliteValue::Integer(1), SqliteValue::Text(encoding)] = table.rows[0].values.as_slice() else { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "snapshot SQLite metadata values are invalid")); };
    validate_snapshot_coordinate(kind, standard, subset, control, SqliteSnapshotPhase::ReconstructSnapshot)?;
    Ok((kind, standard, subset, SnapshotEncoding::parse(encoding)?))
}

/// 🪪️ Reads the decomposed coordinate using an operation-owned admission authority.
pub fn sqlite_snapshot_metadata(database: &SqliteDatabase) -> Result<(ArtifactDialect, SnapshotEncoding), ValueError> {
    let mut callback = |_| true;
    let mut control = SqliteSnapshotControl::new(&mut callback, SqliteDatabaseLimits::default());
    let table = database.tables.iter().find(|table| table.name == "semio_snapshot").ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue, "SQLite file has no semantic snapshot metadata"))?;
    let (kind, standard, subset, encoding) = validate_snapshot_metadata(table, &mut control)?;
    let copy = semio_framework_io_sqlite_snapshot::transfer::copy_text;
    Ok((ArtifactDialect { artifact_kind: copy(kind, SqliteSnapshotPhase::ReconstructSnapshot, &mut control)?, standard: copy(standard, SqliteSnapshotPhase::ReconstructSnapshot, &mut control)?, subset: copy(subset, SqliteSnapshotPhase::ReconstructSnapshot, &mut control)? }, encoding))
}

/// 🪪️ Admits the literal metadata cells and their actual backing before ownership.
pub fn attach_sqlite_snapshot_metadata(database: &mut SqliteDatabase, dialect: &ArtifactDialect, encoding: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<(), ValueError> {
    use semio_framework_io_sqlite_snapshot::transfer::{reserve, copy_text};
    let phase = SqliteSnapshotPhase::ProjectSnapshot;
    control.checkpoint(phase, 0, 0)?;
    validate_snapshot_coordinate(&dialect.artifact_kind, &dialect.standard, &dialect.subset, control, phase)?;
    if database.tables.iter().any(|table| table.name.eq_ignore_ascii_case("semio_snapshot")) { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "domain schema cannot claim reserved snapshot metadata")); }
    let limits = control.limits();
    if database.tables.len().checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "snapshot table count overflow"))? > limits.max_tables || limits.max_columns < 6 { return Err(ValueError::new(ValueRefusalKind::WorkLimit, "snapshot metadata exceeds table or column limit")); }
    let mut rows = 1usize;
    let sql = SNAPSHOT_METADATA_SQL.trim().trim_end_matches(';');
    let mut bytes = 16usize.checked_add(dialect.artifact_kind.len()).and_then(|v| v.checked_add(dialect.standard.len())).and_then(|v| v.checked_add(dialect.subset.len())).and_then(|v| v.checked_add(encoding.as_str().len())).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "snapshot metadata byte count overflow"))?;
    let mut schema_bytes = sql.len().checked_add("semio_snapshot".len()).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "snapshot schema byte count overflow"))?;
    for table in &database.tables {
        schema_bytes = schema_bytes.checked_add(table.sql.len()).and_then(|v| v.checked_add(table.name.len())).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "snapshot schema byte count overflow"))?;
        for row in &table.rows {
            rows = rows.checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "snapshot row count overflow"))?;
            for value in &row.values { bytes = bytes.checked_add(match value { SqliteValue::Null => 0, SqliteValue::Integer(_) | SqliteValue::Real(_) => 8, SqliteValue::Text(text) => text.len(), SqliteValue::Blob(blob) => blob.len() }).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "snapshot value byte count overflow"))?; }
            if rows % 256 == 0 { control.checkpoint(phase, rows, 0)?; }
        }
    }
    control.check_rows(rows)?;
    control.check_value_bytes(bytes)?;
    if schema_bytes > limits.max_schema_bytes { return Err(ValueError::new(ValueRefusalKind::OwnershipLimit, "snapshot metadata exceeds schema byte limit")); }
    let mut values = reserve(6, control)?;
    values.push(SqliteValue::Integer(1));
    for text in [&dialect.artifact_kind, &dialect.standard, &dialect.subset] { values.push(SqliteValue::Text(copy_text(text, phase, control)?)); }
    values.push(SqliteValue::Integer(1));
    values.push(SqliteValue::Text(copy_text(encoding.as_str(), phase, control)?));
    let mut metadata_rows = reserve(1, control)?;
    metadata_rows.push(SqliteRow { rowid: 1, values });
    let name = copy_text("semio_snapshot", phase, control)?;
    let sql = copy_text(sql, phase, control)?;
    if database.tables.len() == database.tables.capacity() {
        let mut tables = reserve(database.tables.len().checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "snapshot table count overflow"))?, control)?;
        let total = database.tables.len();
        for (index, table) in database.tables.drain(..).enumerate() { tables.push(table); if index % 256 == 0 { control.checkpoint(phase, index, total)?; } }
        database.tables = tables;
    }
    database.tables.push(SqliteTable { name, sql, rows: metadata_rows });
    control.checkpoint(phase, rows, rows)
}

/// 🪪️ Moves validated metadata out without cloning its literal coordinate fields.
pub fn take_sqlite_snapshot_metadata_controlled(database: &mut SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<(ArtifactDialect, SnapshotEncoding), ValueError> {
    let index = database.tables.iter().position(|table| table.name == "semio_snapshot").ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue, "SQLite file has no semantic snapshot metadata"))?;
    let (_, _, _, encoding) = validate_snapshot_metadata(&database.tables[index], control)?;
    let count = database.tables.len();
    for at in index..count - 1 { database.tables.swap(at, at + 1); if at % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, at, count)?; } }
    let mut table = database.tables.pop().ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "snapshot metadata disappeared"))?;
    let mut row = table.rows.pop().ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "snapshot metadata row disappeared"))?;
    let mut values = row.values.drain(..);
    values.next();
    let kind = match values.next() { Some(SqliteValue::Text(v)) => v, _ => return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "snapshot metadata kind disappeared")) };
    let standard = match values.next() { Some(SqliteValue::Text(v)) => v, _ => return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "snapshot metadata standard disappeared")) };
    let subset = match values.next() { Some(SqliteValue::Text(v)) => v, _ => return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "snapshot metadata subset disappeared")) };
    Ok((ArtifactDialect { artifact_kind: kind, standard, subset }, encoding))
}

/// 🪪️ Extracts metadata under one operation-owned control.
pub fn take_sqlite_snapshot_metadata(database: &mut SqliteDatabase) -> Result<(ArtifactDialect, SnapshotEncoding), ValueError> {
    take_sqlite_snapshot_metadata_controlled(database, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default()))
}
// 🐛️ terra-io-thunks: every call site below awaits this before `.read()`/`.write()` — the
// codemod that added `async` here left all five callers unfixed (the `no .await needed` case
// insert-await.py handles cannot help: cargo never reached this file to emit the diagnostic,
// see 📓️terra-io-thunks-report.md).

/// ⚠️ A duplicate `(from, into)` registration for a genuinely different entry, or a poisoned
/// registry lock.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IoRegistryError {
    Duplicate { from: Box<ArtifactDialect>, into: Box<ArtifactDialect> },
    InvalidSnapshotDialect(ArtifactDialect),
    InvalidSnapshotSchema { dialect: ArtifactDialect, message: String },
    Unavailable,
}

impl std::fmt::Display for IoRegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

        match self {
            Self::Duplicate { from, into } => write!(f, "io entry already registered for {} -> {}", from.to_coordinate(), into.to_coordinate()),
            Self::InvalidSnapshotDialect(dialect) => write!(f, "invalid native snapshot dialect {}", dialect.to_coordinate()),
            Self::InvalidSnapshotSchema { dialect, message } => write!(f, "invalid semantic SQLite schema for {}: {message}", dialect.to_coordinate()),
            Self::Unavailable => f.write_str("io mechanism registry unavailable"),
        }
    }
}
impl std::error::Error for IoRegistryError {}

// 🐛️ terra-io-thunks: was a plain `fn` calling the now-async `same_io_entry` from an
// unawaitable match guard (E0308: expected bool, found future). `build_proposed` is a regular
// helper, not a fn-pointer slot, so per R2's O1 it keeps the literal `async` keyword rather
// than staying sync — restructured off the match guard so the `.await` has a plain expression
// position to live in.
fn build_proposed(entries: &[&'static IoEntry]) -> Result<EntryMap, IoRegistryError> {
    let mut proposed: EntryMap = BTreeMap::new();
    for entry in entries {
        let key = (ArtifactDialect::from(entry.from), ArtifactDialect::from(entry.into));
        if key.0 == ArtifactDialect::from(SQLITE_SNAPSHOT) || key.1 == ArtifactDialect::from(SQLITE_SNAPSHOT) {
            return Err(IoRegistryError::InvalidSnapshotDialect(ArtifactDialect::from(SQLITE_SNAPSHOT)));
        }
        if let Some(existing) = proposed.get(&key) {
            if same_io_entry(existing, entry) {
                continue;
            }
            return Err(IoRegistryError::Duplicate { from: Box::new(key.0), into: Box::new(key.1) });
        }
        proposed.insert(key, entry);
    }
    Ok(proposed)
}

fn validate_against(existing: &EntryMap, proposed: &EntryMap) -> Result<(), IoRegistryError> {
    for (key, entry) in proposed {
        if let Some(current) = existing.get(key) {
            if !same_io_entry(current, entry) {
                return Err(IoRegistryError::Duplicate { from: Box::new(key.0.clone()), into: Box::new(key.1.clone()) });
            }
        }
    }
    Ok(())
}

/// 📌️ Registers `entries` atomically under the SAME `semio_framework_schema_registry::assembly::begin()`
/// publication barrier `register_composer_entries` (the OLD mechanism, above) uses, so a
/// plugin's old- and new-mechanism registrations never interleave. A duplicate `(from, into)`
/// key for a genuinely DIFFERENT entry is a typed error and leaves the registry unchanged
/// (preflight via `build_proposed`+`validate_against`, THEN commit — all-or-nothing, proven by
/// `registration_is_all_or_nothing` below); re-registering the identical static entry is
/// idempotent, mirroring `register_composer_entries`.
pub fn io_register(entries: &'static [IoEntry]) -> Result<(), IoRegistryError> {
    let _assembly = semio_framework_schema_registry::assembly::begin().map_err(|_| IoRegistryError::Unavailable)?;
    let proposed = build_proposed(&entries.iter().collect::<Vec<_>>())?;
    let mut registry = io_mechanism_registry().write().map_err(|_| IoRegistryError::Unavailable)?;
    validate_against(&registry, &proposed)?;
    for (key, entry) in proposed {
        registry.entry(key).or_insert(entry);
    }
    Ok(())
}
//#endregion 🔖️Registry

//#region 🔖️Route
// 🐛️ terra-io-thunks: recursive `async fn` needs its own recursive call boxed (E0733) — the
// codemod added `async` to a self-recursive traversal without that step. `Box::pin(..).await`
// is the standard idiom, not an E-class exception (nothing here is a fn-pointer slot).
async fn walk_routes(registry: &EntryMap, current: &ArtifactDialect, into: &ArtifactDialect, remaining_hops: u8, path: &mut Vec<&'static IoEntry>, visited: &mut BTreeSet<ArtifactDialect>, candidates: &mut Vec<Vec<&'static IoEntry>>) {
    if remaining_hops == 0 {
        return;
    }
    for (key, entry) in registry.iter() {
        if &key.0 != current || visited.contains(&key.1) {
            continue;
        }
        let next = key.1.clone();
        path.push(entry);
        if &next == into {
            candidates.push(path.clone());
        } else {
            visited.insert(next.clone());
            Box::pin(walk_routes(registry, &next, into, remaining_hops - 1, path, visited, candidates)).await;
            visited.remove(&next);
        }
        path.pop();
    }
}

async fn route_rank(route: &[&'static IoEntry]) -> (std::cmp::Reverse<u8>, usize, String) {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

    let min_fidelity = route.iter().map(|entry| entry.fidelity.rank()).min().unwrap_or(0);
    let joined = route.iter().map(|entry| ArtifactDialect::from(entry.into).to_coordinate()).collect::<Vec<_>>().join(",");
    (std::cmp::Reverse(min_fidelity), route.len(), joined)
}

async fn rank_to_fidelity(rank: u8) -> IoFidelity {
    match rank {
        3 => IoFidelity::Exact,
        2 => IoFidelity::Canonical,
        1 => IoFidelity::Semantic,
        _ => IoFidelity::Lossy,
    }
}

/// 🌉️ Breadth-bounded, cycle-free enumeration of every simple path `from -> into` up to
/// `max_hops` (clamped to ≤3), picking the best by (1) highest MINIMUM fidelity along the
/// route, (2) fewest hops, (3) lexicographic order of the joined `to_coordinate()` strings of
/// every dialect visited after `from`. Sorting the FULL candidate set at the end (rather than
/// short-circuiting on the first hit) plus the registry already being a `BTreeMap` (whose
/// iteration order never depends on insertion order) together give `route_is_deterministic`.
// 🐛️ terra-io-thunks: `Vec::sort_by`'s comparator is a plain synchronous `FnMut` (std's fixed
// signature, not ours to make async) so it can never itself call the now-`async`
// `route_rank`/`rank_to_fidelity`/`descriptor_of` — those stay `async fn` per O1 (they are
// regular helpers, not fn-pointer slots) and this function awaits them into plain values FIRST,
// then sorts/maps over the already-resolved data synchronously.
async fn resolve_route(registry: &EntryMap, from: &ArtifactDialect, into: &ArtifactDialect, max_hops: u8) -> IoResult<IoRoute> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

    let max_hops = max_hops.min(3);
    if max_hops == 0 {
        return Err(IoError::from_value_error(ValueError::new(ValueRefusalKind::InvalidValue, format!("io_route {} -> {}: max_hops clamped to 0", from.to_coordinate(), into.to_coordinate()))));
    }
    let mut candidates: Vec<Vec<&'static IoEntry>> = Vec::new();
    let mut path: Vec<&'static IoEntry> = Vec::new();
    let mut visited: BTreeSet<ArtifactDialect> = BTreeSet::new();
    visited.insert(from.clone());
    walk_routes(registry, from, into, max_hops, &mut path, &mut visited, &mut candidates).await;
    if candidates.is_empty() {
        return Err(IoError::from_value_error(ValueError::new(ValueRefusalKind::UnsupportedOwner, format!("no io route from {} to {} within {max_hops} hops", from.to_coordinate(), into.to_coordinate()))));
    }
    let mut ranked = Vec::with_capacity(candidates.len());
    for route in candidates {
        let rank = route_rank(&route).await;
        ranked.push((rank, route));
    }
    ranked.sort_by(|a, b| a.0.cmp(&b.0));
    let best = ranked.into_iter().next().expect("candidates checked non-empty above").1;
    let fidelity = rank_to_fidelity(best.iter().map(|entry| entry.fidelity.rank()).min().expect("a route has at least one hop")).await;
    let mut hops = Vec::with_capacity(best.len());
    for entry in &best {
        hops.push(descriptor_of(entry));
    }
    Ok(IoOutcome::clean(IoRoute { hops, fidelity }))
}

/// 🌉️ `resolve_route` against the process-wide registry.
pub async fn io_route(from: &ArtifactDialect, into: &ArtifactDialect, max_hops: u8) -> IoResult<IoRoute> {
    if max_hops > 0 {
        let snapshots = native_snapshot_registry().read().map_err(|_| refusal(ValueRefusalKind::InvariantViolated, "native snapshot registry unavailable"))?;
        if let Some(hop) = supplemental_snapshot_entry(&snapshots, from, into) {
            return Ok(IoOutcome::clean(IoRoute { hops: vec![hop], fidelity: IoFidelity::Exact }));
        }
    }
    let registry = io_mechanism_registry().read().map_err(|_| refusal(ValueRefusalKind::InvariantViolated, "io mechanism registry unavailable"))?.clone();
    resolve_route(&registry, from, into, max_hops).await
}

fn typed_snapshot_codec<P:semio_framework_os_store_space_history::lifted_serialization::ArtifactSqliteSnapshotHost+'static>(dialect:&ArtifactDialect)->Result<(),IoError>{
    let registry=native_snapshot_registry().read().map_err(|_|refusal(ValueRefusalKind::InvariantViolated,"native snapshot registry unavailable"))?;
    let codec=registry.get(dialect).ok_or_else(||refusal(ValueRefusalKind::UnsupportedOwner,"unregistered typed snapshot dialect"))?;
    let provider=codec.snapshot_sqlite.as_ref().ok_or_else(||refusal(ValueRefusalKind::UnsupportedOwner,"artifact has no semantic SQLite provider"))?;
    if provider.snapshot_type!=Some(std::any::TypeId::of::<P>())||provider.schema!=P::SQLITE_SCHEMA{return Err(refusal(ValueRefusalKind::InvalidValue,"typed snapshot does not own the requested exact dialect"))}Ok(())
}

#[derive(semio_framework_value::RetireOwned)]
struct TypedSnapshotExportFrame{metadata:SnapshotMetadataFrame,physical:semio_framework_io_sqlite_snapshot::PhysicalExportFrame,validation:Option<IoResult<()>>,result:Option<IoResult<Vec<u8>>>}
impl TypedSnapshotExportFrame{fn new()->Self{Self{metadata:SnapshotMetadataFrame::new(),physical:semio_framework_io_sqlite_snapshot::PhysicalExportFrame::new(),validation:None,result:None}}}
#[derive(semio_framework_value::RetireOwned)]
struct TypedSnapshotImportFrame<P:semio_framework_value::retirement::RetireOwned>{metadata:SnapshotMetadataFrame,physical:semio_framework_io_sqlite_snapshot::PhysicalImportFrame,snapshot:Option<P>,validation:Option<IoResult<()>>,result:Option<IoResult<P>>}
impl<P:semio_framework_value::retirement::RetireOwned> TypedSnapshotImportFrame<P>{fn new()->Self{Self{metadata:SnapshotMetadataFrame::new(),physical:semio_framework_io_sqlite_snapshot::PhysicalImportFrame::new(),snapshot:None,validation:None,result:None}}}

fn typed_file_validation(slot:&mut Option<IoResult<()>>)->Result<(),IoError>{
    match slot.as_ref().unwrap(){Err(_)=>Err(slot.take().unwrap().unwrap_err()),Ok(outcome)if outcome.diagnostics.iter().any(|diagnostic|matches!(diagnostic.severity,semio_framework_diagnostic::Severity::Error|semio_framework_diagnostic::Severity::Fatal))=>Err(IoError{cause:ValueError::literal(ValueRefusalKind::InvalidValue,"owned snapshot violates its requested subset"),diagnostics:slot.take().unwrap().unwrap().diagnostics}),Ok(_)=>Ok(())}
}
macro_rules! typed_file_work{
    ($native:ident,$control:ident,$direction:ident,$bytes:expr)=>{{
        let mut body=semio_framework_io_control::NativeSnapshotBodyWallet::new($native.remaining_grant());
        let result=semio_framework_io_sqlite_snapshot::artifact::receiving::Port{control:$control,native:semio_framework_io_sqlite_snapshot::artifact::receiving::Direction::$direction($native.native()),body:&mut body}.work(1,$bytes);
        $native.record_progress(body.progress())?;result
    }};
}

/// 📤️ Exports every complete snapshot field through the supplied original SQL and native receiving authorities.
pub async fn io_export_sqlite_snapshot<P:semio_framework_os_store_space_history::lifted_serialization::ArtifactSqliteSnapshotHost+semio_framework_value::retirement::RetireOwned+'static>(dialect:&ArtifactDialect,snapshot:&P,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native:&mut semio_framework_io_control::NativeSnapshotDecodeOwner<'_,'_>)->IoResult<Vec<u8>>{
    export_sqlite_snapshot_receiving(dialect,snapshot,encoding,control,native)
}

/// 🧱️ Shares the original complete file producer with synchronous registered snapshot callbacks.
pub(crate) fn export_sqlite_snapshot_receiving<P:semio_framework_os_store_space_history::lifted_serialization::ArtifactSqliteSnapshotHost+semio_framework_value::retirement::RetireOwned+'static>(dialect:&ArtifactDialect,snapshot:&P,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native:&mut semio_framework_io_control::NativeSnapshotDecodeOwner<'_,'_>)->IoResult<Vec<u8>>{
    let result=(||{
        typed_snapshot_codec::<P>(dialect)?;native.native().checkpoint().map_err(IoError::from_value_error)?;
        let before=native.native().owned_bytes();let maximum=before.checked_add(control.allocation_remaining_bytes()).ok_or_else(||refusal(ValueRefusalKind::OwnershipLimit,"typed snapshot original SQL ceiling overflow"))?;
        let settlement=TypedFileSettlement::new(control,before,SqliteSnapshotPhase::WritePages);
        let mut observer=|event:semio_framework_value_native_owned::native_decoding::NativeDecodeProgress|settlement.observe(event.owned_bytes,event.completed,event.total);
        let received=native.scoped_native(maximum,&mut observer,|native|native.receive_nested::<TypedSnapshotExportFrame,IoResult<Vec<u8>>>(|slot,native|{
            let mut borrowed=settlement.control.borrow_mut();let control=&mut **borrowed;
            typed_file_work!(native,control,Decode,size_of::<TypedSnapshotExportFrame>())?;*slot=Some(TypedSnapshotExportFrame::new());let frame=slot.as_mut().unwrap();
            typed_file_work!(native,control,Decode,size_of::<IoResult<Vec<u8>>>())?;
            frame.result=Some((||{
                control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0).map_err(IoError::from_value_error)?;
                typed_file_work!(native,control,Decode,size_of::<Option<SqliteDatabase>>()).map_err(IoError::from_value_error)?;
                frame.metadata.database=Some(snapshot.to_sqlite_database_receiving(control,native).map_err(IoError::from_value_error)?);
                validate_snapshot_schema_decoding(frame.metadata.database.as_ref().unwrap(),P::SQLITE_SCHEMA,control,native).map_err(IoError::from_value_error)?;
                typed_file_work!(native,control,Decode,size_of::<IoResult<()>>()).map_err(IoError::from_value_error)?;
                frame.validation=Some(snapshot.validate_sqlite_snapshot_subset_decoding(dialect,frame.metadata.database.as_ref().unwrap(),control,native));
                typed_file_validation(&mut frame.validation)?;
                attach_metadata_decoding(&mut frame.metadata,dialect,encoding,control,native).map_err(IoError::from_value_error)?;
                let mut body=semio_framework_io_control::NativeSnapshotBodyWallet::new(native.remaining_grant());
                let physical=semio_framework_io_sqlite_snapshot::export_sqlite_database_receiving_into(frame.metadata.database.as_ref().unwrap(),&mut frame.physical,&mut semio_framework_io_sqlite_snapshot::artifact::receiving::Port{control,native:semio_framework_io_sqlite_snapshot::artifact::receiving::Direction::Decode(native.native()),body:&mut body});
                native.record_progress(body.progress()).map_err(IoError::from_value_error)?;physical.map_err(IoError::from_value_error)?;
                typed_file_work!(native,control,Decode,size_of::<Vec<u8>>()+size_of::<Vec<Diagnostic>>()).map_err(IoError::from_value_error)?;
                Ok(IoOutcome{value:std::mem::take(&mut frame.physical.output),diagnostics:frame.validation.take().unwrap().unwrap().diagnostics})
            })());
            let owned=native.native().owned_bytes();drop(borrowed);settlement.finish_body(owned)?;Ok(frame.result.take().unwrap())
        }));
        match received{Ok(output)=>output,Err(error)=>Err(IoError::from_value_error(settlement.refused(native.native().owned_bytes(),error)))}
    })();result.map_err(|mut error|{error.cause=error.cause.with_retained_progress(native.progress());error})
}

/// 📥️ Imports every complete snapshot field while keeping parser, metadata and typed reconstruction under the original recipient.
pub async fn io_import_sqlite_snapshot<P:semio_framework_os_store_space_history::lifted_serialization::ArtifactSqliteSnapshotHost+semio_framework_value::retirement::RetireOwned+'static>(dialect:&ArtifactDialect,bytes:&[u8],control:&mut SqliteSnapshotControl<'_>,native:&mut semio_framework_io_control::NativeSnapshotEncodeOwner<'_,'_>)->IoResult<P>{
    import_sqlite_snapshot_receiving(dialect,bytes,control,native)
}

/// 🧩️ Keeps the complete typed import and original publication boundary shared by every caller.
pub(crate) fn import_sqlite_snapshot_receiving<P:semio_framework_os_store_space_history::lifted_serialization::ArtifactSqliteSnapshotHost+semio_framework_value::retirement::RetireOwned+'static>(dialect:&ArtifactDialect,bytes:&[u8],control:&mut SqliteSnapshotControl<'_>,native:&mut semio_framework_io_control::NativeSnapshotEncodeOwner<'_,'_>)->IoResult<P>{
    let result=(||{
        typed_snapshot_codec::<P>(dialect)?;native.native().checkpoint().map_err(IoError::from_value_error)?;
        let before=native.native().owned_bytes();let maximum=before.checked_add(control.allocation_remaining_bytes()).ok_or_else(||refusal(ValueRefusalKind::OwnershipLimit,"typed snapshot original SQL ceiling overflow"))?;
        let settlement=TypedFileSettlement::new(control,before,SqliteSnapshotPhase::ReconstructSnapshot);
        let mut observer=|event:semio_framework_value::native_encoding::NativeEncodeProgress|settlement.observe(event.owned_bytes,event.completed,event.total);
        let received=native.scoped_native(maximum,&mut observer,|native|native.receive_nested::<TypedSnapshotImportFrame<P>,IoResult<P>>(|slot,native|{
            let mut borrowed=settlement.control.borrow_mut();let control=&mut **borrowed;
            typed_file_work!(native,control,Encode,size_of::<TypedSnapshotImportFrame<P>>())?;*slot=Some(TypedSnapshotImportFrame::new());let frame=slot.as_mut().unwrap();
            typed_file_work!(native,control,Encode,size_of::<IoResult<P>>())?;
            frame.result=Some((||{
                let mut body=semio_framework_io_control::NativeSnapshotBodyWallet::new(native.remaining_grant());
                let physical=semio_framework_io_sqlite_snapshot::import_sqlite_database_receiving_into(bytes,&mut frame.physical,&mut semio_framework_io_sqlite_snapshot::artifact::receiving::Port{control,native:semio_framework_io_sqlite_snapshot::artifact::receiving::Direction::Encode(native.native()),body:&mut body});
                native.record_progress(body.progress()).map_err(IoError::from_value_error)?;physical.map_err(IoError::from_value_error)?;
                typed_file_work!(native,control,Encode,size_of::<SqliteDatabase>()).map_err(IoError::from_value_error)?;
                frame.metadata.database=Some(std::mem::replace(&mut frame.physical.database,SqliteDatabase{tables:Vec::new()}));
                take_metadata_encoding(&mut frame.metadata,control,native).map_err(IoError::from_value_error)?;
                if frame.metadata.dialect.as_ref().unwrap()!=dialect{return Err(refusal(ValueRefusalKind::InvalidValue,"typed SQLite snapshot dialect does not match requested dialect"))}
                validate_snapshot_schema_encoding(frame.metadata.database.as_ref().unwrap(),P::SQLITE_SCHEMA,control,native).map_err(IoError::from_value_error)?;
                control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,0,0).map_err(IoError::from_value_error)?;
                typed_file_work!(native,control,Encode,size_of::<Option<P>>()).map_err(IoError::from_value_error)?;
                frame.snapshot=Some(P::from_sqlite_database_receiving(frame.metadata.database.as_ref().unwrap(),control,native).map_err(IoError::from_value_error)?);
                typed_file_work!(native,control,Encode,size_of::<IoResult<()>>()).map_err(IoError::from_value_error)?;
                frame.validation=Some(frame.snapshot.as_ref().unwrap().validate_sqlite_snapshot_subset_encoding(dialect,frame.metadata.database.as_ref().unwrap(),control,native));
                typed_file_validation(&mut frame.validation)?;
                typed_file_work!(native,control,Encode,size_of::<P>()+size_of::<Vec<Diagnostic>>()).map_err(IoError::from_value_error)?;
                Ok(IoOutcome{value:frame.snapshot.take().unwrap(),diagnostics:frame.validation.take().unwrap().unwrap().diagnostics})
            })());
            let owned=native.native().owned_bytes();drop(borrowed);settlement.finish_body(owned)?;Ok(frame.result.take().unwrap())
        }));
        match received{Ok(output)=>output,Err(error)=>Err(IoError::from_value_error(settlement.refused(native.native().owned_bytes(),error)))}
    })();result.map_err(|mut error|{error.cause=error.cause.with_retained_progress(native.progress());error})
}

/// 🫳️ Exports the exact captured Store root through its registered complete snapshot type.
pub fn export_snapshot_file_read<P:semio_framework_os_store_space_history::lifted_serialization::ArtifactSqliteSnapshotHost+semio_framework_value::retirement::RetireOwned+Sync>(dialect:&ArtifactDialect,read:&semio_framework_os_store_read::ErasedSnapshotRead,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native:&mut semio_framework_io_control::NativeSnapshotDecodeOwner<'_,'_>)->IoResult<Vec<u8>>{
    let result=read.get::<P>().ok_or_else(||refusal(ValueRefusalKind::InvalidValue,"captured snapshot has the wrong registered type")).and_then(|snapshot|export_sqlite_snapshot_receiving(dialect,snapshot,encoding,control,native));
    result.map_err(|mut error|{error.cause=error.cause.with_retained_progress(native.progress());error})
}

/// 🪑️ Publishes a complete reconstructed snapshot into the caller's prevalidated empty typed slot.
pub fn import_snapshot_file_into<P:semio_framework_os_store_space_history::lifted_serialization::ArtifactSqliteSnapshotHost+semio_framework_value::retirement::RetireOwned>(dialect:&ArtifactDialect,bytes:&[u8],destination:&mut(dyn std::any::Any+Send),control:&mut SqliteSnapshotControl<'_>,native:&mut semio_framework_io_control::NativeSnapshotEncodeOwner<'_,'_>)->IoResult<()>{
    let result=(||{
        typed_snapshot_codec::<P>(dialect)?;
        let destination=destination.downcast_mut::<Option<P>>().ok_or_else(||refusal(ValueRefusalKind::InvalidValue,"snapshot file destination has the wrong registered type"))?;
        if destination.is_some(){return Err(refusal(ValueRefusalKind::InvalidValue,"snapshot file destination must be empty"))}
        let mut body=semio_framework_io_control::NativeSnapshotBodyWallet::new(native.remaining_grant());let payment=(||{let mut port=semio_framework_io_sqlite_snapshot::artifact::receiving::Port{control,native:semio_framework_io_sqlite_snapshot::artifact::receiving::Direction::Encode(native.native()),body:&mut body};port.work(2,size_of::<Option<P>>())?;port.work(2,size_of::<Vec<Diagnostic>>())?;port.work(2,size_of::<IoOutcome<()>>())?;Ok::<(),ValueError>(())})();native.record_progress(body.progress()).map_err(IoError::from_value_error)?;payment.map_err(IoError::from_value_error)?;
        let IoOutcome{value,diagnostics}=import_sqlite_snapshot_receiving::<P>(dialect,bytes,control,native)?;
        *destination=Some(value);Ok(IoOutcome{value:(),diagnostics})
    })();result.map_err(|mut error|{error.cause=error.cause.with_retained_progress(native.progress());error})
}

fn snapshot_file_callbacks(dialect:&ArtifactDialect)->Result<(Option<semio_framework_os_store_text_io::ArtifactSqliteSnapshotExportFile>,Option<semio_framework_os_store_text_io::ArtifactSqliteSnapshotImportFile>),IoError>{
    let registry=native_snapshot_registry().read().map_err(|_|refusal(ValueRefusalKind::InvariantViolated,"native snapshot registry unavailable"))?;
    let codec=registry.get(dialect).ok_or_else(||refusal(ValueRefusalKind::UnsupportedOwner,"unregistered typed snapshot dialect"))?;
    let provider=codec.snapshot_sqlite.as_ref().ok_or_else(||refusal(ValueRefusalKind::UnsupportedOwner,"artifact has no complete snapshot file capability"))?;
    if codec.native_identity.snapshot_type().is_none()||provider.snapshot_type.is_none(){return Err(refusal(ValueRefusalKind::UnsupportedOwner,"artifact has no complete snapshot file capability"))}
    if codec.native_identity.snapshot_type()!=provider.snapshot_type{return Err(refusal(ValueRefusalKind::InvalidValue,"complete snapshot provider identity differs"))}Ok((provider.export_file,provider.import_file))
}

/// 🌐️ Exports every complete field from a caller-owned captured read using the global IO registry.
pub async fn io_export_sqlite_snapshot_read(dialect:&ArtifactDialect,read:&semio_framework_os_store_read::ErasedSnapshotRead,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native:&mut semio_framework_io_control::NativeSnapshotDecodeOwner<'_,'_>)->IoResult<Vec<u8>>{
    let result=(||{let callback=snapshot_file_callbacks(dialect)?.0.ok_or_else(||refusal(ValueRefusalKind::UnsupportedOwner,"artifact has no complete snapshot file capability"))?;callback(dialect,read,encoding,control,native)})();result.map_err(|mut error|{error.cause=error.cause.with_retained_progress(native.progress());error})
}

/// 📨️ Imports through the registered complete snapshot owner into its actual caller-owned destination.
pub async fn io_import_sqlite_snapshot_into(dialect:&ArtifactDialect,bytes:&[u8],destination:&mut(dyn std::any::Any+Send),control:&mut SqliteSnapshotControl<'_>,native:&mut semio_framework_io_control::NativeSnapshotEncodeOwner<'_,'_>)->IoResult<()>{
    let result=(||{let callback=snapshot_file_callbacks(dialect)?.1.ok_or_else(||refusal(ValueRefusalKind::UnsupportedOwner,"artifact has no complete snapshot file capability"))?;callback(dialect,bytes,destination,control,native)})();result.map_err(|mut error|{error.cause=error.cause.with_retained_progress(native.progress());error})
}

fn validate_snapshot_metadata_receiving(table:&SqliteTable,port:&mut semio_framework_io_sqlite_snapshot::artifact::receiving::Port<'_,'_,'_,semio_framework_io_control::NativeSnapshotBodyWallet>)->Result<SnapshotEncoding,ValueError>{
    use semio_framework_io_sqlite_snapshot::artifact::receiving::validate_declaration;
    let invalid=||ValueError::literal(ValueRefusalKind::InvalidValue,"snapshot SQLite metadata is invalid");let limits=port.control.limits();if limits.max_columns<6||table.name.len().checked_add(table.sql.len()).is_none_or(|bytes|bytes>limits.max_schema_bytes){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot SQLite metadata exceeds original schema ceiling"))}
    validate_declaration(&table.sql,SNAPSHOT_METADATA_SQL,port)?;port.work(1,0)?;if table.name!="semio_snapshot"||table.rows.len()!=1||table.rows[0].rowid!=1{return Err(invalid())}
    let[SqliteValue::Integer(1),SqliteValue::Text(kind),SqliteValue::Text(standard),SqliteValue::Text(subset),SqliteValue::Integer(1),SqliteValue::Text(encoding)]=table.rows[0].values.as_slice()else{return Err(invalid())};
    for(index,text)in[kind,standard,subset].into_iter().enumerate(){if text.is_empty(){return Err(invalid())}let mut copied=0usize;for(position,character)in text.chars().enumerate(){if position%256==0{port.work(1,0)?}if character.is_control()||index==0&&character=='@'||index==2&&character=='/'{return Err(invalid())}copied=copied.checked_add(character.len_utf8()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot coordinate extent overflow"))?;}port.control.check_value_bytes(copied)?;}
    match encoding.as_str(){"binary"=>Ok(SnapshotEncoding::Binary),"text"=>Ok(SnapshotEncoding::Text),_=>Err(invalid())}
}

fn validate_snapshot_schema_decoding(database:&SqliteDatabase,schema:&str,control:&mut SqliteSnapshotControl<'_>,native:&mut semio_framework_io_control::NativeSnapshotDecodeOwner<'_,'_>)->Result<(),ValueError>{let mut body=semio_framework_io_control::NativeSnapshotBodyWallet::new(native.remaining_grant());let result=semio_framework_io_sqlite_snapshot::artifact::receiving::validate_schema_sql(database,schema,&mut semio_framework_io_sqlite_snapshot::artifact::receiving::Port{control,native:semio_framework_io_sqlite_snapshot::artifact::receiving::Direction::Decode(native.native()),body:&mut body});native.record_progress(body.progress())?;result}
fn validate_snapshot_schema_encoding(database:&SqliteDatabase,schema:&str,control:&mut SqliteSnapshotControl<'_>,native:&mut semio_framework_io_control::NativeSnapshotEncodeOwner<'_,'_>)->Result<(),ValueError>{let mut body=semio_framework_io_control::NativeSnapshotBodyWallet::new(native.remaining_grant());let result=semio_framework_io_sqlite_snapshot::artifact::receiving::validate_schema_sql(database,schema,&mut semio_framework_io_sqlite_snapshot::artifact::receiving::Port{control,native:semio_framework_io_sqlite_snapshot::artifact::receiving::Direction::Encode(native.native()),body:&mut body});native.record_progress(body.progress())?;result}
fn metadata_encoding(table:&SqliteTable,control:&mut SqliteSnapshotControl<'_>,native:&mut semio_framework_io_control::NativeSnapshotEncodeOwner<'_,'_>)->Result<SnapshotEncoding,ValueError>{let mut body=semio_framework_io_control::NativeSnapshotBodyWallet::new(native.remaining_grant());let result=validate_snapshot_metadata_receiving(table,&mut semio_framework_io_sqlite_snapshot::artifact::receiving::Port{control,native:semio_framework_io_sqlite_snapshot::artifact::receiving::Direction::Encode(native.native()),body:&mut body});native.record_progress(body.progress())?;result}

#[derive(semio_framework_value::RetireOwned)]
struct SnapshotMetadataFrame{database:Option<SqliteDatabase>,metadata:Option<SqliteTable>,dialect:Option<ArtifactDialect>,tables:Vec<SqliteTable>}
impl SnapshotMetadataFrame{fn new()->Self{Self{database:None,metadata:None,dialect:None,tables:Vec::new()}}}

#[derive(semio_framework_value::RetireOwned)]
struct SnapshotHopFrame{input:Option<IoPayload>,snapshot:SnapshotMetadataFrame,projected:Option<IoResult<SqliteDatabase>>,reconstructed:Option<IoResult<IoPayload>>,physical_export:Option<semio_framework_io_sqlite_snapshot::PhysicalExportFrame>,physical_import:Option<semio_framework_io_sqlite_snapshot::PhysicalImportFrame>,result:Option<IoResult<IoPayload>>}
impl SnapshotHopFrame{fn new(input:IoPayload)->Self{Self{input:Some(input),snapshot:SnapshotMetadataFrame::new(),projected:None,reconstructed:None,physical_export:None,physical_import:None,result:None}}}

fn attach_metadata_decoding(frame:&mut SnapshotMetadataFrame,dialect:&ArtifactDialect,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native:&mut semio_framework_io_control::NativeSnapshotDecodeOwner<'_,'_>)->Result<(),ValueError>{
    use semio_framework_io_sqlite_snapshot::artifact::receiving::{Port, Direction, Projection, Scalar, row, cell, text_cell, push_row};
    let invalid=||ValueError::literal(ValueRefusalKind::InvalidValue,"snapshot SQLite metadata dialect is invalid");
    let mut body=semio_framework_io_control::NativeSnapshotBodyWallet::new(native.remaining_grant());let preflight=(||{let mut port=Port{control,native:Direction::Decode(native.native()),body:&mut body};let database=frame.database.as_ref().unwrap();let limits=port.control.limits();let count=database.tables.len().checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"snapshot metadata table census overflow"))?;if count>limits.max_tables||limits.max_columns<6{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"snapshot metadata exceeds original table or column ceiling"))}
        let mut rows=1usize;let mut bytes=16usize.checked_add(dialect.artifact_kind.len()).and_then(|bytes|bytes.checked_add(dialect.standard.len())).and_then(|bytes|bytes.checked_add(dialect.subset.len())).and_then(|bytes|bytes.checked_add(encoding.as_str().len())).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot metadata value extent overflow"))?;
        let mut schema=SNAPSHOT_METADATA_SQL.trim().trim_end_matches(';').len()+"semio_snapshot".len();for table in &database.tables{port.work(1,0)?;if table.name.eq_ignore_ascii_case("semio_snapshot"){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"domain schema cannot claim reserved snapshot metadata"))}schema=schema.checked_add(table.name.len()).and_then(|bytes|bytes.checked_add(table.sql.len())).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot metadata schema extent overflow"))?;for row in &table.rows{port.work(1,0)?;rows=rows.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"snapshot metadata row census overflow"))?;for value in &row.values{port.work(1,0)?;bytes=bytes.checked_add(match value{SqliteValue::Null=>0,SqliteValue::Integer(_)|SqliteValue::Real(_)=>8,SqliteValue::Text(text)=>text.len(),SqliteValue::Blob(bytes)=>bytes.len()}).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot metadata value extent overflow"))?;}}}
        if rows>limits.max_rows{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"snapshot metadata exceeds original row ceiling"))}if bytes>limits.max_value_bytes||schema>limits.max_schema_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot metadata exceeds original value or schema ceiling"))}
        for(index,text)in[&dialect.artifact_kind,&dialect.standard,&dialect.subset].into_iter().enumerate(){if text.is_empty(){return Err(invalid())}for(position,character)in text.chars().enumerate(){if position%256==0{port.work(1,0)?}if character.is_control()||index==0&&character=='@'||index==2&&character=='/'{return Err(invalid())}}}Ok(())})();native.record_progress(body.progress())?;preflight?;
    frame.metadata=Some(native.receive::<Projection,SqliteTable>(|slot,native,body|{*slot=Some(Projection::new());let projected=slot.as_mut().unwrap();let mut port=Port{control,native:Direction::Decode(native),body};port.vector(1,&mut projected.database.tables,1)?;port.work(2,size_of::<SqliteTable>())?;projected.database.tables.push(SqliteTable{name:String::new(),sql:String::new(),rows:Vec::new()});let table=&mut projected.database.tables[0];port.text("semio_snapshot",&mut table.name,2)?;port.text(SNAPSHOT_METADATA_SQL.trim().trim_end_matches(';'),&mut table.sql,2)?;port.vector(1,&mut table.rows,2)?;
        row(projected,1,6,&mut port)?;for text in[&dialect.artifact_kind,&dialect.standard,&dialect.subset]{text_cell(projected,text,&mut port)?}cell(projected,Scalar::Integer(1),&mut port)?;text_cell(projected,encoding.as_str(),&mut port)?;push_row(projected,0,&mut port)?;port.work(1,size_of::<SqliteTable>())?;Ok(projected.database.tables.pop().unwrap())})?);
    let mut body=semio_framework_io_control::NativeSnapshotBodyWallet::new(native.remaining_grant());let result=(||{let mut port=Port{control,native:Direction::Decode(native.native()),body:&mut body};let database=frame.database.as_mut().unwrap();let count=database.tables.len().checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"snapshot metadata table census overflow"))?;semio_framework_io_sqlite_snapshot::artifact::receiving::relocate_vector(&mut database.tables,&mut frame.tables,count,1,&mut port)?;port.work(1,size_of::<SqliteTable>())?;database.tables.push(frame.metadata.take().unwrap());Ok(())})();native.record_progress(body.progress())?;result
}

fn take_metadata_encoding(frame:&mut SnapshotMetadataFrame,control:&mut SqliteSnapshotControl<'_>,native:&mut semio_framework_io_control::NativeSnapshotEncodeOwner<'_,'_>)->Result<SnapshotEncoding,ValueError>{
    let database=frame.database.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot metadata original database is absent"))?;
    let mut body=semio_framework_io_control::NativeSnapshotBodyWallet::new(native.remaining_grant());let found=(||{let mut port=semio_framework_io_sqlite_snapshot::artifact::receiving::Port{control,native:semio_framework_io_sqlite_snapshot::artifact::receiving::Direction::Encode(native.native()),body:&mut body};for(index,table)in database.tables.iter().enumerate(){port.work(1,0)?;if table.name=="semio_snapshot"{return Ok(index)}}Err(ValueError::literal(ValueRefusalKind::InvalidValue,"SQLite file has no semantic snapshot metadata"))})();native.record_progress(body.progress())?;let index=found?;
    let encoding=metadata_encoding(&database.tables[index],control,native)?;let shifted=database.tables.len()-index;let copy=shifted.checked_mul(size_of::<SqliteTable>()).and_then(|copy|copy.checked_add(3*size_of::<SqliteValue>()+size_of::<ArtifactDialect>())).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot metadata transfer extent overflow"))?;
    let moved=semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()};if !moved.fits(native.remaining_grant()){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot metadata transfer exceeds original physical grant"))}native.native().checkpoint()?;
    frame.metadata=Some(database.tables.remove(index));let row=&mut frame.metadata.as_mut().unwrap().rows[0];let take=|cell:&mut SqliteValue|match std::mem::replace(cell,SqliteValue::Null){SqliteValue::Text(text)=>text,_=>unreachable!()};frame.dialect=Some(ArtifactDialect{artifact_kind:take(&mut row.values[1]),standard:take(&mut row.values[2]),subset:take(&mut row.values[3])});native.record_progress(moved)?;Ok(encoding)
}

fn run_snapshot_hop(snapshots:&NativeSnapshotMap,hop:&IoEntryDescriptor,input:&mut Option<IoPayload>,native:&mut IoRunControl<'_,'_>,control:&mut SqliteSnapshotControl<'_>)->IoResult<IoPayload>{
    let limits=control.limits();
    if input.is_none(){return Err(IoError::from_value_error(ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot hop original input is absent")))}
    if is_sqlite_snapshot(&hop.into){
        let mut original=native.snapshot_decode().map_err(IoError::from_value_error)?;
        let before=original.native().owned_bytes();let maximum=before.checked_add(control.allocation_remaining_bytes()).ok_or_else(||refusal(ValueRefusalKind::OwnershipLimit,"snapshot hop original SQL ceiling overflow"))?;
        let settlement=TypedFileSettlement::new(control,before,SqliteSnapshotPhase::WritePages);let mut observer=|event:semio_framework_value_native_owned::native_decoding::NativeDecodeProgress|settlement.observe(event.owned_bytes,event.completed,event.total);
        let received=original.scoped_native(maximum,&mut observer,|native|native.receive_nested::<SnapshotHopFrame,IoResult<IoPayload>>(|slot,native|{
            let mut borrowed=settlement.control.borrow_mut();let control=&mut **borrowed;
            typed_file_work!(native,control,Decode,size_of::<SnapshotHopFrame>())?;
            let moved=semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Option<IoPayload>>(),..Default::default()};if !moved.fits(native.remaining_grant()){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot input transfer exceeds original physical grant"))}native.native().checkpoint()?;*slot=Some(SnapshotHopFrame::new(input.take().unwrap()));native.record_progress(moved)?;let frame=slot.as_mut().unwrap();
            typed_file_work!(native,control,Decode,size_of::<IoResult<IoPayload>>())?;
            frame.result=Some((||{
                control.checkpoint(SqliteSnapshotPhase::DecodeNative,0,1).map_err(IoError::from_value_error)?;
                let payload=frame.input.as_ref().unwrap();let size=match payload{IoPayload::Binary(bytes)=>bytes.len(),IoPayload::Text(text)=>text.len()};
                if size>limits.max_file_bytes{return Err(IoError::from_value_error(ValueError::literal(ValueRefusalKind::OwnershipLimit,"native snapshot exceeds SQLite snapshot limits")))}
                let codec=snapshots.get(&hop.from).ok_or_else(||refusal(ValueRefusalKind::UnsupportedOwner,"unregistered native snapshot dialect"))?;
                let provider=codec.snapshot_sqlite.as_ref().ok_or_else(||refusal(ValueRefusalKind::UnsupportedOwner,"artifact has no handwritten semantic SQLite provider"))?;
                typed_file_work!(native,control,Decode,size_of::<IoResult<SqliteDatabase>>()).map_err(IoError::from_value_error)?;
                frame.projected=Some((provider.export)(&codec.schema,&hop.from,payload,control,native));
                match frame.projected.as_ref().unwrap(){Err(_)=>return Err(frame.projected.take().unwrap().unwrap_err()),Ok(_)=>{}}
                typed_file_work!(native,control,Decode,size_of::<Option<SqliteDatabase>>()).map_err(IoError::from_value_error)?;
                let projected=frame.projected.as_mut().unwrap().as_mut().unwrap();frame.snapshot.database=Some(std::mem::replace(&mut projected.value,SqliteDatabase{tables:Vec::new()}));
                let database=frame.snapshot.database.as_mut().unwrap();validate_snapshot_schema_decoding(database,&provider.schema,control,native).map_err(IoError::from_value_error)?;
                let mut body=semio_framework_io_control::NativeSnapshotBodyWallet::new(native.remaining_grant());let extent=semio_framework_io_sqlite_snapshot::artifact::receiving::check_database(database,&mut semio_framework_io_sqlite_snapshot::artifact::receiving::Port{control,native:semio_framework_io_sqlite_snapshot::artifact::receiving::Direction::Decode(native.native()),body:&mut body});native.record_progress(body.progress()).map_err(IoError::from_value_error)?;extent.map_err(IoError::from_value_error)?;
                let encoding=match frame.input.as_ref().unwrap(){IoPayload::Binary(_)=>SnapshotEncoding::Binary,IoPayload::Text(_)=>SnapshotEncoding::Text};
                attach_metadata_decoding(&mut frame.snapshot,&hop.from,encoding,control,native).map_err(IoError::from_value_error)?;
                typed_file_work!(native,control,Decode,size_of::<semio_framework_io_sqlite_snapshot::PhysicalExportFrame>()).map_err(IoError::from_value_error)?;frame.physical_export=Some(semio_framework_io_sqlite_snapshot::PhysicalExportFrame::new());
                let mut body=semio_framework_io_control::NativeSnapshotBodyWallet::new(native.remaining_grant());let physical=semio_framework_io_sqlite_snapshot::export_sqlite_database_receiving_into(frame.snapshot.database.as_ref().unwrap(),frame.physical_export.as_mut().unwrap(),&mut semio_framework_io_sqlite_snapshot::artifact::receiving::Port{control,native:semio_framework_io_sqlite_snapshot::artifact::receiving::Direction::Decode(native.native()),body:&mut body});native.record_progress(body.progress()).map_err(IoError::from_value_error)?;physical.map_err(IoError::from_value_error)?;
                typed_file_work!(native,control,Decode,size_of::<Vec<u8>>()+size_of::<Vec<Diagnostic>>()).map_err(IoError::from_value_error)?;
                Ok(IoOutcome{value:IoPayload::Binary(std::mem::take(&mut frame.physical_export.as_mut().unwrap().output)),diagnostics:frame.projected.take().unwrap().unwrap().diagnostics})
            })());
            let owned=native.native().owned_bytes();drop(borrowed);settlement.finish_body(owned)?;Ok(frame.result.take().unwrap())
        }));
        return match received{Ok(output)=>output,Err(error)=>Err(IoError::from_value_error(settlement.refused(original.native().owned_bytes(),error)))};
    }
    let mut original=native.snapshot_encode().map_err(IoError::from_value_error)?;
    let before=original.native().owned_bytes();let maximum=before.checked_add(control.allocation_remaining_bytes()).ok_or_else(||refusal(ValueRefusalKind::OwnershipLimit,"snapshot hop original SQL ceiling overflow"))?;
    let settlement=TypedFileSettlement::new(control,before,SqliteSnapshotPhase::ReconstructSnapshot);let mut observer=|event:semio_framework_value::native_encoding::NativeEncodeProgress|settlement.observe(event.owned_bytes,event.completed,event.total);
    let received=original.scoped_native(maximum,&mut observer,|native|native.receive_nested::<SnapshotHopFrame,IoResult<IoPayload>>(|slot,native|{
        let mut borrowed=settlement.control.borrow_mut();let control=&mut **borrowed;
        typed_file_work!(native,control,Encode,size_of::<SnapshotHopFrame>())?;
        let moved=semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Option<IoPayload>>(),..Default::default()};if !moved.fits(native.remaining_grant()){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot input transfer exceeds original physical grant"))}native.native().checkpoint()?;*slot=Some(SnapshotHopFrame::new(input.take().unwrap()));native.record_progress(moved)?;let frame=slot.as_mut().unwrap();
        typed_file_work!(native,control,Encode,size_of::<IoResult<IoPayload>>())?;
        frame.result=Some((||{
            control.checkpoint(SqliteSnapshotPhase::DecodeNative,0,1).map_err(IoError::from_value_error)?;
            let IoPayload::Binary(bytes)=frame.input.as_ref().unwrap()else{return Err(refusal(ValueRefusalKind::InvalidValue,"SQLite snapshot import requires a binary payload"))};
            typed_file_work!(native,control,Encode,size_of::<semio_framework_io_sqlite_snapshot::PhysicalImportFrame>()).map_err(IoError::from_value_error)?;frame.physical_import=Some(semio_framework_io_sqlite_snapshot::PhysicalImportFrame::new());
            let mut body=semio_framework_io_control::NativeSnapshotBodyWallet::new(native.remaining_grant());let physical=semio_framework_io_sqlite_snapshot::import_sqlite_database_receiving_into(bytes,frame.physical_import.as_mut().unwrap(),&mut semio_framework_io_sqlite_snapshot::artifact::receiving::Port{control,native:semio_framework_io_sqlite_snapshot::artifact::receiving::Direction::Encode(native.native()),body:&mut body});native.record_progress(body.progress()).map_err(IoError::from_value_error)?;physical.map_err(IoError::from_value_error)?;
            typed_file_work!(native,control,Encode,size_of::<Option<SqliteDatabase>>()).map_err(IoError::from_value_error)?;
            frame.snapshot.database=Some(std::mem::replace(&mut frame.physical_import.as_mut().unwrap().database,SqliteDatabase{tables:Vec::new()}));
            let encoding=take_metadata_encoding(&mut frame.snapshot,control,native).map_err(IoError::from_value_error)?;
            if frame.snapshot.dialect.as_ref().unwrap()!=&hop.into{return Err(refusal(ValueRefusalKind::InvalidValue,"semantic SQLite snapshot dialect does not match requested dialect"))}
            let codec=snapshots.get(&hop.into).ok_or_else(||refusal(ValueRefusalKind::UnsupportedOwner,"unregistered native snapshot dialect"))?;
            let provider=codec.snapshot_sqlite.as_ref().ok_or_else(||refusal(ValueRefusalKind::UnsupportedOwner,"artifact has no handwritten semantic SQLite provider"))?;
            validate_snapshot_schema_encoding(frame.snapshot.database.as_ref().unwrap(),&provider.schema,control,native).map_err(IoError::from_value_error)?;
            typed_file_work!(native,control,Encode,size_of::<IoResult<IoPayload>>()).map_err(IoError::from_value_error)?;
            frame.reconstructed=Some((provider.import)(&codec.schema,&hop.into,&mut frame.snapshot.database,encoding,control,native));
            match frame.reconstructed.as_ref().unwrap(){Err(_)=>return Err(frame.reconstructed.take().unwrap().unwrap_err()),Ok(output)=>{let size=match &output.value{IoPayload::Binary(bytes)=>bytes.len(),IoPayload::Text(text)=>text.len()};if size>limits.max_file_bytes{return Err(refusal(ValueRefusalKind::OwnershipLimit,"reconstructed native snapshot exceeds SQLite snapshot limits"))}}}
            frame.reconstructed.take().unwrap()
        })());
        let owned=native.native().owned_bytes();drop(borrowed);settlement.finish_body(owned)?;Ok(frame.result.take().unwrap())
    }));
    match received{Ok(output)=>output,Err(error)=>Err(IoError::from_value_error(settlement.refused(original.native().owned_bytes(),error)))}
}

fn resolve_run(registry: &EntryMap, route: &IoRoute, payload: IoPayload, control: &mut IoRunControl<'_, '_>) -> IoResult<IoPayload> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

    let mut current = payload;
    let mut diagnostics = Vec::new();
    for hop in &route.hops {
        let key = (hop.from.clone(), hop.into.clone());
        let entry = registry.get(&key).ok_or_else(|| IoError::from_value_error(ValueError::new(ValueRefusalKind::UnsupportedOwner, format!("io_run: no entry registered for hop {} -> {}", hop.from.to_coordinate(), hop.into.to_coordinate()))))?;
        control.checkpoint().map_err(IoError::from_value_error)?;
        let outcome = (entry.run)(&current, control).map_err(|error| IoError { cause: ValueError::new(error.cause.kind, format!("io_run: hop {} -> {} failed: {}", hop.from.to_coordinate(), hop.into.to_coordinate(), error.cause.message)), diagnostics: error.diagnostics })?;
        control.checkpoint().map_err(IoError::from_value_error)?;
        current = outcome.value;
        diagnostics.extend(outcome.diagnostics);
    }
    Ok(IoOutcome { value: current, diagnostics })
}

/// 🫴️ Executes the original route under explicit native and SQLite caller authorities.
pub fn io_run_with_snapshot_control(route: &IoRoute, input: &mut Option<IoPayload>, control: &mut IoRunControl<'_, '_>, snapshot: &mut SqliteSnapshotControl<'_>) -> std::future::Ready<IoResult<IoPayload>> {
    std::future::ready((|| {
        control.checkpoint().map_err(IoError::from_value_error)?;
    if route.hops.is_empty() || route.hops.windows(2).any(|pair| pair[0].into != pair[1].from) {
        return Err(refusal(ValueRefusalKind::InvalidValue, "io_run: route must contain connected hops"));
    }
    if route.hops.iter().any(|hop| is_sqlite_snapshot(&hop.from) || is_sqlite_snapshot(&hop.into)) {
        if route.hops.len() != 1 || route.fidelity != IoFidelity::Exact {
            return Err(refusal(ValueRefusalKind::InvalidValue, "io_run: SQLite snapshots are exact endpoint transfers and cannot be intermediate hops"));
        }
        let hop = &route.hops[0];
        let native = if is_sqlite_snapshot(&hop.from) { &hop.into } else { &hop.from };
        let codec = native_snapshot_registry().read().map_err(|_| refusal(ValueRefusalKind::InvariantViolated, "native snapshot registry unavailable"))?.get(native).cloned();
        let snapshots = codec.map(|codec| BTreeMap::from([(native.clone(), codec)])).unwrap_or_default();
        if supplemental_snapshot_entry(&snapshots, &hop.from, &hop.into).as_ref() != Some(hop) {
            return Err(refusal(ValueRefusalKind::InvalidValue, "io_run: inconsistent SQLite snapshot descriptor"));
        }
        return run_snapshot_hop(&snapshots, hop, input, control, snapshot);
    }
    let registry = io_mechanism_registry().read().map_err(|_| refusal(ValueRefusalKind::InvariantViolated, "io mechanism registry unavailable"))?.clone();
    resolve_run(&registry, route, input.take().ok_or_else(||refusal(ValueRefusalKind::InvariantViolated,"IO route original input is absent"))?, control)
    })())
}
//#endregion 🔖️Route

//#region 🔖️Identify
async fn resolve_identify(registry: &EntryMap, payload: &IoPayload) -> Vec<(ArtifactDialect, Confidence)> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

    let carrier = ArtifactDialect::from(match payload {
        IoPayload::Binary(_) => CARRIER_BINARY,
        IoPayload::Text(_) => CARRIER_TEXT,
    });
    let mut found: Vec<(ArtifactDialect, Confidence)> = registry
        .values()
        .filter(|entry| ArtifactDialect::from(entry.from) == carrier)
        .filter_map(|entry| entry.sniff.map(|sniff| (ArtifactDialect::from(entry.into), sniff(payload))))
        .filter(|(_, confidence)| *confidence != Confidence::None)
        .collect();
    found.sort_by(|a, b| b.1.rank().cmp(&a.1.rank()).then_with(|| a.0.to_coordinate().cmp(&b.0.to_coordinate())));
    found
}

/// 🔍️ Runs `sniff` of every entry whose `from` is the carrier dialect matching `payload`'s own
/// variant (`CARRIER_BINARY` for `Binary`, `CARRIER_TEXT` for `Text` — never both), drops
/// `Confidence::None`, sorts by confidence (descending) then coordinate (ascending).
pub async fn io_identify(payload: &IoPayload) -> Vec<(ArtifactDialect, Confidence)> {
    io_identify_with_snapshot_control(payload, SqliteDatabaseLimits::default(), &mut |_| true).await.unwrap_or_default()
}

/// 🛑️ Validates semantic SQLite identity under caller-controlled resource and cancellation bounds.
pub async fn io_identify_with_snapshot_control(payload: &IoPayload, limits: SqliteDatabaseLimits, progress: &mut dyn FnMut(SqliteSnapshotProgress) -> bool) -> Result<Vec<(ArtifactDialect, Confidence)>, IoError> {
    if let IoPayload::Binary(bytes) = payload {
        if bytes.starts_with(b"SQLite format 3\0") {
            let mut database = import_sqlite_database(bytes, limits, progress).map_err(IoError::from_value_error)?;
            if take_sqlite_snapshot_metadata(&mut database).is_ok() && !database.tables.is_empty() {
                return Ok(vec![(ArtifactDialect::from(SQLITE_SNAPSHOT), Confidence::High)]);
            }
        }
    }
    let registry = io_mechanism_registry().read().map_err(|_| refusal(ValueRefusalKind::InvariantViolated, "io mechanism registry unavailable"))?.clone();
    Ok(resolve_identify(&registry, payload).await)
}

/// 📇️ Every registered entry, erased to owned descriptors — the WIT `list-io-entries` guest
/// export body and the TS `IoEntryDescriptor[]` mirror both use this shape. Builds the vec with
/// a plain `for` loop rather than `.map(..).collect()` because `descriptor_of` is `async` and a
/// `Iterator::map` closure (std's fixed `FnMut` signature) cannot itself await.
pub fn io_entries() -> Vec<IoEntryDescriptor> {
    match io_mechanism_registry().read() {
        Ok(registry) => {
            let mut descriptors = Vec::with_capacity(registry.len());
            for entry in registry.values() {
                descriptors.push(descriptor_of(entry));
            }
            if let Ok(snapshots) = native_snapshot_registry().read() {
                let sqlite = ArtifactDialect::from(SQLITE_SNAPSHOT);
                for dialect in snapshots.keys() {
                    descriptors.push(snapshot_descriptor(dialect.clone(), sqlite.clone()));
                    descriptors.push(snapshot_descriptor(sqlite.clone(), dialect.clone()));
                }
                descriptors.sort_by(|a, b| (&a.from, &a.into).cmp(&(&b.from, &b.into)));
                descriptors.dedup();
            }
            descriptors
        }
        Err(_) => Vec::new(),
    }
}

/// 🛣️ Every registered entry from its native side ([`IoNativeRoute`], registry order) — what `describe` lists as a package's
/// import/export rows. The synthesized native ↔ sqlite-snapshot hops of [`io_entries`] are generic, not registered entries.
pub fn io_native_routes() -> Vec<IoNativeRoute> {
    let Ok(registry) = io_mechanism_registry().read() else { return Vec::new() };
    registry
        .values()
        .map(|entry| {
            let (from, into) = (ArtifactDialect::from(entry.from), ArtifactDialect::from(entry.into));
            match entry.direction {
                IoEntryDirection::Export => IoNativeRoute { native: from, foreign: into, direction: entry.direction },
                IoEntryDirection::Import => IoNativeRoute { native: into, foreign: from, direction: entry.direction },
            }
        })
        .collect()
}
//#endregion 🔖️Identify

//#region 🔖️Constructors
/// 🎹️ Builds an `IoEntry` from a `Serializer<S>` impl for a binary-pack-native `S`. Bounds on
/// `pack::snapshot::ArtifactPack` directly rather than inventing a parallel "native decode" trait —
/// this file already depends on `store::` throughout (e.g. `ArtifactAssemblyRegistryPlan`
/// a few regions up), so this introduces no new dependency. The plugin's own decode function is
/// `S::decode_pack`, resolved by monomorphization of the inner `run` item — never a runtime
/// `fn` pointer parameter threaded through a closure, which `IoEntry.run`'s bare-`fn`-pointer
/// field cannot capture anyway. This is the smallest constructor shape that keeps a plugin from
/// ever hand-writing an `IoEntry` literal while never leaking pack/DSL encoding into its code. A composed head carrier
/// ([`native_carrier`]) hands its owned children to the serializer.
pub fn serializer_entry<S: pack::snapshot::ArtifactPack + pack::snapshot::ArtifactNativeSnapshot, T: Serializer<S>>(own: Dialect) -> IoEntry {
    fn run<S: pack::snapshot::ArtifactPack + pack::snapshot::ArtifactNativeSnapshot, T: Serializer<S>>(payload: &IoPayload, control: &mut IoRunControl<'_, '_>) -> IoResult<IoPayload> {
        let IoPayload::Binary(bytes) = payload else { return Err(refusal(ValueRefusalKind::InvalidValue, "serializer_entry: expected binary native payload")); };
        control.checkpoint().map_err(IoError::from_value_error)?;
        let (parent, children) = native_carrier(bytes,&mut control.snapshot_decode().map_err(IoError::from_value_error)?)?;
        let value = S::decode_native_snapshot(pack::snapshot::NativeSnapshotInput::Binary(&parent), control.decode().map_err(IoError::from_value_error)?).map_err(IoError::from_value_error)?;
        ::semio_framework_async::poll::resolve_ready(T::serialize(&value, &children, control))
    }
    IoEntry { owned_serializer: T::OWNED_FACTORY, from: own, into: T::INTO, fidelity: T::FIDELITY, direction: IoEntryDirection::Export, sniff: None, run: run::<S, T> }
}

/// 📝️ Runs the exact text-native owner under the original native decoder.
pub fn serializer_entry_text<S: pack::snapshot::ArtifactDsl + pack::snapshot::ArtifactNativeSnapshot, T: Serializer<S>>(own: Dialect) -> IoEntry {
    fn run<S: pack::snapshot::ArtifactDsl + pack::snapshot::ArtifactNativeSnapshot, T: Serializer<S>>(payload: &IoPayload, control: &mut IoRunControl<'_, '_>) -> IoResult<IoPayload> {
        control.checkpoint().map_err(IoError::from_value_error)?;
        let (value, children) = match payload {
            IoPayload::Text(text) => (S::decode_native_snapshot(pack::snapshot::NativeSnapshotInput::Text(text), control.decode().map_err(IoError::from_value_error)?).map_err(IoError::from_value_error)?, ArchiveChildren::empty()),
            IoPayload::Binary(bytes) if bytes.first() == Some(&semio_framework_os_channel::DOCUMENT_ARCHIVE_VERSION) => {
                let (parent, children) = native_carrier(bytes,&mut control.snapshot_decode().map_err(IoError::from_value_error)?)?;
                let text = std::str::from_utf8(&parent).map_err(|_| refusal(ValueRefusalKind::InvalidValue, "composed parent is not UTF8 DSL text"))?;
                (S::decode_native_snapshot(pack::snapshot::NativeSnapshotInput::Text(text), control.decode().map_err(IoError::from_value_error)?).map_err(IoError::from_value_error)?, children)
            }
            IoPayload::Binary(_) => return Err(refusal(ValueRefusalKind::InvalidValue, "serializer_entry_text: expected text native payload or composed carrier")),
        };
        ::semio_framework_async::poll::resolve_ready(T::serialize(&value, &children, control))
    }
    IoEntry { owned_serializer: T::OWNED_FACTORY, from: own, into: T::INTO, fidelity: T::FIDELITY, direction: IoEntryDirection::Export, sniff: None, run: run::<S, T> }
}

// 🚫️async: E4 fn-pointer slot — `IoEntry.sniff` is a bare `fn` pointer; `Deserializer::sniff`
// stays `async fn` (it has a default body and may be overridden with real decode logic) and
// this thunk resolves it synchronously via `resolve_ready`.
fn deserializer_sniff<S, T: Deserializer<S>>(payload: &IoPayload) -> Confidence {
    ::semio_framework_async::poll::resolve_ready(T::sniff(payload))
}

/// 🎹️ Builds an `IoEntry` from a `Deserializer<S>` impl for a binary-pack-native `S`. Runs
/// `T::CONFORMANCE` (D5's `SubsetValidator` replacement) after a successful deserialize,
/// folding its diagnostics into the returned `IoOutcome` — `conformance_runs_after_deserialize`
/// below proves the diagnostics reach the caller.
pub fn deserializer_entry<S: pack::snapshot::ArtifactPack + pack::snapshot::ArtifactNativeSnapshot, T: Deserializer<S>>(own: Dialect) -> IoEntry {
    fn run<S: pack::snapshot::ArtifactPack + pack::snapshot::ArtifactNativeSnapshot, T: Deserializer<S>>(payload: &IoPayload, control: &mut IoRunControl<'_, '_>) -> IoResult<IoPayload> {
        let outcome = ::semio_framework_async::poll::resolve_ready(T::deserialize(payload, control))?;
        let mut diagnostics = outcome.diagnostics;
        if let Some(conformance) = T::CONFORMANCE {
            diagnostics.extend(conformance(&outcome.value));
        }
        Ok(IoOutcome { value: outcome.value.encode_native_snapshot(pack::snapshot::NativeSnapshotEncoding::Binary, control.encode().map_err(IoError::from_value_error)?).map_err(IoError::from_value_error)?, diagnostics })
    }
    IoEntry { owned_serializer: None, from: T::FROM, into: own, fidelity: T::FIDELITY, direction: IoEntryDirection::Import, sniff: Some(deserializer_sniff::<S, T>), run: run::<S, T> }
}

/// 🎹️ `deserializer_entry`'s twin for a DSL-text-native `S`.
pub fn deserializer_entry_text<S: pack::snapshot::ArtifactDsl + pack::snapshot::ArtifactNativeSnapshot, T: Deserializer<S>>(own: Dialect) -> IoEntry {
    fn run<S: pack::snapshot::ArtifactDsl + pack::snapshot::ArtifactNativeSnapshot, T: Deserializer<S>>(payload: &IoPayload, control: &mut IoRunControl<'_, '_>) -> IoResult<IoPayload> {
        let outcome = ::semio_framework_async::poll::resolve_ready(T::deserialize(payload, control))?;
        let mut diagnostics = outcome.diagnostics;
        if let Some(conformance) = T::CONFORMANCE {
            diagnostics.extend(conformance(&outcome.value));
        }
        Ok(IoOutcome { value: outcome.value.encode_native_snapshot(pack::snapshot::NativeSnapshotEncoding::Text, control.encode().map_err(IoError::from_value_error)?).map_err(IoError::from_value_error)?, diagnostics })
    }
    IoEntry { owned_serializer: None, from: T::FROM, into: own, fidelity: T::FIDELITY, direction: IoEntryDirection::Import, sniff: Some(deserializer_sniff::<S, T>), run: run::<S, T> }
}
//#endregion 🔖️Constructors

//#region 🔖️Laws
#[cfg(test)]
include!("../🧪️tests/🔬️io-mechanism-laws/🦀️.rs");
//#endregion 🔖️Laws
