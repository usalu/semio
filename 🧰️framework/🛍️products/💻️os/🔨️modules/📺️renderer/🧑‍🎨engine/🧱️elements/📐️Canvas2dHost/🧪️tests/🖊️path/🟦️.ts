/** 🖊️ SVG path geometry reaches the platform Path2D without flattening arcs. */
import { afterEach, expect, it, vi } from "vitest";
import { SVGLoader } from "three/examples/jsm/loaders/SVGLoader.js";
import { buildScenePath } from "../../🎨️paint/🟦️.ts";
import fixture from "../../🧫️fixtures/🖊️path/🔣️.json";

class PathCapture {
  constructor(readonly svg?: string) {}
  moveTo() {}
  lineTo() {}
  quadraticCurveTo() {}
  bezierCurveTo() {}
  closePath() {}
}

afterEach(() => vi.unstubAllGlobals());

for (const testCase of fixture.cases) it(`preserves ${testCase.id}`, () => {
  vi.stubGlobal("Path2D", PathCapture);
  const result = buildScenePath(testCase.segments as Parameters<typeof buildScenePath>[0]) as unknown as PathCapture;
  expect(result.svg).toBe(testCase.svg);
});

it("preserves the circular midpoint through the independent Three.js SVG parser", () => {
  vi.stubGlobal("Path2D", PathCapture);
  const result = buildScenePath(fixture.cases[0]!.segments as Parameters<typeof buildScenePath>[0]) as unknown as PathCapture;
  const oracle = new SVGLoader().parse(`<svg xmlns="http://www.w3.org/2000/svg"><path d="${result.svg}"/></svg>`);
  const midpoint = oracle.paths[0]!.subPaths[0]!.getPoint(0.5);
  expect(midpoint.x).toBeCloseTo(Math.sqrt(50), 12);
  expect(midpoint.y).toBeCloseTo(Math.sqrt(50), 12);
  console.info(`[DEBUG] Canvas arc midpoint agrees with Three.js: ${midpoint.x}, ${midpoint.y}`);
});

it("does not construct a path for an empty scene", () => {
  vi.stubGlobal("Path2D", PathCapture);
  expect(buildScenePath([])).toBeNull();
  expect(buildScenePath(undefined)).toBeNull();
});

import { paintDrawingScene, type PathSegment } from "../../../../../../../../../🔨️modules/◻️2d/🟦️.ts";

it("preserves arcs in raster paint and clipping", () => {
  vi.stubGlobal("Path2D", PathCapture);
  const context = {
    clearRect: vi.fn(), save: vi.fn(), restore: vi.fn(), transform: vi.fn(), beginPath: vi.fn(),
    moveTo: vi.fn(), lineTo: vi.fn(), clip: vi.fn(), fill: vi.fn(), stroke: vi.fn(), setLineDash: vi.fn(),
  };
  const segments = fixture.cases[0]!.segments as PathSegment[];
  paintDrawingScene(context as unknown as CanvasRenderingContext2D, {width: 20, height: 20, nodes: [{
    transform: [1, 0, 0, 1, 0, 0], node: {kind: "path", segments}, clip: segments,
    fill: {kind: "solid", color: [1, 0, 0, 1]}, stroke: {color: [0, 0, 0, 1], width: 1, cap: "butt", join: "miter"},
  }]});
  for (const call of [context.clip, context.fill, context.stroke]) expect(call).toHaveBeenCalledWith(expect.objectContaining({svg: fixture.cases[0]!.svg}));
});
