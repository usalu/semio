//! 🧱️ Neural engine model: dictionary, schema, tree and operator vocabulary with their retirement, hashing and validation laws.

use std::collections::{HashMap, HashSet, VecDeque};
use protocol::causal::transition::HistoryFoldIndex;
use std::sync::Arc;
use std::hash::Hasher;
/// 🔮️ Test-only: production moved to `ToValue`/`FromValue` below (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/01, tenth-seam pass — see `📓️orderedmap-tenth-seam.md`).
#[cfg(test)]
use serde::{Deserialize, Serialize};
use protocol::value::ordered::OrderedMap;
use protocol::value::{DslValue, FromValue, Number, ToValue, ValueError};
use semio_framework_value::{ValueKind, ValueType};
pub use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};

#[path = "../🧵️retirement/🦀️.rs"]
pub mod retirement;
pub use retirement::{ColdDictionaryBuilder,ColdValueOwner,ValueRetirement};
#[path = "../🧊️cold/🦀️.rs"]
pub mod cold;
pub use cold::{ColdOwner, ColdRetire};
#[cfg(any(test, feature = "testkit"))]
#[path = "🧪️tests/🧰️testkit/🦀️.rs"]
pub mod testkit;

//#region 🔖️Children

#[path = "📚️dictionary/🦀️.rs"]
mod dictionary;
pub use dictionary::*;

#[path = "🏷️schema-model/🦀️.rs"]
mod schema_model;
pub use schema_model::*;

#[path = "🌳️neuron-tree/🦀️.rs"]
mod neuron_tree;
pub use neuron_tree::*;

#[path = "⚙️operator-record/🦀️.rs"]
mod operator_record;
pub use operator_record::*;

#[path = "🔏️hash-and-dirty-set/🦀️.rs"]
mod hash_and_dirty_set;
pub use hash_and_dirty_set::*;

#[path = "🧮️evaluator/🦀️.rs"]
mod evaluator;
pub use evaluator::*;

//#endregion 🔖️Children

#[path="../📖️lookup/🦀️.rs"]
mod original_lookup;
pub use original_lookup::OriginalDictionaryLookup;

#[path="../📤️projection/🦀️.rs"]
mod dictionary_projection;

#[path = "../🚦️native/🦀️.rs"]
mod native_controlled;
#[path="../⚠️failure/🦀️.rs"]
mod original_failure_projection;
