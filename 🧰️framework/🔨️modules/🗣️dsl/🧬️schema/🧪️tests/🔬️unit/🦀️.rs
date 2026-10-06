use semio_framework_diagnostic::Limits;
use semio_framework_dsl::TokenClass;
use super::*;
macro_rules! ordinary_fixture_spec {
    ($spec:path) => { crate::RecordSpecProducer { ordinary: $spec, decoding: |_| Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"ordinary-only test metadata has no controlled construction")), encoding: |_| Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"ordinary-only test metadata has no controlled construction")) } };
}


#[path = "../🫳️borrowed-object/🦀️.rs"]
mod borrowed_object_tests;

#[path = "../🔤️occurrences/🦀️.rs"]
mod intrinsic_occurrence_tests;

fn assert_round_trip(text: &str, spec: &RecordSpec) {
    let opts = ParseOptions::default();
    let value = parse(text, spec, &opts).unwrap_or_else(|e| panic!("parse failed for {text:?}: {e}"));
    let printed = print(&value, spec, JoinMode::Document);
    let reparsed = parse(&printed, spec, &opts).unwrap_or_else(|e| panic!("reparse of printed output failed: {e}\nprinted:\n{printed}"));
    assert_eq!(value, reparsed, "round trip diverged;\noriginal print:\n{printed}");
}

fn assert_document_inline_agree(text: &str, spec: &RecordSpec) {
    let doc_opts = ParseOptions { limits: Limits::default(), mode: SourceMode::Document };
    let value = parse(text, spec, &doc_opts).expect("parse document");
    let inline_text = print(&value, spec, JoinMode::Inline);
    assert!(!inline_text.contains('\n'), "inline render must be one line: {inline_text:?}");
    let inline_opts = ParseOptions { limits: Limits::default(), mode: SourceMode::Inline };
    let reparsed = parse(&inline_text, spec, &inline_opts).unwrap_or_else(|e| panic!("inline reparse failed: {e}\ninline:\n{inline_text}"));
    assert_eq!(value, reparsed, "Document and Inline renders must parse to the same value");
}

// --- primitive 1: record with typed scalar fields, order-independent key=value ---
fn camera_spec() -> RecordSpec {
    RecordSpec::new(Some("camera"), RecordLayout::Inline, vec![FieldSpec::new(0, "x", Shape::Float), FieldSpec::new(1, "y", Shape::Float), FieldSpec::new(2, "zoom", Shape::Float), FieldSpec::new(3, "label", Shape::Text).optional()])
}

#[semio_framework_async_macros::async_test]
async fn primitive_scalar_record_round_trips_and_is_order_independent() {
    let spec = camera_spec();
    assert_round_trip("camera x=1 y=2 zoom=3", &spec);
    assert_round_trip("camera zoom=3 x=1 y=2", &spec);
    assert_round_trip("camera x=-1.5 y=0 zoom=2.25 label=\"hi \\\"there\\\"\"", &spec);
    assert_document_inline_agree("camera x=1 y=2 zoom=3", &spec);
}

#[semio_framework_async_macros::async_test]
async fn primitive_optional_field_omits_on_print_and_absent_on_parse() {
    let spec = camera_spec();
    let value = parse("camera x=1 y=2 zoom=1", &spec, &ParseOptions::default()).expect("parse");
    assert_eq!(value.get(3), Some(&FieldValue::Absent));
    let printed = print(&value, &spec, JoinMode::Document);
    assert!(!printed.contains("label"), "optional absent field must be omitted: {printed}");
}

#[semio_framework_async_macros::async_test]
async fn exact_record_parse_rejects_tokens_outside_the_terminal_schema() {
    let spec = camera_spec();
    let options = ParseOptions::default();
    let wire = "camera x=1 y=2 zoom=3";
    let expected = parse(wire, &spec, &options).unwrap();
    assert_eq!(parse_exact(&format!(" \t{wire}\t "), &spec, &options).unwrap(), expected);
    for tail in ["unknown=4", "x=4", "camera x=4 y=5 zoom=6", "garbage"] {
        let composed = format!("{wire} {tail}");
        assert_eq!(parse(&composed, &spec, &options).unwrap(), expected);
        assert!(parse_exact(&composed, &spec, &options).is_err(), "{tail}");
    }
}

// --- primitive: embed — fenced verbatim text (Document) / escaped Text (Inline) ---
fn writer_note_spec() -> RecordSpec {
    RecordSpec::new(Some("query"), RecordLayout::Inline, vec![FieldSpec::new(0, "id", Shape::Text).positional(0), FieldSpec::new(1, "body", Shape::Embed("jack"))])
}

#[semio_framework_async_macros::async_test]
async fn embed_round_trips_multiline_fenced_content_in_document_mode() {
    let spec = writer_note_spec();
    assert_round_trip("query q1 body=```jack\nMATCH (a) RETURN a\nWHERE a.x > 1\n```", &spec);
}

#[semio_framework_async_macros::async_test]
async fn embed_document_and_inline_renders_agree() {
    let spec = writer_note_spec();
    assert_document_inline_agree("query q1 body=```jack\nMATCH (a) RETURN a\n```", &spec);
}

#[semio_framework_async_macros::async_test]
async fn embed_empty_lang_tag_is_accepted_and_canonicalizes_to_the_declared_lang() {
    let spec = writer_note_spec();
    let value = parse("query q1 body=```\nMATCH (a) RETURN a\n```", &spec, &ParseOptions::default()).expect("parse with empty lang tag");
    let printed = print(&value, &spec, JoinMode::Document);
    assert!(printed.contains("```jack"), "empty lang tag must canonicalize to the field's declared lang: {printed}");
}

#[semio_framework_async_macros::async_test]
async fn embed_rejects_a_mismatched_lang_tag() {
    let spec = writer_note_spec();
    let error = parse("query q1 body=```python\nprint(1)\n```", &spec, &ParseOptions::default()).unwrap_err();
    assert!(error.message.contains("jack"), "{error}");
}

#[semio_framework_async_macros::async_test]
async fn embed_inline_mode_accepts_a_quoted_escaped_string_directly() {
    let spec = writer_note_spec();
    let value = parse("query q1 body=\"MATCH (a) RETURN a\"", &spec, &ParseOptions { limits: Limits::default(), mode: SourceMode::Inline }).expect("inline parse");
    let FieldValue::Text(body) = value.get(1).expect("body field") else { panic!("expected Text") };
    assert_eq!(body, "MATCH (a) RETURN a");
}

#[semio_framework_async_macros::async_test]
async fn embed_empty_content_round_trips() {
    let spec = writer_note_spec();
    assert_round_trip("query q1 body=```jack\n```", &spec);
}

// --- primitive: expr — an arithmetic formula literal ---
fn formula_spec() -> RecordSpec {
    RecordSpec::new(Some("combine"), RecordLayout::Inline, vec![FieldSpec::new(0, "id", Shape::Text).positional(0), FieldSpec::new(1, "value", Shape::Expr)])
}

#[semio_framework_async_macros::async_test]
async fn expr_round_trips_a_load_combination_formula() {
    let spec = formula_spec();
    assert_round_trip("combine ULS value=(1.35*G + 1.5*Q)", &spec);
    assert_document_inline_agree("combine ULS value=(1.35*G + 1.5*Q)", &spec);
}

#[semio_framework_async_macros::async_test]
async fn expr_parses_with_correct_precedence() {
    let spec = formula_spec();
    let value = parse("combine c value=(10-2*3)", &spec, &ParseOptions::default()).expect("parse");
    let FieldValue::Expr(expr) = value.get(1).expect("value field") else { panic!("expected Expr") };
    assert_eq!(*expr, ExprValue::Binary(ExprOp::Sub, Box::new(ExprValue::Num(10.0)), Box::new(ExprValue::Binary(ExprOp::Mul, Box::new(ExprValue::Num(2.0)), Box::new(ExprValue::Num(3.0)))),), "10-2*3 must parse as 10-(2*3), not (10-2)*3");
}

#[semio_framework_async_macros::async_test]
async fn expr_right_nested_addition_round_trips_through_parens() {
    // a+(b+c) is structurally distinct from (a+b)+c; canonical print must keep the parens.
    let spec = formula_spec();
    let value = parse("combine c value=(a+(b+c))", &spec, &ParseOptions::default()).expect("parse");
    let printed = print(&value, &spec, JoinMode::Document);
    assert!(printed.contains("(b + c)"), "right-nested addition must keep disambiguating parens: {printed}");
    let reparsed = parse(&printed, &spec, &ParseOptions::default()).expect("reparse");
    assert_eq!(value, reparsed);
}

#[semio_framework_async_macros::async_test]
async fn expr_supports_unary_minus_and_function_calls() {
    let spec = formula_spec();
    assert_round_trip("combine c value=(min(a, b) + -1)", &spec);
}

