//! 📋️ The list windows: the task overview, the settings and the keyboard help. Every row carries the
//! action activating it performs, so a row is never recognized by its text.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/⚙️preferences/🦀️.rs

use super::windows::{Body, Dashboard, Kind};
use crate::ipc::SessionStatus;
use crate::preferences::keymap::{Scope, SEND_PREFIX};
use crate::preferences::Change;
use crate::registry;
use ui_tui::tui::engine::Tui;
use ui_tui::tui::widget::WidgetState;

/// 🎬️ What activating a row does.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum RowAction { None, NewTask, Settings, Help, Refresh, CancelDiscovery, Session(String), Language, Appearance, Terminology, Renderer, Layout, SaveScope, Back }

/// 🪧️ The rows of one list window and where the selection and the visible slice are.
#[derive(Default)]
pub(super) struct Pane { pub rows: Vec<(String, RowAction)>, pub selected: usize, pub offset: usize, pub page: usize }

impl Pane {
    fn reveal(&mut self) {
        let page = self.page.max(1);
        if self.selected < self.offset { self.offset = self.selected; } else if self.selected >= self.offset + page { self.offset = self.selected + 1 - page; }
    }

    pub fn select(&mut self, delta: isize) {
        self.selected = (self.selected as isize + delta).clamp(0, self.rows.len().saturating_sub(1) as isize) as usize;
        self.reveal();
    }

    pub fn current(&self) -> Option<&RowAction> { self.rows.get(self.selected).map(|(_, action)| action) }
}

impl Dashboard {
    fn pane_rows(&self, index: usize) -> Vec<(String, RowAction)> {
        let text = self.text();
        match &self.windows[index].body {
            Body::Overview => {
                let mut rows = vec![(text.row_new_task.as_str().into(), RowAction::NewTask), (text.row_settings.as_str().into(), RowAction::Settings), (text.row_keyboard.as_str().into(), RowAction::Help), (text.row_refresh.as_str().into(), RowAction::Refresh), (text.row_cancel_discovery.as_str().into(), RowAction::CancelDiscovery)];
                if self.sessions.is_empty() { rows.push((text.row_no_tasks.as_str().into(), RowAction::None)); }
                rows.extend(self.sessions.values().map(|session| (self.session_row(session), RowAction::Session(session.session_id.clone()))));
                rows
            }
            Body::Settings => {
                let scope = if self.shared_preferences { text.scope_workspace } else { text.scope_local };
                let setting = |name: &str, value: &str, action: RowAction| (format!("{name}: {value}"), action);
                vec![
                    setting(text.setting_language.as_str(), &self.preferences.language, RowAction::Language),
                    setting(text.setting_appearance.as_str(), &self.preferences.appearance, RowAction::Appearance),
                    setting(text.setting_terminology.as_str(), &self.preferences.terminology, RowAction::Terminology),
                    setting(text.setting_renderer.as_str(), &self.preferences.renderer, RowAction::Renderer),
                    setting(text.setting_layout.as_str(), &self.preferences.layout, RowAction::Layout),
                    setting(text.setting_scope.as_str(), scope.as_str(), RowAction::SaveScope),
                    setting(text.setting_prefix.as_str(), &self.keymap.prefix().label(), RowAction::None),
                    (text.row_back.as_str().into(), RowAction::Back),
                ]
            }
            Body::Help => self.help_rows(),
            _ => Vec::new(),
        }
    }

    fn session_row(&self, session: &crate::ipc::SessionInfo) -> String {
        let text = self.text();
        let word = match session.status {
            SessionStatus::Pending => text.status_pending,
            SessionStatus::Running if session.ready_url.is_some() => text.status_ready,
            SessionStatus::Running => text.status_running,
            SessionStatus::Stopping => text.status_stopping,
            SessionStatus::Exited => text.status_exited,
            SessionStatus::Failed => text.status_failed,
            SessionStatus::Interrupted => text.status_interrupted,
        };
        let code = session.code.map(|code| format!(" · {}", text.exit_code.fill(&[("code", &code.to_string())]).as_str())).unwrap_or_default();
        let url = session.ready_url.as_deref().map(|url| format!(" · {url}")).unwrap_or_default();
        let label = &session.command.label;
        let name = if label.verb.is_empty() && label.subject.is_empty() { session.command.cmd.clone() } else { registry::tab_text(label, self.locale, 48) };
        format!("{name}   {}{code}{url}", word.as_str())
    }

    fn help_rows(&self) -> Vec<(String, RowAction)> {
        let text = self.text();
        let mut rows: Vec<(String, RowAction)> = Vec::new();
        let sections = [(Scope::Prefix, text.help_prefix.fill(&[("prefix", &self.keymap.prefix().label())]).into_string()), (Scope::Window, text.help_window.as_str().to_string()), (Scope::View, text.help_view.as_str().to_string())];
        for (scope, heading) in sections {
            rows.push((heading, RowAction::None));
            for action in self.keymap.actions(scope) {
                let keys = self.keymap.keys_label(scope, &action).unwrap_or_default();
                let keys = if scope == Scope::Prefix && action != SEND_PREFIX { keys.strip_prefix(&format!("{} ", self.keymap.prefix().label())).unwrap_or(&keys).to_string() } else { keys };
                rows.push((format!("  {keys:<16} {}", text.action(&action)), RowAction::None));
            }
        }
        rows.push((text.help_terminal.as_str().into(), RowAction::None));
        rows.push((text.help_customize.as_str().into(), RowAction::None));
        rows
    }

