/** 🔤️ Authored text selects one exact shipped family without inferred substitution. */
export const DRAWING_FONT_FAMILIES=["anta","kellySlab","shareTechMono","notoEmoji"] as const;
export type DrawingFontFamily=typeof DRAWING_FONT_FAMILIES[number];
export function parseDrawingFontFamily(value:unknown):DrawingFontFamily{if(typeof value!=="string"||!DRAWING_FONT_FAMILIES.includes(value as DrawingFontFamily))throw Error("Unsupported authored font family");return value as DrawingFontFamily;}
export function drawingFontCatalogFamily(family:DrawingFontFamily):string{switch(family){case "anta":return "Anta";case "kellySlab":return "Kelly Slab";case "shareTechMono":return "Share Tech Mono";case "notoEmoji":return "Noto Emoji";}}

export function drawingFontFamilyFromCatalog(value:string):DrawingFontFamily{for(const family of DRAWING_FONT_FAMILIES)if(drawingFontCatalogFamily(family)===value)return family;throw Error("Unsupported authored font family");}
