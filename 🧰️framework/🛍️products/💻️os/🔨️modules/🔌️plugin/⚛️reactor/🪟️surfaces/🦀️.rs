//! 🪟️ Ephemeral host projections retained by concrete surface identity for rendering and input.

use semio_framework::ViewModel;

pub(crate) struct SurfaceContext {
    pub(crate) body_key: String,
    pub(crate) view_state: ViewModel,
}

struct SurfaceBinding {
    surface: String,
    body_key: String,
    window_id: Option<String>,
}

pub(crate) struct SurfaceContexts {
    slots: [Option<SurfaceBinding>; semio_framework_ui_contract::UI_RESIDENT_SLOTS],
    view_state: Option<ViewModel>,
}

impl Default for SurfaceContexts {
    fn default() -> Self {
        Self { slots: std::array::from_fn(|_| None), view_state: None }
    }
}

impl SurfaceContexts {
    pub(crate) fn insert(&mut self, surface: String, body_key: String, view_state: ViewModel) -> Result<(), &'static str> {
        let index = self
            .slots
            .iter()
            .position(|slot| slot.as_ref().is_some_and(|context| context.surface == surface))
            .or_else(|| self.slots.iter().position(|slot| slot.as_ref().is_none_or(|context| context.window_id.as_deref().is_some_and(|id| !view_state.window_instances.iter().any(|window| window.id == id)))))
            .ok_or("surface context capacity exhausted")?;
        self.prune_windows(&view_state);
        self.slots[index] = Some(SurfaceBinding { surface, body_key, window_id: view_state.window_id.clone() });
        self.view_state = Some(view_state);
        Ok(())
    }

    pub(crate) fn get(&self, surface: &str) -> Option<SurfaceContext> {
        let binding = self.slots.iter().flatten().find(|context| context.surface == surface)?;
        let view = self.view_state.as_ref()?;
        let view_state = match binding.window_id.as_deref() {
            Some(window) => view.for_window_instance(window)?,
            None => view.for_panel(),
        };
        Some(SurfaceContext { body_key: binding.body_key.clone(), view_state })
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
