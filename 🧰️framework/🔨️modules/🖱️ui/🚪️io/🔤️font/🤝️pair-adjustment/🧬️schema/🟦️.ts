/** 🔎️ Queries two original glyph identities without retaining a font or shaping cache. */
export interface FontPairQuery { readonly leftGlyph: number; readonly rightGlyph: number }
/** 📏️ Returns the accumulated horizontal advance in the original font design units. */
export interface FontPairAdjustment { readonly advanceUnits: number }
