import type { IfcEntity, IfcSnapshot, IfcValue } from "../📸️snapshot/🟦️.ts";
import type { SnapshotPatch } from '../../../../../../../../📇️registry/🧬️contract/✏️editing/🩹️patch/🟦️.ts';

/** 📐️ Typed content mutation for `stdio.ifc` — discriminated union on the `mutation` tag. */
export type IfcMutation =
  | { mutation: "setSnapshot"; snapshot: IfcSnapshot }
  | { readonly mutation: 'patchSnapshot'; readonly patch: SnapshotPatch }
  | { mutation: "setFileDescription"; values: IfcValue[] }
  | { mutation: "setFileName"; values: IfcValue[] }
  | { mutation: "setFileSchema"; values: IfcValue[] }
  | { mutation: "insertEntity"; index: number; entity: IfcEntity }
  | { mutation: "removeEntity"; id: IfcEntity["id"] }
  | { mutation: "setEntityName"; id: IfcEntity["id"]; name: string }
  | { mutation: "setEntityArg"; id: IfcEntity["id"]; index: number; value: IfcValue }
  | { mutation: "insertEntityArg"; id: IfcEntity["id"]; index: number; value: IfcValue }
  | { mutation: "removeEntityArg"; id: IfcEntity["id"]; index: number };
