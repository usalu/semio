/** 🔢️ Explicit IEEE page geometry field coordinates. */
import { pdfAnnotationNumberColumns } from "../../📌️annotation/🔢️number/🟦️.ts";
import type { Ieee754Column } from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
/** 📄️ Preserve all native page rectangle, unit and duration words. */
export function pdfDocumentNumberColumns(table:string):readonly Ieee754Column[]{if(table!=="pdf_page")return pdfAnnotationNumberColumns(table);return [{index:1,width:64},{index:2,width:64},{index:3,width:64},{index:4,width:64},{index:5,width:64},{index:6,width:64},{index:7,width:64},{index:8,width:64},{index:9,width:64},{index:10,width:64},{index:11,width:64},{index:12,width:64},{index:13,width:64},{index:14,width:64},{index:15,width:64},{index:16,width:64},{index:17,width:64},{index:18,width:64},{index:19,width:64},{index:20,width:64},{index:22,width:64},{index:28,width:64}];}
