//! 🕹️ `shell-input` staging of `theme-file-import-and-accessibility`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;
use super::theme_file_import_and_accessibility::theme_geometry;

//#region 🎨️ThemeEditorPanel
/// 🎨️ The eight editable sections React's `buildSettingsThemeTree` carries beside its selector
/// (`📌️ChromePanels/🟦️.tsx:820-827`), in React's own order, each `(section id suffix, label key)`.
pub const SHELL_THEME_EDITOR_SECTIONS: &[(&str, &str)] = &[
    ("colors", "settings.theme.colors"),
    ("spacing", "settings.theme.spacing"),
    ("fonts", "settings.theme.fonts"),
    ("strokes", "settings.theme.strokes"),
    ("radii", "settings.theme.radii"),
    ("opacities", "settings.theme.opacities"),
    ("metrics", "settings.theme.metrics"),
    ("appearances", "settings.theme.appearances"),
];

pub const SHELL_THEME_APPEARANCE_GROUPS: &[(&str, &str)] = &[
    ("board", "settings.theme.group.board"),
    ("map", "settings.theme.group.map"),
    ("canvas", "settings.theme.group.canvas"),
    ("chrome", "settings.theme.group.chrome"),
    ("outcome", "settings.theme.group.outcome"),
    ("diagram", "settings.theme.group.diagram"),
];

pub const SHELL_THEME_EDITOR_SCROLL_ID: &str = "tree:framework.settings.theme.select";

const SHELL_THEME_EDITOR_VIEWPORT_ROWS: usize = 28;

pub const SHELL_THEME_EDITOR_ROW_HEIGHT: f32 = (ui_styling::metrics::chrome::UI_SPACING_COMPACT_PX * ui_styling::metrics::dom::TREE_ROW_UI_SPACING) as f32;

fn theme_editor_input_item(id: &str, label: &str, value: &str, action: &str, args: Option<DslValue>) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.to_string(),
        label: Label::data(label),
        control: Some(UiControlNode::Input(UiInputNode {
            id: id.to_string(),
            input_kind: "text".into(),
            value: value.to_string(),
            placeholder: None,
            accessibility_label: None,
            commit: Some("blur".into()),
            min: None,
            max: None,
            step: None,
            accept: None,
            precision: None,
            snaps: Vec::new(),
            on_change: ActionDescriptor { controller_id: "framework".into(), action: action.into(), args },
            on_submit: None,
            on_abort: None,
            on_repeat_last: None,
            presence: UiPresence::default(),
            menu: None,
            ..Default::default()
        })),
        ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
    }
}

fn theme_editor_control_item(id: &str, label: &str, control: UiControlNode) -> UiTreeItemNode {
    UiTreeItemNode { id: id.to_string(), label: Label::data(label), control: Some(control), ..UiTreeItemNode::base(String::new(), Label::data(String::new())) }
}

impl ShellInputState {
    pub fn theme_section_open(&self, id: &str, default_open: bool) -> bool {
        self.canonical_tree_open(id, default_open)
    }

    pub fn theme_item_open(&self, id: &str) -> bool {
        self.canonical_tree_open(id, false)
    }

    pub fn theme_visible_rows(&self) -> std::ops::Range<usize> {
        let offset = self.scroll_offsets.get(SHELL_THEME_EDITOR_SCROLL_ID).copied().unwrap_or_default().max(0.0);
        let start = (offset / SHELL_THEME_EDITOR_ROW_HEIGHT).floor() as usize;
        start.saturating_sub(2)..start.saturating_add(SHELL_THEME_EDITOR_VIEWPORT_ROWS)
    }

    pub fn theme_window_rows(&self, rows: Vec<UiTreeItemNode>, cursor: &mut usize, visible: &std::ops::Range<usize>) -> (Vec<UiTreeItemNode>, Option<UiTreeWindow>) {
        let total = rows.len();
        let first = visible.start.saturating_sub(*cursor).min(total);
        let last = visible.end.saturating_sub(*cursor).min(total).max(first);
        *cursor += total;
        let materialized: Vec<_> = rows.into_iter().skip(first).take(last - first).collect();
        let window = (materialized.len() != total).then_some(UiTreeWindow { row_extent: UiTreeWindowRowExtent::Standard, total: total as u32, offset: first as u32 });
        (materialized, window)
    }

