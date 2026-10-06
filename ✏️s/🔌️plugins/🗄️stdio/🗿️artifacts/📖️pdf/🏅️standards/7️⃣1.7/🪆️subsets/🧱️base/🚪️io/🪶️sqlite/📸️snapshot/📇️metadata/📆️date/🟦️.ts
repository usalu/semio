/** 📆️ Handwritten complete native date scalars and document info ownership. */
import type { PdfDate,PdfInfo } from "../../../../../🧬️schema/📸️snapshot/🟦️.ts";
import { PdfProjection,PdfReader,pdfInteger,pdfNumber } from "../../🧩️entity/🟦️.ts";
import { writePdfDictionary,readPdfDictionary } from "../../🧩️cos/🟦️.ts";
import { artifactSqliteInteger } from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";

/** 📆️ Project the exact signed and unsigned native fields without date-string normalization. */
export async function writePdfDate(out:PdfProjection,value:PdfDate):Promise<bigint>{return out.insert("pdf_date",[pdfInteger(value.year,32,true),pdfInteger(value.month??1),pdfInteger(value.day??1),pdfInteger(value.hour??0),pdfInteger(value.minute??0),pdfInteger(value.second??0),value.offsetMinutes==null?null:pdfInteger(value.offsetMinutes,32,true)]);}
/** 📥️ Restore each authored native integer field and optional offset. */
export async function readPdfDate(reader:PdfReader,key:bigint):Promise<PdfDate>{const row=await reader.take("pdf_date",key,8);return {year:pdfNumber(row,1,32,true),month:pdfNumber(row,2),day:pdfNumber(row,3),hour:pdfNumber(row,4),minute:pdfNumber(row,5),second:pdfNumber(row,6),offsetMinutes:row.values[7]===null?null:pdfNumber(row,7,32,true)};}
/** 📇️ Project every document info field and its independently owned dates and COS extras. */
export async function writePdfInfo(out:PdfProjection,value:PdfInfo):Promise<bigint>{return out.insert("pdf_document_info",[value.title??null,value.author??null,value.subject??null,value.keywords??null,value.creator??null,value.producer??null,value.creationDate==null?null:await writePdfDate(out,value.creationDate),value.modificationDate==null?null:await writePdfDate(out,value.modificationDate),value.trapped??null,await writePdfDictionary(out,value.extra??[])]);}
/** 📥️ Restore full info without dropping owner fields outside native wire representability. */
export async function readPdfInfo(reader:PdfReader,key:bigint):Promise<PdfInfo>{const row=await reader.take("pdf_document_info",key,11);return {title:await reader.optionalText(row,1),author:await reader.optionalText(row,2),subject:await reader.optionalText(row,3),keywords:await reader.optionalText(row,4),creator:await reader.optionalText(row,5),producer:await reader.optionalText(row,6),creationDate:row.values[7]===null?null:await readPdfDate(reader,artifactSqliteInteger(row,7)),modificationDate:row.values[8]===null?null:await readPdfDate(reader,artifactSqliteInteger(row,8)),trapped:await reader.optionalText(row,9),extra:await readPdfDictionary(reader,artifactSqliteInteger(row,10))};}
