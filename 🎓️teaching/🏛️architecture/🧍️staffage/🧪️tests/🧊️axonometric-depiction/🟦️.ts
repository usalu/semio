/** 🧊️ The axonometric depiction end to end for the sample figure: a normal DrawingScene in all seven representations at every preset scale; colours, patterns, values and legends of the planar views for the same state; pens by role and scale; level of detail by `maxDenominator`; the posed solids against the kernel's pose of the neutral model; what is to scale; determinism; progress and cancellation; the four-cell sheet; the CAD scene; the command line and the cast manifest.
 *
 * THE REFERENCES: the planar depiction of the same state (`depict` elevation: items `shape:<solid id>`), `modelOf` + kernel `tessellate` + `poseModel` (posed vertices, independent of `placementsOf`), the hand-computed numbers of `🧫️fixtures/🧊️axonometric-depiction`.
 * Tolerances: colours and values exact; posed vertices 1e-9 m; extents 1e-6 m.
 *
 * @see ./🥒️.feature — the scenarios
 */
import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { beforeAll, describe, expect, it } from "vitest";
import { poseModel, tessellate, transformPoint } from "../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { VIEWS, type DrawingItem, type DrawingScene, type View } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { PLANAR_VIEWPOINTS } from "../../🧬️schema/🟦️.ts";
import { AXONOMETRIC_NAMES, defaultDepictionOptions, depict, depictAsync, loadAxonometric, modelScene } from "../../🔨️modules/🎨️depiction/🟦️.ts";
import { BANDS, SHADE_AMOUNTS, bandOf, shaded } from "../../🔨️modules/🎨️depiction/🧊️axonometric/🟦️.ts";
import { edgesOf, joinsOf } from "../../🔨️modules/🎨️depiction/🧊️axonometric/🧭️edges/🟦️.ts";
import { poseSoup, qualityAt, solidsAt } from "../../🔨️modules/🎨️depiction/🧊️axonometric/🧱️mesh/🟦️.ts";
import { axonometricCamera, project } from "../../🔨️modules/🎨️depiction/🔭️views/🟦️.ts";
import { PROJECTIONS, composeSheet, modelSheet, resolveSheet } from "../../🔨️modules/📤️export/📑️sheet/🟦️.ts";
import { cameraTag, parseArguments, parseCamera, parseSheetViews, targetOf } from "../../🔨️modules/📤️export/🎬️cli/🟦️.ts";
import { castManifestIssues } from "../../🔨️modules/📤️export/🎭️cast/🧬️schema/🟦️.ts";
import { writeSvg } from "../../🔨️modules/📤️export/🖼️svg/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { solveStructure } from "../../🔨️modules/🏗️structure/🟦️.ts";
import { solveThermalAt } from "../../🔨️modules/🌡️thermal/🟦️.ts";
import { modelOf } from "../../🔨️modules/🧊️volume/🟦️.ts";
import { Slots, frameDataOf } from "../../🔨️modules/🧊️volume/🧷️frames/🟦️.ts";
import { lineLook } from "../../🔨️modules/🧊️volume/🎨️looks/🟦️.ts";
import { fixture, STAFFAGE_ROOT } from "../🧰️support/🧰️.ts";
import { sampleFigure } from "../🧰️support/🧊️.ts";

beforeAll(loadAxonometric);

type Vectors = { bands: { name: string; normal: number[]; lit: number; band: number }[]; shade: { base: string; bands: string[] }; joins: { visibleCandidateLength: number } };
const vectors = fixture<Vectors>("🧊️axonometric-depiction");
const asset = sampleFigure();
const POSE = "stand";
const SCALES = [50, 100, 200, 500] as const;
const selector = (scale: number) => ({ kind: "scale", scale: { denominator: scale } }) as const;
const stateAt = (scale: number, pose = POSE) => evaluate(asset, { kind: "pose", pose }, selector(scale));
const analysisOf = (view: View, scale: number) => {
  const state = stateAt(scale);
  return view === "thermal" ? { thermal: solveThermalAt(asset, state.query) } : view === "forces" || view === "moments" ? { structure: solveStructure(state) } : {};
};
const axo = (view: View, scale: number, extra: object = {}): DrawingScene => depict(stateAt(scale), { ...defaultDepictionOptions(view, "en", scale, "axonometric"), ...extra }, analysisOf(view, scale));
const paths = (scene: DrawingScene): Extract<DrawingItem, { kind: "path" }>[] => scene.layers.flatMap((l) => l.items).flatMap((i) => (i.kind === "path" ? [i] : []));
const byId = (scene: DrawingScene): Map<string, Extract<DrawingItem, { kind: "path" }>> => new Map(paths(scene).map((p) => [p.id, p]));
const points = (item: Extract<DrawingItem, { kind: "path" }>): { x: number; y: number }[] => item.commands.flatMap((c) => (c.op === "Z" ? [] : [{ x: c.x, y: c.y }]));

