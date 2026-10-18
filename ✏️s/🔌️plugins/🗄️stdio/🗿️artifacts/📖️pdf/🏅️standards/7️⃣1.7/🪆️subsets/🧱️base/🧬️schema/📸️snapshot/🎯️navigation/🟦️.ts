/** 🎯️ PDF 1.7 snapshot types of actions, destinations, outlines and page labels. */
import type { Binary64 } from "@semio-tech/framework/value/ieee754";
import type { PdfDictEntry } from "../🧩️cos/🟦️.ts";

export type PdfOpenAction =
  | { kind: "destination"; destination: PdfDestination }
  | { kind: "action"; action: PdfAction };

export interface PdfAction {
  kind: PdfActionKind;
  next?: PdfAction[];
}

export type PdfActionKind =
  | { kind: "goTo"; destination: PdfDestination }
  | { kind: "goToRemote"; file: PdfFileSpecification; destination: PdfDestination; newWindow?: boolean | null }
  | { kind: "goToEmbedded"; destination: PdfDestination; newWindow?: boolean | null }
  | { kind: "launch"; file: PdfFileSpecification; newWindow?: boolean | null }
  | { kind: "thread"; file?: PdfFileSpecification | null; thread: number }
  | { kind: "uri"; uri: string; isMap: boolean }
  | { kind: "sound"; sound: string; volume?: Binary64 | null; synchronous: boolean; repeat: boolean; mix: boolean }
  | { kind: "movie"; annotation?: string | null; operation?: string | null }
  | { kind: "hide"; annotations: string[]; hide: boolean }
  | { kind: "named"; name: string }
  | { kind: "submitForm"; url: string; fields: string[]; flags: number }
  | { kind: "resetForm"; fields: string[]; flags: number }
  | { kind: "importData"; file: PdfFileSpecification }
  | { kind: "javaScript"; script: string }
  | { kind: "setOptionalContentState"; states: PdfDictEntry[]; preserveRadioButtons: boolean }
  | { kind: "rendition"; entries: PdfDictEntry[] }
  | { kind: "transition"; entries: PdfDictEntry[] }
  | { kind: "goTo3dView"; entries: PdfDictEntry[] }
  | { kind: "unknown"; subtype: string; entries: PdfDictEntry[] };

export type PdfFileSpecification =
  | { kind: "path"; path: string }
  | { kind: "embedded"; file: string };

export type PdfDestination =
  | { kind: "page"; page: number; fit: PdfDestinationFit }
  | { kind: "remotePage"; page: number; fit: PdfDestinationFit }
  | { kind: "named"; name: string };

export type PdfDestinationFit =
  | { kind: "xyz"; left?: Binary64 | null; top?: Binary64 | null; zoom?: Binary64 | null }
  | { kind: "fit" }
  | { kind: "fitHorizontal"; top?: Binary64 | null }
  | { kind: "fitVertical"; left?: Binary64 | null }
  | { kind: "fitRectangle"; rect: [Binary64, Binary64, Binary64, Binary64] }
  | { kind: "fitBoundingBox" }
  | { kind: "fitBoundingBoxHorizontal"; top?: Binary64 | null }
  | { kind: "fitBoundingBoxVertical"; left?: Binary64 | null };

export interface PdfPageLabelRange {
  startIndex: number;
  style?: PdfPageLabelStyle | null;
  prefix?: string | null;
  start?: number;
}

export type PdfPageLabelStyle =
  | "decimal"
  | "romanUpper"
  | "romanLower"
  | "lettersUpper"
  | "lettersLower";

export interface PdfNamedDestination {
  name: string;
  destination: PdfDestination;
}

export interface PdfOutlineItem {
  title: string;
  destination?: PdfDestination | null;
  action?: PdfAction | null;
  color?: [Binary64, Binary64, Binary64] | null;
  italic?: boolean;
  bold?: boolean;
  open?: boolean;
  children?: PdfOutlineItem[];
  extra?: PdfDictEntry[];
}
