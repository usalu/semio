import type { PdfAdmittedStreamRole } from "../../🪪️stream-roles/🟦️.ts";
/** 🧹️ Direct remove-object TypeScript payload. */
import type { ObjRef } from '../../📸️snapshot/🟦️.ts';
export interface RemoveObjectMutation {
  mutation: 'removeObject';
  id: ObjRef;
  admittedStreamRoles?: { removed?: number[]; modified?: { index: number; value: PdfAdmittedStreamRole }[]; added?: { index: number; value: PdfAdmittedStreamRole }[] } | null;
}
