//! 🧰️ Window actions, search panes, window scope, shortcuts and the command registry.
//!
//! Window actions and search panes, window scope resolution, chord helpers, the shell shortcut table, staged
//! arguments, the OS command registry and panel, and fuzzy ranking.
//!
//! One chunk of the former `shell` monolith (ticket CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT); [`ShellState`] and the
//! shared imports stay in the parent module and are reached through `use super::*;`.

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

/// 🔎️ The reserved body key of the per-window SEARCH pane. React mounts it as a second `Pane` fed by
/// the SAME `WindowEngagement` (`windowEngagementToSearchSpec`, `🛠️ShellHelpers/🟦️.tsx:1826`), so it
/// has no `UiRefreshSection` of its own and takes the engagements key with its own suffix.
pub(crate) const WINDOW_SEARCH_BODY_KEY: &str = "framework.section.engagements.search";

/// 🔀️ Reserved retained surface suffix for the World Projection pane's Tree body.
pub(crate) const WINDOW_PROJECTION_BODY_KEY: &str = "framework.worldOrbit.projection.body";

/// 🔎️ The retained engine surface one window instance's SEARCH pane body paints into.
pub(crate) fn window_search_surface_id(window_id: &str) -> String {
    format!("{window_id}/{WINDOW_SEARCH_BODY_KEY}")
}

/// 🔀️ The retained Tree surface owned by one World window instance's Projection pane.
pub(crate) fn window_projection_surface_id(window_id: &str) -> String {
    format!("{window_id}/{WINDOW_PROJECTION_BODY_KEY}")
}

/// 📐️ React's `PANE_DEFAULT_SIZE` (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:9912`) — every `Pane` that
/// declares no `size` opens 300 logical px wide, which both these bodies do.
pub(crate) const WINDOW_PANE_BODY_WIDTH_PX: f32 = 300.0;

/// 🆔️ React's id for a window-search input whose engagement names none, or names an internal chrome
/// control (`id={… ? UI_WINDOW_SEARCH.action : input.id}`, `🖱️ui/🎯️targets/⚛️react/🟦️.tsx:10868`).
const WINDOW_SEARCH_ACTION_CONTROL_ID: &str = "ui.windowSearch.action";

/// 🆔️ React's id for the search pane's possibles chevron (`Action id={UI_WINDOW_SEARCH.suggestions}`,
/// `🖱️ui/🎯️targets/⚛️react/🟦️.tsx:10938`).
const WINDOW_SEARCH_SUGGESTIONS_CONTROL_ID: &str = "ui.windowSearch.suggestions";

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

/// 🔎️ React's `searchPossibleRankScore` (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:10269`) — the window search
/// line's OWN ranker, higher-is-better, `-1` for no match. Deliberately NOT `rank_fuzzy_items`: that
/// is the ⌘️K palette's ranker (`//#region 🔎️FuzzyRankingParity`), and React's window search never
/// calls it, so borrowing it here would order the two shells' suggestion lists differently.
fn search_possible_rank_score(query: &str, possible: &ui_wgpu::wgpu::WindowEngagementPossible) -> i32 {
    let needle = normalize_engagement_action_text(query).to_lowercase();
    if needle.is_empty() {
        return -1;
    }
    let label = normalize_engagement_action_text(&possible.label).to_lowercase();
    let detail = possible.detail.clone().unwrap_or_default().to_lowercase();
    let id = possible.id.to_lowercase();
    if label.starts_with(&needle) {
        return 3000 - label.chars().count() as i32;
    }
    if detail.starts_with(&needle) {
        return 2000 - detail.chars().count() as i32;
    }
    if id.starts_with(&needle) {
        return 1000 - id.chars().count() as i32;
    }
    if format!("{label} {detail} {id}").contains(&needle) {
        return 500;
    }
    -1
}

/// ⌨️ React's `normalizeEngagementActionText`: separators dropped, tokens PascalCased, a decimal point
/// INSIDE a number preserved (`3.5` stays `3.5`). Display and matching only — the typed line itself
/// reaches the program verbatim.
pub(crate) fn normalize_engagement_action_text(text: &str) -> String {
    let mut words: Vec<String> = Vec::new();
    let mut word = String::new();
    let chars: Vec<char> = text.chars().collect();
    for (index, ch) in chars.iter().copied().enumerate() {
        let decimal_inside_number = ch == '.' && index > 0 && chars[index - 1].is_ascii_digit() && chars.get(index + 1).is_some_and(char::is_ascii_digit);
        if ch.is_alphanumeric() || decimal_inside_number {
            word.push(ch);
            continue;
        }
        if !word.is_empty() {
            words.push(core::mem::take(&mut word));
        }
    }
    if !word.is_empty() {
        words.push(word);
    }
    words
        .into_iter()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect()
}

/// 🔎️ React's `filterSearchPossibles`: an empty query keeps the host's own order, otherwise the rows
/// that score at all, best first.
pub(crate) fn filter_search_possibles<'a>(query: &str, possibles: &'a [ui_wgpu::wgpu::WindowEngagementPossible]) -> Vec<&'a ui_wgpu::wgpu::WindowEngagementPossible> {
    if normalize_engagement_action_text(query).is_empty() {
        return possibles.iter().collect();
    }
    let mut scored: Vec<(i32, usize, &ui_wgpu::wgpu::WindowEngagementPossible)> = possibles.iter().enumerate().map(|(index, possible)| (search_possible_rank_score(query, possible), index, possible)).filter(|(score, _, _)| *score >= 0).collect();
    scored.sort_by(|left, right| right.0.cmp(&left.0).then(left.1.cmp(&right.1)));
    scored.into_iter().map(|(_, _, possible)| possible).collect()
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

impl ShellState {
    pub(crate) fn chrome_accessibility_checked(&self, id: &str, kind: &HitKind) -> Option<bool> {
        self.widget_maps
            .toggle_metas
            .get(id)
            .map(|(pressed, _)| *pressed)
            .or_else(|| (kind == &HitKind::Toggle).then(|| self.panel_tab_anchor(id).is_some_and(|anchor| self.anchor_open(anchor) && self.anchor_state(anchor).path.first().is_some_and(|path| path == id))))
    }

    pub(crate) fn chrome_accessibility_pressed(&self, id: &str, kind: &HitKind) -> Option<bool> {
        if !matches!(kind, HitKind::PanelTab | HitKind::Toggle) {
            return None;
        }
        let anchor = self.panel_tab_anchor(id);
        (kind == &HitKind::PanelTab || anchor.is_some()).then(|| anchor.is_some_and(|anchor| self.anchor_state(anchor).path.iter().any(|path| path == id)))
    }

    pub(crate) fn chrome_accessibility_selected(&self, id: &str, _kind: &HitKind) -> Option<bool> {
        if let Some((palette, index)) = ShellPaletteKind::row_index(id) {
            return Some(match palette {
                ShellPaletteKind::Search => self.search_selected == index,
                ShellPaletteKind::Find => self.find_selected == index,
            });
        }
        None
    }

