//! 🧪️ Original constructor scope owns each actual Vec backing and borrowed membership keeps authored focus semantics.
use super::*;
#[test]
fn puzzle3d_borrowed_focus_matches_neutral_corpus() {
    let fixture = semio_framework_pack_json::parse(include_str!("🧫️fixtures/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for row in fixture.get("selectionCases").unwrap().as_array().unwrap() {
        let selected = row.get("selected").unwrap().as_array().unwrap().iter().map(|id| id.as_str().unwrap().to_owned()).collect::<Vec<_>>();
        assert_eq!(Puzzle3dFocusSelectionWork::frames_source(&selected, row.get("target").unwrap().as_str().unwrap()), row.get("frames").unwrap().as_bool().unwrap());
    }
}
#[test]
fn puzzle3d_action_constructors_quote_every_actual_buffer() {
    let fixture = semio_framework_pack_json::parse(include_str!("🧫️fixtures/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for row in fixture.get("buffers").unwrap().as_array().unwrap() {
        let count = |key| row.get(key).and_then(|value| value.as_u64()).unwrap_or(0) as usize;
        let expected = count("vortices") * size_of::<semio_s_artifact_puzzle_3d_model::Puzzle3dVortex>() + count("mutations") * size_of::<Puzzle3dMutation>() + count("strings") * size_of::<String>();
        let observed = match row.get("owner").unwrap().as_str().unwrap() {
            "AddBrushObject" => Puzzle3dAddBrushObjectWork::original_birth_demands().unwrap(),
            "AcceptSuggestion" => Puzzle3dAcceptSuggestionWork::original_birth_demands().unwrap(),
            "KindWeight" => semio_s_artifact_puzzle_3d_retained_document::Puzzle3dKindWeightWork::original_birth_demands().unwrap(),
            "AddObjectKind" => semio_s_artifact_puzzle_3d_retained_document::Puzzle3dAddObjectKindWork::original_birth_demands().unwrap(),
            "PatchInspector" => semio_s_artifact_puzzle_3d_retained_document::Puzzle3dPatchInspectorWork::original_birth_demands().unwrap(),
            "SetActiveExample" => semio_s_artifact_puzzle_3d_retained_document::Puzzle3dSetActiveExampleWork::original_birth_demands().unwrap(),
            _ => panic!("unrecognized authored work owner"),
        };
        assert_eq!(observed.capacity_bytes, expected);
        assert_eq!((observed.copy_bytes, observed.release_bytes, observed.depth), (0, 0, 1));
    }
}
