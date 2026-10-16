//! ⚖️ Flow mutation catalogue laws: direct-leaf contracts, structural diff ownership and typed mutation retirement, driven through the public document API.
pub use semio_framework_artifact_flow_flow_document::*;

#[cfg(test)]
use std::sync::Arc;

#[cfg(test)]
use crate::os_spr::{ApplyCapability, DiffAlgebra, Identified, MutationApplyResult, MutationDiff};

#[cfg(test)]
#[path = "../🧪️tests/🌿️vcs/🦀️.rs"]
mod flow_direct_tests;

#[cfg(test)]
#[path = "../🧪️tests/🔬️leaf-contracts/🦀️.rs"]
mod leaf_contracts;

//#region 🧪️Ownership
#[cfg(test)]
#[path = "../🧬️schema/🔺️diff/🧪️tests/🧾️ownership/🦀️.rs"]
mod ownership_tests;
//#endregion 🧪️Ownership

#[cfg(test)]
#[path = "../🧬️schema/🧹️retirement/🧪️tests/🧹️retirement/🦀️.rs"]
mod flow_mutation_retirement;
