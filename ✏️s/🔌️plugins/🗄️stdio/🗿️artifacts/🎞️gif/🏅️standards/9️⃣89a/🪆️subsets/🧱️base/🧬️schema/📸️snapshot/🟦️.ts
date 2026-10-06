/** 🎨️ Exact GIF89a RGB palette entry. */
export interface GifRgb { r: number; g: number; b: number }
/** 🎨️ Optional palette presence is independent of its ordered entry count. */
export interface GifColorTable { sorted: boolean; colors: GifRgb[] }
/** 🗑️ GIF89a's four owned disposal policies. */
export type GifDisposal = "unspecified" | "doNotDispose" | "restoreToBackground" | "restoreToPrevious";
/** 📝️ Optional plain-text rendering block. */
export interface GifPlainText { left: number; top: number; width: number; height: number; cellWidth: number; cellHeight: number; fgColorIndex: number; bgColorIndex: number; text: string }
/** 🧩️ Typed application identifier/authentication octets and intrinsic payload. */
export interface GifAppExtension { identifier: [number,number,number,number,number,number,number,number]; authCode: [number,number,number]; data: number[] }
/** 🎞️ One indexed frame and its complete rendering-control metadata. */
export interface GifFrame { left: number; top: number; width: number; height: number; interlace: boolean; lct: GifColorTable | null; indices: number[]; delayCs: number; disposal: GifDisposal; transparentIndex: number | null; userInput: boolean; plainText: GifPlainText | null }
/** 📸️ Complete GIF89a owned semantic snapshot. */
export interface GifSnapshot { schema: string; width: number; height: number; gct: GifColorTable | null; backgroundColorIndex: number; pixelAspectRatio: number; loopCount: number | null; frames: GifFrame[]; comments: string[]; appExtensions: GifAppExtension[] }
