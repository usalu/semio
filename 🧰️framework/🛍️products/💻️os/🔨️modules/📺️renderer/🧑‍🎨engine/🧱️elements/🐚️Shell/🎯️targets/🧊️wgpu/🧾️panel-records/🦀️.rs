//! 🧾️ Panel records assembler, tool-run panel and app-catalogue attempt.
//!
//! `panel_ui_records`, `PanelProjection`, the settings/staged-argument/agent-chat row helpers, the tool-run
//! panel reveal law and the app-catalogue attempt.
//!
//! One chunk of the former `shell` monolith (ticket CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT); [`ShellState`] and the
//! shared imports stay in the parent module and are reached through `use super::*;`.

use super::*;

//#region ShellLifecycle
//#region 🧾️PanelRecordsAssembler
/// 🧾️ Projects one SHELL-OWNED `UiNode` subtree into the `ui_contract` records a retained panel
/// document carries, pre-order so every parent precedes its children. W1h could only publish the
/// Commands panel by re-expressing it as `WindowMeasure`s (a `Select`/`Toggle` vocabulary with no
/// text, no button and no nesting); this is the general assembler, so a shell-owned panel body is
/// authored once as a `UiNode` — the same dialect React's `📌️ChromePanels/🟦️.tsx` hands its
/// `buildSettings*Tree` output straight to the Interpreter in — and published as-is.
///
/// A node's `key` is its own authored id when it has one (so hits and bindings address the control by
/// the id React uses), else a positional `n<seq>` key inside the surface, which is stable because the
/// walk order is.
pub(crate) fn panel_ui_records(surface_id: &str, root: &UiNode) -> Result<Vec<ui_contract::UiNodeRecord>, String> {
    let mut projection = PanelProjection { surface_id, records: Vec::new(), key_seq: 0 };
    projection.node(root)?;
    projection.records.into_iter().collect::<Option<Vec<_>>>().ok_or_else(|| format!("panel '{surface_id}' projection left a reserved record unplaced"))
}

pub(crate) fn panel_ui_scroll_records(surface_id: &str, root: &UiNode) -> Result<Vec<ui_contract::UiNodeRecord>, String> {
    let mut projection = PanelProjection { surface_id, records: Vec::new(), key_seq: 0 };
    let root_id = projection.node(root)?;
    let Some(record) = projection.records.get_mut((root_id.0 - 1) as usize).and_then(Option::as_mut) else {
        return Err(format!("panel '{surface_id}' projection lost its root record"));
    };
    record.layout = ui_contract::LayoutSpec::Scroll(ui_contract::ScrollLayout { axes: ui_contract::ScrollAxes::Vertical, padding: ui_contract::EdgeSpace::default(), sizing: ui_contract::Sizing::Fill });
    projection.records.into_iter().collect::<Option<Vec<_>>>().ok_or_else(|| format!("panel '{surface_id}' projection left a reserved record unplaced"))
}

fn panel_component_scene_props(scene: &ui_wgpu::wgpu::UiComponentSceneNode) -> Result<ui_contract::SurfaceProps, String> {
    let kind = match scene.component_kind {
        ui_wgpu::wgpu::SurfaceKind::Canvas2d => ui_contract::SurfaceKind::Canvas2d,
        ui_wgpu::wgpu::SurfaceKind::World3d => ui_contract::SurfaceKind::World3d,
        ui_wgpu::wgpu::SurfaceKind::NodeGraph => ui_contract::SurfaceKind::NodeGraph,
        ui_wgpu::wgpu::SurfaceKind::TextEditor => ui_contract::SurfaceKind::TextEditor,
        ui_wgpu::wgpu::SurfaceKind::Table => ui_contract::SurfaceKind::Table,
        ui_wgpu::wgpu::SurfaceKind::Paint2d => ui_contract::SurfaceKind::Paint2d,
        ui_wgpu::wgpu::SurfaceKind::VirtualFileSystem => ui_contract::SurfaceKind::VirtualFileSystem,
        ui_wgpu::wgpu::SurfaceKind::TiledMap => ui_contract::SurfaceKind::TiledMap,
        ui_wgpu::wgpu::SurfaceKind::Board2d => ui_contract::SurfaceKind::Board2d,
        ui_wgpu::wgpu::SurfaceKind::IconRender => ui_contract::SurfaceKind::IconRender,
        ui_wgpu::wgpu::SurfaceKind::InkCanvas => ui_contract::SurfaceKind::InkCanvas,
        ui_wgpu::wgpu::SurfaceKind::GraphTimeline => ui_contract::SurfaceKind::GraphTimeline,
        ui_wgpu::wgpu::SurfaceKind::BlockList => ui_contract::SurfaceKind::BlockList,
        ui_wgpu::wgpu::SurfaceKind::DiffView => ui_contract::SurfaceKind::DiffView,
        ui_wgpu::wgpu::SurfaceKind::EventFeed => ui_contract::SurfaceKind::EventFeed,
    };
    let encoded = match scene.component_kind {
        ui_wgpu::wgpu::SurfaceKind::Canvas2d => scene.canvas_2d.as_ref().map(|value| ui_wgpu::wgpu::encode_surface_doc(kind, value)),
        ui_wgpu::wgpu::SurfaceKind::World3d => scene.world_3d.as_ref().map(|value| ui_wgpu::wgpu::encode_surface_doc(kind, value)),
        ui_wgpu::wgpu::SurfaceKind::NodeGraph => scene.node_graph.as_ref().map(|value| ui_wgpu::wgpu::encode_surface_doc(kind, value)),
        ui_wgpu::wgpu::SurfaceKind::TextEditor => scene.text_editor.as_ref().map(|value| ui_wgpu::wgpu::encode_surface_doc(kind, value)),
        ui_wgpu::wgpu::SurfaceKind::Table => scene.table.as_ref().map(|value| ui_wgpu::wgpu::encode_surface_doc(kind, value)),
        ui_wgpu::wgpu::SurfaceKind::Paint2d => scene.paint_2d.as_ref().map(|value| ui_wgpu::wgpu::encode_surface_doc(kind, value)),
        ui_wgpu::wgpu::SurfaceKind::VirtualFileSystem => scene.virtual_file_system.as_ref().map(|value| ui_wgpu::wgpu::encode_surface_doc(kind, value)),
        ui_wgpu::wgpu::SurfaceKind::TiledMap => scene.tiled_map.as_ref().map(|value| ui_wgpu::wgpu::encode_surface_doc(kind, value)),
        ui_wgpu::wgpu::SurfaceKind::Board2d => scene.board2d.as_ref().map(|value| ui_wgpu::wgpu::encode_surface_doc(kind, value)),
        ui_wgpu::wgpu::SurfaceKind::IconRender => scene.icon_render.as_ref().map(|value| ui_wgpu::wgpu::encode_surface_doc(kind, value)),
        ui_wgpu::wgpu::SurfaceKind::InkCanvas => scene.ink_canvas.as_ref().map(|value| ui_wgpu::wgpu::encode_surface_doc(kind, value)),
        ui_wgpu::wgpu::SurfaceKind::GraphTimeline => scene.graph_timeline.as_ref().map(|value| ui_wgpu::wgpu::encode_surface_doc(kind, value)),
        ui_wgpu::wgpu::SurfaceKind::BlockList => scene.block_list.as_ref().map(|value| ui_wgpu::wgpu::encode_surface_doc(kind, value)),
        ui_wgpu::wgpu::SurfaceKind::DiffView => scene.diff_view.as_ref().map(|value| ui_wgpu::wgpu::encode_surface_doc(kind, value)),
        ui_wgpu::wgpu::SurfaceKind::EventFeed => scene.event_feed.as_ref().map(|value| ui_wgpu::wgpu::encode_surface_doc(kind, value)),
    };
    encoded
        .ok_or_else(|| format!("panel ComponentScene '{}' has no {:?} payload", scene.surface_id, scene.component_kind))?
        .map_err(|error| format!("panel ComponentScene '{}' payload exceeds the retained surface contract: {error:?}", scene.surface_id))
}

/// 🧾️ Everything a panel record carries beside its key, component and layout.
#[derive(Default)]
struct PanelRecord {
    pub(crate) children: ui_contract::UiNodeChildren,
    pub(crate) bindings: ui_contract::UiNodeBindings,
    pub(crate) activity: ui_contract::Activity,
    pub(crate) disabled: bool,
    pub(crate) label: Option<ui_contract::Label>,
    pub(crate) description: Option<ui_contract::Label>,
}

/// 🌳️ One shell-owned panel document under construction — same reserve-then-place idiom as
/// {@link WindowMeasuresProjection}, so the published record order is parent-first.
pub(crate) struct PanelProjection<'a> {
    pub(crate) surface_id: &'a str,
    pub(crate) records: Vec<Option<ui_contract::UiNodeRecord>>,
    key_seq: u32,
}