    /// 🪟️ The window KIND one live instance renders, which is what `resolve_window_actions` scopes an
    /// Actions pane by — `puzzle3d-main-top` and `puzzle3d-main-perspective` are both `puzzle3d-main`.
    fn window_kind_of<'a>(&self, session: &'a ActiveSession, window_id: &str) -> Option<&'a semio_framework::WindowKindDefinition> {
        let kind_id = self.live_window_kind_id(session, window_id).unwrap_or(window_id).to_string();
        session.app.window_kinds.iter().find(|kind| kind.id == kind_id)
    }

    /// 🎬️ Whether this window carries React's `engagement` PROP — `windowEngagementToSpec`'s own
    /// `hasContent`: at least one option, a `control`, a `controls` row or a status line
    /// (`🛠️ShellHelpers/🟦️.tsx:1852`). An engagement carrying only an INPUT is a search spec and no
    /// engagement spec at all, which is why the two panes mount independently.
    pub(crate) fn window_has_engagement_spec(&self, window_id: &str) -> bool {
        self.window_engagements.get(window_id).is_some_and(|engagement| {
            engagement.options.as_ref().is_some_and(|options| !options.is_empty())
                || engagement.control.is_some()
                || engagement.controls.as_ref().is_some_and(|controls| !controls.is_empty())
                || engagement.status.as_ref().is_some_and(|status| !status.is_empty())
        })
    }

    /// 🧰️ Whether this window carries React's `actionPane` PROP — `windowActionPaneNode` answers
    /// `undefined` for a kind whose `resolveWindowActions(...).filter(inPalette)` is empty, "so the
    /// rail chip never renders" (`🛠️ShellHelpers/🟦️.tsx:4260`).
    pub(crate) fn window_has_action_pane(&self, window_id: &str) -> bool {
        let Some(session) = self.session.as_ref() else { return false };
        let Some(kind) = self.window_kind_of(session, window_id) else { return false };
        semio_framework::resolve_window_actions(&session.app, kind).into_iter().any(|action| action.in_palette)
    }

    /// 🔎️ Whether this window carries React's `search` PROP — `windowEngagementToSearchSpec`'s own
    /// `hasContent`: a typed input OR at least one autocomplete possible (`🛠️ShellHelpers/🟦️.tsx:1874`).
    /// The BODY still needs the input (React's `Search` returns `null` without one); the PANE mounts
    /// either way, which is why this is a separate question from [`Self::build_window_search_ui`].
    pub(crate) fn window_has_search_spec(&self, window_id: &str) -> bool {
        self.window_engagements.get(window_id).is_some_and(|engagement| engagement.input.is_some() || engagement.possible_engagements.as_ref().is_some_and(|possibles| !possibles.is_empty()))
    }

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
        let session = self.session.as_ref()?;
        let is_de = self.locale_id == "de";
        let terminology = self.active_terminology();
        let locale = self.active_locale();
        let mut children: Vec<UiNode> = Vec::new();
        if let Some(engagement) = self.window_engagements.get(window_id) {
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
        let kind = self.window_kind_of(session, window_id)?;
        let actions: Vec<semio_framework::ActionDefinition> = semio_framework::resolve_window_actions(&session.app, kind).into_iter().filter(|action| action.in_palette).cloned().collect();
        if actions.is_empty() && children.is_empty() {
            return None;
        }
        let gated = !self.actions_enabled_for_window(&session.app, window_id);
        let expanded = self.action_panel_expanded.get(window_id).cloned().filter(|id| actions.iter().any(|action| &action.id == id && action_requires_staged_form(action)));
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
            let staged = self.staged_map_for(window_id, &action.id);
            let mut items: Vec<UiTreeItemNode> = Vec::new();
            for arg in &action.args {
                if matches!(arg.presentation, Some(semio_framework::ArgPresentation::Hidden)) {
                    continue;
                }
                let value = self.effective_arg_value(window_id, &action.id, arg);
                items.push(staged_action_arg_row(window_id, &action.id, arg, value.as_ref(), &self.interaction_selection, terminology, locale));
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
            let missing = Self::resolved_execute_args(&action.args, &staged).is_none();
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

    /// 🔎️ ONE window's Search pane body — React's top-middle `Pane` content: the typed action line,
    /// the possibles chevron, and (while the chevron is open) the ranked suggestion rows
    /// (`Search`, `🖱️ui/🎯️targets/⚛️react/🟦️.tsx:10758`). `None` when the engagement declares no
    /// input, which is React's own `if (!hasInput) return null`.
    ///
    /// ✍️ React's line binds FOUR moments at once and this one binds the same four (the retained
    /// producers landed with this packet, `🖱️ui/🎯️targets/🧊️/⚡️events`'s `search_line_action`):
    ///
    /// * `onChange` on every keystroke — `commit` is deliberately UNSET so the node fires
    ///   `Trigger::Change` per keystroke, which is what feeds the guest's autocomplete. The line
    ///   used to commit on blur and dispatch `on_submit` there, i.e. the program saw the verb and
    ///   never the typing (`📓️w13b-actions-search-pane-bodies.md` §6 gap 4).
    /// * `onSubmit` on Enter, `onAbort` on Escape — the guest's own descriptors, unaltered.
    /// * `onRepeatLast` on Space over an EMPTY line, bound only OUTSIDE a live session because
    ///   that is React's own branch: `applySearchSpaceAction` routes an empty line to
    ///   `onRepeatLast` when idle and to `onSubmit` during a session
    ///   (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:10512`), and the retained line falls through to the
    ///   submit it did bind when no repeat is offered.
    pub(crate) fn build_window_search_ui(&self, window_id: &str) -> Option<UiNode> {
        let engagement = self.window_engagements.get(window_id)?;
        let input = engagement.input.as_ref()?;
        let is_de = self.locale_id == "de";
        let draft = self.engagement_inputs.get(window_id).cloned().or_else(|| input.value.clone()).unwrap_or_default();
        let placeholder = input.placeholder.clone().unwrap_or_else(|| shell_chrome_string(if engagement.session_active.unwrap_or(false) { "windowSearch.actionActive" } else { "windowSearch.action" }, is_de).to_string());
        let control_id = input.id.clone().filter(|id| !id.is_empty() && id != "search-input").unwrap_or_else(|| WINDOW_SEARCH_ACTION_CONTROL_ID.to_string());
        let session_active = engagement.session_active.unwrap_or(false);
        let mut children: Vec<UiNode> = vec![UiNode::Input(UiInputNode {
            id: control_id,
            input_kind: "text".into(),
            value: draft.clone(),
            placeholder: Some(Label::data(placeholder)),
            accessibility_label: None,
            commit: None,
            min: None,
            max: None,
            step: None,
            accept: None,
            precision: None,
            snaps: Vec::new(),
            on_change: input.on_change.clone().unwrap_or_else(|| ActionDescriptor { controller_id: "framework".into(), action: "setEngagementInput".into(), args: crate::action_args_json!({ "window": window_id.to_string() }) }),
            on_submit: input.on_submit.clone(),
            on_abort: input.on_abort.clone(),
            on_repeat_last: (!session_active).then(|| input.on_repeat_last.clone()).flatten(),
            presence: UiPresence { state: if input.disabled.unwrap_or(false) { ui_wgpu::wgpu::component::ui::UiState::Disabled } else { ui_wgpu::wgpu::component::ui::UiState::Normal }, ..UiPresence::default() },
            menu: None,
            ..Default::default()
        })];
        let possibles: &[ui_wgpu::wgpu::WindowEngagementPossible] = engagement.possible_engagements.as_deref().unwrap_or(&[]);
        if !possibles.is_empty() {
            let open = self.search_possibles_open.get(window_id).copied().unwrap_or(false);
            children.push(UiNode::Toggle(UiToggleNode {
                appearance: ui_contract::ToggleAppearance::Button,
                id: WINDOW_SEARCH_SUGGESTIONS_CONTROL_ID.into(),
                icon_id: if open { IconName::ChevronDown } else { IconName::ChevronRight },
                text: Some(Label::data(shell_chrome_string("windowSearch.suggestions", is_de))),
                on_change: ActionDescriptor { controller_id: "framework".into(), action: "toggleSearchPossibles".into(), args: crate::action_args_json!({ "window": window_id.to_string() }) },
                presence: UiPresence { selected: open, ..UiPresence::default() },
                menu: None,
            }));
            if open {
                let matches = filter_search_possibles(&draft, possibles);
                if matches.is_empty() {
                    children.push(UiNode::Text(UiTextNode { presence: UiPresence::default(), value: Label::data(shell_chrome_string("windowSearch.noMatches", is_de)), emphasize: Some(false), data_attributes: None, menu: None }));
                }
                for possible in matches {
                    let Some(action) = possible.action.clone() else { continue };
                    children.push(UiNode::Button(UiButtonNode {
                        id: Some(possible.id.clone()),
                        icon_id: IconName::CircleDot,
                        label: Label::data(match possible.detail.as_deref() {
                            Some(detail) if !detail.is_empty() => format!("{} — {detail}", normalize_engagement_action_text(&possible.label)),
                            _ => normalize_engagement_action_text(&possible.label),
                        }),
                        action,
                        style: None,
                        presence: UiPresence::default(),
                        menu: None,
                    }));
                }
            }
        }
        Some(UiNode::Stack(UiStackNode { direction: "column".into(), gap: None, padding: None, id: Some("search".into()), children, presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, menu: None }))
    }

    /// 🎬️ Republishes every live window instance's Actions and Search pane documents, beside the
    /// Measures overlay's own pass and through the same revision-keyed ingress, so an unchanged body
    /// pays nothing. A window whose body fails to project keeps no document and reports a surface
    /// fault rather than blanking the refresh.
    pub(crate) fn refresh_window_action_panes(&mut self, windows: &[String], faults: &mut Vec<(String, String, crate::program_bridge::ProgramFault)>) -> Result<(), String> {
        let stale: Vec<String> = self.window_actions_documents.keys().chain(self.window_search_documents.keys()).filter(|window_id| !windows.contains(window_id)).cloned().collect();
        for window_id in stale.into_iter().chain(windows.iter().cloned()) {
            let previous = self.window_actions_documents.remove(&window_id);
            self.retire_one_surface_document(previous)?;
            let previous = self.window_search_documents.remove(&window_id);
            self.retire_one_surface_document(previous)?;
            if !windows.contains(&window_id) {
                continue;
            }
            let surface = window_actions_surface_id(&window_id);
            match self.build_window_actions_ui(&window_id).map(|node| panel_ui_scroll_records(&surface, &node)).transpose() {
                Ok(Some(records)) => match self.publish_surface_records(&surface, records) {
                    Ok(document) => {
                        self.window_actions_documents.insert(window_id.clone(), document);
                    }
                    Err(error) => faults.push((surface.clone(), semio_framework::UiRefreshSection::Engagements.body_key().to_string(), error.into())),
                },
                Ok(None) => {}
                Err(error) => faults.push((surface.clone(), semio_framework::UiRefreshSection::Engagements.body_key().to_string(), error.into())),
            }
            let surface = window_search_surface_id(&window_id);
            match self.build_window_search_ui(&window_id).map(|node| panel_ui_scroll_records(&surface, &node)).transpose() {
                Ok(Some(records)) => match self.publish_surface_records(&surface, records) {
                    Ok(document) => {
                        self.window_search_documents.insert(window_id.clone(), document);
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
        let windows: Vec<String> = self.dock.window_instances().into_iter().map(|(window_id, _)| window_id).collect();
        let mut faults = Vec::new();
        self.refresh_window_action_panes(&windows, &mut faults)?;
        let visited: Vec<String> = windows.iter().flat_map(|window_id| [window_actions_surface_id(window_id), window_search_surface_id(window_id)]).collect();
        self.settle_surface_faults(faults, &visited);
        self.drain_retained_document_arenas();
        Ok(())
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

    /// 🖌️ One paint opportunity of one window's Actions OR Search pane body: the panel backdrop once,
    /// then its retained document through the same stepped paint the Measures overlay takes, then its
    /// hit registry above the window body's. `true` once the body is off the walk's critical path —
    /// painted, absent, folded, or faulted after its opportunity ceiling.
    ///
    /// ⚖️ Each body reads its own fold, matching the two independent React pane controls.
    #[allow(clippy::too_many_arguments, reason = "the chrome walk's own paint context, forwarded unchanged")]
    pub(crate) fn paint_window_pane_body_step(
        &mut self,
        search: bool,
        cursor: &mut ShellChromeChildCursor,
        draw: &mut DrawList,
        overlay: &mut Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<ActionDescriptor>,
        theme: &Theme,
        window_id: &str,
        window_rect: Rect,
        world_resources: &mut infinite_world::world::World3dBuildContext,
        cx:&mut semio_framework_job::StepContext<'_>,
    ) -> bool {
        let folded = if search { self.window_search_folded(window_id) } else { self.window_actions_folded(window_id) };
        if folded || !surface_fits_overlay(theme, window_rect) {
            return true;
        }
        let documents = if search { &mut self.window_search_documents } else { &mut self.window_actions_documents };
        let Some(document) = documents.remove(window_id) else { return true };
        let surface = if search { window_search_surface_id(window_id) } else { window_actions_surface_id(window_id) };
        let height = crate::interpreter::retained_content_height(&surface).unwrap_or(theme.control_height * 2.0);
        let rect = if search { Self::window_search_rect(theme, window_rect, height) } else { Self::window_actions_rect(theme, window_rect, height) };
        if !cursor.flag {
            draw.push_solid([rect.x, rect.y, rect.w, rect.h], theme.panel);
            cursor.flag = true;
        }
        let controller = self.document_controller_id();
        let complete = {
            let scroll_offsets = &mut self.scroll_offsets;
            let collapsed_sections = &mut self.collapsed_sections;
            let open_selects = &mut self.open_selects;
            let widget_maps = &mut self.widget_maps_staging;
            let mut hosts = crate::scenes::SceneEngineHosts { chrome_labels: scene_chrome_labels(self.locale_id == "de"), world3d_states: &mut self.world3d_states, world_resources, window_id: surface.as_str() };
            let viewport_height_for_widgets = draw.screen_height();
            let mut ctx = framework_widget_context(draw, overlay.as_deref_mut(), atlas, Some(icons), input, theme, scroll_offsets, collapsed_sections, open_selects, Some(widget_maps), viewport_height_for_widgets);
            ctx.pick_clip = Some(rect);
            render_ui_document_step(&mut cursor.document, &document, rect, &mut ctx, surface.as_str(), controller.as_str(), self.chrome_build.driver.drag, &mut hosts, cx)
        };
        if search {
            self.window_search_documents.insert(window_id.to_string(), document);
        } else {
            self.window_actions_documents.insert(window_id.to_string(), document);
        }
        if complete {
            self.note_retained_body_painted(&surface);
        } else {
            if document_opportunity_remains(cursor, SHELL_WINDOW_PAINT_OPPORTUNITIES) {
                return false;
            }
            self.record_document_paint_fault(&surface);
        }
        push_chrome_group_border(draw, rect, theme);
        self.register_retained_body_hits(&surface, rect, input);
        true
    }
}

//#endregion 🎬️WindowActionsAndSearchPanes

/// ⌨️ The platform every `PlatformKeybinding.platform` filter is answered against — ONE door, the same
/// `⌨️HostPlatform` region (`🧊️renderer/🦀️.rs`) `format_keybinding_shortcut` reads through
/// [`crate::host_platform_uses_meta`]. Natively the door's cell IS `cfg!(target_os = …)`; in the
/// browser the page resolves `userAgentData.platform`/`navigator.platform` and publishes it with the
/// boot message.
///
/// 🩸️ What this replaces: the `wasm32` arm re-derived the platform from `web_sys::window()`. The wgpu
/// shell runs inside the frame Worker, whose realm owns NO `window`, so the read was always `None`,
/// `unwrap_or_default()` always `""`, and the answer always `Linux` — on every machine. A macOS
/// browser user therefore matched and was shown the `Platform::Windows`/`Linux` chord of every
/// platform-scoped keybinding (`os.toggleFullscreen`'s `f11` instead of `control+meta+f`, and the same
/// for every app-declared Mac-scoped binding), while `format_keybinding_shortcut` — fixed by W7a at
/// the same file — painted `⌘️`. Two functions, one question, two answers
/// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️audit-w14-transport-residual.md` §3).
pub(crate) fn command_host_platform() -> semio_framework::manifest::Platform {
    crate::host_platform()
}

//#region ⌨️WindowScope
/// 🪟️ One tab stack of a seeded mode layout, in layout order — the Rust twin of React's
/// `WindowScopeStackV1` (`🏛️ShellHost/⌨️window-scope/🟦️.ts`), driven by the same fixture
/// (`🏛️ShellHost/🧫️fixtures/⌨️window-scope/🔣️.json`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WindowScopeStackV1 {
    pub window_ids: Vec<String>,
    pub active_window_id: Option<String>,
}

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

/// 🪟️ Flattens a dock tree into its tab stacks, in layout order — twin of `modeLayoutStacksV1`.
pub(crate) fn mode_layout_stacks_v1(node: &crate::dock::DockNode) -> Vec<WindowScopeStackV1> {
    match node {
        crate::dock::DockNode::Stack { windows, active } => {
            let window_ids: Vec<String> = windows.iter().map(|tab| tab.window_id.clone()).collect();
            let active_window_id = if active.is_empty() { window_ids.first().cloned() } else { Some(active.clone()) };
            vec![WindowScopeStackV1 { window_ids, active_window_id }]
        }
        crate::dock::DockNode::Row(children) | crate::dock::DockNode::Column(children) => children.iter().flat_map(|(child, _)| mode_layout_stacks_v1(child)).collect(),
    }
}

/// 🪟️ The window a freshly seeded mode layout opens ACTIVE, or `None` when the seed must be left
/// alone — twin of `dockSeedActiveWindowIdV1`. A mode with windows always has exactly one active
/// window, so every window-scoped verb, chord badge and Actions rail has an owner before the user's
/// first click.
pub(crate) fn dock_seed_active_window_id_v1(stacks: &[WindowScopeStackV1], active_window_id: Option<&str>) -> Option<String> {
    let all: Vec<&String> = stacks.iter().flat_map(|stack| stack.window_ids.iter()).collect();
    if all.is_empty() {
        return None;
    }
    if let Some(active) = active_window_id {
        if all.iter().any(|id| id.as_str() == active) {
            return None;
        }
    }
    for stack in stacks {
        let candidate = stack.active_window_id.clone().or_else(|| stack.window_ids.first().cloned());
        if let Some(candidate) = candidate {
            if stack.window_ids.contains(&candidate) {
                return Some(candidate);
            }
        }
    }
    all.first().map(|id| (*id).clone())
}

/// ⌨️ Resolves the window an app-wide chord addresses — twin of `resolveKeybindingTargetWindowV1`.
/// `mounted` is the ACTIVE MODE's window instances in layout order, never every declared kind: a verb
/// owned by a window this mode does not mount has no reachable owner and must say so.
pub(crate) fn resolve_keybinding_target_window_v1(app: &AppDefinition, mounted: &[WindowScopeInstanceV1], focused_window_id: Option<&str>, action_id: &str) -> (WindowScopeTargetKindV1, Option<String>) {
    let declares = |window_kind_id: &str| app.window_kinds.iter().any(|kind| kind.id == window_kind_id && semio_framework::window_kind_actions(app, kind).iter().any(|action| action.id == action_id));
    if let Some(focused) = focused_window_id.and_then(|id| mounted.iter().find(|instance| instance.id == id)) {
        if declares(&focused.window_kind_id) {
            return (WindowScopeTargetKindV1::Focused, Some(focused.id.clone()));
        }
    }
    if let Some(owner) = mounted.iter().find(|instance| declares(&instance.window_kind_id)) {
        return (WindowScopeTargetKindV1::Owner, Some(owner.id.clone()));
    }
    (WindowScopeTargetKindV1::Unowned, None)
}

/// 🚦️ The hint the user SEES when a chord's verb is owned by no window of the active mode — twin of
/// `KEYBINDING_UNOWNED_LABEL`. No default language: `locale` picks, and only an unknown locale falls
/// back to English, which is the one case where a choice has not been made.
pub(crate) const KEYBINDING_UNOWNED_LABEL_EN: &str = "not available in this mode";
pub(crate) const KEYBINDING_UNOWNED_LABEL_DE: &str = "in diesem Modus nicht verfügbar";
pub(crate) const KEYBINDING_UNOWNED_CODE: &str = "shell.window-scope.unowned-chord";

/// 🚦️ `⌘⇧G · Add Generation — not available in this mode`, in the reader's language.
pub(crate) fn keybinding_unowned_text_v1(locale: &str, chord: &str, label: &str) -> String {
    let tail = if locale == "de" { KEYBINDING_UNOWNED_LABEL_DE } else { KEYBINDING_UNOWNED_LABEL_EN };
    format!("{chord} · {label} — {tail}")
}

/// 🛡️ Whether a chord carries the platform ACCELERATOR (`mod`/`ctrl`/`meta`) — twin of React's
/// `chordCarriesAcceleratorV1`. The rule the wgpu shell's own [`is_reserved_shell_chord`] already
/// applies: a bare key belongs to whichever surface has focus, an accelerator chord the shell's
/// chrome answers is never an app's to shadow.
pub(crate) fn chord_carries_accelerator_v1(chord: &str) -> bool {
    let lowered = chord.to_ascii_lowercase();
    let segments: Vec<&str> = lowered.split('+').collect();
    segments.split_last().map(|(_, modifiers)| modifiers.iter().any(|segment| matches!(*segment, "mod" | "ctrl" | "meta" | "cmd"))).unwrap_or(false)
}

/// 🛡️ The chords the shell's own chrome keeps out of the app-keybinding loop, after the user's
/// overrides — twin of React's `reservedShellChordsV1`.
pub(crate) fn reserved_shell_chords_v1(shell_table: &[(String, String)], overrides: &[(String, String)]) -> std::collections::BTreeSet<String> {
    let mut reserved = std::collections::BTreeSet::new();
    for (control_id, keys) in shell_table {
        let keys = overrides.iter().find(|(id, _)| id == control_id).map_or(keys.as_str(), |(_, keys)| keys.as_str());
        for chord in keys.split(',').map(|key| key.trim().to_ascii_lowercase()).filter(|key| !key.is_empty()) {
            if chord_carries_accelerator_v1(&chord) {
                reserved.insert(chord);
            }
        }
    }
    reserved
}

/// 🪟️ Whether an active-window change is a USER ACTIVATION the guest's command history must carry —
/// the guard behind React's `handleActiveWindowChange`, which notes `shell.windowActivate` only for a
/// changed, non-empty value. A change that crosses a session INSTANCE is a MOUNT, not an activation:
/// the successor's landing window is picked by the switch itself, so the witness is re-seeded in
/// silence and a role switch journals no activation (React's own `role-viewer` journals none either).
pub(crate) fn window_activation_is_user_v1(noted: Option<&(u32, String)>, instance_id: u32, active_window_id: &str) -> bool {
    !active_window_id.is_empty() && noted.is_some_and(|(noted_instance, noted_window)| *noted_instance == instance_id && noted_window != active_window_id)
}
//#endregion ⌨️WindowScope

/// ⌨️ Whether a key event matches a keybinding chord such as `"mod+shift+z"`, `"ctrl+k"`, or `"escape"`.
/// `"mod"` is the platform accelerator (meta OR ctrl). Declared modifiers must be present and no
/// undeclared accelerator/shift/alt may be held, so `mod+z` never fires for `mod+shift+z`.
pub(crate) fn key_event_matches_chord(action: &ui_wgpu::wgpu::KeyAction, modifiers: &PointerModifiers, chord: &str) -> bool {
    let mut want_mod = false;
    let mut want_shift = false;
    let mut want_alt = false;
    let mut want_ctrl = false;
    let mut want_meta = false;
    let mut key_token = String::new();
    for token in chord.split('+') {
        match token.trim().to_ascii_lowercase().as_str() {
            "" => {}
            "mod" => want_mod = true,
            "shift" => want_shift = true,
            "alt" | "option" => want_alt = true,
            "ctrl" | "control" => want_ctrl = true,
            "cmd" | "meta" | "super" | "win" => want_meta = true,
            other => key_token = other.to_string(),
        }
    }
    if key_token.is_empty() {
        return false;
    }
    if modifiers.shift != want_shift || modifiers.alt != want_alt {
        return false;
    }
    if want_mod {
        if want_ctrl || want_meta || modifiers.ctrl == modifiers.meta {
            return false;
        }
    } else if modifiers.ctrl != want_ctrl || modifiers.meta != want_meta {
        return false;
    }
    match action {
        ui_wgpu::wgpu::KeyAction::Char(c) => c.eq_ignore_ascii_case(&key_token),
        ui_wgpu::wgpu::KeyAction::Enter => key_token == "enter" || key_token == "return",
        ui_wgpu::wgpu::KeyAction::Escape => key_token == "escape" || key_token == "esc",
        ui_wgpu::wgpu::KeyAction::Backspace => key_token == "backspace",
        ui_wgpu::wgpu::KeyAction::Delete => key_token == "delete" || key_token == "del",
        ui_wgpu::wgpu::KeyAction::Tab => key_token == "tab",
        ui_wgpu::wgpu::KeyAction::ArrowLeft => key_token == "arrowleft" || key_token == "left",
        ui_wgpu::wgpu::KeyAction::ArrowRight => key_token == "arrowright" || key_token == "right",
        ui_wgpu::wgpu::KeyAction::ArrowUp => key_token == "arrowup" || key_token == "up",
        ui_wgpu::wgpu::KeyAction::ArrowDown => key_token == "arrowdown" || key_token == "down",
        ui_wgpu::wgpu::KeyAction::Home => key_token == "home",
        ui_wgpu::wgpu::KeyAction::End => key_token == "end",
        ui_wgpu::wgpu::KeyAction::PageUp => key_token == "pageup",
        ui_wgpu::wgpu::KeyAction::PageDown => key_token == "pagedown",
        ui_wgpu::wgpu::KeyAction::Function(number) => key_token == format!("f{number}"),
        ui_wgpu::wgpu::KeyAction::Space(_) => key_token == "space",
    }
}

/// ⌨️ The key token a captured chord writes for one key event — the inverse of
/// [`key_event_matches_chord`]'s key arm, so a chord this renderer captures is one it can match again.
/// `None` for a key no chord can name (a bare modifier never reaches here as a `KeyAction`).
pub(crate) fn keybinding_capture_key_token(action: &ui_wgpu::wgpu::KeyAction) -> Option<String> {
    Some(match action {
        ui_wgpu::wgpu::KeyAction::Char(key) => {
            let key = key.trim().to_ascii_lowercase();
            if key.is_empty() {
                return None;
            }
            key
        }
        ui_wgpu::wgpu::KeyAction::Enter => "enter".into(),
        ui_wgpu::wgpu::KeyAction::Escape => return None,
        ui_wgpu::wgpu::KeyAction::Backspace => "backspace".into(),
        ui_wgpu::wgpu::KeyAction::Delete => "delete".into(),
        ui_wgpu::wgpu::KeyAction::Tab => "tab".into(),
        ui_wgpu::wgpu::KeyAction::ArrowLeft => "arrowleft".into(),
        ui_wgpu::wgpu::KeyAction::ArrowRight => "arrowright".into(),
        ui_wgpu::wgpu::KeyAction::ArrowUp => "arrowup".into(),
        ui_wgpu::wgpu::KeyAction::ArrowDown => "arrowdown".into(),
        ui_wgpu::wgpu::KeyAction::Home => "home".into(),
        ui_wgpu::wgpu::KeyAction::End => "end".into(),
        ui_wgpu::wgpu::KeyAction::PageUp => "pageup".into(),
        ui_wgpu::wgpu::KeyAction::PageDown => "pagedown".into(),
        ui_wgpu::wgpu::KeyAction::Function(number) => format!("f{number}"),
        ui_wgpu::wgpu::KeyAction::Space(_) => "space".into(),
    })
}

//#region ⌨️ShellShortcutTable
/// ⌨️ The shell's OWN shortcut table, control id → chords, transcribed from React's `SHELL_KEYBINDINGS`
/// (`🔨️modules/🖱️ui/🔨️modules/🕹️control-keybinding-context/🟦️.tsx`). One table drives all three consumers —
/// [`shell_shortcut_for`] (what a key event means), [`is_reserved_shell_chord`] (what an app may not
/// shadow) and the report's shortcut matrix — so a row can never be reserved without being dispatched,
/// or dispatched without being reserved.
///
/// Only the rows whose verb is a SHELL-chrome transition live here. React's `ui.introduction.*`,
/// `ui.dialog.*` rows carry no accelerator (`escape`/`enter`/arrows): they belong to whichever overlay
/// has focus, and this target answers them in `handle_keyboard`'s tour/sync-card/palette arms ahead of
/// this table, exactly as React answers them inside the dialog/introduction components.
///
/// ⏪️ The three `ui.timeTravel.*` rows are React's too: they dispatch the reserved `historyEdit*` verbs, only
/// while the open session offers them, and none of them is Escape — Escape never discards a history edit.
pub(crate) const SHELL_SHORTCUT_ROWS: &[(&str, &str)] = &[
    ("ui.search.toggle", "mod+p"),
    ("ui.find.toggle", "mod+f"),
    ("ui.nav.back", "mod+["),
    ("ui.nav.forward", "mod+]"),
    ("ui.nav.up", "mod+up"),
    ("ui.shell.panelAnchor.topLeft", "ctrl+b,meta+b"),
    ("ui.shell.panelAnchor.topMiddle", "ctrl+m,meta+m"),
    ("ui.shell.panelAnchor.topRight", "ctrl+shift+b,meta+shift+b"),
    ("ui.shell.panelAnchor.rightMiddle", "ctrl+shift+m,meta+shift+m"),
    ("ui.shell.panelAnchor.bottomRight", "ctrl+alt+shift+b,meta+alt+shift+b"),
    ("ui.shell.panelAnchor.bottomMiddle", "ctrl+alt+m,meta+alt+m"),
    ("ui.shell.panelAnchor.bottomLeft", "ctrl+alt+b,meta+alt+b"),
    ("ui.shell.panelAnchor.leftMiddle", "ctrl+alt+shift+m,meta+alt+shift+m"),
    ("ui.window.close", "mod+shift+w"),
    ("ui.window.focus", "mod+shift+enter"),
    ("ui.window.newWindow", "mod+shift+n"),
    ("ui.shell.mode.next", "mod+alt+arrowright"),
    ("ui.shell.mode.previous", "mod+alt+arrowleft"),
    ("playground.navbar.roles.editor", "mod+alt+e"),
    ("playground.navbar.roles.viewer", "mod+alt+v"),
    ("ui.timeTravel.accept", "alt+enter"),
    ("ui.timeTravel.discard", "alt+backspace"),
    ("ui.timeTravel.exit", "alt+shift+backspace"),
];

/// ⌨️ The verb one [`SHELL_SHORTCUT_ROWS`] row asks for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ShellShortcut {
    ToggleSearch,
    ToggleFind,
    NavBack,
    NavForward,
    NavUp,
    TogglePanelAnchor(PanelAnchor),
    CloseWindow,
    FocusWindow,
    NewWindow,
    ModeStep(i32),
    SurfaceRole(semio_framework::manifest::AppRole),
    /// ⏪️ A reserved history-edit verb — answered only while `HistoryPatch.timeTravel` offers it (see `⏪️time-travel`).
    TimeTravel(TimeTravelVerb),
}

impl ShellShortcut {
    /// ⌨️ Whether this verb needs the async dispatch funnel (a guest round-trip or an os command) —
    /// `false` means `handle_keyboard`'s synchronous chrome-state arms answer it.
    pub(crate) fn is_async(self) -> bool {
        matches!(self, ShellShortcut::ModeStep(_) | ShellShortcut::SurfaceRole(_) | ShellShortcut::TimeTravel(_))
    }
}

/// ⌨️ The verb a control id names — the single mapping from React's control ids to this target's chrome.
pub(crate) fn shell_shortcut_for_control_id(control_id: &str) -> Option<ShellShortcut> {
    Some(match control_id {
        "ui.search.toggle" => ShellShortcut::ToggleSearch,
        "ui.find.toggle" => ShellShortcut::ToggleFind,
        "ui.nav.back" => ShellShortcut::NavBack,
        "ui.nav.forward" => ShellShortcut::NavForward,
        "ui.nav.up" => ShellShortcut::NavUp,
        "ui.shell.panelAnchor.topLeft" => ShellShortcut::TogglePanelAnchor(PanelAnchor::TopLeft),
        "ui.shell.panelAnchor.topMiddle" => ShellShortcut::TogglePanelAnchor(PanelAnchor::TopMiddle),
        "ui.shell.panelAnchor.topRight" => ShellShortcut::TogglePanelAnchor(PanelAnchor::TopRight),
        "ui.shell.panelAnchor.rightMiddle" => ShellShortcut::TogglePanelAnchor(PanelAnchor::RightMiddle),
        "ui.shell.panelAnchor.bottomRight" => ShellShortcut::TogglePanelAnchor(PanelAnchor::BottomRight),
        "ui.shell.panelAnchor.bottomMiddle" => ShellShortcut::TogglePanelAnchor(PanelAnchor::BottomMiddle),
        "ui.shell.panelAnchor.bottomLeft" => ShellShortcut::TogglePanelAnchor(PanelAnchor::BottomLeft),
        "ui.shell.panelAnchor.leftMiddle" => ShellShortcut::TogglePanelAnchor(PanelAnchor::LeftMiddle),
        "ui.window.close" => ShellShortcut::CloseWindow,
        "ui.window.focus" => ShellShortcut::FocusWindow,
        "ui.window.newWindow" => ShellShortcut::NewWindow,
        "ui.shell.mode.next" => ShellShortcut::ModeStep(1),
        "ui.shell.mode.previous" => ShellShortcut::ModeStep(-1),
        "playground.navbar.roles.editor" => ShellShortcut::SurfaceRole(semio_framework::manifest::AppRole::Editor),
        "playground.navbar.roles.viewer" => ShellShortcut::SurfaceRole(semio_framework::manifest::AppRole::Viewer),
        other => ShellShortcut::TimeTravel(TimeTravelVerb::from_shortcut_id(other)?),
    })
}

/// ⌨️ The REMAPPABLE shortcut table: [`SHELL_SHORTCUT_ROWS`] with the user's `keybindingOverrides`
/// applied per control id — the Rust twin of React's `composeControlKeybindings`
/// (`🔨️modules/🖱️ui/🔨️modules/🕹️control-keybinding-context/🟦️.tsx`), whose precedence is
/// shell defaults, then app-declared action bindings (only for ids the shell does not claim), then
/// the user override, unconditionally. App-declared bindings never enter *this* table: they are
/// keyed by action id and dispatched by `match_app_keybinding` on the rung below the shell's, so the
/// only layer over the defaults here is the override map.
///
/// An override whose value carries no parsable chord is dropped rather than blanking the row, exactly
/// like React's `parseUiKeybindingOverrides` gate.
pub(crate) fn shell_shortcut_table(overrides: &HashMap<String, String>) -> Vec<(String, String)> {
    SHELL_SHORTCUT_ROWS
        .iter()
        .map(|(control_id, keys)| {
            let resolved = overrides.get(*control_id).filter(|keys| keys.split(',').any(|chord| !chord.trim().is_empty())).map(String::as_str).unwrap_or(keys);
            ((*control_id).to_string(), resolved.to_string())
        })
        .collect()
}

/// ⌨️ The shell verb a key event asks for against a RESOLVED table, or `None` when the event belongs
/// to the app/content. First matching row wins, in table order, exactly like React's own scan.
pub(crate) fn shell_shortcut_for_in(table: &[(String, String)], action: &ui_wgpu::wgpu::KeyAction, modifiers: &PointerModifiers) -> Option<ShellShortcut> {
    table.iter().find(|(_, keys)| keys.split(',').any(|chord| key_event_matches_chord(action, modifiers, chord.trim()))).and_then(|(control_id, _)| shell_shortcut_for_control_id(control_id))
}

/// ⌨️ The shell verb a key event asks for with NO override applied — the default-table projection the
/// shortcut LAWS assert against (production always resolves through `ShellState::shortcut_table`, so an
/// override is never bypassed).
#[cfg(test)]
pub(crate) fn shell_shortcut_for(action: &ui_wgpu::wgpu::KeyAction, modifiers: &PointerModifiers) -> Option<ShellShortcut> {
    shell_shortcut_for_in(&shell_shortcut_table(&HashMap::new()), action, modifiers)
}

/// ⌨️ One row's remapped state for the Keybindings settings leaf: the chord in force, whether it is a
/// user override, and whether some LATER row claims the same first chord — React's
/// `buildSettingsKeybindingsTree` conflict marker (`📌️ChromePanels/🟦️.tsx`), whose rule is
/// last-writer-wins over the first chord of every row, so exactly the earlier row of a colliding pair
/// is flagged.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ShellKeybindingRow {
    pub control_id: String,
    pub keys: String,
    pub overridden: bool,
    pub conflicted: bool,
}

/// ⌨️ Every shortcut row with its remapped chord and conflict marker, sorted by control id like
/// React's own `localeCompare` row order.
pub(crate) fn shell_keybinding_rows(overrides: &HashMap<String, String>) -> Vec<ShellKeybindingRow> {
    let table = shell_shortcut_table(overrides);
    let first_chord = |keys: &str| keys.split(',').map(str::trim).find(|chord| !chord.is_empty()).map(str::to_ascii_lowercase);
    let mut chord_owners: HashMap<String, String> = HashMap::new();
    for (control_id, keys) in &table {
        if let Some(chord) = first_chord(keys) {
            chord_owners.insert(chord, control_id.clone());
        }
    }
    let mut rows: Vec<ShellKeybindingRow> = table
        .iter()
        .map(|(control_id, keys)| ShellKeybindingRow {
            conflicted: first_chord(keys).is_some_and(|chord| chord_owners.get(&chord) != Some(control_id)),
            control_id: control_id.clone(),
            keys: keys.clone(),
            overridden: overrides.contains_key(control_id),
        })
        .collect();
    rows.sort_by(|left, right| left.control_id.cmp(&right.control_id));
    rows
}

/// ⏪️ The framework-universal edit chords React hardcodes at the very END of `handleAppKeydown`
/// (`🏛️ShellHost/🟦️.tsx`): `mod+z`, `mod+shift+z` and the `mod+y` redo alias. Deliberately NOT part of
/// [`SHELL_SHORTCUT_ROWS`] — React dispatches them AFTER the app-declared keybinding loop, so an app
/// that declares `mod+z` itself shadows them, and they are therefore never reserved chords.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ShellEditVerb {
    Undo,
    Redo,
}

impl ShellEditVerb {
    pub(crate) fn action_id(self) -> &'static str {
        match self {
            ShellEditVerb::Undo => "undo",
            ShellEditVerb::Redo => "redo",
        }
    }
}

/// ⏪️ The undo/redo verb a key event asks for.
pub(crate) fn shell_edit_verb_for(action: &ui_wgpu::wgpu::KeyAction, modifiers: &PointerModifiers) -> Option<ShellEditVerb> {
    if key_event_matches_chord(action, modifiers, "mod+z") {
        return Some(ShellEditVerb::Undo);
    }
    if key_event_matches_chord(action, modifiers, "mod+shift+z") || key_event_matches_chord(action, modifiers, "mod+y") {
        return Some(ShellEditVerb::Redo);
    }
    None
}
//#endregion ⌨️ShellShortcutTable

impl ShellState {
    // #region utility-derivation
    /// 🧰️ The window kind whose utilities/actions the shell chrome currently scopes to (the focused window,
    /// else the view-state's active kind, else the app's first kind).
    pub(crate) fn active_utility_bar_window_kind<'a>(&self, session: &'a ActiveSession) -> &'a semio_framework::WindowKindDefinition {
        let active_id = self.active_window_id.as_deref().or(session.view_state.window_id.as_deref());
        active_id.and_then(|id| self.live_window_kind_id(session, id)).and_then(|kind_id| session.app.window_kinds.iter().find(|kind| kind.id == kind_id)).unwrap_or_else(|| session.app.window_kinds.first())
    }

    /// 🧰️ Derives the ACTIVE window's utility `UtilityNode`s — the roster the shell's own hit router and
    /// keybinding lane address by collection path. The chrome that PAINTS them is each pane's own
    /// `Utilities` group, so this answers `active_utility_bar_window_kind`'s window, never the footer.
    pub(crate) fn derive_utility_nodes(&self, session: &ActiveSession) -> Vec<UtilityNode> {
        self.derive_window_utility_nodes(session, self.active_utility_bar_window_kind(session).id.as_str())
    }

    /// 🧰️ Derives ONE window instance's utility bar from the app's declared utilities scoped to that
    /// window's kind, marking the host-owned active utility pressed (Architecture Decision 5) — React's
    /// `resolveUtilityNodes(session.app, kind, activeUtilityByWindowId[instance.id], instance.id, …)`
    /// (`🏛️ShellHost/🟦️.tsx:10249`), which is derived PER PANE, never once for the shell.
    pub(crate) fn derive_window_utility_nodes(&self, session: &ActiveSession, window_id: &str) -> Vec<UtilityNode> {
        if session.app.utilities.is_empty() {
            return Vec::new();
        }
        let kind_id = self.live_window_kind_id(session, window_id).unwrap_or(window_id).to_string();
        let window_kind = session.app.window_kinds.iter().find(|kind| kind.id == kind_id).unwrap_or_else(|| session.app.window_kinds.first());
        let resolved = resolve_window_utilities(&session.app, window_kind);
        if resolved.is_empty() {
            return Vec::new();
        }
        let specs: Vec<ui_wgpu::wgpu::component::utilities::DerivedUtilitySpec> = resolved
            .iter()
            .map(|utility| ui_wgpu::wgpu::component::utilities::DerivedUtilitySpec {
                id: utility.id.clone(),
                label: utility.label.resolve(self.active_terminology(), self.active_locale()).to_string(),
                icon_id: utility.icon_id.clone(),
                group: utility.group.clone(),
                category: utility.category,
            })
            .collect();
        let active = self.active_utility_by_window.get(window_id).map(String::as_str);
        ui_wgpu::wgpu::component::utilities::derive_utility_nodes(&session.app.controller_id, &specs, active)
    }
    // #endregion

    // #region active-utility
    /// 🧰️ Applies a user-driven `setActiveUtility`: re-selecting the active utility deactivates it, otherwise
    /// it becomes the active utility for that window kind (Architecture Decision 4).
    ///
    /// 🎓️ Advance-by-doing: only the activation branch counts as "the utility was activated" —
    /// see `chrome_tour_note_utility_performed`.
    pub(crate) fn toggle_active_utility(&mut self, window_id: &str, utility_id: &str) {
        let next = (!utility_id.is_empty() && self.active_utility_for_window(window_id) != Some(utility_id)).then_some(utility_id);
        self.set_active_utility(window_id, next);
    }

    /// 🎯️ An explicit guest arm is idempotent and only a changed activation advances its tutorial.
    pub(crate) fn set_active_utility(&mut self, window_id: &str, utility_id: Option<&str>) {
        if self.active_utility_for_window(window_id) == utility_id { return; }
        match utility_id {
            Some(utility_id) => {
                self.active_utility_by_window.insert(window_id.into(), utility_id.into());
                self.chrome_tour_note_utility_performed(utility_id);
            }
            None => { self.active_utility_by_window.remove(window_id); }
        }
    }

    /// 🧰️ The active utility id for a window kind, if any.
    pub(crate) fn active_utility_for_window(&self, window_id: &str) -> Option<&str> {
        self.active_utility_by_window.get(window_id).map(String::as_str)
    }

    /// 🖱️ The cursor the active utility requests while the pointer is inside the active window silhouette — maps
    /// `UtilityDefinition.cursor` onto a {@link ui_wgpu::wgpu::SemioCursor} (P5). `None` when no utility/cursor applies.
    pub(crate) fn utility_cursor_override(&self, x: f32, y: f32) -> Option<ui_wgpu::wgpu::SemioCursor> {
        let session = self.session.as_ref()?;
        let window_id = self.active_window_id.as_deref()?;
        let utility_id = self.active_utility_for_window(window_id)?;
        let cursor_name = session.app.utilities.iter().find(|utility| utility.id == utility_id)?.cursor.as_deref()?;
        let geometry = &self.presented_input_geometry;
        let visible = geometry.window_silhouettes.get(window_id).map_or_else(|| geometry.window_content_rects.get(window_id).is_some_and(|rect| rect.contains(x, y)), |silhouette| silhouette.contains(x, y));
        visible.then(|| semio_cursor_from_name(cursor_name))
    }

    /// 🚦️ Whether window-scoped actions stay enabled: `true` when no utility is active or the active utility
    /// declares `allows_actions_while_active` (P5 — replaces the old `UTILITY_ID_PREFIXES` whitelist).
    pub(crate) fn actions_enabled_for_window(&self, app: &AppDefinition, window_id: &str) -> bool {
        match self.active_utility_for_window(window_id) {
            None => true,
            Some(utility_id) => app.utilities.iter().find(|utility| utility.id == utility_id).map(|utility| utility.allows_actions_while_active).unwrap_or(true),
        }
    }
    // #endregion

    // #region staging
    fn staged_key(window_id: &str, action_id: &str) -> String {
        format!("{window_id}:{action_id}")
    }

    pub(crate) fn stage_arg(&mut self, window_id: &str, action_id: &str, arg_id: &str, value: Value) {
        self.staged_action_args.entry(Self::staged_key(window_id, action_id)).or_default().insert(arg_id.to_string(), value);
    }

    pub(crate) fn staged_map_for(&self, window_id: &str, action_id: &str) -> serde_json::Map<String, Value> {
        self.staged_action_args.get(&Self::staged_key(window_id, action_id)).cloned().unwrap_or_default()
    }

    pub(crate) fn reset_staged_args(&mut self, window_id: &str, action_id: &str) {
        self.staged_action_args.remove(&Self::staged_key(window_id, action_id));
    }

    /// 📝️ The effective value of one argument — staged if present, else the declared default: React's
    /// `effectiveActionArgs` per row (`🛠️ShellHelpers/🟦️.tsx`), which is what the Actions pane's staged
    /// form paints and what Execute merges.
    fn effective_arg_value(&self, window_id: &str, action_id: &str, arg: &semio_framework::ActionArgDef) -> Option<Value> {
        self.staged_action_args.get(&Self::staged_key(window_id, action_id)).and_then(|map| map.get(&arg.id).cloned()).or_else(|| arg.default.as_ref().map(dsl_value_as_json))
    }

    /// 🧮️ Resolves required presence and current catalog membership before native staged execution.
    pub(crate) fn resolved_execute_args(defs: &[semio_framework::ActionArgDef], staged: &serde_json::Map<String, Value>) -> Option<serde_json::Map<String, Value>> {
        let staged_dsl = semio_framework_value::DslValue::from(Value::Object(staged.clone()));
        let effective = semio_framework::effective_action_args(defs, &staged_dsl, None);
        if semio_framework::unresolved_action_args(defs, &effective).is_empty() {
            Value::from(&effective).as_object().cloned()
        } else {
            None
        }
    }

    /// 🚀️ Executes a staged action once (P2): validates required args, dispatches exactly one
    /// `ActionDescriptor` with the merged effective args, and keeps the staged values for tweak-and-
    /// repeat. No-operations when the active utility gates actions or a required arg is still unset.
    pub(crate) fn execute_staged_action<'a>(&'a mut self, window_id: &'a str, action_id: &'a str) -> ShellTurn<'a, Result<(), String>> {
        shell_turn(async move {
            let Some(session) = self.session.clone() else {
                return Ok(());
            };
            if !self.actions_enabled_for_window(&session.app, window_id) {
                return Ok(());
            }
            let Some(action) = window_action_definition(&session.app, window_id, action_id).cloned() else {
                return Ok(());
            };
            let staged = self.staged_map_for(window_id, action_id);
            let Some(effective) = Self::resolved_execute_args(&action.args, &staged) else {
                return Ok(());
            };
            let args = (!effective.is_empty()).then(|| semio_framework::DslValue::from(Value::Object(effective)));
            self.dispatch_action(ActionDescriptor { controller_id: session.app.controller_id.clone(), action: action_id.to_string(), args }).await
        })
    }
    // #endregion

    // #region command-registry
    /// 🔌️ The current session's program manifest (for its `commands: Vec<CommandDefinition>` — Plugin-scope
    /// commands apply whenever any of that plugin's apps is focused, mirroring `os-shell.tsx`'s
    /// `activePluginManifest`).
    fn active_plugin_manifest(&self) -> Option<&semio_framework::PluginManifest> {
        let session = self.session.as_ref()?;
        self.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).map(|entry| &entry.manifest)
    }

    /// 🎛️ Os-level built-in commands — the wgpu mirror of `os-shell.tsx`'s `buildOsCommands`, scoped to
    /// mutation paths that already exist in this shell: `appearance_id`/`driver_id`/`locale_id`/
    /// `terminology_id` (all wired through the existing `"framework"` controller `dispatch_action` switch
    /// — see `apply_os_command`), and the dock's `layout_override`
    /// (`os.resetDock`, applied locally like `RESET_DOCK` resets `dockLayoutStore`/`dockUiStateStore` in
    /// React — no plugin round-trip). Deliberately omits `os.introduceApp`/`os.setThemeId`/`os.setLayout`:
    /// none of the three has any persisted shell state yet (no introduction-playback step, no named
    /// `UiTheme` list, no desktop/tablet layout flag) — inventing that storage is out of this region's
    /// scope (`shell::ShellTypes` owns `ShellState`'s fields and is off-limits this wave).
    pub(crate) fn build_os_commands(&self) -> Vec<semio_framework::CommandDefinition> {
        use semio_framework::manifest::Platform;
        use semio_framework::{ActionArgDef, ActionArgOption, ActionKind, CommandDefinition, PlatformKeybinding};
        let terminology_options: Vec<ActionArgOption> =
            self.active_terminologies().into_iter().map(|id| ActionArgOption { label: if id == "native" { LocalizedLabel::native("Native", "Nativ") } else { LocalizedLabel::data(id.clone()) }, value: id }).collect();
        vec![
            CommandDefinition::new("os.toggleFullscreen", LocalizedLabel::native("Toggle Full Screen", "Vollbild umschalten"), "window", "code", ActionKind::Shell)
                .with_keybinding(PlatformKeybinding::for_platform("f11", Platform::Windows))
                .with_keybinding(PlatformKeybinding::for_platform("f11", Platform::Linux))
                .with_keybinding(PlatformKeybinding::for_platform("control+meta+f", Platform::MacOs)),
            CommandDefinition::new("os.setAppearance", LocalizedLabel::native("Set Appearance", "Erscheinungsbild festlegen"), "appearance", "settings", ActionKind::Shell).with_args([ActionArgDef::select(
                "value",
                LocalizedLabel::native("Appearance", "Erscheinungsbild"),
                vec![
                    ActionArgOption { value: "system".into(), label: LocalizedLabel::native("System", "System") },
                    ActionArgOption { value: "light".into(), label: LocalizedLabel::native("Light", "Hell") },
                    ActionArgOption { value: "dark".into(), label: LocalizedLabel::native("Dark", "Dunkel") },
                ],
            )
            .required()]),
            CommandDefinition::new("os.setDriver", LocalizedLabel::native("Set Driver", "Treiber festlegen"), "layout", "settings", ActionKind::Shell).with_args([ActionArgDef::select(
                "value",
                LocalizedLabel::native("Driver", "Treiber"),
                vec![ActionArgOption { value: "default".into(), label: LocalizedLabel::native("Default", "Standard") }, ActionArgOption { value: "compact".into(), label: LocalizedLabel::native("Compact", "Kompakt") }],
            )
            .required()]),
            CommandDefinition::new("os.setLocale", LocalizedLabel::native("Set Locale", "Sprache festlegen"), "language", "settings", ActionKind::Shell).with_args([ActionArgDef::select(
                "value",
                LocalizedLabel::native("Locale", "Sprache"),
                vec![ActionArgOption { value: "en".into(), label: LocalizedLabel::data("English") }, ActionArgOption { value: "de".into(), label: LocalizedLabel::data("Deutsch") }],
            )
            .required()]),
            CommandDefinition::new("os.setTerminology", LocalizedLabel::native("Set Terminology", "Terminologie festlegen"), "language", "settings", ActionKind::Shell).with_args([ActionArgDef::select(
                "value",
                LocalizedLabel::native("Terminology", "Terminologie"),
                terminology_options,
            )
            .required()]),
            CommandDefinition::new("os.setThemeId", LocalizedLabel::native("Set Theme", "Thema festlegen"), "appearance", "settings", ActionKind::Shell).with_args([ActionArgDef::select(
                "value",
                LocalizedLabel::native("Theme", "Design"),
                std::iter::once(ActionArgOption { value: "semio".into(), label: LocalizedLabel::data("Semio") })
                    .chain(std::iter::once(ActionArgOption { value: "mono".into(), label: LocalizedLabel::data("Mono") }))
                    .chain(self.chrome_build.preferences.custom_themes.keys().cloned().map(|id| {
                        let label = custom_theme_document_from(&self.chrome_build.preferences, &id).map(|theme| theme.label).unwrap_or_else(|| id.clone());
                        ActionArgOption { value: id, label: LocalizedLabel::data(label) }
                    }))
                    .collect(),
            )
            .required()]),
            CommandDefinition::new("os.openTaskManager", LocalizedLabel::native("Open Task Manager", "Task-Manager öffnen"), "window", "cpu", ActionKind::Shell),
            CommandDefinition::new("os.openHub", LocalizedLabel::native("Open Hub", "Hub öffnen"), "general", "users", ActionKind::Shell),
            CommandDefinition::new("os.resetDock", LocalizedLabel::native("Reset Dock Layout", "Dock-Layout zurücksetzen"), "layout", "panel-left", ActionKind::Shell),
        ]
    }

    /// 🎛️ Every command visible for the current session — os built-ins, the active plugin's Plugin-scope
    /// commands, and the app's App-/active-Mode-scope commands. The wgpu mirror of `os-shell.tsx`'s
    /// `resolveCommands(buildOsCommands(...), activePluginManifest, session?.app, activeModeId)` call site.
    pub(crate) fn resolved_commands(&self) -> Vec<ResolvedCommand> {
        let os_commands = self.build_os_commands();
        let Some(session) = self.session.as_ref() else {
            return os_commands.into_iter().map(|definition| ResolvedCommand::new(definition, semio_framework::manifest::CommandOwnerAddress::Os)).collect();
        };
        let active_mode_id = session.view_state.active_mode_id.as_deref().unwrap_or(session.app.default_mode_id.as_str());
        resolve_commands(os_commands, self.active_plugin_manifest(), &session.plugin_id, &session.app, active_mode_id)
    }

    /// 🔍️ Flattens `resolved_commands()` into quick-search-palette entries — EVERY in-palette command,
    /// whatever its owner, exactly as React's `searchItems` memo lists every `resolvedCommands` entry.
    ///
    /// 🩸️ This used to SKIP every arg-carrying Plugin/App/Mode-scope command on the premise that "there
    /// is no `ArtifactApp::handle_command` RPC wired on the plugin bridge yet (only `handle_action`
    /// exists)". That premise is stale: `ProgramBridgeEntry::handle_command` is wired on both the JS and
    /// the wasm backend (`🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs`) and `dispatch_command` already drives
    /// it — so those commands were merely invisible, not unexecutable.
    ///
    /// Two shapes, both reachable:
    /// - zero args → fires immediately (os commands locally, everything else across the command boundary);
    /// - any command carrying args → React's single `…`-suffixed row that redirects to the
    ///   command's staged form (`command-form:`) rather than firing a guess.
    pub(crate) fn command_search_items(&self) -> Vec<SearchPaletteItem> {
        let mut items = Vec::new();
        for entry in self.resolved_commands() {
            let ResolvedCommand { definition, address } = entry;
            if !definition.in_palette {
                continue;
            }
            let category = address.owner.clone();
            let group = command_category_label(&definition.category);
            let is_os = matches!(address.owner, semio_framework::manifest::CommandOwnerAddress::Os);
            let platform = command_host_platform();
            let chords: Vec<String> = definition.keybindings.iter().filter(|binding| binding.platform.is_none_or(|declared| declared == platform)).map(|binding| format_keybinding_shortcut(&binding.chord)).collect();
            let description = (!chords.is_empty()).then(|| chords.join(","));
            let label = definition.label.resolve(self.active_terminology(), self.active_locale()).to_string();
            if definition.args.is_empty() {
                let invocation = semio_framework::manifest::CommandInvocation { address: address.clone(), arguments: BTreeMap::new() };
                items.push(SearchPaletteItem {
                    id: format!("command.{}", command_address_stable_key(&address).replace(':', ".")),
                    label,
                    description,
                    group,
                    dispatch_action: None,
                    action: Some(if is_os { format!("os-command:{}", definition.id) } else { format!("command:{}", semio_framework_pack_json::to_json_string(&invocation)) }),
                    category: Some(category.clone()),
                });
                continue;
            }
            items.push(SearchPaletteItem {
                id: format!("command.{}", command_address_stable_key(&address).replace(':', ".")),
                label: format!("{label}…"),
                description,
                group,
                dispatch_action: None,
                action: Some(format!("command-form:{}", command_address_stable_key(&address))),
                category: Some(category),
            });
        }
        items
    }

    /// 🚀️ Executes an os-level command by id (the wgpu mirror of `os-shell.tsx`'s `dispatchOsCommand`):
    /// `os.resetDock` clears the persisted layout override locally (no document round-trip, exactly like
    /// React's `RESET_DOCK` resetting `dockLayoutStore`/`dockUiStateStore`); every other command reuses the
    /// existing `"framework"` controller `dispatch_action` switch (`setAppearance`/`setDriver`/
    /// `setLocale`/`setTerminology`) that already backs the Settings panel's selects, so this
    /// never invents a new mutation path.
    /// 🗣️ The resolved locale, and what it actually resolves the chrome, the app's own window labels
    /// and the os command registry TO.
    ///
    /// This is the ONE runtime witness of a locale on a GPU canvas. Every translated string this
    /// target renders is painted into pixels: it is not in the retained document (which carries the
    /// user's own node names, identical in every tongue) and not in the DOM, so two lanes in a row
    /// could neither prove nor disprove a German session from outside the canvas
    /// (`📓️wgpu-status-a11y-i18n-2026-09-13.md` §5.3, ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    /// Printed once per RESOLUTION — a preferences load and a `setLocale` — never per frame.
    pub(crate) fn trace_resolved_locale(&self, reason: &str) {
        let is_de = self.locale_id == "de";
        let chrome = ["overlay.search.title", "overlay.find.title", "example.overlay.title", "common.cancel", "common.close", "common.focus", "common.loading", "nodeGraph.fitGraph", "contextMenu.goHome"]
            .iter()
            .map(|key| format!("{key}={}", shell_chrome_string(key, is_de)))
            .collect::<Vec<_>>();
        let windows = self.session.as_ref().map(|session| session.app.window_kinds.iter().map(|kind| format!("{}={}", kind.id, kind.label.resolve(self.active_terminology(), self.active_locale()))).collect::<Vec<_>>()).unwrap_or_default();
        let commands = self.build_os_commands().into_iter().map(|definition| format!("{}={}", definition.id, definition.label.resolve(self.active_terminology(), self.active_locale()))).collect::<Vec<_>>();
        Self::debug_log(&format!("[TRACE] wgpu-shell locale resolved={} terminology={} reason={reason} chrome=[{}] windows=[{}] commands=[{}]", self.locale_id, self.terminology_id, chrome.join(", "), windows.join(", "), commands.join(", ")));
    }

    /// 🧭️ React's `RESET_DOCK` clears BOTH persisted dock layers — the arrangement override
    /// (`dockLayoutStore`) and the per-anchor chrome (`dockUiStateStore`) — so the panel dock
    /// returns to its computed default, not just the window layout.
    pub(crate) fn apply_os_command<'a>(&'a mut self, command_id: &'a str, option_value: Option<&'a str>) -> ShellTurn<'a, Result<(), String>> {

        shell_turn(async move {
            Self::debug_log(&format!("[TRACE] wgpu-shell os command id={command_id} value={option_value:?}"));
            match command_id {
                "os.toggleFullscreen" => {
                    self.fullscreen_toggle_requested = true;
                    Ok(())
                }
                "os.openTaskManager" => {
                    self.sync_dock_tabs();
                    self.reveal_dock_tab(FRAMEWORK_TASK_MANAGER_PANEL_ID);
                    Ok(())
                }
                "os.openHub" => {
                    self.push_uri("/hub".to_string());
                    self.apply_shell_uri("/hub").await
                }
                "os.resetDock" => {
                    self.layout_override = None;
                    self.sync_dock();
                    self.dock_override = None;
                    self.panel_anchors = Default::default();
                    self.dock_collapsed_branches.clear();
                    self.sync_dock_tabs();
                    let app_id = self.dock_store_app_id();
                    save_dock_skeleton_to_store(app_id.as_deref(), None);
                    save_dock_ui_to_store(app_id.as_deref(), None);
                    self.chrome_present.last_persisted_dock_skeleton = Some(None);
                    self.chrome_present.last_persisted_dock_ui = Some(self.dock_ui_snapshot());
                    Ok(())
                }
                "os.setAppearance" | "os.setDriver" | "os.setLocale" | "os.setTerminology" | "os.setThemeId" => {
                    let Some(value) = option_value else {
                        return Ok(());
                    };
                    let action = match command_id {
                        "os.setAppearance" => "setAppearance",
                        "os.setDriver" => "setDriver",
                        "os.setLocale" => "setLocale",
                        "os.setTerminology" => "setTerminology",
                        "os.setThemeId" => "setThemeId",
                        _ => unreachable!(),
                    };
                    self.dispatch_action(ActionDescriptor { controller_id: "framework".into(), action: action.into(), args: crate::action_args_json!({ "value": value }) }).await
                }
                _ => Ok(()),
            }

        })

    }

    //#region ShellCommandHistory
    /// 🕒️ Looks up an `os.*` setting command's display label from `build_os_commands`' registry
    /// (`os.setAppearance`/`os.setDriver`/`os.setLocale`/`os.setTerminology`/`os.setThemeId`), falling
    /// back to the theme reset/delete buttons' own `shell_chrome_string` i18n labels for the two
    /// `build_settings_theme_ui` commands that never got a `CommandDefinition` (see that fn's own
    /// `os.setThemeId`-only doc comment) — the single label source `note_shell_command_action` call
    /// sites below draw from, so a chrome label edit never drifts from what the history panel shows.
    pub(crate) fn shell_command_label_for_setting(&self, command_id: &str) -> String {
        match command_id {
            "os.resetThemeId" => shell_chrome_string("settings.theme.reset", self.locale_id == "de").to_string(),
            "os.deleteThemeId" => shell_chrome_string("settings.theme.delete", self.locale_id == "de").to_string(),
            _ => self.build_os_commands().into_iter().find(|definition| definition.id == command_id).map(|definition| definition.label.resolve(self.active_terminology(), self.active_locale()).to_string()).unwrap_or_else(|| command_id.to_string()),
        }
    }

    /// 🧭️ Parses `dock.tab.{path}.{window}.{action}` into `(path, window_id)`.
    pub(crate) fn parse_dock_tab_action_id<'a>(control_id: &'a str, action: &str) -> Option<(Vec<usize>, &'a str)> {
        let rest = control_id.strip_prefix("dock.tab.")?;
        let rest = rest.strip_suffix(&format!(".{action}"))?;
        let (path_str_value, window_id) = rest.split_once('.')?;
        Some((parse_path(path_str_value), window_id))
    }

    /// 🕒️ Maps a chrome control id to its `noteShellCommand` `(commandId, label)` pair — factored out
    /// of `handle_shell_hit`'s per-arm dispatch so the mapping is unit-testable without a full
    /// `ShellState` fixture. Only control ids that represent a discrete, loggable user command are
    /// covered; everything else is `None` (`handle_shell_hit` keeps its existing behavior either way).
    ///
    /// 🈳️ A dock tab's FOCUS chip is deliberately absent. React's chip is
    /// `dock.activateWindow(tab.id)` plus `dock.toggleMaximize(stackPath)`
    /// (`🖱️ui/🧱️elements/🎨️Canvas/🟦️.tsx:1076-1086`): the activation is journaled by
    /// `handleActiveWindowChange`'s own `shell.windowActivate` note — which this shell arms through
    /// [`Self::arm_window_activation_note`] — and the maximize is pure local state that React notes
    /// NOWHERE (`🏛️ShellHost/🟦️.tsx` declares no `shell.windowMaximize` command at all). This renderer
    /// used to note one, so `window-cap-focus` journaled a `noteShellCommand` React never emits
    /// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `🗑️generated/w12c-parity-run-19/steps.json` step 16).
    pub(crate) fn shell_command_for_control(control_id: &str, is_de: bool) -> Option<(&'static str, String)> {
        if control_id.starts_with("dock.tab.") && control_id.ends_with(".close") {
            return Some(("shell.windowClose", shell_chrome_string("common.close", is_de).to_string()));
        }
        if control_id == "ui.window.close" {
            return Some(("shell.windowClose", shell_chrome_string("common.close", is_de).to_string()));
        }
        if control_id.starts_with("shell.layout.") {
            return Some(("shell.applyNamedLayout", shell_chrome_string("command.applyLayout", is_de).to_string()));
        }
        if control_id == "ui.panelToggle.details" {
            return Some(("shell.panelToggle", shell_chrome_string("panelToggle.details", is_de).to_string()));
        }
        if control_id == "ui.panelToggle.settings" {
            return Some(("shell.panelToggle", shell_chrome_string("panelToggle.settings", is_de).to_string()));
        }
        if control_id == "ui.panelToggle.chat" {
            return Some(("shell.panelToggle", shell_chrome_string("panelToggle.chat", is_de).to_string()));
        }
        None
    }

    /// 🕒️ Builds a `noteShellCommand` `ActionDescriptor` — same inline-construction shape as
    /// `map_action`/`scene_action`/`board_action` elsewhere in this crate, targeting the
    /// framework-injected, session-only command-history tap (Shell-kind, intercepted before the app
    /// ever sees it — see the plugin crate's universal command-recording mechanism). `detail` is the
    /// caller-supplied JSON payload (e.g. `{"value": ...}`/`{"windowId": ...}`), left out of `args`
    /// entirely when `None` rather than serialized as `null`.
    pub(crate) fn note_shell_command_action(controller_id: &str, command_id: &str, label: &str, detail: Option<Value>) -> ActionDescriptor {
        let mut args = serde_json::json!({ "commandId": command_id, "label": label });
        if let Some(detail) = detail {
            args["detail"] = detail;
        }
        ActionDescriptor { controller_id: controller_id.to_string(), action: "noteShellCommand".into(), args: Some(semio_framework::DslValue::from(args)) }
    }

    /// 🕒️ The `dispatch_action`-recursion delivery path (mechanism (a) — a `noteShellCommand`'s
    /// action id falls through every special-cased arm, including this one's own `"framework"`
    /// branch, straight to the normal plugin-forwarding tail) for the framework settings arms below.
    /// The recursion itself is exactly what the ticket calls for; `dispatch_action` is a [`ShellTurn`], boxed at its definition.
    /// No-ops without an active session: nothing to log a shell command against.
    pub(crate) async fn note_shell_setting_command(&mut self, command_id: &str, value: Option<&str>) -> Result<(), String> {
        let Some(controller_id) = self.host_controller_id() else {
            return Ok(());
        };
        let label = self.shell_command_label_for_setting(command_id);
        let detail = value.map(|value| serde_json::json!({ "value": value }));
        let note = Self::note_shell_command_action(&controller_id, command_id, &label, detail);
        self.dispatch_action(note).await
    }

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
        let Some(instance_id) = self.session.as_ref().map(|session| session.instance_id) else {
            return;
        };
        let Some(active) = self.active_window_id.clone().filter(|id| !id.is_empty()) else {
            return;
        };
        let owed = window_activation_is_user_v1(self.noted_active_window.as_ref(), instance_id, &active);
        self.noted_active_window = Some((instance_id, active.clone()));
        if !owed {
            return;
        }
        let Some(controller_id) = self.shell_command_controller_id() else {
            return;
        };
        let label = shell_chrome_string("command.activateWindow", self.locale_id == "de");
        self.deferred_actions.push(Self::note_shell_command_action(&controller_id, "shell.windowActivate", label, Some(serde_json::json!({ "windowId": active }))));
    }

    /// 📨️ Arms `hostEvent{windowId, kind: blur}` for the pane that lost the activation — React's pane `onBlur` twin — so
    /// the program ends an open gesture there. The first observation of a session seeds the witness without a blur.
    pub(crate) fn arm_window_blur_host_event(&mut self) {
        let Some(instance_id) = self.session.as_ref().map(|session| session.instance_id) else {
            return;
        };
        let Some(active) = self.active_window_id.clone().filter(|id| !id.is_empty()) else {
            return;
        };
        let blurred = self.host_event_active_window.replace((instance_id, active.clone())).filter(|(seen, window)| *seen == instance_id && *window != active).map(|(_, window)| window);
        if let (Some(blurred), Some(controller_id)) = (blurred, self.shell_command_controller_id()) {
            self.deferred_actions.push(window_host_event_action(&controller_id, &blurred, semio_framework::HOST_EVENT_KIND_BLUR));
        }
    }

    /// 🫥️ Takes a pending host window blur ([`note_host_window_blur`]): every World3d surface hears a `Blur` cancel (a live
    /// gumball that streamed or an open paint stroke aborts with zero trace), and `hostEvent{windowId, kind: blur}` is armed for
    /// the pane holding the activation, so its program ends an open gesture there. `true` when one was pending.
    pub(crate) fn arm_host_window_blur(&mut self) -> bool {
        if !std::mem::take(&mut *HOST_WINDOW_BLUR_PENDING.borrow_mut()) {
            return false;
        }
        for state in self.world3d_states.values_mut() {
            if enqueue_world3d_events(state, [WorldInteractionIntent::cancel(infinite_world::world::WorldCancelReason::Blur)]).is_err() {
                Self::debug_log("[TRACE] wgpu-shell world3d blur cancel refused: the intent queue is full");
            }
        }
        let active = self.active_window_id.clone().filter(|id| !id.is_empty());
        if let (Some(active), Some(_), Some(controller_id)) = (active, self.session.as_ref(), self.shell_command_controller_id()) {
            self.deferred_actions.push(window_host_event_action(&controller_id, &active, semio_framework::HOST_EVENT_KIND_BLUR));
        }
        true
    }

    /// 🕒️ `handle_shell_hit`'s generic seam into `shell_command_for_control`'s `(commandId, label)`
    /// mapping — every discrete chrome-control arm that should log a history row calls this instead
    /// of hand-rolling the same `host_controller_id`/`dispatch_action` boilerplate. A silent no-op for
    /// any control id `shell_command_for_control` doesn't recognize, and without an active session
    /// (nothing to log a shell command against). Not itself inside `dispatch_action`, so a plain
    /// `.await` (no `Box::pin`) suffices here.
    pub(crate) async fn note_control_command(&mut self, control_id: &str, detail: Option<Value>) -> Result<(), String> {
        let Some((command_id, label)) = Self::shell_command_for_control(control_id, self.locale_id == "de") else {
            return Ok(());
        };
        let Some(controller_id) = self.shell_command_controller_id() else {
            return Ok(());
        };
        let note = Self::note_shell_command_action(&controller_id, command_id, &label, detail);
        self.dispatch_action(note).await
    }

    pub(crate) fn arm_control_command(&mut self, control_id: &str, detail: Option<Value>) {
        let Some((command_id, label)) = Self::shell_command_for_control(control_id, self.locale_id == "de") else {
            return;
        };
        let Some(controller_id) = self.shell_command_controller_id() else {
            return;
        };
        self.deferred_actions.push(Self::note_shell_command_action(&controller_id, command_id, &label, detail));
    }

    /// 🕒️ `ui.panelToggle.details`/`ui.panelToggle.settings` share this: both log the same
    /// `shell.panelToggle` command, differing only in the `panel` id and the post-toggle `visible`
    /// flag — the caller has already flipped `right_panel_open`/`active_right_kind` before this runs,
    /// so `visible` reads the POST-toggle state directly (no separate before/after bookkeeping).
    pub(crate) async fn note_panel_toggle_command(&mut self, control_id: &str, panel: &str) -> Result<(), String> {
        let visible = match panel {
            "details" => self.anchor_open(PanelAnchor::TopRight),
            "settings" => self.anchor_open(PanelAnchor::BottomRight),
            "chat" => self.anchor_open(PanelAnchor::TopRight) && self.anchor_state(PanelAnchor::TopRight).active_tab() == Some(FRAMEWORK_CHAT_PANEL_ID),
            _ => false,
        };
        self.note_control_command(control_id, Some(serde_json::json!({ "panel": panel, "visible": visible }))).await
    }
    //#endregion ShellCommandHistory

    /// 🎛️ One command-category leaf's retained document — the shell's OWN publication, minted through
    /// the same `publish_window_measures` ingress every window overlay uses (revision-keyed, so an
    /// unchanged category pays no ingress). This is what makes the `bottom-middle` Command anchor a real
    /// painted panel rather than a builder with no reader.
    fn publish_command_panel_document(&mut self, tab_id: &str, category_id: &str) -> Result<Option<UiDocumentLease>, String> {
        let node = self.build_command_category_ui(category_id);
        let records = panel_ui_records(tab_id, &node)?;
        self.publish_surface_records(tab_id, records).map(Some)
    }

    /// 🧾️ Every SHELL-OWNED panel leaf id this frame publishes a retained document for — the Command
    /// categories, the five Settings leaves, the Marketplace leaf, each mode tool's leaf and the chat
    /// transcript. Derived from `default_dock`'s own sources, so a leaf can never be dockable without
    /// a body or carry a body without being dockable.
    pub(crate) fn shell_owned_panel_leaves(&self) -> Vec<String> {
        let mut leaves: Vec<String> = self.command_category_tabs().into_iter().map(|tab| tab.id).collect();
        leaves.push(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID.to_string());
        leaves.push(FRAMEWORK_DISPLAY_LAYOUT_TAB_ID.to_string());
        leaves.extend(self.framework_settings_tabs().into_iter().map(|tab| tab.id));
        leaves.push(FRAMEWORK_MARKETPLACE_TAB_ID.to_string());
        leaves.push(FRAMEWORK_SYNC_PANEL_TAB_ID.to_string());
        leaves.push(FRAMEWORK_TASK_MANAGER_PANEL_ID.to_string());
        leaves.push(FRAMEWORK_CHAT_PANEL_ID.to_string());
        leaves.push(crate::hub_connection::FRAMEWORK_HUB_PANEL_ID.to_string());
        leaves.extend(self.tool_panel_tabs().into_iter().map(|tab| tab.id));
        leaves
    }

    /// 🧾️ One shell-owned leaf's retained document, routed by leaf id to its builder and minted through
    /// {@link panel_ui_records} + {@link ShellState::publish_surface_records} — the same revision-keyed
    /// ingress every window overlay uses, so an unchanged body pays no ingress. `None` for a leaf whose
    /// builder has nothing to show (React's own "no rows, no tree" case).
    pub(crate) fn publish_shell_panel_document(&mut self, tab_id: &str) -> Result<Option<UiDocumentLease>, String> {
        if let Some(category) = tab_id.strip_prefix(FRAMEWORK_COMMAND_CATEGORY_TAB_PREFIX).map(ToOwned::to_owned) {
            return self.publish_command_panel_document(tab_id, &category);
        }
        let node = match tab_id {
            FRAMEWORK_DISPLAY_WINDOWS_TAB_ID => self.build_display_windows_ui(),
            FRAMEWORK_DISPLAY_LAYOUT_TAB_ID => self.build_display_layout_ui(),
            FRAMEWORK_SETTINGS_GENERAL_TAB_ID => self.build_settings_general_ui(),
            FRAMEWORK_SETTINGS_THEME_TAB_ID => self.build_settings_theme_ui(),
            FRAMEWORK_SETTINGS_KEYBINDINGS_TAB_ID => self.build_settings_keybindings_ui(),
            FRAMEWORK_SETTINGS_DEFAULT_APPS_TAB_ID => self.build_settings_default_apps_ui(),
            FRAMEWORK_SETTINGS_CONFLICTS_TAB_ID => self.build_settings_conflicts_ui(),
            FRAMEWORK_MARKETPLACE_TAB_ID => self.build_marketplace_ui(),
            FRAMEWORK_SYNC_PANEL_TAB_ID => self.build_sync_attach_ui(),
            FRAMEWORK_TASK_MANAGER_PANEL_ID => self.build_task_manager_ui(),
            FRAMEWORK_CHAT_PANEL_ID => self.build_agent_chat_ui(),
            crate::hub_connection::FRAMEWORK_HUB_PANEL_ID => crate::hub_connection::build_hub_workspace_ui(&self.hub_workspace, self.active_locale()),
            other => match other.strip_prefix(FRAMEWORK_TOOL_PANEL_TAB_PREFIX) {
                Some(tool_id) => self.build_tool_panel_ui(tool_id),
                None => return Ok(None),
            },
        };
        let records = if ui_node_is_windowed(&node) { panel_ui_scroll_records(tab_id, &node)? } else { panel_ui_records(tab_id, &node)? };
        self.publish_surface_records(tab_id, records).map(Some)
    }

    /// ♻️ Replaces one mounted host-owned retained panel after its local authority changes, without
    /// asking the guest to refresh unrelated bodies. Retirement admission is reserved before the
    /// successor is published; the exact prior lease enters that reservation before the map swaps.
    pub(crate) fn republish_shell_panel_document(&mut self, tab_id: &str) -> Result<(), String> {
        if !self.panel_documents.contains_key(tab_id) {
            return Ok(());
        }
        let Some(admission) = self.closing_documents.try_reserve_admission() else {
            return Err("shell: document retirement registry refused the exact prior owner".to_string());
        };
        let Some(document) = self.publish_shell_panel_document(tab_id)? else {
            let previous = self.panel_documents.remove(tab_id);
            if let Some(previous) = previous {
                self.closing_documents.admit_reserved(admission, previous);
                self.drain_retained_document_arenas();
            }
            return Ok(());
        };
        let previous = self.panel_documents.remove(tab_id).expect("mounted shell panel retains its exact prior owner");
        self.closing_documents.admit_reserved(admission, previous);
        self.panel_documents.insert(tab_id.to_string(), document);
        self.drain_retained_document_arenas();
        Ok(())
    }

    /// 🎛️ One category leaf's body — the wgpu twin of React's `buildCommandCategoryTree`
    /// (`🛠️ShellHelpers/🟦️.tsx`): the STAGED FORM section for the expanded command first (so a bottom
    /// anchor's reversed section order paints it above the list, exactly as React's does), then the
    /// list of every other command in the category.
    ///
    /// 🎛️ This is [`ShellState::expanded_command_id`]'s reader — the gap `📓️w1c` recorded. React's
    /// `autoExpandedSingleton` rule is honoured too: a category with exactly one arg-carrying command
    /// opens that command's form with no expansion write at all.
    pub(crate) fn build_command_category_ui(&self, category_id: &str) -> UiNode {
        let is_de = self.locale_id == "de";
        let terminology = self.active_terminology();
        let locale = self.active_locale();
        let entries: Vec<ResolvedCommand> = self.resolved_commands().into_iter().filter(|entry| entry.definition.in_palette && entry.definition.category == category_id).collect();
        let arg_carrying: Vec<&ResolvedCommand> = entries.iter().filter(|entry| !entry.definition.args.is_empty()).collect();
        let expanded_key = match self.expanded_command_id.as_deref() {
            Some(key) if arg_carrying.iter().any(|entry| command_address_stable_key(&entry.address) == key) => Some(key.to_string()),
            _ => (arg_carrying.len() == 1).then(|| command_address_stable_key(&arg_carrying[0].address)),
        };
        let mut sections: Vec<UiTreeSectionNode> = Vec::new();
        if let Some(expanded) = expanded_key.as_ref().and_then(|key| entries.iter().find(|entry| &command_address_stable_key(&entry.address) == key)) {
            let key = command_address_stable_key(&expanded.address);
            let staged = self.staged_command_args.get(&key).cloned().unwrap_or_default();
            let staged_dsl = semio_framework_value::DslValue::from(Value::Object(staged.clone()));
            let effective_dsl = semio_framework::effective_action_args(&expanded.definition.args, &staged_dsl, None);
            let missing = !semio_framework::unresolved_action_args(&expanded.definition.args, &effective_dsl).is_empty();
            let effective = Value::from(&effective_dsl).as_object().cloned().unwrap_or_default();
            let mut rows = Vec::new();
            for arg in &expanded.definition.args {
                rows.push(staged_command_arg_row(&key, arg, effective.get(&arg.id), &self.interaction_selection, terminology, locale));
            }
            let element_key = key.replace(':', ".");
            let toolbar = UiStackNode {
                direction: "horizontal".into(),
                gap: None,
                padding: None,
                id: Some(format!("command.category.{category_id}.form.toolbar")),
                children: vec![
                    UiNode::Button(UiButtonNode {
                        id: Some(format!("command-{element_key}-execute")),
                        icon_id: IconName::Check,
                        label: Label::data(shell_chrome_string("common.execute", is_de)),
                        action: ActionDescriptor { controller_id: "framework".into(), action: "executeStagedCommand".into(), args: crate::action_args_json!({ "value": key.clone() }) },
                        style: None,
                        presence: UiPresence { state: if missing { ui_wgpu::wgpu::component::ui::UiState::Disabled } else { ui_wgpu::wgpu::component::ui::UiState::Normal }, ..UiPresence::default() },
                        menu: None,
                    }),
                    UiNode::Button(UiButtonNode {
                        id: Some(format!("command-{element_key}-reset")),
                        icon_id: IconName::Undo,
                        label: Label::data(shell_chrome_string("common.reset", is_de)),
                        action: ActionDescriptor { controller_id: "framework".into(), action: "resetStagedCommand".into(), args: crate::action_args_json!({ "value": key.clone() }) },
                        style: None,
                        presence: UiPresence::default(),
                        menu: None,
                    }),
                ],
                activate: None,
                drop_action: None,
                drop_overlay: None,
                presence: UiPresence::default(),
                menu: None,
            };
            sections.push(UiTreeSectionNode {
                id: format!("command.category.{category_id}.form"),
                label: Some(Label::data(expanded.definition.label.resolve(terminology, locale))),
                default_open: None,
                header_toolbar: Some(toolbar),
                presence: UiPresence::default(),
                items: rows,
                window: None,
            });
        }
        let list: Vec<UiTreeItemNode> = entries.iter().filter(|entry| expanded_key.as_deref() != Some(command_address_stable_key(&entry.address).as_str())).map(|entry| self.build_command_panel_row(entry)).collect();
        if !list.is_empty() {
            sections.push(UiTreeSectionNode { id: "command.category.list".into(), label: None, default_open: None, header_toolbar: None, presence: UiPresence::default(), items: list, window: None });
        }
        UiNode::Tree(UiTreeNode { presentation: Default::default(), sections, presence: UiPresence::default(), drop_action: None, menu: None, interaction_domain: None })
    }

    /// 🎛️ ALL categories' command rows in one tree — the language-agnostic ORACLE the command-registry
    /// law asserts against. What the `bottom-middle` Command anchor actually publishes is
    /// {@link ShellState::build_command_category_ui}, one document per category leaf with the staged
    /// form for the expanded command; this function's value is that it enumerates every category and
    /// every row in one structure, which is exactly what a parity assertion needs and what a per-leaf
    /// body must never be.
    /// Every row for a command whose id already has a `"framework"` `dispatch_action` arm
    /// (appearance/driver/locale/terminology/themeId) is fully interactive; `os.resetDock` has no such
    /// arm to attach a plain `ActionDescriptor` to (only the ⌘️K search's `"os-command:"` string redirect
    /// can reach `apply_os_command` for it), so it renders as a pointer to command search instead of a
    /// non-functional button.
    #[cfg(test)]
    pub(crate) fn build_command_panel_ui(&self) -> UiNode {
        let resolved: Vec<_> = self.resolved_commands().into_iter().filter(|entry| entry.definition.in_palette).collect();
        let categories = command_categories(&resolved);
        let mut sections = Vec::new();
        for (category_id, category_label) in categories {
            let mut rows = Vec::new();
            rows.extend(resolved.iter().filter(|entry| entry.definition.category == category_id).map(|entry| self.build_command_panel_row(entry)));
            sections.push(UiTreeSectionNode { id: format!("shell.commands.category.{category_id}"), label: Some(Label::data(category_label)), default_open: None, header_toolbar: None, presence: UiPresence::default(), items: rows, window: None });
        }
        UiNode::Tree(UiTreeNode { presentation: Default::default(), sections, presence: UiPresence::default(), drop_action: None, menu: None, interaction_domain: None })
    }

    /// 🎛️ One `build_command_panel_ui` row: a `Select` for the four os commands whose single arg already
    /// has a "framework" `dispatch_action` arm, else a plain (non-interactive) label — see
    /// `build_command_panel_ui`'s doc comment for why `os.resetDock` and any future arg-carrying
    /// Plugin/App/Mode command fall into that last case.
    pub(crate) fn build_command_panel_row(&self, entry: &ResolvedCommand) -> UiTreeItemNode {
        let definition = &entry.definition;
        let key = command_address_stable_key(&entry.address);
        let arg_carrying = !definition.args.is_empty();
        UiTreeItemNode {
            id: format!("command.{}", key.replace(':', ".")),
            label: Label::data(if arg_carrying { format!("{}…", definition.label.resolve(self.active_terminology(), self.active_locale())) } else { definition.label.resolve(self.active_terminology(), self.active_locale()).to_string() }),
            icon_id: Some(if arg_carrying { IconName::ChevronUp } else { definition.icon_id.clone() }),
            action: Some(ActionDescriptor { controller_id: "framework".into(), action: if arg_carrying { "setCommandExpanded" } else { "executeResolvedCommand" }.into(), args: crate::action_args_json!({ "value": key }) }),
            ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
        }
    }
    // #endregion
}

