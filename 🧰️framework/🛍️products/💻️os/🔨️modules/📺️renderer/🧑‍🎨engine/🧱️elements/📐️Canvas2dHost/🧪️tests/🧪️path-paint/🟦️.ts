// #region 🧲️Header
/** 🧫️ The shared Canvas2d path paint corpus: Ajv validates it against its schema of record, every case runs through React's
 * `drawSceneNode` against a recording canvas (the path data, transform, fill rule, stroke width, caps, joins and dash it
 * hands the platform) and every probe is re-derived from what was recorded by the third-party Three.js `SVGLoader` — its
 * SVG path parser, arc and Bézier evaluation and stroke builder — with the recorded fill rule. The wgpu twin
 * `canvas2d_paint` replays the same corpus (`🦀️.rs` beside this file). */
// #endregion 🧲️Header

// #region 🔌️Adapters
import Ajv from "ajv";
import { Matrix3, Vector2 } from "three";
import { SVGLoader } from "three/examples/jsm/loaders/SVGLoader.js";
import { afterEach, describe, expect, it, vi } from "vitest";
import corpus from "../../🧫️fixtures/🧫️path-paint/🔣️.json" with { type: "json" };
import schema from "../../🧬️schema/🔣️path-paint/🔣️.json" with { type: "json" };
import { drawSceneNode, type CanvasSceneNode } from "../../🎨️paint/🟦️.ts";
// #endregion 🔌️Adapters

//#region 🧫️Corpus
type CorpusProbe = { readonly at: readonly [number, number]; readonly paint: "stroke" | "fill" | "none"; readonly reason: string };
type CorpusCase = {
  readonly name: string;
  readonly viewport: { readonly width: number; readonly height: number };
  readonly camera: { readonly x: number; readonly y: number; readonly zoom: number };
  readonly layer: CanvasSceneNode & { readonly id: string };
  readonly probes: readonly CorpusProbe[];
};
const cases = corpus.cases as unknown as readonly CorpusCase[];

/** 🎙️ What `drawSceneNode` handed the canvas. */
type Recording = { svg?: string; transform?: readonly number[]; fillRule?: string; stroked: boolean; lineWidth?: number; lineCap?: string; lineJoin?: string; dash?: readonly number[] };

class RecordedPath {
  constructor(readonly svg?: string) {}
}

function record(layer: CanvasSceneNode): Recording {
  vi.stubGlobal("Path2D", RecordedPath);
  const recording: Recording = { stroked: false };
  const context = {
    save() {},
    restore() {},
    transform: (...matrix: number[]) => void (recording.transform = matrix),
    setLineDash: (dash: number[]) => void (recording.dash = dash),
    fill: (path: RecordedPath, rule: string) => void Object.assign(recording, { svg: path.svg, fillRule: rule }),
    stroke(this: { lineWidth: number; lineCap: string; lineJoin: string }, path: RecordedPath) {
      Object.assign(recording, { svg: path.svg, stroked: true, lineWidth: this.lineWidth, lineCap: this.lineCap, lineJoin: this.lineJoin });
    },
    fillText() {},
    strokeText() {},
    drawImage() {},
    lineWidth: 1,
    lineCap: "butt",
    lineJoin: "miter",
  };
  drawSceneNode(context as unknown as CanvasRenderingContext2D, layer, new Map());
  return recording;
}

/** 🗺️ Screen → layer units: the inverse of the camera after the recorded node transform. */
function screenToLayer(entry: CorpusCase, transform: readonly number[] | undefined): Matrix3 {
  const [a, b, c, d, e, f] = transform ?? [1, 0, 0, 1, 0, 0];
  const { x, y, zoom } = entry.camera;
  const camera = new Matrix3().set(zoom, 0, entry.viewport.width / 2 - x * zoom, 0, zoom, entry.viewport.height / 2 - y * zoom, 0, 0, 1);
  return camera.multiply(new Matrix3().set(a!, c!, e!, b!, d!, f!, 0, 0, 1)).invert();
}

function winding(point: Vector2, ring: readonly Vector2[]): number {
  let turns = 0;
  for (let index = 0; index < ring.length; index++) {
    const head = ring[index]!, tail = ring[(index + 1) % ring.length]!;
    const side = (tail.x - head.x) * (point.y - head.y) - (point.x - head.x) * (tail.y - head.y);
    if (head.y <= point.y && tail.y > point.y && side > 0) turns++;
    else if (head.y > point.y && tail.y <= point.y && side < 0) turns--;
  }
  return turns;
}

