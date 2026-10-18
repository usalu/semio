/** 🧬️ Canonical owned PDF1.7 domain with explicit admission from its native JSON schema. */
import { validateRoleInputs, type PdfAdmittedStreamRole } from "../🪪️stream-roles/🟦️.ts";
import schema from "./🔣️.json" with { type: "json" };
import { parseBinary64 } from "@semio-tech/framework/value/ieee754";
import type { ObjRef, PdfCcittParameters, PdfDecimal, PdfDictEntry, PdfObject, PdfPredictor, PdfStreamFilter } from "./🧩️cos/🟦️.ts";
import type { PdfIndirectObject, PdfNamedColorSpace, PdfNamedProperties, PdfPage } from "./📄️document/🟦️.ts";
import type { PdfDate, PdfEmbeddedFile, PdfEncryption, PdfEncryptionAlgorithm, PdfInfo, PdfMarkInfo, PdfOutputIntent, PdfPageLayout, PdfPageMode, PdfViewerPreferences } from "./📇️metadata/🟦️.ts";
import type { PdfAction, PdfActionKind, PdfDestination, PdfDestinationFit, PdfFileSpecification, PdfNamedDestination, PdfOpenAction, PdfOutlineItem, PdfPageLabelRange, PdfPageLabelStyle } from "./🎯️navigation/🟦️.ts";
import type { PdfAcroForm, PdfFormField, PdfFormFieldKind, PdfOptionalContent, PdfOptionalContentGroup } from "./📝️form/🟦️.ts";
import type { PdfColorSpace, PdfFunction } from "./🌈️color/🟦️.ts";
import type { PdfInlineImage, PdfOp, PdfPropertyList, PdfTextArrayItem, PdfTextString } from "./🖋️content/🟦️.ts";
import type { PdfImage, PdfImageBody, PdfImageMask } from "./🖼️resource/🟦️.ts";
import type { PdfPattern, PdfPatternKind, PdfShading, PdfShadingKind } from "./🖼️resource/🌈️shading/🟦️.ts";
import type { PdfExtGState, PdfFormXObject, PdfLineCap, PdfLineJoin, PdfSoftMask, PdfTransparencyGroup } from "./🖼️resource/🎚️state/🟦️.ts";
import type { PdfBaseEncoding, PdfCMap, PdfCharProc, PdfCidFont, PdfCidMapping, PdfCidSystemInfo, PdfCidToGid, PdfCidVerticalRun, PdfCidWidthRun, PdfCodespaceRange, PdfEmbeddedCMap, PdfEncodingDifference, PdfFont, PdfFontDescriptor, PdfFontKind, PdfFontProgram, PdfSimpleEncoding, PdfToUnicode, PdfToUnicodeMapping } from "./🔤️font/🟦️.ts";
import type { PdfAnnotation, PdfAppearance, PdfAppearanceEntry, PdfAppearanceState, PdfBorderStyle, PdfMarkupAnnotation } from "./📌️annotation/🟦️.ts";
import type { PdfAnnotationKind } from "./📌️annotation/🏷️kind/🟦️.ts";

export interface PdfSnapshot {
  schema: string;
  declaredVersion: string;
  pages: PdfPage[];
  fonts: PdfFont[];
  images: PdfImage[];
  forms: PdfFormXObject[];
  extGStates: PdfExtGState[];
  shadings: PdfShading[];
  patterns: PdfPattern[];
  colorSpaces: PdfNamedColorSpace[];
  properties: PdfNamedProperties[];
  outlines: PdfOutlineItem[];
  namedDestinations: PdfNamedDestination[];
  pageLabels: PdfPageLabelRange[];
  embeddedFiles: PdfEmbeddedFile[];
  outputIntents: PdfOutputIntent[];
  acroForm?: PdfAcroForm | null;
  optionalContent?: PdfOptionalContent | null;
  pageLayout?: PdfPageLayout | null;
  pageMode?: PdfPageMode | null;
  viewerPreferences?: PdfViewerPreferences | null;
  openAction?: PdfOpenAction | null;
  language?: string | null;
  markInfo?: PdfMarkInfo | null;
  metadata?: string | null;
  documentId?: [number[], number[]] | null;
  encryption?: PdfEncryption | null;
  info: PdfInfo;
  catalogExtra: PdfDictEntry[];
  objects: PdfIndirectObject[];
  trailer: PdfDictEntry[];
  admittedStreamRoles: PdfAdmittedStreamRole[];
}

