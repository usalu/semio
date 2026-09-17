use super::*;

#[test]
fn puzzle2d_document_vcs_replays_granular_operations() {
    use crate::standards::v1::subsets::any::schema::empty_puzzle2d_snapshot;
    use crate::standards::v1::subsets::any::schema::mutations::create_node;
    use crate::{Puzzle2dNode, PUZZLE_2D_SCHEMA};
    use store::{create_document_envelope, ArtifactCommand};

    let mut store = semio_framework::io::resolve_ready(puzzle2d_store(create_document_envelope(PUZZLE_2D_SCHEMA, "puzzle2d", empty_puzzle2d_snapshot(), None))).expect("store");
    semio_framework::io::resolve_ready(store.dispatch(ArtifactCommand::Apply { mutations: vec![create_node(Puzzle2dNode { id: "n1".into(), ..Default::default() }, None)], description: None })).expect("apply");
    let projection = store.snapshot().expect("projection");
    assert_eq!(projection.nodes.len(), 1);
    assert_eq!(projection.nodes[0].id, "n1");
    close_puzzle2d_store(&mut store).expect("the standalone store retires to its terminal-empty shell");
}
