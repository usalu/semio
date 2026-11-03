//! ♿️ Accessibility dispatch and focus: the accessibility event dispatch, the select check and the retained node focus kind and identity. See [`super`].

use super::*;

//#region RetainedEngineCutover
pub fn dispatch_accessibility_event(window_id: &str, window_generation: u64, node_id: u64, node_key: &str, event: semio_framework_ui_wgpu_events::AccessibilityUiEvent, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Option<Vec<semio_framework_ui_wgpu_events::UiCommand>> {
    if !accessibility_window_is_visible(window_id) {
        return None;
    }
    let text_editor = UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        if engine.surface_generation(window_id) != Some(window_generation) {
            return None;
        }
        text_editor_accessibility_entries(engine.tree(window_id)?).into_iter().find(|entry| entry.key == node_key && scene_virtual_accessibility_node_id(&entry.key) == node_id)
    });
    if let Some(entry) = text_editor {
        match event {
            semio_framework_ui_wgpu_events::AccessibilityUiEvent::Focus | semio_framework_ui_wgpu_events::AccessibilityUiEvent::Activate => {
                let focus = UI_ENGINE.with(|cell| text_editor_focus_in(&cell.borrow(), window_id, window_generation, entry.scene_node, &entry.host_id))?;
                install_text_editor_focus(focus.clone());
                reset_text_editor_caret(&focus);
            }
            semio_framework_ui_wgpu_events::AccessibilityUiEvent::Blur => {
                let focus = FOCUSED_TEXT_EDITOR.with(|cell| {
                    let mut focus = cell.borrow_mut();
                    if focus.as_ref().is_some_and(|focus| focus.window_id == window_id && focus.window_generation == window_generation && focus.node == entry.scene_node && focus.host_id == entry.host_id) {
                        return focus.take();
                    }
                    None
                });
                if let Some(focus) = focus {
                    blur_text_editor(&focus);
                }
            }
            semio_framework_ui_wgpu_events::AccessibilityUiEvent::Value(value) => {
                let focus = UI_ENGINE.with(|cell| text_editor_focus_in(&cell.borrow(), window_id, window_generation, entry.scene_node, &entry.host_id))?;
                let outcome = with_live_text_editor(&focus, |scene| semio_framework_os_renderer_wgpu_engine_text::text_editor::text_editor_replace_all_into(scene, &value, input));
                match outcome {
                    Some(Ok(_)) => {
                        reset_text_editor_caret(&focus);
                    }
                    Some(Err(fault)) => input.record_action_fault(fault),
                    None => return None,
                }
            }
        }
        return Some(Vec::new());
    }
    let table_editable = UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        if engine.surface_generation(window_id) != Some(window_generation) {
            return None;
        }
        table_editable_text_accessibility_entries(engine.tree(window_id)?).into_iter().find(|entry| entry.cell.key == node_key && scene_virtual_accessibility_node_id(&entry.cell.key) == node_id)
    });
    if let Some(entry) = table_editable {
        match event {
            semio_framework_ui_wgpu_events::AccessibilityUiEvent::Focus | semio_framework_ui_wgpu_events::AccessibilityUiEvent::Activate => {
                let result = UI_ENGINE.with(|cell| focus_table_editable_text(&cell.borrow(), window_id, window_generation, entry.scene_node, &entry.host_id, entry.cell.target.clone(), input));
                if let Err(fault) = result {
                    input.record_action_fault(fault);
                }
            }
            semio_framework_ui_wgpu_events::AccessibilityUiEvent::Blur => {
                let outcome = UI_ENGINE.with(|cell| commit_focused_table_editable_text_with_engine(&cell.borrow(), input));
                match outcome {
                    Some(Ok(semio_framework_os_renderer_wgpu_scene_tables::TableEditableTextCommitOutcome::Conflict | semio_framework_os_renderer_wgpu_scene_tables::TableEditableTextCommitOutcome::Publishing | semio_framework_os_renderer_wgpu_scene_tables::TableEditableTextCommitOutcome::AwaitingEcho)) => {}
                    Some(Err(fault)) => input.record_action_fault(fault),
                    _ => clear_focused_table_editable_text(input),
                }
            }
            semio_framework_ui_wgpu_events::AccessibilityUiEvent::Value(value) => {
                set_focused_table_editable_text_draft(window_id, entry.scene_node, &entry.host_id, &entry.cell.target.row_id, &entry.cell.target.column_id, &value);
                let outcome = UI_ENGINE.with(|cell| commit_focused_table_editable_text_with_engine(&cell.borrow(), input));
                if let Some(Err(fault)) = outcome {
                    input.record_action_fault(fault);
                }
            }
        }
        return Some(Vec::new());
    }
    let table_stepper = UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        if engine.surface_generation(window_id) != Some(window_generation) {
            return None;
        }
        table_stepper_accessibility_entries(engine.tree(window_id)?).into_iter().find(|entry| entry.cell.key == node_key && scene_virtual_accessibility_node_id(&entry.cell.key) == node_id)
    });
    if let Some(entry) = table_stepper {
        match event {
            semio_framework_ui_wgpu_events::AccessibilityUiEvent::Focus | semio_framework_ui_wgpu_events::AccessibilityUiEvent::Activate => focus_table_stepper(window_id, window_generation, entry.scene_node, &entry.host_id, entry.cell.target),
            semio_framework_ui_wgpu_events::AccessibilityUiEvent::Blur => {
                FOCUSED_TABLE_STEPPER.with(|cell| {
                    let mut focus = cell.borrow_mut();
                    if focus.as_ref().is_some_and(|focus| {
                        focus.window_id == window_id
                            && focus.window_generation == window_generation
                            && focus.node == entry.scene_node
                            && focus.host_id == entry.host_id
                            && focus.row_id == entry.cell.target.row_id
                            && focus.column_id == entry.cell.target.column_id
                    }) {
                        *focus = None;
                    }
                });
            }
            semio_framework_ui_wgpu_events::AccessibilityUiEvent::Value(_) => {}
        }
        return Some(Vec::new());
    }
    let table_button = UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        if engine.surface_generation(window_id) != Some(window_generation) {
            return None;
        }
        table_button_accessibility_entries(engine.tree(window_id)?).into_iter().find(|entry| entry.cell.key == node_key && scene_virtual_accessibility_node_id(&entry.cell.key) == node_id)
    });
    if let Some(entry) = table_button {
        if matches!(event, semio_framework_ui_wgpu_events::AccessibilityUiEvent::Activate) {
            let outcome = UI_ENGINE.with(|cell| {
                let engine = cell.borrow();
                let retained = engine.tree(window_id).and_then(|tree| tree.node(entry.scene_node))?;
                let UiNode::ComponentScene(scene) = &retained.spec.0 else { return None };
                (scene.host_id == entry.host_id && scene.component_kind == semio_framework_ui_declaration::ui::SurfaceKind::Table).then_some(())?;
                semio_framework_os_renderer_wgpu_scene_tables::table_button_accessibility_activate(scene, &entry.cell.key, input)
            });
            match outcome {
                Some(Ok(())) => {}
                Some(Err(fault)) => input.record_action_fault(fault),
                None => return None,
            }
        }
        return Some(Vec::new());
    }
    let vfs_control = UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        if engine.surface_generation(window_id) != Some(window_generation) {
            return None;
        }
        vfs_accessibility_entries(engine.tree(window_id)?).into_iter().find(|entry| entry.control.key == node_key && scene_virtual_accessibility_node_id(&entry.control.key) == node_id)
    });
    if let Some(entry) = vfs_control {
        let target = semio_framework_os_renderer_wgpu_scene_state::VfsAccessibilityFocusTarget { row_id: entry.control.row_id, kind: entry.control.kind };
        match event {
            semio_framework_ui_wgpu_events::AccessibilityUiEvent::Focus => focus_vfs_control(window_id, window_generation, entry.scene_node, &entry.host_id, target),
            semio_framework_ui_wgpu_events::AccessibilityUiEvent::Activate => {
                focus_vfs_control(window_id, window_generation, entry.scene_node, &entry.host_id, target.clone());
                let outcome = UI_ENGINE.with(|cell| {
                    let engine = cell.borrow();
                    let retained = engine.tree(window_id).and_then(|tree| tree.node(entry.scene_node))?;
                    let UiNode::ComponentScene(scene) = &retained.spec.0 else { return None };
                    (scene.host_id == entry.host_id && scene.component_kind == semio_framework_ui_declaration::ui::SurfaceKind::VirtualFileSystem).then_some(())?;
                    semio_framework_os_renderer_wgpu_scene_text_vfs::vfs_accessibility_activate(scene, &target, input)
                });
                match outcome {
                    Some(Ok(())) => {}
                    Some(Err(fault)) => input.record_action_fault(fault),
                    None => return None,
                }
            }
            semio_framework_ui_wgpu_events::AccessibilityUiEvent::Blur => {
                FOCUSED_VFS_CONTROL.with(|cell| {
                    let mut focus = cell.borrow_mut();
                    if focus.as_ref().is_some_and(|focus| focus.window_id == window_id && focus.window_generation == window_generation && focus.node == entry.scene_node && focus.host_id == entry.host_id && focus.target == target) {
                        *focus = None;
                    }
                });
            }
            semio_framework_ui_wgpu_events::AccessibilityUiEvent::Value(_) => {}
        }
        return Some(Vec::new());
    }
    let block_list_control = UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        if engine.surface_generation(window_id) != Some(window_generation) {
            return None;
        }
        block_list_accessibility_entries(engine.tree(window_id)?).into_iter().find(|entry| entry.control.key == node_key && scene_virtual_accessibility_node_id(&entry.control.key) == node_id)
    });
    if let Some(entry) = block_list_control {
        if matches!(event, semio_framework_ui_wgpu_events::AccessibilityUiEvent::Activate) {
            let outcome = UI_ENGINE.with(|cell| {
                let engine = cell.borrow();
                let retained = engine.tree(window_id).and_then(|tree| tree.node(entry.scene_node))?;
                let UiNode::ComponentScene(scene) = &retained.spec.0 else { return None };
                (scene.host_id == entry.host_id && scene.component_kind == semio_framework_ui_declaration::ui::SurfaceKind::BlockList).then_some(())?;
                semio_framework_os_renderer_wgpu_scene_tables::block_list_accessibility_activate(scene, &entry.control.key, input)
            });
            match outcome {
                Some(Ok(())) => {}
                Some(Err(fault)) => input.record_action_fault(fault),
                None => return None,
            }
        }
        return Some(Vec::new());
    }
    let event_feed_control = UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        if engine.surface_generation(window_id) != Some(window_generation) {
            return None;
        }
        event_feed_accessibility_entries(engine.tree(window_id)?).into_iter().find(|entry| entry.control.key == node_key && scene_virtual_accessibility_node_id(&entry.control.key) == node_id)
    });
    if let Some(entry) = event_feed_control {
        if matches!(event, semio_framework_ui_wgpu_events::AccessibilityUiEvent::Activate) {
            let outcome = UI_ENGINE.with(|cell| {
                let engine = cell.borrow();
                let retained = engine.tree(window_id).and_then(|tree| tree.node(entry.scene_node))?;
                let UiNode::ComponentScene(scene) = &retained.spec.0 else { return None };
                (scene.host_id == entry.host_id && scene.component_kind == semio_framework_ui_declaration::ui::SurfaceKind::EventFeed).then_some(())?;
                semio_framework_os_renderer_wgpu_scene_feeds::event_feed_accessibility_activate(scene, &entry.control.key, input)
            });
            match outcome {
                Some(Ok(())) => {}
                Some(Err(fault)) => input.record_action_fault(fault),
                None => return None,
            }
        }
        return Some(Vec::new());
    }
    let graph_timeline_control = UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        if engine.surface_generation(window_id) != Some(window_generation) {
            return None;
        }
        graph_timeline_accessibility_entries(engine.tree(window_id)?).into_iter().find(|entry| entry.control.key == node_key && scene_virtual_accessibility_node_id(&entry.control.key) == node_id)
    });
    if let Some(entry) = graph_timeline_control {
        if matches!(event, semio_framework_ui_wgpu_events::AccessibilityUiEvent::Activate) {
            let outcome = UI_ENGINE.with(|cell| {
                let engine = cell.borrow();
                let retained = engine.tree(window_id).and_then(|tree| tree.node(entry.scene_node))?;
                let UiNode::ComponentScene(scene) = &retained.spec.0 else { return None };
                (scene.host_id == entry.host_id && scene.component_kind == semio_framework_ui_declaration::ui::SurfaceKind::GraphTimeline).then_some(())?;
                semio_framework_os_renderer_wgpu_scene_feeds::graph_timeline_accessibility_activate(scene, &entry.control.key, input)
            });
            match outcome {
                Some(Ok(())) => {}
                Some(Err(fault)) => input.record_action_fault(fault),
                None => return None,
            }
        }
        return Some(Vec::new());
    }
    let (mut commands, clock) = UI_ENGINE.with(|cell| {
        let mut engine = cell.borrow_mut();
        let clock = stamp_window_clock(&mut engine, window_id);
        let dispatched = engine.dispatch_accessibility_event(window_id, window_generation, node_id, node_key, event)?;
        let mut commands = engine.drain_commands();
        commands.extend(dispatched);
        Some((commands, clock))
    })?;
    apply_window_clock_caret(window_id, clock.as_ref());
    apply_ui_commands(&mut commands, None, input);
    Some(commands)
}

