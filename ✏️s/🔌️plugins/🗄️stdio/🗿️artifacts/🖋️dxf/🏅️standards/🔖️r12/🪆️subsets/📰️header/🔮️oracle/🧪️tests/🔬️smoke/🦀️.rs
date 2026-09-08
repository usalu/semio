
use super::{oracle_apply_mutation, oracle_apply_mutation_inverse, project_dxf_r12};
use semio_repo_test_host::parse_json;

const FIXTURE: &str = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/📚️examples/🚏️bus-shelter/🖼️assets/🖊️bus-shelter-r12.dxf";

const ROWS: &[(&str, &str)] = &[
    ("no-mutation", "{}"),
    ("set-snapshot", r#"{"insertionBase": [5, 5, 0], "layers": [{"name": "0", "color": 7, "linetype": "CONTINUOUS"}], "entities": [{"entityKind": "circle", "layer": "0", "center": [0, 0, 0], "radius": 42}]}"#),
    ("set-header-var", r#"{"name": "$INSBASE", "value": [15, 25, 0]}"#),
    ("remove-header-var", r#"{"name": "$INSBASE"}"#),
    ("insert-layer", r#"{"index": 1, "name": "MARKERS", "color": 6, "linetype": "CONTINUOUS"}"#),
    ("remove-layer", r#"{"name": "DIMS"}"#),
    ("set-layer", r#"{"name": "DIMS", "color": 4, "linetype": "DASHED"}"#),
    ("insert-style", r#"{"index": 1, "name": "LABELS", "font": "arial.ttf"}"#),
    ("remove-style", r#"{"name": "NOTES"}"#),
    ("set-style", r#"{"name": "NOTES", "font": "romans.shx"}"#),
    ("insert-linetype", r#"{"index": 1, "name": "CENTER", "description": "Center line"}"#),
    ("remove-linetype", r#"{"name": "DASHED"}"#),
    ("set-linetype", r#"{"name": "DASHED", "description": "Dash pattern"}"#),
    ("insert-entity", r#"{"index": 2, "entityKind": "circle", "layer": "0", "center": [1200, 100, 0], "radius": 30}"#),
    ("remove-entity", r#"{"index": 3}"#),
    ("set-entity", r#"{"index": 5, "entityKind": "text", "layer": "DIMS", "position": [200, 260, 0], "height": 80, "value": "WAVE 7 SHELTER"}"#),
    ("insert-block", r#"{"index": 1, "name": "BENCH_MARK", "basePoint": [0, 0, 0], "entities": [{"entityKind": "line", "layer": "0", "start": [0, 0, 0], "end": [100, 0, 0]}]}"#),
    ("remove-block", r#"{"index": 1}"#),
    ("set-block", r#"{"index": 0, "name": "SHELTER_POST", "basePoint": [0, 0, 0], "entities": [{"entityKind": "circle", "layer": "0", "center": [0, 0, 0], "radius": 20}]}"#),
];

#[test]
fn all_kinds_mutate_and_invert_cleanly() {
    assert_eq!(ROWS.len(), 19, "must exercise all 19 declared kinds");
    let input = std::fs::read(FIXTURE).expect("read committed fixture");
    let base_projection = project_dxf_r12(&input).expect("project base fixture");

    for (kind, params) in ROWS {
        let spec_text = format!(r#"{{"kind": "{kind}", "params": {params}}}"#);
        let spec = parse_json(&spec_text).unwrap_or_else(|e| panic!("bad spec JSON for {kind}: {e}"));

        let mutated = oracle_apply_mutation(&input, &spec).unwrap_or_else(|e| panic!("mutate {kind} failed: {e}"));
        assert!(!mutated.is_empty(), "mutate {kind} produced empty bytes");
        let mutated_projection = project_dxf_r12(&mutated).unwrap_or_else(|e| panic!("project mutate {kind} output failed: {e}"));
        if *kind != "no-mutation" {
            assert_ne!(mutated_projection, base_projection, "mutate {kind} produced no semantic change");
        }

        let inverted = oracle_apply_mutation_inverse(&input, &spec).unwrap_or_else(|e| panic!("inverse {kind} failed: {e}"));
        let inverted_projection = project_dxf_r12(&inverted).unwrap_or_else(|e| panic!("project inverse {kind} output failed: {e}"));
        assert_eq!(inverted_projection, base_projection, "inverse {kind} did not restore the base projection");
    }
}

#[test]
fn identity_round_trip_is_not_byte_identical() {
    let input = std::fs::read(FIXTURE).expect("read committed fixture");
    let spec = parse_json(r#"{"kind": "no-mutation", "params": {}}"#).expect("valid spec");
    let output = oracle_apply_mutation(&input, &spec).expect("no-mutation re-encode");
    assert_ne!(output, input, "byte pass-through: dxf-crate re-encode is bit-identical to the input");
    let base_projection = project_dxf_r12(&input).expect("project input");
    let output_projection = project_dxf_r12(&output).expect("project output");
    assert_eq!(base_projection, output_projection, "no-mutation re-encode changed the semantic projection");
}
