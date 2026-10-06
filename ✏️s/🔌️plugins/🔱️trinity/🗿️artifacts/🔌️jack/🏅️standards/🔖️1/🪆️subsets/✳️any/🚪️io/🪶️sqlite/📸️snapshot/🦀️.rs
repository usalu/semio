//! 🔌️ Jack manifest entities, independently owned type chains and literal Graph child.
use crate::standards::v1::subsets::any::schema::snapshot::JackSnapshot;
use crate::{Camera, EdgeKindDef, JackContentChild, Manifest, NodeKindDef, PortDirection, PortKindDef, PropertyDef, PropertyKind};
use semio_framework_value::{ValueType, ValueError, ValueRefusalKind};
use std::collections::{BTreeMap, BTreeSet};
use store::sqlite_snapshot::{
    artifact::{insert_ieee754, read_binary64, Cell, FloatColumn, Projection, Reconstruction},
    SnapshotEncoding, SqliteDatabase, SqliteRow, SqliteSnapshotControl, SqliteSnapshotPhase,
};
const SQL: &str = include_str!("🗄️.sql");
const TABLES: [(&str, usize); 13] = [
    ("jack_document", 6),
    ("jack_camera", 11),
    ("jack_content_child", 7),
    ("jack_node_kind", 4),
    ("jack_edge_kind", 4),
    ("jack_port_kind", 5),
    ("jack_node_kind_port", 4),
    ("jack_value_type", 2),
    ("jack_value_type_list", 3),
    ("jack_value_type_schema", 3),
    ("jack_node_property", 7),
    ("jack_edge_property", 7),
    ("jack_port_property", 7),
];
const CAMERA: [FloatColumn; 3] = [FloatColumn::Binary64(2), FloatColumn::Binary64(3), FloatColumn::Binary64(4)];
fn optional(value: &Option<String>) -> Cell<'_> {
    value.as_deref().map(Cell::Text).unwrap_or(Cell::Null)
}
fn add(total: &mut usize, count: usize, c: &SqliteSnapshotControl<'_>) -> Result<(), ValueError> {
    *total = total.checked_add(count).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "Jack row count overflow"))?;
    c.check_rows(*total)
}
fn type_rows(mut value: &ValueType, total: &mut usize, c: &mut SqliteSnapshotControl<'_>, phase: SqliteSnapshotPhase) -> Result<(), ValueError> {
    loop {
        add(total, 1, c)?;
        if *total % 256 == 0 {
            c.checkpoint(phase, *total, 0)?;
        }
        match value {
            ValueType::List(inner) => {
                add(total, 1, c)?;
                value = inner
            }
            ValueType::Schema(_) => {
                add(total, 1, c)?;
                break;
            }
            _ => break,
        }
    }
    Ok(())
}
fn property_rows(values: &[PropertyDef], total: &mut usize, c: &mut SqliteSnapshotControl<'_>, phase: SqliteSnapshotPhase) -> Result<(), ValueError> {
    for value in values {
        add(total, 1, c)?;
        type_rows(&value.value_type, total, c, phase)?;
    }
    Ok(())
}
pub(super) fn forecast(value: &JackSnapshot, c: &mut SqliteSnapshotControl<'_>, phase: SqliteSnapshotPhase) -> Result<usize, ValueError> {
    c.checkpoint(phase, 0, 0)?;
    let limits = c.limits();
    if limits.max_tables < 13 || limits.max_columns < 11 {
        return Err(ValueError::new(ValueRefusalKind::WorkLimit, "Jack schema table or column limit"));
    }
    if SQL.len() + TABLES.iter().map(|(name, _)| name.len()).sum::<usize>() > limits.max_schema_bytes {
        return Err(ValueError::new(ValueRefusalKind::OwnershipLimit, "Jack schema byte limit"));
    }
    let mut total = 3;
    c.check_rows(total)?;
    for kind in &value.manifest.node_kinds {
        add(&mut total, 1, c)?;
        add(&mut total, kind.port_kinds.len(), c)?;
        property_rows(&kind.properties, &mut total, c, phase)?;
    }
    for kind in &value.manifest.edge_kinds {
        add(&mut total, 1, c)?;
        property_rows(&kind.properties, &mut total, c, phase)?;
    }
    for kind in &value.manifest.port_kinds {
        add(&mut total, 1, c)?;
        property_rows(&kind.properties, &mut total, c, phase)?;
    }
    c.checkpoint(phase, total, total)?;
    Ok(total)
}
pub(super) fn variant(value: &ValueType) -> &'static str {
    match value {
        ValueType::Boolean => "boolean",
        ValueType::Integer => "integer",
        ValueType::Decimal => "decimal",
        ValueType::Text => "text",
        ValueType::List(_) => "list",
        ValueType::Schema(_) => "schema",
        ValueType::Any => "any",
    }
}
fn project_type(mut value: &ValueType, out: &mut Projection<'_, '_>) -> Result<i64, ValueError> {
    let mut root = 0;
    let mut parent = None;
    loop {
        let id = out.insert("jack_value_type", &[Cell::Text(variant(value))])?;
        if root == 0 {
            root = id
        }
        if let Some(parent) = parent {
            out.insert("jack_value_type_list", &[Cell::Integer(parent), Cell::Integer(id)])?;
        }
        match value {
            ValueType::List(inner) => {
                parent = Some(id);
                value = inner
            }
            ValueType::Schema(schema) => {
                out.insert("jack_value_type_schema", &[Cell::Integer(id), Cell::Text(schema)])?;
                break;
            }
            _ => break,
        }
    }
    Ok(root)
}
fn project_properties(values: &[PropertyDef], table: &str, parent: i64, out: &mut Projection<'_, '_>) -> Result<(), ValueError> {
    for (ordinal, value) in values.iter().enumerate() {
        let ty = project_type(&value.value_type, out)?;
        out.insert(
            table,
            &[
                Cell::Integer(parent),
                Cell::Integer(i64::try_from(ordinal).map_err(|_| ValueError::new(ValueRefusalKind::WorkLimit, "Jack ordinal exceeds signed range"))?),
                Cell::Text(&value.name),
                Cell::Text(match value.kind {
                    PropertyKind::Data => "data",
                    PropertyKind::Derived => "derived",
                }),
                optional(&value.expr),
                Cell::Integer(ty),
            ],
        )?;
    }
    Ok(())
}
pub(super) fn project(value: &JackSnapshot, c: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, ValueError> {
    let total = forecast(value, c, SqliteSnapshotPhase::ProjectSnapshot)?;
    let mut out = Projection::new(SQL, c)?;
    let doc = out.insert("jack_document", &[Cell::Text(&value.schema), Cell::Text(&value.name), optional(&value.manifest_id), optional(&value.root_node_id), Cell::Text(&value.query)])?;
    insert_ieee754(&mut out, "jack_camera", &[Cell::Integer(doc), Cell::Real(value.camera.x), Cell::Real(value.camera.y), Cell::Real(value.camera.zoom)], &CAMERA)?;
    let child = &value.content;
    let target = &child.target;
    out.insert("jack_content_child", &[Cell::Integer(doc), Cell::Text(&child.child_id), Cell::Text(&target.artifact_id), Cell::Text(&target.dialect.artifact_kind), Cell::Text(&target.dialect.standard), Cell::Text(&target.dialect.subset)])?;
    for (ordinal, kind) in value.manifest.node_kinds.iter().enumerate() {
        let id = out.insert("jack_node_kind", &[Cell::Integer(doc), Cell::Integer(i64::try_from(ordinal).map_err(|_| ValueError::new(ValueRefusalKind::WorkLimit, "Jack ordinal exceeds signed range"))?), Cell::Text(&kind.name)])?;
        for (n, port) in kind.port_kinds.iter().enumerate() {
            out.insert("jack_node_kind_port", &[Cell::Integer(id), Cell::Integer(i64::try_from(n).map_err(|_| ValueError::new(ValueRefusalKind::WorkLimit, "Jack ordinal exceeds signed range"))?), Cell::Text(port)])?;
        }
        project_properties(&kind.properties, "jack_node_property", id, &mut out)?;
    }
    for (ordinal, kind) in value.manifest.edge_kinds.iter().enumerate() {
        let id = out.insert("jack_edge_kind", &[Cell::Integer(doc), Cell::Integer(i64::try_from(ordinal).map_err(|_| ValueError::new(ValueRefusalKind::WorkLimit, "Jack ordinal exceeds signed range"))?), Cell::Text(&kind.name)])?;
        project_properties(&kind.properties, "jack_edge_property", id, &mut out)?;
    }
    for (ordinal, kind) in value.manifest.port_kinds.iter().enumerate() {
        let id = out.insert(
            "jack_port_kind",
            &[
                Cell::Integer(doc),
                Cell::Integer(i64::try_from(ordinal).map_err(|_| ValueError::new(ValueRefusalKind::WorkLimit, "Jack ordinal exceeds signed range"))?),
                Cell::Text(&kind.name),
                Cell::Text(match kind.direction {
                    PortDirection::In => "in",
                    PortDirection::Out => "out",
                }),
            ],
        )?;
        project_properties(&kind.properties, "jack_port_property", id, &mut out)?;
    }
    out.checkpoint_total(total)?;
    out.finish()
}
pub(super) fn retire_type(value: ValueType) {
    <ValueType as semio_framework_value::FromValue>::retire_decoded(value)
}

pub(super) fn retire_properties(values: Vec<PropertyDef>) {
    for value in values {
        retire_type(value.value_type)
    }
}
pub(super) fn retire_nodes(values: Vec<NodeKindDef>) {
    for value in values {
        retire_properties(value.properties)
    }
}
pub(super) fn retire_edges(values: Vec<EdgeKindDef>) {
    for value in values {
        retire_properties(value.properties)
    }
}
pub(super) fn retire_ports(values: Vec<PortKindDef>) {
    for value in values {
        retire_properties(value.properties)
    }
}
pub(super) fn retire_manifest(value: Manifest) {
    retire_nodes(value.node_kinds);
    retire_edges(value.edge_kinds);
    retire_ports(value.port_kinds)
}
struct Rows<'a> {
    database: &'a SqliteDatabase,
    used: BTreeSet<(usize, i64)>,
    work: usize,
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
        Ok(Self { database, used: BTreeSet::new(), work: 0 })
    }
    fn row(&mut self, index: usize, id: i64, c: &mut SqliteSnapshotControl<'_>) -> Result<&'a SqliteRow, ValueError> {
        let mut found = None;
        for row in &self.database.table(TABLES[index].0)?.rows {
            self.work = self.work.checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "Jack scan work overflow"))?;
            if self.work % 256 == 0 {
                c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, self.work, 0)?;
            }
            if row.rowid == id {
                found = Some(row);
                break;
            }
        }
        let row = found.ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue, "Jack dangling relationship"))?;
        if !self.used.insert((index, id)) {
            return Err(ValueError::new(ValueRefusalKind::InvalidValue, "Jack entity has multiple owners or a cycle"));
        }
        Ok(row)
    }
    fn singleton(&mut self, index: usize, c: &mut SqliteSnapshotControl<'_>) -> Result<&'a SqliteRow, ValueError> {
        let row = self.database.table(TABLES[index].0)?.single_row()?;
        self.row(index, row.rowid, c)
    }
    fn children(&mut self, index: usize, parent: i64, ordered: bool, c: &mut SqliteSnapshotControl<'_>) -> Result<Vec<&'a SqliteRow>, ValueError> {
        let mut children = BTreeMap::new();
        for row in &self.database.table(TABLES[index].0)?.rows {
            self.work = self.work.checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "Jack scan work overflow"))?;
            if self.work % 256 == 0 {
                c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, self.work, 0)?;
            }
            if row.integer(1)? == parent {
                let ordinal = if ordered { row.integer(2)? } else { 0 };
                if ordinal < 0 || children.insert(ordinal, row).is_some() {
                    return Err(ValueError::new(ValueRefusalKind::InvalidValue, "Jack relationship cardinality or ordinal"));
                }
            }
        }
        let mut result = Vec::with_capacity(children.len());
        for (ordinal, row) in children {
            if ordered && ordinal != result.len() as i64 {
                return Err(ValueError::new(ValueRefusalKind::InvalidValue, "Jack ordinals are not contiguous"));
            }
            self.row(index, row.rowid, c)?;
            result.push(row)
        }
        Ok(result)
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
fn reconstruct_type(root: i64, rows: &mut Rows<'_>, c: &mut SqliteSnapshotControl<'_>) -> Result<ValueType, ValueError> {
    let mut id = root;
    let mut depth = 0usize;
    let value = loop {
        Reconstruction::new(c)?.scalar()?;
        let row = rows.row(7, id, c)?;
        let lists = rows.children(8, id, false, c)?;
        let schemas = rows.children(9, id, false, c)?;
        let value = match row.text(1)? {
            "list" => {
                if lists.len() != 1 || !schemas.is_empty() {
                    return Err(ValueError::new(ValueRefusalKind::InvalidValue, "Jack list type requires one child"));
                }
                depth = depth.checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "Jack type depth overflow"))?;
                id = lists[0].integer(2)?;
                continue;
            }
            "schema" => {
                if schemas.len() != 1 || !lists.is_empty() {
                    return Err(ValueError::new(ValueRefusalKind::InvalidValue, "Jack schema type requires text"));
                }
                ValueType::Schema(text(schemas[0], 2, c)?)
            }
            "boolean" => ValueType::Boolean,
            "integer" => ValueType::Integer,
            "decimal" => ValueType::Decimal,
            "text" => ValueType::Text,
            "any" => ValueType::Any,
            _ => return Err(ValueError::new(ValueRefusalKind::InvalidValue, "Jack unknown value type")),
        };
        if !matches!(value, ValueType::Schema(_)) && (!lists.is_empty() || !schemas.is_empty()) {
            return Err(ValueError::new(ValueRefusalKind::InvalidValue, "Jack scalar type has child entities"));
        }
        break value;
    };
    let mut value = semio_framework_value::DecodedValue::new(value, retire_type);
    for _ in 0..depth {
        Reconstruction::new(c)?.scalar()?;
        value = semio_framework_value::DecodedValue::new(ValueType::List(Box::new(value.take())), retire_type);
    }
    Ok(value.take())
}
fn reconstruct_properties(index: usize, parent: i64, rows: &mut Rows<'_>, c: &mut SqliteSnapshotControl<'_>) -> Result<Vec<PropertyDef>, ValueError> {
    let mut result = semio_framework_value::DecodedValue::new(Vec::new(), retire_properties);
    for row in rows.children(index, parent, true, c)? {
        let kind = match row.text(4)? {
            "data" => PropertyKind::Data,
            "derived" => PropertyKind::Derived,
            _ => return Err(ValueError::new(ValueRefusalKind::InvalidValue, "Jack property kind")),
        };
        let name = text(row, 3, c)?;
        let expr = optional_text(row, 5, c)?;
        let value_type = reconstruct_type(row.integer(6)?, rows, c)?;
        result.get_mut().push(PropertyDef { name, kind, value_type, expr });
    }
    Ok(result.take())
}
pub(super) fn reconstruct(database: &SqliteDatabase, c: &mut SqliteSnapshotControl<'_>) -> Result<JackSnapshot, ValueError> {
    let mut rows = Rows::new(database, c)?;
    let document = rows.singleton(0, c)?;
    let schema = text(document, 1, c)?;
    let name = text(document, 2, c)?;
    let manifest_id = optional_text(document, 3, c)?;
    let root_node_id = optional_text(document, 4, c)?;
    let query = text(document, 5, c)?;
    let camera = rows.singleton(1, c)?;
    let child = rows.singleton(2, c)?;
    if camera.integer(1)? != document.rowid || child.integer(1)? != document.rowid {
        return Err(ValueError::new(ValueRefusalKind::InvalidValue, "Jack parent singleton relationship"));
    }
    for _ in 0..3 {
        Reconstruction::new(c)?.scalar()?;
    }
    let camera = Camera { x: read_binary64(camera, 2, &CAMERA)?, y: read_binary64(camera, 3, &CAMERA)?, zoom: read_binary64(camera, 4, &CAMERA)? };
    let mut nodes = semio_framework_value::DecodedValue::new(Vec::new(), retire_nodes);
    for row in rows.children(3, document.rowid, true, c)? {
        let name = text(row, 3, c)?;
        let mut port_kinds = Vec::new();
        for port in rows.children(6, row.rowid, true, c)? {
            port_kinds.push(text(port, 3, c)?)
        }
        let properties = reconstruct_properties(10, row.rowid, &mut rows, c)?;
        nodes.get_mut().push(NodeKindDef { name, properties, port_kinds });
    }
    let mut edges = semio_framework_value::DecodedValue::new(Vec::new(), retire_edges);
    for row in rows.children(4, document.rowid, true, c)? {
        let name = text(row, 3, c)?;
        let properties = reconstruct_properties(11, row.rowid, &mut rows, c)?;
        edges.get_mut().push(EdgeKindDef { name, properties });
    }
    let mut ports = semio_framework_value::DecodedValue::new(Vec::new(), retire_ports);
    for row in rows.children(5, document.rowid, true, c)? {
        let name = text(row, 3, c)?;
        let direction = match row.text(4)? {
            "in" => PortDirection::In,
            "out" => PortDirection::Out,
            _ => return Err(ValueError::new(ValueRefusalKind::InvalidValue, "Jack port direction")),
        };
        let properties = reconstruct_properties(12, row.rowid, &mut rows, c)?;
        ports.get_mut().push(PortKindDef { name, direction, properties });
    }
    let manifest = semio_framework_value::DecodedValue::new(Manifest { node_kinds: nodes.take(), edge_kinds: edges.take(), port_kinds: ports.take() }, retire_manifest);
    let content = JackContentChild::new(
        text(child, 2, c)?,
        store::io_schema::ArtifactRef { artifact_id: text(child, 3, c)?, dialect: store::io_schema::ArtifactDialect { artifact_kind: text(child, 4, c)?, standard: text(child, 5, c)?, subset: text(child, 6, c)? } },
    );
    let value = semio_framework_value::DecodedValue::new(JackSnapshot { schema, name, manifest_id, manifest: manifest.take(), camera, content, root_node_id, query }, retire_snapshot);
    rows.finish()?;
    validate(value.get())?;
    Reconstruction::new(c)?.checkpoint()?;
    Ok(value.take())
}

