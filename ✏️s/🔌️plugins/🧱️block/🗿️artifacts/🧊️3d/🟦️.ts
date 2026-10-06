/** 🧱️ Actual canonical Block3d domain records. */
export * from "./🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts";

/** 🔘️ Composed catalog presentation keeps its actual five literal strings. */
export interface Block3dVortexKind{id:string;name:string;label:string;color:string;defaultCableKind:string}
/** 🪟 Per-window-instance view state (representation subset, layout, active utility). */
export interface Block3dWindowView {
  windowId: string;
  representationIds: string[];
  arrangement: string;
  spacing: number;
}

import * as scalar from "../../🧬️schema/🧱️shared/📐️scalar/🟦️.ts";
/** 🔘️ Admit each literal composed catalog kind field. */
export function parseBlock3dVortexKind(v:unknown):Block3dVortexKind{const r=scalar.row(v);return{id:scalar.text(r.id),name:scalar.text(r.name),label:scalar.text(r.label),color:scalar.text(r.color),defaultCableKind:scalar.text(r.defaultCableKind)}}
