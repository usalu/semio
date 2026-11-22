//! 🔌️ Jack parent state and literal Graph child; the manifest is an interned registry resource named by `manifest_id`, never stored here.
use pack::snapshot::ArtifactSqliteSnapshot as _;
use crate::standards::v1::subsets::any::schema::snapshot::JackSnapshot;
use crate::standards::v1::subsets::any::schema::graph::Camera;
use crate::standards::v1::subsets::any::schema::content::JackContentChild;
use semio_framework_value::{ValueError, ValueRefusalKind};
use std::collections::BTreeSet;
use store::sqlite_snapshot::{
    artifact::{insert_ieee754, read_binary64, Cell, FloatColumn, Projection, Reconstruction},
    SnapshotEncoding, SqliteDatabase, SqliteRow, SqliteSnapshotControl, SqliteSnapshotPhase,
};
const SQL: &str = include_str!("🗄️.sql");
const TABLES: [(&str, usize); 3] = [("jack_document", 6), ("jack_camera", 11), ("jack_content_child", 7)];
const CAMERA: [FloatColumn; 3] = [FloatColumn::Binary64(2), FloatColumn::Binary64(3), FloatColumn::Binary64(4)];
fn optional(value: &Option<String>) -> Cell<'_> {
    value.as_deref().map(Cell::Text).unwrap_or(Cell::Null)
}
pub(crate) fn forecast(_: &JackSnapshot, c: &mut SqliteSnapshotControl<'_>, phase: SqliteSnapshotPhase) -> Result<usize, ValueError> {
    c.checkpoint(phase, 0, 0)?;
    let limits = c.limits();
    if limits.max_tables < 3 || limits.max_columns < 11 {
        return Err(ValueError::new(ValueRefusalKind::WorkLimit, "Jack schema table or column limit"));
    }
    if SQL.len() + TABLES.iter().map(|(name, _)| name.len()).sum::<usize>() > limits.max_schema_bytes {
        return Err(ValueError::new(ValueRefusalKind::OwnershipLimit, "Jack schema byte limit"));
    }
    c.check_rows(3)?;
    c.checkpoint(phase, 3, 3)?;
    Ok(3)
}
pub(crate) fn project(value: &JackSnapshot, c: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, ValueError> {
    let total = forecast(value, c, SqliteSnapshotPhase::ProjectSnapshot)?;
    let mut out = Projection::new(SQL, c)?;
    let doc = out.insert("jack_document", &[Cell::Text(&value.schema), Cell::Text(&value.name), Cell::Text(&value.manifest_id), optional(&value.root_node_id), Cell::Text(&value.query)])?;
    insert_ieee754(&mut out, "jack_camera", &[Cell::Integer(doc), Cell::Real(value.camera.x), Cell::Real(value.camera.y), Cell::Real(value.camera.zoom)], &CAMERA)?;
    let child = &value.content;
    let target = &child.target;
    out.insert("jack_content_child", &[Cell::Integer(doc), Cell::Text(&child.child_id), Cell::Text(&target.artifact_id), Cell::Text(&target.dialect.artifact_kind), Cell::Text(&target.dialect.standard), Cell::Text(&target.dialect.subset)])?;
    out.checkpoint_total(total)?;
    out.finish()
}
struct Rows<'a> {
    database: &'a SqliteDatabase,
    used: BTreeSet<(usize, i64)>,
}
impl<'a> Rows<'a> {
    fn new(database: &'a SqliteDatabase, c: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError> {
        store::sqlite_snapshot::validate_sqlite_database_schema(database, SQL, c.limits())?;
        c.check_database(database, SqliteSnapshotPhase::ReconstructSnapshot)?;
        for (name, width) in TABLES {
            for row in &database.table(name)?.rows {
                if row.rowid <= 0 || row.values.len() != width || row.integer(0)? != row.rowid {
                    return Err(ValueError::new(ValueRefusalKind::InvalidValue, "Jack row identity or width"));
                }
            }
        }
        Ok(Self { database, used: BTreeSet::new() })
    }
    fn singleton(&mut self, index: usize) -> Result<&'a SqliteRow, ValueError> {
        let row = self.database.table(TABLES[index].0)?.single_row()?;
        if !self.used.insert((index, row.rowid)) {
            return Err(ValueError::new(ValueRefusalKind::InvalidValue, "Jack entity has multiple owners or a cycle"));
        }
        Ok(row)
    }
    fn finish(&self) -> Result<(), ValueError> {
        if self.used.len() != self.database.tables.iter().map(|t| t.rows.len()).sum::<usize>() {
            return Err(ValueError::new(ValueRefusalKind::InvalidValue, "Jack unowned entity"));
        }
        Ok(())
    }
}
fn text(row: &SqliteRow, index: usize, c: &mut SqliteSnapshotControl<'_>) -> Result<String, ValueError> {
    Reconstruction::new(c)?.text(row.text(index)?)
}
fn optional_text(row: &SqliteRow, index: usize, c: &mut SqliteSnapshotControl<'_>) -> Result<Option<String>, ValueError> {
    row.optional_text(index)?.map(|value| Reconstruction::new(c)?.text(value)).transpose()
}
pub(crate) fn reconstruct(database: &SqliteDatabase, c: &mut SqliteSnapshotControl<'_>) -> Result<JackSnapshot, ValueError> {
    let mut rows = Rows::new(database, c)?;
    let document = rows.singleton(0)?;
    let schema = text(document, 1, c)?;
    let name = text(document, 2, c)?;
    let manifest_id = text(document, 3, c)?;
    let root_node_id = optional_text(document, 4, c)?;
    let query = text(document, 5, c)?;
    let camera = rows.singleton(1)?;
    let child = rows.singleton(2)?;
    if camera.integer(1)? != document.rowid || child.integer(1)? != document.rowid {
        return Err(ValueError::new(ValueRefusalKind::InvalidValue, "Jack parent singleton relationship"));
    }
    for _ in 0..3 {
        Reconstruction::new(c)?.scalar()?;
    }
    let camera = Camera { x: read_binary64(camera, 2, &CAMERA)?, y: read_binary64(camera, 3, &CAMERA)?, zoom: read_binary64(camera, 4, &CAMERA)? };
    let content = JackContentChild::new(
        text(child, 2, c)?,
        semio_framework_artifact_reference::ArtifactRef { artifact_id: text(child, 3, c)?, dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: text(child, 4, c)?, standard: text(child, 5, c)?, subset: text(child, 6, c)? } },
    );
    let value = semio_framework_value::DecodedValue::new(JackSnapshot { schema, name, manifest_id, camera, content, root_node_id, query }, retire_snapshot);
    rows.finish()?;
    validate(value.get())?;
    Reconstruction::new(c)?.checkpoint()?;
    Ok(value.take())
}

