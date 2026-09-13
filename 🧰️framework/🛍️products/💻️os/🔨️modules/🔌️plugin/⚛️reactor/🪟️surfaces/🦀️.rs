//! 🪟️ Ephemeral host projections retained by concrete surface identity for rendering and input.

use semio_framework::{UiRefreshSection, ViewModel};

pub(crate) struct SurfaceContext {
    pub(crate) body_key: String,
    pub(crate) view_state: ViewModel,
}

/// 🎭️ What a mounted surface IS, fixed at mount and replayed on every later host refresh: one
/// concrete window instance, an app-level panel, or a reserved section.
///
/// 🧩️ A reserved section keeps the FULL, unnarrowed view it was mounted with — it is rendered for no
/// window and for no panel, and no window closing prunes it, so a later host refresh must not
/// re-narrow it either.
enum SurfaceRole {
    Window(String),
    Panel,
    Section,
}

impl SurfaceRole {
    fn window_id(&self) -> Option<&str> {
        match self {
            Self::Window(id) => Some(id.as_str()),
            _ => None,
        }
    }

    fn project(&self, view: &ViewModel) -> Option<ViewModel> {
        match self {
            Self::Window(id) => view.for_window_instance(id),
            Self::Panel => Some(view.for_panel()),
            Self::Section => Some(view.clone()),
        }
    }
}

/// 🪟️ ONE mounted surface: its authored body, its role, and — the point of this type — its OWN
/// projected host view.
struct SurfaceBinding {
    surface: String,
    body_key: String,
    role: SurfaceRole,
    view_state: ViewModel,
}

pub(crate) struct SurfaceContexts {
    slots: [Option<SurfaceBinding>; semio_framework_ui_contract::UI_RESIDENT_SLOTS],
    /// 🪟️ The last host view this INSTANCE was mounted or refreshed with — diagnostics and
    /// window-scoped background work only (`ArtifactApp::pending_effects`). Never the source a body
    /// is rendered from: that is each binding's own [`SurfaceBinding::view_state`].
    view_state: Option<ViewModel>,
}

impl Default for SurfaceContexts {
    fn default() -> Self {
        Self { slots: std::array::from_fn(|_| None), view_state: None }
    }
}

