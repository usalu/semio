/** 📆️ Handwritten complete native date scalars and document info ownership. */
import type { PdfInfo } from "../../../../../🧬️schema/📸️snapshot/📇️metadata/🟦️.ts";
import { PdfProjection, PdfReader } from "../../🧩️entity/🟦️.ts";
import { writePdfDictionary,readPdfDictionary } from "../../🧩️cos/🟦️.ts";
import { artifactSqliteInteger } from "@semio-tech/framework/io/sqlite-snapshot/artifact";
import { writePdfDate, readPdfDate } from "../../🗓️date/🟦️.ts";

/** 📇️ Project every document info field and its independently owned dates and COS extras. */
export async function writePdfInfo(out:PdfProjection,value:PdfInfo):Promise<bigint>{return out.insert("pdf_document_info",[value.title??null,value.author??null,value.subject??null,value.keywords??null,value.creator??null,value.producer??null,value.creationDate==null?null:await writePdfDate(out,value.creationDate),value.modificationDate==null?null:await writePdfDate(out,value.modificationDate),value.trapped??null,await writePdfDictionary(out,value.extra??[])]);}
/** 📥️ Restore full info without dropping owner fields outside native wire representability. */
export async function readPdfInfo(reader:PdfReader,key:bigint):Promise<PdfInfo>{const row=await reader.take("pdf_document_info",key,11);return {title:await reader.optionalText(row,1),author:await reader.optionalText(row,2),subject:await reader.optionalText(row,3),keywords:await reader.optionalText(row,4),creator:await reader.optionalText(row,5),producer:await reader.optionalText(row,6),creationDate:row.values[7]===null?null:await readPdfDate(reader,artifactSqliteInteger(row,7)),modificationDate:row.values[8]===null?null:await readPdfDate(reader,artifactSqliteInteger(row,8)),trapped:await reader.optionalText(row,9),extra:await readPdfDictionary(reader,artifactSqliteInteger(row,10))};}
