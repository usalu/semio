// #region 🧲️Header
/** 🧫️ The shared Canvas2d gumball dispatch corpus: Ajv validates it against its schema of record (the meta layer resolved
 * by `$id`), every case replays through React's pure gumball algebra — press hit test, live stream/commit/abort, non-live
 * one-shot — and the net motion of every released gesture is re-derived with the third-party `gl-matrix` (affine
 * screen → model map, signed angle, reach ratio). The wgpu twin `canvas2d_gumball` replays the same corpus
 * (`🧪️tests/🧪️wgpu-gumball-dispatch`). */
// #endregion 🧲️Header

// #region 🔌️Adapters
import Ajv from "ajv";
import { glMatrix, mat2d, vec2 } from "gl-matrix";
import { describe, expect, it } from "vitest";
import corpus from "../../🧫️fixtures/🧫️gumball-dispatch/🔣️.json" with { type: "json" };
import schema from "../../🧬️schema/🔣️gumball-dispatch/🔣️.json" with { type: "json" };
import metaSchema from "../../🧬️schema/🔣️gumball-meta/🔣️.json" with { type: "json" };
import {
  canvas2dGumballBegin,
  canvas2dGumballCancel,
  canvas2dGumballDrag,
  canvas2dGumballHandleAt,
  canvas2dGumballRelease,
  type Canvas2dGumballCancelReason,
  type Canvas2dGumballMeta,
  type Canvas2dGumballTransformPayload,
  type Canvas2dGumballView,
} from "../../🧭️gumball/🟦️.tsx";
// #endregion 🔌️Adapters

//#region 🧫️Corpus
type CorpusEvent = { readonly kind: "move" | "release"; readonly x: number; readonly y: number } | { readonly kind: "cancel"; readonly reason: Canvas2dGumballCancelReason };
type CorpusCase = {
  readonly name: string;
  readonly viewport: { readonly width: number; readonly height: number };
  readonly camera: { readonly x: number; readonly y: number; readonly zoom: number };
  readonly layer: { readonly gumball: Canvas2dGumballMeta };
  readonly press: { readonly x: number; readonly y: number };
  readonly events: readonly CorpusEvent[];
  readonly expect: { readonly handle: string | null; readonly dispatches: readonly Canvas2dGumballTransformPayload[] };
};
const cases = corpus.cases as unknown as readonly CorpusCase[];

function viewOf(entry: CorpusCase): Canvas2dGumballView {
  return { camera: entry.camera, viewportWidth: entry.viewport.width, viewportHeight: entry.viewport.height };
}

/** ▶️ One case through the pure algebra: the handle the press grabs and every dispatch the gesture sends. */
function replay(entry: CorpusCase): { readonly handle: string | null; readonly dispatches: readonly Canvas2dGumballTransformPayload[] } {
  const view = viewOf(entry);
  const meta = entry.layer.gumball;
  const handle = canvas2dGumballHandleAt(meta, view, entry.press.x, entry.press.y);
  const dispatches: Canvas2dGumballTransformPayload[] = [];
  if (!handle) return { handle, dispatches };
  let gesture = canvas2dGumballBegin(meta, handle, entry.press.x, entry.press.y);
  for (const event of entry.events) {
    if (event.kind === "cancel") {
      const abort = canvas2dGumballCancel(gesture, event.reason);
      if (abort) dispatches.push(abort);
      break;
    }
    if (event.kind === "release") {
      const commit = canvas2dGumballRelease(gesture, view, event.x, event.y);
      if (commit) dispatches.push(commit);
      break;
    }
    const step = canvas2dGumballDrag(gesture, view, event.x, event.y);
    gesture = step.gesture;
    if (step.dispatch) dispatches.push(step.dispatch);
  }
  return { handle, dispatches };
}

function expectClose(actual: unknown, expected: unknown, tolerance: number, path: string): void {
  if (typeof expected === "number") {
    expect(typeof actual, path).toBe("number");
    expect(Math.abs((actual as number) - expected), `${path}: ${actual} vs ${expected}`).toBeLessThanOrEqual(tolerance);
    return;
  }
  if (Array.isArray(expected)) {
    expect(Array.isArray(actual), path).toBe(true);
    expect((actual as unknown[]).length, path).toBe(expected.length);
    expected.forEach((item, index) => expectClose((actual as unknown[])[index], item, tolerance, `${path}[${index}]`));
    return;
  }
  if (expected && typeof expected === "object") {
    expect(Object.keys(actual as object).sort(), path).toEqual(Object.keys(expected).sort());
    for (const [key, value] of Object.entries(expected)) expectClose((actual as Record<string, unknown>)[key], value, tolerance, `${path}.${key}`);
    return;
  }
  expect(actual, path).toBe(expected);
}