#[semio_framework_async_macros::async_test]
async fn expr_glued_negative_number_after_operand_canonicalizes_to_spaced_subtraction() {
    // Hand-written "10-2" (no space) hits the lexer's negative-number-literal rule, not a bare
    // Minus token; the Expr parser must still interpret it as subtraction, and re-print it with
    // real spacing so the ambiguity never reappears in canonical output.
    let spec = formula_spec();
    let value = parse("combine c value=(10-2)", &spec, &ParseOptions::default()).expect("parse");
    let FieldValue::Expr(expr) = value.get(1).expect("value field") else { panic!("expected Expr") };
    assert_eq!(*expr, ExprValue::Binary(ExprOp::Sub, Box::new(ExprValue::Num(10.0)), Box::new(ExprValue::Num(2.0))));
    let printed = print(&value, &spec, JoinMode::Document);
    assert!(printed.contains("10 - 2"), "canonical print must space out the operator: {printed}");
}

// --- primitive: quantity/angle — a Shape::Float refinement that prints/parses a glued unit suffix ---
fn material_spec() -> RecordSpec {
    RecordSpec::new(
        Some("material"),
        RecordLayout::Inline,
        vec![
            FieldSpec::new(0, "e", Shape::Quantity(semio_framework_dsl::unit_by_symbol("GPa").unwrap())),
            FieldSpec::new(1, "rho", Shape::Quantity(semio_framework_dsl::unit_by_symbol("kg/m3").unwrap())),
            FieldSpec::new(2, "rotation", Shape::Angle(semio_framework_dsl::unit_by_symbol("deg").unwrap())),
        ],
    )
}

#[semio_framework_async_macros::async_test]
async fn quantity_and_angle_round_trip_in_their_declared_unit() {
    let spec = material_spec();
    assert_round_trip("material e=210GPa rho=7850kg/m3 rotation=45deg", &spec);
    assert_round_trip("material e=210GPa rho=7850kg/m3 rotation=30deg", &spec);
    assert_document_inline_agree("material e=210GPa rho=7850kg/m3 rotation=30deg", &spec);
}

#[semio_framework_async_macros::async_test]
async fn quantity_accepts_a_compatible_alien_unit_and_canonicalizes_to_the_declared_one() {
    let spec = material_spec();
    let value = parse("material e=210000MPa rho=7850kg/m3 rotation=45deg", &spec, &ParseOptions::default()).expect("parse alien unit");
    let printed = print(&value, &spec, JoinMode::Document);
    assert!(printed.contains("e=210GPa"), "alien-unit input must canonicalize to the declared unit: {printed}");
}

#[semio_framework_async_macros::async_test]
async fn quantity_with_no_suffix_is_already_in_the_declared_unit() {
    let spec = material_spec();
    let with_suffix = parse("material e=210GPa rho=7850kg/m3 rotation=45deg", &spec, &ParseOptions::default()).expect("parse with suffix");
    let bare = parse("material e=210 rho=7850kg/m3 rotation=45deg", &spec, &ParseOptions::default()).expect("parse bare number");
    assert_eq!(with_suffix.get(0), bare.get(0), "a bare number must equal the same value spelled with its declared unit's suffix");
}

// --- primitive: coord/dir/dim/range/count/ref — sigil-and-glyph notation literals ---
fn placement_spec() -> RecordSpec {
    RecordSpec::new(
        Some("object"),
        RecordLayout::Inline,
        vec![
            FieldSpec::new(0, "id", Shape::Text).positional(0),
            FieldSpec::new(1, "material", Shape::Ref("material")),
            FieldSpec::new(2, "position", Shape::Coord(3)),
            FieldSpec::new(3, "axis", Shape::Dir),
            FieldSpec::new(4, "size", Shape::Dim(3)),
            FieldSpec::new(5, "slider", Shape::Range),
            FieldSpec::new(6, "count", Shape::Count),
        ],
    )
}

#[semio_framework_async_macros::async_test]
async fn coord_dir_dim_range_count_ref_round_trip() {
    let spec = placement_spec();
    assert_round_trip("object col-a material=s355 position=@1.35,0,0 axis=^0,1,0 size=2.4x0.12x0.24 slider=(0..10,0.5) count=x24", &spec);
    assert_document_inline_agree("object col-a material=s355 position=@1.35,0,0 axis=^0,1,0 size=2.4x0.12x0.24 slider=(0..10,0.5) count=x24", &spec);
}

#[semio_framework_async_macros::async_test]
async fn range_without_step_round_trips_with_two_elements() {
    let spec = RecordSpec::new(Some("slot"), RecordLayout::Inline, vec![FieldSpec::new(0, "window", Shape::Range)]);
    assert_round_trip("slot window=(0..10)", &spec);
}

#[semio_framework_async_macros::async_test]
async fn coord_dir_dim_range_count_reject_wrong_arity_or_form() {
    let spec = placement_spec();
    // Coord declared as 3 components; only 2 given.
    let err = parse("object col-a material=s355 position=@1.35,0 axis=^0,1,0 size=2.4x0.12x0.24 slider=(0..10) count=x1", &spec, &ParseOptions::default()).unwrap_err();
    assert!(err.message.contains("coordinate") || err.message.contains("expected"), "{err}");
    // Dim declared as 3 components; only one number, no glued 'x' suffix at all.
    let err2 = parse("object col-a material=s355 position=@1,2,3 axis=^0,1,0 size=2.4 slider=(0..10) count=x1", &spec, &ParseOptions::default()).unwrap_err();
    assert!(err2.message.contains("dimension"), "{err2}");
    // Count without the 'x' prefix is not a valid count literal.
    let err3 = parse("object col-a material=s355 position=@1,2,3 axis=^0,1,0 size=2.4x0.12x0.24 slider=(0..10) count=24", &spec, &ParseOptions::default()).unwrap_err();
    assert!(err3.message.contains("count"), "{err3}");
}

#[semio_framework_async_macros::async_test]
async fn quantity_rejects_an_incompatible_unit() {
    let spec = material_spec();
    let error = parse("material e=210kg rho=7850kg/m3 rotation=45deg", &spec, &ParseOptions::default()).unwrap_err();
    assert!(error.message.contains("not compatible"), "wrong-dimension suffix must be a parse error, got: {error}");
}

// --- primitive 2 + 3: keyword-led statements, homogeneous ordered collection ---
// 🚫️async: E4 fn-pointer slot — passed by name into `Shape::Statements` (`fn() -> RecordSpec`,
// unnameable if async) — see R9/E4.
fn layer_variant_spec() -> RecordSpec {
    RecordSpec::new(Some("layer"), RecordLayout::Inline, vec![FieldSpec::new(0, "id", Shape::Text).positional(0), FieldSpec::new(1, "opacity", Shape::Float)])
}

fn document_with_layers_spec() -> RecordSpec {
    RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(0, "schema", Shape::Text), FieldSpec::new(1, "layers", Shape::Statements(vec![("layer".to_string(), ordinary_fixture_spec!(layer_variant_spec))]))])
}

#[semio_framework_async_macros::async_test]
async fn primitive_statements_collection_preserves_order_and_round_trips() {
    let spec = document_with_layers_spec();
    assert_round_trip("schema=doc layer a opacity=1 layer b opacity=0.5 layer c opacity=1", &spec);
    let value = parse("schema=doc layer a opacity=1 layer b opacity=0.5", &spec, &ParseOptions::default()).expect("parse");
    let FieldValue::Statements(items) = value.get(1).unwrap() else { panic!("expected statements") };
    assert_eq!(items.len(), 2);
    assert_eq!(items[0].0, "layer");
}

// --- primitive 4: recursive sub-blocks ---
/// 🌳️ Genuinely self-referential: `children`'s own variant table names `group_spec`
/// itself. Lazy `fn() -> RecordSpec` entries make this sound — `group_spec()` doesn't recurse
/// just to build the table, only `parse`/`print` calling the stored fn pointer one level at a
/// time (as deep as real input actually nests) ever evaluates it again.
// 🚫️async: E4 fn-pointer slot — stored bare as `fn() -> RecordSpec` in `Shape::Statements` above
fn group_spec() -> RecordSpec {
    RecordSpec::new(Some("group"), RecordLayout::Inline, vec![FieldSpec::new(0, "id", Shape::Text).positional(0), FieldSpec::new(1, "children", Shape::Block(Box::new(Shape::Statements(vec![("group".to_string(), ordinary_fixture_spec!(group_spec))])))).optional()])
}

#[semio_framework_async_macros::async_test]
async fn primitive_recursive_blocks_round_trip() {
    let spec = group_spec();
    assert_round_trip("group root children { group a group b }", &spec);
    assert_round_trip("group leaf", &spec);
}

