//! 📣️ Borrows the mounted instance's original host-event routing owners.
use super::*;
impl<A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + Send + 'static> VcsArtifactApp<A, M> {
 pub(crate) fn shared_command_source_pending(&self)->bool{self.host_events.delivering||!self.host_events.action_delivery.terminal_is_empty()||self.host_events.routes.cache_busy()||self.tools.original_tool_tag_capture.is_some()||self.tools.original_tool_tag_retirement.is_some()||self.tools.original_scrub_dispatch.is_some()||self.tools.original_press_settlement.is_some()||self.tools.original_typing_settlement.is_some()||self.tools.original_tool_follow.is_some()||self.original_typed_tag.is_some()||!self.original_typed_tag_close.is_empty()||self.original_typed_command.pending()||self.original_typing.pending()||self.original_emission.pending()}
 pub(crate) fn host_event_routing_ctx(&mut self) -> semio_plugin_instance_dispatch::host_event_routing::HostEventRoutingCtx<'_, A> {
  let shared_pending = self.shared_command_source_pending();
  semio_plugin_instance_dispatch::host_event_routing::HostEventRoutingCtx { host_events: &mut self.host_events, tools: &mut self.tools, stores: &mut self.stores, original_host_event_command: &mut self.original_host_event_command, original_host_event_scope: &mut self.original_host_event_scope, original_command_scope: &mut self.original_command_scope, shared_pending }
 }
 pub(crate) fn host_event_routing_ref(&self) -> semio_plugin_instance_dispatch::host_event_routing::HostEventRoutingRef<'_, A> { semio_plugin_instance_dispatch::host_event_routing::HostEventRoutingRef { host_events: &self.host_events, original_host_event_command: &self.original_host_event_command, original_host_event_scope: &self.original_host_event_scope, original_command_scope: &self.original_command_scope } }

}
