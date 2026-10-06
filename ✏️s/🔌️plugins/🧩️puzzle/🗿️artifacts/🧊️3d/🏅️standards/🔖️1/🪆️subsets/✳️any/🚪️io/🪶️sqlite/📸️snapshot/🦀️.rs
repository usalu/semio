//! 🧩️ Complete Puzzle3d inline catalogs and literal relationships.
use crate::standards::v1::subsets::any::schema::snapshot::Puzzle3dSnapshot;
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
fn invalid(message: impl Into<String>) -> ValueError { ValueError::new(ValueRefusalKind::InvalidValue, message) }

const TABLES: [(&str, usize); 23] = [
    ("puzzle3_document", 3),
    ("puzzle3_meta", 2),
    ("puzzle3_catalog", 2),
    ("puzzle3_object_kind", 11),
    ("puzzle3_object_kind_base", 4),
    ("puzzle3_representation", 9),
    ("puzzle3_representation_tag", 4),
    ("puzzle3_vortex_template", 34),
    ("puzzle3_attribute", 7),
    ("puzzle3_author", 8),
    ("puzzle3_vortex_kind", 11),
    ("puzzle3_vortex_kind_compatible", 4),
    ("puzzle3_cable_kind", 7),
    ("puzzle3_attraction_kind", 6),
    ("puzzle3_compatibility", 8),
    ("puzzle3_object", 31),
    ("puzzle3_object_scale", 15),
    ("puzzle3_vortex", 29),
    ("puzzle3_attraction", 30),
    ("puzzle3_target", 27),
    ("puzzle3_target_scale", 15),
    ("puzzle3_reference", 18),
    ("puzzle3_reference_source", 4),
];
fn class(value: f64) -> &'static str {
    if value.is_nan() {
        "nan"
    } else if value == f64::INFINITY {
        "positiveInfinity"
    } else if value == f64::NEG_INFINITY {
        "negativeInfinity"
    } else {
        "finite"
    }
}
fn ordinal(value: usize) -> Result<Cell<'static>, ValueError> {
    Ok(Cell::Integer(i64::try_from(value).map_err(|e| invalid(e.to_string()))?))
}
fn optional_text(value: &Option<String>) -> Cell<'_> {
    value.as_deref().map(Cell::Text).unwrap_or(Cell::Null)
}
fn optional_bool(value: Option<bool>) -> Cell<'static> {
    value.map(|v| Cell::Integer(i64::from(v))).unwrap_or(Cell::Null)
}
fn optional_i32(value: Option<i32>) -> Cell<'static> {
    value.map(|v| Cell::Integer(i64::from(v))).unwrap_or(Cell::Null)
}
struct Cells<'a> {
    values: [Cell<'a>; 40],
    len: usize,
}
impl<'a> Cells<'a> {
    fn new(values: &[Cell<'a>]) -> Self {
        let mut row = Self { values: [Cell::Null; 40], len: 0 };
        for value in values {
            row.push(*value);
        }
        row
    }
    fn push(&mut self, value: Cell<'a>) {
        self.values[self.len] = value;
        self.len += 1;
    }
    fn f64(&mut self, value: f64) {
        self.push(if value.is_nan() { Cell::Null } else { Cell::Real(value) });
        self.push(Cell::Integer(value.to_bits() as i64));
        self.push(Cell::Text(class(value)));
    }
    fn optional(&mut self, value: Option<f64>) {
        if let Some(value) = value {
            self.f64(value);
        } else {
            for _ in 0..3 {
                self.push(Cell::Null);
            }
        }
    }
    fn xyz(&mut self, value: [f64; 3]) {
        for value in value {
            self.f64(value);
        }
    }
    fn direction(&mut self, value: Option<[f64; 3]>) {
        for i in 0..3 {
            self.optional(value.map(|v| v[i]));
        }
    }
    fn orientation(&mut self, value: Option<[f64; 4]>) {
        for i in 0..4 {
            self.optional(value.map(|v| v[i]));
        }
    }
    fn scale(&mut self, value: Puzzle3dScale) {
        match value {
            Puzzle3dScale::Uniform(v) => {
                self.push(Cell::Text("uniform"));
                self.f64(v);
                self.direction(None);
            }
            Puzzle3dScale::Vec3(v) => {
                self.push(Cell::Text("vec3"));
                self.optional(None);
                self.xyz(v);
            }
        }
    }
    fn insert(&self, out: &mut Projection<'_, '_>, table: usize) -> Result<i64, ValueError> {
        out.insert(TABLES[table].0, &self.values[..self.len])
    }
}
fn add(total: &mut usize, count: usize, c: &SqliteSnapshotControl<'_>) -> Result<(), ValueError> {
    *total = total.checked_add(count).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "Puzzle3d row count overflow"))?;
    c.check_rows(*total)
}
fn admit_schema(c: &SqliteSnapshotControl<'_>) -> Result<(), ValueError> {
    let bytes = TABLES.iter().try_fold(Puzzle3dSnapshot::SQLITE_SCHEMA.len(), |sum, (name, _)| sum.checked_add(name.len()).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "Puzzle3d schema count overflow")))?;
    if bytes > c.limits().max_schema_bytes {
        return Err(ValueError::new(ValueRefusalKind::OwnershipLimit, "Puzzle3d authored SQL exceeds schema byte limit"));
    }
    Ok(())
}
fn forecast(v: &Puzzle3dSnapshot, c: &mut SqliteSnapshotControl<'_>, phase: SqliteSnapshotPhase) -> Result<usize, ValueError> {
    admit_schema(c)?;
    let mut total = 2;
    c.check_rows(total)?;
    c.checkpoint(phase, 0, 0)?;
    for count in [v.meta.kind_compatibility.len(), v.attractions.len(), v.references.len().checked_mul(2).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "Puzzle3d reference count overflow"))?] {
        add(&mut total, count, c)?;
    }
    for (n, o) in v.objects.iter().enumerate() {
        add(&mut total, 1 + usize::from(o.scale.is_some()), c)?;
        add(&mut total, o.vortices.len(), c)?;
        if n % 256 == 0 {
            c.checkpoint(phase, n, v.objects.len())?;
        }
    }
    for (n, t) in v.target_volumes.iter().enumerate() {
        add(&mut total, 1 + usize::from(t.scale.is_some()), c)?;
        if n % 256 == 0 {
            c.checkpoint(phase, n, v.target_volumes.len())?;
        }
    }
    if let Some(e) = &v.meta.kind_catalogs {
        add(&mut total, 1, c)?;
        add(&mut total, e.cables.len(), c)?;
        add(&mut total, e.attractions.len(), c)?;
        for (n, k) in e.objects.iter().enumerate() {
            for count in [1, k.base_kinds.len(), k.vortices.len(), k.attributes.len(), k.authors.len()] {
                add(&mut total, count, c)?;
            }
            for (i, r) in k.representations.iter().enumerate() {
                add(&mut total, 1, c)?;
                add(&mut total, r.tags.len(), c)?;
                if i % 256 == 0 {
                    c.checkpoint(phase, i, k.representations.len())?;
                }
            }
            if n % 256 == 0 {
                c.checkpoint(phase, n, e.objects.len())?;
            }
        }
        for (n, k) in e.vortices.iter().enumerate() {
            add(&mut total, 1, c)?;
            add(&mut total, k.compatible_with.len(), c)?;
            if n % 256 == 0 {
                c.checkpoint(phase, n, e.vortices.len())?;
            }
        }
    }
    c.checkpoint(phase, total, total)?;
    Ok(total)
}
fn native_record(value: Option<&semio_framework_dsl_record::FieldValue>) -> Result<Option<&semio_framework_dsl_record::RecordValue>, ValueError> {
    match value {
        None | Some(semio_framework_dsl_record::FieldValue::Absent) => Ok(None),
        Some(semio_framework_dsl_record::FieldValue::Record(record)) => Ok(Some(record)),
        Some(semio_framework_dsl_record::FieldValue::Block(value)) => native_record(Some(value)),
        _ => Err(invalid("Puzzle3d native entity requires its literal record")),
    }
}
fn native_list(value: Option<&semio_framework_dsl_record::FieldValue>) -> Result<&[semio_framework_dsl_record::FieldValue], ValueError> {
    match value {
        None | Some(semio_framework_dsl_record::FieldValue::Absent) => Ok(&[]),
        Some(semio_framework_dsl_record::FieldValue::List(values)) => Ok(values),
        _ => Err(invalid("Puzzle3d native collection requires its literal list")),
    }
}
fn native_add(total: &mut usize, count: usize, maximum: usize) -> Result<(), ValueError> {
    *total = total.checked_add(count).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "Puzzle3d native entity count overflow"))?;
    if *total > maximum {
        return Err(ValueError::new(ValueRefusalKind::WorkLimit, "Puzzle3d native entity count exceeds row limit"));
    }
    Ok(())
}
fn admit_native_rows(record: &semio_framework_dsl_record::RecordValue, maximum: usize, native: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<(),semio_framework_value::ValueError> {
    native
        .scoped_stage(|native| -> Result<(), ValueError> {
            let mut count = 2;
            native_add(&mut count, 0, maximum)?;
            if let Some(meta) = native_record(record.get(2))? {
                native_add(&mut count, native_list(meta.get(1))?.len(), maximum)?;
                if let Some(catalog) = native_record(meta.get(0))? {
                    native_add(&mut count, 1, maximum)?;
                    native_add(&mut count, native_list(catalog.get(2))?.len(), maximum)?;
                    native_add(&mut count, native_list(catalog.get(3))?.len(), maximum)?;
                    let objects = native_list(catalog.get(0))?;
                    native.begin_stage(objects.len())?;
                    for object in objects {
                        let k = native_record(Some(object))?.ok_or_else(|| invalid("Puzzle3d catalog object missing"))?;
                        for number in [1, native_list(k.get(8))?.len(), native_list(k.get(10))?.len(), native_list(k.get(11))?.len(), native_list(k.get(12))?.len()] {
                            native_add(&mut count, number, maximum)?;
                        }
                        let representations = native_list(k.get(9))?;
                        native.scoped_stage(|native| -> Result<(), ValueError> {
                            native.begin_stage(representations.len())?;
                            for representation in representations {
                                let r = native_record(Some(representation))?.ok_or_else(|| invalid("Puzzle3d representation missing"))?;
                                native_add(&mut count, 1, maximum)?;
                                native_add(&mut count, native_list(r.get(4))?.len(), maximum)?;
                                native.step()?;
                            }
                            Ok(())
                        })?;
                        native.step()?;
                    }
                    let vortices = native_list(catalog.get(1))?;
                    native.begin_stage(vortices.len())?;
                    for vortex in vortices {
                        let k = native_record(Some(vortex))?.ok_or_else(|| invalid("Puzzle3d catalog vortex missing"))?;
                        native_add(&mut count, 1, maximum)?;
                        native_add(&mut count, native_list(k.get(4))?.len(), maximum)?;
                        native.step()?;
                    }
                }
            }
            let objects = native_list(record.get(3))?;
            native.begin_stage(objects.len())?;
            for object in objects {
                let o = native_record(Some(object))?.ok_or_else(|| invalid("Puzzle3d object missing"))?;
                native_add(&mut count, 1 + usize::from(o.get(6).is_some_and(|v| !matches!(v, semio_framework_dsl_record::FieldValue::Absent))), maximum)?;
                native_add(&mut count, native_list(o.get(8))?.len(), maximum)?;
                native.step()?;
            }
            native_add(&mut count, native_list(record.get(4))?.len(), maximum)?;
            let targets = native_list(record.get(5))?;
            native.begin_stage(targets.len())?;
            for target in targets {
                let t = native_record(Some(target))?.ok_or_else(|| invalid("Puzzle3d target missing"))?;
                native_add(&mut count, 1 + usize::from(t.get(3).is_some_and(|v| !matches!(v, semio_framework_dsl_record::FieldValue::Absent))), maximum)?;
                native.step()?;
            }
            native_add(&mut count, native_list(record.get(6))?.len().checked_mul(2).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "Puzzle3d native reference count overflow"))?, maximum)?;
            native.checkpoint()?;
            Ok(())
        })
        
}
struct Cursor<'a, 'c, 'p> {
    row: &'a SqliteRow,
    index: usize,
    control: &'c mut SqliteSnapshotControl<'p>,
}
impl<'a, 'c, 'p> Cursor<'a, 'c, 'p> {
    fn new(row: &'a SqliteRow, index: usize, control: &'c mut SqliteSnapshotControl<'p>) -> Self {
        Self { row, index, control }
    }
    fn next(&mut self) -> Result<&'a SqliteValue, ValueError> {
        let value = self.row.values.get(self.index).ok_or_else(|| invalid("Puzzle3d missing cell"))?;
        self.index += 1;
        Ok(value)
    }
    fn borrowed(&mut self) -> Result<&'a str, ValueError> {
        match self.next()? {
            SqliteValue::Text(v) => Ok(v),
            _ => Err(invalid("Puzzle3d TEXT required")),
        }
    }
    fn text(&mut self) -> Result<String, ValueError> {
        let value = self.borrowed()?;
        Reconstruction::new(self.control)?.text(value)
    }
    fn integer(&mut self) -> Result<i64, ValueError> {
        match self.next()? {
            SqliteValue::Integer(v) => Ok(*v),
            _ => Err(invalid("Puzzle3d INTEGER required")),
        }
    }
    fn optional_text(&mut self) -> Result<Option<String>, ValueError> {
        if self.row.values.get(self.index) == Some(&SqliteValue::Null) {
            self.next()?;
            Ok(None)
        } else {
            self.text().map(Some)
        }
    }
    fn boolean(&mut self) -> Result<bool, ValueError> {
        let value = match self.integer()? {
            0 => false,
            1 => true,
            _ => return Err(invalid("Puzzle3d boolean differs")),
        };
        Reconstruction::new(self.control)?.scalar()?;
        Ok(value)
    }
    fn optional_bool(&mut self) -> Result<Option<bool>, ValueError> {
        if self.row.values.get(self.index) == Some(&SqliteValue::Null) {
            self.next()?;
            Ok(None)
        } else {
            self.boolean().map(Some)
        }
    }
    fn optional_i32(&mut self) -> Result<Option<i32>, ValueError> {
        if self.row.values.get(self.index) == Some(&SqliteValue::Null) {
            self.next()?;
            Ok(None)
        } else {
            let value = i32::try_from(self.integer()?).map_err(|e| invalid(e.to_string()))?;
            Reconstruction::new(self.control)?.scalar()?;
            Ok(Some(value))
        }
    }
    fn absent(&mut self) -> Result<(), ValueError> {
        if self.next()? != &SqliteValue::Null {
            return Err(invalid("Puzzle3d partial optional presence"));
        }
        Ok(())
    }
    fn f64(&mut self) -> Result<f64, ValueError> {
        let query = self.next()?;
        let value = f64::from_bits(self.integer()? as u64);
        if self.borrowed()? != class(value) {
            return Err(invalid("Puzzle3d IEEE class differs"));
        }
        let exact = if value.is_nan() {
            query == &SqliteValue::Null
        } else {
            match query {
                SqliteValue::Real(v) => *v == value,
                SqliteValue::Integer(v) => value.is_finite() && value.fract() == 0.0 && value >= i64::MIN as f64 && value < -(i64::MIN as f64) && value as i64 == *v,
                _ => false,
            }
        };
        if !exact {
            return Err(invalid("Puzzle3d IEEE query differs"));
        }
        Reconstruction::new(self.control)?.scalar()?;
        Ok(value)
    }
    fn optional_float(&mut self) -> Result<Option<f64>, ValueError> {
        if self.row.values.get(self.index + 1) == Some(&SqliteValue::Null) {
            self.absent()?;
            self.absent()?;
            self.absent()?;
            Ok(None)
        } else {
            self.f64().map(Some)
        }
    }
    fn xyz(&mut self) -> Result<[f64; 3], ValueError> {
        Ok([self.f64()?, self.f64()?, self.f64()?])
    }
    fn direction(&mut self) -> Result<Option<[f64; 3]>, ValueError> {
        match (self.optional_float()?, self.optional_float()?, self.optional_float()?) {
            (None, None, None) => Ok(None),
            (Some(x), Some(y), Some(z)) => Ok(Some([x, y, z])),
            _ => Err(invalid("Puzzle3d partial vector presence")),
        }
    }
    fn orientation(&mut self) -> Result<Option<[f64; 4]>, ValueError> {
        match (self.optional_float()?, self.optional_float()?, self.optional_float()?, self.optional_float()?) {
            (None, None, None, None) => Ok(None),
            (Some(w), Some(x), Some(y), Some(z)) => Ok(Some([w, x, y, z])),
            _ => Err(invalid("Puzzle3d partial quaternion presence")),
        }
    }
    fn scale(&mut self) -> Result<Puzzle3dScale, ValueError> {
        let kind = self.borrowed()?;
        let uniform = self.optional_float()?;
        let axes = self.direction()?;
        match (kind, uniform, axes) {
            ("uniform", Some(v), None) => Ok(Puzzle3dScale::Uniform(v)),
            ("vec3", None, Some(v)) => Ok(Puzzle3dScale::Vec3(v)),
            _ => Err(invalid("Puzzle3d scale variant fields differ")),
        }
    }
    fn done(self) -> Result<(), ValueError> {
        if self.index != self.row.values.len() {
            return Err(invalid("Puzzle3d extra cell"));
        }
        Ok(())
    }
}
impl ArtifactSqliteSnapshot for Puzzle3dSnapshot {
    const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");
    fn decode_sqlite_snapshot_native(payload: &store::io_schema::IoPayload, c: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError> {
        admit_schema(c)?;
        c.check_rows(2)?;
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
        let mut out = Projection::new(Self::SQLITE_SCHEMA, c)?;
        let doc = out.insert(TABLES[0].0, &[Cell::Text(&self.schema), Cell::Text(&self.domain)])?;
        let meta = out.insert(TABLES[1].0, &[Cell::Integer(doc)])?;
        if let Some(catalog) = &self.meta.kind_catalogs {
            let owner = out.insert(TABLES[2].0, &[Cell::Integer(meta)])?;
            for (n, k) in catalog.objects.iter().enumerate() {
                let id = out.insert(
                    TABLES[3].0,
                    &[
                        Cell::Integer(owner),
                        ordinal(n)?,
                        Cell::Text(&k.id),
                        Cell::Text(&k.name),
                        Cell::Text(&k.label),
                        Cell::Text(&k.description),
                        Cell::Text(&k.icon),
                        Cell::Text(&k.image),
                        Cell::Text(&k.unit),
                        Cell::Integer(i64::from(k.is_abstract)),
                    ],
                )?;
                for (i, value) in k.base_kinds.iter().enumerate() {
                    out.insert(TABLES[4].0, &[Cell::Integer(id), ordinal(i)?, Cell::Text(value)])?;
                }
                for (i, r) in k.representations.iter().enumerate() {
                    let rid = out.insert(TABLES[5].0, &[Cell::Integer(id), ordinal(i)?, Cell::Text(&r.id), Cell::Text(&r.name), Cell::Text(&r.url), Cell::Text(&r.mime), optional_text(&r.lod), Cell::Text(&r.description)])?;
                    for (j, tag) in r.tags.iter().enumerate() {
                        out.insert(TABLES[6].0, &[Cell::Integer(rid), ordinal(j)?, Cell::Text(tag)])?;
                    }
                }
                for (i, t) in k.vortices.iter().enumerate() {
                    let mut row = Cells::new(&[Cell::Integer(id), ordinal(i)?, Cell::Text(&t.id), Cell::Text(&t.name), Cell::Text(&t.label), Cell::Text(&t.description), Cell::Text(&t.icon), optional_text(&t.vortex_kind)]);
                    row.xyz(t.point);
                    row.xyz(t.direction);
                    row.optional(t.t);
                    row.push(optional_bool(t.mandatory));
                    row.optional(t.radius);
                    row.insert(&mut out, 7)?;
                }
                for (i, a) in k.attributes.iter().enumerate() {
                    out.insert(TABLES[8].0, &[Cell::Integer(id), ordinal(i)?, Cell::Text(&a.id), Cell::Text(&a.key), Cell::Text(&a.value), optional_text(&a.definition)])?;
                }
                for (i, a) in k.authors.iter().enumerate() {
                    out.insert(TABLES[9].0, &[Cell::Integer(id), ordinal(i)?, Cell::Text(&a.id), Cell::Text(&a.name), Cell::Text(&a.email), optional_text(&a.role), optional_i32(a.rank)])?;
                }
            }
            for (n, k) in catalog.vortices.iter().enumerate() {
                let id = out.insert(
                    TABLES[10].0,
                    &[
                        Cell::Integer(owner),
                        ordinal(n)?,
                        Cell::Text(&k.id),
                        optional_text(&k.code),
                        optional_text(&k.label),
                        optional_i32(k.order),
                        Cell::Text(&k.description),
                        Cell::Text(&k.icon),
                        Cell::Text(&k.color),
                        Cell::Text(&k.default_cable_kind),
                    ],
                )?;
                for (i, value) in k.compatible_with.iter().enumerate() {
                    out.insert(TABLES[11].0, &[Cell::Integer(id), ordinal(i)?, Cell::Text(value)])?;
                }
            }
            for (n, k) in catalog.cables.iter().enumerate() {
                out.insert(TABLES[12].0, &[Cell::Integer(owner), ordinal(n)?, Cell::Text(&k.id), Cell::Text(&k.label), Cell::Text(&k.name), Cell::Text(&k.default_attraction_kind)])?;
            }
            for (n, k) in catalog.attractions.iter().enumerate() {
                out.insert(TABLES[13].0, &[Cell::Integer(owner), ordinal(n)?, Cell::Text(&k.id), Cell::Text(&k.label), Cell::Text(&k.name)])?;
            }
        }
        for (n, k) in self.meta.kind_compatibility.iter().enumerate() {
            let kind = match k.specificity {
                Puzzle3dCompatSpecificity::General => "general",
                Puzzle3dCompatSpecificity::Object => "object",
                Puzzle3dCompatSpecificity::Attraction => "attraction",
                Puzzle3dCompatSpecificity::Cable => "cable",
                Puzzle3dCompatSpecificity::Vortex => "vortex",
            };
            out.insert(TABLES[14].0, &[Cell::Integer(meta), ordinal(n)?, Cell::Text(&k.source), Cell::Text(&k.target), Cell::Integer(i64::from(k.bidirectional)), Cell::Integer(i64::from(k.important)), Cell::Text(kind)])?;
        }
        for (n, o) in self.objects.iter().enumerate() {
            let mut row = Cells::new(&[
                Cell::Integer(doc),
                ordinal(n)?,
                Cell::Text(&o.id),
                optional_text(&o.label),
                optional_text(&o.object_kind),
                Cell::Text(match o.anchor {
                    Puzzle3dObjectAnchor::Fixed => "fixed",
                    Puzzle3dObjectAnchor::Derived => "derived",
                }),
            ]);
            row.xyz(o.origin);
            row.orientation(o.orientation);
            row.push(optional_text(&o.mesh_url));
            row.push(Cell::Integer(i64::from(o.hidden)));
            row.push(Cell::Integer(i64::from(o.locked)));
            let id = row.insert(&mut out, 15)?;
            if let Some(scale) = o.scale {
                let mut row = Cells::new(&[Cell::Integer(id)]);
                row.scale(scale);
                row.insert(&mut out, 16)?;
            }
            for (i, v) in o.vortices.iter().enumerate() {
                let mut row = Cells::new(&[Cell::Integer(id), ordinal(i)?, Cell::Text(&v.id), optional_text(&v.vortex_kind), optional_text(&v.label)]);
                row.xyz(v.position);
                row.direction(v.direction);
                row.optional(v.radius);
                row.push(Cell::Integer(i64::from(v.hidden)));
                row.push(Cell::Integer(i64::from(v.locked)));
                row.insert(&mut out, 17)?;
            }
            if n % 256 == 0 {
                out.checkpoint_total(total)?;
            }
        }
        for (n, a) in self.attractions.iter().enumerate() {
            let mut row = Cells::new(&[Cell::Integer(doc), ordinal(n)?, Cell::Text(&a.id), Cell::Text(&a.attracting), Cell::Text(&a.attracted)]);
            for value in [a.gap, a.shift, a.rise, a.rotation, a.turn, a.tilt, a.x, a.y] {
                row.f64(value);
            }
            row.insert(&mut out, 18)?;
        }
        for (n, t) in self.target_volumes.iter().enumerate() {
            let mut row = Cells::new(&[Cell::Integer(doc), ordinal(n)?, Cell::Text(&t.id)]);
            row.xyz(t.origin);
            row.orientation(t.orientation);
            row.push(Cell::Integer(i64::from(t.hidden)));
            row.push(Cell::Integer(i64::from(t.locked)));
            let id = row.insert(&mut out, 19)?;
            if let Some(scale) = t.scale {
                let mut row = Cells::new(&[Cell::Integer(id)]);
                row.scale(scale);
                row.insert(&mut out, 20)?;
            }
        }
        for (n, r) in self.references.iter().enumerate() {
            let mut row = Cells::new(&[Cell::Integer(doc), ordinal(n)?, Cell::Text(&r.id)]);
            row.xyz(r.origin);
            row.f64(r.width_world);
            row.push(Cell::Integer(i64::from(r.locked)));
            row.push(Cell::Integer(i64::from(r.hidden)));
            let id = row.insert(&mut out, 21)?;
            out.insert(TABLES[22].0, &[Cell::Integer(id), Cell::Text(&r.source.url), optional_text(&r.source.media_kind)])?;
        }
        out.checkpoint_total(total)?;
        out.finish()
    }
    fn from_sqlite_database(d: &SqliteDatabase, c: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError> {
        reconstruct(d, c)
    }
    fn validate_sqlite_snapshot_subset(&self, dialect: &store::io_schema::ArtifactDialect, _d: &SqliteDatabase, c: &mut SqliteSnapshotControl<'_>) -> store::io_schema::IoResult<()> {
        c.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 1).map_err(store::io_schema::IoError::from_value_error)?;
        if dialect.artifact_kind != "s.puzzle.puzzle3d" || dialect.standard != "1" || dialect.subset != "*" {
            return Err(store::io_schema::IoError::from_value_error(invalid("Puzzle3d owned dialect differs")));
        }
        c.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 1, 1).map_err(store::io_schema::IoError::from_value_error)?;
        Ok(store::io_schema::IoOutcome::clean(()))
    }
}
struct Rows<'a> {
    groups: [BTreeMap<i64, BTreeMap<i64, &'a SqliteRow>>; 23],
}
impl<'a> Rows<'a> {
    fn new(d: &'a SqliteDatabase, c: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError> {
        store::sqlite_snapshot::validate_sqlite_database_schema(d, Puzzle3dSnapshot::SQLITE_SCHEMA, c.limits())?;
        c.check_database(d, SqliteSnapshotPhase::ReconstructSnapshot)?;
        let mut ids: [BTreeSet<i64>; 23] = std::array::from_fn(|_| BTreeSet::new());
        let mut groups = std::array::from_fn(|_| BTreeMap::<i64, BTreeMap<i64, &SqliteRow>>::new());
        for (table, (name, width)) in TABLES.iter().enumerate() {
            let rows = &d.table(name)?.rows;
            for (n, row) in rows.iter().enumerate() {
                if row.rowid <= 0 || row.integer(0)? != row.rowid || row.values.len() != *width || !ids[table].insert(row.rowid) {
                    return Err(invalid("Puzzle3d row alias/width/identity differs"));
                }
                if n % 256 == 0 {
                    c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, n, rows.len())?;
                }
            }
        }
        let parents: [usize; 23] = [0, 0, 1, 2, 3, 3, 5, 3, 3, 3, 2, 10, 2, 2, 1, 0, 15, 15, 0, 0, 19, 0, 21];
        for (table, (name, _)) in TABLES.iter().enumerate() {
            let rows = &d.table(name)?.rows;
            for (n, row) in rows.iter().enumerate() {
                let parent = if table == 0 { 0 } else { row.integer(1)? };
                if table != 0 && !ids[parents[table]].contains(&parent) {
                    return Err(invalid("Puzzle3d orphan owned relationship"));
                }
                let order = if matches!(table, 0 | 1 | 2 | 16 | 20 | 22) {
                    row.rowid
                } else {
                    let order = row.integer(2)?;
                    if order < 0 {
                        return Err(invalid("Puzzle3d negative ordinal"));
                    }
                    order
                };
                if groups[table].entry(parent).or_default().insert(order, row).is_some() {
                    return Err(invalid("Puzzle3d duplicate ownership ordinal"));
                }
                if n % 256 == 0 {
                    c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, n, rows.len())?;
                }
            }
        }
        Ok(Self { groups })
    }
    fn take(&mut self, table: usize, parent: i64, c: &mut SqliteSnapshotControl<'_>) -> Result<Vec<&'a SqliteRow>, ValueError> {
        let rows = self.groups[table].remove(&parent).unwrap_or_default();
        let total = rows.len();
        c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 0, total)?;
        let mut result = Vec::new();
        result.try_reserve_exact(total).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "Puzzle3d relationship allocation failed"))?;
        for (n, (order, row)) in rows.into_iter().enumerate() {
            if order != i64::try_from(n).map_err(|e| invalid(e.to_string()))? {
                return Err(invalid("Puzzle3d noncontiguous ordinal"));
            }
            result.push(row);
            if (n + 1) % 256 == 0 {
                c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, n + 1, total)?;
            }
        }
        c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, total, total)?;
        Ok(result)
    }
    fn one(&mut self, table: usize, parent: i64, required: bool) -> Result<Option<&'a SqliteRow>, ValueError> {
        let rows = self.groups[table].remove(&parent).unwrap_or_default();
        if rows.len() > 1 || required && rows.len() != 1 {
            return Err(invalid("Puzzle3d owned relationship cardinality differs"));
        }
        Ok(rows.into_values().next())
    }
    fn strings(&mut self, table: usize, parent: i64, c: &mut SqliteSnapshotControl<'_>) -> Result<Vec<String>, ValueError> {
        let mut result = Vec::new();
        for row in self.take(table, parent, c)? {
            let mut value = Cursor::new(row, 3, c);
            result.push(value.text()?);
            value.done()?;
        }
        Ok(result)
    }
    fn scale(&mut self, table: usize, parent: i64, c: &mut SqliteSnapshotControl<'_>) -> Result<Option<Puzzle3dScale>, ValueError> {
        if let Some(row) = self.one(table, parent, false)? {
            let mut value = Cursor::new(row, 2, c);
            let scale = value.scale()?;
            value.done()?;
            Ok(Some(scale))
        } else {
            Ok(None)
        }
    }
    fn finish(self, c: &mut SqliteSnapshotControl<'_>) -> Result<(), ValueError> {
        for groups in self.groups {
            if !groups.is_empty() {
                return Err(invalid("Puzzle3d unowned entity"));
            }
        }
        c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 1, 1)
    }
}
fn reconstruct(d: &SqliteDatabase, c: &mut SqliteSnapshotControl<'_>) -> Result<Puzzle3dSnapshot, ValueError> {
    let mut rows = Rows::new(d, c)?;
    let doc = rows.one(0, 0, true)?.ok_or_else(|| invalid("Puzzle3d document missing"))?;
    let mut root = Cursor::new(doc, 1, c);
    let schema = root.text()?;
    let domain = root.text()?;
    root.done()?;
    let meta = rows.one(1, doc.rowid, true)?.ok_or_else(|| invalid("Puzzle3d metadata missing"))?;
    let kind_catalogs = if let Some(catalog) = rows.one(2, meta.rowid, false)? {
        let (mut objects, mut vortices, mut cables, mut attractions) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
        for row in rows.take(3, catalog.rowid, c)? {
            let mut value = Cursor::new(row, 3, c);
            let id = value.text()?;
            let name = value.text()?;
            let label = value.text()?;
            let description = value.text()?;
            let icon = value.text()?;
            let image = value.text()?;
            let unit = value.text()?;
            let is_abstract = value.boolean()?;
            value.done()?;
            let base_kinds = rows.strings(4, row.rowid, c)?;
            let (mut representations, mut templates, mut attributes, mut authors) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
            for row in rows.take(5, row.rowid, c)? {
                let mut value = Cursor::new(row, 3, c);
                let id = value.text()?;
                let name = value.text()?;
                let url = value.text()?;
                let mime = value.text()?;
                let lod = value.optional_text()?;
                let description = value.text()?;
                value.done()?;
                let tags = rows.strings(6, row.rowid, c)?;
                representations.push(Puzzle3dRepresentation { id, name, url, mime, tags, lod, description });
            }
            for row in rows.take(7, row.rowid, c)? {
                let mut value = Cursor::new(row, 3, c);
                templates.push(Puzzle3dCatalogVortexTemplate {
                    id: value.text()?,
                    name: value.text()?,
                    label: value.text()?,
                    description: value.text()?,
                    icon: value.text()?,
                    vortex_kind: value.optional_text()?,
                    point: value.xyz()?,
                    direction: value.xyz()?,
                    t: value.optional_float()?,
                    mandatory: value.optional_bool()?,
                    radius: value.optional_float()?,
                });
                value.done()?;
            }
            for row in rows.take(8, row.rowid, c)? {
                let mut value = Cursor::new(row, 3, c);
                attributes.push(Puzzle3dAttribute { id: value.text()?, key: value.text()?, value: value.text()?, definition: value.optional_text()? });
                value.done()?;
            }
            for row in rows.take(9, row.rowid, c)? {
                let mut value = Cursor::new(row, 3, c);
                authors.push(Puzzle3dAuthor { id: value.text()?, name: value.text()?, email: value.text()?, role: value.optional_text()?, rank: value.optional_i32()? });
                value.done()?;
            }
            objects.push(Puzzle3dCatalogObjectKind { id, name, label, description, icon, image, unit, is_abstract, base_kinds, representations, vortices: templates, attributes, authors });
        }
        for row in rows.take(10, catalog.rowid, c)? {
            let mut value = Cursor::new(row, 3, c);
            let id = value.text()?;
            let code = value.optional_text()?;
            let label = value.optional_text()?;
            let order = value.optional_i32()?;
            let description = value.text()?;
            let icon = value.text()?;
            let color = value.text()?;
            let default_cable_kind = value.text()?;
            value.done()?;
            let compatible_with = rows.strings(11, row.rowid, c)?;
            vortices.push(Puzzle3dCatalogVortexKind { id, code, label, order, description, icon, color, default_cable_kind, compatible_with });
        }
        for row in rows.take(12, catalog.rowid, c)? {
            let mut value = Cursor::new(row, 3, c);
            cables.push(Puzzle3dCatalogCableKind { id: value.text()?, label: value.text()?, name: value.text()?, default_attraction_kind: value.text()? });
            value.done()?;
        }
        for row in rows.take(13, catalog.rowid, c)? {
            let mut value = Cursor::new(row, 3, c);
            attractions.push(Puzzle3dCatalogAttractionKind { id: value.text()?, label: value.text()?, name: value.text()? });
            value.done()?;
        }
        Some(Puzzle3dKindCatalogs { objects, vortices, cables, attractions })
    } else {
        None
    };
    let (mut kind_compatibility, mut objects, mut attractions, mut target_volumes, mut references) = (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new());
    for row in rows.take(14, meta.rowid, c)? {
        let mut value = Cursor::new(row, 3, c);
        let source = value.text()?;
        let target = value.text()?;
        let bidirectional = value.boolean()?;
        let important = value.boolean()?;
        let specificity = match value.borrowed()? {
            "general" => Puzzle3dCompatSpecificity::General,
            "object" => Puzzle3dCompatSpecificity::Object,
            "attraction" => Puzzle3dCompatSpecificity::Attraction,
            "cable" => Puzzle3dCompatSpecificity::Cable,
            "vortex" => Puzzle3dCompatSpecificity::Vortex,
            _ => return Err(invalid("Puzzle3d compatibility variant differs")),
        };
        value.done()?;
        kind_compatibility.push(Puzzle3dKindCompatibility { source, target, bidirectional, important, specificity });
    }
    for row in rows.take(15, doc.rowid, c)? {
        let mut value = Cursor::new(row, 3, c);
        let id = value.text()?;
        let label = value.optional_text()?;
        let object_kind = value.optional_text()?;
        let anchor = match value.borrowed()? {
            "fixed" => Puzzle3dObjectAnchor::Fixed,
            "derived" => Puzzle3dObjectAnchor::Derived,
            _ => return Err(invalid("Puzzle3d anchor variant differs")),
        };
        let origin = value.xyz()?;
        let orientation = value.orientation()?;
        let mesh_url = value.optional_text()?;
        let hidden = value.boolean()?;
        let locked = value.boolean()?;
        value.done()?;
        let scale = rows.scale(16, row.rowid, c)?;
        let mut vortices = Vec::new();
        for row in rows.take(17, row.rowid, c)? {
            let mut value = Cursor::new(row, 3, c);
            vortices.push(Puzzle3dVortex {
                id: value.text()?,
                vortex_kind: value.optional_text()?,
                label: value.optional_text()?,
                position: value.xyz()?,
                direction: value.direction()?,
                radius: value.optional_float()?,
                hidden: value.boolean()?,
                locked: value.boolean()?,
            });
            value.done()?;
        }
        objects.push(Puzzle3dObject { id, label, object_kind, anchor, origin, orientation, scale, mesh_url, vortices, hidden, locked });
    }
    for row in rows.take(18, doc.rowid, c)? {
        let mut value = Cursor::new(row, 3, c);
        attractions.push(Puzzle3dAttraction {
            id: value.text()?,
            attracting: value.text()?,
            attracted: value.text()?,
            gap: value.f64()?,
            shift: value.f64()?,
            rise: value.f64()?,
            rotation: value.f64()?,
            turn: value.f64()?,
            tilt: value.f64()?,
            x: value.f64()?,
            y: value.f64()?,
        });
        value.done()?;
    }
    for row in rows.take(19, doc.rowid, c)? {
        let mut value = Cursor::new(row, 3, c);
        let id = value.text()?;
        let origin = value.xyz()?;
        let orientation = value.orientation()?;
        let hidden = value.boolean()?;
        let locked = value.boolean()?;
        value.done()?;
        let scale = rows.scale(20, row.rowid, c)?;
        target_volumes.push(Puzzle3dTargetVolume { id, origin, orientation, scale, hidden, locked });
    }
    for row in rows.take(21, doc.rowid, c)? {
        let mut value = Cursor::new(row, 3, c);
        let id = value.text()?;
        let origin = value.xyz()?;
        let width_world = value.f64()?;
        let locked = value.boolean()?;
        let hidden = value.boolean()?;
        value.done()?;
        let mut source = Cursor::new(rows.one(22, row.rowid, true)?.ok_or_else(|| invalid("Puzzle3d reference source missing"))?, 2, c);
        let source_value = Puzzle3dReferenceSource { url: source.text()?, media_kind: source.optional_text()? };
        source.done()?;
        references.push(Puzzle3dReference { id, source: source_value, origin, width_world, locked, hidden });
    }
    rows.finish(c)?;
    Ok(Puzzle3dSnapshot { schema, domain, meta: Puzzle3dMeta { kind_catalogs, kind_compatibility }, objects, attractions, target_volumes, references })
}
impl ArtifactSqliteSnapshot for crate::Puzzle3dPlaySnapshot {
    const SQLITE_SCHEMA: &'static str = Puzzle3dSnapshot::SQLITE_SCHEMA;
    fn to_sqlite_database(&self, c: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, ValueError> {
        self.typed().to_sqlite_database(c)
    }
    fn from_sqlite_database(d: &SqliteDatabase, c: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError> {
        Puzzle3dSnapshot::from_sqlite_database(d, c).map(Self::from_typed)
    }
    fn decode_sqlite_snapshot_native(payload: &store::io_schema::IoPayload, c: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError> {
        Puzzle3dSnapshot::decode_sqlite_snapshot_native(payload, c).map(Self::from_typed)
    }
    fn encode_sqlite_snapshot_native(&self, encoding: SnapshotEncoding, c: &mut SqliteSnapshotControl<'_>) -> Result<store::io_schema::IoPayload, ValueError> {
        self.typed().encode_sqlite_snapshot_native(encoding, c)
    }
    fn validate_sqlite_snapshot_subset(&self, dialect: &store::io_schema::ArtifactDialect, d: &SqliteDatabase, c: &mut SqliteSnapshotControl<'_>) -> store::io_schema::IoResult<()> {
        self.typed().validate_sqlite_snapshot_subset(dialect, d, c)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

