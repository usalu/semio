/** 🧬️ Canonical owned PDF1.7 domain with explicit admission from its native JSON schema. */
import type { Binary64 } from "../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
import { pdfCosFromNativeJson, pdfDictionaryFromNativeJson, pdfFunctionFromNativeJson, pdfColorFromNativeJson } from "./🪪️native-json/🟦️.ts";
import { pdfOperationFromNativeJson, pdfPropertyFromNativeJson, pdfInlineFromNativeJson, pdfTextFromNativeJson, pdfTextItemFromNativeJson } from "./🪪️native-json/🖋️content/🟦️.ts";
import { pdfFontFromNativeJson, pdfFontKindFromNativeJson, pdfCidFontFromNativeJson, pdfProgramFromNativeJson, pdfGidFromNativeJson, pdfWidthRunFromNativeJson, pdfVerticalRunFromNativeJson, pdfDescriptorFromNativeJson, pdfCMapFromNativeJson, pdfUnicodeFromNativeJson, pdfUnicodeMappingFromNativeJson, pdfCidMappingFromNativeJson, pdfCharProcFromNativeJson, pdfEncodingFromNativeJson } from "./🪪️native-json/🔤️font/🟦️.ts";
export type { Binary64 };
import { pdfImageFromNativeJson,pdfImageCodecFromNativeJson,pdfImageMaskFromNativeJson,pdfStateFromNativeJson,pdfSoftMaskFromNativeJson,pdfFormFromNativeJson,pdfGroupFromNativeJson,pdfShadingFromNativeJson,pdfShadingKindFromNativeJson,pdfPatternFromNativeJson,pdfPatternKindFromNativeJson,pdfCcittFromNativeJson } from "./🪪️native-json/🖼️resource/🟦️.ts";
import { pdfDestinationFromNativeJson,pdfFitFromNativeJson,pdfFileFromNativeJson,pdfActionFromNativeJson,pdfActionKindFromNativeJson,pdfOutlineFromNativeJson,pdfOpenActionFromNativeJson,pdfNamedDestinationFromNativeJson } from "./🪪️native-json/🎯️navigation/🟦️.ts";
import { pdfAnnotationFromNativeJson,pdfAnnotationKindFromNativeJson,pdfMarkupFromNativeJson,pdfAppearanceFromNativeJson,pdfAppearanceEntryFromNativeJson,pdfAppearanceStateFromNativeJson,pdfBorderFromNativeJson,pdfDateFromNativeJson } from "./🪪️native-json/📌️annotation/🟦️.ts";
import { pdfFieldFromNativeJson,pdfFieldKindFromNativeJson,pdfAcroFromNativeJson,pdfOptionalFromNativeJson,pdfOptionalGroupFromNativeJson } from "./🪪️native-json/📝️form/🟦️.ts";
import { pdfInfoFromNativeJson,pdfEmbeddedFromNativeJson,pdfIntentFromNativeJson,pdfEncryptionFromNativeJson,pdfPreferencesFromNativeJson,pdfMarkFromNativeJson } from "./🪪️native-json/📇️metadata/🟦️.ts";
import { pdfSnapshotFromNativeJson,pdfPageFromNativeJson,pdfIndirectFromNativeJson,pdfNamedColorFromNativeJson,pdfNamedPropertiesFromNativeJson,pdfLabelFromNativeJson } from "./🪪️native-json/📄️document/🟦️.ts";

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
}

export interface PdfDictEntry {
  key: string;
  value: PdfObject;
}

export type PdfObject =
  | { kind: "null" }
  | { kind: "bool"; value: boolean }
  | { kind: "int"; value: bigint }
  | ({ kind: "real" } & PdfDecimal)
  | { kind: "str"; value: number[] }
  | { kind: "name"; value: string }
  | { kind: "array"; value: PdfObject[] }
  | { kind: "dict"; value: PdfDictEntry[] }
  | ({ kind: "ref" } & ObjRef)
  | { kind: "stream"; dict: PdfDictEntry[]; data: number[]; filters: PdfStreamFilter[] };

export type PdfStreamFilter =
  | { kind: "flate"; predictor?: PdfPredictor | null }
  | { kind: "lzw"; predictor?: PdfPredictor | null; earlyChange: boolean }
  | { kind: "asciiHex" }
  | { kind: "ascii85" }
  | { kind: "runLength" }
  | { kind: "dct"; colorTransform?: number | null }
  | { kind: "jpx" }
  | { kind: "ccitt"; parameters: PdfCcittParameters }
  | { kind: "jbig2"; globals?: number[] | null }
  | { kind: "crypt"; name?: string | null };

export interface PdfCcittParameters {
  k?: number;
  columns?: number;
  rows?: number;
  blackIs1?: boolean;
  encodedByteAlign?: boolean;
  endOfLine?: boolean;
  endOfBlock?: boolean;
  damagedRowsBeforeError?: number;
}

export interface PdfPredictor {
  predictor: number;
  colors: number;
  bitsPerComponent: number;
  columns: number;
}

export interface ObjRef {
  num: number;
  gen: number;
}

export interface PdfDecimal {
  negative: boolean;
  coefficient: string;
  scale: number;
}

export interface PdfIndirectObject {
  id: ObjRef;
  value: PdfObject;
}

export interface PdfInfo {
  title?: string | null;
  author?: string | null;
  subject?: string | null;
  keywords?: string | null;
  creator?: string | null;
  producer?: string | null;
  creationDate?: PdfDate | null;
  modificationDate?: PdfDate | null;
  trapped?: string | null;
  extra?: PdfDictEntry[];
}

export interface PdfDate {
  year: number;
  month?: number;
  day?: number;
  hour?: number;
  minute?: number;
  second?: number;
  offsetMinutes?: number | null;
}

export interface PdfEncryption {
  algorithm: PdfEncryptionAlgorithm;
  permissions?: number;
  userPassword?: string;
  ownerPassword?: string | null;
  encryptMetadata?: boolean;
}

export type PdfEncryptionAlgorithm =
  | "rc4_40"
  | "rc4_128"
  | "aes128"
  | "aes256";

export interface PdfMarkInfo {
  marked?: boolean;
  userProperties?: boolean;
  suspects?: boolean;
}

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

export interface PdfViewerPreferences {
  hideToolbar?: boolean;
  hideMenubar?: boolean;
  hideWindowUi?: boolean;
  fitWindow?: boolean;
  centerWindow?: boolean;
  displayDocTitle?: boolean;
  nonFullScreenPageMode?: PdfPageMode | null;
  direction?: string | null;
  viewArea?: string | null;
  viewClip?: string | null;
  printArea?: string | null;
  printClip?: string | null;
  printScaling?: string | null;
  duplex?: string | null;
  pickTrayByPdfSize?: boolean;
  printPageRange?: number[];
  numCopies?: number | null;
  extra?: PdfDictEntry[];
}

export type PdfPageMode =
  | "useNone"
  | "useOutlines"
  | "useThumbs"
  | "fullScreen"
  | "useOc"
  | "useAttachments";

export type PdfPageLayout =
  | "singlePage"
  | "oneColumn"
  | "twoColumnLeft"
  | "twoColumnRight"
  | "twoPageLeft"
  | "twoPageRight";

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

export interface PdfOutputIntent {
  subtype: string;
  conditionIdentifier: string;
  condition?: string | null;
  registryName?: string | null;
  info?: string | null;
  profile?: number[] | null;
}

export interface PdfEmbeddedFile {
  id: string;
  fileName: string;
  description?: string | null;
  mimeType?: string | null;
  data: number[];
  creationDate?: PdfDate | null;
  modificationDate?: PdfDate | null;
  relationship?: string | null;
  listed?: boolean;
}

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

export interface PdfNamedProperties {
  name: string;
  entries: PdfDictEntry[];
}

export interface PdfNamedColorSpace {
  name: string;
  colorSpace: PdfColorSpace;
}

export type PdfColorSpace =
  | { kind: "deviceGray" }
  | { kind: "deviceRgb" }
  | { kind: "deviceCmyk" }
  | { kind: "calGray"; whitePoint: [Binary64, Binary64, Binary64]; blackPoint?: [Binary64, Binary64, Binary64] | null; gamma?: Binary64 | null }
  | { kind: "calRgb"; whitePoint: [Binary64, Binary64, Binary64]; blackPoint?: [Binary64, Binary64, Binary64] | null; gamma?: [Binary64, Binary64, Binary64] | null; matrix?: [Binary64, Binary64, Binary64, Binary64, Binary64, Binary64, Binary64, Binary64, Binary64] | null }
  | { kind: "lab"; whitePoint: [Binary64, Binary64, Binary64]; blackPoint?: [Binary64, Binary64, Binary64] | null; range?: [Binary64, Binary64, Binary64, Binary64] | null }
  | { kind: "iccBased"; components: number; profile: number[]; alternate?: PdfColorSpace | null; range?: Binary64[] | null }
  | { kind: "indexed"; base: PdfColorSpace; hival: number; lookup: number[] }
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

export interface PdfPattern {
  id: string;
  matrix?: [Binary64, Binary64, Binary64, Binary64, Binary64, Binary64];
  kind: PdfPatternKind;
  extra?: PdfDictEntry[];
}

export type PdfPatternKind =
  | { kind: "tiling"; paintType: number; tilingType: number; bbox: [Binary64, Binary64, Binary64, Binary64]; xStep: Binary64; yStep: Binary64; content: PdfOp[] }
  | { kind: "shading"; shading: string; extGState?: string | null };

export type PdfOp =
  | { op: "setLineWidth"; width: Binary64 }
  | { op: "setLineCap"; cap: PdfLineCap }
  | { op: "setLineJoin"; join: PdfLineJoin }
  | { op: "setMiterLimit"; limit: Binary64 }
  | { op: "setDash"; array: Binary64[]; phase: Binary64 }
  | { op: "setRenderingIntent"; intent: string }
  | { op: "setFlatness"; flatness: Binary64 }
  | { op: "setExtGState"; name: string }
  | { op: "save" }
  | { op: "restore" }
  | { op: "transform"; matrix: [Binary64, Binary64, Binary64, Binary64, Binary64, Binary64] }
  | { op: "moveTo"; x: Binary64; y: Binary64 }
  | { op: "lineTo"; x: Binary64; y: Binary64 }
  | { op: "curveTo"; x1: Binary64; y1: Binary64; x2: Binary64; y2: Binary64; x3: Binary64; y3: Binary64 }
  | { op: "curveToInitial"; x2: Binary64; y2: Binary64; x3: Binary64; y3: Binary64 }
  | { op: "curveToFinal"; x1: Binary64; y1: Binary64; x3: Binary64; y3: Binary64 }
  | { op: "closePath" }
  | { op: "rectangle"; x: Binary64; y: Binary64; width: Binary64; height: Binary64 }
  | { op: "stroke" }
  | { op: "closeStroke" }
  | { op: "fill" }
  | { op: "fillEvenOdd" }
  | { op: "fillStroke" }
  | { op: "fillStrokeEvenOdd" }
  | { op: "closeFillStroke" }
  | { op: "closeFillStrokeEvenOdd" }
  | { op: "endPath" }
  | { op: "clip" }
  | { op: "clipEvenOdd" }
  | { op: "beginText" }
  | { op: "endText" }
  | { op: "setCharSpacing"; spacing: Binary64 }
  | { op: "setWordSpacing"; spacing: Binary64 }
  | { op: "setHorizontalScale"; scale: Binary64 }
  | { op: "setLeading"; leading: Binary64 }
  | { op: "setFont"; name: string; size: Binary64 }
  | { op: "setTextRenderingMode"; mode: number }
  | { op: "setTextRise"; rise: Binary64 }
  | { op: "moveText"; tx: Binary64; ty: Binary64 }
  | { op: "moveTextSetLeading"; tx: Binary64; ty: Binary64 }
  | { op: "setTextMatrix"; matrix: [Binary64, Binary64, Binary64, Binary64, Binary64, Binary64] }
  | { op: "nextLine" }
  | { op: "showText"; text: PdfTextString }
  | { op: "showTextArray"; items: PdfTextArrayItem[] }
  | { op: "nextLineShowText"; text: PdfTextString }
  | { op: "nextLineShowTextSpaced"; wordSpacing: Binary64; charSpacing: Binary64; text: PdfTextString }
  | { op: "setGlyphWidth"; wx: Binary64; wy: Binary64 }
  | { op: "setGlyphWidthAndBox"; wx: Binary64; wy: Binary64; llx: Binary64; lly: Binary64; urx: Binary64; ury: Binary64 }
  | { op: "setStrokeColorSpace"; name: string }
  | { op: "setFillColorSpace"; name: string }
  | { op: "setStrokeColor"; components: Binary64[] }
  | { op: "setStrokeColorN"; components: Binary64[]; pattern?: string | null }
  | { op: "setFillColor"; components: Binary64[] }
  | { op: "setFillColorN"; components: Binary64[]; pattern?: string | null }
  | { op: "setStrokeGray"; gray: Binary64 }
  | { op: "setFillGray"; gray: Binary64 }
  | { op: "setStrokeRgb"; r: Binary64; g: Binary64; b: Binary64 }
  | { op: "setFillRgb"; r: Binary64; g: Binary64; b: Binary64 }
  | { op: "setStrokeCmyk"; c: Binary64; m: Binary64; y: Binary64; k: Binary64 }
  | { op: "setFillCmyk"; c: Binary64; m: Binary64; y: Binary64; k: Binary64 }
  | { op: "paintShading"; name: string }
  | { op: "paintXObject"; name: string }
  | { op: "inlineImage"; image: PdfInlineImage }
  | { op: "markedContentPoint"; tag: string }
  | { op: "markedContentPointWithProperties"; tag: string; properties: PdfPropertyList }
  | { op: "beginMarkedContent"; tag: string }
  | { op: "beginMarkedContentWithProperties"; tag: string; properties: PdfPropertyList }
  | { op: "endMarkedContent" }
  | { op: "beginCompatibility" }
  | { op: "endCompatibility" }
  | { op: "unknown"; operator: string; operands: PdfObject[] };

export type PdfPropertyList =
  | { kind: "named"; name: string }
  | { kind: "inline"; entries: PdfDictEntry[] };

export interface PdfInlineImage {
  width: number;
  height: number;
  bitsPerComponent?: number;
  colorSpace?: PdfColorSpace | null;
  imageMask?: boolean;
  decode?: Binary64[];
  interpolate?: boolean;
  filters?: PdfStreamFilter[];
  data: number[];
  extra?: PdfDictEntry[];
}

export type PdfTextString =
  | { kind: "text"; text: string }
  | { kind: "codes"; bytes: number[] };

export type PdfTextArrayItem =
  | { kind: "text"; text: string }
  | { kind: "codes"; bytes: number[] }
  | { kind: "adjust"; amount: Binary64 };

export type PdfLineJoin =
  | "miter"
  | "round"
  | "bevel";

