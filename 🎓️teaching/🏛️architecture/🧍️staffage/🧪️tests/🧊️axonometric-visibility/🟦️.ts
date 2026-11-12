/** 🧊️ Hidden-surface and hidden-line removal of the axonometric view: constructed scenes (box behind box, interpenetration, coplanar faces, a cycle of overlaps) against hand-computed areas, polygon-clipping (independent polygon boolean library) and three's per-point Raycaster, and the visible surfaces of the sample figure against the kernel viewer's software rasteriser for the same camera.
 *
 * THE REFERENCES (test-only): three `Raycaster` with `FrontSide` meshes for the nearest front-facing hit at a point (view space, rays along -z), `polygon-clipping` for areas of differences and unions, the kernel viewer `SoftwareBackend` (`sceneOf`, `frameOf`) for the figure, hand-computed numbers in `🧫️fixtures/🧊️axonometric-visibility`.
 * Tolerances: areas 1e-9 relative; per-point owner of the constructed scenes 0 mismatches (points within 1e-6 of a fragment boundary are skipped); the sample figure agrees with the rasteriser on at least 99 % of the covered pixels (the rest are sampling ties on edges and the rasteriser's pixel-centre rule).
 *
 * @see ./🥒️.feature — the scenarios
 */
import polygonClipping from "polygon-clipping";
import { BoxGeometry, FrontSide, Mesh, MeshBasicMaterial, Raycaster, Vector3 } from "three";
import { beforeAll, describe, expect, it } from "vitest";
import { cameraOf as viewerCameraOf, fitCamera } from "../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🖥️viewer/📷️camera/🟦️.ts";
import { frameOf as viewerFrame } from "../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🖥️viewer/🎞️frame/🟦️.ts";
import { sceneOf } from "../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🖥️viewer/🧱️scene/🟦️.ts";
import { SoftwareBackend } from "../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🖥️viewer/🖼️software/🟦️.ts";
import { tessellate, type Material } from "../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { defaultDepictionOptions, depict, loadAxonometric } from "../../🔨️modules/🎨️depiction/🟦️.ts";
import { qualityAt, surfacesAt } from "../../🔨️modules/🎨️depiction/🧊️axonometric/🧱️mesh/🟦️.ts";
import { Visibility, toleranceOf } from "../../🔨️modules/🎨️depiction/🧊️axonometric/📐️visibility/🟦️.ts";
import { axonometricCamera } from "../../🔨️modules/🎨️depiction/🔭️views/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { modelOf } from "../../🔨️modules/🧊️volume/🟦️.ts";
import { solidsAt } from "../../🔨️modules/🎨️depiction/🧊️axonometric/🧱️mesh/🟦️.ts";
import { fixture } from "../🧰️support/🧰️.ts";
import { sampleFigure } from "../🧰️support/🧊️.ts";

beforeAll(loadAxonometric);

type Boxes = Record<string, [number, number, number, number, number, number]>;
type Vectors = {
  cases: { name: string; boxes: Boxes; backFaceArea: number; hiddenArea: number; visibleArea: number }[];
  interpenetration: { name: string; boxes: Boxes };
  coplanar: { name: string; boxes: Boxes; unionFrontArea: number };
  cyclic: { name: string; boxes: Boxes };
  tolerance: { sampleMismatch: number; areaRelative: number };
};
const vectors = fixture<Vectors>("🧊️axonometric-visibility");

type Scene = { vertices: Float64Array; indices: Uint32Array; owner: Uint32Array; names: string[]; meshes: Mesh[] };

