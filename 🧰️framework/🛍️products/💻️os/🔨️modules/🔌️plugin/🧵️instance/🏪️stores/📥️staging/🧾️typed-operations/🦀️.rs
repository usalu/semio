//! 🪤️ Staged slice of the `typed-operations` band of the mounted artifact instance: its `instance-stores` members (WP6 step S0 of the instance family).
//!
//! 🧾️ Typed operations of the mounted artifact instance: typed-operation publication, typed-command dispatch, configuration and the history panel.
use super::*;

impl<A: ArtifactApp> InstanceStores<A> {
    /// 🔀️ Ticket 26/09/16/INPUT-CAUSALITY-LEDGER §2 C — THE point the reactor first owns a
    /// typed command's emit. The interaction verbs are peeled here, before this same call
    /// can reach the effect lane below: an emit with no durable lane (fem2d's
    /// `canvasPointerUp` is `effects: [interactionSelect], ui_scope: partial` and nothing
    /// else) used to fall straight through to `emit.effects.pop()` → `typed_effect_outbox`
    /// on the install pump, while `mounted_typed_inline_interaction_verbs_are_next` had
    /// never seen the emit (it gates on `mounted.publication`, which was still `None`
    /// when this pump's ladder branch was chosen). Per-turn law: no host-visible turn of a
    /// typed operation ever carries one of the six interaction verbs as an effect.
    ///
    /// 🪜️ Hand the pump back to the ladder so its own lane order picks the first unit: the
    /// `InteractionWrite` lane and the inline fold both precede the effect/`Ui`/terminal
    /// pages of this same emit, and the fold must land BEFORE the `Ui` page is minted so
    /// that page already carries the verbs' widened scope and their leftover view.
    ///
    /// 🔒️ The three emit-time backstops of contract §2.3 clause 2, §5.10 and the tool-run
    /// freeze live in `dispatch_emit_inner`, which the migrated publication ladder never
    /// calls for the single-store artifact lane (FP5 §3.4 measured the same thing for the
    /// kind guard above). A migrated dispatch answers with an admission receipt and its
    /// mutations only exist HERE, so this is the only point at which they can be refused.
    ///
    /// 🔀️ Composite-mutation proposal (contract §5.1): apply NOTHING and stash a
    /// `TransactionProposalDraft` instead, drained once by `plugin_exchange`.
    ///
    /// 🧺️ ONE gesture is ONE batched publication: the whole artifact lane is drained
    /// into a single staged `Edit` (one history ledger slot, one undo step), instead
    /// of one edit per mutation per turn — see `store::begin_apply_batch`.
    ///
    /// 🧵️ A config lane is one plain edit per emission (design §20.1): a continuous view/config gesture rides its press
    /// in the window transient and commits its ONE config edit at gesture end (`settle_press`), so nothing here folds
    /// ticks into an earlier edit.
    ///
    /// 🎚️ The authority was captured when this command was DISPATCHED, and a window's
    /// config partition is its OWN exact-base document: any earlier command that
    /// settled against the same window has already moved that partition's generation
    /// and revision on, so a second window-config command dispatched in the same turn
    /// carries a stale base by the time its batch begins. `begin_apply_batch` then
    /// refuses it — and because the mutation was already POPPED out of `emit`, the
    /// retry `advance_typed_operation_publication_unit` grants on a publication fault
    /// found nothing left to publish and the operation completed clean: no page, no
    /// fault, the amend lost (measured on flow as one window-config page for two
    /// commands, ticket 26/09/19 `📓️flow.md` §5.2). Two repairs, both of which every
    /// sibling lane of this ladder already had: re-read the authority before beginning
    /// (the window-TRANSIENT arm below always did), and hand the mutation BACK to
    /// `emit` when admission refuses it, so a retry is a real retry and an exhausted
    /// one refuses by name on the Fault lane instead of vanishing.
    ///
    /// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4) — the
    /// task lane, ONE task per publication unit. The unit spends itself here
    /// and hands the pump straight back to the ladder without a host page of
    /// its own: a spawn is actor-local bookkeeping, and the task's eventual
    /// `TaskResolution` re-enters as its own follow-up dispatch
    /// (`drain_task_resumes`), never as this operation's result. `remove(0)`
    /// because spawn order is observable — a same-key respawn cancels the
    /// live task under that key, so the LAST declared task under a key must
    /// be the one that survives. A refused spawn (quota, key supersession,
    /// executor capacity) is this operation's own `Fault`: the ladder above
    /// gives the slot a chance to free and then mints its terminal fault page.
    pub(crate) fn handoff_mounted_durable_publication(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>) -> Result<bool, Fault> {
        let Some(pending) = mounted.pending_artifact_publication.take() else {
            return Ok(false);
        };
        let (result, restore) = match pending {
            PendingArtifactStorePublication::Artifact(publication) => {
                let mut owner = Some(publication);
                let result = self.store.handoff_batch_publication_close(&mut owner).map_err(|error| plugin_sdk_fault(error.to_string()));
                (result, owner.map(PendingArtifactStorePublication::Artifact))
            }
            PendingArtifactStorePublication::Config(publication) => {
                let mut owner = Some(publication);
                let result = self.config_store.handoff_batch_publication_close(&mut owner).map_err(|error| plugin_sdk_fault(error.to_string()));
                (result, owner.map(PendingArtifactStorePublication::Config))
            }
            PendingArtifactStorePublication::Draft(publication) => {
                let mut owner = Some(publication);
                let result = self.draft_store.handoff_batch_publication_close(&mut owner).map_err(|error| plugin_sdk_fault(error.to_string()));
                (result, owner.map(PendingArtifactStorePublication::Draft))
            }
            pending => {
                mounted.pending_artifact_publication = Some(pending);
                return Ok(false);
            }
        };
        if let Some(owner) = restore {
            mounted.pending_artifact_publication = Some(owner);
        }
        result
    }

