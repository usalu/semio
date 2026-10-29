//! 🏀️ Staged `host-events` members of the `instance-dispatch` use cases, as methods of the borrowed bundle (WP7 of the instance family).
//!
//! 📣️ Host events: the window facts delivered to the app as ordinary typed operations.
use semio_framework_plugin_decl_emission::{ActionMeta, Emit, HostEvent};
use semio_framework_plugin_command::command_admission::original_typed_command::OriginalTypedCommandSource;
use semio_framework_value::{ValueError, ValueRefusalKind};
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use crate::dispatch_bundle::DispatchCtx;
use semio_framework_value::DslValue;
use semio_framework::{Fault, FaultCode, FaultFrom, FaultOrigin, ViewModel};
use std::future::Future;
use semio_framework_plugin_instance_extension::InstanceExtension;
use semio_framework::kernel::{InvocationResult, UiDirtyScope};
use store::{MemberFactory, Mutation, ReferenceSpaceMember};
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_instance_jobs::jobs_components::ToolOperations;
use semio_framework_ui_contract::*;

/// 📬️ What one funded turn of window-fact delivery decided for the action being dispatched.
pub enum HostEventTurn {
    /// ✅️ Nothing waits: the action proceeds.
    Idle,
    /// ⏳️ A delivery is in flight or just drained: the action returns for another turn.
    Pending,
    /// 💬️ The `hostEvent` action's own reply, after its fact was delivered.
    Reply(InvocationResult),
}

impl<'ctx, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X: InstanceExtension> DispatchCtx<'ctx, A, M, X> {
    /// 📨️ Ends the event's window's open gesture with the fact's reason (the runtime's ONE mapping, design §22.10), then
    /// queues `event` when the app answers it ([`ArtifactApp::host_event`]): its typed answer dispatches to the event's window
    /// — addressed through the delivering dispatch's view when it holds that window, else through the last roster — as an
    /// ordinary typed operation that is never a history row of its own. An event the app ignores queues nothing.
    fn queue_host_event(&mut self, event: HostEvent) {
        self.as_render().end_window_gesture(&event);
        if A::host_event(&event).is_some() {
            self.host_events.queue.push_back(event);
        }
    }

    /// 🪟️ The metadata a queued fact is delivered under: `meta` retargeted at the view of the fact's window, `None` when no view holds it.
    fn host_event_meta(&self, event: &HostEvent, meta: &ActionMeta) -> Option<ActionMeta> {
        let window_id = event.window_id();
        let view = meta.view_state.as_ref().and_then(|view| view.for_window_instance(window_id)).or_else(|| ViewModel { window_instances: self.host_events.host_event_roster.clone(), ..meta.view_state.as_ref()?.clone() }.for_window_instance(window_id))?;
        Some(ActionMeta { view_state: Some(view), ..meta.clone() })
    }

    /// 🧰️ Queues [`HostEvent::UtilityChanged`] for every window whose utility `meta`'s view moved since a dispatch
    /// last showed it — the targeted window reads the per-call overlay, every other one the view's map, and a window
    /// the view names no utility for is unknown, never a switch — and remembers the view's window roster.
    pub fn queue_utility_host_events(&mut self, meta: &ActionMeta) {
        let Some(view) = meta.view_state.as_ref() else { return };
        if !view.window_instances.is_empty() && self.host_events.host_event_roster != view.window_instances {
            self.host_events.host_event_roster = view.window_instances.clone();
            self.host_events.host_event_utilities.retain(|window_id, _| view.window_instances.iter().any(|window| window.id == *window_id));
        }
        let mut switched = Vec::new();
        for window in &view.window_instances {
            let utility = if view.window_id.as_deref() == Some(window.id.as_str()) { Some(view.active_utility_id.as_deref()) } else { view.active_utility_by_window_id.get(&window.id).map(|utility| Some(utility.as_str())) };
            let Some(utility) = utility else { continue };
            match self.host_events.host_event_utilities.get_mut(&window.id) {
                Some(previous) if previous.as_deref() == utility => {}
                Some(previous) => {
                    let from = std::mem::replace(previous, utility.map(str::to_string));
                    switched.push(HostEvent::UtilityChanged { window_id: window.id.clone(), from, to: utility.map(str::to_string) });
                }
                None => {
                    self.host_events.host_event_utilities.insert(window.id.clone(), utility.map(str::to_string));
                }
            }
        }
        for event in switched {
            self.queue_host_event(event);
        }
    }

    /// 🌐️ Queues one instance-wide event, built per window, for every window of `meta`'s view, else of the last roster.
    pub fn queue_host_event_to_every_window(&mut self, event: impl Fn(String) -> HostEvent, meta: &ActionMeta) {
        self.as_render().end_every_gesture(&event(String::new()));
        let roster = meta.view_state.as_ref().map(|view| view.window_instances.as_slice()).filter(|roster| !roster.is_empty()).unwrap_or(&self.host_events.host_event_roster);
        let windows: Vec<String> = roster.iter().map(|window| window.id.clone()).collect();
        for window_id in windows {
            self.queue_host_event(event(window_id));
        }
    }

