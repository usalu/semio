import {pdfImageBodyFromNativeJson} from "../🖼️resource/🟦️.ts";
/** 🖋️ Explicit native content JSON admission into the canonical owned PDF operator model. */
import type { PdfOp, PdfTextString, PdfTextArrayItem, PdfPropertyList, PdfInlineImage, Binary64 } from "../../../../../🧬️schema/📸️snapshot/🟦️.ts";
import { record, array, text, boolean, integer, integers, real, reals, pdfDictionaryFromNativeJson, pdfCosFromNativeJson, pdfColorFromNativeJson, filters } from "../🟦️.ts";

/** 📐️ Admit a finite native six-component matrix as six exact owned IEEE words. */
export function pdfMatrixFromNativeJson(input: unknown): [Binary64,Binary64,Binary64,Binary64,Binary64,Binary64] { const values=reals(input);if(values.length!==6)throw new Error("PDF matrix requires six components");return [values[0]!,values[1]!,values[2]!,values[3]!,values[4]!,values[5]!]; }
/** 🅰️ Admit native text or logical unsigned character codes. */
export function pdfTextFromNativeJson(input: unknown): PdfTextString { const row=record(input);const kind=text(row.kind);switch(kind){case "text":return {kind,text:text(row.text)};case "codes":return {kind,codes:integers(row.codes).map(code=>{if(code<0||code>0xffffffff)throw new Error("PDF logical code exceeds u32");return code;})};default:throw new Error("Unknown PDF text operand");} }
/** 🔠️ Admit every typed native text-array item including owned adjustment words. */
export function pdfTextItemFromNativeJson(input: unknown): PdfTextArrayItem { const row=record(input);if(row.kind==="adjust")return {kind:"adjust",amount:real(row.amount)};return pdfTextFromNativeJson(input); }
/** 🏷️ Admit named or inline property-list ownership explicitly. */
export function pdfPropertyFromNativeJson(input: unknown): PdfPropertyList { const row=record(input);const kind=text(row.kind);switch(kind){case "named":return {kind,name:text(row.name)};case "inline":return {kind,entries:pdfDictionaryFromNativeJson(row.entries)};default:throw new Error("Unknown PDF property list");} }
/** 🖼️ Admit all intrinsic inline image fields and exact finite decode words. */
export function pdfInlineFromNativeJson(input: unknown): PdfInlineImage { const row=record(input);return {width:integer(row.width),height:integer(row.height),bitsPerComponent:integer(row.bitsPerComponent??0),colorSpace:row.colorSpace==null?null:pdfColorFromNativeJson(row.colorSpace),imageMask:boolean(row.imageMask??false),decode:reals(row.decode??[]),interpolate:boolean(row.interpolate??false),body:pdfImageBodyFromNativeJson(row.body),extra:pdfDictionaryFromNativeJson(row.extra??[])}; }
/** 🖋️ Admit each concrete native operator without a second reflected snapshot model. */
export function pdfOperationFromNativeJson(input: unknown): PdfOp {
  const row=record(input);const op=text(row.op);
  switch(op){
    case "setLineWidth":return {op,width:real(row.width)};
    case "setLineCap":{const cap=text(row.cap);if(cap!=="butt"&&cap!=="round"&&cap!=="square")throw new Error("Unknown PDF cap");return {op,cap};}
    case "setLineJoin":{const join=text(row.join);if(join!=="miter"&&join!=="round"&&join!=="bevel")throw new Error("Unknown PDF join");return {op,join};}
    case "setMiterLimit":return {op,limit:real(row.limit)};
    case "setDash":return {op,array:reals(row.array),phase:real(row.phase)};
    case "setRenderingIntent":return {op,intent:text(row.intent)};
    case "setFlatness":return {op,flatness:real(row.flatness)};
    case "setExtGState":return {op,name:text(row.name)};
    case "transform":case "setTextMatrix":return {op,matrix:pdfMatrixFromNativeJson(row.matrix)};
    case "moveTo":case "lineTo":return {op,x:real(row.x),y:real(row.y)};
    case "curveTo":return {op,x1:real(row.x1),y1:real(row.y1),x2:real(row.x2),y2:real(row.y2),x3:real(row.x3),y3:real(row.y3)};
    case "curveToInitial":return {op,x2:real(row.x2),y2:real(row.y2),x3:real(row.x3),y3:real(row.y3)};
    case "curveToFinal":return {op,x1:real(row.x1),y1:real(row.y1),x3:real(row.x3),y3:real(row.y3)};
    case "rectangle":return {op,x:real(row.x),y:real(row.y),width:real(row.width),height:real(row.height)};
    case "setCharSpacing":case "setWordSpacing":return {op,spacing:real(row.spacing)};
    case "setHorizontalScale":return {op,scale:real(row.scale)};
    case "setLeading":return {op,leading:real(row.leading)};
    case "setFont":return {op,name:text(row.name),size:real(row.size)};
    case "setTextRenderingMode":return {op,mode:integer(row.mode)};
    case "setTextRise":return {op,rise:real(row.rise)};
    case "moveText":case "moveTextSetLeading":return {op,tx:real(row.tx),ty:real(row.ty)};
    case "showText":case "nextLineShowText":return {op,text:pdfTextFromNativeJson(row.text)};
    case "nextLineShowTextSpaced":return {op,wordSpacing:real(row.wordSpacing),charSpacing:real(row.charSpacing),text:pdfTextFromNativeJson(row.text)};
    case "showTextArray":return {op,items:array(row.items).map(pdfTextItemFromNativeJson)};
    case "setGlyphWidth":return {op,wx:real(row.wx),wy:real(row.wy)};
    case "setGlyphWidthAndBox":return {op,wx:real(row.wx),wy:real(row.wy),llx:real(row.llx),lly:real(row.lly),urx:real(row.urx),ury:real(row.ury)};
    case "setStrokeColorSpace":case "setFillColorSpace":case "paintShading":case "paintXObject":return {op,name:text(row.name)};
    case "setStrokeColor":case "setFillColor":return {op,components:reals(row.components)};
    case "setStrokeColorN":case "setFillColorN":return {op,components:reals(row.components),pattern:row.pattern==null?null:text(row.pattern)};
    case "setStrokeGray":case "setFillGray":return {op,gray:real(row.gray)};
    case "setStrokeRgb":case "setFillRgb":return {op,r:real(row.r),g:real(row.g),b:real(row.b)};
    case "setStrokeCmyk":case "setFillCmyk":return {op,c:real(row.c),m:real(row.m),y:real(row.y),k:real(row.k)};
    case "inlineImage":return {op,image:pdfInlineFromNativeJson(row.image)};
    case "markedContentPoint":case "beginMarkedContent":return {op,tag:text(row.tag)};
    case "markedContentPointWithProperties":case "beginMarkedContentWithProperties":return {op,tag:text(row.tag),properties:pdfPropertyFromNativeJson(row.properties)};
    case "unknown":return {op,operator:text(row.operator),operands:array(row.operands).map(pdfCosFromNativeJson)};
    case "save":case "restore":case "closePath":case "stroke":case "closeStroke":case "fill":case "fillEvenOdd":case "fillStroke":case "fillStrokeEvenOdd":case "closeFillStroke":case "closeFillStrokeEvenOdd":case "endPath":case "clip":case "clipEvenOdd":case "beginText":case "endText":case "nextLine":case "endMarkedContent":case "beginCompatibility":case "endCompatibility":return {op};
    default:throw new Error("Unknown PDF native operation");
  }
}