pub fn retained_node_is_select(window_id: &str, node_id: NodeId) -> bool {
    UI_ENGINE.with(|cell| cell.borrow().tree(window_id).and_then(|tree| tree.node(node_id)).is_some_and(|node| matches!(node.spec.0, UiNode::Select(_))))
}

/// 🧩️ Closed component kind retained with a focused reconciliation key.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedNodeFocusKind {
    Stack,
    Text,
    Button,
    Separator,
    Input,
    Select,
    Toggle,
    KeyValue,
    Slider,
    NumberStepper,
    Ring,
    IconSelect,
    Progress,
    Field,
    Section,
    Group,
    Tree,
    Image,
    ComponentScene,
    ExternalSlot,
}

impl RetainedNodeFocusKind {
    fn from_node(node: &UiNode) -> Self {
        match node {
            UiNode::Stack(_) => Self::Stack,
            UiNode::Text(_) => Self::Text,
            UiNode::Button(_) => Self::Button,
            UiNode::Separator(_) => Self::Separator,
            UiNode::Input(_) => Self::Input,
            UiNode::Select(_) => Self::Select,
            UiNode::Toggle(_) => Self::Toggle,
            UiNode::KeyValue(_) => Self::KeyValue,
            UiNode::Slider(_) => Self::Slider,
            UiNode::NumberStepper(_) => Self::NumberStepper,
            UiNode::Ring(_) => Self::Ring,
            UiNode::IconSelect(_) => Self::IconSelect,
            UiNode::Progress(_) => Self::Progress,
            UiNode::Field(_) => Self::Field,
            UiNode::Section(_) => Self::Section,
            UiNode::Group(_) => Self::Group,
            UiNode::Tree(_) => Self::Tree,
            UiNode::Image(_) => Self::Image,
            UiNode::ComponentScene(_) => Self::ComponentScene,
            UiNode::ExternalSlot(_) => Self::ExternalSlot,
        }
    }
}