describe("🧊️ the axonometric view is a normal DrawingScene in the seven representations", () => {
  for (const view of VIEWS)
    for (const scale of SCALES)
      it(`${view} at 1:${scale}: finite geometry, unique ids, the camera and what is to scale in the metadata`, () => {
        const scene = axo(view, scale);
        expect(scene.metadata.viewpoint).toBe("axonometric");
        expect(scene.metadata.view).toBe(view);
        expect(scene.metadata.fidelity).toBe("physical");
        expect(scene.metadata.mmPerMetre).toBeCloseTo(1000 / scale, 12);
        expect(scene.metadata.projection).toMatchObject({ kind: "isometric", axisScale: { x: expect.closeTo(1, 12), y: expect.closeTo(1, 12), z: expect.closeTo(1, 12) } });
        const items = scene.layers.flatMap((l) => l.items);
        expect(new Set(items.map((i) => i.id)).size).toBe(items.length);
        for (const p of paths(scene)) for (const c of p.commands) if (c.op !== "Z") expect(Number.isFinite(c.x) && Number.isFinite(c.y) && c.x >= -1e-6 && c.y >= -1e-6 && c.x <= scene.widthMm + 1e-6 && c.y <= scene.heightMm + 1e-6, `${p.id} stays on the sheet`).toBe(true);
        expect(paths(scene).filter((p) => p.id.startsWith("shape:")).length).toBeGreaterThan(8);
        expect(scene.layers.some((l) => l.id === "ground")).toBe(true);
        expect(scene.background).toBe(view.startsWith("dark") ? "#001117" : "#f7f3e3");
        if (view === "thermal" || view === "forces" || view === "moments") expect(scene.legends.length).toBeGreaterThan(0);
      });

  it("is deterministic: two runs give equal scenes and equal SVG, and the sync and async entry points agree", async () => {
    for (const view of ["dark-line", "light-solid", "thermal"] as const) {
      const a = axo(view, 100);
      expect(JSON.stringify(axo(view, 100))).toBe(JSON.stringify(a));
      expect(writeSvg(axo(view, 100))).toBe(writeSvg(a));
      expect(JSON.stringify(await depictAsync(stateAt(100), defaultDepictionOptions(view, "en", 100, "axonometric"), analysisOf(view, 100)))).toBe(JSON.stringify(a));
    }
  });

  it("depicts any camera: dimetric and free angles through the same path, with their names in the caption and their axis scales in the metadata", () => {
    const dim = depict(stateAt(100), { ...defaultDepictionOptions("light-line", "en", 100, "axonometric"), axonometric: { kind: "dimetric" } });
    expect(dim.metadata.projection!.axisScale.z).toBeCloseTo(0.5, 12);
    expect(JSON.stringify(dim)).toContain(AXONOMETRIC_NAMES.dimetric.en);
    const free = depict(stateAt(100), { ...defaultDepictionOptions("light-line", "de", 100, "axonometric"), axonometric: { kind: "free", azimuthDeg: -30, elevationDeg: 20 } });
    expect(free.metadata.projection).toMatchObject({ kind: "free", azimuthDeg: -30, elevationDeg: 20 });
    expect(JSON.stringify(free)).toContain(AXONOMETRIC_NAMES.free.de);
    expect(JSON.stringify(dim)).not.toBe(JSON.stringify(free));
  });
});

