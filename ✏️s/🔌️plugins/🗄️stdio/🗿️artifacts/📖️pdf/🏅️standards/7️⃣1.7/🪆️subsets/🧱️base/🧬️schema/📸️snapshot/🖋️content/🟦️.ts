/** 🖋️ PDF 1.7 snapshot types of content stream operands and text. */
import type { Binary64 } from "@semio-tech/framework/value/ieee754";
import type { PdfColorSpace } from "../🌈️color/🟦️.ts";
import type { PdfDictEntry, PdfObject } from "../🧩️cos/🟦️.ts";
import type { PdfImageBody } from "../🖼️resource/🟦️.ts";
import type { PdfLineCap, PdfLineJoin } from "../🖼️resource/🎚️state/🟦️.ts";

export type PdfOp =
  | { op: "setLineWidth"; width: Binary64 }
  | { op: "setLineCap"; cap: PdfLineCap }
  | { op: "setLineJoin"; join: PdfLineJoin }
  | { op: "setMiterLimit"; limit: Binary64 }
  | { op: "setDash"; array: Binary64[]; phase: Binary64 }
  | { op: "setRenderingIntent"; intent: string }
  | { op: "setFlatness"; flatness: Binary64 }
  | { op: "setExtGState"; name: string }
  | { op: "save" }
  | { op: "restore" }
  | { op: "transform"; matrix: [Binary64, Binary64, Binary64, Binary64, Binary64, Binary64] }
  | { op: "moveTo"; x: Binary64; y: Binary64 }
  | { op: "lineTo"; x: Binary64; y: Binary64 }
  | { op: "curveTo"; x1: Binary64; y1: Binary64; x2: Binary64; y2: Binary64; x3: Binary64; y3: Binary64 }
  | { op: "curveToInitial"; x2: Binary64; y2: Binary64; x3: Binary64; y3: Binary64 }
  | { op: "curveToFinal"; x1: Binary64; y1: Binary64; x3: Binary64; y3: Binary64 }
  | { op: "closePath" }
  | { op: "rectangle"; x: Binary64; y: Binary64; width: Binary64; height: Binary64 }
  | { op: "stroke" }
  | { op: "closeStroke" }
  | { op: "fill" }
  | { op: "fillEvenOdd" }
  | { op: "fillStroke" }
  | { op: "fillStrokeEvenOdd" }
  | { op: "closeFillStroke" }
  | { op: "closeFillStrokeEvenOdd" }
  | { op: "endPath" }
  | { op: "clip" }
  | { op: "clipEvenOdd" }
  | { op: "beginText" }
  | { op: "endText" }
  | { op: "setCharSpacing"; spacing: Binary64 }
  | { op: "setWordSpacing"; spacing: Binary64 }
  | { op: "setHorizontalScale"; scale: Binary64 }
  | { op: "setLeading"; leading: Binary64 }
  | { op: "setFont"; name: string; size: Binary64 }
  | { op: "setTextRenderingMode"; mode: number }
  | { op: "setTextRise"; rise: Binary64 }
  | { op: "moveText"; tx: Binary64; ty: Binary64 }
  | { op: "moveTextSetLeading"; tx: Binary64; ty: Binary64 }
  | { op: "setTextMatrix"; matrix: [Binary64, Binary64, Binary64, Binary64, Binary64, Binary64] }
  | { op: "nextLine" }
  | { op: "showText"; text: PdfTextString }
  | { op: "showTextArray"; items: PdfTextArrayItem[] }
  | { op: "nextLineShowText"; text: PdfTextString }
  | { op: "nextLineShowTextSpaced"; wordSpacing: Binary64; charSpacing: Binary64; text: PdfTextString }
  | { op: "setGlyphWidth"; wx: Binary64; wy: Binary64 }
  | { op: "setGlyphWidthAndBox"; wx: Binary64; wy: Binary64; llx: Binary64; lly: Binary64; urx: Binary64; ury: Binary64 }
  | { op: "setStrokeColorSpace"; name: string }
  | { op: "setFillColorSpace"; name: string }
  | { op: "setStrokeColor"; components: Binary64[] }
  | { op: "setStrokeColorN"; components: Binary64[]; pattern?: string | null }
  | { op: "setFillColor"; components: Binary64[] }
  | { op: "setFillColorN"; components: Binary64[]; pattern?: string | null }
  | { op: "setStrokeGray"; gray: Binary64 }
  | { op: "setFillGray"; gray: Binary64 }
  | { op: "setStrokeRgb"; r: Binary64; g: Binary64; b: Binary64 }
  | { op: "setFillRgb"; r: Binary64; g: Binary64; b: Binary64 }
  | { op: "setStrokeCmyk"; c: Binary64; m: Binary64; y: Binary64; k: Binary64 }
  | { op: "setFillCmyk"; c: Binary64; m: Binary64; y: Binary64; k: Binary64 }
  | { op: "paintShading"; name: string }
  | { op: "paintXObject"; name: string }
  | { op: "inlineImage"; image: PdfInlineImage }
  | { op: "markedContentPoint"; tag: string }
  | { op: "markedContentPointWithProperties"; tag: string; properties: PdfPropertyList }
  | { op: "beginMarkedContent"; tag: string }
  | { op: "beginMarkedContentWithProperties"; tag: string; properties: PdfPropertyList }
  | { op: "endMarkedContent" }
  | { op: "beginCompatibility" }
  | { op: "endCompatibility" }
  | { op: "unknown"; operator: string; operands: PdfObject[] };

export type PdfPropertyList =
  | { kind: "named"; name: string }
  | { kind: "inline"; entries: PdfDictEntry[] };

export interface PdfInlineImage {
  width: number;
  height: number;
  bitsPerComponent?: number;
  colorSpace?: PdfColorSpace | null;
  imageMask?: boolean;
  decode?: Binary64[];
  interpolate?: boolean;
  body: PdfImageBody;
  extra?: PdfDictEntry[];
}

export type PdfTextString =
  | { kind: "text"; text: string }
  | { kind: "codes"; codes: number[] };

export type PdfTextArrayItem =
  | { kind: "text"; text: string }
  | { kind: "codes"; codes: number[] }
  | { kind: "adjust"; amount: Binary64 };
