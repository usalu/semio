//! 🎚️ IO fidelity: the fidelity class operations and the manifest-facing fidelity declaration.
//!
//! Holds the `🔖️IoFidelity` region of the module; the root re-exports its items.

use super::*;

//#region 🔖️IoFidelity


/// 📜 Manifest-facing IO fidelity declaration for a subset dialect.
#[derive(Clone, Debug, PartialEq, Eq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct IoFidelityDeclaration {
    pub class: IoFidelityClass,
    /// ➖️ Field paths dropped under Lossy codecs; must be empty for stronger classes.
    pub drops: Vec<String>,
}

impl IoFidelityDeclaration {
    pub async fn validate(&self) -> Result<(), String> {
        if self.class != IoFidelityClass::Lossy && !self.drops.is_empty() {
            return Err("drops must be empty unless fidelity is lossy".into());
        }
        if self.class == IoFidelityClass::Lossy && self.drops.is_empty() {
            return Err("lossy fidelity requires a non-empty minimal drops set".into());
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "../🧪️tests/🔬️io-fidelity/🦀️.rs"]
mod io_fidelity_tests;
//#endregion 🔖️IoFidelity
