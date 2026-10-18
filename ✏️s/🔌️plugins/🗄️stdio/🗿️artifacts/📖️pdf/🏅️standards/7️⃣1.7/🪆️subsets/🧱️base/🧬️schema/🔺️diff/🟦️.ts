import type { PdfAdmittedStreamRole } from "../🪪️stream-roles/🟦️.ts";
/** 🧬️ PdfDiff — TypeScript facet of `s.stdio.pdf.1.7` (diff), generated from the Rust model
 *  by 🐍️generate-schema-facets.py (ticket 26/09/18/PDF-ARTIFACT-SPEC-COMPLETE). */
import type { ObjRef, PdfCcittParameters, PdfDecimal, PdfDictEntry, PdfObject, PdfPredictor, PdfStreamFilter } from '../📸️snapshot/🧩️cos/🟦️.ts';
import type { PdfAcroForm, PdfFormField, PdfFormFieldKind, PdfOptionalContent, PdfOptionalContentGroup } from '../📸️snapshot/📝️form/🟦️.ts';
import type { PdfAction, PdfActionKind, PdfDestination, PdfDestinationFit, PdfFileSpecification, PdfNamedDestination, PdfOpenAction, PdfOutlineItem, PdfPageLabelRange } from '../📸️snapshot/🎯️navigation/🟦️.ts';
import type { PdfAnnotation, PdfAppearance, PdfAppearanceEntry, PdfAppearanceState, PdfBorderStyle, PdfMarkupAnnotation } from '../📸️snapshot/📌️annotation/🟦️.ts';
import type { PdfAnnotationKind } from '../📸️snapshot/📌️annotation/🏷️kind/🟦️.ts';
import type { PdfColorSpace, PdfFunction } from '../📸️snapshot/🌈️color/🟦️.ts';
import type { PdfDate, PdfEmbeddedFile, PdfEncryption, PdfEncryptionAlgorithm, PdfMarkInfo, PdfOutputIntent, PdfPageLayout, PdfPageMode, PdfViewerPreferences } from '../📸️snapshot/📇️metadata/🟦️.ts';
import type { PdfExtGState, PdfFormXObject, PdfLineCap, PdfLineJoin, PdfTransparencyGroup } from '../📸️snapshot/🖼️resource/🎚️state/🟦️.ts';
import type { PdfFont } from '../📸️snapshot/🔤️font/🟦️.ts';
import type { PdfImage } from '../📸️snapshot/🖼️resource/🟦️.ts';
import type { PdfInlineImage, PdfOp, PdfPropertyList, PdfTextArrayItem, PdfTextString } from '../📸️snapshot/🖋️content/🟦️.ts';
import type { PdfNamedColorSpace, PdfNamedProperties, PdfPage } from '../📸️snapshot/📄️document/🟦️.ts';
import type { PdfPattern, PdfShading } from '../📸️snapshot/🖼️resource/🌈️shading/🟦️.ts';
import { registerSchemaDocument, validateAgainst, SchemaRefusal } from '../📸️snapshot/🟦️.ts';
import schemaDocument from "./🔣️.json" with { type: "json" };

