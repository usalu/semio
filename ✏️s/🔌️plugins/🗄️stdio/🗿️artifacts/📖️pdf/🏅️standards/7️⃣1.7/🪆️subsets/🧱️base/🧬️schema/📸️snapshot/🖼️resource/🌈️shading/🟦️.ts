/** 🌅️ PDF 1.7 snapshot types of shadings and patterns. */
import type { ArtifactRef } from "@semio-tech/framework/schema/artifact-reference";
import type { Binary64 } from "@semio-tech/framework/value/ieee754";
import type { PdfColorSpace, PdfFunction } from "../../🌈️color/🟦️.ts";
import type { PdfDictEntry } from "../../🧩️cos/🟦️.ts";
import type { PdfOp } from "../../🖋️content/🟦️.ts";

export interface PdfPattern {
  id: string;
  matrix?: [Binary64, Binary64, Binary64, Binary64, Binary64, Binary64];
  kind: PdfPatternKind;
  extra?: PdfDictEntry[];
}

export type PdfPatternKind =
  | { kind: "tiling"; paintType: number; tilingType: number; bbox: [Binary64, Binary64, Binary64, Binary64]; xStep: Binary64; yStep: Binary64; content: PdfOp[] }
  | { kind: "shading"; shading: string; extGState?: string | null };

export interface PdfShading {
  id: string;
  colorSpace: PdfColorSpace;
  kind: PdfShadingKind;
  background?: Binary64[] | null;
  bbox?: [Binary64, Binary64, Binary64, Binary64] | null;
  antiAlias?: boolean;
  extra?: PdfDictEntry[];
}

export type PdfShadingKind =
  | { kind: "functionBased"; domain?: [Binary64, Binary64, Binary64, Binary64] | null; matrix?: [Binary64, Binary64, Binary64, Binary64, Binary64, Binary64] | null; function: PdfFunction }
  | { kind: "axial"; coords: [Binary64, Binary64, Binary64, Binary64]; domain?: [Binary64, Binary64] | null; function: PdfFunction; extend: [boolean, boolean] }
  | { kind: "radial"; coords: [Binary64, Binary64, Binary64, Binary64, Binary64, Binary64]; domain?: [Binary64, Binary64] | null; function: PdfFunction; extend: [boolean, boolean] }
  | { kind: "mesh"; shadingType: number; bitsPerCoordinate: number; bitsPerComponent: number; bitsPerFlag?: number | null; verticesPerRow?: number | null; decode: Binary64[]; function?: PdfFunction | null; reference: ArtifactRef };
