use semio_s_plugin_stdio_drawing_test_oracle::project_dxf_r12;

use super::{oracle_apply_mutation, oracle_apply_mutation_inverse, oracle_round_trip};
use semio_repo_test_host::parse_json;

const FIXTURE: &[u8] = include_bytes!("../../../🖼️assets/🚏️bus-shelter/🖊️.dxf");

const ROWS: &[(&str, &str)] = &[
    ("set-header-var", r#"{"name": "$INSBASE", "headerVar": {"name": "$INSBASE", "groupCode": 10, "value": {"kind": "point", "value": [15, 25, 0]}}}"#),
    ("remove-header-var", r#"{"name": "$INSBASE"}"#),
    ("insert-layer", r#"{"index": 1, "layer": {"name": "MARKERS", "color": 6, "linetype": "CONTINUOUS", "flags": 0}}"#),
    ("remove-layer", r#"{"name": "DIMS"}"#),
    ("set-layer", r#"{"name": "DIMS", "layer": {"name": "DIMS", "color": 4, "linetype": "DASHED", "flags": 0}}"#),
    ("insert-style", r#"{"index": 1, "style": {"name": "LABELS", "flags": 0, "fontName": "arial.ttf"}}"#),
    ("remove-style", r#"{"name": "NOTES"}"#),
    ("set-style", r#"{"name": "NOTES", "style": {"name": "NOTES", "flags": 0, "fontName": "romans.shx"}}"#),
    ("insert-linetype", r#"{"index": 1, "linetype": {"name": "CENTER", "flags": 0, "description": "Center line"}}"#),
    ("remove-linetype", r#"{"name": "DASHED"}"#),
    ("set-linetype", r#"{"name": "DASHED", "linetype": {"name": "DASHED", "flags": 0, "description": "Dash pattern"}}"#),
    ("insert-entity", r#"{"index": 2, "entity": {"circle": {"center": [1200, 100, 0], "radius": 30, "layer": "0"}}}"#),
    ("remove-entity", r#"{"index": 3}"#),
    ("set-entity", r#"{"index": 5, "entity": {"text": {"position": [200, 260, 0], "height": 80, "value": "WAVE 7 SHELTER", "layer": "DIMS"}}}"#),
    ("insert-block", r#"{"index": 1, "block": {"name": "BENCH_MARK", "basePoint": [0, 0, 0], "entities": [{"line": {"start": [0, 0, 0], "end": [100, 0, 0], "layer": "0"}}]}}"#),
    ("remove-block", r#"{"index": 1}"#),
    ("set-block", r#"{"index": 0, "block": {"name": "SHELTER_POST", "basePoint": [0, 0, 0], "entities": [{"circle": {"center": [0, 0, 0], "radius": 20, "layer": "0"}}]}}"#),
];

#[test]
fn all_kinds_mutate_and_invert_cleanly() {
    assert_eq!(ROWS.len(), 18, "must exercise all 18 declared kinds");
    let input = FIXTURE.to_vec();
    let base_projection = project_dxf_r12(&input).expect("project base fixture");

    for (kind, params) in ROWS {
        let spec_text = format!(r#"{{"kind": "{kind}", "params": {params}}}"#);
        let spec = parse_json(&spec_text).unwrap_or_else(|e| panic!("bad spec JSON for {kind}: {e}"));

        let mutated = oracle_apply_mutation(&input, &spec).unwrap_or_else(|e| panic!("mutate {kind} failed: {e}"));
        assert!(!mutated.is_empty(), "mutate {kind} produced empty bytes");
        let mutated_projection = project_dxf_r12(&mutated).unwrap_or_else(|e| panic!("project mutate {kind} output failed: {e}"));
        assert_ne!(mutated_projection, base_projection, "mutate {kind} produced no semantic change");

        let inverted = oracle_apply_mutation_inverse(&input, &spec).unwrap_or_else(|e| panic!("inverse {kind} failed: {e}"));
        let inverted_projection = project_dxf_r12(&inverted).unwrap_or_else(|e| panic!("project inverse {kind} output failed: {e}"));
        assert_eq!(inverted_projection, base_projection, "inverse {kind} did not restore the base projection");
    }
}

#[test]
fn identity_round_trip_is_not_byte_identical() {
    let input = FIXTURE.to_vec();
    let output = oracle_round_trip(&input).expect("round-trip re-encode");
    assert_ne!(output, input, "byte pass-through: dxf-crate re-encode is bit-identical to the input");
    let base_projection = project_dxf_r12(&input).expect("project input");
    let output_projection = project_dxf_r12(&output).expect("project output");
    assert_eq!(base_projection, output_projection, "the round-trip re-encode changed the semantic projection");
}
