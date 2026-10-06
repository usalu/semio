/** 📍️ Handwritten destinations, complete fit scalars and file specifications. */
import type { PdfDestination,PdfDestinationFit,PdfFileSpecification } from "../../../../../🧬️schema/📸️snapshot/🟦️.ts";
import { PdfProjection,PdfReader,pdfInteger,pdfNumber,type PdfCell } from "../../🧩️entity/🟦️.ts";
import { artifactSqliteText } from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";

/** 📍️ Project exactly the fields declared by each native destination fit variant. */
export async function writePdfDestination(out:PdfProjection,value:PdfDestination):Promise<bigint>{
  if(value.kind==="named")return out.insert("pdf_destination",[value.kind,null,value.name,null,null,null,null,null,null,null,null]);
  if(value.kind!=="page"&&value.kind!=="remotePage")throw new Error("Unknown PDF destination");
  const fields:PdfCell[]=Array(7).fill(null);const fit=value.fit;
  switch(fit.kind){case "xyz":fields[0]=fit.left??null;fields[1]=fit.top??null;fields[2]=fit.zoom??null;break;case "fit":case "fitBoundingBox":break;case "fitHorizontal":case "fitBoundingBoxHorizontal":fields[1]=fit.top??null;break;case "fitVertical":case "fitBoundingBoxVertical":fields[0]=fit.left??null;break;case "fitRectangle":if(fit.rect.length!==4)throw new Error("PDF destination rectangle requires four coordinates");fields[3]=fit.rect[0];fields[4]=fit.rect[1];fields[5]=fit.rect[2];fields[6]=fit.rect[3];break;default:throw new Error("Unknown PDF destination fit");}
  return out.insert("pdf_destination",[value.kind,pdfInteger(value.page),null,fit.kind,...fields]);
}
/** 📥️ Reject foreign variant words and preserve optional exceptional scalars. */
export async function readPdfDestination(reader:PdfReader,key:bigint):Promise<PdfDestination>{
  const row=await reader.take("pdf_destination",key,12);const kind=artifactSqliteText(row,1);
  if(kind==="named"){reader.nullExcept("pdf_destination",row,2,12,[3]);return {kind,name:await reader.text(row,3)};}
  if(kind!=="page"&&kind!=="remotePage")throw new Error("Unknown PDF destination");reader.nullExcept("pdf_destination",row,3,4,[]);
  const real=(column:number)=>reader.real("pdf_destination",row,column);const optional=(column:number)=>reader.isNull("pdf_destination",row,column)?null:real(column);
  let fit:PdfDestinationFit;let present:number[];
  switch(artifactSqliteText(row,4)){case "xyz":fit={kind:"xyz",left:optional(5),top:optional(6),zoom:optional(7)};present=[5,6,7];break;case "fit":fit={kind:"fit"};present=[];break;case "fitBoundingBox":fit={kind:"fitBoundingBox"};present=[];break;case "fitHorizontal":fit={kind:"fitHorizontal",top:optional(6)};present=[6];break;case "fitBoundingBoxHorizontal":fit={kind:"fitBoundingBoxHorizontal",top:optional(6)};present=[6];break;case "fitVertical":fit={kind:"fitVertical",left:optional(5)};present=[5];break;case "fitBoundingBoxVertical":fit={kind:"fitBoundingBoxVertical",left:optional(5)};present=[5];break;case "fitRectangle":fit={kind:"fitRectangle",rect:[real(8),real(9),real(10),real(11)]};present=[8,9,10,11];break;default:throw new Error("Unknown PDF destination fit");}
  reader.nullExcept("pdf_destination",row,5,12,present);return {kind,page:pdfNumber(row,2),fit};
}
/** 📂️ Project each authored file specification variant. */
export async function writePdfFileSpecification(out:PdfProjection,value:PdfFileSpecification):Promise<bigint>{switch(value.kind){case "path":return out.insert("pdf_file_specification",[value.kind,value.path]);case "embedded":return out.insert("pdf_file_specification",[value.kind,value.file]);default:throw new Error("Unknown PDF file specification");}}
/** 📥️ Restore the exact path or attachment relation. */
export async function readPdfFileSpecification(reader:PdfReader,key:bigint):Promise<PdfFileSpecification>{const row=await reader.take("pdf_file_specification",key,3);switch(artifactSqliteText(row,1)){case "path":return {kind:"path",path:await reader.text(row,2)};case "embedded":return {kind:"embedded",file:await reader.text(row,2)};default:throw new Error("Unknown PDF file specification");}}
