//! 🔑️ Structural pack-schema identity, run natively and on `wasm32-wasip2`: the canonical graph of
//! every fixture schema equals its language-neutral JSON, an independent encoder reproduces the
//! canonical bytes, third-party `blake3` reproduces the pinned hash, nested changes flip it,
//! recursion and duplicated spec functions collapse, and text-only presentation is ignored.
use semio_framework_os_kernel::os_dsl::schema::{FieldSpec, RecordLayout, RecordSpec, Shape};
use semio_framework_os_kernel::os_pack::{schema_hash, PackSchemaGraph};

fn inner_spec() -> RecordSpec {
    RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(0, "x", Shape::Float)])
}

fn inner_changed_spec() -> RecordSpec {
    RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(0, "x", Shape::Int)])
}

fn other_spec() -> RecordSpec {
    RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(0, "mode", Shape::Enum(vec![("beta".into(), 1), ("alpha".into(), 0)]))])
}

fn hashed_nested_spec(inner: fn() -> RecordSpec) -> RecordSpec {
    RecordSpec::new(
        Some("nested"),
        RecordLayout::Lines,
        vec![FieldSpec::new(2, "maybe", Shape::Record(other_spec)).optional(), FieldSpec::new(0, "inner", Shape::Record(inner)), FieldSpec::new(1, "items", Shape::List(Box::new(Shape::Record(inner))))],
    )
}

fn node_spec() -> RecordSpec {
    RecordSpec::new(Some("node"), RecordLayout::Inline, vec![FieldSpec::new(0, "name", Shape::Text), FieldSpec::new(1, "children", Shape::List(Box::new(Shape::Record(node_spec)))), FieldSpec::new(2, "alias", Shape::Record(node_twin_spec)).optional()])
}

fn node_twin_spec() -> RecordSpec {
    RecordSpec::new(None, RecordLayout::Lines, vec![FieldSpec::new(2, "alias", Shape::Record(node_spec)).optional(), FieldSpec::new(1, "children", Shape::List(Box::new(Shape::Record(node_twin_spec)))), FieldSpec::new(0, "name", Shape::Text)])
}

fn step_spec() -> RecordSpec {
    RecordSpec::new(Some("move"), RecordLayout::Inline, vec![FieldSpec::new(0, "to", Shape::Coord(3))])
}

fn wait_spec() -> RecordSpec {
    RecordSpec::new(Some("wait"), RecordLayout::Inline, vec![FieldSpec::new(0, "ms", Shape::UInt)])
}

fn row_spec() -> RecordSpec {
    RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(0, "x", Shape::Float), FieldSpec::new(1, "y", Shape::Float)])
}

fn every_shape_spec() -> RecordSpec {
    RecordSpec::new(
        Some("every"),
        RecordLayout::Lines,
        vec![
            FieldSpec::new(1, "flag", Shape::Bool),
            FieldSpec::new(2, "count", Shape::UInt),
            FieldSpec::new(3, "blob", Shape::Bytes64).optional(),
            FieldSpec::new(4, "point", Shape::Tuple(Box::new(Shape::Float), Some(3))),
            FieldSpec::new(5, "pair", Shape::Tuple(Box::new(Shape::Int), None)),
            FieldSpec::new(6, "body", Shape::Block(Box::new(Shape::Text))),
            FieldSpec::new(7, "steps", Shape::Statements(vec![("wait".into(), wait_spec as fn() -> RecordSpec), ("move".into(), step_spec)])),
            FieldSpec::new(8, "tags", Shape::Map(Box::new(Shape::Text))),
            FieldSpec::new(9, "any", Shape::Value),
            FieldSpec::new(10, "rows", Shape::Table(row_spec)),
            FieldSpec::new(11, "edge", Shape::Wire),
            FieldSpec::new(12, "force", Shape::Quantity(semio_framework_os_kernel::os_dsl::unit_by_symbol("GPa").unwrap())),
            FieldSpec::new(13, "turn", Shape::Angle(semio_framework_os_kernel::os_dsl::unit_by_symbol("deg").unwrap())),
            FieldSpec::new(14, "material", Shape::Ref("material")),
            FieldSpec::new(15, "at", Shape::Coord(2)),
            FieldSpec::new(16, "axis", Shape::Dir),
            FieldSpec::new(17, "size", Shape::Dim(3)),
            FieldSpec::new(18, "span", Shape::Range),
            FieldSpec::new(19, "times", Shape::Count),
            FieldSpec::new(20, "formula", Shape::Expr),
            FieldSpec::new(21, "code", Shape::Embed("jack")),
            FieldSpec::new(22, "script", Shape::EmbedFrom("lang")),
            FieldSpec::new(23, "origin", Shape::Record(row_spec)).flatten(),
        ],
    )
}

