import { expect, test } from "bun:test";
import Ajv from "ajv";
import { applyPatch, type Operation } from "fast-json-patch";
import { planRasterRegion, rasterRegionPatch, type RasterRegion, type RasterRegionPlan } from "../🟦️.ts";

const fixture = await Bun.file(new URL("../🧫️fixtures/🔣️.json", import.meta.url)).json();
const schema = await Bun.file(new URL("../🧬️schema/🔣️.json", import.meta.url)).json();
const validate = new Ajv({ strict: true }).compile(schema);

for (const row of fixture.cases) test(`raster region ${row.id}`, () => {
  expect(validate(row.region)).toBe(true);
  const plan = planRasterRegion(row.width, row.height, row.pixels.length, row.region as RasterRegion, { maximumRasterBytes: 1024, maximumPatchBytes: row.patchBytes, maximumPatches: 8 });
  expect(plan.patchCount).toBe(row.patches);
  expect(Object.isFrozen(plan) && Object.isFrozen(plan.region) && Object.isFrozen(plan.region.color)).toBe(true);
  const before = Uint8Array.from(row.pixels);
  const after = before.slice();
  const oracle: Operation[] = [];
  const inverse: Operation[] = [];
  for (let ordinal = 0; ordinal < plan.patchCount; ordinal++) {
    const patch = rasterRegionPatch(plan, before, ordinal);
    if (!patch) continue;
    expect(patch.pixels.length).toBeLessThanOrEqual(row.patchBytes);
    after.set(patch.pixels, patch.index);
    patch.pixels.forEach((value, offset) => {
      const path = `/${patch.index + offset}`;
      oracle.push({ op: "replace", path, value });
      inverse.push({ op: "replace", path, value: before[patch.index + offset] });
    });
  }
  expect([...after]).toEqual(row.expected);
  expect(applyPatch([...before], oracle).newDocument).toEqual(row.expected);
  expect(applyPatch([...after], inverse).newDocument).toEqual([...before]);
  expect(() => rasterRegionPatch(plan, before, plan.patchCount)).toThrow("invalid-ordinal");
  expect(() => rasterRegionPatch(plan, new Uint8Array(0), 0)).toThrow("noncanonical-raster");
  if (row.id === "unchanged") expect(oracle).toHaveLength(0);
  console.log(`[DEBUG] raster ${row.id}: ${plan.patchCount} bounded spans; independent patch and inverse agree`);
});

for (const row of fixture.rejected) test(`raster region refuses ${row.id}`, () => {
  expect(() => planRasterRegion(row.width, row.height, row.byteLength, row.region, { maximumRasterBytes: 1024, maximumPatchBytes: row.patchBytes, maximumPatches: row.maxPatches })).toThrow(row.error);
});

test("raster schema rejects unknown fields and invalid channels", () => {
  expect(validate({ ...fixture.cases[0].region, other: true })).toBe(false);
  expect(validate({ ...fixture.cases[0].region, color: [256, 0, 0, 255] })).toBe(false);
});

test("raster plans retain checked ownership across caller edits and refuse structural copies", () => {
  const contract = fixture.planOwnership;
  const row = fixture.cases.find((entry: { id: string }) => entry.id === contract.case);
  const region = { ...row.region, color: [...row.region.color] } as RasterRegion;
  const plan = planRasterRegion(row.width, row.height, row.pixels.length, region, { maximumRasterBytes: 1024, maximumPatchBytes: row.patchBytes, maximumPatches: 8 });
  (region.color as number[]).splice(0, 4, ...contract.replacementColor);
  const before = Uint8Array.from(row.pixels);
  const patch = rasterRegionPatch(plan, before, 0)!;
  const oracle: Operation[] = Array.from(patch.pixels, (value, offset) => ({ op: "replace", path: `/${patch.index + offset}`, value }));
  expect(applyPatch([...before], oracle).newDocument).toEqual(row.expected);
  for (const candidate of [{ ...plan }, JSON.parse(JSON.stringify(plan)), Object.create(plan), { ...plan, rowsPerPatch: 0, chunksPerRow: 0 }]) {
    expect(() => rasterRegionPatch(candidate as RasterRegionPlan, before, 0)).toThrow(contract.foreignPlanError);
    expect([...before]).toEqual(row.pixels);
  }
  console.log("[DEBUG] raster plan ownership: copied caller input remains stable; four foreign plans refused atomically");
});