// --- primitive 6/7: escaped inline text, formerly-trailing free text ---
#[semio_framework_async_macros::async_test]
async fn primitive_escaped_text_handles_quotes_newlines_and_trailing_position() {
    let spec = camera_spec();
    let value = parse("camera x=1 y=1 zoom=1 label=\"line1\\nline2 with \\\"quotes\\\"\"", &spec, &ParseOptions::default()).expect("parse");
    assert_eq!(value.get(3), Some(&FieldValue::Text("line1\nline2 with \"quotes\"".to_string())));
}

// --- primitive 9: graph endpoints (wire literal) ---
fn wire_spec() -> RecordSpec {
    RecordSpec::new(Some("edge"), RecordLayout::Inline, vec![FieldSpec::new(0, "link", Shape::Wire).positional(0)])
}

#[semio_framework_async_macros::async_test]
async fn primitive_wire_literal_directed_and_undirected_round_trip() {
    let spec = wire_spec();
    assert_round_trip("edge a:Kind@out->b:Kind2@in", &spec);
    assert_round_trip("edge a--b", &spec);
    assert_round_trip("edge solo", &spec);
    assert_round_trip("edge a -e1:Connection> b", &spec);
}

// --- RecordLayout::Call: `<name> = <keyword>(args)` construction-chain notation ---
fn call_spec() -> RecordSpec {
    RecordSpec::new(
        Some("brep.solid.extrude"),
        RecordLayout::Call,
        vec![FieldSpec::new(0, "name", Shape::Text).call_name(), FieldSpec::new(1, "profile", Shape::Text).positional(0), FieldSpec::new(2, "axis", Shape::Text).positional(1), FieldSpec::new(3, "height", Shape::Float).optional()],
    )
}

#[semio_framework_async_macros::async_test]
async fn call_layout_prints_name_equals_dotted_keyword_parens_args() {
    let spec = call_spec();
    let opts = ParseOptions::default();
    let value = parse("extrude = brep.solid.extrude(w1 v1 height=6)", &spec, &opts).expect("parse");
    assert_eq!(value.get(0), Some(&FieldValue::Text("extrude".to_string())));
    assert_eq!(value.get(1), Some(&FieldValue::Text("w1".to_string())));
    let printed = print(&value, &spec, JoinMode::Inline);
    assert_eq!(printed, "extrude = brep.solid.extrude(w1 v1 height=6)");
}

#[semio_framework_async_macros::async_test]
async fn call_layout_round_trips_with_and_without_the_optional_keyed_arg() {
    let spec = call_spec();
    assert_round_trip("extrude = brep.solid.extrude(w1 v1 height=6)", &spec);
    assert_round_trip("extrude = brep.solid.extrude(w1 v1)", &spec);
}

#[semio_framework_async_macros::async_test]
async fn call_layout_rejects_the_wrong_call_target() {
    let spec = call_spec();
    let opts = ParseOptions::default();
    let err = parse("extrude = brep.solid.revolve(w1 v1)", &spec, &opts).unwrap_err();
    assert!(err.to_string().contains("expected call target"), "unexpected error: {err}");
}

#[semio_framework_async_macros::async_test]
async fn call_layout_spec_without_a_call_name_field_is_a_clear_parse_error_not_a_panic() {
    let bad_spec = RecordSpec::new(Some("brep.solid.extrude"), RecordLayout::Call, vec![FieldSpec::new(0, "profile", Shape::Text).positional(0)]);
    let opts = ParseOptions::default();
    let err = parse("extrude = brep.solid.extrude(w1)", &bad_spec, &opts).unwrap_err();
    assert!(err.to_string().contains("call_name"), "unexpected error: {err}");
}

// --- primitive 10: packed tuples / lists / base64 ---
fn geometry_spec() -> RecordSpec {
    RecordSpec::new(
        Some("vertex"),
        RecordLayout::Inline,
        vec![FieldSpec::new(0, "pos", Shape::Tuple(Box::new(Shape::Float), Some(3))).positional(0), FieldSpec::new(1, "tags", Shape::List(Box::new(Shape::Text))).optional(), FieldSpec::new(2, "blob", Shape::Bytes64).optional()],
    )
}

#[semio_framework_async_macros::async_test]
async fn primitive_tuple_list_and_base64_round_trip() {
    let spec = geometry_spec();
    assert_round_trip("vertex 1,2,3", &spec);
    assert_round_trip("vertex 1,2,3 tags=[a b c]", &spec);
    assert_round_trip("vertex 0,0,0 blob=\"aGVsbG8=\"", &spec);
    let value = parse("vertex 0,0,0 blob=\"aGVsbG8=\"", &spec, &ParseOptions::default()).expect("parse");
    assert_eq!(value.get(2), Some(&FieldValue::Bytes64(b"hello".to_vec())));
}

// --- primitive 11: dynamic value literal ---
fn value_spec() -> RecordSpec {
    RecordSpec::new(Some("payload"), RecordLayout::Inline, vec![FieldSpec::new(0, "data", Shape::Value)])
}

#[semio_framework_async_macros::async_test]
async fn primitive_dynamic_value_round_trips() {
    let spec = value_spec();
    assert_round_trip("payload data={a=1 b=[1 2 3] c=\"x\"}", &spec);
    let value = parse("payload data={a=1}", &spec, &ParseOptions::default()).expect("parse");
    let FieldValue::Value(dsl_value) = value.get(0).unwrap().clone() else { panic!() };
    assert_eq!(dsl_value.get("a"), Some(&DslValue::uint(1)));
}

// --- primitive 12: sparse patch records (Option<T> absent != null) ---
#[semio_framework_async_macros::async_test]
async fn primitive_sparse_patch_distinguishes_absent_from_present() {
    let spec = camera_spec();
    let with = parse("camera x=1 y=1 zoom=1 label=\"x\"", &spec, &ParseOptions::default()).expect("parse with");
    let without = parse("camera x=1 y=1 zoom=1", &spec, &ParseOptions::default()).expect("parse without");
    assert_ne!(with.get(3), without.get(3));
    assert_eq!(without.get(3), Some(&FieldValue::Absent));
}

// --- primitive 15: comments ---
#[semio_framework_async_macros::async_test]
async fn primitive_comments_are_skipped_as_trivia() {
    let spec = camera_spec();
    let value = parse("# a comment\ncamera x=1 y=2 zoom=3 # trailing comment", &spec, &ParseOptions::default()).expect("parse with comments");
    assert_eq!(value.get(0), Some(&FieldValue::Float(1.0)));
}

// --- primitive 16: real spans ---
#[semio_framework_async_macros::async_test]
async fn primitive_spans_are_real_on_parse_error() {
    let spec = camera_spec();
    let error = parse("camera x=1\ny=notanumber zoom=1", &spec, &ParseOptions::default()).unwrap_err();
    assert_eq!(error.span.line, 2, "error span must point at the real line, not (1,1)");
}

// --- bare-string printing: `is_bare_ident` values print unquoted, reserved/number-shaped/
// multi-word values stay quoted (unified syntax law: strings bare-preferred) ---
#[semio_framework_async_macros::async_test]
async fn bare_strings_print_unquoted_and_reserved_or_number_shaped_values_stay_quoted() {
    let spec = camera_spec();
    let value = parse("camera x=1 y=2 zoom=3 label=alpha", &spec, &ParseOptions::default()).expect("parse");
    let printed = print(&value, &spec, JoinMode::Document);
    assert!(printed.contains("label=alpha"), "a bare-ident-shaped value must print unquoted: {printed}");
    assert!(!printed.contains("\"alpha\""), "must not quote a value that already lexes as a bare ident: {printed}");

    for reserved in ["_", "true", "3", "two words"] {
        let mut writer = Writer::new();
        print_shape(&FieldValue::Text(reserved.to_string()), &Shape::Text, &mut writer);
        let out = writer.render(JoinMode::Inline);
        assert!(out.starts_with('"') && out.ends_with('"'), "{reserved:?} must print quoted, got {out:?}");
    }
}

// --- `Writer::glue()`: exact-string spacing assertions for every composite shape's
// `key=value` fusion (the "key= value" bug this replaces) ---
// 🚫️async: E4 fn-pointer slot — stored bare as `fn() -> RecordSpec` via `Shape::Record` below
fn nested_point_spec() -> RecordSpec {
    RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(0, "x", Shape::Float), FieldSpec::new(1, "y", Shape::Float)])
}
fn marker_spec() -> RecordSpec {
    RecordSpec::new(Some("marker"), RecordLayout::Inline, vec![FieldSpec::new(0, "at", Shape::Record(ordinary_fixture_spec!(nested_point_spec)))])
}
fn edge_keyed_wire_spec() -> RecordSpec {
    RecordSpec::new(Some("edge2"), RecordLayout::Inline, vec![FieldSpec::new(0, "link", Shape::Wire)])
}
fn tags_map_spec() -> RecordSpec {
    RecordSpec::new(Some("meta"), RecordLayout::Inline, vec![FieldSpec::new(0, "props", Shape::Map(Box::new(Shape::Text)))])
}

