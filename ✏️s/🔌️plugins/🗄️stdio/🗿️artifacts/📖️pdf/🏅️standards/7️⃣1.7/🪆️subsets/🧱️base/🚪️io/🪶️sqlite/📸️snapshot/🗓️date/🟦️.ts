/** 🗓️ The PDF date row codec shared by the document information rows and the COS object rows. */

import type { PdfDate } from "../../../../🧬️schema/📸️snapshot/📆️date/🟦️.ts";
import { PdfProjection, PdfReader, pdfInteger, pdfNumber } from "../🧩️entity/🟦️.ts";

/** 📆️ Project the exact signed and unsigned native fields without date-string normalization. */
export async function writePdfDate(out:PdfProjection,value:PdfDate):Promise<bigint>{return out.insert("pdf_date",[pdfInteger(value.year,32,true),pdfInteger(value.month??1),pdfInteger(value.day??1),pdfInteger(value.hour??0),pdfInteger(value.minute??0),pdfInteger(value.second??0),value.offsetMinutes==null?null:pdfInteger(value.offsetMinutes,32,true)]);}

/** 📥️ Restore each authored native integer field and optional offset. */
export async function readPdfDate(reader:PdfReader,key:bigint):Promise<PdfDate>{const row=await reader.take("pdf_date",key,8);return {year:pdfNumber(row,1,32,true),month:pdfNumber(row,2),day:pdfNumber(row,3),hour:pdfNumber(row,4),minute:pdfNumber(row,5),second:pdfNumber(row,6),offsetMinutes:row.values[7]===null?null:pdfNumber(row,7,32,true)};}
