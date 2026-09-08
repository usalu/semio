
use super::*;
use crate::os_dsl::schema::{ExprOp, ExprValue};
use crate::os_dsl::schema::{FieldSpec, RecordLayout};

//#region 🔖️Fixtures
// 🚫️async: E4 fn-pointer slot — passed by name into `Shape::Record`/`Shape::Statements`
// (`fn() -> RecordSpec`, unnameable if async) — see R9/E4.
fn nested_spec() -> RecordSpec {
    RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(1, "a", Shape::Int), FieldSpec::new(2, "b", Shape::Text).optional()])
}

// 🚫️async: E4 fn-pointer slot — see nested_spec above
fn table_row_spec() -> RecordSpec {
    RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(1, "id", Shape::UInt), FieldSpec::new(2, "name", Shape::Text), FieldSpec::new(3, "score", Shape::Float), FieldSpec::new(4, "active", Shape::Bool)])
}

// 🚫️async: E4 fn-pointer slot — see nested_spec above
fn header_spec() -> RecordSpec {
    RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(1, "name", Shape::Text), FieldSpec::new(2, "description", Shape::Text).optional()])
}

// 🚫️async: E4 fn-pointer slot — see nested_spec above
fn table_row_with_nested_record_spec() -> RecordSpec {
    RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(1, "id", Shape::UInt), FieldSpec::new(2, "header", Shape::Record(header_spec))])
}

// 🚫️async: E4 fn-pointer slot — see nested_spec above
fn table_row_with_tuple_spec() -> RecordSpec {
    RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(1, "id", Shape::UInt), FieldSpec::new(2, "distortion", Shape::Tuple(Box::new(Shape::Float), Some(5)))])
}

// 🚫️async: E4 fn-pointer slot — see nested_spec above
fn stmt_foo_spec() -> RecordSpec {
    RecordSpec::new(Some("foo"), RecordLayout::Lines, vec![FieldSpec::new(1, "x", Shape::Int)])
}

// 🚫️async: E4 fn-pointer slot — see nested_spec above
fn stmt_bar_spec() -> RecordSpec {
    RecordSpec::new(Some("bar"), RecordLayout::Lines, vec![FieldSpec::new(1, "y", Shape::Text)])
}

/// @emoji 🧬️ One field of every `Shape` variant, exercising every wire tag in a single spec.
fn full_spec() -> RecordSpec {
    RecordSpec::new(
        None,
        RecordLayout::Lines,
        vec![
            FieldSpec::new(1, "bool_field", Shape::Bool),
            FieldSpec::new(2, "int_field", Shape::Int),
            FieldSpec::new(3, "uint_field", Shape::UInt),
            FieldSpec::new(4, "float_field", Shape::Float),
            FieldSpec::new(5, "text_field", Shape::Text),
            FieldSpec::new(6, "bytes_field", Shape::Bytes64),
            FieldSpec::new(7, "enum_field", Shape::Enum(vec![("red".to_string(), 0), ("green".to_string(), 1), ("blue".to_string(), 2)])),
            FieldSpec::new(8, "tuple_field", Shape::Tuple(Box::new(Shape::Int), Some(3))),
            FieldSpec::new(9, "list_field", Shape::List(Box::new(Shape::Text))),
            FieldSpec::new(10, "record_field", Shape::Record(nested_spec)),
            FieldSpec::new(11, "block_field", Shape::Block(Box::new(Shape::Int))),
            FieldSpec::new(12, "statements_field", Shape::Statements(vec![("foo".to_string(), stmt_foo_spec), ("bar".to_string(), stmt_bar_spec)])),
            FieldSpec::new(13, "map_field", Shape::Map(Box::new(Shape::Int))),
            FieldSpec::new(14, "value_field", Shape::Value),
            FieldSpec::new(15, "table_field", Shape::Table(table_row_spec)),
            FieldSpec::new(16, "wire_field", Shape::Wire),
            FieldSpec::new(17, "quantity_field", Shape::Quantity(crate::os_dsl::unit_by_symbol("GPa").unwrap())),
            FieldSpec::new(18, "angle_field", Shape::Angle(crate::os_dsl::unit_by_symbol("deg").unwrap())),
            FieldSpec::new(19, "ref_field", Shape::Ref("material")),
            FieldSpec::new(20, "coord_field", Shape::Coord(3)),
            FieldSpec::new(21, "dir_field", Shape::Dir),
            FieldSpec::new(22, "dim_field", Shape::Dim(2)),
            FieldSpec::new(23, "range_field", Shape::Range),
            FieldSpec::new(24, "count_field", Shape::Count),
            FieldSpec::new(25, "expr_field", Shape::Expr),
            FieldSpec::new(26, "embed_field", Shape::Embed("jack")),
        ],
    )
}