describe("🧊️ the seven representations paint the colours of the planar views", () => {
  const planar = (view: View, scale: number): Map<string, Extract<DrawingItem, { kind: "path" }>> => byId(depict(stateAt(scale), defaultDepictionOptions(view, "en", scale, "elevation"), analysisOf(view, scale)));
  for (const view of VIEWS)
    it(`${view}: with flat shading every fill, opacity, hatch pattern and value of a solid equals the planar item of the same id`, () => {
      const scale = 50;
      const flat = byId(axo(view, scale, { shading: "flat" }));
      const reference = planar(view, scale);
      let compared = 0;
      for (const solid of asset.volume!.solids) {
        const ours = flat.get(`shape:${solid.id}`);
        const theirs = reference.get(`shape:${solid.id}`);
        if (ours?.fill === undefined || theirs?.fill === undefined) continue;
        expect(ours.fill.color, `${view} ${solid.id}`).toBe(theirs.fill!.color);
        expect(ours.fill.opacity, `${view} ${solid.id} opacity`).toBe(theirs.fill!.opacity);
        expect(ours.fill.pattern, `${view} ${solid.id} pattern`).toBe(theirs.fill!.pattern);
        expect(ours.value, `${view} ${solid.id} value`).toEqual(theirs.value);
        expect(ours.bone).toBe(theirs.bone);
        compared++;
      }
      expect(compared, `${view} solids compared`).toBeGreaterThan(8);
    });

  it("thermal, forces and moments carry the legends of the planar views (quantity, unit, range, ramp)", () => {
    for (const view of ["thermal", "forces", "moments"] as const) {
      const ours = axo(view, 100).legends;
      const theirs = depict(stateAt(100), defaultDepictionOptions(view, "en", 100, "elevation"), analysisOf(view, 100)).legends;
      for (const legend of theirs.slice(0, 1)) {
        const mine = ours.find((l) => l.quantity === legend.quantity)!;
        expect(mine, view).toBeDefined();
        expect([mine.unit, mine.min, mine.max, mine.range, mine.stops]).toEqual([legend.unit, legend.min, legend.max, legend.range, legend.stops]);
      }
    }
  });

  it("the colours are those of the looks the neutral model uses (one source of truth): light-solid, thermal and moments", async () => {
    for (const view of ["light-solid", "thermal", "moments"] as const) {
      const doc = await modelOf(asset, { scale: 50, poses: [POSE], clips: [], views: [view] });
      const flat = byId(axo(view, 50, { shading: "flat" }));
      let compared = 0;
      for (const solid of doc.solids.filter((s) => !s.tags?.includes("glyph"))) {
        const item = flat.get(`shape:${solid.id}`);
        if (item?.fill === undefined) continue;
        const bound = doc.variants.find((v) => v.id === view)!.bindings[solid.material]!;
        const material = doc.materials.find((m) => m.id === bound)!;
        const enc = (c: number): string => Math.round(255 * (c <= 0.0031308 ? 12.92 * c : 1.055 * c ** (1 / 2.4) - 0.055)).toString(16).padStart(2, "0");
        expect(item.fill.color, `${view} ${solid.id}`).toBe(`#${enc(material.baseColor[0]!)}${enc(material.baseColor[1]!)}${enc(material.baseColor[2]!)}`);
        compared++;
      }
      expect(compared).toBeGreaterThan(8);
    }
  });

  it("the banded solid views darken the tone colour in the four documented bands and keep the brightest band exact", () => {
    for (const c of vectors.bands) {
      const length = Math.hypot(...(c.normal as [number, number, number]));
      expect(bandOf(c.normal[0]!, c.normal[1]!, c.normal[2]!), c.name).toBe(c.band);
      expect(bandOf(c.normal[0]! * 3 / length, c.normal[1]! * 3 / length, c.normal[2]! * 3 / length), `${c.name} is scale free`).toBe(c.band);
    }
    expect([0, 1, 2, 3].map((band) => shaded(vectors.shade.base, band))).toEqual(vectors.shade.bands);
    expect(BANDS).toBe(4);
    expect(SHADE_AMOUNTS).toEqual([0.36, 0.24, 0.12, 0]);
    const flat = byId(axo("light-solid", 50, { shading: "flat" }));
    const banded = paths(axo("light-solid", 50)).filter((p) => p.id.startsWith("shape:"));
    for (const item of banded) {
      const base = flat.get(`shape:${item.id.slice(6).replace(/:band-\d+$/u, "")}`)!.fill!.color;
      expect(item.fill!.opacity === undefined ? [0, 1, 2, 3].map((b) => shaded(base, b)) : [base]).toContain(item.fill!.color);
    }
    expect(banded.some((p) => /:band-\d$/u.test(p.id)), "a solid with several bands").toBe(true);
    const lines = paths(axo("light-line", 50)).filter((p) => p.id.startsWith("shape:"));
    expect(new Set(lines.map((p) => p.fill!.color))).toEqual(new Set(["#f7f3e3"]));
  });
});

