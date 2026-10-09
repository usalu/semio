//! 🧱️ Handwritten Block owned relational fields.
use crate::*;
use std::collections::{BTreeMap, BTreeSet};
use store::{
    sqlite_snapshot::{
        artifact::{Cell, Projection, Reconstruction},
        SnapshotEncoding, SqliteDatabase, SqliteRow, SqliteSnapshotControl, SqliteSnapshotPhase, SqliteValue,
    },
    ArtifactSqliteSnapshot,
};
use semio_framework_value::{ValueError, ValueRefusalKind};
fn invalid(message: &'static str) -> ValueError {
    ValueError::new(ValueRefusalKind::InvalidValue, message)
}

fn class(v: f64) -> &'static str {
    if v.is_nan() {
        "nan"
    } else if v == f64::INFINITY {
        "positiveInfinity"
    } else if v == f64::NEG_INFINITY {
        "negativeInfinity"
    } else {
        "finite"
    }
}
fn text(v: &Option<String>) -> Cell<'_> {
    v.as_deref().map(Cell::Text).unwrap_or(Cell::Null)
}
fn ordinal(n: usize) -> Result<Cell<'static>, ValueError> {
    Ok(Cell::Integer(i64::try_from(n).map_err(|_| ValueError::new(ValueRefusalKind::WorkLimit, "Block ordinal exceeds INTEGER"))?))
}
struct Cells<'a> {
    values: [Cell<'a>; 32],
    len: usize,
}
impl<'a> Cells<'a> {
    fn new(values: &[Cell<'a>]) -> Self {
        let mut out = Self { values: [Cell::Null; 32], len: 0 };
        for value in values {
            out.push(*value);
        }
        out
    }
    fn push(&mut self, v: Cell<'a>) {
        self.values[self.len] = v;
        self.len += 1;
    }
    fn float(&mut self, v: f64) {
        self.push(if v.is_nan() { Cell::Null } else { Cell::Real(v) });
        self.push(Cell::Integer(v.to_bits() as i64));
        self.push(Cell::Text(class(v)));
    }
    fn optional(&mut self, v: Option<f64>) {
        if let Some(v) = v {
            self.float(v);
        } else {
            self.push(Cell::Null);
            self.push(Cell::Null);
            self.push(Cell::Null);
        }
    }
    fn insert(&self, out: &mut Projection<'_, '_>, table: usize) -> Result<i64, ValueError> {
        out.insert(TABLES[table].0, &self.values[..self.len])
    }
}
fn add(total: &mut usize, count: usize, c: &SqliteSnapshotControl<'_>) -> Result<(), ValueError> {
    *total = total.checked_add(count).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "Block row count overflow"))?;
    c.check_rows(*total)
}
fn admit_schema(c: &SqliteSnapshotControl<'_>) -> Result<(), ValueError> {
    let size = TABLES.iter().try_fold(SQL.len(), |sum, (name, _)| sum.checked_add(name.len()).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "Block schema count overflow")))?;
    if size > c.limits().max_schema_bytes {
        return Err(ValueError::new(ValueRefusalKind::OwnershipLimit, "Block authored schema exceeds byte limit"));
    }
    Ok(())
}
fn native_list(v: Option<&semio_framework_dsl_record::FieldValue>) -> Result<&[semio_framework_dsl_record::FieldValue], ValueError> {
    match v {
        None | Some(semio_framework_dsl_record::FieldValue::Absent) => Ok(&[]),
        Some(semio_framework_dsl_record::FieldValue::List(v)) => Ok(v),
        _ => Err(invalid("Block owned list required")),
    }
}
fn native_add(total: &mut usize, n: usize, maximum: usize) -> Result<(), ValueError> {
    *total = total.checked_add(n).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "Block native rows overflow"))?;
    if *total > maximum {
        return Err(ValueError::new(ValueRefusalKind::WorkLimit, "Block native rows exceed limit"));
    }
    Ok(())
}
struct Cursor<'a, 'c, 'p> {
    row: &'a SqliteRow,
    index: usize,
    c: &'c mut SqliteSnapshotControl<'p>,
}
impl<'a, 'c, 'p> Cursor<'a, 'c, 'p> {
    fn new(row: &'a SqliteRow, index: usize, c: &'c mut SqliteSnapshotControl<'p>) -> Self {
        Self { row, index, c }
    }
    fn next(&mut self) -> Result<&'a SqliteValue, ValueError> {
        let v = self.row.values.get(self.index).ok_or_else(|| invalid("Block missing cell"))?;
        self.index += 1;
        Ok(v)
    }
    fn borrowed(&mut self) -> Result<&'a str, ValueError> {
        match self.next()? {
            SqliteValue::Text(v) => Ok(v),
            _ => Err(invalid("Block TEXT required")),
        }
    }
    fn text(&mut self) -> Result<String, ValueError> {
        let v = self.borrowed()?;
        Reconstruction::new(self.c)?.text(v)
    }
    fn optional_text(&mut self) -> Result<Option<String>, ValueError> {
        if self.row.values.get(self.index) == Some(&SqliteValue::Null) {
            self.next()?;
            Ok(None)
        } else {
            self.text().map(Some)
        }
    }
    fn integer(&mut self) -> Result<i64, ValueError> {
        match self.next()? {
            SqliteValue::Integer(v) => Ok(*v),
            _ => Err(invalid("Block INTEGER required")),
        }
    }
    fn boolean(&mut self) -> Result<bool, ValueError> {
        let v = match self.integer()? {
            0 => false,
            1 => true,
            _ => return Err(invalid("Block boolean differs")),
        };
        Reconstruction::new(self.c)?.scalar()?;
        Ok(v)
    }
    fn float(&mut self) -> Result<f64, ValueError> {
        let query = self.next()?;
        let v = f64::from_bits(self.integer()? as u64);
        if self.borrowed()? != class(v) {
            return Err(invalid("Block numeric class differs"));
        }
        let matches = if v.is_nan() {
            query == &SqliteValue::Null
        } else {
            match query {
                SqliteValue::Real(q) => *q == v,
                SqliteValue::Integer(q) => v.is_finite() && v.fract() == 0.0 && v >= i64::MIN as f64 && v < -(i64::MIN as f64) && v as i64 == *q,
                _ => false,
            }
        };
        if !matches {
            return Err(invalid("Block numeric query differs"));
        }
        Reconstruction::new(self.c)?.scalar()?;
        Ok(v)
    }
    fn optional_float(&mut self) -> Result<Option<f64>, ValueError> {
        if self.row.values.get(self.index + 1) == Some(&SqliteValue::Null) {
            for _ in 0..3 {
                if self.next()? != &SqliteValue::Null {
                    return Err(invalid("Block partial optional scalar"));
                }
            }
            Ok(None)
        } else {
            self.float().map(Some)
        }
    }
    fn done(self) -> Result<(), ValueError> {
        if self.index != self.row.values.len() {
            return Err(invalid("Block extra cell"));
        }
        Ok(())
    }
}
struct Rows<'a> {
    groups: BTreeMap<(usize, i64), BTreeMap<i64, &'a SqliteRow>>,
}
impl<'a> Rows<'a> {
    fn new(d: &'a SqliteDatabase, c: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError> {
        store::sqlite_snapshot::validate_sqlite_database_schema(d, SQL, c.limits())?;
        c.check_database(d, SqliteSnapshotPhase::ReconstructSnapshot)?;
        let mut ids: BTreeMap<usize, BTreeSet<i64>> = BTreeMap::new();
        let mut groups = BTreeMap::<(usize, i64), BTreeMap<i64, &SqliteRow>>::new();
        for (t, (name, width)) in TABLES.iter().enumerate() {
            for (n, row) in d.table(name)?.rows.iter().enumerate() {
                if row.rowid <= 0 || row.integer(0)? != row.rowid || row.values.len() != *width || !ids.entry(t).or_default().insert(row.rowid) {
                    return Err(invalid("Block alias/width differs"));
                }
                if n % 256 == 0 {
                    c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, n, d.table(name)?.rows.len())?;
                }
            }
        }
        for (t, (name, _)) in TABLES.iter().enumerate() {
            for (n, row) in d.table(name)?.rows.iter().enumerate() {
                let parent = if t == 0 { 0 } else { row.integer(1)? };
                if t != 0 && !ids.get(&PARENTS[t]).is_some_and(|ids| ids.contains(&parent)) {
                    return Err(invalid("Block orphan ownership"));
                }
                let order = if SINGLETONS.contains(&t) {
                    row.rowid
                } else {
                    let order = row.integer(2)?;
                    if order < 0 {
                        return Err(invalid("Block negative ordinal"));
                    }
                    order
                };
                if groups.entry((t, parent)).or_default().insert(order, row).is_some() {
                    return Err(invalid("Block duplicate ordinal"));
                }
                if n % 256 == 0 {
                    c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, n, d.table(name)?.rows.len())?;
                }
            }
        }
        Ok(Self { groups })
    }
    fn one(&mut self, t: usize, parent: i64) -> Result<&'a SqliteRow, ValueError> {
        let group = self.groups.remove(&(t, parent)).unwrap_or_default();
        if group.len() != 1 {
            return Err(invalid("Block singleton cardinality differs"));
        }
        group.into_values().next().ok_or_else(|| invalid("Block singleton missing"))
    }
    fn take(&mut self, t: usize, parent: i64, c: &mut SqliteSnapshotControl<'_>) -> Result<Vec<&'a SqliteRow>, ValueError> {
        let group = self.groups.remove(&(t, parent)).unwrap_or_default();
        let total = group.len();
        c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 0, total)?;
        let mut out = Vec::new();
        out.try_reserve_exact(total).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "Block relationship allocation failed"))?;
        for (n, (order, row)) in group.into_iter().enumerate() {
            if order != i64::try_from(n).map_err(|_| ValueError::new(ValueRefusalKind::WorkLimit, "Block ordinal exceeds INTEGER"))? {
                return Err(invalid("Block noncontiguous ordinal"));
            }
            out.push(row);
            if (n + 1) % 256 == 0 {
                c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, n + 1, total)?;
            }
        }
        c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, total, total)?;
        Ok(out)
    }
    fn finish(self, c: &mut SqliteSnapshotControl<'_>) -> Result<(), ValueError> {
        if !self.groups.is_empty() {
            return Err(invalid("Block unowned entity"));
        }
        c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 1, 1)
    }
}
fn project_kind(out: &mut Projection<'_, '_>, owner: i64, k: &BlockKindIdentity) -> Result<(), ValueError> {
    out.insert(TABLES[1].0, &[Cell::Integer(owner), Cell::Text(&k.id), Cell::Text(&k.name), Cell::Text(&k.label), text(&k.variant), Cell::Text(&k.description), text(&k.icon), text(&k.unit)])?;
    Ok(())
}
fn kind(row: &SqliteRow, c: &mut SqliteSnapshotControl<'_>) -> Result<BlockKindIdentity, ValueError> {
    let mut v = Cursor::new(row, 2, c);
    let out = BlockKindIdentity { id: v.text()?, name: v.text()?, label: v.text()?, variant: v.optional_text()?, description: v.text()?, icon: v.optional_text()?, unit: v.optional_text()? };
    v.done()?;
    Ok(out)
}
fn attribute(row: &SqliteRow, c: &mut SqliteSnapshotControl<'_>) -> Result<BlockAttribute, ValueError> {
    let mut v = Cursor::new(row, 3, c);
    let out = BlockAttribute { key: v.text()?, value: v.text()?, definition: v.optional_text()? };
    v.done()?;
    Ok(out)
}
fn author(row: &SqliteRow, c: &mut SqliteSnapshotControl<'_>) -> Result<BlockAuthor, ValueError> {
    let mut v = Cursor::new(row, 3, c);
    let out = BlockAuthor { id: v.text()?, name: v.text()?, email: v.optional_text()? };
    v.done()?;
    Ok(out)
}
fn compatibility(row: &SqliteRow, c: &mut SqliteSnapshotControl<'_>) -> Result<BlockCompatibilityRule, ValueError> {
    let mut v = Cursor::new(row, 3, c);
    let out = BlockCompatibilityRule { id: v.text()?, source: v.text()?, target: v.text()?, bidirectional: v.boolean()? };
    v.done()?;
    Ok(out)
}