describe("canvas2d gumball dispatch corpus", () => {
  it("is valid against its schema of record, which refuses a pivotless gumball and an undeclared field", () => {
    const validate = new Ajv({ strict: true, allowUnionTypes: true }).addSchema(metaSchema).compile(schema);
    expect(validate(corpus), JSON.stringify(validate.errors)).toBe(true);
    const pivotless = structuredClone(corpus) as { cases: { layer: { gumball: Record<string, unknown> } }[] };
    delete pivotless.cases[0]!.layer.gumball.pivotLayer;
    expect(validate(pivotless)).toBe(false);
    const stray = structuredClone(corpus) as { cases: { layer: { gumball: Record<string, unknown> } }[] };
    stray.cases[0]!.layer.gumball.space = "fem2d";
    expect(validate(stray)).toBe(false);
  });

  for (const entry of cases) {
    it(`replays: ${entry.name}`, () => {
      const actual = replay(entry);
      expect(actual.handle).toBe(entry.expect.handle);
      expectClose(actual.dispatches, entry.expect.dispatches, corpus.tolerance, "dispatches");
    });
  }
});
//#endregion 🧫️Corpus

//#region 🔮️Oracle
glMatrix.setMatrixArrayType(Array);

/** 🔮️ The net motion a released gesture's dispatches add up to: offsets summed, angles summed, factors multiplied. */
function net(dispatches: readonly Canvas2dGumballTransformPayload[]): Record<string, number> {
  const motion = { dx: 0, dy: 0, angle: 0, sx: 1, sy: 1 };
  for (const { args } of dispatches) {
    if (typeof args.dx === "number") motion.dx += args.dx;
    if (typeof args.dy === "number") motion.dy += args.dy;
    if (typeof args.angle === "number") motion.angle += args.angle;
    if (typeof args.sx === "number") motion.sx *= args.sx;
    if (typeof args.sy === "number") motion.sy *= args.sy;
  }
  return motion;
}

/** 🗺️ The gl-matrix affine map from screen-logical pixels to model units of one case. */
function screenToModel(entry: CorpusCase): mat2d {
  const map = entry.layer.gumball.modelToLayer ?? { scale: [1, 1], offset: [0, 0] };
  const zoom = entry.camera.zoom || 1;
  const modelToScreen = mat2d.fromValues(map.scale[0] * zoom, 0, 0, map.scale[1] * zoom, (map.offset[0] - entry.camera.x) * zoom + entry.viewport.width / 2, (map.offset[1] - entry.camera.y) * zoom + entry.viewport.height / 2);
  return mat2d.invert(mat2d.create(), modelToScreen) as mat2d;
}

describe("canvas2d gumball net motion against gl-matrix", () => {
  for (const entry of cases.filter((candidate) => candidate.expect.handle && candidate.events.at(-1)?.kind === "release" && candidate.expect.dispatches.length > 0)) {
    it(`re-derives: ${entry.name}`, () => {
      const release = entry.events.at(-1) as { readonly x: number; readonly y: number };
      const motion = net(replay(entry).dispatches);
      const press = vec2.fromValues(entry.press.x, entry.press.y);
      const end = vec2.fromValues(release.x, release.y);
      const pivotLayer = entry.layer.gumball.pivotLayer;
      const zoom = entry.camera.zoom || 1;
      const pivot = vec2.fromValues((pivotLayer[0] - entry.camera.x) * zoom + entry.viewport.width / 2, (pivotLayer[1] - entry.camera.y) * zoom + entry.viewport.height / 2);
      const handle = entry.expect.handle;
      if (handle === "moveX" || handle === "moveY") {
        const toModel = screenToModel(entry);
        const delta = vec2.sub(vec2.create(), vec2.transformMat2d(vec2.create(), end, toModel), vec2.transformMat2d(vec2.create(), press, toModel));
        expect(Math.abs((handle === "moveX" ? motion.dx : motion.dy)! - (handle === "moveX" ? delta[0] : delta[1]))).toBeLessThan(1e-9);
        expect(handle === "moveX" ? motion.dy : motion.dx).toBe(0);
        return;
      }
      const from = vec2.sub(vec2.create(), press, pivot);
      const to = vec2.sub(vec2.create(), end, pivot);
      if (handle === "rotate") {
        const signed = Math.sign(from[0] * to[1] - from[1] * to[0]) * vec2.angle(from, to);
        const residue = motion.angle! - signed;
        expect(Math.abs(residue - 2 * Math.PI * Math.round(residue / (2 * Math.PI)))).toBeLessThan(1e-9);
        return;
      }
      const ratio = vec2.length(to) / vec2.length(from);
      expect(Math.abs(motion.sx! - (handle === "scaleY" ? 1 : ratio))).toBeLessThan(1e-9);
      expect(Math.abs(motion.sy! - (handle === "scaleX" ? 1 : ratio))).toBeLessThan(1e-9);
    });
  }
});
//#endregion 🔮️Oracle
