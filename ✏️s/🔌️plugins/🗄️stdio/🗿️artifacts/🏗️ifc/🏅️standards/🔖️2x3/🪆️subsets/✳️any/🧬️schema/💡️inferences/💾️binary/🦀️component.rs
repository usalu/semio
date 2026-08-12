//! 📡️ s.stdio.ifc.2x3.inference — the normative handcrafted binary protocol for this facet. Same
//! declaration-only shape as the sibling `📝️text` leaf: inference values are only ever computed
//! and (optionally) cached, never decoded from an authored binary document, so there is no
//! `encode`/`decode` pair here — just the protocol spec text every other representation leaf
//! declares for its own facet.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol
