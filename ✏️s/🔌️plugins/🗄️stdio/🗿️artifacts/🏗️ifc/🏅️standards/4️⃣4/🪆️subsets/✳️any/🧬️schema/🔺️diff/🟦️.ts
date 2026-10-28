import type { IfcComplexType, IfcEntity, IfcValue } from "../📸️snapshot/🟦️.ts";

/** 🔺️ One `args.modified[]`/`added[]` entry — `IfcValue` is a weak/value leaf. */
export interface IfcArgModified {
  index: number;
  value: IfcValue;
}
export interface IfcArgAdded {
  index: number;
  value: IfcValue;
}

/** 🔺️ Index-keyed collection triple for one entity's positional `args`. */
export interface IfcArgsDiff {
  removed?: number[];
  modified?: IfcArgModified[];
  added?: IfcArgAdded[];
}

/** 🔺️ Sparse per-field diff for one `IfcEntity`. `id` is identity, never diffed. */
export interface IfcEntityDiff {
  name?: string;
  args?: IfcArgsDiff;
  complex?: IfcComplexType[];
}

/** ➖️ An original entity identity and position, required for exact composition. */
export interface IfcEntityRemoved {
  id: IfcEntity["id"];
  index: number;
}

export interface IfcEntityModified {
  id: IfcEntity["id"];
  diff: IfcEntityDiff;
}
export interface IfcEntityAdded {
  index: number;
  entity: IfcEntity;
}

/** 📦️ Sparse id-keyed `entities` triple. */
export interface IfcEntitiesDiff {
  removed?: IfcEntityRemoved[];
  modified?: IfcEntityModified[];
  added?: IfcEntityAdded[];
}

/** 🔺️ IfcDiff — no `snapshot` full-replace slot. `schema` is identity and never appears. */
export interface IfcDiff {
  fileDescription?: IfcValue[];
  fileName?: IfcValue[];
  fileSchema?: IfcValue[];
  entities?: IfcEntitiesDiff;
}

/** 🧮️ Counts positions at or before a slot in a sorted index set. */
function countAtOrBefore(indices: readonly number[], index: number): number {
  let low = 0, high = indices.length;
  while (low < high) { const mid = (low + high) >>> 1; if (indices[mid]! <= index) low = mid + 1; else high = mid; }
  return low;
}

/** 📐️ Restores a rank to its position among excluded slots. */
function unrank(rank: number, excluded: readonly number[]): number {
  let index = rank;
  for (;;) { const next = rank + countAtOrBefore(excluded, index); if (next === index) return index; index = next; }
}

/** 🚚️ Transports an original surviving position through removals and insertions. */
function transport(index: number, removed: readonly number[], added: readonly number[]): number {
  return unrank(index - countAtOrBefore(removed, index), added);
}

/** 🔍️ Checks an indexed triple against its original sequence before any writes. */
function validateIndices(length: number, diff: IfcArgsDiff): void {
  const removed = new Set<number>(), modified = new Set<number>(), added = new Set<number>();
  for (const index of diff.removed ?? []) {
    if (!Number.isSafeInteger(index) || index < 0 || index >= length || removed.has(index)) throw new Error("mutation.apply.invalid-remove-index");
    removed.add(index);
  }
  for (const entry of diff.modified ?? []) {
    if (!Number.isSafeInteger(entry.index) || entry.index < 0 || entry.index >= length || removed.has(entry.index) || modified.has(entry.index)) throw new Error("mutation.apply.invalid-modify-index");
    modified.add(entry.index);
  }
  let current = length - removed.size;
  for (const entry of [...diff.added ?? []].sort((a, b) => a.index - b.index)) {
    if (!Number.isSafeInteger(entry.index) || entry.index < 0 || entry.index > current || added.has(entry.index)) throw new Error("mutation.apply.invalid-add-index");
    added.add(entry.index); current++;
  }
}

/** 🧩️ Applies validated positional argument edits to a fresh sequence. */
function applyArgs(base: readonly IfcValue[], diff: IfcArgsDiff): IfcValue[] {
  validateIndices(base.length, diff);
  const next = structuredClone([...base]);
  for (const entry of diff.modified ?? []) next[entry.index] = structuredClone(entry.value);
  for (const index of [...diff.removed ?? []].sort((a, b) => b - a)) next.splice(index, 1);
  for (const entry of [...diff.added ?? []].sort((a, b) => a.index - b.index)) next.splice(entry.index, 0, structuredClone(entry.value));
  return next;
}