/// 🎛️ One aggregated command plus its fully-qualified owner address.
#[derive(Clone, Debug)]
pub(crate) struct ResolvedCommand {
    pub definition: semio_framework::CommandDefinition,
    pub address: semio_framework::manifest::CommandAddress,
}

impl ResolvedCommand {
    pub(crate) fn new(definition: semio_framework::CommandDefinition, owner: semio_framework::manifest::CommandOwnerAddress) -> Self {
        let command_id = definition.id.clone();
        Self { definition, address: semio_framework::manifest::CommandAddress { owner, command_id } }
    }
}

pub(crate) fn command_address_stable_key(address: &semio_framework::manifest::CommandAddress) -> String {
    match &address.owner {
        semio_framework::manifest::CommandOwnerAddress::Os => format!("os:{}", address.command_id),
        semio_framework::manifest::CommandOwnerAddress::Plugin { plugin_id } => format!("plugin:{plugin_id}:{}", address.command_id),
        semio_framework::manifest::CommandOwnerAddress::App { plugin_id, app_id } => format!("app:{plugin_id}:{app_id}:{}", address.command_id),
        semio_framework::manifest::CommandOwnerAddress::Mode { plugin_id, app_id, mode_id } => format!("mode:{plugin_id}:{app_id}:{mode_id}:{}", address.command_id),
    }
}

