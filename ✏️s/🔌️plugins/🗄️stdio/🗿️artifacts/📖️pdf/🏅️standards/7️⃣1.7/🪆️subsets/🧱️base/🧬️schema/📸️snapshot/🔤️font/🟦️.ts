/** 🔤️ PDF 1.7 snapshot types of fonts, encodings and character maps. */
import type { ArtifactRef } from "@semio-tech/framework/schema/artifact-reference";
import type { Binary64 } from "@semio-tech/framework/value/ieee754";
import type { PdfDictEntry } from "../🧩️cos/🟦️.ts";
import type { PdfOp } from "../🖋️content/🟦️.ts";

export interface PdfFont {
  id: string;
  kind: PdfFontKind;
  toUnicode?: PdfToUnicode | null;
  extra?: PdfDictEntry[];
}

export interface PdfToUnicode {
  byteWidth: number;
  mappings?: PdfToUnicodeMapping[];
}

export type PdfToUnicodeMapping =
  | { kind: "char"; code: number; text: string }
  | { kind: "range"; low: number; high: number; text: string };

export type PdfFontKind =
  | { kind: "type1"; baseFont: string; encoding: PdfSimpleEncoding; firstChar: number; widths: Binary64[]; descriptor?: PdfFontDescriptor | null; program?: PdfFontProgram | null }
  | { kind: "trueType"; baseFont: string; encoding: PdfSimpleEncoding; firstChar: number; widths: Binary64[]; descriptor?: PdfFontDescriptor | null; program?: PdfFontProgram | null }
  | { kind: "type3"; fontMatrix: [Binary64, Binary64, Binary64, Binary64, Binary64, Binary64]; fontBbox: [Binary64, Binary64, Binary64, Binary64]; encoding: PdfSimpleEncoding; firstChar: number; widths: Binary64[]; charProcs: PdfCharProc[]; descriptor?: PdfFontDescriptor | null }
  | { kind: "type0"; baseFont: string; cmap: PdfCMap; descendant: PdfCidFont };

export interface PdfCidFont {
  trueType: boolean;
  baseFont: string;
  systemInfo?: PdfCidSystemInfo;
  descriptor: PdfFontDescriptor;
  defaultWidth?: Binary64;
  widths?: PdfCidWidthRun[];
  defaultVertical?: [Binary64, Binary64] | null;
  verticalMetrics?: PdfCidVerticalRun[];
  cidToGid?: PdfCidToGid | null;
  program?: PdfFontProgram | null;
  extra?: PdfDictEntry[];
}

export type PdfFontProgram =
  { kind: "type1"; reference: ArtifactRef }
  | { kind: "trueType"; reference: ArtifactRef }
  | { kind: "cff"; reference: ArtifactRef }
  | { kind: "cidCff"; reference: ArtifactRef }
  | { kind: "openType"; reference: ArtifactRef };

export type PdfCidToGid =
  | { kind: "identity" }
  | { kind: "map"; glyphs: number[] };

export interface PdfCidVerticalRun {
  startCid: number;
  metrics: [Binary64, Binary64, Binary64][];
}

export interface PdfCidWidthRun {
  startCid: number;
  widths: Binary64[];
}

export interface PdfFontDescriptor {
  fontName: string;
  flags?: number;
  fontBbox?: [Binary64, Binary64, Binary64, Binary64];
  italicAngle?: Binary64;
  ascent?: Binary64;
  descent?: Binary64;
  capHeight?: Binary64;
  stemV?: Binary64;
  stemH?: Binary64 | null;
  xHeight?: Binary64 | null;
  leading?: Binary64 | null;
  avgWidth?: Binary64 | null;
  maxWidth?: Binary64 | null;
  missingWidth?: Binary64 | null;
  fontFamily?: string | null;
  fontStretch?: string | null;
  fontWeight?: Binary64 | null;
  charSet?: string | null;
  extra?: PdfDictEntry[];
}

export interface PdfCidSystemInfo {
  registry: string;
  ordering: string;
  supplement: number;
}

export type PdfCMap =
  | { kind: "predefined"; name: string }
  | { kind: "embedded"; cmap: PdfEmbeddedCMap };

export interface PdfEmbeddedCMap {
  name: string;
  vertical?: boolean;
  codespace?: PdfCodespaceRange[];
  mappings?: PdfCidMapping[];
  useCmap?: string | null;
}

export type PdfCidMapping =
  | { kind: "char"; code: number; cid: number }
  | { kind: "range"; low: number; high: number; cid: number };

export interface PdfCodespaceRange {
  byteWidth: number;
  low: number;
  high: number;
}

export interface PdfCharProc {
  name: string;
  content: PdfOp[];
}

export interface PdfSimpleEncoding {
  base?: PdfBaseEncoding | null;
  differences?: PdfEncodingDifference[];
}

export interface PdfEncodingDifference {
  code: number;
  glyph: string;
}

export type PdfBaseEncoding =
  | "standard"
  | "winAnsi"
  | "macRoman"
  | "macExpert";