describe("🧊️ pens, levels of detail and what is to scale", () => {
  const pensOf = (scale: number): Record<string, number | null> => (asset.representations as unknown as Record<string, { pens: Record<string, number | null> }>)[`${scale}`]!.pens;

  for (const scale of SCALES)
    it(`1:${scale}: every line is stroked with the pen of the role of its solid, times 1 in the line views, ½ in the analytic views and in the solid views at 1:100 and larger, none in the solid views at 1:${scale > 100 ? scale : "—"}`, () => {
      const pens = pensOf(scale);
      const roles = new Map(asset.volume!.solids.map((s) => [s.id, s.role]));
      for (const view of VIEWS) {
        const multiplier = view.endsWith("-line") ? 1 : view.endsWith("-solid") ? (scale <= 100 ? 0.5 : 0) : 0.5;
        const scene = axo(view, scale);
        const edges = paths(scene).filter((p) => p.id.startsWith("edge:"));
        if (multiplier === 0) {
          expect(edges, `${view} draws no edges at 1:${scale}`).toHaveLength(0);
          continue;
        }
        expect(edges.length, `${view} edges`).toBeGreaterThan(0);
        for (const edge of edges) {
          const role = roles.get(edge.id.slice(5))!;
          expect(edge.stroke!.widthMm, `${view} ${edge.id}`).toBeCloseTo(pens[role]! * multiplier, 9);
        }
        const ground = paths(scene).find((p) => p.id === "ground:outline")!;
        expect(ground.stroke!.widthMm).toBeCloseTo(pens.ground!, 9);
      }
    });

  it("level of detail: a solid that is shown only at 1:50 (`maxDenominator`) is drawn at 1:50 and absent at 1:100", () => {
    const fine = asset.volume!.solids.filter((s) => s.maxDenominator === 50).map((s) => s.id);
    expect(fine.length).toBeGreaterThan(0);
    expect(solidsAt(asset, 50).map((s) => s.id)).toEqual(expect.arrayContaining(fine));
    expect(solidsAt(asset, 100).map((s) => s.id).filter((id) => fine.includes(id))).toEqual([]);
    const at50 = [...byId(axo("light-line", 50)).keys()].filter((id) => id.startsWith("shape:") && fine.includes(id.slice(6).replace(/:band-\d+$/u, "")));
    const at100 = [...byId(axo("light-line", 100)).keys()].filter((id) => id.startsWith("shape:") && fine.includes(id.slice(6)));
    expect(at50.length).toBeGreaterThan(0);
    expect(at100).toEqual([]);
  });

  it("the tessellation follows the scale: the chord is 0.04 paper millimetres, so a smaller scale has coarser triangles", () => {
    expect(qualityAt(50).chordM).toBeCloseTo(0.002, 12);
    expect(qualityAt(500).chordM).toBeCloseTo(0.02, 12);
    expect(qualityAt(1).chordM).toBe(0.0005);
    expect(qualityAt(10000).chordM).toBe(0.05);
  });

  it("the vertical axis is to scale: the overall dimension is the true height times mm per metre and equals the height of the evaluated state within the catalog tolerance", () => {
    for (const scale of SCALES) {
      const scene = axo("light-line", scale);
      const dimension = paths(scene).find((p) => p.id === "dimension:line")!;
      const ys = points(dimension).map((p) => p.y);
      const lengthMm = Math.max(...ys) - Math.min(...ys);
      expect(lengthMm / scene.metadata.mmPerMetre).toBeCloseTo(dimension.value!.value, 9);
      const state = stateAt(scale);
      const height = state.bounds.maxY - state.bounds.minY;
      expect(Math.abs(dimension.value!.value - height)).toBeLessThanOrEqual(Math.max(0.02 * height, 0.015));
    }
  });

  it("the ground outline is to scale along the axes: its sides measure the footprint of the posed model plus 0.3 m on each side, along the ahead and the lateral axis", async () => {
    const doc = await tessellate(await modelOf(asset, { scale: 100, poses: [POSE], clips: [], views: [] }), qualityAt(100));
    const world = poseModel(doc, POSE).world;
    let x0 = Infinity, x1 = -Infinity, z0 = Infinity, z1 = -Infinity;
    for (const node of doc.nodes) {
      const mesh = doc.meshes.find((m) => m.id === node.mesh);
      if (mesh === undefined || node.id === "skinned") continue;
      for (const primitive of mesh.primitives) for (let i = 0; i < primitive.positions.length; i += 3) {
        const p = transformPoint(world[node.id]!, [primitive.positions[i]!, primitive.positions[i + 1]!, primitive.positions[i + 2]!]);
        x0 = Math.min(x0, p[0]); x1 = Math.max(x1, p[0]); z0 = Math.min(z0, p[2]); z1 = Math.max(z1, p[2]);
      }
    }
    for (const spec of [{ kind: "isometric" }, { kind: "dimetric" }] as const) {
      const state = stateAt(100);
      const cad = modelScene(state, { viewpoint: "axonometric", axonometric: spec });
      const ground = cad.layers.find((l) => l.name === "GROUND")!.polylines[0]!;
      const camera = axonometricCamera(spec);
      const corners = [[x0 - 0.3, z0 - 0.3], [x1 + 0.3, z0 - 0.3], [x1 + 0.3, z1 + 0.3], [x0 - 0.3, z1 + 0.3]].map(([X, Z]) => project("axonometric", { x: Z!, y: 0, z: X! }, camera));
      const drawn = ground.points;
      const onEdge = (p: { x: number; y: number }): boolean => corners.some((c, i) => { const d = corners[(i + 1) % 4]!; const t = Math.max(0, Math.min(1, ((p.x - c.x) * (d.x - c.x) + (p.y - c.y) * (d.y - c.y)) / ((d.x - c.x) ** 2 + (d.y - c.y) ** 2))); return Math.hypot(p.x - (c.x + t * (d.x - c.x)), p.y - (c.y + t * (d.y - c.y))) < 2e-3; });
      for (const p of drawn) expect(onEdge(p), `${spec.kind} ground point ${p.x},${p.y} lies on the projected footprint`).toBe(true);
      expect(corners.filter((c) => drawn.some((p) => Math.hypot(p.x - c.x, p.y - c.y) < 2e-3)).length, `${spec.kind} visible corners`).toBeGreaterThanOrEqual(3);
      const side = Math.hypot(corners[1]!.x - corners[0]!.x, corners[1]!.y - corners[0]!.y);
      expect(side).toBeCloseTo((x1 - x0 + 0.6) * camera.axisScale.z, 9);
      expect(Math.hypot(corners[2]!.x - corners[1]!.x, corners[2]!.y - corners[1]!.y)).toBeCloseTo((z1 - z0 + 0.6) * camera.axisScale.x, 9);
    }
  });
});