function scene(boxes: Boxes): Scene {
  const names = Object.keys(boxes);
  const vertices: number[] = [];
  const indices: number[] = [];
  const owner: number[] = [];
  const meshes: Mesh[] = [];
  names.forEach((name, k) => {
    const [cx, cy, cz, w, h, d] = boxes[name]!;
    const geometry = new BoxGeometry(w, h, d).translate(cx, cy, cz);
    const base = vertices.length / 3;
    vertices.push(...Array.from(geometry.getAttribute("position").array));
    const index = Array.from(geometry.index!.array);
    for (const i of index) indices.push(i + base);
    for (let t = 0; t < index.length / 3; t++) owner.push(k);
    meshes.push(new Mesh(geometry, new MeshBasicMaterial({ side: FrontSide })));
  });
  meshes.forEach((m) => m.updateMatrixWorld(true));
  return { vertices: new Float64Array(vertices), indices: Uint32Array.from(indices), owner: Uint32Array.from(owner), names, meshes };
}

const structure = (s: Scene): Visibility => new Visibility({ vertices: s.vertices, indices: s.indices, opaque: new Uint8Array(s.indices.length / 3).fill(1) }, toleranceOf(20));

function nearestHit(s: Scene, x: number, y: number): { mesh: number; face: number } | null {
  const raycaster = new Raycaster(new Vector3(x, y, 100), new Vector3(0, 0, -1), 0, 1000);
  let best: { mesh: number; face: number; distance: number } | null = null;
  let offset = 0;
  s.meshes.forEach((mesh, k) => {
    const hit = raycaster.intersectObject(mesh, false)[0];
    if (hit !== undefined && (best === null || hit.distance < best.distance)) best = { mesh: k, face: offset + hit.faceIndex!, distance: hit.distance };
    offset += mesh.geometry.index!.count / 3;
  });
  const found = best as { mesh: number; face: number } | null;
  return found === null ? null : { mesh: found.mesh, face: found.face };
}

const area = (poly: readonly number[]): number => {
  let sum = 0;
  for (let i = 0, j = poly.length / 2 - 1; i < poly.length / 2; j = i++) sum += poly[2 * j]! * poly[2 * i + 1]! - poly[2 * i]! * poly[2 * j + 1]!;
  return Math.abs(sum) / 2;
};

function insideConvex(poly: readonly number[], x: number, y: number, margin: number): boolean | "edge" {
  let inside = true;
  const n = poly.length / 2;
  for (let i = 0; i < n; i++) {
    const ax = poly[2 * i]!, ay = poly[2 * i + 1]!, bx = poly[2 * ((i + 1) % n)]!, by = poly[2 * ((i + 1) % n) + 1]!;
    const cross = ((bx - ax) * (y - ay) - (by - ay) * (x - ax)) / Math.hypot(bx - ax, by - ay);
    if (Math.abs(cross) < margin) return "edge";
    if (cross < 0) inside = false;
  }
  return inside;
}

/** 🎯️ Compares the owner of every grid point: the triangle whose fragment covers it equals the nearest front-facing hit of the ray caster. Returns the mismatches and the points compared. */
function compareOwners(s: Scene, vis: Visibility, window: [number, number, number, number], steps: number): { mismatches: number; compared: number; coveredTwice: number } {
  const fragments = new Map<number, number[][]>();
  for (let t = 0; t < s.indices.length / 3; t++) if (vis.isLive(t)) fragments.set(t, vis.fragments(t));
  let mismatches = 0, compared = 0, coveredTwice = 0;
  for (let i = 0; i < steps; i++)
    for (let j = 0; j < steps; j++) {
      const x = window[0] + ((window[2] - window[0]) * (i + 0.37)) / steps;
      const y = window[1] + ((window[3] - window[1]) * (j + 0.61)) / steps;
      const owners: number[] = [];
      let tie = false;
      for (const [t, list] of fragments)
        for (const polygon of list) {
          const inside = insideConvex(polygon, x, y, 1e-6);
          if (inside === "edge") tie = true;
          else if (inside) owners.push(t);
        }
      if (tie) continue;
      compared++;
      if (owners.length > 1) coveredTwice++;
      const hit = nearestHit(s, x, y);
      const expected = hit === null ? [] : [hit.face];
      if (owners.length !== expected.length || (owners.length === 1 && owners[0] !== expected[0])) mismatches++;
    }
  return { mismatches, compared, coveredTwice };
}


