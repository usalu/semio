//! 🧩️ The neutral extension point of the mounted artifact instance. A feature module (history editing, tool runs, …)
//! contributes command routes, document refusals, maintenance turns and framework bodies through [`InstanceExtension`];
//! the instance consults the registered list [`crate::InstanceExtensions`] and never names the feature itself.
use super::*;
use semio_framework_ui_contract::BuiltNode;
use semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority;

/// 🧱️ One feature's contribution to every mounted instance. Every hook defaults to "no contribution", so an extension
/// overrides only what it owns. A registered list is a right-nested tuple `(First, (Second, ()))`: earlier entries are
/// consulted first, the first claimant of a route, refusal or body wins, and every entry's maintenance turn runs in order.
pub(crate) trait InstanceExtension {
    /// 🧭️ Whether this extension owns the dispatch of `action`.
    fn claims_action(_action: &str) -> bool {
        false
    }

    /// 📜️ Whether `action` needs the document's history, so a head-only pure evaluation refuses it.
    fn reads_history(_action: &str) -> bool {
        false
    }

    /// ⏳️ Whether `action` stays admitted while the document is still loading.
    fn admitted_while_loading(_action: &str) -> bool {
        false
    }

    /// 🚦️ Dispatches one action this extension claims.
    async fn dispatch_action<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static>(_app: &mut VcsArtifactApp<A, M>, action: &str, _args: Option<&DslValue>, _meta: &ActionMeta, _identity: &mut EntityIdentityAuthority<'_>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        Err(plugin_sdk_fault(format!("no instance extension claims action '{action}'")))
    }

    /// 🔒️ The refusal of a framework-reserved `action` this extension currently freezes.
    fn reserved_action_refusal<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static>(_app: &VcsArtifactApp<A, M>, _action: &str) -> Option<Fault> {
        None
    }

    /// 🛑️ The refusal of a `verb` that would emit artifact mutations while this extension holds them.
    fn mutation_refusal<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static>(_app: &VcsArtifactApp<A, M>, _verb: &str) -> Option<Fault> {
        None
    }

    /// ❄️ Whether this extension currently freezes every local change to the document.
    fn freezes_document<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static>(_app: &VcsArtifactApp<A, M>) -> bool {
        false
    }

    /// 🚫️ The refusal of a `verb` that would change the document (artifact mutations or child emits) while it is frozen.
    fn document_refusal<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static>(_app: &VcsArtifactApp<A, M>, _verb: &str) -> Option<Fault> {
        None
    }

    /// 🔀️ The refusal of a foreign transaction `txn_id` that would change the document while it is frozen.
    fn transaction_refusal<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static>(_app: &VcsArtifactApp<A, M>, _txn_id: &str) -> Option<Fault> {
        None
    }

    /// 🎚️ Observes one committed window-config publication.
    fn window_config_published<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static>(_app: &mut VcsArtifactApp<A, M>) {}

    /// 🏃️ Whether this extension owes maintenance work.
    fn has_pending_work<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static>(_app: &VcsArtifactApp<A, M>) -> bool {
        false
    }

    /// 🧾️ Whether this extension holds an original receipt its recipient has not taken yet.
    fn holds_receipt<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static>(_app: &VcsArtifactApp<A, M>) -> bool {
        false
    }

    /// 🪦️ Whether this extension retains nothing a close still has to retire.
    fn terminal_is_empty<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static>(_app: &VcsArtifactApp<A, M>) -> bool {
        true
    }

    /// 🔁️ Drives one maintenance turn under the caller's current retained grant; `true` ends the instance's maintenance
    /// turn here.
    async fn drive_turn<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static>(_app: &mut VcsArtifactApp<A, M>, _retained: &mut RetainedCloneGrant, _identity: &mut EntityIdentityAuthority<'_>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, crate::PluginLifecycleFailure> {
        Ok(false)
    }

    /// 🖼️ The framework body this extension renders for `body_key`.
    fn render_body<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static>(_app: &mut VcsArtifactApp<A, M>, _body_key: &str, _view_state: &ViewModel) -> Option<UiAssemblyResult<BuiltNode>> {
        None
    }