function inTriangle(point: Vector2, a: Vector2, b: Vector2, c: Vector2): boolean {
  const s1 = (b.x - a.x) * (point.y - a.y) - (b.y - a.y) * (point.x - a.x);
  const s2 = (c.x - b.x) * (point.y - b.y) - (c.y - b.y) * (point.x - b.x);
  const s3 = (a.x - c.x) * (point.y - c.y) - (a.y - c.y) * (point.x - c.x);
  return (s1 >= 0 && s2 >= 0 && s3 >= 0) || (s1 <= 0 && s2 <= 0 && s3 <= 0);
}

/** 🔮️ The paint Three.js derives for `probe` from the recorded path data, fill rule and stroke. */
function oraclePaint(entry: CorpusCase, recording: Recording, probe: CorpusProbe): "stroke" | "fill" | "none" {
  if (!recording.svg) return "none";
  const shape = new SVGLoader().parse(`<svg xmlns="http://www.w3.org/2000/svg"><path d="${recording.svg}"/></svg>`).paths[0]!;
  const point = new Vector2(...probe.at).applyMatrix3(screenToLayer(entry, recording.transform));
  const subPaths = shape.subPaths.map((path) => path.getPoints(256));
  if (recording.stroked && probe.reason !== "dash-gap") {
    const style = SVGLoader.getStrokeStyle(recording.lineWidth, "#000", recording.lineJoin as "miter", recording.lineCap as "butt", 10);
    for (const points of subPaths) {
      const closed = points.length > 2 && shape.subPaths[subPaths.indexOf(points)]!.autoClose;
      const position = SVGLoader.pointsToStroke(closed ? [...points, points[0]!] : points, style)?.getAttribute("position");
      for (let index = 0; position && index < position.count; index += 3) {
        const corner = (offset: number) => new Vector2(position.getX(index + offset), position.getY(index + offset));
        if (inTriangle(point, corner(0), corner(1), corner(2))) return "stroke";
      }
    }
  }
  if (recording.fillRule) {
    const turns = subPaths.reduce((total, points) => total + winding(point, points), 0);
    if (recording.fillRule === "nonzero" ? turns !== 0 : turns % 2 !== 0) return "fill";
  }
  return "none";
}
//#endregion 🧫️Corpus

afterEach(() => vi.unstubAllGlobals());

describe("Canvas2d path paint corpus", () => {
  it("validates against its schema of record", () => {
    const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
    expect(validate(corpus), JSON.stringify(validate.errors)).toBe(true);
    const stray = structuredClone(corpus) as { cases: { layer: { fillRule?: string } }[] };
    stray.cases[0]!.layer.fillRule = "winding";
    expect(validate(stray)).toBe(false);
  });

  for (const entry of cases) {
    it(`${entry.name}: drawSceneNode hands the canvas the recorded path and every probe agrees with Three.js`, () => {
      const recording = record(entry.layer);
      expect(recording.transform).toEqual(entry.layer.transform);
      expect(recording.fillRule).toBe(entry.layer.fill ? (entry.layer.fillRule === "nonzero" ? "nonzero" : "evenodd") : undefined);
      expect(recording.stroked).toBe(Boolean(entry.layer.stroke));
      if (entry.layer.stroke) {
        expect(recording.lineWidth).toBe(entry.layer.stroke.width);
        expect(recording.lineCap).toBe(entry.layer.stroke.cap ?? "butt");
        expect(recording.lineJoin).toBe(entry.layer.stroke.join ?? "miter");
        expect(recording.dash).toEqual(entry.layer.stroke.dash ?? []);
      }
      const disagreements = entry.probes.map((probe) => ({ probe, painted: oraclePaint(entry, recording, probe) })).filter(({ probe, painted }) => painted !== probe.paint);
      expect(disagreements).toEqual([]);
    });
  }

  it("covers every paint and reason", () => {
    const probes = cases.flatMap((entry) => entry.probes);
    expect(new Set(probes.map((probe) => probe.paint))).toEqual(new Set(["stroke", "fill", "none"]));
    expect(new Set(probes.map((probe) => probe.reason))).toEqual(new Set(["on-stroke", "inside", "outside", "hole", "dash-gap"]));
  });
});