fn full_record() -> RecordValue {
    let mut fields = HashMap::new();
    fields.insert(1, FieldValue::Bool(true));
    fields.insert(2, FieldValue::Int(-42));
    fields.insert(3, FieldValue::UInt(9_999_999_999));
    fields.insert(4, FieldValue::Float(3.5));
    fields.insert(5, FieldValue::Text("hello café".to_string()));
    fields.insert(6, FieldValue::Bytes64(vec![1, 2, 3, 4, 5]));
    fields.insert(7, FieldValue::Enum(1));
    fields.insert(8, FieldValue::Tuple(vec![FieldValue::Int(1), FieldValue::Int(2), FieldValue::Int(3)]));
    fields.insert(9, FieldValue::List(vec![FieldValue::Text("x".to_string()), FieldValue::Text("y".to_string())]));
    let mut nested = HashMap::new();
    nested.insert(1, FieldValue::Int(7));
    nested.insert(2, FieldValue::Text("nested".to_string()));
    fields.insert(10, FieldValue::Record(RecordValue { fields: nested }));
    fields.insert(11, FieldValue::Block(Box::new(FieldValue::Int(99))));
    let mut foo_fields = HashMap::new();
    foo_fields.insert(1, FieldValue::Int(5));
    let mut bar_fields = HashMap::new();
    bar_fields.insert(1, FieldValue::Text("statement text".to_string()));
    fields.insert(12, FieldValue::Statements(vec![("foo".to_string(), RecordValue { fields: foo_fields }), ("bar".to_string(), RecordValue { fields: bar_fields })]));
    // `Map`/`DslValue::Object` are `Vec`-backed so `PartialEq` is order-sensitive; since
    // canonical encoding always sorts entries by key bytes, these fixtures are pre-sorted
    // ("aaa" < "zzz", "arr" < "k") so the round-trip equality check below holds exactly.
    fields.insert(13, FieldValue::Map(vec![("aaa".to_string(), FieldValue::Int(2)), ("zzz".to_string(), FieldValue::Int(1))]));
    fields.insert(14, FieldValue::Value(DslValue::Object(vec![("arr".to_string(), DslValue::Array(vec![DslValue::Bool(true), DslValue::Null])), ("k".to_string(), DslValue::float(1.0))])));
    let table_rows: Vec<FieldValue> = (0..3)
        .map(|i| {
            let mut row = HashMap::new();
            row.insert(1, FieldValue::UInt(i as u64));
            row.insert(2, FieldValue::Text(format!("row{i}")));
            row.insert(3, FieldValue::Float(i as f64 * 1.5));
            row.insert(4, FieldValue::Bool(i % 2 == 0));
            FieldValue::Record(RecordValue { fields: row })
        })
        .collect();
    fields.insert(15, FieldValue::List(table_rows));
    fields.insert(
        16,
        FieldValue::Wire(WireValue {
            from: WireNode { id: "a".to_string(), kind: Some("Kind".to_string()), port: None },
            edge: Some((true, WireNode { id: "b".to_string(), kind: None, port: Some("out".to_string()) })),
            edge_label: WireEdgeLabel::default(),
            properties: DslValue::Object(vec![("weight".to_string(), DslValue::float(2.0))]),
        }),
    );
    fields.insert(17, FieldValue::Float(210.0));
    fields.insert(18, FieldValue::Float(0.5));
    fields.insert(19, FieldValue::Text("s355".to_string()));
    fields.insert(20, FieldValue::Tuple(vec![FieldValue::Float(1.35), FieldValue::Float(0.0), FieldValue::Float(-2.4)]));
    fields.insert(21, FieldValue::Tuple(vec![FieldValue::Float(0.0), FieldValue::Float(1.0), FieldValue::Float(0.0)]));
    fields.insert(22, FieldValue::Tuple(vec![FieldValue::Float(2.4), FieldValue::Float(0.12)]));
    fields.insert(23, FieldValue::Tuple(vec![FieldValue::Float(0.0), FieldValue::Float(10.0), FieldValue::Float(0.5)]));
    fields.insert(24, FieldValue::UInt(24));
    fields.insert(
        25,
        FieldValue::Expr(ExprValue::Binary(
            ExprOp::Add,
            Box::new(ExprValue::Binary(ExprOp::Mul, Box::new(ExprValue::Num(1.35)), Box::new(ExprValue::Var("G".to_string())))),
            Box::new(ExprValue::Binary(ExprOp::Mul, Box::new(ExprValue::Num(1.5)), Box::new(ExprValue::Var("Q".to_string())))),
        )),
    );
    fields.insert(26, FieldValue::Text("MATCH (a) RETURN a".to_string()));
    RecordValue { fields }
}
//#endregion 🔖️Fixtures

