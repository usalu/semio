/** 🗓️ The PDF date admission of native JSON shared by the metadata, the annotations and the COS objects. */

import type { PdfDate } from "../../../../../🧬️schema/📸️snapshot/📆️date/🟦️.ts";
import { record, integer } from "../🧰️scalars/🟦️.ts";

/** 📆️ Admit native date fields without narrowing their signed or unsigned widths. */
export function pdfDateFromNativeJson(input:unknown):PdfDate{const row=record(input);return {year:integer(row.year),month:integer(row.month??1),day:integer(row.day??1),hour:integer(row.hour??0),minute:integer(row.minute??0),second:integer(row.second??0),offsetMinutes:row.offsetMinutes==null?null:integer(row.offsetMinutes)};}
