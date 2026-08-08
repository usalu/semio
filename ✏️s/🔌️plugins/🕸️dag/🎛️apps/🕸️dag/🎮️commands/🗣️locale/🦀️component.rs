//! 🗣️ DAG play app commands — the host-pushed locale change.
//!
//! Not declared as a manifest action (locale is host-pushed, not a user-facing app action needing a
//! palette entry), which is why its wire keyword stays the bare `"locale"` rather than the kebab-cased
//! `"set-locale"` its command id would suggest — mirrors `shooting_protocol::ShootingCommand::SetLocale`'s
//! equally-undeclared precedent; see the `as` literal in `crate::apps::dag`'s `app_commands!` invocation.

use crate::apps::dag::config::{DagConfig, DagConfigMutation};
use crate::artifacts::dag::op::DagMutation;
use crate::artifacts::dag::DagDocument;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetLocale
pub mod set_locale {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "locale")]
    pub struct SetLocale {
        pub value: String,
    }

    pub fn handle(payload: &SetLocale, _doc: &DocumentView<'_, DagDocument>, _cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
        Ok(Emit::config(vec![DagConfigMutation::SetLocale { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️SetLocale

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::dag::testkit;
    use crate::apps::dag::{DagCommand, DAG_PLAY_BODY_DOCUMENT};
    use semio_framework_plugin::PluginApp;

    #[test]
    fn dag_play_labels_resolve_native_english_and_german() {
        let mut app = testkit::new_app();
        let node = app.render(DAG_PLAY_BODY_DOCUMENT, None, &semio_framework_plugin::ViewModel::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Nodes"));
        assert!(json.contains("Edges"));

        app.dispatch_typed(DagCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }), &semio_framework_plugin::testkit::meta("local")).expect("set locale");
        let node = app.render(DAG_PLAY_BODY_DOCUMENT, None, &semio_framework_plugin::ViewModel::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Knoten"));
        assert!(json.contains("Kanten"));
    }
}
//#endregion 🧪️Tests