#[semio_framework_async_macros::async_test]
async fn glue_removes_the_key_equals_space_for_every_composite_shape() {
    // List
    let spec = geometry_spec();
    let value = parse("vertex 1,2,3 tags=[a b c]", &spec, &ParseOptions::default()).expect("parse list");
    let printed = print(&value, &spec, JoinMode::Document);
    assert!(printed.contains("tags=[ a b c ]"), "List field must glue key= directly onto '[': {printed}");
    assert!(!printed.contains("tags= ["), "must never leave a stray space after 'key=': {printed}");

    // Value (dynamic)
    let spec = value_spec();
    let value = parse("payload data={a=1}", &spec, &ParseOptions::default()).expect("parse value");
    let printed = print(&value, &spec, JoinMode::Document);
    assert!(printed.contains("data={"), "Value field must glue key= directly onto '{{': {printed}");
    assert!(!printed.contains("data= {"), "must never leave a stray space before the glued brace: {printed}");

    // Map
    let spec = tags_map_spec();
    let value = parse("meta props={a=\"x\" b=\"y\"}", &spec, &ParseOptions::default()).expect("parse map");
    let printed = print(&value, &spec, JoinMode::Document);
    assert!(printed.contains("props={"), "Map field must glue key= directly onto '{{': {printed}");
    assert_round_trip("meta props={a=\"x\" b=\"y\"}", &spec);

    // Record (nested, un-blocked — prints inline without its own keyword)
    let spec = marker_spec();
    let value = parse("marker at=x=1 y=2", &spec, &ParseOptions::default()).expect("parse record");
    let printed = print(&value, &spec, JoinMode::Document);
    assert!(printed.contains("at=x=1"), "Record field must glue key= directly onto its first field: {printed}");
    assert!(!printed.contains("at= x=1"), "must never leave a stray space before a nested record: {printed}");

    // Wire (keyed, not positional)
    let spec = edge_keyed_wire_spec();
    let value = parse("edge2 link=a->b", &spec, &ParseOptions::default()).expect("parse wire");
    let printed = print(&value, &spec, JoinMode::Document);
    assert!(printed.contains("link=a->b"), "Wire field must glue key= directly onto the wire literal: {printed}");
    assert!(!printed.contains("link= a"), "must never leave a stray space before a keyed wire literal: {printed}");
}

// --- wire `<-` normalization: accepted sugar only, always stored/printed as `->` with
// endpoints swapped ---
#[semio_framework_async_macros::async_test]
async fn wire_back_arrow_normalizes_to_forward_arrow_with_swapped_endpoints() {
    let spec = wire_spec();
    let backward = parse("edge b<-a", &spec, &ParseOptions::default()).expect("parse backward");
    let forward = parse("edge a->b", &spec, &ParseOptions::default()).expect("parse forward");
    assert_eq!(backward, forward, "'b<-a' must parse to the same value as 'a->b'");
    let printed = print(&backward, &spec, JoinMode::Document);
    assert!(printed.contains("a->b"), "must print using '->': {printed}");
    assert!(!printed.contains("<-"), "must never print '<-': {printed}");
}

#[semio_framework_async_macros::async_test]
async fn parse_wire_text_parses_a_standalone_wire_literal_with_back_arrow() {
    let value = parse_wire_text("b<-a").expect("parse_wire_text");
    assert_eq!(value.from.id, "a");
    let (directed, to) = value.edge.expect("edge");
    assert!(directed);
    assert_eq!(to.id, "b");
}

// --- primitive 17: `Shape::Table` — SoA columnar collection ---
// 🚫️async: E4 fn-pointer slot — stored bare as `fn() -> RecordSpec` via `Shape::Table` below
fn table_row_spec() -> RecordSpec {
    RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(0, "id", Shape::Text), FieldSpec::new(1, "x", Shape::Float), FieldSpec::new(2, "y", Shape::Float), FieldSpec::new(3, "link", Shape::Wire).optional()])
}
fn table_doc_spec() -> RecordSpec {
    RecordSpec::new(Some("scene"), RecordLayout::Inline, vec![FieldSpec::new(0, "nodes", Shape::Table(ordinary_fixture_spec!(table_row_spec)))])
}

#[semio_framework_async_macros::async_test]
async fn table_soa_round_trips_with_underscore_absent_cell_and_a_wire_column() {
    let spec = table_doc_spec();
    let text = "scene nodes [id:TEXT x:NUM y:NUM link:WIRE] { a 1 2 _  b 3 4 a@out->b@in }";
    assert_round_trip(text, &spec);
    let value = parse(text, &spec, &ParseOptions::default()).expect("parse");
    let FieldValue::List(rows) = value.get(0).unwrap() else { panic!("expected a table (List) value") };
    assert_eq!(rows.len(), 2);
    let FieldValue::Record(row0) = &rows[0] else { panic!("expected a Record row") };
    assert_eq!(row0.get(3), Some(&FieldValue::Absent), "the '_' cell must parse as Absent");
    let FieldValue::Record(row1) = &rows[1] else { panic!("expected a Record row") };
    assert!(matches!(row1.get(3), Some(FieldValue::Wire(_))), "the wire-typed column must parse as FieldValue::Wire");

    let printed = print(&value, &spec, JoinMode::Document);
    assert!(printed.contains("nodes [id:TEXT x:NUM y:NUM link:WIRE]"), "header must print tight SoA, no inner spaces: {printed}");
}

#[semio_framework_async_macros::async_test]
async fn table_accepts_verbose_aos_input_and_canonicalizes_to_soa_output() {
    let spec = table_doc_spec();
    let aos_text = "scene nodes=[ {id=a x=1 y=2} {id=b x=3 y=4} ]";
    let value = parse(aos_text, &spec, &ParseOptions::default()).expect("parse AoS-verbose");
    let printed = print(&value, &spec, JoinMode::Document);
    assert!(printed.contains("nodes [id:TEXT x:NUM y:NUM link:WIRE]"), "AoS input must canonicalize to the SoA header on print: {printed}");
    assert!(!printed.contains("nodes="), "must never print the old AoS '=' form: {printed}");
    let reparsed = parse(&printed, &spec, &ParseOptions::default()).expect("reparse canonicalized SoA");
    assert_eq!(value, reparsed, "AoS-in/SoA-out must still round trip to the same value");
}

#[semio_framework_async_macros::async_test]
async fn table_header_without_explicit_type_tags_is_still_parseable() {
    let spec = table_doc_spec();
    let text = "scene nodes [id x y link] { a 1 2 _  b 3 4 a@out->b@in }";
    let value = parse(text, &spec, &ParseOptions::default()).expect("parse header without explicit types");
    let FieldValue::List(rows) = value.get(0).unwrap() else { panic!("expected a table (List) value") };
    assert_eq!(rows.len(), 2);
}

#[semio_framework_async_macros::async_test]
async fn table_document_and_inline_renders_agree() {
    let spec = table_doc_spec();
    assert_document_inline_agree("scene nodes [id:TEXT x:NUM y:NUM link:WIRE] { a 1 2 _  b 3 4 a@out->b@in }", &spec);
}

#[semio_framework_async_macros::async_test]
async fn table_rejects_non_self_delimiting_column_shapes_at_spec_build_time() {
    // 🚫️async: E4 fn-pointer slot — stored bare as `fn() -> RecordSpec` via `Shape::Table` below
    fn unbounded_tuple_row_spec() -> RecordSpec {
        RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(0, "vals", Shape::Tuple(Box::new(Shape::Float), None))])
    }
    fn bad_table_doc_spec() -> RecordSpec {
        RecordSpec::new(Some("bad"), RecordLayout::Inline, vec![FieldSpec::new(0, "rows", Shape::Table(ordinary_fixture_spec!(unbounded_tuple_row_spec)))])
    }
    let spec = bad_table_doc_spec();
    let result = parse("bad rows [vals:TUPLE] { 1,2,3 }", &spec, &ParseOptions::default());
    assert!(result.is_err(), "an unbounded Tuple column must be rejected, not silently accepted");
}

// --- regression: a table row whose own field is ITSELF a `#[dsl(table)]` (nested SoA output
// used to break the parser's row-boundary counting; see `print_table_list`/`parse_table_list`) ---
// 🚫️async: E4 fn-pointer slot — stored bare as `fn() -> RecordSpec` via `Shape::Table` below
fn nested_inner_row_spec() -> RecordSpec {
    RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(0, "id", Shape::Text), FieldSpec::new(1, "val", Shape::Float).optional()])
}
// 🚫️async: E4 fn-pointer slot — stored bare as `fn() -> RecordSpec` via `Shape::Table` below
fn nested_outer_row_spec() -> RecordSpec {
    RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(0, "id", Shape::Text), FieldSpec::new(1, "children", Shape::Table(ordinary_fixture_spec!(nested_inner_row_spec)))])
}
fn nested_table_doc_spec() -> RecordSpec {
    RecordSpec::new(Some("doc"), RecordLayout::Inline, vec![FieldSpec::new(0, "items", Shape::Table(ordinary_fixture_spec!(nested_outer_row_spec)))])
}