/// 🔑️ Captures the reconciliation key and component kind held by a focused retained node.
pub fn retained_node_focus_identity(window_id: &str, node_id: NodeId) -> Option<(semio_framework_ui_retained_tree::tree::NodeKey, RetainedNodeFocusKind)> {
    UI_ENGINE.with(|cell| cell.borrow().tree(window_id).and_then(|tree| tree.node(node_id)).map(|node| (node.key.clone(), RetainedNodeFocusKind::from_node(&node.spec.0))))
}

/// 🎯️ Rejects stale node generations, replacement component kinds, and hidden surfaces before retained keyboard dispatch.
pub fn retained_node_has_visible_focus(window_id: &str, node_id: NodeId, expected_key: &semio_framework_ui_retained_tree::tree::NodeKey, expected_kind: RetainedNodeFocusKind) -> bool {
    accessibility_window_is_visible(window_id)
        && UI_ENGINE.with(|cell| {
            let engine = cell.borrow();
            engine.window_has_focus(window_id) && engine.tree(window_id).and_then(|tree| tree.node(node_id)).is_some_and(|node| &node.key == expected_key && RetainedNodeFocusKind::from_node(&node.spec.0) == expected_kind)
        })
}
/// ✏️ One control a tree row holds, as the retained document describes it: the key its author gave it, the document node id an accessibility event addresses it by, the kind name the row-control law reads (`text`, `longText`, `number`, `date`, `color`, `file` for an input; `button`, `toggle`, `select`, `slider`, `stepper` or `other`) and whether it is disabled.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RetainedRowControl {
    pub key: String,
    pub node_id: u64,
    pub kind: String,
    pub disabled: bool,
}

