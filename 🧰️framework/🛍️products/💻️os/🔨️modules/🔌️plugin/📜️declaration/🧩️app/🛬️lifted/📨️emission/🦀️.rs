use semio_framework_plugin_window_transient::window_transient::WindowTransientMutation;
use crate::ArtifactApp;

/// 👥️🫧️ The two EPHEMERAL lanes' emission, deliberately separate from {@link Emit}.
///
/// 🎯️ Why not more fields on `Emit`: the document lanes (artifact/config/draft) all have an op
/// log, an edit id, an undo group and a failure mode; presence and transient have NONE of those.
/// They cannot fail, cannot be undone, never enter a checkpoint and never appear in the command
/// log. Folding them into `Emit` would put five type parameters on ~1000 signatures to express a
/// thing that shares none of `Emit`'s machinery — and would force every app that emits no
/// presence to name its presence type anyway.
///
/// Apps opt in by overriding {@link ArtifactApp::ephemeral}; the default emits nothing, so an
/// app with no shareable or UI-local state writes no code at all.
#[derive(Debug)]
pub struct EphemeralEmit<A: ArtifactApp> {
    /// 👥️ Ephemeral SHARED — broadcast to peers, never persisted.
    pub presence: Vec<A::PresenceMutation>,
    /// 🫧️ Ephemeral LOCAL-ONLY — never leaves this client.
    pub transient: Vec<A::TransientMutation>,
    /// 🪟️ Ephemeral LOCAL-ONLY state addressed to an exact concrete window instance.
    pub window_transient: Vec<WindowTransientMutation>,
}

impl<A: ArtifactApp> Default for EphemeralEmit<A> {
    fn default() -> Self {
        Self { presence: Vec::new(), transient: Vec::new(), window_transient: Vec::new() }
    }
}

impl<A: ArtifactApp> EphemeralEmit<A> {
    /// 👥️ Presence-only emission — the common case (a moved cursor, a changed selection).
    pub async fn presence(presence: Vec<A::PresenceMutation>) -> Self {
        Self { presence, transient: Vec::new(), window_transient: Vec::new() }
    }

    /// 🫧️ Transient-only emission — the common case (a hover, an in-flight gesture).
    pub async fn transient(transient: Vec<A::TransientMutation>) -> Self {
        Self { presence: Vec::new(), transient, window_transient: Vec::new() }
    }

    pub async fn window_transient(window_transient: Vec<WindowTransientMutation>) -> Self {
        Self { presence: Vec::new(), transient: Vec::new(), window_transient }
    }

    /// 🈳️ Whether this emission touches neither ephemeral lane.
    pub async fn is_empty(&self) -> bool {
        self.presence.is_empty() && self.transient.is_empty() && self.window_transient.is_empty()
    }
}
