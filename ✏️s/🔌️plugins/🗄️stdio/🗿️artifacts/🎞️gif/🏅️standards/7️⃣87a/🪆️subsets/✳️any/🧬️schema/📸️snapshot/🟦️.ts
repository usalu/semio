/** 🎨️ Exact GIF87a RGB palette entry. */
export interface GifRgb { r: number; g: number; b: number }
/** 🎨️ Optional palette presence is independent of its ordered entry count. */
export interface GifColorTable { sorted: boolean; colors: GifRgb[] }
/** 🖼️ One natural-order indexed image with its own placement and optional palette. */
export interface GifImage { left: number; top: number; width: number; height: number; interlace: boolean; lct: GifColorTable | null; indices: number[] }
/** 📸️ Complete GIF87a owned semantic snapshot. */
export interface GifSnapshot { schema: string; width: number; height: number; gct: GifColorTable | null; backgroundColorIndex: number; pixelAspectRatio: number; images: GifImage[] }

export * from "./🪶️sqlite/🟦️.ts";