impl PanelProjection<'_> {
    pub(crate) fn reserve(&mut self) -> ui_contract::UiNodeId {
        self.records.push(None);
        ui_contract::UiNodeId(self.records.len() as u64)
    }

    pub(crate) fn key(&mut self, id: Option<&str>) -> String {
        self.key_seq += 1;
        match id.filter(|id| !id.is_empty()) {
            Some(id) => format!("{}/{id}", self.surface_id),
            None => format!("{}/n{}", self.surface_id, self.key_seq),
        }
    }

    /// 🧳️ Preserves a Tree row's native transfer map across the bounded retained-document seam.
    pub(crate) fn drag_data(&self, item_id: &str, source: Option<&HashMap<String, String>>) -> Result<Option<ui_contract::UiFixedMap<UiText>>, String> {
        source
            .map(|source| {
                let mut projected = ui_contract::UiFixedMap::default();
                for (mime, payload) in source {
                    let mime = UiText::try_from_str(mime).ok_or_else(|| format!("panel '{}' tree row '{item_id}' drag MIME exceeds the retained contract", self.surface_id))?;
                    let payload = UiText::try_from_str(payload).ok_or_else(|| format!("panel '{}' tree row '{item_id}' drag payload exceeds the retained contract", self.surface_id))?;
                    projected.try_insert(mime, payload).map_err(|_| format!("panel '{}' tree row '{item_id}' drag map exceeds the retained contract", self.surface_id))?;
                }
                Ok(projected)
            })
            .transpose()
    }

    pub(crate) fn place(&mut self, id: ui_contract::UiNodeId, key: String, component: ui_contract::Component, layout: ui_contract::LayoutSpec, record: PanelRecord) -> Result<ui_contract::UiNodeId, String> {
        if self.records.len() > ui_contract::UI_DOCUMENT_NODES {
            return Err(format!("panel '{}' exceeds {} document nodes", self.surface_id, ui_contract::UI_DOCUMENT_NODES));
        }
        let key = UiText::try_from_string(key).map_err(|key| format!("panel key '{key}' exceeds the retained contract"))?;
        let PanelRecord { children, bindings, activity, disabled, label, description } = record;
        self.records[(id.0 - 1) as usize] = Some(ui_contract::UiNodeRecord {
            id,
            key,
            component,
            layout,
            style: Default::default(),
            activity,
            disabled,
            transition: None,
            accessibility: ui_contract::AccessibilitySpec { label, description, ..Default::default() },
            bindings,
            menu: None,
            children,
        });
        Ok(id)
    }

    pub(crate) fn stack_layout(axis: ui_contract::Axis, gap: ui_contract::SpaceToken) -> ui_contract::LayoutSpec {
        ui_contract::LayoutSpec::Stack(ui_contract::StackLayout { axis, gap, ..Default::default() })
    }

    fn children_of(&mut self, nodes: &[UiNode]) -> Result<ui_contract::UiNodeChildren, String> {
        let mut children = ui_contract::UiNodeChildren::default();
        for child in nodes {
            let id = self.node(child)?;
            children.try_push(id).map_err(|_| format!("panel '{}' packs more children than one record admits", self.surface_id))?;
        }
        Ok(children)
    }

    fn propagate_field_accessibility(&mut self, id: ui_contract::UiNodeId, label: &ui_contract::Label, description: Option<&ui_contract::Label>) {
        let Some(record) = self.records.get_mut((id.0 - 1) as usize).and_then(Option::as_mut) else { return };
        let activatable = record.bindings.iter().any(|binding| binding.trigger == ui_contract::Trigger::Activate);
        let children: Vec<_> = record.children.iter().copied().collect();
        if ui_contract::accessibility_is_focusable(&record.component, activatable) {
            record.accessibility.label.get_or_insert_with(|| label.clone());
            if let Some(description) = description {
                record.accessibility.description.get_or_insert_with(|| description.clone());
            }
        }
        for child in children {
            self.propagate_field_accessibility(child, label, description);
        }
    }

    /// 🧾️ One `UiNode` as one (or, for a `Field`, two) records. Every variant a shell-owned builder
    /// authors is projected; a variable-content variant no shell panel authors (`Image`,
    /// `ComponentScene`, `ExternalSlot`, `Group`, `Ring`, `IconSelect`, `NumberStepper`) is refused
    /// loudly rather than silently dropped, so a builder that grows one fails at publication instead
    /// of painting a hole.
    ///
    /// 🌳️ `Tree` is projected because React's per-window Actions pane IS a `Tree`
    /// (`WindowActionPane`, `🛠️ShellHelpers/🟦️.tsx`), and a section/item pair is the only shape whose
    /// rows both renderers register as controls — a `Section` container registers none
    /// (`retained_hit_registration`), so a category header authored as one would have no twin for
    /// React's `action.category.<id>` row.
    ///
    /// ⏎️⎋️🔁️ React's window-search line binds `onSubmit`/`onAbort`/`onRepeatLast` BESIDE
    /// `onChange`, all on one field — so this record carries one binding per moment rather
    /// than resolving them into the single commit slot (`🖥️ui/🎯️targets/⚛️react/🟦️.tsx`'s
    /// `Search`; `reconcile::input_node` reads them back and `events::search_line_action`
    /// fires them).
    ///
    /// 🔢️💍️ React's `<Engagement/>` renders a `stepper` and a `ring` control beside the slider,
    /// toggle group and select (`EngagementControlView`, `🖥️ui/🎯️targets/⚛️react/🟦️.tsx:10648`),
    /// and both already have a retained `Component` and a `retained_hit_registration` arm — only
    /// this assembler refused them, so an engagement carrying one published a hole.
    pub(crate) fn node(&mut self, node: &UiNode) -> Result<ui_contract::UiNodeId, String> {
        let presence = node.presence();
        let activity = match presence.status {
            ui_wgpu::wgpu::component::ui::UiStatus::Loading => ui_contract::Activity::Loading,
            ui_wgpu::wgpu::component::ui::UiStatus::Waiting => ui_contract::Activity::Waiting,
            ui_wgpu::wgpu::component::ui::UiStatus::Finished => ui_contract::Activity::Finished,
            ui_wgpu::wgpu::component::ui::UiStatus::Idle => ui_contract::Activity::Idle,
        };
        let disabled = matches!(presence.state, ui_wgpu::wgpu::component::ui::UiState::Disabled);
        match node {
            UiNode::Stack(stack) => {
                let key = self.key(stack.id.as_deref());
                let id = self.reserve();
                let children = self.children_of(&stack.children)?;
                let axis = if stack.direction == "horizontal" { ui_contract::Axis::Horizontal } else { ui_contract::Axis::Vertical };
                self.place(
                    id,
                    key,
                    ui_contract::Component::Container(measure_container(ui_contract::ContainerRole::Plain, None, None)),
                    Self::stack_layout(axis, ui_contract::SpaceToken::Sm),
                    PanelRecord { children, activity, disabled, ..PanelRecord::default() },
                )
            }
            UiNode::Section(section) => {
                let key = self.key(Some(section.id.as_str()));
                let id = self.reserve();
                let children = self.children_of(&section.children)?;
                let label = section.label.as_ref().map(|label| label.as_str().to_string());
                self.place(
                    id,
                    key,
                    ui_contract::Component::Container(measure_container(ui_contract::ContainerRole::Section, label.as_deref(), section.default_open)),
                    Self::stack_layout(ui_contract::Axis::Vertical, ui_contract::SpaceToken::Sm),
                    PanelRecord { children, activity, disabled, label: label.as_deref().map(measure_label), ..PanelRecord::default() },
                )
            }
            UiNode::Field(field) => {
                let key = self.key(Some(field.id.as_str()));
                let id = self.reserve();
                let mut children = ui_contract::UiNodeChildren::default();
                let child = self.node(&field.child)?;
                children.try_push(child).map_err(|_| "panel field admits its control".to_string())?;
                let label = field.label.as_str().to_string();
                let accessibility_label = measure_label(&label);
                let accessibility_description = field.description.as_deref().map(measure_label);
                self.propagate_field_accessibility(child, &accessibility_label, accessibility_description.as_ref());
                self.place(
                    id,
                    key,
                    ui_contract::Component::Container(measure_container(ui_contract::ContainerRole::Field, Some(&label), None)),
                    Self::stack_layout(ui_contract::Axis::Vertical, ui_contract::SpaceToken::Xs),
                    PanelRecord { children, activity, disabled, label: Some(accessibility_label), description: accessibility_description, ..PanelRecord::default() },
                )
            }
            UiNode::Text(text) => {
                let key = self.key(None);
                let id = self.reserve();
                let props = ui_contract::TextProps { value: measure_label(text.value.as_str()), emphasize: text.emphasize, data_attributes: None };
                self.place(
                    id,
                    key,
                    ui_contract::Component::Text(props),
                    Self::stack_layout(ui_contract::Axis::Vertical, ui_contract::SpaceToken::None),
                    PanelRecord { activity, disabled, label: Some(measure_label(text.value.as_str())), ..PanelRecord::default() },
                )
            }
            UiNode::Separator(_) => {
                let key = self.key(None);
                let id = self.reserve();
                self.place(id, key, ui_contract::Component::Separator(ui_contract::SeparatorProps {}), Self::stack_layout(ui_contract::Axis::Vertical, ui_contract::SpaceToken::None), PanelRecord::default())
            }
            UiNode::Button(button) => {
                let key = self.key(button.id.as_deref());
                let id = self.reserve();
                let props = ui_contract::ButtonProps { icon: UiText::clipped(button.icon_id.as_str()), label: measure_label(button.label.as_str()) };
                let bindings = measure_bindings(ui_contract::Trigger::Activate, &button.action, None)?;
                self.place(
                    id,
                    key,
                    ui_contract::Component::Button(props),
                    Self::stack_layout(ui_contract::Axis::Horizontal, ui_contract::SpaceToken::None),
                    PanelRecord { bindings, activity, disabled, label: Some(measure_label(button.label.as_str())), ..PanelRecord::default() },
                )
            }
            UiNode::Select(select) => {
                let key = self.key(Some(select.id.as_str()));
                let id = self.reserve();
                let mut items = UiFixedList::default();
                for item in &select.items {
                    items.try_push(ui_contract::SelectItem { value: UiText::clipped(&item.value), label: measure_label(item.label.as_str()) }).map_err(|_| format!("panel select '{}' has more options than one select admits", select.id))?;
                }
                let props = ui_contract::SelectProps { value: UiText::clipped(&select.value), items, placeholder: select.placeholder.as_ref().map(|label| measure_label(label.as_str())), appearance: ui_contract::SelectAppearance::Menu };
                let bindings = measure_bindings(ui_contract::Trigger::Change, &select.on_change, None)?;
                self.place(id, key, ui_contract::Component::Select(props), Self::stack_layout(ui_contract::Axis::Horizontal, ui_contract::SpaceToken::None), PanelRecord { bindings, activity, disabled, ..PanelRecord::default() })
            }
            UiNode::Toggle(toggle) => {
                let key = self.key(Some(toggle.id.as_str()));
                let id = self.reserve();
                let on = presence.selected;
                let props = ui_contract::ToggleProps { appearance: toggle.appearance, on, icon: UiText::clipped(toggle.icon_id.as_str()), text: toggle.text.as_ref().map(|label| measure_label(label.as_str())) };
                let bindings = measure_bindings(ui_contract::Trigger::Change, &toggle.on_change, Some(("pressed", semio_framework_value::DslValue::Bool(!on))))?;
                self.place(
                    id,
                    key,
                    ui_contract::Component::Toggle(props),
                    Self::stack_layout(ui_contract::Axis::Horizontal, ui_contract::SpaceToken::None),
                    PanelRecord { bindings, activity, disabled, label: toggle.text.as_ref().map(|label| measure_label(label.as_str())), ..PanelRecord::default() },
                )
            }
            UiNode::Input(input) => {
                let key = self.key(Some(input.id.as_str()));
                let id = self.reserve();
                let kind = match input.input_kind.as_str() {
                    "number" => ui_contract::InputKind::Number,
                    "longText" | "textarea" => ui_contract::InputKind::LongText,
                    "date" => ui_contract::InputKind::Date,
                    "color" => ui_contract::InputKind::Color,
                    "file" => ui_contract::InputKind::File,
                    _ => ui_contract::InputKind::Text,
                };
                let props = ui_contract::InputProps {
                    kind,
                    value: UiText::clipped(&input.value),
                    placeholder: input.placeholder.as_ref().map(|label| measure_label(label.as_str())),
                    commit: input.commit.as_deref().map(UiText::clipped),
                    min: input.min,
                    max: input.max,
                    step: input.step,
                    accept: input.accept.as_deref().map(UiText::clipped),
                    precision: None,
                    ..Default::default()
                };
                let trigger = if input.commit.is_some() { ui_contract::Trigger::Commit } else { ui_contract::Trigger::Change };
                let mut bindings = measure_bindings(trigger, &input.on_change, None)?;
                for (trigger, action) in [(ui_contract::Trigger::Submit, input.on_submit.as_ref()), (ui_contract::Trigger::Abort, input.on_abort.as_ref()), (ui_contract::Trigger::RepeatLast, input.on_repeat_last.as_ref())] {
                    let Some(action) = action else { continue };
                    bindings.try_push(measure_binding(trigger, action, None)?).map_err(|_| format!("panel input '{}' binds more moments than one record admits", input.id))?;
                }
                self.place(
                    id,
                    key,
                    ui_contract::Component::Input(props),
                    Self::stack_layout(ui_contract::Axis::Horizontal, ui_contract::SpaceToken::None),
                    PanelRecord { bindings, activity, disabled, label: input.accessibility_label.as_ref().map(|label| measure_label(label.as_str())), ..PanelRecord::default() },
                )
            }
            UiNode::KeyValue(list) => {
                let key = self.key(None);
                let id = self.reserve();
                let mut entries = UiFixedList::default();
                for entry in &list.entries {
                    entries.try_push(ui_contract::KeyValueEntry { label: measure_label(entry.label.as_str()), value: UiText::clipped(&entry.value) }).map_err(|_| format!("panel '{}' key-value list exceeds one record", self.surface_id))?;
                }
                self.place(
                    id,
                    key,
                    ui_contract::Component::KeyValueList(ui_contract::KeyValueListProps { entries }),
                    Self::stack_layout(ui_contract::Axis::Vertical, ui_contract::SpaceToken::None),
                    PanelRecord { activity, disabled, ..PanelRecord::default() },
                )
            }
            UiNode::Progress(progress) => {
                let key = self.key(Some(progress.id.as_str()));
                let id = self.reserve();
                let props = ui_contract::ProgressProps { completed: progress.completed, total: progress.total, value_text: measure_label(progress.value_text.as_str()) };
                self.place(id, key, ui_contract::Component::Progress(props), Self::stack_layout(ui_contract::Axis::Horizontal, ui_contract::SpaceToken::None), PanelRecord { activity, disabled, ..PanelRecord::default() })
            }
            UiNode::Slider(slider) => {
                let key = self.key(Some(slider.id.as_str()));
                let id = self.reserve();
                let bindings = measure_bindings(ui_contract::Trigger::Change, &slider.on_change, None)?;
                self.place(
                    id,
                    key,
                    measure_slider(slider.value, slider.min, slider.max, Some(slider.step), slider.unit.as_deref()),
                    Self::stack_layout(ui_contract::Axis::Horizontal, ui_contract::SpaceToken::None),
                    PanelRecord { bindings, activity, disabled, ..PanelRecord::default() },
                )
            }
            UiNode::NumberStepper(stepper) => {
                let key = self.key(Some(stepper.id.as_str()));
                let id = self.reserve();
                let props = ui_contract::NumberStepperProps { value: stepper.value, step: stepper.step, uniform: stepper.uniform, ..Default::default() };
                let mut bindings = measure_bindings(ui_contract::Trigger::Change, &stepper.on_absolute, None)?;
                bindings.try_push(measure_binding(ui_contract::Trigger::Delta, &stepper.on_delta, None)?).map_err(|_| format!("panel stepper '{}' binds more moments than one record admits", stepper.id))?;
                self.place(id, key, ui_contract::Component::NumberStepper(props), Self::stack_layout(ui_contract::Axis::Horizontal, ui_contract::SpaceToken::None), PanelRecord { bindings, activity, disabled, ..PanelRecord::default() })
            }
            UiNode::Ring(ring) => {
                let key = self.key(Some(ring.id.as_str()));
                let id = self.reserve();
                let props = ui_contract::RingProps { orb_id: UiText::clipped(&ring.orb_id), t: ring.t };
                let bindings = measure_bindings(ui_contract::Trigger::Change, &ring.on_change, None)?;
                self.place(id, key, ui_contract::Component::Ring(props), Self::stack_layout(ui_contract::Axis::Horizontal, ui_contract::SpaceToken::None), PanelRecord { bindings, activity, disabled, ..PanelRecord::default() })
            }
            UiNode::Tree(tree) => {
                let key = self.key(None);
                let id = self.reserve();
                let mut children = ui_contract::UiNodeChildren::default();
                for section in &tree.sections {
                    let section = self.tree_section(section)?;
                    children.try_push(section).map_err(|_| format!("panel '{}' packs more tree sections than one record admits", self.surface_id))?;
                }
                self.place(
                    id,
                    key,
                    ui_contract::Component::Tree(ui_contract::TreeProps { presentation: Default::default(), interaction_domain: tree.interaction_domain.as_deref().map(UiText::clipped) }),
                    Self::stack_layout(ui_contract::Axis::Vertical, ui_contract::SpaceToken::None),
                    PanelRecord { children, activity, disabled, ..PanelRecord::default() },
                )
            }
            UiNode::ComponentScene(scene) => {
                let key = self.key(Some(scene.surface_id.as_str()));
                let id = self.reserve();
                self.place(
                    id,
                    key,
                    ui_contract::Component::Surface(panel_component_scene_props(scene)?),
                    ui_contract::LayoutSpec::Leaf(ui_contract::LeafLayout { width: ui_contract::Sizing::Fill, height: ui_contract::Sizing::Fill }),
                    PanelRecord { activity, disabled, ..PanelRecord::default() },
                )
            }
            other => Err(format!("panel '{}' authored a {} node this assembler does not project", self.surface_id, panel_ui_node_tag(other))),
        }
    }

    /// 🌲️ One `UiTreeSectionNode` as its `Component::TreeSection` record, its items as children —
    /// the shape `reconcile::ui_node_from_record`'s `Tree` arm reads back.
    fn tree_section(&mut self, section: &UiTreeSectionNode) -> Result<ui_contract::UiNodeId, String> {
        let key = self.key(Some(section.id.as_str()));
        let id = self.reserve();
        let mut children = ui_contract::UiNodeChildren::default();
        let header_toolbar = if let Some(toolbar) = section.header_toolbar.as_ref() {
            let toolbar = self.tree_section_header_toolbar(&section.id, toolbar)?;
            children.try_push(toolbar).map_err(|_| format!("panel '{}' tree section '{}' admits its header toolbar", self.surface_id, section.id))?;
            Some(toolbar)
        } else {
            None
        };
        for item in &section.items {
            let item = self.tree_item(item)?;
            children.try_push(item).map_err(|_| format!("panel '{}' tree section '{}' packs more rows than one record admits", self.surface_id, section.id))?;
        }
        let props = ui_contract::TreeSectionProps {
            label: section.label.as_ref().map(|label| measure_label(label.as_str())),
            default_open: section.default_open,
            header_toolbar,
            window: section.window.map(|window| ui_contract::TreeWindow {
                total: window.total,
                offset: window.offset,
                row_extent: match window.row_extent {
                    UiTreeWindowRowExtent::Standard => ui_contract::TreeWindowRowExtent::Standard,
                    UiTreeWindowRowExtent::CompactText => ui_contract::TreeWindowRowExtent::CompactText,
                    UiTreeWindowRowExtent::CompactSmallControl => ui_contract::TreeWindowRowExtent::CompactSmallControl,
                    UiTreeWindowRowExtent::CompactControl => ui_contract::TreeWindowRowExtent::CompactControl,
                },
            }),
        };
        self.place(id, key, ui_contract::Component::TreeSection(props), Self::stack_layout(ui_contract::Axis::Vertical, ui_contract::SpaceToken::None), PanelRecord { children, ..PanelRecord::default() })
    }

    /// 🎛️ Projects a section header's authored horizontal stack as the exact Toolbar relation the
    /// retained Tree contract validates, preserving each Button's disabled state and binding.
    fn tree_section_header_toolbar(&mut self, section_id: &str, toolbar: &UiStackNode) -> Result<ui_contract::UiNodeId, String> {
        if toolbar.direction != "horizontal" {
            return Err(format!("panel '{}' tree section '{}' header toolbar is not horizontal", self.surface_id, section_id));
        }
        let fallback = format!("{section_id}.toolbar");
        let key = self.key(Some(toolbar.id.as_deref().unwrap_or(&fallback)));
        let id = self.reserve();
        let mut children = ui_contract::UiNodeChildren::default();
        for child in &toolbar.children {
            if !matches!(child, UiNode::Button(_)) {
                return Err(format!("panel '{}' tree section '{}' header toolbar contains a non-button", self.surface_id, section_id));
            }
            let button = self.node(child)?;
            children.try_push(button).map_err(|_| format!("panel '{}' tree section '{}' header toolbar packs too many buttons", self.surface_id, section_id))?;
        }
        self.place(
            id,
            key,
            ui_contract::Component::Container(measure_container(ui_contract::ContainerRole::Toolbar, None, None)),
            Self::stack_layout(ui_contract::Axis::Horizontal, ui_contract::SpaceToken::None),
            PanelRecord { children, ..PanelRecord::default() },
        )
    }

    /// 🌿️ One `UiTreeItemNode` as its `Component::TreeItem` record. The row's own click is its target's
    /// activation verb (React's `TreeDataItem.onClick`), and nested rows are ordinary
    /// children — except a homogeneous strip of action-only leaves, which is React's inline
    /// resolution toolbar (`ContainerRole::Toolbar` of `Button`s), never more `TreeItem`s.
    ///
    /// 🎛️ React's `TreeDataItem.control` is an inline property row rendered INSIDE the item
    /// (`🌳️Tree/🟦️.tsx`), which is how the staged-argument form's fields live under their own
    /// `action.<id>.arg.<argId>` row. Dropping it published a labelled row with no editor at all.
    pub(crate) fn tree_item(&mut self, item: &UiTreeItemNode) -> Result<ui_contract::UiNodeId, String> {
        let key = self.key(Some(item.id.as_str()));
        let id = self.reserve();
        let mut children = ui_contract::UiNodeChildren::default();
        if let Some(control) = item.control.as_ref() {
            let control = self.node(&control_ui_node(control))?;
            children.try_push(control).map_err(|_| format!("panel '{}' tree row '{}' admits its control", self.surface_id, item.id))?;
        }
        let detail = if let Some(detail) = item.detail.as_ref() {
            let detail = self.node(&UiNode::ComponentScene(detail.clone()))?;
            children.try_push(detail).map_err(|_| format!("panel '{}' tree row '{}' admits its detail Surface", self.surface_id, item.id))?;
            Some(detail)
        } else {
            None
        };
        let nested: Vec<&UiTreeItemNode> = item.items.iter().flatten().collect();
        let inline_action_leaves = !nested.is_empty() && nested.iter().copied().all(tree_item_is_inline_action_leaf);
        let inline_toolbar = if inline_action_leaves {
            let toolbar = self.tree_item_inline_action_toolbar(&item.id, &nested)?;
            children.try_push(toolbar).map_err(|_| format!("panel '{}' tree row '{}' admits its inline toolbar", self.surface_id, item.id))?;
            Some(toolbar)
        } else {
            for child in nested {
                let child = self.tree_item(child)?;
                children.try_push(child).map_err(|_| format!("panel '{}' tree row '{}' packs more children than one record admits", self.surface_id, item.id))?;
            }
            None
        };
        let (row_actions, target) = self.tree_item_row_target(item)?;
        let props = ui_contract::TreeItemProps {
            label: measure_label(item.label.as_str()),
            description: item.description.as_deref().map(UiText::clipped),
            icon: item.icon_id.as_ref().map(|icon| UiText::clipped(icon.as_str())),
            default_open: item.default_open,
            draggable: item.draggable,
            drag_data: self.drag_data(&item.id, item.drag_data.as_ref())?,
            dimmed: item.dimmed,
            selected: None,
            window: item.window.map(|window| ui_contract::TreeWindow {
                total: window.total,
                offset: window.offset,
                row_extent: match window.row_extent {
                    UiTreeWindowRowExtent::Standard => ui_contract::TreeWindowRowExtent::Standard,
                    UiTreeWindowRowExtent::CompactText => ui_contract::TreeWindowRowExtent::CompactText,
                    UiTreeWindowRowExtent::CompactSmallControl => ui_contract::TreeWindowRowExtent::CompactSmallControl,
                    UiTreeWindowRowExtent::CompactControl => ui_contract::TreeWindowRowExtent::CompactControl,
                },
            }),
            granularity: None,
            inline_toolbar,
            detail,
            row_actions,
            target,
        };
        self.place(
            id,
            key,
            ui_contract::Component::TreeItem(props),
            Self::stack_layout(ui_contract::Axis::Vertical, ui_contract::SpaceToken::None),
            PanelRecord { children, disabled: matches!(item.presence.state, ui_wgpu::wgpu::component::ui::UiState::Disabled), ..PanelRecord::default() },
        )
    }

    /// 🎯️ A row's trailing actions as the contract's ONE target plus verbs (U6's row model): the target is the row's own click — its
    /// activation — or, for an action-only row, its first action's controller and argument map; every action must fire on that
    /// same controller and map and names only its verb, so N actions cost N verbs, never N argument maps.
    #[allow(clippy::type_complexity, reason = "the two halves of one row's projected action model")]
    fn tree_item_row_target(&self, item: &UiTreeItemNode) -> Result<(ui_contract::UiFixedList<ui_contract::RowAction>, Option<ui_contract::RowTarget>), String> {
        let actions = item.actions.as_deref().unwrap_or_default();
        let Some(anchor) = item.action.as_ref().or_else(|| actions.first().map(|action| &action.action)) else { return Ok((ui_contract::UiFixedList::default(), None)) };
        let mut target = measure_row_target(anchor)?;
        if item.action.is_none() {
            target.activation = None;
        }
        let mut row_actions = ui_contract::UiFixedList::default();
        for action in actions {
            if action.action.controller_id != anchor.controller_id || action.action.args != anchor.args {
                return Err(format!("panel '{}' tree row '{}' action '{}' fires outside the row's one target", self.surface_id, item.id, action.action.action));
            }
            let verb = UiText::try_from_str(&action.action.action).ok_or_else(|| format!("panel '{}' tree row '{}' verb exceeds the retained contract", self.surface_id, item.id))?;
            let placement = match action.placement() {
                ui_wgpu::wgpu::component::ui::UiTreeActionPlacement::Row => ui_contract::RowActionPlacement::Row,
                ui_wgpu::wgpu::component::ui::UiTreeActionPlacement::Menu => ui_contract::RowActionPlacement::Menu,
            };
            let row_action = ui_contract::RowAction { icon: UiText::clipped(action.icon_id.as_str()), label: action.label.as_ref().map(|label| measure_label(label.as_str())), verb, placement, disabled: action.disabled, reason: action.reason.as_ref().filter(|_| action.disabled).map(|reason| measure_label(reason.as_str())) };
            row_actions.try_push(row_action).map_err(|_| format!("panel '{}' tree row '{}' packs more row actions than one record admits", self.surface_id, item.id))?;
        }
        Ok((row_actions, Some(target)))
    }

    /// 🎛️ Accept/Discard leaves authored as nested
    /// `UiTreeItemNode`s with only an Activate action — React's inline control slot — become one
    /// horizontal `Toolbar` of real `Button` records so schema `rowCount` stays the parent alone.
    fn tree_item_inline_action_toolbar(&mut self, parent_id: &str, verbs: &[&UiTreeItemNode]) -> Result<ui_contract::UiNodeId, String> {
        let key = self.key(Some(&format!("{parent_id}.toolbar")));
        let id = self.reserve();
        let mut children = ui_contract::UiNodeChildren::default();
        for verb in verbs {
            let action = verb.action.as_ref().ok_or_else(|| format!("panel '{}' inline verb '{}' lost its Activate action", self.surface_id, verb.id))?;
            let button =
                UiNode::Button(UiButtonNode { id: Some(verb.id.clone()), icon_id: verb.icon_id.clone().unwrap_or(IconName::Check), label: verb.label.clone(), action: action.clone(), style: None, presence: verb.presence.clone(), menu: None });
            let button = self.node(&button)?;
            children.try_push(button).map_err(|_| format!("panel '{}' inline toolbar under '{}' packs more verbs than one record admits", self.surface_id, parent_id))?;
        }
        self.place(
            id,
            key,
            ui_contract::Component::Container(measure_container(ui_contract::ContainerRole::Toolbar, None, None)),
            Self::stack_layout(ui_contract::Axis::Horizontal, ui_contract::SpaceToken::None),
            PanelRecord { children, ..PanelRecord::default() },
        )
    }
}

