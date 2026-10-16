//! 📪️ Staged slice of the `instance-root` band of the mounted artifact instance: its `instance-jobs` members (WP6 step S0 of the instance family).
//!
//! 🧵️ The mounted artifact instance `VcsArtifactApp`: its state, its turns and its `PluginApp` implementation.
use super::*;

pub(crate) fn original_render_posted_since(before:RetainedCloneProgress,after:RetainedCloneProgress)->Result<RetainedCloneProgress,ValueError>{
 let delta=|after:usize,before:usize|after.checked_sub(before).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original render recipient moved backwards"));
 Ok(RetainedCloneProgress{copied_items:delta(after.copied_items,before.copied_items)?,copied_bytes:delta(after.copied_bytes,before.copied_bytes)?,retained_capacity_bytes:delta(after.retained_capacity_bytes,before.retained_capacity_bytes)?,released_bytes:delta(after.released_bytes,before.released_bytes)?})
}

pub(crate) struct OriginalRenderTreeIdentity {
    pub(crate) operation: semio_framework_job::OperationId,
    pub(crate) generation: semio_framework_job::Generation,
    pub(crate) parent: usize,
    pub(crate) recipient: usize,
    pub(crate) cancellation: usize,
    pub(crate) epoch: Option<u64>,
    pub(crate) clock: fn()->Option<u64>,
    pub(crate) deadline: u64,
}

impl OriginalRenderTreeIdentity {
    pub(crate) fn capture(cx:&semio_framework_job::StepContext<'_>)->Result<Self,ValueError> {
        Ok(Self { operation:cx.operation(), generation:cx.generation(), parent:cx.original_source_parent()?.source_identity()?, recipient:cx.original_recipient_identity(), cancellation:cx.original_cancel_token() as *const _ as usize, epoch:cx.original_source_epoch(), clock:cx.clock_source(), deadline:cx.deadline_us() })
    }
    pub(crate) fn validate(&mut self,cx:&semio_framework_job::StepContext<'_>)->Result<(),ValueError> {
        if self.operation!=cx.operation() || self.generation!=cx.generation() || self.parent!=cx.original_source_parent()?.source_identity()? || self.recipient!=cx.original_recipient_identity() || self.cancellation!=cx.original_cancel_token() as *const _ as usize || !std::ptr::fn_addr_eq(self.clock,cx.clock_source()) || self.deadline!=cx.deadline_us() || matches!((self.epoch,cx.original_source_epoch()),(Some(before),Some(after)) if after<before) || self.epoch.is_some()!=cx.original_source_epoch().is_some() {
            return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"stationary render source requires its original parent recipient control and work lifetime"));
        }
        self.epoch=cx.original_source_epoch();
        Ok(())
    }
}

pub(crate) struct OriginalRenderInteractionStamp<'a> {
    pub(crate) registry: &'a AppActionRegistry,
    pub(crate) interaction_hover: &'a InteractionHoverState,
    pub(crate) peer_presence: &'a PeerPresenceRoot,
    pub(crate) own_color: Option<u8>,
    pub(crate) local_interaction_authority: &'a mut crate::local_interaction::authority::LocalInteractionTopologyAuthority,
    pub(crate) interaction_ui_topology: &'a mut HashMap<String, protocol::DomainTopology>,
    pub(crate) pending_presence: &'a mut Vec<PresenceUpdate>,
    pub(crate) presence_marked_keys: &'a mut BTreeMap<String, BTreeSet<String>>,
}