pub(crate) fn validate(value: &JackSnapshot) -> Result<(), ValueError> {
    value.validate_schema().map_err(|error| ValueError::new(ValueRefusalKind::InvalidValue, error.to_string()))?;
    Ok(())
}
pub(crate) fn retire_snapshot(value: JackSnapshot) {
    crate::standards::v1::subsets::any::schema::content::retire_owned_to_terminal(value).expect("Jack explicit retirement");
}
impl store::os_store::serialization::ArtifactSqliteSnapshotHost for JackSnapshot {
    
    
    
    fn decode_sqlite_snapshot_native(
        payload: &store::io_schema::IoPayload,
        c: &mut SqliteSnapshotControl<'_>,
        native_owner: &mut store::NativeSnapshotDecodeOwner<'_, '_>,
        output: &mut Option<store::os_store::serialization::SnapshotReceivingOutput<Self>>,
    ) -> store::os_store::serialization::SnapshotReceivingOutcome {
        store::decode_sqlite_snapshot_record_receiving(
            payload,
            <Self as pack::snapshot::ArtifactDsl>::envelope_id(),
            crate::standards::v1::subsets::any::io::text::snapshot::JackPackRecord::__dsl_spec_producer(),
            &mut |record, snapshot_output, owner, sql| store::native_snapshot_turn(|work| {
                let snapshot = work.measured(owner.native(), |native| {
                    let flat = crate::standards::v1::subsets::any::io::text::snapshot::JackPackRecord::__dsl_from_record_controlled(record, native)?;
                    flat.admit_rows(sql.limits().max_rows, native)?;
                    flat.into_snapshot_controlled(native)
                })?;
                forecast(snapshot_output.insert(snapshot), sql, SqliteSnapshotPhase::DecodeNative)?;
                Ok(())
            }),
            c,
            native_owner,
            output,
        )
    }
    fn encode_sqlite_snapshot_native(&self, encoding: SnapshotEncoding, c: &mut SqliteSnapshotControl<'_>,native_owner:&mut semio_os_kernel::NativeSnapshotEncodeOwner<'_, '_>,output:&mut Option<store::os_store::serialization::SnapshotReceivingOutput<store::io_schema::IoPayload>>)->store::os_store::serialization::SnapshotReceivingOutcome {if let Err(error)=self.preflight_sqlite_snapshot_encoding(encoding,c){return semio_framework_value::RetainedWorkOutcome{result:Err(store::sqlite_snapshot::SqliteStageFailure::NotStarted(error)),work:0}}store::encode_sqlite_snapshot_record_native(
            encoding,
            <Self as pack::snapshot::ArtifactDsl>::envelope_id(),
            crate::standards::v1::subsets::any::io::text::snapshot::JackPackRecord::__dsl_spec_producer(), &mut |native| {
                let flat = crate::standards::v1::subsets::any::io::text::snapshot::JackPackRecord::from_snapshot_controlled(self, native)?;
                flat.__dsl_to_record_controlled(native)
            },
            c,
        native_owner,output)}
    fn preflight_sqlite_snapshot_encoding(&self, _: SnapshotEncoding, c: &mut SqliteSnapshotControl<'_>) -> Result<(), ValueError> {
        validate(self)?;
        forecast(self, c, SqliteSnapshotPhase::EncodeNative)?;
        Ok(())
    }
    fn retire_sqlite_snapshot(self) {
        retire_snapshot(self)
    }
    fn validate_sqlite_snapshot_subset(&self, dialect: &semio_framework_artifact_reference::ArtifactDialect, _: &SqliteDatabase, c: &mut SqliteSnapshotControl<'_>) -> store::io_schema::IoResult<()> {
        (|| -> Result<_, ValueError> {
            c.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 0)?;
            if dialect.artifact_kind != "s.trinity.jack" || dialect.standard != "1" || dialect.subset != "*" {
                return Err(ValueError::new(ValueRefusalKind::InvalidValue, "Jack SQLite dialect mismatch"));
            }
            validate(self)?;
            Ok(store::io_schema::IoOutcome::clean(()))
        })().map_err(store::io_schema::IoError::from_value_error)
    }
}
impl pack::snapshot::ArtifactSqliteSnapshot for JackSnapshot {
const SQLITE_SCHEMA: &'static str = SQL;
fn to_sqlite_database(&self, c: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, ValueError> {
        validate(self)?;
        project(self, c)
    }
fn from_sqlite_database(database: &SqliteDatabase, c: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError> {
        reconstruct(database, c)
    }
}