impl SurfaceContexts {
    /// 🪟️ Binds one concrete surface to its OWN projection of the host view it was mounted with.
    ///
    /// 🎯️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B25: this used to store the raw view in ONE
    /// shared field that every mount overwrote, and [`Self::get`] rebuilt a body's `ViewModel` from
    /// whichever sibling mounted LAST — so the Inspection panel rendered against the last pane's
    /// projection (wrong `focusedWindowId`/`panelJson`/`locale`) and the first pane rendered against
    /// the synthetic `window` alias's. The shell already sends a distinct context per surface
    /// (`windowViewContext`/`panelViewContext`/`sectionViewContext`, `🛂️manifest/🟦️.ts`) and mounts
    /// them across refresh generations; nothing but this type ever conflated them.
    ///
    /// 🚫️ WAVE B56: the synthetic surface named `window` is REFUSED for an app whose host view
    /// carries window instances. It existed only so a guest dirty addressed to the bare `window` name
    /// would find a host context (wave W-G3 §8.29); every such dirty now names the real instance, and
    /// admitting the synthetic name again would remount the surface whose reconcile reservation refusal
    /// starved every real one (wave B54 §6.4). An app that genuinely authors a window instance called
    /// `window` still mounts it — the refusal is about a name no instance claims.
    pub(crate) fn insert(&mut self, surface: String, body_key: String, view_state: ViewModel) -> Result<(), &'static str> {
        if synthetic_window_surface(&surface, &view_state) {
            return Err("a surface named `window` is synthetic: an app with window instances is addressed by instance");
        }
        let role = if UiRefreshSection::from_body_key(&body_key).is_some() {
            SurfaceRole::Section
        } else {
            match view_state.window_id.clone() {
                Some(window) => SurfaceRole::Window(window),
                None => SurfaceRole::Panel,
            }
        };
        let projected = role.project(&view_state).ok_or("surface context names a window instance the host view does not carry")?;
        let index = self
            .slots
            .iter()
            .position(|slot| slot.as_ref().is_some_and(|context| context.surface == surface))
            .or_else(|| self.slots.iter().position(|slot| slot.as_ref().is_none_or(|context| context.role.window_id().is_some_and(|id| !view_state.window_instances.iter().any(|window| window.id == id)))))
            .ok_or("surface context capacity exhausted")?;
        self.prune_windows(&view_state);
        self.slots[index] = Some(SurfaceBinding { surface, body_key, role, view_state: projected });
        self.view_state = Some(view_state);
        Ok(())
    }

    pub(crate) fn get(&self, surface: &str) -> Option<SurfaceContext> {
        let binding = self.slots.iter().flatten().find(|context| context.surface == surface)?;
        Some(SurfaceContext { body_key: binding.body_key.clone(), view_state: binding.view_state.clone() })
    }

    /// 🪟️ The last host view this instance was refreshed or mounted with — the attached-window
    /// roster an app needs to address window-scoped background work outside a render pass
    /// (`ArtifactApp::pending_effects`). `None` until the first surface is mounted.
    pub(crate) fn view(&self) -> Option<&ViewModel> {
        self.view_state.as_ref()
    }

    /// 🔄️ Replays one fresh host view onto EVERY live binding through that binding's own role, so a
    /// locale switch or a rearmed tool reaches every mounted surface without any of them borrowing a
    /// sibling's window identity. Reserved sections are deliberately untouched (see [`SurfaceRole`]).
    pub(crate) fn update_view(&mut self, view: &ViewModel) {
        self.prune_windows(view);
        for binding in self.slots.iter_mut().flatten() {
            if matches!(binding.role, SurfaceRole::Section) {
                continue;
            }
            if let Some(projected) = binding.role.project(view) {
                binding.view_state = projected;
            }
        }
        self.view_state = Some(view.clone());
    }

    fn prune_windows(&mut self, view: &ViewModel) {
        for slot in &mut self.slots {
            if slot.as_ref().and_then(|context| context.role.window_id()).is_some_and(|id| !view.window_instances.iter().any(|window| window.id == id)) {
                *slot = None;
            }
        }
    }

    /// 🪟️ The concrete surfaces background work addresses when it owns no surface of its own — a
    /// spawned job's progress or completion, and a document-backbone message.
    ///
    /// 🐛️ WAVE B56: those three reactor sites used to mint `"<instance>:window"` and dirty THAT, which
    /// re-rendered the last-mounted window's body under a surface no pane reads while the real panes
    /// never saw the update at all. Every mounted window instance is dirtied now; an instance with no
    /// mounted window falls back to its app-level panels, which is the document scope such work belongs
    /// to when there is no window to address (ticket 26/09/02/PUZZLE-3D-END-TO-END).
    pub(crate) fn background_surfaces(&self) -> Vec<String> {
        let windows: Vec<String> = self.slots.iter().flatten().filter(|binding| binding.role.window_id().is_some()).map(|binding| binding.surface.clone()).collect();
        if !windows.is_empty() {
            return windows;
        }
        self.slots.iter().flatten().filter(|binding| matches!(binding.role, SurfaceRole::Panel)).map(|binding| binding.surface.clone()).collect()
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

    #[cfg(test)]
    fn is_window(&self, surface: &str) -> bool {
        self.slots.iter().flatten().any(|binding| binding.surface == surface && binding.role.window_id().is_some())
    }

    #[cfg(test)]
    fn is_section(&self, surface: &str) -> bool {
        self.slots.iter().flatten().any(|binding| binding.surface == surface && matches!(binding.role, SurfaceRole::Section))
    }
}

/// 🪟️ Whether one mounted surface name is the synthetic `window` alias rather than a real window
/// instance: the name is exactly `window` after the instance prefix, the host view carries window
/// instances, and none of them is itself called `window`.
fn synthetic_window_surface(surface: &str, view_state: &ViewModel) -> bool {
    let body = surface.rsplit_once(':').map_or(surface, |(_, body)| body);
    body == SYNTHETIC_WINDOW_SURFACE && !view_state.window_instances.is_empty() && !view_state.window_instances.iter().any(|window| window.id == SYNTHETIC_WINDOW_SURFACE)
}

/// 🪟️ The surface name no app may mount while it has window instances — see [`synthetic_window_surface`].
pub(crate) const SYNTHETIC_WINDOW_SURFACE: &str = "window";

#[cfg(test)]
#[path = "🧪️tests/🪟️surface-context-lifecycle/🦀️.rs"]
mod tests;