    /// 🪟️ Continues the window-config authority refresh a published receipt retains until the displaced snapshot has admitted custody; `true` once it is complete, when the use case tells the extensions and queues the receipt's result page; every unit reports its actual progress to the host turn.
    pub(crate) fn advance_window_config_refresh(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, retained: RetainedCloneGrant) -> Result<bool, Fault> {
        mounted.pending_window_config_receipt.ok_or_else(|| plugin_sdk_fault("window config refresh lost its retained publication receipt"))?;
        let authority = mounted.window_config_authority.as_mut().ok_or_else(|| plugin_sdk_fault("window config receipt lost its captured window authority"))?;
        self.window_config_store.refresh_demands(authority, retained.maximum_copy_bytes).map_err(plugin_retirement_fault)?;
        let step = self.window_config_store.refresh(authority, retained)?;
        let (super::super::window_mutation::WindowAuthorityRefreshStep::Pending(progress) | super::super::window_mutation::WindowAuthorityRefreshStep::Ready(progress)) = step;
        if progress != RetainedCloneProgress::default() {
            assert!(mounted.publication_ownership_progress.is_none(), "window config refresh cannot overwrite an unreturned original receipt");
            mounted.publication_ownership_progress = Some((retained, progress));
            mounted.publication_progress = mounted.publication_progress.saturating_add(1);
        }
        if matches!(step, super::super::window_mutation::WindowAuthorityRefreshStep::Pending(_)) { return Ok(false); }
        Ok(true)
    }

    /// 🪟️ Queues the retained window-config receipt's result page once the refresh is complete and the extensions have been told.
    pub(crate) fn queue_window_config_receipt(mounted: &mut MountedTypedCommandFullOperation<A>) -> Result<(), Fault> {
        let receipt = mounted.pending_window_config_receipt.take().ok_or_else(|| plugin_sdk_fault("window config refresh lost its retained publication receipt"))?;
        let token = mounted.next_token();
        mounted.queue_page(TypedOperationResultPage::try_serialize(token, TypedOperationResultLane::WindowConfig, &receipt)?)
    }
}
