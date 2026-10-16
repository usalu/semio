//! 🪝️ `shell-window-actions` staging of `window-actions-and-commands`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ActionPanelAndUtilities
//#region 🎬️WindowActionsAndSearchPanes
/// 🎬️ The retained engine surface one window instance's ACTIONS pane body paints into — React's
/// top-left `Pane` (`🪟️Window/🟦️.tsx:379`), whose body is `<Engagement/>` followed by the
/// `WindowActionPane` tree. Keyed like the Measures overlay: the window instance, then the reserved
/// section body key, so it can never collide with an app-authored surface.
pub(crate) fn window_actions_surface_id(window_id: &str) -> String {
    format!("{window_id}/{}", semio_framework::UiRefreshSection::Engagements.body_key())
}

/// 🔎️ The retained engine surface one window instance's SEARCH pane body paints into.
pub(crate) fn window_search_surface_id(window_id: &str) -> String {
    format!("{window_id}/{WINDOW_SEARCH_BODY_KEY}")
}

/// 📐️ React's `PANE_DEFAULT_SIZE` (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:9912`) — every `Pane` that
/// declares no `size` opens 300 logical px wide, which both these bodies do.
pub(crate) const WINDOW_PANE_BODY_WIDTH_PX: f32 = 300.0;

/// 🚦️ React's `FRAMEWORK_RESERVED_ACTION_IDS` (`🛠️ShellHelpers/🟦️.tsx:308`) — the framework's OWN
/// verbs, which an armed utility never gates: their chords keep firing while a brush owns the
/// pointer, so a pane row that refused them at the same moment would contradict its own keybinding.
pub(crate) const FRAMEWORK_RESERVED_ACTION_IDS: [&str; 22] = [
    "undo",
    "redo",
    "commitCheckpoint",
    "createAlternative",
    "switchAlternative",
    "checkoutCheckpoint",
    "copy",
    "cut",
    "paste",
    "revertToCommand",
    "setHistoryCommandFilter",
    "noteShellCommand",
    "recordTutorial",
    "startIntroduction",
    "startTutorial",
    "setActiveUtility",
    "setActiveTool",
    semio_framework::EXPORT_ARTIFACT_DOCUMENT_ACTION_ID,
    semio_framework::IMPORT_ARTIFACT_DOCUMENT_ACTION_ID,
    semio_framework::OPEN_ARTIFACT_FILE_ACTION_ID,
    semio_framework::SAVE_ARTIFACT_FILE_ACTION_ID,
    semio_framework::HOST_EVENT_ACTION_ID,
];

/// 🗂️ React's `actionCategoryId` (`🛠️ShellHelpers/🟦️.tsx`): the declared category, else `history` for
/// a history verb, else `actions`.
fn action_category_id(action: &semio_framework::ActionDefinition) -> String {
    action.category.clone().unwrap_or_else(|| if action.kind == semio_framework::ActionKind::History { "history".to_string() } else { "actions".to_string() })
}

/// 🗂️ React's `actionCategoryLabel`: the shared `ui.ribbon.parent.*` chrome vocabulary for a known
/// taxonomy id, else the raw id. (React consults the app's `group` label overlay first; this renderer
/// carries no overlay, the same scoping every other shell-owned label here takes.)
fn action_category_label(category: &str, is_de: bool) -> String {
    ui_wgpu::wgpu::ribbon_parent_label(category, is_de).unwrap_or(category).to_string()
}

/// 🧰️ React's `actionRequiresStagedForm`: an action carrying a user-facing argument stages a form
/// instead of firing (P1–P4); an action whose every argument is hidden fires bare.
fn action_requires_staged_form(action: &semio_framework::ActionDefinition) -> bool {
    action.args.iter().any(|arg| !matches!(arg.presentation, Some(semio_framework::ArgPresentation::Hidden)))
}

/// 🆔️ React's `ButtonGroup id={UI_ENGAGEMENT.actions}` — the one id the engagement's quick-action
/// row carries (`🖥️ui/🎯️targets/⚛️react/🟦️.tsx:10198`/`:11032`).
const ENGAGEMENT_OPTIONS_CONTROL_ID: &str = "ui.engagement.actions";