/// 🎛️ A nested tree leaf that carries only an Activate verb (no control, no further children) is
/// React's inline resolution/control slot, not another expandable row.
fn tree_item_is_inline_action_leaf(item: &UiTreeItemNode) -> bool {
    item.action.is_some() && item.control.is_none() && item.default_open.is_none() && item.items.as_ref().map_or(true, |items| items.is_empty())
}

/// 👁️✏️ Byte-identical to React's `DEFAULT_APP_NONE_VALUE` (`📌️ChromePanels/🟦️.tsx`) — the
/// default-apps `Select` value that clears a pin rather than naming an app.
pub(crate) const DEFAULT_APP_NONE_VALUE: &str = "none";

/// 🎛️ One [`UiControlNode`] as the [`UiNode`] it IS — the enum is a narrowed projection of the
/// same variants, so a tree row's inline control projects through the one assembler every other node
/// takes rather than a second, parallel arm table.
fn control_ui_node(control: &ui_wgpu::wgpu::component::ui::UiControlNode) -> UiNode {
    use ui_wgpu::wgpu::component::ui::UiControlNode;
    match control.clone() {
        UiControlNode::Input(node) => UiNode::Input(node),
        UiControlNode::Select(node) => UiNode::Select(node),
        UiControlNode::Toggle(node) => UiNode::Toggle(node),
        UiControlNode::Button(node) => UiNode::Button(node),
        UiControlNode::KeyValue(node) => UiNode::KeyValue(node),
        UiControlNode::Slider(node) => UiNode::Slider(node),
        UiControlNode::NumberStepper(node) => UiNode::NumberStepper(node),
        UiControlNode::Ring(node) => UiNode::Ring(node),
        UiControlNode::IconSelect(node) => UiNode::IconSelect(node),
    }
}

