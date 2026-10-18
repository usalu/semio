/** 📄️ PDF 1.7 snapshot types of pages and indirect objects. */
import type { Binary64 } from "@semio-tech/framework/value/ieee754";
import type { ObjRef, PdfDictEntry, PdfObject } from "../🧩️cos/🟦️.ts";
import type { PdfAnnotation } from "../📌️annotation/🟦️.ts";
import type { PdfColorSpace } from "../🌈️color/🟦️.ts";
import type { PdfOp } from "../🖋️content/🟦️.ts";
import type { PdfTransparencyGroup } from "../🖼️resource/🎚️state/🟦️.ts";

export interface PdfIndirectObject {
  id: ObjRef;
  value: PdfObject;
}

export interface PdfNamedProperties {
  name: string;
  entries: PdfDictEntry[];
}

export interface PdfNamedColorSpace {
  name: string;
  colorSpace: PdfColorSpace;
}

export interface PdfPage {
  mediaBox: [Binary64, Binary64, Binary64, Binary64];
  cropBox?: [Binary64, Binary64, Binary64, Binary64] | null;
  bleedBox?: [Binary64, Binary64, Binary64, Binary64] | null;
  trimBox?: [Binary64, Binary64, Binary64, Binary64] | null;
  artBox?: [Binary64, Binary64, Binary64, Binary64] | null;
  rotate?: number;
  userUnit?: Binary64 | null;
  content?: PdfOp[];
  annotations?: PdfAnnotation[];
  group?: PdfTransparencyGroup | null;
  thumbnail?: string | null;
  structParents?: number | null;
  transition?: PdfDictEntry[] | null;
  duration?: Binary64 | null;
  metadata?: string | null;
  additionalActions?: PdfDictEntry[];
  extra?: PdfDictEntry[];
}