/// 🏷️ React's `engagementControlLabel` — a numeric control with a unit reads `Label (unit)`; a
/// ring, toggle group or select never appends one (`🖥️ui/🎯️targets/⚛️react/🟦️.tsx:10638`).
fn engagement_control_label(control: &ui_wgpu::wgpu::WindowEngagementControl) -> Option<String> {
    use ui_wgpu::wgpu::WindowEngagementControl as Control;
    let (label, unit) = match control {
        Control::Slider { label, unit, .. } | Control::Stepper { label, unit, .. } => (label.as_ref()?, unit.as_deref()),
        Control::Ring { label, .. } | Control::ToggleGroup { label, .. } | Control::Select { label, .. } => (label.as_ref()?, None),
    };
    Some(match unit.filter(|unit| !unit.is_empty()) {
        Some(unit) => format!("{label} ({unit})"),
        None => label.clone(),
    })
}

/// 🎛️ ONE engagement control as React's `EngagementControlView` renders it
/// (`🖥️ui/🎯️targets/⚛️react/🟦️.tsx:10648`-`:10751`): an optional label row, then the control under
/// React's OWN id — `control.id` when the guest names one, else the `engagement-control.<kind>`
/// fallback React writes for each kind. Empty-option rings, toggle groups and selects render NOTHING,
/// which is React's `if (!control.options.length) return null`.
///
/// ⚖️ A toggle group is React's `ButtonGroup`, whose items carry the OPTION's own id and dispatch
/// `onSelect(option.id)`; this renderer's `Toggle` is the row that can also read PRESSED, which is
/// what React paints the selected option with (`interactiveActiveFillClass`).
///
/// 💍️ `RingProps` carries ONE orb per record (`🧬️contract/🧩️component/🦀️.rs`), where React's `<Ring/>`
/// takes the whole orb list on one element — so the ring projects as a container under React's ring
/// id holding one orb node per option, each carrying the orb id React exposes as `data-orb-id` and
/// each dispatching the same `onOrbSelect` verb with its own `id`.
pub(crate) fn engagement_control_rows(control: &ui_wgpu::wgpu::WindowEngagementControl, is_de: bool) -> Vec<UiNode> {
    use ui_wgpu::wgpu::WindowEngagementControl as Control;
    let inert = || ActionDescriptor { controller_id: "framework".into(), action: String::new(), args: None };
    let with_id = |action: &Option<ActionDescriptor>, option_id: &str| -> Option<ActionDescriptor> {
        action.clone().map(|mut action| {
            let mut entries = match action.args.take() {
                Some(semio_framework_value::DslValue::Object(entries)) => entries,
                _ => Vec::new(),
            };
            entries.retain(|(key, _)| key != "id");
            entries.push(("id".to_string(), semio_framework_value::DslValue::String(option_id.to_string())));
            action.args = Some(semio_framework_value::DslValue::Object(entries));
            action
        })
    };
    let state = |disabled: bool| if disabled { ui_wgpu::wgpu::component::ui::UiState::Disabled } else { ui_wgpu::wgpu::component::ui::UiState::Normal };
    let mut body = match control {
        Control::Slider { id, value, min, max, step, unit, disabled, on_change, .. } => vec![UiNode::Slider(UiSliderNode {
            id: id.clone().unwrap_or_else(|| "engagement-control.slider".to_string()),
            value: *value,
            min: *min,
            max: *max,
            step: step.unwrap_or(1.0),
            unit: unit.clone(),
            snaps: Vec::new(),
            on_change: on_change.clone().unwrap_or_else(inert),
            presence: UiPresence { state: state(disabled.unwrap_or(false)), ..UiPresence::default() },
            menu: None,
            ..Default::default()
        })],
        Control::Stepper { id, value, step, disabled, on_change, .. } => vec![UiNode::NumberStepper(UiNumberStepperNode {
            id: id.clone().unwrap_or_else(|| "engagement-control.stepper".to_string()),
            value: *value,
            step: step.unwrap_or(1.0),
            uniform: false,
            min: None,
            max: None,
            precision: None,
            on_absolute: on_change.clone().unwrap_or_else(inert),
            on_delta: on_change.clone().unwrap_or_else(inert),
            presence: UiPresence { state: state(disabled.unwrap_or(false)), ..UiPresence::default() },
            menu: None,
            ..Default::default()
        })],
        Control::ToggleGroup { id, value, options, disabled, on_select, .. } => {
            if options.is_empty() {
                Vec::new()
            } else {
                vec![UiNode::Stack(UiStackNode {
                    direction: "horizontal".into(),
                    gap: None,
                    padding: None,
                    id: id.clone(),
                    children: options
                        .iter()
                        .map(|option| {
                            UiNode::Toggle(UiToggleNode {
                                appearance: ui_contract::ToggleAppearance::Button,
                                id: option.id.clone(),
                                icon_id: IconName::CircleDot,
                                text: Some(Label::data(option.label.clone())),
                                on_change: with_id(on_select, &option.id).unwrap_or_else(inert),
                                presence: UiPresence { selected: value.as_deref() == Some(option.id.as_str()), state: state(option.disabled.unwrap_or(false) || disabled.unwrap_or(false)), ..UiPresence::default() },
                                menu: None,
                            })
                        })
                        .collect(),
                    presence: UiPresence::default(),
                    activate: None,
                    drop_action: None,
                    drop_overlay: None,
                    menu: None,
                })]
            }
        }
        Control::Select { id, value, placeholder, items, disabled, on_change, .. } => {
            if items.is_empty() {
                Vec::new()
            } else {
                vec![UiNode::Select(UiSelectNode {
                    presence: UiPresence { state: state(disabled.unwrap_or(false)), ..UiPresence::default() },
                    id: id.clone().unwrap_or_else(|| "engagement-control.select".to_string()),
                    value: value.clone().unwrap_or_default(),
                    items: items.iter().map(|item| UiSelectItem { value: item.value.clone(), label: Label::data(item.label.clone()) }).collect(),
                    placeholder: Some(Label::data(placeholder.clone().unwrap_or_else(|| shell_chrome_string("common.select", is_de).to_string()))),
                    on_change: on_change.clone().unwrap_or_else(inert),
                    menu: None,
                })]
            }
        }
        Control::Ring { id, value, options, disabled, on_select, .. } => {
            if options.is_empty() {
                Vec::new()
            } else {
                let count = options.len() as f64;
                vec![UiNode::Stack(UiStackNode {
                    direction: "horizontal".into(),
                    gap: None,
                    padding: None,
                    id: Some(id.clone().unwrap_or_else(|| "engagement-control.ring".to_string())),
                    children: options
                        .iter()
                        .enumerate()
                        .map(|(index, option)| {
                            UiNode::Ring(UiRingNode {
                                id: option.id.clone(),
                                orb_id: option.id.clone(),
                                t: if options.len() > 1 { index as f64 / count } else { 0.0 },
                                on_change: with_id(on_select, &option.id).unwrap_or_else(inert),
                                presence: UiPresence { selected: value.as_deref() == Some(option.id.as_str()), state: state(option.disabled.unwrap_or(false) || disabled.unwrap_or(false)), ..UiPresence::default() },
                                menu: None,
                            })
                        })
                        .collect(),
                    presence: UiPresence::default(),
                    activate: None,
                    drop_action: None,
                    drop_overlay: None,
                    menu: None,
                })]
            }
        }
    };
    if body.is_empty() {
        return Vec::new();
    }
    let Some(label) = engagement_control_label(control) else { return body };
    let field_id = match control {
        Control::Slider { id, .. } => id.as_deref().unwrap_or("engagement-control.slider"),
        Control::Stepper { id, .. } => id.as_deref().unwrap_or("engagement-control.stepper"),
        Control::ToggleGroup { id, .. } => id.as_deref().unwrap_or("engagement-control.toggle-group"),
        Control::Select { id, .. } => id.as_deref().unwrap_or("engagement-control.select"),
        Control::Ring { id, .. } => id.as_deref().unwrap_or("engagement-control.ring"),
    };
    vec![UiNode::Field(UiFieldNode { id: format!("{field_id}.field"), label: Label::data(label), description: None, required: None, error: None, child: Box::new(body.remove(0)), presence: UiPresence::default(), menu: None })]
}

