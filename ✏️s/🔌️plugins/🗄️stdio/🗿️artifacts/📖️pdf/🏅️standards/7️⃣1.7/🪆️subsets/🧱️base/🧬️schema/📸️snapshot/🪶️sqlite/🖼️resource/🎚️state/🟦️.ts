/** 🎚️ Handwritten graphics state, soft masks, form resources and transparency groups. */
import type { PdfExtGState,PdfSoftMask,PdfFormXObject,PdfTransparencyGroup,PdfLineCap,PdfLineJoin } from "../../../🟦️.ts";
import { PdfProjection,PdfReader,pdfInteger,pdfNumber,pdfBoolean,type SqliteRow,type Binary64 } from "../../🧩️entity/🟦️.ts";
import { writePdfDictionary,readPdfDictionary } from "../../🧩️cos/🟦️.ts";
import { writePdfColorSpace,readPdfColorSpace,writePdfFunction,readPdfFunction,writePdfReals,readPdfReals } from "../../🌈️color/🟦️.ts";
import { writePdfOperations,readPdfOperations } from "../../🖋️content/🟦️.ts";
import { artifactSqliteInteger } from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";

function optionalBoolean(row:SqliteRow,column:number):boolean|null{return row.values[column]===null?null:pdfBoolean(row,column);}
function optionalInteger(row:SqliteRow,column:number):number|null{return row.values[column]===null?null:pdfNumber(row,column);}
function bool(value:boolean|null|undefined):bigint|null{return value==null?null:value?1n:0n;}
function integer(value:number|null|undefined):bigint|null{return value==null?null:pdfInteger(value);}
function optionalReal(reader:PdfReader,table:string,row:SqliteRow,column:number):Binary64|null{return reader.isNull(table,row,column)?null:reader.real(table,row,column);}
function cap(value:string|null|undefined):PdfLineCap|null{if(value==null)return null;if(value!=="butt"&&value!=="round"&&value!=="square")throw new Error("Unknown PDF line cap");return value;}
function join(value:string|null|undefined):PdfLineJoin|null{if(value==null)return null;if(value!=="miter"&&value!=="round"&&value!=="bevel")throw new Error("Unknown PDF line join");return value;}
const identity:[Binary64,Binary64,Binary64,Binary64,Binary64,Binary64]=[{bits:0x3ff0000000000000n},{bits:0n},{bits:0n},{bits:0x3ff0000000000000n},{bits:0n},{bits:0n}];

/** 🎭️ Project transparency flags and the optional color entity. */
export async function writePdfGroup(out:PdfProjection,value:PdfTransparencyGroup):Promise<bigint>{return out.insert("pdf_transparency_group",[value.colorSpace==null?null:await writePdfColorSpace(out,value.colorSpace),value.isolated?1n:0n,value.knockout?1n:0n]);}
/** 📥️ Restore transparency with exact native boolean fields. */
export async function readPdfGroup(reader:PdfReader,key:bigint):Promise<PdfTransparencyGroup>{const row=await reader.take("pdf_transparency_group",key,4);return {colorSpace:row.values[1]===null?null:await readPdfColorSpace(reader,artifactSqliteInteger(row,1)),isolated:pdfBoolean(row,2),knockout:pdfBoolean(row,3)};}
/** 🧩️ Project every form field and its owned content and transparency entities. */
export async function writePdfForm(out:PdfProjection,value:PdfFormXObject):Promise<bigint>{const content=await writePdfOperations(out,value.content??[]);const group=value.group==null?null:await writePdfGroup(out,value.group);const extra=await writePdfDictionary(out,value.extra??[]);return out.insert("pdf_form_xobject",[value.id,...value.bbox,...(value.matrix??identity),content,group,value.optionalContent??null,integer(value.structParent),extra]);}
/** 📥️ Restore complete form ownership without native wire encoding. */
export async function readPdfForm(reader:PdfReader,key:bigint):Promise<PdfFormXObject>{const row=await reader.take("pdf_form_xobject",key,17);const real=(column:number)=>reader.real("pdf_form_xobject",row,column);return {id:await reader.text(row,1),bbox:[real(2),real(3),real(4),real(5)],matrix:[real(6),real(7),real(8),real(9),real(10),real(11)],content:await readPdfOperations(reader,artifactSqliteInteger(row,12)),group:row.values[13]===null?null:await readPdfGroup(reader,artifactSqliteInteger(row,13)),optionalContent:await reader.optionalText(row,14),structParent:optionalInteger(row,15),extra:await readPdfDictionary(reader,artifactSqliteInteger(row,16))};}

