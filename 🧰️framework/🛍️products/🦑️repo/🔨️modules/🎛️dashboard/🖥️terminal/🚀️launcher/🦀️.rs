//! 🚀️ The launcher model: a tree of the registry's commands filed by verb with incremental search,
//! then a form for the parameters the chosen command accepts, then a confirmation for commands that
//! change the repository. It is a pure state machine over abstract inputs and the registry; nothing
//! here knows a terminal, so every implementation drives it with the same journeys.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🎮️registry/🦀️.rs
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧫️fixtures/🚀️launcher/🔣️.json

use super::labels::{labels, DashboardLabels};
use crate::preferences::Preferences;
use crate::registry::{split_arguments, verb_text, Entry, Launch, Parameter, ParameterKind, Registry, Request};
use std::collections::HashMap;
use std::sync::Arc;
use ui_locale::Locale;
use ui_tui::tui::event::{mods, Key, KeyEvent};
use ui_tui::tui::widget::{TreeItem, TreeState, WidgetSignal, WidgetState};

// #region 🔖️Model
/// 🪜️ How far the developer has come with one command.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage { Browse, Configure, Confirm }

/// ⌨️ What the developer did, independent of which key did it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Input { Char(char), Backspace, Clear, DeleteWord, First, Last, Up, Down, Left, Right, PageUp, PageDown, Activate, Back }

/// ▶️ A confirmed request to start a command with the chosen parameters.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Start { pub id: String, pub chosen: Vec<(String, String)>, pub extra: Vec<String>, pub env: Vec<(String, String)> }

impl Start {
    /// 🧾️ The registry request this start stands for.
    pub fn request(&self) -> Request { Request { parameters: self.chosen.clone(), args: self.extra.clone(), env: self.env.clone() } }
}

/// 📣️ What an input asks of the view.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Outcome { Idle, Leave, Start(Start) }

/// 🖼️ Everything the view paints for the launcher: the text field, a caption and the visible lines.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Screen { pub input: String, pub placeholder: String, pub caption: String, pub items: Vec<String>, pub selected: usize }

#[derive(Clone, Debug)]
struct Node { key: String, label: String, depth: u16, entry: Option<usize>, children: usize }

#[derive(Clone, Debug)]
struct Field { parameter: Parameter, value: Option<String> }

#[derive(Clone, Debug)]
struct Form { entry: usize, id: String, mutating: bool, fields: Vec<Field>, extra_args: String, extra_env: String, focus: usize }

impl Form {
    fn args_slot(&self) -> usize { self.fields.len() }
    fn env_slot(&self) -> usize { self.fields.len() + 1 }
    fn start_slot(&self) -> usize { self.fields.len() + 2 }

    fn effective(field: &Field) -> Option<&str> { field.value.as_deref().or(field.parameter.default.as_deref()) }

    fn start(&self) -> Result<Start, String> {
        let extra = split_arguments(&self.extra_args)?;
        let env = parse_environment(&self.extra_env)?;
        let chosen = self.fields.iter().filter_map(|field| {
            let parameter = &field.parameter;
            match parameter.kind {
                ParameterKind::Flag => { let on = Self::effective(field) == Some("true"); (on != (parameter.default.as_deref() == Some("true"))).then(|| (parameter.id.clone(), on.to_string())) }
                _ => field.value.as_deref().filter(|value| Some(*value) != parameter.default.as_deref()).map(|value| (parameter.id.clone(), value.to_string())),
            }
        }).collect();
        Ok(Start { id: self.id.clone(), chosen, extra, env })
    }
}

/// ✂️ The text without its last word.
fn drop_word(text: &str) -> String { text.trim_end().rsplit_once(char::is_whitespace).map_or_else(String::new, |(head, _)| head.trim_end().to_string()) }

fn parse_environment(text: &str) -> Result<Vec<(String, String)>, String> {
    split_arguments(text)?.into_iter().map(|word| {
        let (name, value) = word.split_once('=').ok_or_else(|| format!("{word:?} is not KEY=value"))?;
        if name.is_empty() || !name.bytes().all(|byte| byte.is_ascii_alphanumeric() || byte == b'_') || name.as_bytes()[0].is_ascii_digit() { return Err(format!("{name:?} is not a variable name")); }
        Ok((name.to_string(), value.to_string()))
    }).collect()
}

