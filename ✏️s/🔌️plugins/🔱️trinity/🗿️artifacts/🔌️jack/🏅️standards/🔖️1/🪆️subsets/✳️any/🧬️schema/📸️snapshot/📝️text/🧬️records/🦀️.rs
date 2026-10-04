//! 🔌️ Literal native Jack manifest records and iterative property type chains.
use crate::{Camera, EdgeKindDef, JackContentChild, JackSnapshot, Manifest, NodeKindDef, PortDirection, PortKindDef, PropertyDef, PropertyKind};
use semio_framework_value::{DecodedValue, NativeDecodeControl, NativeEncodeControl, ValueType, ValueError, ValueRefusalKind};
#[derive(semio_framework_dsl_record_derive::DslRecord)]
#[dsl(id = "trinity.jack", layout = "lines")]
pub(crate) struct JackPackRecord {
    schema: String,
    name: String,
    manifest_id: Option<String>,
    node_kinds: Vec<NodeKindRecord>,
    edge_kinds: Vec<EdgeKindRecord>,
    port_kinds: Vec<PortKindRecord>,
    camera: Camera,
    content: ContentRecord,
    root_node_id: Option<String>,
    query: String,
}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct NodeKindRecord {
    name: String,
    properties: Vec<PropertyRecord>,
    port_kinds: Vec<String>,
}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct EdgeKindRecord {
    name: String,
    properties: Vec<PropertyRecord>,
}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct PortKindRecord {
    name: String,
    direction: String,
    properties: Vec<PropertyRecord>,
}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct PropertyRecord {
    name: String,
    kind: String,
    type_path: Vec<String>,
    type_schema: Option<String>,
    expression: Option<String>,
}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct ContentRecord {
    child_id: String,
    artifact_id: String,
    artifact_kind: String,
    standard: String,
    subset: String,
}
trait Copies {
    fn text(&mut self, value: &str) -> Result<String, ValueError>;
    fn vector<T>(&mut self, count: usize) -> Result<Vec<T>, ValueError>;
    fn scalar(&mut self, bytes: usize) -> Result<(), ValueError>;
    fn step(&mut self) -> Result<(), ValueError>;
}
struct Ordinary;
impl Copies for Ordinary {
    fn text(&mut self, value: &str) -> Result<String, ValueError> {
        Ok(value.into())
    }
    fn vector<T>(&mut self, count: usize) -> Result<Vec<T>, ValueError> {
        Ok(Vec::with_capacity(count))
    }
    fn scalar(&mut self, _: usize) -> Result<(), ValueError> {
        Ok(())
    }
    fn step(&mut self) -> Result<(), ValueError> {
        Ok(())
    }
}
impl Copies for NativeDecodeControl<'_> {
    fn text(&mut self, value: &str) -> Result<String, ValueError> {
        self.copy_text(value)
    }
    fn vector<T>(&mut self, count: usize) -> Result<Vec<T>, ValueError> {
        self.allocate_vec(count)
    }
    fn scalar(&mut self, bytes: usize) -> Result<(), ValueError> {
        self.charge(bytes)
    }
    fn step(&mut self) -> Result<(), ValueError> {
        NativeDecodeControl::step(self)
    }
}
impl Copies for NativeEncodeControl<'_> {
    fn text(&mut self, value: &str) -> Result<String, ValueError> {
        self.copy_text(value)
    }
    fn vector<T>(&mut self, count: usize) -> Result<Vec<T>, ValueError> {
        self.allocate_vec(count)
    }
    fn scalar(&mut self, bytes: usize) -> Result<(), ValueError> {
        self.charge(bytes)
    }
    fn step(&mut self) -> Result<(), ValueError> {
        NativeEncodeControl::step(self)
    }
}
fn optional(value: &Option<String>, c: &mut impl Copies) -> Result<Option<String>, ValueError> {
    value.as_deref().map(|v| c.text(v)).transpose()
}
fn properties(values: &[PropertyDef], c: &mut impl Copies) -> Result<Vec<PropertyRecord>, ValueError> {
    let mut result = c.vector(values.len())?;
    for value in values {
        let mut leaf = &value.value_type;
        let mut count = 1usize;
        while let ValueType::List(inner) = leaf {
            count = count.checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "Jack type length overflow"))?;
            leaf = inner;
            c.step()?;
        }
        let mut path = c.vector(count)?;
        let mut ty = &value.value_type;
        loop {
            let kind = super::super::sqlite::variant(ty);
            path.push(c.text(kind)?);
            c.step()?;
            if let ValueType::List(inner) = ty {
                ty = inner
            } else {
                break;
            }
        }
        let type_schema = if let ValueType::Schema(schema) = ty { Some(c.text(schema)?) } else { None };
        result.push(PropertyRecord {
            name: c.text(&value.name)?,
            kind: c.text(match value.kind {
                PropertyKind::Data => "data",
                PropertyKind::Derived => "derived",
            })?,
            type_path: path,
            type_schema,
            expression: optional(&value.expr, c)?,
        });
        c.step()?;
    }
    Ok(result)
}
impl JackPackRecord {
    pub(crate) fn admit_rows(&self, maximum: usize, c: &mut NativeDecodeControl<'_>) -> Result<(), ValueError> {
        c.scoped_stage(|c| {
            c.begin_stage(0)?;
            let mut count = 3usize;
            let mut add = |amount: usize| {
                count = count.checked_add(amount).filter(|n| *n <= maximum).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "Jack native row limit"))?;
                c.step()
            };
            add(0)?;
            let property_rows = |property: &PropertyRecord| {
                1usize
                    .checked_add(property.type_path.len())
                    .and_then(|n| n.checked_add(property.type_path.len().saturating_sub(1)))
                    .and_then(|n| n.checked_add(usize::from(property.type_schema.is_some())))
                    .ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "Jack native row count overflow"))
            };
            for kind in &self.node_kinds {
                add(1usize.checked_add(kind.port_kinds.len()).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "Jack native row count overflow"))?)?;
                for property in &kind.properties {
                    add(property_rows(property)?)?;
                }
            }
            for kind in &self.edge_kinds {
                add(1)?;
                for property in &kind.properties {
                    add(property_rows(property)?)?;
                }
            }
            for kind in &self.port_kinds {
                add(1)?;
                for property in &kind.properties {
                    add(property_rows(property)?)?;
                }
            }
            c.checkpoint()
        })
    }

    fn copy(value: &JackSnapshot, c: &mut impl Copies) -> Result<Self, ValueError> {
        c.scalar(size_of::<Self>())?;
        let mut node_kinds = c.vector(value.manifest.node_kinds.len())?;
        for kind in &value.manifest.node_kinds {
            let mut port_kinds = c.vector(kind.port_kinds.len())?;
            for port in &kind.port_kinds {
                port_kinds.push(c.text(port)?);
                c.step()?
            }
            node_kinds.push(NodeKindRecord { name: c.text(&kind.name)?, properties: properties(&kind.properties, c)?, port_kinds });
            c.step()?;
        }
        let mut edge_kinds = c.vector(value.manifest.edge_kinds.len())?;
        for kind in &value.manifest.edge_kinds {
            edge_kinds.push(EdgeKindRecord { name: c.text(&kind.name)?, properties: properties(&kind.properties, c)? });
            c.step()?;
        }
        let mut port_kinds = c.vector(value.manifest.port_kinds.len())?;
        for kind in &value.manifest.port_kinds {
            port_kinds.push(PortKindRecord {
                name: c.text(&kind.name)?,
                direction: c.text(match kind.direction {
                    PortDirection::In => "in",
                    PortDirection::Out => "out",
                })?,
                properties: properties(&kind.properties, c)?,
            });
            c.step()?;
        }
        c.scalar(size_of::<Camera>())?;
        let child = &value.content;
        let target = &child.target;
        Ok(Self {
            schema: c.text(&value.schema)?,
            name: c.text(&value.name)?,
            manifest_id: optional(&value.manifest_id, c)?,
            node_kinds,
            edge_kinds,
            port_kinds,
            camera: Camera { x: value.camera.x, y: value.camera.y, zoom: value.camera.zoom },
            content: ContentRecord {
                child_id: c.text(&child.child_id)?,
                artifact_id: c.text(&target.artifact_id)?,
                artifact_kind: c.text(&target.dialect.artifact_kind)?,
                standard: c.text(&target.dialect.standard)?,
                subset: c.text(&target.dialect.subset)?,
            },
            root_node_id: optional(&value.root_node_id, c)?,
            query: c.text(&value.query)?,
        })
    }
    pub(crate) fn from_snapshot(value: &JackSnapshot) -> Self {
        Self::copy(value, &mut Ordinary).expect("owned native Jack records")
    }
    pub(crate) fn from_snapshot_controlled(value: &JackSnapshot, c: &mut NativeEncodeControl<'_>) -> Result<Self, ValueError> {
        c.scoped_stage(|c| {
            c.begin_stage(0)?;
            Self::copy(value, c)
        })
    }
    fn materialize(self, c: &mut impl Copies) -> Result<JackSnapshot, ValueError> {
        c.scalar(size_of::<JackSnapshot>())?;
        let mut nodes = DecodedValue::new(c.vector(self.node_kinds.len())?, super::super::sqlite::retire_nodes);
        for kind in self.node_kinds {
            let properties = materialize_properties(kind.properties, c)?;
            nodes.get_mut().push(NodeKindDef { name: kind.name, properties, port_kinds: kind.port_kinds });
            c.step()?;
        }
        let mut edges = DecodedValue::new(c.vector(self.edge_kinds.len())?, super::super::sqlite::retire_edges);
        for kind in self.edge_kinds {
            let properties = materialize_properties(kind.properties, c)?;
            edges.get_mut().push(EdgeKindDef { name: kind.name, properties });
            c.step()?;
        }
        let mut ports = DecodedValue::new(c.vector(self.port_kinds.len())?, super::super::sqlite::retire_ports);
        for kind in self.port_kinds {
            let direction = match kind.direction.as_str() {
                "in" => PortDirection::In,
                "out" => PortDirection::Out,
                _ => return Err(ValueError::new(ValueRefusalKind::InvalidValue, "Jack direction")),
            };
            let properties = materialize_properties(kind.properties, c)?;
            ports.get_mut().push(PortKindDef { name: kind.name, direction, properties });
            c.step()?;
        }
        let value = DecodedValue::new(
            JackSnapshot {
                schema: self.schema,
                name: self.name,
                manifest_id: self.manifest_id,
                manifest: Manifest { node_kinds: nodes.take(), edge_kinds: edges.take(), port_kinds: ports.take() },
                camera: self.camera,
                content: JackContentChild::new(
                    self.content.child_id,
                    store::io_schema::ArtifactRef { artifact_id: self.content.artifact_id, dialect: store::io_schema::ArtifactDialect { artifact_kind: self.content.artifact_kind, standard: self.content.standard, subset: self.content.subset } },
                ),
                root_node_id: self.root_node_id,
                query: self.query,
            },
            super::super::sqlite::retire_snapshot,
        );
        super::super::sqlite::validate(value.get())?;
        c.step()?;
        Ok(value.take())
    }
    pub(crate) fn into_snapshot(self) -> Result<JackSnapshot, ValueError> {
        self.materialize(&mut Ordinary)
    }
    pub(crate) fn into_snapshot_controlled(self, c: &mut NativeDecodeControl<'_>) -> Result<JackSnapshot, ValueError> {
        c.scoped_stage(|c| {
            c.begin_stage(0)?;
            self.materialize(c)
        })
    }
}
fn materialize_properties(values: Vec<PropertyRecord>, c: &mut impl Copies) -> Result<Vec<PropertyDef>, ValueError> {
    let mut result = DecodedValue::new(c.vector(values.len())?, super::super::sqlite::retire_properties);
    for value in values {
        let mut path = value.type_path;
        let terminal = path.pop().ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue, "Jack empty property type"))?;
        let mut type_schema = value.type_schema;
        let leaf = match terminal.as_str() {
            "boolean" => ValueType::Boolean,
            "integer" => ValueType::Integer,
            "decimal" => ValueType::Decimal,
            "text" => ValueType::Text,
            "schema" => ValueType::Schema(type_schema.take().ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue, "Jack missing type schema"))?),
            "any" => ValueType::Any,
            _ => return Err(ValueError::new(ValueRefusalKind::InvalidValue, "Jack type chain terminal")),
        };
        if terminal != "schema" && type_schema.is_some() {
            return Err(ValueError::new(ValueRefusalKind::InvalidValue, "Jack unexpected type schema"));
        }
        let mut ty = DecodedValue::new(leaf, super::super::sqlite::retire_type);
        for kind in path.into_iter().rev() {
            if kind != "list" {
                return Err(ValueError::new(ValueRefusalKind::InvalidValue, "Jack non-list type ancestor"));
            }
            c.scalar(size_of::<ValueType>())?;
            ty = DecodedValue::new(ValueType::List(Box::new(ty.take())), super::super::sqlite::retire_type);
            c.step()?;
        }
        let kind = match value.kind.as_str() {
            "data" => PropertyKind::Data,
            "derived" => PropertyKind::Derived,
            _ => return Err(ValueError::new(ValueRefusalKind::InvalidValue, "Jack property kind")),
        };
        result.get_mut().push(PropertyDef { name: value.name, kind, value_type: ty.take(), expr: value.expression });
        c.step()?;
    }
    Ok(result.take())
}
