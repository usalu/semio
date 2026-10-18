/** 📝️ PDF 1.7 snapshot types of interactive forms and optional content. */
import type { PdfDictEntry, PdfObject } from "../🧩️cos/🟦️.ts";

export interface PdfOptionalContent {
  groups?: PdfOptionalContentGroup[];
  name?: string | null;
  baseStateOff?: boolean;
  on?: string[];
  off?: string[];
  order?: PdfObject[];
  extra?: PdfDictEntry[];
}

export interface PdfOptionalContentGroup {
  id: string;
  name: string;
  intent?: string[];
  usage?: PdfDictEntry[];
}

export interface PdfAcroForm {
  fields?: PdfFormField[];
  needAppearances?: boolean;
  signatureFlags?: number;
  defaultAppearance?: string | null;
  quadding?: number | null;
  defaultFonts?: string[];
  extra?: PdfDictEntry[];
}

export interface PdfFormField {
  name: string;
  kind: PdfFormFieldKind;
  flags?: number;
  alternateName?: string | null;
  mappingName?: string | null;
  defaultAppearance?: string | null;
  quadding?: number | null;
  widgets?: [number, number][];
  children?: PdfFormField[];
  additionalActions?: PdfDictEntry[];
  extra?: PdfDictEntry[];
}

export type PdfFormFieldKind =
  | { kind: "button"; value?: string | null; defaultValue?: string | null; options: string[] }
  | { kind: "text"; value?: string | null; defaultValue?: string | null; maxLength?: number | null; richValue?: string | null }
  | { kind: "choice"; values: string[]; defaultValues: string[]; options: [string, string][]; topIndex?: number | null }
  | { kind: "signature"; value?: PdfDictEntry[] | null }
  | { kind: "container" };