/// 🛰️ The launcher of one window.
pub struct LauncherState {
    registry: Option<Arc<Registry>>,
    preferences: Preferences,
    locale: Locale,
    nodes: Vec<Node>,
    offset: usize,
    page: usize,
    stage: Stage,
    form: Option<Form>,
}
// #endregion 🔖️Model

// #region 🔖️Tree
fn build_nodes(registry: &Registry, locale: Locale) -> Vec<Node> {
    let mut nodes: Vec<Node> = Vec::new();
    let mut open: Vec<usize> = Vec::new();
    for (position, entry) in registry.entries().iter().enumerate().filter(|(_, entry)| entry.listed) {
        let mut groups: Vec<(String, String)> = vec![(format!("verb:{}", entry.verb), verb_text(&entry.verb, locale).to_string())];
        let (leaf, parents) = entry.path.split_last().map_or((entry.verb.clone(), &[][..]), |(last, rest)| (last.label.clone(), rest));
        groups.extend(parents.iter().map(|segment| (segment.key.clone(), segment.label.clone())));
        let key_of = |depth: usize| groups[..=depth].iter().map(|(key, _)| key.as_str()).collect::<Vec<_>>().join("/");
        let mut keep = 0;
        while keep < open.len() && keep < groups.len() && nodes[open[keep]].key == key_of(keep) { keep += 1; }
        open.truncate(keep);
        for (depth, (_, label)) in groups.iter().enumerate().skip(keep) {
            let parent = open.last().copied();
            nodes.push(Node { key: key_of(depth), label: label.clone(), depth: depth as u16, entry: None, children: 0 });
            if let Some(parent) = parent { nodes[parent].children += 1; }
            open.push(nodes.len() - 1);
        }
        let parent = open.last().copied();
        nodes.push(Node { key: format!("entry:{}", entry.id), label: leaf, depth: groups.len() as u16, entry: Some(position), children: 0 });
        if let Some(parent) = parent { nodes[parent].children += 1; }
    }
    nodes
}
// #endregion 🔖️Tree

impl LauncherState {
    /// 🌱️ A launcher without commands; they arrive with [`LauncherState::set_registry`].
    pub fn new(preferences: Preferences, locale: Locale) -> Self {
        Self { registry: None, preferences, locale, nodes: Vec::new(), offset: 0, page: 10, stage: Stage::Browse, form: None }
    }

    pub fn stage(&self) -> Stage { self.stage }

    /// 📚️ Adopts a newer registry; the view rebuilds its tree with [`LauncherState::rebuild_tree`].
    pub fn set_registry(&mut self, registry: Arc<Registry>) {
        self.nodes = build_nodes(&registry, self.locale);
        self.registry = Some(registry);
    }

    /// 🌐️ Re-files the tree under the verbs of another language.
    pub fn set_locale(&mut self, locale: Locale) {
        if self.locale == locale { return; }
        self.locale = locale;
        if let Some(registry) = self.registry.clone() { self.set_registry(registry); }
    }

    /// ⚙️ Adopts changed preferences, which pre-select the parameters of forms opened afterwards.
    pub fn set_preferences(&mut self, preferences: Preferences) { self.preferences = preferences; }

    /// 📏️ States how many lines of the form fit, so its focus keeps itself in view.
    pub fn set_page(&mut self, page: usize) {
        self.page = page.max(1);
        self.reveal();
    }

    /// 🌳️ Every node of the registry's tree as a flat pre-order forest; a command's label carries its id so the
    /// tree's own filter finds it by id as well.
    pub fn tree_items(&self) -> Vec<TreeItem> {
        let entries = self.registry.as_ref().map(|registry| registry.entries()).unwrap_or_default();
        self.nodes.iter().map(|node| match node.entry.and_then(|entry| entries.get(entry)) {
            Some(entry) => TreeItem::new(node.key.clone(), format!("{}{}   {}", node.label, if entry.mutating { " ⚠" } else { "" }, entry.id), node.depth),
            None => TreeItem::new(node.key.clone(), node.label.clone(), node.depth),
        }).collect()
    }

