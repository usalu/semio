//! 🗣️ 🗣️ DAG play app commands command — `set-locale`.

use crate::editor::dag::config::{DagConfig, DagConfigMutation};
use crate::artifacts::dag::op::DagMutation;
use crate::artifacts::dag::DagSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "locale")]
pub struct SetLocale {
    pub value: String,
}

pub async fn handle(payload: &SetLocale, _doc: &ArtifactView<'_, DagSnapshot>, _cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
    Ok(Emit::config(vec![DagConfigMutation::SetLocale { value: payload.value.clone() }]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::dag::testkit;
    use crate::editor::dag::{DagCommand, DAG_PLAY_BODY_DOCUMENT};
    use semio_framework_plugin::PluginApp;

    #[test]
    async fn dag_play_labels_resolve_native_english_and_german() {
        let mut app = testkit::new_app();
        let node = app.render(DAG_PLAY_BODY_DOCUMENT, None, &semio_framework_plugin::ViewModel::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Nodes"));
        assert!(json.contains("Edges"));

        app.dispatch_typed(DagCommand::SetLocale(SetLocale { value: "de-DE".into() }), &semio_framework_plugin::testkit::meta("local")).expect("set locale");
        let node = app.render(DAG_PLAY_BODY_DOCUMENT, None, &semio_framework_plugin::ViewModel::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Knoten"));
        assert!(json.contains("Kanten"));
    }
}
//#endregion 🧪️Tests