    /// 🔁️ Recomputes the rows of every list window and paints the visible slice.
    pub fn refresh_panes(&mut self, tui: &mut Tui) { for index in 0..self.windows.len() { self.refresh_pane(tui, index); } }

    pub fn refresh_pane(&mut self, tui: &mut Tui, index: usize) {
        if !matches!(self.windows[index].body, Body::Overview | Body::Settings | Body::Help) { return; }
        let rows = self.pane_rows(index);
        let page = usize::from(tui.scene.rect(self.windows[index].list).height).max(1);
        let window = &mut self.windows[index];
        let keep = window.pane.current().cloned();
        window.pane.rows = rows;
        window.pane.page = page;
        window.pane.selected = keep.filter(|action| *action != RowAction::None).and_then(|action| window.pane.rows.iter().position(|(_, candidate)| *candidate == action)).unwrap_or_else(|| window.pane.selected.min(window.pane.rows.len().saturating_sub(1)));
        window.pane.reveal();
        let (offset, selected) = (window.pane.offset, window.pane.selected);
        let items: Vec<String> = window.pane.rows.iter().skip(offset).take(page).map(|(text, _)| text.clone()).collect();
        let statuses: Vec<Option<ui_tui::tui::theme::Status>> = window.pane.rows.iter().skip(offset).take(page).map(|(_, action)| match action { RowAction::Session(id) => self.sessions.get(id).map(Self::status_of), _ => None }).collect();
        let node = self.windows[index].list;
        let unchanged = matches!(&tui.scene.node(node).content, ui_tui::tui::scene::NodeContent::Widget(WidgetState::List(list)) if list.items == items && list.selected == selected - offset && list.statuses == statuses);
        if unchanged { return; }
        if let Some(WidgetState::List(list)) = tui.scene.node_mut(node).widget() {
            list.marks = vec![false; items.len()];
            list.items = items;
            list.statuses = statuses;
            list.selected = selected - offset;
            list.offset = 0;
        }
    }

    /// 👆️ Performs what the selected row of a list window stands for.
    pub fn activate_row(&mut self, tui: &mut Tui, index: usize) {
        let Some(action) = self.windows[index].pane.current().cloned() else { return };
        match action {
            RowAction::None => {}
            RowAction::NewTask => self.set_body(tui, index, Kind::Launcher, None),
            RowAction::Settings => self.set_body(tui, index, Kind::Settings, None),
            RowAction::Help => self.set_body(tui, index, Kind::Help, None),
            RowAction::Back => self.set_body(tui, index, Kind::Overview, None),
            RowAction::Refresh => self.refresh_inventory(),
            RowAction::CancelDiscovery => self.cancel_inventory(),
            RowAction::Session(id) => self.reveal_session(tui, &id),
            RowAction::Language => self.queue_preference(Change { language: Some(if self.preferences.language == "en" { "de" } else { "en" }.into()), ..Default::default() }),
            RowAction::Appearance => self.queue_preference(Change { appearance: Some(if self.light { "dark" } else { "light" }.into()), ..Default::default() }),
            RowAction::Terminology => self.queue_preference(Change { terminology: Some(if self.preferences.terminology == "native" { "reuse" } else { "native" }.into()), ..Default::default() }),
            RowAction::Renderer => self.queue_preference(Change { renderer: Some(match self.preferences.renderer.as_str() { "react" => "wgpu-wasm", "wgpu-wasm" => "wgpu-native", _ => "react" }.into()), ..Default::default() }),
            RowAction::Layout => self.queue_preference(Change { layout: Some(match self.preferences.layout.as_str() { "tabs" => "columns", "columns" => "rows", _ => "tabs" }.into()), ..Default::default() }),
            RowAction::SaveScope => { self.shared_preferences = !self.shared_preferences; self.refresh_pane(tui, index); }
        }
    }
}

// #region 🔖️Preferences
impl Dashboard {
    /// 💾️ Publishes one preference event off the UI thread; the projection follows when it is durable.
    pub fn queue_preference(&mut self, change: Change) {
        if self.saving_preferences.is_some() { self.notice = Some(self.text().prefs_busy.as_str().into()); return; }
        let path = if self.shared_preferences { crate::preferences::shared_path(&self.root) } else { self.preference_path.clone() };
        let (sender, receiver) = std::sync::mpsc::channel();
        std::thread::spawn(move || { let result = crate::preferences::append(&path, &change); let _ = sender.send((change, result)); });
        self.saving_preferences = Some(receiver);
        self.notice = Some(self.text().prefs_saving.as_str().into());
    }

    /// 📥️ Folds a durable preference event into the live view: language, appearance, keymap and launchers.
    pub fn adopt_preference(&mut self, tui: &mut Tui, change: &Change) {
        self.preferences.apply(change);
        self.locale = self.preferences.locale();
        self.set_appearance(tui, self.preferences.appearance == "light");
        let (keymap, problems) = self.preferences.keymap();
        self.keymap = keymap;
        self.notice = Some(problems.first().map_or_else(|| self.text().prefs_saved.as_str().to_string(), |problem| self.text().prefs_keymap_problem.fill(&[("problem", problem)]).into_string()));
        for window in &mut self.windows {
            let (Body::Launcher(launcher), Some(node)) = (&mut window.body, window.tree) else { continue };
            launcher.set_locale(self.locale);
            launcher.set_preferences(self.preferences.clone());
            let mut found = tui.scene.node_mut(node);
            if let Some(WidgetState::Tree(tree)) = found.widget() { *tree = launcher.rebuild_tree(Some(tree)); }
        }
        self.remount(tui);
    }
}
// #endregion 🔖️Preferences
