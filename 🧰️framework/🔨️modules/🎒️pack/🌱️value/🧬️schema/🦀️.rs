//! 🏷️ Record wire vocabulary: the field and element tags, shape navigation and the canonical schema graph hash every encoder, decoder and cursor of a Pack record shares.

use semio_framework_value::{ValueError, ValueRefusalKind};
use semio_framework_pack_error::PackRefusal;
use semio_framework_dsl_record::{FieldSpec, RecordSpec, RecordSpecProducer, Shape};
use semio_framework_replication_codec::write_varint_u64;
use std::collections::HashMap;

#[path = "../🏭️schema/🦀️.rs"]
pub mod controlled_schema;

//#region 🔖️Tags
/// 🕳️ `FieldValue::Absent` — never written at record-field granularity (canonical mode
/// omits it entirely) but valid as a decode target, e.g. inside a sparse `TableSoA` fallback
/// column or a hand-crafted non-canonical file.
pub const TAG_ABSENT: u8 = 0x00;
pub const TAG_FALSE: u8 = 0x01;
pub const TAG_TRUE: u8 = 0x02;
pub const TAG_INT: u8 = 0x03;
pub const TAG_UINT: u8 = 0x04;
pub const TAG_F64: u8 = 0x05;
pub const TAG_STR: u8 = 0x06;
pub const TAG_STR_INLINE: u8 = 0x07;
pub const TAG_BYTES: u8 = 0x08;
pub const TAG_BYTES_CHUNKED: u8 = 0x09;
pub const TAG_ENUM: u8 = 0x0A;
pub const TAG_TUPLE: u8 = 0x0B;
pub const TAG_LIST: u8 = 0x0C;
pub const TAG_RECORD: u8 = 0x0D;
pub const TAG_BLOCK: u8 = 0x0E;
pub const TAG_STATEMENTS: u8 = 0x0F;
pub const TAG_MAP: u8 = 0x10;
pub const TAG_VALUE: u8 = 0x11;
pub const TAG_NULL: u8 = 0x12;
pub const TAG_WIRE: u8 = 0x13;
pub const TAG_TABLE_SOA: u8 = 0x14;
pub const TAG_PACKED_F64: u8 = 0x15;
pub const TAG_PACKED_VARINT: u8 = 0x16;
pub const TAG_EXPR: u8 = 0x17;
//#endregion 🔖️Tags

//#region 🔖️Shapes
/// ✂️ Extracts a `List`/`Tuple` field's element `Shape`, if `shape` is one of those two
/// variants — the seam shared by encode and decode so both walk exactly the same element type.
pub fn elem_shape_of(shape: Option<&Shape>) -> Option<&Shape> {
    match shape {
        Some(Shape::Tuple(elem, _)) | Some(Shape::List(elem)) => Some(elem),
        _ => None,
    }
}

/// 📊️ Extracts a `Table` field's lazy element-spec constructor, if `shape` is `Table`.
pub fn table_spec_of(shape: Option<&Shape>) -> Option<RecordSpecProducer> {
    match shape {
        Some(Shape::Table(spec_fn)) => Some(*spec_fn),
        _ => None,
    }
}

pub fn block_inner_shape(shape: Option<&Shape>) -> Option<&Shape> {
    match shape {
        Some(Shape::Block(inner)) => Some(inner),
        _ => None,
    }
}

pub fn statements_variants(shape: Option<&Shape>) -> Option<&Vec<(String, RecordSpecProducer)>> {
    match shape {
        Some(Shape::Statements(variants)) => Some(variants),
        _ => None,
    }
}

pub fn map_inner_shape(shape: Option<&Shape>) -> Option<&Shape> {
    match shape {
        Some(Shape::Map(inner)) => Some(inner),
        _ => None,
    }
}

/// 🔢️ Which packed form a homogeneous numeric sequence is eligible for.
pub enum NumKind {
    F64,
    Varint,
}

/// 🏷️ `TableSoA` per-column element-type tags — local to this crate's columnar encoding,
/// distinct from (and not overlapping the meaning of) the top-level wire tag space.
pub const ELEM_FALLBACK: u8 = 0;

pub const ELEM_BOOL: u8 = 1;

pub const ELEM_INT: u8 = 2;

pub const ELEM_UINT: u8 = 3;

pub const ELEM_F64: u8 = 4;

pub const ELEM_STR: u8 = 5;

pub const ELEM_ENUM: u8 = 6;

