//! 🪶️ IDML semantic SQLite snapshot: one package row, ordered part rows and ordered byte rows, with the native wire reproduced exactly.
use crate::{IdmlSnapshot, DOCUMENT_SCHEMA};
use crate::schema::IdmlPart;
use semio_os_kernel as store;
use semio_os_kernel::os_store::serialization::ArtifactSqliteSnapshotHost;
use semio_os_kernel::sqlite_snapshot::{artifact::{reconstruct_text, Cell, Projection}, validate_sqlite_database_schema, SnapshotEncoding, SqliteDatabase, SqliteRow, SqliteSnapshotControl, SqliteSnapshotPhase};
use semio_framework_pack::snapshot::{ArtifactDsl, ArtifactPack};
use semio_framework_pack::snapshot::ArtifactSqliteSnapshot;
use semio_framework_value::{ValueError, ValueRefusalKind};
use std::collections::{BTreeMap, BTreeSet};

const SCHEMA: &str = "CREATE TABLE idml_package (id INTEGER PRIMARY KEY CHECK (id = 1), schema TEXT NOT NULL, dom_version TEXT NOT NULL);\nCREATE TABLE idml_part (id INTEGER PRIMARY KEY, package_id INTEGER NOT NULL REFERENCES idml_package(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0), path TEXT NOT NULL);\nCREATE TABLE idml_part_byte (id INTEGER PRIMARY KEY, part_id INTEGER NOT NULL REFERENCES idml_part(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0), value INTEGER NOT NULL CHECK (value BETWEEN 0 AND 255));\n";

fn refusal(message: impl Into<String>) -> ValueError {
    ValueError::new(ValueRefusalKind::InvalidValue, message)
}

fn ordinal(index: usize) -> Result<i64, ValueError> {
    i64::try_from(index).map_err(|error| ValueError::new(ValueRefusalKind::WorkLimit, error.to_string()))
}

fn children<'a>(database: &'a SqliteDatabase, name: &str, parents: &BTreeSet<i64>, control: &mut SqliteSnapshotControl<'_>) -> Result<BTreeMap<i64, Vec<&'a SqliteRow>>, ValueError> {
    let table = database.table(name)?;
    let mut identities = BTreeSet::new();
    let mut grouped = BTreeMap::<i64, Vec<&SqliteRow>>::new();
    for (index, row) in table.rows.iter().enumerate() {
        if index % 256 == 0 {
            control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, index, table.rows.len())?;
        }
        if row.rowid <= 0 || row.integer(0)? != row.rowid || !identities.insert(row.rowid) || !parents.contains(&row.integer(1)?) {
            return Err(refusal(format!("{name} has an invalid identity or parent")));
        }
        grouped.entry(row.integer(1)?).or_default().push(row);
    }
    for rows in grouped.values_mut() {
        let mut slots = vec![None; rows.len()];
        for row in rows.iter() {
            let index = usize::try_from(row.integer(2)?).map_err(|error| refusal(error.to_string()))?;
            let slot = slots.get_mut(index).ok_or_else(|| refusal(format!("{name} requires contiguous child ordinals")))?;
            if slot.replace(*row).is_some() {
                return Err(refusal(format!("{name} requires unique child ordinals")));
            }
        }
        *rows = slots.into_iter().map(|row| row.ok_or_else(|| refusal(format!("{name} requires contiguous child ordinals")))).collect::<Result<_, _>>()?;
    }
    Ok(grouped)
}

fn census(snapshot: &IdmlSnapshot, control: &mut SqliteSnapshotControl<'_>, phase: SqliteSnapshotPhase) -> Result<(), ValueError> {
    control.checkpoint(phase, 0, 0)?;
    let mut rows = 1usize;
    let mut bytes = DOCUMENT_SCHEMA.len().saturating_add(snapshot.dom_version.len());
    for part in &snapshot.parts {
        rows = rows.checked_add(1).and_then(|total| total.checked_add(part.bytes.len())).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "IDML semantic row count overflow"))?;
        bytes = bytes.saturating_add(part.path.len()).saturating_add(part.bytes.len().saturating_mul(32)).saturating_add(40);
        control.check_rows(rows)?;
        control.check_value_bytes(bytes)?;
    }
    control.checkpoint(phase, rows, rows)
}

