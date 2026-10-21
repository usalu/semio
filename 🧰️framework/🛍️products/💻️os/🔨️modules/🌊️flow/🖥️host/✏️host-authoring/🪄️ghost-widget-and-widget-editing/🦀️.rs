//! 🪄️ Ghost widget and widget editing of the flow host: the ghost widget and adding, removing and moving widgets. See [`super`].

use super::*;

impl FlowHost {
    pub fn set_ghost_widget(&mut self, descriptor_json: &str, world_x: f64, world_y: f64) -> Result<(), FlowCoreError> {
        let descriptor: WidgetDescriptor = semio_framework_pack_json::from_json_str(descriptor_json, semio_framework_pack_json::JsonMemberPolicy::Reject)?;
        let id: String = "__ghost__".into();
        let widget = widget_from_descriptor(&descriptor, id.clone(), &self.kind_infos);
        let mut layout = semio_framework_value_ordered::ordered::OrderedMap::new();
        layout.insert(id, WidgetLayout { x: world_x, y: world_y });
        let mut node = widget_to_dag_node(&widget, 0, &layout, &[], &self.kind_infos, widget_node_size(&widget, &[], &self.kind_infos));
        widget.retire_cold();
        let mut retirement = semio_framework_artifact_flow_flow::retained::FlowRetirement::default();
        retirement.push_cold(semio_framework_artifact_flow_flow::retained::FlowOwner::Layouts(layout));
        retirement.retire_cold();
        fit_node_size(&mut node);
        self.ghost_node = Some(node.clone());
        self.dag.pointer.set_ghost_node(Some(node));
        Ok(())
    }

    pub fn clear_ghost_widget(&mut self) {
        self.ghost_node = None;
        self.dag.pointer.set_ghost_node(None);
    }

    pub fn add_widget(&mut self, descriptor_json: &str, world_x: f64, world_y: f64) -> Result<String, FlowCoreError> {
        let descriptor: WidgetDescriptor = semio_framework_pack_json::from_json_str(descriptor_json, semio_framework_pack_json::JsonMemberPolicy::Reject)?;
        self.add_widget_descriptor(&descriptor, world_x, world_y)
    }

    /// 🧩️ Creates a widget from the admitted native descriptor.
    pub fn add_widget_descriptor(&mut self, descriptor: &WidgetDescriptor, world_x: f64, world_y: f64) -> Result<String, FlowCoreError> {
        self.begin_change();
        self.clear_ghost_widget();
        let id = descriptor_explicit_id(descriptor).unwrap_or_else(|| self.next_widget_id(descriptor));
        if self.host_snapshot.widgets.iter().any(|widget| widget_id_for(widget) == id) {
            return Err(FlowCoreError::WidgetIdExists(id));
        }
        let widget = widget_from_descriptor(descriptor, id.clone(), &self.kind_infos);
        let added = FlowMutation::AddWidget(AddWidget { index: u32::try_from(self.host_snapshot.widgets.len()).unwrap_or(u32::MAX), widget: widget.clone() });
        self.host_snapshot.widgets.push(widget);
        self.host_snapshot.layout.insert(id.clone(), WidgetLayout { x: world_x, y: world_y });
        self.note_leaves([added].into_iter().chain(self.layout_leaf([id.as_str()])));
        self.rebuild_dag();
        self.checked_change()?;
        Ok(id)
    }

    pub fn remove_widget(&mut self, widget_id: &str) -> Result<(), FlowCoreError> {
        self.begin_change();
        let before = self.host_snapshot.widgets.len();
        let touching: Vec<String> = self.host_snapshot.synapses.iter().filter(|s| s.from == widget_id || s.to == widget_id).map(|s| s.id.clone()).collect();
        let had_layout = self.host_snapshot.layout.contains_key(widget_id);
        self.host_snapshot.widgets.retain(|w| widget_id_for(w) != widget_id);
        if self.host_snapshot.widgets.len() == before {
            return Err(FlowCoreError::UnknownWidget(widget_id.to_string()));
        }
        self.host_snapshot.layout.remove(widget_id);
        self.host_snapshot.synapses.retain(|s| s.from != widget_id && s.to != widget_id);
        let unplaced = had_layout.then(|| FlowMutation::ChangeLayout(ChangeLayout { entries: vec![FlowLayoutEntry { id: widget_id.to_string(), layout: None }] }));
        self.note_leaves(touching.into_iter().map(|id| FlowMutation::RemoveSynapse(RemoveSynapse { id })).chain(unplaced).chain([FlowMutation::RemoveWidget(RemoveWidget { id: widget_id.to_string() })]));
        self.rebuild_dag();
        self.checked_change()?;
        Ok(())
    }

    pub fn move_widget(&mut self, widget_id: &str, x: f64, y: f64) -> Result<(), FlowCoreError> {
        if !self.host_snapshot.widgets.iter().any(|w| widget_id_for(w) == widget_id) {
            return Err(FlowCoreError::UnknownWidget(widget_id.to_string()));
        }
        self.begin_change();
        self.host_snapshot.layout.insert(widget_id.to_string(), WidgetLayout { x, y });
        let moved = self.layout_leaf([widget_id]);
        self.note_leaves(moved);
        self.dag.graph.set_widget_position(widget_id, x, y)?;
        self.checked_change()?;
        Ok(())
    }
}