/// 🎛️ Merges os-built-in, Plugin-scope, App-scope, and active-Mode-scope commands into one list — the
/// wgpu mirror of `os-shell.tsx`'s `resolveCommands`. A `Mode`-scope command only resolves when
/// `active_mode_id`'s `ModeDefinition.commands` references it, exactly like the React source.
pub(crate) fn resolve_commands(os_commands: Vec<semio_framework::CommandDefinition>, plugin_manifest: Option<&semio_framework::PluginManifest>, plugin_id: &str, app: &AppDefinition, active_mode_id: &str) -> Vec<ResolvedCommand> {
    let mut resolved: Vec<ResolvedCommand> = os_commands.into_iter().map(|definition| ResolvedCommand::new(definition, semio_framework::manifest::CommandOwnerAddress::Os)).collect();
    if let Some(manifest) = plugin_manifest {
        resolved.extend(manifest.commands.iter().cloned().map(|definition| ResolvedCommand::new(definition, semio_framework::manifest::CommandOwnerAddress::Plugin { plugin_id: plugin_id.to_string() })));
    }
    resolved.extend(app.commands.iter().cloned().map(|definition| ResolvedCommand::new(definition, semio_framework::manifest::CommandOwnerAddress::App { plugin_id: plugin_id.to_string(), app_id: app.id.clone() })));
    if let Some(mode) = app.modes.iter().find(|mode| mode.id == active_mode_id) {
        resolved.extend(mode.commands.iter().cloned().map(|definition| ResolvedCommand::new(definition, semio_framework::manifest::CommandOwnerAddress::Mode { plugin_id: plugin_id.to_string(), app_id: app.id.clone(), mode_id: mode.id.clone() })));
    }
    resolved
}