/// 📝️ One staged ACTION argument's row — [`staged_arg_row`] with React's own row id (`action.<actionId>.arg.<argId>`)
/// dispatching `stageActionArg`, so the value buffers in [`ShellState::staged_action_args`] and NOTHING fires until
/// Execute (P2).
///
/// 🌳️ React's form row IS a `TreeDataItem` whose `control` is the editor: the ROW carries
/// `action.<actionId>.arg.<argId>` and the editor inside it carries the bare `def.id`
/// (`renderStagedArgControl`'s `fieldId = field?.id ?? def.id`, `🛠️ShellHelpers/🟦️.tsx:3869`), which
/// is also why the two never collide on one published surface key.
pub(crate) fn staged_action_arg_row(window_id: &str, action_id: &str, arg: &semio_framework::ActionArgDef, value: Option<&Value>, selection: &HashMap<String, semio_framework::DomainSelection>, terminology: Terminology, locale: Locale) -> UiTreeItemNode {
    let stage = ActionDescriptor { controller_id: "framework".into(), action: "stageActionArg".into(), args: crate::action_args_json!({ "window": window_id.to_string(), "action": action_id.to_string(), "arg": arg.id.clone() }) };
    staged_arg_row(format!("action.{action_id}.arg.{}", arg.id), &arg.id, arg, value, &stage, selection, terminology, locale)
}

