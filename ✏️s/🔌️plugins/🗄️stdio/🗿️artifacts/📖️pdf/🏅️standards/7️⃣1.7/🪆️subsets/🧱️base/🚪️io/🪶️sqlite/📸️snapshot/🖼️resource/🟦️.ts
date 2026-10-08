/** 🖼️ Handwritten PDF image codecs, intrinsic octets and exact mask/decode relationships. */
import type { PdfImage,PdfImageBody,PdfImageMask } from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
import { PdfProjection,PdfReader,pdfInteger,pdfNumber,pdfBoolean,type SqliteRow } from "../🧩️entity/🟦️.ts";
import { writePdfDictionary,readPdfDictionary } from "../🧩️cos/🟦️.ts";
import { writePdfColorSpace,readPdfColorSpace,writePdfReals,readPdfReals } from "../🌈️color/🟦️.ts";
import { artifactSqliteInteger,artifactSqliteText } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import {writePdfArtifactReference,readPdfArtifactReference} from "../📦️artifact-reference/🟦️.ts";
export { pdfResourceNumberColumns } from "./🔢️number/🟦️.ts";

function optionalInteger(value:number|null|undefined):bigint|null{return value==null?null:pdfInteger(value);}
function optionalNumber(row:SqliteRow,column:number):number|null{return row.values[column]===null?null:pdfNumber(row,column);}

/** 🖼️ Project every image field, absent/empty matte state and ordered color-key coordinates. */
export async function writePdfImage(out:PdfProjection,image:PdfImage):Promise<bigint>{
  const color=image.colorSpace==null?null:await writePdfColorSpace(out,image.colorSpace);const reference=image.body.kind==="artifact"?await writePdfArtifactReference(out,image.body.reference):null;const extra=await writePdfDictionary(out,image.extra??[]);
  let maskKind:string|null=null;let stencil:string|null=null;
  if(image.mask!=null){switch(image.mask.kind){case "stencil":maskKind=image.mask.kind;stencil=image.mask.image;break;case "colorKey":maskKind=image.mask.kind;break;default:throw new Error("Unknown PDF image mask");}}
  const key=await out.insert("pdf_image",[image.id,pdfInteger(image.width),pdfInteger(image.height),color,pdfInteger(image.bitsPerComponent??0),image.imageMask?1n:0n,image.interpolate?1n:0n,image.body.kind,reference,image.softMask??null,optionalInteger(image.softMaskInData),maskKind,stencil,image.matte==null?0n:1n,image.intent??null,image.optionalContent??null,optionalInteger(image.structParent),extra]);
  if(image.body.kind==="samples"){out.checkRowsAdditional(image.body.values.length);for(const[ordinal,value]of image.body.values.entries())await out.insert("pdf_image_sample",[key,BigInt(ordinal),pdfInteger(value,16)]);}
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
  let body:PdfImageBody;switch(artifactSqliteText(row,8)){case "samples":{reader.nullExcept("pdf_image",row,9,10,[]);const values:number[]=[];for(const child of await reader.children("pdf_image_sample",1,2,key)){const value=await reader.take("pdf_image_sample",child.rowid,4);values.push(pdfNumber(value,3,16));}body={kind:"samples",values};break;}case "artifact":body={kind:"artifact",reference:await readPdfArtifactReference(reader,artifactSqliteInteger(row,9))};break;default:throw new Error("Unknown PDF image body");}
  return {id:await reader.text(row,1),width:pdfNumber(row,2),height:pdfNumber(row,3),colorSpace:row.values[4]===null?null:await readPdfColorSpace(reader,artifactSqliteInteger(row,4)),bitsPerComponent:pdfNumber(row,5),imageMask:pdfBoolean(row,6),decode:await readPdfReals(reader,"pdf_image_real",key,"decode"),interpolate:pdfBoolean(row,7),body,softMask:await reader.optionalText(row,10),softMaskInData:optionalNumber(row,11),mask,matte:pdfBoolean(row,14)?await readPdfReals(reader,"pdf_image_real",key,"matte"):null,intent:await reader.optionalText(row,15),optionalContent:await reader.optionalText(row,16),structParent:optionalNumber(row,17),extra:await readPdfDictionary(reader,artifactSqliteInteger(row,18))};
}
