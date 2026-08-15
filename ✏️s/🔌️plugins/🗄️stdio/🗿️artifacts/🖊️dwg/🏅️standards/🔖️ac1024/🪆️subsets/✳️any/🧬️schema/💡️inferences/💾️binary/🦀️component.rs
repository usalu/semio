//! 📡️ s.stdio.dwg.inference (ac1024) — the normative handcrafted binary protocol for this facet.
//! Same declaration-only shape as the sibling `📝️text` leaf: inference values are only ever
//! computed and optionally cached, never decoded from an authored binary document. The protocol
//! nevertheless declares the structured record and all seven logical metrics rather than an
//! opaque terminal byte sequence.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol
