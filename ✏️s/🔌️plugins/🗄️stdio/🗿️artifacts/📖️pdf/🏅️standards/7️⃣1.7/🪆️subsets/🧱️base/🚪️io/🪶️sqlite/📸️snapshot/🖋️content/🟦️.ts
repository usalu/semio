/** 🖋️ Handwritten PDF operators, named operands and intrinsic inline image entities. */
import type { PdfOp, PdfTextString, PdfTextArrayItem, PdfPropertyList, PdfInlineImage, Binary64 } from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
import { PdfProjection, PdfReader, pdfInteger, pdfNumber, pdfBoolean, type PdfCell, type SqliteRow } from "../🧩️entity/🟦️.ts";
import { writePdfDictionary, readPdfDictionary, writePdfObject, readPdfObject, writePdfFilters, readPdfFilters } from "../🧩️cos/🟦️.ts";
import { writePdfColorSpace, readPdfColorSpace, pdfColorNumberColumns } from "../🌈️color/🟦️.ts";
import { artifactSqliteInteger, artifactSqliteText } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import type { Ieee754Column } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";

enum O { LineWidth, LineCap, LineJoin, MiterLimit, DashPhase, RenderingIntent, Flatness, ExtGState, X1, Y1, X2, Y2, X3, Y3, Width, Height, CharSpacing, WordSpacing, HorizontalScale, Leading, FontName, FontSize, TextRenderingMode, TextRise, Tx, Ty, TextKind, TextValue, TextCodes, GlyphWx, GlyphWy, BboxLlx, BboxLly, BboxUrx, BboxUry, ColorSpaceName, PatternName, Gray, Red, Green, Blue, Cyan, Magenta, Yellow, Black, ShadingName, XobjectName, MarkedTag, PropertyKind, PropertyName, PropertyDictionary, InlineImage, UnknownOperator }
const width = O.UnknownOperator + 1;
const column = (value: O): number => value + 4;

/** 🔢️ Actual content binary64 operands have explicitly declared SQL companion locations. */
export function pdfContentNumberColumns(table: string): readonly Ieee754Column[] {
  switch (table) {
    case "pdf_operation": return [{index:4,width:64},{index:7,width:64},{index:8,width:64},{index:10,width:64},{index:12,width:64},{index:13,width:64},{index:14,width:64},{index:15,width:64},{index:16,width:64},{index:17,width:64},{index:18,width:64},{index:19,width:64},{index:20,width:64},{index:21,width:64},{index:22,width:64},{index:23,width:64},{index:25,width:64},{index:27,width:64},{index:28,width:64},{index:29,width:64},{index:33,width:64},{index:34,width:64},{index:35,width:64},{index:36,width:64},{index:37,width:64},{index:38,width:64},{index:41,width:64},{index:42,width:64},{index:43,width:64},{index:44,width:64},{index:45,width:64},{index:46,width:64},{index:47,width:64},{index:48,width:64}];
    case "pdf_operation_matrix": return [{index:1,width:64},{index:2,width:64},{index:3,width:64},{index:4,width:64},{index:5,width:64},{index:6,width:64}];
    case "pdf_operation_component": case "pdf_inline_decode": return [{index:3,width:64}];
    case "pdf_text_array_item": return [{index:6,width:64}];
    default: return pdfColorNumberColumns(table);
  }
}

/** 🖼️ Project complete inline images using intrinsic octets and explicit decode rows. */
export async function writePdfInlineImage(out: PdfProjection, image: PdfInlineImage): Promise<bigint> {
  const color = image.colorSpace == null ? null : await writePdfColorSpace(out, image.colorSpace);
  const filters = await writePdfFilters(out, image.filters ?? []); const extra = await writePdfDictionary(out, image.extra ?? []);
  const key = await out.insert("pdf_inline_image", [pdfInteger(image.width), pdfInteger(image.height), pdfInteger(image.bitsPerComponent ?? 0), color, image.imageMask ? 1n : 0n, image.interpolate ? 1n : 0n, filters, await out.bytes(image.data), extra]);
  out.checkRowsAdditional(image.decode?.length ?? 0);
  for (const [ordinal, value] of (image.decode ?? []).entries()) await out.insert("pdf_inline_decode", [key, BigInt(ordinal), value]);
  return key;
}
/** 📥️ Restore intrinsic inline image bytes and exact IEEE decode words. */
export async function readPdfInlineImage(reader: PdfReader, key: bigint): Promise<PdfInlineImage> {
  const row = await reader.take("pdf_inline_image", key, 10); const decode: Binary64[] = [];
  for (const child of await reader.children("pdf_inline_decode",1,2,key)) { const row = await reader.take("pdf_inline_decode",child.rowid,4); decode.push(reader.real("pdf_inline_decode",row,3)); }
  return { width:pdfNumber(row,1),height:pdfNumber(row,2),bitsPerComponent:pdfNumber(row,3),colorSpace:row.values[4] === null ? null : await readPdfColorSpace(reader,artifactSqliteInteger(row,4)),imageMask:pdfBoolean(row,5),interpolate:pdfBoolean(row,6),filters:await readPdfFilters(reader,artifactSqliteInteger(row,7)),data:await reader.bytes(row,8),extra:await readPdfDictionary(reader,artifactSqliteInteger(row,9)),decode };
}

