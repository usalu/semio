//! 🤝️ Borrowed font IO exchanges only glyph identities and design-unit adjustments.
/// 🔎️ Queries two original glyph identities without retaining a font or shaping cache.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
pub struct FontPairQuery {pub left_glyph:u16,pub right_glyph:u16}
/// 📏️ Returns the accumulated horizontal advance in the original font design units.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
pub struct FontPairAdjustment {pub advance_units:i32}
