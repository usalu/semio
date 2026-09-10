//! 🪟️ Ephemeral host projections retained by concrete surface identity for rendering and input.

use semio_framework::{UiRefreshSection, ViewModel};

pub(crate) struct SurfaceContext {
    pub(crate) body_key: String,
    pub(crate) view_state: ViewModel,
}

struct SurfaceBinding {
    surface: String,
    body_key: String,
    window_id: Option<String>,
    section: bool,
}

pub(crate) struct SurfaceContexts {
    slots: [Option<SurfaceBinding>; semio_framework_ui_contract::UI_RESIDENT_SLOTS],
    view_state: Option<ViewModel>,
    /// 🧩️ The last full, unnarrowed host view a reserved section surface was mounted with — kept apart
    /// from `view_state` because every window/panel mount overwrites that one with its own projection,
    /// which would make a section's own view depend on which surface happened to mount last.
    section_view: Option<ViewModel>,
}

impl Default for SurfaceContexts {
    fn default() -> Self {
        Self { slots: std::array::from_fn(|_| None), view_state: None, section_view: None }
    }
}

impl SurfaceContexts {
    pub(crate) fn insert(&mut self, surface: String, body_key: String, view_state: ViewModel) -> Result<(), &'static str> {
        let section = UiRefreshSection::from_body_key(&body_key).is_some();
        let index = self
            .slots
            .iter()
            .position(|slot| slot.as_ref().is_some_and(|context| context.surface == surface))
            .or_else(|| self.slots.iter().position(|slot| slot.as_ref().is_none_or(|context| context.window_id.as_deref().is_some_and(|id| !view_state.window_instances.iter().any(|window| window.id == id)))))
            .ok_or("surface context capacity exhausted")?;
        self.prune_windows(&view_state);
        let window_id = if section { None } else { view_state.window_id.clone() };
        self.slots[index] = Some(SurfaceBinding { surface, body_key, window_id, section });
        if section {
            self.section_view = Some(view_state.clone());
        }
        self.view_state = Some(view_state);
        Ok(())
    }

    pub(crate) fn get(&self, surface: &str) -> Option<SurfaceContext> {
        let binding = self.slots.iter().flatten().find(|context| context.surface == surface)?;
        if binding.section {
            let view = self.section_view.as_ref().or(self.view_state.as_ref())?;
            return Some(SurfaceContext { body_key: binding.body_key.clone(), view_state: view.clone() });
        }
        let view = self.view_state.as_ref()?;
        let view_state = match binding.window_id.as_deref() {
            Some(window) => view.for_window_instance(window)?,
            None => view.for_panel(),
        };
        Some(SurfaceContext { body_key: binding.body_key.clone(), view_state })
    }

    /// 🪟️ The last host view this instance was refreshed or mounted with — the attached-window
    /// roster an app needs to address window-scoped background work outside a render pass
    /// (`ArtifactApp::pending_effects`). `None` until the first surface is mounted.
    pub(crate) fn view(&self) -> Option<&ViewModel> {
        self.view_state.as_ref()
    }

    pub(crate) fn update_view(&mut self, view: &ViewModel) {
        self.prune_windows(view);
        self.view_state = Some(view.clone());
    }

    fn prune_windows(&mut self, view: &ViewModel) {
        for slot in &mut self.slots {
            if slot.as_ref().and_then(|context| context.window_id.as_deref()).is_some_and(|id| !view.window_instances.iter().any(|window| window.id == id)) {
                *slot = None;
            }
        }
    }

    pub(crate) fn remove(&mut self, surface: &str) {
        if let Some(slot) = self.slots.iter_mut().find(|slot| slot.as_ref().is_some_and(|context| context.surface == surface)) {
            *slot = None;
        }
        if !self.slots.iter().flatten().any(|binding| binding.section) {
            self.section_view = None;
        }
        if self.slots.iter().all(Option::is_none) {
            self.view_state = None;
        }
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        self.slots.iter().flatten().count()
    }
}

#[cfg(test)]
#[path = "🧪️tests/🪟️surface-context-lifecycle/🦀️.rs"]
mod tests;
