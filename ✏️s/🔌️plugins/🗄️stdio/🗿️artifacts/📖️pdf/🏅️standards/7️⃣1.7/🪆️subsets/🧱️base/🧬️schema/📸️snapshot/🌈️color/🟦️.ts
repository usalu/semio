/** 🌈️ PDF 1.7 snapshot types of color spaces and functions. */
import type { ArtifactRef } from "@semio-tech/framework/schema/artifact-reference";
import type { Binary64 } from "@semio-tech/framework/value/ieee754";
import type { PdfDictEntry } from "../🧩️cos/🟦️.ts";

export type PdfColorSpace =
  | { kind: "deviceGray" }
  | { kind: "deviceRgb" }
  | { kind: "deviceCmyk" }
  | { kind: "calGray"; whitePoint: [Binary64, Binary64, Binary64]; blackPoint?: [Binary64, Binary64, Binary64] | null; gamma?: Binary64 | null }
  | { kind: "calRgb"; whitePoint: [Binary64, Binary64, Binary64]; blackPoint?: [Binary64, Binary64, Binary64] | null; gamma?: [Binary64, Binary64, Binary64] | null; matrix?: [Binary64, Binary64, Binary64, Binary64, Binary64, Binary64, Binary64, Binary64, Binary64] | null }
  | { kind: "lab"; whitePoint: [Binary64, Binary64, Binary64]; blackPoint?: [Binary64, Binary64, Binary64] | null; range?: [Binary64, Binary64, Binary64, Binary64] | null }
  | { kind: "iccBased"; components: number; profile: ArtifactRef; alternate?: PdfColorSpace | null; range?: Binary64[] | null }
  | { kind: "indexed"; base: PdfColorSpace; hival: number; palette: number[] }
  | { kind: "separation"; name: string; alternate: PdfColorSpace; tintTransform: PdfFunction }
  | { kind: "deviceN"; names: string[]; alternate: PdfColorSpace; tintTransform: PdfFunction; attributes?: PdfDictEntry[] | null }
  | { kind: "pattern"; base?: PdfColorSpace | null }
  | { kind: "named"; name: string };

export type PdfFunction =
  | { kind: "sampled"; domain: Binary64[]; range: Binary64[]; size: number[]; bitsPerSample: number; order?: number | null; encode?: Binary64[] | null; decode?: Binary64[] | null; samples: number[] }
  | { kind: "exponential"; domain: Binary64[]; range?: Binary64[] | null; c0: Binary64[]; c1: Binary64[]; n: Binary64 }
  | { kind: "stitching"; domain: Binary64[]; range?: Binary64[] | null; functions: PdfFunction[]; bounds: Binary64[]; encode: Binary64[] }
  | { kind: "postScript"; domain: Binary64[]; range: Binary64[]; code: string }
  | { kind: "array"; functions: PdfFunction[] };
