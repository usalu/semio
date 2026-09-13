import assert from "node:assert/strict";
import { createRequire } from "node:module";
import Ajv from "ajv";
import corpus from "./🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

export function testNumericalPageOwners(): void {
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  assert(validate(corpus), JSON.stringify(validate.errors));
  const { applyPatch } = createRequire(import.meta.url)("fast-json-patch");
  const logicalControls = Array.from({ length: corpus.workControlEntries }, (_, index) => index);
  const controls = corpus.workRetirementLanes.map((closeLane) => applyPatch({ logicalControls, closeLane }, [{ op: "remove", path: "/closeLane" }], true, false).newDocument);
  assert.deepEqual(controls[0], controls[1]);
  for (const row of corpus.stickyFaultCases) {
    const before = { fault: null as string | null, item: 0, allocations: 0 };
    const first = applyPatch(before, [{ op: "replace", path: "/fault", value: row.first }], true, false).newDocument;
    const after = applyPatch(first, first.fault === null ? [{ op: "replace", path: "/fault", value: row.later }] : [], true, false).newDocument;
    assert.deepEqual(after, { fault: row.expected, item: 0, allocations: 0 });
  }
  for (const row of corpus.matrixRestoreCases) {
    assert(row.length <= row.rows * row.cols);
    const shape = new DataView(new ArrayBuffer(24));
    shape.setBigUint64(0, BigInt(row.rows), true);
    shape.setBigUint64(8, BigInt(row.cols), true);
    shape.setBigUint64(16, BigInt(row.length), true);
    assert.deepEqual([0, 8, 16].map((offset) => Number(shape.getBigUint64(offset, true))), [row.rows, row.cols, row.length]);
    let item = 0;
    const pages = [];
    do {
      pages.push(item);
      item += Math.min(row.length - item, Math.floor((corpus.pageBytes - corpus.headerBytes - (item === 0 ? 24 : 0)) / 8));
    } while (item < row.length);
    assert.deepEqual(pages, row.pageItems);
  }
  for (const row of corpus.closeCases) {
    const before = ["rows", "columns"];
    const after = applyPatch(before, row.grant === "exact" ? [{ op: "remove", path: "/0" }] : [], true, false).newDocument;
    assert.equal(before.length - after.length, row.expectedRetiredOwners);
  }
  for (const row of corpus.cases) {
    let state = { item: 0, owner: 0 };
    const pages = [];
    do {
      const overhead = corpus.headerBytes + (state.owner === 0 ? corpus.lengthBytes : 0);
      const scalars = Math.min(row.count - state.item, Math.floor((corpus.pageBytes - overhead) / corpus.scalarBytes));
      pages.push({ ...state, bytes: overhead + scalars * corpus.scalarBytes, scalars });
      state = applyPatch(state, [
        { op: "replace", path: "/item", value: state.item + scalars },
        { op: "replace", path: "/owner", value: 1 },
      ], true, false).newDocument;
    } while (state.item < row.count);
    assert.deepEqual(pages, row.pages, row.id);
    const bytes = new DataView(new ArrayBuffer(row.count * 8));
    for (let i = 0; i < row.count; i++) bytes.setFloat64(i * 8, i * 0.25 - 100, true);
    const reconstructed = pages.flatMap((page) => Array.from({ length: page.scalars }, (_, i) => bytes.getFloat64((page.item + i) * 8, true)));
    assert.deepEqual(reconstructed, Array.from({ length: row.count }, (_, i) => i * 0.25 - 100), row.id);
  }
  for (const row of corpus.boundaryCases) {
    const metadata = row.kind === "length" || row.kind === "matrix-shape";
    const width = row.kind === "matrix-shape" ? 24 : row.kind === "pair" ? 12 : row.kind === "u32" || row.kind === "paged-u32" ? 4 : 8;
    assert.equal(row.width, width);
    const view = new DataView(new ArrayBuffer(width));
    if (row.kind === "matrix-shape") for (let i = 0; i < 3; i++) view.setBigUint64(i * 8, 1n, true);
    else if (row.kind === "length") view.setBigUint64(0, 2n, true);
    else if (row.kind === "pair") { view.setUint32(0, 7, true); view.setFloat64(4, 1.5, true); }
    else if (row.kind === "u32" || row.kind === "paged-u32") view.setUint32(0, 7, true);
    else if (row.kind === "u64" || row.kind === "usize") view.setBigUint64(0, 7n, true);
    else view.setFloat64(0, 1.5, true);
    const actual = applyPatch({ committed: false, owner: metadata ? 0 : 1, item: 0, entry: [] }, [
      { op: "replace", path: "/committed", value: row.remaining < width },
      { op: "replace", path: "/owner", value: 1 },
      { op: "replace", path: "/item", value: metadata ? 0 : 1 },
      { op: "replace", path: "/entry", value: Array.from(new Uint8Array(view.buffer)) },
    ], true, false).newDocument;
    assert.deepEqual(actual, row.expected, row.kind);
  }
  for (const row of corpus.restoreFaultCases) {
    const next = row.continuation ? 2043 : 0;
    const prefix = row.continuation ? 0 : 8;
    const bytes = prefix + Math.min(Math.max(0, row.length - next), Math.floor((corpus.pageBytes - corpus.headerBytes - prefix) / 8)) * 8;
    const validPage = new Ajv({ strict: true }).compile({
      type: "object", additionalProperties: false, required: ["owner", "item", "length", "bodyBytes"],
      properties: { owner: { const: row.continuation ? 1 : 0 }, item: { const: next }, length: { type: "integer", minimum: 0, maximum: 8192 }, bodyBytes: { const: bytes } },
    });
    assert(!validPage({ owner: row.owner, item: row.item, length: row.length, bodyBytes: row.bodyBytes }), row.id);
  }
  console.log("[DEBUG] Numerical owners match five scalar pages, twenty entry widths, nine hostile pages, three matrix shapes, three cleanup grants, two first faults and two local retirement positions");
}
