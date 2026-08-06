// #region platform
//! 🖥️ Root shell: apps, URI chrome, panel toggles, and shared action bus.

use crate::action_bus::ActionBus;
use crate::ui::AppDefinition;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct PanelVisibility {
    pub left_side_panel: bool,
    pub right_side_panel: bool,
}

#[derive(Clone, Debug, Default)]
pub struct PlatformSpec {
    pub id: String,
    pub name: String,
    pub default_active_app_id: Option<String>,
    pub initial_panel_visibility: Option<PanelVisibility>,
}

pub struct Platform {
    pub action_bus: ActionBus,
    pub apps: Vec<AppDefinition>,
    pub active_app_id: String,
    pub generation: u64,
    pub chrome_generation: u64,
    pub uri: String,
    pub panel_visibility: PanelVisibility,
    pub id: String,
    pub name: String,
    generation_counter: AtomicU64,
    chrome_generation_counter: AtomicU64,
}

impl Platform {
    pub fn new(spec: Option<PlatformSpec>) -> Self {
        let spec = spec.unwrap_or_default();
        let panel_visibility = spec.initial_panel_visibility.clone().unwrap_or_default();
        Self {
            action_bus: ActionBus::new(),
            apps: Vec::new(),
            active_app_id: spec.default_active_app_id.clone().unwrap_or_default(),
            generation: 0,
            chrome_generation: 0,
            uri: "/".into(),
            panel_visibility,
            id: spec.id,
            name: spec.name,
            generation_counter: AtomicU64::new(0),
            chrome_generation_counter: AtomicU64::new(0),
        }
    }

    pub fn add_app(&mut self, app: AppDefinition) {
        if self.active_app_id.is_empty() {
            self.active_app_id = app.id.clone();
        }
        self.apps.push(app);
        self.notify();
    }

    pub fn get_active_app(&self) -> Option<&AppDefinition> {
        self.apps
            .iter()
            .find(|app| app.id == self.active_app_id)
            .or_else(|| self.apps.first())
    }

    pub fn set_active_app_id(&mut self, id: String) {
        if self.active_app_id == id {
            return;
        }
        self.active_app_id = id;
        self.notify_chrome();
    }

    pub fn set_panel_visibility(&mut self, next: PanelVisibility) {
        if self.panel_visibility == next {
            return;
        }
        self.panel_visibility = next;
        self.notify_chrome();
    }

    pub fn notify(&mut self) {
        self.generation = self.generation_counter.fetch_add(1, Ordering::SeqCst) + 1;
    }