#[semio_framework_async_macros::async_test]
async fn table_row_containing_its_own_table_field_round_trips_without_desync() {
    let spec = nested_table_doc_spec();
    let text = "doc items [id:TEXT children:TABLE] { p1 [ {id=c1 val=1.5} {id=c2} ]  p2 [ {id=c3 val=2} ] }";
    assert_round_trip(text, &spec);
    assert_document_inline_agree(text, &spec);
    let value = parse(text, &spec, &ParseOptions::default()).expect("parse nested table");
    let FieldValue::List(outer_rows) = value.get(0).unwrap() else { panic!("expected outer table (List)") };
    assert_eq!(outer_rows.len(), 2);
    let FieldValue::Record(row0) = &outer_rows[0] else { panic!("expected outer Record row") };
    let FieldValue::List(inner_rows) = row0.get(1).unwrap() else { panic!("expected nested table (List)") };
    assert_eq!(inner_rows.len(), 2, "the inner table's own row count must not desync from its header");
    let FieldValue::Record(inner_row1) = &inner_rows[1] else { panic!("expected inner Record row") };
    assert_eq!(inner_row1.get(1), Some(&FieldValue::Absent), "the second inner row's absent 'val' must round trip as Absent, not corrupt later parsing");
}

// --- regression: a table row with 2+ columns of the exact same nested `DslRecord` type (the
// greedy same-key consumption bug: an unset field on column N used to silently eat a later
// column's same-named present value; see `print_table_cell`/`parse_table_cell`) ---
// 🚫️async: E4 fn-pointer slot — stored bare as `fn() -> RecordSpec` via `Shape::Record` below
fn quantity_spec() -> RecordSpec {
    RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(0, "target", Shape::Float).optional(), FieldSpec::new(1, "actual", Shape::Float).optional()])
}
// 🚫️async: E4 fn-pointer slot — stored bare as `fn() -> RecordSpec` via `Shape::Table` below
fn duplicate_type_row_spec() -> RecordSpec {
    RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(0, "id", Shape::Text), FieldSpec::new(1, "area", Shape::Record(ordinary_fixture_spec!(quantity_spec))), FieldSpec::new(2, "volume", Shape::Record(ordinary_fixture_spec!(quantity_spec)))])
}
fn duplicate_type_table_doc_spec() -> RecordSpec {
    RecordSpec::new(Some("doc"), RecordLayout::Inline, vec![FieldSpec::new(0, "rows", Shape::Table(ordinary_fixture_spec!(duplicate_type_row_spec)))])
}

#[semio_framework_async_macros::async_test]
async fn table_row_with_two_columns_of_the_same_record_type_does_not_cross_contaminate() {
    let spec = duplicate_type_table_doc_spec();
    // `area`'s `target` is left absent (never printed) while `volume`'s `target=4` is present
    // right after it — the exact shape of the reported corruption, since both columns share
    // the identical field name.
    let text = "doc rows [id:TEXT area:REC volume:REC] { r1 {actual=3} {target=4 actual=1} }";
    assert_round_trip(text, &spec);
    assert_document_inline_agree(text, &spec);
    let value = parse(text, &spec, &ParseOptions::default()).expect("parse duplicate-type columns");
    let FieldValue::List(rows) = value.get(0).unwrap() else { panic!("expected table (List)") };
    let FieldValue::Record(row0) = &rows[0] else { panic!("expected Record row") };
    let FieldValue::Record(area) = row0.get(1).unwrap() else { panic!("expected area Record") };
    let FieldValue::Record(volume) = row0.get(2).unwrap() else { panic!("expected volume Record") };
    assert_eq!(area.get(0), Some(&FieldValue::Absent), "area.target must stay absent, not stolen from volume's column");
    assert_eq!(area.get(1), Some(&FieldValue::Float(3.0)), "area.actual must be area's own value");
    assert_eq!(volume.get(0), Some(&FieldValue::Float(4.0)), "volume.target must not be consumed by area's parse");
    assert_eq!(volume.get(1), Some(&FieldValue::Float(1.0)), "volume.actual must be volume's own value");
}

// --- idempotent canonicalization ---
#[semio_framework_async_macros::async_test]
async fn canonicalization_is_idempotent() {
    let spec = camera_spec();
    let once = canonicalize("camera   zoom=3   x=1 y=2", &spec, &ParseOptions::default()).expect("canonicalize once");
    let twice = canonicalize(&once, &spec, &ParseOptions::default()).expect("canonicalize twice");
    assert_eq!(once, twice, "canonicalize(canonicalize(x)) must equal canonicalize(x)");
}

// --- limits enforced, not panicking ---

#[semio_framework_async_macros::async_test]
async fn deeply_nested_blocks_hit_the_depth_limit_as_a_diagnostic() {
    // `group_spec()` (primitive 4, above) is already genuinely self-referential, so it needs no
    // pre-unrolling to exercise real depth this many levels deep — `parse` only ever expands one
    // level of its lazy `Statements` fn pointer at a time, following the actual input text.
    let levels = 20;
    let spec = group_spec();
    let mut nested = String::from("group root");
    for _ in 0..levels {
        nested.push_str(" children { group a");
    }
    for _ in 0..levels {
        nested.push('}');
    }
    let tiny_limits = Limits { max_depth: 10, ..Limits::default() };
    let opts = ParseOptions { limits: tiny_limits, mode: SourceMode::Document };
    let result = parse(&nested, &spec, &opts);
    assert!(result.is_err(), "exceeding max_depth must produce an error, not a stack overflow");

    let generous_limits = Limits { max_depth: 100, ..Limits::default() };
    let generous_opts = ParseOptions { limits: generous_limits, mode: SourceMode::Document };
    assert!(parse(&nested, &spec, &generous_opts).is_ok(), "the same nesting must parse fine under a generous depth limit");
}

// --- LanguageService ---
#[semio_framework_async_macros::async_test]
async fn language_service_reports_semantic_tokens_and_diagnostics() {
    let spec = camera_spec();
    let service = LanguageService::new(&spec);
    let classes = service.semantic_tokens("camera x=1 y=2 zoom=3");
    assert!(classes.iter().any(|(class, _)| *class == TokenClass::Keyword));
    assert!(service.diagnostics("camera x=1 y=2 zoom=3").is_empty());
    assert!(!service.diagnostics("camera x=notanumber").is_empty());
}

#[semio_framework_async_macros::async_test]
async fn language_service_completions_include_every_declared_key() {
    let spec = camera_spec();
    let service = LanguageService::new(&spec);
    let labels: Vec<String> = service.completions("", 0).into_iter().map(|c| c.label).collect();
    assert!(labels.contains(&"x".to_string()));
    assert!(labels.contains(&"zoom".to_string()));
    assert!(labels.contains(&"label".to_string()));
}

// --- 10k-iteration generative round trip over the flat-scalar shape ---
#[semio_framework_async_macros::async_test]
async fn generative_round_trip_over_scalar_records() {
    let spec = camera_spec();
    let mut state: u64 = 0xD1B54A32D192ED03;
    let mut next = || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    for _ in 0..2_000 {
        let x = (next() % 2000) as i64 - 1000;
        let y = (next() % 2000) as i64 - 1000;
        let zoom = (next() % 2000) as i64 - 1000;
        let text = format!("camera x={x} y={y} zoom={zoom}");
        assert_round_trip(&text, &spec);
    }
}

