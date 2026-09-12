
use super::*;

const FIXTURE: &str = "solid box\n  facet normal 0 0 -1\n    outer loop\n      vertex 0 0 0\n      vertex 0 1 0\n      vertex 1 0 0\n    endloop\n  endfacet\n  facet normal 0 0 1\n    outer loop\n      vertex 0 0 1\n      vertex 1 0 1\n      vertex 0 1 1\n    endloop\n  endfacet\nendsolid box\n";

fn spec(kind: &str, params: Json) -> Json {
    spec_of(kind, params)
}

#[test]
fn no_mutation_re_emits_the_document_rather_than_handing_the_bytes_back() {
    let output = oracle_apply_mutation(FIXTURE.as_bytes(), &spec("no-mutation", Json::Object(vec![]))).unwrap();
    assert_eq!(ascii::read_name(&output).unwrap(), "box", "the solid name survives");
    assert_eq!(triangle_soup::read(&output).unwrap().len(), 2, "and so does every facet");
    assert_eq!(
        String::from_utf8(output).unwrap(),
        "solid box\n  facet normal 0 0 -1\n    outer loop\n      vertex 0 0 0\n      vertex 0 1 0\n      vertex 1 0 0\n    endloop\n  endfacet\n  facet normal 0 0 1\n    outer loop\n      vertex 0 0 1\n      vertex 1 0 1\n      vertex 0 1 1\n    endloop\n  endfacet\nendsolid box\n",
        "re-emitted from the parsed model, not copied"
    );
}

#[test]
fn every_kind_emits_ascii_that_keeps_the_solid_name() {
    for (kind, params) in [
        ("no-mutation", Json::Object(vec![])),
        ("remove-triangle", Json::Object(vec![("index".to_string(), Json::Number(0.0))])),
        ("set-triangle-normal", Json::Object(vec![("index".to_string(), Json::Number(0.0)), ("normal".to_string(), Json::Array(vec![Json::Number(0.0), Json::Number(1.0), Json::Number(0.0)]))])),
    ] {
        let output = oracle_apply_mutation(FIXTURE.as_bytes(), &spec(kind, params)).unwrap();
        assert_eq!(ascii::read_name(&output).unwrap(), "box", "{kind} must not lose the solid name the way a binary re-encode would");
    }
}

#[test]
fn the_projection_sees_the_two_fields_a_triangle_soup_reader_cannot() {
    let renamed = oracle_apply_mutation(FIXTURE.as_bytes(), &spec("set-solid-name", Json::Object(vec![("name".to_string(), Json::String("renamed".to_string()))]))).unwrap();
    let turned =
        oracle_apply_mutation(FIXTURE.as_bytes(), &spec("set-triangle-normal", Json::Object(vec![("index".to_string(), Json::Number(0.0)), ("normal".to_string(), Json::Array(vec![Json::Number(0.0), Json::Number(1.0), Json::Number(0.0)]))])))
            .unwrap();
    let base = oracle_document_projection(FIXTURE.as_bytes()).unwrap();
    assert_ne!(oracle_document_projection(&renamed).unwrap(), base, "set-solid-name has to be visible somewhere");
    assert_ne!(oracle_document_projection(&turned).unwrap(), base, "and so does set-triangle-normal");
}

#[test]
fn set_solid_name_rewrites_only_the_header_and_trailer() {
    let output = oracle_apply_mutation(FIXTURE.as_bytes(), &spec("set-solid-name", Json::Object(vec![("name".to_string(), Json::String("renamed".to_string()))]))).unwrap();
    let text = String::from_utf8(output).unwrap();
    assert!(text.starts_with("solid renamed\n"));
    assert!(text.trim_end().ends_with("endsolid renamed"));
    assert_eq!(text.matches("facet normal").count(), 2);
}

