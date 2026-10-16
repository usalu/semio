//! 🏀️ Staged `host-events` members of the `instance-dispatch` use cases, as methods of the borrowed bundle (WP7 of the instance family).
//!
//! 📣️ Host events: the window facts delivered to the app as ordinary typed operations.
use super::*;
use crate::tool_jobs::time_travel;

impl<'ctx, A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension> DispatchCtx<'ctx, A, M, X> {
    /// 📨️ Ends the event's window's open gesture with the fact's reason (the runtime's ONE mapping, design §22.10), then
    /// delivers `event` to the app: its typed answer ([`ArtifactApp::host_event`]) dispatches to the event's window —
    /// addressed through `meta`'s view when it holds that window, else through the last roster — as an ordinary typed
    /// operation that is never a history row of its own. An event the app ignores, or for a window no view holds,
    /// dispatches nothing.
    async fn deliver_host_event(&mut self, event: HostEvent, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), Fault> {
        self.as_render().end_window_gesture(&event);
        let Some(command) = A::host_event(&event) else { return Ok(()) };
        let window_id = event.window_id();
        let view = meta.view_state.as_ref().and_then(|view| view.for_window_instance(window_id)).or_else(|| ViewModel { window_instances: self.host_events.host_event_roster.clone(), ..meta.view_state.as_ref()?.clone() }.for_window_instance(window_id));
        let Some(view) = view else { return Ok(()) };
        let window_meta = ActionMeta { view_state: Some(view), ..meta.clone() };
        let verb = A::command_id(&command).await;
        let payload = ::protocol::OpBinary::encode_op(&command).map_err(|error| error.into_fault())?;
        let admission = self.tools.admit_command_wire(&*self.stores, verb, &payload, 1).await?;
        self.host_events.delivering_host_event = true;
        let result = Box::pin(self.dispatch_typed_command_inner(Box::new(command), admission, &window_meta, identity, cx)).await;
        self.host_events.delivering_host_event = false;
        result.map(|_| ())
    }

    /// 🧰️ Delivers [`HostEvent::UtilityChanged`] for every window whose utility `meta`'s view moved since a dispatch
    /// last showed it — the targeted window reads the per-call overlay, every other one the view's map, and a window
    /// the view names no utility for is unknown, never a switch — and remembers the view's window roster.
    pub(super) async fn deliver_utility_host_events(&mut self, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), Fault> {
        let Some(view) = meta.view_state.as_ref().filter(|_| !self.host_events.delivering_host_event) else { return Ok(()) };
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
            self.deliver_host_event(event, meta, identity, cx).await?;
        }
        Ok(())
    }

    /// 🌐️ Delivers one instance-wide event, built per window, to every window of `meta`'s view, else of the last roster.
    pub(crate) async fn deliver_host_event_to_every_window(&mut self, event: impl Fn(String) -> HostEvent, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), Fault> {
        self.as_render().end_every_gesture(&event(String::new()));
        let roster = meta.view_state.as_ref().map(|view| view.window_instances.as_slice()).filter(|roster| !roster.is_empty()).unwrap_or(&self.host_events.host_event_roster);
        let windows: Vec<String> = roster.iter().map(|window| window.id.clone()).collect();
        for window_id in windows {
            self.deliver_host_event(event(window_id), meta, identity, cx).await?;
        }
        Ok(())
    }

    /// 🌐️ Delivers [`HostEvent::BaseMoved`] to every window once a remote edit moved the base, as the local actor.
    pub(crate) async fn deliver_base_moved(&mut self, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), Fault> {
        let Some(instance_id) = self.tools.live_runtime_instance_id else { return Ok(()) };
        let meta = ActionMeta { actor: self.stores.store.local_actor_id().0.clone(), instance_id, view_state: None };
        self.deliver_host_event_to_every_window(|window_id| HostEvent::BaseMoved { window_id }, &meta, identity, cx).await
    }

    /// 📨️ `hostEvent{windowId, kind}`: the host-forwarded window fact, delivered at once; an unknown kind or a
    /// missing window is refused.
    pub(super) async fn dispatch_host_event_action(&mut self, args: Option<&DslValue>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        let text = |key: &str| args.and_then(|value| value.get(key)).and_then(DslValue::as_str);
        let event = text(semio_framework::HOST_EVENT_ARG_WINDOW_ID).zip(text(semio_framework::HOST_EVENT_ARG_KIND)).and_then(|(window_id, kind)| HostEvent::forwarded(window_id, kind));
        let Some(event) = event else {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("hostEvent.invalid"), "hostEvent requires a windowId and a kind of blur, captureLost or retiring"));
        };
        self.deliver_host_event(event, meta, identity, cx).await?;
        let ui_scope = if self.as_render().take_gesture_ended() { UiDirtyScope::Full } else { UiDirtyScope::None };
        Ok(ToolOperations::<A>::empty_result(semio_framework::HOST_EVENT_ACTION_ID, meta, Vec::new(), Vec::new(), ui_scope).await)
    }
}