/// ✏️ The controls of one tree row in row order (a nested row's controls are its own), with the window that presents the row and the generation an accessibility event must carry.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RetainedRowControls {
    pub window_id: String,
    pub window_generation: u64,
    pub controls: Vec<RetainedRowControl>,
}

fn row_control_kind(component: &ui_contract::Component) -> &'static str {
    match component {
        ui_contract::Component::Input(input) => match input.kind {
            ui_contract::InputKind::Text => "text",
            ui_contract::InputKind::LongText => "longText",
            ui_contract::InputKind::Number => "number",
            ui_contract::InputKind::Date => "date",
            ui_contract::InputKind::Color => "color",
            ui_contract::InputKind::File => "file",
        },
        ui_contract::Component::Button(_) => "button",
        ui_contract::Component::Toggle(_) => "toggle",
        ui_contract::Component::Select(_) => "select",
        ui_contract::Component::Slider(_) => "slider",
        ui_contract::Component::NumberStepper(_) => "stepper",
        _ => "other",
    }
}

/// 🔎️ The controls of the tree row authored as `row_key`, from the first window (in id order) whose document holds such a row; `None` while no window presents it. Backs the `focusRowControl` verb; the rule that picks the cell is [`semio_framework::plan_row_control_focus`].
pub fn retained_row_controls(row_key: &str) -> Option<RetainedRowControls> {
    UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        let mut windows: Vec<&str> = engine.window_ids().collect();
        windows.sort_unstable();
        windows.into_iter().find_map(|window_id| {
            let window_generation = engine.surface_generation(window_id)?;
            let document = engine.tree(window_id)?.document()?;
            let row = document.nodes.values().find(|record| record.key.as_str() == row_key && matches!(record.component, ui_contract::Component::TreeItem(_)))?;
            let mut controls = Vec::new();
            let mut pending = vec![(row.id, 0usize)];
            while let Some((id, depth)) = pending.pop() {
                let Some(record) = document.record(id) else { continue };
                if depth > 0 && matches!(record.component, ui_contract::Component::TreeItem(_)) {
                    continue;
                }
                if depth > 0 && !matches!(record.component, ui_contract::Component::Container(_)) {
                    controls.push(RetainedRowControl { key: record.key.as_str().to_string(), node_id: record.id.0, kind: row_control_kind(&record.component).to_string(), disabled: record.disabled });
                }
                let children: Vec<_> = record.children.iter().map(|child| *child).collect();
                pending.extend(children.into_iter().rev().map(|child| (child, depth + 1)));
            }
            Some(RetainedRowControls { window_id: window_id.to_string(), window_generation, controls })
        })
    })
}

/// ⌨️ The authored key of the tree row that holds the keyboard focus of `window_id`: `Some` while the focused node is a tree row itself, `None` when it is a control inside a row (that control owns the key) or anything else.
pub fn retained_focused_row_key(window_id: &str, node_id: NodeId) -> Option<String> {
    UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        let tree = engine.tree(window_id)?;
        let record = tree.document()?.record(tree.document_id(node_id)?)?;
        matches!(record.component, ui_contract::Component::TreeItem(_)).then(|| record.key.as_str().to_string())
    })
}

//#endregion RetainedEngineCutover
