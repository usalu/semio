use super::*;

#[test]
fn puzzle5d_host_admission_neutral_projection_and_granular_inverse_match_independent_json() {
    let text = include_str!("🧫️fixtures/🔣️.json");
    let corpus = parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("neutral fixture");
    let independent: serde_json::Value = serde_json::from_str(text).expect("independent neutral JSON");
    let admit = |name: &str| {
        let mut document: Puzzle5dDocument = puzzle5d_record_from_projection(corpus[name].clone()).expect("host record");
        document.schema = PUZZLE5D_SCHEMA.into();
        puzzle5d_snapshot_from_document(&document).expect("typed admission")
    };
    let before = admit("before");
    let after = admit("after");
    assert_eq!(after.parts[0].part_2d.x, independent["after"]["parts"][0]["2d"]["x"].as_f64().expect("independent coordinate"));
    use crate::standards::v1::subsets::any::schema::mutations::{create_part, delete_part, move_part_2d, move_part_3d};
    let operations = vec![delete_part("p1".into()), move_part_2d("p2".into(), 9.0, 0.0), move_part_3d("p2".into(), [9.0, 0.0, 0.0]), create_part(after.parts[1].clone(), None)];
    assert!(operations.iter().any(|operation| matches!(operation, Puzzle5dMutation::CreatePart(_))));
    assert!(operations.iter().any(|operation| matches!(operation, Puzzle5dMutation::DeletePart(_))));
    assert!(operations.iter().any(|operation| matches!(operation, Puzzle5dMutation::MovePart2d(_))));
    let mut state = before.clone();
    let mut inverses = Vec::new();
    for operation in &operations {
        inverses.extend(protocol::Mutation::<Puzzle5dSnapshot>::inverse(operation, &state).expect("typed inverse"));
        state = protocol::apply_diff(protocol::Mutation::<Puzzle5dSnapshot>::diff(operation, &state).diff(), &state).expect("typed intent");
    }
    assert_eq!(state, after);
    for inverse in inverses.iter().rev() {
        state = protocol::apply_diff(protocol::Mutation::<Puzzle5dSnapshot>::diff(inverse, &state).diff(), &state).expect("typed inverse intent");
    }
    assert_eq!(state, before);
    let projected = puzzle5d_document_from_snapshot(&after).expect("owned native projection");
    assert_eq!(puzzle5d_snapshot_from_document(&projected).expect("readmitted"), after);
    eprintln!("[DEBUG] Puzzle5d neutral host admission, independent JSON coordinate, authored concrete forward and inverse all agree");
}

#[test]
fn puzzle5d_host_admission_refuses_wrong_schema_and_malformed_owned_inputs() {
    let corpus = parse(include_str!("🧫️fixtures/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("neutral refusal fixture");
    let mut document = empty_document();
    document.schema = corpus["refusedSchema"].as_str().expect("neutral schema").into();
    assert!(puzzle5d_snapshot_from_document(&document).is_err());
    assert!(puzzle5d_paste_placement(&semio_framework_pack_json::json!({"anchor":corpus["refusedAnchor"].clone()})).is_err());
    assert!(puzzle5d_paste_placement(&semio_framework_pack_json::json!({"position":corpus["refusedPosition"].clone()})).is_err());
    document.schema = PUZZLE5D_SCHEMA.into();
    document.meta = Some(corpus["refusedMeta"].clone());
    assert!(puzzle5d_snapshot_from_document(&document).is_err());
    document.meta = None;
    document.kind_catalogs = Some(corpus["refusedCatalogs"].clone());
    assert!(puzzle5d_snapshot_from_document(&document).is_err());
    document.kind_catalogs = None;
    let mut part: Puzzle5dPart = puzzle5d_record_from_projection(semio_framework_pack_json::json!({"id":"p1","partKind":"kind"})).expect("host part");
    part.part_3d.scale = Some(corpus["refusedScale"].clone());
    document.parts.push(part);
    assert!(puzzle5d_snapshot_from_document(&document).is_err());
    eprintln!("[DEBUG] Puzzle5d malformed schema, placement, metadata, catalogue and scale are refused before semantic mutation admission");
}