export type PdfLineCap =
  | "butt"
  | "round"
  | "square";

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
  | { kind: "mesh"; shadingType: number; bitsPerCoordinate: number; bitsPerComponent: number; bitsPerFlag?: number | null; verticesPerRow?: number | null; decode: Binary64[]; function?: PdfFunction | null; data: number[] };

export interface PdfExtGState {
  id: string;
  lineWidth?: Binary64 | null;
  lineCap?: PdfLineCap | null;
  lineJoin?: PdfLineJoin | null;
  miterLimit?: Binary64 | null;
  dash?: [Binary64[], Binary64] | null;
  renderingIntent?: string | null;
  overprintStroke?: boolean | null;
  overprintFill?: boolean | null;
  overprintMode?: number | null;
  font?: [string, Binary64] | null;
  blendMode?: string[] | null;
  softMask?: PdfSoftMask | null;
  strokeAlpha?: Binary64 | null;
  fillAlpha?: Binary64 | null;
  alphaIsShape?: boolean | null;
  strokeAdjust?: boolean | null;
  flatness?: Binary64 | null;
  smoothness?: Binary64 | null;
  textKnockout?: boolean | null;
  extra?: PdfDictEntry[];
}

export type PdfSoftMask =
  | { kind: "none" }
  | { kind: "alpha"; group: string; transfer?: PdfFunction | null }
  | { kind: "luminosity"; group: string; backdrop?: Binary64[] | null; transfer?: PdfFunction | null };

export interface PdfFormXObject {
  id: string;
  bbox: [Binary64, Binary64, Binary64, Binary64];
  matrix?: [Binary64, Binary64, Binary64, Binary64, Binary64, Binary64];
  content?: PdfOp[];
  group?: PdfTransparencyGroup | null;
  optionalContent?: string | null;
  structParent?: number | null;
  extra?: PdfDictEntry[];
}

export interface PdfTransparencyGroup {
  colorSpace?: PdfColorSpace | null;
  isolated?: boolean;
  knockout?: boolean;
}

