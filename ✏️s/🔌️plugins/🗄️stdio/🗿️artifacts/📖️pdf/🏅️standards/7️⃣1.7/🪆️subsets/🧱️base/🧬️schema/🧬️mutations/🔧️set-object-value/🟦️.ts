import type { PdfAdmittedStreamRole } from "../../🪪️stream-roles/🟦️.ts";
/** 🔧️ Direct set-object-value TypeScript payload. */
import type { ObjRef, PdfCcittParameters, PdfDecimal, PdfDictEntry, PdfObject, PdfPredictor, PdfStreamFilter } from '../../📸️snapshot/🟦️.ts';
export interface SetObjectValueMutation {
  mutation: 'setObjectValue';
  id: ObjRef;
  value: PdfObject;
  index?: number | null;
  admittedStreamRoles?: { removed?: number[]; modified?: { index: number; value: PdfAdmittedStreamRole }[]; added?: { index: number; value: PdfAdmittedStreamRole }[] } | null;
}
