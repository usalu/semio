//! 🎚️ IO fidelity: the fidelity class operations and the manifest-facing fidelity declaration.
//!
//! Holds the `🔖️IoFidelity` region of the module; the root re-exports its items.

use super::*;

//#region 🔖️IoFidelity


impl IoFidelityClass {
    pub async fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Canonical => "canonical",
            Self::Semantic => "semantic",
            Self::Lossy => "lossy",
        }
    }

    pub async fn parse(s: &str) -> Result<Self, String> {
        match s {
            "exact" => Ok(Self::Exact),
            "canonical" => Ok(Self::Canonical),
            "semantic" => Ok(Self::Semantic),
            "lossy" => Ok(Self::Lossy),
            other => Err(format!("unknown io fidelity class {other:?}")),
        }
    }

    /// Ordered strength: Exact > Canonical > Semantic > Lossy
    pub async fn rank(self) -> u8 {
        match self {
            Self::Exact => 3,
            Self::Canonical => 2,
            Self::Semantic => 1,
            Self::Lossy => 0,
        }
    }
}

/// 📜 Manifest-facing IO fidelity declaration for a subset dialect.
#[derive(Clone, Debug, PartialEq, Eq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct IoFidelityDeclaration {
    pub class: IoFidelityClass,
    /// Field paths dropped under Lossy codecs; must be empty for stronger classes.
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
