import {test as register} from "node:test";
export const records: {name:string; passed:boolean; error?:string}[] = [];
export const checks: Promise<void>[] = [];
export const metrics = {pairs: 0, triples: 0, oracleComparisons: 0};
const test = (name:string, run:()=>void) => checks.push(register(name, () => {try {run(); records.push({name,passed:true});} catch (error) {records.push({name,passed:false,error:String(error)}); throw error;}}));
import {deepStrictEqual as strictEqual, throws} from "node:assert/strict";
const canonical = (value: unknown): string => typeof value === "bigint" ? "bigint:" + value : Array.isArray(value) ? "[" + value.map(canonical).join(",") + "]" : value && typeof value === "object" ? "{" + Object.entries(value).sort(([a],[b]) => a.localeCompare(b)).map(([key,item]) => JSON.stringify(key) + ":" + canonical(item)).join(",") + "}" : JSON.stringify(value);
const deepStrictEqual = (actual:unknown, expected:unknown) => strictEqual(canonical(actual), canonical(expected));
import patchLibrary from "fast-json-patch";
import type {Operation} from "fast-json-patch";
const {applyPatch} = patchLibrary;
import {applyIfcDiff, inverseIfcDiff, absorbIfcDiff, type IfcDiff} from "../🟦️.ts";
import type {IfcSnapshot, IfcEntity} from "../../📸️snapshot/🟦️.ts";

const entity = (id: bigint): IfcEntity => ({id, name: "IFCWALL", args: [{kind: "integer", value: 1n}, {kind: "integer", value: 2n}], complex: []});
const original: IfcSnapshot = {schema: "stdio.ifc", header: {fileDescription: [], fileName: [], fileSchema: []}, entities: [entity(1n), entity(2n), entity(3n)]};

/** 🧪️ Generates independent RFC 6902 reference operations for one sparse edit. */
function patches(base: IfcSnapshot, diff: IfcDiff): Operation[] {
  const ops: Operation[] = [];
  for (const field of ["fileDescription", "fileName", "fileSchema"] as const) if (diff[field] !== undefined) ops.push({op: "replace", path: "/header/" + field, value: diff[field]});
  for (const entry of diff.entities?.modified ?? []) {
    const index = base.entities.findIndex(entity => entity.id === entry.id), prefix = "/entities/" + index;
    for (const field of ["name", "complex"] as const) if (entry.diff[field] !== undefined) ops.push({op: "replace", path: prefix + "/" + field, value: entry.diff[field]});
    for (const value of entry.diff.args?.modified ?? []) ops.push({op: "replace", path: prefix + "/args/" + value.index, value: value.value});
    for (const value of [...entry.diff.args?.removed ?? []].sort((a,b) => b-a)) ops.push({op: "remove", path: prefix + "/args/" + value});
    for (const value of [...entry.diff.args?.added ?? []].sort((a,b) => a.index-b.index)) ops.push({op: "add", path: prefix + "/args/" + value.index, value: value.value});
  }
  for (const entry of [...diff.entities?.removed ?? []].sort((a,b) => b.index-a.index)) ops.push({op: "remove", path: "/entities/" + entry.index});
  for (const entry of [...diff.entities?.added ?? []].sort((a,b) => a.index-b.index)) ops.push({op: "add", path: "/entities/" + entry.index, value: entry.entity});
  return ops;
}

/** 🎲️ Enumerates all valid one-operation entity and argument edits. */
function edits(base: IfcSnapshot, newId: bigint): IfcDiff[] {
  const result: IfcDiff[] = [{}];
  for (let index = 0; index <= base.entities.length; index++) result.push({entities: {added: [{index, entity: entity(newId)}]}});
  for (const [index, row] of base.entities.entries()) {
    result.push({entities: {removed: [{id: row.id, index}]}}, {entities: {modified: [{id: row.id, diff: {name: "IFCSLAB"}}]}});
    for (let position = 0; position <= row.args.length; position++) result.push({entities: {modified: [{id: row.id, diff: {args: {added: [{index: position, value: {kind: "string", value: "insert"}}]}}}]}});
    for (let position = 0; position < row.args.length; position++) result.push({entities: {modified: [{id: row.id, diff: {args: {removed: [position]}}}]}}, {entities: {modified: [{id: row.id, diff: {args: {modified: [{index: position, value: {kind: "string", value: "replace"}}]}}}]}});
  }
  return result;
}