/** 🏗️ Applies a per-entity sparse edit without altering the source entity. */
function applyEntity(base: IfcEntity, diff: IfcEntityDiff): IfcEntity {
  return { ...structuredClone(base), ...(diff.name === undefined ? {} : {name: diff.name}), ...(diff.args === undefined ? {} : {args: applyArgs(base.args, diff.args)}), ...(diff.complex === undefined ? {} : {complex: structuredClone(diff.complex)}) };
}

/** ✅️ The checked central TypeScript applier for IFC sparse diffs. */
export function applyIfcDiff(base: import("../📸️snapshot/🟦️.ts").IfcSnapshot, diff: IfcDiff): import("../📸️snapshot/🟦️.ts").IfcSnapshot {
  const entities = diff.entities, byId = new Map(base.entities.map(entity => [entity.id, entity]));
  if (entities && byId.size !== base.entities.length) throw new Error("mutation.apply.duplicate-base-target");
  if (entities) {
    const removed = new Set<IfcEntity["id"]>(), modified = new Set<IfcEntity["id"]>(), added = new Set<IfcEntity["id"]>(), positions = new Set<number>();
    for (const entry of entities.removed ?? []) {
      if (!Number.isSafeInteger(entry.index) || base.entities[entry.index]?.id !== entry.id || removed.has(entry.id)) throw new Error("mutation.apply.invalid-remove-target");
      removed.add(entry.id);
    }
    for (const entry of entities.modified ?? []) {
      const original = byId.get(entry.id);
      if (!original || removed.has(entry.id) || modified.has(entry.id)) throw new Error("mutation.apply.invalid-modify-target");
      modified.add(entry.id); if (entry.diff.args) validateIndices(original.args.length, entry.diff.args);
    }
    let length = base.entities.length - removed.size;
    for (const entry of [...entities.added ?? []].sort((a, b) => a.index - b.index)) {
      if ((byId.has(entry.entity.id) && !removed.has(entry.entity.id)) || added.has(entry.entity.id) || !Number.isSafeInteger(entry.index) || entry.index < 0 || entry.index > length || positions.has(entry.index)) throw new Error("mutation.apply.invalid-add-target");
      added.add(entry.entity.id); positions.add(entry.index); length++;
    }
  }
  const next = structuredClone(base);
  if (diff.fileDescription !== undefined) next.header.fileDescription = structuredClone(diff.fileDescription);
  if (diff.fileName !== undefined) next.header.fileName = structuredClone(diff.fileName);
  if (diff.fileSchema !== undefined) next.header.fileSchema = structuredClone(diff.fileSchema);
  if (entities) {
    const removed = new Set((entities.removed ?? []).map(entry => entry.id)), modified = new Map((entities.modified ?? []).map(entry => [entry.id, entry.diff]));
    next.entities = next.entities.filter(entity => !removed.has(entity.id)).map(entity => modified.has(entity.id) ? applyEntity(entity, modified.get(entity.id)!) : entity);
    for (const entry of [...entities.added ?? []].sort((a, b) => a.index - b.index)) next.entities.splice(entry.index, 0, structuredClone(entry.entity));
  }
  return next;
}

/** ⏪️ Builds the exact sparse inverse against the original snapshot. */
export function inverseIfcDiff(base: import("../📸️snapshot/🟦️.ts").IfcSnapshot, diff: IfcDiff): IfcDiff {
  applyIfcDiff(base, diff);
  const inverse: IfcDiff = {};
  if (diff.fileDescription !== undefined) inverse.fileDescription = structuredClone(base.header.fileDescription);
  if (diff.fileName !== undefined) inverse.fileName = structuredClone(base.header.fileName);
  if (diff.fileSchema !== undefined) inverse.fileSchema = structuredClone(base.header.fileSchema);
  if (diff.entities) {
    inverse.entities = {removed: (diff.entities.added ?? []).map(entry => ({id: entry.entity.id, index: entry.index})), added: (diff.entities.removed ?? []).map(entry => ({index: entry.index, entity: structuredClone(base.entities[entry.index]!)})), modified: (diff.entities.modified ?? []).map(entry => {
      const entity = base.entities.find(entity => entity.id === entry.id)!, source = entry.diff, restored: IfcEntityDiff = {};
      if (source.name !== undefined) restored.name = entity.name;
      if (source.complex !== undefined) restored.complex = structuredClone(entity.complex ?? []);
      if (source.args) {
        const removed = [...source.args.removed ?? []].sort((a, b) => a - b), added = (source.args.added ?? []).map(entry => entry.index).sort((a, b) => a - b);
        restored.args = {removed: added, added: removed.map(index => ({index, value: structuredClone(entity.args[index]!)})), modified: (source.args.modified ?? []).map(entry => ({index: transport(entry.index, removed, added), value: structuredClone(entity.args[entry.index]!)}))};
      }
      return {id: entry.id, diff: restored};
    })};
  }
  return inverse;
}