//#region 🔖️RoundTrip
#[test]
fn round_trips_every_shape_variant_in_one_document() {
    let spec = full_spec();
    let record = full_record();
    let bytes = encode_document(&spec, &record, &EncodeOptions::default()).expect("encode");
    let (decoded, report) = decode_document(&bytes, &spec, &DecodeOptions::default()).expect("decode");
    assert!(report.unknown_field_ids.is_empty());
    assert!(!report.schema_drift);
    assert_eq!(decoded, record);
}

#[test]
fn round_trips_scalar_edge_cases() {
    let spec = RecordSpec::new(
        None,
        RecordLayout::Inline,
        vec![
            FieldSpec::new(1, "empty_text", Shape::Text),
            FieldSpec::new(2, "nan", Shape::Float),
            FieldSpec::new(3, "neg_zero", Shape::Float),
            FieldSpec::new(4, "big_int", Shape::Int),
            FieldSpec::new(5, "big_uint", Shape::UInt),
            FieldSpec::new(6, "unicode", Shape::Text),
            FieldSpec::new(7, "empty_list", Shape::List(Box::new(Shape::Int))),
            FieldSpec::new(8, "empty_map", Shape::Map(Box::new(Shape::Int))),
            FieldSpec::new(9, "empty_bytes", Shape::Bytes64),
        ],
    );
    let mut fields = HashMap::new();
    fields.insert(1, FieldValue::Text(String::new()));
    fields.insert(2, FieldValue::Float(f64::NAN));
    fields.insert(3, FieldValue::Float(-0.0));
    fields.insert(4, FieldValue::Int(i64::MIN));
    fields.insert(5, FieldValue::UInt(u64::MAX));
    fields.insert(6, FieldValue::Text("héllo wörld 🔖️ 日本語".to_string()));
    fields.insert(7, FieldValue::List(vec![]));
    fields.insert(8, FieldValue::Map(vec![]));
    fields.insert(9, FieldValue::Bytes64(vec![]));
    let record = RecordValue { fields };

    let bytes = encode_document(&spec, &record, &EncodeOptions::default()).expect("encode");
    let (decoded, _) = decode_document(&bytes, &spec, &DecodeOptions::default()).expect("decode");

    assert_eq!(decoded.get(1), Some(&FieldValue::Text(String::new())));
    match decoded.get(2) {
        Some(FieldValue::Float(f)) => assert!(f.is_nan()),
        other => panic!("expected NaN float, got {other:?}"),
    }
    assert_eq!(decoded.get(3), Some(&FieldValue::Float(-0.0)));
    assert!(matches!(decoded.get(3), Some(FieldValue::Float(f)) if f.is_sign_negative()));
    assert_eq!(decoded.get(4), Some(&FieldValue::Int(i64::MIN)));
    assert_eq!(decoded.get(5), Some(&FieldValue::UInt(u64::MAX)));
    assert_eq!(decoded.get(6), Some(&FieldValue::Text("héllo wörld 🔖️ 日本語".to_string())));
    assert_eq!(decoded.get(7), Some(&FieldValue::List(vec![])));
    assert_eq!(decoded.get(8), Some(&FieldValue::Map(vec![])));
    assert_eq!(decoded.get(9), Some(&FieldValue::Bytes64(vec![])));
}

#[test]
fn packed_numeric_list_and_tuple_round_trip_and_use_packed_tags() {
    let spec = RecordSpec::new(
        None,
        RecordLayout::Inline,
        vec![
            FieldSpec::new(1, "ints", Shape::List(Box::new(Shape::Int))),
            FieldSpec::new(2, "floats", Shape::List(Box::new(Shape::Float))),
            FieldSpec::new(3, "uints", Shape::Tuple(Box::new(Shape::UInt), None)),
            FieldSpec::new(4, "mixed", Shape::List(Box::new(Shape::Value))),
        ],
    );
    let mut fields = HashMap::new();
    fields.insert(1, FieldValue::List(vec![FieldValue::Int(1), FieldValue::Int(-2), FieldValue::Int(3)]));
    fields.insert(2, FieldValue::List(vec![FieldValue::Float(1.5), FieldValue::Float(-2.5)]));
    fields.insert(3, FieldValue::Tuple(vec![FieldValue::UInt(1), FieldValue::UInt(2), FieldValue::UInt(3)]));
    fields.insert(4, FieldValue::List(vec![FieldValue::Value(DslValue::Bool(true)), FieldValue::Value(DslValue::Null)]));
    let record = RecordValue { fields };

    let bytes = encode_document(&spec, &record, &EncodeOptions::default()).expect("encode");
    let (decoded, _) = decode_document(&bytes, &spec, &DecodeOptions::default()).expect("decode");
    assert_eq!(decoded.get(1), record.get(1));
    assert_eq!(decoded.get(2), record.get(2));
    assert_eq!(decoded.get(3), record.get(3));
    assert_eq!(decoded.get(4), record.get(4));
}

