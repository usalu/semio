//! 🔁️ A history move (undo, redo) issued while the root advanced is accepted against the live root, never refused as stale; every other commit stays strict.

use semio_framework_job::{validate_commit_with, CommitPolicy, CommitValidation, Generation, Operation, OperationId, RevisionId};

fn moved_on() -> (Operation, RevisionId, Generation) {
    (Operation::new(OperationId(1), RevisionId(7), Generation(2), 0), RevisionId(9), Generation(3))
}

#[test]
fn a_history_move_issued_while_the_root_advanced_is_accepted_not_stale() {
    let (op, revision, generation) = moved_on();
    assert!(matches!(validate_commit_with(CommitPolicy::Strict, &op, revision, generation), CommitValidation::Stale { .. }));
    assert_eq!(validate_commit_with(CommitPolicy::RebaseOnLive, &op, revision, generation), CommitValidation::Accepted);
    assert_eq!(validate_commit_with(CommitPolicy::RebaseOnLive, &op, op.base_revision, op.generation), CommitValidation::Accepted);
}

#[test]
fn only_undo_and_redo_rebase_and_every_other_verb_stays_strict() {
    let (op, revision, generation) = moved_on();
    for verb in ["undo", "redo", "deleteElement", "commitCheckpoint", "solve"] {
        let accepted = validate_commit_with(CommitPolicy::for_verb(verb), &op, revision, generation) == CommitValidation::Accepted;
        assert_eq!(accepted, matches!(verb, "undo" | "redo"), "{verb}");
    }
}