describe("🧊️ hidden-surface removal of constructed scenes", () => {
  for (const c of vectors.cases)
    it(`${c.name}: the visible area of the back face is the hand-computed ${c.visibleArea}, and polygon-clipping agrees`, () => {
      const s = scene(c.boxes);
      const vis = structure(s);
      const k = s.names.indexOf("back");
      const [cx, cy, cz, w, h, d] = c.boxes.back!;
      const frontZ = cz + d / 2;
      let visible = 0;
      for (let t = 0; t < s.indices.length / 3; t++) {
        if (s.owner[t] !== k) continue;
        const z = [0, 1, 2].map((i) => s.vertices[3 * s.indices[3 * t + i]! + 2]!);
        if (!z.every((v) => Math.abs(v - frontZ) < 1e-9)) continue;
        for (const polygon of vis.fragments(t)) visible += area(polygon);
      }
      expect(visible).toBeCloseTo(c.visibleArea, 9);
      const face = [[[cx - w / 2, cy - h / 2], [cx + w / 2, cy - h / 2], [cx + w / 2, cy + h / 2], [cx - w / 2, cy + h / 2], [cx - w / 2, cy - h / 2]]] as [number, number][][];
      const [fx, fy, , fw, fh] = c.boxes.front!;
      const cover = [[[fx - fw / 2, fy - fh / 2], [fx + fw / 2, fy - fh / 2], [fx + fw / 2, fy + fh / 2], [fx - fw / 2, fy + fh / 2], [fx - fw / 2, fy - fh / 2]]] as [number, number][][];
      const left = polygonClipping.difference(face, cover);
      const clipped = left.reduce((sum, polygon) => sum + polygon.reduce((s2, ring, i) => { let a = 0; for (let q = 0; q < ring.length - 1; q++) a += ring[q]![0] * ring[q + 1]![1] - ring[q + 1]![0] * ring[q]![1]; return s2 + (i === 0 ? 1 : -1) * Math.abs(a) / 2; }, 0), 0);
      expect(visible).toBeCloseTo(clipped, 9);
      expect(w * h - visible).toBeCloseTo(c.hiddenArea, 9);
      const result = compareOwners(s, vis, [-8, -8, 8, 8], 60);
      expect(result.mismatches).toBe(vectors.tolerance.sampleMismatch);
      expect(result.coveredTwice).toBe(0);
      expect(result.compared).toBeGreaterThan(2000);
    });

  it("interpenetrating bars: every point is owned by exactly the nearest front-facing triangle, the crossing included", () => {
    const s = scene(vectors.interpenetration.boxes);
    const result = compareOwners(s, structure(s), [-3.5, -3.5, 3.5, 3.5], 90);
    expect(result.mismatches).toBe(0);
    expect(result.coveredTwice).toBe(0);
  });

  it("coplanar flush faces: the union of the fronts is covered once, never twice", () => {
    const s = scene(vectors.coplanar.boxes);
    const vis = structure(s);
    let total = 0;
    for (let t = 0; t < s.indices.length / 3; t++) {
      const z = [0, 1, 2].map((i) => s.vertices[3 * s.indices[3 * t + i]! + 2]!);
      if (!z.every((v) => Math.abs(v - 1) < 1e-9)) continue;
      for (const polygon of vis.fragments(t)) total += area(polygon);
    }
    expect(total).toBeCloseTo(vectors.coplanar.unionFrontArea, 9);
    expect(compareOwners(s, vis, [-3, -2, 3, 2], 70).coveredTwice).toBe(0);
  });

  it("a cycle of overlaps (a over b over c over a) is resolved without any ordering assumption", () => {
    const s = scene(vectors.cyclic.boxes);
    const result = compareOwners(s, structure(s), [-3.5, -4, 3.5, 3], 90);
    expect(result.mismatches).toBe(0);
    expect(result.coveredTwice).toBe(0);
  });
});