    /// 🖌️ Decorates the app's own rendered body for `body_key` after it is retained.
    fn after_app_render<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static>(_app: &mut VcsArtifactApp<A, M>, _body_key: &str, _view_state: &ViewModel) -> Result<(), Fault> {
        Ok(())
    }
}

/// 🫙️ The empty list: no route, no refusal, no work.
impl InstanceExtension for () {}

/// 🔗️ A registered list: `First` is consulted before every entry of `Rest`.
impl<First: InstanceExtension, Rest: InstanceExtension> InstanceExtension for (First, Rest) {
    fn claims_action(action: &str) -> bool {
        First::claims_action(action) || Rest::claims_action(action)
    }

    fn reads_history(action: &str) -> bool {
        First::reads_history(action) || Rest::reads_history(action)
    }

    fn admitted_while_loading(action: &str) -> bool {
        First::admitted_while_loading(action) || Rest::admitted_while_loading(action)
    }

    async fn dispatch_action<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static>(app: &mut VcsArtifactApp<A, M>, action: &str, args: Option<&DslValue>, meta: &ActionMeta, identity: &mut EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        if First::claims_action(action) {
            First::dispatch_action(app, action, args, meta, identity, cx).await
        } else {
            Rest::dispatch_action(app, action, args, meta, identity, cx).await
        }
    }

    fn reserved_action_refusal<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static>(app: &VcsArtifactApp<A, M>, action: &str) -> Option<Fault> {
        First::reserved_action_refusal(app, action).or_else(|| Rest::reserved_action_refusal(app, action))
    }

    fn mutation_refusal<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static>(app: &VcsArtifactApp<A, M>, verb: &str) -> Option<Fault> {
        First::mutation_refusal(app, verb).or_else(|| Rest::mutation_refusal(app, verb))
    }

    fn freezes_document<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static>(app: &VcsArtifactApp<A, M>) -> bool {
        First::freezes_document(app) || Rest::freezes_document(app)
    }

    fn document_refusal<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static>(app: &VcsArtifactApp<A, M>, verb: &str) -> Option<Fault> {
        First::document_refusal(app, verb).or_else(|| Rest::document_refusal(app, verb))
    }

    fn transaction_refusal<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static>(app: &VcsArtifactApp<A, M>, txn_id: &str) -> Option<Fault> {
        First::transaction_refusal(app, txn_id).or_else(|| Rest::transaction_refusal(app, txn_id))
    }

    fn window_config_published<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static>(app: &mut VcsArtifactApp<A, M>) {
        First::window_config_published(app);
        Rest::window_config_published(app);
    }

    fn has_pending_work<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static>(app: &VcsArtifactApp<A, M>) -> bool {
        First::has_pending_work(app) || Rest::has_pending_work(app)
    }

    fn holds_receipt<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static>(app: &VcsArtifactApp<A, M>) -> bool {
        First::holds_receipt(app) || Rest::holds_receipt(app)
    }

    fn terminal_is_empty<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static>(app: &VcsArtifactApp<A, M>) -> bool {
        First::terminal_is_empty(app) && Rest::terminal_is_empty(app)
    }

    async fn drive_turn<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static>(app: &mut VcsArtifactApp<A, M>, retained: &mut RetainedCloneGrant, identity: &mut EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, crate::PluginLifecycleFailure> {
        *retained = cx.retained_grant();
        if First::drive_turn(app, retained, identity, cx).await? {
            return Ok(true);
        }
        Rest::drive_turn(app, retained, identity, cx).await
    }

    fn render_body<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static>(app: &mut VcsArtifactApp<A, M>, body_key: &str, view_state: &ViewModel) -> Option<UiAssemblyResult<BuiltNode>> {
        match First::render_body(app, body_key, view_state) {
            Some(body) => Some(body),
            None => Rest::render_body(app, body_key, view_state),
        }
    }

    fn after_app_render<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static>(app: &mut VcsArtifactApp<A, M>, body_key: &str, view_state: &ViewModel) -> Result<(), Fault> {
        First::after_app_render(app, body_key, view_state)?;
        Rest::after_app_render(app, body_key, view_state)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