pub fn elem_tag_for_shape(shape: &Shape) -> u8 {
    match shape {
        Shape::Float | Shape::Quantity(_) | Shape::Angle(_) => ELEM_F64,
        Shape::Int => ELEM_INT,
        Shape::UInt | Shape::Count => ELEM_UINT,
        Shape::Enum(_) => ELEM_ENUM,
        Shape::Bool => ELEM_BOOL,
        Shape::Text | Shape::Ref(_) => ELEM_STR,
        _ => ELEM_FALLBACK,
    }
}
//#endregion 🔖️Shapes

//#region 🔖️SchemaHash
/// 🧭️ One field value shape of a [`PackSchemaGraph`]: `Shape` with every lazy nested record
/// resolved to the index of its canonical record, enum and statement tables sorted by their
/// schema-declared ordinal/keyword, and unit/ref/language refinements carried by name.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PackSchemaShape {
    Bool,
    Int,
    UInt,
    Float,
    Text,
    Bytes64,
    Enum(Vec<(u32, String)>),
    Tuple(Box<PackSchemaShape>, Option<u64>),
    List(Box<PackSchemaShape>),
    Record(u32),
    Block(Box<PackSchemaShape>),
    Statements(Vec<(String, u32)>),
    Map(Box<PackSchemaShape>),
    Value,
    Table(u32),
    Wire,
    Quantity(String),
    Angle(String),
    Ref(String),
    Coord(u8),
    Dir,
    Dim(u8),
    Range,
    Count,
    Expr,
    Embed(String),
    EmbedFrom(String),
}

/// 🪪️ One field of a [`PackSchemaGraph`] record: everything that decides what the pack bytes
/// of that field mean. Text-only presentation (`keyword`, `layout`, `position`, `call_name`,
/// `defines`) is not part of the pack identity.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PackSchemaField {
    pub id: u16,
    pub key: String,
    pub optional: bool,
    pub flatten: bool,
    pub shape: PackSchemaShape,
}

/// 🕸️ The canonical structural form of a `RecordSpec` tree: the minimal record graph of the
/// fully unfolded schema (bisimulation quotient, so recursion and duplicated `RecordSpecProducer`
/// items collapse to the same graph), numbered by breadth-first first visit from the root in
/// schema order (fields by id, statements by keyword). Record `0` is the root; nested records and
/// cycles are `Record(index)`/`Table(index)`/`Statements(.., index)` edges.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PackSchemaGraph {
    pub records: Vec<Vec<PackSchemaField>>,
}

fn pack_schema_shape(shape: &Shape, edges: &mut Vec<RecordSpecProducer>) -> PackSchemaShape {
    let mut edge = |spec_fn: RecordSpecProducer| {
        edges.push(spec_fn);
        (edges.len() - 1) as u32
    };
    match shape {
        Shape::Bool => PackSchemaShape::Bool,
        Shape::Int => PackSchemaShape::Int,
        Shape::UInt => PackSchemaShape::UInt,
        Shape::Float => PackSchemaShape::Float,
        Shape::Text => PackSchemaShape::Text,
        Shape::Bytes64 => PackSchemaShape::Bytes64,
        Shape::Enum(variants) => {
            let mut sorted: Vec<(u32, String)> = variants.iter().map(|(tag, ordinal)| (*ordinal, tag.clone())).collect();
            sorted.sort();
            PackSchemaShape::Enum(sorted)
        }
        Shape::Tuple(inner, len) => PackSchemaShape::Tuple(Box::new(pack_schema_shape(inner, edges)), len.map(|len| len as u64)),
        Shape::List(inner) => PackSchemaShape::List(Box::new(pack_schema_shape(inner, edges))),
        Shape::Record(spec_fn) => PackSchemaShape::Record(edge(*spec_fn)),
        Shape::Block(inner) => PackSchemaShape::Block(Box::new(pack_schema_shape(inner, edges))),
        Shape::Statements(variants) => {
            let mut sorted: Vec<&(String, RecordSpecProducer)> = variants.iter().collect();
            sorted.sort_by(|a, b| a.0.cmp(&b.0));
            PackSchemaShape::Statements(sorted.into_iter().map(|(keyword, spec_fn)| (keyword.clone(), edge(*spec_fn))).collect())
        }
        Shape::Map(inner) => PackSchemaShape::Map(Box::new(pack_schema_shape(inner, edges))),
        Shape::Value => PackSchemaShape::Value,
        Shape::Table(spec_fn) => PackSchemaShape::Table(edge(*spec_fn)),
        Shape::Wire => PackSchemaShape::Wire,
        Shape::Quantity(unit) => PackSchemaShape::Quantity(unit.symbol.to_string()),
        Shape::Angle(unit) => PackSchemaShape::Angle(unit.symbol.to_string()),
        Shape::Ref(kind) => PackSchemaShape::Ref(kind.to_string()),
        Shape::Coord(dims) => PackSchemaShape::Coord(*dims),
        Shape::Dir => PackSchemaShape::Dir,
        Shape::Dim(dims) => PackSchemaShape::Dim(*dims),
        Shape::Range => PackSchemaShape::Range,
        Shape::Count => PackSchemaShape::Count,
        Shape::Expr => PackSchemaShape::Expr,
        Shape::Embed(lang) => PackSchemaShape::Embed(lang.to_string()),
        Shape::EmbedFrom(key) => PackSchemaShape::EmbedFrom(key.to_string()),
    }
}

