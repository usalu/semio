// #region 🔌️Adapters
import Ajv from "ajv";
import { easeCubicInOut } from "d3-ease";
import * as clippingModule from "polygon-clipping";
import { describe, expect, it } from "vitest";
import fixture from "../../../../🧫️fixtures/🥞️layered-overview/🔣️.json" with { type: "json" };
import schema from "../../../../🧬️schema/🥞️layered-overview/🔣️.json" with { type: "json" };
import {
  LAYERED_VIEW,
  centeredLastRowCells,
  centeredRowSpan,
  clampOffset,
  coverPlacement,
  glideRect,
  restRect,
  trackTemplate,
  veilForRect,
  viewRect,
  type LayeredRect,
  type LayeredVeil,
  easeInOutCubic,
  followStep,
  glideOffset,
  inWindow,
  nearSquareGrid,
  nextWarmBoot,
  occupiedColumns,
  panesOverBudget,
  panesToRelease,
  pointerOffset,
  resolveLifecycle,
  scheduleIdle,
  stripTransform,
  veilClip,
  veilClipPath,
  veilPolygon,
  warmDelay,
  windowAround,
  type LayeredLifecycle,
} from "../../🟦️.ts";
// #endregion 🔌️Adapters

// #region 🥞️LayeredOverviewGeometry
type Point = readonly [number, number];
type Ring = readonly Point[];

const clipping = (clippingModule as unknown as { readonly default?: typeof clippingModule }).default ?? clippingModule;

/** 🔺️ Area a polygon fills under the `evenodd` rule — the rule `clip-path: polygon(evenodd, …)` paints with: every horizontal slab between two
 * vertex heights is cut at its middle, the crossings are paired left to right, and the paired widths times the slab height are summed (exact for
 * straight edges, because widths vary linearly within a slab). */
function evenOddArea(polygon: Ring): number {
  const edges = polygon.map((from, index) => [from, polygon[(index + 1) % polygon.length]!] as const).filter(([from, to]) => from[1] !== to[1]);
  const heights = [...new Set(polygon.map(([, y]) => y))].sort((a, b) => a - b);
  let area = 0;
  for (let index = 0; index + 1 < heights.length; index += 1) {
    const [low, high] = [heights[index]!, heights[index + 1]!];
    const y = (low + high) / 2;
    const xs = edges.filter(([from, to]) => Math.min(from[1], to[1]) < y && y < Math.max(from[1], to[1])).map(([from, to]) => from[0] + ((y - from[1]) * (to[0] - from[0])) / (to[1] - from[1]));
    xs.sort((a, b) => a - b);
    for (let pair = 0; pair + 1 < xs.length; pair += 2) area += (xs[pair + 1]! - xs[pair]!) * (high - low);
  }
  return area;
}

/** 📐️ Shoelace area of one ring. */
function ringArea(ring: Ring): number {
  return Math.abs(ring.reduce((sum, [x, y], index) => sum + x * ring[(index + 1) % ring.length]![1] - ring[(index + 1) % ring.length]![0] * y, 0)) / 2;
}

/** 🔮️ `polygon-clipping`'s view − hole, as an area: outer rings minus their holes. */
function oracleVeilArea(polygon: Ring): number {
  const view: [number, number][][] = [polygon.slice(0, 5).map(([x, y]) => [x, y] as [number, number])];
  const cut: [number, number][][] = [polygon.slice(5).map(([x, y]) => [x, y] as [number, number])];
  return clipping.difference(view, cut).reduce((sum, part) => sum + ringArea(part[0]!) - part.slice(1).reduce((holes, ring) => holes + ringArea(ring), 0), 0);
}

type VeilScenario = { readonly kind: string; readonly clipPath: string; readonly area: number; readonly hole?: Readonly<Record<"left" | "top" | "right" | "bottom", number>>; readonly polygon?: readonly (readonly number[])[] };