/// 🎛️ Loose title-case for an open-set command category id (e.g. `"appearance"` -> `"Appearance"`,
/// `"named-layout"` -> `"Named Layout"`) — the wgpu mirror of `os-shell.tsx`'s `titleizeCommandCategory`.
/// wgpu has no `ui.settings.tab.*` translation table to special-case chrome-known ids against, so every
/// category (chrome-known or app/plugin-invented) goes through this uniformly.
pub(crate) fn command_category_label(category: &str) -> String {
    category
        .split(|c: char| c == '-' || c == '_')
        .filter(|segment| !segment.is_empty())
        .map(|segment| {
            let mut chars = segment.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// 🎛️ Ordered, deduped `(category id, display label)` pairs derived from whatever commands actually
/// resolved — the wgpu mirror of `os-shell.tsx`'s `commandCategories`.
pub(crate) fn command_categories(commands: &[ResolvedCommand]) -> Vec<(String, String)> {
    let mut seen = std::collections::HashSet::new();
    let mut categories = Vec::new();
    for entry in commands {
        let category = entry.definition.category.as_str();
        if seen.insert(category.to_string()) {
            categories.push((category.to_string(), command_category_label(category)));
        }
    }
    categories
}

//#region 🔎️FuzzyRankingParity
/// 🔎️ The Rust twin of `🔨️modules/🖱️ui/🔨️modules/🔎️fuzzy-ranking/🟦️.ts` — the SAME ranking both shell
/// search surfaces use in React (`🔎️ShellSearch/🟦️.tsx`'s `UISearch`/`UIFind` both call `rankFuzzyItems`
/// with `label` ×2, `description` ×1, `category` ×0.5, threshold `0.4`, limit `20`).
///
/// It replaces this target's former hand-rolled `fuzzy_match_score` (a plain subsequence scorer with no
/// threshold, no per-field weights, no token splitting and no typo tolerance): identical queries ranked
/// differently in the two renderers, and a query matching nothing still returned every item in id order
/// rather than an empty palette. Ported branch for branch — prefix, substring, gapped-subsequence and
/// bounded Damerau–Levenshtein — so the constants below are the TypeScript constants, not new ones.
///
const FUZZY_TEXT_LIMIT: usize = 96;
pub(crate) const FUZZY_SEARCH_THRESHOLD: f64 = 0.4;
pub(crate) const FUZZY_SEARCH_LIMIT: usize = 20;

/// 🔎️ `normalizeFuzzyText` — lowercase, marks stripped, trimmed, bounded.
pub(crate) fn normalize_fuzzy_text(value: &str) -> String {
    ui_wgpu::wgpu::normalize_nfkd_text(value, false, FUZZY_TEXT_LIMIT)
}

/// 🔎️ `boundedDamerauLevenshtein` — edit distance with transpositions, abandoned past `maximum`.
fn bounded_damerau_levenshtein(left: &[char], right: &[char], maximum: usize) -> Option<usize> {
    if left.len().abs_diff(right.len()) > maximum {
        return None;
    }
    let width = right.len() + 1;
    let mut previous_previous = vec![0usize; width];
    let mut previous: Vec<usize> = (0..width).collect();
    let mut current = vec![0usize; width];
    for row in 1..=left.len() {
        current[0] = row;
        let mut row_minimum = row;
        let from = row.saturating_sub(maximum).max(1);
        let to = (row + maximum).min(right.len());
        for slot in current.iter_mut().take(from).skip(1) {
            *slot = maximum + 1;
        }
        for column in from..=to {
            let substitution = previous[column - 1] + usize::from(left[row - 1] != right[column - 1]);
            let mut distance = (previous[column] + 1).min(current[column - 1] + 1).min(substitution);
            if row > 1 && column > 1 && left[row - 1] == right[column - 2] && left[row - 2] == right[column - 1] {
                distance = distance.min(previous_previous[column - 2] + 1);
            }
            current[column] = distance;
            row_minimum = row_minimum.min(distance);
        }
        for slot in current.iter_mut().take(width).skip(to + 1) {
            *slot = maximum + 1;
        }
        if row_minimum > maximum {
            return None;
        }
        core::mem::swap(&mut previous_previous, &mut previous);
        core::mem::swap(&mut previous, &mut current);
    }
    (previous[right.len()] <= maximum).then_some(previous[right.len()])
}

/// 🔎️ `fuzzyTokenScore` — LOWER is better, `None` means "no match at all". Both already normalized.
pub(crate) fn fuzzy_token_score(query: &str, value: &str, threshold: f64) -> Option<f64> {
    if value.is_empty() {
        return None;
    }
    if value == query {
        return Some(0.0);
    }
    let query_chars: Vec<char> = query.chars().collect();
    let value_chars: Vec<char> = value.chars().collect();
    let query_len = query_chars.len() as f64;
    let value_len = value_chars.len() as f64;
    let extra_ratio = (value_len - query_len).max(0.0) / value_len.max(1.0);
    if value.starts_with(query) {
        return Some(0.04 + extra_ratio * 0.12);
    }
    if let Some(byte_index) = value.find(query) {
        let substring_index = value[..byte_index].chars().count() as f64;
        return Some(0.1 + (substring_index / value_len) * 0.12 + extra_ratio * 0.12);
    }
    let mut query_index = 0usize;
    let mut first_index: Option<usize> = None;
    let mut last_index = 0usize;
    for (value_index, candidate) in value_chars.iter().enumerate() {
        if query_index >= query_chars.len() {
            break;
        }
        if *candidate != query_chars[query_index] {
            continue;
        }
        first_index.get_or_insert(value_index);
        last_index = value_index;
        query_index += 1;
    }
    if query_index == query_chars.len() {
        let first_index = first_index.unwrap_or(0);
        let span = (last_index - first_index + 1) as f64;
        let gaps = (span - query_len).max(0.0);
        let score = 0.2 + (gaps / span.max(1.0)) * 0.18 + extra_ratio * 0.08 + (first_index as f64 / value_len) * 0.06;
        if score <= threshold {
            return Some(score);
        }
    }
    let maximum_distance = ((query_len * threshold) as usize).max(1);
    let mut best_distance: Option<usize> = None;
    for candidate in value.split(|character: char| !character.is_alphanumeric()) {
        if candidate.is_empty() {
            continue;
        }
        let candidate_chars: Vec<char> = candidate.chars().collect();
        if let Some(distance) = bounded_damerau_levenshtein(&query_chars, &candidate_chars, maximum_distance) {
            if best_distance.is_none_or(|current| distance < current) {
                best_distance = Some(distance);
            }
        }
    }
    let score = 0.18 + (best_distance? as f64 / query_len.max(1.0)) * 0.45;
    (score <= threshold).then_some(score)
}

/// 🔎️ `rankFuzzyItems` — weighted multi-field, multi-token ranking; ties break on declaration order.
/// An empty query keeps the declared order (React's `items.slice(0, limit)`), so the palette's cold list
/// is never reshuffled by the ranker.
pub(crate) fn rank_fuzzy_items<Item>(items: Vec<Item>, query: &str, fields: &[(fn(&Item) -> Option<&str>, f64)], threshold: f64, limit: usize) -> Vec<Item> {
    if limit == 0 {
        return Vec::new();
    }
    let normalized_query = normalize_fuzzy_text(query);
    if normalized_query.is_empty() {
        let mut items = items;
        items.truncate(limit);
        return items;
    }
    let tokens: Vec<String> = normalized_query.split_whitespace().map(str::to_string).collect();
    let maximum_weight = fields.iter().fold(1.0f64, |best, (_, weight)| best.max(weight.max(0.0)));
    let mut ranked: Vec<(f64, usize, Item)> = Vec::new();
    for (ref_index, item) in items.into_iter().enumerate() {
        let values: Vec<(String, f64)> = fields.iter().map(|(read, weight)| (normalize_fuzzy_text(read(&item).unwrap_or_default()), weight.max(0.0))).collect();
        let mut score = 0.0;
        let mut matched = true;
        for token in &tokens {
            let mut token_score: Option<f64> = None;
            for (value, weight) in &values {
                let Some(field_score) = fuzzy_token_score(token, value, threshold) else {
                    continue;
                };
                let weighted = field_score + ((maximum_weight - weight) / maximum_weight) * 0.08;
                if token_score.is_none_or(|best| weighted < best) {
                    token_score = Some(weighted);
                }
            }
            match token_score {
                Some(value) if value <= threshold => score += value,
                _ => {
                    matched = false;
                    break;
                }
            }
        }
        if matched {
            ranked.push((score / tokens.len() as f64, ref_index, item));
        }
    }
    ranked.sort_by(|left, right| left.0.partial_cmp(&right.0).unwrap_or(core::cmp::Ordering::Equal).then(left.1.cmp(&right.1)));
    ranked.into_iter().take(limit).map(|(_, _, item)| item).collect()
}
//#endregion 🔎️FuzzyRankingParity

//#endregion ActionPanelAndUtilities
