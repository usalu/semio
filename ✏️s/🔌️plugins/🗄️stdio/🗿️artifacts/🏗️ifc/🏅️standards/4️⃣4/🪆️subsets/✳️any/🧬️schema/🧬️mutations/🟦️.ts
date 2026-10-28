import type { IfcComplexType, IfcEntity, IfcSnapshot, IfcValue } from "../📸️snapshot/🟦️.ts";
import {applyIfcDiff, inverseIfcDiff} from "../🔺️diff/🟦️.ts";
import type {IfcDiff, IfcEntityDiff} from "../🔺️diff/🟦️.ts";

/** 📐️ Typed content mutation for `stdio.ifc` — discriminated union on the `mutation` tag. */
export type IfcMutation =
  | { mutation: "setFileDescription"; values: IfcValue[] }
  | { mutation: "setFileName"; values: IfcValue[] }
  | { mutation: "setFileSchema"; values: IfcValue[] }
  | { mutation: "insertEntity"; index: number; entity: IfcEntity }
  | { mutation: "removeEntity"; id: IfcEntity["id"] }
  | { mutation: "setEntityName"; id: IfcEntity["id"]; name: string }
  | { mutation: "setEntityArg"; id: IfcEntity["id"]; index: number; value: IfcValue }
  | { mutation: "insertEntityArg"; id: IfcEntity["id"]; index: number; value: IfcValue }
  | { mutation: "removeEntityArg"; id: IfcEntity["id"]; index: number }
  | { mutation: "setEntityComplex"; id: IfcEntity["id"]; complex: IfcComplexType[] };

/** 🚫️ Refuses mutation kinds outside the exhaustive typed vocabulary. */
function unknownIfcMutation(_mutation: never): never {
  throw new Error("mutation.kind.unknown");
}

/** 🧾️ Declares the maximum undo rows carried by each primitive IFC mutation. */
export function ifcMutationInverseRows(mutation: IfcMutation): number {
  switch (mutation.mutation) {
    case "setFileDescription": case "setFileName": case "setFileSchema":
    case "insertEntity": case "removeEntity": case "setEntityName":
    case "setEntityArg": case "insertEntityArg": case "removeEntityArg": case "setEntityComplex": return 1;
    default: return unknownIfcMutation(mutation);
  }
}

/** 🧩️ Addresses one entity field through its stable identity. */
function entityChange(id: IfcEntity["id"], diff: IfcEntityDiff): IfcDiff {
  return {entities: {modified: [{id, diff}]}};
}

/** 🧬️ Builds the sparse diff directly from a typed mutation and its original target positions. */
export function diffIfcMutation(base: IfcSnapshot, mutation: IfcMutation): IfcDiff {
  switch (mutation.mutation) {
    case "setFileDescription": return {fileDescription: structuredClone(mutation.values)};
    case "setFileName": return {fileName: structuredClone(mutation.values)};
    case "setFileSchema": return {fileSchema: structuredClone(mutation.values)};
    case "insertEntity": return {entities: {added: [{index: mutation.index, entity: structuredClone(mutation.entity)}]}};
    case "removeEntity": {
      const index = base.entities.findIndex(entity => entity.id === mutation.id);
      if (index < 0) throw new Error("mutation.apply.invalid-remove-target");
      return {entities: {removed: [{id: mutation.id, index}]}};
    }
    case "setEntityName": return entityChange(mutation.id, {name: mutation.name});
    case "setEntityArg": return entityChange(mutation.id, {args: {modified: [{index: mutation.index, value: structuredClone(mutation.value)}]}});
    case "insertEntityArg": return entityChange(mutation.id, {args: {added: [{index: mutation.index, value: structuredClone(mutation.value)}]}});
    case "removeEntityArg": return entityChange(mutation.id, {args: {removed: [mutation.index]}});
    case "setEntityComplex": return entityChange(mutation.id, {complex: structuredClone(mutation.complex)});
    default: return unknownIfcMutation(mutation);
  }
}

/** ▶️ Applies one mutation through the checked central IFC diff applier. */
export function applyIfcMutation(base: IfcSnapshot, mutation: IfcMutation): IfcSnapshot {
  return applyIfcDiff(base, diffIfcMutation(base, mutation));
}

/** ↩️ Generates concrete undo mutations from the centrally retained sparse inverse. */
export function inverseIfcMutation(base: IfcSnapshot, mutation: IfcMutation): IfcMutation[] {
  const inverse = inverseIfcDiff(base, diffIfcMutation(base, mutation));
  const modified = inverse.entities?.modified?.[0];
  switch (mutation.mutation) {
    case "setFileDescription": return [{mutation: "setFileDescription", values: inverse.fileDescription!}];
    case "setFileName": return [{mutation: "setFileName", values: inverse.fileName!}];
    case "setFileSchema": return [{mutation: "setFileSchema", values: inverse.fileSchema!}];
    case "insertEntity": return [{mutation: "removeEntity", id: inverse.entities!.removed![0]!.id}];
    case "removeEntity": {
      const added = inverse.entities!.added![0]!;
      return [{mutation: "insertEntity", index: added.index, entity: added.entity}];
    }
    case "setEntityName": return [{mutation: "setEntityName", id: modified!.id, name: modified!.diff.name!}];
    case "setEntityArg": {
      const argument = modified!.diff.args!.modified![0]!;
      return [{mutation: "setEntityArg", id: modified!.id, index: argument.index, value: argument.value}];
    }
    case "insertEntityArg": return [{mutation: "removeEntityArg", id: modified!.id, index: modified!.diff.args!.removed![0]!}];
    case "removeEntityArg": {
      const argument = modified!.diff.args!.added![0]!;
      return [{mutation: "insertEntityArg", id: modified!.id, index: argument.index, value: argument.value}];
    }
    case "setEntityComplex": return [{mutation: "setEntityComplex", id: modified!.id, complex: modified!.diff.complex!}];
    default: return unknownIfcMutation(mutation);
  }
}