/** 🖋️ Every operator projects its actual named operands and separately ordered child entities. */
export async function writePdfOperations(out: PdfProjection, operations: readonly PdfOp[]): Promise<bigint> {
  out.checkRowsAdditional(operations.length + 1); const content = await out.insert("pdf_content", []);
  for (const [ordinal, operation] of operations.entries()) {
    const fields: PdfCell[] = new Array(width).fill(null);
    const put = (indices: readonly O[], values: readonly PdfCell[]): void => { for (let index=0;index<indices.length;index++) fields[indices[index]!] = values[index]!; };
    const text = async (value: PdfTextString): Promise<void> => { switch (value.kind) { case "text": put([O.TextKind,O.TextValue],[value.kind,value.text]); break; case "codes": put([O.TextKind,O.TextCodes],[value.kind,await out.bytes(value.bytes)]); break; default: throw new Error("Unknown PDF text operand"); } };
    switch (operation.op) {
      case "setLineWidth": put([O.LineWidth],[operation.width]); break;
      case "setLineCap": if (!["butt","round","square"].includes(operation.cap)) throw new Error("Unknown PDF line cap"); put([O.LineCap],[operation.cap]); break;
      case "setLineJoin": if (!["miter","round","bevel"].includes(operation.join)) throw new Error("Unknown PDF line join"); put([O.LineJoin],[operation.join]); break;
      case "setMiterLimit": put([O.MiterLimit],[operation.limit]); break;
      case "setDash": put([O.DashPhase],[operation.phase]); break;
      case "setRenderingIntent": put([O.RenderingIntent],[operation.intent]); break;
      case "setFlatness": put([O.Flatness],[operation.flatness]); break;
      case "setExtGState": put([O.ExtGState],[operation.name]); break;
      case "moveTo": case "lineTo": put([O.X1,O.Y1],[operation.x,operation.y]); break;
      case "curveTo": put([O.X1,O.Y1,O.X2,O.Y2,O.X3,O.Y3],[operation.x1,operation.y1,operation.x2,operation.y2,operation.x3,operation.y3]); break;
      case "curveToInitial": put([O.X2,O.Y2,O.X3,O.Y3],[operation.x2,operation.y2,operation.x3,operation.y3]); break;
      case "curveToFinal": put([O.X1,O.Y1,O.X3,O.Y3],[operation.x1,operation.y1,operation.x3,operation.y3]); break;
      case "rectangle": put([O.X1,O.Y1,O.Width,O.Height],[operation.x,operation.y,operation.width,operation.height]); break;
      case "setCharSpacing": put([O.CharSpacing],[operation.spacing]); break;
      case "setWordSpacing": put([O.WordSpacing],[operation.spacing]); break;
      case "setHorizontalScale": put([O.HorizontalScale],[operation.scale]); break;
      case "setLeading": put([O.Leading],[operation.leading]); break;
      case "setFont": put([O.FontName,O.FontSize],[operation.name,operation.size]); break;
      case "setTextRenderingMode": put([O.TextRenderingMode],[pdfInteger(operation.mode)]); break;
      case "setTextRise": put([O.TextRise],[operation.rise]); break;
      case "moveText": case "moveTextSetLeading": put([O.Tx,O.Ty],[operation.tx,operation.ty]); break;
      case "showText": case "nextLineShowText": await text(operation.text); break;
      case "nextLineShowTextSpaced": put([O.WordSpacing,O.CharSpacing],[operation.wordSpacing,operation.charSpacing]); await text(operation.text); break;
      case "setGlyphWidth": put([O.GlyphWx,O.GlyphWy],[operation.wx,operation.wy]); break;
      case "setGlyphWidthAndBox": put([O.GlyphWx,O.GlyphWy,O.BboxLlx,O.BboxLly,O.BboxUrx,O.BboxUry],[operation.wx,operation.wy,operation.llx,operation.lly,operation.urx,operation.ury]); break;
      case "setStrokeColorSpace": case "setFillColorSpace": put([O.ColorSpaceName],[operation.name]); break;
      case "setStrokeColorN": case "setFillColorN": put([O.PatternName],[operation.pattern ?? null]); break;
      case "setStrokeGray": case "setFillGray": put([O.Gray],[operation.gray]); break;
      case "setStrokeRgb": case "setFillRgb": put([O.Red,O.Green,O.Blue],[operation.r,operation.g,operation.b]); break;
      case "setStrokeCmyk": case "setFillCmyk": put([O.Cyan,O.Magenta,O.Yellow,O.Black],[operation.c,operation.m,operation.y,operation.k]); break;
      case "paintShading": put([O.ShadingName],[operation.name]); break;
      case "paintXObject": put([O.XobjectName],[operation.name]); break;
      case "inlineImage": put([O.InlineImage],[await writePdfInlineImage(out,operation.image)]); break;
      case "markedContentPoint": case "beginMarkedContent": put([O.MarkedTag],[operation.tag]); break;
      case "markedContentPointWithProperties": case "beginMarkedContentWithProperties": {
        const properties=operation.properties; put([O.MarkedTag,O.PropertyKind],[operation.tag,properties.kind]);
        switch (properties.kind) { case "named": put([O.PropertyName],[properties.name]); break; case "inline": put([O.PropertyDictionary],[await writePdfDictionary(out,properties.entries)]); break; default: throw new Error("Unknown PDF property list"); } break;
      }
      case "unknown": put([O.UnknownOperator],[operation.operator]); break;
      case "save": case "restore": case "transform": case "closePath": case "stroke": case "closeStroke": case "fill": case "fillEvenOdd": case "fillStroke": case "fillStrokeEvenOdd": case "closeFillStroke": case "closeFillStrokeEvenOdd": case "endPath": case "clip": case "clipEvenOdd": case "beginText": case "endText": case "setTextMatrix": case "nextLine": case "showTextArray": case "setStrokeColor": case "setFillColor": case "endMarkedContent": case "beginCompatibility": case "endCompatibility": break;
      default: throw new Error("Unknown PDF operation");
    }
    const key = await out.insert("pdf_operation",[content,BigInt(ordinal),operation.op,...fields]);
    switch (operation.op) {
      case "transform": case "setTextMatrix": if (operation.matrix.length !== 6) throw new Error("PDF matrix requires six components"); await out.insert("pdf_operation_matrix",operation.matrix,key); break;
      case "setDash": out.checkRowsAdditional(operation.array.length); for (const [ordinal,value] of operation.array.entries()) await out.insert("pdf_operation_component",[key,BigInt(ordinal),value]); break;
      case "setStrokeColor": case "setFillColor": case "setStrokeColorN": case "setFillColorN": out.checkRowsAdditional(operation.components.length); for (const [ordinal,value] of operation.components.entries()) await out.insert("pdf_operation_component",[key,BigInt(ordinal),value]); break;
      case "showTextArray": {
        out.checkRowsAdditional(operation.items.length);
        for (const [ordinal,item] of operation.items.entries()) { switch (item.kind) { case "text": await out.insert("pdf_text_array_item",[key,BigInt(ordinal),item.kind,item.text,null,null]); break; case "codes": await out.insert("pdf_text_array_item",[key,BigInt(ordinal),item.kind,null,await out.bytes(item.bytes),null]); break; case "adjust": await out.insert("pdf_text_array_item",[key,BigInt(ordinal),item.kind,null,null,item.amount]); break; default: throw new Error("Unknown PDF text array item"); } } break;
      }
      case "unknown": out.checkRowsAdditional(operation.operands.length); for (const [ordinal,operand] of operation.operands.entries()) await out.insert("pdf_unknown_operand",[key,BigInt(ordinal),await writePdfObject(out,operand)]); break;
    }
  }
  return content;
}