export interface PdfDiff {
  declaredVersion?: string | null;
  pages?: PdfPagesDiff | null;
  fonts?: PdfKeyedDiffPdfFont | null;
  images?: PdfKeyedDiffPdfImage | null;
  forms?: PdfKeyedDiffPdfFormXObject | null;
  extGStates?: PdfKeyedDiffPdfExtGState | null;
  shadings?: PdfKeyedDiffPdfShading | null;
  patterns?: PdfKeyedDiffPdfPattern | null;
  colorSpaces?: PdfKeyedDiffPdfNamedColorSpace | null;
  properties?: PdfKeyedDiffPdfNamedProperties | null;
  outlines?: PdfIndexedDiffPdfOutlineItem | null;
  namedDestinations?: PdfIndexedDiffPdfNamedDestination | null;
  pageLabels?: PdfIndexedDiffPdfPageLabelRange | null;
  embeddedFiles?: PdfKeyedDiffPdfEmbeddedFile | null;
  outputIntents?: PdfIndexedDiffPdfOutputIntent | null;
  acroForm?: PdfSetPdfAcroForm | null;
  optionalContent?: PdfSetPdfOptionalContent | null;
  pageLayout?: PdfSetPdfPageLayout | null;
  pageMode?: PdfSetPdfPageMode | null;
  viewerPreferences?: PdfSetPdfViewerPreferences | null;
  openAction?: PdfSetPdfOpenAction | null;
  language?: PdfSetString | null;
  markInfo?: PdfSetPdfMarkInfo | null;
  metadata?: PdfSetString | null;
  documentId?: PdfSetArrVec_u8x2 | null;
  encryption?: PdfSetPdfEncryption | null;
  info?: PdfInfoDiff | null;
  catalogExtra?: PdfDictDiff | null;
  objects?: PdfObjectsDiff | null;
  trailer?: PdfDictDiff | null;
  graphEdit?: boolean;
  admittedStreamRoles?: { removed?: number[]; modified?: { index: number; value: PdfAdmittedStreamRole }[]; added?: { index: number; value: PdfAdmittedStreamRole }[] } | null;
}

export interface PdfInfoDiff {
  title?: PdfSetString | null;
  author?: PdfSetString | null;
  subject?: PdfSetString | null;
  keywords?: PdfSetString | null;
  creator?: PdfSetString | null;
  producer?: PdfSetString | null;
  creationDate?: PdfSetPdfDate | null;
  modificationDate?: PdfSetPdfDate | null;
  trapped?: PdfSetString | null;
  extra?: PdfDictEntry[] | null;
}

export type PdfSetPdfDate =
  | { kind: "clear" }
  | { kind: "set"; value: PdfDate };

export interface PdfDictDiff {
  removed?: string[];
  modified?: PdfDictModified[];
  added?: PdfDictAdded[];
}

export interface PdfDictAdded {
  index: number;
  key: string;
  item: PdfObject;
}

export interface PdfDictModified {
  key: string;
  diff: PdfValueDiff;
}

export type PdfValueDiff =
  | { kind: "replace"; value: PdfObject }
  | { kind: "bool"; value: boolean }
  | { kind: "int"; value: number }
  | { kind: "real"; value: PdfDecimal }
  | { kind: "str"; value: number[] }
  | { kind: "name"; value: string }
  | { kind: "ref"; value: ObjRef }
  | { kind: "array"; diff: PdfArrayDiff }
  | { kind: "dict"; diff: PdfDictDiff }
  | { kind: "stream"; dict?: PdfDictDiff | null; data?: number[] | null; filters?: PdfStreamFilter[] | null };

export interface PdfArrayDiff {
  removed?: number[];
  modified?: PdfArrayModified[];
  added?: PdfArrayAdded[];
}

export interface PdfArrayAdded {
  index: number;
  item: PdfObject;
}

export interface PdfArrayModified {
  index: number;
  diff: PdfValueDiff;
}

export interface PdfObjectsDiff {
  removed?: ObjRef[];
  modified?: PdfObjectModified[];
  added?: PdfObjectAdded[];
}

export interface PdfObjectAdded {
  index: number;
  id: ObjRef;
  value: PdfObject;
}

export interface PdfObjectModified {
  id: ObjRef;
  diff: PdfValueDiff;
}

export type PdfSetPdfEncryption =
  | { kind: "clear" }
  | { kind: "set"; value: PdfEncryption };

export type PdfSetArrVec_u8x2 =
  | { kind: "clear" }
  | { kind: "set"; value: [number[], number[]] };

export type PdfSetString =
  | { kind: "clear" }
  | { kind: "set"; value: string };

export type PdfSetPdfMarkInfo =
  | { kind: "clear" }
  | { kind: "set"; value: PdfMarkInfo };

export type PdfSetPdfOpenAction =
  | { kind: "clear" }
  | { kind: "set"; value: PdfOpenAction };

export type PdfSetPdfViewerPreferences =
  | { kind: "clear" }
  | { kind: "set"; value: PdfViewerPreferences };

export type PdfSetPdfPageMode =
  | { kind: "clear" }
  | { kind: "set"; value: PdfPageMode };