impl<'ctx> WindowActionsCtx<'ctx> {
    /// 🎬️ ONE window's Actions pane body — React's top-left `Pane` content, in its own order:
    /// `<Engagement/>` (the status line and the quick-action options) then the `WindowActionPane`
    /// tree, one section per category, a row per panel-eligible action
    /// (`buildActionCategoryTree`, `🛠️ShellHelpers/🟦️.tsx:4060`).
    ///
    /// ⚖️ A zero-arg row fires the app's verb directly; an arg-carrying row (`label…`) toggles the
    /// expansion instead, and the expanded action's staged form follows as its own
    /// `action.category.<id>.form` section with Execute/Reset — React's list/form split verbatim.
    ///
    /// 🚦️ While an active utility declares `allows_actions_while_active: false` every row renders
    /// disabled, EXCEPT the framework's own reserved verbs, which stay live because their chords do
    /// (React's `FRAMEWORK_RESERVED_ACTION_IDS` gate, `🛠️ShellHelpers/🟦️.tsx:4085`).
    ///
    /// 🗣️ React's `<Engagement/>` body order, top to bottom: the session's own step heading, the
    /// primary `control`, the `controls` row, the remaining status lines, then the quick-action
    /// options (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:11004`-`:11045`). Only a LIVE session promotes the
    /// `engagement-step` row into the heading; outside one every status row is a secondary line.
    ///
    /// 🌳️ The staged form is a `TreeDataSection` on React's side, labelled with the expanded
    /// action's own name and collapsible from its own header (`buildActionCategoryTree`,
    /// `🛠️ShellHelpers/🟦️.tsx:4155`). It used to project as a `UiNode::Section`, and a `Section`
    /// CONTAINER registers no hit (`retained_hit_registration`), so React's
    /// `action.category.<id>.form` header had no pressable twin here at all
    /// (`📓️w13b-actions-search-pane-bodies.md` §6 gap 5).
    ///
    /// ⚡️ React renders that section's own `actions` — Execute and Reset — in the form header's
    /// action row; a `TreeSection` carries no action row on this renderer, so the pair follows
    /// the form as its own band, keeping React's two ids and their order.
    pub(crate) fn build_window_actions_ui(&self, window_id: &str) -> Option<UiNode> {
        let session = self.docs.session.as_ref()?;
        let is_de = self.prefs.locale_id == "de";
        let terminology = self.prefs.active_terminology();
        let locale = self.prefs.active_locale();
        let mut children: Vec<UiNode> = Vec::new();
        if let Some(engagement) = self.windows.window_engagements.get(window_id) {
            let session_active = engagement.session_active.unwrap_or(false);
            if session_active {
                if let Some(step) = engagement.status.iter().flatten().find(|row| row.id == "engagement-step") {
                    children.push(UiNode::Text(UiTextNode { presence: UiPresence::default(), value: Label::data(step.text.clone()), emphasize: Some(true), data_attributes: None, menu: None }));
                }
            }
            for control in engagement.control.iter().chain(engagement.controls.iter().flatten()) {
                children.extend(engagement_control_rows(control, is_de));
            }
            for status in engagement.status.iter().flatten().filter(|row| !(session_active && row.id == "engagement-step")) {
                children.push(UiNode::Text(UiTextNode { presence: UiPresence::default(), value: Label::data(status.text.clone()), emphasize: Some(false), data_attributes: None, menu: None }));
            }
            let options: Vec<UiNode> = engagement
                .options
                .iter()
                .flatten()
                .filter_map(|option| {
                    let action = option.action.clone()?;
                    Some(UiNode::Toggle(UiToggleNode {
                        appearance: ui_contract::ToggleAppearance::Button,
                        id: option.id.clone(),
                        icon_id: option.icon_id.clone().unwrap_or(IconName::CircleDot),
                        text: option.label.clone().map(Label::data),
                        on_change: action,
                        presence: UiPresence {
                            selected: option.pressed.unwrap_or(false),
                            state: if option.disabled.unwrap_or(false) { ui_wgpu::wgpu::component::ui::UiState::Disabled } else { ui_wgpu::wgpu::component::ui::UiState::Normal },
                            ..UiPresence::default()
                        },
                        menu: None,
                    }))
                })
                .collect();
            if !options.is_empty() {
                children.push(UiNode::Stack(UiStackNode {
                    direction: "horizontal".into(),
                    gap: None,
                    padding: None,
                    id: Some(ENGAGEMENT_OPTIONS_CONTROL_ID.to_string()),
                    children: options,
                    presence: UiPresence::default(),
                    activate: None,
                    drop_action: None,
                    drop_overlay: None,
                    menu: None,
                }));
            }
        }
        let kind = self.dock.window_kind_of(session, window_id)?;
        let actions: Vec<semio_framework::ActionDefinition> = semio_framework::resolve_window_actions(&session.app, kind).into_iter().filter(|action| action.in_palette).cloned().collect();
        if actions.is_empty() && children.is_empty() {
            return None;
        }
        let gated = !self.windows.actions_enabled_for_window(&session.app, window_id);
        let expanded = self.windows.action_panel_expanded.get(window_id).cloned().filter(|id| actions.iter().any(|action| &action.id == id && action_requires_staged_form(action)));
        let mut categories: Vec<String> = Vec::new();
        for action in &actions {
            let category = action_category_id(action);
            if !categories.contains(&category) {
                categories.push(category);
            }
        }
        let mut sections: Vec<UiTreeSectionNode> = Vec::new();
        for category in &categories {
            let items: Vec<UiTreeItemNode> = actions
                .iter()
                .filter(|action| &action_category_id(action) == category)
                .map(|action| {
                    let label = action.label.resolve(terminology, locale).to_string();
                    let staged = action_requires_staged_form(action);
                    let disabled = gated && !FRAMEWORK_RESERVED_ACTION_IDS.contains(&action.id.as_str());
                    let open = expanded.as_deref() == Some(action.id.as_str());
                    let dispatch = if staged {
                        ActionDescriptor { controller_id: "framework".into(), action: "setActionExpanded".into(), args: crate::action_args_json!({ "window": window_id.to_string(), "action": action.id.clone() }) }
                    } else {
                        ActionDescriptor { controller_id: session.app.controller_id.clone(), action: action.id.clone(), args: None }
                    };
                    UiTreeItemNode {
                        id: format!("action.{}", action.id),
                        label: Label::data(if staged { format!("{label}…") } else { label }),
                        icon_id: Some(action.icon_id.clone()),
                        presence: UiPresence { state: if disabled { ui_wgpu::wgpu::component::ui::UiState::Disabled } else { ui_wgpu::wgpu::component::ui::UiState::Normal }, ..UiPresence::default() },
                        default_open: Some(open),
                        action: (!disabled).then_some(dispatch),
                        ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
                    }
                })
                .collect();
            sections.push(UiTreeSectionNode {
                header_toolbar: None,
                id: format!("action.category.{category}"),
                label: Some(Label::data(action_category_label(category, is_de))),
                default_open: Some(true),
                presence: UiPresence::default(),
                items,
                window: None,
            });
        }
        if !sections.is_empty() {
            children.push(UiNode::Tree(UiTreeNode { presentation: Default::default(), sections, presence: UiPresence::default(), drop_action: None, menu: None, interaction_domain: None }));
        }
        if let Some(action) = expanded.as_deref().and_then(|id| actions.iter().find(|action| action.id == id)) {
            let staged = self.commands.staged_map_for(window_id, &action.id);
            let mut items: Vec<UiTreeItemNode> = Vec::new();
            for arg in &action.args {
                if matches!(arg.presentation, Some(semio_framework::ArgPresentation::Hidden)) {
                    continue;
                }
                let value = self.commands.effective_arg_value(window_id, &action.id, arg);
                items.push(staged_action_arg_row(window_id, &action.id, arg, value.as_ref(), &self.input.interaction_selection, terminology, locale));
            }
            let category = action_category_id(action);
            children.push(UiNode::Tree(UiTreeNode {
                presentation: Default::default(),
                sections: vec![UiTreeSectionNode {
                    header_toolbar: None,
                    id: format!("action.category.{category}.form"),
                    label: Some(Label::data(action.label.resolve(terminology, locale).to_string())),
                    default_open: Some(true),
                    presence: UiPresence::default(),
                    items,
                    window: None,
                }],
                presence: UiPresence::default(),
                drop_action: None,
                menu: None,
                interaction_domain: None,
            }));
            let missing = resolved_execute_args(&action.args, &staged).is_none();
            let mut rows: Vec<UiNode> = Vec::new();
            rows.push(UiNode::Button(UiButtonNode {
                id: Some(semio_framework::child_element_id(WINDOW_PANE_CHIP_PARENT, &[window_id, "action", action.id.as_str(), "execute"])),
                icon_id: IconName::Check,
                label: Label::data(shell_chrome_string("common.execute", is_de)),
                action: ActionDescriptor { controller_id: "framework".into(), action: "executeStagedAction".into(), args: crate::action_args_json!({ "window": window_id.to_string(), "action": action.id.clone() }) },
                style: None,
                presence: UiPresence { state: if gated || missing { ui_wgpu::wgpu::component::ui::UiState::Disabled } else { ui_wgpu::wgpu::component::ui::UiState::Normal }, ..UiPresence::default() },
                menu: None,
            }));
            rows.push(UiNode::Button(UiButtonNode {
                id: Some(semio_framework::child_element_id(WINDOW_PANE_CHIP_PARENT, &[window_id, "action", action.id.as_str(), "reset"])),
                icon_id: IconName::Undo,
                label: Label::data(shell_chrome_string("common.reset", is_de)),
                action: ActionDescriptor { controller_id: "framework".into(), action: "resetActionArgs".into(), args: crate::action_args_json!({ "window": window_id.to_string(), "action": action.id.clone() }) },
                style: None,
                presence: UiPresence { state: if gated { ui_wgpu::wgpu::component::ui::UiState::Disabled } else { ui_wgpu::wgpu::component::ui::UiState::Normal }, ..UiPresence::default() },
                menu: None,
            }));
            children.extend(rows);
        }
        Some(UiNode::Stack(UiStackNode { direction: "column".into(), gap: None, padding: None, id: Some("window-action-pane".into()), children, presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, menu: None }))
    }

