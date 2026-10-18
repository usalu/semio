/** 🖼️ PDF 1.7 snapshot types of image XObjects. */
import type { ArtifactRef } from "@semio-tech/framework/schema/artifact-reference";
import type { Binary64 } from "@semio-tech/framework/value/ieee754";
import type { PdfColorSpace } from "../🌈️color/🟦️.ts";
import type { PdfDictEntry } from "../🧩️cos/🟦️.ts";

export interface PdfImage {
  id: string;
  width: number;
  height: number;
  colorSpace?: PdfColorSpace | null;
  bitsPerComponent?: number;
  imageMask?: boolean;
  decode?: Binary64[];
  interpolate?: boolean;
  body: PdfImageBody;
  softMask?: string | null;
  softMaskInData?: number | null;
  mask?: PdfImageMask | null;
  matte?: Binary64[] | null;
  intent?: string | null;
  optionalContent?: string | null;
  structParent?: number | null;
  extra?: PdfDictEntry[];
}

export type PdfImageMask =
  | { kind: "stencil"; image: string }
  | { kind: "colorKey"; ranges: number[] };

export type PdfImageBody = { kind: "samples"; values: number[] } | { kind: "artifact"; reference: ArtifactRef };