#[test]
fn table_soa_round_trips_with_sparse_columns() {
    let spec = RecordSpec::new(None, RecordLayout::Lines, vec![FieldSpec::new(1, "rows", Shape::Table(table_row_spec))]);
    let mut row0 = HashMap::new();
    row0.insert(1, FieldValue::UInt(10));
    row0.insert(2, FieldValue::Text("alpha".to_string()));
    row0.insert(3, FieldValue::Float(1.25));
    row0.insert(4, FieldValue::Bool(true));
    let mut row1 = HashMap::new();
    row1.insert(1, FieldValue::UInt(20));
    // row1 omits "name" (sparse Text column) and "active" (sparse Bool column).
    row1.insert(3, FieldValue::Float(-9.5));
    let mut fields = HashMap::new();
    fields.insert(1, FieldValue::List(vec![FieldValue::Record(RecordValue { fields: row0 }), FieldValue::Record(RecordValue { fields: row1 })]));
    let record = RecordValue { fields };

    let bytes = encode_document(&spec, &record, &EncodeOptions::default()).expect("encode");
    let (decoded, _) = decode_document(&bytes, &spec, &DecodeOptions::default()).expect("decode");
    let Some(FieldValue::List(rows)) = decoded.get(1) else { panic!("expected table rows") };
    assert_eq!(rows.len(), 2);
    let FieldValue::Record(r0) = &rows[0] else { panic!("row0 not a record") };
    assert_eq!(r0.get(2), Some(&FieldValue::Text("alpha".to_string())));
    assert_eq!(r0.get(4), Some(&FieldValue::Bool(true)));
    let FieldValue::Record(r1) = &rows[1] else { panic!("row1 not a record") };
    assert_eq!(r1.get(2), Some(&FieldValue::Absent));
    assert_eq!(r1.get(4), Some(&FieldValue::Absent));
    assert_eq!(r1.get(3), Some(&FieldValue::Float(-9.5)));
}

/// @emoji 🪟️ Regression for a `TableSoA` column whose element type is a nested (non-Option)
/// `Record` with its own `Option` sub-field left absent — `decode_table_soa`'s fallback branch
/// must thread the known column shape through so `decode_record_fields` still backfills that
/// sub-field as `Absent` instead of leaving it missing from the decoded `RecordValue` map.
#[test]
fn table_soa_nested_record_column_backfills_absent_option_subfield() {
    let spec = RecordSpec::new(None, RecordLayout::Lines, vec![FieldSpec::new(1, "rows", Shape::Table(table_row_with_nested_record_spec))]);
    let mut header_fields = HashMap::new();
    header_fields.insert(1, FieldValue::Text("Stakeholder A".to_string()));
    // "description" (field 2, Option<Text>) is intentionally omitted from the fixture — it
    // encodes as `Absent` and canonical-mode compaction drops it from the wire entirely.
    let mut row = HashMap::new();
    row.insert(1, FieldValue::UInt(1));
    row.insert(2, FieldValue::Record(RecordValue { fields: header_fields }));
    let mut fields = HashMap::new();
    fields.insert(1, FieldValue::List(vec![FieldValue::Record(RecordValue { fields: row })]));
    let record = RecordValue { fields };

    let bytes = encode_document(&spec, &record, &EncodeOptions::default()).expect("encode");
    let (decoded, _) = decode_document(&bytes, &spec, &DecodeOptions::default()).expect("decode");
    let Some(FieldValue::List(rows)) = decoded.get(1) else { panic!("expected table rows") };
    let FieldValue::Record(row0) = &rows[0] else { panic!("row0 not a record") };
    let Some(FieldValue::Record(header)) = row0.get(2) else { panic!("expected header record") };
    assert_eq!(header.get(1), Some(&FieldValue::Text("Stakeholder A".to_string())));
    assert_eq!(header.get(2), Some(&FieldValue::Absent), "nested record's Option sub-field must backfill to Absent, not be missing");
}