    /// 🎬️ Republishes every live window instance's Actions and Search pane documents, beside the
    /// Measures overlay's own pass and through the same revision-keyed ingress, so an unchanged body
    /// pays nothing. A window whose body fails to project keeps no document and reports a surface
    /// fault rather than blanking the refresh.
    pub(crate) fn refresh_window_action_panes(&mut self, windows: &[String], faults: &mut Vec<(String, String, crate::program_bridge::ProgramFault)>) -> Result<(), String> {
        let stale: Vec<String> = self.docs.window_actions_documents.keys().chain(self.docs.window_search_documents.keys()).filter(|window_id| !windows.contains(window_id)).cloned().collect();
        for window_id in stale.into_iter().chain(windows.iter().cloned()) {
            let previous = self.docs.window_actions_documents.remove(&window_id);
            self.docs.retire_one_surface_document(previous)?;
            let previous = self.docs.window_search_documents.remove(&window_id);
            self.docs.retire_one_surface_document(previous)?;
            if !windows.contains(&window_id) {
                continue;
            }
            let surface = window_actions_surface_id(&window_id);
            match self.build_window_actions_ui(&window_id).map(|node| panel_ui_scroll_records(&surface, &node)).transpose() {
                Ok(Some(records)) => match self.windows.publish_surface_records(&surface, records) {
                    Ok(document) => {
                        self.docs.window_actions_documents.insert(window_id.clone(), document);
                    }
                    Err(error) => faults.push((surface.clone(), semio_framework::UiRefreshSection::Engagements.body_key().to_string(), error.into())),
                },
                Ok(None) => {}
                Err(error) => faults.push((surface.clone(), semio_framework::UiRefreshSection::Engagements.body_key().to_string(), error.into())),
            }
            let surface = window_search_surface_id(&window_id);
            match self.commands.build_window_search_ui(&window_id, &self.prefs, &self.windows).map(|node| panel_ui_scroll_records(&surface, &node)).transpose() {
                Ok(Some(records)) => match self.windows.publish_surface_records(&surface, records) {
                    Ok(document) => {
                        self.docs.window_search_documents.insert(window_id.clone(), document);
                    }
                    Err(error) => faults.push((surface.clone(), WINDOW_SEARCH_BODY_KEY.to_string(), error.into())),
                },
                Ok(None) => {}
                Err(error) => faults.push((surface.clone(), WINDOW_SEARCH_BODY_KEY.to_string(), error.into())),
            }
        }
        Ok(())
    }