describe("🧊️ hidden-line removal", () => {
  it("a segment behind a box is hidden exactly over the box's shadow and visible elsewhere, and the ray caster agrees along the whole line", () => {
    const s = scene({ front: [0, 0, 2, 2, 2, 2] });
    const vis = structure(s);
    const a: [number, number, number] = [-3, 0.25, -1];
    const b: [number, number, number] = [3, 0.25, -1];
    const hidden = vis.hidden(a, b);
    expect(hidden).toHaveLength(1);
    expect(a[0] + (b[0] - a[0]) * hidden[0]![0]).toBeCloseTo(-1, 9);
    expect(a[0] + (b[0] - a[0]) * hidden[0]![1]).toBeCloseTo(1, 9);
    for (let i = 0; i <= 300; i++) {
      const t = i / 300;
      const x = a[0] + (b[0] - a[0]) * t;
      if (Math.abs(Math.abs(x) - 1) < 1e-3) continue;
      const hit = nearestHit(s, x, a[1]);
      const blocked = hit !== null;
      const inInterval = hidden.some(([t0, t1]) => t >= t0 && t <= t1);
      expect(inInterval, `x = ${x}`).toBe(blocked);
    }
  });

  it("the edge of a face lying in front of a box is never hidden by the box it belongs to, and a long edge in front of another box stays whole", () => {
    const s = scene({ front: [0, 0, 2, 2, 2, 2], back: [0, 0, -2, 2, 2, 2] });
    const vis = structure(s);
    const own = new Set<number>([...Array(s.indices.length / 3).keys()].filter((t) => s.owner[t] === 0));
    expect(vis.visible([-1, 1, 3], [1, 1, 3], own)).toEqual([[0, 1]]);
    expect(vis.visible([-1, 1, 3], [1, 1, 3])).toEqual([[0, 1]]);
  });

  it("an edge that passes through a solid is hidden only where it is inside it (depth changes sign along the segment)", () => {
    const s = scene({ block: [0, 0, 0, 2, 2, 2] });
    const vis = structure(s);
    const hidden = vis.hidden([-2, 0, 3], [2, 0, -3]);
    expect(hidden).toHaveLength(1);
    const [t0, t1] = hidden[0]!;
    const at = (t: number): [number, number] => [-2 + 4 * t, 3 - 6 * t];
    expect(at(t0)[0]).toBeCloseTo(-2 / 3, 9);
    expect(at(t0)[1]).toBeCloseTo(1, 9);
    expect(at(t1)[0]).toBeCloseTo(1, 9);
    expect(at(t1)[1]).toBeCloseTo(-1.5, 9);
  });
});