describe("🧊️ the volume is posed by the evaluated state", () => {
  it("the posed vertices of every bone equal the kernel's pose of the neutral model (placementsOf against poseModel), in the stand pose and a stride", async () => {
    for (const pose of [POSE, "stride-left-forward"]) {
      const state = stateAt(50, pose);
      const doc = await tessellate(await modelOf(asset, { scale: 50, poses: [pose], clips: [], views: [] }), qualityAt(50));
      const world = poseModel(doc, pose).world;
      const frame = frameDataOf({ asset, views: ["light-line"], theme: "light", mm: 0.05, longestMm: 10, looks: { thermal: undefined, forces: undefined, moments: undefined, fixed: { "light-line": lineLook("light", "light-line") } }, slots: new Slots() }, state, {});
      const soup = poseSoup(asset, frame, new Map(), 50);
      const key = (p: readonly number[]): string => p.map((v) => Math.round(v * 1e7)).join(",");
      for (const bone of asset.rig.bones) {
        const mesh = doc.meshes.find((m) => m.id === doc.nodes.find((n) => n.id === bone.id)?.mesh);
        if (mesh === undefined) continue;
        const kernel = new Set<string>();
        for (const primitive of mesh.primitives) for (let i = 0; i < primitive.positions.length; i += 3) kernel.add(key(transformPoint(world[bone.id]!, [primitive.positions[i]!, primitive.positions[i + 1]!, primitive.positions[i + 2]!])));
        const ours = new Set<string>();
        for (const part of soup.parts.filter((p) => p.bone === bone.id && p.solid!.weights === undefined)) {
          const used = new Set<number>();
          for (let t = part.first; t < part.first + part.count; t++) for (let c = 0; c < 3; c++) used.add(soup.indices[3 * t + c]!);
          for (const v of used) ours.add(key([soup.vertices[3 * v]!, soup.vertices[3 * v + 1]!, soup.vertices[3 * v + 2]!]));
        }
        expect(ours.size, `${pose} ${bone.id}`).toBeGreaterThan(0);
        for (const k of ours) expect(kernel.has(k), `${pose} ${bone.id} vertex ${k}`).toBe(true);
      }
    }
  });

  it("a clip instant and a pose give different pictures of the same asset, one state each", () => {
    const rest = depict(stateAt(50), defaultDepictionOptions("light-line", "en", 50, "axonometric"));
    const walking = depict(evaluate(asset, { kind: "clip", clip: "walk", timeS: 0.3 }, selector(50)), defaultDepictionOptions("light-line", "en", 50, "axonometric"));
    expect(JSON.stringify(walking.layers)).not.toBe(JSON.stringify(rest.layers));
    expect(walking.metadata.query).toEqual({ kind: "clip", clip: "walk", timeS: 0.3 });
  });
});