/// 🖥️ One Display leaf's body: a single `Tree` under a stack the panel can address — the shape
/// every other shell-owned leaf publishes, so the two Display leaves need no new paint path.
/// 🪟️ Whether a shell-owned body streams a windowed Tree section — such a body IS its scroll viewport (the window streams against
/// it, its spacers give it the whole extent), so it publishes as a scroll root (`panel_ui_scroll_records`).
pub(crate) fn ui_node_is_windowed(node: &UiNode) -> bool {
    match node {
        UiNode::Tree(tree) => tree.sections.iter().any(|section| section.window.is_some()),
        UiNode::Stack(stack) => stack.children.iter().any(ui_node_is_windowed),
        _ => false,
    }
}

pub(crate) fn display_panel_body(panel_id: &str, sections: Vec<UiTreeSectionNode>) -> UiNode {
    UiNode::Stack(UiStackNode {
        direction: "column".into(),
        gap: None,
        padding: None,
        id: Some(panel_id.to_string()),
        children: vec![UiNode::Tree(UiTreeNode { presentation: Default::default(), sections, presence: UiPresence::default(), drop_action: None, menu: None, interaction_domain: None })],
        presence: UiPresence::default(),
        activate: None,
        drop_action: None,
        drop_overlay: None,
        menu: None,
    })
}

/// 🖥️ React's own empty/unavailable section for a Display leaf with nothing to show
/// (`buildDisplayWindowsTree`'s `framework.display.windows.empty` and the host-less
/// `{ sections: [{ id: "unavailable", … }] }` fallback, folded into one row here because this
/// renderer never has a "no host" state distinct from "no window kinds").
pub(crate) fn display_unavailable_section(id: &str, is_de: bool) -> UiTreeSectionNode {
    UiTreeSectionNode {
        header_toolbar: None,
        id: id.to_string(),
        label: None,
        default_open: Some(true),
        presence: UiPresence::default(),
        items: vec![UiTreeItemNode::base(format!("{id}.row"), Label::data(shell_chrome_string("display.unavailable", is_de)))],
        window: None,
    }
}