export type PdfSetPdfPageLayout =
  | { kind: "clear" }
  | { kind: "set"; value: PdfPageLayout };

export type PdfSetPdfOptionalContent =
  | { kind: "clear" }
  | { kind: "set"; value: PdfOptionalContent };

export type PdfSetPdfAcroForm =
  | { kind: "clear" }
  | { kind: "set"; value: PdfAcroForm };

export interface PdfIndexedDiffPdfOutputIntent {
  removed?: number[];
  modified?: PdfIndexedItemPdfOutputIntent[];
  added?: PdfIndexedItemPdfOutputIntent[];
}

export interface PdfIndexedItemPdfAnnotation {
  index: number;
  value: PdfAnnotation;
}

export interface PdfIndexedItemPdfEmbeddedFile {
  index: number;
  value: PdfEmbeddedFile;
}

export interface PdfIndexedItemPdfExtGState {
  index: number;
  value: PdfExtGState;
}

export interface PdfIndexedItemPdfFont {
  index: number;
  value: PdfFont;
}

export interface PdfIndexedItemPdfFormXObject {
  index: number;
  value: PdfFormXObject;
}

export interface PdfIndexedItemPdfImage {
  index: number;
  value: PdfImage;
}

export interface PdfIndexedItemPdfNamedColorSpace {
  index: number;
  value: PdfNamedColorSpace;
}

export interface PdfIndexedItemPdfNamedDestination {
  index: number;
  value: PdfNamedDestination;
}

export interface PdfIndexedItemPdfNamedProperties {
  index: number;
  value: PdfNamedProperties;
}

export interface PdfIndexedItemPdfOp {
  index: number;
  value: PdfOp;
}

export interface PdfIndexedItemPdfOutlineItem {
  index: number;
  value: PdfOutlineItem;
}

export interface PdfIndexedItemPdfOutputIntent {
  index: number;
  value: PdfOutputIntent;
}

export interface PdfIndexedItemPdfPageLabelRange {
  index: number;
  value: PdfPageLabelRange;
}

export interface PdfIndexedItemPdfPattern {
  index: number;
  value: PdfPattern;
}

export interface PdfIndexedItemPdfShading {
  index: number;
  value: PdfShading;
}

export interface PdfKeyedDiffPdfEmbeddedFile {
  removed?: string[];
  modified?: PdfKeyedItemPdfEmbeddedFile[];
  added?: PdfIndexedItemPdfEmbeddedFile[];
}

export interface PdfKeyedItemPdfEmbeddedFile {
  key: string;
  value: PdfEmbeddedFile;
}

export interface PdfKeyedItemPdfExtGState {
  key: string;
  value: PdfExtGState;
}

export interface PdfKeyedItemPdfFont {
  key: string;
  value: PdfFont;
}

export interface PdfKeyedItemPdfFormXObject {
  key: string;
  value: PdfFormXObject;
}

export interface PdfKeyedItemPdfImage {
  key: string;
  value: PdfImage;
}

export interface PdfKeyedItemPdfNamedColorSpace {
  key: string;
  value: PdfNamedColorSpace;
}

export interface PdfKeyedItemPdfNamedProperties {
  key: string;
  value: PdfNamedProperties;
}

export interface PdfKeyedItemPdfPattern {
  key: string;
  value: PdfPattern;
}

export interface PdfKeyedItemPdfShading {
  key: string;
  value: PdfShading;
}

export interface PdfIndexedDiffPdfPageLabelRange {
  removed?: number[];
  modified?: PdfIndexedItemPdfPageLabelRange[];
  added?: PdfIndexedItemPdfPageLabelRange[];
}

export interface PdfIndexedDiffPdfNamedDestination {
  removed?: number[];
  modified?: PdfIndexedItemPdfNamedDestination[];
  added?: PdfIndexedItemPdfNamedDestination[];
}

export interface PdfIndexedDiffPdfOutlineItem {
  removed?: number[];
  modified?: PdfIndexedItemPdfOutlineItem[];
  added?: PdfIndexedItemPdfOutlineItem[];
}

export interface PdfKeyedDiffPdfNamedProperties {
  removed?: string[];
  modified?: PdfKeyedItemPdfNamedProperties[];
  added?: PdfIndexedItemPdfNamedProperties[];
}