describe("🧊️ lines: silhouettes, creases and interpenetration curves", () => {
  const bar = (cx: number, cy: number, cz: number, w: number, h: number, d: number): { v: number[]; i: number[] } => {
    const x = [cx - w / 2, cx + w / 2], y = [cy - h / 2, cy + h / 2], z = [cz - d / 2, cz + d / 2];
    const v = [x[0], y[0], z[1], x[1], y[0], z[1], x[1], y[1], z[1], x[0], y[1], z[1], x[1], y[0], z[0], x[0], y[0], z[0], x[0], y[1], z[0], x[1], y[1], z[0], x[0], y[0], z[0], x[0], y[0], z[1], x[0], y[1], z[1], x[0], y[1], z[0], x[1], y[0], z[1], x[1], y[0], z[0], x[1], y[1], z[0], x[1], y[1], z[1], x[0], y[1], z[1], x[1], y[1], z[1], x[1], y[1], z[0], x[0], y[1], z[0], x[0], y[0], z[0], x[1], y[0], z[0], x[1], y[0], z[1], x[0], y[0], z[1]] as number[];
    const i: number[] = [];
    for (let f = 0; f < 6; f++) i.push(4 * f, 4 * f + 1, 4 * f + 2, 4 * f, 4 * f + 2, 4 * f + 3);
    return { v, i };
  };
  const join = (a: ReturnType<typeof bar>, b: ReturnType<typeof bar>) => ({ vertices: Float64Array.from([...a.v, ...b.v]), indices: Uint32Array.from([...a.i, ...b.i.map((n) => n + 24)]), part: Uint32Array.from([...a.i.map(() => 0), ...b.i.map(() => 1)].filter((_, k) => k % 3 === 0)) });

  it("the curves where two crossing bars meet have the hand-computed length and lie on both surfaces", () => {
    const frame = join(bar(0, 0, 0, 6, 1, 1), bar(0, 0, 0, 1, 6, 1.5));
    const found = joinsOf(frame, () => true, 2);
    const length = found.reduce((sum, c) => sum + Math.hypot(c.a[0] - c.b[0], c.a[1] - c.b[1], c.a[2] - c.b[2]), 0);
    expect(length).toBeCloseTo(vectors.joins.visibleCandidateLength, 9);
    for (const c of found) for (const p of [c.a, c.b]) {
      expect(Math.abs(Math.abs(p[0]) - 0.5), "on the vertical bar's side").toBeLessThan(1e-9);
      expect(p[2]).toBeCloseTo(0.5, 9);
    }
  });

  it("a cube seen face-on has four silhouette edges, turned by 45 degrees it has one crease edge between its two visible faces", () => {
    const cube = bar(0, 0, 0, 1, 1, 1);
    const frame = { vertices: Float64Array.from(cube.v), indices: Uint32Array.from(cube.i), part: Uint32Array.from(cube.i.filter((_, k) => k % 3 === 0).map(() => 0)) };
    const edges = edgesOf(frame, () => true);
    expect(edges.filter((e) => e.kind === "silhouette")).toHaveLength(4);
    expect(edges.filter((e) => e.kind === "crease")).toHaveLength(0);
    expect(edges.filter((e) => e.kind === "boundary")).toHaveLength(0);
    const bevel = bar(0, 0, 0, 1, 1, 1);
    const turned = Float64Array.from(bevel.v.map((v, k) => (k % 3 === 0 ? v * Math.SQRT1_2 - bevel.v[k + 2]! * Math.SQRT1_2 : k % 3 === 2 ? bevel.v[k - 2]! * Math.SQRT1_2 + v * Math.SQRT1_2 : v)));
    const creases = edgesOf({ ...frame, vertices: turned }, () => true).filter((e) => e.kind === "crease");
    expect(creases).toHaveLength(1);
  });
});

describe("🧊️ progress and cancellation", () => {
  const request = () => ({ state: stateAt(50), options: defaultDepictionOptions("light-solid", "en", 50, "axonometric") });

  it("reports monotonic progress from 0 to 1000 permille with bilingual labels", () => {
    const { state, options } = request();
    const seen: { done: number; total: number; label: { en: string; de: string } }[] = [];
    depict(state, options, {}, { progress: (done, total, label) => void seen.push({ done, total, label }) });
    expect(seen.length).toBeGreaterThan(3);
    expect(seen[0]!.done).toBe(0);
    expect(seen[seen.length - 1]!.done).toBe(1000);
    for (let i = 1; i < seen.length; i++) expect(seen[i]!.done).toBeGreaterThanOrEqual(seen[i - 1]!.done);
    for (const s of seen) expect(s.label.en.length * s.label.de.length).toBeGreaterThan(0);
  });

  it("a signal aborted before or during the work throws its reason and returns nothing", async () => {
    const { state, options } = request();
    const before = new AbortController();
    before.abort(new Error("stop"));
    expect(() => depict(state, options, {}, { signal: before.signal })).toThrow("stop");
    const during = new AbortController();
    let steps = 0;
    expect(() => depict(state, options, {}, { signal: during.signal, progress: () => (++steps === 2 ? during.abort() : undefined) })).toThrow();
    expect(steps).toBe(2);
    const late = new AbortController();
    await expect(depictAsync(state, options, {}, { signal: late.signal, progress: () => late.abort() })).rejects.toThrow();
  });

  it("the asynchronous form lets the event loop run between the steps", async () => {
    const { state, options } = request();
    let ticks = 0;
    const timer = setInterval(() => void ticks++, 0);
    await depictAsync(state, options);
    clearInterval(timer);
    expect(ticks).toBeGreaterThan(0);
  });
});