/// 🔄️ The `createWorldProjectionTemplates` taxonomy as nested Display rows, reconstructed from
/// [`WORLD_PROJECTION_TEMPLATES`]' own `depth` column: a row's id is its parent's id plus its template
/// id, composed through the element-id grammar exactly as React's `childElementId` composes it
/// (`framework.display.windows.puzzle3dMain.projection.parallel.axonometric.axonometricIsometric`).
/// `cursor` walks the flat table once; `depth` is the level this call materialises.
/// Each level is pre-reversed for the bottom-anchored Display tree's up-flow layout.
///
/// 🛡️ Unreachable for the declared table (a child always follows its own parent), and a
/// skip rather than a panic so a future row ordering cannot wedge the panel.
pub(crate) fn world_projection_template_rows(prefix: &str, window_kind_id: &str, depth: u8, cursor: &mut usize) -> Vec<UiTreeItemNode> {
    let mut rows = Vec::new();
    while let Some(template) = WORLD_PROJECTION_TEMPLATES.get(*cursor) {
        if template.depth < depth {
            break;
        }
        if template.depth > depth {
            *cursor += 1;
            continue;
        }
        let id = semio_framework::child_element_id(prefix, &[template.id]);
        *cursor += 1;
        let children = world_projection_template_rows(&id, window_kind_id, depth + 1, cursor);
        rows.push(UiTreeItemNode {
            id: id.clone(),
            label: Label::data(template.label),
            icon_id: IconName::from_str(template.icon_id),
            default_open: Some(false),
            draggable: Some(true),
            drag_data: Some(window_template_drag_data(window_kind_id, Some(template.id))),
            items: (!children.is_empty()).then_some(children),
            ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
        });
    }
    rows.reverse();
    rows
}

/// 🖥️ React's `groupNamedLayoutsToTreeItems`: a layout with no `groupPath` is a root row, one with
/// a path folds into `framework.display.layout.group.<a/b>` folders minted on first use. A user
/// layout additionally carries React's `framework.display.delete.<id>` action — a row CONTROL here,
/// because this renderer's panel tree projects a row's control and not its `actions` menu.
pub(crate) fn named_layout_rows(layouts: &[ui_wgpu::wgpu::NamedLayout], deletable: bool, is_de: bool) -> Vec<UiTreeItemNode> {
    let mut root: Vec<UiTreeItemNode> = Vec::new();
    for entry in layouts {
        let leaf = UiTreeItemNode {
            id: format!("framework.display.layout.{}", entry.id),
            label: Label::data(entry.label.clone()),
            action: Some(ActionDescriptor { controller_id: "framework".into(), action: "applyNamedLayout".into(), args: crate::action_args_json!({ "layoutId": entry.id.clone() }) }),
            control: deletable.then(|| {
                ui_wgpu::wgpu::component::ui::UiControlNode::Button(UiButtonNode {
                    id: Some(format!("framework.display.delete.{}", entry.id)),
                    icon_id: IconName::Trash2,
                    label: Label::data(shell_chrome_string("display.deleteLayout", is_de)),
                    action: ActionDescriptor { controller_id: "framework".into(), action: "deleteUserLayout".into(), args: crate::action_args_json!({ "layoutId": entry.id.clone() }) },
                    style: None,
                    presence: UiPresence::default(),
                    menu: None,
                })
            }),
            ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
        };
        let path = entry.group_path.clone().unwrap_or_default();
        place_named_layout_row(&mut root, &path, &mut String::new(), leaf);
    }
    root
}

/// 🖥️ One layout row into its folder chain, minting a `framework.display.layout.group.<a/b>`
/// folder on first use — the recursive half of {@link named_layout_rows}, kept separate so the
/// walk owns one `&mut Vec` at a time.
fn place_named_layout_row(rows: &mut Vec<UiTreeItemNode>, path: &[String], path_key: &mut String, leaf: UiTreeItemNode) {
    let Some((segment, rest)) = path.split_first() else {
        rows.push(leaf);
        return;
    };
    if !path_key.is_empty() {
        path_key.push('/');
    }
    path_key.push_str(segment);
    let folder_id = format!("framework.display.layout.group.{path_key}");
    if !rows.iter().any(|row| row.id == folder_id) {
        rows.push(UiTreeItemNode { id: folder_id.clone(), label: Label::data(segment.clone()), default_open: Some(false), items: Some(Vec::new()), ..UiTreeItemNode::base(String::new(), Label::data(String::new())) });
    }
    let folder = rows.iter_mut().find(|row| row.id == folder_id).expect("the folder row was just ensured");
    place_named_layout_row(folder.items.get_or_insert_with(Vec::new), rest, path_key, leaf);
}

/// 🛠️ React's `windowMeasuresToTreeItems` (`🛠️ShellHelpers/🟦️.tsx`): one tree row per measure,
/// the row's id the measure's OWN id and the row's control the measure's own editor. Declaration
/// order is written straight through — React reverses only to cancel its bottom-anchored `Tree`'s
/// own per-level reversal (`reverseForUpPanel`), which this renderer's panel tree does not do.
pub(crate) fn window_measure_tree_rows(measures: &[WindowMeasure]) -> Vec<UiTreeItemNode> {
    use ui_wgpu::wgpu::component::ui::{UiControlNode, UiState, UiStatus};
    let activity = |loading: Option<bool>, waiting: Option<bool>| {
        if loading.unwrap_or(false) {
            UiStatus::Loading
        } else if waiting.unwrap_or(false) {
            UiStatus::Waiting
        } else {
            UiStatus::Idle
        }
    };
    let state = |disabled: Option<bool>| if disabled.unwrap_or(false) { UiState::Disabled } else { UiState::Normal };
    measures
        .iter()
        .map(|measure| match measure {
            WindowMeasure::Group { id, label, default_open, children, .. } => UiTreeItemNode {
                id: id.clone(),
                label: Label::data(label.clone()),
                default_open: *default_open,
                items: (!children.is_empty()).then(|| window_measure_tree_rows(children)),
                ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
            },
            WindowMeasure::Select { id, label, value, items, on_change } => UiTreeItemNode {
                id: id.clone(),
                label: Label::data(label.clone().unwrap_or_default()),
                control: Some(UiControlNode::Select(UiSelectNode {
                    presence: UiPresence::default(),
                    id: id.clone(),
                    value: value.clone(),
                    items: items.iter().map(|item| UiSelectItem { value: item.value.clone(), label: Label::data(item.label.clone()) }).collect(),
                    placeholder: None,
                    on_change: on_change.clone(),
                    menu: None,
                })),
                ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
            },
            WindowMeasure::Slider { id, label, value, min, max, step, loading, waiting, disabled, on_change, .. } => UiTreeItemNode {
                id: id.clone(),
                label: Label::data(label.clone().unwrap_or_default()),
                presence: UiPresence { status: activity(*loading, *waiting), state: state(*disabled), ..UiPresence::default() },
                control: Some(UiControlNode::Slider(UiSliderNode {
                    id: id.clone(),
                    value: *value,
                    min: *min,
                    max: *max,
                    step: step.unwrap_or(1.0),
                    unit: None,
                    snaps: Vec::new(),
                    on_change: on_change.clone(),
                    presence: UiPresence { state: state(*disabled), ..UiPresence::default() },
                    menu: None,
                    ..Default::default()
                })),
                ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
            },
            WindowMeasure::Number { id, label, value, min, max, step, loading, waiting, disabled, on_change, .. } => UiTreeItemNode {
                id: id.clone(),
                label: Label::data(label.clone().unwrap_or_default()),
                presence: UiPresence { status: activity(*loading, *waiting), state: state(*disabled), ..UiPresence::default() },
                control: Some(UiControlNode::Input(UiInputNode {
                    id: id.clone(),
                    input_kind: "number".into(),
                    value: format_measure_number(*value),
                    placeholder: None,
                    accessibility_label: None,
                    commit: Some("enterOrBlur".into()),
                    min: *min,
                    max: *max,
                    step: *step,
                    accept: None,
                    precision: None,
                    snaps: Vec::new(),
                    on_change: on_change.clone(),
                    on_submit: None,
                    on_abort: None,
                    on_repeat_last: None,
                    presence: UiPresence { state: state(*disabled), ..UiPresence::default() },
                    menu: None,
                    ..Default::default()
                })),
                ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
            },
            WindowMeasure::Toggle { id, icon_id, label, pressed, text, on_change } => UiTreeItemNode {
                id: id.clone(),
                label: Label::data(label.clone().or_else(|| text.clone()).unwrap_or_default()),
                icon_id: Some(icon_id.clone()),
                control: Some(UiControlNode::Toggle(UiToggleNode {
                    appearance: ui_contract::ToggleAppearance::Button,
                    id: id.clone(),
                    icon_id: icon_id.clone(),
                    text: text.clone().map(Label::data),
                    on_change: on_change.clone(),
                    presence: UiPresence { selected: *pressed, ..UiPresence::default() },
                    menu: None,
                })),
                ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
            },
        })
        .collect()
}

/// 🔢️ A measure's numeric value as the text its entry shows — an integral value prints without a
/// fractional tail, which is what React's `String(value)` does for the same number.
fn format_measure_number(value: f64) -> String {
    if value.fract() == 0.0 && value.abs() < 1e15 {
        format!("{}", value as i64)
    } else {
        format!("{value}")
    }
}

/// ⚙️ One collapsible settings section — React's `TreeDataSection` with `defaultOpen: true`.
pub(crate) fn settings_section(id: &str, label: &str, children: Vec<UiNode>) -> UiNode {
    UiNode::Section(UiSectionNode { id: id.to_string(), label: Some(Label::data(label)), default_open: Some(true), presence: UiPresence::default(), menu: None, children })
}

/// ⚙️ One control-less text row — React's `TreeDataItem` carrying only a `label`.
pub(crate) fn settings_text_row(text: &str) -> UiNode {
    UiNode::Text(UiTextNode { presence: UiPresence::default(), value: Label::data(text), emphasize: Some(false), data_attributes: None, menu: None })
}

/// 🆔️ The stable node id one conversation row carries — React's `entryElementId`
/// (`💬️AgentChatPanel/🟦️.tsx`), so a probe addresses a row by identity rather than by position.
pub(crate) fn agent_chat_entry_id(entry: &crate::agent_bridge::AgentConversationEntry) -> String {
    use crate::agent_bridge::AgentConversationEntry;
    let kind = match entry {
        AgentConversationEntry::UserMessage { .. } => "userMessage",
        AgentConversationEntry::ToolCall { .. } => "toolCall",
        AgentConversationEntry::Approval { .. } => "approval",
        AgentConversationEntry::AgentMessage { .. } => "agentMessage",
    };
    format!("framework.chat.entry.{kind}.{}", entry.id())
}

/// 🏷️ The translated role noun one conversation row leads with — React's
/// `os.agent.chat.{youRole,toolCallRole,approvalRole}`.
fn agent_chat_role_label(entry: &crate::agent_bridge::AgentConversationEntry, is_de: bool) -> &'static str {
    use crate::agent_bridge::AgentConversationEntry;
    match entry {
        AgentConversationEntry::UserMessage { .. } => shell_chrome_string("chat.youRole", is_de),
        AgentConversationEntry::ToolCall { .. } => shell_chrome_string("chat.toolCallRole", is_de),
        AgentConversationEntry::Approval { .. } => shell_chrome_string("chat.approvalRole", is_de),
        AgentConversationEntry::AgentMessage { .. } => shell_chrome_string("chat.agentRole", is_de),
    }
}

