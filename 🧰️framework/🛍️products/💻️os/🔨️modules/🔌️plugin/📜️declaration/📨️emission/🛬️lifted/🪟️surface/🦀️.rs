use semio_framework::kernel::UiDirtyScope;
use semio_framework::kernel::Effect;
use semio_framework_ui_contract::*;

/// 👁️ What a viewer may emit — structurally cannot carry an artifact or draft mutation (contract
/// §2.2): the read-only guarantee is a type property, not a runtime check.
pub struct ViewEmit<ConfigMutation> {
    pub config_mutations: Vec<ConfigMutation>,
    pub effects: Vec<Effect>,
    pub ui_dirty: UiDirtyScope,
}

impl<ConfigMutation> Default for ViewEmit<ConfigMutation> {
    fn default() -> Self {
        Self { config_mutations: Vec::new(), effects: Vec::new(), ui_dirty: UiDirtyScope::default() }
    }
}

impl<ConfigMutation> ViewEmit<ConfigMutation> {
    pub fn new() -> Self {
        Self::default()
    }
    /// 🧮️ A config-operation emission carrying `config_mutations` and nothing else.
    pub fn config(config_mutations: Vec<ConfigMutation>) -> Self {
        Self { config_mutations, ..Default::default() }
    }
    /// 🐚️ A single host effect and no config operations.
    pub fn effect(effect: Effect) -> Self {
        Self { effects: vec![effect], ..Default::default() }
    }
    /// 🐢️ Narrows which rendered UI sections this emission invalidates.
    pub fn dirty(ui_dirty: UiDirtyScope) -> Self {
        Self { ui_dirty, ..Default::default() }
    }
}