    /// 🍃️ A tree of the current commands that keeps the open folders, the filter and the selection of `old`.
    pub fn rebuild_tree(&self, old: Option<&TreeState>) -> TreeState {
        let mut tree = TreeState::new(self.tree_items());
        tree.empty = labels(self.locale).launcher_empty.as_str().to_string();
        let Some(old) = old else { return tree };
        let at: HashMap<String, usize> = (0..tree.len()).filter_map(|item| tree.item(item).map(|found| (found.id.clone(), item))).collect();
        let named = |item: usize| old.item(item).and_then(|found| at.get(&found.id)).copied();
        for item in (0..old.len()).filter(|item| old.is_folder(*item) && old.is_expanded(*item)) { if let Some(found) = named(item) { tree.set_expanded(found, true); } }
        if !old.query().is_empty() { tree.set_query(old.query()); }
        if let Some(found) = old.selected_item().and_then(named) { tree.select_item(found); }
        tree
    }

    fn entry_of(&self, item: usize) -> Option<usize> { self.nodes.get(item).and_then(|node| node.entry) }

    /// 🔎️ The ids of the commands the tree shows now, in tree order.
    pub fn visible_ids(&self, tree: &TreeState) -> Vec<String> {
        let entries = self.registry.as_ref().map(|registry| registry.entries()).unwrap_or_default();
        (0..tree.count()).filter_map(|row| tree.row_item(row)).filter_map(|item| self.entry_of(item)).filter_map(|entry| entries.get(entry)).map(|entry| entry.id.clone()).collect()
    }

    /// 📜️ The rows of the tree as plain text, indented, for readers that cannot see the screen.
    pub fn rows(&self, tree: &TreeState) -> Vec<String> {
        (0..tree.count()).filter_map(|row| tree.row_item(row)).filter_map(|item| tree.item(item).map(|found| {
            let marker = if !tree.is_folder(item) { "  " } else if tree.is_expanded(item) || !tree.query().is_empty() { "▾ " } else { "▸ " };
            format!("{}{marker}{}", "  ".repeat(usize::from(found.depth)), found.label)
        })).collect()
    }

    /// 🏷️ The line above the tree: how many commands exist, or how many match the filter.
    pub fn caption(&self, tree: &TreeState) -> String {
        let text = labels(self.locale);
        match &self.registry {
            None => text.launcher_loading.as_str().to_string(),
            Some(_) if tree.count() == 0 => text.launcher_empty.as_str().to_string(),
            Some(registry) => {
                let total = registry.entries().iter().filter(|entry| entry.listed).count().to_string();
                let counts = if tree.query().is_empty() { text.state_commands.fill(&[("count", &total)]).into_string() } else { text.launcher_counts.fill(&[("shown", &self.leaf_count(tree).to_string()), ("total", &total)]).into_string() };
                match self.preview_of(registry, tree) { Some(preview) => format!("{counts} · {preview}"), None => counts }
            }
        }
    }

    /// 👁️ What the selected command would run with its defaults: the command line, or its id when it needs input first.
    fn preview_of(&self, registry: &Registry, tree: &TreeState) -> Option<String> {
        let entry = registry.entries().get(self.entry_of(tree.selected_item()?)?)?;
        if entry.repo_action().is_some() { return Some(entry.id.clone()); }
        Some(match registry.resolve(&entry.id, &Request::default()) {
            Ok(launch) => launch.processes.first().map_or_else(|| entry.id.clone(), |process| format!("{} {}", process.cmd, process.args.join(" "))),
            Err(_) => entry.id.clone(),
        })
    }

    fn leaf_count(&self, tree: &TreeState) -> usize { (0..tree.count()).filter_map(|row| tree.row_item(row)).filter(|item| self.entry_of(*item).is_some()).count() }

