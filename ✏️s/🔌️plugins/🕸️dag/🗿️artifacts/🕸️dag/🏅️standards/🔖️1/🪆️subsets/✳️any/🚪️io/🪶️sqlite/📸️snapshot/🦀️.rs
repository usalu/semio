//! 🕸️ Explicit DAG relational parent and literal Graph child relationship.
use crate::standards::v1::subsets::any::schema::snapshot::DagSnapshot;
use crate::DagContentChild;
use semio_framework_diagnostic::{TextError, TextSpan};
use semio_framework_value::{ValueError, ValueRefusalKind};
use store::sqlite_snapshot::{
    artifact::{Cell, Projection, Reconstruction},
    SnapshotEncoding, SqliteDatabase, SqliteRow, SqliteSnapshotControl, SqliteSnapshotPhase,
};
const SQL: &str = include_str!("🗄️.sql");
fn admit(c: &mut SqliteSnapshotControl<'_>, phase: SqliteSnapshotPhase) -> Result<(), ValueError> {
    c.checkpoint(phase, 0, 2)?;
    c.check_rows(2)?;
    let limits = c.limits();
    if limits.max_tables < 2 || limits.max_columns < 7 || limits.max_schema_bytes < SQL.len() + "dag_document".len() + "dag_content_child".len() {
        return Err(ValueError::new(ValueRefusalKind::OwnershipLimit, "DAG SQLite schema limit"));
    }
    Ok(())
}
fn singleton<'a>(d: &'a SqliteDatabase, name: &str, width: usize) -> Result<&'a SqliteRow, ValueError> {
    let row = d.table(name)?.single_row()?;
    if row.rowid <= 0 || row.values.len() != width || row.integer(0)? != row.rowid {
        return Err(ValueError::new(ValueRefusalKind::InvalidValue, "DAG SQLite identity or width"));
    }
    Ok(row)
}
fn valid(snapshot: &DagSnapshot) -> Result<(), ValueError> {
    snapshot.validate().map_err(|message| ValueError::new(ValueRefusalKind::InvalidValue, message))
}
impl store::ArtifactSqliteSnapshot for DagSnapshot {
    const SQLITE_SCHEMA: &'static str = SQL;
    fn to_sqlite_database(&self, c: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, ValueError> {
        admit(c, SqliteSnapshotPhase::ProjectSnapshot)?;
        valid(self)?;
        let mut out = Projection::new(SQL, c)?;
        let document = out.insert("dag_document", &[Cell::Text(&self.schema)])?;
        let child = &self.content;
        let target = &child.target;
        out.insert("dag_content_child", &[Cell::Integer(document), Cell::Text(&child.child_id), Cell::Text(&target.artifact_id), Cell::Text(&target.dialect.artifact_kind), Cell::Text(&target.dialect.standard), Cell::Text(&target.dialect.subset)])?;
        out.checkpoint_total(2)?;
        out.finish()
    }
    fn from_sqlite_database(d: &SqliteDatabase, c: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError> {
        admit(c, SqliteSnapshotPhase::ReconstructSnapshot)?;
        store::sqlite_snapshot::validate_sqlite_database_schema(d, SQL, c.limits())?;
        c.check_database(d, SqliteSnapshotPhase::ReconstructSnapshot)?;
        let document = singleton(d, "dag_document", 2)?;
        let child = singleton(d, "dag_content_child", 7)?;
        if child.integer(1)? != document.rowid {
            return Err(ValueError::new(ValueRefusalKind::InvalidValue, "DAG SQLite orphan child relationship"));
        }
        let mut read = Reconstruction::new(c)?;
        let schema = read.text(document.text(1)?)?;
        let content = DagContentChild::new(
            read.text(child.text(2)?)?,
            semio_framework_artifact_reference::ArtifactRef { artifact_id: read.text(child.text(3)?)?, dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: read.text(child.text(4)?)?, standard: read.text(child.text(5)?)?, subset: read.text(child.text(6)?)? } },
        );
        let value = semio_framework_value::DecodedValue::new(Self { schema, content }, <Self as semio_framework_value::FromValue>::retire_decoded);
        valid(value.get())?;
        Ok(value.take())
    }
    fn decode_sqlite_snapshot_native(payload: &store::io_schema::IoPayload, c: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError> {
        admit(c, SqliteSnapshotPhase::DecodeNative)?;
        store::decode_sqlite_snapshot_record_native(
            payload,
            <Self as store::ArtifactDsl>::envelope_id(),
            Self::__dsl_spec_producer(),
            |record, native| {
                let value = Self::__dsl_from_record_controlled(record, native)?;
                let guard = semio_framework_value::DecodedValue::new(value, <Self as semio_framework_value::FromValue>::retire_decoded);
                guard.get().validate().map_err(|message|ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,message))?;
                Ok(guard.take())
            },
            c,
        )
    }
    fn encode_sqlite_snapshot_native(&self, encoding: SnapshotEncoding, c: &mut SqliteSnapshotControl<'_>) -> Result<store::io_schema::IoPayload, ValueError> {
        admit(c, SqliteSnapshotPhase::EncodeNative)?;
        valid(self)?;
        store::encode_sqlite_snapshot_record_native(encoding, <Self as store::ArtifactDsl>::envelope_id(), Self::__dsl_spec_producer(), |native| self.__dsl_to_record_controlled(native), c)
    }
    fn retire_sqlite_snapshot(self) {
        <Self as semio_framework_value::FromValue>::retire_decoded(self)
    }
    fn validate_sqlite_snapshot_subset(&self, dialect: &semio_framework_artifact_reference::ArtifactDialect, _d: &SqliteDatabase, c: &mut SqliteSnapshotControl<'_>) -> store::io_schema::IoResult<()> {
        c.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 2).map_err(store::io_schema::IoError::from_value_error)?;
        if dialect.artifact_kind != "s.dag.dag" || dialect.standard != "1" || dialect.subset != "*" {
            return Err(store::io_schema::IoError::from_value_error(ValueError::new(ValueRefusalKind::UnsupportedOwner, "DAG SQLite dialect mismatch")));
        }
        valid(self).map_err(store::io_schema::IoError::from_value_error)?;
        Ok(store::io_schema::IoOutcome::clean(()))
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

