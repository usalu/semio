//! 🕸️ Pure graph foundation: topology markers, node/handle/edge kinds, and index-based algorithms; the interactive board engine lives in `infinite_board`.

use std::collections::{BTreeMap, BTreeSet};

use semio_framework_graph_manifest::{PropertyBag, PropertyValue};

// #region 🔖️Ids
/// 🧩️ Stable node identifier.
pub type NodeId = u64;
/// 🪝️ Stable handle identifier.
pub type HandleId = u64;
/// 🪢️ Stable edge identifier.
pub type EdgeId = u64;
// #endregion 🔖️Ids

// #region 🔖️Edge
/// 🪢️ Edge with typed endpoints (node id or handle id).
// 🧬️ `semio_framework_value_derive::{ToValue, FromValue}` additive (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01) — never had `serde`; `E` is always `P::Endpoint` (`NodeId`/`HandleId`, both `u64`) in
// practice, so the derive's auto-synthesized `E: ToValue + FromValue` bound is always satisfied.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub struct CoreEdge<E> {
    pub id: EdgeId,
    pub source: E,
    pub target: E,
}

impl<E: Copy + Ord> CoreEdge<E> {
    /// 📐️ Normalize endpoints for undirected storage.
    pub fn normalize_undirected(source: E, target: E) -> (E, E) {
        if source <= target {
            (source, target)
        } else {
            (target, source)
        }
    }
}
// #endregion 🔖️Edge

// #region 🔖️Directedness
/// ↔ Compile-time directed vs undirected graph axis.
pub trait Directedness {
    const DIRECTED: bool;
}

/// ➡️ Directed edges keep source→target order.
// 🧬️ ToValue/FromValue additive (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01), hand-written rather than derived: `#[derive(ToValue, FromValue)]` rejects a
// SEMICOLON-terminated unit struct (`Fields::Unit`, distinct from an empty-brace `struct Foo {}`,
// which IS `Fields::Named`) — changing to brace form would ripple into every value-level
// construction site of this marker type. Zero-field, so the wire shape is `null` (serde's own
// default for a unit struct, had this type ever derived `Serialize`).
#[derive(Clone, Copy, Debug, Default)]
pub struct Directed;

impl semio_framework_value::ToValue for Directed {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::Null
    }
}
impl semio_framework_value::FromValue for Directed {
    fn from_value(_value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        Ok(Directed)
    }
}

impl Directedness for Directed {
    const DIRECTED: bool = true;
}

/// ↔ Undirected edges store ordered endpoint pair.
// 🧬️ Hand-written ToValue/FromValue, see `Directed` above.
#[derive(Clone, Copy, Debug, Default)]
pub struct Undirected;

impl semio_framework_value::ToValue for Undirected {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::Null
    }
}
impl semio_framework_value::FromValue for Undirected {
    fn from_value(_value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        Ok(Undirected)
    }
}

impl Directedness for Undirected {
    const DIRECTED: bool = false;
}

/// 📐️ Apply directedness when storing edge endpoints.
#[inline]
pub fn orient_endpoints<E: Copy + Ord, D: Directedness>(source: E, target: E) -> (E, E) {
    if D::DIRECTED {
        (source, target)
    } else {
        CoreEdge::<E>::normalize_undirected(source, target)
    }
}
// #endregion 🔖️Directedness

// #region 🔖️PortModel
/// 🔌️ Compile-time normal (node) vs ported (handle) graph axis.
pub trait PortModel {
    type Endpoint: Copy + Ord + std::fmt::Debug;
    const HAS_PORTS: bool;
    /// 🪢️ Whether this port model allows parallel edges between the same pair (the port axis IS the multi-edge axis: `Ported` ~ NetworkX `Multi(Di)Graph`, `Normal` ~ NetworkX `(Di)Graph`).
    const MULTI_EDGES: bool;
    fn endpoint_as_u64(endpoint: Self::Endpoint) -> u64;
    fn try_handle_endpoint(handle_id: HandleId) -> Option<Self::Endpoint>;
    fn endpoint_as_handle(endpoint: Self::Endpoint) -> Option<HandleId>;
}

/// 🟠️ Node-to-node edges without handles.
// 🧬️ Hand-written ToValue/FromValue (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01), not derived — see `Directed`'s note on why a semicolon-terminated unit struct can't
// use `#[derive(ToValue, FromValue)]`.
#[derive(Clone, Copy, Debug, Default)]
pub struct Normal;

impl semio_framework_value::ToValue for Normal {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::Null
    }
}
impl semio_framework_value::FromValue for Normal {
    fn from_value(_value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        Ok(Normal)
    }
}

impl PortModel for Normal {
    type Endpoint = NodeId;
    const HAS_PORTS: bool = false;
    const MULTI_EDGES: bool = false;
    fn endpoint_as_u64(endpoint: Self::Endpoint) -> u64 {
        endpoint
    }
    fn try_handle_endpoint(_: HandleId) -> Option<Self::Endpoint> {
        None
    }
    fn endpoint_as_handle(_: Self::Endpoint) -> Option<HandleId> {
        None
    }
}

/// 🪝️ Handle-to-handle edges on nodes.
// 🧬️ Hand-written ToValue/FromValue, see `Normal` above.
#[derive(Clone, Copy, Debug, Default)]
pub struct Ported;

impl semio_framework_value::ToValue for Ported {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::Null
    }
}
impl semio_framework_value::FromValue for Ported {
    fn from_value(_value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        Ok(Ported)
    }
}

impl PortModel for Ported {
    type Endpoint = HandleId;
    const HAS_PORTS: bool = true;
    const MULTI_EDGES: bool = true;
    fn endpoint_as_u64(endpoint: Self::Endpoint) -> u64 {
        endpoint
    }
    fn try_handle_endpoint(handle_id: HandleId) -> Option<Self::Endpoint> {
        Some(handle_id)
    }
    fn endpoint_as_handle(endpoint: Self::Endpoint) -> Option<HandleId> {
        Some(endpoint)
    }
}
// #endregion 🔖️PortModel

#[path = "🗄️storage/🦀️.rs"]
mod graph_storage;
pub use graph_storage::*;

#[path = "🪟️views/🦀️.rs"]
mod graph_views;
pub use graph_views::*;

#[path = "🧰️support/🦀️.rs"]
mod graph_support;
pub use graph_support::*;

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests

use geometry::Point;
#[path = "🔖️kinds/🦀️.rs"]
mod graph_kinds;
pub use graph_kinds::*;

#[path = "🌊️max-flow/🦀️.rs"]
mod graph_flow;
pub use graph_flow::*;