    pub fn theme_leaf_section(&self, id: &str, label: &str, rows: Vec<UiTreeItemNode>, cursor: &mut usize, visible: &std::ops::Range<usize>) -> UiTreeSectionNode {
        *cursor += 1;
        let open = self.theme_section_open(id, false);
        let total = rows.len();
        let (items, window) = if open { self.theme_window_rows(rows, cursor, visible) } else { (Vec::new(), Some(UiTreeWindow { row_extent: UiTreeWindowRowExtent::Standard, total: total as u32, offset: 0 })) };
        UiTreeSectionNode { header_toolbar: None, id: id.to_string(), label: Some(Label::data(label)), default_open: Some(open), presence: UiPresence::default(), items, window }
    }

    pub fn build_settings_theme_editor_tree(&self, document: &ThemeDocument, is_de: bool, chrome: &ChromeState, prefs: &PrefsState) -> UiNode {
        let active_id = chrome.chrome_build.preferences.theme_id.clone();
        let mut themes = vec![UiSelectItem { value: "semio".into(), label: Label::data("Semio") }, UiSelectItem { value: "mono".into(), label: Label::data("Mono") }];
        for id in chrome.chrome_build.preferences.custom_themes.keys().cloned() {
            let label = custom_theme_document_from(&chrome.chrome_build.preferences, &id).map(|theme| theme.label).unwrap_or_else(|| id.clone());
            themes.push(UiSelectItem { value: id, label: Label::data(label) });
        }
        let mut selector = vec![theme_editor_control_item(
            "framework.settings.theme.select.picker",
            shell_chrome_string("settings.theme.select", is_de),
            UiControlNode::Select(UiSelectNode {
                presence: UiPresence::default(),
                id: "framework.settings.theme.select".into(),
                value: active_id.clone(),
                items: themes,
                placeholder: None,
                on_change: ActionDescriptor { controller_id: "framework".into(), action: "setThemeId".into(), args: None },
                menu: None,
            }),
        )];
        selector.push(theme_editor_input_item("framework.settings.theme.saveLabel", shell_chrome_string("common.name", is_de), &prefs.theme_save_label, "setThemeSaveLabel", None));
        let button = |id: &str, label: &'static str, icon_id: IconName, action: &str, args: Option<DslValue>, disabled: bool| {
            theme_editor_control_item(
                &format!("{id}.action"),
                shell_chrome_string(label, is_de),
                UiControlNode::Button(UiButtonNode {
                    id: Some(id.to_string()),
                    icon_id,
                    label: Label::data(shell_chrome_string(label, is_de)),
                    action: ActionDescriptor { controller_id: "framework".into(), action: action.into(), args },
                    style: None,
                    presence: UiPresence { state: if disabled { ui_wgpu::wgpu::component::ui::UiState::Disabled } else { ui_wgpu::wgpu::component::ui::UiState::Normal }, ..UiPresence::default() },
                    menu: None,
                }),
            )
        };
        selector.push(button("framework.settings.theme.save", "settings.theme.save", IconName::Save, "saveTheme", None, prefs.theme_save_label.trim().is_empty()));
        selector.push(button("framework.settings.theme.reset", "settings.theme.reset", IconName::RotateCcw, "resetThemeId", None, prefs.theme_draft.is_none() && active_id == "semio"));
        selector.push(button("framework.settings.theme.export", "settings.theme.export", IconName::Export, "exportTheme", None, false));
        selector.push(button("framework.settings.theme.import", "settings.theme.import", IconName::Import, "importTheme", None, false));
        if active_id.starts_with("custom.") {
            selector.push(button("framework.settings.theme.delete", "settings.theme.delete", IconName::Trash2, "deleteThemeId", semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "value": active_id }), false));
        }
        let visible = self.theme_visible_rows();
        let select_open = self.theme_section_open("framework.settings.theme.select", true);
        let mut cursor = 1 + usize::from(select_open) * selector.len();
        let mut sections = vec![UiTreeSectionNode {
            header_toolbar: None,
            id: "framework.settings.theme.select".into(),
            label: Some(Label::data(shell_chrome_string("settings.theme.select", is_de))),
            default_open: Some(select_open),
            presence: UiPresence::default(),
            items: if select_open { selector } else { Vec::new() },
            window: None,
        }];
        let leaf_rows = |suffix: &str| -> Vec<UiTreeItemNode> {
            let id = format!("framework.settings.theme.{suffix}");
            match suffix {
                "colors" => document.colors.iter().map(|(key, value)| theme_editor_input_item(&format!("{id}.{key}"), key, value, "setThemeColor", semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "key": key }))).collect(),
                "spacing" => document.spacing.iter().map(|(key, value)| {
                    let mut item = theme_editor_input_item(&format!("{id}.{key}"), key, value, "setThemeSpacing", semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "key": key }));
                    if key == "compact" {
                        let ui = &theme_geometry::contract()["properties"]["compact"]["x-semio-ui"];
                        let locale = if is_de { "de" } else { "en" };
                        let label = ui["label"][locale].as_str().expect("localized compact label");
                        item.label = Label::data(label);
                        item.description = Some(ui["description"][locale].as_str().expect("localized compact description").into());
                        if let Some(UiControlNode::Input(input)) = item.control.as_mut() { input.accessibility_label = Some(Label::data(label)); }
                    }
                    item
                }).collect(),
                "fonts" => document.font_stacks.iter().map(|(key, value)| theme_editor_input_item(&format!("{id}.{key}"), key, value, "setThemeFontStack", semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "key": key }))).collect(),
                "strokes" => document.strokes.iter().map(|(key, value)| theme_editor_input_item(&format!("{id}.{key}"), key, &value.as_text(), "setThemeStroke", semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "key": key }))).collect(),
                "radii" => document.radii.iter().map(|(key, value)| theme_editor_input_item(&format!("{id}.{key}"), key, &value.as_text(), "setThemeRadius", semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "key": key }))).collect(),
                "opacities" => document.opacities.iter().map(|(key, value)| theme_editor_input_item(&format!("{id}.{key}"), key, &value.as_text(), "setThemeOpacity", semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "key": key }))).collect(),
                _ => Vec::new(),
            }
        };
        for (suffix, label_key) in SHELL_THEME_EDITOR_SECTIONS.iter().take(6) {
            let id = format!("framework.settings.theme.{suffix}");
            sections.push(self.theme_leaf_section(&id, shell_chrome_string(*label_key, is_de), leaf_rows(suffix), &mut cursor, &visible));
        }
        sections.push(self.theme_metrics_section(document, is_de, &mut cursor, &visible));
        sections.push(self.theme_appearances_section(document, is_de, &mut cursor, &visible));
        UiNode::Tree(UiTreeNode { presentation: Default::default(), sections, presence: UiPresence::default(), drop_action: None, menu: None, interaction_domain: None })
    }

    pub fn theme_metrics_section(&self, document: &ThemeDocument, is_de: bool, cursor: &mut usize, visible: &std::ops::Range<usize>) -> UiTreeSectionNode {
        let id = "framework.settings.theme.metrics";
        *cursor += 1;
        let open = self.theme_section_open(id, false);
        let mut items = Vec::new();
        if open {
            for (section, entries) in &document.metrics {
                let section_id = format!("{id}.{section}");
                *cursor += 1;
                let group_open = self.theme_item_open(&section_id);
                let rows = entries.iter().map(|(key, value)| {
                    let mut item = theme_editor_input_item(&format!("{section_id}.{key}"), key, &value.as_text(), "setThemeMetric", semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "section": section, "key": key }));
                    let guidance = if section == "dom" && key == "rootRemPx" { Some("positiveRoot") }
                        else if theme_geometry::bindings()["bindings"].as_array().is_some_and(|bindings| bindings.iter().any(|binding| binding["section"].as_str() == Some(section.as_str()) && binding["key"].as_str() == Some(key.as_str()))) { Some("nonnegativeScalar") }
                        else { None };
                    if let Some(guidance) = guidance {
                        item.description = theme_geometry::contract()["x-semio-resolution"]["metricUi"][guidance][if is_de { "de" } else { "en" }].as_str().map(str::to_string);
                        if let Some(UiControlNode::Input(input)) = item.control.as_mut() { input.accessibility_label = Some(Label::data(key)); }
                    }
                    item
                }).collect();
                let total = entries.len();
                let (children, window) = if group_open { self.theme_window_rows(rows, cursor, visible) } else { (Vec::new(), Some(UiTreeWindow { row_extent: UiTreeWindowRowExtent::Standard, total: total as u32, offset: 0 })) };
                items.push(UiTreeItemNode { id: section_id, label: Label::data(section), default_open: Some(group_open), items: Some(children), window, ..UiTreeItemNode::base(String::new(), Label::data(String::new())) });
            }
        }
        UiTreeSectionNode { header_toolbar: None, id: id.into(), label: Some(Label::data(shell_chrome_string("settings.theme.metrics", is_de))), default_open: Some(open), presence: UiPresence::default(), items, window: None }
    }

    pub fn theme_appearances_section(&self, document: &ThemeDocument, is_de: bool, cursor: &mut usize, visible: &std::ops::Range<usize>) -> UiTreeSectionNode {
        let id = "framework.settings.theme.appearances";
        *cursor += 1;
        let open = self.theme_section_open(id, false);
        let mut appearances = Vec::new();
        if open {
            for appearance in ["light", "dark"] {
                let appearance_id = format!("{id}.{appearance}");
                *cursor += 1;
                let appearance_open = self.theme_item_open(&appearance_id);
                let mut groups = Vec::new();
                if appearance_open {
                    for (group, label_key) in SHELL_THEME_APPEARANCE_GROUPS {
                        let group_id = format!("{appearance_id}.{group}");
                        *cursor += 1;
                        let group_open = self.theme_item_open(&group_id);
                        let mut rows = Vec::new();
                        for (paint, reference) in document.appearances.get(appearance).and_then(|entries| entries.get(*group)).into_iter().flatten() {
                            let (hex, alpha) = theme_paint_editor_values(&document.colors, reference);
                            rows.push(theme_editor_input_item(&format!("{group_id}.{paint}"), paint, &hex, "setThemeAppearancePaint", semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "appearance": appearance, "group": group, "paint": paint, "channel": "hex" })));
                            rows.push(theme_editor_input_item(
                                &format!("{group_id}.{paint}.alpha"),
                                paint,
                                &format!("{alpha:.2}"),
                                "setThemeAppearancePaint",
                                semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "appearance": appearance, "group": group, "paint": paint, "channel": "alpha" }),
                            ));
                        }
                        let total = rows.len();
                        let (children, window) = if group_open { self.theme_window_rows(rows, cursor, visible) } else { (Vec::new(), Some(UiTreeWindow { row_extent: UiTreeWindowRowExtent::Standard, total: total as u32, offset: 0 })) };
                        groups.push(UiTreeItemNode {
                            id: group_id,
                            label: Label::data(shell_chrome_string(*label_key, is_de)),
                            default_open: Some(group_open),
                            items: Some(children),
                            window,
                            ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
                        });
                    }
                }
                let label = shell_chrome_string(if appearance == "light" { "settings.theme.appearance.light" } else { "settings.theme.appearance.dark" }, is_de);
                appearances.push(UiTreeItemNode { id: appearance_id, label: Label::data(label), default_open: Some(appearance_open), items: Some(groups), ..UiTreeItemNode::base(String::new(), Label::data(String::new())) });
            }
        }
        UiTreeSectionNode { header_toolbar: None, id: id.into(), label: Some(Label::data(shell_chrome_string("settings.theme.appearances", is_de))), default_open: Some(open), presence: UiPresence::default(), items: appearances, window: None }
    }
}

