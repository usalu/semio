// #region 🔌️Adapters
import { easeCubicInOut, easeCubicOut } from "d3-ease";
import * as clippingModule from "polygon-clipping";
import { MathUtils } from "three";
import { describe, expect, it } from "vitest";
import fixture from "../../../../🧫️fixtures/🥞️layered-overview/🔣️.json" with { type: "json" };
import {
  LAYERED_DECELERATION,
  LAYERED_FOLLOW_FRAME_MS,
  LAYERED_FOLLOW_LERP,
  LAYERED_SETTLE_MS,
  followFactor,
  centeredLastRowCells,
  centeredRowSpan,
  clampOffset,
  type LayeredAxis,
  type LayeredVeil,
  easeInOutCubic,
  easeOutCubic,
  flingStep,
  followStep,
  glideOffset,
  inWindow,
  nearSquareGrid,
  nearestCell,
  neighbourWarmBoot,
  nextWarmBoot,
  restingCell,
  rubberBand,
  settleOffset,
  swipeAxis,
  swipeDragOffset,
  swipeNeighbours,
  swipeWrapTarget,
  stripGrid,
  swipeStep,
  swipeTarget,
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

const never = (value: number | null): number => value ?? Number.POSITIVE_INFINITY;
const lifecycleOf = (given: Partial<Record<keyof LayeredLifecycle, number>>): LayeredLifecycle => resolveLifecycle(given);

/** 🥞️ The layered overview's pure geometry and lifecycle policy, read from `🧫️fixtures/🥞️layered-overview/🔣️.json` (whose swipe vectors the
 * ticket's independent Python twin computed) and judged against `polygon-clipping` (veil area), `d3-ease` (glide and settle easing) and three.js
 * (the follow's damping and the fling's travel). */
describe("🥞️ layered overview geometry", () => {
  

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
      const { offset, settled } = followStep(scenario.current, scenario.target, scenario.elapsedMs);
      expect(offset.x).toBeCloseTo(scenario.offset.x, 12);
      expect(offset.y).toBeCloseTo(scenario.offset.y, 12);
      expect(settled).toBe(scenario.settled);
    });
  }

  for (const scenario of fixture.followFactors) it(`follow factor: ${scenario.name}`, () => expect(followFactor(scenario.elapsedMs)).toBeCloseTo(scenario.factor, 12));

  it("follows by elapsed time: one 60 Hz frame by default, and the same gap closed however the time is cut into frames", () => {
    expect(followFactor()).toBeCloseTo(LAYERED_FOLLOW_LERP, 15);
    expect(LAYERED_FOLLOW_FRAME_MS).toBeCloseTo(1000 / 60, 12);
    for (const frames of [[400], [16, 16, 16, 352], [100, 100, 100, 100], Array.from({ length: 48 }, () => 400 / 48)]) {
      const left = frames.reduce((gap, elapsed) => gap * (1 - followFactor(elapsed)), 1);
      expect(left).toBeCloseTo(1 - followFactor(400), 12);
    }
  });

  it("damps exactly like three.js's MathUtils.damp", () => {
    const lambda = -Math.log(1 - LAYERED_FOLLOW_LERP) / LAYERED_FOLLOW_FRAME_MS;
    for (let elapsed = 0; elapsed <= 500; elapsed += 2.5) {
      expect(followFactor(elapsed)).toBeCloseTo(MathUtils.damp(0, 1, lambda, elapsed), 12);
      expect(followStep({ x: -3, y: 2 }, { x: 5, y: 2.5 }, elapsed).offset.x).toBeCloseTo(MathUtils.damp(-3, 5, lambda, elapsed), 12);
    }
  });

  for (const scenario of fixture.stripTransforms) it(`strip transform: ${scenario.name}`, () => expect(stripTransform(scenario.offset, scenario.grid)).toBe(scenario.transform));

  for (const scenario of fixture.veils) it(`veil: ${scenario.name}`, () => expectVeil(veilClip(scenario.cell, scenario.offset), scenario));

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

  const cellsOf = (count: number) => centeredLastRowCells(count);
  const close = (actual: { readonly x: number; readonly y: number }, expected: { readonly x: number; readonly y: number }) => {
    expect(actual.x).toBeCloseTo(expected.x, 9);
    expect(actual.y).toBeCloseTo(expected.y, 9);
  };

  for (const scenario of fixture.nearestCells) it(`nearest cell: ${scenario.name}`, () => expect(nearestCell(scenario.offset, cellsOf(scenario.count))).toEqual(scenario.cell));
  for (const scenario of fixture.restingCells) it(`resting cell: ${scenario.name}`, () => expect(restingCell(scenario.offset)).toEqual(scenario.cell));
  for (const scenario of fixture.swipeAxes) it(`swipe axis: ${scenario.name}`, () => expect(swipeAxis(scenario.dx, scenario.dy)).toBe(scenario.axis));
  for (const scenario of fixture.swipeSteps) it(`swipe step: ${scenario.name}`, () => expect(swipeStep(scenario.movedPx, scenario.velocityPxPerMs, scenario.viewPx)).toBe(scenario.step));
  for (const scenario of fixture.swipeTargets) it(`swipe target: ${scenario.name}`, () => expect(swipeTarget(scenario.from, scenario.axis as LayeredAxis, scenario.step, cellsOf(scenario.count))).toEqual(scenario.target));
  for (const scenario of fixture.swipeNeighbours) it(`swipe neighbours: ${scenario.name}`, () => expect(swipeNeighbours(scenario.from, cellsOf(scenario.count))).toEqual(scenario.neighbours));

  for (const scenario of fixture.swipeWrapTargets) it(`swipe wrap target: ${scenario.name}`, () => expect(swipeWrapTarget(scenario.from, scenario.axis as LayeredAxis, scenario.step, cellsOf(scenario.count))).toEqual(scenario.landing));

  it("names as a neighbour exactly the landing of a swipe that way, for every cell of every grid", () => {
    for (let count = 1; count <= 60; count += 1) {
      const cells = cellsOf(count);
      for (const from of cells) {
        const named = new Map(swipeNeighbours(from, cells).map(({ direction, cell, slot }) => [direction, { cell, slot }]));
        for (const [direction, axis, step] of [["up", "y", -1], ["left", "x", -1], ["right", "x", 1], ["down", "y", 1]] as const) {
          const landing = swipeWrapTarget(from, axis, step, cells);
          expect(named.get(direction) ?? null).toEqual(landing);
          if (landing !== null) expect(swipeDragOffset(from, axis, step, cells)).toEqual(landing.slot);
        }
      }
    }
  });

  it("wraps like an endless canvas: every page comes in right beside the view, nothing ends but a row or column of one page, and a swipe back along the rows returns", () => {
    for (let count = 2; count <= 60; count += 1) {
      const cells = cellsOf(count);
      const { rows } = stripGrid(cells);
      const occupied = new Set(cells.map((cell) => `${cell.column}:${cell.row}`));
      for (const from of cells) {
        for (const axis of ["x", "y"] as const) {
          for (const step of [-1, 1]) {
            const landing = swipeWrapTarget(from, axis, step, cells);
            const alone = axis === "x" ? cells.filter((cell) => cell.row === from.row).length === 1 : rows === 1;
            expect(landing === null, `${count} panes, ${from.column}:${from.row} ${axis}${step}`).toBe(alone);
            if (landing === null) continue;
            expect(occupied.has(`${landing.cell.column}:${landing.cell.row}`)).toBe(true);
            expect(axis === "x" ? landing.slot.x - from.column : landing.slot.y - from.row).toBe(step);
            if (axis === "x") expect(landing.slot.y).toBe(from.row);
            if (axis === "x") expect(swipeWrapTarget(landing.cell, axis, -step, cells)?.cell).toEqual(from);
          }
        }
      }
    }
  });

  for (const scenario of fixture.rubberBands) it(`rubber band: ${scenario.overshoot} cells past the end`, () => expect(rubberBand(scenario.overshoot)).toBeCloseTo(scenario.pull, 12));
  for (const scenario of fixture.swipeDrags) it(`swipe drag: ${scenario.name}`, () => close(swipeDragOffset(scenario.from, scenario.axis as LayeredAxis, scenario.shift, cellsOf(scenario.count)), scenario.offset));

  for (const scenario of fixture.settles) {
    it(`settle: ${scenario.name}`, () => {
      for (const sample of scenario.samples) {
        const { offset, done } = settleOffset(scenario.from, scenario.to, sample.elapsedMs, scenario.durationMs);
        close(offset, sample.offset);
        expect(done).toBe(sample.done);
        if (done) expect(offset).toBe(scenario.to);
      }
    });
  }

  for (const scenario of fixture.flings) {
    it(`fling: ${scenario.name}`, () => {
      const step = flingStep(scenario.velocityPxPerMs, scenario.elapsedMs);
      expect(step.distance).toBeCloseTo(scenario.distance, 6);
      expect(step.velocity).toBeCloseTo(scenario.velocity, 9);
      expect(step.done).toBe(scenario.done);
    });
  }

  for (const scenario of fixture.neighbourWarmBoots) {
    it(`neighbour warm boot: ${scenario.name}`, () => {
      const ids = Array.from({ length: scenario.count }, (_, index) => `p${index}`);
      expect(neighbourWarmBoot(ids, cellsOf(scenario.count), scenario.at, new Set(scenario.live), scenario.openedId, scenario.budget)).toEqual(scenario.step);
    });
  }

  it("settles exactly like d3-ease's easeCubicOut, in the settle's own time", () => {
    for (let step = 0; step <= 256; step += 1) expect(easeOutCubic(step / 256)).toBeCloseTo(easeCubicOut(step / 256), 12);
    expect(settleOffset({ x: 0, y: 0 }, { x: 1, y: 2 }, LAYERED_SETTLE_MS / 2).offset.y).toBeCloseTo(2 * easeCubicOut(0.5), 12);
  });

  it("flings a scroll as far as three.js's MathUtils.damp carries it toward its resting point, and at any frame rate the same", () => {
    const lambda = -Math.log(LAYERED_DECELERATION);
    for (let elapsed = 0; elapsed <= 3000; elapsed += 12.5) expect(flingStep(1.7, elapsed).distance).toBeCloseTo(MathUtils.damp(0, 1.7 / lambda, lambda, elapsed), 9);
    for (const frames of [[1000], [16, 16, 968], Array.from({ length: 60 }, () => 1000 / 60)]) {
      const travelled = frames.reduce(({ distance, velocity }, elapsed) => {
        const step = flingStep(velocity, elapsed);
        return { distance: distance + step.distance, velocity: step.velocity };
      }, { distance: 0, velocity: 1.7 });
      expect(travelled.distance).toBeCloseTo(flingStep(1.7, 1000).distance, 9);
    }
  });

  it("lands every swipe of every grid on an occupied cell, and a swipe back on the cell it left along the rows", () => {
    for (let count = 1; count <= 60; count += 1) {
      const cells = cellsOf(count);
      const occupied = new Set(cells.map((cell) => `${cell.column}:${cell.row}`));
      for (const from of cells) {
        for (const axis of ["x", "y"] as const) {
          for (const step of [-1, 1]) {
            const target = swipeTarget(from, axis, step, cells);
            if (target === null) continue;
            expect(occupied.has(`${target.column}:${target.row}`)).toBe(true);
            if (axis === "x") expect(swipeTarget(target, axis, -step, cells)).toEqual(from);
          }
        }
      }
    }
  });

  it("never pulls a rubber band a whole cell, and pulls it ever less per cell", () => {
    let previous = 0;
    let gained = Number.POSITIVE_INFINITY;
    for (let overshoot = 0.25; overshoot <= 40; overshoot += 0.25) {
      const pull = rubberBand(overshoot);
      expect(pull).toBeGreaterThan(previous);
      expect(pull).toBeLessThan(1);
      expect(pull - previous).toBeLessThan(gained);
      gained = pull - previous;
      previous = pull;
    }
  });

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
