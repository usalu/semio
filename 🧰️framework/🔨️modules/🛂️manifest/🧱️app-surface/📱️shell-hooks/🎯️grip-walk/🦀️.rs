//! 🎯️ Keyboard focus of the grips of an open artifact: which grip has the keyboard, and how Tab, Shift+Tab and Ctrl+Arrow move it. Domain-neutral (a grip is an id and the id of the entity that owns it), ephemeral and local to this user (never persisted, never shared). The walk rules are pinned by the language-agnostic fixture `🐚️Shell/🧪️tests/🧫️fixtures/🎯️grip-focus/🔣️.json`, which the React twin (`🏛️ShellHost/🎯️grip-focus`) answers as well.
//!
//! 🔗️ Paint: a plugin's lane overlays; keys: the plugin's [`super::PluginShellHooks::handle_key`]; see `visual-editor-architecture.md` §8.2.

use std::sync::{Mutex, PoisonError};

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

/// 🕹️ One grip the walk can reach: its handle id and the id of the entity that owns it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GripRef {
    pub id: String,
    pub entity: String,
}

/// 🚶️ How a key moves the grip focus: the whole selection in order (Tab, ends leave the walk) or only the grips of the focused grip's entity (Ctrl+Arrow, wraps around).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GripWalk {
    Forward,
    Backward,
    ForwardInEntity,
    BackwardInEntity,
}

/// 🚶️ The outcome of a walk: the grip that takes the focus, or `Leave` when Tab ran past either end of the selection's grips (the walk ends and the shell's own Tab order continues).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GripStep {
    To(String),
    Leave,
    Nothing,
}

/// 🚶️ The next focus. Without a current grip Tab and Shift+Tab start at the first or last grip and Ctrl+Arrow at the first or last grip of the first entity; a current grip that is no longer among the grips (the selection changed) counts as none.
pub fn walk_grips(grips: &[GripRef], current: Option<&str>, how: GripWalk) -> GripStep {
    if grips.is_empty() {
        return GripStep::Nothing;
    }
    let forward = matches!(how, GripWalk::Forward | GripWalk::ForwardInEntity);
    let at = current.and_then(|id| grips.iter().position(|grip| grip.id == id));
    match how {
        GripWalk::Forward | GripWalk::Backward => match (at, forward) {
            (None, true) => GripStep::To(grips[0].id.clone()),
            (None, false) => GripStep::To(grips[grips.len() - 1].id.clone()),
            (Some(i), true) if i + 1 < grips.len() => GripStep::To(grips[i + 1].id.clone()),
            (Some(i), false) if i > 0 => GripStep::To(grips[i - 1].id.clone()),
            _ => GripStep::Leave,
        },
        GripWalk::ForwardInEntity | GripWalk::BackwardInEntity => {
            let entity = at.map_or_else(|| grips[0].entity.clone(), |i| grips[i].entity.clone());
            let ring: Vec<&GripRef> = grips.iter().filter(|grip| grip.entity == entity).collect();
            let place = at.and_then(|i| ring.iter().position(|grip| grip.id == grips[i].id));
            let next = match (place, forward) {
                (Some(p), true) => (p + 1) % ring.len(),
                (Some(p), false) => (p + ring.len() - 1) % ring.len(),
                (None, true) => 0,
                (None, false) => ring.len() - 1,
            };
            GripStep::To(ring[next].id.clone())
        }
    }
}

#[derive(Default)]
struct Registry {
    grips: Vec<(String, Vec<GripRef>)>,
    focus: Vec<(String, String)>,
}

static REGISTRY: Mutex<Registry> = Mutex::new(Registry { grips: Vec::new(), focus: Vec::new() });

fn registry() -> std::sync::MutexGuard<'static, Registry> {
    REGISTRY.lock().unwrap_or_else(PoisonError::into_inner)
}

/// 📣 Records the grips the viewport of `domain` just drew, in Tab order; a focus on a grip that is gone (the selection changed) is dropped.
pub fn note_grips(domain: &str, grips: Vec<GripRef>) {
    let mut registry = registry();
    if let Some((_, handle)) = registry.focus.iter().find(|(known, _)| known == domain) {
        if !grips.iter().any(|grip| grip.id == *handle) {
            registry.focus.retain(|(known, _)| known != domain);
        }
    }
    match registry.grips.iter_mut().find(|(known, _)| known == domain) {
        Some((_, slot)) => *slot = grips,
        None => registry.grips.push((domain.to_owned(), grips)),
    }
}

/// 🕹️ The grips last drawn for `domain`.
pub fn noted_grips(domain: &str) -> Vec<GripRef> {
    registry().grips.iter().find(|(known, _)| known == domain).map(|(_, grips)| grips.clone()).unwrap_or_default()
}

/// 🎯️ The grip of `domain` that has the keyboard focus, `None` outside the handle walk.
pub fn focused_grip(domain: &str) -> Option<String> {
    registry().focus.iter().find(|(known, _)| known == domain).map(|(_, handle)| handle.clone())
}

/// 🎯️ Gives the keyboard focus to a grip of `domain`, or takes it away.
pub fn set_focused_grip(domain: &str, handle: Option<String>) {
    let mut registry = registry();
    registry.focus.retain(|(known, _)| known != domain);
    if let Some(handle) = handle {
        registry.focus.push((domain.to_owned(), handle));
    }
}

/// 🚶️ Walks the focus of `domain` over its noted grips and records the result; the step says whether a grip took the focus, the walk ended or there was nothing to walk.
pub fn walk_focus(domain: &str, how: GripWalk) -> GripStep {
    let step = walk_grips(&noted_grips(domain), focused_grip(domain).as_deref(), how);
    match &step {
        GripStep::To(handle) => set_focused_grip(domain, Some(handle.clone())),
        GripStep::Leave => set_focused_grip(domain, None),
        GripStep::Nothing => {}
    }
    step
}
