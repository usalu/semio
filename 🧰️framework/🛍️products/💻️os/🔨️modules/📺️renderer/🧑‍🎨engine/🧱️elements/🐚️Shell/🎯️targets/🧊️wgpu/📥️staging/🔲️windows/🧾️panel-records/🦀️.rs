//! 🔲️ `shell-windows` staging of `panel-records`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellLifecycle
//#region 🧾️PanelRecordsAssembler
pub fn panel_ui_scroll_records(surface_id: &str, root: &UiNode) -> Result<Vec<ui_contract::UiNodeRecord>, String> {
    let mut projection = PanelProjection { surface_id, records: Vec::new(), key_seq: 0 };
    let root_id = projection.node(root)?;
    let Some(record) = projection.records.get_mut((root_id.0 - 1) as usize).and_then(Option::as_mut) else {
        return Err(format!("panel '{surface_id}' projection lost its root record"));
    };
    record.layout = ui_contract::LayoutSpec::Scroll(ui_contract::ScrollLayout { axes: ui_contract::ScrollAxes::Vertical, padding: ui_contract::EdgeSpace::default(), sizing: ui_contract::Sizing::Fill });
    projection.records.into_iter().collect::<Option<Vec<_>>>().ok_or_else(|| format!("panel '{surface_id}' projection left a reserved record unplaced"))
}

/// 🌳️ One shell-owned panel document under construction — same reserve-then-place idiom as
/// {@link WindowMeasuresProjection}, so the published record order is parent-first.
pub struct PanelProjection<'a> {
    pub surface_id: &'a str,
    pub records: Vec<Option<ui_contract::UiNodeRecord>>,
    pub key_seq: u32,
}