/// 🚥️ The translated state chip one conversation row shows to its right — React's
/// `os.agent.chat.{running,failed,succeeded,approvalPending}`, or the human's own decision once an
/// approval is resolved. A sent turn carries no chip, exactly as React's empty `state` renders none.
fn agent_chat_state_label(entry: &crate::agent_bridge::AgentConversationEntry, is_de: bool, locale: Locale) -> Option<String> {
    use crate::agent_bridge::{AgentApprovalState, AgentConversationEntry, AgentReplyState, AgentToolCallState};
    match entry {
        AgentConversationEntry::UserMessage { .. } => None,
        AgentConversationEntry::AgentMessage { state, .. } => matches!(state, AgentReplyState::Streaming).then(|| shell_chrome_string("chat.replyStreaming", is_de).to_string()),
        AgentConversationEntry::ToolCall { state, .. } => Some(
            match state {
                AgentToolCallState::Running => shell_chrome_string("chat.running", is_de),
                AgentToolCallState::Cancelling => shell_chrome_string("chat.cancelling", is_de),
                AgentToolCallState::Failed => shell_chrome_string("chat.failed", is_de),
                AgentToolCallState::Ok => shell_chrome_string("chat.succeeded", is_de),
            }
            .to_string(),
        ),
        AgentConversationEntry::Approval { state, decision, .. } => match (state, decision) {
            (AgentApprovalState::Pending, _) => Some(shell_chrome_string("chat.approvalPending", is_de).to_string()),
            (AgentApprovalState::Resolved, Some(decision)) => Some(crate::agent_approvals::approvals_decision_label(*decision, locale)),
            (AgentApprovalState::Resolved, None) => None,
            (AgentApprovalState::Withdrawn(reason), _) => Some(crate::agent_approvals::approvals_withdrawal_label(*reason, locale)),
        },
    }
}

/// 🏷️ The row's own `kind` attribute — React's `data-semio-agent-chat-entry`, whose vocabulary is
/// `AgentConversationEntry`'s own discriminant spelled in camelCase.
pub(crate) fn agent_chat_entry_kind(entry: &crate::agent_bridge::AgentConversationEntry) -> &'static str {
    use crate::agent_bridge::AgentConversationEntry;
    match entry {
        AgentConversationEntry::UserMessage { .. } => "userMessage",
        AgentConversationEntry::ToolCall { .. } => "toolCall",
        AgentConversationEntry::Approval { .. } => "approval",
        AgentConversationEntry::AgentMessage { .. } => "agentMessage",
    }
}

/// 🚥️ The row's own `state` attribute — React's `data-agent-chat-state`: a tool call's or approval's
/// live state, and the literal `"sent"` for a human turn, which has none.
pub(crate) fn agent_chat_entry_state_attribute(entry: &crate::agent_bridge::AgentConversationEntry) -> &'static str {
    use crate::agent_bridge::{AgentApprovalState, AgentConversationEntry, AgentReplyState, AgentToolCallState};
    match entry {
        AgentConversationEntry::UserMessage { .. } => "sent",
        AgentConversationEntry::AgentMessage { state, .. } => match state {
            AgentReplyState::Streaming => "streaming",
            AgentReplyState::Complete => "complete",
        },
        AgentConversationEntry::ToolCall { state, .. } => match state {
            AgentToolCallState::Running => "running",
            AgentToolCallState::Cancelling => "cancelling",
            AgentToolCallState::Ok => "ok",
            AgentToolCallState::Failed => "failed",
        },
        AgentConversationEntry::Approval { state, .. } => match state {
            AgentApprovalState::Pending => "pending",
            AgentApprovalState::Resolved => "resolved",
            AgentApprovalState::Withdrawn(_) => "withdrawn",
        },
    }
}

/// 💬️ One conversation row — the wgpu twin of React's `AgentChatEntry`: role noun, state chip, then
/// the body this kind carries (the human's text, the approval's summary, or the tool's name, its real
/// arguments and — once the result frame lands — its summary behind the `Result` noun).
///
/// 🏷️ The leading role line carries React's own two row attributes, so a probe (and the accessibility
/// mirror `dumpAccessibility` publishes) can tell a tool call from an approval, and a running call
/// from a failed one, WITHOUT reading the translated noun — which is what React's `li` does.
pub(crate) fn agent_chat_entry_node(entry: &crate::agent_bridge::AgentConversationEntry, is_de: bool, locale: Locale) -> UiNode {
    use crate::agent_bridge::AgentConversationEntry;
    let mut role = UiTextNode { presence: UiPresence::default(), value: Label::data(agent_chat_role_label(entry, is_de)), emphasize: Some(false), data_attributes: None, menu: None };
    role.data_attributes = Some(HashMap::from([("data-semio-agent-chat-entry".to_string(), agent_chat_entry_kind(entry).to_string()), ("data-agent-chat-state".to_string(), agent_chat_entry_state_attribute(entry).to_string())]));
    let mut rows = vec![UiNode::Text(role)];
    if let Some(state) = agent_chat_state_label(entry, is_de, locale) {
        rows.push(settings_text_row(&state));
    }
    if matches!(entry, AgentConversationEntry::ToolCall { state: crate::agent_bridge::AgentToolCallState::Running, .. }) {
        rows.push(UiNode::Button(UiButtonNode {
            id: Some(format!("framework.chat.cancel.{}", entry.id())),
            icon_id: IconName::Square,
            label: Label::data(shell_chrome_string("chat.cancel", is_de)),
            action: ActionDescriptor { controller_id: "framework".into(), action: "cancelToolCall".into(), args: crate::action_args_json!({ "invocationId": entry.id().to_string() }) },
            style: Some(ui_wgpu::wgpu::component::ui::StyleSpec { variant: Some("ghost".into()), size: None, density: None }),
            presence: UiPresence::default(),
            menu: None,
        }));
    }
    match entry {
        AgentConversationEntry::UserMessage { text, .. } => rows.push(settings_text_row(text)),
        AgentConversationEntry::AgentMessage { text, .. } => rows.push(settings_text_row(text)),
        AgentConversationEntry::Approval { summary, .. } => rows.push(settings_text_row(summary)),
        AgentConversationEntry::ToolCall { tool_name, arguments, summary, .. } => {
            rows.push(settings_text_row(tool_name));
            if !arguments.is_empty() {
                rows.push(settings_text_row(arguments));
            }
            if let Some(summary) = summary {
                rows.push(settings_text_row(&format!("{}: {summary}", shell_chrome_string("chat.toolResultRole", is_de))));
            }
        }
    }
    UiNode::Stack(UiStackNode { direction: "column".into(), gap: None, padding: None, id: Some(agent_chat_entry_id(entry)), children: rows, presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, menu: None })
}

/// ⚙️ One labelled `Select` row — React's `{ id, label, control: <Select …/> }` item, whose
/// `onValueChange` becomes the node's `on_change` over the `"framework"` controller.
fn settings_select_row(id: &str, label: &str, value: &str, options: Vec<(String, String)>, action: &str) -> UiNode {
    settings_select_row_with(id, label, value, options, action, None)
}

/// ⚙️ {@link settings_select_row} plus authored action args — the row's own coordinate, for a table
/// whose action needs to know WHICH row changed (the default-apps `(dialect × role)` pairing).
pub(crate) fn settings_select_row_with(id: &str, label: &str, value: &str, options: Vec<(String, String)>, action: &str, args: Option<DslValue>) -> UiNode {
    UiNode::Field(UiFieldNode {
        id: format!("{id}.field"),
        label: Label::data(label),
        description: None,
        required: None,
        error: None,
        child: Box::new(UiNode::Select(UiSelectNode {
            presence: UiPresence::default(),
            id: id.to_string(),
            value: value.to_string(),
            items: options.into_iter().map(|(value, label)| UiSelectItem { value, label: Label::data(label) }).collect(),
            placeholder: None,
            on_change: ActionDescriptor { controller_id: "framework".into(), action: action.into(), args },
            menu: None,
        })),
        presence: UiPresence::default(),
        menu: None,
    })
}

/// 🌳️ One React `TreeDataItem` property row with its Select in the inline control cell.
pub(crate) fn settings_tree_select_item(id: &str, label: &str, value: &str, options: Vec<(String, String)>, action: &str) -> UiTreeItemNode {
    settings_tree_select_item_with(id, label, value, options, action, None)
}

/// 🌳️ One inline settings Select whose action keeps its authored row arguments.
pub(crate) fn settings_tree_select_item_with(id: &str, label: &str, value: &str, options: Vec<(String, String)>, action: &str, args: Option<DslValue>) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.to_string(),
        label: Label::data(label),
        control: Some(UiControlNode::Select(UiSelectNode {
            presence: UiPresence::default(),
            id: id.to_string(),
            value: value.to_string(),
            items: options.into_iter().map(|(value, label)| UiSelectItem { value, label: Label::data(label) }).collect(),
            placeholder: None,
            on_change: ActionDescriptor { controller_id: "framework".into(), action: action.into(), args },
            menu: None,
        })),
        ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
    }
}

/// 📝️ One staged command argument as a labelled row — [`staged_arg_row`] dispatching `stageCommandArg`, so the
/// value buffers in {@link ShellState::staged_command_args} and nothing fires until Execute.
pub(crate) fn staged_command_arg_row(command_key: &str, arg: &semio_framework::ActionArgDef, value: Option<&Value>, selection: &HashMap<String, semio_framework::DomainSelection>, terminology: Terminology, locale: Locale) -> UiTreeItemNode {
    let id = format!("command.{}.arg.{}", command_key.replace(':', "."), arg.id);
    let stage = ActionDescriptor { controller_id: "framework".into(), action: "stageCommandArg".into(), args: crate::action_args_json!({ "command": command_key.to_string(), "arg": arg.id.clone() }) };
    staged_arg_row(id.clone(), &id, arg, value, &stage, selection, terminology, locale)
}