/** 🎚️ Project scalar state fields and every explicitly optional ordered relationship. */
export async function writePdfState(out:PdfProjection,value:PdfExtGState):Promise<bigint>{
  let kind:string|null=null;let group:string|null=null;let backdrop:bigint|null=null;let transfer:bigint|null=null;
  if(value.softMask!=null){kind=value.softMask.kind;switch(value.softMask.kind){case "none":break;case "alpha":group=value.softMask.group;transfer=value.softMask.transfer==null?null:await writePdfFunction(out,value.softMask.transfer);break;case "luminosity":group=value.softMask.group;backdrop=value.softMask.backdrop==null?0n:1n;transfer=value.softMask.transfer==null?null:await writePdfFunction(out,value.softMask.transfer);break;default:throw new Error("Unknown PDF soft mask");}}
  const extra=await writePdfDictionary(out,value.extra??[]);
  const key=await out.insert("pdf_ext_g_state",[value.id,value.lineWidth??null,cap(value.lineCap),join(value.lineJoin),value.miterLimit??null,value.dash?.[1]??null,value.renderingIntent??null,bool(value.overprintStroke),bool(value.overprintFill),integer(value.overprintMode),value.font?.[0]??null,value.font?.[1]??null,value.blendMode==null?0n:1n,kind,group,backdrop,transfer,value.strokeAlpha??null,value.fillAlpha??null,bool(value.alphaIsShape),bool(value.strokeAdjust),value.flatness??null,value.smoothness??null,bool(value.textKnockout),extra]);
  if(value.dash!=null)await writePdfReals(out,"pdf_g_state_real",key,"dash",value.dash[0]);
  if(value.softMask?.kind==="luminosity"&&value.softMask.backdrop!=null)await writePdfReals(out,"pdf_g_state_real",key,"backdrop",value.softMask.backdrop);
  if(value.blendMode!=null){out.checkRowsAdditional(value.blendMode.length);for(const[ordinal,mode]of value.blendMode.entries())await out.insert("pdf_blend_mode",[key,BigInt(ordinal),mode]);}
  return key;
}
/** 📥️ Reject mixed mask variants and incomplete font tuples while restoring exact words. */
export async function readPdfState(reader:PdfReader,key:bigint):Promise<PdfExtGState>{
  const row=await reader.take("pdf_ext_g_state",key,26);const real=(column:number)=>optionalReal(reader,"pdf_ext_g_state",row,column);const name=await reader.optionalText(row,11);const size=real(12);if((name===null)!==(size===null))throw new Error("PDF graphics font requires both name and size");
  let blendMode:string[]|null=null;if(pdfBoolean(row,13)){blendMode=[];for(const child of await reader.children("pdf_blend_mode",1,2,key)){const value=await reader.take("pdf_blend_mode",child.rowid,4);blendMode.push(await reader.text(value,3));}}
  let softMask:PdfSoftMask|null=null;const mask=await reader.optionalText(row,14);
  switch(mask){case null:case "none":reader.nullExcept("pdf_ext_g_state",row,15,18,[]);softMask=mask===null?null:{kind:"none"};break;case "alpha":reader.nullExcept("pdf_ext_g_state",row,16,17,[]);softMask={kind:mask,group:await reader.text(row,15),transfer:row.values[17]===null?null:await readPdfFunction(reader,artifactSqliteInteger(row,17))};break;case "luminosity":softMask={kind:mask,group:await reader.text(row,15),backdrop:pdfBoolean(row,16)?await readPdfReals(reader,"pdf_g_state_real",key,"backdrop"):null,transfer:row.values[17]===null?null:await readPdfFunction(reader,artifactSqliteInteger(row,17))};break;default:throw new Error("Unknown PDF soft mask");}
  const phase=real(6);return {id:await reader.text(row,1),lineWidth:real(2),lineCap:cap(await reader.optionalText(row,3)),lineJoin:join(await reader.optionalText(row,4)),miterLimit:real(5),dash:phase===null?null:[await readPdfReals(reader,"pdf_g_state_real",key,"dash"),phase],renderingIntent:await reader.optionalText(row,7),overprintStroke:optionalBoolean(row,8),overprintFill:optionalBoolean(row,9),overprintMode:optionalInteger(row,10),font:name===null?null:[name,size!],blendMode,softMask,strokeAlpha:real(18),fillAlpha:real(19),alphaIsShape:optionalBoolean(row,20),strokeAdjust:optionalBoolean(row,21),flatness:real(22),smoothness:real(23),textKnockout:optionalBoolean(row,24),extra:await readPdfDictionary(reader,artifactSqliteInteger(row,25))};
}