export interface PdfKeyedDiffPdfNamedColorSpace {
  removed?: string[];
  modified?: PdfKeyedItemPdfNamedColorSpace[];
  added?: PdfIndexedItemPdfNamedColorSpace[];
}

export interface PdfKeyedDiffPdfPattern {
  removed?: string[];
  modified?: PdfKeyedItemPdfPattern[];
  added?: PdfIndexedItemPdfPattern[];
}

export interface PdfKeyedDiffPdfShading {
  removed?: string[];
  modified?: PdfKeyedItemPdfShading[];
  added?: PdfIndexedItemPdfShading[];
}

export interface PdfKeyedDiffPdfExtGState {
  removed?: string[];
  modified?: PdfKeyedItemPdfExtGState[];
  added?: PdfIndexedItemPdfExtGState[];
}

export interface PdfKeyedDiffPdfFormXObject {
  removed?: string[];
  modified?: PdfKeyedItemPdfFormXObject[];
  added?: PdfIndexedItemPdfFormXObject[];
}

export interface PdfKeyedDiffPdfImage {
  removed?: string[];
  modified?: PdfKeyedItemPdfImage[];
  added?: PdfIndexedItemPdfImage[];
}

export interface PdfKeyedDiffPdfFont {
  removed?: string[];
  modified?: PdfKeyedItemPdfFont[];
  added?: PdfIndexedItemPdfFont[];
}

export interface PdfPagesDiff {
  removed?: number[];
  modified?: PdfPageModified[];
  added?: PdfPageAdded[];
}

export interface PdfPageAdded {
  index: number;
  page: PdfPage;
}

export interface PdfPageModified {
  index: number;
  diff: PdfPageDiff;
}

export interface PdfPageDiff {
  mediaBox?: [number, number, number, number] | null;
  cropBox?: PdfSetPdfRect | null;
  bleedBox?: PdfSetPdfRect | null;
  trimBox?: PdfSetPdfRect | null;
  artBox?: PdfSetPdfRect | null;
  rotate?: number | null;
  userUnit?: PdfSetf64 | null;
  content?: PdfIndexedDiffPdfOp | null;
  annotations?: PdfIndexedDiffPdfAnnotation | null;
  group?: PdfSetPdfTransparencyGroup | null;
  thumbnail?: PdfSetString | null;
  structParents?: PdfSetu32 | null;
  transition?: PdfSetVec_PdfDictEntry | null;
  duration?: PdfSetf64 | null;
  metadata?: PdfSetString | null;
  additionalActions?: PdfDictEntry[] | null;
  extra?: PdfDictDiff | null;
}

export type PdfSetf64 =
  | { kind: "clear" }
  | { kind: "set"; value: number };

export type PdfSetVec_PdfDictEntry =
  | { kind: "clear" }
  | { kind: "set"; value: PdfDictEntry[] };

export type PdfSetu32 =
  | { kind: "clear" }
  | { kind: "set"; value: number };

export type PdfSetPdfTransparencyGroup =
  | { kind: "clear" }
  | { kind: "set"; value: PdfTransparencyGroup };

export interface PdfIndexedDiffPdfAnnotation {
  removed?: number[];
  modified?: PdfIndexedItemPdfAnnotation[];
  added?: PdfIndexedItemPdfAnnotation[];
}

export interface PdfIndexedDiffPdfOp {
  removed?: number[];
  modified?: PdfIndexedItemPdfOp[];
  added?: PdfIndexedItemPdfOp[];
}

export type PdfSetPdfRect =
  | { kind: "clear" }
  | { kind: "set"; value: [number, number, number, number] };

/** 🧭️ One step into one object's `PdfObject` tree — the `path` of `SetDictEntry`/`RemoveDictEntry`. */
export type PdfPathSegment =
  | { kind: "arrayIndex"; index: number }
  | { kind: "dictKey"; key: string };

/** 📐️ Which optional page box `SetPageBox` addresses. */
export type PdfPageBox = "crop" | "bleed" | "trim" | "art";

/** 🔣️ The JSON Schema document this facet is validated against. */
export const schema = schemaDocument;