/** 🕳️ One veil vector: kind, clip path, hole, polygon, and its area under the evenodd rule, also as computed by `polygon-clipping`. */
function expectVeil(veil: LayeredVeil, scenario: VeilScenario): void {
  expect(veil.kind).toBe(scenario.kind);
  expect(veilClipPath(veil)).toBe(scenario.clipPath);
  if (veil.kind !== "hole" || !scenario.hole || !scenario.polygon) {
    expect(scenario.area).toBe(veil.kind === "clear" ? 0 : 10000);
    return;
  }
  for (const side of ["left", "top", "right", "bottom"] as const) expect(veil[side]).toBeCloseTo(scenario.hole[side], 12);
  const polygon = scenario.polygon as unknown as Ring;
  expect(veilPolygon(veil)).toEqual(polygon);
  expect(evenOddArea(polygon)).toBeCloseTo(scenario.area, 6);
  expect(oracleVeilArea(polygon)).toBeCloseTo(scenario.area, 6);
}

/** ▭️ A rectangle equals the expected one to twelve decimals. */
function expectRect(rect: LayeredRect, expected: LayeredRect): void {
  for (const key of ["x", "y", "width", "height"] as const) expect(rect[key]).toBeCloseTo(expected[key], 12);
}

const never = (value: number | null): number => value ?? Number.POSITIVE_INFINITY;
const lifecycleOf = (given: Partial<Record<keyof LayeredLifecycle, number>>): LayeredLifecycle => resolveLifecycle(given);

/** 🥞️ The layered overview's pure geometry and lifecycle policy, read from `🧫️fixtures/🥞️layered-overview/🔣️.json` and judged against
 * `polygon-clipping` (veil area) and `d3-ease` (easing). */