/// @emoji 🎯️ Regression for a `TableSoA` column whose element type is a fixed-size `Tuple`
/// (e.g. a `[f32; 5]` lens-distortion field) — because every element is the same numeric
/// kind, `encode_seq` collapses it to the packed `TAG_PACKED_F64` wire form, which carries no
/// tuple-vs-list marker of its own. `decode_table_soa`'s fallback branch must thread the
/// known column `Shape::Tuple` through so `decode_value` reconstructs a `FieldValue::Tuple`,
/// not a `FieldValue::List` — a `List` fails `[T; N]`'s `DslField::from_value` downstream.
#[test]
fn table_soa_tuple_column_round_trips_as_tuple_not_list() {
    let spec = RecordSpec::new(None, RecordLayout::Lines, vec![FieldSpec::new(1, "rows", Shape::Table(table_row_with_tuple_spec))]);
    let mut row = HashMap::new();
    row.insert(1, FieldValue::UInt(1));
    row.insert(2, FieldValue::Tuple(vec![FieldValue::Float(0.1), FieldValue::Float(0.2), FieldValue::Float(0.3), FieldValue::Float(0.4), FieldValue::Float(0.5)]));
    let mut fields = HashMap::new();
    fields.insert(1, FieldValue::List(vec![FieldValue::Record(RecordValue { fields: row })]));
    let record = RecordValue { fields };

    let bytes = encode_document(&spec, &record, &EncodeOptions::default()).expect("encode");
    let (decoded, _) = decode_document(&bytes, &spec, &DecodeOptions::default()).expect("decode");
    let Some(FieldValue::List(rows)) = decoded.get(1) else { panic!("expected table rows") };
    let FieldValue::Record(row0) = &rows[0] else { panic!("row0 not a record") };
    assert_eq!(
        row0.get(2),
        Some(&FieldValue::Tuple(vec![FieldValue::Float(0.1), FieldValue::Float(0.2), FieldValue::Float(0.3), FieldValue::Float(0.4), FieldValue::Float(0.5)])),
        "tuple-shaped table column must decode as FieldValue::Tuple, not FieldValue::List"
    );
}

#[test]
fn wire_literal_round_trips_bare_node_and_undirected_edge() {
    let spec = RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(1, "w", Shape::Wire)]);
    let mut fields = HashMap::new();
    fields.insert(1, FieldValue::Wire(WireValue { from: WireNode { id: "solo".to_string(), kind: None, port: None }, edge: None, edge_label: WireEdgeLabel::default(), properties: DslValue::Object(vec![]) }));
    let record = RecordValue { fields };
    let bytes = encode_document(&spec, &record, &EncodeOptions::default()).expect("encode");
    let (decoded, _) = decode_document(&bytes, &spec, &DecodeOptions::default()).expect("decode");
    assert_eq!(decoded.get(1), record.get(1));
}
//#endregion 🔖️RoundTrip

//#region 🔖️Canonical
#[test]
fn canonical_encoding_is_byte_stable_across_shuffled_insertion_order() {
    let spec = full_spec();
    let record_a = full_record();
    // Rebuild an equal `RecordValue` by inserting fields in a deliberately different order —
    // `HashMap` insertion order never affects iteration order anyway, but this at minimum
    // proves two independent builds of an equal value encode identically, twice in a row.
    let mut shuffled_fields = HashMap::new();
    let mut ids: Vec<u16> = record_a.fields.keys().copied().collect();
    ids.sort_unstable_by(|a, b| b.cmp(a));
    for id in ids {
        shuffled_fields.insert(id, record_a.fields.get(&id).unwrap().clone());
    }
    let record_b = RecordValue { fields: shuffled_fields };
    assert_eq!(record_a, record_b);

    let bytes_a = encode_document(&spec, &record_a, &EncodeOptions::default()).expect("encode a");
    let bytes_b = encode_document(&spec, &record_b, &EncodeOptions::default()).expect("encode b");
    assert_eq!(bytes_a, bytes_b, "canonical encoding must be byte-identical regardless of HashMap insertion order");

    let bytes_a_again = encode_document(&spec, &record_a, &EncodeOptions::default()).expect("encode a again");
    assert_eq!(bytes_a, bytes_a_again, "encoding the same document twice must be byte-identical");
}

