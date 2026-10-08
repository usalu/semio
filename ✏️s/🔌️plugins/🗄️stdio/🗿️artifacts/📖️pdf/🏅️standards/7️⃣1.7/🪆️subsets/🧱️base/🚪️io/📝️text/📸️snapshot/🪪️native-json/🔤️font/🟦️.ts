import {parseArtifactRef} from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🗿️artifact-reference/🟦️.ts";
/** 🔤️ Explicit native font JSON admission into the same owned PDF relational domain. */
import type { PdfFont, PdfFontKind, PdfFontProgram, PdfFontDescriptor, PdfCidFont, PdfCidWidthRun, PdfCidVerticalRun, PdfSimpleEncoding, PdfBaseEncoding, PdfCMap, PdfToUnicode, PdfToUnicodeMapping, PdfCidMapping, PdfCidToGid, PdfCharProc } from "../../../../../🧬️schema/📸️snapshot/🟦️.ts";
import { record, array, text, boolean, integer, integers, real, reals, three, four, pdfDictionaryFromNativeJson } from "../🟦️.ts";
import { pdfMatrixFromNativeJson, pdfOperationFromNativeJson } from "../🖋️content/🟦️.ts";

/** 📏️ Admit every descriptor field and default as an owned binary64 scalar. */
export function pdfDescriptorFromNativeJson(input: unknown): PdfFontDescriptor {
  const row=record(input);const optional=(value:unknown)=>value==null?null:real(value);const optionalText=(value:unknown)=>value==null?null:text(value);
  return {fontName:text(row.fontName),flags:integer(row.flags??0),fontBbox:four(row.fontBbox??[0,0,0,0]),italicAngle:real(row.italicAngle??0),ascent:real(row.ascent??0),descent:real(row.descent??0),capHeight:real(row.capHeight??0),stemV:real(row.stemV??0),stemH:optional(row.stemH),xHeight:optional(row.xHeight),leading:optional(row.leading),avgWidth:optional(row.avgWidth),maxWidth:optional(row.maxWidth),missingWidth:optional(row.missingWidth),fontFamily:optionalText(row.fontFamily),fontStretch:optionalText(row.fontStretch),fontWeight:optional(row.fontWeight),charSet:optionalText(row.charSet),extra:pdfDictionaryFromNativeJson(row.extra??[])};
}
/** 🅰️ Admit every native font program and its intrinsic bytes. */
export function pdfProgramFromNativeJson(input:unknown):PdfFontProgram{const row=record(input);const kind=text(row.kind);const reference=parseArtifactRef(row.reference);switch(kind){case "type1":case "trueType":case "cff":case "cidCff":case "openType":return {kind,reference};default:throw new Error("Unknown PDF font program");}}
/** 🔡️ Admit optional base encodings and ordered differences. */
export function pdfEncodingFromNativeJson(input: unknown): PdfSimpleEncoding { const row=record(input);const value=row.base;let base:PdfBaseEncoding|null;switch(value){case null:case undefined:base=null;break;case "standard":case "winAnsi":case "macRoman":case "macExpert":base=value;break;default:throw new Error("Unknown PDF encoding");}return {base,differences:array(row.differences??[]).map(input=>{const row=record(input);return {code:integer(row.code),glyph:text(row.glyph)};})}; }
/** 🈴️ Admit ordered Unicode char and range mappings. */
export function pdfUnicodeMappingFromNativeJson(input: unknown): PdfToUnicodeMapping { const row=record(input);const kind=text(row.kind);switch(kind){case "char":return {kind,code:integer(row.code),text:text(row.text)};case "range":return {kind,low:integer(row.low),high:integer(row.high),text:text(row.text)};default:throw new Error("Unknown PDF Unicode mapping");} }
/** 🈴️ Admit native Unicode mapping ownership. */
export function pdfUnicodeFromNativeJson(input: unknown): PdfToUnicode { const row=record(input);return {byteWidth:integer(row.byteWidth),mappings:array(row.mappings??[]).map(pdfUnicodeMappingFromNativeJson)}; }
/** 🗺️ Admit ordered CID char and range mappings. */
export function pdfCidMappingFromNativeJson(input: unknown): PdfCidMapping { const row=record(input);const kind=text(row.kind);switch(kind){case "char":return {kind,code:integer(row.code),cid:integer(row.cid)};case "range":return {kind,low:integer(row.low),high:integer(row.high),cid:integer(row.cid)};default:throw new Error("Unknown PDF CID mapping");} }
/** 🗺️ Admit predefined and embedded native CMaps. */
export function pdfCMapFromNativeJson(input: unknown): PdfCMap {const row=record(input);const kind=text(row.kind);switch(kind){case "predefined":return {kind,name:text(row.name)};case "embedded":{const cmap=record(row.cmap);return {kind,cmap:{name:text(cmap.name),vertical:boolean(cmap.vertical??false),codespace:array(cmap.codespace??[]).map(input=>{const row=record(input);return {byteWidth:integer(row.byteWidth),low:integer(row.low),high:integer(row.high)};}),mappings:array(cmap.mappings??[]).map(pdfCidMappingFromNativeJson),useCmap:cmap.useCmap==null?null:text(cmap.useCmap)}};}default:throw new Error("Unknown PDF CMap");} }
/** 🔗️ Admit native CID glyph-map ownership. */
export function pdfGidFromNativeJson(input: unknown): PdfCidToGid {const row=record(input);const kind=text(row.kind);switch(kind){case "identity":return {kind};case "map":return {kind,glyphs:integers(row.glyphs)};default:throw new Error("Unknown PDF glyph map");} }
/** 📐️ Admit every native CID width-run word. */
export function pdfWidthRunFromNativeJson(input: unknown): PdfCidWidthRun {const row=record(input);return {startCid:integer(row.startCid),widths:reals(row.widths)};}
/** 📐️ Admit every native CID vertical triple. */
export function pdfVerticalRunFromNativeJson(input: unknown): PdfCidVerticalRun {const row=record(input);return {startCid:integer(row.startCid),metrics:array(row.metrics).map(three)};}
/** 🈶️ Admit complete CID font defaults and optional entities. */
export function pdfCidFontFromNativeJson(input: unknown): PdfCidFont {
  const row=record(input);const system=record(row.systemInfo??{registry:"Adobe",ordering:"Identity",supplement:0});const vertical=row.defaultVertical==null?null:reals(row.defaultVertical);if(vertical!==null&&vertical.length!==2)throw new Error("PDF vertical default requires two components");
  return {trueType:boolean(row.trueType),baseFont:text(row.baseFont),systemInfo:{registry:text(system.registry),ordering:text(system.ordering),supplement:integer(system.supplement)},descriptor:pdfDescriptorFromNativeJson(row.descriptor),defaultWidth:real(row.defaultWidth??1000),widths:array(row.widths??[]).map(pdfWidthRunFromNativeJson),defaultVertical:vertical===null?null:[vertical[0]!,vertical[1]!],verticalMetrics:array(row.verticalMetrics??[]).map(pdfVerticalRunFromNativeJson),cidToGid:row.cidToGid==null?null:pdfGidFromNativeJson(row.cidToGid),program:row.program==null?null:pdfProgramFromNativeJson(row.program),extra:pdfDictionaryFromNativeJson(row.extra??[])};
}
/** 🔣️ Admit Type3 character procedure ownership and exact content operands. */
export function pdfCharProcFromNativeJson(input: unknown): PdfCharProc {const row=record(input);return {name:text(row.name),content:array(row.content).map(pdfOperationFromNativeJson)};}
/** 🔤️ Admit all four native font variants directly into the canonical owner. */
export function pdfFontKindFromNativeJson(input: unknown): PdfFontKind {
  const row=record(input);const kind=text(row.kind);const descriptor=()=>row.descriptor==null?null:pdfDescriptorFromNativeJson(row.descriptor);
  switch(kind){
    case "type1":case "trueType":return {kind,baseFont:text(row.baseFont),encoding:pdfEncodingFromNativeJson(row.encoding),firstChar:integer(row.firstChar),widths:reals(row.widths),descriptor:descriptor(),program:row.program==null?null:pdfProgramFromNativeJson(row.program)};
    case "type3":return {kind,fontMatrix:pdfMatrixFromNativeJson(row.fontMatrix),fontBbox:four(row.fontBbox),encoding:pdfEncodingFromNativeJson(row.encoding),firstChar:integer(row.firstChar),widths:reals(row.widths),charProcs:array(row.charProcs).map(pdfCharProcFromNativeJson),descriptor:descriptor()};
    case "type0":return {kind,baseFont:text(row.baseFont),cmap:pdfCMapFromNativeJson(row.cmap),descendant:pdfCidFontFromNativeJson(row.descendant)};
    default:throw new Error("Unknown PDF font kind");
  }
}
/** 🔤️ Admit a full native font with Unicode and ordered extra COS entries. */
export function pdfFontFromNativeJson(input: unknown): PdfFont {const row=record(input);return {id:text(row.id),kind:pdfFontKindFromNativeJson(row.kind),toUnicode:row.toUnicode==null?null:pdfUnicodeFromNativeJson(row.toUnicode),extra:pdfDictionaryFromNativeJson(row.extra??[])};}