//#endregion 🎨️ThemeEditorPanel

//#region ♿️ChromeAccessibleNames
/// 🚫️ Records that a painted chrome control is disabled (`aria-disabled`) — a band control a session refusal blocks.
pub fn note_chrome_control_disabled(control_id: &str, disabled: bool) {
    with_chrome_control_names(|names| {
        if let Some(presentation) = names.get_mut(control_id) {
            presentation.disabled = disabled;
        }
    });
}

/// ♿️ Records the consequence text a reader hears with one chrome control (`aria-describedby`).
pub fn note_chrome_control_description(control_id: &str, description: Option<&str>) {
    with_chrome_control_names(|names| {
        if let Some(presentation) = names.get_mut(control_id) {
            presentation.description = description.map(str::to_string);
        }
    });
}

/// ♿️ Records one chrome control's painted label. Called from the two painters every chrome chip goes
/// through, so a new chip is announced without its author doing anything.
pub fn note_chrome_control_name(control_id: &str, label: Option<&str>) {
    let (Some(label), false) = (label, control_id.is_empty()) else { return };
    with_chrome_control_names(|names| {
        if names.len() < SHELL_CHROME_ACCESSIBLE_NAME_CAPACITY || names.contains_key(control_id) {
            names.insert(control_id.to_string(), ChromeControlPresentation { label: label.to_string(), description: None, role: None, pressed: None, selected: None, controls: None, value_text: None, value_range: None, disabled: false });
        }
    });
}