#[test]
fn schema_hash_is_order_independent_and_content_sensitive() {
    let spec_a = RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(2, "b", Shape::Text), FieldSpec::new(1, "a", Shape::Int)]);
    let spec_b = RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(1, "a", Shape::Int), FieldSpec::new(2, "b", Shape::Text)]);
    assert_eq!(schema_hash(&spec_a), schema_hash(&spec_b));

    let spec_c = RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(1, "a", Shape::Int), FieldSpec::new(2, "b", Shape::Float)]);
    assert_ne!(schema_hash(&spec_a), schema_hash(&spec_c));
}
//#endregion 🔖️Canonical

//#region 🔖️Unknown
#[test]
fn unknown_field_round_trips_through_decode_then_reencode() {
    let full = full_spec();
    let mut record = full_record();
    // Add a field id absent from `narrow_spec` below but present in `full` for the initial
    // encode, simulating "a writer on a newer schema version wrote an extra field".
    record.fields.insert(200, FieldValue::Text("extra field payload".to_string()));
    record.fields.insert(201, FieldValue::List(vec![FieldValue::Int(1), FieldValue::Int(2), FieldValue::Int(3)]));

    let mut widened_fields = full.fields.clone();
    widened_fields.push(FieldSpec::new(200, "extra", Shape::Text));
    widened_fields.push(FieldSpec::new(201, "extra_list", Shape::List(Box::new(Shape::Int))));
    let widened_spec = RecordSpec::new(full.keyword.as_deref(), full.layout, widened_fields);

    let bytes = encode_document(&widened_spec, &record, &EncodeOptions::default()).expect("encode with widened spec");

    // Decode against the NARROW spec (doesn't know fields 200/201) — they must still decode
    // and be reported as unknown.
    let (decoded, report) = decode_document(&bytes, &full, &DecodeOptions::default()).expect("decode with narrow spec");
    let mut unknown_sorted = report.unknown_field_ids;
    unknown_sorted.sort_unstable();
    assert_eq!(unknown_sorted, vec![200, 201]);
    assert_eq!(decoded.get(200), Some(&FieldValue::Text("extra field payload".to_string())));
    assert_eq!(decoded.get(201), Some(&FieldValue::List(vec![FieldValue::Int(1), FieldValue::Int(2), FieldValue::Int(3)])));

    // Re-encode the decoded (narrow-spec) RecordValue — the unknown fields must survive.
    let reencoded = encode_document(&full, &decoded, &EncodeOptions::default()).expect("re-encode");
    let (decoded_again, report_again) = decode_document(&reencoded, &full, &DecodeOptions::default()).expect("decode again");
    assert_eq!(decoded_again.get(200), Some(&FieldValue::Text("extra field payload".to_string())));
    assert_eq!(decoded_again.get(201), Some(&FieldValue::List(vec![FieldValue::Int(1), FieldValue::Int(2), FieldValue::Int(3)])));
    let mut unknown_again_sorted = report_again.unknown_field_ids;
    unknown_again_sorted.sort_unstable();
    assert_eq!(unknown_again_sorted, vec![200, 201]);
}

#[test]
fn preserve_unknown_false_drops_unknown_fields_from_decoded_value_but_still_reports_them() {
    let narrow = RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(1, "a", Shape::Int)]);
    let wide = RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(1, "a", Shape::Int), FieldSpec::new(2, "b", Shape::Text)]);
    let mut fields = HashMap::new();
    fields.insert(1, FieldValue::Int(1));
    fields.insert(2, FieldValue::Text("dropped on decode".to_string()));
    let record = RecordValue { fields };

    let bytes = encode_document(&wide, &record, &EncodeOptions::default()).expect("encode");
    let mut options = DecodeOptions::default();
    options.preserve_unknown = false;
    let (decoded, report) = decode_document(&bytes, &narrow, &options).expect("decode");
    assert_eq!(report.unknown_field_ids, vec![2]);
    assert_eq!(decoded.get(2), None);
    assert_eq!(decoded.get(1), Some(&FieldValue::Int(1)));
}
//#endregion 🔖️Unknown

//#region 🔖️Chunking
#[test]
fn large_bytes_field_is_chunked_and_round_trips() {
    let spec = RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(1, "blob", Shape::Bytes64)]);
    let payload: Vec<u8> = (0..600_000u32).map(|i| (i % 256) as u8).collect();
    let mut fields = HashMap::new();
    fields.insert(1, FieldValue::Bytes64(payload.clone()));
    let record = RecordValue { fields };

    let mut options = EncodeOptions::default();
    options.chunk_threshold = 1024;
    options.chunk_size = 64 * 1024;
    let bytes = encode_document(&spec, &record, &options).expect("encode");
    let (decoded, _) = decode_document(&bytes, &spec, &DecodeOptions::default()).expect("decode");
    assert_eq!(decoded.get(1), Some(&FieldValue::Bytes64(payload)));
}