const SQL: &str = include_str!("🗄️.sql");
const TABLES: [(&str, usize); 10] =
    [("block2_document", 2), ("block2_kind", 9), ("block2_presentation", 14), ("block2_handle_kind", 8), ("block2_handle", 11), ("block2_compatibility", 7), ("block2_attribute", 6), ("block2_author", 6), ("block2_camera2d", 11), ("block2_meta", 3)];
const PARENTS: [usize; 10] = [0; 10];
const SINGLETONS: [usize; 5] = [0, 1, 2, 8, 9];
fn forecast(v: &Block2dSnapshot, c: &mut SqliteSnapshotControl<'_>, phase: SqliteSnapshotPhase) -> Result<usize, ValueError> {
    admit_schema(c)?;
    let mut total = 5;
    c.check_rows(total)?;
    c.checkpoint(phase, 0, 0)?;
    for count in [v.handle_kinds.len(), v.handles.len(), v.compatibility.len(), v.attributes.len(), v.authors.len()] {
        add(&mut total, count, c)?;
    }
    c.checkpoint(phase, total, total)?;
    Ok(total)
}
fn admit_native_rows(record: &semio_framework_dsl_record::RecordValue, maximum: usize, native: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<(),semio_framework_value::ValueError> {
    native
        .scoped_stage(|native| -> Result<(), ValueError> {
            let mut total = 5;
            native_add(&mut total, 0, maximum)?;
            native.begin_stage(5)?;
            for field in [3, 4, 5, 6, 7] {
                native_add(&mut total, native_list(record.get(field))?.len(), maximum)?;
                native.step()?;
            }
            Ok(())
        })
        
}
impl ArtifactSqliteSnapshot for Block2dSnapshot {
    const SQLITE_SCHEMA: &'static str = SQL;
    fn decode_sqlite_snapshot_native(payload: &store::io_schema::IoPayload, c: &mut SqliteSnapshotControl<'_>,native_control: &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>) -> Result<Self, ValueError> {
        admit_schema(c)?;
        c.check_rows(5)?;
        let maximum = c.limits().max_rows;
        store::decode_sqlite_snapshot_record_native(
            payload,
            <Self as store::ArtifactDsl>::envelope_id(),
            Self::__dsl_spec_producer(),
            |record, snapshot_output, native,_body| { let constructed: Result<_, semio_framework_value::ValueError> = (|| {
                admit_native_rows(record, maximum, native)?;
                Self::__dsl_from_record_controlled(record, native)
            })(); *snapshot_output = Some(constructed?); Ok(()) },
            c,
        native_control)
    }
    fn encode_sqlite_snapshot_native(&self, encoding: SnapshotEncoding, c: &mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>) -> Result<store::io_schema::IoPayload, ValueError> {
        forecast(self, c, SqliteSnapshotPhase::EncodeNative)?;
        store::encode_sqlite_snapshot_record_native(encoding, <Self as store::ArtifactDsl>::envelope_id(), Self::__dsl_spec_producer(), |native| self.__dsl_to_record_controlled(native), c,native_owner)
    }
    fn to_sqlite_database(&self, c: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, ValueError> {
        let total = forecast(self, c, SqliteSnapshotPhase::ProjectSnapshot)?;
        let mut out = Projection::new(SQL, c)?;
        let doc = out.insert(TABLES[0].0, &[Cell::Text(&self.schema)])?;
        project_kind(&mut out, doc, &self.node_kind)?;
        let v = &self.presentation;
        let mut row = Cells::new(&[Cell::Integer(doc), text(&v.shape)]);
        row.optional(v.radius);
        row.optional(v.width);
        row.optional(v.height);
        row.push(text(&v.color));
        row.push(text(&v.icon_kind));
        row.insert(&mut out, 2)?;
        for (n, v) in self.handle_kinds.iter().enumerate() {
            out.insert(TABLES[3].0, &[Cell::Integer(doc), ordinal(n)?, Cell::Text(&v.id), Cell::Text(&v.name), Cell::Text(&v.label), Cell::Text(&v.color), Cell::Text(&v.default_wire_kind)])?;
        }
        for (n, v) in self.handles.iter().enumerate() {
            let mut row = Cells::new(&[Cell::Integer(doc), ordinal(n)?, Cell::Text(&v.id), Cell::Text(&v.handle_kind)]);
            row.float(v.angle);
            row.float(v.radius);
            row.insert(&mut out, 4)?;
        }
        for (n, v) in self.compatibility.iter().enumerate() {
            out.insert(TABLES[5].0, &[Cell::Integer(doc), ordinal(n)?, Cell::Text(&v.id), Cell::Text(&v.source), Cell::Text(&v.target), Cell::Integer(i64::from(v.bidirectional))])?;
        }
        for (n, v) in self.attributes.iter().enumerate() {
            out.insert(TABLES[6].0, &[Cell::Integer(doc), ordinal(n)?, Cell::Text(&v.key), Cell::Text(&v.value), text(&v.definition)])?;
        }
        for (n, v) in self.authors.iter().enumerate() {
            out.insert(TABLES[7].0, &[Cell::Integer(doc), ordinal(n)?, Cell::Text(&v.id), Cell::Text(&v.name), text(&v.email)])?;
        }
        let v = &self.camera2d;
        let mut row = Cells::new(&[Cell::Integer(doc)]);
        row.float(v.x);
        row.float(v.y);
        row.float(v.zoom);
        row.insert(&mut out, 8)?;
        out.insert(TABLES[9].0, &[Cell::Integer(doc), Cell::Text(&self.meta.description)])?;
        out.checkpoint_total(total)?;
        out.finish()
    }
    fn from_sqlite_database(d: &SqliteDatabase, c: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError> {
        let mut rows = Rows::new(d, c)?;
        let doc = rows.one(0, 0)?;
        let mut v = Cursor::new(doc, 1, c);
        let schema = v.text()?;
        v.done()?;
        let node_kind = kind(rows.one(1, doc.rowid)?, c)?;
        let mut v = Cursor::new(rows.one(2, doc.rowid)?, 2, c);
        let presentation = Block2dPresentation { shape: v.optional_text()?, radius: v.optional_float()?, width: v.optional_float()?, height: v.optional_float()?, color: v.optional_text()?, icon_kind: v.optional_text()? };
        v.done()?;
        let (mut handle_kinds, mut handles, mut rules, mut attributes, mut authors) = (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new());
        for row in rows.take(3, doc.rowid, c)? {
            let mut v = Cursor::new(row, 3, c);
            handle_kinds.push(Block2dHandleKind { id: v.text()?, name: v.text()?, label: v.text()?, color: v.text()?, default_wire_kind: v.text()? });
            v.done()?;
        }
        for row in rows.take(4, doc.rowid, c)? {
            let mut v = Cursor::new(row, 3, c);
            handles.push(Block2dHandleTemplate { id: v.text()?, handle_kind: v.text()?, angle: v.float()?, radius: v.float()? });
            v.done()?;
        }
        for row in rows.take(5, doc.rowid, c)? {
            rules.push(compatibility(row, c)?);
        }
        for row in rows.take(6, doc.rowid, c)? {
            attributes.push(attribute(row, c)?);
        }
        for row in rows.take(7, doc.rowid, c)? {
            authors.push(author(row, c)?);
        }
        let mut v = Cursor::new(rows.one(8, doc.rowid)?, 2, c);
        let camera2d = BlockCamera2d { x: v.float()?, y: v.float()?, zoom: v.float()? };
        v.done()?;
        let mut v = Cursor::new(rows.one(9, doc.rowid)?, 2, c);
        let meta = BlockMeta { description: v.text()? };
        v.done()?;
        rows.finish(c)?;
        Ok(Self { schema, node_kind, presentation, handle_kinds, handles, compatibility: rules, attributes, authors, camera2d, meta })
    }
    fn validate_sqlite_snapshot_subset(&self, dialect: &semio_framework_artifact_reference::ArtifactDialect, _d: &SqliteDatabase, c: &mut SqliteSnapshotControl<'_>) -> store::io_schema::IoResult<()> {
        c.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 1).map_err(store::io_schema::IoError::from_value_error)?;
        if dialect.artifact_kind != "s.block.block2d" || dialect.standard != "1" || dialect.subset != "*" {
            return Err(store::io_schema::IoError::from_value_error(invalid("Block2d declared dialect differs")));
        }
        c.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 1, 1).map_err(store::io_schema::IoError::from_value_error)?;
        Ok(store::io_schema::IoOutcome::clean(()))
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