/// 📝️ One staged argument's row — the wgpu twin of React's `renderStagedArgControl` (`🛠️ShellHelpers/🟦️.tsx`), shared
/// by the command palette's staged form and a window's Actions form. `stage` is `stageCommandArg`/`stageActionArg`
/// with the row's own coordinates; every editor sends it, so the value buffers and nothing fires until Execute.
///
/// 🎛️ One control per scalar kind — text, number, a stepper, a slider or dial, a toggle, a select. Every number control
/// carries the facets [`semio_framework::ActionArgDef::number_facets`] derives (the one mapping the time-travel editor
/// shares): look and axis, travel or key range, step, stored and shown unit, display factor, precision, detents painted
/// as ticks, and the schema's hard limits, whose localized refusal names the crossed bound and which make a slider's
/// travel soft — a staged value is never clamped to the travel. The multi-part kinds open child rows of one control
/// each, so every part is its own focus stop with its own accessible name: a segmented choice's pressed buttons
/// (`option`), a vector's labelled axes (`index`/`dims`, spliced into the staged tuple by [`staged_arg_value`]), and a
/// reference list's removable chips plus its "use current selection" button, which stages the live selection of the
/// argument's domain.
#[allow(clippy::too_many_arguments, reason = "one staged row: its two ids, the definition, the value, its stage verb and the live selection")]
pub(crate) fn staged_arg_row(row_id: String, control_id: &str, arg: &semio_framework::ActionArgDef, value: Option<&Value>, stage: &ActionDescriptor, selection: &HashMap<String, semio_framework::DomainSelection>, terminology: Terminology, locale: Locale) -> UiTreeItemNode {
    use ui_wgpu::wgpu::component::ui::UiControlNode as Control;
    let label = arg.label.resolve(terminology, locale).to_string();
    let description = arg.description.as_ref().map(|description| description.resolve(terminology, locale).to_string());
    let with = |extra: Value| ActionDescriptor { controller_id: stage.controller_id.clone(), action: stage.action.clone(), args: staged_arg_args(stage.args.as_ref(), extra) };
    let text = value.map(|value| value.as_str().map(ToOwned::to_owned).unwrap_or_else(|| value.to_string())).unwrap_or_default();
    let number = value.and_then(staged_number);
    let facets = arg.number_facets(locale);
    let child = |id: String, label: String, control: Control| UiTreeItemNode { id, label: Label::data(label), control: Some(control), ..UiTreeItemNode::base(String::new(), Label::data(String::new())) };
    let field = |id: String, input_kind: &str, value: String, accessibility_label: Option<String>, facets: Option<&semio_framework::ActionArgNumberFacets>, on_change: ActionDescriptor| {
        Control::Input(UiInputNode {
            id,
            input_kind: input_kind.into(),
            value,
            placeholder: Some(Label::data(label.clone())),
            accessibility_label: accessibility_label.map(Label::data),
            commit: Some("blur".into()),
            min: facets.and_then(|facets| facets.min),
            max: facets.and_then(|facets| facets.max),
            step: facets.and_then(|facets| facets.step),
            accept: None,
            precision: facets.and_then(|facets| facets.precision),
            snaps: facets.map(|facets| facets.snaps.clone()).unwrap_or_default(),
            display_factor: facets.and_then(|facets| facets.display_factor),
            limits: facets.map(|facets| facets.limits.clone()),
            on_change,
            on_submit: None,
            on_abort: None,
            on_repeat_last: None,
            presence: UiPresence::default(),
            menu: None,
            ..Default::default()
        })
    };
    let (control, mut items) = match arg.control() {
        semio_framework::ActionArgControl::Select { options } => (
            Some(Control::Select(UiSelectNode {
                presence: UiPresence::default(),
                id: control_id.to_string(),
                value: text,
                items: options.iter().map(|option| UiSelectItem { value: option.value.clone(), label: Label::data(option.label.resolve(terminology, locale)) }).collect(),
                placeholder: Some(Label::data(label.clone())),
                on_change: stage.clone(),
                menu: None,
            })),
            None,
        ),
        semio_framework::ActionArgControl::Segmented { options } => (
            None,
            Some(
                options
                    .iter()
                    .map(|option| {
                        let option_label = option.label.resolve(terminology, locale).to_string();
                        child(
                            format!("{row_id}.option.{}", option.value),
                            option_label.clone(),
                            Control::Toggle(UiToggleNode {
                                appearance: ui_contract::ToggleAppearance::Button,
                                id: format!("{control_id}.option.{}", option.value),
                                icon_id: IconName::Check,
                                text: Some(Label::data(option_label)),
                                on_change: with(serde_json::json!({ "option": option.value })),
                                presence: UiPresence { selected: text == option.value, ..UiPresence::default() },
                                menu: None,
                            }),
                        )
                    })
                    .collect(),
            ),
        ),
        semio_framework::ActionArgControl::Toggle => (
            Some(Control::Toggle(UiToggleNode {
                appearance: ui_contract::ToggleAppearance::Button,
                id: control_id.to_string(),
                icon_id: IconName::Check,
                text: Some(Label::data(label.clone())),
                on_change: stage.clone(),
                presence: UiPresence { selected: value.and_then(Value::as_bool).unwrap_or(text == "true"), ..UiPresence::default() },
                menu: None,
            })),
            None,
        ),
        semio_framework::ActionArgControl::Slider { .. } | semio_framework::ActionArgControl::Dial { .. } => {
            let facets = facets.unwrap_or_default();
            let min = facets.min.unwrap_or(0.0);
            (
                Some(Control::Slider(UiSliderNode {
                    id: control_id.to_string(),
                    value: number.unwrap_or(min),
                    min,
                    max: facets.max.unwrap_or(0.0),
                    step: facets.step.unwrap_or(1.0),
                    unit: facets.unit,
                    snaps: facets.snaps,
                    appearance: facets.appearance,
                    scale: facets.scale,
                    precision: facets.precision,
                    display_unit: facets.display_unit,
                    display_factor: facets.display_factor,
                    limits: Some(facets.limits),
                    on_change: stage.clone(),
                    ..Default::default()
                })),
                None,
            )
        }
        semio_framework::ActionArgControl::Stepper { .. } => {
            let facets = facets.unwrap_or_default();
            let unit = facets.shown_unit().map(ToOwned::to_owned);
            (
                Some(Control::NumberStepper(UiNumberStepperNode {
                    id: control_id.to_string(),
                    value: number.or(facets.min).unwrap_or(0.0),
                    step: facets.step.unwrap_or(1.0),
                    uniform: true,
                    min: facets.min,
                    max: facets.max,
                    precision: facets.precision,
                    snaps: facets.snaps,
                    unit,
                    display_factor: facets.display_factor,
                    limits: Some(facets.limits),
                    on_absolute: stage.clone(),
                    on_delta: ActionDescriptor { controller_id: String::new(), action: String::new(), args: None },
                    ..Default::default()
                })),
                None,
            )
        }
        semio_framework::ActionArgControl::Number { .. } => (Some(field(control_id.to_string(), "number", text, None, facets.as_ref(), stage.clone())), None),
        semio_framework::ActionArgControl::Vector { dims, .. } => {
            let dims = dims as usize;
            let facets = facets.unwrap_or_default();
            let mut tuple: Vec<f64> = value.and_then(Value::as_array).map(|components| components.iter().map(|component| staged_number(component).unwrap_or(0.0)).collect()).unwrap_or_default();
            tuple.resize(dims, 0.0);
            let axes = ["x", "y", "z", "w"];
            let items = (0..dims)
                .map(|index| {
                    let axis = axes.get(index).map_or_else(|| index.to_string(), |axis| (*axis).to_string());
                    let on_change = with(serde_json::json!({ "index": index, "dims": dims, "tuple": tuple }));
                    let shown = match (facets.display_factor, facets.precision) {
                        (None, Some(digits)) => ui_contract::format_ui_number_fixed(tuple[index], digits),
                        _ => ui_contract::format_ui_number(tuple[index]),
                    };
                    child(format!("{row_id}.{axis}"), axis.clone(), field(format!("{control_id}.{axis}"), "number", shown, Some(format!("{label} {axis}")), Some(&facets), on_change))
                })
                .collect();
            (None, Some(items))
        }
        semio_framework::ActionArgControl::Color { alpha } => {
            let mut rgba = [0.0, 0.0, 0.0, 1.0];
            for (slot, component) in rgba.iter_mut().zip(value.and_then(Value::as_array).into_iter().flatten()) {
                *slot = staged_number(component).unwrap_or(0.0).clamp(0.0, 1.0);
            }
            let color = serde_json::json!({ "color": { "alpha": alpha, "rgba": &rgba[..if alpha { 4 } else { 3 }] } });
            let hex = LocalizedLabel::native("Hex", "Hex").resolve(terminology, locale).to_string();
            let opacity = LocalizedLabel::native("Opacity", "Deckkraft").resolve(terminology, locale).to_string();
            let mut items = vec![
                child(format!("{row_id}.swatch"), label.clone(), field(format!("{control_id}.swatch"), "color", ui_contract::ui_color_hex(&rgba, false), Some(label.clone()), None, with(color.clone()))),
                child(format!("{row_id}.hex"), hex.clone(), field(format!("{control_id}.hex"), "text", ui_contract::ui_color_hex(&rgba, alpha), Some(hex), None, with(color.clone()))),
            ];
            if alpha {
                items.push(child(format!("{row_id}.alpha"), opacity, Control::Slider(UiSliderNode { id: format!("{control_id}.alpha"), value: rgba[3], min: 0.0, max: 1.0, step: 0.01, unit: None, snaps: Vec::new(), on_change: with(color), presence: UiPresence::default(), menu: None, ..Default::default() })));
            }
            (None, Some(items))
        }
        semio_framework::ActionArgControl::Reference { domain, granularity, many, id_type, .. } => {
            let text = |id: &Value| semio_framework::reference_id_text(&semio_framework_value::DslValue::from(id));
            let ids: Vec<String> = match value {
                Some(Value::Array(ids)) => ids.iter().filter_map(text).collect(),
                Some(id) => text(id).into_iter().collect(),
                None => Vec::new(),
            };
            let references = |ids: Vec<String>| {
                let mut values = ids.iter().filter_map(|id| id_type.id_value(id)).map(Value::from);
                if many { Value::Array(values.collect()) } else { values.next().unwrap_or(Value::Null) }
            };
            let use_label = LocalizedLabel::native("Use current selection", "Aktuelle Auswahl verwenden").resolve(terminology, locale).to_string();
            let remove = LocalizedLabel::native("Remove {item}", "{item} entfernen").resolve(terminology, locale).to_string();
            let empty = LocalizedLabel::native("Nothing selected", "Nichts ausgewählt").resolve(terminology, locale).to_string();
            let picked = reference_selection_ids(selection, domain.as_deref(), granularity.as_deref());
            let mut items: Vec<UiTreeItemNode> = ids
                .iter()
                .map(|id| {
                    let remaining = references(ids.iter().filter(|other| *other != id).cloned().collect());
                    child(
                        format!("{row_id}.reference.{id}"),
                        id.clone(),
                        Control::Button(UiButtonNode { id: Some(format!("{control_id}.remove.{id}")), icon_id: IconName::X, label: Label::data(remove.replace("{item}", id)), action: with(serde_json::json!({ "value": remaining })), style: None, presence: UiPresence::default(), menu: None }),
                    )
                })
                .collect();
            if ids.is_empty() {
                items.push(UiTreeItemNode { id: format!("{row_id}.empty"), label: Label::data(empty), dimmed: Some(true), ..UiTreeItemNode::base(String::new(), Label::data(String::new())) });
            }
            let use_selection = UiButtonNode {
                id: Some(format!("{control_id}.useSelection")),
                icon_id: IconName::Crosshair,
                label: Label::data(use_label.clone()),
                action: with(serde_json::json!({ "value": references(picked.clone()) })),
                style: None,
                presence: UiPresence { state: if picked.is_empty() { ui_wgpu::wgpu::component::ui::UiState::Disabled } else { ui_wgpu::wgpu::component::ui::UiState::Normal }, ..UiPresence::default() },
                menu: None,
            };
            items.push(child(format!("{row_id}.useSelection"), use_label, Control::Button(use_selection)));
            (None, Some(items))
        }
        _ => (Some(field(control_id.to_string(), "text", text, None, None, stage.clone())), None),
    };
    if arg.nullable {
        let clear_label = match locale {
            Locale::En => "Clear",
            Locale::De => "Leeren",
        };
        let cleared = matches!(value, Some(Value::Null));
        let clear = UiButtonNode {
            id: Some(format!("{control_id}.clear")),
            icon_id: IconName::X,
            label: Label::data(format!("{clear_label} {label}")),
            action: with(serde_json::json!({ "value": Value::Null })),
            style: None,
            presence: UiPresence { state: if cleared { ui_wgpu::wgpu::component::ui::UiState::Disabled } else { ui_wgpu::wgpu::component::ui::UiState::Normal }, ..UiPresence::default() },
            menu: None,
        };
        items.get_or_insert_with(Vec::new).push(child(format!("{row_id}.clear"), clear_label.to_string(), Control::Button(clear)));
    }
    UiTreeItemNode { id: row_id, label: Label::data(label), description, control, default_open: items.as_ref().map(|_| true), items, ..UiTreeItemNode::base(String::new(), Label::data(String::new())) }
}

