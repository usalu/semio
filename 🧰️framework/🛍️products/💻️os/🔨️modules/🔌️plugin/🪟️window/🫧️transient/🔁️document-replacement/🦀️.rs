/// 🔁️ Shared document replacement invalidates concrete window transients and retains displaced owners.
impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A, M> {
    fn prepare_document_window_reset(&self) -> Result<WindowTransientOwnerRegistry, Fault> {
        if self.close_started {
            return Err(plugin_sdk_fault("a closing app cannot replace its document window owners"));
        }
        let generation = self.window_transient_store.document_generation().checked_add(1).ok_or_else(|| plugin_sdk_fault("document window transient generation is exhausted"))?;
        if !self.window_transient_store.terminal_is_empty() && !self.retired_window_transient_stores.can_insert(generation) {
            return Err(plugin_sdk_fault("document replacement awaits bounded window transient retirement capacity"));
        }
        let mut replacement = WindowTransientOwnerRegistry::for_document_generation(generation);
        A::register_window_transient_owners(&mut replacement)?;
        Ok(replacement)
    }

    fn commit_document_window_reset(&mut self, replacement: WindowTransientOwnerRegistry) {
        let generation = replacement.document_generation();
        let displaced = std::mem::replace(&mut self.window_transient_store, replacement);
        if !displaced.terminal_is_empty() {
            self.retired_window_transient_stores.insert_admitted(generation, displaced);
        }
        self.tool_cancellations.renew_scope_generation();
        self.cache = None;
    }

    fn retire_document_windows_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        if maximum_items == 0 {
            return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        let Some((_, generation)) = self.retired_window_transient_stores.next_id_from(0) else { return Ok(PluginCloseStep::Complete) };
        let retired = self.retired_window_transient_stores.get_mut(generation).expect("selected window transient retirement remains owned");
        let step = retired.close_step(maximum_items.min(1), maximum_bytes)?;
        if step != PluginCloseStep::Complete {
            return Ok(step);
        }
        if !retired.terminal_is_empty() {
            return Err(plugin_sdk_fault("document window transient retirement reported complete with live owners"));
        }
        drop(self.retired_window_transient_stores.remove(generation));
        Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 })
    }
}
