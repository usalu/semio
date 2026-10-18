/** 📌️ PDF 1.7 snapshot types of annotations and appearances. */
import type { Binary64 } from "@semio-tech/framework/value/ieee754";
import type { PdfAnnotationKind } from "./🏷️kind/🟦️.ts";
import type { PdfDate } from "../📇️metadata/🟦️.ts";
import type { PdfDictEntry } from "../🧩️cos/🟦️.ts";

export interface PdfAnnotation {
  rect: [Binary64, Binary64, Binary64, Binary64];
  kind: PdfAnnotationKind;
  contents?: string | null;
  name?: string | null;
  modified?: string | null;
  flags?: number;
  border?: PdfBorderStyle | null;
  color?: Binary64[];
  appearance?: PdfAppearance | null;
  appearanceState?: string | null;
  markup?: PdfMarkupAnnotation | null;
  optionalContent?: string | null;
  structParent?: number | null;
  extra?: PdfDictEntry[];
}

export interface PdfMarkupAnnotation {
  title?: string | null;
  popup?: bigint | null;
  opacity?: Binary64 | null;
  richContents?: string | null;
  creationDate?: PdfDate | null;
  inReplyTo?: bigint | null;
  subject?: string | null;
  replyType?: string | null;
  intent?: string | null;
}

export interface PdfAppearance {
  normal: PdfAppearanceEntry;
  rollover?: PdfAppearanceEntry | null;
  down?: PdfAppearanceEntry | null;
}

export type PdfAppearanceEntry =
  | { kind: "single"; form: string }
  | { kind: "states"; states: PdfAppearanceState[] };

export interface PdfAppearanceState {
  state: string;
  form: string;
}

export interface PdfBorderStyle {
  width: Binary64;
  style?: string | null;
  dash?: Binary64[] | null;
  radii?: [Binary64, Binary64] | null;
}
