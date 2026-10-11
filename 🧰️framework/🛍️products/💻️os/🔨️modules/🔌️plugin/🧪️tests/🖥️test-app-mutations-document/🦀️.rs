//#region 🧬️TestDocumentMutationRoot
//! 🧪️ Shared document state and direct mutation fixtures for the Plugin contract.

use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};
use store::ArtifactPack;

//#region 🧫️Snapshot
pub(crate) const MAXIMUM_CHILD_PROBE_BYTES: usize = 4 * 1024 * 1024;
pub(crate) static MAXIMUM_CHILD_CLONES: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
pub(crate) static MAXIMUM_CHILD_ENCODINGS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

#[derive(semio_framework_dsl_record_derive::DslRecord, Debug, Default, PartialEq, Serialize, ToValue, Deserialize, FromValue, semio_framework_os_kernel::DslArtifact, semio_framework_value::RetainedClone)]
#[artifact(extension = "testkit-macro")]
pub(crate) struct TestSnapshot {
    pub(crate) count: i32,
    pub(crate) label: String,
    /// 🧒️ The members declared in this document's one owned-child slot, `"slot"` — the field a
    /// schema-derived snapshot spells `#[child(kind = "s.test.child")]`. `store::ArtifactChild`
    /// carries `ToValue`/`FromValue`/`DslField` but no serde impl, so the derived serde half skips
    /// it and the hand-written pack/DSL codecs below carry it as its member's canonical
    /// `ArtifactRef` uri.
    #[serde(skip)]
    pub(crate) slot: Vec<store::ArtifactChild<TestSnapshot>>,
}

impl store::ArtifactSqliteSnapshot for TestSnapshot {
    const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");
    fn to_sqlite_database(&self, control: &mut store::sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<store::sqlite_snapshot::SqliteDatabase, semio_framework_value::ValueError> {
        use store::sqlite_snapshot::{SqliteDatabase, SqliteRow, SqliteSnapshotPhase, SqliteValue};
        control.check_rows(self.slot.len().checked_add(1).ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit, "child row count overflow"))?)?;
        control.check_value_bytes(self.label.len())?;
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, self.slot.len() + 1)?;
        let mut database = SqliteDatabase::from_schema(Self::SQLITE_SCHEMA)?;
        database.table_mut("document_state")?.rows.push(SqliteRow { rowid: 1, values: vec![SqliteValue::Integer(1), SqliteValue::Integer(i64::from(self.count)), SqliteValue::Text(self.label.clone())] });
        for (position, child) in self.slot.iter().enumerate() {
            if position % 256 == 0 {
                control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, position + 1, self.slot.len() + 1)?;
            }
            let id = i64::try_from(position + 1).map_err(|error| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string()))?;
            database.table_mut("slot_children")?.rows.push(SqliteRow {
                rowid: id,
                values: vec![
                    SqliteValue::Integer(id),
                    SqliteValue::Integer(1),
                    SqliteValue::Integer(position as i64),
                    SqliteValue::Text(child.target.artifact_id.clone()),
                    SqliteValue::Text(child.target.dialect.artifact_kind.clone()),
                    SqliteValue::Text(child.target.dialect.standard.clone()),
                    SqliteValue::Text(child.target.dialect.subset.clone()),
                ],
            });
        }
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, self.slot.len() + 1, self.slot.len() + 1)?;
        Ok(database)
    }
    fn from_sqlite_database(database: &store::sqlite_snapshot::SqliteDatabase, control: &mut store::sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<Self, semio_framework_value::ValueError> {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

        use store::sqlite_snapshot::SqliteSnapshotPhase;
        let rows = &database.table("document_state")?.rows;
        let children = &database.table("slot_children")?.rows;
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 0, children.len() + 1)?;
        if rows.len() != 1 || rows[0].rowid != 1 || rows[0].integer(0)? != 1 {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "document snapshot requires one state row"));
        }
        let mut snapshot =
            Self { count: i32::try_from(rows[0].integer(1)?).map_err(|error| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string()))?, label: rows[0].text(2)?.to_string(), slot: Vec::new() };
        let mut ordered = children.iter().collect::<Vec<_>>();
        ordered.sort_by_key(|row| row.integer(2).unwrap_or(-1));
        for (position, row) in ordered.into_iter().enumerate() {
            if position % 256 == 0 {
                control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, position + 1, children.len() + 1)?;
            }
            if row.rowid != row.integer(0)? || row.integer(1)? != 1 || row.integer(2)? != position as i64 {
                return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "invalid ordered document child relationship"));
            }
            let target = semio_framework_artifact_reference::ArtifactRef { artifact_id: row.text(3)?.to_string(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: row.text(4)?.to_string(), standard: row.text(5)?.to_string(), subset: row.text(6)?.to_string() } };
            let target = semio_framework_artifact_reference::ArtifactRef::parse_uri(&target.to_uri()).map_err(|error| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error))?;
            snapshot.slot.push(store::ArtifactChild::new(target.artifact_id.clone(), target));
        }
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, children.len() + 1, children.len() + 1)?;
        Ok(snapshot)
    }
}

