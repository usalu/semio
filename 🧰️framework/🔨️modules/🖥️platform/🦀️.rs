// #region platform
//! 🖥️ Root shell: apps, URI chrome, panel toggles, and shared action bus.

use crate::action_bus::ActionBus;
use crate::ui::AppDefinition;

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
}

impl Platform {
    pub async fn new(spec: Option<PlatformSpec>) -> Self {
        let spec = spec.unwrap_or_default();
        let panel_visibility = spec.initial_panel_visibility.clone().unwrap_or_default();
        Self { action_bus: ActionBus::production(), apps: Vec::new(), active_app_id: spec.default_active_app_id.clone().unwrap_or_default(), generation: 0, chrome_generation: 0, uri: "/".into(), panel_visibility, id: spec.id, name: spec.name }
    }

    pub async fn add_app(&mut self, app: AppDefinition) {
        if self.active_app_id.is_empty() {
            self.active_app_id = app.id.clone();
        }
        self.apps.push(app);
        self.notify().await;
    }

    pub async fn get_active_app(&self) -> Option<&AppDefinition> {
        self.apps.iter().find(|app| app.id == self.active_app_id).or_else(|| self.apps.first())
    }

    pub async fn set_active_app_id(&mut self, id: String) {
        if self.active_app_id == id {
            return;
        }
        self.active_app_id = id;
        self.notify_chrome().await;
    }

    pub async fn set_panel_visibility(&mut self, next: PanelVisibility) {
        if self.panel_visibility == next {
            return;
        }
        self.panel_visibility = next;
        self.notify_chrome().await;
    }

    pub async fn notify(&mut self) {
        self.generation = self.generation.saturating_add(1);
    }

    pub async fn notify_chrome(&mut self) {
        self.chrome_generation = self.chrome_generation.saturating_add(1);
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion platform