describe("🥞️ layered overview geometry", () => {
  it("reads a fixture that satisfies its schema", () => {
    const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  });

  for (const scenario of fixture.grids) it(`grid: ${scenario.name}`, () => expect(nearSquareGrid(scenario.count)).toEqual(scenario.grid));

  it("refuses a grid without a whole positive pane count", () => {
    for (const count of fixture.invalidGridCounts) expect(() => nearSquareGrid(count)).toThrow(/at least one pane/);
  });

  it("keeps every near-square grid within two columns of square and without an empty column", () => {
    for (let count = 1; count <= 200; count += 1) {
      const { columns, rows } = nearSquareGrid(count);
      expect(columns * rows).toBeGreaterThanOrEqual(count);
      expect(columns - rows).toBeGreaterThanOrEqual(0);
      expect(columns - rows).toBeLessThanOrEqual(2);
      expect((columns - 1) * rows).toBeLessThan(count);
    }
  });

  for (const scenario of fixture.rowSpans) it(`row span: ${scenario.name}`, () => expect(centeredRowSpan(scenario.row, scenario.count)).toEqual(scenario.span));

  it("refuses a row outside the grid", () => {
    expect(() => centeredRowSpan(7, 58)).toThrow(/outside/);
    expect(() => centeredRowSpan(-1, 58)).toThrow(/outside/);
  });

  for (const scenario of fixture.cells) it(`cell: ${scenario.name}`, () => expect(centeredLastRowCells(scenario.count)[scenario.index]).toEqual(scenario.cell));

  it("places every pane in its own cell inside the grid, reachable by a clamped pan", () => {
    for (let count = 1; count <= 200; count += 1) {
      const cells = centeredLastRowCells(count);
      const { columns, rows } = nearSquareGrid(count);
      expect(new Set(cells.map((cell) => `${cell.column}/${cell.row}`)).size).toBe(count);
      for (const cell of cells) {
        expect(cell.column).toBeLessThan(columns);
        expect(cell.row).toBeLessThan(rows);
        expect(clampOffset({ x: cell.column, y: cell.row }, cells)).toEqual({ x: cell.column, y: cell.row });
      }
    }
  });

  for (const scenario of fixture.occupied) it(`occupied columns: ${scenario.name}`, () => expect(occupiedColumns(centeredLastRowCells(scenario.count), scenario.y)).toEqual(scenario.span));
  for (const scenario of fixture.clamps) it(`clamp: ${scenario.name}`, () => expect(clampOffset(scenario.offset, centeredLastRowCells(scenario.count))).toEqual(scenario.clamped));
  for (const scenario of fixture.pointers) it(`pointer: ${scenario.name}`, () => expect(pointerOffset(scenario.fx, scenario.fy, scenario.grid)).toEqual(scenario.offset));

  it("eases in and out cubically at the sampled points", () => {
    for (const { t, eased } of fixture.easing) expect(easeInOutCubic(t)).toBeCloseTo(eased, 12);
  });

  it("eases exactly like d3-ease's easeCubicInOut", () => {
    for (let step = 0; step <= 256; step += 1) expect(easeInOutCubic(step / 256)).toBeCloseTo(easeCubicInOut(step / 256), 12);
  });

  for (const scenario of fixture.glides) {
    it(`glide: ${scenario.name}`, () => {
      for (const sample of scenario.samples) {
        const { offset, done } = glideOffset(scenario.from, scenario.to, sample.elapsedMs, scenario.durationMs);
        expect(offset.x).toBeCloseTo(sample.offset.x, 12);
        expect(offset.y).toBeCloseTo(sample.offset.y, 12);
        expect(done).toBe(sample.done);
        if (done) expect(offset).toBe(scenario.to);
      }
    });
  }

  for (const scenario of fixture.follows) {
    it(`follow: ${scenario.name}`, () => {
      const { offset, settled } = followStep(scenario.current, scenario.target);
      expect(offset.x).toBeCloseTo(scenario.offset.x, 12);
      expect(offset.y).toBeCloseTo(scenario.offset.y, 12);
      expect(settled).toBe(scenario.settled);
    });
  }

  for (const scenario of fixture.stripTransforms) it(`strip transform: ${scenario.name}`, () => expect(stripTransform(scenario.offset, scenario.grid)).toBe(scenario.transform));

  for (const scenario of fixture.veils) it(`veil: ${scenario.name}`, () => expectVeil(veilClip(scenario.cell, scenario.offset), scenario));

  for (const scenario of fixture.rests) it(`grid rest: ${scenario.name}`, () => expectRect(restRect(scenario.cell, scenario.grid, scenario.tracks), scenario.rect));

  it("tiles the view with the rest cells of every grid, whatever the tracks", () => {
    for (const tracks of [{}, { columns: [1, 1.5, 1], rows: [1, 1.4, 1] }, { columns: [3], rows: [0.5, 2] }]) {
      const rects = centeredLastRowCells(9).map((cell) => restRect(cell, { columns: 3, rows: 3 }, tracks));
      expect(rects.reduce((sum, rect) => sum + rect.width * rect.height, 0)).toBeCloseTo(1, 12);
      expect(Math.max(...rects.map((rect) => rect.x + rect.width))).toBeCloseTo(1, 12);
      expect(Math.max(...rects.map((rect) => rect.y + rect.height))).toBeCloseTo(1, 12);
    }
  });

  for (const scenario of fixture.invalidTracks) it(`grid rest refuses ${scenario.name}`, () => expect(() => restRect({ column: 0, row: 0 }, scenario.grid, scenario.tracks)).toThrow(/positive/));
  for (const scenario of fixture.trackTemplates) it(`track template: ${scenario.name}`, () => expect(trackTemplate(scenario.weights ?? undefined, scenario.count)).toBe(scenario.template));
  for (const scenario of fixture.views) it(`zoom view: ${scenario.name}`, () => expectRect(viewRect(scenario.rest, scenario.camera), scenario.view));

  for (const scenario of fixture.zooms) {
    it(`zoom: ${scenario.name}`, () => {
      for (const sample of scenario.samples) {
        const { rect, done } = glideRect(scenario.from, scenario.to, sample.elapsedMs, scenario.durationMs);
        expectRect(rect, sample.rect);
        expect(done).toBe(sample.done);
        if (done) expect(rect).toBe(scenario.to);
      }
    });
  }

  for (const scenario of fixture.placements) {
    it(`placement: ${scenario.name}`, () => {
      const placement = coverPlacement(scenario.view);
      expect(placement.transform).toBe(scenario.transform);
      expect(placement.clipPath).toBe(scenario.clipPath);
    });
  }

  it("keeps every placed page undistorted and exactly inside its view rectangle", () => {
    for (const view of fixture.placements.map((scenario) => scenario.view)) {
      const numbers = (text: string) => (text.match(/-?\d+(?:\.\d+)?/gu) ?? []).map(Number);
      const [tx = 0, ty = 0, , scale = 1] = numbers(coverPlacement(view).transform.replace(/^translate3d/u, ""));
      const [top = 0, right = 0, bottom = 0, left = 0] = numbers(coverPlacement(view).clipPath);
      expect(tx / 100 + (left / 100) * scale).toBeCloseTo(view.x, 3);
      expect(ty / 100 + (top / 100) * scale).toBeCloseTo(view.y, 3);
      expect((1 - left / 100 - right / 100) * scale).toBeCloseTo(view.width, 3);
      expect((1 - top / 100 - bottom / 100) * scale).toBeCloseTo(view.height, 3);
    }
  });

  for (const scenario of fixture.rectVeils) it(`grid veil: ${scenario.name}`, () => expectVeil(veilForRect(scenario.rect), scenario));

  it("cuts the hole of a zoom from the page's current view", () => {
    const rest = restRect({ column: 1, row: 1 }, { columns: 3, rows: 3 });
    for (const elapsed of [0, 100, 250, 400, 500]) {
      const camera = glideRect(LAYERED_VIEW, rest, elapsed).rect;
      const view = viewRect(rest, camera);
      const veil = veilForRect(view);
      if (elapsed >= 500) expect(veil.kind).toBe("clear");
      else if (veil.kind === "hole") for (const [side, value] of [["left", view.x], ["top", view.y], ["right", view.x + view.width], ["bottom", view.y + view.height]] as const) expect(veil[side]).toBeCloseTo(value, 12);
      else throw new Error(`expected a hole at ${elapsed} ms, got ${veil.kind}`);
    }
  });

  for (const scenario of fixture.windows) it(`window: ${scenario.name}`, () => expect(windowAround(scenario.offset, scenario.radius)).toEqual(scenario.window));

  for (const scenario of fixture.windowedCells) {
    it(`windowed cells: ${scenario.name}`, () => {
      const window = windowAround(scenario.offset, scenario.radius);
      expect(centeredLastRowCells(scenario.count).filter((cell) => inWindow(cell, window))).toHaveLength(scenario.windowed);
    });
  }

  for (const scenario of fixture.lifecycles) {
    it(`lifecycle: ${scenario.name}`, () => {
      const resolved = lifecycleOf(scenario.given);
      for (const [key, value] of Object.entries(scenario.resolved)) expect(resolved[key as keyof LayeredLifecycle]).toBe(never(value));
    });
  }

  for (const scenario of fixture.warmBoots) it(`warm boot: ${scenario.name}`, () => expect(nextWarmBoot(scenario.ids, new Set(scenario.booted), scenario.liveCount, scenario.openedId, scenario.budget)).toEqual(scenario.step));
  for (const scenario of fixture.warmDelays) it(`warm delay: ${scenario.name}`, () => expect(warmDelay(scenario.warmedBefore, lifecycleOf(scenario.lifecycle))).toBe(scenario.delayMs));
  for (const scenario of fixture.budgets) it(`budget: ${scenario.name}`, () => expect(panesOverBudget(scenario.liveByRecency, new Set(scenario.keep), scenario.budget)).toEqual(scenario.released));

  for (const scenario of fixture.releases) {
    it(`release: ${scenario.name}`, () => {
      expect(panesToRelease(scenario.live, new Set(scenario.keep), scenario.now, { opened: scenario.opened, hidden: scenario.hidden }, lifecycleOf(scenario.lifecycle))).toEqual(scenario.released);
    });
  }

  it("never enters the idle queue before the minimum delay, and cancels both stages", () => {
    const delayed: (() => void)[] = [];
    const cancelled: string[] = [];
    let calls = 0;
    const cancel = scheduleIdle(() => (calls += 1), 1_500, {
      setTimeout: (callback, delayMs) => {
        expect(delayMs).toBe(1_500);
        delayed.push(callback);
        return 1;
      },
      clearTimeout: () => cancelled.push("timeout"),
      requestIdleCallback: (callback, options) => {
        expect(options).toEqual({ timeout: 1_000 });
        callback();
        return 2;
      },
      cancelIdleCallback: () => cancelled.push("idle"),
    });
    expect(calls).toBe(0);
    delayed.shift()?.();
    expect(calls).toBe(1);
    cancel();
    expect(cancelled).toEqual(["timeout", "idle"]);
  });

  it("runs the callback right after the delay where the browser has no idle queue", () => {
    let calls = 0;
    scheduleIdle(() => (calls += 1), 10, { setTimeout: (callback) => (callback(), 1), clearTimeout: () => undefined });
    expect(calls).toBe(1);
  });
});
// #endregion 🥞️LayeredOverviewGeometry
