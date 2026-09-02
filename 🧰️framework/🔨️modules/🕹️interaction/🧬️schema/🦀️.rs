//! 🧬️ Schema leaf: canonical Rust mirror of `🔣️.json` for the 🕹️interaction module.
//! Every type here is `pub use`d from the module root (`super`) rather than redefined — `super`
//! itself re-exports `PresenceInteraction`/`PresenceDomain`/`InteractionState`/friends from
//! `semio-framework-os-kernel` (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM
//! crate-layering fix), the same relocation that let `PresencePeer.interaction: Option<PresenceInteraction>`
//! be wired directly beside `PresencePeer` in `📡️spr/📡️wire/🦀️.rs` (wave 2a) without this
//! leaf owning the type.

pub use super::{
    DomainHover, DomainSelection, DomainTopology, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, InteractionState, InteractionTarget, InteractionTopology, MergeMode, PresenceDomain, PresenceInteraction,
    SelectionMethod, SelectionMode, SelectionSpec, TopologyNode,
};
