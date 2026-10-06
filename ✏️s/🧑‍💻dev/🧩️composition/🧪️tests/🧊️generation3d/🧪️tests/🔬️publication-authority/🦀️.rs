//! 🔐️ The publication door onto this binary's ONE serial lock ([`semio_s_artifact_procedural_generation3d::test_serial`]).
//!
//! The lease table is a PROCESS-GLOBAL fixed registry of `GENERATION3D_PUBLICATION_SLOTS` (4)
//! entries (`🚪️io/💾️binary/🧬️mutations/🦀️.rs`) whose admission is a `try_lock`, so two laws
//! holding leases at the same time saturate it and the loser fails with
//! `generation3d-publication.contended` — an order- and thread-count-dependent failure that has
//! nothing to do with what either law asserts (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
//!
//! 🐛️ This used to be its OWN mutex, which serialised publication laws against each other and
//! against nothing else: an editor law driving a retained tool job holds a lease too, held the
//! editor's separate mutex, and collided here anyway. One lock over one shared state.

/// 🔐️ Takes the crate's one test serial lock.
pub(crate) fn lock() -> crate::test_serial::TestSerialGuard {
    crate::test_serial::lock()
}

/// 🗂️ Seeds the concrete generation roster required by window-action laws.
pub(crate) fn seed_law_generations(root: &mut semio_framework_artifact_playbook_playbook::GenerationPlayRoot) {
    let entry = |id: &str, name: &str| semio_framework_artifact_playbook_playbook::FormGeneration { id: id.into(), name: name.into(), values: Default::default() };
    let state = root.cold_builder_mut().expect("unique cold generation owner");
    state.generations.clear();
    state.generations.push(entry("generation-1", "Generation 1"));
    state.generations.push(entry("generation-2", "Generation 2"));
    state.selected_generation_id = Some("generation-1".into());
}