    fn snap_to_command(tree: &mut TreeState) {
        if tree.query().is_empty() { return; }
        let Some(current) = tree.selected_item() else { return };
        if !tree.is_folder(current) { return; }
        if let Some(leaf) = (current + 1..tree.len()).find(|item| !tree.is_folder(*item) && tree.position_of(*item).is_some()) { tree.select_item(leaf); }
    }

    fn reveal(&mut self) {
        let line = match (self.stage, self.form.as_ref()) { (Stage::Configure, Some(form)) => Self::slot_lines(form, labels(self.locale), self.locale)[form.focus], _ => 0 };
        if line < self.offset { self.offset = line; } else if line >= self.offset + self.page { self.offset = line + 1 - self.page; }
    }

    /// 🎮️ Applies one input and says what the view must do next. While browsing, `widget` is the tree.
    pub fn apply(&mut self, widget: &mut WidgetState, input: Input) -> Outcome {
        let outcome = match self.stage {
            Stage::Browse => self.browse(widget, input),
            Stage::Configure => self.configure(input),
            Stage::Confirm => match input {
                Input::Activate => self.form.as_ref().and_then(|form| form.start().ok()).map_or(Outcome::Idle, Outcome::Start),
                Input::Back => { self.stage = Stage::Configure; Outcome::Idle }
                _ => Outcome::Idle,
            },
        };
        if self.stage == Stage::Confirm { self.offset = 0; }
        self.reveal();
        outcome
    }

    /// 🖱️ Selects the line `item` of the form; a second click on the start line starts.
    pub fn click(&mut self, widget: &mut WidgetState, item: usize) -> Outcome {
        if self.stage == Stage::Confirm {
            let start = self.form.as_ref().map(|form| Self::slot_lines(form, labels(self.locale), self.locale)[form.start_slot()] + 1);
            return if start == Some(self.offset + item) { self.apply(widget, Input::Activate) } else { Outcome::Idle };
        }
        if self.stage != Stage::Configure { return Outcome::Idle; }
        let line = self.offset + item;
        let Some(form) = self.form.as_ref() else { return Outcome::Idle };
        let slot = Self::slot_lines(form, labels(self.locale), self.locale).iter().position(|candidate| *candidate == line);
        let start = form.start_slot();
        match (slot, self.form.as_mut()) {
            (Some(slot), Some(form)) if slot == start => { form.focus = slot; self.apply(widget, Input::Activate) },
            (Some(slot), Some(form)) => { form.focus = slot; self.reveal(); Outcome::Idle }
            _ => Outcome::Idle,
        }
    }

    /// 🌿️ A command of the tree was activated (key or double click).
    pub fn activated(&mut self, item: usize) -> Outcome {
        if self.stage == Stage::Browse { if let Some(entry) = self.entry_of(item) { self.open_form(entry); } }
        Outcome::Idle
    }

    fn browse(&mut self, widget: &mut WidgetState, input: Input) -> Outcome {
        let WidgetState::Tree(tree) = widget else { return Outcome::Idle };
        let key = |key: Key, held: u8| KeyEvent { key, mods: held };
        let event = match input {
            Input::Char(c) => key(Key::Char(c), 0),
            Input::Backspace => key(Key::Backspace, 0),
            Input::Clear => key(Key::Char('u'), mods::CTRL),
            Input::First => key(Key::Home, 0),
            Input::Last => key(Key::End, 0),
            Input::DeleteWord => {
                let shorter = drop_word(tree.query());
                tree.set_query(&shorter);
                Self::snap_to_command(tree);
                return Outcome::Idle;
            }
            Input::Up => key(Key::Up, 0),
            Input::Down => key(Key::Down, 0),
            Input::Left => key(Key::Left, 0),
            Input::Right => key(Key::Right, 0),
            Input::PageUp => key(Key::PageUp, 0),
            Input::PageDown => key(Key::PageDown, 0),
            Input::Activate => key(Key::Enter, 0),
            Input::Back if tree.query().is_empty() => return Outcome::Leave,
            Input::Back => key(Key::Esc, 0),
        };
        match widget.on_key(&event) {
            Some(WidgetSignal::Activated(item)) => self.activated(item),
            Some(WidgetSignal::ValueChanged(_)) => { if let WidgetState::Tree(tree) = widget { Self::snap_to_command(tree); } Outcome::Idle }
            _ => Outcome::Idle,
        }
    }

