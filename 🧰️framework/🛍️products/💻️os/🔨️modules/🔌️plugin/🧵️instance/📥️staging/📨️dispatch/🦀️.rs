//! 🎴️ Staged slice of the `dispatch` band of the mounted artifact instance: its `instance-app` members (WP6 step S0 of the instance family).
//!
//! 📨️ Dispatch of the mounted artifact instance: test snapshot helpers, command log, history view, patch and refresh, preview, `dispatch_emit` and transaction groups.
use super::*;
use crate::tool_jobs::time_travel;

impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A, M> {
    /// 🧪️ Direct access to the wrapped app — used to assert on app-private test fixtures (e.g.
    /// `TestApp::received_actions`) that a framework-owned interception must never populate.
    #[cfg(test)]
    pub(crate) async fn test_app(&self) -> &A {
        &self.app
    }
}