#[test]
fn document_body_splits_across_multiple_frames_when_frame_size_is_small() {
    let spec = RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(1, "text", Shape::Text)]);
    let mut fields = HashMap::new();
    fields.insert(1, FieldValue::Text("x".repeat(5000)));
    let record = RecordValue { fields };

    let mut options = EncodeOptions::default();
    options.frame_size = 64;
    let bytes = encode_document(&spec, &record, &options).expect("encode");
    let (decoded, _) = decode_document(&bytes, &spec, &DecodeOptions::default()).expect("decode");
    assert_eq!(decoded.get(1), record.get(1));
}
//#endregion 🔖️Chunking

//#region 🔖️RecordBody
#[test]
fn record_body_round_trips_every_shape() {
    let spec = full_spec();
    let record = full_record();
    let bytes = encode_record_body(&spec, &record, &EncodeOptions::default()).expect("encode");
    let (decoded, report) = decode_record_body(&bytes, &spec, &DecodeOptions::default()).expect("decode");
    assert_eq!(decoded, record);
    assert!(report.unknown_field_ids.is_empty());
    assert_eq!(decode_record_body_exact(&bytes, &spec, &DecodeOptions::default()).expect("exact decode"), record);
}

#[test]
fn record_body_is_deterministic_for_equal_inputs() {
    let spec = full_spec();
    let a = encode_record_body(&spec, &full_record(), &EncodeOptions::default()).expect("encode a");
    let b = encode_record_body(&spec, &full_record(), &EncodeOptions::default()).expect("encode b");
    assert_eq!(a, b);
}

#[test]
fn record_body_keeps_oversized_bytes_inline_instead_of_chunking() {
    let spec = RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(1, "blob", Shape::Bytes64)]);
    let payload: Vec<u8> = (0..600_000u32).map(|i| (i % 256) as u8).collect();
    let mut fields = HashMap::new();
    fields.insert(1, FieldValue::Bytes64(payload.clone()));
    let record = RecordValue { fields };

    let mut options = EncodeOptions::default();
    options.chunk_threshold = 1024;
    let bytes = encode_record_body(&spec, &record, &options).expect("encode");
    let (decoded, _) = decode_record_body(&bytes, &spec, &DecodeOptions::default()).expect("decode");
    assert_eq!(decoded.get(1), Some(&FieldValue::Bytes64(payload)));
}

#[test]
fn record_body_preserves_and_reports_unknown_fields() {
    let wide = RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(1, "a", Shape::Int), FieldSpec::new(9, "extra", Shape::Text)]);
    let narrow = RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(1, "a", Shape::Int)]);
    let mut fields = HashMap::new();
    fields.insert(1, FieldValue::Int(3));
    fields.insert(9, FieldValue::Text("kept".to_string()));
    let record = RecordValue { fields };

    let bytes = encode_record_body(&wide, &record, &EncodeOptions::default()).expect("encode");
    let (decoded, report) = decode_record_body(&bytes, &narrow, &DecodeOptions::default()).expect("decode");
    assert_eq!(decoded.get(9), Some(&FieldValue::Text("kept".to_string())));
    assert_eq!(report.unknown_field_ids, vec![9]);
    for preserve_unknown in [false, true] {
        let options = DecodeOptions { preserve_unknown, ..DecodeOptions::default() };
        assert!(decode_record_body_exact(&bytes, &narrow, &options).is_err());
    }
}

#[test]
fn exact_record_body_rejects_trailing_bytes_without_changing_compositional_decode() {
    let spec = full_spec();
    let record = full_record();
    let mut bytes = encode_record_body(&spec, &record, &EncodeOptions::default()).expect("encode");
    bytes.push(0);
    assert_eq!(decode_record_body(&bytes, &spec, &DecodeOptions::default()).expect("compositional decode").0, record);
    assert!(decode_record_body_exact(&bytes, &spec, &DecodeOptions::default()).is_err());
}