    fn open_form(&mut self, entry: usize) {
        let Some(registry) = self.registry.clone() else { return };
        let Some(command) = registry.entries().get(entry) else { return };
        let parameters = registry.parameters(command);
        let preferred = self.preferences.launch_defaults(&parameters);
        let fields = parameters.into_iter().map(|parameter| { let value = preferred.iter().find(|(id, _)| *id == parameter.id).map(|(_, value)| value.clone()); Field { parameter, value } }).collect();
        self.form = Some(Form { entry, id: command.id.clone(), mutating: command.mutating, fields, extra_args: String::new(), extra_env: String::new(), focus: 0 });
        if let Some(form) = self.form.as_mut() { form.focus = form.fields.iter().position(|field| field.parameter.required).unwrap_or_else(|| form.start_slot()); }
        self.stage = Stage::Configure;
        self.offset = 0;
    }

    fn configure(&mut self, input: Input) -> Outcome {
        let Some(form) = self.form.as_mut() else { self.stage = Stage::Browse; return Outcome::Idle };
        let last = form.start_slot();
        match input {
            Input::Up => form.focus = form.focus.saturating_sub(1),
            Input::Down => form.focus = (form.focus + 1).min(last),
            Input::PageUp => form.focus = 0,
            Input::PageDown => form.focus = last,
            Input::Left => Self::cycle(form, false),
            Input::Right => Self::cycle(form, true),
            Input::Char(' ') if form.focus < form.fields.len() && form.fields[form.focus].parameter.kind != ParameterKind::Text => Self::cycle(form, true),
            Input::Char(c) => { if let Some(text) = Self::text(form) { text.push(c); } }
            Input::Backspace => { if let Some(text) = Self::text(form) { text.pop(); } }
            Input::Clear => { if let Some(text) = Self::text(form) { text.clear(); } }
            Input::DeleteWord => { if let Some(text) = Self::text(form) { *text = drop_word(text); } }
            Input::First => form.focus = 0,
            Input::Last => form.focus = last,
            Input::Back => { self.form = None; self.stage = Stage::Browse; self.offset = 0; return Outcome::Idle; }
            Input::Activate => {
                let Some(registry) = self.registry.clone() else { return Outcome::Idle };
                let start = match form.start() { Ok(start) => start, Err(_) => return Outcome::Idle };
                if registry.resolve(&start.id, &start.request()).is_err() { return Outcome::Idle; }
                if form.mutating { self.stage = Stage::Confirm; return Outcome::Idle; }
                return Outcome::Start(start);
            }
        }
        Self::normalize(form);
        Outcome::Idle
    }

    fn text(form: &mut Form) -> Option<&mut String> {
        let (args, env) = (form.args_slot(), form.env_slot());
        match form.focus {
            slot if slot == args => Some(&mut form.extra_args),
            slot if slot == env => Some(&mut form.extra_env),
            slot if slot < form.fields.len() && form.fields[slot].parameter.kind == ParameterKind::Text => Some(form.fields[slot].value.get_or_insert_with(String::new)),
            _ => None,
        }
    }

    fn normalize(form: &mut Form) { for field in &mut form.fields { if field.parameter.kind == ParameterKind::Text && field.value.as_deref() == Some("") { field.value = None; } } }

    fn cycle(form: &mut Form, forward: bool) {
        let Some(field) = form.fields.get_mut(form.focus) else { return };
        let parameter = &field.parameter;
        match parameter.kind {
            ParameterKind::Text => {}
            ParameterKind::Flag => field.value = Some(if forward { "true" } else { "false" }.into()),
            ParameterKind::Choice => {
                let mut options: Vec<Option<&str>> = parameter.values.iter().map(|(value, _)| Some(value.as_str())).collect();
                if parameter.default.is_none() && !parameter.required { options.insert(0, None); }
                let current = options.iter().position(|option| *option == Form::effective(field)).unwrap_or(0);
                let next = if forward { (current + 1) % options.len() } else { (current + options.len() - 1) % options.len() };
                field.value = options[next].map(str::to_string);
            }
        }
    }