#[test]
fn retained_record_writer_resumes_physical_text_under_exact_work_and_retirement_grants() {
    use semio_framework_value::{NativeEncodeControl,retirement::owned_retirement};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🧵️continuation/🔣️.json")).unwrap();let label=fixture["textUnit"].as_str().unwrap().repeat(fixture["textRepeats"].as_u64().unwrap()as usize);let expected=format!("document label={} null=true value=int64(1)\n",serde_json::to_string(&label).unwrap());
    let spec=||RecordSpec::new(Some("document"),RecordLayout::Inline,vec![FieldSpec::new(0,"label",Shape::Text),FieldSpec::new(1,"null",Shape::Bool),FieldSpec::new(2,"value",Shape::Value)]);let source=||RecordValue{fields:[(0,FieldValue::Text(label.clone())),(1,FieldValue::Bool(true)),(2,FieldValue::Value(DslValue::Number(Number::Int(1))))].into_iter().collect()};
    let retire=|writer:RetainedRecordWriter|{let mut retirement=owned_retirement(writer);assert!(matches!(retirement.close_step(0,0).unwrap(),semio_framework_value::SnapshotRetirementStep::Pending {released_items:0,released_bytes:0}));let mut turns=0;while !retirement.terminal_is_empty(){turns+=1;assert!(turns<100000);if let semio_framework_value::SnapshotRetirementStep::Pending {released_bytes,..}=retirement.close_step(1,3).unwrap(){assert!(released_bytes<=3);}}};
    for budget in fixture["budgets"].as_array().unwrap(){let original=source();assert_eq!(print(&original,&spec(),JoinMode::Document),expected);let mut writer=RetainedRecordWriter::new(original,spec(),JoinMode::Document,fixture["maximumBytes"].as_u64().unwrap()as usize);let mut accepted=|_|true;let mut control=NativeEncodeControl::new(1000000,&mut accepted);let mut turns=0;let text=loop {let before=writer.progress();assert!(writer.step(0,&mut control).unwrap().is_none());assert_eq!(writer.progress(),before);turns+=1;assert!(turns<100000);if let Some(text)=writer.step(budget.as_u64().unwrap()as usize,&mut control).unwrap(){break text;}};assert!(turns>1);assert_eq!(text,expected);assert_eq!(parse_exact(&text,&spec(),&ParseOptions::default()).unwrap(),source());retire(writer);}
    for stop in fixture["cancelUnits"].as_array().unwrap(){let mut writer=RetainedRecordWriter::new(source(),spec(),JoinMode::Document,1000000);let canceled=std::cell::Cell::new(false);let mut callback=|_|!canceled.get();let mut control=NativeEncodeControl::new(1000000,&mut callback);for _ in 0..stop.as_u64().unwrap(){assert!(writer.step(1,&mut control).unwrap().is_none());}let before=writer.progress();canceled.set(true);assert_eq!(writer.step(1,&mut control).unwrap_err().kind,ValueRefusalKind::Canceled);assert_eq!(writer.progress(),before);retire(writer);}
    eprintln!("[DEBUG] same record writer budgets1/8/256; independent serde scalar text; static null key; signed-positive intrinsic; bounded retirement and cancel");
}

#[test]
fn retained_record_writer_intrinsic_containers_sort_resume_and_retire_the_same_source() {
    use semio_framework_value::{NativeEncodeControl,retirement::owned_retirement};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🧵️continuation/🔣️.json")).unwrap();
    let intrinsic=&fixture["intrinsic"];let prefix=intrinsic["keyPrefix"].as_str().unwrap().repeat(intrinsic["keyRepeats"].as_u64().unwrap()as usize);let label=fixture["textUnit"].as_str().unwrap().repeat(intrinsic["textRepeats"].as_u64().unwrap()as usize);let bytes=intrinsic["bytes"].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap()as u8).collect::<Vec<_>>();
    fn neutral(value:&serde_json::Value)->DslValue{match value{serde_json::Value::Null=>DslValue::Null,serde_json::Value::Bool(value)=>DslValue::Bool(*value),serde_json::Value::Number(value)=>DslValue::Number(Number::UInt(value.as_u64().unwrap())),serde_json::Value::String(value)=>DslValue::String(value.clone()),serde_json::Value::Array(items)=>DslValue::Array(items.iter().map(neutral).collect()),serde_json::Value::Object(items)=>DslValue::Object(items.iter().map(|(key,value)|(key.clone(),neutral(value))).collect())}}
    let source=||RecordValue{fields:[(0,FieldValue::Value(DslValue::Object(vec![(format!("{prefix}z"),DslValue::Array(vec![DslValue::Bool(false),DslValue::Null,DslValue::Number(Number::UInt(3))])),(format!("{prefix}a"),DslValue::Object(vec![("z".into(),DslValue::Bytes(bytes.clone())),("a".into(),DslValue::String(label.clone()))])),("cases".into(),neutral(&intrinsic["cases"]))])))].into_iter().collect()};
    let spec=||RecordSpec::new(Some("document"),RecordLayout::Inline,vec![FieldSpec::new(0,"value",Shape::Value)]);
    let ordinary=print(&source(),&spec(),JoinMode::Inline);assert!(ordinary.contains(&serde_json::to_string(&label).unwrap()));
    {use std::{io::Write,process::{Command,Stdio}};let mut child=Command::new("bun").args(["-e","const x=JSON.parse(await Bun.stdin.text());if(!x.text.includes('bytes64(\"'+Buffer.from(x.bytes).toString('base64')+'\")'))throw Error('byte projection');"]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(serde_json::json!({"text":ordinary,"bytes":bytes}).to_string().as_bytes()).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));}
    for budget in fixture["budgets"].as_array().unwrap(){let mut writer=RetainedRecordWriter::new(source(),spec(),JoinMode::Inline,1000000);let accepted=std::cell::Cell::new(true);let mut callback=|_|accepted.get();let mut control=NativeEncodeControl::new(1000000,&mut callback);let mut units=0;let text=loop{let before=writer.progress();assert!(writer.step(0,&mut control).unwrap().is_none());assert_eq!(writer.progress(),before);units+=1;assert!(units<2000000);if let Some(text)=writer.step(budget.as_u64().unwrap()as usize,&mut control).unwrap(){break text}};assert!(units>1);assert_eq!(text,ordinary);let parsed=parse_exact(&text,&spec(),&ParseOptions{mode:SourceMode::Inline,..Default::default()}).unwrap();assert_eq!(print(&parsed,&spec(),JoinMode::Inline),ordinary);let mut retirement=owned_retirement(writer);while !retirement.terminal_is_empty(){if let semio_framework_value::SnapshotRetirementStep::Pending{released_bytes,..}=retirement.close_step(1,3).unwrap(){assert!(released_bytes<=3)}}}
    for stop in [1,64,512,4096]{let mut writer=RetainedRecordWriter::new(source(),spec(),JoinMode::Inline,1000000);let accepted=std::cell::Cell::new(true);let mut callback=|_|accepted.get();let mut control=NativeEncodeControl::new(1000000,&mut callback);for _ in 0..stop{assert!(writer.step(1,&mut control).unwrap().is_none())}accepted.set(false);assert_eq!(writer.step(1,&mut control).unwrap_err().kind,ValueRefusalKind::Canceled);let mut retirement=owned_retirement(writer);while !retirement.terminal_is_empty(){if let semio_framework_value::SnapshotRetirementStep::Pending{released_bytes,..}=retirement.close_step(1,3).unwrap(){assert!(released_bytes<=3)}}}
    eprintln!("[DEBUG] same retained intrinsic owner sorted long keys, nested arrays/objects, Unicode/base64, budgets1/8/256 and cancel/retirement");
}

