import assert from "node:assert/strict";
import Ajv2020 from "ajv/dist/2020.js";
import { applyPatch, type Operation } from "fast-json-patch";
import fixture from "./🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

export function testFemPcgWireOracle(): void {
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
  assert(validate(fixture), JSON.stringify(validate.errors));
  assert(!validate({ ...fixture, localWriter: 1 }));
  for (const row of fixture.cases) {
    assert.equal(row.fields.length, row.id === "checkpoint" ? 11 : 5);
    const pages: number[][] = [];
    for (const field of row.fields) {
      assert.equal(field.field, pages.length);
      const width = field.type === "u32" ? 4 : 8;
      const header = new DataView(new ArrayBuffer(40));
      new Uint8Array(header.buffer).set(new TextEncoder().encode(row.magic));
      header.setUint16(8, 1, true);
      header.setUint16(10, row.kind, true);
      header.setUint16(12, field.field, true);
      header.setBigUint64(32, BigInt(field.values.length), true);
      const operations: Operation[] = field.values.map((value) => {
        const scalar = new DataView(new ArrayBuffer(width));
        if (field.type === "u32") scalar.setUint32(0, Number(value), true);
        else if (field.type === "f64") scalar.setFloat64(0, Number(value), true);
        else scalar.setBigUint64(0, BigInt(value), true);
        return { op: "add", path: "/-", value: Array.from(new Uint8Array(scalar.buffer)) };
      });
      const parts = applyPatch([Array.from(new Uint8Array(header.buffer))], operations, true, false).newDocument as number[][];
      const bytes = parts.flat();
      assert.equal(Buffer.from(bytes).toString("hex"), field.hex, row.id + "/" + field.field);
      pages.push(bytes);
    }
    const control = row.fields[0]!.values;
    assert.deepEqual(control.slice(0, 5).map(Number), [fixture.operation, fixture.revision, fixture.generation, fixture.seed, fixture.order]);
    const identitySchema = new Ajv2020({ strict: true }).compile({ type: "array", minItems: 4, maxItems: 4, prefixItems: [fixture.operation, fixture.revision, fixture.generation, fixture.seed].map((value) => ({ const: value })), items: false });
    const identity = control.slice(0, 4).map(Number);
    assert(identitySchema(identity));
    for (let index = 0; index < 4; index += 1) {
      const foreign = applyPatch(identity, [{ op: "replace", path: "/" + index, value: identity[index]! + 1 }], true, false).newDocument;
      assert(!identitySchema(foreign));
    }
    if (row.id === "checkpoint") {
      assert.equal(control.length, fixture.controlNames.length);
      const tol = new DataView(new ArrayBuffer(8));
      tol.setBigUint64(0, BigInt(control[5]!), true);
      assert.equal(tol.getFloat64(0, true), fixture.tolRel);
      tol.setBigUint64(0, BigInt(control[9]!), true);
      assert.equal(tol.getFloat64(0, true), Math.hypot(...fixture.b));
      assert.deepEqual(control.slice(24), ["0", "0"]);
      assert.deepEqual(row.fields[4]!.values.map(Number), fixture.b);
      assert.deepEqual(row.fields[5]!.values.map(Number), fixture.x);
    } else {
      assert.equal(Number(control[5]), row.stage);
      assert.equal(Number(control[6]), row.converged ? 2 : row.coarsePublished ? 1 : 0);
      const residual = row.fields[2]!.values.map(Number);
      assert.deepEqual(row.fields[1]!.values.map(Number), fixture.x);
      assert.deepEqual(row.fields[3]!.values.map(Number), residual.map((value) => -value));
      assert.deepEqual(row.fields[4]!.values.map(Number), fixture.x.map(Math.abs));
    }
  }
  for (const row of fixture.rhsNorm.cases) {
    const norm = Math.hypot(...row.rhs);
    assert(Math.abs(norm - row.norm) <= Math.max(row.norm * 1e-14, Number.MIN_VALUE), row.id);
    const expected = applyPatch({ norm: 0 }, [{ op: "replace", path: "/norm", value: Math.max(row.norm, 1e-300) }], true, false).newDocument;
    assert.deepEqual(expected, { norm: Math.max(row.norm, 1e-300) });
  }
  for (const row of fixture.rhsNorm.invalid) {
    const scalars = row.rhsBits.map((bits) => {
      const view = new DataView(new ArrayBuffer(8));
      view.setBigUint64(0, BigInt(`0x${bits}`), true);
      return view.getFloat64(0, true);
    });
    assert(!Number.isFinite(Math.hypot(...scalars)), row.id);
    assert.equal(row.fault, scalars.every(Number.isFinite) ? "pcg-construction-rhs-norm" : "pcg-construction-rhs-scalar");
  }
  const dense = fixture.denseRestore;
  assert.equal(dense.entries, dense.order * dense.order);
  const reference = applyPatch([], dense.expected.map((_, row) => ({ op: "add", path: "/-", value: (row + 1) / dense.order - (dense.order + 1) / (4 * dense.order) } as Operation)), true, false).newDocument as number[];
  assert.deepEqual(reference, dense.expected);
  for (let row = 0; row < dense.order; row += 1) {
    assert.equal(reference.reduce((sum, value, column) => sum + value * (row === column ? dense.diagonal : dense.offDiagonal), 0), row + 1);
  }
  for (const [width, expected] of [[4, dense.indexPageItems], [8, dense.valuePageItems]] as const) {
    const items = [];
    let item = 0;
    do {
      items.push(item);
      item += Math.min(dense.entries - item, Math.floor((16384 - 32 - (item === 0 ? 8 : 0)) / width));
    } while (item < dense.entries);
    assert.deepEqual(items, expected);
  }
  const canonical = fixture.cases[0]!.fields.map((field) => Array.from(Buffer.from(field.hex, "hex")));
  const exact = new Ajv2020({ strict: true }).compile({ const: canonical });
  for (const row of fixture.faultCases) {
    let pages = structuredClone(canonical);
    if (row.action === "write") {
      const bytes = Uint8Array.from(pages[row.page]!);
      const view = new DataView(bytes.buffer);
      const offset = row.offset!;
      const value = BigInt(row.value!);
      if (row.width === 1) view.setUint8(offset, Number(value));
      else if (row.width === 2) view.setUint16(offset, Number(value), true);
      else if (row.width === 4) view.setUint32(offset, Number(value), true);
      else view.setBigUint64(offset, value, true);
      if (row.id.startsWith("nonfinite") || row.id.startsWith("negative-norm")) {
        const numerical = new Ajv2020({ strict: true }).compile({ type: "number", ...(row.id.startsWith("negative-norm") ? { minimum: 0 } : {}) });
        assert(!numerical(view.getFloat64(offset, true)), row.id);
      }
      pages = applyPatch(pages, [{ op: "replace", path: "/" + row.page, value: Array.from(bytes) }], true, false).newDocument;
    } else if (row.action === "truncate") {
      pages = applyPatch(pages, [{ op: "replace", path: "/" + row.page, value: pages[row.page]!.slice(0, row.length) }], true, false).newDocument;
    } else if (row.action === "remove") {
      pages = applyPatch(pages, [{ op: "remove", path: "/" + row.page }], true, false).newDocument;
    } else {
      pages = applyPatch(pages, [{ op: "add", path: "/-", value: pages[row.page] }], true, false).newDocument;
    }
    assert(!exact(pages), row.id);
    const first = applyPatch({ fault: null }, [{ op: "replace", path: "/fault", value: row.fault }], true, false).newDocument;
    assert.deepEqual(applyPatch(first, [], true, false).newDocument, { fault: row.fault });
  }
  const turns = fixture.cases.map((row) => row.fields.reduce((sum, field) => sum + field.values.length + 4, 2));
  assert.deepEqual(turns, [93, 40, 40]);
  assert.equal(turns.reduce((sum, count) => sum + count, 0), 173);
  console.log("[DEBUG] PCG wire: 21 exact pages agree with DataView and fast-json-patch, including derived negative zero");
}
