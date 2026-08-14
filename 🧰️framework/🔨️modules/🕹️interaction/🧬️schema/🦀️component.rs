//! 🧬️ Schema leaf: canonical Rust mirror of `🔣️component.json` for the 🕹️interaction module.
//! `InteractionDefinition`/`InteractionState`/friends are re-exported from the module root
//! (`super`) rather than redefined here — this leaf only newly defines `PresenceInteraction`/
//! `PresenceDomain`, the broadcast payload shape that does not otherwise have a home yet
//! (wave 2a wires `PresencePeer.interaction: Option<PresenceInteraction>` in
//! `📡️spr/🧾️wire/🦀️component.rs`, importing this type rather than redefining it).

use serde::{Deserialize, Serialize};

pub use super::{
    DomainHover, DomainSelection, DomainTopology, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef,
    InteractionState, InteractionTarget, InteractionTopology, MergeMode, SelectionMethod, SelectionMode, SelectionSpec, TopologyNode,
};

//#region 🔖️PresenceInteraction
/// 📡️ One peer's interaction roster for one app instance, mirrored onto `PresencePeer.interaction`
/// (bit 7) on the heartbeat — typed (not app-opaque `presence_pack`) so the Shell renders every peer's
/// selection/hover generically. Only explicit ids broadcast; receivers expand transitive closures via
/// their own topology.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct PresenceInteraction {
    pub app_id: String,
    pub domains: Vec<PresenceDomain>,
}

/// 📡️ One domain's broadcast slice of `PresenceInteraction` — the peer-facing mirror of a domain's
/// `DomainSelection`/`DomainHover`, flattened to raw explicit ids (no transitive expansion on the wire).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct PresenceDomain {
    pub domain: String,
    pub granularity: String,
    pub selected: Vec<String>,
    pub hovered: Vec<String>,
}
//#endregion 🔖️PresenceInteraction
