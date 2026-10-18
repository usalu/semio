/** 🎚️ PDF 1.7 snapshot types of graphics state, soft masks and form XObjects. */
import type { Binary64 } from "@semio-tech/framework/value/ieee754";
import type { PdfColorSpace, PdfFunction } from "../../🌈️color/🟦️.ts";
import type { PdfDictEntry } from "../../🧩️cos/🟦️.ts";
import type { PdfOp } from "../../🖋️content/🟦️.ts";

export type PdfLineJoin =
  | "miter"
  | "round"
  | "bevel";

export type PdfLineCap =
  | "butt"
  | "round"
  | "square";

export interface PdfExtGState {
  id: string;
  lineWidth?: Binary64 | null;
  lineCap?: PdfLineCap | null;
  lineJoin?: PdfLineJoin | null;
  miterLimit?: Binary64 | null;
  dash?: [Binary64[], Binary64] | null;
  renderingIntent?: string | null;
  overprintStroke?: boolean | null;
  overprintFill?: boolean | null;
  overprintMode?: number | null;
  font?: [string, Binary64] | null;
  blendMode?: string[] | null;
  softMask?: PdfSoftMask | null;
  strokeAlpha?: Binary64 | null;
  fillAlpha?: Binary64 | null;
  alphaIsShape?: boolean | null;
  strokeAdjust?: boolean | null;
  flatness?: Binary64 | null;
  smoothness?: Binary64 | null;
  textKnockout?: boolean | null;
  extra?: PdfDictEntry[];
}

export type PdfSoftMask =
  | { kind: "none" }
  | { kind: "alpha"; group: string; transfer?: PdfFunction | null }
  | { kind: "luminosity"; group: string; backdrop?: Binary64[] | null; transfer?: PdfFunction | null };

export interface PdfFormXObject {
  id: string;
  bbox: [Binary64, Binary64, Binary64, Binary64];
  matrix?: [Binary64, Binary64, Binary64, Binary64, Binary64, Binary64];
  content?: PdfOp[];
  group?: PdfTransparencyGroup | null;
  optionalContent?: string | null;
  structParent?: number | null;
  extra?: PdfDictEntry[];
}

export interface PdfTransparencyGroup {
  colorSpace?: PdfColorSpace | null;
  isolated?: boolean;
  knockout?: boolean;
}
