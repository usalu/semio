//! ✉️ Staged slice of the `dispatch` band of the mounted artifact instance: its `instance-docs` members (WP6 step S0 of the instance family).
//!
//! 📨️ Dispatch of the mounted artifact instance: test snapshot helpers, command log, history view, patch and refresh, preview, `dispatch_emit` and transaction groups.
use semio_framework::{ActionKind, Fault, FaultCode, FaultFrom, FaultOrigin};
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework::kernel::Effect;
use semio_framework_plugin_decl_emission::Emit;
use std::future::Future;
use crate::docs_components::MediaTransfers;
use store::{MemberFactory, Mutation, SpaceMember};
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_ui_contract::*;

impl<A: ArtifactApp> MediaTransfers<A> {
    /// 🧹️ Test-only probe: whether a cancelled segmented output still waits in bounded cleanup.
    #[cfg(test)]
    pub fn test_segmented_closure_contains(&self, operation_id: u64) -> bool {
        self.segmented_closures.get(operation_id).is_some()
    }

    /// 🧬️ Shared dispatch tail for `handle_action`/`handle_command`/`import_media`: given the
    /// app's `ActionEmit`, either records the op-less dispatch and returns an empty result, or commits
    /// `Apply` or the transaction commands, records the resulting edit, and builds the `InvocationResult` from it.
    /// `verb` is the action/command id, used to resolve the registry kind/label and to synthesize the
    /// `InvocationId`.
    /// 🧬️ B1: the single dispatch tail for BOTH `dispatch_typed_command` (the app's own
    /// `Emit`) and the framework-reserved clipboard actions below — commits `artifact_mutations` to
    /// the document store and `config_mutations` to the config store (independently, each its own
    /// undo stack), records exactly one command-log row carrying whichever edit id(s) were produced,
    /// and builds the resulting `InvocationResult` from the document side (config-only dispatches
    /// never touch `KernelMutation`/space-sync — see `🔖️CommandLog`'s doc region for why).
    /// 🔀️ Builds a typed rejection `Fault` using one of the frozen transaction rejection codes
    /// (contract-freeze.md §5's "Rejection taxonomy") — shared by `dispatch_emit`'s freeze check
    /// and every `transaction_prepare`/`transaction_commit`/`transaction_rollback`/
    /// `transaction_undo`/`transaction_redo` rejection path below.
    /// 🧮️ B1: `materialize_args` (JSON-args default-fill + required-arg enforcement for app-declared
    /// actions/commands, plus its now-unused `effective_action_args`/`missing_required_args` imports)
    /// was deleted — dead code once `dispatch_action`'s generic app-action fallback and
    /// `dispatch_command`'s registry-backed dispatch were removed (an app's own behavior now dispatches
    /// exclusively through the typed `Self::Command` channel, where a Rust caller supplies a complete
    /// value — there is no "missing arg" to materialize).
    ///
    /// 🚫️async: E1 pure error constructor consumed pervasively by sync closures — see R9.
    pub fn transaction_fault(origin: FaultOrigin, code: &'static str, message: impl Into<String>) -> Fault {
        Fault::new(origin, FaultCode::new(code), message)
    }

    /// 🔁️ Drains `emit.extension_invocations` through the guest request registry: each entry
    /// allocates a `RequestId`, records its `response_action` continuation against that id and
    /// queues the resulting `Effect::InvokeExtension` on the registry's own outbound queue (the
    /// one `reactor::poll` drains into `turn-result.effects` later in the SAME turn). Also the
    /// fail-closed gate on the defect this lane replaces: an app that smuggles a hand-built
    /// `Effect::InvokeExtension` into `emit.effects` is rejected outright rather than having its
    /// result silently dropped at `Event::Completed` for an id that owns no registry slot.
    pub fn mint_extension_invocations(instance: u32, emit: &mut Emit<A::Mutation, A::ConfigMutation, A::DraftMutation>) -> Result<(), Fault> {
        if emit.effects.iter().any(|effect| matches!(effect, Effect::InvokeExtension { .. })) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.unminted-extension-invocation"), "Effect::InvokeExtension carries a registry-minted request id — emit it through Emit::extension_invocations instead"));
        }
        for invocation in std::mem::take(&mut emit.extension_invocations) {
            semio_framework_plugin_reactor_tasks::tasks_extension_continuation::queue_extension_invocation(instance, &invocation)?;
        }
        Ok(())
    }

    /// 🧩️ `dispatch_emit`'s composite-gesture branch, taken whenever `emit.child_emits` is
    /// non-empty. Routes `artifact_mutations` (the parent's own ops) plus every `ChildEmit`
    /// through `store::os_store::composition_coordinator::CompositionCoordinator::dispatch_group` as ONE atomic multi-document
    /// gesture, so a single user action spanning a parent and N owned children still produces ONE
    /// `UndoGroup`: `inverse_group.member_edits` names every touched document (parent included),
    /// and `dispatch_action`'s history-action arm routes an undo/redo of a group-tailed edit
    /// through `CompositionCoordinator::undo_group`/`redo_group` — see that arm's own doc comment.
    /// Deliberately narrower than the solitary (zero-children) path above: `dispatch_group`'s
    /// phase 2 always issues a plain `Apply` on every touched member (there is no group-aware
    /// streamed open edit), so a composite gesture with children is
    /// always ONE described edit per member, never folded across ticks — only the solitary path
    /// streams a transaction's ticks into one open edit. `inverse_group.mutations`/`.inverse_mutations`
    /// are best-effort for CHILD members specifically: real per-op inverse bytes are retrievable
    /// only through `ArtifactStore<P, Mutation>`'s own inherent `edit_mutations()`, and each
    /// child's concrete `P`/`Mutation` is erased behind the generic `M: SpaceMember` — the REAL
    /// reversal mechanism for a child member is `CompositionCoordinator::undo_group` calling
    /// `undo()` directly on the live child store (driven by `inverse_group.member_edits`, not by
    /// replaying these bytes).
    /// 🚦️ Kind discipline where the operations actually arrive. A migrated dispatch hands its
    /// reducer to a worker, so the `View`/`🐚️Shell`-kind refusal `ArtifactApp::command_id`'s own doc
    /// promises can only be made against the published emit, never against the admission receipt the
    /// caller gets back. An unresolved verb (registry-less construction, an ad-hoc port id) is not a
    /// declared `View`, so it is not refused here.
    /// 🚫️async: E1 pure classification over an already-resolved kind
    pub fn require_operation_emitting_kind(&self, verb: &str, declared_kind: Option<ActionKind>) -> Result<(), Fault> {
        match declared_kind {
            Some(kind @ (ActionKind::View | ActionKind::Shell)) => Err(Fault::from(format!("{kind:?}-kind command '{verb}' must not emit operations"))),
            _ => Ok(()),
        }
    }
}
