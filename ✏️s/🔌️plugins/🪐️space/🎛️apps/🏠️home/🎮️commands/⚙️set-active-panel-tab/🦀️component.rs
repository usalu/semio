//! ⚙️ ⚙️ S Home launcher app command — `set-active-panel-tab`.

use crate::apps::home::config::{HomeConfig, HomeConfigMutation};
use crate::artifacts::home::op::SHomeMutation;
use crate::artifacts::home::SHomeSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "active-panel-tab")]
pub struct SetActivePanelTab {
    pub tab_id: String,
}

pub fn handle(payload: &SetActivePanelTab, _doc: &ArtifactView<'_, SHomeSnapshot>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    Ok(Emit::config(vec![HomeConfigMutation::SetActivePanelTab { tab_id: payload.tab_id.clone() }]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::HistoryView;

    #[test]
    fn home_command_op_text_round_trips_every_variant() {
        use crate::apps::home::HomeCommand;
        store::os_store::test_support::assert_op_line_round_trip(&HomeCommand::SetActivePanelTab(SetActivePanelTab { tab_id: "tab-1".into() }));
    }

    #[test]
    fn set_active_panel_tab_emits_config_operation() {
        let projection = SHomeSnapshot { schema: "s.home".into(), catalog_generation: 0 };
        let history = HistoryView::empty();
        let doc = ArtifactView::new(&projection, &history);
        let config = HomeConfig::default();
        let cfg = ConfigView { snapshot: &config };
        let emit = handle(&SetActivePanelTab { tab_id: "tab-1".into() }, &doc, &cfg).expect("handle");
        assert_eq!(emit.config_mutations, vec![HomeConfigMutation::SetActivePanelTab { tab_id: "tab-1".into() }]);
        assert!(emit.artifact_mutations.is_empty());
    }
}
//#endregion 🧪️Tests