    pub fn notify_chrome(&mut self) {
        self.chrome_generation = self.chrome_generation_counter.fetch_add(1, Ordering::SeqCst) + 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::{ModeDefinition, WindowKindDefinition};
    use ui_wgpu::LocalizedLabel;

    #[test]
    fn adds_first_app_as_active() {
        let mut platform = Platform::new(None);
        platform.add_app(AppDefinition {
            id: "draw-play".into(),
            label: LocalizedLabel::data("Draw"),
            document: vec!["semio".into(), "draw".into()],
            icon_id: None,
            controller_id: "draw-play".into(),
            modes: crate::ui::Modes::one(ModeDefinition {
                id: "edit".into(),
                label: LocalizedLabel::data("Edit"),
                icon_id: "pencil".into(),
                tools: Vec::new(),
                layout_id: None,
                commands: Vec::new(),
            }),
            default_mode_id: "edit".into(),
            window_kinds: crate::ui::WindowKinds::one(WindowKindDefinition {
                id: "composite".into(),
                label: LocalizedLabel::data("Canvas"),
                body_key: "composite".into(),
                surface_kind: ui_wgpu::SurfaceKind::Canvas2d,
                icon_id: "pen-tool".into(),
                options: ui_wgpu::WindowOptions::default(),
                actions: Vec::new(),
                utilities: Vec::new(),
                params_schema: None,
                document_projection_schema: None,
                input_event_schema: None,
                output_schema: None,
                capabilities: Vec::new(),
            }),
            panel_tabs: vec![],
            keybindings: vec![],
            actions: vec![],
            utilities: vec![],
            tools: vec![],
            commands: vec![],
            named_layouts: Vec::new(),
            default_layout: None,
            terminologies: Vec::new(),
            terminology_documents: std::collections::HashMap::new(),
            introduction: None,
            tutorials: Vec::new(),
            dialogs: Vec::new(),
            media_inputs: Vec::new(),
            media_outputs: Vec::new(),
            artifact_kinds: Vec::new(),
            config: crate::ConfigSpec::empty(),
            command_grammar: crate::CommandGrammar::empty(),
            io: crate::AppIo::default(),
        });
        assert_eq!(platform.active_app_id, "draw-play");
    }

    fn minimal_app(id: &str) -> AppDefinition {
        AppDefinition {
            id: id.into(),
            label: LocalizedLabel::data(id),
            document: vec!["semio".into(), id.into()],
            icon_id: None,
            controller_id: id.into(),
            modes: crate::ui::Modes::one(ModeDefinition {
                id: "edit".into(),
                label: LocalizedLabel::data("Edit"),
                icon_id: "pencil".into(),
                tools: Vec::new(),
                layout_id: None,
                commands: Vec::new(),
            }),
            default_mode_id: "edit".into(),
            window_kinds: crate::ui::WindowKinds::one(WindowKindDefinition {
                id: "main".into(),
                label: LocalizedLabel::data("Main"),
                body_key: "main".into(),
                surface_kind: ui_wgpu::SurfaceKind::Canvas2d,
                icon_id: "pen-tool".into(),
                options: ui_wgpu::WindowOptions::default(),
                actions: Vec::new(),
                utilities: Vec::new(),
                params_schema: None,
                document_projection_schema: None,
                input_event_schema: None,
                output_schema: None,
                capabilities: Vec::new(),
            }),
            panel_tabs: vec![],
            keybindings: vec![],
            actions: vec![],
            utilities: vec![],
            tools: vec![],
            commands: vec![],
            named_layouts: Vec::new(),
            default_layout: None,
            terminologies: Vec::new(),
            terminology_documents: std::collections::HashMap::new(),
            introduction: None,
            tutorials: Vec::new(),
            dialogs: Vec::new(),
            media_inputs: Vec::new(),
            media_outputs: Vec::new(),
            artifact_kinds: Vec::new(),
            config: crate::ConfigSpec::empty(),
            command_grammar: crate::CommandGrammar::empty(),
            io: crate::AppIo::default(),
        }
    }

    #[test]
    fn set_active_app_id_is_noop_when_unchanged() {
        let mut platform = Platform::new(None);
        platform.add_app(minimal_app("draw"));
        let generation_before = platform.chrome_generation;
        platform.set_active_app_id("draw".into());
        assert_eq!(platform.chrome_generation, generation_before, "same id must not bump chrome_generation");
        platform.set_active_app_id("other".into());
        assert_eq!(platform.chrome_generation, generation_before + 1);
        assert_eq!(platform.active_app_id, "other");
    }

    #[test]
    fn get_active_app_falls_back_to_first_when_active_id_unknown() {
        let mut platform = Platform::new(None);
        platform.add_app(minimal_app("draw"));
        platform.active_app_id = "missing".into();
        assert_eq!(platform.get_active_app().unwrap().id, "draw");
    }

    #[test]
    fn set_panel_visibility_is_noop_when_unchanged_else_bumps_chrome_generation() {
        let mut platform = Platform::new(None);
        let generation_before = platform.chrome_generation;
        platform.set_panel_visibility(PanelVisibility::default());
        assert_eq!(platform.chrome_generation, generation_before, "same visibility must not bump generation");
        platform.set_panel_visibility(PanelVisibility { left_side_panel: true, right_side_panel: false });
        assert_eq!(platform.chrome_generation, generation_before + 1);
    }

    #[test]
    fn notify_and_notify_chrome_increment_independently() {
        let mut platform = Platform::new(None);
        platform.notify();
        platform.notify();
        platform.notify_chrome();
        assert_eq!(platform.generation, 2);
        assert_eq!(platform.chrome_generation, 1);
    }
}
// #endregion platform
