//! 🧬️ Schema leaf: canonical Rust mirror of `🔣️.json` for the 🕹️interaction module.
//! Every type here is `pub use`d from the module root (`super`) rather than redefined — `super`
//! itself re-exports `PresenceInteraction`/`PresenceDomain`/`InteractionState`/friends from
//! `semio-framework-replication`, where `PresencePeer.interaction: Option<PresenceInteraction>`
//! is wired directly beside `PresencePeer` in `🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs` without this
//! leaf owning the type.

pub use super::{
    DomainHover, DomainSelection, DomainTopology, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, InteractionState, InteractionTarget, InteractionTopology, MergeMode, PresenceDomain, PresenceInteraction,
    SelectionMethod, SelectionMode, SelectionSpec, TopologyNode,
};
