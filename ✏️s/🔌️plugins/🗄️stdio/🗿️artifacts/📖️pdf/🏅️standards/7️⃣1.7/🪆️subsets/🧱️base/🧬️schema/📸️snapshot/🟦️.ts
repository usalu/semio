/** 🧬️ Canonical owned PDF1.7 domain with explicit admission from its native JSON schema. */
import {parseBinary64,type Binary64} from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
export type { Binary64 };

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
  | { kind: "text"; value: string }
  | { kind: "date"; value: PdfDate }
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
  | { kind: "codes"; codes: number[] };

export type PdfTextArrayItem =
  | { kind: "text"; text: string }
  | { kind: "codes"; codes: number[] }
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
      "type": "string",
      "x-semio-ui": {
        "widget": "text",
        "label": {
          "en": "Snapshot schema",
          "de": "Snapshot-Schema"
        },
        "description": {
          "en": "Identifier of the snapshot schema version.",
          "de": "Kennung der Version des Snapshot-Schemas."
        }
      }
    },
    "declaredVersion": {
      "type": "string",
      "x-semio-ui": {
        "widget": "text",
        "label": {
          "en": "Declared PDF version",
          "de": "Deklarierte PDF-Version"
        },
        "description": {
          "en": "Version from the file header, e.g. 1.7.",
          "de": "Version aus dem Dateikopf, z. B. 1.7."
        }
      }
    },
    "pages": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfPage"
      },
      "x-semio-ui": {
        "label": {
          "en": "Pages",
          "de": "Seiten"
        }
      }
    },
    "fonts": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfFont"
      },
      "x-semio-ui": {
        "label": {
          "en": "Fonts",
          "de": "Schriften"
        },
        "description": {
          "en": "Font resources shared by the pages.",
          "de": "Von den Seiten gemeinsam genutzte Schriftressourcen."
        }
      }
    },
    "images": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfImage"
      },
      "x-semio-ui": {
        "label": {
          "en": "Images",
          "de": "Bilder"
        },
        "description": {
          "en": "Image XObjects shared by the pages.",
          "de": "Von den Seiten gemeinsam genutzte Bild-XObjekte."
        }
      }
    },
    "forms": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfFormXObject"
      },
      "x-semio-ui": {
        "label": {
          "en": "Form XObjects",
          "de": "Formular-XObjekte"
        },
        "description": {
          "en": "Reusable content streams shared by the pages.",
          "de": "Von den Seiten gemeinsam genutzte wiederverwendbare Inhaltsströme."
        }
      }
    },
    "extGStates": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfExtGState"
      },
      "x-semio-ui": {
        "label": {
          "en": "Graphics states",
          "de": "Grafikzustände"
        },
        "description": {
          "en": "Extended graphics state parameter dictionaries (ExtGState).",
          "de": "Wörterbücher erweiterter Grafikzustandsparameter (ExtGState)."
        }
      }
    },
    "shadings": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfShading"
      },
      "x-semio-ui": {
        "label": {
          "en": "Shadings",
          "de": "Schattierungen"
        }
      }
    },
    "patterns": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfPattern"
      },
      "x-semio-ui": {
        "label": {
          "en": "Patterns",
          "de": "Muster"
        }
      }
    },
    "colorSpaces": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfNamedColorSpace"
      },
      "x-semio-ui": {
        "label": {
          "en": "Color spaces",
          "de": "Farbräume"
        }
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
      },
      "x-semio-ui": {
        "label": {
          "en": "Bookmarks",
          "de": "Lesezeichen"
        },
        "description": {
          "en": "Document outline in reading order.",
          "de": "Dokumentgliederung in Lesereihenfolge."
        }
      }
    },
    "namedDestinations": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfNamedDestination"
      },
      "x-semio-ui": {
        "label": {
          "en": "Named destinations",
          "de": "Benannte Ziele"
        }
      }
    },
    "pageLabels": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfPageLabelRange"
      },
      "x-semio-ui": {
        "label": {
          "en": "Page labels",
          "de": "Seitenbeschriftungen"
        },
        "description": {
          "en": "Page numbering ranges, e.g. roman front matter.",
          "de": "Seitennummerierungsbereiche, z. B. römisch nummerierte Titelei."
        }
      }
    },
    "embeddedFiles": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfEmbeddedFile"
      },
      "x-semio-ui": {
        "label": {
          "en": "Embedded files",
          "de": "Eingebettete Dateien"
        }
      }
    },
    "outputIntents": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfOutputIntent"
      },
      "x-semio-ui": {
        "label": {
          "en": "Output intents",
          "de": "Ausgabebedingungen"
        },
        "description": {
          "en": "Intended output devices and their ICC profiles.",
          "de": "Vorgesehene Ausgabegeräte und ihre ICC-Profile."
        }
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
      ],
      "x-semio-ui": {
        "label": {
          "en": "Interactive form",
          "de": "Interaktives Formular"
        },
        "description": {
          "en": "AcroForm fields and their defaults.",
          "de": "AcroForm-Felder und ihre Vorgaben."
        }
      }
    },
    "optionalContent": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfOptionalContent"
        },
        {
          "type": "null"
        }
      ],
      "x-semio-ui": {
        "label": {
          "en": "Optional content",
          "de": "Optionale Inhalte"
        },
        "description": {
          "en": "Layers (optional content groups) and their default visibility.",
          "de": "Ebenen (optionale Inhaltsgruppen) und ihre Standardsichtbarkeit."
        }
      }
    },
    "pageLayout": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfPageLayout"
        },
        {
          "type": "null"
        }
      ],
      "x-semio-ui": {
        "widget": "select",
        "label": {
          "en": "Page layout",
          "de": "Seitenlayout"
        },
        "description": {
          "en": "Page arrangement when the document opens.",
          "de": "Seitenanordnung beim Öffnen des Dokuments."
        },
        "options": {
          "singlePage": {
            "en": "Single page",
            "de": "Einzelne Seite"
          },
          "oneColumn": {
            "en": "One column",
            "de": "Fortlaufend"
          },
          "twoColumnLeft": {
            "en": "Two columns, odd pages left",
            "de": "Zwei Spalten, ungerade Seiten links"
          },
          "twoColumnRight": {
            "en": "Two columns, odd pages right",
            "de": "Zwei Spalten, ungerade Seiten rechts"
          },
          "twoPageLeft": {
            "en": "Two pages, odd pages left",
            "de": "Doppelseite, ungerade Seiten links"
          },
          "twoPageRight": {
            "en": "Two pages, odd pages right",
            "de": "Doppelseite, ungerade Seiten rechts"
          }
        }
      }
    },
    "pageMode": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfPageMode"
        },
        {
          "type": "null"
        }
      ],
      "x-semio-ui": {
        "widget": "select",
        "label": {
          "en": "Page mode",
          "de": "Seitenmodus"
        },
        "description": {
          "en": "Panel shown when the document opens.",
          "de": "Beim Öffnen des Dokuments angezeigte Leiste."
        },
        "options": {
          "useNone": {
            "en": "Page only",
            "de": "Nur Seite"
          },
          "useOutlines": {
            "en": "Bookmarks panel",
            "de": "Lesezeichenfenster"
          },
          "useThumbs": {
            "en": "Page thumbnails",
            "de": "Seitenminiaturen"
          },
          "fullScreen": {
            "en": "Full screen",
            "de": "Vollbild"
          },
          "useOc": {
            "en": "Layers panel",
            "de": "Ebenenfenster"
          },
          "useAttachments": {
            "en": "Attachments panel",
            "de": "Anlagenfenster"
          }
        }
      }
    },
    "viewerPreferences": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfViewerPreferences"
        },
        {
          "type": "null"
        }
      ],
      "x-semio-ui": {
        "label": {
          "en": "Viewer preferences",
          "de": "Anzeigeeinstellungen"
        }
      }
    },
    "openAction": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfOpenAction"
        },
        {
          "type": "null"
        }
      ],
      "x-semio-ui": {
        "label": {
          "en": "Open action",
          "de": "Aktion beim Öffnen"
        },
        "description": {
          "en": "Destination or action performed when the document opens.",
          "de": "Ziel oder Aktion beim Öffnen des Dokuments."
        }
      }
    },
    "language": {
      "anyOf": [
        {
          "type": "string"
        },
        {
          "type": "null"
        }
      ],
      "x-semio-ui": {
        "widget": "text",
        "label": {
          "en": "Document language",
          "de": "Dokumentsprache"
        },
        "description": {
          "en": "BCP 47 tag of the natural language, e.g. de-DE.",
          "de": "BCP-47-Kennung der natürlichen Sprache, z. B. de-DE."
        }
      }
    },
    "markInfo": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfMarkInfo"
        },
        {
          "type": "null"
        }
      ],
      "x-semio-ui": {
        "label": {
          "en": "Tagged PDF marks",
          "de": "Tagged-PDF-Kennzeichnung"
        },
        "description": {
          "en": "Whether the document is a tagged PDF and how it is marked.",
          "de": "Ob das Dokument ein Tagged PDF ist und wie es gekennzeichnet ist."
        }
      }
    },
    "metadata": {
      "anyOf": [
        {
          "type": "string"
        },
        {
          "type": "null"
        }
      ],
      "x-semio-ui": {
        "widget": "multiline",
        "label": {
          "en": "XMP metadata",
          "de": "XMP-Metadaten"
        },
        "description": {
          "en": "Document-level XMP packet.",
          "de": "XMP-Paket auf Dokumentebene."
        }
      }
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
      ],
      "x-semio-ui": {
        "label": {
          "en": "Document ID",
          "de": "Dokument-ID"
        },
        "description": {
          "en": "Permanent and changing identifier byte strings of the trailer ID.",
          "de": "Beständige und veränderliche Kennungsbytefolgen der Trailer-ID."
        }
      }
    },
    "encryption": {
      "anyOf": [
        {
          "$ref": "#/$defs/PdfEncryption"
        },
        {
          "type": "null"
        }
      ],
      "x-semio-ui": {
        "label": {
          "en": "Encryption",
          "de": "Verschlüsselung"
        }
      }
    },
    "info": {
      "$ref": "#/$defs/PdfInfo",
      "x-semio-ui": {
        "label": {
          "en": "Document information",
          "de": "Dokumentinformationen"
        }
      }
    },
    "catalogExtra": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfDictEntry"
      },
      "x-semio-ui": {
        "label": {
          "en": "Other catalog entries",
          "de": "Weitere Katalogeinträge"
        },
        "description": {
          "en": "Document catalog entries kept verbatim.",
          "de": "Unverändert erhaltene Einträge des Dokumentkatalogs."
        }
      }
    },
    "objects": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfIndirectObject"
      },
      "x-semio-ui": {
        "label": {
          "en": "Indirect objects",
          "de": "Indirekte Objekte"
        }
      }
    },
    "trailer": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/PdfDictEntry"
      },
      "x-semio-ui": {
        "label": {
          "en": "Trailer",
          "de": "Trailer"
        },
        "description": {
          "en": "Trailer dictionary entries.",
          "de": "Einträge des Trailer-Wörterbuchs."
        }
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
          "minimum": 0,
          "x-semio-ui": {
            "widget": "stepper",
            "label": {
              "en": "Object number",
              "de": "Objektnummer"
            }
          }
        },
        "gen": {
          "type": "integer",
          "minimum": 0,
          "x-semio-ui": {
            "widget": "stepper",
            "label": {
              "en": "Generation number",
              "de": "Generationsnummer"
            }
          }
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
          },
          "x-semio-ui": {
            "label": {
              "en": "Form fields",
              "de": "Formularfelder"
            }
          }
        },
        "needAppearances": {
          "type": "boolean",
          "x-semio-ui": {
            "widget": "toggle",
            "label": {
              "en": "Generate appearances",
              "de": "Erscheinungsbilder erzeugen"
            },
            "description": {
              "en": "NeedAppearances: viewers regenerate field appearances.",
              "de": "NeedAppearances: Anzeigeprogramme erzeugen Feld-Erscheinungsbilder neu."
            }
          }
        },
        "signatureFlags": {
          "type": "integer",
          "minimum": 0,
          "x-semio-ui": {
            "widget": "stepper",
            "label": {
              "en": "Signature flags",
              "de": "Signatur-Flags"
            },
            "description": {
              "en": "SigFlags: 1 signatures exist, 2 append only.",
              "de": "SigFlags: 1 Signaturen vorhanden, 2 nur anhängen."
            }
          }
        },
        "defaultAppearance": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "text",
            "label": {
              "en": "Default appearance",
              "de": "Standard-Erscheinungsbild"
            },
            "description": {
              "en": "DA: content stream operators for variable text, e.g. /Helv 12 Tf 0 g.",
              "de": "DA: Inhaltsstrom-Operatoren für variablen Text, z. B. /Helv 12 Tf 0 g."
            }
          }
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
          ],
          "x-semio-ui": {
            "widget": "stepper",
            "label": {
              "en": "Text alignment",
              "de": "Textausrichtung"
            },
            "description": {
              "en": "Q: 0 left, 1 centered, 2 right.",
              "de": "Q: 0 links, 1 zentriert, 2 rechts."
            }
          }
        },
        "defaultFonts": {
          "type": "array",
          "items": {
            "type": "string"
          },
          "x-semio-ui": {
            "label": {
              "en": "Default resources",
              "de": "Standardressourcen"
            },
            "description": {
              "en": "DR: font resources available to form fields.",
              "de": "DR: Schriftressourcen für Formularfelder."
            }
          }
        },
        "extra": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfDictEntry"
          },
          "x-semio-ui": {
            "label": {
              "en": "Additional entries",
              "de": "Weitere Einträge"
            },
            "description": {
              "en": "Dictionary entries without a dedicated field, kept verbatim.",
              "de": "Wörterbucheinträge ohne eigenes Feld, unverändert erhalten."
            }
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
          },
          "x-semio-ui": {
            "label": {
              "en": "Next actions",
              "de": "Folgeaktionen"
            }
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
                  "type": "object",
                  "semioPrimitive": "binary64",
                  "properties": {
                    "bits": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{16}$"
                    }
                  },
                  "required": [
                    "bits"
                  ],
                  "additionalProperties": false
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
            "type": "object",
            "semioPrimitive": "binary64",
            "properties": {
              "bits": {
                "type": "string",
                "pattern": "^[0-9a-f]{16}$"
              }
            },
            "required": [
              "bits"
            ],
            "additionalProperties": false
          },
          "minItems": 4,
          "maxItems": 4,
          "x-semio-ui": {
            "widget": "vector",
            "label": {
              "en": "Rectangle",
              "de": "Rechteck"
            },
            "description": {
              "en": "Annotation location on the page as [llx lly urx ury].",
              "de": "Position der Anmerkung auf der Seite als [llx lly urx ury]."
            },
            "unit": "pt"
          }
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
          ],
          "x-semio-ui": {
            "widget": "multiline",
            "label": {
              "en": "Contents",
              "de": "Inhalt"
            },
            "description": {
              "en": "Text shown for the annotation or its alternate description.",
              "de": "Angezeigter Text der Anmerkung oder ihre Ersatzbeschreibung."
            }
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
        "modified": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "text",
            "label": {
              "en": "Last modified",
              "de": "Zuletzt geändert"
            }
          }
        },
        "flags": {
          "type": "integer",
          "minimum": 0,
          "x-semio-ui": {
            "widget": "stepper",
            "label": {
              "en": "Annotation flags",
              "de": "Anmerkungs-Flags"
            },
            "description": {
              "en": "F: bit set, e.g. 1 invisible, 2 hidden, 4 print.",
              "de": "F: Bitmenge, z. B. 1 unsichtbar, 2 ausgeblendet, 4 drucken."
            }
          }
        },
        "border": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfBorderStyle"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "label": {
              "en": "Border",
              "de": "Rahmen"
            }
          }
        },
        "color": {
          "type": "array",
          "items": {
            "type": "object",
            "semioPrimitive": "binary64",
            "properties": {
              "bits": {
                "type": "string",
                "pattern": "^[0-9a-f]{16}$"
              }
            },
            "required": [
              "bits"
            ],
            "additionalProperties": false
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
          ],
          "x-semio-ui": {
            "label": {
              "en": "Appearance",
              "de": "Erscheinungsbild"
            }
          }
        },
        "appearanceState": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "text",
            "label": {
              "en": "Appearance state",
              "de": "Erscheinungszustand"
            },
            "description": {
              "en": "AS: selected state of the appearance dictionary, e.g. On or Off.",
              "de": "AS: gewählter Zustand des Erscheinungsbilds, z. B. On oder Off."
            }
          }
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
          ],
          "x-semio-ui": {
            "widget": "text",
            "label": {
              "en": "Optional content",
              "de": "Optionaler Inhalt (Ebene)"
            },
            "description": {
              "en": "OC: layer or membership that controls visibility.",
              "de": "OC: Ebene oder Zugehörigkeit, die die Sichtbarkeit steuert."
            }
          }
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
          ],
          "x-semio-ui": {
            "widget": "stepper",
            "label": {
              "en": "Structure parent",
              "de": "Strukturelternschlüssel"
            },
            "description": {
              "en": "Key into the structural parent tree.",
              "de": "Schlüssel in den Strukturelternbaum."
            }
          }
        },
        "extra": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfDictEntry"
          },
          "x-semio-ui": {
            "label": {
              "en": "Additional entries",
              "de": "Weitere Einträge"
            },
            "description": {
              "en": "Dictionary entries without a dedicated field, kept verbatim.",
              "de": "Wörterbucheinträge ohne eigenes Feld, unverändert erhalten."
            }
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
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
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
                    "type": "object",
                    "semioPrimitive": "binary64",
                    "properties": {
                      "bits": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{16}$"
                      }
                    },
                    "required": [
                      "bits"
                    ],
                    "additionalProperties": false
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
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
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
                    "type": "object",
                    "semioPrimitive": "binary64",
                    "properties": {
                      "bits": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{16}$"
                      }
                    },
                    "required": [
                      "bits"
                    ],
                    "additionalProperties": false
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
                  "type": "object",
                  "semioPrimitive": "binary64",
                  "properties": {
                    "bits": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{16}$"
                    }
                  },
                  "required": [
                    "bits"
                  ],
                  "additionalProperties": false
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
                    "type": "object",
                    "semioPrimitive": "binary64",
                    "properties": {
                      "bits": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{16}$"
                      }
                    },
                    "required": [
                      "bits"
                    ],
                    "additionalProperties": false
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
                    "type": "object",
                    "semioPrimitive": "binary64",
                    "properties": {
                      "bits": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{16}$"
                      }
                    },
                    "required": [
                      "bits"
                    ],
                    "additionalProperties": false
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
                    "type": "object",
                    "semioPrimitive": "binary64",
                    "properties": {
                      "bits": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{16}$"
                      }
                    },
                    "required": [
                      "bits"
                    ],
                    "additionalProperties": false
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
                    "type": "object",
                    "semioPrimitive": "binary64",
                    "properties": {
                      "bits": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{16}$"
                      }
                    },
                    "required": [
                      "bits"
                    ],
                    "additionalProperties": false
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
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
              }
            },
            "interiorColor": {
              "anyOf": [
                {
                  "type": "array",
                  "items": {
                    "type": "object",
                    "semioPrimitive": "binary64",
                    "properties": {
                      "bits": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{16}$"
                      }
                    },
                    "required": [
                      "bits"
                    ],
                    "additionalProperties": false
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
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
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
                    "type": "object",
                    "semioPrimitive": "binary64",
                    "properties": {
                      "bits": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{16}$"
                      }
                    },
                    "required": [
                      "bits"
                    ],
                    "additionalProperties": false
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
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
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
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
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
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
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
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
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
                    "type": "object",
                    "semioPrimitive": "binary64",
                    "properties": {
                      "bits": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{16}$"
                      }
                    },
                    "required": [
                      "bits"
                    ],
                    "additionalProperties": false
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
                  "type": "object",
                  "semioPrimitive": "binary64",
                  "properties": {
                    "bits": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{16}$"
                    }
                  },
                  "required": [
                    "bits"
                  ],
                  "additionalProperties": false
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
                  "minimum": 0,
                  "semioPrimitive": "u64",
                  "maximum": 18446744073709551615
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
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
              }
            },
            "interiorColor": {
              "anyOf": [
                {
                  "type": "array",
                  "items": {
                    "type": "object",
                    "semioPrimitive": "binary64",
                    "properties": {
                      "bits": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{16}$"
                      }
                    },
                    "required": [
                      "bits"
                    ],
                    "additionalProperties": false
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
          ],
          "x-semio-ui": {
            "label": {
              "en": "Rollover appearance",
              "de": "Erscheinungsbild „Rollover“"
            }
          }
        },
        "down": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfAppearanceEntry"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "label": {
              "en": "Down appearance",
              "de": "Erscheinungsbild „Gedrückt“"
            }
          }
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
          "type": "object",
          "semioPrimitive": "binary64",
          "properties": {
            "bits": {
              "type": "string",
              "pattern": "^[0-9a-f]{16}$"
            }
          },
          "required": [
            "bits"
          ],
          "additionalProperties": false
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
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
              }
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "label": {
              "en": "Dash pattern",
              "de": "Strichmuster"
            },
            "description": {
              "en": "Dash and gap lengths in points.",
              "de": "Strich- und Lückenlängen in Punkt."
            }
          }
        },
        "radii": {
          "anyOf": [
            {
              "type": "array",
              "items": {
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
              },
              "minItems": 2,
              "maxItems": 2
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "vector",
            "label": {
              "en": "Corner radii",
              "de": "Eckenradien"
            },
            "description": {
              "en": "Horizontal and vertical corner radius of the border.",
              "de": "Horizontaler und vertikaler Eckenradius des Rahmens."
            },
            "unit": "pt"
          }
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
          "type": "object",
          "semioPrimitive": "binary64",
          "properties": {
            "bits": {
              "type": "string",
              "pattern": "^[0-9a-f]{16}$"
            }
          },
          "required": [
            "bits"
          ],
          "additionalProperties": false
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
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
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
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
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
            "type": "object",
            "semioPrimitive": "binary64",
            "properties": {
              "bits": {
                "type": "string",
                "pattern": "^[0-9a-f]{16}$"
              }
            },
            "required": [
              "bits"
            ],
            "additionalProperties": false
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
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
              },
              "minItems": 3,
              "maxItems": 3
            },
            "blackPoint": {
              "anyOf": [
                {
                  "type": "array",
                  "items": {
                    "type": "object",
                    "semioPrimitive": "binary64",
                    "properties": {
                      "bits": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{16}$"
                      }
                    },
                    "required": [
                      "bits"
                    ],
                    "additionalProperties": false
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
                  "type": "object",
                  "semioPrimitive": "binary64",
                  "properties": {
                    "bits": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{16}$"
                    }
                  },
                  "required": [
                    "bits"
                  ],
                  "additionalProperties": false
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
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
              },
              "minItems": 3,
              "maxItems": 3
            },
            "blackPoint": {
              "anyOf": [
                {
                  "type": "array",
                  "items": {
                    "type": "object",
                    "semioPrimitive": "binary64",
                    "properties": {
                      "bits": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{16}$"
                      }
                    },
                    "required": [
                      "bits"
                    ],
                    "additionalProperties": false
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
                    "type": "object",
                    "semioPrimitive": "binary64",
                    "properties": {
                      "bits": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{16}$"
                      }
                    },
                    "required": [
                      "bits"
                    ],
                    "additionalProperties": false
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
                    "type": "object",
                    "semioPrimitive": "binary64",
                    "properties": {
                      "bits": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{16}$"
                      }
                    },
                    "required": [
                      "bits"
                    ],
                    "additionalProperties": false
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
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
              },
              "minItems": 3,
              "maxItems": 3
            },
            "blackPoint": {
              "anyOf": [
                {
                  "type": "array",
                  "items": {
                    "type": "object",
                    "semioPrimitive": "binary64",
                    "properties": {
                      "bits": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{16}$"
                      }
                    },
                    "required": [
                      "bits"
                    ],
                    "additionalProperties": false
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
                    "type": "object",
                    "semioPrimitive": "binary64",
                    "properties": {
                      "bits": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{16}$"
                      }
                    },
                    "required": [
                      "bits"
                    ],
                    "additionalProperties": false
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
                    "type": "object",
                    "semioPrimitive": "binary64",
                    "properties": {
                      "bits": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{16}$"
                      }
                    },
                    "required": [
                      "bits"
                    ],
                    "additionalProperties": false
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
          "minimum": 0,
          "x-semio-ui": {
            "widget": "stepper",
            "label": {
              "en": "Month",
              "de": "Monat"
            }
          }
        },
        "day": {
          "type": "integer",
          "minimum": 0,
          "x-semio-ui": {
            "widget": "stepper",
            "label": {
              "en": "Day",
              "de": "Tag"
            }
          }
        },
        "hour": {
          "type": "integer",
          "minimum": 0,
          "x-semio-ui": {
            "widget": "stepper",
            "label": {
              "en": "Hour",
              "de": "Stunde"
            }
          }
        },
        "minute": {
          "type": "integer",
          "minimum": 0,
          "x-semio-ui": {
            "widget": "stepper",
            "label": {
              "en": "Minute",
              "de": "Minute"
            }
          }
        },
        "second": {
          "type": "integer",
          "minimum": 0,
          "x-semio-ui": {
            "widget": "stepper",
            "label": {
              "en": "Second",
              "de": "Sekunde"
            }
          }
        },
        "offsetMinutes": {
          "anyOf": [
            {
              "type": "integer"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "stepper",
            "label": {
              "en": "UTC offset",
              "de": "UTC-Versatz"
            },
            "description": {
              "en": "Offset of local time from UTC in minutes.",
              "de": "Abweichung der Ortszeit von UTC in Minuten."
            },
            "unit": "min"
          }
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
                  "type": "object",
                  "semioPrimitive": "binary64",
                  "properties": {
                    "bits": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{16}$"
                    }
                  },
                  "required": [
                    "bits"
                  ],
                  "additionalProperties": false
                },
                {
                  "type": "null"
                }
              ]
            },
            "top": {
              "anyOf": [
                {
                  "type": "object",
                  "semioPrimitive": "binary64",
                  "properties": {
                    "bits": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{16}$"
                    }
                  },
                  "required": [
                    "bits"
                  ],
                  "additionalProperties": false
                },
                {
                  "type": "null"
                }
              ]
            },
            "zoom": {
              "anyOf": [
                {
                  "type": "object",
                  "semioPrimitive": "binary64",
                  "properties": {
                    "bits": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{16}$"
                    }
                  },
                  "required": [
                    "bits"
                  ],
                  "additionalProperties": false
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
                  "type": "object",
                  "semioPrimitive": "binary64",
                  "properties": {
                    "bits": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{16}$"
                    }
                  },
                  "required": [
                    "bits"
                  ],
                  "additionalProperties": false
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
                  "type": "object",
                  "semioPrimitive": "binary64",
                  "properties": {
                    "bits": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{16}$"
                    }
                  },
                  "required": [
                    "bits"
                  ],
                  "additionalProperties": false
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
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
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
                  "type": "object",
                  "semioPrimitive": "binary64",
                  "properties": {
                    "bits": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{16}$"
                    }
                  },
                  "required": [
                    "bits"
                  ],
                  "additionalProperties": false
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
                  "type": "object",
                  "semioPrimitive": "binary64",
                  "properties": {
                    "bits": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{16}$"
                    }
                  },
                  "required": [
                    "bits"
                  ],
                  "additionalProperties": false
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
          ],
          "x-semio-ui": {
            "widget": "text",
            "label": {
              "en": "MIME type",
              "de": "MIME-Typ"
            }
          }
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
          ],
          "x-semio-ui": {
            "label": {
              "en": "Creation date",
              "de": "Erstellungsdatum"
            }
          }
        },
        "modificationDate": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfDate"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "label": {
              "en": "Modification date",
              "de": "Änderungsdatum"
            }
          }
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
          "type": "boolean",
          "x-semio-ui": {
            "widget": "toggle",
            "label": {
              "en": "Listed as attachment",
              "de": "Als Anlage aufgeführt"
            },
            "description": {
              "en": "Listed in the document's EmbeddedFiles name tree.",
              "de": "Im Namensbaum EmbeddedFiles des Dokuments aufgeführt."
            }
          }
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
          "$ref": "#/$defs/PdfEncryptionAlgorithm",
          "x-semio-ui": {
            "widget": "select",
            "label": {
              "en": "Encryption algorithm",
              "de": "Verschlüsselungsverfahren"
            },
            "options": {
              "rc4_40": {
                "en": "RC4, 40-bit",
                "de": "RC4, 40 Bit"
              },
              "rc4_128": {
                "en": "RC4, 128-bit",
                "de": "RC4, 128 Bit"
              },
              "aes128": {
                "en": "AES, 128-bit",
                "de": "AES, 128 Bit"
              },
              "aes256": {
                "en": "AES, 256-bit",
                "de": "AES, 256 Bit"
              }
            }
          }
        },
        "permissions": {
          "type": "integer",
          "x-semio-ui": {
            "widget": "stepper",
            "label": {
              "en": "Permissions",
              "de": "Berechtigungen"
            },
            "description": {
              "en": "P: bit set of allowed operations (print, modify, copy, annotate, …).",
              "de": "P: Bitmenge erlaubter Vorgänge (Drucken, Ändern, Kopieren, Kommentieren, …)."
            }
          }
        },
        "userPassword": {
          "type": "string",
          "x-semio-ui": {
            "widget": "text",
            "label": {
              "en": "Document open password",
              "de": "Kennwort zum Öffnen des Dokuments"
            }
          }
        },
        "ownerPassword": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "text",
            "label": {
              "en": "Permissions password",
              "de": "Berechtigungskennwort"
            },
            "description": {
              "en": "Owner password that lifts the permission restrictions.",
              "de": "Besitzerkennwort, das die Berechtigungseinschränkungen aufhebt."
            }
          }
        },
        "encryptMetadata": {
          "type": "boolean",
          "x-semio-ui": {
            "widget": "toggle",
            "label": {
              "en": "Encrypt metadata",
              "de": "Metadaten verschlüsseln"
            }
          }
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
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "stepper",
            "label": {
              "en": "Line width",
              "de": "Linienstärke"
            },
            "description": {
              "en": "LW: stroke width in user space units.",
              "de": "LW: Konturbreite in Einheiten des Benutzerraums."
            }
          }
        },
        "lineCap": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfLineCap"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "segmented",
            "label": {
              "en": "Line cap",
              "de": "Linienende"
            },
            "options": {
              "butt": {
                "en": "Butt cap",
                "de": "Abgeschnitten"
              },
              "round": {
                "en": "Round cap",
                "de": "Rund"
              },
              "square": {
                "en": "Projecting square cap",
                "de": "Überstehend quadratisch"
              }
            }
          }
        },
        "lineJoin": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfLineJoin"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "segmented",
            "label": {
              "en": "Line join",
              "de": "Linienverbindung"
            },
            "options": {
              "miter": {
                "en": "Miter join",
                "de": "Gehrung"
              },
              "round": {
                "en": "Round join",
                "de": "Rund"
              },
              "bevel": {
                "en": "Bevel join",
                "de": "Abgeflacht"
              }
            }
          }
        },
        "miterLimit": {
          "anyOf": [
            {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "stepper",
            "label": {
              "en": "Miter limit",
              "de": "Gehrungsgrenze"
            }
          }
        },
        "dash": {
          "anyOf": [
            {
              "type": "array",
              "items": [
                {
                  "type": "array",
                  "items": {
                    "type": "object",
                    "semioPrimitive": "binary64",
                    "properties": {
                      "bits": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{16}$"
                      }
                    },
                    "required": [
                      "bits"
                    ],
                    "additionalProperties": false
                  }
                },
                {
                  "type": "object",
                  "semioPrimitive": "binary64",
                  "properties": {
                    "bits": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{16}$"
                    }
                  },
                  "required": [
                    "bits"
                  ],
                  "additionalProperties": false
                }
              ],
              "minItems": 2,
              "maxItems": 2
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "label": {
              "en": "Dash pattern",
              "de": "Strichmuster"
            },
            "description": {
              "en": "[dash array, phase] of the line dash pattern.",
              "de": "[Strichmuster, Phase] des Linienstrichmusters."
            }
          }
        },
        "renderingIntent": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "text",
            "label": {
              "en": "Rendering intent",
              "de": "Rendering Intent"
            }
          }
        },
        "overprintStroke": {
          "anyOf": [
            {
              "type": "boolean"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "toggle",
            "label": {
              "en": "Overprint stroke",
              "de": "Kontur überdrucken"
            }
          }
        },
        "overprintFill": {
          "anyOf": [
            {
              "type": "boolean"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "toggle",
            "label": {
              "en": "Overprint fill",
              "de": "Füllung überdrucken"
            }
          }
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
          ],
          "x-semio-ui": {
            "widget": "stepper",
            "label": {
              "en": "Overprint mode",
              "de": "Überdruckmodus"
            },
            "description": {
              "en": "OPM: 0 or 1 (non-zero overprint).",
              "de": "OPM: 0 oder 1 (Überdrucken ungleich null)."
            }
          }
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
                  "type": "object",
                  "semioPrimitive": "binary64",
                  "properties": {
                    "bits": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{16}$"
                    }
                  },
                  "required": [
                    "bits"
                  ],
                  "additionalProperties": false
                }
              ],
              "minItems": 2,
              "maxItems": 2
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "label": {
              "en": "Font",
              "de": "Schrift"
            },
            "description": {
              "en": "[font reference, size] set by the graphics state.",
              "de": "[Schriftreferenz, Größe], die der Grafikzustand setzt."
            }
          }
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
          ],
          "x-semio-ui": {
            "label": {
              "en": "Blend mode",
              "de": "Füllmethode"
            },
            "description": {
              "en": "BM: blend mode names, the first supported one applies.",
              "de": "BM: Namen von Füllmethoden; die erste unterstützte gilt."
            }
          }
        },
        "softMask": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfSoftMask"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "label": {
              "en": "Soft mask",
              "de": "Weiche Maske"
            }
          }
        },
        "strokeAlpha": {
          "anyOf": [
            {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "stepper",
            "label": {
              "en": "Stroke opacity",
              "de": "Deckkraft der Kontur"
            },
            "description": {
              "en": "CA: constant alpha for stroke operations, 0 to 1.",
              "de": "CA: konstanter Alphawert für Konturen, 0 bis 1."
            }
          }
        },
        "fillAlpha": {
          "anyOf": [
            {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "stepper",
            "label": {
              "en": "Fill opacity",
              "de": "Deckkraft der Füllung"
            },
            "description": {
              "en": "ca: constant alpha for fill operations, 0 to 1.",
              "de": "ca: konstanter Alphawert für Füllungen, 0 bis 1."
            }
          }
        },
        "alphaIsShape": {
          "anyOf": [
            {
              "type": "boolean"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "toggle",
            "label": {
              "en": "Alpha is shape",
              "de": "Alpha als Form"
            },
            "description": {
              "en": "AIS: soft mask and alpha are shape instead of opacity.",
              "de": "AIS: Weiche Maske und Alpha gelten als Form statt als Deckkraft."
            }
          }
        },
        "strokeAdjust": {
          "anyOf": [
            {
              "type": "boolean"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "toggle",
            "label": {
              "en": "Automatic stroke adjustment",
              "de": "Automatische Konturanpassung"
            }
          }
        },
        "flatness": {
          "anyOf": [
            {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "stepper",
            "label": {
              "en": "Flatness tolerance",
              "de": "Flachheitstoleranz"
            },
            "description": {
              "en": "FL: maximum curve approximation error in device pixels.",
              "de": "FL: maximaler Fehler der Kurvennäherung in Gerätepixeln."
            }
          }
        },
        "smoothness": {
          "anyOf": [
            {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "stepper",
            "label": {
              "en": "Smoothness tolerance",
              "de": "Glättungstoleranz"
            },
            "description": {
              "en": "SM: maximum color error of shading approximation, 0 to 1.",
              "de": "SM: maximaler Farbfehler der Verlaufsnäherung, 0 bis 1."
            }
          }
        },
        "textKnockout": {
          "anyOf": [
            {
              "type": "boolean"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "toggle",
            "label": {
              "en": "Text knockout",
              "de": "Text-Aussparung"
            }
          }
        },
        "extra": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfDictEntry"
          },
          "x-semio-ui": {
            "label": {
              "en": "Additional entries",
              "de": "Weitere Einträge"
            },
            "description": {
              "en": "Dictionary entries without a dedicated field, kept verbatim.",
              "de": "Wörterbucheinträge ohne eigenes Feld, unverändert erhalten."
            }
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
          ],
          "x-semio-ui": {
            "label": {
              "en": "ToUnicode CMap",
              "de": "ToUnicode-CMap"
            },
            "description": {
              "en": "Maps character codes to Unicode for text extraction.",
              "de": "Ordnet Zeichencodes für die Textextraktion Unicode zu."
            }
          }
        },
        "extra": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfDictEntry"
          },
          "x-semio-ui": {
            "label": {
              "en": "Additional entries",
              "de": "Weitere Einträge"
            },
            "description": {
              "en": "Dictionary entries without a dedicated field, kept verbatim.",
              "de": "Wörterbucheinträge ohne eigenes Feld, unverändert erhalten."
            }
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
            "type": "object",
            "semioPrimitive": "binary64",
            "properties": {
              "bits": {
                "type": "string",
                "pattern": "^[0-9a-f]{16}$"
              }
            },
            "required": [
              "bits"
            ],
            "additionalProperties": false
          },
          "minItems": 4,
          "maxItems": 4
        },
        "italicAngle": {
          "type": "object",
          "semioPrimitive": "binary64",
          "properties": {
            "bits": {
              "type": "string",
              "pattern": "^[0-9a-f]{16}$"
            }
          },
          "required": [
            "bits"
          ],
          "additionalProperties": false
        },
        "ascent": {
          "type": "object",
          "semioPrimitive": "binary64",
          "properties": {
            "bits": {
              "type": "string",
              "pattern": "^[0-9a-f]{16}$"
            }
          },
          "required": [
            "bits"
          ],
          "additionalProperties": false
        },
        "descent": {
          "type": "object",
          "semioPrimitive": "binary64",
          "properties": {
            "bits": {
              "type": "string",
              "pattern": "^[0-9a-f]{16}$"
            }
          },
          "required": [
            "bits"
          ],
          "additionalProperties": false
        },
        "capHeight": {
          "type": "object",
          "semioPrimitive": "binary64",
          "properties": {
            "bits": {
              "type": "string",
              "pattern": "^[0-9a-f]{16}$"
            }
          },
          "required": [
            "bits"
          ],
          "additionalProperties": false
        },
        "stemV": {
          "type": "object",
          "semioPrimitive": "binary64",
          "properties": {
            "bits": {
              "type": "string",
              "pattern": "^[0-9a-f]{16}$"
            }
          },
          "required": [
            "bits"
          ],
          "additionalProperties": false
        },
        "stemH": {
          "anyOf": [
            {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            {
              "type": "null"
            }
          ]
        },
        "xHeight": {
          "anyOf": [
            {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            {
              "type": "null"
            }
          ]
        },
        "leading": {
          "anyOf": [
            {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            {
              "type": "null"
            }
          ]
        },
        "avgWidth": {
          "anyOf": [
            {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            {
              "type": "null"
            }
          ]
        },
        "maxWidth": {
          "anyOf": [
            {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            {
              "type": "null"
            }
          ]
        },
        "missingWidth": {
          "anyOf": [
            {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
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
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
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
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
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
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
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
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
              },
              "minItems": 6,
              "maxItems": 6
            },
            "fontBbox": {
              "type": "array",
              "items": {
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
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
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
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
          "minimum": 0,
          "x-semio-ui": {
            "widget": "stepper",
            "label": {
              "en": "Field flags",
              "de": "Feld-Flags"
            },
            "description": {
              "en": "Ff: bit set, e.g. 1 read-only, 2 required, 4 no export.",
              "de": "Ff: Bitmenge, z. B. 1 schreibgeschützt, 2 erforderlich, 4 nicht exportieren."
            }
          }
        },
        "alternateName": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "text",
            "label": {
              "en": "Tooltip",
              "de": "QuickInfo"
            },
            "description": {
              "en": "TU: user-facing field name shown as tooltip.",
              "de": "TU: für Benutzer sichtbarer Feldname, als QuickInfo angezeigt."
            }
          }
        },
        "mappingName": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "text",
            "label": {
              "en": "Export name",
              "de": "Exportname"
            },
            "description": {
              "en": "TM: field name used when exporting form data.",
              "de": "TM: Feldname beim Export der Formulardaten."
            }
          }
        },
        "defaultAppearance": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "text",
            "label": {
              "en": "Default appearance",
              "de": "Standard-Erscheinungsbild"
            },
            "description": {
              "en": "DA: content stream operators for variable text, e.g. /Helv 12 Tf 0 g.",
              "de": "DA: Inhaltsstrom-Operatoren für variablen Text, z. B. /Helv 12 Tf 0 g."
            }
          }
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
          ],
          "x-semio-ui": {
            "widget": "stepper",
            "label": {
              "en": "Text alignment",
              "de": "Textausrichtung"
            },
            "description": {
              "en": "Q: 0 left, 1 centered, 2 right.",
              "de": "Q: 0 links, 1 zentriert, 2 rechts."
            }
          }
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
          },
          "x-semio-ui": {
            "label": {
              "en": "Widget annotations",
              "de": "Widget-Anmerkungen"
            }
          }
        },
        "children": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfFormField"
          },
          "x-semio-ui": {
            "label": {
              "en": "Child fields",
              "de": "Untergeordnete Felder"
            }
          }
        },
        "additionalActions": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfDictEntry"
          },
          "x-semio-ui": {
            "label": {
              "en": "Additional actions",
              "de": "Zusätzliche Aktionen"
            },
            "description": {
              "en": "Trigger events (AA) and the actions they run.",
              "de": "Auslöseereignisse (AA) und die ausgeführten Aktionen."
            }
          }
        },
        "extra": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfDictEntry"
          },
          "x-semio-ui": {
            "label": {
              "en": "Additional entries",
              "de": "Weitere Einträge"
            },
            "description": {
              "en": "Dictionary entries without a dedicated field, kept verbatim.",
              "de": "Wörterbucheinträge ohne eigenes Feld, unverändert erhalten."
            }
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
            "type": "object",
            "semioPrimitive": "binary64",
            "properties": {
              "bits": {
                "type": "string",
                "pattern": "^[0-9a-f]{16}$"
              }
            },
            "required": [
              "bits"
            ],
            "additionalProperties": false
          },
          "minItems": 4,
          "maxItems": 4,
          "x-semio-ui": {
            "widget": "vector",
            "label": {
              "en": "Bounding box",
              "de": "Begrenzungsrahmen"
            },
            "description": {
              "en": "[llx lly urx ury] in the object's coordinate space.",
              "de": "[llx lly urx ury] im Koordinatenraum des Objekts."
            }
          }
        },
        "matrix": {
          "type": "array",
          "items": {
            "type": "object",
            "semioPrimitive": "binary64",
            "properties": {
              "bits": {
                "type": "string",
                "pattern": "^[0-9a-f]{16}$"
              }
            },
            "required": [
              "bits"
            ],
            "additionalProperties": false
          },
          "minItems": 6,
          "maxItems": 6,
          "x-semio-ui": {
            "label": {
              "en": "Form matrix",
              "de": "Formularmatrix"
            },
            "description": {
              "en": "Maps form space to user space [a b c d e f].",
              "de": "Bildet den Formularraum auf den Benutzerraum ab [a b c d e f]."
            }
          }
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
          ],
          "x-semio-ui": {
            "label": {
              "en": "Transparency group",
              "de": "Transparenzgruppe"
            }
          }
        },
        "optionalContent": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "text",
            "label": {
              "en": "Optional content",
              "de": "Optionaler Inhalt (Ebene)"
            },
            "description": {
              "en": "OC: layer or membership that controls visibility.",
              "de": "OC: Ebene oder Zugehörigkeit, die die Sichtbarkeit steuert."
            }
          }
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
          ],
          "x-semio-ui": {
            "widget": "stepper",
            "label": {
              "en": "Structure parent",
              "de": "Strukturelternschlüssel"
            },
            "description": {
              "en": "Key into the structural parent tree.",
              "de": "Schlüssel in den Strukturelternbaum."
            }
          }
        },
        "extra": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfDictEntry"
          },
          "x-semio-ui": {
            "label": {
              "en": "Additional entries",
              "de": "Weitere Einträge"
            },
            "description": {
              "en": "Dictionary entries without a dedicated field, kept verbatim.",
              "de": "Wörterbucheinträge ohne eigenes Feld, unverändert erhalten."
            }
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
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
              }
            },
            "range": {
              "type": "array",
              "items": {
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
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
                    "type": "object",
                    "semioPrimitive": "binary64",
                    "properties": {
                      "bits": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{16}$"
                      }
                    },
                    "required": [
                      "bits"
                    ],
                    "additionalProperties": false
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
                    "type": "object",
                    "semioPrimitive": "binary64",
                    "properties": {
                      "bits": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{16}$"
                      }
                    },
                    "required": [
                      "bits"
                    ],
                    "additionalProperties": false
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
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
              }
            },
            "range": {
              "anyOf": [
                {
                  "type": "array",
                  "items": {
                    "type": "object",
                    "semioPrimitive": "binary64",
                    "properties": {
                      "bits": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{16}$"
                      }
                    },
                    "required": [
                      "bits"
                    ],
                    "additionalProperties": false
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
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
              }
            },
            "c1": {
              "type": "array",
              "items": {
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
              }
            },
            "n": {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
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
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
              }
            },
            "range": {
              "anyOf": [
                {
                  "type": "array",
                  "items": {
                    "type": "object",
                    "semioPrimitive": "binary64",
                    "properties": {
                      "bits": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{16}$"
                      }
                    },
                    "required": [
                      "bits"
                    ],
                    "additionalProperties": false
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
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
              }
            },
            "encode": {
              "type": "array",
              "items": {
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
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
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
              }
            },
            "range": {
              "type": "array",
              "items": {
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
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
          ],
          "x-semio-ui": {
            "label": {
              "en": "Color space",
              "de": "Farbraum"
            }
          }
        },
        "bitsPerComponent": {
          "type": "integer",
          "minimum": 0,
          "x-semio-ui": {
            "widget": "stepper",
            "label": {
              "en": "Bits per component",
              "de": "Bits pro Komponente"
            },
            "unit": "bit"
          }
        },
        "imageMask": {
          "type": "boolean",
          "x-semio-ui": {
            "widget": "toggle",
            "label": {
              "en": "Image mask",
              "de": "Bildmaske"
            },
            "description": {
              "en": "The image is a 1-bit stencil mask painted with the current fill color.",
              "de": "Das Bild ist eine 1-Bit-Schablone, gemalt mit der aktuellen Füllfarbe."
            }
          }
        },
        "decode": {
          "type": "array",
          "items": {
            "type": "object",
            "semioPrimitive": "binary64",
            "properties": {
              "bits": {
                "type": "string",
                "pattern": "^[0-9a-f]{16}$"
              }
            },
            "required": [
              "bits"
            ],
            "additionalProperties": false
          },
          "x-semio-ui": {
            "label": {
              "en": "Decode array",
              "de": "Decode-Array"
            },
            "description": {
              "en": "Maps sample values to the color space range, one pair per component.",
              "de": "Bildet Abtastwerte auf den Farbraumbereich ab, ein Paar je Komponente."
            }
          }
        },
        "interpolate": {
          "type": "boolean",
          "x-semio-ui": {
            "widget": "toggle",
            "label": {
              "en": "Interpolate",
              "de": "Interpolieren"
            }
          }
        },
        "codec": {
          "$ref": "#/$defs/PdfImageCodec",
          "x-semio-ui": {
            "label": {
              "en": "Image encoding",
              "de": "Bildkodierung"
            },
            "description": {
              "en": "Filter and parameters the image data is stored with.",
              "de": "Filter und Parameter, mit denen die Bilddaten gespeichert sind."
            }
          }
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
          ],
          "x-semio-ui": {
            "widget": "text",
            "label": {
              "en": "Soft mask",
              "de": "Weiche Maske"
            }
          }
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
          ],
          "x-semio-ui": {
            "widget": "stepper",
            "label": {
              "en": "Soft mask in data",
              "de": "Weiche Maske im Datenstrom"
            },
            "description": {
              "en": "SMaskInData for JPX images: 0 ignore, 1 alpha, 2 pre-blended alpha.",
              "de": "SMaskInData für JPX-Bilder: 0 ignorieren, 1 Alpha, 2 vorgemischtes Alpha."
            }
          }
        },
        "mask": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfImageMask"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "label": {
              "en": "Mask",
              "de": "Maske"
            }
          }
        },
        "matte": {
          "anyOf": [
            {
              "type": "array",
              "items": {
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
              }
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "label": {
              "en": "Matte color",
              "de": "Matte-Farbe"
            },
            "description": {
              "en": "Color the soft-mask image was pre-blended with.",
              "de": "Farbe, mit der das Bild der weichen Maske vorgemischt wurde."
            }
          }
        },
        "intent": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "text",
            "label": {
              "en": "Rendering intent",
              "de": "Rendering Intent"
            }
          }
        },
        "optionalContent": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "text",
            "label": {
              "en": "Optional content",
              "de": "Optionaler Inhalt (Ebene)"
            },
            "description": {
              "en": "OC: layer or membership that controls visibility.",
              "de": "OC: Ebene oder Zugehörigkeit, die die Sichtbarkeit steuert."
            }
          }
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
          ],
          "x-semio-ui": {
            "widget": "stepper",
            "label": {
              "en": "Structure parent",
              "de": "Strukturelternschlüssel"
            },
            "description": {
              "en": "Key into the structural parent tree.",
              "de": "Schlüssel in den Strukturelternbaum."
            }
          }
        },
        "extra": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfDictEntry"
          },
          "x-semio-ui": {
            "label": {
              "en": "Additional entries",
              "de": "Weitere Einträge"
            },
            "description": {
              "en": "Dictionary entries without a dedicated field, kept verbatim.",
              "de": "Wörterbucheinträge ohne eigenes Feld, unverändert erhalten."
            }
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
          ],
          "x-semio-ui": {
            "widget": "text",
            "label": {
              "en": "Subject",
              "de": "Thema"
            }
          }
        },
        "keywords": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "text",
            "label": {
              "en": "Keywords",
              "de": "Stichwörter"
            }
          }
        },
        "creator": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "text",
            "label": {
              "en": "Creator application",
              "de": "Erstellt mit"
            },
            "description": {
              "en": "Application that created the original document.",
              "de": "Anwendung, die das Originaldokument erstellt hat."
            }
          }
        },
        "producer": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "text",
            "label": {
              "en": "PDF producer",
              "de": "PDF erstellt mit"
            },
            "description": {
              "en": "Application that converted the document to PDF.",
              "de": "Anwendung, die das Dokument in PDF umgewandelt hat."
            }
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
          ],
          "x-semio-ui": {
            "label": {
              "en": "Creation date",
              "de": "Erstellungsdatum"
            }
          }
        },
        "modificationDate": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfDate"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "label": {
              "en": "Modification date",
              "de": "Änderungsdatum"
            }
          }
        },
        "trapped": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "text",
            "label": {
              "en": "Trapped",
              "de": "Überfüllt"
            },
            "description": {
              "en": "True, False or Unknown.",
              "de": "True, False oder Unknown."
            }
          }
        },
        "extra": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfDictEntry"
          },
          "x-semio-ui": {
            "label": {
              "en": "Additional entries",
              "de": "Weitere Einträge"
            },
            "description": {
              "en": "Dictionary entries without a dedicated field, kept verbatim.",
              "de": "Wörterbucheinträge ohne eigenes Feld, unverändert erhalten."
            }
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
            "type": "object",
            "semioPrimitive": "binary64",
            "properties": {
              "bits": {
                "type": "string",
                "pattern": "^[0-9a-f]{16}$"
              }
            },
            "required": [
              "bits"
            ],
            "additionalProperties": false
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
          "type": "boolean",
          "x-semio-ui": {
            "widget": "toggle",
            "label": {
              "en": "Tagged PDF",
              "de": "Getaggtes PDF"
            },
            "description": {
              "en": "The document conforms to the Tagged PDF conventions.",
              "de": "Das Dokument folgt den Konventionen für getaggtes PDF."
            }
          }
        },
        "userProperties": {
          "type": "boolean",
          "x-semio-ui": {
            "widget": "toggle",
            "label": {
              "en": "User properties",
              "de": "Benutzereigenschaften"
            }
          }
        },
        "suspects": {
          "type": "boolean",
          "x-semio-ui": {
            "widget": "toggle",
            "label": {
              "en": "Suspects",
              "de": "Verdächtige Tags"
            },
            "description": {
              "en": "The tag structure may be inaccurate.",
              "de": "Die Tag-Struktur ist möglicherweise ungenau."
            }
          }
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
              "minimum": 0,
              "semioPrimitive": "u64",
              "maximum": 18446744073709551615
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "stepper",
            "label": {
              "en": "Pop-up",
              "de": "Popup"
            },
            "description": {
              "en": "Object number of the associated pop-up annotation.",
              "de": "Objektnummer der zugehörigen Popup-Anmerkung."
            }
          }
        },
        "opacity": {
          "anyOf": [
            {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
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
          ],
          "x-semio-ui": {
            "widget": "multiline",
            "label": {
              "en": "Rich text",
              "de": "Formatierter Text"
            },
            "description": {
              "en": "RC: XHTML formatted contents.",
              "de": "RC: Inhalt als formatiertes XHTML."
            }
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
          ],
          "x-semio-ui": {
            "label": {
              "en": "Creation date",
              "de": "Erstellungsdatum"
            }
          }
        },
        "inReplyTo": {
          "anyOf": [
            {
              "type": "integer",
              "minimum": 0,
              "semioPrimitive": "u64",
              "maximum": 18446744073709551615
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "stepper",
            "label": {
              "en": "In reply to",
              "de": "Antwort auf"
            },
            "description": {
              "en": "Object number of the annotation this one replies to.",
              "de": "Objektnummer der Anmerkung, auf die diese antwortet."
            }
          }
        },
        "subject": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "text",
            "label": {
              "en": "Subject",
              "de": "Betreff"
            }
          }
        },
        "replyType": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "text",
            "label": {
              "en": "Reply type",
              "de": "Antworttyp"
            },
            "description": {
              "en": "R (reply) or Group.",
              "de": "R (Antwort) oder Group."
            }
          }
        },
        "intent": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "text",
            "label": {
              "en": "Intent",
              "de": "Zweck"
            },
            "description": {
              "en": "IT: purpose of the markup, e.g. FreeTextCallout.",
              "de": "IT: Zweck der Markierung, z. B. FreeTextCallout."
            }
          }
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
          "$ref": "#/$defs/PdfColorSpace",
          "x-semio-ui": {
            "label": {
              "en": "Color space",
              "de": "Farbraum"
            }
          }
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
          "$ref": "#/$defs/PdfDestination",
          "x-semio-ui": {
            "label": {
              "en": "Destination",
              "de": "Ziel"
            }
          }
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
          },
          "x-semio-ui": {
            "label": {
              "en": "Entries",
              "de": "Einträge"
            }
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
              "type": "integer",
              "semioPrimitive": "i64",
              "minimum": -9223372036854775808,
              "maximum": 9223372036854775807
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
        },
        {
          "type": "object",
          "additionalProperties": false,
          "required": [
            "kind",
            "value"
          ],
          "properties": {
            "kind": {
              "const": "text"
            },
            "value": {
              "type": "string"
            }
          }
        },
        {
          "type": "object",
          "additionalProperties": false,
          "required": [
            "kind",
            "value"
          ],
          "properties": {
            "kind": {
              "const": "date"
            },
            "value": {
              "$ref": "#/$defs/PdfDate"
            }
          }
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
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
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
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
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
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
              }
            },
            "phase": {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
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
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
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
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
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
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            "y": {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
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
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            "y": {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
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
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            "y1": {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            "x2": {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            "y2": {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            "x3": {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            "y3": {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
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
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            "y2": {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            "x3": {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            "y3": {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
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
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            "y1": {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            "x3": {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            "y3": {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
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
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            "y": {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            "width": {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            "height": {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
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
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
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
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
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
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
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
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
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
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
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
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
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
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            "ty": {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
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
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            "ty": {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
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
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
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
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            "charSpacing": {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
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
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            "wy": {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
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
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            "wy": {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            "llx": {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            "lly": {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            "urx": {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            "ury": {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
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
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
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
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
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
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
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
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
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
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
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
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
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
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            "g": {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            "b": {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
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
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            "g": {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            "b": {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
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
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            "m": {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            "y": {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            "k": {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
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
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            "m": {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            "y": {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            "k": {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
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
          },
          "x-semio-ui": {
            "label": {
              "en": "Optional content groups",
              "de": "Ebenen (optionale Inhalte)"
            }
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
          "type": "boolean",
          "x-semio-ui": {
            "widget": "toggle",
            "label": {
              "en": "Base state off",
              "de": "Grundzustand aus"
            },
            "description": {
              "en": "Groups start hidden unless listed as on.",
              "de": "Ebenen sind ausgeblendet, sofern nicht als eingeblendet aufgeführt."
            }
          }
        },
        "on": {
          "type": "array",
          "items": {
            "type": "string"
          },
          "x-semio-ui": {
            "label": {
              "en": "Visible groups",
              "de": "Eingeblendete Ebenen"
            }
          }
        },
        "off": {
          "type": "array",
          "items": {
            "type": "string"
          },
          "x-semio-ui": {
            "label": {
              "en": "Hidden groups",
              "de": "Ausgeblendete Ebenen"
            }
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
          },
          "x-semio-ui": {
            "label": {
              "en": "Additional entries",
              "de": "Weitere Einträge"
            },
            "description": {
              "en": "Dictionary entries without a dedicated field, kept verbatim.",
              "de": "Wörterbucheinträge ohne eigenes Feld, unverändert erhalten."
            }
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
          },
          "x-semio-ui": {
            "label": {
              "en": "Intent",
              "de": "Zweck"
            },
            "description": {
              "en": "View and/or Design.",
              "de": "View und/oder Design."
            }
          }
        },
        "usage": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfDictEntry"
          },
          "x-semio-ui": {
            "label": {
              "en": "Usage",
              "de": "Verwendung"
            }
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
          ],
          "x-semio-ui": {
            "label": {
              "en": "Destination",
              "de": "Ziel"
            }
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
        "color": {
          "anyOf": [
            {
              "type": "array",
              "items": {
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
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
          "type": "boolean",
          "x-semio-ui": {
            "widget": "toggle",
            "label": {
              "en": "Italic",
              "de": "Kursiv"
            }
          }
        },
        "bold": {
          "type": "boolean",
          "x-semio-ui": {
            "widget": "toggle",
            "label": {
              "en": "Bold",
              "de": "Fett"
            }
          }
        },
        "open": {
          "type": "boolean",
          "x-semio-ui": {
            "widget": "toggle",
            "label": {
              "en": "Expanded",
              "de": "Aufgeklappt"
            }
          }
        },
        "children": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfOutlineItem"
          },
          "x-semio-ui": {
            "label": {
              "en": "Child bookmarks",
              "de": "Untergeordnete Lesezeichen"
            }
          }
        },
        "extra": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfDictEntry"
          },
          "x-semio-ui": {
            "label": {
              "en": "Additional entries",
              "de": "Weitere Einträge"
            },
            "description": {
              "en": "Dictionary entries without a dedicated field, kept verbatim.",
              "de": "Wörterbucheinträge ohne eigenes Feld, unverändert erhalten."
            }
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
          "type": "string",
          "x-semio-ui": {
            "widget": "text",
            "label": {
              "en": "Output condition identifier",
              "de": "Kennung der Ausgabebedingung"
            }
          }
        },
        "condition": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "text",
            "label": {
              "en": "Output condition",
              "de": "Ausgabebedingung"
            },
            "description": {
              "en": "Human-readable output condition, e.g. FOGRA39.",
              "de": "Lesbare Ausgabebedingung, z. B. FOGRA39."
            }
          }
        },
        "registryName": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "text",
            "label": {
              "en": "Registry name",
              "de": "Registrierungsname"
            },
            "description": {
              "en": "Registry of the output condition, e.g. http://www.color.org.",
              "de": "Register der Ausgabebedingung, z. B. http://www.color.org."
            }
          }
        },
        "info": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "text",
            "label": {
              "en": "Additional information",
              "de": "Zusatzinformationen"
            }
          }
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
          ],
          "x-semio-ui": {
            "label": {
              "en": "ICC output profile",
              "de": "ICC-Ausgabeprofil"
            },
            "description": {
              "en": "DestOutputProfile bytes.",
              "de": "Bytes des DestOutputProfile."
            }
          }
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
            "type": "object",
            "semioPrimitive": "binary64",
            "properties": {
              "bits": {
                "type": "string",
                "pattern": "^[0-9a-f]{16}$"
              }
            },
            "required": [
              "bits"
            ],
            "additionalProperties": false
          },
          "minItems": 4,
          "maxItems": 4,
          "x-semio-ui": {
            "widget": "vector",
            "label": {
              "en": "Media box",
              "de": "Medienrahmen (MediaBox)"
            },
            "description": {
              "en": "Physical medium boundaries as [llx lly urx ury].",
              "de": "Grenzen des physischen Mediums als [llx lly urx ury]."
            },
            "unit": "pt"
          }
        },
        "cropBox": {
          "anyOf": [
            {
              "type": "array",
              "items": {
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
              },
              "minItems": 4,
              "maxItems": 4
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "vector",
            "label": {
              "en": "Crop box",
              "de": "Maskenrahmen (CropBox)"
            },
            "description": {
              "en": "Visible region of the page as [llx lly urx ury].",
              "de": "Sichtbarer Bereich der Seite als [llx lly urx ury]."
            },
            "unit": "pt"
          }
        },
        "bleedBox": {
          "anyOf": [
            {
              "type": "array",
              "items": {
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
              },
              "minItems": 4,
              "maxItems": 4
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "vector",
            "label": {
              "en": "Bleed box",
              "de": "Anschnittrahmen (BleedBox)"
            },
            "description": {
              "en": "Clip region for production output as [llx lly urx ury].",
              "de": "Beschnittbereich für die Druckausgabe als [llx lly urx ury]."
            },
            "unit": "pt"
          }
        },
        "trimBox": {
          "anyOf": [
            {
              "type": "array",
              "items": {
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
              },
              "minItems": 4,
              "maxItems": 4
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "vector",
            "label": {
              "en": "Trim box",
              "de": "Endformatrahmen (TrimBox)"
            },
            "description": {
              "en": "Finished page size after trimming as [llx lly urx ury].",
              "de": "Endformat der Seite nach dem Beschnitt als [llx lly urx ury]."
            },
            "unit": "pt"
          }
        },
        "artBox": {
          "anyOf": [
            {
              "type": "array",
              "items": {
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
              },
              "minItems": 4,
              "maxItems": 4
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "vector",
            "label": {
              "en": "Art box",
              "de": "Objektrahmen (ArtBox)"
            },
            "description": {
              "en": "Meaningful page content as [llx lly urx ury].",
              "de": "Sinnvoller Seiteninhalt als [llx lly urx ury]."
            },
            "unit": "pt"
          }
        },
        "rotate": {
          "type": "integer",
          "x-semio-ui": {
            "widget": "stepper",
            "label": {
              "en": "Rotation",
              "de": "Drehung"
            },
            "description": {
              "en": "Clockwise page rotation in multiples of 90°.",
              "de": "Seitendrehung im Uhrzeigersinn in Vielfachen von 90°."
            },
            "unit": "deg",
            "step": 90
          }
        },
        "userUnit": {
          "anyOf": [
            {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "stepper",
            "label": {
              "en": "User unit",
              "de": "Benutzereinheit"
            },
            "description": {
              "en": "Size of one default user space unit in multiples of 1/72 inch.",
              "de": "Größe einer Einheit des Standard-Benutzerraums in Vielfachen von 1/72 Zoll."
            }
          }
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
          },
          "x-semio-ui": {
            "label": {
              "en": "Annotations",
              "de": "Anmerkungen"
            }
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
          ],
          "x-semio-ui": {
            "label": {
              "en": "Transparency group",
              "de": "Transparenzgruppe"
            }
          }
        },
        "thumbnail": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "text",
            "label": {
              "en": "Thumbnail",
              "de": "Seitenminiatur"
            }
          }
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
          ],
          "x-semio-ui": {
            "widget": "stepper",
            "label": {
              "en": "Structure parents",
              "de": "Strukturelternschlüssel"
            },
            "description": {
              "en": "Key into the structural parent tree for the page's content.",
              "de": "Schlüssel in den Strukturelternbaum für den Seiteninhalt."
            }
          }
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
          ],
          "x-semio-ui": {
            "label": {
              "en": "Page transition",
              "de": "Seitenübergang"
            }
          }
        },
        "duration": {
          "anyOf": [
            {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "stepper",
            "label": {
              "en": "Display duration",
              "de": "Anzeigedauer"
            },
            "description": {
              "en": "Seconds the page is shown during a presentation.",
              "de": "Sekunden, die die Seite in einer Präsentation angezeigt wird."
            },
            "unit": "s"
          }
        },
        "metadata": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "text",
            "label": {
              "en": "Metadata stream",
              "de": "Metadatenstrom"
            }
          }
        },
        "additionalActions": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfDictEntry"
          },
          "x-semio-ui": {
            "label": {
              "en": "Additional actions",
              "de": "Zusätzliche Aktionen"
            },
            "description": {
              "en": "Trigger events (AA) and the actions they run.",
              "de": "Auslöseereignisse (AA) und die ausgeführten Aktionen."
            }
          }
        },
        "extra": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfDictEntry"
          },
          "x-semio-ui": {
            "label": {
              "en": "Additional entries",
              "de": "Weitere Einträge"
            },
            "description": {
              "en": "Dictionary entries without a dedicated field, kept verbatim.",
              "de": "Wörterbucheinträge ohne eigenes Feld, unverändert erhalten."
            }
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
          "minimum": 0,
          "x-semio-ui": {
            "widget": "stepper",
            "label": {
              "en": "First page",
              "de": "Erste Seite"
            },
            "description": {
              "en": "Zero-based page index where the range starts.",
              "de": "Nullbasierter Seitenindex, an dem der Bereich beginnt."
            }
          }
        },
        "style": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfPageLabelStyle"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "options": {
              "decimal": {
                "en": "Arabic numerals (1, 2, 3)",
                "de": "Arabische Ziffern (1, 2, 3)"
              },
              "romanUpper": {
                "en": "Uppercase Roman (I, II, III)",
                "de": "Römisch, groß (I, II, III)"
              },
              "romanLower": {
                "en": "Lowercase Roman (i, ii, iii)",
                "de": "Römisch, klein (i, ii, iii)"
              },
              "lettersUpper": {
                "en": "Uppercase letters (A, B, C)",
                "de": "Großbuchstaben (A, B, C)"
              },
              "lettersLower": {
                "en": "Lowercase letters (a, b, c)",
                "de": "Kleinbuchstaben (a, b, c)"
              }
            }
          }
        },
        "prefix": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "text",
            "label": {
              "en": "Label prefix",
              "de": "Präfix der Beschriftung"
            }
          }
        },
        "start": {
          "type": "integer",
          "minimum": 0,
          "x-semio-ui": {
            "widget": "stepper",
            "label": {
              "en": "First number",
              "de": "Startnummer"
            },
            "description": {
              "en": "Numeric value of the first page label in the range.",
              "de": "Zahlenwert der ersten Seitenbeschriftung im Bereich."
            }
          }
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
            "type": "object",
            "semioPrimitive": "binary64",
            "properties": {
              "bits": {
                "type": "string",
                "pattern": "^[0-9a-f]{16}$"
              }
            },
            "required": [
              "bits"
            ],
            "additionalProperties": false
          },
          "minItems": 6,
          "maxItems": 6,
          "x-semio-ui": {
            "label": {
              "en": "Pattern matrix",
              "de": "Mustermatrix"
            },
            "description": {
              "en": "Maps pattern space to the default space of the parent [a b c d e f].",
              "de": "Bildet den Musterraum auf den Standardraum des Elternobjekts ab [a b c d e f]."
            }
          }
        },
        "kind": {
          "$ref": "#/$defs/PdfPatternKind"
        },
        "extra": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfDictEntry"
          },
          "x-semio-ui": {
            "label": {
              "en": "Additional entries",
              "de": "Weitere Einträge"
            },
            "description": {
              "en": "Dictionary entries without a dedicated field, kept verbatim.",
              "de": "Wörterbucheinträge ohne eigenes Feld, unverändert erhalten."
            }
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
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
              },
              "minItems": 4,
              "maxItems": 4
            },
            "xStep": {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
            },
            "yStep": {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
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
          "$ref": "#/$defs/PdfColorSpace",
          "x-semio-ui": {
            "label": {
              "en": "Color space",
              "de": "Farbraum"
            }
          }
        },
        "kind": {
          "$ref": "#/$defs/PdfShadingKind"
        },
        "background": {
          "anyOf": [
            {
              "type": "array",
              "items": {
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
              }
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "label": {
              "en": "Background color",
              "de": "Hintergrundfarbe"
            },
            "description": {
              "en": "Color components in the shading's color space.",
              "de": "Farbkomponenten im Farbraum des Verlaufs."
            }
          }
        },
        "bbox": {
          "anyOf": [
            {
              "type": "array",
              "items": {
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
              },
              "minItems": 4,
              "maxItems": 4
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "vector",
            "label": {
              "en": "Bounding box",
              "de": "Begrenzungsrahmen"
            },
            "description": {
              "en": "[llx lly urx ury] in the object's coordinate space.",
              "de": "[llx lly urx ury] im Koordinatenraum des Objekts."
            }
          }
        },
        "antiAlias": {
          "type": "boolean",
          "x-semio-ui": {
            "widget": "toggle",
            "label": {
              "en": "Anti-aliasing",
              "de": "Kantenglättung"
            }
          }
        },
        "extra": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfDictEntry"
          },
          "x-semio-ui": {
            "label": {
              "en": "Additional entries",
              "de": "Weitere Einträge"
            },
            "description": {
              "en": "Dictionary entries without a dedicated field, kept verbatim.",
              "de": "Wörterbucheinträge ohne eigenes Feld, unverändert erhalten."
            }
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
                    "type": "object",
                    "semioPrimitive": "binary64",
                    "properties": {
                      "bits": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{16}$"
                      }
                    },
                    "required": [
                      "bits"
                    ],
                    "additionalProperties": false
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
                    "type": "object",
                    "semioPrimitive": "binary64",
                    "properties": {
                      "bits": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{16}$"
                      }
                    },
                    "required": [
                      "bits"
                    ],
                    "additionalProperties": false
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
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
              },
              "minItems": 4,
              "maxItems": 4
            },
            "domain": {
              "anyOf": [
                {
                  "type": "array",
                  "items": {
                    "type": "object",
                    "semioPrimitive": "binary64",
                    "properties": {
                      "bits": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{16}$"
                      }
                    },
                    "required": [
                      "bits"
                    ],
                    "additionalProperties": false
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
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
              },
              "minItems": 6,
              "maxItems": 6
            },
            "domain": {
              "anyOf": [
                {
                  "type": "array",
                  "items": {
                    "type": "object",
                    "semioPrimitive": "binary64",
                    "properties": {
                      "bits": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{16}$"
                      }
                    },
                    "required": [
                      "bits"
                    ],
                    "additionalProperties": false
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
                "type": "object",
                "semioPrimitive": "binary64",
                "properties": {
                  "bits": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "bits"
                ],
                "additionalProperties": false
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
                    "type": "object",
                    "semioPrimitive": "binary64",
                    "properties": {
                      "bits": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{16}$"
                      }
                    },
                    "required": [
                      "bits"
                    ],
                    "additionalProperties": false
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
            "codes": {
              "type": "array",
              "items": {
                "type": "integer",
                "minimum": 0,
                "maximum": 4294967295
              }
            }
          },
          "required": [
            "kind",
            "codes"
          ]
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "adjust"
            },
            "amount": {
              "type": "object",
              "semioPrimitive": "binary64",
              "properties": {
                "bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{16}$"
                }
              },
              "required": [
                "bits"
              ],
              "additionalProperties": false
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
            "codes": {
              "type": "array",
              "items": {
                "type": "integer",
                "minimum": 0,
                "maximum": 4294967295
              }
            }
          },
          "required": [
            "kind",
            "codes"
          ]
        }
      ]
    },
    "PdfToUnicode": {
      "type": "object",
      "properties": {
        "byteWidth": {
          "type": "integer",
          "minimum": 0,
          "x-semio-ui": {
            "widget": "stepper",
            "label": {
              "en": "Code width",
              "de": "Codebreite"
            },
            "description": {
              "en": "Bytes per character code in the CMap.",
              "de": "Bytes je Zeichencode in der CMap."
            },
            "unit": "B"
          }
        },
        "mappings": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfToUnicodeMapping"
          },
          "x-semio-ui": {
            "label": {
              "en": "Character mappings",
              "de": "Zeichenzuordnungen"
            }
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
          ],
          "x-semio-ui": {
            "label": {
              "en": "Color space",
              "de": "Farbraum"
            }
          }
        },
        "isolated": {
          "type": "boolean",
          "x-semio-ui": {
            "widget": "toggle",
            "label": {
              "en": "Isolated group",
              "de": "Isolierte Gruppe"
            }
          }
        },
        "knockout": {
          "type": "boolean",
          "x-semio-ui": {
            "widget": "toggle",
            "label": {
              "en": "Knockout group",
              "de": "Aussparungsgruppe"
            }
          }
        }
      }
    },
    "PdfViewerPreferences": {
      "type": "object",
      "properties": {
        "hideToolbar": {
          "type": "boolean",
          "x-semio-ui": {
            "widget": "toggle",
            "label": {
              "en": "Hide toolbars",
              "de": "Werkzeugleisten ausblenden"
            }
          }
        },
        "hideMenubar": {
          "type": "boolean",
          "x-semio-ui": {
            "widget": "toggle",
            "label": {
              "en": "Hide menu bar",
              "de": "Menüleiste ausblenden"
            }
          }
        },
        "hideWindowUi": {
          "type": "boolean",
          "x-semio-ui": {
            "widget": "toggle",
            "label": {
              "en": "Hide window controls",
              "de": "Fenstersteuerelemente ausblenden"
            }
          }
        },
        "fitWindow": {
          "type": "boolean",
          "x-semio-ui": {
            "widget": "toggle",
            "label": {
              "en": "Fit window",
              "de": "Fenster an Seite anpassen"
            }
          }
        },
        "centerWindow": {
          "type": "boolean",
          "x-semio-ui": {
            "widget": "toggle",
            "label": {
              "en": "Center window",
              "de": "Fenster zentrieren"
            }
          }
        },
        "displayDocTitle": {
          "type": "boolean",
          "x-semio-ui": {
            "widget": "toggle",
            "label": {
              "en": "Display document title",
              "de": "Dokumenttitel anzeigen"
            }
          }
        },
        "nonFullScreenPageMode": {
          "anyOf": [
            {
              "$ref": "#/$defs/PdfPageMode"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "select",
            "label": {
              "en": "Page mode after full screen",
              "de": "Seitenmodus nach Vollbild"
            },
            "options": {
              "useNone": {
                "en": "Page only",
                "de": "Nur Seite"
              },
              "useOutlines": {
                "en": "Bookmarks panel",
                "de": "Lesezeichenfenster"
              },
              "useThumbs": {
                "en": "Page thumbnails",
                "de": "Seitenminiaturen"
              },
              "fullScreen": {
                "en": "Full screen",
                "de": "Vollbild"
              },
              "useOc": {
                "en": "Layers panel",
                "de": "Ebenenfenster"
              },
              "useAttachments": {
                "en": "Attachments panel",
                "de": "Anlagenfenster"
              }
            }
          }
        },
        "direction": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "text",
            "label": {
              "en": "Reading direction",
              "de": "Leserichtung"
            },
            "description": {
              "en": "L2R or R2L.",
              "de": "L2R oder R2L."
            }
          }
        },
        "viewArea": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "text",
            "label": {
              "en": "View area",
              "de": "Anzeigebereich"
            },
            "description": {
              "en": "Page boundary box used for display.",
              "de": "Seitenrahmen für die Anzeige."
            }
          }
        },
        "viewClip": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "text",
            "label": {
              "en": "View clip",
              "de": "Anzeigebeschnitt"
            },
            "description": {
              "en": "Page boundary box displayed content is clipped to.",
              "de": "Seitenrahmen, auf den angezeigter Inhalt beschnitten wird."
            }
          }
        },
        "printArea": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "text",
            "label": {
              "en": "Print area",
              "de": "Druckbereich"
            },
            "description": {
              "en": "Page boundary box used when printing.",
              "de": "Seitenrahmen, der beim Drucken verwendet wird."
            }
          }
        },
        "printClip": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "text",
            "label": {
              "en": "Print clip",
              "de": "Druckbeschnitt"
            },
            "description": {
              "en": "Page boundary box printed content is clipped to.",
              "de": "Seitenrahmen, auf den gedruckter Inhalt beschnitten wird."
            }
          }
        },
        "printScaling": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "text",
            "label": {
              "en": "Print scaling",
              "de": "Druckskalierung"
            },
            "description": {
              "en": "None or AppDefault.",
              "de": "None oder AppDefault."
            }
          }
        },
        "duplex": {
          "anyOf": [
            {
              "type": "string"
            },
            {
              "type": "null"
            }
          ],
          "x-semio-ui": {
            "widget": "text",
            "label": {
              "en": "Duplex mode",
              "de": "Duplexmodus"
            },
            "description": {
              "en": "Simplex, DuplexFlipShortEdge or DuplexFlipLongEdge.",
              "de": "Simplex, DuplexFlipShortEdge oder DuplexFlipLongEdge."
            }
          }
        },
        "pickTrayByPdfSize": {
          "type": "boolean",
          "x-semio-ui": {
            "widget": "toggle",
            "label": {
              "en": "Choose paper source by PDF page size",
              "de": "Papierquelle nach PDF-Seitengröße wählen"
            }
          }
        },
        "printPageRange": {
          "type": "array",
          "items": {
            "type": "integer",
            "minimum": 0
          },
          "x-semio-ui": {
            "label": {
              "en": "Print page ranges",
              "de": "Druckseitenbereiche"
            },
            "description": {
              "en": "Pairs of first and last page for the print dialog.",
              "de": "Paare aus erster und letzter Seite für den Druckdialog."
            }
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
          ],
          "x-semio-ui": {
            "widget": "stepper",
            "label": {
              "en": "Number of copies",
              "de": "Anzahl der Exemplare"
            }
          }
        },
        "extra": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/PdfDictEntry"
          },
          "x-semio-ui": {
            "label": {
              "en": "Additional entries",
              "de": "Weitere Einträge"
            },
            "description": {
              "en": "Dictionary entries without a dedicated field, kept verbatim.",
              "de": "Wörterbucheinträge ohne eigenes Feld, unverändert erhalten."
            }
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
  if (schema.semioPrimitive === "binary64") { try { parseBinary64(value); return true; } catch { return (errors.push(`${at}: expected owned binary64`), false); } }
  if (schema.semioPrimitive === "i64" || schema.semioPrimitive === "u64") return typeof value === "bigint" && value >= (schema.semioPrimitive === "i64" ? -9223372036854775808n : 0n) && value <= (schema.semioPrimitive === "i64" ? 9223372036854775807n : 18446744073709551615n) || (errors.push(`${at}: expected owned integer word`), false);
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
export const parsePdfSnapshot = (value: unknown): PdfSnapshot => validateAgainst<PdfSnapshot>(schema, "", value);
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
export const parsePdfImageCodec = (value: unknown): PdfImageCodec => validateAgainst<PdfImageCodec>(schema, "/$defs/PdfImageCodec", value);
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
