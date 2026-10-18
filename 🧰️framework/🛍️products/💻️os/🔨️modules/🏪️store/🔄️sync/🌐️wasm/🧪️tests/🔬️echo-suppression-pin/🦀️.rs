//! 📌️ Source pin of the browser actor: it admits what it receives and records what it authors by operation identity, settles committed operations before admission, discards no frame by its origin and pushes no unchecked rollback.

#[test]
fn the_browser_actor_admits_authors_and_settles_by_operation_identity() {
    let source = concat!(
        include_str!("../../🎭️actor/🎛️wasm-actor-state-status-and-connect/🦀️.rs"),
        include_str!("../../🎭️actor/🚇️wasm-actor-relay-and-bootstrap/🦀️.rs"),
        include_str!("../../🎭️actor/📡️wasm-actor-protocol-and-spawn/🦀️.rs"),
        include_str!("../../🎭️actor/🏠️wasm-artifact-host/🦀️.rs"),
        include_str!("../../🎭️actor/🦀️.rs"),
    );
    assert!(!source.contains("Some(origin.0.as_str())"), "no Commands frame is discarded by its origin");
    assert_eq!(source.matches("admit_remote_envelopes(&mut self.applied_op_ids").count(), 1, "the browser actor admits by operation identity");
    assert_eq!(source.matches("note_authored_envelopes(&mut self.applied_op_ids, envelopes);").count(), 1, "the browser actor records what it authors");
    assert_eq!(source.matches("let settled = settle_committed_envelopes(&mut self.outbox, &mut self.pending_batches, &envelopes);").count(), 1, "the browser Commands arm settles");
    assert_eq!(source.matches("rollbacks.push(rollback_envelope(").count(), 0, "no rollback path pushes an unchecked rollback");
}
