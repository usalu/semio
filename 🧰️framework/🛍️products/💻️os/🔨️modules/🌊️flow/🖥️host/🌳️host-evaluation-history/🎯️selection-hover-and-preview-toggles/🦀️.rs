//! 🎯️ Selection, hover and preview toggles of the flow host: the selected widget ids, hover facts, the selection alignment and the preview off set. See [`super`].

use super::*;

impl FlowHost {
    /// 🎯️ Projects selected semantic widget identities.
    pub fn selected_widget_ids(&self)->Vec<String>{self.dag.graph.selected_node_ids()}

    /// 🎯️ Projects typed node, edge and handle selection.
    pub fn selection_domains(&self)->DagSelectionDomains{self.dag.graph.selection_domains()}

    /// 🖱️ Hovered widget id when the pointer is over a node or port handle.
    pub fn hovered_widget_id(&self) -> Option<String> {
        hovered_node_id(&self.dag.viewport, &self.dag.graph)
    }

    /// 🎯️ All pick targets under a screen point as JSON for DOM disambiguation menus.
    pub fn pick_targets_at_screen_json(&self, sx: f64, sy: f64) -> String {
        dag_pick_targets_at_screen_json(&self.dag.viewport, &self.dag.graph, sx, sy)
    }

    /// 🎯️ Screen-space geometry for a live entity (`domain`/`id` in the pick-target grammar) —
    /// see `DagHost::entity_screen_json`. Powers introduction-demonstration semantic targeting.
    pub fn entity_screen_json(&self, domain: &str, id: &str) -> String {
        entity_screen_json(&self.dag.viewport, &self.dag.graph, domain, id)
    }

    /// 🖱️ Projects admitted widget hover and live wire refusal facts.
    pub fn hover_facts(&self)->semio_framework_os_infinite::board::schema::dag_input::DagHoverFacts{self.dag.graph.hover_facts()}

    /// 🔌️ Projects selected typed widget channels.
    pub fn selected_channels(&self)->Vec<dag::DagChannelRef>{self.dag.graph.selected_channels()}

    /// ✅️ Replaces selection from admitted semantic domains.
    pub fn set_selection_domains(&mut self,domains:&DagSelectionDomains){self.dag.graph.set_selection_domains(domains);}

    /// ✅️ Replaces the node domain selection directly.
    pub fn set_selection(&mut self,ids:&[String]){self.dag.graph.set_selection(ids);}

    /// 📦️ Screen-space union bounds of the current selection for DOM overlays.
    pub fn selection_union_bounds_screen_json(&self) -> String {
        selection_union_bounds_screen_json(&self.dag.viewport, &self.dag.graph)
    }

    /// 📐️ Aligns or distributes the selection and journals what it moved as node-graph gesture records (design §13.3,
    /// one per distinct offset), which the renderer drains ([`Self::take_graph_edits_json`]) and dispatches; an align whose
    /// records outgrow one dispatch is refused whole and every node stays where it was.
    pub fn align_selection(&mut self, mode: &str) -> Result<(), FlowCoreError> {
        self.begin_change();
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        let baseline = self.dag.graph.node_positions();
        self.dag.graph.align_selection(mode)?;
        let refused = self.dag.graph.journal_moves_since(&dag::dag_drag_gesture_id(self.interaction_revision), &baseline).is_err();
        if refused {
            self.dag.graph.restore_node_positions(&baseline);
        }
        self.sync_from_dag();
        if !refused {
            let aligned = self.dag.graph.selected_node_ids();
            let moved = self.layout_leaf(aligned.iter().map(String::as_str));
            self.note_leaves(moved);
        }
        self.checked_change()?;
        Ok(())
    }

    /// 🖱️ Sets hover to a widget id, or clears hover.
    pub fn set_hover(&mut self, widget_id: Option<&str>) {
        self.dag.graph.set_hover(widget_id);
    }

    /// 🔌️ Sets hover to a widget channel, or clears hover.
    pub fn set_hover_channel(&mut self, widget_id: Option<&str>, port_id: Option<&str>) {
        set_hover_channel(&mut self.dag.viewport, &mut self.dag.graph, widget_id, port_id);
    }

    /// 🔌️ Replaces admitted typed channel selection.
    pub fn set_selected_channels(&mut self,channels:&[dag::DagChannelRef]){set_selected_channels(&mut self.dag.viewport, &mut self.dag.graph, channels);}

    /// 🌫️ Widget ids with preview disabled.
    pub fn preview_off_widget_ids(&self) -> Vec<String> {
        self.host_snapshot
            .widgets
            .iter()
            .filter_map(|widget| match widget {
                Widget::Neuron { id, preview: false, .. } => Some(id.clone()),
                _ => None,
            })
            .collect()
    }

    /// 🌫️ Sets preview-off neurons from a JSON array of widget ids.
    pub fn set_preview_off_json(&mut self, json: &str) {
        let ids: Vec<String> = semio_framework_pack_json::from_json_str(json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_default();
        for widget in &mut self.host_snapshot.widgets {
            if let Widget::Neuron { id, preview, .. } = widget {
                *preview = !ids.contains(id);
            }
        }
        self.sync_preview_dimmed();
    }

    /// 👁️ Toggles preview on a neuron widget.
    pub fn toggle_preview(&mut self, widget_id: &str) -> Result<(), FlowCoreError> {
        let Some(widget) = self.host_snapshot.widgets.iter_mut().find(|w| widget_id_for(w) == widget_id) else {
            return Err(FlowCoreError::UnknownWidget(widget_id.to_string()));
        };
        let Widget::Neuron { preview, .. } = widget else {
            return Err(FlowCoreError::WidgetNotNeuron(widget_id.to_string()));
        };
        *preview = !*preview;
        self.sync_preview_dimmed();
        Ok(())
    }
}