export interface PdfImage {
  id: string;
  width: number;
  height: number;
  colorSpace?: PdfColorSpace | null;
  bitsPerComponent?: number;
  imageMask?: boolean;
  decode?: Binary64[];
  interpolate?: boolean;
  codec?: PdfImageCodec;
  data: number[];
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

export type PdfImageCodec =
  | { kind: "raw" }
  | { kind: "dct"; colorTransform?: number | null }
  | { kind: "jpx" }
  | { kind: "ccitt"; parameters: PdfCcittParameters }
  | { kind: "jbig2"; globals?: number[] | null };

export interface PdfFont {
  id: string;
  kind: PdfFontKind;
  toUnicode?: PdfToUnicode | null;
  extra?: PdfDictEntry[];
}

export interface PdfToUnicode {
  byteWidth: number;
  mappings?: PdfToUnicodeMapping[];
}

export type PdfToUnicodeMapping =
  | { kind: "char"; code: number; text: string }
  | { kind: "range"; low: number; high: number; text: string };

export type PdfFontKind =
  | { kind: "type1"; baseFont: string; encoding: PdfSimpleEncoding; firstChar: number; widths: Binary64[]; descriptor?: PdfFontDescriptor | null; program?: PdfFontProgram | null }
  | { kind: "trueType"; baseFont: string; encoding: PdfSimpleEncoding; firstChar: number; widths: Binary64[]; descriptor?: PdfFontDescriptor | null; program?: PdfFontProgram | null }
  | { kind: "type3"; fontMatrix: [Binary64, Binary64, Binary64, Binary64, Binary64, Binary64]; fontBbox: [Binary64, Binary64, Binary64, Binary64]; encoding: PdfSimpleEncoding; firstChar: number; widths: Binary64[]; charProcs: PdfCharProc[]; descriptor?: PdfFontDescriptor | null }
  | { kind: "type0"; baseFont: string; cmap: PdfCMap; descendant: PdfCidFont };

export interface PdfCidFont {
  trueType: boolean;
  baseFont: string;
  systemInfo?: PdfCidSystemInfo;
  descriptor: PdfFontDescriptor;
  defaultWidth?: Binary64;
  widths?: PdfCidWidthRun[];
  defaultVertical?: [Binary64, Binary64] | null;
  verticalMetrics?: PdfCidVerticalRun[];
  cidToGid?: PdfCidToGid | null;
  program?: PdfFontProgram | null;
  extra?: PdfDictEntry[];
}

export type PdfFontProgram =
  | { kind: "type1"; data: number[]; length1: number; length2: number; length3: number }
  | { kind: "trueType"; data: number[] }
  | { kind: "cff"; data: number[] }
  | { kind: "cidCff"; data: number[] }
  | { kind: "openType"; data: number[] };

export type PdfCidToGid =
  | { kind: "identity" }
  | { kind: "map"; data: number[] };

export interface PdfCidVerticalRun {
  startCid: number;
  metrics: [Binary64, Binary64, Binary64][];
}

export interface PdfCidWidthRun {
  startCid: number;
  widths: Binary64[];
}

export interface PdfFontDescriptor {
  fontName: string;
  flags?: number;
  fontBbox?: [Binary64, Binary64, Binary64, Binary64];
  italicAngle?: Binary64;
  ascent?: Binary64;
  descent?: Binary64;
  capHeight?: Binary64;
  stemV?: Binary64;
  stemH?: Binary64 | null;
  xHeight?: Binary64 | null;
  leading?: Binary64 | null;
  avgWidth?: Binary64 | null;
  maxWidth?: Binary64 | null;
  missingWidth?: Binary64 | null;
  fontFamily?: string | null;
  fontStretch?: string | null;
  fontWeight?: Binary64 | null;
  charSet?: string | null;
  extra?: PdfDictEntry[];
}

export interface PdfCidSystemInfo {
  registry: string;
  ordering: string;
  supplement: number;
}

export type PdfCMap =
  | { kind: "predefined"; name: string }
  | { kind: "embedded"; cmap: PdfEmbeddedCMap };

export interface PdfEmbeddedCMap {
  name: string;
  vertical?: boolean;
  codespace?: PdfCodespaceRange[];
  mappings?: PdfCidMapping[];
  useCmap?: string | null;
}

export type PdfCidMapping =
  | { kind: "char"; code: number; cid: number }
  | { kind: "range"; low: number; high: number; cid: number };

export interface PdfCodespaceRange {
  byteWidth: number;
  low: number;
  high: number;
}

export interface PdfCharProc {
  name: string;
  content: PdfOp[];
}

export interface PdfSimpleEncoding {
  base?: PdfBaseEncoding | null;
  differences?: PdfEncodingDifference[];
}

export interface PdfEncodingDifference {
  code: number;
  glyph: string;
}

export type PdfBaseEncoding =
  | "standard"
  | "winAnsi"
  | "macRoman"
  | "macExpert";

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

/** 🔣️ The JSON Schema document this facet is validated against. */
export const schema = {
  "$schema": "http://json-schema.org/draft-07/schema#",
  "$id": "https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.7/base/snapshot.json",
  "title": "PdfSnapshot",
  "type": "object",
  "properties": {
    "schema": {
      "type": "string"
    },
    "declaredVersion": {
      "type": "string"
    },
    "pages": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfPage"
      }
    },
    "fonts": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfFont"
      }
    },
    "images": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfImage"
      }
    },
    "forms": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfFormXObject"
      }
    },
    "extGStates": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfExtGState"
      }
    },
    "shadings": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfShading"
      }
    },
    "patterns": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfPattern"
      }
    },
    "colorSpaces": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfNamedColorSpace"
      }
    },
    "properties": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfNamedProperties"
      }
    },
    "outlines": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfOutlineItem"
      }
    },
    "namedDestinations": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfNamedDestination"
      }
    },
    "pageLabels": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfPageLabelRange"
      }
    },
    "embeddedFiles": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfEmbeddedFile"
      }
    },
    "outputIntents": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfOutputIntent"
      }
    },
    "acroForm": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfAcroForm"
        },
        {
          "type": "null"
        }
      ]
    },
    "optionalContent": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfOptionalContent"
        },
        {
          "type": "null"
        }
      ]
    },
    "pageLayout": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfPageLayout"
        },
        {
          "type": "null"
        }
      ]
    },
    "pageMode": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfPageMode"
        },
        {
          "type": "null"
        }
      ]
    },
    "viewerPreferences": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfViewerPreferences"
        },
        {
          "type": "null"
        }
      ]
    },
    "openAction": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfOpenAction"
        },
        {
          "type": "null"
        }
      ]
    },
    "language": {
      "anyOf": [
        {
          "type": "string"
        },
        {
          "type": "null"
        }
      ]
    },
    "markInfo": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfMarkInfo"
        },
        {
          "type": "null"
        }
      ]
    },
    "metadata": {
      "anyOf": [
        {
          "type": "string"
        },
        {
          "type": "null"
        }
      ]
    },
    "documentId": {
      "anyOf": [
        {
          "type": "array",
          "items": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 0
            }
          },
          "minItems": 2,
          "maxItems": 2
        },
        {
          "type": "null"
        }
      ]
    },
    "encryption": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfEncryption"
        },
        {
          "type": "null"
        }
      ]
    },
    "info": {
      "$ref": "#/$defs/PdfInfo"
    },
    "catalogExtra": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfDictEntry"
      }
    },
    "objects": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfIndirectObject"
      }
    },
    "trailer": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfDictEntry"
      }
    }
  },
  "required": [
    "schema",
    "declaredVersion",
    "pages",
    "fonts",
    "images",
    "forms",
    "extGStates",
    "shadings",
    "patterns",
    "colorSpaces",
    "properties",
    "outlines",
    "namedDestinations",
    "pageLabels",
    "embeddedFiles",
    "outputIntents",
    "info",
    "catalogExtra",
    "objects",
    "trailer"
  ],
  "$defs": {
    "ObjRef": {
      "type": "object",
      "properties": {
        "num": {
          "type": "integer",
          "minimum": 0
        },
        "gen": {
          "type": "integer",
          "minimum": 0
        }
      },
      "required": [
        "num",
        "gen"
      ]
    },
    "PdfAcroForm": {
      "type": "object",
      "properties": {
        "fields": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfFormField"
          }
        },
        "needAppearances": {
          "type": "boolean"
        },
        "signatureFlags": {
          "type": "integer",
          "minimum": 0
        },
        "defaultAppearance": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "quadding": {
          "anyOf": [
            {
              "type": "integer",
              "minimum": 0
            },
            {
              "type": "null"
            }
          ]
        },
        "defaultFonts": {
          "type": "array",
          "items": {
            "type": "string"
          }
        },
        "extra": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfDictEntry"
          }
        }
      }
    },
    "PdfAction": {
      "type": "object",
      "properties": {
        "kind": {
          "$ref": "#/$defs/PdfActionKind"
        },
        "next": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfAction"
          }
        }
      },
      "required": [
        "kind"
      ]
    },
    "PdfActionKind": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "goTo"
            },
            "destination": {
              "$ref": "#/$defs/PdfDestination"
            }
          },
          "required": [
            "kind",
            "destination"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "goToRemote"
            },
            "file": {
              "$ref": "#/$defs/PdfFileSpecification"
            },
            "destination": {
              "$ref": "#/$defs/PdfDestination"
            },
            "newWindow": {
              "anyOf": [
                {
                  "type": "boolean"
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "required": [
            "kind",
            "file",
            "destination"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "goToEmbedded"
            },
            "destination": {
              "$ref": "#/$defs/PdfDestination"
            },
            "newWindow": {
              "anyOf": [
                {
                  "type": "boolean"
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "required": [
            "kind",
            "destination"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "launch"
            },
            "file": {
              "$ref": "#/$defs/PdfFileSpecification"
            },
            "newWindow": {
              "anyOf": [
                {
                  "type": "boolean"
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "required": [
            "kind",
            "file"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "thread"
            },
            "file": {
              "anyOf": [
                {
                  "$ref": "#/$defs/PdfFileSpecification"
                },
                {
                  "type": "null"
                }
              ]
            },
            "thread": {
              "type": "integer",
              "minimum": 0
            }
          },
          "required": [
            "kind",
            "thread"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "uri"
            },
            "uri": {
              "type": "string"
            },
            "isMap": {
              "type": "boolean"
            }
          },
          "required": [
            "kind",
            "uri",
            "isMap"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "sound"
            },
            "sound": {
              "type": "string"
            },
            "volume": {
              "anyOf": [
                {
                  "type": "number"
                },
                {
                  "type": "null"
                }
              ]
            },
            "synchronous": {
              "type": "boolean"
            },
            "repeat": {
              "type": "boolean"
            },
            "mix": {
              "type": "boolean"
            }
          },
          "required": [
            "kind",
            "sound",
            "synchronous",
            "repeat",
            "mix"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "movie"
            },
            "annotation": {
              "anyOf": [
                {
                  "type": "string"
                },
                {
                  "type": "null"
                }
              ]
            },
            "operation": {
              "anyOf": [
                {
                  "type": "string"
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "hide"
            },
            "annotations": {
              "type": "array",
              "items": {
                "type": "string"
              }
            },
            "hide": {
              "type": "boolean"
            }
          },
          "required": [
            "kind",
            "annotations",
            "hide"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "named"
            },
            "name": {
              "type": "string"
            }
          },
          "required": [
            "kind",
            "name"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "submitForm"
            },
            "url": {
              "type": "string"
            },
            "fields": {
              "type": "array",
              "items": {
                "type": "string"
              }
            },
            "flags": {
              "type": "integer",
              "minimum": 0
            }
          },
          "required": [
            "kind",
            "url",
            "fields",
            "flags"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "resetForm"
            },
            "fields": {
              "type": "array",
              "items": {
                "type": "string"
              }
            },
            "flags": {
              "type": "integer",
              "minimum": 0
            }
          },
          "required": [
            "kind",
            "fields",
            "flags"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "importData"
            },
            "file": {
              "$ref": "#/$defs/PdfFileSpecification"
            }
          },
          "required": [
            "kind",
            "file"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "javaScript"
            },
            "script": {
              "type": "string"
            }
          },
          "required": [
            "kind",
            "script"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "setOptionalContentState"
            },
            "states": {
              "type": "array",
              "items": {
                "$ref": "#/$defs/PdfDictEntry"
              }
            },
            "preserveRadioButtons": {
              "type": "boolean"
            }
          },
          "required": [
            "kind",
            "states",
            "preserveRadioButtons"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "rendition"
            },
            "entries": {
              "type": "array",
              "items": {
                "$ref": "#/$defs/PdfDictEntry"
              }
            }
          },
          "required": [
            "kind",
            "entries"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "transition"
            },
            "entries": {
              "type": "array",
              "items": {
                "$ref": "#/$defs/PdfDictEntry"
              }
            }
          },
          "required": [
            "kind",
            "entries"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "goTo3dView"
            },
            "entries": {
              "type": "array",
              "items": {
                "$ref": "#/$defs/PdfDictEntry"
              }
            }
          },
          "required": [
            "kind",
            "entries"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "unknown"
            },
            "subtype": {
              "type": "string"
            },
            "entries": {
              "type": "array",
              "items": {
                "$ref": "#/$defs/PdfDictEntry"
              }
            }
          },
          "required": [
            "kind",
            "subtype",
            "entries"
          ]
        }
      ]
    },
    "PdfAnnotation": {
      "type": "object",
      "properties": {
        "rect": {
          "type": "array",
          "items": {
            "type": "number"
          },
          "minItems": 4,
          "maxItems": 4
        },
        "kind": {
          "$ref": "#/$defs/PdfAnnotationKind"
        },
        "contents": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "name": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "modified": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "flags": {
          "type": "integer",
          "minimum": 0
        },
        "border": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfBorderStyle"
            },
            {
              "type": "null"
            }
          ]
        },
        "color": {
          "type": "array",
          "items": {
            "type": "number"
          }
        },
        "appearance": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfAppearance"
            },
            {
              "type": "null"
            }
          ]
        },
        "appearanceState": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "markup": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfMarkupAnnotation"
            },
            {
              "type": "null"
            }
          ]
        },
        "optionalContent": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "structParent": {
          "anyOf": [
            {
              "type": "integer",
              "minimum": 0
            },
            {
              "type": "null"
            }
          ]
        },
        "extra": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfDictEntry"
          }
        }
      },
      "required": [
        "rect",
        "kind"
      ]
    },
    "PdfAnnotationKind": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "text"
            },
            "open": {
              "type": "boolean"
            },
            "icon": {
              "anyOf": [
                {
                  "type": "string"
                },
                {
                  "type": "null"
                }
              ]
            },
            "state": {
              "anyOf": [
                {
                  "type": "string"
                },
                {
                  "type": "null"
                }
              ]
            },
            "stateModel": {
              "anyOf": [
                {
                  "type": "string"
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "required": [
            "kind",
            "open"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "link"
            },
            "action": {
              "anyOf": [
                {
                  "$ref": "#/$defs/PdfAction"
                },
                {
                  "type": "null"
                }
              ]
            },
            "destination": {
              "anyOf": [
                {
                  "$ref": "#/$defs/PdfDestination"
                },
                {
                  "type": "null"
                }
              ]
            },
            "highlight": {
              "anyOf": [
                {
                  "type": "string"
                },
                {
                  "type": "null"
                }
              ]
            },
            "quadPoints": {
              "type": "array",
              "items": {
                "type": "number"
              }
            }
          },
          "required": [
            "kind",
            "quadPoints"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "freeText"
            },
            "defaultAppearance": {
              "type": "string"
            },
            "quadding": {
              "type": "integer",
              "minimum": 0
            },
            "callout": {
              "anyOf": [
                {
                  "type": "array",
                  "items": {
                    "type": "number"
                  }
                },
                {
                  "type": "null"
                }
              ]
            },
            "lineEnding": {
              "anyOf": [
                {
                  "type": "string"
                },
                {
                  "type": "null"
                }
              ]
            },
            "richText": {
              "anyOf": [
                {
                  "type": "string"
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "required": [
            "kind",
            "defaultAppearance",
            "quadding"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "line"
            },
            "points": {
              "type": "array",
              "items": {
                "type": "number"
              },
              "minItems": 4,
              "maxItems": 4
            },
            "lineEndings": {
              "anyOf": [
                {
                  "type": "array",
                  "items": {
                    "type": "string"
                  },
                  "minItems": 2,
                  "maxItems": 2
                },
                {
                  "type": "null"
                }
              ]
            },
            "interiorColor": {
              "anyOf": [
                {
                  "type": "array",
                  "items": {
                    "type": "number"
                  }
                },
                {
                  "type": "null"
                }
              ]
            },
            "leaderLength": {
              "anyOf": [
                {
                  "type": "number"
                },
                {
                  "type": "null"
                }
              ]
            },
            "caption": {
              "type": "boolean"
            }
          },
          "required": [
            "kind",
            "points",
            "caption"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "square"
            },
            "interiorColor": {
              "anyOf": [
                {
                  "type": "array",
                  "items": {
                    "type": "number"
                  }
                },
                {
                  "type": "null"
                }
              ]
            },
            "rectDifferences": {
              "anyOf": [
                {
                  "type": "array",
                  "items": {
                    "type": "number"
                  },
                  "minItems": 4,
                  "maxItems": 4
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "circle"
            },
            "interiorColor": {
              "anyOf": [
                {
                  "type": "array",
                  "items": {
                    "type": "number"
                  }
                },
                {
                  "type": "null"
                }
              ]
            },
            "rectDifferences": {
              "anyOf": [
                {
                  "type": "array",
                  "items": {
                    "type": "number"
                  },
                  "minItems": 4,
                  "maxItems": 4
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "polygon"
            },
            "vertices": {
              "type": "array",
              "items": {
                "type": "number"
              }
            },
            "interiorColor": {
              "anyOf": [
                {
                  "type": "array",
                  "items": {
                    "type": "number"
                  }
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "required": [
            "kind",
            "vertices"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "polyLine"
            },
            "vertices": {
              "type": "array",
              "items": {
                "type": "number"
              }
            },
            "lineEndings": {
              "anyOf": [
                {
                  "type": "array",
                  "items": {
                    "type": "string"
                  },
                  "minItems": 2,
                  "maxItems": 2
                },
                {
                  "type": "null"
                }
              ]
            },
            "interiorColor": {
              "anyOf": [
                {
                  "type": "array",
                  "items": {
                    "type": "number"
                  }
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "required": [
            "kind",
            "vertices"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "highlight"
            },
            "quadPoints": {
              "type": "array",
              "items": {
                "type": "number"
              }
            }
          },
          "required": [
            "kind",
            "quadPoints"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "underline"
            },
            "quadPoints": {
              "type": "array",
              "items": {
                "type": "number"
              }
            }
          },
          "required": [
            "kind",
            "quadPoints"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "squiggly"
            },
            "quadPoints": {
              "type": "array",
              "items": {
                "type": "number"
              }
            }
          },
          "required": [
            "kind",
            "quadPoints"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "strikeOut"
            },
            "quadPoints": {
              "type": "array",
              "items": {
                "type": "number"
              }
            }
          },
          "required": [
            "kind",
            "quadPoints"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "stamp"
            },
            "icon": {
              "anyOf": [
                {
                  "type": "string"
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "caret"
            },
            "rectDifferences": {
              "anyOf": [
                {
                  "type": "array",
                  "items": {
                    "type": "number"
                  },
                  "minItems": 4,
                  "maxItems": 4
                },
                {
                  "type": "null"
                }
              ]
            },
            "symbol": {
              "anyOf": [
                {
                  "type": "string"
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "ink"
            },
            "paths": {
              "type": "array",
              "items": {
                "type": "array",
                "items": {
                  "type": "number"
                }
              }
            }
          },
          "required": [
            "kind",
            "paths"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "popup"
            },
            "parent": {
              "anyOf": [
                {
                  "type": "integer",
                  "minimum": 0
                },
                {
                  "type": "null"
                }
              ]
            },
            "open": {
              "type": "boolean"
            }
          },
          "required": [
            "kind",
            "open"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "fileAttachment"
            },
            "file": {
              "$ref": "#/$defs/PdfFileSpecification"
            },
            "icon": {
              "anyOf": [
                {
                  "type": "string"
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "required": [
            "kind",
            "file"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "sound"
            },
            "sound": {
              "type": "array",
              "items": {
                "$ref": "#/$defs/PdfDictEntry"
              }
            },
            "icon": {
              "anyOf": [
                {
                  "type": "string"
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "required": [
            "kind",
            "sound"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "movie"
            },
            "title": {
              "anyOf": [
                {
                  "type": "string"
                },
                {
                  "type": "null"
                }
              ]
            },
            "movie": {
              "type": "array",
              "items": {
                "$ref": "#/$defs/PdfDictEntry"
              }
            },
            "activation": {
              "anyOf": [
                {
                  "type": "array",
                  "items": {
                    "$ref": "#/$defs/PdfDictEntry"
                  }
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "required": [
            "kind",
            "movie"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "widget"
            },
            "field": {
              "anyOf": [
                {
                  "type": "string"
                },
                {
                  "type": "null"
                }
              ]
            },
            "highlight": {
              "anyOf": [
                {
                  "type": "string"
                },
                {
                  "type": "null"
                }
              ]
            },
            "characteristics": {
              "type": "array",
              "items": {
                "$ref": "#/$defs/PdfDictEntry"
              }
            },
            "action": {
              "anyOf": [
                {
                  "$ref": "#/$defs/PdfAction"
                },
                {
                  "type": "null"
                }
              ]
            },
            "additionalActions": {
              "type": "array",
              "items": {
                "$ref": "#/$defs/PdfDictEntry"
              }
            }
          },
          "required": [
            "kind",
            "characteristics",
            "additionalActions"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "screen"
            },
            "title": {
              "anyOf": [
                {
                  "type": "string"
                },
                {
                  "type": "null"
                }
              ]
            },
            "characteristics": {
              "type": "array",
              "items": {
                "$ref": "#/$defs/PdfDictEntry"
              }
            },
            "action": {
              "anyOf": [
                {
                  "$ref": "#/$defs/PdfAction"
                },
                {
                  "type": "null"
                }
              ]
            },
            "additionalActions": {
              "type": "array",
              "items": {
                "$ref": "#/$defs/PdfDictEntry"
              }
            }
          },
          "required": [
            "kind",
            "characteristics",
            "additionalActions"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "printerMark"
            },
            "markStyle": {
              "anyOf": [
                {
                  "type": "string"
                },
                {
                  "type": "null"
                }
              ]
            },
            "colorants": {
              "type": "array",
              "items": {
                "$ref": "#/$defs/PdfDictEntry"
              }
            }
          },
          "required": [
            "kind",
            "colorants"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "trapNet"
            },
            "entries": {
              "type": "array",
              "items": {
                "$ref": "#/$defs/PdfDictEntry"
              }
            }
          },
          "required": [
            "kind",
            "entries"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "watermark"
            },
            "fixedPrint": {
              "anyOf": [
                {
                  "type": "array",
                  "items": {
                    "$ref": "#/$defs/PdfDictEntry"
                  }
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "threeD"
            },
            "entries": {
              "type": "array",
              "items": {
                "$ref": "#/$defs/PdfDictEntry"
              }
            }
          },
          "required": [
            "kind",
            "entries"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "redact"
            },
            "quadPoints": {
              "type": "array",
              "items": {
                "type": "number"
              }
            },
            "interiorColor": {
              "anyOf": [
                {
                  "type": "array",
                  "items": {
                    "type": "number"
                  }
                },
                {
                  "type": "null"
                }
              ]
            },
            "overlayText": {
              "anyOf": [
                {
                  "type": "string"
                },
                {
                  "type": "null"
                }
              ]
            },
            "repeat": {
              "type": "boolean"
            },
            "defaultAppearance": {
              "anyOf": [
                {
                  "type": "string"
                },
                {
                  "type": "null"
                }
              ]
            },
            "quadding": {
              "type": "integer",
              "minimum": 0
            }
          },
          "required": [
            "kind",
            "quadPoints",
            "repeat",
            "quadding"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "unknown"
            },
            "subtype": {
              "type": "string"
            },
            "entries": {
              "type": "array",
              "items": {
                "$ref": "#/$defs/PdfDictEntry"
              }
            }
          },
          "required": [
            "kind",
            "subtype",
            "entries"
          ]
        }
      ]
    },
    "PdfAppearance": {
      "type": "object",
      "properties": {
        "normal": {
          "$ref": "#/$defs/PdfAppearanceEntry"
        },
        "rollover": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfAppearanceEntry"
            },
            {
              "type": "null"
            }
          ]
        },
        "down": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfAppearanceEntry"
            },
            {
              "type": "null"
            }
          ]
        }
      },
      "required": [
        "normal"
      ]
    },
    "PdfAppearanceEntry": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "single"
            },
            "form": {
              "type": "string"
            }
          },
          "required": [
            "kind",
            "form"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "states"
            },
            "states": {
              "type": "array",
              "items": {
                "$ref": "#/$defs/PdfAppearanceState"
              }
            }
          },
          "required": [
            "kind",
            "states"
          ]
        }
      ]
    },
    "PdfAppearanceState": {
      "type": "object",
      "properties": {
        "state": {
          "type": "string"
        },
        "form": {
          "type": "string"
        }
      },
      "required": [
        "state",
        "form"
      ]
    },
    "PdfBaseEncoding": {
      "type": "string",
      "enum": [
        "standard",
        "winAnsi",
        "macRoman",
        "macExpert"
      ]
    },
    "PdfBorderStyle": {
      "type": "object",
      "properties": {
        "width": {
          "type": "number"
        },
        "style": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "dash": {
          "anyOf": [
            {
              "type": "array",
              "items": {
                "type": "number"
              }
            },
            {
              "type": "null"
            }
          ]
        },
        "radii": {
          "anyOf": [
            {
              "type": "array",
              "items": {
                "type": "number"
              },
              "minItems": 2,
              "maxItems": 2
            },
            {
              "type": "null"
            }
          ]
        }
      },
      "required": [
        "width"
      ]
    },
    "PdfCMap": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "predefined"
            },
            "name": {
              "type": "string"
            }
          },
          "required": [
            "kind",
            "name"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "embedded"
            },
            "cmap": {
              "$ref": "#/$defs/PdfEmbeddedCMap"
            }
          },
          "required": [
            "kind",
            "cmap"
          ]
        }
      ]
    },
    "PdfCcittParameters": {
      "type": "object",
      "properties": {
        "k": {
          "type": "integer"
        },
        "columns": {
          "type": "integer",
          "minimum": 0
        },
        "rows": {
          "type": "integer",
          "minimum": 0
        },
        "blackIs1": {
          "type": "boolean"
        },
        "encodedByteAlign": {
          "type": "boolean"
        },
        "endOfLine": {
          "type": "boolean"
        },
        "endOfBlock": {
          "type": "boolean"
        },
        "damagedRowsBeforeError": {
          "type": "integer",
          "minimum": 0
        }
      }
    },
    "PdfCharProc": {
      "type": "object",
      "properties": {
        "name": {
          "type": "string"
        },
        "content": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfOp"
          }
        }
      },
      "required": [
        "name",
        "content"
      ]
    },
    "PdfCidFont": {
      "type": "object",
      "properties": {
        "trueType": {
          "type": "boolean"
        },
        "baseFont": {
          "type": "string"
        },
        "systemInfo": {
          "$ref": "#/$defs/PdfCidSystemInfo"
        },
        "descriptor": {
          "$ref": "#/$defs/PdfFontDescriptor"
        },
        "defaultWidth": {
          "type": "number"
        },
        "widths": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfCidWidthRun"
          }
        },
        "defaultVertical": {
          "anyOf": [
            {
              "type": "array",
              "items": {
                "type": "number"
              },
              "minItems": 2,
              "maxItems": 2
            },
            {
              "type": "null"
            }
          ]
        },
        "verticalMetrics": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfCidVerticalRun"
          }
        },
        "cidToGid": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfCidToGid"
            },
            {
              "type": "null"
            }
          ]
        },
        "program": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfFontProgram"
            },
            {
              "type": "null"
            }
          ]
        },
        "extra": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfDictEntry"
          }
        }
      },
      "required": [
        "trueType",
        "baseFont",
        "descriptor"
      ]
    },
    "PdfCidMapping": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "char"
            },
            "code": {
              "type": "integer",
              "minimum": 0
            },
            "cid": {
              "type": "integer",
              "minimum": 0
            }
          },
          "required": [
            "kind",
            "code",
            "cid"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "range"
            },
            "low": {
              "type": "integer",
              "minimum": 0
            },
            "high": {
              "type": "integer",
              "minimum": 0
            },
            "cid": {
              "type": "integer",
              "minimum": 0
            }
          },
          "required": [
            "kind",
            "low",
            "high",
            "cid"
          ]
        }
      ]
    },
    "PdfCidSystemInfo": {
      "type": "object",
      "properties": {
        "registry": {
          "type": "string"
        },
        "ordering": {
          "type": "string"
        },
        "supplement": {
          "type": "integer",
          "minimum": 0
        }
      },
      "required": [
        "registry",
        "ordering",
        "supplement"
      ]
    },
    "PdfCidToGid": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "identity"
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "map"
            },
            "data": {
              "type": "array",
              "items": {
                "type": "integer",
                "minimum": 0
              }
            }
          },
          "required": [
            "kind",
            "data"
          ]
        }
      ]
    },
    "PdfCidVerticalRun": {
      "type": "object",
      "properties": {
        "startCid": {
          "type": "integer",
          "minimum": 0
        },
        "metrics": {
          "type": "array",
          "items": {
            "type": "array",
            "items": {
              "type": "number"
            },
            "minItems": 3,
            "maxItems": 3
          }
        }
      },
      "required": [
        "startCid",
        "metrics"
      ]
    },
    "PdfCidWidthRun": {
      "type": "object",
      "properties": {
        "startCid": {
          "type": "integer",
          "minimum": 0
        },
        "widths": {
          "type": "array",
          "items": {
            "type": "number"
          }
        }
      },
      "required": [
        "startCid",
        "widths"
      ]
    },
    "PdfCodespaceRange": {
      "type": "object",
      "properties": {
        "byteWidth": {
          "type": "integer",
          "minimum": 0
        },
        "low": {
          "type": "integer",
          "minimum": 0
        },
        "high": {
          "type": "integer",
          "minimum": 0
        }
      },
      "required": [
        "byteWidth",
        "low",
        "high"
      ]
    },
    "PdfColorSpace": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "deviceGray"
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "deviceRgb"
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "deviceCmyk"
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "calGray"
            },
            "whitePoint": {
              "type": "array",
              "items": {
                "type": "number"
              },
              "minItems": 3,
              "maxItems": 3
            },
            "blackPoint": {
              "anyOf": [
                {
                  "type": "array",
                  "items": {
                    "type": "number"
                  },
                  "minItems": 3,
                  "maxItems": 3
                },
                {
                  "type": "null"
                }
              ]
            },
            "gamma": {
              "anyOf": [
                {
                  "type": "number"
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "required": [
            "kind",
            "whitePoint"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "calRgb"
            },
            "whitePoint": {
              "type": "array",
              "items": {
                "type": "number"
              },
              "minItems": 3,
              "maxItems": 3
            },
            "blackPoint": {
              "anyOf": [
                {
                  "type": "array",
                  "items": {
                    "type": "number"
                  },
                  "minItems": 3,
                  "maxItems": 3
                },
                {
                  "type": "null"
                }
              ]
            },
            "gamma": {
              "anyOf": [
                {
                  "type": "array",
                  "items": {
                    "type": "number"
                  },
                  "minItems": 3,
                  "maxItems": 3
                },
                {
                  "type": "null"
                }
              ]
            },
            "matrix": {
              "anyOf": [
                {
                  "type": "array",
                  "items": {
                    "type": "number"
                  },
                  "minItems": 9,
                  "maxItems": 9
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "required": [
            "kind",
            "whitePoint"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "lab"
            },
            "whitePoint": {
              "type": "array",
              "items": {
                "type": "number"
              },
              "minItems": 3,
              "maxItems": 3
            },
            "blackPoint": {
              "anyOf": [
                {
                  "type": "array",
                  "items": {
                    "type": "number"
                  },
                  "minItems": 3,
                  "maxItems": 3
                },
                {
                  "type": "null"
                }
              ]
            },
            "range": {
              "anyOf": [
                {
                  "type": "array",
                  "items": {
                    "type": "number"
                  },
                  "minItems": 4,
                  "maxItems": 4
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "required": [
            "kind",
            "whitePoint"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "iccBased"
            },
            "components": {
              "type": "integer",
              "minimum": 0
            },
            "profile": {
              "type": "array",
              "items": {
                "type": "integer",
                "minimum": 0
              }
            },
            "alternate": {
              "anyOf": [
                {
                  "$ref": "#/$defs/PdfColorSpace"
                },
                {
                  "type": "null"
                }
              ]
            },
            "range": {
              "anyOf": [
                {
                  "type": "array",
                  "items": {
                    "type": "number"
                  }
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "required": [
            "kind",
            "components",
            "profile"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "indexed"
            },
            "base": {
              "$ref": "#/$defs/PdfColorSpace"
            },
            "hival": {
              "type": "integer",
              "minimum": 0
            },
            "lookup": {
              "type": "array",
              "items": {
                "type": "integer",
                "minimum": 0
              }
            }
          },
          "required": [
            "kind",
            "base",
            "hival",
            "lookup"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "separation"
            },
            "name": {
              "type": "string"
            },
            "alternate": {
              "$ref": "#/$defs/PdfColorSpace"
            },
            "tintTransform": {
              "$ref": "#/$defs/PdfFunction"
            }
          },
          "required": [
            "kind",
            "name",
            "alternate",
            "tintTransform"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "deviceN"
            },
            "names": {
              "type": "array",
              "items": {
                "type": "string"
              }
            },
            "alternate": {
              "$ref": "#/$defs/PdfColorSpace"
            },
            "tintTransform": {
              "$ref": "#/$defs/PdfFunction"
            },
            "attributes": {
              "anyOf": [
                {
                  "type": "array",
                  "items": {
                    "$ref": "#/$defs/PdfDictEntry"
                  }
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "required": [
            "kind",
            "names",
            "alternate",
            "tintTransform"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "pattern"
            },
            "base": {
              "anyOf": [
                {
                  "$ref": "#/$defs/PdfColorSpace"
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "named"
            },
            "name": {
              "type": "string"
            }
          },
          "required": [
            "kind",
            "name"
          ]
        }
      ]
    },
    "PdfDate": {
      "type": "object",
      "properties": {
        "year": {
          "type": "integer"
        },
        "month": {
          "type": "integer",
          "minimum": 0
        },
        "day": {
          "type": "integer",
          "minimum": 0
        },
        "hour": {
          "type": "integer",
          "minimum": 0
        },
        "minute": {
          "type": "integer",
          "minimum": 0
        },
        "second": {
          "type": "integer",
          "minimum": 0
        },
        "offsetMinutes": {
          "anyOf": [
            {
              "type": "integer"
            },
            {
              "type": "null"
            }
          ]
        }
      },
      "required": [
        "year"
      ]
    },
    "PdfDecimal": {
      "type": "object",
      "properties": {
        "negative": {
          "type": "boolean"
        },
        "coefficient": {
          "type": "string"
        },
        "scale": {
          "type": "integer",
          "minimum": 0
        }
      },
      "required": [
        "negative",
        "coefficient",
        "scale"
      ]
    },
    "PdfDestination": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "page"
            },
            "page": {
              "type": "integer",
              "minimum": 0
            },
            "fit": {
              "$ref": "#/$defs/PdfDestinationFit"
            }
          },
          "required": [
            "kind",
            "page",
            "fit"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "remotePage"
            },
            "page": {
              "type": "integer",
              "minimum": 0
            },
            "fit": {
              "$ref": "#/$defs/PdfDestinationFit"
            }
          },
          "required": [
            "kind",
            "page",
            "fit"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "named"
            },
            "name": {
              "type": "string"
            }
          },
          "required": [
            "kind",
            "name"
          ]
        }
      ]
    },
    "PdfDestinationFit": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "xyz"
            },
            "left": {
              "anyOf": [
                {
                  "type": "number"
                },
                {
                  "type": "null"
                }
              ]
            },
            "top": {
              "anyOf": [
                {
                  "type": "number"
                },
                {
                  "type": "null"
                }
              ]
            },
            "zoom": {
              "anyOf": [
                {
                  "type": "number"
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "fit"
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "fitHorizontal"
            },
            "top": {
              "anyOf": [
                {
                  "type": "number"
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "fitVertical"
            },
            "left": {
              "anyOf": [
                {
                  "type": "number"
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "fitRectangle"
            },
            "rect": {
              "type": "array",
              "items": {
                "type": "number"
              },
              "minItems": 4,
              "maxItems": 4
            }
          },
          "required": [
            "kind",
            "rect"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "fitBoundingBox"
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "fitBoundingBoxHorizontal"
            },
            "top": {
              "anyOf": [
                {
                  "type": "number"
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "fitBoundingBoxVertical"
            },
            "left": {
              "anyOf": [
                {
                  "type": "number"
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "required": [
            "kind"
          ]
        }
      ]
    },
    "PdfDictEntry": {
      "type": "object",
      "properties": {
        "key": {
          "type": "string"
        },
        "value": {
          "$ref": "#/$defs/PdfObject"
        }
      },
      "required": [
        "key",
        "value"
      ]
    },
    "PdfEmbeddedCMap": {
      "type": "object",
      "properties": {
        "name": {
          "type": "string"
        },
        "vertical": {
          "type": "boolean"
        },
        "codespace": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfCodespaceRange"
          }
        },
        "mappings": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfCidMapping"
          }
        },
        "useCmap": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        }
      },
      "required": [
        "name"
      ]
    },
    "PdfEmbeddedFile": {
      "type": "object",
      "properties": {
        "id": {
          "type": "string"
        },
        "fileName": {
          "type": "string"
        },
        "description": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "mimeType": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "data": {
          "type": "array",
          "items": {
            "type": "integer",
            "minimum": 0
          }
        },
        "creationDate": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfDate"
            },
            {
              "type": "null"
            }
          ]
        },
        "modificationDate": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfDate"
            },
            {
              "type": "null"
            }
          ]
        },
        "relationship": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "listed": {
          "type": "boolean"
        }
      },
      "required": [
        "id",
        "fileName",
        "data"
      ]
    },
    "PdfEncodingDifference": {
      "type": "object",
      "properties": {
        "code": {
          "type": "integer",
          "minimum": 0
        },
        "glyph": {
          "type": "string"
        }
      },
      "required": [
        "code",
        "glyph"
      ]
    },
    "PdfEncryption": {
      "type": "object",
      "properties": {
        "algorithm": {
          "$ref": "#/$defs/PdfEncryptionAlgorithm"
        },
        "permissions": {
          "type": "integer"
        },
        "userPassword": {
          "type": "string"
        },
        "ownerPassword": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "encryptMetadata": {
          "type": "boolean"
        }
      },
      "required": [
        "algorithm"
      ]
    },
    "PdfEncryptionAlgorithm": {
      "type": "string",
      "enum": [
        "rc4_40",
        "rc4_128",
        "aes128",
        "aes256"
      ]
    },
    "PdfExtGState": {
      "type": "object",
      "properties": {
        "id": {
          "type": "string"
        },
        "lineWidth": {
          "anyOf": [
            {
              "type": "number"
            },
            {
              "type": "null"
            }
          ]
        },
        "lineCap": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfLineCap"
            },
            {
              "type": "null"
            }
          ]
        },
        "lineJoin": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfLineJoin"
            },
            {
              "type": "null"
            }
          ]
        },
        "miterLimit": {
          "anyOf": [
            {
              "type": "number"
            },
            {
              "type": "null"
            }
          ]
        },
        "dash": {
          "anyOf": [
            {
              "type": "array",
              "items": [
                {
                  "type": "array",
                  "items": {
                    "type": "number"
                  }
                },
                {
                  "type": "number"
                }
              ],
              "minItems": 2,
              "maxItems": 2
            },
            {
              "type": "null"
            }
          ]
        },
        "renderingIntent": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "overprintStroke": {
          "anyOf": [
            {
              "type": "boolean"
            },
            {
              "type": "null"
            }
          ]
        },
        "overprintFill": {
          "anyOf": [
            {
              "type": "boolean"
            },
            {
              "type": "null"
            }
          ]
        },
        "overprintMode": {
          "anyOf": [
            {
              "type": "integer",
              "minimum": 0
            },
            {
              "type": "null"
            }
          ]
        },
        "font": {
          "anyOf": [
            {
              "type": "array",
              "items": [
                {
                  "type": "string"
                },
                {
                  "type": "number"
                }
              ],
              "minItems": 2,
              "maxItems": 2
            },
            {
              "type": "null"
            }
          ]
        },
        "blendMode": {
          "anyOf": [
            {
              "type": "array",
              "items": {
                "type": "string"
              }
            },
            {
              "type": "null"
            }
          ]
        },
        "softMask": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfSoftMask"
            },
            {
              "type": "null"
            }
          ]
        },
        "strokeAlpha": {
          "anyOf": [
            {
              "type": "number"
            },
            {
              "type": "null"
            }
          ]
        },
        "fillAlpha": {
          "anyOf": [
            {
              "type": "number"
            },
            {
              "type": "null"
            }
          ]
        },
        "alphaIsShape": {
          "anyOf": [
            {
              "type": "boolean"
            },
            {
              "type": "null"
            }
          ]
        },
        "strokeAdjust": {
          "anyOf": [
            {
              "type": "boolean"
            },
            {
              "type": "null"
            }
          ]
        },
        "flatness": {
          "anyOf": [
            {
              "type": "number"
            },
            {
              "type": "null"
            }
          ]
        },
        "smoothness": {
          "anyOf": [
            {
              "type": "number"
            },
            {
              "type": "null"
            }
          ]
        },
        "textKnockout": {
          "anyOf": [
            {
              "type": "boolean"
            },
            {
              "type": "null"
            }
          ]
        },
        "extra": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfDictEntry"
          }
        }
      },
      "required": [
        "id"
      ]
    },
    "PdfFileSpecification": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "path"
            },
            "path": {
              "type": "string"
            }
          },
          "required": [
            "kind",
            "path"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "embedded"
            },
            "file": {
              "type": "string"
            }
          },
          "required": [
            "kind",
            "file"
          ]
        }
      ]
    },
    "PdfFont": {
      "type": "object",
      "properties": {
        "id": {
          "type": "string"
        },
        "kind": {
          "$ref": "#/$defs/PdfFontKind"
        },
        "toUnicode": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfToUnicode"
            },
            {
              "type": "null"
            }
          ]
        },
        "extra": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfDictEntry"
          }
        }
      },
      "required": [
        "id",
        "kind"
      ]
    },
    "PdfFontDescriptor": {
      "type": "object",
      "properties": {
        "fontName": {
          "type": "string"
        },
        "flags": {
          "type": "integer",
          "minimum": 0
        },
        "fontBbox": {
          "type": "array",
          "items": {
            "type": "number"
          },
          "minItems": 4,
          "maxItems": 4
        },
        "italicAngle": {
          "type": "number"
        },
        "ascent": {
          "type": "number"
        },
        "descent": {
          "type": "number"
        },
        "capHeight": {
          "type": "number"
        },
        "stemV": {
          "type": "number"
        },
        "stemH": {
          "anyOf": [
            {
              "type": "number"
            },
            {
              "type": "null"
            }
          ]
        },
        "xHeight": {
          "anyOf": [
            {
              "type": "number"
            },
            {
              "type": "null"
            }
          ]
        },
        "leading": {
          "anyOf": [
            {
              "type": "number"
            },
            {
              "type": "null"
            }
          ]
        },
        "avgWidth": {
          "anyOf": [
            {
              "type": "number"
            },
            {
              "type": "null"
            }
          ]
        },
        "maxWidth": {
          "anyOf": [
            {
              "type": "number"
            },
            {
              "type": "null"
            }
          ]
        },
        "missingWidth": {
          "anyOf": [
            {
              "type": "number"
            },
            {
              "type": "null"
            }
          ]
        },
        "fontFamily": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "fontStretch": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "fontWeight": {
          "anyOf": [
            {
              "type": "number"
            },
            {
              "type": "null"
            }
          ]
        },
        "charSet": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "extra": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfDictEntry"
          }
        }
      },
      "required": [
        "fontName"
      ]
    },
    "PdfFontKind": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "type1"
            },
            "baseFont": {
              "type": "string"
            },
            "encoding": {
              "$ref": "#/$defs/PdfSimpleEncoding"
            },
            "firstChar": {
              "type": "integer",
              "minimum": 0
            },
            "widths": {
              "type": "array",
              "items": {
                "type": "number"
              }
            },
            "descriptor": {
              "anyOf": [
                {
                  "$ref": "#/$defs/PdfFontDescriptor"
                },
                {
                  "type": "null"
                }
              ]
            },
            "program": {
              "anyOf": [
                {
                  "$ref": "#/$defs/PdfFontProgram"
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "required": [
            "kind",
            "baseFont",
            "encoding",
            "firstChar",
            "widths"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "trueType"
            },
            "baseFont": {
              "type": "string"
            },
            "encoding": {
              "$ref": "#/$defs/PdfSimpleEncoding"
            },
            "firstChar": {
              "type": "integer",
              "minimum": 0
            },
            "widths": {
              "type": "array",
              "items": {
                "type": "number"
              }
            },
            "descriptor": {
              "anyOf": [
                {
                  "$ref": "#/$defs/PdfFontDescriptor"
                },
                {
                  "type": "null"
                }
              ]
            },
            "program": {
              "anyOf": [
                {
                  "$ref": "#/$defs/PdfFontProgram"
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "required": [
            "kind",
            "baseFont",
            "encoding",
            "firstChar",
            "widths"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "type3"
            },
            "fontMatrix": {
              "type": "array",
              "items": {
                "type": "number"
              },
              "minItems": 6,
              "maxItems": 6
            },
            "fontBbox": {
              "type": "array",
              "items": {
                "type": "number"
              },
              "minItems": 4,
              "maxItems": 4
            },
            "encoding": {
              "$ref": "#/$defs/PdfSimpleEncoding"
            },
            "firstChar": {
              "type": "integer",
              "minimum": 0
            },
            "widths": {
              "type": "array",
              "items": {
                "type": "number"
              }
            },
            "charProcs": {
              "type": "array",
              "items": {
                "$ref": "#/$defs/PdfCharProc"
              }
            },
            "descriptor": {
              "anyOf": [
                {
                  "$ref": "#/$defs/PdfFontDescriptor"
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "required": [
            "kind",
            "fontMatrix",
            "fontBbox",
            "encoding",
            "firstChar",
            "widths",
            "charProcs"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "type0"
            },
            "baseFont": {
              "type": "string"
            },
            "cmap": {
              "$ref": "#/$defs/PdfCMap"
            },
            "descendant": {
              "$ref": "#/$defs/PdfCidFont"
            }
          },
          "required": [
            "kind",
            "baseFont",
            "cmap",
            "descendant"
          ]
        }
      ]
    },
    "PdfFontProgram": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "type1"
            },
            "data": {
              "type": "array",
              "items": {
                "type": "integer",
                "minimum": 0
              }
            },
            "length1": {
              "type": "integer",
              "minimum": 0
            },
            "length2": {
              "type": "integer",
              "minimum": 0
            },
            "length3": {
              "type": "integer",
              "minimum": 0
            }
          },
          "required": [
            "kind",
            "data",
            "length1",
            "length2",
            "length3"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "trueType"
            },
            "data": {
              "type": "array",
              "items": {
                "type": "integer",
                "minimum": 0
              }
            }
          },
          "required": [
            "kind",
            "data"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "cff"
            },
            "data": {
              "type": "array",
              "items": {
                "type": "integer",
                "minimum": 0
              }
            }
          },
          "required": [
            "kind",
            "data"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "cidCff"
            },
            "data": {
              "type": "array",
              "items": {
                "type": "integer",
                "minimum": 0
              }
            }
          },
          "required": [
            "kind",
            "data"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "openType"
            },
            "data": {
              "type": "array",
              "items": {
                "type": "integer",
                "minimum": 0
              }
            }
          },
          "required": [
            "kind",
            "data"
          ]
        }
      ]
    },
    "PdfFormField": {
      "type": "object",
      "properties": {
        "name": {
          "type": "string"
        },
        "kind": {
          "$ref": "#/$defs/PdfFormFieldKind"
        },
        "flags": {
          "type": "integer",
          "minimum": 0
        },
        "alternateName": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "mappingName": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "defaultAppearance": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "quadding": {
          "anyOf": [
            {
              "type": "integer",
              "minimum": 0
            },
            {
              "type": "null"
            }
          ]
        },
        "widgets": {
          "type": "array",
          "items": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 0
            },
            "minItems": 2,
            "maxItems": 2
          }
        },
        "children": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfFormField"
          }
        },
        "additionalActions": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfDictEntry"
          }
        },
        "extra": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfDictEntry"
          }
        }
      },
      "required": [
        "name",
        "kind"
      ]
    },
    "PdfFormFieldKind": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "button"
            },
            "value": {
              "anyOf": [
                {
                  "type": "string"
                },
                {
                  "type": "null"
                }
              ]
            },
            "defaultValue": {
              "anyOf": [
                {
                  "type": "string"
                },
                {
                  "type": "null"
                }
              ]
            },
            "options": {
              "type": "array",
              "items": {
                "type": "string"
              }
            }
          },
          "required": [
            "kind",
            "options"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "text"
            },
            "value": {
              "anyOf": [
                {
                  "type": "string"
                },
                {
                  "type": "null"
                }
              ]
            },
            "defaultValue": {
              "anyOf": [
                {
                  "type": "string"
                },
                {
                  "type": "null"
                }
              ]
            },
            "maxLength": {
              "anyOf": [
                {
                  "type": "integer",
                  "minimum": 0
                },
                {
                  "type": "null"
                }
              ]
            },
            "richValue": {
              "anyOf": [
                {
                  "type": "string"
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "choice"
            },
            "values": {
              "type": "array",
              "items": {
                "type": "string"
              }
            },
            "defaultValues": {
              "type": "array",
              "items": {
                "type": "string"
              }
            },
            "options": {
              "type": "array",
              "items": {
                "type": "array",
                "items": [
                  {
                    "type": "string"
                  },
                  {
                    "type": "string"
                  }
                ],
                "minItems": 2,
                "maxItems": 2
              }
            },
            "topIndex": {
              "anyOf": [
                {
                  "type": "integer",
                  "minimum": 0
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "required": [
            "kind",
            "values",
            "defaultValues",
            "options"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "signature"
            },
            "value": {
              "anyOf": [
                {
                  "type": "array",
                  "items": {
                    "$ref": "#/$defs/PdfDictEntry"
                  }
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "container"
            }
          },
          "required": [
            "kind"
          ]
        }
      ]
    },
    "PdfFormXObject": {
      "type": "object",
      "properties": {
        "id": {
          "type": "string"
        },
        "bbox": {
          "type": "array",
          "items": {
            "type": "number"
          },
          "minItems": 4,
          "maxItems": 4
        },
        "matrix": {
          "type": "array",
          "items": {
            "type": "number"
          },
          "minItems": 6,
          "maxItems": 6
        },
        "content": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfOp"
          }
        },
        "group": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfTransparencyGroup"
            },
            {
              "type": "null"
            }
          ]
        },
        "optionalContent": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "structParent": {
          "anyOf": [
            {
              "type": "integer",
              "minimum": 0
            },
            {
              "type": "null"
            }
          ]
        },
        "extra": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfDictEntry"
          }
        }
      },
      "required": [
        "id",
        "bbox"
      ]
    },
    "PdfFunction": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "sampled"
            },
            "domain": {
              "type": "array",
              "items": {
                "type": "number"
              }
            },
            "range": {
              "type": "array",
              "items": {
                "type": "number"
              }
            },
            "size": {
              "type": "array",
              "items": {
                "type": "integer",
                "minimum": 0
              }
            },
            "bitsPerSample": {
              "type": "integer",
              "minimum": 0
            },
            "order": {
              "anyOf": [
                {
                  "type": "integer",
                  "minimum": 0
                },
                {
                  "type": "null"
                }
              ]
            },
            "encode": {
              "anyOf": [
                {
                  "type": "array",
                  "items": {
                    "type": "number"
                  }
                },
                {
                  "type": "null"
                }
              ]
            },
            "decode": {
              "anyOf": [
                {
                  "type": "array",
                  "items": {
                    "type": "number"
                  }
                },
                {
                  "type": "null"
                }
              ]
            },
            "samples": {
              "type": "array",
              "items": {
                "type": "integer",
                "minimum": 0
              }
            }
          },
          "required": [
            "kind",
            "domain",
            "range",
            "size",
            "bitsPerSample",
            "samples"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "exponential"
            },
            "domain": {
              "type": "array",
              "items": {
                "type": "number"
              }
            },
            "range": {
              "anyOf": [
                {
                  "type": "array",
                  "items": {
                    "type": "number"
                  }
                },
                {
                  "type": "null"
                }
              ]
            },
            "c0": {
              "type": "array",
              "items": {
                "type": "number"
              }
            },
            "c1": {
              "type": "array",
              "items": {
                "type": "number"
              }
            },
            "n": {
              "type": "number"
            }
          },
          "required": [
            "kind",
            "domain",
            "c0",
            "c1",
            "n"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "stitching"
            },
            "domain": {
              "type": "array",
              "items": {
                "type": "number"
              }
            },
            "range": {
              "anyOf": [
                {
                  "type": "array",
                  "items": {
                    "type": "number"
                  }
                },
                {
                  "type": "null"
                }
              ]
            },
            "functions": {
              "type": "array",
              "items": {
                "$ref": "#/$defs/PdfFunction"
              }
            },
            "bounds": {
              "type": "array",
              "items": {
                "type": "number"
              }
            },
            "encode": {
              "type": "array",
              "items": {
                "type": "number"
              }
            }
          },
          "required": [
            "kind",
            "domain",
            "functions",
            "bounds",
            "encode"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "postScript"
            },
            "domain": {
              "type": "array",
              "items": {
                "type": "number"
              }
            },
            "range": {
              "type": "array",
              "items": {
                "type": "number"
              }
            },
            "code": {
              "type": "string"
            }
          },
          "required": [
            "kind",
            "domain",
            "range",
            "code"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "array"
            },
            "functions": {
              "type": "array",
              "items": {
                "$ref": "#/$defs/PdfFunction"
              }
            }
          },
          "required": [
            "kind",
            "functions"
          ]
        }
      ]
    },
    "PdfImage": {
      "type": "object",
      "properties": {
        "id": {
          "type": "string"
        },
        "width": {
          "type": "integer",
          "minimum": 0
        },
        "height": {
          "type": "integer",
          "minimum": 0
        },
        "colorSpace": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfColorSpace"
            },
            {
              "type": "null"
            }
          ]
        },
        "bitsPerComponent": {
          "type": "integer",
          "minimum": 0
        },
        "imageMask": {
          "type": "boolean"
        },
        "decode": {
          "type": "array",
          "items": {
            "type": "number"
          }
        },
        "interpolate": {
          "type": "boolean"
        },
        "codec": {
          "$ref": "#/$defs/PdfImageCodec"
        },
        "data": {
          "type": "array",
          "items": {
            "type": "integer",
            "minimum": 0
          }
        },
        "softMask": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "softMaskInData": {
          "anyOf": [
            {
              "type": "integer",
              "minimum": 0
            },
            {
              "type": "null"
            }
          ]
        },
        "mask": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfImageMask"
            },
            {
              "type": "null"
            }
          ]
        },
        "matte": {
          "anyOf": [
            {
              "type": "array",
              "items": {
                "type": "number"
              }
            },
            {
              "type": "null"
            }
          ]
        },
        "intent": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "optionalContent": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "structParent": {
          "anyOf": [
            {
              "type": "integer",
              "minimum": 0
            },
            {
              "type": "null"
            }
          ]
        },
        "extra": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfDictEntry"
          }
        }
      },
      "required": [
        "id",
        "width",
        "height",
        "data"
      ]
    },
    "PdfImageCodec": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "raw"
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "dct"
            },
            "colorTransform": {
              "anyOf": [
                {
                  "type": "integer",
                  "minimum": 0
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "jpx"
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "ccitt"
            },
            "parameters": {
              "$ref": "#/$defs/PdfCcittParameters"
            }
          },
          "required": [
            "kind",
            "parameters"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "jbig2"
            },
            "globals": {
              "anyOf": [
                {
                  "type": "array",
                  "items": {
                    "type": "integer",
                    "minimum": 0
                  }
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "required": [
            "kind"
          ]
        }
      ]
    },
    "PdfImageMask": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "stencil"
            },
            "image": {
              "type": "string"
            }
          },
          "required": [
            "kind",
            "image"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "colorKey"
            },
            "ranges": {
              "type": "array",
              "items": {
                "type": "integer",
                "minimum": 0
              }
            }
          },
          "required": [
            "kind",
            "ranges"
          ]
        }
      ]
    },
    "PdfIndirectObject": {
      "type": "object",
      "properties": {
        "id": {
          "$ref": "#/$defs/ObjRef"
        },
        "value": {
          "$ref": "#/$defs/PdfObject"
        }
      },
      "required": [
        "id",
        "value"
      ]
    },
    "PdfInfo": {
      "type": "object",
      "properties": {
        "title": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "author": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "subject": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "keywords": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "creator": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "producer": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "creationDate": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfDate"
            },
            {
              "type": "null"
            }
          ]
        },
        "modificationDate": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfDate"
            },
            {
              "type": "null"
            }
          ]
        },
        "trapped": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "extra": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfDictEntry"
          }
        }
      }
    },
    "PdfInlineImage": {
      "type": "object",
      "properties": {
        "width": {
          "type": "integer",
          "minimum": 0
        },
        "height": {
          "type": "integer",
          "minimum": 0
        },
        "bitsPerComponent": {
          "type": "integer",
          "minimum": 0
        },
        "colorSpace": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfColorSpace"
            },
            {
              "type": "null"
            }
          ]
        },
        "imageMask": {
          "type": "boolean"
        },
        "decode": {
          "type": "array",
          "items": {
            "type": "number"
          }
        },
        "interpolate": {
          "type": "boolean"
        },
        "filters": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfStreamFilter"
          }
        },
        "data": {
          "type": "array",
          "items": {
            "type": "integer",
            "minimum": 0
          }
        },
        "extra": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfDictEntry"
          }
        }
      },
      "required": [
        "width",
        "height",
        "data"
      ]
    },
    "PdfLineCap": {
      "type": "string",
      "enum": [
        "butt",
        "round",
        "square"
      ]
    },
    "PdfLineJoin": {
      "type": "string",
      "enum": [
        "miter",
        "round",
        "bevel"
      ]
    },
    "PdfMarkInfo": {
      "type": "object",
      "properties": {
        "marked": {
          "type": "boolean"
        },
        "userProperties": {
          "type": "boolean"
        },
        "suspects": {
          "type": "boolean"
        }
      }
    },
    "PdfMarkupAnnotation": {
      "type": "object",
      "properties": {
        "title": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "popup": {
          "anyOf": [
            {
              "type": "integer",
              "minimum": 0
            },
            {
              "type": "null"
            }
          ]
        },
        "opacity": {
          "anyOf": [
            {
              "type": "number"
            },
            {
              "type": "null"
            }
          ]
        },
        "richContents": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "creationDate": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfDate"
            },
            {
              "type": "null"
            }
          ]
        },
        "inReplyTo": {
          "anyOf": [
            {
              "type": "integer",
              "minimum": 0
            },
            {
              "type": "null"
            }
          ]
        },
        "subject": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "replyType": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "intent": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        }
      }
    },
    "PdfNamedColorSpace": {
      "type": "object",
      "properties": {
        "name": {
          "type": "string"
        },
        "colorSpace": {
          "$ref": "#/$defs/PdfColorSpace"
        }
      },
      "required": [
        "name",
        "colorSpace"
      ]
    },
    "PdfNamedDestination": {
      "type": "object",
      "properties": {
        "name": {
          "type": "string"
        },
        "destination": {
          "$ref": "#/$defs/PdfDestination"
        }
      },
      "required": [
        "name",
        "destination"
      ]
    },
    "PdfNamedProperties": {
      "type": "object",
      "properties": {
        "name": {
          "type": "string"
        },
        "entries": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfDictEntry"
          }
        }
      },
      "required": [
        "name",
        "entries"
      ]
    },
    "PdfObject": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "null"
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "bool"
            },
            "value": {
              "type": "boolean"
            }
          },
          "required": [
            "kind",
            "value"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "int"
            },
            "value": {
              "type": "integer"
            }
          },
          "required": [
            "kind",
            "value"
          ]
        },
        {
          "allOf": [
            {
              "$ref": "#/$defs/PdfDecimal"
            },
            {
              "type": "object",
              "properties": {
                "kind": {
                  "const": "real"
                }
              },
              "required": [
                "kind"
              ]
            }
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "str"
            },
            "value": {
              "type": "array",
              "items": {
                "type": "integer",
                "minimum": 0
              }
            }
          },
          "required": [
            "kind",
            "value"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "name"
            },
            "value": {
              "type": "string"
            }
          },
          "required": [
            "kind",
            "value"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "array"
            },
            "value": {
              "type": "array",
              "items": {
                "$ref": "#/$defs/PdfObject"
              }
            }
          },
          "required": [
            "kind",
            "value"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "dict"
            },
            "value": {
              "type": "array",
              "items": {
                "$ref": "#/$defs/PdfDictEntry"
              }
            }
          },
          "required": [
            "kind",
            "value"
          ]
        },
        {
          "allOf": [
            {
              "$ref": "#/$defs/ObjRef"
            },
            {
              "type": "object",
              "properties": {
                "kind": {
                  "const": "ref"
                }
              },
              "required": [
                "kind"
              ]
            }
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "stream"
            },
            "dict": {
              "type": "array",
              "items": {
                "$ref": "#/$defs/PdfDictEntry"
              }
            },
            "data": {
              "type": "array",
              "items": {
                "type": "integer",
                "minimum": 0
              }
            },
            "filters": {
              "type": "array",
              "items": {
                "$ref": "#/$defs/PdfStreamFilter"
              }
            }
          },
          "required": [
            "kind",
            "dict",
            "data",
            "filters"
          ]
        }
      ]
    },
    "PdfOp": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "setLineWidth"
            },
            "width": {
              "type": "number"
            }
          },
          "required": [
            "op",
            "width"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "setLineCap"
            },
            "cap": {
              "$ref": "#/$defs/PdfLineCap"
            }
          },
          "required": [
            "op",
            "cap"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "setLineJoin"
            },
            "join": {
              "$ref": "#/$defs/PdfLineJoin"
            }
          },
          "required": [
            "op",
            "join"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "setMiterLimit"
            },
            "limit": {
              "type": "number"
            }
          },
          "required": [
            "op",
            "limit"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "setDash"
            },
            "array": {
              "type": "array",
              "items": {
                "type": "number"
              }
            },
            "phase": {
              "type": "number"
            }
          },
          "required": [
            "op",
            "array",
            "phase"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "setRenderingIntent"
            },
            "intent": {
              "type": "string"
            }
          },
          "required": [
            "op",
            "intent"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "setFlatness"
            },
            "flatness": {
              "type": "number"
            }
          },
          "required": [
            "op",
            "flatness"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "setExtGState"
            },
            "name": {
              "type": "string"
            }
          },
          "required": [
            "op",
            "name"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "save"
            }
          },
          "required": [
            "op"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "restore"
            }
          },
          "required": [
            "op"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "transform"
            },
            "matrix": {
              "type": "array",
              "items": {
                "type": "number"
              },
              "minItems": 6,
              "maxItems": 6
            }
          },
          "required": [
            "op",
            "matrix"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "moveTo"
            },
            "x": {
              "type": "number"
            },
            "y": {
              "type": "number"
            }
          },
          "required": [
            "op",
            "x",
            "y"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "lineTo"
            },
            "x": {
              "type": "number"
            },
            "y": {
              "type": "number"
            }
          },
          "required": [
            "op",
            "x",
            "y"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "curveTo"
            },
            "x1": {
              "type": "number"
            },
            "y1": {
              "type": "number"
            },
            "x2": {
              "type": "number"
            },
            "y2": {
              "type": "number"
            },
            "x3": {
              "type": "number"
            },
            "y3": {
              "type": "number"
            }
          },
          "required": [
            "op",
            "x1",
            "y1",
            "x2",
            "y2",
            "x3",
            "y3"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "curveToInitial"
            },
            "x2": {
              "type": "number"
            },
            "y2": {
              "type": "number"
            },
            "x3": {
              "type": "number"
            },
            "y3": {
              "type": "number"
            }
          },
          "required": [
            "op",
            "x2",
            "y2",
            "x3",
            "y3"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "curveToFinal"
            },
            "x1": {
              "type": "number"
            },
            "y1": {
              "type": "number"
            },
            "x3": {
              "type": "number"
            },
            "y3": {
              "type": "number"
            }
          },
          "required": [
            "op",
            "x1",
            "y1",
            "x3",
            "y3"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "closePath"
            }
          },
          "required": [
            "op"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "rectangle"
            },
            "x": {
              "type": "number"
            },
            "y": {
              "type": "number"
            },
            "width": {
              "type": "number"
            },
            "height": {
              "type": "number"
            }
          },
          "required": [
            "op",
            "x",
            "y",
            "width",
            "height"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "stroke"
            }
          },
          "required": [
            "op"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "closeStroke"
            }
          },
          "required": [
            "op"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "fill"
            }
          },
          "required": [
            "op"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "fillEvenOdd"
            }
          },
          "required": [
            "op"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "fillStroke"
            }
          },
          "required": [
            "op"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "fillStrokeEvenOdd"
            }
          },
          "required": [
            "op"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "closeFillStroke"
            }
          },
          "required": [
            "op"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "closeFillStrokeEvenOdd"
            }
          },
          "required": [
            "op"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "endPath"
            }
          },
          "required": [
            "op"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "clip"
            }
          },
          "required": [
            "op"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "clipEvenOdd"
            }
          },
          "required": [
            "op"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "beginText"
            }
          },
          "required": [
            "op"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "endText"
            }
          },
          "required": [
            "op"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "setCharSpacing"
            },
            "spacing": {
              "type": "number"
            }
          },
          "required": [
            "op",
            "spacing"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "setWordSpacing"
            },
            "spacing": {
              "type": "number"
            }
          },
          "required": [
            "op",
            "spacing"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "setHorizontalScale"
            },
            "scale": {
              "type": "number"
            }
          },
          "required": [
            "op",
            "scale"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "setLeading"
            },
            "leading": {
              "type": "number"
            }
          },
          "required": [
            "op",
            "leading"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "setFont"
            },
            "name": {
              "type": "string"
            },
            "size": {
              "type": "number"
            }
          },
          "required": [
            "op",
            "name",
            "size"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "setTextRenderingMode"
            },
            "mode": {
              "type": "integer",
              "minimum": 0
            }
          },
          "required": [
            "op",
            "mode"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "setTextRise"
            },
            "rise": {
              "type": "number"
            }
          },
          "required": [
            "op",
            "rise"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "moveText"
            },
            "tx": {
              "type": "number"
            },
            "ty": {
              "type": "number"
            }
          },
          "required": [
            "op",
            "tx",
            "ty"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "moveTextSetLeading"
            },
            "tx": {
              "type": "number"
            },
            "ty": {
              "type": "number"
            }
          },
          "required": [
            "op",
            "tx",
            "ty"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "setTextMatrix"
            },
            "matrix": {
              "type": "array",
              "items": {
                "type": "number"
              },
              "minItems": 6,
              "maxItems": 6
            }
          },
          "required": [
            "op",
            "matrix"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "nextLine"
            }
          },
          "required": [
            "op"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "showText"
            },
            "text": {
              "$ref": "#/$defs/PdfTextString"
            }
          },
          "required": [
            "op",
            "text"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "showTextArray"
            },
            "items": {
              "type": "array",
              "items": {
                "$ref": "#/$defs/PdfTextArrayItem"
              }
            }
          },
          "required": [
            "op",
            "items"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "nextLineShowText"
            },
            "text": {
              "$ref": "#/$defs/PdfTextString"
            }
          },
          "required": [
            "op",
            "text"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "nextLineShowTextSpaced"
            },
            "wordSpacing": {
              "type": "number"
            },
            "charSpacing": {
              "type": "number"
            },
            "text": {
              "$ref": "#/$defs/PdfTextString"
            }
          },
          "required": [
            "op",
            "wordSpacing",
            "charSpacing",
            "text"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "setGlyphWidth"
            },
            "wx": {
              "type": "number"
            },
            "wy": {
              "type": "number"
            }
          },
          "required": [
            "op",
            "wx",
            "wy"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "setGlyphWidthAndBox"
            },
            "wx": {
              "type": "number"
            },
            "wy": {
              "type": "number"
            },
            "llx": {
              "type": "number"
            },
            "lly": {
              "type": "number"
            },
            "urx": {
              "type": "number"
            },
            "ury": {
              "type": "number"
            }
          },
          "required": [
            "op",
            "wx",
            "wy",
            "llx",
            "lly",
            "urx",
            "ury"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "setStrokeColorSpace"
            },
            "name": {
              "type": "string"
            }
          },
          "required": [
            "op",
            "name"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "setFillColorSpace"
            },
            "name": {
              "type": "string"
            }
          },
          "required": [
            "op",
            "name"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "setStrokeColor"
            },
            "components": {
              "type": "array",
              "items": {
                "type": "number"
              }
            }
          },
          "required": [
            "op",
            "components"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "setStrokeColorN"
            },
            "components": {
              "type": "array",
              "items": {
                "type": "number"
              }
            },
            "pattern": {
              "anyOf": [
                {
                  "type": "string"
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "required": [
            "op",
            "components"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "setFillColor"
            },
            "components": {
              "type": "array",
              "items": {
                "type": "number"
              }
            }
          },
          "required": [
            "op",
            "components"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "setFillColorN"
            },
            "components": {
              "type": "array",
              "items": {
                "type": "number"
              }
            },
            "pattern": {
              "anyOf": [
                {
                  "type": "string"
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "required": [
            "op",
            "components"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "setStrokeGray"
            },
            "gray": {
              "type": "number"
            }
          },
          "required": [
            "op",
            "gray"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "setFillGray"
            },
            "gray": {
              "type": "number"
            }
          },
          "required": [
            "op",
            "gray"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "setStrokeRgb"
            },
            "r": {
              "type": "number"
            },
            "g": {
              "type": "number"
            },
            "b": {
              "type": "number"
            }
          },
          "required": [
            "op",
            "r",
            "g",
            "b"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "setFillRgb"
            },
            "r": {
              "type": "number"
            },
            "g": {
              "type": "number"
            },
            "b": {
              "type": "number"
            }
          },
          "required": [
            "op",
            "r",
            "g",
            "b"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "setStrokeCmyk"
            },
            "c": {
              "type": "number"
            },
            "m": {
              "type": "number"
            },
            "y": {
              "type": "number"
            },
            "k": {
              "type": "number"
            }
          },
          "required": [
            "op",
            "c",
            "m",
            "y",
            "k"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "setFillCmyk"
            },
            "c": {
              "type": "number"
            },
            "m": {
              "type": "number"
            },
            "y": {
              "type": "number"
            },
            "k": {
              "type": "number"
            }
          },
          "required": [
            "op",
            "c",
            "m",
            "y",
            "k"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "paintShading"
            },
            "name": {
              "type": "string"
            }
          },
          "required": [
            "op",
            "name"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "paintXObject"
            },
            "name": {
              "type": "string"
            }
          },
          "required": [
            "op",
            "name"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "inlineImage"
            },
            "image": {
              "$ref": "#/$defs/PdfInlineImage"
            }
          },
          "required": [
            "op",
            "image"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "markedContentPoint"
            },
            "tag": {
              "type": "string"
            }
          },
          "required": [
            "op",
            "tag"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "markedContentPointWithProperties"
            },
            "tag": {
              "type": "string"
            },
            "properties": {
              "$ref": "#/$defs/PdfPropertyList"
            }
          },
          "required": [
            "op",
            "tag",
            "properties"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "beginMarkedContent"
            },
            "tag": {
              "type": "string"
            }
          },
          "required": [
            "op",
            "tag"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "beginMarkedContentWithProperties"
            },
            "tag": {
              "type": "string"
            },
            "properties": {
              "$ref": "#/$defs/PdfPropertyList"
            }
          },
          "required": [
            "op",
            "tag",
            "properties"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "endMarkedContent"
            }
          },
          "required": [
            "op"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "beginCompatibility"
            }
          },
          "required": [
            "op"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "endCompatibility"
            }
          },
          "required": [
            "op"
          ]
        },
        {
          "type": "object",
          "properties": {
            "op": {
              "const": "unknown"
            },
            "operator": {
              "type": "string"
            },
            "operands": {
              "type": "array",
              "items": {
                "$ref": "#/$defs/PdfObject"
              }
            }
          },
          "required": [
            "op",
            "operator",
            "operands"
          ]
        }
      ]
    },
    "PdfOpenAction": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "destination"
            },
            "destination": {
              "$ref": "#/$defs/PdfDestination"
            }
          },
          "required": [
            "kind",
            "destination"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "action"
            },
            "action": {
              "$ref": "#/$defs/PdfAction"
            }
          },
          "required": [
            "kind",
            "action"
          ]
        }
      ]
    },
    "PdfOptionalContent": {
      "type": "object",
      "properties": {
        "groups": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfOptionalContentGroup"
          }
        },
        "name": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "baseStateOff": {
          "type": "boolean"
        },
        "on": {
          "type": "array",
          "items": {
            "type": "string"
          }
        },
        "off": {
          "type": "array",
          "items": {
            "type": "string"
          }
        },
        "order": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfObject"
          }
        },
        "extra": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfDictEntry"
          }
        }
      }
    },
    "PdfOptionalContentGroup": {
      "type": "object",
      "properties": {
        "id": {
          "type": "string"
        },
        "name": {
          "type": "string"
        },
        "intent": {
          "type": "array",
          "items": {
            "type": "string"
          }
        },
        "usage": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfDictEntry"
          }
        }
      },
      "required": [
        "id",
        "name"
      ]
    },
    "PdfOutlineItem": {
      "type": "object",
      "properties": {
        "title": {
          "type": "string"
        },
        "destination": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfDestination"
            },
            {
              "type": "null"
            }
          ]
        },
        "action": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfAction"
            },
            {
              "type": "null"
            }
          ]
        },
        "color": {
          "anyOf": [
            {
              "type": "array",
              "items": {
                "type": "number"
              },
              "minItems": 3,
              "maxItems": 3
            },
            {
              "type": "null"
            }
          ]
        },
        "italic": {
          "type": "boolean"
        },
        "bold": {
          "type": "boolean"
        },
        "open": {
          "type": "boolean"
        },
        "children": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfOutlineItem"
          }
        },
        "extra": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfDictEntry"
          }
        }
      },
      "required": [
        "title"
      ]
    },
    "PdfOutputIntent": {
      "type": "object",
      "properties": {
        "subtype": {
          "type": "string"
        },
        "conditionIdentifier": {
          "type": "string"
        },
        "condition": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "registryName": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "info": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "profile": {
          "anyOf": [
            {
              "type": "array",
              "items": {
                "type": "integer",
                "minimum": 0
              }
            },
            {
              "type": "null"
            }
          ]
        }
      },
      "required": [
        "subtype",
        "conditionIdentifier"
      ]
    },
    "PdfPage": {
      "type": "object",
      "properties": {
        "mediaBox": {
          "type": "array",
          "items": {
            "type": "number"
          },
          "minItems": 4,
          "maxItems": 4
        },
        "cropBox": {
          "anyOf": [
            {
              "type": "array",
              "items": {
                "type": "number"
              },
              "minItems": 4,
              "maxItems": 4
            },
            {
              "type": "null"
            }
          ]
        },
        "bleedBox": {
          "anyOf": [
            {
              "type": "array",
              "items": {
                "type": "number"
              },
              "minItems": 4,
              "maxItems": 4
            },
            {
              "type": "null"
            }
          ]
        },
        "trimBox": {
          "anyOf": [
            {
              "type": "array",
              "items": {
                "type": "number"
              },
              "minItems": 4,
              "maxItems": 4
            },
            {
              "type": "null"
            }
          ]
        },
        "artBox": {
          "anyOf": [
            {
              "type": "array",
              "items": {
                "type": "number"
              },
              "minItems": 4,
              "maxItems": 4
            },
            {
              "type": "null"
            }
          ]
        },
        "rotate": {
          "type": "integer"
        },
        "userUnit": {
          "anyOf": [
            {
              "type": "number"
            },
            {
              "type": "null"
            }
          ]
        },
        "content": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfOp"
          }
        },
        "annotations": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfAnnotation"
          }
        },
        "group": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfTransparencyGroup"
            },
            {
              "type": "null"
            }
          ]
        },
        "thumbnail": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "structParents": {
          "anyOf": [
            {
              "type": "integer",
              "minimum": 0
            },
            {
              "type": "null"
            }
          ]
        },
        "transition": {
          "anyOf": [
            {
              "type": "array",
              "items": {
                "$ref": "#/$defs/PdfDictEntry"
              }
            },
            {
              "type": "null"
            }
          ]
        },
        "duration": {
          "anyOf": [
            {
              "type": "number"
            },
            {
              "type": "null"
            }
          ]
        },
        "metadata": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "additionalActions": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfDictEntry"
          }
        },
        "extra": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfDictEntry"
          }
        }
      },
      "required": [
        "mediaBox"
      ]
    },
    "PdfPageLabelRange": {
      "type": "object",
      "properties": {
        "startIndex": {
          "type": "integer",
          "minimum": 0
        },
        "style": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfPageLabelStyle"
            },
            {
              "type": "null"
            }
          ]
        },
        "prefix": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "start": {
          "type": "integer",
          "minimum": 0
        }
      },
      "required": [
        "startIndex"
      ]
    },
    "PdfPageLabelStyle": {
      "type": "string",
      "enum": [
        "decimal",
        "romanUpper",
        "romanLower",
        "lettersUpper",
        "lettersLower"
      ]
    },
    "PdfPageLayout": {
      "type": "string",
      "enum": [
        "singlePage",
        "oneColumn",
        "twoColumnLeft",
        "twoColumnRight",
        "twoPageLeft",
        "twoPageRight"
      ]
    },
    "PdfPageMode": {
      "type": "string",
      "enum": [
        "useNone",
        "useOutlines",
        "useThumbs",
        "fullScreen",
        "useOc",
        "useAttachments"
      ]
    },
    "PdfPattern": {
      "type": "object",
      "properties": {
        "id": {
          "type": "string"
        },
        "matrix": {
          "type": "array",
          "items": {
            "type": "number"
          },
          "minItems": 6,
          "maxItems": 6
        },
        "kind": {
          "$ref": "#/$defs/PdfPatternKind"
        },
        "extra": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfDictEntry"
          }
        }
      },
      "required": [
        "id",
        "kind"
      ]
    },
    "PdfPatternKind": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "tiling"
            },
            "paintType": {
              "type": "integer",
              "minimum": 0
            },
            "tilingType": {
              "type": "integer",
              "minimum": 0
            },
            "bbox": {
              "type": "array",
              "items": {
                "type": "number"
              },
              "minItems": 4,
              "maxItems": 4
            },
            "xStep": {
              "type": "number"
            },
            "yStep": {
              "type": "number"
            },
            "content": {
              "type": "array",
              "items": {
                "$ref": "#/$defs/PdfOp"
              }
            }
          },
          "required": [
            "kind",
            "paintType",
            "tilingType",
            "bbox",
            "xStep",
            "yStep",
            "content"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "shading"
            },
            "shading": {
              "type": "string"
            },
            "extGState": {
              "anyOf": [
                {
                  "type": "string"
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "required": [
            "kind",
            "shading"
          ]
        }
      ]
    },
    "PdfPredictor": {
      "type": "object",
      "properties": {
        "predictor": {
          "type": "integer",
          "minimum": 0
        },
        "colors": {
          "type": "integer",
          "minimum": 0
        },
        "bitsPerComponent": {
          "type": "integer",
          "minimum": 0
        },
        "columns": {
          "type": "integer",
          "minimum": 0
        }
      },
      "required": [
        "predictor",
        "colors",
        "bitsPerComponent",
        "columns"
      ]
    },
    "PdfPropertyList": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "named"
            },
            "name": {
              "type": "string"
            }
          },
          "required": [
            "kind",
            "name"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "inline"
            },
            "entries": {
              "type": "array",
              "items": {
                "$ref": "#/$defs/PdfDictEntry"
              }
            }
          },
          "required": [
            "kind",
            "entries"
          ]
        }
      ]
    },
    "PdfShading": {
      "type": "object",
      "properties": {
        "id": {
          "type": "string"
        },
        "colorSpace": {
          "$ref": "#/$defs/PdfColorSpace"
        },
        "kind": {
          "$ref": "#/$defs/PdfShadingKind"
        },
        "background": {
          "anyOf": [
            {
              "type": "array",
              "items": {
                "type": "number"
              }
            },
            {
              "type": "null"
            }
          ]
        },
        "bbox": {
          "anyOf": [
            {
              "type": "array",
              "items": {
                "type": "number"
              },
              "minItems": 4,
              "maxItems": 4
            },
            {
              "type": "null"
            }
          ]
        },
        "antiAlias": {
          "type": "boolean"
        },
        "extra": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfDictEntry"
          }
        }
      },
      "required": [
        "id",
        "colorSpace",
        "kind"
      ]
    },
    "PdfShadingKind": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "functionBased"
            },
            "domain": {
              "anyOf": [
                {
                  "type": "array",
                  "items": {
                    "type": "number"
                  },
                  "minItems": 4,
                  "maxItems": 4
                },
                {
                  "type": "null"
                }
              ]
            },
            "matrix": {
              "anyOf": [
                {
                  "type": "array",
                  "items": {
                    "type": "number"
                  },
                  "minItems": 6,
                  "maxItems": 6
                },
                {
                  "type": "null"
                }
              ]
            },
            "function": {
              "$ref": "#/$defs/PdfFunction"
            }
          },
          "required": [
            "kind",
            "function"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "axial"
            },
            "coords": {
              "type": "array",
              "items": {
                "type": "number"
              },
              "minItems": 4,
              "maxItems": 4
            },
            "domain": {
              "anyOf": [
                {
                  "type": "array",
                  "items": {
                    "type": "number"
                  },
                  "minItems": 2,
                  "maxItems": 2
                },
                {
                  "type": "null"
                }
              ]
            },
            "function": {
              "$ref": "#/$defs/PdfFunction"
            },
            "extend": {
              "type": "array",
              "items": {
                "type": "boolean"
              },
              "minItems": 2,
              "maxItems": 2
            }
          },
          "required": [
            "kind",
            "coords",
            "function",
            "extend"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "radial"
            },
            "coords": {
              "type": "array",
              "items": {
                "type": "number"
              },
              "minItems": 6,
              "maxItems": 6
            },
            "domain": {
              "anyOf": [
                {
                  "type": "array",
                  "items": {
                    "type": "number"
                  },
                  "minItems": 2,
                  "maxItems": 2
                },
                {
                  "type": "null"
                }
              ]
            },
            "function": {
              "$ref": "#/$defs/PdfFunction"
            },
            "extend": {
              "type": "array",
              "items": {
                "type": "boolean"
              },
              "minItems": 2,
              "maxItems": 2
            }
          },
          "required": [
            "kind",
            "coords",
            "function",
            "extend"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "mesh"
            },
            "shadingType": {
              "type": "integer",
              "minimum": 0
            },
            "bitsPerCoordinate": {
              "type": "integer",
              "minimum": 0
            },
            "bitsPerComponent": {
              "type": "integer",
              "minimum": 0
            },
            "bitsPerFlag": {
              "anyOf": [
                {
                  "type": "integer",
                  "minimum": 0
                },
                {
                  "type": "null"
                }
              ]
            },
            "verticesPerRow": {
              "anyOf": [
                {
                  "type": "integer",
                  "minimum": 0
                },
                {
                  "type": "null"
                }
              ]
            },
            "decode": {
              "type": "array",
              "items": {
                "type": "number"
              }
            },
            "function": {
              "anyOf": [
                {
                  "$ref": "#/$defs/PdfFunction"
                },
                {
                  "type": "null"
                }
              ]
            },
            "data": {
              "type": "array",
              "items": {
                "type": "integer",
                "minimum": 0
              }
            }
          },
          "required": [
            "kind",
            "shadingType",
            "bitsPerCoordinate",
            "bitsPerComponent",
            "decode",
            "data"
          ]
        }
      ]
    },
    "PdfSimpleEncoding": {
      "type": "object",
      "properties": {
        "base": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfBaseEncoding"
            },
            {
              "type": "null"
            }
          ]
        },
        "differences": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfEncodingDifference"
          }
        }
      }
    },
    "PdfSoftMask": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "none"
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "alpha"
            },
            "group": {
              "type": "string"
            },
            "transfer": {
              "anyOf": [
                {
                  "$ref": "#/$defs/PdfFunction"
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "required": [
            "kind",
            "group"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "luminosity"
            },
            "group": {
              "type": "string"
            },
            "backdrop": {
              "anyOf": [
                {
                  "type": "array",
                  "items": {
                    "type": "number"
                  }
                },
                {
                  "type": "null"
                }
              ]
            },
            "transfer": {
              "anyOf": [
                {
                  "$ref": "#/$defs/PdfFunction"
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "required": [
            "kind",
            "group"
          ]
        }
      ]
    },
    "PdfStreamFilter": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "flate"
            },
            "predictor": {
              "anyOf": [
                {
                  "$ref": "#/$defs/PdfPredictor"
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "lzw"
            },
            "predictor": {
              "anyOf": [
                {
                  "$ref": "#/$defs/PdfPredictor"
                },
                {
                  "type": "null"
                }
              ]
            },
            "earlyChange": {
              "type": "boolean"
            }
          },
          "required": [
            "kind",
            "earlyChange"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "asciiHex"
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "ascii85"
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "runLength"
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "dct"
            },
            "colorTransform": {
              "anyOf": [
                {
                  "type": "integer",
                  "minimum": 0
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "jpx"
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "ccitt"
            },
            "parameters": {
              "$ref": "#/$defs/PdfCcittParameters"
            }
          },
          "required": [
            "kind",
            "parameters"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "jbig2"
            },
            "globals": {
              "anyOf": [
                {
                  "type": "array",
                  "items": {
                    "type": "integer",
                    "minimum": 0
                  }
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "required": [
            "kind"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "crypt"
            },
            "name": {
              "anyOf": [
                {
                  "type": "string"
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "required": [
            "kind"
          ]
        }
      ]
    },
    "PdfTextArrayItem": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "text"
            },
            "text": {
              "type": "string"
            }
          },
          "required": [
            "kind",
            "text"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "codes"
            },
            "bytes": {
              "type": "array",
              "items": {
                "type": "integer",
                "minimum": 0
              }
            }
          },
          "required": [
            "kind",
            "bytes"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "adjust"
            },
            "amount": {
              "type": "number"
            }
          },
          "required": [
            "kind",
            "amount"
          ]
        }
      ]
    },
    "PdfTextString": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "text"
            },
            "text": {
              "type": "string"
            }
          },
          "required": [
            "kind",
            "text"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "codes"
            },
            "bytes": {
              "type": "array",
              "items": {
                "type": "integer",
                "minimum": 0
              }
            }
          },
          "required": [
            "kind",
            "bytes"
          ]
        }
      ]
    },
    "PdfToUnicode": {
      "type": "object",
      "properties": {
        "byteWidth": {
          "type": "integer",
          "minimum": 0
        },
        "mappings": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfToUnicodeMapping"
          }
        }
      },
      "required": [
        "byteWidth"
      ]
    },
    "PdfToUnicodeMapping": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "char"
            },
            "code": {
              "type": "integer",
              "minimum": 0
            },
            "text": {
              "type": "string"
            }
          },
          "required": [
            "kind",
            "code",
            "text"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "range"
            },
            "low": {
              "type": "integer",
              "minimum": 0
            },
            "high": {
              "type": "integer",
              "minimum": 0
            },
            "text": {
              "type": "string"
            }
          },
          "required": [
            "kind",
            "low",
            "high",
            "text"
          ]
        }
      ]
    },
    "PdfTransparencyGroup": {
      "type": "object",
      "properties": {
        "colorSpace": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfColorSpace"
            },
            {
              "type": "null"
            }
          ]
        },
        "isolated": {
          "type": "boolean"
        },
        "knockout": {
          "type": "boolean"
        }
      }
    },
    "PdfViewerPreferences": {
      "type": "object",
      "properties": {
        "hideToolbar": {
          "type": "boolean"
        },
        "hideMenubar": {
          "type": "boolean"
        },
        "hideWindowUi": {
          "type": "boolean"
        },
        "fitWindow": {
          "type": "boolean"
        },
        "centerWindow": {
          "type": "boolean"
        },
        "displayDocTitle": {
          "type": "boolean"
        },
        "nonFullScreenPageMode": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfPageMode"
            },
            {
              "type": "null"
            }
          ]
        },
        "direction": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "viewArea": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "viewClip": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "printArea": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "printClip": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "printScaling": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "duplex": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ]
        },
        "pickTrayByPdfSize": {
          "type": "boolean"
        },
        "printPageRange": {
          "type": "array",
          "items": {
            "type": "integer",
            "minimum": 0
          }
        },
        "numCopies": {
          "anyOf": [
            {
              "type": "integer",
              "minimum": 0
            },
            {
              "type": "null"
            }
          ]
        },
        "extra": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfDictEntry"
          }
        }
      }
    }
  }
} as const;

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
const resolveRef = (ref: string, own: Schema): Schema => {
  const [documentId, pointer] = ref.split("#");
  const document = documentId === "" ? own : documents.get(documentId);
  if (!document) throw new SchemaRefusal("$ref", `unknown schema document ${documentId}`);
  let node: unknown = document;
  for (const step of (pointer ?? "").split("/").filter((s) => s.length > 0)) node = (node as Record<string, unknown>)[step];
  if (!node) throw new SchemaRefusal("$ref", `unresolved pointer ${ref}`);
  return node as Schema;
};
const matches = (schema: Schema, value: unknown, own: Schema, at: string, errors: string[]): boolean => {
  if (typeof schema["$ref"] === "string") return matches(resolveRef(schema["$ref"] as string, own), value, own, at, errors);
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
    for (const [key, sub] of Object.entries(properties)) if (row[key] !== undefined && !matches(sub, row[key], own, `${at}.${key}`, errors)) return false;
  }
  return true;
};
export const validateAgainst = <T,>(schema: Schema, pointer: string, value: unknown): T => {
  const node = pointer === "" ? schema : resolveRef(`#${pointer}`, schema);
  const errors: string[] = [];
  if (!matches(node, value, schema, "$", errors)) throw new SchemaRefusal("$", errors[0] ?? "invalid");
  return value as T;
};
//#endregion 🚪️Validation
registerSchemaDocument(schema);
export const parsePdfSnapshot = (value: unknown): PdfSnapshot => pdfSnapshotFromNativeJson(validateAgainst<unknown>(schema, "", value));
export const parsePdfDictEntry = (value: unknown): PdfDictEntry => pdfDictionaryFromNativeJson([validateAgainst<unknown>(schema, "/$defs/PdfDictEntry", value)])[0]!;
export const parsePdfObject = (value: unknown): PdfObject => pdfCosFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfObject", value));
export const parsePdfStreamFilter = (value: unknown): PdfStreamFilter => validateAgainst<PdfStreamFilter>(schema, "/$defs/PdfStreamFilter", value);
export const parsePdfCcittParameters = (value: unknown): PdfCcittParameters => pdfCcittFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfCcittParameters", value));
export const parsePdfPredictor = (value: unknown): PdfPredictor => validateAgainst<PdfPredictor>(schema, "/$defs/PdfPredictor", value);
export const parseObjRef = (value: unknown): ObjRef => validateAgainst<ObjRef>(schema, "/$defs/ObjRef", value);
export const parsePdfDecimal = (value: unknown): PdfDecimal => validateAgainst<PdfDecimal>(schema, "/$defs/PdfDecimal", value);
export const parsePdfIndirectObject = (value: unknown): PdfIndirectObject => pdfIndirectFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfIndirectObject", value));
export const parsePdfInfo = (value: unknown): PdfInfo => pdfInfoFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfInfo", value));
export const parsePdfDate = (value: unknown): PdfDate => pdfDateFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfDate", value));
export const parsePdfEncryption = (value: unknown): PdfEncryption => pdfEncryptionFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfEncryption", value));
export const parsePdfEncryptionAlgorithm = (value: unknown): PdfEncryptionAlgorithm => validateAgainst<PdfEncryptionAlgorithm>(schema, "/$defs/PdfEncryptionAlgorithm", value);
export const parsePdfMarkInfo = (value: unknown): PdfMarkInfo => pdfMarkFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfMarkInfo", value));
export const parsePdfOpenAction = (value: unknown): PdfOpenAction => pdfOpenActionFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfOpenAction", value));
export const parsePdfAction = (value: unknown): PdfAction => pdfActionFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfAction", value));
export const parsePdfActionKind = (value: unknown): PdfActionKind => pdfActionKindFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfActionKind", value));
export const parsePdfFileSpecification = (value: unknown): PdfFileSpecification => pdfFileFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfFileSpecification", value));
export const parsePdfDestination = (value: unknown): PdfDestination => pdfDestinationFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfDestination", value));
export const parsePdfDestinationFit = (value: unknown): PdfDestinationFit => pdfFitFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfDestinationFit", value));
export const parsePdfViewerPreferences = (value: unknown): PdfViewerPreferences => pdfPreferencesFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfViewerPreferences", value));
export const parsePdfPageMode = (value: unknown): PdfPageMode => validateAgainst<PdfPageMode>(schema, "/$defs/PdfPageMode", value);
export const parsePdfPageLayout = (value: unknown): PdfPageLayout => validateAgainst<PdfPageLayout>(schema, "/$defs/PdfPageLayout", value);
export const parsePdfOptionalContent = (value: unknown): PdfOptionalContent => pdfOptionalFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfOptionalContent", value));
export const parsePdfOptionalContentGroup = (value: unknown): PdfOptionalContentGroup => pdfOptionalGroupFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfOptionalContentGroup", value));
export const parsePdfAcroForm = (value: unknown): PdfAcroForm => pdfAcroFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfAcroForm", value));
export const parsePdfFormField = (value: unknown): PdfFormField => pdfFieldFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfFormField", value));
export const parsePdfFormFieldKind = (value: unknown): PdfFormFieldKind => pdfFieldKindFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfFormFieldKind", value));
export const parsePdfOutputIntent = (value: unknown): PdfOutputIntent => pdfIntentFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfOutputIntent", value));
export const parsePdfEmbeddedFile = (value: unknown): PdfEmbeddedFile => pdfEmbeddedFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfEmbeddedFile", value));
export const parsePdfPageLabelRange = (value: unknown): PdfPageLabelRange => pdfLabelFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfPageLabelRange", value));
export const parsePdfPageLabelStyle = (value: unknown): PdfPageLabelStyle => validateAgainst<PdfPageLabelStyle>(schema, "/$defs/PdfPageLabelStyle", value);
export const parsePdfNamedDestination = (value: unknown): PdfNamedDestination => pdfNamedDestinationFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfNamedDestination", value));
export const parsePdfOutlineItem = (value: unknown): PdfOutlineItem => pdfOutlineFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfOutlineItem", value));
export const parsePdfNamedProperties = (value: unknown): PdfNamedProperties => pdfNamedPropertiesFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfNamedProperties", value));
export const parsePdfNamedColorSpace = (value: unknown): PdfNamedColorSpace => pdfNamedColorFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfNamedColorSpace", value));
export const parsePdfColorSpace = (value: unknown): PdfColorSpace => pdfColorFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfColorSpace", value));
export const parsePdfFunction = (value: unknown): PdfFunction => pdfFunctionFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfFunction", value));
export const parsePdfPattern = (value: unknown): PdfPattern => pdfPatternFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfPattern", value));
export const parsePdfPatternKind = (value: unknown): PdfPatternKind => pdfPatternKindFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfPatternKind", value));
export const parsePdfOp = (value: unknown): PdfOp => pdfOperationFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfOp", value));
export const parsePdfPropertyList = (value: unknown): PdfPropertyList => pdfPropertyFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfPropertyList", value));
export const parsePdfInlineImage = (value: unknown): PdfInlineImage => pdfInlineFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfInlineImage", value));
export const parsePdfTextString = (value: unknown): PdfTextString => pdfTextFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfTextString", value));
export const parsePdfTextArrayItem = (value: unknown): PdfTextArrayItem => pdfTextItemFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfTextArrayItem", value));
export const parsePdfLineJoin = (value: unknown): PdfLineJoin => validateAgainst<PdfLineJoin>(schema, "/$defs/PdfLineJoin", value);
export const parsePdfLineCap = (value: unknown): PdfLineCap => validateAgainst<PdfLineCap>(schema, "/$defs/PdfLineCap", value);
export const parsePdfShading = (value: unknown): PdfShading => pdfShadingFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfShading", value));
export const parsePdfShadingKind = (value: unknown): PdfShadingKind => pdfShadingKindFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfShadingKind", value));
export const parsePdfExtGState = (value: unknown): PdfExtGState => pdfStateFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfExtGState", value));
export const parsePdfSoftMask = (value: unknown): PdfSoftMask => pdfSoftMaskFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfSoftMask", value));
export const parsePdfFormXObject = (value: unknown): PdfFormXObject => pdfFormFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfFormXObject", value));
export const parsePdfTransparencyGroup = (value: unknown): PdfTransparencyGroup => pdfGroupFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfTransparencyGroup", value));
export const parsePdfImage = (value: unknown): PdfImage => pdfImageFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfImage", value));
export const parsePdfImageMask = (value: unknown): PdfImageMask => pdfImageMaskFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfImageMask", value));
export const parsePdfImageCodec = (value: unknown): PdfImageCodec => pdfImageCodecFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfImageCodec", value));
export const parsePdfFont = (value: unknown): PdfFont => pdfFontFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfFont", value));
export const parsePdfToUnicode = (value: unknown): PdfToUnicode => pdfUnicodeFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfToUnicode", value));
export const parsePdfToUnicodeMapping = (value: unknown): PdfToUnicodeMapping => pdfUnicodeMappingFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfToUnicodeMapping", value));
export const parsePdfFontKind = (value: unknown): PdfFontKind => pdfFontKindFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfFontKind", value));
export const parsePdfCidFont = (value: unknown): PdfCidFont => pdfCidFontFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfCidFont", value));
export const parsePdfFontProgram = (value: unknown): PdfFontProgram => pdfProgramFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfFontProgram", value));
export const parsePdfCidToGid = (value: unknown): PdfCidToGid => pdfGidFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfCidToGid", value));
export const parsePdfCidVerticalRun = (value: unknown): PdfCidVerticalRun => pdfVerticalRunFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfCidVerticalRun", value));
export const parsePdfCidWidthRun = (value: unknown): PdfCidWidthRun => pdfWidthRunFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfCidWidthRun", value));
export const parsePdfFontDescriptor = (value: unknown): PdfFontDescriptor => pdfDescriptorFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfFontDescriptor", value));
export const parsePdfCidSystemInfo = (value: unknown): PdfCidSystemInfo => validateAgainst<PdfCidSystemInfo>(schema, "/$defs/PdfCidSystemInfo", value);
export const parsePdfCMap = (value: unknown): PdfCMap => pdfCMapFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfCMap", value));
export const parsePdfEmbeddedCMap = (value: unknown): PdfEmbeddedCMap => { const row = pdfCMapFromNativeJson({ kind:"embedded",cmap:validateAgainst<unknown>(schema,"/$defs/PdfEmbeddedCMap",value) });if(row.kind!=="embedded")throw new Error("Invalid native CMap owner");return row.cmap; };
export const parsePdfCidMapping = (value: unknown): PdfCidMapping => pdfCidMappingFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfCidMapping", value));
export const parsePdfCodespaceRange = (value: unknown): PdfCodespaceRange => validateAgainst<PdfCodespaceRange>(schema, "/$defs/PdfCodespaceRange", value);
export const parsePdfCharProc = (value: unknown): PdfCharProc => pdfCharProcFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfCharProc", value));
export const parsePdfSimpleEncoding = (value: unknown): PdfSimpleEncoding => pdfEncodingFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfSimpleEncoding", value));
export const parsePdfEncodingDifference = (value: unknown): PdfEncodingDifference => validateAgainst<PdfEncodingDifference>(schema, "/$defs/PdfEncodingDifference", value);
export const parsePdfBaseEncoding = (value: unknown): PdfBaseEncoding => validateAgainst<PdfBaseEncoding>(schema, "/$defs/PdfBaseEncoding", value);
export const parsePdfPage = (value: unknown): PdfPage => pdfPageFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfPage", value));
export const parsePdfAnnotation = (value: unknown): PdfAnnotation => pdfAnnotationFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfAnnotation", value));
export const parsePdfMarkupAnnotation = (value: unknown): PdfMarkupAnnotation => pdfMarkupFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfMarkupAnnotation", value));
export const parsePdfAppearance = (value: unknown): PdfAppearance => pdfAppearanceFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfAppearance", value));
export const parsePdfAppearanceEntry = (value: unknown): PdfAppearanceEntry => pdfAppearanceEntryFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfAppearanceEntry", value));
export const parsePdfAppearanceState = (value: unknown): PdfAppearanceState => pdfAppearanceStateFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfAppearanceState", value));
export const parsePdfBorderStyle = (value: unknown): PdfBorderStyle => pdfBorderFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfBorderStyle", value));
export const parsePdfAnnotationKind = (value: unknown): PdfAnnotationKind => pdfAnnotationKindFromNativeJson(validateAgainst<unknown>(schema, "/$defs/PdfAnnotationKind", value));