test("IFC composition, inverse and immutable input agree with independent JSON Patch", () => {
  const saved = structuredClone(original);
  for (const a of edits(original, 10n)) {
    metrics.oracleComparisons++;
    const mid = applyIfcDiff(original, a);
    deepStrictEqual(mid, applyPatch(structuredClone(original), patches(original, a), true, true).newDocument);
    deepStrictEqual(applyIfcDiff(mid, inverseIfcDiff(original, a)), original);
    for (const b of edits(mid, 20n)) {
      metrics.pairs++; metrics.oracleComparisons++;
      const end = applyIfcDiff(mid, b), combined = absorbIfcDiff(a, b);
      deepStrictEqual(end, applyPatch(structuredClone(mid), patches(mid, b), true, true).newDocument);
      deepStrictEqual(applyIfcDiff(original, combined), end);
      deepStrictEqual(applyIfcDiff(end, inverseIfcDiff(original, combined)), original);
    }
  }
  deepStrictEqual(original, saved);
});

test("IFC entity composition associates over all structural three-edit sequences", () => {
  const structure = (base: IfcSnapshot, id: bigint) => edits(base, id).filter(diff => !(diff.entities?.modified?.length));
  for (const a of structure(original, 10n)) {
    const mid1 = applyIfcDiff(original, a);
    for (const b of structure(mid1, 20n)) {
      const mid2 = applyIfcDiff(mid1, b);
      for (const c of structure(mid2, 30n)) {
        metrics.triples++;
        const expected = applyIfcDiff(mid2, c);
        deepStrictEqual(applyIfcDiff(original, absorbIfcDiff(absorbIfcDiff(a, b), c)), expected);
        deepStrictEqual(applyIfcDiff(original, absorbIfcDiff(a, absorbIfcDiff(b, c))), expected);
      }
    }
  }
});

test("IFC same-id replacement can move an entity and invert exactly", () => {
  const remove: IfcDiff = {entities: {removed: [{id: 1n, index: 0}]}};
  const insert: IfcDiff = {entities: {added: [{index: 2, entity: {...entity(1n), name: "IFCSLAB"}}]}};
  const combined = absorbIfcDiff(remove, insert), next = applyIfcDiff(original, combined);
  deepStrictEqual(next.entities.map(row => row.id), [2n, 3n, 1n]);
  deepStrictEqual(applyIfcDiff(next, inverseIfcDiff(original, combined)), original);
});

test("IFC central apply refuses mismatched removal positions before writes", () => {
  const saved = structuredClone(original);
  throws(() => applyIfcDiff(original, {fileName: [], entities: {removed: [{id: 2n, index: 0}]}}), /invalid-remove-target/);
  throws(() => applyIfcDiff(original, {entities: {added: [{index: -1, entity: entity(10n)}]}}), /invalid-add-target/);
  deepStrictEqual(original, saved);
});

test("IFC omitted complex constituents clear through inverse and undefined fields stay absent", () => {
  const base = structuredClone(original);
  delete (base.entities[0] as Partial<IfcEntity>).complex;
  const edit: IfcDiff = {entities: {modified: [{id: 1n, diff: {complex: [{name: "IFCOBJECT", args: []}]}}]}};
  const changed = applyIfcDiff(base, edit), restored = applyIfcDiff(changed, inverseIfcDiff(base, edit));
  deepStrictEqual(restored.entities[0]?.complex, []);
  const first: IfcDiff = {fileName: [{kind: "string", value: "keep"}], entities: {modified: [{id: 1n, diff: {name: "IFCSLAB"}}]}};
  deepStrictEqual(applyIfcDiff(original, absorbIfcDiff(first, {fileName: undefined, entities: undefined})), applyIfcDiff(original, first));
  deepStrictEqual(applyIfcDiff(original, absorbIfcDiff(first, {entities: {modified: [{id: 1n, diff: {name: undefined}}]}})), applyIfcDiff(original, first));
});