//#region 🚪️Validation
type Schema = Record<string, unknown>;
export class SchemaRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}
const documents = new Map<string, Schema>();
export const registerSchemaDocument = (schema: Schema): void => {
  documents.set(String(schema["$id"]), schema);
};
const resolveRef = (ref: string, own: Schema): { node: Schema; document: Schema } => {
  const [documentId, pointer] = ref.split("#");
  const document = documentId === "" ? own : documents.get(documentId);
  if (!document) throw new SchemaRefusal("$ref", `unknown schema document ${documentId}`);
  let node: unknown = document;
  for (const step of (pointer ?? "").split("/").filter((s) => s.length > 0)) node = (node as Record<string, unknown>)[step];
  if (!node) throw new SchemaRefusal("$ref", `unresolved pointer ${ref}`);
  return { node: node as Schema, document };
};
const matches = (schema: Schema, value: unknown, own: Schema, at: string, errors: string[]): boolean => {
  if (schema["x-semio-primitive"] === "binary64" || schema["$ref"] === "https://json.schemas.assets.semio-tech.com/framework/value/schema.json#/$defs/Binary64Transport" || schema["$ref"] === "https://json.schemas.assets.semio-tech.com/framework/value/schema.json#/$defs/Binary64") { try { parseBinary64(value); return true; } catch { return (errors.push(`${at}: expected owned binary64`), false); } }
  if (schema["x-semio-primitive"] === "i64" || schema["x-semio-primitive"] === "u64") return typeof value === "bigint" && value >= (schema["x-semio-primitive"] === "i64" ? -9223372036854775808n : 0n) && value <= (schema["x-semio-primitive"] === "i64" ? 9223372036854775807n : 18446744073709551615n) || (errors.push(`${at}: expected owned integer word`), false);
  if (typeof schema["$ref"] === "string") { const resolved = resolveRef(schema["$ref"] as string, own); return matches(resolved.node, value, resolved.document, at, errors); }
  if (schema["const"] !== undefined) return value === schema["const"] || (errors.push(`${at}: expected ${JSON.stringify(schema["const"])}`), false);
  if (Array.isArray(schema["enum"])) return (schema["enum"] as unknown[]).includes(value) || (errors.push(`${at}: not one of ${(schema["enum"] as unknown[]).join(", ")}`), false);
  if (Array.isArray(schema["anyOf"])) return (schema["anyOf"] as Schema[]).some((s) => matches(s, value, own, at, [])) || (errors.push(`${at}: matches no alternative`), false);
  if (Array.isArray(schema["oneOf"])) return (schema["oneOf"] as Schema[]).filter((s) => matches(s, value, own, at, [])).length === 1 || (errors.push(`${at}: matches no single alternative`), false);
  if (Array.isArray(schema["allOf"])) return (schema["allOf"] as Schema[]).every((s) => matches(s, value, own, at, errors));
  const types = Array.isArray(schema["type"]) ? (schema["type"] as string[]) : typeof schema["type"] === "string" ? [schema["type"] as string] : [];
  const kind = value === null ? "null" : Array.isArray(value) ? "array" : typeof value === "number" ? (Number.isInteger(value) ? "integer" : "number") : typeof value;
  if (types.length > 0 && !types.includes(kind) && !(kind === "integer" && types.includes("number"))) return (errors.push(`${at}: expected ${types.join("|")}, found ${kind}`), false);
  if (kind === "integer" || kind === "number") {
    if (typeof schema["minimum"] === "number" && (value as number) < (schema["minimum"] as number)) return (errors.push(`${at}: below ${schema["minimum"]}`), false);
    if (typeof schema["maximum"] === "number" && (value as number) > (schema["maximum"] as number)) return (errors.push(`${at}: above ${schema["maximum"]}`), false);
  }
  if (kind === "array") {
    const items = value as unknown[];
    if (typeof schema["minItems"] === "number" && items.length < (schema["minItems"] as number)) return (errors.push(`${at}: fewer than ${schema["minItems"]} items`), false);
    if (typeof schema["maxItems"] === "number" && items.length > (schema["maxItems"] as number)) return (errors.push(`${at}: more than ${schema["maxItems"]} items`), false);
    if (Array.isArray(schema["items"])) return items.every((item, index) => matches((schema["items"] as Schema[])[index] ?? {}, item, own, `${at}[${index}]`, errors));
    if (schema["items"]) return items.every((item, index) => matches(schema["items"] as Schema, item, own, `${at}[${index}]`, errors));
  }
  if (kind === "object") {
    const row = value as Record<string, unknown>;
    for (const key of (schema["required"] as string[] | undefined) ?? []) if (row[key] === undefined) return (errors.push(`${at}.${key}: missing`), false);
    const properties = (schema["properties"] as Record<string, Schema> | undefined) ?? {};
    if(schema["additionalProperties"]===false && Object.keys(row).some(key=>!(key in properties)))return (errors.push(`${at}: unexpected field`),false);
    for (const [key, sub] of Object.entries(properties)) if (row[key] !== undefined && !matches(sub, row[key], own, `${at}.${key}`, errors)) return false;
  }
  return true;
};
export const validateAgainst = <T,>(schema: Schema, pointer: string, value: unknown): T => {
  const node = pointer === "" ? schema : resolveRef(`#${pointer}`, schema).node;
  const errors: string[] = [];
  if (!matches(node, value, schema, "$", errors)) throw new SchemaRefusal("$", errors[0] ?? "invalid");
  return value as T;
};
//#endregion 🚪️Validation
registerSchemaDocument(schema);
export const parsePdfSnapshot = (value: unknown): PdfSnapshot => {const snapshot=validateAgainst<PdfSnapshot>(schema,"",value);validateRoleInputs(snapshot.objects,snapshot.objects,[],snapshot.admittedStreamRoles);return snapshot;};
export const parsePdfDictEntry = (value: unknown): PdfDictEntry => validateAgainst<PdfDictEntry>(schema, "/$defs/PdfDictEntry", value);
export const parsePdfObject = (value: unknown): PdfObject => validateAgainst<PdfObject>(schema, "/$defs/PdfObject", value);
export const parsePdfStreamFilter = (value: unknown): PdfStreamFilter => validateAgainst<PdfStreamFilter>(schema, "/$defs/PdfStreamFilter", value);
export const parsePdfCcittParameters = (value: unknown): PdfCcittParameters => validateAgainst<PdfCcittParameters>(schema, "/$defs/PdfCcittParameters", value);
export const parsePdfPredictor = (value: unknown): PdfPredictor => validateAgainst<PdfPredictor>(schema, "/$defs/PdfPredictor", value);
export const parseObjRef = (value: unknown): ObjRef => validateAgainst<ObjRef>(schema, "/$defs/ObjRef", value);
export const parsePdfDecimal = (value: unknown): PdfDecimal => validateAgainst<PdfDecimal>(schema, "/$defs/PdfDecimal", value);
export const parsePdfIndirectObject = (value: unknown): PdfIndirectObject => validateAgainst<PdfIndirectObject>(schema, "/$defs/PdfIndirectObject", value);
export const parsePdfInfo = (value: unknown): PdfInfo => validateAgainst<PdfInfo>(schema, "/$defs/PdfInfo", value);
export const parsePdfDate = (value: unknown): PdfDate => validateAgainst<PdfDate>(schema, "/$defs/PdfDate", value);
export const parsePdfEncryption = (value: unknown): PdfEncryption => validateAgainst<PdfEncryption>(schema, "/$defs/PdfEncryption", value);
export const parsePdfEncryptionAlgorithm = (value: unknown): PdfEncryptionAlgorithm => validateAgainst<PdfEncryptionAlgorithm>(schema, "/$defs/PdfEncryptionAlgorithm", value);
export const parsePdfMarkInfo = (value: unknown): PdfMarkInfo => validateAgainst<PdfMarkInfo>(schema, "/$defs/PdfMarkInfo", value);
export const parsePdfOpenAction = (value: unknown): PdfOpenAction => validateAgainst<PdfOpenAction>(schema, "/$defs/PdfOpenAction", value);
export const parsePdfAction = (value: unknown): PdfAction => validateAgainst<PdfAction>(schema, "/$defs/PdfAction", value);
export const parsePdfActionKind = (value: unknown): PdfActionKind => validateAgainst<PdfActionKind>(schema, "/$defs/PdfActionKind", value);
export const parsePdfFileSpecification = (value: unknown): PdfFileSpecification => validateAgainst<PdfFileSpecification>(schema, "/$defs/PdfFileSpecification", value);
export const parsePdfDestination = (value: unknown): PdfDestination => validateAgainst<PdfDestination>(schema, "/$defs/PdfDestination", value);
export const parsePdfDestinationFit = (value: unknown): PdfDestinationFit => validateAgainst<PdfDestinationFit>(schema, "/$defs/PdfDestinationFit", value);
export const parsePdfViewerPreferences = (value: unknown): PdfViewerPreferences => validateAgainst<PdfViewerPreferences>(schema, "/$defs/PdfViewerPreferences", value);
export const parsePdfPageMode = (value: unknown): PdfPageMode => validateAgainst<PdfPageMode>(schema, "/$defs/PdfPageMode", value);
export const parsePdfPageLayout = (value: unknown): PdfPageLayout => validateAgainst<PdfPageLayout>(schema, "/$defs/PdfPageLayout", value);
export const parsePdfOptionalContent = (value: unknown): PdfOptionalContent => validateAgainst<PdfOptionalContent>(schema, "/$defs/PdfOptionalContent", value);
export const parsePdfOptionalContentGroup = (value: unknown): PdfOptionalContentGroup => validateAgainst<PdfOptionalContentGroup>(schema, "/$defs/PdfOptionalContentGroup", value);
export const parsePdfAcroForm = (value: unknown): PdfAcroForm => validateAgainst<PdfAcroForm>(schema, "/$defs/PdfAcroForm", value);
export const parsePdfFormField = (value: unknown): PdfFormField => validateAgainst<PdfFormField>(schema, "/$defs/PdfFormField", value);
export const parsePdfFormFieldKind = (value: unknown): PdfFormFieldKind => validateAgainst<PdfFormFieldKind>(schema, "/$defs/PdfFormFieldKind", value);
export const parsePdfOutputIntent = (value: unknown): PdfOutputIntent => validateAgainst<PdfOutputIntent>(schema, "/$defs/PdfOutputIntent", value);
export const parsePdfEmbeddedFile = (value: unknown): PdfEmbeddedFile => validateAgainst<PdfEmbeddedFile>(schema, "/$defs/PdfEmbeddedFile", value);
export const parsePdfPageLabelRange = (value: unknown): PdfPageLabelRange => validateAgainst<PdfPageLabelRange>(schema, "/$defs/PdfPageLabelRange", value);
export const parsePdfPageLabelStyle = (value: unknown): PdfPageLabelStyle => validateAgainst<PdfPageLabelStyle>(schema, "/$defs/PdfPageLabelStyle", value);
export const parsePdfNamedDestination = (value: unknown): PdfNamedDestination => validateAgainst<PdfNamedDestination>(schema, "/$defs/PdfNamedDestination", value);
export const parsePdfOutlineItem = (value: unknown): PdfOutlineItem => validateAgainst<PdfOutlineItem>(schema, "/$defs/PdfOutlineItem", value);
export const parsePdfNamedProperties = (value: unknown): PdfNamedProperties => validateAgainst<PdfNamedProperties>(schema, "/$defs/PdfNamedProperties", value);
export const parsePdfNamedColorSpace = (value: unknown): PdfNamedColorSpace => validateAgainst<PdfNamedColorSpace>(schema, "/$defs/PdfNamedColorSpace", value);
export const parsePdfColorSpace = (value: unknown): PdfColorSpace => validateAgainst<PdfColorSpace>(schema, "/$defs/PdfColorSpace", value);
export const parsePdfFunction = (value: unknown): PdfFunction => validateAgainst<PdfFunction>(schema, "/$defs/PdfFunction", value);
export const parsePdfPattern = (value: unknown): PdfPattern => validateAgainst<PdfPattern>(schema, "/$defs/PdfPattern", value);
export const parsePdfPatternKind = (value: unknown): PdfPatternKind => validateAgainst<PdfPatternKind>(schema, "/$defs/PdfPatternKind", value);
export const parsePdfOp = (value: unknown): PdfOp => validateAgainst<PdfOp>(schema, "/$defs/PdfOp", value);
export const parsePdfPropertyList = (value: unknown): PdfPropertyList => validateAgainst<PdfPropertyList>(schema, "/$defs/PdfPropertyList", value);
export const parsePdfInlineImage = (value: unknown): PdfInlineImage => validateAgainst<PdfInlineImage>(schema, "/$defs/PdfInlineImage", value);
export const parsePdfTextString = (value: unknown): PdfTextString => validateAgainst<PdfTextString>(schema, "/$defs/PdfTextString", value);
export const parsePdfTextArrayItem = (value: unknown): PdfTextArrayItem => validateAgainst<PdfTextArrayItem>(schema, "/$defs/PdfTextArrayItem", value);
export const parsePdfLineJoin = (value: unknown): PdfLineJoin => validateAgainst<PdfLineJoin>(schema, "/$defs/PdfLineJoin", value);
export const parsePdfLineCap = (value: unknown): PdfLineCap => validateAgainst<PdfLineCap>(schema, "/$defs/PdfLineCap", value);
export const parsePdfShading = (value: unknown): PdfShading => validateAgainst<PdfShading>(schema, "/$defs/PdfShading", value);
export const parsePdfShadingKind = (value: unknown): PdfShadingKind => validateAgainst<PdfShadingKind>(schema, "/$defs/PdfShadingKind", value);
export const parsePdfExtGState = (value: unknown): PdfExtGState => validateAgainst<PdfExtGState>(schema, "/$defs/PdfExtGState", value);
export const parsePdfSoftMask = (value: unknown): PdfSoftMask => validateAgainst<PdfSoftMask>(schema, "/$defs/PdfSoftMask", value);
export const parsePdfFormXObject = (value: unknown): PdfFormXObject => validateAgainst<PdfFormXObject>(schema, "/$defs/PdfFormXObject", value);
export const parsePdfTransparencyGroup = (value: unknown): PdfTransparencyGroup => validateAgainst<PdfTransparencyGroup>(schema, "/$defs/PdfTransparencyGroup", value);
export const parsePdfImage = (value: unknown): PdfImage => validateAgainst<PdfImage>(schema, "/$defs/PdfImage", value);
export const parsePdfImageMask = (value: unknown): PdfImageMask => validateAgainst<PdfImageMask>(schema, "/$defs/PdfImageMask", value);
export const parsePdfFont = (value: unknown): PdfFont => validateAgainst<PdfFont>(schema, "/$defs/PdfFont", value);
export const parsePdfToUnicode = (value: unknown): PdfToUnicode => validateAgainst<PdfToUnicode>(schema, "/$defs/PdfToUnicode", value);
export const parsePdfToUnicodeMapping = (value: unknown): PdfToUnicodeMapping => validateAgainst<PdfToUnicodeMapping>(schema, "/$defs/PdfToUnicodeMapping", value);
export const parsePdfFontKind = (value: unknown): PdfFontKind => validateAgainst<PdfFontKind>(schema, "/$defs/PdfFontKind", value);
export const parsePdfCidFont = (value: unknown): PdfCidFont => validateAgainst<PdfCidFont>(schema, "/$defs/PdfCidFont", value);
export const parsePdfFontProgram = (value: unknown): PdfFontProgram => validateAgainst<PdfFontProgram>(schema, "/$defs/PdfFontProgram", value);
export const parsePdfCidToGid = (value: unknown): PdfCidToGid => validateAgainst<PdfCidToGid>(schema, "/$defs/PdfCidToGid", value);
export const parsePdfCidVerticalRun = (value: unknown): PdfCidVerticalRun => validateAgainst<PdfCidVerticalRun>(schema, "/$defs/PdfCidVerticalRun", value);
export const parsePdfCidWidthRun = (value: unknown): PdfCidWidthRun => validateAgainst<PdfCidWidthRun>(schema, "/$defs/PdfCidWidthRun", value);
export const parsePdfFontDescriptor = (value: unknown): PdfFontDescriptor => validateAgainst<PdfFontDescriptor>(schema, "/$defs/PdfFontDescriptor", value);
export const parsePdfCidSystemInfo = (value: unknown): PdfCidSystemInfo => validateAgainst<PdfCidSystemInfo>(schema, "/$defs/PdfCidSystemInfo", value);
export const parsePdfCMap = (value: unknown): PdfCMap => validateAgainst<PdfCMap>(schema, "/$defs/PdfCMap", value);
export const parsePdfEmbeddedCMap = (value:unknown):PdfEmbeddedCMap=>validateAgainst<PdfEmbeddedCMap>(schema,"/$defs/PdfEmbeddedCMap",value);
export const parsePdfCidMapping = (value: unknown): PdfCidMapping => validateAgainst<PdfCidMapping>(schema, "/$defs/PdfCidMapping", value);
export const parsePdfCodespaceRange = (value: unknown): PdfCodespaceRange => validateAgainst<PdfCodespaceRange>(schema, "/$defs/PdfCodespaceRange", value);
export const parsePdfCharProc = (value: unknown): PdfCharProc => validateAgainst<PdfCharProc>(schema, "/$defs/PdfCharProc", value);
export const parsePdfSimpleEncoding = (value: unknown): PdfSimpleEncoding => validateAgainst<PdfSimpleEncoding>(schema, "/$defs/PdfSimpleEncoding", value);
export const parsePdfEncodingDifference = (value: unknown): PdfEncodingDifference => validateAgainst<PdfEncodingDifference>(schema, "/$defs/PdfEncodingDifference", value);
export const parsePdfBaseEncoding = (value: unknown): PdfBaseEncoding => validateAgainst<PdfBaseEncoding>(schema, "/$defs/PdfBaseEncoding", value);
export const parsePdfPage = (value: unknown): PdfPage => validateAgainst<PdfPage>(schema, "/$defs/PdfPage", value);
export const parsePdfAnnotation = (value: unknown): PdfAnnotation => validateAgainst<PdfAnnotation>(schema, "/$defs/PdfAnnotation", value);
export const parsePdfMarkupAnnotation = (value: unknown): PdfMarkupAnnotation => validateAgainst<PdfMarkupAnnotation>(schema, "/$defs/PdfMarkupAnnotation", value);
export const parsePdfAppearance = (value: unknown): PdfAppearance => validateAgainst<PdfAppearance>(schema, "/$defs/PdfAppearance", value);
export const parsePdfAppearanceEntry = (value: unknown): PdfAppearanceEntry => validateAgainst<PdfAppearanceEntry>(schema, "/$defs/PdfAppearanceEntry", value);
export const parsePdfAppearanceState = (value: unknown): PdfAppearanceState => validateAgainst<PdfAppearanceState>(schema, "/$defs/PdfAppearanceState", value);
export const parsePdfBorderStyle = (value: unknown): PdfBorderStyle => validateAgainst<PdfBorderStyle>(schema, "/$defs/PdfBorderStyle", value);
export const parsePdfAnnotationKind = (value: unknown): PdfAnnotationKind => validateAgainst<PdfAnnotationKind>(schema, "/$defs/PdfAnnotationKind", value);

export const parsePdfImageBody = (value: unknown): PdfImageBody => validateAgainst<PdfImageBody>(schema, "/$defs/PdfImageBody", value);
