import type { IfcEntity, IfcSnapshot, IfcValue } from "../📸️snapshot/🟦️component.ts";

/** 📐️ Typed content mutation for `stdio.ifc` — discriminated union on the `mutation` tag. */
export type IfcMutation =
  | { mutation: "setSnapshot"; snapshot: IfcSnapshot }
  | { mutation: "setFileDescription"; values: IfcValue[] }
  | { mutation: "setFileName"; values: IfcValue[] }
  | { mutation: "setFileSchema"; values: IfcValue[] }
  | { mutation: "insertEntity"; index: number; entity: IfcEntity }
  | { mutation: "removeEntity"; id: number }
  | { mutation: "setEntityName"; id: number; name: string }
  | { mutation: "setEntityArg"; id: number; index: number; value: IfcValue }
  | { mutation: "insertEntityArg"; id: number; index: number; value: IfcValue }
  | { mutation: "removeEntityArg"; id: number; index: number };
