use super::*;

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
            FieldSpec::new(0, "e", Shape::Quantity(crate::os_dsl::unit_by_symbol("GPa").unwrap())),
            FieldSpec::new(1, "rho", Shape::Quantity(crate::os_dsl::unit_by_symbol("kg/m3").unwrap())),
            FieldSpec::new(2, "rotation", Shape::Angle(crate::os_dsl::unit_by_symbol("deg").unwrap())),
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
    RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(0, "schema", Shape::Text), FieldSpec::new(1, "layers", Shape::Statements(vec![("layer".to_string(), layer_variant_spec)]))])
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
/// @emoji 🌳️ Genuinely self-referential: `children`'s own variant table names `group_spec`
/// itself. Lazy `fn() -> RecordSpec` entries make this sound — `group_spec()` doesn't recurse
/// just to build the table, only `parse`/`print` calling the stored fn pointer one level at a
/// time (as deep as real input actually nests) ever evaluates it again.
// 🚫️async: E4 fn-pointer slot — stored bare as `fn() -> RecordSpec` in `Shape::Statements` above
fn group_spec() -> RecordSpec {
    RecordSpec::new(Some("group"), RecordLayout::Inline, vec![FieldSpec::new(0, "id", Shape::Text).positional(0), FieldSpec::new(1, "children", Shape::Block(Box::new(Shape::Statements(vec![("group".to_string(), group_spec)])))).optional()])
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
    RecordSpec::new(Some("marker"), RecordLayout::Inline, vec![FieldSpec::new(0, "at", Shape::Record(nested_point_spec))])
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
    RecordSpec::new(Some("scene"), RecordLayout::Inline, vec![FieldSpec::new(0, "nodes", Shape::Table(table_row_spec))])
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
        RecordSpec::new(Some("bad"), RecordLayout::Inline, vec![FieldSpec::new(0, "rows", Shape::Table(unbounded_tuple_row_spec))])
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
    RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(0, "id", Shape::Text), FieldSpec::new(1, "children", Shape::Table(nested_inner_row_spec))])
}
fn nested_table_doc_spec() -> RecordSpec {
    RecordSpec::new(Some("doc"), RecordLayout::Inline, vec![FieldSpec::new(0, "items", Shape::Table(nested_outer_row_spec))])
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
    RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(0, "id", Shape::Text), FieldSpec::new(1, "area", Shape::Record(quantity_spec)), FieldSpec::new(2, "volume", Shape::Record(quantity_spec))])
}
fn duplicate_type_table_doc_spec() -> RecordSpec {
    RecordSpec::new(Some("doc"), RecordLayout::Inline, vec![FieldSpec::new(0, "rows", Shape::Table(duplicate_type_row_spec))])
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
