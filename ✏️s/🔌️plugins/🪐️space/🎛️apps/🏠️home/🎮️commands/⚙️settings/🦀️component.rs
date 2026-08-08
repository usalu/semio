//! ⚙️ S Home launcher app — launcher settings commands.
//!
//! One nested `pub mod` per payload (the `app_commands!` shape — see `apps::home::🦀️component.rs`'s
//! `🔖️HomeCommand` region, which `use`s each of these modules flat).

use crate::apps::home::config::{HomeConfig, HomeConfigMutation};
use crate::artifacts::home::op::SHomeMutation;
use crate::artifacts::home::SHomeDocument;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};

//#region 🔖️SetActivePanelTab
pub mod set_active_panel_tab {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "active-panel-tab")]
    pub struct SetActivePanelTab {
        pub tab_id: String,
    }

    pub fn handle(payload: &SetActivePanelTab, _doc: &DocumentView<'_, SHomeDocument>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
        Ok(Emit::config(vec![HomeConfigMutation::SetActivePanelTab { tab_id: payload.tab_id.clone() }]))
    }
}
//#endregion 🔖️SetActivePanelTab

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::HistoryView;

    #[test]
    fn home_command_op_text_round_trips_every_variant() {
        use crate::apps::home::HomeCommand;
        store::test_support::assert_op_line_round_trip(&HomeCommand::SetActivePanelTab(set_active_panel_tab::SetActivePanelTab { tab_id: "tab-1".into() }));
    }

    #[test]
    fn set_active_panel_tab_emits_config_operation() {
        let projection = SHomeDocument { schema: "s.home".into(), catalog_generation: 0 };
        let history = HistoryView::empty();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = HomeConfig::default();
        let cfg = ConfigView { projection: &config };
        let emit = set_active_panel_tab::handle(&set_active_panel_tab::SetActivePanelTab { tab_id: "tab-1".into() }, &doc, &cfg).expect("handle");
        assert_eq!(emit.config_mutations, vec![HomeConfigMutation::SetActivePanelTab { tab_id: "tab-1".into() }]);
        assert!(emit.document_mutations.is_empty());
    }
}
//#endregion 🧪️Tests