#[test]
fn retained_record_writer_nested_blocks_and_statements_keep_one_physical_owner() {
    use semio_framework_value::{NativeEncodeControl,NativeDecodeControl,retirement::owned_retirement};
    fn row()->RecordSpec{RecordSpec::new(Some("widget"),RecordLayout::Inline,vec![FieldSpec::new(0,"id",Shape::Text).positional(0),FieldSpec::new(1,"label",Shape::Text),FieldSpec::new(2,"value",Shape::Value)])}
    fn row_encode(control:&mut NativeEncodeControl<'_>)->Result<RecordSpec,ValueError>{let mut fields=control.allocate_vec(3)?;fields.push(crate::producer::field(0,"id",Shape::Text,control)?.positional(0));fields.push(crate::producer::field(1,"label",Shape::Text,control)?);fields.push(crate::producer::field(2,"value",Shape::Value,control)?);crate::producer::record(Some("widget"),RecordLayout::Inline,fields,control)}
    fn row_decode(_: &mut NativeDecodeControl<'_>)->Result<RecordSpec,ValueError>{Err(ValueError::new(ValueRefusalKind::UnsupportedOwner,"encoding fixture"))}
    let producer=RecordSpecProducer{ordinary:row,encoding:row_encode,decoding:row_decode};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🧵️continuation/🔣️.json")).unwrap();let count=fixture["nested"]["rows"].as_u64().unwrap()as usize;let prefix=fixture["nested"]["labelPrefix"].as_str().unwrap();
    let record=|index:usize|RecordValue{fields:[(0,FieldValue::Text(format!("w{index}"))),(1,FieldValue::Text(format!("{prefix}{index}"))),(2,FieldValue::Value(DslValue::Array(vec![DslValue::String(format!("{prefix}{index}")),DslValue::Number(Number::Int(index as i64))])))].into_iter().collect()};
    let source=||RecordValue{fields:[(0,FieldValue::Block(Box::new(FieldValue::Record(record(0))))),(1,FieldValue::Block(Box::new(FieldValue::Statements((0..count).map(|index|("widget".to_string(),record(index))).collect()))))].into_iter().collect()};
    let spec=||RecordSpec::new(Some("document"),RecordLayout::Inline,vec![FieldSpec::new(0,"camera",Shape::Block(Box::new(Shape::Record(producer)))),FieldSpec::new(1,"widgets",Shape::Block(Box::new(Shape::Statements(vec![("widget".into(),producer)]))))]);
    for mode in [JoinMode::Document,JoinMode::Inline]{let expected=print(&source(),&spec(),mode);assert!(expected.contains(&serde_json::to_string(&format!("{prefix}0")).unwrap()));for budget in fixture["budgets"].as_array().unwrap(){let mut writer=RetainedRecordWriter::new(source(),spec(),mode,1000000);let accepted=std::cell::Cell::new(true);let mut callback=|_|accepted.get();let mut control=NativeEncodeControl::new(1000000,&mut callback);let mut turns=0;let text=loop{assert!(writer.step(0,&mut control).unwrap().is_none());turns+=1;assert!(turns<1000000);if let Some(text)=writer.step(budget.as_u64().unwrap()as usize,&mut control).unwrap(){break text}};assert_eq!(text,expected);assert_eq!(parse_exact(&text,&spec(),&ParseOptions{mode:if mode==JoinMode::Document{SourceMode::Document}else{SourceMode::Inline},..Default::default()}).unwrap(),source());let mut close=owned_retirement(writer);while !close.terminal_is_empty(){if let semio_framework_value::SnapshotRetirementStep::Pending{released_bytes,..}=close.close_step(1,3).unwrap(){assert!(released_bytes<=3)}}}}
    for stop in [1,64,1024,4096]{let mut writer=RetainedRecordWriter::new(source(),spec(),JoinMode::Document,1000000);let accepted=std::cell::Cell::new(true);let mut callback=|_|accepted.get();let mut control=NativeEncodeControl::new(1000000,&mut callback);for _ in 0..stop{assert!(writer.step(1,&mut control).unwrap().is_none())}accepted.set(false);assert_eq!(writer.step(1,&mut control).unwrap_err().kind,ValueRefusalKind::Canceled);let mut close=owned_retirement(writer);while !close.terminal_is_empty(){if let semio_framework_value::SnapshotRetirementStep::Pending{released_bytes,..}=close.close_step(1,3).unwrap(){assert!(released_bytes<=3)}}}
    eprintln!("[DEBUG] nested record physical ownership Document/Inline, blocks/statements, budgets1/8/256, cancel and three-byte retirement");
}

#[test]
fn retained_declared_table_streams_columns_and_rows_under_one_source_owner() {
    use semio_framework_value::{NativeEncodeControl,NativeDecodeControl,retirement::owned_retirement};
    fn row()->RecordSpec{RecordSpec::new(Some("ignored-row-keyword"),RecordLayout::Inline,vec![FieldSpec::new(0,"id",Shape::Text),FieldSpec::new(1,"cost",Shape::Float),FieldSpec::new(2,"label",Shape::Text),FieldSpec::new(3,"value",Shape::Value)])}
    fn encode(control:&mut NativeEncodeControl<'_>)->Result<RecordSpec,ValueError>{let mut fields=control.allocate_vec(4)?;for (id,key,shape) in [(0,"id",Shape::Text),(1,"cost",Shape::Float),(2,"label",Shape::Text),(3,"value",Shape::Value)]{fields.push(crate::producer::field(id,key,shape,control)?);}crate::producer::record(Some("ignored-row-keyword"),RecordLayout::Inline,fields,control)}
    fn decode(_: &mut NativeDecodeControl<'_>)->Result<RecordSpec,ValueError>{Err(ValueError::new(ValueRefusalKind::UnsupportedOwner,"encoding fixture"))}
    let producer=RecordSpecProducer{ordinary:row,encoding:encode,decoding:decode};let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🧵️continuation/🔣️.json")).unwrap();let count=fixture["table"]["rows"].as_u64().unwrap()as usize;let prefix=fixture["table"]["labelPrefix"].as_str().unwrap();
    let source=||RecordValue{fields:[(0,FieldValue::List((0..count).map(|index|FieldValue::Record(RecordValue{fields:[(0,FieldValue::Text(format!("r{index}"))),(1,FieldValue::Float(index as f64+0.5)),(2,if index%3==0{FieldValue::Absent}else{FieldValue::Text(format!("{prefix}{index}"))}),(3,FieldValue::Value(DslValue::Array(vec![DslValue::Number(Number::Int(index as i64)),DslValue::Bool(index%2==0)])))].into_iter().collect()})).collect()))].into_iter().collect()};let spec=||RecordSpec::new(Some("document"),RecordLayout::Inline,vec![FieldSpec::new(0,"rows",Shape::Table(producer))]);
    for mode in [JoinMode::Document,JoinMode::Inline]{let expected=print(&source(),&spec(),mode);for name in fixture["table"]["columns"].as_array().unwrap(){assert!(expected.contains(&format!("{}:",name.as_str().unwrap())))}assert!(expected.contains(&serde_json::to_string(&format!("{prefix}1")).unwrap()));for budget in fixture["budgets"].as_array().unwrap(){let mut writer=RetainedRecordWriter::new(source(),spec(),mode,1000000);let mut accepted=|_|true;let mut control=NativeEncodeControl::new(1000000,&mut accepted);let mut turns=0;let text=loop{assert!(writer.step(0,&mut control).unwrap().is_none());turns+=1;assert!(turns<1000000);if let Some(text)=writer.step(budget.as_u64().unwrap()as usize,&mut control).unwrap(){break text}};assert_eq!(text,expected);let parsed=parse_exact(&text,&spec(),&ParseOptions{mode:if mode==JoinMode::Document{SourceMode::Document}else{SourceMode::Inline},..Default::default()}).unwrap();assert_eq!(print(&parsed,&spec(),mode),expected);let mut close=owned_retirement(writer);while !close.terminal_is_empty(){if let semio_framework_value::SnapshotRetirementStep::Pending{released_bytes,..}=close.close_step(1,3).unwrap(){assert!(released_bytes<=3)}}}}
    for stop in [1,64,1024,8192]{let mut writer=RetainedRecordWriter::new(source(),spec(),JoinMode::Document,1000000);let accepted=std::cell::Cell::new(true);let mut callback=|_|accepted.get();let mut control=NativeEncodeControl::new(1000000,&mut callback);for _ in 0..stop{assert!(writer.step(1,&mut control).unwrap().is_none())}accepted.set(false);assert_eq!(writer.step(1,&mut control).unwrap_err().kind,ValueRefusalKind::Canceled);let mut close=owned_retirement(writer);while !close.terminal_is_empty(){if let semio_framework_value::SnapshotRetirementStep::Pending{released_bytes,..}=close.close_step(1,3).unwrap(){assert!(released_bytes<=3)}}}
    eprintln!("[DEBUG] same columnar table owner rows64 budgets1/8/256 under1MB; canonical/parse/serde text; cancel and three-byte release");
}

#[test]
fn retained_declared_containers_keep_list_map_record_and_tuple_payloads() {
    use semio_framework_value::{NativeEncodeControl,NativeDecodeControl,retirement::owned_retirement};
    fn row()->RecordSpec{RecordSpec::new(None,RecordLayout::Inline,vec![FieldSpec::new(0,"label",Shape::Text),FieldSpec::new(1,"value",Shape::Value)])}
    fn encode(control:&mut NativeEncodeControl<'_>)->Result<RecordSpec,ValueError>{let mut fields=control.allocate_vec(2)?;fields.push(crate::producer::field(0,"label",Shape::Text,control)?);fields.push(crate::producer::field(1,"value",Shape::Value,control)?);crate::producer::record(None,RecordLayout::Inline,fields,control)}
    fn decode(_: &mut NativeDecodeControl<'_>)->Result<RecordSpec,ValueError>{Err(ValueError::new(ValueRefusalKind::UnsupportedOwner,"encoding fixture"))}
    let producer=RecordSpecProducer{ordinary:row,encoding:encode,decoding:decode};let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🧵️continuation/🔣️.json")).unwrap();let keys=fixture["containers"]["keys"].as_array().unwrap().iter().map(|key|key.as_str().unwrap()).collect::<Vec<_>>();
    let record=|label:&str|RecordValue{fields:[(0,FieldValue::Text(label.into())),(1,FieldValue::Value(DslValue::Object(vec![("z".into(),DslValue::String(label.into())),("a".into(),DslValue::Number(Number::Int(1)))])))].into_iter().collect()};
    let source=||RecordValue{fields:[(0,FieldValue::List(fixture["containers"]["list"].as_array().unwrap().iter().map(|text|FieldValue::Text(text.as_str().unwrap().into())).collect())),(1,FieldValue::List(keys.iter().map(|key|FieldValue::Record(record(key))).collect())),(2,FieldValue::Map(keys.iter().map(|key|(key.to_string(),FieldValue::Record(record(key)))).collect())),(3,FieldValue::Map(keys.iter().map(|key|(key.to_string(),FieldValue::Value(DslValue::Array(vec![DslValue::String(key.to_string()),DslValue::Number(Number::Int(1))])))).collect())),(4,FieldValue::Tuple(fixture["containers"]["tuple"].as_array().unwrap().iter().map(|value|FieldValue::Float(value.as_f64().unwrap())).collect()))].into_iter().collect()};
    let spec=||RecordSpec::new(Some("document"),RecordLayout::Inline,vec![FieldSpec::new(0,"list",Shape::List(Box::new(Shape::Text))),FieldSpec::new(1,"records",Shape::List(Box::new(Shape::Record(producer)))),FieldSpec::new(2,"map",Shape::Map(Box::new(Shape::Record(producer)))),FieldSpec::new(3,"values",Shape::Map(Box::new(Shape::Value))),FieldSpec::new(4,"tuple",Shape::Tuple(Box::new(Shape::Float),Some(2)))]);
    for mode in [JoinMode::Document,JoinMode::Inline]{let expected=print(&source(),&spec(),mode);assert!(expected.contains(&serde_json::to_string("Mesh 😀").unwrap()));for budget in fixture["budgets"].as_array().unwrap(){let mut writer=RetainedRecordWriter::new(source(),spec(),mode,1000000);let mut accepted=|_|true;let mut control=NativeEncodeControl::new(1000000,&mut accepted);let mut turns=0;let text=loop{assert!(writer.step(0,&mut control).unwrap().is_none());turns+=1;assert!(turns<1000000);if let Some(text)=writer.step(budget.as_u64().unwrap()as usize,&mut control).unwrap(){break text}};assert_eq!(text,expected);let parsed=parse_exact(&text,&spec(),&ParseOptions{mode:if mode==JoinMode::Document{SourceMode::Document}else{SourceMode::Inline},..Default::default()}).unwrap();assert_eq!(print(&parsed,&spec(),mode),expected);let mut close=owned_retirement(writer);while !close.terminal_is_empty(){if let semio_framework_value::SnapshotRetirementStep::Pending{released_bytes,..}=close.close_step(1,3).unwrap(){assert!(released_bytes<=3)}}}}
    eprintln!("[DEBUG] same declared list/map/record/tuple owner; dynamic reserved/Unicode keys, budgets1/8/256, canonical/serde text and bounded release");
}

#[test]
fn retained_declared_wire_streams_glued_endpoints_labels_and_properties() {
    use semio_framework_value::{NativeEncodeControl,retirement::owned_retirement};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🧵️continuation/🔣️.json")).unwrap();let row=&fixture["wire"];let from=row["from"].as_str().unwrap().repeat(row["repeats"].as_u64().unwrap()as usize);let source=|directed:bool,label:bool,edge:bool|RecordValue{fields:[(0,FieldValue::Wire(WireValue{from:WireNode{id:from.clone(),kind:Some(row["fromKind"].as_str().unwrap().into()),port:Some(row["fromPort"].as_str().unwrap().into())},edge:edge.then(||(directed,WireNode{id:row["to"].as_str().unwrap().into(),kind:Some(row["toKind"].as_str().unwrap().into()),port:Some(row["toPort"].as_str().unwrap().into())})),edge_label:if label{WireEdgeLabel{id:Some(row["labelId"].as_str().unwrap().into()),kind:Some(row["labelKind"].as_str().unwrap().into())}}else{WireEdgeLabel::default()},properties:DslValue::Object(vec![("z".into(),DslValue::String(from.clone())),("a".into(),DslValue::Number(Number::Int(1)))])}))].into_iter().collect()};let spec=||RecordSpec::new(Some("document"),RecordLayout::Inline,vec![FieldSpec::new(0,"wire",Shape::Wire)]);
    for mode in [JoinMode::Document,JoinMode::Inline]{for (directed,label,edge) in [(true,false,true),(false,false,true),(true,true,true),(false,true,true),(true,false,false)]{let expected=print(&source(directed,label,edge),&spec(),mode);assert!(expected.contains(&serde_json::to_string(&from).unwrap()));for budget in fixture["budgets"].as_array().unwrap(){let mut writer=RetainedRecordWriter::new(source(directed,label,edge),spec(),mode,1000000);let mut accepted=|_|true;let mut control=NativeEncodeControl::new(1000000,&mut accepted);let mut turns=0;let text=loop{assert!(writer.step(0,&mut control).unwrap().is_none());turns+=1;assert!(turns<1000000);if let Some(text)=writer.step(budget.as_u64().unwrap()as usize,&mut control).unwrap(){break text}};assert_eq!(text,expected);let parsed=parse_exact(&text,&spec(),&ParseOptions{mode:if mode==JoinMode::Document{SourceMode::Document}else{SourceMode::Inline},..Default::default()}).unwrap();assert_eq!(print(&parsed,&spec(),mode),expected);let mut close=owned_retirement(writer);while !close.terminal_is_empty(){if let semio_framework_value::SnapshotRetirementStep::Pending{released_bytes,..}=close.close_step(1,3).unwrap(){assert!(released_bytes<=3)}}}}}
    for stop in [1,64,512,2048]{let mut writer=RetainedRecordWriter::new(source(true,true,true),spec(),JoinMode::Document,1000000);let accepted=std::cell::Cell::new(true);let mut callback=|_|accepted.get();let mut control=NativeEncodeControl::new(1000000,&mut callback);for _ in 0..stop{assert!(writer.step(1,&mut control).unwrap().is_none())}accepted.set(false);assert_eq!(writer.step(1,&mut control).unwrap_err().kind,ValueRefusalKind::Canceled);let mut close=owned_retirement(writer);while !close.terminal_is_empty(){if let semio_framework_value::SnapshotRetirementStep::Pending{released_bytes,..}=close.close_step(1,3).unwrap(){assert!(released_bytes<=3)}}}
    eprintln!("[DEBUG] same retained Wire text directed/undirected/labeled/bare node, long Unicode and reserved port, intrinsic props, budgets1/8/256, cancel/retirement");
}

#[test]
fn retained_declared_record_depth_matches_the_existing_physical_limit() {
    use semio_framework_value::{NativeEncodeControl,NativeDecodeControl,retirement::owned_retirement};
    fn producer()->RecordSpecProducer{RecordSpecProducer{ordinary:spec,encoding:encode,decoding:decode}}
    fn spec()->RecordSpec{RecordSpec::new(None,RecordLayout::Inline,vec![FieldSpec::new(0,"child",Shape::Record(producer()))])}
    fn encode(control:&mut NativeEncodeControl<'_>)->Result<RecordSpec,ValueError>{let mut fields=control.allocate_vec(1)?;fields.push(crate::producer::field(0,"child",Shape::Record(producer()),control)?);crate::producer::record(None,RecordLayout::Inline,fields,control)}
    fn decode(_: &mut NativeDecodeControl<'_>)->Result<RecordSpec,ValueError>{Err(ValueError::new(ValueRefusalKind::UnsupportedOwner,"encoding fixture"))}
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🧵️continuation/🔣️.json")).unwrap();
    for case in fixture["depthCases"].as_array().unwrap(){let mut source=RecordValue{fields:[(0,FieldValue::Text("leaf".into()))].into_iter().collect()};for _ in 0..case["wraps"].as_u64().unwrap(){source=RecordValue{fields:[(0,FieldValue::Record(source))].into_iter().collect()};}let mut accepted=|_|true;let expected=print_controlled(&source,&spec(),JoinMode::Inline,1000000,&mut NativeEncodeControl::new(1000000,&mut accepted));assert_eq!(expected.is_ok(),case["accept"].as_bool().unwrap());let mut writer=RetainedRecordWriter::new(source,spec(),JoinMode::Inline,1000000);let mut control=NativeEncodeControl::new(1000000,&mut accepted);let mut turns=0;let result=loop{turns+=1;assert!(turns<1000000);match writer.step(1,&mut control){Ok(Some(text))=>break Ok(text),Ok(None)=>{},Err(error)=>break Err(error)}};let agrees=match(&expected,&result){(Ok(expected),Ok(actual))=>actual==expected,(Err(expected),Err(actual))=>actual.kind==expected.kind,_=>false};let mut close=owned_retirement(writer);while !close.terminal_is_empty(){if let semio_framework_value::SnapshotRetirementStep::Pending{released_bytes,..}=close.close_step(1,3).unwrap(){assert!(released_bytes<=3)}}assert!(agrees,"retained physical depth diverges expected={expected:?} actual={result:?}");}
    eprintln!("[DEBUG] retained and controlled physical record depth agree at64; owned source drains after refusal");
}