    /// 🎭️ Republishes the Actions and Search pane documents after the shell's OWN pane state moved — the expanded action, a
    /// staged argument, a reset, the possibles chevron — with no guest round trip and no refresh pass. Those presses used
    /// to change the state and nothing else: a refresh pass republishes the panes, but nothing owed one, so an
    /// arg-carrying row ("Move…") toggled its expansion and its staged form — the argument fields, Execute, Reset — never
    /// reached the canvas or the mirror (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING, live fault F17). The ingress is
    /// revision-keyed, so an unchanged body costs nothing.
    pub(crate) fn republish_window_action_panes(&mut self) -> Result<(), String> {
        let windows: Vec<String> = self.dock.dock.window_instances().into_iter().map(|(window_id, _)| window_id).collect();
        let mut faults = Vec::new();
        self.refresh_window_action_panes(&windows, &mut faults)?;
        let visited: Vec<String> = windows.iter().flat_map(|window_id| [window_actions_surface_id(window_id), window_search_surface_id(window_id)]).collect();
        self.prefs.settle_surface_faults(faults, &visited, &mut self.chrome, &mut self.plugins);
        self.docs.drain_retained_document_arenas();
        Ok(())
    }

}

/// 🧭️ Parses `dock.tab.{path}.{window}.{action}` into `(path, window_id)`.
pub(crate) fn parse_dock_tab_action_id<'a>(control_id: &'a str, action: &str) -> Option<(Vec<usize>, &'a str)> {
    let rest = control_id.strip_prefix("dock.tab.")?;
    let rest = rest.strip_suffix(&format!(".{action}"))?;
    let (path_str_value, window_id) = rest.split_once('.')?;
    Some((parse_path(path_str_value), window_id))
}