pub fn with_chrome_control_names<R>(f: impl FnOnce(&mut BTreeMap<String, ChromeControlPresentation>) -> R) -> R {
    #[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
    {
        CHROME_CONTROL_NAMES.with(|cell| f(&mut cell.borrow_mut()))
    }
    #[cfg(not(any(target_arch = "wasm32", any(test, feature = "testkit"))))]
    {
        f(&mut CHROME_CONTROL_NAMES.borrow_mut())
    }
}

#[cfg(not(any(target_arch = "wasm32", any(test, feature = "testkit"))))]
pub static CHROME_CONTROL_NAMES: semio_framework_os_renderer_wgpu_contracts::WorkerCell<BTreeMap<String, ChromeControlPresentation>> = semio_framework_os_renderer_wgpu_contracts::WorkerCell::new(Default::default);

/// ♿️ Semantic properties captured beside the chrome label during its paint walk.
pub struct ChromeControlPresentation {
    pub label: String,
    pub description: Option<String>,
    pub role: Option<&'static str>,
    pub pressed: Option<bool>,
    pub selected: Option<bool>,
    pub controls: Option<String>,
    pub value_text: Option<String>,
    pub value_range: Option<[f64; 3]>,
    pub disabled: bool,
}

/// ♿️ The accessible NAME each chrome control painted this walk, keyed by control id.
///
/// 🩸️ A GPU canvas has no elements, so the only accessible names this target ever published came out
/// of the retained DOCUMENT projection (`dumpAccessibility` over `UI_ENGINE`) — the scene trees. Every
/// navbar chip, footer pill, panel tab, pane chip and surface control was invisible to a reader
/// (audit W14 §C4), even though React gives each of them an `sr-only` label for free. The two central
/// chrome item painters record the label they are ABOUT to paint here, so the published name is
/// always the text on screen rather than a second, drifting table.
///
/// ⛓️ Staged per walk and promoted by `publish_retained_hit_registry` together with the hit registry,
/// so names and targets can never describe different frames — the same double-buffer discipline
/// `retained_hit_windows` follows. Bounded by [`SHELL_CHROME_ACCESSIBLE_NAME_CAPACITY`].
pub const SHELL_CHROME_ACCESSIBLE_NAME_CAPACITY: usize = 512;

/// 🪡️ Per thread on wasm and in tests — each test owns its shell, so a concurrent test's frame setup can never
/// clear the names another test is about to publish — and one worker-wide cell natively.
#[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
thread_local! {
    pub static CHROME_CONTROL_NAMES: std::cell::RefCell<BTreeMap<String, ChromeControlPresentation>> = std::cell::RefCell::new(BTreeMap::new());
}
//#endregion ♿️ChromeAccessibleNames