impl PackSchemaShape {
    fn map_edges(&mut self, map: &impl Fn(u32) -> u32) {
        match self {
            Self::Tuple(inner, _) | Self::List(inner) | Self::Block(inner) | Self::Map(inner) => inner.map_edges(map),
            Self::Record(index) | Self::Table(index) => *index = map(*index),
            Self::Statements(variants) => variants.iter_mut().for_each(|(_, index)| *index = map(*index)),
            _ => {}
        }
    }

    fn write_canonical(&self, out: &mut Vec<u8>) {
        let text = |out: &mut Vec<u8>, value: &str| {
            write_varint_u64(out, value.len() as u64);
            out.extend_from_slice(value.as_bytes());
        };
        match self {
            Self::Bool => out.push(1),
            Self::Int => out.push(2),
            Self::UInt => out.push(3),
            Self::Float => out.push(4),
            Self::Text => out.push(5),
            Self::Bytes64 => out.push(6),
            Self::Enum(variants) => {
                out.push(7);
                write_varint_u64(out, variants.len() as u64);
                for (ordinal, tag) in variants {
                    write_varint_u64(out, *ordinal as u64);
                    text(out, tag);
                }
            }
            Self::Tuple(inner, len) => {
                out.push(8);
                inner.write_canonical(out);
                match len {
                    Some(len) => {
                        out.push(1);
                        write_varint_u64(out, *len);
                    }
                    None => out.push(0),
                }
            }
            Self::List(inner) => {
                out.push(9);
                inner.write_canonical(out);
            }
            Self::Record(index) => {
                out.push(10);
                write_varint_u64(out, *index as u64);
            }
            Self::Block(inner) => {
                out.push(11);
                inner.write_canonical(out);
            }
            Self::Statements(variants) => {
                out.push(12);
                write_varint_u64(out, variants.len() as u64);
                for (keyword, index) in variants {
                    text(out, keyword);
                    write_varint_u64(out, *index as u64);
                }
            }
            Self::Map(inner) => {
                out.push(13);
                inner.write_canonical(out);
            }
            Self::Value => out.push(14),
            Self::Table(index) => {
                out.push(15);
                write_varint_u64(out, *index as u64);
            }
            Self::Wire => out.push(16),
            Self::Quantity(unit) => {
                out.push(17);
                text(out, unit);
            }
            Self::Angle(unit) => {
                out.push(18);
                text(out, unit);
            }
            Self::Ref(kind) => {
                out.push(19);
                text(out, kind);
            }
            Self::Coord(dims) => out.extend_from_slice(&[20, *dims]),
            Self::Dir => out.push(21),
            Self::Dim(dims) => out.extend_from_slice(&[22, *dims]),
            Self::Range => out.push(23),
            Self::Count => out.push(24),
            Self::Expr => out.push(25),
            Self::Embed(lang) => {
                out.push(26);
                text(out, lang);
            }
            Self::EmbedFrom(key) => {
                out.push(27);
                text(out, key);
            }
        }
    }
}