pub(super) fn validate(value: &JackSnapshot) -> Result<(), ValueError> {
    value.validate_schema().map_err(|error| ValueError::new(ValueRefusalKind::InvalidValue, error.to_string()))?;
    let dialect = &value.content.target.dialect;
    if dialect.artifact_kind != "s.stdio.semio" || dialect.standard != "v1" || dialect.subset != "graph" {
        return Err(ValueError::new(ValueRefusalKind::InvalidValue, "Jack Graph child dialect mismatch"));
    }
    Ok(())
}
pub(super) fn retire_snapshot(value: JackSnapshot) {
    let mut cursor = store::ArtifactOwnedValueRetirementFactory::retire_owned(&crate::host::JackSnapshotRetirementFactory, value);
    loop {
        match cursor.close_step(256, usize::MAX).expect("Jack explicit retirement") {
            store::SnapshotRetirementStep::Complete => break,
            store::SnapshotRetirementStep::Pending { .. } => {}
            store::SnapshotRetirementStep::Blocked => panic!("Jack retirement blocked"),
        }
    }
    assert!(cursor.terminal_is_empty());
}
impl store::ArtifactSqliteSnapshot for JackSnapshot {
    const SQLITE_SCHEMA: &'static str = SQL;
    fn to_sqlite_database(&self, c: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, ValueError> {
        validate(self)?;
        project(self, c)
    }
    fn from_sqlite_database(database: &SqliteDatabase, c: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError> {
        reconstruct(database, c)
    }
    fn decode_sqlite_snapshot_native(payload: &store::io_schema::IoPayload, c: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError> {
        c.checkpoint(SqliteSnapshotPhase::DecodeNative, 0, 0)?;
        let max_rows = c.limits().max_rows;
        let value = store::decode_sqlite_snapshot_record_native(
            payload,
            <Self as store::ArtifactDsl>::envelope_id(),
            crate::standards::v1::subsets::any::schema::snapshot::text::JackPackRecord::__dsl_spec_producer(),
            |record, native| {
                let flat = crate::standards::v1::subsets::any::schema::snapshot::text::JackPackRecord::__dsl_from_record_controlled(record, native)?;
                flat.admit_rows(max_rows, native)?;
                flat.into_snapshot_controlled(native)
            },
            c,
        )?;
        let value = semio_framework_value::DecodedValue::new(value, retire_snapshot);
        forecast(value.get(), c, SqliteSnapshotPhase::DecodeNative)?;
        Ok(value.take())
    }
    fn encode_sqlite_snapshot_native(&self, encoding: SnapshotEncoding, c: &mut SqliteSnapshotControl<'_>) -> Result<store::io_schema::IoPayload, ValueError> {
        validate(self)?;
        forecast(self, c, SqliteSnapshotPhase::EncodeNative)?;
        store::encode_sqlite_snapshot_record_native(
            encoding,
            <Self as store::ArtifactDsl>::envelope_id(),
            crate::standards::v1::subsets::any::schema::snapshot::text::JackPackRecord::__dsl_spec_producer(),
            |native| {
                let flat = crate::standards::v1::subsets::any::schema::snapshot::text::JackPackRecord::from_snapshot_controlled(self, native)?;
                flat.__dsl_to_record_controlled(native)
            },
            c,
        )
    }
    fn retire_sqlite_snapshot(self) {
        retire_snapshot(self)
    }
    fn validate_sqlite_snapshot_subset(&self, dialect: &store::io_schema::ArtifactDialect, _: &SqliteDatabase, c: &mut SqliteSnapshotControl<'_>) -> store::io_schema::IoResult<()> {
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

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