/** 🔗️ Composes argument triples using their original and final slot coordinates. */
function absorbArgs(first: IfcArgsDiff, second: IfcArgsDiff): IfcArgsDiff {
  const removed1 = [...first.removed ?? []].sort((a, b) => a - b), added1 = (first.added ?? []).map(entry => entry.index).sort((a, b) => a - b), removed2 = [...second.removed ?? []].sort((a, b) => a - b), added2 = (second.added ?? []).map(entry => entry.index).sort((a, b) => a - b);
  const removed = new Set(removed1), modified = new Map((first.modified ?? []).map(entry => [entry.index, structuredClone(entry.value)])), added = new Map((first.added ?? []).map(entry => [entry.index, structuredClone(entry.value)]));
  for (const index of removed2) { if (added.has(index)) added.delete(index); else removed.add(unrank(index - countAtOrBefore(added1, index), removed1)); }
  for (const index of removed) modified.delete(index);
  for (const entry of second.modified ?? []) {
    if (added1.includes(entry.index)) { if (added.has(entry.index)) added.set(entry.index, structuredClone(entry.value)); }
    else { const index = unrank(entry.index - countAtOrBefore(added1, entry.index), removed1); if (!removed.has(index)) modified.set(index, structuredClone(entry.value)); }
  }
  return {removed: [...removed].sort((a, b) => a - b), modified: [...modified].map(([index, value]) => ({index, value})), added: [...[...added].map(([index, value]) => ({index: transport(index, removed2, added2), value})), ...structuredClone(second.added ?? [])].sort((a, b) => a.index - b.index)};
}

/** 🪢️ Coalesces entity fields without retaining references to either input diff. */
function absorbEntity(first: IfcEntityDiff, second: IfcEntityDiff): IfcEntityDiff {
  const result = structuredClone(first);
  if (second.name !== undefined) result.name = second.name;
  if (second.complex !== undefined) result.complex = structuredClone(second.complex);
  if (second.args !== undefined) result.args = first.args ? absorbArgs(first.args, second.args) : structuredClone(second.args);
  return result;
}

/** ➕️ Exactly composes sequential sparse IFC diffs without needing the base snapshot. */
export function absorbIfcDiff(first: IfcDiff, second: IfcDiff): IfcDiff {
  const result = structuredClone(first);
  if (second.fileDescription !== undefined) result.fileDescription = structuredClone(second.fileDescription);
  if (second.fileName !== undefined) result.fileName = structuredClone(second.fileName);
  if (second.fileSchema !== undefined) result.fileSchema = structuredClone(second.fileSchema);
  if (second.entities !== undefined) result.entities = structuredClone(second.entities);
  if (!first.entities || !second.entities) return result;
  const a = first.entities, b = second.entities, removed1 = (a.removed ?? []).map(entry => entry.index).sort((x, y) => x - y), added1 = (a.added ?? []).map(entry => entry.index).sort((x, y) => x - y), removed2 = (b.removed ?? []).map(entry => entry.index).sort((x, y) => x - y), added2 = (b.added ?? []).map(entry => entry.index).sort((x, y) => x - y);
  const addedIds = new Set((a.added ?? []).map(entry => entry.entity.id)), removedIds = new Set((b.removed ?? []).map(entry => entry.id));
  const removed = structuredClone(a.removed ?? []);
  for (const entry of b.removed ?? []) if (!addedIds.has(entry.id)) removed.push({id: entry.id, index: unrank(entry.index - countAtOrBefore(added1, entry.index), removed1)});
  const modified = new Map((a.modified ?? []).filter(entry => !removedIds.has(entry.id)).map(entry => [entry.id, structuredClone(entry.diff)]));
  const added = new Map((a.added ?? []).filter(entry => !removedIds.has(entry.entity.id)).map(entry => [entry.entity.id, {index: transport(entry.index, removed2, added2), entity: structuredClone(entry.entity)}]));
  for (const entry of b.modified ?? []) { const insert = added.get(entry.id); if (insert) insert.entity = applyEntity(insert.entity, entry.diff); else modified.set(entry.id, absorbEntity(modified.get(entry.id) ?? {}, entry.diff)); }
  result.entities = {removed: removed.sort((x, y) => x.index - y.index), modified: [...modified].map(([id, diff]) => ({id, diff})), added: [...added.values(), ...structuredClone(b.added ?? [])].sort((x, y) => x.index - y.index)};
  if (!result.entities.removed?.length && !result.entities.modified?.length && !result.entities.added?.length) delete result.entities;
  return result;
}