async function components(reader: PdfReader,key: bigint): Promise<Binary64[]> { const values: Binary64[]=[]; for (const child of await reader.children("pdf_operation_component",1,2,key)) { const row=await reader.take("pdf_operation_component",child.rowid,4); values.push(reader.real("pdf_operation_component",row,3)); } return values; }
async function matrix(reader: PdfReader,key: bigint): Promise<[Binary64,Binary64,Binary64,Binary64,Binary64,Binary64]> { const row=await reader.take("pdf_operation_matrix",key,7); return [reader.real("pdf_operation_matrix",row,1),reader.real("pdf_operation_matrix",row,2),reader.real("pdf_operation_matrix",row,3),reader.real("pdf_operation_matrix",row,4),reader.real("pdf_operation_matrix",row,5),reader.real("pdf_operation_matrix",row,6)]; }

/** 📥️ Consume exact operator relationships and reject scalar or IEEE payloads of another variant. */
export async function readPdfOperations(reader: PdfReader,content: bigint): Promise<PdfOp[]> {
  await reader.take("pdf_content",content,1); const operations: PdfOp[]=[];
  for (const child of await reader.children("pdf_operation",1,2,content)) {
    const row=await reader.take("pdf_operation",child.rowid,width+4); const key=row.rowid; const present: number[]=[];
    const use=(value: O): number => { const index=column(value); present.push(index); return index; };
    const real=(value: O): Binary64 => reader.real("pdf_operation",row,use(value));
    const text=(value: O): Promise<string> => reader.text(row,use(value));
    const optionalText=(value: O): Promise<string|null> => reader.optionalText(row,use(value));
    const readText=async (): Promise<PdfTextString> => {
      const kind=await text(O.TextKind);
      switch (kind) { case "text": return {kind,text:await text(O.TextValue)}; case "codes": return {kind,bytes:await reader.bytes(row,use(O.TextCodes))}; default: throw new Error("Unknown PDF text operand"); }
    };
    const properties=async (): Promise<PdfPropertyList> => { const kind=await text(O.PropertyKind); switch (kind) { case "named": return {kind,name:await text(O.PropertyName)}; case "inline": return {kind,entries:await readPdfDictionary(reader,artifactSqliteInteger(row,use(O.PropertyDictionary)))}; default: throw new Error("Unknown PDF property list"); } };
    const op=artifactSqliteText(row,3); let operation: PdfOp;
    switch (op) {
      case "setLineWidth": operation={op,width:real(O.LineWidth)}; break;
      case "setLineCap": { const cap=await text(O.LineCap); if (cap!=="butt"&&cap!=="round"&&cap!=="square") throw new Error("Unknown PDF line cap"); operation={op,cap}; break; }
      case "setLineJoin": { const join=await text(O.LineJoin); if (join!=="miter"&&join!=="round"&&join!=="bevel") throw new Error("Unknown PDF line join"); operation={op,join}; break; }
      case "setMiterLimit": operation={op,limit:real(O.MiterLimit)}; break;
      case "setDash": operation={op,array:await components(reader,key),phase:real(O.DashPhase)}; break;
      case "setRenderingIntent": operation={op,intent:await text(O.RenderingIntent)}; break;
      case "setFlatness": operation={op,flatness:real(O.Flatness)}; break;
      case "setExtGState": operation={op,name:await text(O.ExtGState)}; break;
      case "transform": case "setTextMatrix": operation={op,matrix:await matrix(reader,key)}; break;
      case "moveTo": case "lineTo": operation={op,x:real(O.X1),y:real(O.Y1)}; break;
      case "curveTo": operation={op,x1:real(O.X1),y1:real(O.Y1),x2:real(O.X2),y2:real(O.Y2),x3:real(O.X3),y3:real(O.Y3)}; break;
      case "curveToInitial": operation={op,x2:real(O.X2),y2:real(O.Y2),x3:real(O.X3),y3:real(O.Y3)}; break;
      case "curveToFinal": operation={op,x1:real(O.X1),y1:real(O.Y1),x3:real(O.X3),y3:real(O.Y3)}; break;
      case "rectangle": operation={op,x:real(O.X1),y:real(O.Y1),width:real(O.Width),height:real(O.Height)}; break;
      case "setCharSpacing": operation={op,spacing:real(O.CharSpacing)}; break;
      case "setWordSpacing": operation={op,spacing:real(O.WordSpacing)}; break;
      case "setHorizontalScale": operation={op,scale:real(O.HorizontalScale)}; break;
      case "setLeading": operation={op,leading:real(O.Leading)}; break;
      case "setFont": operation={op,name:await text(O.FontName),size:real(O.FontSize)}; break;
      case "setTextRenderingMode": operation={op,mode:pdfNumber(row,use(O.TextRenderingMode))}; break;
      case "setTextRise": operation={op,rise:real(O.TextRise)}; break;
      case "moveText": case "moveTextSetLeading": operation={op,tx:real(O.Tx),ty:real(O.Ty)}; break;
      case "showText": case "nextLineShowText": operation={op,text:await readText()}; break;
      case "nextLineShowTextSpaced": operation={op,wordSpacing:real(O.WordSpacing),charSpacing:real(O.CharSpacing),text:await readText()}; break;
      case "showTextArray": {
        const items: PdfTextArrayItem[]=[];
        for (const child of await reader.children("pdf_text_array_item",1,2,key)) {
          const row=await reader.take("pdf_text_array_item",child.rowid,7); const kind=artifactSqliteText(row,3);
          switch (kind) { case "text": reader.nullExcept("pdf_text_array_item",row,4,7,[4]); items.push({kind,text:await reader.text(row,4)}); break; case "codes": reader.nullExcept("pdf_text_array_item",row,4,7,[5]); items.push({kind,bytes:await reader.bytes(row,5)}); break; case "adjust": reader.nullExcept("pdf_text_array_item",row,4,7,[6]); items.push({kind,amount:reader.real("pdf_text_array_item",row,6)}); break; default: throw new Error("Unknown PDF text array item"); }
        }
        operation={op,items}; break;
      }
      case "setGlyphWidth": operation={op,wx:real(O.GlyphWx),wy:real(O.GlyphWy)}; break;
      case "setGlyphWidthAndBox": operation={op,wx:real(O.GlyphWx),wy:real(O.GlyphWy),llx:real(O.BboxLlx),lly:real(O.BboxLly),urx:real(O.BboxUrx),ury:real(O.BboxUry)}; break;
      case "setStrokeColorSpace": case "setFillColorSpace": operation={op,name:await text(O.ColorSpaceName)}; break;
      case "setStrokeColor": case "setFillColor": operation={op,components:await components(reader,key)}; break;
      case "setStrokeColorN": case "setFillColorN": operation={op,components:await components(reader,key),pattern:await optionalText(O.PatternName)}; break;
      case "setStrokeGray": case "setFillGray": operation={op,gray:real(O.Gray)}; break;
      case "setStrokeRgb": case "setFillRgb": operation={op,r:real(O.Red),g:real(O.Green),b:real(O.Blue)}; break;
      case "setStrokeCmyk": case "setFillCmyk": operation={op,c:real(O.Cyan),m:real(O.Magenta),y:real(O.Yellow),k:real(O.Black)}; break;
      case "paintShading": operation={op,name:await text(O.ShadingName)}; break;
      case "paintXObject": operation={op,name:await text(O.XobjectName)}; break;
      case "inlineImage": operation={op,image:await readPdfInlineImage(reader,artifactSqliteInteger(row,use(O.InlineImage)))}; break;
      case "markedContentPoint": case "beginMarkedContent": operation={op,tag:await text(O.MarkedTag)}; break;
      case "markedContentPointWithProperties": case "beginMarkedContentWithProperties": operation={op,tag:await text(O.MarkedTag),properties:await properties()}; break;
      case "unknown": {
        const operands=[]; for (const child of await reader.children("pdf_unknown_operand",1,2,key)) { const row=await reader.take("pdf_unknown_operand",child.rowid,4); operands.push(await readPdfObject(reader,artifactSqliteInteger(row,3))); }
        operation={op,operator:await text(O.UnknownOperator),operands}; break;
      }
      case "save": case "restore": case "closePath": case "stroke": case "closeStroke": case "fill": case "fillEvenOdd": case "fillStroke": case "fillStrokeEvenOdd": case "closeFillStroke": case "closeFillStrokeEvenOdd": case "endPath": case "clip": case "clipEvenOdd": case "beginText": case "endText": case "nextLine": case "endMarkedContent": case "beginCompatibility": case "endCompatibility": operation={op}; break;
      default: throw new Error("Unknown PDF operation");
    }
    reader.nullExcept("pdf_operation",row,4,width+4,present); operations.push(operation);
  }
  return operations;
}