/// 🧮️ A staged value read as a number — a number, or the text a number field committed.
fn staged_number(value: &Value) -> Option<f64> {
    value.as_f64().or_else(|| value.as_str().and_then(|text| text.trim().parse::<f64>().ok())).filter(|number| number.is_finite())
}

/// 🧩️ A stage verb's own coordinates (`window`/`action`/`arg` or `command`/`arg`) with one editor part's extra keys.
fn staged_arg_args(base: Option<&DslValue>, extra: Value) -> Option<DslValue> {
    let mut args = base.map(dsl_value_as_json).and_then(|args| args.as_object().cloned()).unwrap_or_default();
    if let Value::Object(extra) = extra {
        args.extend(extra);
    }
    Some(semio_framework_value::DslValue::from(Value::Object(args)))
}

/// 📝️ The value one staged-arg dispatch stages: a segmented button's `option`, a colour part's colour, a vector axis
/// spliced into the tuple at `index` (the tuple already staged, else the `tuple` the row painted), else the editor's
/// own `value`.
pub(crate) fn staged_arg_value(args: &Value, current: Option<&Value>) -> Value {
    if let Some(option) = args.get("option") {
        return option.clone();
    }
    if let Some(color) = args.get("color") {
        return staged_color_value(color, args.get("value"), current);
    }
    if let (Some(index), Some(dims)) = (args.get("index").and_then(Value::as_u64), args.get("dims").and_then(Value::as_u64)) {
        let mut tuple: Vec<Value> = current.or_else(|| args.get("tuple")).and_then(Value::as_array).cloned().unwrap_or_default();
        tuple.resize(usize::try_from(dims).unwrap_or(0), Value::from(0.0));
        if let (Some(slot), Some(number)) = (usize::try_from(index).ok().and_then(|index| tuple.get_mut(index)), args.get("value").and_then(staged_number)) {
            *slot = Value::from(number);
        }
        return Value::Array(tuple);
    }
    args.get("value").cloned().unwrap_or(Value::Null)
}

/// 🎨️ A colour row's staged value: the hex its swatch or hex field committed (`ui_contract::parse_ui_color_hex`, keeping
/// the alpha when the text carries none) or the alpha its slider committed, over the colour already staged, else the
/// one the row painted; an unreadable hex keeps the colour.
pub(crate) fn staged_color_value(color: &Value, value: Option<&Value>, current: Option<&Value>) -> Value {
    let alpha = color.get("alpha").and_then(Value::as_bool).unwrap_or(false);
    let mut rgba = [0.0, 0.0, 0.0, 1.0];
    for (slot, component) in rgba.iter_mut().zip(current.or_else(|| color.get("rgba")).and_then(Value::as_array).into_iter().flatten()) {
        *slot = staged_number(component).unwrap_or(0.0).clamp(0.0, 1.0);
    }
    match value {
        Some(Value::String(text)) => {
            if let Some(parsed) = ui_contract::parse_ui_color_hex(text) {
                let typed_alpha = matches!(text.trim().trim_start_matches('#').len(), 4 | 8);
                rgba = [parsed[0], parsed[1], parsed[2], if typed_alpha { parsed[3] } else { rgba[3] }];
            }
        }
        Some(number) => {
            if let Some(number) = staged_number(number) {
                rgba[3] = number.clamp(0.0, 1.0);
            }
        }
        None => {}
    }
    Value::Array(rgba[..if alpha { 4 } else { 3 }].iter().map(|component| Value::from(*component)).collect())
}

/// 🎯️ The ids "use current selection" stages for a reference argument: the live selection of its `domain` (or, when
/// it names none, of the one domain that selects anything), kept only at its `granularity` when it declares one.
pub(crate) fn reference_selection_ids(selection: &HashMap<String, semio_framework::DomainSelection>, domain: Option<&str>, granularity: Option<&str>) -> Vec<String> {
    let picked = match domain {
        Some(domain) => selection.get(domain),
        None => {
            let mut live = selection.values().filter(|selected| !selected.ids.is_empty());
            let first = live.next();
            if live.next().is_some() {
                None
            } else {
                first
            }
        }
    };
    picked.filter(|selected| granularity.is_none_or(|granularity| selected.granularity == granularity)).map(|selected| selected.ids.clone()).unwrap_or_default()
}

/// ⌨️ A control id as a readable row label — React's `humanizeControlId`: the last dot segment,
/// camelCase split on word boundaries, first letter upper-cased.
pub(crate) fn humanize_control_id(control_id: &str) -> String {
    let leaf = control_id.rsplit('.').next().unwrap_or(control_id);
    let mut out = String::new();
    for (index, ch) in leaf.chars().enumerate() {
        if index > 0 && ch.is_ascii_uppercase() {
            out.push(' ');
            out.push(ch.to_ascii_lowercase());
            continue;
        }
        out.push(if index == 0 { ch.to_ascii_uppercase() } else { ch });
    }
    out
}

/// 🧾️ The variant name a refused `UiNode` reports under — the assembler's own error vocabulary.
fn panel_ui_node_tag(node: &UiNode) -> &'static str {
    match node {
        UiNode::Stack(_) => "stack",
        UiNode::Text(_) => "text",
        UiNode::Button(_) => "button",
        UiNode::Separator(_) => "separator",
        UiNode::Input(_) => "input",
        UiNode::Select(_) => "select",
        UiNode::Toggle(_) => "toggle",
        UiNode::KeyValue(_) => "keyValue",
        UiNode::Slider(_) => "slider",
        UiNode::NumberStepper(_) => "numberStepper",
        UiNode::Ring(_) => "ring",
        UiNode::IconSelect(_) => "iconSelect",
        UiNode::Progress(_) => "progress",
        UiNode::Field(_) => "field",
        UiNode::Section(_) => "section",
        UiNode::Group(_) => "group",
        UiNode::Tree(_) => "tree",
        UiNode::Image(_) => "image",
        UiNode::ComponentScene(_) => "componentScene",
        UiNode::ExternalSlot(_) => "externalSlot",
    }
}
//#endregion 🧾️PanelRecordsAssembler

//#region ⏯️ToolRunPanel
/// ⏯️ Whether the freshly rendered framework ToolRun panel `document` holds a run `seen` has not — the moment the shell
/// reveals the panel, exactly as the React `ShellHost` does through `toolRunPanelReveal`. The group-key law is
/// [`semio_framework::tool_run_panel_new_runs`], pinned by the `⏯️tool-run` lifecycle fixture.
pub(crate) fn tool_run_panel_reveals(seen: &mut std::collections::BTreeSet<u64>, document: &UiDocumentLease) -> Result<bool, String> {
    let header = document.header().map_err(|error| format!("the ToolRun panel header is unreadable: {error:?}"))?;
    let mut keys = Vec::with_capacity(header.node_count);
    for index in 0..header.node_count {
        if let Some(page) = document.read_node_page(index).map_err(|error| format!("the ToolRun panel node {index} is unreadable: {error:?}"))? {
            keys.push(page.record().key.as_str().to_string());
        }
    }
    let (runs, added) = semio_framework::tool_run_panel_new_runs(seen, keys.iter().map(String::as_str));
    *seen = runs;
    Ok(!added.is_empty())
}

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧪️tests/⏯️wgpu-tool-run-panel/🦀️.rs"]
mod tool_run_panel_tests;
//#endregion ⏯️ToolRunPanel

//#region 🛍️AppCatalogueAttempt
/// 🛍️ The "fetch the app-static catalogue once per app instance" rule, owned in ONE place because it
/// has to hold on BOTH outcomes.
///
/// `recorded` is the instance the catalogue was last ATTEMPTED for, not the one it last succeeded
/// for. `true` means the caller now owns that attempt and must perform the fetch; every later call
/// for the same instance answers `false` whether the fetch succeeded, faulted or never published.
/// Recording only successes made a failed catalogue re-fetch once per settled command, and the
/// second fetch of `framework.section.catalogue` never returned from its own turn — which wedged
/// `boot_shell` before the canvas ever bound a pointer listener
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-input-hit-runtime-2026-09-13.md`).
///
/// A new instance always re-claims, so switching app instance still refetches exactly once.
pub(crate) fn claim_app_catalogue_fetch(recorded: &mut Option<u32>, instance_id: u32) -> bool {
    if *recorded == Some(instance_id) {
        return false;
    }
    *recorded = Some(instance_id);
    true
}
//#endregion 🛍️AppCatalogueAttempt

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧪️tests/🛍️app-catalogue-attempt/🦀️.rs"]
mod app_catalogue_attempt_tests;

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧪️tests/🪟️action-window-scope/🦀️.rs"]
mod action_window_scope_tests;
//#endregion ShellLifecycle