impl TestSnapshot {
    /// 🧩️ The JSON carriage both hand-written codecs share. An undeclared slot writes no key at
    /// all, so a document with no child encodes exactly the bytes it always did.
    fn to_json(&self) -> Result<serde_json::Value, String> {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

        let mut value = serde_json::to_value(self).map_err(|error| error.to_string())?;
        if !self.slot.is_empty() {
            let object = value.as_object_mut().ok_or_else(|| "test snapshot encodes as a json object".to_string())?;
            object.insert("slot".into(), serde_json::Value::Array(self.slot.iter().map(|child| serde_json::Value::String(child.target.to_uri())).collect()));
        }
        Ok(value)
    }

    /// 🧩️ Inverse of {@link TestSnapshot::to_json}: every declared row is one `ArtifactRef` uri,
    /// and a child's id IS its artifact id (what `ChildRestoreProjection` admits).
    fn from_json(mut value: serde_json::Value) -> Result<Self, String> {
        let declared = value.as_object_mut().and_then(|object| object.remove("slot"));
        let mut snapshot: Self = serde_json::from_value(value).map_err(|error| error.to_string())?;
        let Some(declared) = declared else { return Ok(snapshot) };
        for row in declared.as_array().ok_or_else(|| "declared child slot is an array of artifact ref uris".to_string())? {
            snapshot.slot.push(test_child_handle(row.as_str().ok_or_else(|| "declared child is an artifact ref uri".to_string())?)?);
        }
        Ok(snapshot)
    }
}

/// 🧒️ One declared owned-child handle from its canonical `ArtifactRef` uri.
pub(crate) fn test_child_handle(uri: &str) -> Result<store::ArtifactChild<TestSnapshot>, String> {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

    let target = semio_framework_artifact_reference::ArtifactRef::parse_uri(uri)?;
    Ok(store::ArtifactChild::new(target.artifact_id.clone(), target))
}

/// ♻️ The fixture child is an openable member, so its snapshot needs the same bounded owned-value
/// retirement a real member's does — `store::PackMemberSnapshotOpen` retires the decoded snapshot
/// through it when an open is cancelled or rejected mid-flight.
semio_framework_value::artifact_retire_struct!(TestSnapshot { count, label, slot });

impl semio_framework_schema_composition::ArtifactCompositionFields for TestSnapshot {
    fn visit_child_refs<'a, V: semio_framework_schema_composition::ChildRefVisitor<'a>>(&'a self, visitor: &mut V) -> Result<(), V::Error> {
        semio_framework_schema_composition::ChildFieldRefs::visit_child_field(&self.slot, "slot", visitor)
    }

    fn child_slots() -> &'static [semio_framework_schema_composition::ChildSlotSpec] {
        &[semio_framework_schema_composition::ChildSlotSpec { name: "slot", kind: "s.test.child", many: true }]
    }
}

