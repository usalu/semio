struct MountedCommandPrune {
    cursor: CommandPrune,
    requested: u64,
    revision: [u8; 32],
    generation: u64,
    edits: usize,
    transitions: usize,
    field: usize,
    index: usize,
    offset: usize,
    keep: bool,
}

impl MountedCommandPrune {
    fn equal_prefix(left: &str, right: &str, offset: &mut usize) -> Option<bool> {
        if left.len() != right.len() { *offset = 0; return Some(false); }
        let end = (*offset + (left.len() - *offset).min(64)).min(left.len());
        if left.as_bytes()[*offset..end] != right.as_bytes()[*offset..end] { *offset = 0; return Some(false); }
        *offset = end;
        (end == left.len()).then_some(true)
    }
}

impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A, M> {
    fn command_prune_requested(&self) -> bool { self.command_prune_generation != self.command_prune_processed }
    fn command_prune_next_release_byte_demand(&self) -> usize {
        if self.pending_command_prune.is_some() { 0 } else if !self.command_log.pruned_terminal_is_empty() { self.command_log.next_pruned_close_byte_demand().unwrap_or(1) } else { 0 }
    }
    /// 🧹️ Advances one original metadata comparison, checked visibility decision or funded hidden owner.
    fn advance_command_prune_step(&mut self, maximum_items: usize, maximum_bytes: usize, closing: bool) -> Result<PluginCloseStep, Fault> {
        let pending = PluginCloseStep::Pending { released_items: 0, released_bytes: 0 };
        if maximum_items == 0 { return Ok(pending); }
        if closing {
            if let Some(mut owner) = self.pending_command_prune.take() {
                self.command_log.cancel_prune(&mut owner.cursor, 1).map_err(|_| plugin_sdk_fault("mounted command visibility lost its original cancellation lease"))?;
                self.command_prune_processed = self.command_prune_generation;
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if self.command_prune_requested() {
                self.command_prune_processed = self.command_prune_generation;
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
        }
        if self.pending_command_prune.is_none() && !self.command_log.pruned_terminal_is_empty() {
            let demand = self.command_log.next_pruned_close_byte_demand().map_err(|error| plugin_sdk_fault(error.into_message()))?;
            if maximum_bytes < demand { return Ok(pending); }
            if let Some(record) = self.command_log.next_pruned_record() {
                self.shell_undone.remove(&record.seq);
                self.history_dirty_sequences.remove(&record.seq);
            }
            let step = self.command_log.close_pruned_step(RetainedCloneGrant { maximum_items: 1, maximum_capacity_bytes: 0, maximum_copy_bytes: 0, maximum_release_bytes: maximum_bytes, maximum_depth: 64 }).map_err(|error| plugin_sdk_fault(error.into_message()))?;
            return Ok(PluginCloseStep::Pending { released_items: step.progress().copied_items, released_bytes: step.progress().released_bytes });
        }
        if self.pending_command_prune.is_none() {
            if !self.command_prune_requested() { return Ok(PluginCloseStep::Complete); }
            self.refresh_supersede_ledger();
            let cursor = self.command_log.begin_prune().map_err(|_| plugin_sdk_fault("mounted command visibility requires its original idle history owner"))?;
            self.pending_command_prune = Some(MountedCommandPrune { cursor, requested: self.command_prune_generation, revision: self.store.content_revision_now(), generation: self.store.generation_now(), edits: self.store.envelope().vcs.edits.len(), transitions: self.supersedes.records.len(), field: 0, index: 0, offset: 0, keep: true });
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        let mut owner = self.pending_command_prune.take().unwrap();
        let stale = owner.requested != self.command_prune_generation || owner.revision != self.store.content_revision_now() || owner.generation != self.store.generation_now() || owner.edits != self.store.envelope().vcs.edits.len() || owner.transitions != self.supersedes.records.len();
        let record = if stale { None } else { self.command_log.prune_current(&owner.cursor).ok() };
        let Some(record) = record else {
            self.command_log.cancel_prune(&mut owner.cursor, 1).map_err(|_| plugin_sdk_fault("stale command visibility requires its exact retained cancellation"))?;
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        };
        let Some(record) = record else {
            let removed = self.command_log.commit_prune(&mut owner.cursor).map_err(|_| plugin_sdk_fault("mounted command visibility changed before its constant decision"))?;
            self.command_prune_processed = owner.requested;
            if removed != 0 { self.history_backfill = None; self.log_generation += 1; }
            return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        };
        match owner.field {
            0 => match record.edit_id.as_deref() {
                None => { owner.field = 1; owner.index = 0; owner.offset = 0; },
                Some(id) => match self.store.envelope().vcs.edits.get(owner.index) {
                    None => { owner.keep = false; owner.field = 2; },
                    Some(edit) => match MountedCommandPrune::equal_prefix(id, &edit.id, &mut owner.offset) {
                        Some(true) => { owner.field = 1; owner.index = 0; owner.offset = 0; },
                        Some(false) => { owner.index += 1; },
                        None => {},
                    },
                },
            },
            1 => match record.transition_id.as_deref() {
                None => { owner.field = 2; },
                Some(id) => match self.supersedes.records.get(owner.index) {
                    None => { owner.keep = false; owner.field = 2; },
                    Some(transition) => match MountedCommandPrune::equal_prefix(id, &transition.transition_id, &mut owner.offset) {
                        Some(true) => { owner.field = 2; },
                        Some(false) => { owner.index += 1; },
                        None => {},
                    },
                },
            },
            _ => {
                self.command_log.advance_prune(&mut owner.cursor, owner.keep, RetainedCloneGrant { maximum_items: 1, maximum_capacity_bytes: 0, maximum_copy_bytes: 64, maximum_release_bytes: 0, maximum_depth: 64 }).map_err(|_| plugin_sdk_fault("prepared command visibility changed before its original row link"))?;
                owner.field = 0; owner.index = 0; owner.offset = 0; owner.keep = true;
            },
        }
        self.pending_command_prune = Some(owner);
        Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 })
    }
}