describe("🧊️ the multiview sheet gains the axonometric view as its fourth cell", () => {
  const opts = () => {
    const { view: _v, viewpoint: _p, scale: _s, representation: _r, ...rest } = defaultDepictionOptions("light-line", "en", 100, "elevation");
    return rest as never;
  };
  const cellOf = (scene: DrawingScene, prefix: string) => {
    const boxes = paths(scene).filter((p) => p.id.startsWith(`${prefix}:`) && !p.id.includes("caption")).flatMap(points);
    return { minX: Math.min(...boxes.map((p) => p.x)), maxX: Math.max(...boxes.map((p) => p.x)), minY: Math.min(...boxes.map((p) => p.y)), maxY: Math.max(...boxes.map((p) => p.y)) };
  };

  for (const projection of PROJECTIONS)
    it(`${projection}-angle: the cell lies in the free quadrant, at the same scale, from the same state`, () => {
      const query = { kind: "pose", pose: POSE } as const;
      const four = resolveSheet(asset, query, { kind: "scale", denominator: 100 }, "light-line", opts(), projection, "en", { kind: "isometric" });
      const three = resolveSheet(asset, query, { kind: "scale", denominator: 100 }, "light-line", opts(), projection, "en");
      expect(three.sheet.sheet.axonometric).toBeUndefined();
      const info = four.sheet.sheet.axonometric!;
      expect(info.projection.kind).toBe("isometric");
      expect(info.fidelity).toBe("physical");
      const a = cellOf(four.sheet, "axonometric");
      const plan = cellOf(four.sheet, "plan");
      const front = cellOf(four.sheet, "front");
      if (projection === "third") {
        expect(a.minX, "right of the plan").toBeGreaterThanOrEqual(plan.maxX - 1e-6);
        expect(a.maxY, "above the front").toBeLessThanOrEqual(front.minY + 1e-6);
      } else {
        expect(a.maxX, "left of the plan").toBeLessThanOrEqual(plan.minX + 1e-6);
        expect(a.minY, "below the front").toBeGreaterThanOrEqual(front.maxY - 1e-6);
      }
      const svg = writeSvg(four.sheet);
      expect(svg).toContain('viewpoint="elevation+front+plan+axonometric"');
      expect(svg).toContain('axonometric-kind="isometric"');
      expect(four.sheet.metadata.mmPerMetre).toBeCloseTo(10, 12);
      expect(Object.keys(three.sheet.sheet.originsMm)).toEqual(PLANAR_VIEWPOINTS);
    });

  it("the CAD sheet carries the axonometric layers and caption, and a sheet cannot mix scales", () => {
    const state = stateAt(100);
    const cad = modelSheet(state, { projection: "third", denominator: 100, language: "de", axonometric: { kind: "dimetric" } });
    expect(cad.layers.some((l) => l.name.startsWith("AXONOMETRIC-"))).toBe(true);
    expect(cad.layers.find((l) => l.name === "SHEET-LABELS")!.texts.map((t) => t.text)).toContain("Axonometrie");
    const plain = modelSheet(state, { projection: "third", denominator: 100, language: "de" });
    expect(plain.layers.some((l) => l.name.startsWith("AXONOMETRIC-"))).toBe(false);
    const other = depict(stateAt(50), defaultDepictionOptions("light-line", "en", 50, "axonometric"));
    const three = resolveSheet(asset, { kind: "pose", pose: POSE }, { kind: "scale", denominator: 100 }, "light-line", opts(), "third", "en").sheet;
    void three;
    const scenes = Object.fromEntries(PLANAR_VIEWPOINTS.map((v) => [v, depict(stateAt(100), { ...defaultDepictionOptions("light-line", "en", 100, v) })])) as never;
    expect(() => composeSheet(scenes, "third", "en", other)).toThrow(/share one scale/u);
  });
});