/// 📐️ The Actions pane's box inside a window body — React's `top-left` `anchorPositionStyle` at
/// the pane inset, [`WINDOW_PANE_BODY_WIDTH_PX`] wide, growing DOWN from under its own chip row.
pub(crate) fn window_actions_rect(theme: &Theme, body: Rect, height: f32) -> Rect {
    let top = body.y + theme.panel_inset + theme.control_height + theme.gap_standard;
    Rect::new(body.x + theme.panel_inset, top, WINDOW_PANE_BODY_WIDTH_PX.min((body.w - theme.panel_inset * 2.0).max(0.0)), height.min((body.y + body.h - theme.panel_inset - top).max(0.0)))
}

/// 📐️ The Search pane's box — React's `top-middle` anchor: centred on the body's own width, the
/// same 300 px, under its own chip row.
pub(crate) fn window_search_rect(theme: &Theme, body: Rect, height: f32) -> Rect {
    let width = WINDOW_PANE_BODY_WIDTH_PX.min((body.w - theme.panel_inset * 2.0).max(0.0));
    let top = body.y + theme.panel_inset + theme.control_height + theme.gap_standard;
    Rect::new(body.x + (body.w - width) * 0.5, top, width, height.min((body.y + body.h - theme.panel_inset - top).max(0.0)))
}

//#endregion 🎬️WindowActionsAndSearchPanes
//#region ⌨️WindowScope
/// 🪟️ Whether an active-window change is a USER ACTIVATION the guest's command history must carry —
/// the guard behind React's `handleActiveWindowChange`, which notes `shell.windowActivate` only for a
/// changed, non-empty value. A change that crosses a session INSTANCE is a MOUNT, not an activation:
/// the successor's landing window is picked by the switch itself, so the witness is re-seeded in
/// silence and a role switch journals no activation (React's own `role-viewer` journals none either).
pub(crate) fn window_activation_is_user_v1(noted: Option<&(u32, String)>, instance_id: u32, active_window_id: &str) -> bool {
    !active_window_id.is_empty() && noted.is_some_and(|(noted_instance, noted_window)| *noted_instance == instance_id && noted_window != active_window_id)
}

//#endregion ⌨️WindowScope
impl<'ctx> WindowActionsCtx<'ctx> {
    //#region ShellCommandHistory

