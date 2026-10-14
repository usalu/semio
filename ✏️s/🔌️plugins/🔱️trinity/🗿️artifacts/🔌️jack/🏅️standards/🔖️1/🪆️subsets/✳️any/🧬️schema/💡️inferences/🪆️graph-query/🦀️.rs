//! 🪆️ Borrowed graph behavior owned by Jack's inference consumer.
use semio_framework_schema_composition::ChildRefFields;
use semio_framework_value::{ValueError, ValueRefusalKind};

/// 🧷️ Literal endpoint coordinates borrowed from the retained graph owner.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JackGraphEndpoint<'a> {
    pub id: &'a str,
    pub source: &'a str,
    pub target: &'a str,
}

/// 🔎️ Reads identities and numeric displacement without a structural graph projection.
pub trait JackGraphQuery {
    fn identity(&self) -> ChildRefFields<'_>;
    fn node_count(&self) -> usize;
    fn node_id(&self, index: usize) -> Option<&str>;
    fn edge_count(&self) -> usize;
    fn edge(&self, index: usize) -> Option<JackGraphEndpoint<'_>>;
    fn displacement(&self, edge_id: &str) -> Result<(f64, f64), ValueError>;
}

/// 🔤️ Uses the shared literal UTF-16 order without allocating normalized identities.
pub(crate) fn identity_order(left:&str,right:&str)->std::cmp::Ordering{left.encode_utf16().cmp(right.encode_utf16())}

/// 🎯️ Admits all five coordinates before asking a graph owner for an entity.
pub fn retained_jack_graph<'a>(selected: ChildRefFields<'_>, graph: &'a dyn JackGraphQuery) -> Result<&'a dyn JackGraphQuery, ValueError> {
    if selected != graph.identity() {
        return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "Jack graph capability does not address the selected child"));
    }
    Ok(graph)
}

#[path="🧭topology/🦀️.rs"]
mod topology;
#[path="🎛️flat-position/🦀️.rs"]
mod flat_position;
pub use topology::{JackTopology,infer_jack_topology};
pub use flat_position::{JackFlatPosition,JackFlatPositionUv,infer_jack_flat_position};

#[cfg(test)]
#[path = "../🧪️tests/🪆️query/🦀️.rs"]
mod tests;

#[path="👁️read/🦀️.rs"]
mod rich_read;
pub use rich_read::{JackContentNodeView,JackContentPortView,JackContentEdgeView,JackContentVisual};
