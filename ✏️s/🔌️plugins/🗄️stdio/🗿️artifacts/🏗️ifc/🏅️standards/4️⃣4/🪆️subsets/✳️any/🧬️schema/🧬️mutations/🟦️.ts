import type { IfcEntity, IfcSnapshot, IfcValue } from "../📸️snapshot/🟦️.ts";

/** 📐️ Typed content mutation for `stdio.ifc` — discriminated union on the `mutation` tag. */
export type IfcMutation =
  | { mutation: "setSnapshot"; snapshot: IfcSnapshot }
  | { mutation: "setFileDescription"; values: IfcValue[] }
  | { mutation: "setFileName"; values: IfcValue[] }
  | { mutation: "setFileSchema"; values: IfcValue[] }
  | { mutation: "insertEntity"; index: number; entity: IfcEntity }
  | { mutation: "removeEntity"; id: IfcEntity["id"] }
  | { mutation: "setEntityName"; id: IfcEntity["id"]; name: string }
  | { mutation: "setEntityArg"; id: IfcEntity["id"]; index: number; value: IfcValue }
  | { mutation: "insertEntityArg"; id: IfcEntity["id"]; index: number; value: IfcValue }
  | { mutation: "removeEntityArg"; id: IfcEntity["id"]; index: number };