impl ArtifactSqliteSnapshot for IdmlSnapshot {
    const SQLITE_SCHEMA: &'static str = SCHEMA;

    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, ValueError> {
        let mut out = Projection::new(Self::SQLITE_SCHEMA, control)?;
        out.insert("idml_package", &[Cell::Text(DOCUMENT_SCHEMA), Cell::Text(&self.dom_version)])?;
        for (index, part) in self.parts.iter().enumerate() {
            let id = out.insert("idml_part", &[Cell::Integer(1), Cell::Integer(ordinal(index)?), Cell::Text(&part.path)])?;
            for (position, value) in part.bytes.iter().enumerate() {
                out.insert("idml_part_byte", &[Cell::Integer(id), Cell::Integer(ordinal(position)?), Cell::Integer(i64::from(*value))])?;
            }
        }
        out.finish()
    }

    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError> {
        validate_sqlite_database_schema(database, Self::SQLITE_SCHEMA, control.limits())?;
        control.check_database(database, SqliteSnapshotPhase::ReconstructSnapshot)?;
        let package = database.table("idml_package")?.single_row()?;
        if package.rowid != 1 || package.integer(0)? != 1 || package.text(1)? != DOCUMENT_SCHEMA {
            return Err(refusal("IDML package identity and schema must be 1 and stdio.idml"));
        }
        let dom_version = reconstruct_text(control, package.text(2)?)?;
        let mut parts = children(database, "idml_part", &BTreeSet::from([1]), control)?;
        let parts = parts.remove(&1).unwrap_or_default();
        let keys: BTreeSet<i64> = parts.iter().map(|row| row.rowid).collect();
        let mut octets = children(database, "idml_part_byte", &keys, control)?;
        let mut result = Vec::with_capacity(parts.len());
        for row in parts {
            let bytes = octets.remove(&row.rowid).unwrap_or_default().into_iter().map(|byte| u8::try_from(byte.integer(3)?).map_err(|error| refusal(error.to_string()))).collect::<Result<Vec<_>, _>>()?;
            result.push(IdmlPart { path: reconstruct_text(control, row.text(3)?)?, bytes });
        }
        let snapshot = IdmlSnapshot { dom_version, parts: result };
        snapshot.package()?;
        Ok(snapshot)
    }
}

impl ArtifactSqliteSnapshotHost for IdmlSnapshot {
    fn preflight_sqlite_snapshot_encoding(&self, _encoding: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<(), ValueError> {
        census(self, control, SqliteSnapshotPhase::EncodeNative)
    }

    fn decode_sqlite_snapshot_native(payload: &store::io_schema::IoPayload, control: &mut SqliteSnapshotControl<'_>, native_control: &mut store::NativeSnapshotDecodeOwner<'_, '_>) -> Result<Self, ValueError> {
        native_control.native().checkpoint()?;
        control.checkpoint(SqliteSnapshotPhase::DecodeNative, 0, 1)?;
        let snapshot = match payload {
            store::io_schema::IoPayload::Binary(bytes) => Self::decode_pack(bytes).map_err(|error| refusal(error.to_string()))?,
            store::io_schema::IoPayload::Text(text) => Self::parse_dsl(text).map_err(|error| refusal(error.to_string()))?,
        };
        census(&snapshot, control, SqliteSnapshotPhase::DecodeNative)?;
        snapshot.package()?;
        Ok(snapshot)
    }

    fn encode_sqlite_snapshot_native(&self, encoding: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>, native_owner: &mut store::NativeSnapshotEncodeOwner<'_, '_>) -> Result<store::io_schema::IoPayload, ValueError> {
        native_owner.native().checkpoint()?;
        census(self, control, SqliteSnapshotPhase::EncodeNative)?;
        Ok(match encoding {
            SnapshotEncoding::Binary => store::io_schema::IoPayload::Binary(self.encode_pack()),
            SnapshotEncoding::Text => store::io_schema::IoPayload::Text(self.print_dsl()),
        })
    }
}
