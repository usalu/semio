/** 🖼️ Handwritten PDF image codecs, intrinsic octets and exact mask/decode relationships. */
import type { PdfImage,PdfImageCodec,PdfImageMask,PdfCcittParameters } from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
import { PdfProjection,PdfReader,pdfInteger,pdfNumber,pdfBoolean,type SqliteRow } from "../🧩️entity/🟦️.ts";
import { writePdfDictionary,readPdfDictionary } from "../🧩️cos/🟦️.ts";
import { writePdfColorSpace,readPdfColorSpace,writePdfReals,readPdfReals } from "../🌈️color/🟦️.ts";
import { artifactSqliteInteger,artifactSqliteText } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
export { pdfResourceNumberColumns } from "./🔢️number/🟦️.ts";

function optionalInteger(value:number|null|undefined):bigint|null{return value==null?null:pdfInteger(value);}
function optionalNumber(row:SqliteRow,column:number):number|null{return row.values[column]===null?null:pdfNumber(row,column);}

/** 📠️ Project every native codec parameter and optional intrinsic globals stream. */
export async function writePdfImageCodec(out:PdfProjection,value:PdfImageCodec):Promise<bigint>{
  let transform:bigint|null=null;let globals:Uint8Array|null=null;
  switch(value.kind){case "raw":case "jpx":case "ccitt":break;case "dct":transform=optionalInteger(value.colorTransform);break;case "jbig2":globals=value.globals==null?null:await out.bytes(value.globals);break;default:throw new Error("Unknown PDF image codec");}
  const key=await out.insert("pdf_image_codec",[value.kind,transform,globals]);
  if(value.kind==="ccitt"){
    const fields=value.parameters;
    await out.insert("pdf_image_ccitt",[pdfInteger(fields.k??0,32,true),pdfInteger(fields.columns??1728),pdfInteger(fields.rows??0),fields.blackIs1?1n:0n,fields.encodedByteAlign?1n:0n,fields.endOfLine?1n:0n,(fields.endOfBlock??true)?1n:0n,pdfInteger(fields.damagedRowsBeforeError??0)],key);
  }
  return key;
}
/** 📥️ Restore only the parameters owned by the exact codec variant. */
export async function readPdfImageCodec(reader:PdfReader,key:bigint):Promise<PdfImageCodec>{
  const row=await reader.take("pdf_image_codec",key,4);const kind=artifactSqliteText(row,1);
  reader.nullExcept("pdf_image_codec",row,2,4,kind==="dct"?[2]:kind==="jbig2"?[3]:[]);
  switch(kind){
    case "raw":case "jpx":return {kind};
    case "dct":return {kind,colorTransform:optionalNumber(row,2)};
    case "jbig2":return {kind,globals:row.values[3]===null?null:await reader.bytes(row,3)};
    case "ccitt":{
      const fields=await reader.take("pdf_image_ccitt",key,9);const parameters:PdfCcittParameters={k:pdfNumber(fields,1,32,true),columns:pdfNumber(fields,2),rows:pdfNumber(fields,3),blackIs1:pdfBoolean(fields,4),encodedByteAlign:pdfBoolean(fields,5),endOfLine:pdfBoolean(fields,6),endOfBlock:pdfBoolean(fields,7),damagedRowsBeforeError:pdfNumber(fields,8)};return {kind,parameters};
    }
    default:throw new Error("Unknown PDF image codec");
  }
}

/** 🖼️ Project every image field, absent/empty matte state and ordered color-key coordinates. */
export async function writePdfImage(out:PdfProjection,image:PdfImage):Promise<bigint>{
  const color=image.colorSpace==null?null:await writePdfColorSpace(out,image.colorSpace);const codec=await writePdfImageCodec(out,image.codec??{kind:"raw"});const extra=await writePdfDictionary(out,image.extra??[]);
  let maskKind:string|null=null;let stencil:string|null=null;
  if(image.mask!=null){switch(image.mask.kind){case "stencil":maskKind=image.mask.kind;stencil=image.mask.image;break;case "colorKey":maskKind=image.mask.kind;break;default:throw new Error("Unknown PDF image mask");}}
  const key=await out.insert("pdf_image",[image.id,pdfInteger(image.width),pdfInteger(image.height),color,pdfInteger(image.bitsPerComponent??0),image.imageMask?1n:0n,image.interpolate?1n:0n,codec,await out.bytes(image.data),image.softMask??null,optionalInteger(image.softMaskInData),maskKind,stencil,image.matte==null?0n:1n,image.intent??null,image.optionalContent??null,optionalInteger(image.structParent),extra]);
  await writePdfReals(out,"pdf_image_real",key,"decode",image.decode??[]);
  if(image.matte!=null)await writePdfReals(out,"pdf_image_real",key,"matte",image.matte);
  if(image.mask?.kind==="colorKey"){out.checkRowsAdditional(image.mask.ranges.length);for(const[ordinal,value]of image.mask.ranges.entries())await out.insert("pdf_image_color_key",[key,BigInt(ordinal),pdfInteger(value)]);}
  return key;
}

/** 📥️ Restore complete image ownership and exact optional mask and matte semantics. */
export async function readPdfImage(reader:PdfReader,key:bigint):Promise<PdfImage>{
  const row=await reader.take("pdf_image",key,19);const kind=await reader.optionalText(row,12);let mask:PdfImageMask|null=null;
  switch(kind){
    case null:reader.nullExcept("pdf_image",row,13,14,[]);break;
    case "stencil":mask={kind,image:await reader.text(row,13)};break;
    case "colorKey":{
      reader.nullExcept("pdf_image",row,13,14,[]);const ranges:number[]=[];
      for(const child of await reader.children("pdf_image_color_key",1,2,key)){const row=await reader.take("pdf_image_color_key",child.rowid,4);ranges.push(pdfNumber(row,3));}mask={kind,ranges};break;
    }
    default:throw new Error("Unknown PDF image mask");
  }
  return {id:await reader.text(row,1),width:pdfNumber(row,2),height:pdfNumber(row,3),colorSpace:row.values[4]===null?null:await readPdfColorSpace(reader,artifactSqliteInteger(row,4)),bitsPerComponent:pdfNumber(row,5),imageMask:pdfBoolean(row,6),decode:await readPdfReals(reader,"pdf_image_real",key,"decode"),interpolate:pdfBoolean(row,7),codec:await readPdfImageCodec(reader,artifactSqliteInteger(row,8)),data:await reader.bytes(row,9),softMask:await reader.optionalText(row,10),softMaskInData:optionalNumber(row,11),mask,matte:pdfBoolean(row,14)?await readPdfReals(reader,"pdf_image_real",key,"matte"):null,intent:await reader.optionalText(row,15),optionalContent:await reader.optionalText(row,16),structParent:optionalNumber(row,17),extra:await readPdfDictionary(reader,artifactSqliteInteger(row,18))};
}