    /// 🌐️ Queues [`HostEvent::BaseMoved`] for every window once a remote edit moved the base, as the local actor.
    pub fn queue_base_moved(&mut self) {
        let Some(instance_id) = self.tools.live_runtime_instance_id else { return };
        let meta = ActionMeta { actor: self.stores.store.local_actor_id().0.clone(), instance_id, view_state: None };
        self.queue_host_event_to_every_window(|window_id| HostEvent::BaseMoved { window_id }, &meta);
    }

    /// 📨️ `hostEvent{windowId, kind}`: the host-forwarded window fact, queued ahead of the action's own reply; an unknown
    /// kind or a missing window is refused.
    fn queue_host_event_action(&mut self, args: Option<&DslValue>) -> Result<(), Fault> {
        let text = |key: &str| args.and_then(|value| value.get(key)).and_then(DslValue::as_str);
        let event = text(semio_framework::HOST_EVENT_ARG_WINDOW_ID).zip(text(semio_framework::HOST_EVENT_ARG_KIND)).and_then(|(window_id, kind)| HostEvent::forwarded(window_id, kind));
        let Some(event) = event else {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("hostEvent.invalid"), "hostEvent requires a windowId and a kind of blur, captureLost or retiring"));
        };
        self.queue_host_event(event);
        self.host_events.reply = true;
        Ok(())
    }

    /// 💬️ The `hostEvent` action's reply once its fact has been delivered.
    async fn host_event_reply(&mut self, meta: &ActionMeta) -> InvocationResult {
        self.host_events.reply = false;
        let ui_scope = if self.as_render().take_gesture_ended() { UiDirtyScope::Full } else { UiDirtyScope::None };
        ToolOperations::<A>::empty_result(semio_framework::HOST_EVENT_ACTION_ID, meta, Vec::new(), Vec::new(), ui_scope).await
    }

    /// 🏃️ One funded turn of window-fact delivery for the action being dispatched: queues the utility switches its view shows and the
    /// `hostEvent` action's own fact, then drives the queue front through the shared original typed source until it has drained.
    pub async fn host_event_turn(&mut self, source: &mut OriginalTypedCommandSource<A>, action: &str, args: Option<&DslValue>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<HostEventTurn, semio_framework_job::OriginalTurnFailure> {
        let is_action = action == semio_framework::HOST_EVENT_ACTION_ID;
        if !self.host_events.delivering {
            self.queue_utility_host_events(meta);
            if is_action { self.queue_host_event_action(args)?; }
            if self.host_events.queue.is_empty() {
                return Ok(if is_action { HostEventTurn::Reply(self.host_event_reply(meta).await) } else { HostEventTurn::Idle });
            }
            self.host_events.delivering = true;
            return Ok(HostEventTurn::Pending);
        }
        if !self.drive_host_event_delivery(source, meta, identity, cx).await? { return Ok(HostEventTurn::Pending) }
        Ok(if self.host_events.reply { HostEventTurn::Reply(self.host_event_reply(meta).await) } else { HostEventTurn::Pending })
    }

    /// 📬️ Delivers the queue front: its typed answer is produced, admitted and mounted by the runtime-command lane; `true` once the queue is empty and the source idle.
    async fn drive_host_event_delivery(&mut self, source: &mut OriginalTypedCommandSource<A>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if cx.is_cancelled() {
            if source.pending() {
                cx.with_original_retirement(|cx| source.close_step(cx))?;
                return Ok(false);
            }
            self.host_events.queue.clear();
            self.host_events.delivering = false;
            self.host_events.reply = false;
            return Err(ValueError::literal(ValueRefusalKind::Canceled, "window fact delivery was cancelled").into());
        }
        let Some(event) = self.host_events.queue.front().cloned() else {
            if source.pending() {
                cx.with_original_retirement(|cx| source.close_step(cx))?;
                return Ok(false);
            }
            self.host_events.delivering = false;
            return Ok(true);
        };
        if !source.pending() && self.host_event_meta(&event, meta).is_none() {
            self.host_events.queue.pop_front();
            return Ok(false);
        }
        let mut metadata = self.host_event_meta(&event, meta);
        let mut tag = None;
        let delivered = self.drive_runtime_typed_command(source, || A::host_event(&event).map(Box::new), &mut tag, &mut metadata, identity, cx).await?;
        if let Some(operation) = source.operation {
            self.host_events.host_event_operations.insert(operation.operation.0);
        }
        if delivered.is_some() {
            self.host_events.queue.pop_front();
        }
        Ok(false)
    }
}