impl PackSchemaGraph {
    /// 🕸️ Builds the canonical graph of `spec`. Discovery follows each distinct
    /// `RecordSpecProducer` once (so it terminates on recursive schemas), then Moore partition
    /// refinement merges every pair of structurally identical records, which makes the result
    /// independent of how often the compiler duplicated a spec function.
    pub fn of(spec: &RecordSpec) -> Self {
        let mut nodes: Vec<(Vec<PackSchemaField>, Vec<usize>)> = Vec::new();
        let mut pending: Vec<RecordSpec> = Vec::new();
        let mut seen: HashMap<usize, usize> = HashMap::new();
        let mut cursor = 0usize;
        loop {
            let current = if cursor == 0 { spec } else if let Some(next) = pending.get(cursor - 1) { next } else { break };
            let mut ordered: Vec<&FieldSpec> = current.fields.iter().collect();
            ordered.sort_by_key(|field| field.id);
            let mut edges: Vec<RecordSpecProducer> = Vec::new();
            let fields = ordered.into_iter().map(|field| PackSchemaField { id: field.id, key: field.key.clone(), optional: field.optional, flatten: field.flatten, shape: pack_schema_shape(&field.shape, &mut edges) }).collect();
            let mut discovered = Vec::new();
            let targets = edges
                .into_iter()
                .map(|spec_fn| {
                    *seen.entry(spec_fn.ordinary as usize).or_insert_with(|| {
                        discovered.push(spec_fn);
                        pending.len() + discovered.len()
                    })
                })
                .collect();
            pending.extend(discovered.into_iter().map(|spec_fn| (spec_fn.ordinary)()));
            nodes.push((fields, targets));
            cursor += 1;
        }
        let mut labels: HashMap<&Vec<PackSchemaField>, u32> = HashMap::new();
        let mut class: Vec<u32> = nodes.iter().map(|(fields, _)| {
            let next = labels.len() as u32;
            *labels.entry(fields).or_insert(next)
        }).collect();
        let mut count = labels.len();
        loop {
            let mut signatures: HashMap<(u32, Vec<u32>), u32> = HashMap::new();
            let refined: Vec<u32> = nodes.iter().enumerate().map(|(node, (_, targets))| {
                let next = signatures.len() as u32;
                *signatures.entry((class[node], targets.iter().map(|target| class[*target]).collect())).or_insert(next)
            }).collect();
            let stable = signatures.len() == count;
            count = signatures.len();
            class = refined;
            if stable {
                break;
            }
        }
        let mut ordinal: Vec<Option<u32>> = vec![None; count];
        let mut order: Vec<usize> = Vec::new();
        let mut queue = std::collections::VecDeque::from([0usize]);
        ordinal[class[0] as usize] = Some(0);
        while let Some(node) = queue.pop_front() {
            order.push(node);
            for target in &nodes[node].1 {
                let slot = &mut ordinal[class[*target] as usize];
                if slot.is_none() {
                    *slot = Some(order.len() as u32 + queue.len() as u32);
                    queue.push_back(*target);
                }
            }
        }
        let records = order
            .into_iter()
            .map(|node| {
                let (fields, targets) = &nodes[node];
                let map = |edge: u32| ordinal[class[targets[edge as usize]] as usize].expect("reachable record has an ordinal");
                fields.iter().cloned().map(|mut field| {
                    field.shape.map_edges(&map);
                    field
                }).collect()
            })
            .collect();
        Self { records }
    }

    /// 🧮️ Hashes the owned graph incrementally without copying a canonical-byte carrier.
    pub fn hash_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(&self,control:&mut C)->Result<[u8;32],PackRefusal>{controlled_schema::hash_graph(self,control).map_err(PackRefusal::from)}