impl Clone for TestSnapshot {
    fn clone(&self) -> Self {
        if self.label.len() >= MAXIMUM_CHILD_PROBE_BYTES {
            MAXIMUM_CHILD_CLONES.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
        Self { count: self.count, label: self.label.clone(), slot: self.slot.clone() }
    }
}

impl store::ArtifactDsl for TestSnapshot {
    const EXTENSION: &'static str = "testkit-macro";
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        if text.trim().is_empty() {
            return Ok(Self::default());
        }
        let value: serde_json::Value =
            serde_json::from_str(text).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        Self::from_json(value).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        self.to_json().and_then(|value| serde_json::to_string(&value).map_err(|error| error.to_string())).unwrap_or_default()
    }
}

impl ArtifactPack for TestSnapshot {
    /// 🪶️ Publishes this owner's actual relational snapshot capability.
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> {
        Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())
    }

    fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        if self.label.len() >= MAXIMUM_CHILD_PROBE_BYTES {
            MAXIMUM_CHILD_ENCODINGS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
        let value = self.to_json().map_err(|error| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error)))?;
        serde_json::to_vec(&value).map_err(|error| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string())))
    }
    fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
        let value: serde_json::Value = serde_json::from_slice(bytes).map_err(|error| match (u32::try_from(error.line()), u32::try_from(error.column())) { (Ok(line), Ok(column)) => store::PackError::from(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string(), semio_framework_diagnostic::TextSpan::at(line, column))), _ => store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::OwnershipLimit, error.to_string())) })?;
        Self::from_json(value).map_err(|error| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error)))
    }
}
//#endregion 🧫️Snapshot

//#region 🔺️Diff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
pub(crate) struct TestDiff {
    pub(crate) count: Option<i32>,
    pub(crate) label: Option<String>,
    /// 🧒️ The whole declared membership of the `"slot"` child slot, as canonical `ArtifactRef`
    /// uris — a child declaration is replaced wholesale, never merged row by row.
    pub(crate) slot: Option<Vec<String>>,
}

fn test_slot_uris(snapshot: &TestSnapshot) -> Vec<String> {
    use semio_framework_artifact_reference::io::text::artifact_reference::ArtifactReferenceText as _;
    snapshot.slot.iter().map(|child| child.target.to_uri()).collect()
}

impl protocol::DiffAlgebra<TestSnapshot> for TestDiff {
    fn inverse(&self, base: &TestSnapshot) -> Self {
        Self { count: self.count.map(|_| base.count), label: self.label.as_ref().map(|_| base.label.clone()), slot: self.slot.as_ref().map(|_| test_slot_uris(base)) }
    }
    fn is_empty(&self) -> bool {
        *self == Self::default()
    }
}

impl protocol::MutationDiff<TestSnapshot> for TestDiff {
    fn apply(&self, snapshot: &TestSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<TestSnapshot> {
        let slot = match &self.slot {
            None => snapshot.slot.clone(),
            Some(rows) => rows.iter().map(|uri| test_child_handle(uri)).collect::<Result<Vec<_>, String>>().map_err(|error| protocol::MutationApplyError::new("mutation.apply.declared-child-uri", error))?,
        };
        Ok(TestSnapshot { count: self.count.unwrap_or(snapshot.count), label: self.label.clone().unwrap_or_else(|| snapshot.label.clone()), slot })
    }

    fn absorb(&mut self, other: Self) {
        if other.count.is_some() {
            self.count = other.count;
        }
        if other.label.is_some() {
            self.label = other.label;
        }
        if other.slot.is_some() {
            self.slot = other.slot;
        }
    }
}
//#endregion 🔺️Diff

//#region 🧬️Mutations
#[path = "../../🧪️testing/🖥️test-app-mutations/🧬️document/🧬️mutations/🦀️.rs"]
pub mod mutations;
pub(crate) use mutations::{SetCount, SetLabel, SetSlotChildren, TestMutation};
//#endregion 🧬️Mutations
//#endregion 🧬️TestDocumentMutationRoot
