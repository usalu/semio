import type { PdfAdmittedStreamRole } from "../../🪪️stream-roles/🟦️.ts";
/** 📦️ Direct insert-object TypeScript payload. */
import type { ObjRef, PdfCcittParameters, PdfDecimal, PdfDictEntry, PdfObject, PdfPredictor, PdfStreamFilter } from '../../📸️snapshot/🟦️.ts';
export interface InsertObjectMutation {
  mutation: 'insertObject';
  id: ObjRef;
  value: PdfObject;
  index?: number | null;
  admittedStreamRoles?: { removed?: number[]; modified?: { index: number; value: PdfAdmittedStreamRole }[]; added?: { index: number; value: PdfAdmittedStreamRole }[] } | null;
}