#[test]
fn retained_value_vm_covers_every_wire_tag_and_terminal_empty_close() {
    let mut bytes = vec![23];
    let values: Vec<Vec<u8>> = vec![
        vec![TAG_ABSENT],
        vec![TAG_FALSE],
        vec![TAG_TRUE],
        vec![TAG_INT, 2],
        vec![TAG_UINT, 1],
        [vec![TAG_F64], 1f64.to_le_bytes().to_vec()].concat(),
        vec![TAG_STR, 0],
        vec![TAG_STR_INLINE, 1, b'a'],
        vec![TAG_BYTES, 1, 7],
        vec![TAG_BYTES_CHUNKED, 1, 0],
        vec![TAG_ENUM, 1],
        vec![TAG_TUPLE, 0],
        vec![TAG_LIST, 0],
        vec![TAG_RECORD, 0],
        vec![TAG_BLOCK, TAG_TRUE],
        vec![TAG_STATEMENTS, 0],
        vec![TAG_MAP, 0],
        vec![TAG_VALUE, TAG_NULL],
        vec![TAG_WIRE, 0, 0, TAG_STR_INLINE, 1, b'n', TAG_NULL],
        vec![TAG_TABLE_SOA, 0, 0],
        vec![TAG_PACKED_F64, 0],
        vec![TAG_PACKED_VARINT, 0],
        vec![TAG_EXPR, TAG_STR_INLINE, 1, b'x'],
    ];
    for (field, value) in values.iter().enumerate() {
        bytes.push(field as u8);
        bytes.extend_from_slice(value);
    }
    let limits = PackLimits { max_file_len: 4096, max_segment_len: 4096, max_symbols: 8, max_depth: 32, max_items: 64, max_total_alloc: 4096 };
    let mut cursor = RetainedValueCursor::try_new(limits).expect("cursor");
    let mut tags = Vec::new();
    for (offset, byte) in bytes.iter().copied().enumerate() {
        cursor.admit_byte(offset as u64, byte).expect("admission");
        while cursor.pending.is_some() {
            if let Some(RetainedValueToken::Tag { value, .. }) = cursor.grant().expect("grant") {
                tags.push(value);
            }
        }
    }
    cursor.seal(bytes.len() as u64).expect("seal");
    loop {
        if matches!(cursor.grant().expect("finish"), Some(RetainedValueToken::Complete { .. })) {
            break;
        }
    }
    for tag in 0u8..=0x17 {
        assert!(tags.contains(&tag), "missing retained tag {tag:#04x}");
    }
    while cursor.close_step(1) != crate::os_pack::format::RetainedPackCloseStep::Complete {}
    assert!(cursor.terminal_is_empty());
}

#[test]
fn retained_value_vm_rejects_truncation_utf8_depth_and_counts() {
    let limits = PackLimits { max_file_len: 64, max_segment_len: 8, max_symbols: 1, max_depth: 1, max_items: 1, max_total_alloc: 64 };
    let mut truncated = RetainedValueCursor::try_new(limits.clone()).expect("truncated");
    truncated.admit_byte(0, 1).expect("count");
    truncated.grant().expect("count grant");
    truncated.seal(1).expect("seal");
    while truncated.grant().expect_err("truncated value") != PackError::Truncated(1) {}
    while truncated.close_step(1) != crate::os_pack::format::RetainedPackCloseStep::Complete {}

    let mut count = RetainedValueCursor::try_new(limits.clone()).expect("count");
    count.admit_byte(0, 2).expect("count byte");
    assert!(matches!(count.grant(), Err(PackError::LimitExceeded(_))));
    while count.close_step(1) != crate::os_pack::format::RetainedPackCloseStep::Complete {}

    let mut utf8 = RetainedValueCursor::try_new(limits).expect("utf8");
    let mut invalid_utf8 = false;
    for (offset, byte) in [1, 0, TAG_STR_INLINE, 1, 0xff].into_iter().enumerate() {
        utf8.admit_byte(offset as u64, byte).expect("utf8 admission");
        while utf8.pending.is_some() {
            if utf8.grant().is_err() {
                invalid_utf8 = true;
                break;
            }
        }
    }
    assert!(invalid_utf8);
    while utf8.close_step(1) != crate::os_pack::format::RetainedPackCloseStep::Complete {}

    let depth_limits = PackLimits { max_file_len: 64, max_segment_len: 8, max_symbols: 1, max_depth: 1, max_items: 4, max_total_alloc: 64 };
    let mut depth = RetainedValueCursor::try_new(depth_limits).expect("depth");
    let mut depth_failed = false;
    for (offset, byte) in [1, 0, TAG_BLOCK, TAG_BLOCK, TAG_TRUE].into_iter().enumerate() {
        depth.admit_byte(offset as u64, byte).expect("depth admission");
        while depth.pending.is_some() {
            if matches!(depth.grant(), Err(PackError::LimitExceeded(_))) {
                depth_failed = true;
                break;
            }
        }
        if depth_failed {
            break;
        }
    }
    assert!(depth_failed);
    while depth.close_step(1) != crate::os_pack::format::RetainedPackCloseStep::Complete {}
}
//#endregion 🔖️RecordBody
