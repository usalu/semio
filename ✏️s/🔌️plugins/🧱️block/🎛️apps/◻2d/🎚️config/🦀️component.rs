//! 🧮️ Block 2D play app — the view-state config artifact and its operation enum.
//!
//! Session-only but real, undoable config: it round-trips through the config `DocumentStore` exactly
//! like document content, with a true `backwards` per operation. Nothing here is document state — the
//! node kind's identity/presentation/handles live in `crate::artifacts::block2d`.

use protocol::Operation;
use serde::{Deserialize, Serialize};

//#region 🔖️Config
/// 🧮️ `Block2dPlayApp`'s real `DocumentApp::Config` — B1 pure-trait conversion. Absorbs the former
/// `Block2dPlayApp::selected_ids` `RefCell` field plus the locale this app resolves itself.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "block2dcfg")]
#[dsl(layout = "lines")]
pub struct Block2dConfig {
    /// 👁️ Multi-selected row ids in the document tree — was `Block2dPlayApp::selected_ids`.
    pub selected_ids: Vec<String>,
    /// 🗣️ BCP-47 locale tag — was read off the deleted `ViewModel.locale`.
    pub locale: String,
}

impl Default for Block2dConfig {
    fn default() -> Self {
        Self { selected_ids: Vec::new(), locale: "en-US".into() }
    }
}

store::impl_whole_record_config!(Block2dConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// 🧮️ `Block2dConfig`'s operation enum — one variant per settled interaction (mirrors the pre-B1
/// `Block2dPlayApp` `RefCell` field write), plus a generic `Snapshot` every variant's `backwards()`
/// returns.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum Block2dConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: Block2dConfig,
    },
    #[dsl(key = "selection")]
    SetSelection { ids: Vec<String> },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

impl Operation<Block2dConfig> for Block2dConfigOperation {
    type Diff = Block2dConfig;

    fn diff(&self, base: &Block2dConfig) -> Block2dConfig {
        let mut next = base.clone();
        match self {
            Block2dConfigOperation::Snapshot { config } => return config.clone(),
            Block2dConfigOperation::SetSelection { ids } => next.selected_ids = ids.clone(),
            Block2dConfigOperation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn backwards(&self, base: &Block2dConfig) -> Vec<Self> {
        vec![Block2dConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block2d_config_default_has_no_selection() {
        let config = Block2dConfig::default();
        assert!(config.selected_ids.is_empty());
        assert_eq!(config.locale, "en-US");
    }

    #[test]
    fn config_operation_backwards_restores_the_pre_operation_snapshot() {
        let base = Block2dConfig::default();
        let operation = Block2dConfigOperation::SetSelection { ids: vec!["h0".into()] };
        let next = operation.diff(&base);
        assert_eq!(next.selected_ids, vec!["h0".to_string()]);
        let inverse = operation.backwards(&base);
        assert_eq!(inverse, vec![Block2dConfigOperation::Snapshot { config: base.clone() }]);
        assert_eq!(inverse[0].diff(&next), base);
    }
}
//#endregion 🧪️Tests
