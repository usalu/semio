
use super::*;

#[test]
fn puzzle5d_document_vcs_replays_granular_operations() {
    use crate::standards::v1::subsets::any::schema::mutations::create_part;
    use crate::standards::v1::subsets::any::schema::empty_puzzle5d_snapshot;
    use crate::{PUZZLE_5D_SCHEMA, Puzzle5dPart};
    use store::{ArtifactCommand, create_document_envelope};

    let mut store = semio_framework::io::resolve_ready(Puzzle5dStore::new(create_document_envelope(PUZZLE_5D_SCHEMA, "puzzle5d", empty_puzzle5d_snapshot(), None))).expect("store");
    semio_framework::io::resolve_ready(store.dispatch(ArtifactCommand::Apply { mutations: vec![create_part(Puzzle5dPart { id: "p1".into(), ..Default::default() }, None)], description: None })).expect("apply");
    let projection = store.snapshot().expect("projection");
    assert_eq!(projection.parts.len(), 1);
    assert_eq!(projection.parts[0].id, "p1");
}
