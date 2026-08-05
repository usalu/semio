//! 🧮️ Block 5D play app — the view-state config artifact and its operation enum.
//!
//! Session-only but real, undoable config: it round-trips through the config `DocumentStore` exactly
//! like document content, with a true `backwards` per operation. Nothing here is document state — the
//! part kind's identity/presentations/grips live in `crate::artifacts::block5d`.

use protocol::Operation;
use serde::{Deserialize, Serialize};

//#region 🔖️Config
/// 🧮️ `Block5dPlayApp`'s real `DocumentApp::Config` — B1 pure-trait conversion. Absorbs the former
/// `Block5dPlayApp::selected_ids` `RefCell` field plus the locale this app resolves itself.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "block5dcfg")]
#[dsl(layout = "lines")]
pub struct Block5dConfig {
    /// 👁️ Multi-selected row ids in the document tree — was `Block5dPlayApp::selected_ids`.
    pub selected_ids: Vec<String>,
    /// 🗣️ BCP-47 locale tag — was read off the deleted `ViewState.locale`.
    pub locale: String,
}

impl Default for Block5dConfig {
    fn default() -> Self {
        Self { selected_ids: Vec::new(), locale: "en-US".into() }
    }
}

store::impl_whole_record_config!(Block5dConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// 🧮️ `Block5dConfig`'s operation enum — one variant per settled interaction (mirrors the pre-B1
/// `Block5dPlayApp` `RefCell` field write), plus a generic `Snapshot` every variant's `backwards()`
/// returns.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum Block5dConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: Block5dConfig,
    },
    #[dsl(key = "selection")]
    SetSelection { ids: Vec<String> },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

impl Operation<Block5dConfig> for Block5dConfigOperation {
    type Diff = Block5dConfig;

    fn diff(&self, base: &Block5dConfig) -> Block5dConfig {
        let mut next = base.clone();
        match self {
            Block5dConfigOperation::Snapshot { config } => return config.clone(),
            Block5dConfigOperation::SetSelection { ids } => next.selected_ids = ids.clone(),
            Block5dConfigOperation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn backwards(&self, base: &Block5dConfig) -> Vec<Self> {
        vec![Block5dConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block5d_config_default_has_no_selection() {
        let config = Block5dConfig::default();
        assert!(config.selected_ids.is_empty());
        assert_eq!(config.locale, "en-US");
    }

    #[test]
    fn config_operation_backwards_restores_the_pre_operation_snapshot() {
        let base = Block5dConfig::default();
        let operation = Block5dConfigOperation::SetSelection { ids: vec!["g0".into()] };
        let next = operation.diff(&base);
        assert_eq!(next.selected_ids, vec!["g0".to_string()]);
        let inverse = operation.backwards(&base);
        assert_eq!(inverse, vec![Block5dConfigOperation::Snapshot { config: base.clone() }]);
        assert_eq!(inverse[0].diff(&next), base);
    }
}
//#endregion 🧪️Tests