export { registerSchemaDocument, validateAgainst, SchemaRefusal };
registerSchemaDocument(schema);
export const parsePdfDiff = (value: unknown): PdfDiff => validateAgainst<PdfDiff>(schema, "", value);
export const parsePdfDictDiff = (value: unknown): PdfDictDiff => validateAgainst<PdfDictDiff>(schema, "/$defs/PdfDictDiff", value);
export const parsePdfDictAdded = (value: unknown): PdfDictAdded => validateAgainst<PdfDictAdded>(schema, "/$defs/PdfDictAdded", value);
export const parsePdfDictModified = (value: unknown): PdfDictModified => validateAgainst<PdfDictModified>(schema, "/$defs/PdfDictModified", value);
export const parsePdfValueDiff = (value: unknown): PdfValueDiff => validateAgainst<PdfValueDiff>(schema, "/$defs/PdfValueDiff", value);
export const parsePdfArrayDiff = (value: unknown): PdfArrayDiff => validateAgainst<PdfArrayDiff>(schema, "/$defs/PdfArrayDiff", value);
export const parsePdfArrayAdded = (value: unknown): PdfArrayAdded => validateAgainst<PdfArrayAdded>(schema, "/$defs/PdfArrayAdded", value);
export const parsePdfArrayModified = (value: unknown): PdfArrayModified => validateAgainst<PdfArrayModified>(schema, "/$defs/PdfArrayModified", value);
export const parsePdfObjectsDiff = (value: unknown): PdfObjectsDiff => validateAgainst<PdfObjectsDiff>(schema, "/$defs/PdfObjectsDiff", value);
export const parsePdfObjectAdded = (value: unknown): PdfObjectAdded => validateAgainst<PdfObjectAdded>(schema, "/$defs/PdfObjectAdded", value);
export const parsePdfObjectModified = (value: unknown): PdfObjectModified => validateAgainst<PdfObjectModified>(schema, "/$defs/PdfObjectModified", value);
export const parsePdfSetPdfEncryption = (value: unknown): PdfSetPdfEncryption => validateAgainst<PdfSetPdfEncryption>(schema, "/$defs/PdfSetPdfEncryption", value);
export const parsePdfSetArrVec_u8x2 = (value: unknown): PdfSetArrVec_u8x2 => validateAgainst<PdfSetArrVec_u8x2>(schema, "/$defs/PdfSetArrVec_u8x2", value);
export const parsePdfSetString = (value: unknown): PdfSetString => validateAgainst<PdfSetString>(schema, "/$defs/PdfSetString", value);
export const parsePdfSetPdfMarkInfo = (value: unknown): PdfSetPdfMarkInfo => validateAgainst<PdfSetPdfMarkInfo>(schema, "/$defs/PdfSetPdfMarkInfo", value);
export const parsePdfSetPdfOpenAction = (value: unknown): PdfSetPdfOpenAction => validateAgainst<PdfSetPdfOpenAction>(schema, "/$defs/PdfSetPdfOpenAction", value);
export const parsePdfSetPdfViewerPreferences = (value: unknown): PdfSetPdfViewerPreferences => validateAgainst<PdfSetPdfViewerPreferences>(schema, "/$defs/PdfSetPdfViewerPreferences", value);
export const parsePdfSetPdfPageMode = (value: unknown): PdfSetPdfPageMode => validateAgainst<PdfSetPdfPageMode>(schema, "/$defs/PdfSetPdfPageMode", value);
export const parsePdfSetPdfPageLayout = (value: unknown): PdfSetPdfPageLayout => validateAgainst<PdfSetPdfPageLayout>(schema, "/$defs/PdfSetPdfPageLayout", value);
export const parsePdfSetPdfOptionalContent = (value: unknown): PdfSetPdfOptionalContent => validateAgainst<PdfSetPdfOptionalContent>(schema, "/$defs/PdfSetPdfOptionalContent", value);
export const parsePdfSetPdfAcroForm = (value: unknown): PdfSetPdfAcroForm => validateAgainst<PdfSetPdfAcroForm>(schema, "/$defs/PdfSetPdfAcroForm", value);
export const parsePdfIndexedDiffPdfOutputIntent = (value: unknown): PdfIndexedDiffPdfOutputIntent => validateAgainst<PdfIndexedDiffPdfOutputIntent>(schema, "/$defs/PdfIndexedDiffPdfOutputIntent", value);
export const parsePdfKeyedDiffPdfEmbeddedFile = (value: unknown): PdfKeyedDiffPdfEmbeddedFile => validateAgainst<PdfKeyedDiffPdfEmbeddedFile>(schema, "/$defs/PdfKeyedDiffPdfEmbeddedFile", value);
export const parsePdfIndexedDiffPdfPageLabelRange = (value: unknown): PdfIndexedDiffPdfPageLabelRange => validateAgainst<PdfIndexedDiffPdfPageLabelRange>(schema, "/$defs/PdfIndexedDiffPdfPageLabelRange", value);
export const parsePdfIndexedDiffPdfNamedDestination = (value: unknown): PdfIndexedDiffPdfNamedDestination => validateAgainst<PdfIndexedDiffPdfNamedDestination>(schema, "/$defs/PdfIndexedDiffPdfNamedDestination", value);
export const parsePdfIndexedDiffPdfOutlineItem = (value: unknown): PdfIndexedDiffPdfOutlineItem => validateAgainst<PdfIndexedDiffPdfOutlineItem>(schema, "/$defs/PdfIndexedDiffPdfOutlineItem", value);
export const parsePdfKeyedDiffPdfNamedProperties = (value: unknown): PdfKeyedDiffPdfNamedProperties => validateAgainst<PdfKeyedDiffPdfNamedProperties>(schema, "/$defs/PdfKeyedDiffPdfNamedProperties", value);
export const parsePdfKeyedDiffPdfNamedColorSpace = (value: unknown): PdfKeyedDiffPdfNamedColorSpace => validateAgainst<PdfKeyedDiffPdfNamedColorSpace>(schema, "/$defs/PdfKeyedDiffPdfNamedColorSpace", value);
export const parsePdfKeyedDiffPdfPattern = (value: unknown): PdfKeyedDiffPdfPattern => validateAgainst<PdfKeyedDiffPdfPattern>(schema, "/$defs/PdfKeyedDiffPdfPattern", value);
export const parsePdfKeyedDiffPdfShading = (value: unknown): PdfKeyedDiffPdfShading => validateAgainst<PdfKeyedDiffPdfShading>(schema, "/$defs/PdfKeyedDiffPdfShading", value);
export const parsePdfKeyedDiffPdfExtGState = (value: unknown): PdfKeyedDiffPdfExtGState => validateAgainst<PdfKeyedDiffPdfExtGState>(schema, "/$defs/PdfKeyedDiffPdfExtGState", value);
export const parsePdfKeyedDiffPdfFormXObject = (value: unknown): PdfKeyedDiffPdfFormXObject => validateAgainst<PdfKeyedDiffPdfFormXObject>(schema, "/$defs/PdfKeyedDiffPdfFormXObject", value);
export const parsePdfKeyedDiffPdfImage = (value: unknown): PdfKeyedDiffPdfImage => validateAgainst<PdfKeyedDiffPdfImage>(schema, "/$defs/PdfKeyedDiffPdfImage", value);
export const parsePdfKeyedDiffPdfFont = (value: unknown): PdfKeyedDiffPdfFont => validateAgainst<PdfKeyedDiffPdfFont>(schema, "/$defs/PdfKeyedDiffPdfFont", value);
export const parsePdfPagesDiff = (value: unknown): PdfPagesDiff => validateAgainst<PdfPagesDiff>(schema, "/$defs/PdfPagesDiff", value);
export const parsePdfPageAdded = (value: unknown): PdfPageAdded => validateAgainst<PdfPageAdded>(schema, "/$defs/PdfPageAdded", value);
export const parsePdfPageModified = (value: unknown): PdfPageModified => validateAgainst<PdfPageModified>(schema, "/$defs/PdfPageModified", value);
export const parsePdfPageDiff = (value: unknown): PdfPageDiff => validateAgainst<PdfPageDiff>(schema, "/$defs/PdfPageDiff", value);
export const parsePdfSetf64 = (value: unknown): PdfSetf64 => validateAgainst<PdfSetf64>(schema, "/$defs/PdfSetf64", value);
export const parsePdfSetVec_PdfDictEntry = (value: unknown): PdfSetVec_PdfDictEntry => validateAgainst<PdfSetVec_PdfDictEntry>(schema, "/$defs/PdfSetVec_PdfDictEntry", value);
export const parsePdfSetu32 = (value: unknown): PdfSetu32 => validateAgainst<PdfSetu32>(schema, "/$defs/PdfSetu32", value);
export const parsePdfSetPdfTransparencyGroup = (value: unknown): PdfSetPdfTransparencyGroup => validateAgainst<PdfSetPdfTransparencyGroup>(schema, "/$defs/PdfSetPdfTransparencyGroup", value);
export const parsePdfIndexedDiffPdfAnnotation = (value: unknown): PdfIndexedDiffPdfAnnotation => validateAgainst<PdfIndexedDiffPdfAnnotation>(schema, "/$defs/PdfIndexedDiffPdfAnnotation", value);
export const parsePdfIndexedDiffPdfOp = (value: unknown): PdfIndexedDiffPdfOp => validateAgainst<PdfIndexedDiffPdfOp>(schema, "/$defs/PdfIndexedDiffPdfOp", value);
export const parsePdfSetPdfRect = (value: unknown): PdfSetPdfRect => validateAgainst<PdfSetPdfRect>(schema, "/$defs/PdfSetPdfRect", value);
export const parsePdfPathSegment = (value: unknown): PdfPathSegment => validateAgainst<PdfPathSegment>(schema, "/$defs/PdfPathSegment", value);
export const parsePdfPageBox = (value: unknown): PdfPageBox => validateAgainst<PdfPageBox>(schema, "/$defs/PdfPageBox", value);
export const parsePdfInfoDiff = (value: unknown): PdfInfoDiff => validateAgainst<PdfInfoDiff>(schema, "/$defs/PdfInfoDiff", value);
export const parsePdfSetPdfDate = (value: unknown): PdfSetPdfDate => validateAgainst<PdfSetPdfDate>(schema, "/$defs/PdfSetPdfDate", value);
export const parsePdfIndexedItemPdfAnnotation = (value: unknown): PdfIndexedItemPdfAnnotation => validateAgainst<PdfIndexedItemPdfAnnotation>(schema, "/$defs/PdfIndexedItemPdfAnnotation", value);
export const parsePdfIndexedItemPdfEmbeddedFile = (value: unknown): PdfIndexedItemPdfEmbeddedFile => validateAgainst<PdfIndexedItemPdfEmbeddedFile>(schema, "/$defs/PdfIndexedItemPdfEmbeddedFile", value);
export const parsePdfIndexedItemPdfExtGState = (value: unknown): PdfIndexedItemPdfExtGState => validateAgainst<PdfIndexedItemPdfExtGState>(schema, "/$defs/PdfIndexedItemPdfExtGState", value);
export const parsePdfIndexedItemPdfFont = (value: unknown): PdfIndexedItemPdfFont => validateAgainst<PdfIndexedItemPdfFont>(schema, "/$defs/PdfIndexedItemPdfFont", value);
export const parsePdfIndexedItemPdfFormXObject = (value: unknown): PdfIndexedItemPdfFormXObject => validateAgainst<PdfIndexedItemPdfFormXObject>(schema, "/$defs/PdfIndexedItemPdfFormXObject", value);
export const parsePdfIndexedItemPdfImage = (value: unknown): PdfIndexedItemPdfImage => validateAgainst<PdfIndexedItemPdfImage>(schema, "/$defs/PdfIndexedItemPdfImage", value);
export const parsePdfIndexedItemPdfNamedColorSpace = (value: unknown): PdfIndexedItemPdfNamedColorSpace => validateAgainst<PdfIndexedItemPdfNamedColorSpace>(schema, "/$defs/PdfIndexedItemPdfNamedColorSpace", value);
export const parsePdfIndexedItemPdfNamedDestination = (value: unknown): PdfIndexedItemPdfNamedDestination => validateAgainst<PdfIndexedItemPdfNamedDestination>(schema, "/$defs/PdfIndexedItemPdfNamedDestination", value);
export const parsePdfIndexedItemPdfNamedProperties = (value: unknown): PdfIndexedItemPdfNamedProperties => validateAgainst<PdfIndexedItemPdfNamedProperties>(schema, "/$defs/PdfIndexedItemPdfNamedProperties", value);
export const parsePdfIndexedItemPdfOp = (value: unknown): PdfIndexedItemPdfOp => validateAgainst<PdfIndexedItemPdfOp>(schema, "/$defs/PdfIndexedItemPdfOp", value);
export const parsePdfIndexedItemPdfOutlineItem = (value: unknown): PdfIndexedItemPdfOutlineItem => validateAgainst<PdfIndexedItemPdfOutlineItem>(schema, "/$defs/PdfIndexedItemPdfOutlineItem", value);
export const parsePdfIndexedItemPdfOutputIntent = (value: unknown): PdfIndexedItemPdfOutputIntent => validateAgainst<PdfIndexedItemPdfOutputIntent>(schema, "/$defs/PdfIndexedItemPdfOutputIntent", value);
export const parsePdfIndexedItemPdfPageLabelRange = (value: unknown): PdfIndexedItemPdfPageLabelRange => validateAgainst<PdfIndexedItemPdfPageLabelRange>(schema, "/$defs/PdfIndexedItemPdfPageLabelRange", value);
export const parsePdfIndexedItemPdfPattern = (value: unknown): PdfIndexedItemPdfPattern => validateAgainst<PdfIndexedItemPdfPattern>(schema, "/$defs/PdfIndexedItemPdfPattern", value);
export const parsePdfIndexedItemPdfShading = (value: unknown): PdfIndexedItemPdfShading => validateAgainst<PdfIndexedItemPdfShading>(schema, "/$defs/PdfIndexedItemPdfShading", value);
export const parsePdfKeyedItemPdfEmbeddedFile = (value: unknown): PdfKeyedItemPdfEmbeddedFile => validateAgainst<PdfKeyedItemPdfEmbeddedFile>(schema, "/$defs/PdfKeyedItemPdfEmbeddedFile", value);
export const parsePdfKeyedItemPdfExtGState = (value: unknown): PdfKeyedItemPdfExtGState => validateAgainst<PdfKeyedItemPdfExtGState>(schema, "/$defs/PdfKeyedItemPdfExtGState", value);
export const parsePdfKeyedItemPdfFont = (value: unknown): PdfKeyedItemPdfFont => validateAgainst<PdfKeyedItemPdfFont>(schema, "/$defs/PdfKeyedItemPdfFont", value);
export const parsePdfKeyedItemPdfFormXObject = (value: unknown): PdfKeyedItemPdfFormXObject => validateAgainst<PdfKeyedItemPdfFormXObject>(schema, "/$defs/PdfKeyedItemPdfFormXObject", value);
export const parsePdfKeyedItemPdfImage = (value: unknown): PdfKeyedItemPdfImage => validateAgainst<PdfKeyedItemPdfImage>(schema, "/$defs/PdfKeyedItemPdfImage", value);
export const parsePdfKeyedItemPdfNamedColorSpace = (value: unknown): PdfKeyedItemPdfNamedColorSpace => validateAgainst<PdfKeyedItemPdfNamedColorSpace>(schema, "/$defs/PdfKeyedItemPdfNamedColorSpace", value);
export const parsePdfKeyedItemPdfNamedProperties = (value: unknown): PdfKeyedItemPdfNamedProperties => validateAgainst<PdfKeyedItemPdfNamedProperties>(schema, "/$defs/PdfKeyedItemPdfNamedProperties", value);
export const parsePdfKeyedItemPdfPattern = (value: unknown): PdfKeyedItemPdfPattern => validateAgainst<PdfKeyedItemPdfPattern>(schema, "/$defs/PdfKeyedItemPdfPattern", value);
export const parsePdfKeyedItemPdfShading = (value: unknown): PdfKeyedItemPdfShading => validateAgainst<PdfKeyedItemPdfShading>(schema, "/$defs/PdfKeyedItemPdfShading", value);