impl OriginalRenderInteractionStamp<'_> {
    pub(crate) async fn stamp_and_cache_interaction_ui(&mut self, tree: &ComponentTree, state: &protocol::InteractionState, body_key: &str) -> UiAssemblyResult<()> {
        let hover = self.interaction_hover;
        let peers = self.peer_presence;
        let own_color = self.own_color;
        let interaction = InteractionView { state, hover, peers, draft_references: empty_draft_references() };
        struct Frame<'a> {
            node: &'a TreeNode,
            next: usize,
            entered: bool,
            domain: Option<UiText>,
        }
        let mut stack: UiFixedList<Frame<'_>, { semio_framework_ui_runtime::COMPONENT_TREE_PRODUCER_DEPTH }> = UiFixedList::default();
        let mut marked: BTreeSet<String> = BTreeSet::new();
        stack.try_push(Frame { node: &tree.root, next: 0, entered: false, domain: None }).map_err(|_| ui_assembly_error("interaction-walk.depth"))?;
        while let Some(mut frame) = stack.pop() {
            if !frame.entered {
                if let Component::Tree(props) = &frame.node.component {
                    if let Some(domain_id) = props.interaction_domain.clone() {
                        let granularity = self.registry.interaction(domain_id.as_str()).await.and_then(|def| def.granularities.first()).map(|granularity| granularity.id.clone()).unwrap_or_default();
                        let topology = ui_tree_domain_topology(&frame.node.children, &granularity).await?;
                        self.local_interaction_authority.before_cache_mutation().map_err(ui_assembly_error)?;
                        self.interaction_ui_topology.insert(domain_id.to_string(), topology);
                        frame.domain = Some(domain_id);
                    }
                }
                if let Some(domain_id) = frame.domain.as_ref() {
                    if self.derive_node_presence(frame.node, state, &interaction, domain_id.as_str(), body_key, own_color).await? {
                        marked.insert(frame.node.key.to_string());
                    }
                }
                frame.entered = true;
            }
            let Some(child) = frame.node.children.get(frame.next) else { continue };
            frame.next = frame.next.checked_add(1).ok_or_else(|| ui_assembly_error("interaction-walk.cursor"))?;
            let child_domain = frame.domain.clone();
            stack.try_push(frame).map_err(|_| ui_assembly_error("interaction-walk.depth"))?;
            stack.try_push(Frame { node: child, next: 0, entered: false, domain: child_domain }).map_err(|_| ui_assembly_error("interaction-walk.depth"))?;
        }
        self.retire_dropped_presence(body_key, &marked)?;
        Ok(())
    }

    /// 🧹️ Pushes one CLEARED `PresenceUpdate` per node key this body marked last render and did not
    /// mark this render, then records `marked` as the new set. `derive_node_presence` reports only
    /// nodes with something to say and `PresenceHub::record_own` holds own presence until it is
    /// overwritten, so this diff is the only write that ever says "this row is no longer selected" —
    /// without it a deselect is invisible and every renderer keeps the stale mark.
    fn retire_dropped_presence(&mut self, body_key: &str, marked: &BTreeSet<String>) -> UiAssemblyResult<()> {
        let previous = self.presence_marked_keys.get(body_key).cloned().unwrap_or_default();
        let surface = SurfaceId::try_from(body_key).map_err(|_| ui_assembly_error("presence.surface"))?;
        for key in previous.difference(marked) {
            self.pending_presence.push(PresenceUpdate { surface: surface.clone(), node_key: key.clone(), own: OwnPresence::default(), peers: Vec::new(), ttl_ms: PRESENCE_TTL_MS });
        }
        if marked.is_empty() {
            self.presence_marked_keys.remove(body_key);
        } else {
            self.presence_marked_keys.insert(body_key.to_string(), marked.clone());
        }
        Ok(())
    }

    /// 👥️ M2: derives `node`'s `PresenceUpdate` (if it has any own/peer selection or hover on
    /// `domain_id`) and pushes it onto `self.pending_presence`, answering whether it marked the node.
    /// A node with nothing to report is skipped entirely — an idle node costs no outbox entry,
    /// matching `PresenceHub::flush`'s own "only dirty keys" contract downstream; the `false` answer
    /// is what `retire_dropped_presence` diffs against to retire a mark this render dropped.
    async fn derive_node_presence(&mut self, node: &TreeNode, state: &protocol::InteractionState, interaction: &InteractionView<'_>, domain_id: &str, body_key: &str, own_color: Option<u8>) -> UiAssemblyResult<bool> {
        let key = node.key.as_str();
        let own_selected = state.selection.get(domain_id).is_some_and(|selection| selection.ids.iter().any(|id| id == key));
        let own_hovered = self.interaction_hover.get(domain_id).is_some_and(|hover| hover.ids.iter().any(|id| id == key));
        let selecting = interaction.peers_selecting(domain_id, key);
        let hovering = interaction.peers_hovering(domain_id, key);
        if !own_selected && !own_hovered && selecting.is_empty() && hovering.is_empty() {
            return Ok(false);
        }
        let mut marks: UiFixedList<PeerMark> = UiFixedList::default();
        for mark in &selecting {
            let existing = marks.iter().position(|entry| entry.actor == mark.actor);
            if let Some(index) = existing {
                if let Some(entry) = marks.get_mut(index) {
                    entry.selected = true;
                }
            } else {
                marks.try_push(PeerMark { actor: mark.actor.to_string(), color: mark.color, hovered: false, selected: true, label: mark.actor.to_string() }).map_err(|_| ui_assembly_error("presence.peer-marks"))?;
            }
        }
        for mark in &hovering {
            let existing = marks.iter().position(|entry| entry.actor == mark.actor);
            if let Some(index) = existing {
                if let Some(entry) = marks.get_mut(index) {
                    entry.hovered = true;
                }
            } else {
                marks.try_push(PeerMark { actor: mark.actor.to_string(), color: mark.color, hovered: true, selected: false, label: mark.actor.to_string() }).map_err(|_| ui_assembly_error("presence.peer-marks"))?;
            }
        }
        self.pending_presence.push(PresenceUpdate {
            surface: SurfaceId::try_from(body_key).map_err(|_| ui_assembly_error("presence.surface"))?,
            node_key: node.key.to_string(),
            own: OwnPresence { hovered: own_hovered, selected: own_selected, previewed: false, color: own_color },
            peers: marks.into_iter().collect(),
            ttl_ms: PRESENCE_TTL_MS,
        });
        Ok(true)
    }

}

/// ⏱️ M2 (ticket 26/08/17 `design-unified.md`): default TTL for every `PresenceUpdate`
/// `stamp_and_cache_interaction_ui` derives — a peer mark not refreshed within this window ages
/// out on the reactor's `PresenceHub` with no goodbye message (matches the design's decided
/// default; own presence never expires by this or any other TTL — see `ui_runtime::PresenceHub`'s
/// own doc).
const PRESENCE_TTL_MS: u32 = 4_000;
