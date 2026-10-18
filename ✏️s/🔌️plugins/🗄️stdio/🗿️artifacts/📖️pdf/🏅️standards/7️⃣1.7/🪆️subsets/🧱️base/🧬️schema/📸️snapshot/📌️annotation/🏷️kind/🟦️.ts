/** 🏷️ PDF 1.7 snapshot types of annotation kinds. */
import type { Binary64 } from "@semio-tech/framework/value/ieee754";
import type { PdfAction, PdfDestination, PdfFileSpecification } from "../../🎯️navigation/🟦️.ts";
import type { PdfDictEntry } from "../../🧩️cos/🟦️.ts";

export type PdfAnnotationKind =
  | { kind: "text"; open: boolean; icon?: string | null; state?: string | null; stateModel?: string | null }
  | { kind: "link"; action?: PdfAction | null; destination?: PdfDestination | null; highlight?: string | null; quadPoints: Binary64[] }
  | { kind: "freeText"; defaultAppearance: string; quadding: number; callout?: Binary64[] | null; lineEnding?: string | null; richText?: string | null }
  | { kind: "line"; points: [Binary64, Binary64, Binary64, Binary64]; lineEndings?: [string, string] | null; interiorColor?: Binary64[] | null; leaderLength?: Binary64 | null; caption: boolean }
  | { kind: "square"; interiorColor?: Binary64[] | null; rectDifferences?: [Binary64, Binary64, Binary64, Binary64] | null }
  | { kind: "circle"; interiorColor?: Binary64[] | null; rectDifferences?: [Binary64, Binary64, Binary64, Binary64] | null }
  | { kind: "polygon"; vertices: Binary64[]; interiorColor?: Binary64[] | null }
  | { kind: "polyLine"; vertices: Binary64[]; lineEndings?: [string, string] | null; interiorColor?: Binary64[] | null }
  | { kind: "highlight"; quadPoints: Binary64[] }
  | { kind: "underline"; quadPoints: Binary64[] }
  | { kind: "squiggly"; quadPoints: Binary64[] }
  | { kind: "strikeOut"; quadPoints: Binary64[] }
  | { kind: "stamp"; icon?: string | null }
  | { kind: "caret"; rectDifferences?: [Binary64, Binary64, Binary64, Binary64] | null; symbol?: string | null }
  | { kind: "ink"; paths: Binary64[][] }
  | { kind: "popup"; parent?: bigint | null; open: boolean }
  | { kind: "fileAttachment"; file: PdfFileSpecification; icon?: string | null }
  | { kind: "sound"; sound: PdfDictEntry[]; icon?: string | null }
  | { kind: "movie"; title?: string | null; movie: PdfDictEntry[]; activation?: PdfDictEntry[] | null }
  | { kind: "widget"; field?: string | null; highlight?: string | null; characteristics: PdfDictEntry[]; action?: PdfAction | null; additionalActions: PdfDictEntry[] }
  | { kind: "screen"; title?: string | null; characteristics: PdfDictEntry[]; action?: PdfAction | null; additionalActions: PdfDictEntry[] }
  | { kind: "printerMark"; markStyle?: string | null; colorants: PdfDictEntry[] }
  | { kind: "trapNet"; entries: PdfDictEntry[] }
  | { kind: "watermark"; fixedPrint?: PdfDictEntry[] | null }
  | { kind: "threeD"; entries: PdfDictEntry[] }
  | { kind: "redact"; quadPoints: Binary64[]; interiorColor?: Binary64[] | null; overlayText?: string | null; repeat: boolean; defaultAppearance?: string | null; quadding: number }
  | { kind: "unknown"; subtype: string; entries: PdfDictEntry[] };