    /// 🪟️ Arms one `shell.windowActivate` history note for a REAL activation — the wgpu twin of
    /// React's `handleActiveWindowChange` (`🏛️ShellHost/🟦️.tsx`: `if (value) noteShellCommand(
    /// "shell.windowActivate", …)`), which every window activation in that shell crosses.
    ///
    /// 🩸️ Without it the wgpu shell's command history never carried an activation at all, so `mod+z`
    /// popped whatever OTHER command happened to sit on the guest's stack: React's own `chord-undo`
    /// journals `undo` AND the guest's replay of `shell.windowActivate` (origin `guest`, refused
    /// `undeclared-action`), which is a command this shell had never put there
    /// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, packet W10b, `🗑️generated/parity-run-2/steps.json`).
    ///
    /// Armed rather than dispatched, because every activation site is a chrome-walk or pointer arm
    /// that must not own a guest round trip; `drain_deferred_actions` takes it on the same lane every
    /// other deferred action crosses, which is also the lane the chrome ledger taps.
    pub(crate) fn arm_window_activation_note(&mut self) {
        let Some(instance_id) = self.docs.session.as_ref().map(|session| session.instance_id) else {
            return;
        };
        let Some(active) = self.windows.active_window_id.clone().filter(|id| !id.is_empty()) else {
            return;
        };
        let owed = window_activation_is_user_v1(self.windows.noted_active_window.as_ref(), instance_id, &active);
        self.windows.noted_active_window = Some((instance_id, active.clone()));
        if !owed {
            return;
        }
        let Some(controller_id) = self.docs.shell_command_controller_id(&self.plugins) else {
            return;
        };
        let label = shell_chrome_string("command.activateWindow", self.prefs.locale_id == "de");
        self.settle.deferred_actions.push(note_shell_command_action(&controller_id, "shell.windowActivate", label, Some(serde_json::json!({ "windowId": active }))));
    }

    /// 🫥️ Takes a pending host window blur ([`note_host_window_blur`]): every World3d surface hears a `Blur` cancel (a live
    /// gumball that streamed or an open paint stroke aborts with zero trace), and `hostEvent{windowId, kind: blur}` is armed for
    /// the pane holding the activation, so its program ends an open gesture there. `true` when one was pending.
    pub(crate) fn arm_host_window_blur(&mut self) -> bool {
        if !std::mem::take(&mut *HOST_WINDOW_BLUR_PENDING.borrow_mut()) {
            return false;
        }
        for state in self.surfaces.world3d_states.values_mut() {
            if enqueue_world3d_events(&mut state.authority, [WorldInteractionIntent::cancel(infinite_world::world::WorldCancelReason::Blur)]).is_err() {
                debug_log("[TRACE] wgpu-shell world3d blur cancel refused: the intent queue is full");
            }
        }
        let active = self.windows.active_window_id.clone().filter(|id| !id.is_empty());
        if let (Some(active), Some(_), Some(controller_id)) = (active, self.docs.session.as_ref(), self.docs.shell_command_controller_id(&self.plugins)) {
            self.settle.deferred_actions.push(window_host_event_action(&controller_id, &active, semio_framework::HOST_EVENT_KIND_BLUR));
        }
        true
    }
    //#endregion ShellCommandHistory
}

//#endregion ActionPanelAndUtilities

//#region ActionPanelAndUtilities
//#region ⌨️WindowScope
/// 🪟️ A live window instance of the active mode, with the kind whose declaration owns its verbs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WindowScopeInstanceV1 {
    pub id: String,
    pub window_kind_id: String,
}

/// ⌨️ Where an app-wide chord lands — twin of React's `WindowScopeTargetKindV1`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum WindowScopeTargetKindV1 {
    Focused,
    Owner,
    Unowned,
}

/// 🚦️ The hint the user SEES when a chord's verb is owned by no window of the active mode — twin of
/// `KEYBINDING_UNOWNED_LABEL`. No default language: `locale` picks, and only an unknown locale falls
/// back to English, which is the one case where a choice has not been made.
pub(crate) const KEYBINDING_UNOWNED_LABEL_EN: &str = "not available in this mode";

pub(crate) const KEYBINDING_UNOWNED_LABEL_DE: &str = "in diesem Modus nicht verfügbar";

pub(crate) const KEYBINDING_UNOWNED_CODE: &str = "shell.window-scope.unowned-chord";
//#endregion ⌨️WindowScope
//#endregion ActionPanelAndUtilities