#[test]
fn insert_and_remove_triangle_are_inverse_on_a_real_shaped_mesh() {
    let triangle = Json::Object(vec![
        ("normal".to_string(), Json::Array(vec![Json::Number(1.0), Json::Number(0.0), Json::Number(0.0)])),
        (
            "vertices".to_string(),
            Json::Array(vec![
                Json::Array(vec![Json::Number(9.0), Json::Number(0.0), Json::Number(0.0)]),
                Json::Array(vec![Json::Number(10.0), Json::Number(0.0), Json::Number(0.0)]),
                Json::Array(vec![Json::Number(9.0), Json::Number(1.0), Json::Number(0.0)]),
            ]),
        ),
    ]);
    let insert_spec = spec("insert-triangle", Json::Object(vec![("index".to_string(), Json::Number(1.0)), ("triangle".to_string(), triangle)]));
    let inserted = oracle_apply_mutation(FIXTURE.as_bytes(), &insert_spec).unwrap();
    assert_eq!(triangle_soup::read(&inserted).unwrap().len(), 3);

    let inverse = oracle_inverse_spec(FIXTURE.as_bytes(), &insert_spec).unwrap();
    assert_eq!(inverse.str("kind"), "remove-triangle");
    let restored = oracle_apply_mutation(&inserted, &inverse).unwrap();
    let before = triangle_soup::read(FIXTURE.as_bytes()).unwrap();
    let after = triangle_soup::read(&restored).unwrap();
    assert_eq!(before.len(), after.len());
    for (a, b) in before.iter().zip(after.iter()) {
        assert_eq!(a.normal, b.normal);
        assert_eq!(a.vertices, b.vertices);
    }
}

#[test]
fn remove_triangle_inverse_reinserts_the_original_triangle() {
    let remove_spec = spec("remove-triangle", Json::Object(vec![("index".to_string(), Json::Number(0.0))]));
    let removed = oracle_apply_mutation(FIXTURE.as_bytes(), &remove_spec).unwrap();
    assert_eq!(triangle_soup::read(&removed).unwrap().len(), 1);

    let inverse = oracle_inverse_spec(FIXTURE.as_bytes(), &remove_spec).unwrap();
    assert_eq!(inverse.str("kind"), "insert-triangle");
    let restored = oracle_apply_mutation(&removed, &inverse).unwrap();
    assert_eq!(triangle_soup::read(&restored).unwrap().len(), 2);
}

#[test]
fn set_snapshot_replaces_the_whole_triangle_list() {
    let one_triangle = Json::Array(vec![Json::Object(vec![
        ("normal".to_string(), Json::Array(vec![Json::Number(0.0), Json::Number(0.0), Json::Number(1.0)])),
        (
            "vertices".to_string(),
            Json::Array(vec![
                Json::Array(vec![Json::Number(0.0), Json::Number(0.0), Json::Number(0.0)]),
                Json::Array(vec![Json::Number(1.0), Json::Number(0.0), Json::Number(0.0)]),
                Json::Array(vec![Json::Number(0.0), Json::Number(1.0), Json::Number(0.0)]),
            ]),
        ),
    ])]);
    let output = oracle_apply_mutation(FIXTURE.as_bytes(), &spec("set-snapshot", Json::Object(vec![("triangles".to_string(), one_triangle)]))).unwrap();
    assert_eq!(triangle_soup::read(&output).unwrap().len(), 1);
}

/// 🔁️ The real committed fixture is written by a `f64` producer — `0.0`, `-8.881784197001252e-16`
/// — while `stl_io` resolves every coordinate through `f32`, so a genuine re-emission cannot
/// reproduce it. This vector carries that same shape, which `FIXTURE`'s tidy integers do not:
/// against `FIXTURE` the writer legitimately lands on the input again, and asserting otherwise
/// there would be asserting a coincidence of formatting rather than the law.
#[test]
fn round_trip_re_emits_a_real_producer_document_rather_than_copying_it() {
    const REAL_SHAPED: &str = "solid forest\n  facet normal -0.8660253933154181 0.0 -0.5000000181328751\n    outer loop\n      vertex 0.0 2.734999895095825 -8.881784197001252e-16\n      vertex 0.0 3.0 -8.881784197001252e-16\n      vertex 2.700000047683716 3.0 -4.676537036895752\n    endloop\n  endfacet\nendsolid forest\n";
    let output = oracle_round_trip(REAL_SHAPED.as_bytes()).unwrap();
    assert_ne!(output, REAL_SHAPED.as_bytes(), "the document was re-emitted from the parsed model, so the f64 source decimals cannot survive verbatim");
    assert_eq!(triangle_soup::read(&output).unwrap().len(), 1);
    assert_eq!(ascii::read_name(&output).unwrap(), "forest");
}

#[test]
fn unknown_kind_is_an_error_never_a_silent_no_op() {
    let result = oracle_apply_mutation(FIXTURE.as_bytes(), &spec("not-a-real-kind", Json::Object(vec![])));
    assert!(result.is_err(), "an unrecognised kind must fail loudly");
}
