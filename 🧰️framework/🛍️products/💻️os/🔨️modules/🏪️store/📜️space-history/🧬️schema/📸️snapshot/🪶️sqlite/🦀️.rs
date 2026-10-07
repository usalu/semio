//! 📜️ Literal space checkpoint, clock, author, member and alternative entities.
#[path="📏️preflight/🦀️.rs"]mod preflight;
use crate::os_spr::HybridLogicalTimestamp;
use crate::os_store::{ArtifactSqliteSnapshot, S_SPACE_HISTORY_SCHEMA, SpaceAlternative, SpaceCheckpoint, SpaceHistoryMutation, SpaceHistorySnapshot, SpaceMemberPin};
use crate::os_vcs::Author;
use crate::sqlite_snapshot::{
    SnapshotEncoding, SqliteDatabase, SqliteSnapshotControl, SqliteSnapshotPhase,
    artifact::{Cell, RowWriter},
    validate_sqlite_database_schema,
};
use semio_framework_value::{ValueError, ValueRefusalKind};
#[path = "../../../../../🪐️space/🪶️sqlite/🦀️.rs"]
mod fields;
#[path = "🚦️native/🦀️.rs"]
mod native;
pub(crate) use native::decode_with as decode_native_cst;
#[path = "🛂️admission/🦀️.rs"]
pub(crate) mod admission;
/// 🚪️ The new explicitly authored SQLite coordinate of the real persisted history owner.
pub const SQLITE_SNAPSHOT_DIALECT: semio_framework_artifact_reference::Dialect = semio_framework_artifact_reference::Dialect { artifact_kind: S_SPACE_HISTORY_SCHEMA, standard: semio_framework_artifact_reference::StandardId("1"), subset: semio_framework_artifact_reference::SubsetId("*") };
/// 📣️ Explicit owner registration publishes the history factory and semantic capability atomically.
/// Call before registry I/O; generic Store construction and retained hydration do not publish codecs.
pub fn register_sqlite_snapshot() -> Result<(), crate::os_io::ArtifactAssemblyRegistryError> {
    crate::os_io::register_native_snapshot_codec(SQLITE_SNAPSHOT_DIALECT, crate::os_store::ArtifactCodec::bare::<SpaceHistorySnapshot, SpaceHistoryMutation>(S_SPACE_HISTORY_SCHEMA))
}
fn schema(control: &mut SqliteSnapshotControl<'_>) -> Result<(), ValueError> {
    if SpaceHistorySnapshot::SQLITE_SCHEMA.len() > control.limits().max_schema_bytes { Err(ValueError::new(ValueRefusalKind::OwnershipLimit, "space history authored schema byte limit exceeded")) } else { Ok(()) }
}
fn rows(value: &SpaceHistorySnapshot, control: &mut SqliteSnapshotControl<'_>, phase: SqliteSnapshotPhase) -> Result<usize, ValueError> {
    let add = |left: usize, right: usize| left.checked_add(right).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "space row count overflow"));
    let work = add(value.checkpoints.len(), value.alternatives.len())?;
    control.checkpoint(phase, 0, work)?;
    let mut count = 1;
    for (index, row) in value.checkpoints.iter().enumerate() {
        count = add(count, add(2, add(row.authors.len(), row.members.len())?)?)?;
        control.check_rows(count)?;
        if (index + 1) % 256 == 0 {
            control.checkpoint(phase, index + 1, work)?;
        }
    }
    for (index, row) in value.alternatives.iter().enumerate() {
        count = add(count, add(1, row.checkpoint_ids.len())?)?;
        control.check_rows(count)?;
        if (index + 1) % 256 == 0 {
            control.checkpoint(phase, value.checkpoints.len() + index + 1, work)?;
        }
    }
    control.check_rows(count)?;
    control.checkpoint(phase, work, work)?;
    Ok(count)
}
fn word(row: &crate::sqlite_snapshot::SqliteRow, index: usize) -> Result<u64, ValueError> {
    let high = u32::try_from(row.integer(index)?).map_err(|_| fields::invalid("space history high word exceeds u32"))?;
    let low = u32::try_from(row.integer(index + 1)?).map_err(|_| fields::invalid("space history low word exceeds u32"))?;
    Ok((u64::from(high) << 32) | u64::from(low))
}
fn pair(value: u64) -> [Cell<'static>; 2] {
    [Cell::Integer((value >> 32) as i64), Cell::Integer((value & 0xffff_ffff) as i64)]
}
fn write_rows(value:&SpaceHistorySnapshot,total:usize,p:&mut RowWriter<'_, '_>)->Result<(),ValueError>{
            let document = p.insert("space_history_document", &[value.active_alternative_id.as_deref().map(Cell::Text).unwrap_or(Cell::Null)])?;
            p.checkpoint_total(total)?;
            for (index, row) in value.checkpoints.iter().enumerate() {
                let checkpoint =
                    p.insert("space_history_checkpoint", &[Cell::Integer(document), Cell::Integer(fields::ordinal(index)?), Cell::Text(&row.id), row.parent_id.as_deref().map(Cell::Text).unwrap_or(Cell::Null), Cell::Text(&row.message)])?;
                p.checkpoint_total(total)?;
                let actor = pair(row.timestamp.actor);
                let physical = pair(row.timestamp.physical_ms);
                let logical = pair(row.timestamp.logical);
                p.insert_key("space_history_timestamp", checkpoint, &[actor[0], actor[1], physical[0], physical[1], logical[0], logical[1]])?;
                p.checkpoint_total(total)?;
                for (index, author) in row.authors.iter().enumerate() {
                    p.insert("space_history_author", &[Cell::Integer(checkpoint), Cell::Integer(fields::ordinal(index)?), Cell::Text(&author.id), Cell::Text(&author.name), author.avatar.as_deref().map(Cell::Text).unwrap_or(Cell::Null)])?;
                    p.checkpoint_total(total)?;
                }
                for (index, pin) in row.members.iter().enumerate() {
                    p.insert("space_history_member_pin", &[Cell::Integer(checkpoint), Cell::Integer(fields::ordinal(index)?), Cell::Text(&pin.document_id), Cell::Text(&pin.checkpoint_id), Cell::Text(&pin.alternative_id)])?;
                    p.checkpoint_total(total)?;
                }
            }
            for (index, row) in value.alternatives.iter().enumerate() {
                let alternative = p.insert("space_history_alternative", &[Cell::Integer(document), Cell::Integer(fields::ordinal(index)?), Cell::Text(&row.id), Cell::Text(&row.name)])?;
                p.checkpoint_total(total)?;
                for (index, id) in row.checkpoint_ids.iter().enumerate() {
                    p.insert("space_history_alternative_checkpoint", &[Cell::Integer(alternative), Cell::Integer(fields::ordinal(index)?), Cell::Text(id)])?;
                    p.checkpoint_total(total)?;
                }
            }
    Ok(())
}
fn semantic(value:&SpaceHistorySnapshot,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 schema(control)?;let total=rows(value,control,phase)?;let mut writer=RowWriter::borrowed(control,phase)?;write_rows(value,total,&mut writer)?;writer.finish_borrowed()
}
impl ArtifactSqliteSnapshot for SpaceHistorySnapshot {
    const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");
    fn preflight_sqlite_snapshot_encoding(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{preflight::check(self,encoding,control)}
    fn encode_sqlite_snapshot_native(&self, encoding: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<crate::io_schema::IoPayload, ValueError> {
        semantic(self,SqliteSnapshotPhase::EncodeNative,control)?;
        native::encode(self, encoding, control)
    }
    fn decode_sqlite_snapshot_native(payload: &crate::io_schema::IoPayload, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError> {
        schema(control)?;
        native::decode(payload, control)
    }
    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, ValueError> {
        let result = (|| -> Result<SqliteDatabase, ValueError> {
            schema(control)?;
            let total = rows(self, control, SqliteSnapshotPhase::ProjectSnapshot)?;
            let mut p = RowWriter::new(Self::SQLITE_SCHEMA, control)?;
            write_rows(self,total,&mut p)?;
            p.finish()
        })();
        result
    }
    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError> {
        schema(control)?;
        validate_sqlite_database_schema(database, Self::SQLITE_SCHEMA, control.limits())?;
        control.check_database(database, SqliteSnapshotPhase::ReconstructSnapshot)?;
        let document = database.table("space_history_document")?.single_row()?;
        if document.values.len() != 2 || document.integer(0)? != document.rowid {
            return Err(ValueError::new(ValueRefusalKind::InvalidValue, "space history document identity differs"));
        }
        fields::reconstruct(control, |mut c| {
            c.charge(std::mem::size_of::<Self>())?;
            let active_alternative_id = fields::optional(document, 1, &mut c)?;
            let checkpoint_table = database.table("space_history_checkpoint")?;
            let checkpoint_keys = fields::keyed(checkpoint_table, 6, &mut c)?;
            let checkpoint_rows = fields::ordered(checkpoint_table, document.rowid, 6, &mut c)?;
            let timestamps = fields::keyed(database.table("space_history_timestamp")?, 7, &mut c)?;
            if timestamps.len() != checkpoint_keys.len() {
                return Err(fields::invalid("space history requires one clock per checkpoint"));
            }
            let authors = fields::grouped(database.table("space_history_author")?, 6, &checkpoint_keys, &mut c)?;
            let members = fields::grouped(database.table("space_history_member_pin")?, 6, &checkpoint_keys, &mut c)?;
            let mut checkpoints = c.allocate_vec(checkpoint_rows.len())?;
            c.begin_stage(checkpoint_rows.len())?;
            for row in checkpoint_rows {
                let owner = checkpoint_keys.binary_search_by_key(&row.rowid, |row| row.rowid).map_err(|_| fields::invalid("space history checkpoint is missing"))?;
                let clock = timestamps.binary_search_by_key(&row.rowid, |row| row.rowid).map_err(|_| fields::invalid("space history checkpoint clock is missing"))?;
                let clock = timestamps[clock];
                let authors = c.scoped_stage(|c| -> Result<Vec<Author>, semio_framework_value::ValueError> {
                    let mut output = c.allocate_vec(authors[owner].len())?;
                    c.begin_stage(authors[owner].len())?;
                    for author in &authors[owner] {
                        output.push(Author { id: fields::text(author, 3, c)?, name: fields::text(author, 4, c)?, avatar: fields::optional(author, 5, c)? });
                        c.step()?;
                    }
                    Ok(output)
                })?;
                let members = c.scoped_stage(|c| -> Result<Vec<SpaceMemberPin>, semio_framework_value::ValueError> {
                    let mut output = c.allocate_vec(members[owner].len())?;
                    c.begin_stage(members[owner].len())?;
                    for pin in &members[owner] {
                        output.push(SpaceMemberPin { document_id: fields::text(pin, 3, c)?, checkpoint_id: fields::text(pin, 4, c)?, alternative_id: fields::text(pin, 5, c)? });
                        c.step()?;
                    }
                    Ok(output)
                })?;
                checkpoints.push(SpaceCheckpoint {
                    id: fields::text(row, 3, &mut c)?,
                    parent_id: fields::optional(row, 4, &mut c)?,
                    message: fields::text(row, 5, &mut c)?,
                    timestamp: HybridLogicalTimestamp { actor: word(clock, 1)?, physical_ms: word(clock, 3)?, logical: word(clock, 5)? },
                    authors,
                    members,
                });
                c.step()?;
            }
            let alternative_table = database.table("space_history_alternative")?;
            let alternative_keys = fields::keyed(alternative_table, 5, &mut c)?;
            let alternative_rows = fields::ordered(alternative_table, document.rowid, 5, &mut c)?;
            let pins = fields::grouped(database.table("space_history_alternative_checkpoint")?, 4, &alternative_keys, &mut c)?;
            let mut alternatives = c.allocate_vec(alternative_rows.len())?;
            c.begin_stage(alternative_rows.len())?;
            for row in alternative_rows {
                let owner = alternative_keys.binary_search_by_key(&row.rowid, |row| row.rowid).map_err(|_| fields::invalid("space history alternative is missing"))?;
                let checkpoint_ids = c.scoped_stage(|c| -> Result<Vec<String>, semio_framework_value::ValueError> {
                    let mut output = c.allocate_vec(pins[owner].len())?;
                    c.begin_stage(pins[owner].len())?;
                    for pin in &pins[owner] {
                        output.push(fields::text(pin, 3, c)?);
                        c.step()?;
                    }
                    Ok(output)
                })?;
                alternatives.push(SpaceAlternative { id: fields::text(row, 3, &mut c)?, name: fields::text(row, 4, &mut c)?, checkpoint_ids });
                c.step()?;
            }
            c.checkpoint()?;
            Ok(Self { checkpoints, alternatives, active_alternative_id })
        })
    }
    fn validate_sqlite_snapshot_subset(&self, dialect: &semio_framework_artifact_reference::ArtifactDialect, database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> crate::io_schema::IoResult<()> {
        (|| -> Result<crate::io_schema::IoOutcome<()>, ValueError> {
            control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 0)?;
            if dialect.artifact_kind != S_SPACE_HISTORY_SCHEMA || dialect.standard != "1" || dialect.subset != "*" {
                return Err(ValueError::new(ValueRefusalKind::InvalidValue, "space history does not own this SQLite coordinate"));
            }
            let candidate = Self::from_sqlite_database(database, control)?;
            if self != &candidate {
                return Err(ValueError::new(ValueRefusalKind::InvalidValue, "space history semantic state differs"));
            }
            Ok(crate::io_schema::IoOutcome::clean(()))
        })()
        .map_err(crate::io_schema::IoError::from_value_error)
    }
}
