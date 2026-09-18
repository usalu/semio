/** 🧹️ Direct remove-object TypeScript payload. */
import type { ObjRef } from '../../📸️snapshot/🟦️.ts';
export interface RemoveObjectMutation {
  mutation: 'removeObject';
  id: ObjRef;
}
