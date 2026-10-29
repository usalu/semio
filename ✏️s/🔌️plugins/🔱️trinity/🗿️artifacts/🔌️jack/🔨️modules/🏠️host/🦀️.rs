//! 🏠️ Jack retained publication, replay and document host ownership.
#![allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::mutations::TrinityGraphMutation;
use crate::standards::v1::subsets::any::schema::mutations::set_query;
use crate::standards::v1::subsets::any::schema::graph::GraphEffect;
use crate::standards::v1::subsets::any::schema::graph::{Edge, EntityRef, Node, Port};
use crate::standards::v1::subsets::any::schema::snapshot::JackSnapshot;
use semio_framework_graph_manifest::{PropertyBag, PropertyDef, PropertyValue};
use protocol::{Mutation, MutationDiff, OpBinary, OpText};
use semio_framework_diagnostic::TextError;
use semio_framework_value::retained_clone::{admit_retained_clone_close, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_value::{OwnedValueRetirementFactory, SharedValueRetirementFactory};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
#[cfg(test)]
use crate::standards::v1::subsets::any::io::binary::mutations::{encode_op, decode_op};
//#region 🔖️OwnedSprCatalog

#[path = "📸️snapshot-decode/🦀️.rs"]
pub mod snapshot_decode;
pub use snapshot_decode::*;

#[path = "🔄️mutation-decode/🦀️.rs"]
pub mod mutation_decode;
pub use mutation_decode::*;

//#endregion 🔖️OwnedSprCatalog

//#region 🔖️RetainedSnapshotClone

#[path = "📑️snapshot-clone/🦀️.rs"]
pub mod snapshot_clone;
pub use snapshot_clone::*;

//#endregion 🔖️RetainedSnapshotClone

//#region 🧪️Tests
//#endregion 🧪️Tests