    /// 👀️ Resolves what the form would start, the same way the start itself does.
    fn preview(&self, form: &Form) -> Result<Launch, String> {
        let registry = self.registry.as_ref().ok_or("no registry")?;
        let start = form.start()?;
        registry.resolve(&start.id, &start.request())
    }
}

// #region 🔖️Screen
impl LauncherState {
    /// 🎨️ What the view paints now.
    pub fn screen(&self) -> Screen {
        let text = labels(self.locale);
        match (self.stage, &self.form) {
            (Stage::Configure, Some(form)) => {
                let (lines, focus) = self.form_lines(form, text);
                let (input, placeholder) = self.editor(form, text);
                let caption = match form.fields.get(form.focus).map(|field| field.parameter.kind) {
                    Some(ParameterKind::Text) => text.form_prompt_text.as_str(),
                    Some(_) => text.form_prompt_choice.as_str(),
                    None => "",
                };
                self.window(lines, focus, input, placeholder, caption.to_string())
            }
            (Stage::Confirm, Some(form)) => {
                let (mut lines, _) = self.form_lines(form, text);
                let command = self.preview(form).map_or_else(|_| form.id.clone(), |launch| crate::registry::window_title(&launch.label, self.locale));
                lines.insert(0, text.confirm_prompt.fill(&[("command", &command)]).into_string());
                self.window(lines, 0, String::new(), String::new(), text.form_mutating.as_str().to_string())
            }
            _ => Screen::default(),
        }
    }

    fn window(&self, lines: Vec<String>, selected: usize, input: String, placeholder: String, caption: String) -> Screen {
        let offset = self.offset.min(lines.len().saturating_sub(1));
        let items: Vec<String> = lines.into_iter().skip(offset).take(self.page).collect();
        Screen { input, placeholder, caption, selected: selected.saturating_sub(offset).min(items.len().saturating_sub(1)), items }
    }

    /// ✍️️ Whether the selected form slot accepts text and owns the terminal cursor.
    pub fn editing(&self) -> bool {
        self.editor_slot().is_some()
    }

    /// 🧷️ The identity of the current editor; a different form slot resets its caret.
    pub fn editor_slot(&self) -> Option<usize> {
        self.form.as_ref().filter(|form| self.stage == Stage::Configure && (form.focus == form.args_slot() || form.focus == form.env_slot() || form.fields.get(form.focus).is_some_and(|field| field.parameter.kind == ParameterKind::Text))).map(|form| form.focus)
    }

    /// 📨️ Commits a value edited by the shared input widget to the selected form slot.
    pub fn edited(&mut self, value: String) {
        if !self.editing() { return; }
        if let Some(form) = self.form.as_mut() { if let Some(text) = Self::text(form) { *text = value; } Self::normalize(form); }
    }

    fn editor(&self, form: &Form, text: &DashboardLabels) -> (String, String) {
        let (args, env) = (form.args_slot(), form.env_slot());
        match form.focus {
            slot if slot == args => (form.extra_args.clone(), text.form_extra_args_prompt.as_str().to_string()),
            slot if slot == env => (form.extra_env.clone(), text.form_extra_env_prompt.as_str().to_string()),
            slot if slot < form.fields.len() && form.fields[slot].parameter.kind == ParameterKind::Text => (form.fields[slot].value.clone().unwrap_or_default(), form.fields[slot].parameter.id.clone()),
            _ => (String::new(), String::new()),
        }
    }

