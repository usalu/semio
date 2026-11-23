import { describe, expect, test } from "vitest";
import Ajv from "ajv/dist/2020.js";
import { Box2, Vector2 } from "three";
import schema from "./frame.schema.json";
import cases from "./frame.cases.json";
import { WINDOW_FRAME_GEOMETRY_SCHEMA, createWindowFrameGeometry, normalizeWindowFrameChips, normalizeWindowFrameMetrics, pendingWindowFrameMetrics, windowFrameBorderPath, windowFrameChipRegions, windowFrameContains, windowFrameSafeClearances, type WindowFrameMetrics } from "./🟦️.tsx";

const box = (rect: { x: number; y: number; width: number; height: number }) => new Box2(new Vector2(rect.x, rect.y), new Vector2(rect.x + rect.width, rect.y + rect.height));
const overlap = (a: Box2, b: Box2) => { const size = a.clone().intersect(b).getSize(new Vector2()); return Math.max(0, size.x) * Math.max(0, size.y); };

function seeded(seed: number): () => number {
  let state = seed;
  return () => { state = (state * 1664525 + 1013904223) % 4294967296; return state / 4294967296; };
}

describe("window frame geometry", () => {
  test("neutral cases conform to their schema", () => expect(new Ajv().validate(schema, cases)).toBe(true));
  for (const entry of cases) test(entry.name, () => {
    const metrics = entry.metrics as WindowFrameMetrics;
    const geometry = createWindowFrameGeometry(metrics);
    expect(geometry.schema).toBe(WINDOW_FRAME_GEOMETRY_SCHEMA);
    expect(geometry.frame).toEqual(entry.expected.frame);
    expect(geometry.chipRegions).toEqual(entry.expected.chipRegions);
    expect(geometry.safeClearances).toEqual(entry.expected.safeClearances);
    expect(geometry.borderPath).toBe(entry.expected.borderPath);
    expect(windowFrameChipRegions(metrics)).toEqual(entry.expected.chipRegions);
    expect(windowFrameSafeClearances(metrics)).toEqual(entry.expected.safeClearances);
    expect(windowFrameBorderPath(metrics)).toBe(entry.expected.borderPath);
    for (const point of entry.expected.contains) {
      expect(windowFrameContains(metrics, point.x, point.y), `${point.x},${point.y}`).toBe(point.inside);
      expect(box(geometry.frame).containsPoint(new Vector2(point.x, point.y))).toBe(point.inside);
    }
  });
  test("a geometry without measurement is pending and carries only its clearances", () => {
    const geometry = createWindowFrameGeometry(null, { width: 120, height: 90, topClearance: 24, bottomClearance: 8 });
    expect(geometry.state).toBe("pending");
    expect(geometry.chipRegions).toEqual([]);
    expect(geometry.frame).toEqual({ x: 0, y: 0, width: 120, height: 90 });
    expect(geometry.safeClearances).toEqual({ top: 24, right: 0, bottom: 8, left: 0 });
    expect(pendingWindowFrameMetrics().width).toBe(0);
    expect(createWindowFrameGeometry({ width: 0, height: 40, top: { depth: 4, chips: [] }, bottom: { depth: 0, chips: [] } }).state).toBe("pending");
  });
  test("non-finite input collapses to zero", () => {
    const metrics = normalizeWindowFrameMetrics({ width: Number.NaN, height: Number.POSITIVE_INFINITY, top: { depth: Number.NaN, chips: [{ left: Number.NaN, right: 4 }] }, bottom: { depth: 1, chips: [] } });
    expect(metrics).toEqual({ width: 0, height: 0, top: { depth: 0, chips: [] }, bottom: { depth: 0, chips: [] } });
    expect(windowFrameContains(metrics, Number.NaN, 0)).toBe(false);
    expect(normalizeWindowFrameChips([{ left: 8, right: 2 }], Number.NaN, 10)).toEqual([]);
  });
  test("randomized frames agree with an independent box and brute-force sampling", () => {
    const next = seeded(7);
    const int = (max: number) => Math.floor(next() * (max + 1));
    for (let round = 0; round < 200; round++) {
      const width = 20 + int(200), height = 20 + int(200);
      const chips = (count: number) => Array.from({ length: count }, () => { const left = int(width + 20) - 10; return { left, right: left + 1 + int(60) }; });
      const metrics: WindowFrameMetrics = { width, height, top: { depth: int(height), chips: chips(int(4)) }, bottom: { depth: int(height), chips: chips(int(4)) } };
      const geometry = createWindowFrameGeometry(metrics);
      const frame = box(geometry.frame);
      const regions = geometry.chipRegions.map(region => ({ region, box: box(region) }));
      for (const { region, box: regionBox } of regions) {
        expect(frame.containsBox(regionBox), `${round} ${JSON.stringify(region)}`).toBe(true);
        expect(region.y === 0 || region.y + region.height === height).toBe(true);
      }
      regions.forEach((a, i) => regions.slice(i + 1).forEach(b => expect(overlap(a.box, b.box)).toBe(0)));
      const clearances = windowFrameSafeClearances(metrics);
      expect(clearances.top + clearances.bottom).toBeLessThanOrEqual(height);
      for (const dock of ["top", "bottom"] as const) {
        const depth = dock === "top" ? clearances.top : clearances.bottom;
        for (let x = 0; x < width; x++) {
          const brute = depth > 0.5 && metrics[dock].chips.some(chip => chip.left <= x && x + 1 <= chip.right);
          const covered = regions.some(({ region }) => region.dock === dock && region.x <= x + 0.5 && x + 0.5 <= region.x + region.width);
          if (brute) expect(covered, `${round} ${dock} ${x}`).toBe(true);
          if (!metrics[dock].chips.some(chip => chip.left <= x + 0.5 && x + 0.5 <= chip.right)) expect(covered, `${round} ${dock} ${x}`).toBe(false);
        }
      }
      for (let k = 0; k < 40; k++) {
        const x = next() * (width + 8) - 4, y = next() * (height + 8) - 4;
        expect(windowFrameContains(metrics, x, y)).toBe(frame.containsPoint(new Vector2(x, y)));
      }
    }
  });
});
