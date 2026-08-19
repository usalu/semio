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

pub struct ActionBus {
    controllers: HashMap<String, Box<dyn ActionHandler>>,
}

impl Default for ActionBus {
    fn default() -> Self {
        Self::new()
    }
}

impl ActionBus {
    pub async fn new() -> Self {
        Self {
            controllers: HashMap::new(),
        }
    }

    pub async fn register(&mut self, handler: Box<dyn ActionHandler>) {
        let id = handler.id().to_string();
        self.controllers.insert(id, handler);
    }

    pub async fn unregister(&mut self, controller_id: &str) {
        self.controllers.remove(controller_id);
    }

    pub async fn dispatch(&mut self, controller_id: &str, action: &str, args: Option<&DslValue>) -> Vec<String> {
        self.controllers
            .get_mut(controller_id)
            .map(|handler| handler.handle(action, args))
            .unwrap_or_default()
    }
}

/// @emoji 🔀️ Bridges staged `serde_json::Value` action args into `ActionDescriptor.args`.
pub async fn optional_json_to_dsl(args: Option<serde_json::Value>) -> Option<DslValue> {
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

    #[test]
    async fn dispatches_to_registered_handler() {
        let mut bus = ActionBus::new();
        bus.register(Box::new(EchoHandler { id: "app".into() }));
        let operations = bus.dispatch("app", "ping", None);
        assert_eq!(operations, vec!["ping:ok"]);
    }

    #[test]
    async fn dispatch_to_unknown_controller_returns_empty() {
        let mut bus = ActionBus::new();
        assert!(bus.dispatch("missing", "ping", None).is_empty());
    }

    #[test]
    async fn unregister_removes_handler_so_dispatch_becomes_noop() {
        let mut bus = ActionBus::new();
        bus.register(Box::new(EchoHandler { id: "app".into() }));
        bus.unregister("app");
        assert!(bus.dispatch("app", "ping", None).is_empty());
    }
}
// #endregion action_bus