    /// 🗺️ The language-neutral JSON form of this graph (`🧪️tests/🧫️fixtures/🔑️schema-hash`): one array
    /// of field objects per record, each shape `{ "kind": .., payload }`, so any language can
    /// recompute [`Self::canonical_bytes`] and the hash from it.
    pub fn to_json(&self) -> semio_framework_pack_json::Value {
        use semio_framework_pack_json::{object, Value};
        fn shape(value: &PackSchemaShape) -> Value {
            let kind = |name: &str, payload: Vec<(&str, Value)>| object(std::iter::once(("kind".to_string(), Value::from(name))).chain(payload.into_iter().map(|(key, value)| (key.to_string(), value))));
            let edge = |index: &u32| Value::from(*index);
            match value {
                PackSchemaShape::Bool => kind("bool", vec![]),
                PackSchemaShape::Int => kind("int", vec![]),
                PackSchemaShape::UInt => kind("uint", vec![]),
                PackSchemaShape::Float => kind("float", vec![]),
                PackSchemaShape::Text => kind("text", vec![]),
                PackSchemaShape::Bytes64 => kind("bytes64", vec![]),
                PackSchemaShape::Enum(variants) => kind("enum", vec![("variants", Value::Array(variants.iter().map(|(ordinal, tag)| Value::Array(vec![Value::from(*ordinal), Value::from(tag.as_str())])).collect()))]),
                PackSchemaShape::Tuple(item, len) => kind("tuple", vec![("item", shape(item)), ("len", len.map_or(Value::Null, Value::from))]),
                PackSchemaShape::List(item) => kind("list", vec![("item", shape(item))]),
                PackSchemaShape::Record(index) => kind("record", vec![("record", edge(index))]),
                PackSchemaShape::Block(item) => kind("block", vec![("item", shape(item))]),
                PackSchemaShape::Statements(variants) => kind("statements", vec![("variants", Value::Array(variants.iter().map(|(keyword, index)| Value::Array(vec![Value::from(keyword.as_str()), edge(index)])).collect()))]),
                PackSchemaShape::Map(item) => kind("map", vec![("item", shape(item))]),
                PackSchemaShape::Value => kind("value", vec![]),
                PackSchemaShape::Table(index) => kind("table", vec![("record", edge(index))]),
                PackSchemaShape::Wire => kind("wire", vec![]),
                PackSchemaShape::Quantity(unit) => kind("quantity", vec![("unit", Value::from(unit.as_str()))]),
                PackSchemaShape::Angle(unit) => kind("angle", vec![("unit", Value::from(unit.as_str()))]),
                PackSchemaShape::Ref(entity) => kind("ref", vec![("entity", Value::from(entity.as_str()))]),
                PackSchemaShape::Coord(dims) => kind("coord", vec![("dims", Value::from(u64::from(*dims)))]),
                PackSchemaShape::Dir => kind("dir", vec![]),
                PackSchemaShape::Dim(dims) => kind("dim", vec![("dims", Value::from(u64::from(*dims)))]),
                PackSchemaShape::Range => kind("range", vec![]),
                PackSchemaShape::Count => kind("count", vec![]),
                PackSchemaShape::Expr => kind("expr", vec![]),
                PackSchemaShape::Embed(lang) => kind("embed", vec![("lang", Value::from(lang.as_str()))]),
                PackSchemaShape::EmbedFrom(key) => kind("embedFrom", vec![("key", Value::from(key.as_str()))]),
            }
        }
        Value::Array(
            self.records
                .iter()
                .map(|fields| {
                    Value::Array(
                        fields
                            .iter()
                            .map(|field| {
                                object([
                                    ("id".to_string(), Value::from(u64::from(field.id))),
                                    ("key".to_string(), Value::from(field.key.as_str())),
                                    ("optional".to_string(), Value::from(field.optional)),
                                    ("flatten".to_string(), Value::from(field.flatten)),
                                    ("shape".to_string(), shape(&field.shape)),
                                ])
                            })
                            .collect(),
                    )
                })
                .collect(),
        )
    }

    /// 🧾️ The canonical byte encoding hashed by [`schema_hash`]: `varint(records)`, then per
    /// record `varint(fields)`, then per field `varint(id) text(key) flags(optional=1|flatten=2)
    /// shape`, where `text = varint(len) utf8` and `shape = tag payload` (tags 1..=27 in
    /// `PackSchemaShape` declaration order; see `🧪️tests/🧫️fixtures/🔑️schema-hash`).
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        write_varint_u64(&mut out, self.records.len() as u64);
        for fields in &self.records {
            write_varint_u64(&mut out, fields.len() as u64);
            for field in fields {
                write_varint_u64(&mut out, field.id as u64);
                write_varint_u64(&mut out, field.key.len() as u64);
                out.extend_from_slice(field.key.as_bytes());
                out.push(u8::from(field.optional) | (u8::from(field.flatten) << 1));
                field.shape.write_canonical(&mut out);
            }
        }
        out
    }
}

/// 🔑️ `blake3` over [`PackSchemaGraph::canonical_bytes`] of `spec`: a nested record, enum
/// table, collection, option or composed child/link change flips the hash; declaration order,
/// text-only presentation and memory layout do not.
pub fn schema_hash(spec: &RecordSpec) -> [u8; 32] {
    *semio_framework_hash::hash(&PackSchemaGraph::of(spec).canonical_bytes()).as_bytes()
}
/// 🧮️ Preserves canonical graph identity under the declared metadata factories and one ownership ceiling.
pub fn schema_hash_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(spec:&RecordSpec,control:&mut C)->Result<[u8;32],PackRefusal>{controlled_schema::hash(spec,control).map_err(PackRefusal::from)}

//#endregion 🔖️SchemaHash