fn schema_hash_case_spec(name: &str) -> RecordSpec {
    match name {
        "flat" => RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(2, "b", Shape::Text), FieldSpec::new(1, "a", Shape::Int)]),
        "nested" => hashed_nested_spec(inner_spec),
        "nested-inner-field-changed" => hashed_nested_spec(inner_changed_spec),
        "recursive" => node_spec(),
        "every-shape" => every_shape_spec(),
        other => panic!("unknown schema hash case {other}"),
    }
}

fn schema_hash_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🔑️schema-hash/🔣️.json")).expect("schema hash fixture")
}

fn hex_of(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn graph_json(graph: &PackSchemaGraph) -> serde_json::Value {
    serde_json::from_str(&semio_framework_os_kernel::os_pack::json::to_string(&graph.to_json())).expect("graph json")
}

fn independent_varint(out: &mut Vec<u8>, mut value: u64) {
    loop {
        let byte = (value & 0x7f) as u8;
        value >>= 7;
        if value == 0 {
            out.push(byte);
            return;
        }
        out.push(byte | 0x80);
    }
}

fn independent_text(out: &mut Vec<u8>, value: &serde_json::Value) {
    let text = value.as_str().unwrap();
    independent_varint(out, text.len() as u64);
    out.extend_from_slice(text.as_bytes());
}

fn independent_shape(out: &mut Vec<u8>, tags: &serde_json::Value, shape: &serde_json::Value) {
    let kind = shape["kind"].as_str().unwrap();
    out.push(tags[kind].as_u64().unwrap() as u8);
    match kind {
        "enum" => {
            let variants = shape["variants"].as_array().unwrap();
            independent_varint(out, variants.len() as u64);
            for variant in variants {
                independent_varint(out, variant[0].as_u64().unwrap());
                independent_text(out, &variant[1]);
            }
        }
        "tuple" => {
            independent_shape(out, tags, &shape["item"]);
            match shape["len"].as_u64() {
                Some(len) => {
                    out.push(1);
                    independent_varint(out, len);
                }
                None => out.push(0),
            }
        }
        "list" | "block" | "map" => independent_shape(out, tags, &shape["item"]),
        "record" | "table" => independent_varint(out, shape["record"].as_u64().unwrap()),
        "statements" => {
            let variants = shape["variants"].as_array().unwrap();
            independent_varint(out, variants.len() as u64);
            for variant in variants {
                independent_text(out, &variant[0]);
                independent_varint(out, variant[1].as_u64().unwrap());
            }
        }
        "quantity" | "angle" => independent_text(out, &shape["unit"]),
        "ref" => independent_text(out, &shape["entity"]),
        "coord" | "dim" => out.push(shape["dims"].as_u64().unwrap() as u8),
        "embed" => independent_text(out, &shape["lang"]),
        "embedFrom" => independent_text(out, &shape["key"]),
        _ => {}
    }
}

fn independent_canonical_bytes(tags: &serde_json::Value, graph: &serde_json::Value) -> Vec<u8> {
    let mut out = Vec::new();
    let records = graph.as_array().unwrap();
    independent_varint(&mut out, records.len() as u64);
    for fields in records {
        let fields = fields.as_array().unwrap();
        independent_varint(&mut out, fields.len() as u64);
        for field in fields {
            independent_varint(&mut out, field["id"].as_u64().unwrap());
            independent_text(&mut out, &field["key"]);
            out.push(u8::from(field["optional"].as_bool().unwrap()) | (u8::from(field["flatten"].as_bool().unwrap()) << 1));
            independent_shape(&mut out, tags, &field["shape"]);
        }
    }
    out
}

#[test]
fn schema_graph_matches_the_fixture_and_an_independent_blake3() {
    let fixture = schema_hash_fixture();
    for case in fixture["cases"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let spec = schema_hash_case_spec(name);
        let graph = PackSchemaGraph::of(&spec);
        assert_eq!(graph_json(&graph), case["graph"], "{name} graph");
        let canonical = graph.canonical_bytes();
        assert_eq!(independent_canonical_bytes(&fixture["shapeTags"], &case["graph"]), canonical, "{name} independent canonical bytes");
        assert_eq!(hex_of(&canonical), case["canonicalHex"].as_str().unwrap(), "{name} canonical hex");
        let hash = schema_hash(&spec);
        assert_eq!(blake3::hash(&canonical).as_bytes(), &hash, "{name} blake3");
        assert_eq!(hex_of(&hash), case["schemaHash"].as_str().unwrap(), "{name} pinned hash");
        assert_eq!(schema_hash(&spec), hash, "{name} rerun");
    }
}

#[test]
fn nested_structure_changes_flip_the_schema_hash() {
    let base = schema_hash(&hashed_nested_spec(inner_spec));
    assert_ne!(base, schema_hash(&hashed_nested_spec(inner_changed_spec)));
    let variants = |table: Vec<(&str, u32)>| RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(0, "mode", Shape::Enum(table.into_iter().map(|(tag, ordinal)| (tag.to_string(), ordinal)).collect()))]);
    assert_ne!(schema_hash(&variants(vec![("alpha", 0), ("beta", 1)])), schema_hash(&variants(vec![("alpha", 1), ("beta", 0)])));
    assert_ne!(schema_hash(&variants(vec![("alpha", 0), ("beta", 1)])), schema_hash(&variants(vec![("alpha", 0), ("gamma", 1)])));
    let single = |field: FieldSpec| RecordSpec::new(None, RecordLayout::Inline, vec![field]);
    assert_ne!(schema_hash(&single(FieldSpec::new(0, "a", Shape::Text))), schema_hash(&single(FieldSpec::new(0, "a", Shape::Text).optional())));
    assert_ne!(schema_hash(&single(FieldSpec::new(0, "a", Shape::Tuple(Box::new(Shape::Float), Some(2))))), schema_hash(&single(FieldSpec::new(0, "a", Shape::Tuple(Box::new(Shape::Float), Some(3))))));
    assert_ne!(schema_hash(&single(FieldSpec::new(0, "a", Shape::List(Box::new(Shape::Record(inner_spec)))))), schema_hash(&single(FieldSpec::new(0, "a", Shape::List(Box::new(Shape::Record(inner_changed_spec)))))));
    assert_ne!(schema_hash(&single(FieldSpec::new(0, "a", Shape::Statements(vec![("move".into(), step_spec as fn() -> RecordSpec)])))), schema_hash(&single(FieldSpec::new(0, "a", Shape::Statements(vec![("go".into(), step_spec as fn() -> RecordSpec)])))));
    assert_ne!(schema_hash(&single(FieldSpec::new(0, "a", Shape::Embed("jack")))), schema_hash(&single(FieldSpec::new(0, "a", Shape::EmbedFrom("jack")))));
}

#[test]
fn recursive_and_duplicated_spec_functions_share_one_canonical_graph() {
    assert_eq!(PackSchemaGraph::of(&node_spec()), PackSchemaGraph::of(&node_twin_spec()));
    assert_eq!(PackSchemaGraph::of(&node_spec()).records.len(), 1);
    assert_eq!(schema_hash(&node_spec()), schema_hash(&node_twin_spec()));
}

#[test]
fn schema_hash_ignores_text_only_presentation() {
    let presented = RecordSpec::new(Some("shown"), RecordLayout::Lines, vec![FieldSpec::new(0, "a", Shape::Text).positional(0).call_name().defines("thing"), FieldSpec::new(1, "b", Shape::Record(inner_spec))]);
    let bare = RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(1, "b", Shape::Record(inner_spec)), FieldSpec::new(0, "a", Shape::Text)]);
    assert_eq!(schema_hash(&presented), schema_hash(&bare));
}
