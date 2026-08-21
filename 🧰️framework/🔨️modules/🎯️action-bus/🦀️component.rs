// #region action_bus
//! 🎯️ Action routing between renderer and app controllers.
//!
//! `ActionBus` is an ephemeral controller dispatch table for native chrome prototypes — production
//! shells route through OS `ArtifactApp::dispatch_action`; this type is not durable CORE state.

use dsl::DslValue;
use std::collections::HashMap;

pub trait ActionHandler: Send {
    async fn id(&self) -> &str;
    async fn handle(&mut self, action: &str, args: Option<&DslValue>) -> Vec<String>;
}

// 🧬️ `ActionBus` used to hold `Box<dyn ActionHandler>` — a registry meant to carry more than one
// concrete controller type per instance. R11: an open-shaped registry like this becomes generic
// (`H: ActionHandler`) rather than `dyn ActionHandler`; a caller that genuinely needs to mix several
// concrete handler types in one bus declares its own closed enum over just the types it needs (same
// shape as `machine::persist::Migration`/`NoMigrations`) and instantiates `ActionBus<ThatEnum>`. Only
// `EchoHandler` (test-only) implements `ActionHandler` today — every real controller currently routes
// through OS `ArtifactApp::dispatch_action` per this file's own doc comment — so production callers
// that never register anything use `NoActionHandlers` below.
pub struct ActionBus<H: ActionHandler> {
    controllers: HashMap<String, H>,
}

/// 🈳️ Placeholder [`ActionHandler`] for an [`ActionBus`] that never registers a controller —
/// uninhabited (hand-written rather than via `dyn_enum_close!`, which would need this module's own
/// `semio-framework-dispatch-macros` dependency; a zero-variant enum needs no macro to prove
/// unreachable), so `ActionBus<NoActionHandlers>` structurally documents "nothing registers here yet"
/// at the type level instead of by convention. Widen to a real closed enum once a first production
/// [`ActionHandler`] impl exists.
pub enum NoActionHandlers {}

impl ActionHandler for NoActionHandlers {
    async fn id(&self) -> &str {
        match *self {}
    }

    async fn handle(&mut self, _action: &str, _args: Option<&DslValue>) -> Vec<String> {
        match *self {}
    }
}

impl<H: ActionHandler> Default for ActionBus<H> {
    fn default() -> Self {
        Self::new()
    }
}

impl<H: ActionHandler> ActionBus<H> {
    // 🚫️async: E1 `Default::default` (external trait) is the only non-test consumer and cannot be
    // async; `new` itself does no I/O, so R9 keeps it sync rather than making `Default` a dead end.
    pub fn new() -> Self {
        Self { controllers: HashMap::new() }
    }

    pub async fn register(&mut self, handler: H) {
        let id = handler.id().await.to_string();
        self.controllers.insert(id, handler);
    }

    pub async fn unregister(&mut self, controller_id: &str) {
        self.controllers.remove(controller_id);
    }

    pub async fn dispatch(&mut self, controller_id: &str, action: &str, args: Option<&DslValue>) -> Vec<String> {
        match self.controllers.get_mut(controller_id) {
            Some(handler) => handler.handle(action, args).await,
            None => Vec::new(),
        }
    }
}

/// @emoji 🔀️ Bridges staged `serde_json::Value` action args into `ActionDescriptor.args`.
pub fn optional_json_to_dsl(args: Option<serde_json::Value>) -> Option<DslValue> {
    args.map(|value| dsl::to_dsl_value(&value).unwrap_or(DslValue::Null))
}

#[cfg(test)]
mod tests {
    use super::*;

    struct EchoHandler {
        id: String,
    }

    impl ActionHandler for EchoHandler {
        async fn id(&self) -> &str {
            &self.id
        }

        async fn handle(&mut self, action: &str, _args: Option<&DslValue>) -> Vec<String> {
            vec![format!("{action}:ok")]
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn dispatches_to_registered_handler() {
        let mut bus: ActionBus<EchoHandler> = ActionBus::new();
        bus.register(EchoHandler { id: "app".into() }).await;
        let operations = bus.dispatch("app", "ping", None).await;
        assert_eq!(operations, vec!["ping:ok"]);
    }

    #[semio_framework_async_macros::async_test]
    async fn dispatch_to_unknown_controller_returns_empty() {
        let mut bus: ActionBus<EchoHandler> = ActionBus::new();
        assert!(bus.dispatch("missing", "ping", None).await.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn unregister_removes_handler_so_dispatch_becomes_noop() {
        let mut bus: ActionBus<EchoHandler> = ActionBus::new();
        bus.register(EchoHandler { id: "app".into() }).await;
        bus.unregister("app").await;
        assert!(bus.dispatch("app", "ping", None).await.is_empty());
    }
}
// #endregion action_bus