describe("🧊️ the CAD scene is the visible linework in the picture plane", () => {
  it("has hidden-line-clean outlines on the role layers, the ground, finite coordinates and the pens as lineweights", () => {
    const cad = modelScene(stateAt(100), { viewpoint: "axonometric" });
    expect(cad.units).toBe("m");
    const names = cad.layers.map((l) => l.name);
    expect(names).toEqual(expect.arrayContaining(["STAFFAGE-BODY", "GROUND"]));
    const body = cad.layers.find((l) => l.name === "STAFFAGE-BODY")!;
    expect(body.lineweight).toBe(Math.round((asset.representations["100"] as { pens: { body: number } }).pens.body * 100));
    for (const layer of cad.layers) for (const line of layer.polylines) for (const p of line.points) expect(Number.isFinite(p.x) && Number.isFinite(p.y)).toBe(true);
    const ys = cad.layers.flatMap((l) => l.polylines.flatMap((line) => line.points.map((p) => p.y)));
    expect(Math.max(...ys) - Math.min(...ys)).toBeGreaterThan(asset.dimensions.heightM * 0.9);
  });

  it("a structural result adds the outlines of the 3D arrows to ANALYSIS-ARROWS", () => {
    const state = stateAt(100);
    const cad = modelScene(state, { viewpoint: "axonometric", analysis: { structure: solveStructure(state) } });
    expect(cad.layers.some((l) => l.name === "ANALYSIS-ARROWS" && l.polylines.length > 0)).toBe(true);
  });
});

describe("🧊️ the command line, the cast manifest and the Python consumer ask for the same view", () => {
  const base = ["--asset", "sample-figure", "--pose", POSE, "--view", "light-line", "--scale", "100", "--lang", "en", "--out", "out"];

  it("--viewpoint axonometric with --camera, --views and the file names of D14", () => {
    const one = parseArguments([...base, "--format", "svg", "--viewpoint", "axonometric"]);
    expect(targetOf(one)).toEqual({ kind: "view", viewpoint: "axonometric" });
    expect(one.axonometric).toBeUndefined();
    const dim = parseArguments([...base, "--format", "svg", "--viewpoint", "axonometric", "--camera", "dimetric"]);
    expect(dim.axonometric).toEqual({ kind: "dimetric" });
    expect(cameraTag(dim.axonometric)).toBe("-dimetric");
    expect(parseCamera("30,25")).toEqual({ kind: "free", azimuthDeg: 30, elevationDeg: 25 });
    expect(cameraTag(parseCamera("30,25"))).toBe("-az30-el25");
    expect(cameraTag(undefined)).toBe("");
    const sheet = parseArguments([...base, "--format", "sheet-svg", "--projection", "first", "--views", "plan,front,elevation,axonometric", "--camera", "isometric"]);
    expect(targetOf(sheet)).toEqual({ kind: "sheet", projection: "first", axonometric: true });
    expect(targetOf(parseArguments([...base, "--format", "sheet-svg", "--projection", "first", "--views", "elevation,front,plan"]))).toEqual({ kind: "sheet", projection: "first", axonometric: false });
  });

  it("refuses nonsense by name: a camera without the axonometric view, views on a single view, bad cells, bad angles", () => {
    expect(() => parseArguments([...base, "--format", "svg", "--viewpoint", "plan", "--camera", "dimetric"])).toThrow(/--camera applies to the axonometric view only/u);
    expect(() => parseArguments([...base, "--format", "svg", "--viewpoint", "axonometric", "--views", "elevation,front,plan"])).toThrow(/--views applies to the sheet formats only/u);
    expect(() => parseSheetViews("elevation,front")).toThrow(/--views must name elevation, front and plan[\s\S]*--views muss/u);
    expect(() => parseSheetViews("elevation,front,plan,roof")).toThrow(/--views must name/u);
    expect(() => parseCamera("30")).toThrow(/--camera must be[\s\S]*--camera muss/u);
    expect(() => parseCamera("30,80")).toThrow(/camera-elevation/u);
    expect(() => parseArguments([...base, "--format", "svg", "--viewpoint", "oblique"])).toThrow(/one of/u);
  });

  it("the cast manifest accepts the axonometric viewpoint, with the mirror rule intact", () => {
    const cast = { asset: "sample-figure", pose: POSE, view: "light-line", viewpoint: "axonometric", scale: 100, theme: "light" };
    expect(castManifestIssues({ version: 1, casts: { good: cast } })).toEqual([]);
    expect(castManifestIssues({ version: 1, casts: { bad: { ...cast, mirror: true } } }).join("\n")).toContain("mirror needs viewpoint elevation");
    const schema = JSON.parse(readFileSync(`${STAFFAGE_ROOT}/🔨️modules/📤️export/🎭️cast/🧬️schema/🔣️.schema.json`, "utf8"));
    expect(JSON.stringify(schema).split("axonometric").length - 1).toBe(3);
  });

  it("the Python consumer compiles, offers the axonometric viewpoint and no longer refuses it", () => {
    const file = `${STAFFAGE_ROOT}/../⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🐍️.py`;
    execFileSync("python3", ["-m", "py_compile", file]);
    const text = readFileSync(file, "utf8");
    expect(text).toContain('_VIEWPOINTS = ("elevation", "front", "plan", "axonometric")');
    expect(text).not.toContain("VolumeUnavailableError");
  });
});
