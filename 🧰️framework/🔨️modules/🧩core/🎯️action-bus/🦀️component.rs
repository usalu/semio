// #region action_bus
//! 🎯️ Action routing between renderer and app controllers.

use dsl::DslValue;
use std::collections::HashMap;

pub trait ActionHandler: Send {
    fn id(&self) -> &str;
    fn handle(&mut self, action: &str, args: Option<&DslValue>) -> Vec<String>;
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
    pub fn new() -> Self {
        Self {
            controllers: HashMap::new(),
        }
    }

    pub fn register(&mut self, handler: Box<dyn ActionHandler>) {
        let id = handler.id().to_string();
        self.controllers.insert(id, handler);
    }

    pub fn unregister(&mut self, controller_id: &str) {
        self.controllers.remove(controller_id);
    }

    pub fn dispatch(&mut self, controller_id: &str, action: &str, args: Option<&DslValue>) -> Vec<String> {
        self.controllers
            .get_mut(controller_id)
            .map(|handler| handler.handle(action, args))
            .unwrap_or_default()
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
        fn id(&self) -> &str {
            &self.id
        }

        fn handle(&mut self, action: &str, _args: Option<&DslValue>) -> Vec<String> {
            vec![format!("{action}:ok")]
        }
    }

    #[test]
    fn dispatches_to_registered_handler() {
        let mut bus = ActionBus::new();
        bus.register(Box::new(EchoHandler { id: "app".into() }));
        let operations = bus.dispatch("app", "ping", None);
        assert_eq!(operations, vec!["ping:ok"]);
    }

    #[test]
    fn dispatch_to_unknown_controller_returns_empty() {
        let mut bus = ActionBus::new();
        assert!(bus.dispatch("missing", "ping", None).is_empty());
    }

    #[test]
    fn unregister_removes_handler_so_dispatch_becomes_noop() {
        let mut bus = ActionBus::new();
        bus.register(Box::new(EchoHandler { id: "app".into() }));
        bus.unregister("app");
        assert!(bus.dispatch("app", "ping", None).is_empty());
    }
}
// #endregion action_bus