    fn field_text(field: &Field, text: &DashboardLabels, focused: bool) -> String {
        let parameter = &field.parameter;
        let marker = if focused { "▸ " } else { "  " };
        let required = if parameter.required { format!(" ({})", text.form_required.as_str()) } else { String::new() };
        match parameter.kind {
            ParameterKind::Flag => {
                let on = Form::effective(field) == Some("true");
                format!("{marker}[{}] {}{required}  {}", if on { "x" } else { " " }, parameter.id, if on { text.form_on.as_str() } else { text.form_off.as_str() })
            }
            ParameterKind::Text => format!("{marker}{}: {}{required}", parameter.id, field.value.clone().unwrap_or_else(|| format!("({})", text.form_unset.as_str()))),
            ParameterKind::Choice => {
                let shown = Form::effective(field).map_or_else(|| format!("({})", text.form_unset.as_str()), |value| format!("‹{value}›{}", if parameter.default.as_deref() == Some(value) { format!(" ({})", text.form_default.as_str()) } else { String::new() }));
                let options = parameter.values.iter().take(6).map(|(value, _)| value.as_str()).collect::<Vec<_>>().join(" | ");
                format!("{marker}{}: {shown}{required}  [{options}{}]", parameter.id, if parameter.values.len() > 6 { " | …" } else { "" })
            }
        }
    }

    fn slot_lines(form: &Form, text: &DashboardLabels, locale: Locale) -> Vec<usize> { Self::layout(form, text, locale, None).1 }

    fn form_lines(&self, form: &Form, text: &DashboardLabels) -> (Vec<String>, usize) {
        let (lines, slots) = Self::layout(form, text, self.locale, self.registry.as_deref().map(|registry| (registry, self.preview(form))));
        (lines, slots.get(form.focus).copied().unwrap_or(0))
    }

    fn layout(form: &Form, text: &DashboardLabels, locale: Locale, preview: Option<(&Registry, Result<Launch, String>)>) -> (Vec<String>, Vec<usize>) {
        let mut lines: Vec<String> = Vec::new();
        let mut slots = vec![0; form.start_slot() + 1];
        let marker = |slot: usize| if form.focus == slot { "▸ " } else { "  " };
        let entry: Option<&Entry> = preview.as_ref().and_then(|(registry, _)| registry.entries().get(form.entry));
        lines.push(entry.map_or_else(|| form.id.clone(), |entry| entry.label.clone()));
        lines.push(form.id.clone());
        if form.mutating { lines.push(format!("⚠ {}", text.form_mutating.as_str())); }
        lines.push(if form.fields.is_empty() { text.form_no_parameters.as_str() } else { text.form_parameters.as_str() }.to_string());
        for (slot, field) in form.fields.iter().enumerate() { slots[slot] = lines.len(); lines.push(Self::field_text(field, text, slot == form.focus)); }
        slots[form.args_slot()] = lines.len();
        lines.push(format!("{}{}: {}", marker(form.args_slot()), text.form_extra_args.as_str(), form.extra_args));
        slots[form.env_slot()] = lines.len();
        lines.push(format!("{}{}: {}", marker(form.env_slot()), text.form_extra_env.as_str(), form.extra_env));
        slots[form.start_slot()] = lines.len();
        lines.push(format!("{}[ {} ]", marker(form.start_slot()), text.form_start.as_str()));
        if let Some((_, result)) = preview {
            match result {
                Ok(launch) => {
                    if !launch.requires.is_empty() {
                        lines.push(text.form_requires.as_str().to_string());
                        lines.extend(launch.requires.iter().map(|required| format!("  {}", crate::registry::window_title(&required.label, locale))));
                    }
                    if launch.processes.len() > 1 {
                        lines.push(text.form_members.fill(&[("count", &launch.processes.len().to_string())]).into_string());
                        lines.extend(launch.processes.iter().map(|process| format!("  {}", crate::registry::window_title(&process.label, locale))));
                    }
                    lines.push(text.form_command.as_str().to_string());
                    lines.extend(launch.processes.iter().map(|process| format!("  {} {}", process.cmd, process.args.join(" "))));
                }
                Err(problem) => lines.push(text.form_problem.fill(&[("problem", &problem)]).into_string()),
            }
        }
        (lines, slots)
    }
}
// #endregion 🔖️Screen

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
