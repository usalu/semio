use super::*;
use serde_json::Value;

/// 🧭️ The compositor's `nativeSeams` vectors (shared with the browser): two adjacent cells, the cell sitting on the writing-direction side mirrored for `rtl`, resolve to ONE seam painted in the state the browser CSS gives it.
#[test]
fn native_seams_resolve_incident_states_in_presentation_priority() {
    let cases: Value = serde_json::from_str(include_str!("../../🧱️elements/🗂️WindowChrome/compositor.cases.json")).unwrap();
    let state_of = |markup: &str| {
        if markup.contains("data-introduced=\"true\"") { SharedBorderState::Introduced }
        else if markup.contains("border-loading") { SharedBorderState::Loading }
        else if markup.contains("data-state=\"active\"") || markup.contains("aria-pressed=\"true\"") { SharedBorderState::Active }
        else { SharedBorderState::Normal }
    };
    let expected_state = |color: &str| match color {
        "rgb(255, 0, 0)" => SharedBorderState::Active, "rgb(0, 0, 255)" => SharedBorderState::Loading, "rgb(0, 128, 0)" => SharedBorderState::Introduced,
        other => panic!("Unknown seam colour {other}"),
    };
    let cells = |orientation: &str, dir: &str, states: [SharedBorderState; 2]| {
        let mirror = orientation == "horizontal" && dir == "rtl";
        let (first, second) = match (orientation, mirror) { ("horizontal", false) => ([0.0, 0.0], [10.0, 0.0]), ("horizontal", true) => ([10.0, 0.0], [0.0, 0.0]), _ => ([0.0, 0.0], [0.0, 10.0]) };
        let mut input = Vec::new();
        for (owner, ([x, y], state)) in [(first, states[0]), (second, states[1])].into_iter().enumerate() {
            for (side, (from, to)) in [([x, y], [x + 10.0, y]), ([x + 10.0, y], [x + 10.0, y + 10.0]), ([x, y + 10.0], [x + 10.0, y + 10.0]), ([x, y], [x, y + 10.0])].into_iter().enumerate() {
                input.push(SharedBorderSegment { id: owner * 4 + side, owner, from, to, state });
            }
        }
        input
    };
    for case in cases["nativeSeams"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let (orientation, dir) = (case["orientation"].as_str().unwrap(), case["dir"].as_str().unwrap());
        let markup = |key: &str| case.get(key).and_then(Value::as_str).unwrap_or("").to_string();
        let states = [state_of(&markup("first")), state_of(&(markup("second") + &markup("secondContent")))];
        let resolved = resolve_shared_border_segments(&cells(orientation, dir, states)).unwrap();
        let seams: Vec<_> = resolved.iter().filter(|edge| edge.contributors.len() == 2).collect();
        assert_eq!(seams.len(), 1, "{name}: exactly one shared seam");
        let seam = seams[0].segment;
        let on_seam = if orientation == "horizontal" { seam.from[0] == 10.0 && seam.to[0] == 10.0 } else { seam.from[1] == 10.0 && seam.to[1] == 10.0 };
        assert!(on_seam, "{name}: the seam lies on the cells' common edge, {seam:?}");
        assert_eq!(seam.state, expected_state(case["color"].as_str().unwrap()), "{name}");
    }
}