impl PanelProjection<'_> {
    pub fn reserve(&mut self) -> ui_contract::UiNodeId {
        self.records.push(None);
        ui_contract::UiNodeId(self.records.len() as u64)
    }

    pub fn key(&mut self, id: Option<&str>) -> String {
        self.key_seq += 1;
        match id.filter(|id| !id.is_empty()) {
            Some(id) => format!("{}/{id}", self.surface_id),
            None => format!("{}/n{}", self.surface_id, self.key_seq),
        }
    }

    /// 🧳️ Preserves a Tree row's native transfer map across the bounded retained-document seam.
    pub fn drag_data(&self, item_id: &str, source: Option<&HashMap<String, String>>) -> Result<Option<ui_contract::UiFixedMap<UiText>>, String> {
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

    pub fn place(&mut self, id: ui_contract::UiNodeId, key: String, component: ui_contract::Component, layout: ui_contract::LayoutSpec, record: PanelRecord) -> Result<ui_contract::UiNodeId, String> {
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

    pub fn stack_layout(axis: ui_contract::Axis, gap: ui_contract::SpaceToken) -> ui_contract::LayoutSpec {
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
    pub fn node(&mut self, node: &UiNode) -> Result<ui_contract::UiNodeId, String> {
        let presence = node.presence();
        let activity = match presence.status {
            semio_framework_ui_declaration::ui::UiStatus::Loading => ui_contract::Activity::Loading,
            semio_framework_ui_declaration::ui::UiStatus::Waiting => ui_contract::Activity::Waiting,
            semio_framework_ui_declaration::ui::UiStatus::Finished => ui_contract::Activity::Finished,
            semio_framework_ui_declaration::ui::UiStatus::Idle => ui_contract::Activity::Idle,
        };
        let disabled = matches!(presence.state, semio_framework_ui_declaration::ui::UiState::Disabled);
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
    pub fn tree_item(&mut self, item: &UiTreeItemNode) -> Result<ui_contract::UiNodeId, String> {
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
            PanelRecord { children, disabled: matches!(item.presence.state, semio_framework_ui_declaration::ui::UiState::Disabled), ..PanelRecord::default() },
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
                semio_framework_ui_declaration::ui::UiTreeActionPlacement::Row => ui_contract::RowActionPlacement::Row,
                semio_framework_ui_declaration::ui::UiTreeActionPlacement::Menu => ui_contract::RowActionPlacement::Menu,
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

/// 🛠️ React's `windowMeasuresToTreeItems` (`🛠️ShellHelpers/🟦️.tsx`): one tree row per measure,
/// the row's id the measure's OWN id and the row's control the measure's own editor. Declaration
/// order is written straight through — React reverses only to cancel its bottom-anchored `Tree`'s
/// own per-level reversal (`reverseForUpPanel`), which this renderer's panel tree does not do.
pub fn window_measure_tree_rows(measures: &[WindowMeasure]) -> Vec<UiTreeItemNode> {
    use semio_framework_ui_declaration::ui::{UiControlNode, UiState, UiStatus};
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

//#endregion 🧾️PanelRecordsAssembler
//#endregion ShellLifecycle

//#region ShellLifecycle
//#region 🧾️PanelRecordsAssembler
/// 🧾️ The variant name a refused `UiNode` reports under — the assembler's own error vocabulary.
pub fn panel_ui_node_tag(node: &UiNode) -> &'static str {
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

/// 🎛️ One [`UiControlNode`] as the [`UiNode`] it IS — the enum is a narrowed projection of the
/// same variants, so a tree row's inline control projects through the one assembler every other node
/// takes rather than a second, parallel arm table.
pub fn control_ui_node(control: &semio_framework_ui_declaration::ui::UiControlNode) -> UiNode {
    use semio_framework_ui_declaration::ui::UiControlNode;
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

/// 🎛️ A nested tree leaf that carries only an Activate verb (no control, no further children) is
/// React's inline resolution/control slot, not another expandable row.
pub fn tree_item_is_inline_action_leaf(item: &UiTreeItemNode) -> bool {
    item.action.is_some() && item.control.is_none() && item.default_open.is_none() && item.items.as_ref().map_or(true, |items| items.is_empty())
}

/// 🧾️ Everything a panel record carries beside its key, component and layout.
#[derive(Default)]
pub struct PanelRecord {
    pub children: ui_contract::UiNodeChildren,
    pub bindings: ui_contract::UiNodeBindings,
    pub activity: ui_contract::Activity,
    pub disabled: bool,
    pub label: Option<ui_contract::Label>,
    pub description: Option<ui_contract::Label>,
}

pub fn panel_component_scene_props(scene: &semio_framework_ui_declaration::ui::UiComponentSceneNode) -> Result<ui_contract::SurfaceProps, String> {
    let kind = match scene.component_kind {
        semio_framework_ui_declaration::ui::SurfaceKind::Canvas2d => ui_contract::SurfaceKind::Canvas2d,
        semio_framework_ui_declaration::ui::SurfaceKind::World3d => ui_contract::SurfaceKind::World3d,
        semio_framework_ui_declaration::ui::SurfaceKind::NodeGraph => ui_contract::SurfaceKind::NodeGraph,
        semio_framework_ui_declaration::ui::SurfaceKind::TextEditor => ui_contract::SurfaceKind::TextEditor,
        semio_framework_ui_declaration::ui::SurfaceKind::Table => ui_contract::SurfaceKind::Table,
        semio_framework_ui_declaration::ui::SurfaceKind::Paint2d => ui_contract::SurfaceKind::Paint2d,
        semio_framework_ui_declaration::ui::SurfaceKind::VirtualFileSystem => ui_contract::SurfaceKind::VirtualFileSystem,
        semio_framework_ui_declaration::ui::SurfaceKind::TiledMap => ui_contract::SurfaceKind::TiledMap,
        semio_framework_ui_declaration::ui::SurfaceKind::Board2d => ui_contract::SurfaceKind::Board2d,
        semio_framework_ui_declaration::ui::SurfaceKind::IconRender => ui_contract::SurfaceKind::IconRender,
        semio_framework_ui_declaration::ui::SurfaceKind::InkCanvas => ui_contract::SurfaceKind::InkCanvas,
        semio_framework_ui_declaration::ui::SurfaceKind::GraphTimeline => ui_contract::SurfaceKind::GraphTimeline,
        semio_framework_ui_declaration::ui::SurfaceKind::BlockList => ui_contract::SurfaceKind::BlockList,
        semio_framework_ui_declaration::ui::SurfaceKind::DiffView => ui_contract::SurfaceKind::DiffView,
        semio_framework_ui_declaration::ui::SurfaceKind::EventFeed => ui_contract::SurfaceKind::EventFeed,
    };
    let encoded = match scene.component_kind {
        semio_framework_ui_declaration::ui::SurfaceKind::Canvas2d => scene.canvas_2d.as_ref().map(|value| semio_framework_ui_declaration::ui::encode_surface_doc(kind, value)),
        semio_framework_ui_declaration::ui::SurfaceKind::World3d => scene.world_3d.as_ref().map(|value| semio_framework_ui_declaration::ui::encode_surface_doc(kind, value)),
        semio_framework_ui_declaration::ui::SurfaceKind::NodeGraph => scene.node_graph.as_ref().map(|value| semio_framework_ui_declaration::ui::encode_surface_doc(kind, value)),
        semio_framework_ui_declaration::ui::SurfaceKind::TextEditor => scene.text_editor.as_ref().map(|value| semio_framework_ui_declaration::ui::encode_surface_doc(kind, value)),
        semio_framework_ui_declaration::ui::SurfaceKind::Table => scene.table.as_ref().map(|value| semio_framework_ui_declaration::ui::encode_surface_doc(kind, value)),
        semio_framework_ui_declaration::ui::SurfaceKind::Paint2d => scene.paint_2d.as_ref().map(|value| semio_framework_ui_declaration::ui::encode_surface_doc(kind, value)),
        semio_framework_ui_declaration::ui::SurfaceKind::VirtualFileSystem => scene.virtual_file_system.as_ref().map(|value| semio_framework_ui_declaration::ui::encode_surface_doc(kind, value)),
        semio_framework_ui_declaration::ui::SurfaceKind::TiledMap => scene.tiled_map.as_ref().map(|value| semio_framework_ui_declaration::ui::encode_surface_doc(kind, value)),
        semio_framework_ui_declaration::ui::SurfaceKind::Board2d => scene.board2d.as_ref().map(|value| semio_framework_ui_declaration::ui::encode_surface_doc(kind, value)),
        semio_framework_ui_declaration::ui::SurfaceKind::IconRender => scene.icon_render.as_ref().map(|value| semio_framework_ui_declaration::ui::encode_surface_doc(kind, value)),
        semio_framework_ui_declaration::ui::SurfaceKind::InkCanvas => scene.ink_canvas.as_ref().map(|value| semio_framework_ui_declaration::ui::encode_surface_doc(kind, value)),
        semio_framework_ui_declaration::ui::SurfaceKind::GraphTimeline => scene.graph_timeline.as_ref().map(|value| semio_framework_ui_declaration::ui::encode_surface_doc(kind, value)),
        semio_framework_ui_declaration::ui::SurfaceKind::BlockList => scene.block_list.as_ref().map(|value| semio_framework_ui_declaration::ui::encode_surface_doc(kind, value)),
        semio_framework_ui_declaration::ui::SurfaceKind::DiffView => scene.diff_view.as_ref().map(|value| semio_framework_ui_declaration::ui::encode_surface_doc(kind, value)),
        semio_framework_ui_declaration::ui::SurfaceKind::EventFeed => scene.event_feed.as_ref().map(|value| semio_framework_ui_declaration::ui::encode_surface_doc(kind, value)),
    };
    encoded
        .ok_or_else(|| format!("panel ComponentScene '{}' has no {:?} payload", scene.surface_id, scene.component_kind))?
        .map_err(|error| format!("panel ComponentScene '{}' payload exceeds the retained surface contract: {error:?}", scene.surface_id))
}
//#endregion 🧾️PanelRecordsAssembler
//#endregion ShellLifecycle
