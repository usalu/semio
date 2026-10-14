/** 🔎️ jack direct `set-query` payload mirror of `SetQuery`. */
export interface SetQuery {
  value: string;
}
import type {JackSnapshot} from "../../📸️snapshot/🟦️.ts";
/** 📝️ A query event replaces its parent field and retains all original child and manifest references. */
export function applySetQuery(snapshot:JackSnapshot,mutation:SetQuery):JackSnapshot{return snapshot.query===mutation.value?snapshot:{...snapshot,query:mutation.value};}
/** ↩️ The prior parent query supplies the exact inverse event. */
export function inverseSetQuery(snapshot:JackSnapshot):SetQuery{return{value:snapshot.query};}
