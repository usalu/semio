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
use semio_framework_value::ValueError;
fn invalid(message:impl Into<String>)->ValueError{ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,message)}

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
    Ok(Cell::Integer(i64::try_from(n).map_err(|e|invalid(e.to_string()))?))
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
    fn xyz(&mut self, v: [f64; 3]) {
        for v in v {
            self.float(v);
        }
    }
    fn insert(&self, out: &mut Projection<'_, '_>, table: usize) -> Result<i64, ValueError> {
        out.insert(TABLES[table].0, &self.values[..self.len])
    }
}
fn add(total: &mut usize, count: usize, c: &SqliteSnapshotControl<'_>) -> Result<(), ValueError> {
    *total = total.checked_add(count).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"Block row count overflow"))?;
    c.check_rows(*total)
}
fn admit_schema(c: &SqliteSnapshotControl<'_>) -> Result<(), ValueError> {
    let size = TABLES.iter().try_fold(SQL.len(), |sum, (name, _)| sum.checked_add(name.len()).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"Block schema count overflow")))?;
    if size > c.limits().max_schema_bytes {
        return Err(invalid("Block authored schema exceeds byte limit"));
    }
    Ok(())
}
fn native_record(v: Option<&semio_framework_dsl_record::FieldValue>) -> Result<Option<&semio_framework_dsl_record::RecordValue>, ValueError> {
    match v {
        None | Some(semio_framework_dsl_record::FieldValue::Absent) => Ok(None),
        Some(semio_framework_dsl_record::FieldValue::Record(v)) => Ok(Some(v)),
        Some(semio_framework_dsl_record::FieldValue::Block(v)) => native_record(Some(v)),
        _ => Err(invalid("Block owned record required")),
    }
}
fn native_list(v: Option<&semio_framework_dsl_record::FieldValue>) -> Result<&[semio_framework_dsl_record::FieldValue], ValueError> {
    match v {
        None | Some(semio_framework_dsl_record::FieldValue::Absent) => Ok(&[]),
        Some(semio_framework_dsl_record::FieldValue::List(v)) => Ok(v),
        _ => Err(invalid("Block owned list required")),
    }
}
fn native_add(total: &mut usize, n: usize, maximum: usize) -> Result<(), ValueError> {
    *total = total.checked_add(n).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"Block native rows overflow"))?;
    if *total > maximum {
        return Err(invalid("Block native rows exceed limit"));
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
        let v = self.row.values.get(self.index).ok_or_else(||invalid("Block missing cell"))?;
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
    fn xyz(&mut self) -> Result<[f64; 3], ValueError> {
        Ok([self.float()?, self.float()?, self.float()?])
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
        group.into_values().next().ok_or_else(||invalid("Block singleton missing"))
    }
    fn take(&mut self, t: usize, parent: i64, c: &mut SqliteSnapshotControl<'_>) -> Result<Vec<&'a SqliteRow>, ValueError> {
        let group = self.groups.remove(&(t, parent)).unwrap_or_default();
        let total = group.len();
        c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 0, total)?;
        let mut out = Vec::new();
        out.try_reserve_exact(total).map_err(|_|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::AllocationFailed,"Block relationship allocation failed"))?;
        for (n, (order, row)) in group.into_iter().enumerate() {
            if order != i64::try_from(n).map_err(|e|invalid(e.to_string()))? {
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
const TABLES: [(&str, usize); 13] = [
    ("block3_document", 2),
    ("block3_kind", 9),
    ("block3_catalog_child", 7),
    ("block3_representation", 8),
    ("block3_representation_tag", 4),
    ("block3_representation_attribute", 6),
    ("block3_vortex_kind_extra", 8),
    ("block3_vortex", 27),
    ("block3_compatibility", 7),
    ("block3_attribute", 6),
    ("block3_author", 6),
    ("block3_camera3d", 23),
    ("block3_meta", 3),
];
const PARENTS: [usize; 13] = [0, 0, 0, 0, 3, 3, 0, 0, 0, 0, 0, 0, 0];
const SINGLETONS: [usize; 5] = [0, 1, 2, 11, 12];
fn forecast(v: &Block3dSnapshot, c: &mut SqliteSnapshotControl<'_>, phase: SqliteSnapshotPhase) -> Result<usize, ValueError> {
    admit_schema(c)?;
    let mut total = 5;
    c.check_rows(total)?;
    c.checkpoint(phase, 0, 0)?;
    for count in [v.representations.len(), v.vortex_kind_extra.len(), v.vortices.len(), v.compatibility.len(), v.attributes.len(), v.authors.len()] {
        add(&mut total, count, c)?;
    }
    for (n, r) in v.representations.iter().enumerate() {
        add(&mut total, r.tags.len(), c)?;
        add(&mut total, r.attributes.len(), c)?;
        if n % 256 == 0 {
            c.checkpoint(phase, n, v.representations.len())?;
        }
    }
    c.checkpoint(phase, total, total)?;
    Ok(total)
}
fn admit_native_rows(record: &semio_framework_dsl_record::RecordValue, maximum: usize, native: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<(),semio_framework_value::ValueError> {
    native
        .scoped_stage(|native| -> Result<(), ValueError> {
            let mut total = 5;
            native_add(&mut total, 0, maximum)?;
            native.begin_stage(6)?;
            for field in [2, 4, 5, 6, 7, 8] {
                native_add(&mut total, native_list(record.get(field))?.len(), maximum)?;
                native.step()?;
            }
            let reps = native_list(record.get(2))?;
            native.begin_stage(reps.len())?;
            for r in reps {
                let r = native_record(Some(r))?.ok_or_else(||invalid("Block representation missing"))?;
                native_add(&mut total, native_list(r.get(3))?.len(), maximum)?;
                native_add(&mut total, native_list(r.get(6))?.len(), maximum)?;
                native.step()?;
            }
            Ok(())
        })
        
}
impl ArtifactSqliteSnapshot for Block3dSnapshot {
    const SQLITE_SCHEMA: &'static str = SQL;
    fn decode_sqlite_snapshot_native(payload: &store::io_schema::IoPayload, c: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError> {
        admit_schema(c)?;
        c.check_rows(5)?;
        let maximum = c.limits().max_rows;
        store::decode_sqlite_snapshot_record_native(
            payload,
            <Self as store::ArtifactDsl>::envelope_id(),
            Self::__dsl_spec_producer(),
            |record, native| {
                admit_native_rows(record, maximum, native)?;
                Self::__dsl_from_record_controlled(record, native)
            },
            c,
        )
    }
    fn encode_sqlite_snapshot_native(&self, encoding: SnapshotEncoding, c: &mut SqliteSnapshotControl<'_>) -> Result<store::io_schema::IoPayload, ValueError> {
        forecast(self, c, SqliteSnapshotPhase::EncodeNative)?;
        store::encode_sqlite_snapshot_record_native(encoding, <Self as store::ArtifactDsl>::envelope_id(), Self::__dsl_spec_producer(), |native| self.__dsl_to_record_controlled(native), c)
    }
    fn to_sqlite_database(&self, c: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, ValueError> {
        let total = forecast(self, c, SqliteSnapshotPhase::ProjectSnapshot)?;
        let mut out = Projection::new(SQL, c)?;
        let doc = out.insert(TABLES[0].0, &[Cell::Text(&self.schema)])?;
        project_kind(&mut out, doc, &self.object_kind)?;
        let h = &self.catalog;
        let t = &h.target;
        out.insert(TABLES[2].0, &[Cell::Integer(doc), Cell::Text(&h.child_id), Cell::Text(&t.artifact_id), Cell::Text(&t.dialect.artifact_kind), Cell::Text(&t.dialect.standard), Cell::Text(&t.dialect.subset)])?;
        for (n, v) in self.representations.iter().enumerate() {
            let id = out.insert(TABLES[3].0, &[Cell::Integer(doc), ordinal(n)?, Cell::Text(&v.id), Cell::Text(&v.name), text(&v.mesh_url), text(&v.lod), Cell::Text(&v.description)])?;
            for (i, v) in v.tags.iter().enumerate() {
                out.insert(TABLES[4].0, &[Cell::Integer(id), ordinal(i)?, Cell::Text(v)])?;
            }
            for (i, v) in v.attributes.iter().enumerate() {
                out.insert(TABLES[5].0, &[Cell::Integer(id), ordinal(i)?, Cell::Text(&v.key), Cell::Text(&v.value), text(&v.definition)])?;
            }
        }
        for (n, v) in self.vortex_kind_extra.iter().enumerate() {
            out.insert(TABLES[6].0, &[Cell::Integer(doc), ordinal(n)?, Cell::Text(&v.id), Cell::Text(&v.name), Cell::Text(&v.label), Cell::Text(&v.color), Cell::Text(&v.default_cable_kind)])?;
        }
        for (n, v) in self.vortices.iter().enumerate() {
            let mut row = Cells::new(&[Cell::Integer(doc), ordinal(n)?, Cell::Text(&v.id), Cell::Text(&v.vortex_kind)]);
            row.xyz(v.position);
            row.xyz(v.direction);
            row.float(v.radius);
            row.push(text(&v.label));
            row.insert(&mut out, 7)?;
        }
        for (n, v) in self.compatibility.iter().enumerate() {
            out.insert(TABLES[8].0, &[Cell::Integer(doc), ordinal(n)?, Cell::Text(&v.id), Cell::Text(&v.source), Cell::Text(&v.target), Cell::Integer(i64::from(v.bidirectional))])?;
        }
        for (n, v) in self.attributes.iter().enumerate() {
            out.insert(TABLES[9].0, &[Cell::Integer(doc), ordinal(n)?, Cell::Text(&v.key), Cell::Text(&v.value), text(&v.definition)])?;
        }
        for (n, v) in self.authors.iter().enumerate() {
            out.insert(TABLES[10].0, &[Cell::Integer(doc), ordinal(n)?, Cell::Text(&v.id), Cell::Text(&v.name), text(&v.email)])?;
        }
        let v = &self.camera3d;
        let mut row = Cells::new(&[Cell::Integer(doc)]);
        row.xyz(v.position);
        row.xyz(v.target);
        row.float(v.zoom);
        row.insert(&mut out, 11)?;
        out.insert(TABLES[12].0, &[Cell::Integer(doc), Cell::Text(&self.meta.description)])?;
        out.checkpoint_total(total)?;
        out.finish()
    }
    fn from_sqlite_database(d: &SqliteDatabase, c: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError> {
        let mut rows = Rows::new(d, c)?;
        let doc = rows.one(0, 0)?;
        let mut v = Cursor::new(doc, 1, c);
        let schema = v.text()?;
        v.done()?;
        let object_kind = kind(rows.one(1, doc.rowid)?, c)?;
        let mut v = Cursor::new(rows.one(2, doc.rowid)?, 2, c);
        let child_id = v.text()?;
        let target = store::os_io::ArtifactRef { artifact_id: v.text()?, dialect: store::os_io::ArtifactDialect { artifact_kind: v.text()?, standard: v.text()?, subset: v.text()? } };
        v.done()?;
        let catalog = store::ArtifactChild::new(child_id, target);
        let (mut representations, mut vortex_kind_extra, mut vortices, mut rules, mut attributes, mut authors) = (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new());
        for row in rows.take(3, doc.rowid, c)? {
            let mut v = Cursor::new(row, 3, c);
            let id = v.text()?;
            let name = v.text()?;
            let mesh_url = v.optional_text()?;
            let lod = v.optional_text()?;
            let description = v.text()?;
            v.done()?;
            let (mut tags, mut attrs) = (Vec::new(), Vec::new());
            for tag in rows.take(4, row.rowid, c)? {
                let mut v = Cursor::new(tag, 3, c);
                tags.push(v.text()?);
                v.done()?;
            }
            for attr in rows.take(5, row.rowid, c)? {
                attrs.push(attribute(attr, c)?);
            }
            representations.push(BlockRepresentation { id, name, mesh_url, tags, lod, description, attributes: attrs });
        }
        for row in rows.take(6, doc.rowid, c)? {
            let mut v = Cursor::new(row, 3, c);
            vortex_kind_extra.push(Block3dVortexKindExtra { id: v.text()?, name: v.text()?, label: v.text()?, color: v.text()?, default_cable_kind: v.text()? });
            v.done()?;
        }
        for row in rows.take(7, doc.rowid, c)? {
            let mut v = Cursor::new(row, 3, c);
            vortices.push(Block3dVortexTemplate { id: v.text()?, vortex_kind: v.text()?, position: v.xyz()?, direction: v.xyz()?, radius: v.float()?, label: v.optional_text()? });
            v.done()?;
        }
        for row in rows.take(8, doc.rowid, c)? {
            rules.push(compatibility(row, c)?);
        }
        for row in rows.take(9, doc.rowid, c)? {
            attributes.push(attribute(row, c)?);
        }
        for row in rows.take(10, doc.rowid, c)? {
            authors.push(author(row, c)?);
        }
        let mut v = Cursor::new(rows.one(11, doc.rowid)?, 2, c);
        let camera3d = BlockCamera3d { position: v.xyz()?, target: v.xyz()?, zoom: v.float()? };
        v.done()?;
        let mut v = Cursor::new(rows.one(12, doc.rowid)?, 2, c);
        let meta = BlockMeta { description: v.text()? };
        v.done()?;
        rows.finish(c)?;
        Ok(Self { schema, object_kind, representations, catalog, vortex_kind_extra, vortices, compatibility: rules, attributes, authors, camera3d, meta })
    }
    fn validate_sqlite_snapshot_subset(&self, dialect: &store::io_schema::ArtifactDialect, _d: &SqliteDatabase, c: &mut SqliteSnapshotControl<'_>) -> store::io_schema::IoResult<()> {
        c.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 1).map_err(store::io_schema::IoError::from_value_error)?;
        if dialect.artifact_kind != "s.block.block3d" || dialect.standard != "1" || dialect.subset != "*" {
            return Err(store::io_schema::IoError::from_value_error(invalid("Block3d declared dialect differs")));
        }
        if self.catalog.target.dialect.artifact_kind != "s.stdio.semio" {
            return Err(store::io_schema::IoError::from_value_error(invalid("Block3d catalog declared child kind differs")));
        }
        c.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 1, 1).map_err(store::io_schema::IoError::from_value_error)?;
        Ok(store::io_schema::IoOutcome::clean(()))
    }
}