describe("🧊️ the visible surfaces of the sample figure equal the kernel viewer's rasteriser for the same camera", () => {
  const asset = sampleFigure();
  const SCALE = 50;
  const WIDTH = 220;
  const HEIGHT = 300;

  it("agrees on at least 99 % of the covered pixels, surface by surface", async () => {
    const pose = "stand";
    const doc = await modelOf(asset, { scale: SCALE, poses: [pose], clips: [], views: [] });
    const resolved = await tessellate(doc, qualityAt(SCALE));
    const surfaces = surfacesAt(asset, SCALE);
    const palette = surfaces.map((surface, i) => ({ surface: surface.id, rgb: [40 + ((i * 53) % 200), 30 + ((i * 97) % 210), 20 + ((i * 29) % 220)] as [number, number, number] }));
    const lin = (c: number): number => (c / 255) ** 2.2;
    const painted = { ...resolved, materials: resolved.materials.map((m): Material => { const p = palette.find((q) => q.surface === m.id); return p === undefined ? m : { ...m, baseColor: [lin(p.rgb[0]), lin(p.rgb[1]), lin(p.rgb[2]), 1], alphaMode: "opaque", emissive: [0, 0, 0], unlit: true, doubleSided: false }; }), variants: [] };
    const viewScene = sceneOf(painted);
    const camera = axonometricCamera({ kind: "isometric" });
    const state = evaluate(asset, { kind: "pose", pose }, { kind: "scale", scale: { denominator: SCALE } });
    const drawn = depict(state, { ...defaultDepictionOptions("light-line", "en", SCALE, "axonometric"), annotations: { dimensions: false, scaleBar: false, legend: false, labels: false, ground: false }, marginMm: 0 });
    const bounds = { min: [-0.6, -0.1, -0.6] as [number, number, number], max: [0.6, asset.dimensions.heightM + 0.05, 0.6] as [number, number, number] };
    const fitted = fitCamera(viewerCameraOf("axonometric"), bounds, WIDTH / HEIGHT, 1.05);
    const backend = new SoftwareBackend(WIDTH, HEIGHT);
    backend.load(viewScene);
    backend.render(viewerFrame(viewScene, { pose, camera: fitted, aspect: WIDTH / HEIGHT }), { background: [0, 0, 0, 1] });
    const halfH = fitted.orthoHeight / 2;
    const halfW = halfH * (WIDTH / HEIGHT);
    const k = drawn.metadata.mmPerMetre;
    const origin = drawn.metadata.origin;
    const target = { x: fitted.target[2], y: fitted.target[1], z: fitted.target[0] };
    const tx = camera.normalisation * (camera.right.x * target.x + camera.right.y * target.y + camera.right.z * target.z);
    const ty = camera.normalisation * (camera.up.x * target.x + camera.up.y * target.y + camera.up.z * target.z);
    const solidSurface = new Map(surfaces.flatMap((surface) => surface.solids.map((id) => [id, surface.id] as const)));
    const items = drawn.layers.flatMap((layer) => layer.items).flatMap((item) => (item.kind === "path" && item.id.startsWith("shape:") && item.fill !== undefined ? [{ id: item.id.slice(6).replace(/:band-\d+$/u, ""), rings: rings(item.commands) }] : []));
    let covered = 0, mismatches = 0;
    for (let j = 0; j < HEIGHT; j++)
      for (let i = 0; i < WIDTH; i++) {
        const u = ((i + 0.5) / WIDTH) * 2 - 1;
        const w = 1 - ((j + 0.5) / HEIGHT) * 2;
        const x = camera.normalisation * u * halfW + tx;
        const y = camera.normalisation * w * halfH + ty;
        const px = origin.xMm + x * k;
        const py = origin.yMm - y * k;
        const o = (j * WIDTH + i) * 4;
        const rgb = [backend.pixels[o]!, backend.pixels[o + 1]!, backend.pixels[o + 2]!];
        const theirs = palette.find((p) => Math.abs(p.rgb[0] - rgb[0]!) <= 3 && Math.abs(p.rgb[1] - rgb[1]!) <= 3 && Math.abs(p.rgb[2] - rgb[2]!) <= 3)?.surface;
        const hit = items.find((item) => inside(item.rings, px, py));
        const ours = hit === undefined ? undefined : solidSurface.get(hit.id);
        if (theirs === undefined && ours === undefined) continue;
        covered++;
        if (theirs !== ours) mismatches++;
      }
    expect(covered).toBeGreaterThan(3000);
    expect(mismatches / covered, `${mismatches} of ${covered} covered pixels differ`).toBeLessThanOrEqual(0.01);
    expect(solidsAt(asset, SCALE).length).toBeGreaterThan(10);
  }, 120_000);
});

type Ring = { x: number; y: number }[];

function rings(commands: readonly { op: string; x?: number; y?: number }[]): Ring[] {
  const out: Ring[] = [];
  let current: Ring = [];
  for (const c of commands) {
    if (c.op === "M") {
      if (current.length > 0) out.push(current);
      current = [{ x: c.x!, y: c.y! }];
    } else if (c.op === "L") current.push({ x: c.x!, y: c.y! });
  }
  if (current.length > 0) out.push(current);
  return out;
}

function inside(list: readonly Ring[], x: number, y: number): boolean {
  let odd = false;
  for (const ring of list)
    for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
      const a = ring[i]!, b = ring[j]!;
      if (a.y > y !== b.y > y && x < ((b.x - a.x) * (y - a.y)) / (b.y - a.y) + a.x) odd = !odd;
    }
  return odd;
}

