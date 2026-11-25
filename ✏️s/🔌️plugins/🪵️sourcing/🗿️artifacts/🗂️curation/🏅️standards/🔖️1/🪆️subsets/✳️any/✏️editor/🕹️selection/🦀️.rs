//! 🕹️ Sourcing curation selection: the `rows` interaction domain is the stock catalogue in document order, so Select All reaches every kind and a range selection spans the kinds between two picks.

use crate::editor::sourcing::editor_vocabulary::{SOURCING_OBJECT_GRANULARITY, SOURCING_ROWS_DOMAIN};
use crate::CurationSnapshot;
use semio_framework::{DomainTopology, InteractionTopology, TopologyNode};

/// 🧭️ The topology of the `rows` domain: one flat `object` node per stock kind.
pub fn rows_topology(document: &CurationSnapshot) -> InteractionTopology {
    let ordered = document.stock_extra.iter().map(|kind| TopologyNode { id: kind.id.clone(), granularity: SOURCING_OBJECT_GRANULARITY.into(), parent: None }).collect();
    InteractionTopology { domains: std::collections::BTreeMap::from([(SOURCING_ROWS_DOMAIN.to_string(), DomainTopology { ordered })]) }
}
