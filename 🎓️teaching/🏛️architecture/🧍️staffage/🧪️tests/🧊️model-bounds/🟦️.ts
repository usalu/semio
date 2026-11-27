/** 🧊️ The extent of the 3D model: at rest against the physical `dimensions` of the asset, and for every named pose against the elevation, front and plan extents of the same evaluated state (decision D18: the larger of 2 % and 15 mm).
 *
 * THE REFERENCES are three.js (`Box3.expandByObject(object, true)`, the precise skinned extent of the GLB loaded by `GLTFLoader`), the 2D shapes of `evaluate`, and the hand-computed extent of the sample figure's rest frames in `🧫️fixtures/🧊️volume-schema`.
 *
 * The ahead and up extents are compared with the elevation (the view that shows both), the lateral extent with the front and the plan; the sample figure's authored front shapes stay on the ground line in a stride, so their vertical extent is not a reference.
 *
 * @see ./🥒️.feature — the scenarios
 */
import { readFileSync } from "node:fs";
import { Box3, Matrix4, Quaternion, Ray, Triangle, Vector3 } from "three";
import { tessellateSolid } from "../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import type { Asset } from "../../🧬️schema/🟦️.ts";
import { describe, expect, it } from "vitest";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { writeGlb } from "../../🔨️modules/📤️export/🧊️glb/🟦️.ts";
import { modelOf } from "../../🔨️modules/🧊️volume/🟦️.ts";
import { extentTolerance, loadGlb, restExtent, sampleFigure, sceneExtent, shapeExtent } from "../🧰️support/🧊️.ts";

const SCALE = 50;
type Scale = 50 | 100 | 200 | 500;
type Model = Awaited<ReturnType<typeof modelOf>>;
const SCALES: readonly Scale[] = [50, 100, 200, 500];
const VIEWS = ["light-solid", "thermal", "forces"] as const;
const within = (got: number, want: number, what: string): void => void expect(Math.abs(got - want), `${what}: ${got} vs ${want}`).toBeLessThanOrEqual(extentTolerance(want));

/** 📜️ The decisions of the band of 1:`scale` state `pattern`. A case that expects what the body of a band resolves names the declaration it follows (the README "Typification", D52), so the body and its words cannot drift apart: a band that stops declaring its simplification fails here, and a body that stops following its declaration fails the case that reads it. */
function said(asset: Asset, scale: Scale, pattern: RegExp): void {
  const volume = (asset.volumes as unknown as Readonly<Record<string, { readonly decisions?: readonly { readonly en: string }[] }>>)[String(scale)];
  expect(pattern.test((volume?.decisions ?? []).map((decision) => decision.en).join(" ")), `${asset.id} 1:${scale} decisions state ${pattern}`).toBe(true);
}

/** 🌍️ The matrix of a node in the frame of the model: the local transforms multiplied down the chain of its parents, so a solid is measured where the model puts it and not in the frame of an earlier rig. */
function worldOf(model: Model, id: string): Matrix4 {
  const nodes = new Map(model.nodes.map((node) => [node.id, node] as const));
  const chain = (at: string): Matrix4 => {
    const node = nodes.get(at)!;
    const local = new Matrix4().compose(new Vector3(...node.translation), new Quaternion(...node.rotation), new Vector3(...node.scale));
    return node.parent === null ? local : chain(node.parent).multiply(local);
  };
  return chain(id);
}

/** 🧱️ The merged spans along one axis (0 x, 1 y, 2 z) that the triangles of the solids occupy. */
function occupied(solids: Model["solids"], axis: number): number[][] {
  const spans: number[][] = [];
  for (const solid of solids) {
    const mesh = tessellateSolid(solid, { chordM: 0.0001, minSegments: 24, maxSegments: 48 });
    for (let i = 0; i < mesh.indices.length; i += 3) {
      const points = [0, 1, 2].map((j) => mesh.positions[3 * mesh.indices[i + j]! + axis]!);
      spans.push([Math.min(...points), Math.max(...points)]);
    }
  }
  const merged: number[][] = [];
  for (const [lo, hi] of spans.sort((a, b) => a[0]! - b[0]!)) {
    const last = merged.at(-1);
    if (last && lo! <= last[1]! + 0.000001) last[1] = Math.max(last[1]!, hi!);
    else merged.push([lo!, hi!]);
  }
  return merged;
}

/** 🔢️ The most separate spans any axis shows for the solids: how many bands, courses, rods or plates the body keeps apart. */
const separate = (solids: Model["solids"]): number => Math.max(...[0, 1, 2].map((axis) => occupied(solids, axis).length));

/** 🎯️ What the body of one band resolves of a part, stated once per band: the solids matching `parts` (`count` of them standing for `copies` repeated instances, `spans` separate along their widest axis), the ids that are present and the patterns that must match nothing; `words` is the declaration of the band that says so. */
type Resolves = { readonly parts?: RegExp; readonly count?: number; readonly copies?: number; readonly spans?: number; readonly present?: readonly string[]; readonly absent?: readonly RegExp[]; readonly words?: RegExp };

function resolved(asset: Asset, scale: Scale, solids: Model["solids"], want: Resolves, what: string): void {
  if (want.words !== undefined) said(asset, scale, want.words);
  const parts = want.parts === undefined ? [] : solids.filter((solid) => want.parts!.test(solid.id));
  if (want.count !== undefined) expect(parts.map((part) => part.id), `${asset.id} 1:${scale} ${what} solids`).toHaveLength(want.count);
  if (want.copies !== undefined) expect(parts.reduce((sum, part) => sum + ((part as { readonly repeat?: { readonly count: number } }).repeat?.count ?? 1), 0), `${asset.id} 1:${scale} ${what} copies`).toBe(want.copies);
  if (want.spans !== undefined) expect(separate(parts), `${asset.id} 1:${scale} ${what} spans`).toBe(want.spans);
  for (const id of want.present ?? []) expect(solids.some((solid) => solid.id === id), `${asset.id} 1:${scale} ${what} ${id}`).toBe(true);
  for (const pattern of want.absent ?? []) expect(solids.filter((solid) => pattern.test(solid.id)).map((solid) => solid.id), `${asset.id} 1:${scale} ${what} ${pattern}`).toEqual([]);
}

describe("🧊️ the extent of the 3D model", () => {
  const asset = sampleFigure();

  it("has at rest the extent of the physical dimensions: ahead the width, up the height, lateral the depth", async () => {
    const e = await restExtent(asset, await modelOf(asset, { scale: SCALE, view: "light-solid", poses: [], clips: [] }));
    within(e.max[0] - e.min[0], asset.dimensions.widthM, "widthM");
    within(e.max[1] - e.min[1], asset.dimensions.heightM, "heightM");
    within(e.max[2] - e.min[2], asset.dimensions.depthM, "depthM");
  });

  it.each(asset.poses.filter((p) => p.key !== true).map((p) => p.id))("has in pose %s the extent of the 2D elevation, front and plan shapes of the same state", async (pose) => {
    const { scene } = await loadGlb(await writeGlb(asset, { pose }));
    const e = sceneExtent(scene);
    const state = evaluate(asset, { kind: "pose", pose }, { kind: "scale", scale: { denominator: SCALE } });
    const elevation = shapeExtent(state, "elevation");
    const front = shapeExtent(state, "front");
    const plan = shapeExtent(state, "plan");
    within(e.max[0] - e.min[0], elevation.horizontal[1] - elevation.horizontal[0], "elevation x extent");
    within(e.min[0], elevation.horizontal[0], "elevation x min");
    within(e.max[0], elevation.horizontal[1], "elevation x max");
    within(e.max[1], elevation.vertical[1], "elevation y max");
    within(e.max[1] - e.min[1], elevation.vertical[1] - elevation.vertical[0], "elevation y extent");
    within(e.max[2] - e.min[2], front.horizontal[1] - front.horizontal[0], "front z extent");
    within(e.max[0] - e.min[0], plan.horizontal[1] - plan.horizontal[0], "plan x extent");
    within(e.max[2] - e.min[2], plan.vertical[1] - plan.vertical[0], "plan z extent");
  });

  it("keeps the hands and feet of a pose on the ground and the solids above it: the lowest point of a standing pose is the ground", async () => {
    const { scene } = await loadGlb(await writeGlb(asset, { pose: "stand" }));
    const e = sceneExtent(scene);
    expect(Math.abs(e.min[1])).toBeLessThanOrEqual(0.015);
  });
});

describe("laptop desk contacts", () => {
  const laptop: Asset = JSON.parse(readFileSync(new URL("../../🗂️catalog/🔌️appliances/laptop/🔣️.json", import.meta.url), "utf8")).asset;
  const centres = [0.020, 0.198].flatMap((x) => [-0.134, 0.134].map((z) => new Vector3(x, 0, z)));

  it("places each support anchor on a sole and uses the four sole areas for heat contact", () => {
    const anchors = laptop.anchors.filter((a) => laptop.structure.supports.includes(a.id));
    expect(anchors).toHaveLength(4);
    for (const centre of centres) {
      const anchor = anchors.find((a) => new Vector3(a.x, a.y, a.z ?? 0).distanceTo(centre) < 1e-6);
      expect(anchor?.footprintLengthM).toBeCloseTo(0.0104, 6);
      expect(anchor?.footprintWidthM).toBeCloseTo(0.0104, 6);
    }
    const underside = laptop.thermal.regions.find((r) => r.id === "underside")!;
    const fraction = 4 * Math.PI * 0.0052 ** 2 / underside.areaM2;
    for (const pose of laptop.poses) {
      expect([...pose.contacts].sort()).toEqual(anchors.map((a) => a.id).sort());
      expect(pose.thermal?.regions?.underside?.contact?.fraction).toBeCloseTo(fraction, 6);
    }
    for (const clip of laptop.clips) {
      expect(clip.contacts.map((c) => c.anchor).sort()).toEqual(anchors.map((a) => a.id).sort());
      for (const contact of clip.contacts) expect([contact.fromS, contact.toS]).toEqual([0, clip.durationS]);
    }
  });

  it.each([50, 100, 200, 500] as const)("carries the body, thermal shell and structure on four rubber soles at 1:%i", async (scale) => {
    for (const view of ["light-solid", "thermal", "forces"] as const) {
      const model = await modelOf(laptop, { scale, view, poses: [], clips: [] });
      const areas = centres.map(() => 0);
      let underside = Infinity;
      for (const solid of model.solids.filter((s) => s.node === "base" && !s.tags?.includes("part-mass"))) {
        const mesh = tessellateSolid(solid, { chordM: 0.0001, minSegments: 24, maxSegments: 64 });
        const point = (i: number) => new Vector3().fromArray(mesh.positions, 3 * mesh.indices[i]!);
        for (let i = 0; i < mesh.indices.length; i += 3) {
          const triangle = new Triangle(point(i), point(i + 1), point(i + 2));
          const vertices = [triangle.a, triangle.b, triangle.c];
          if (["base", "tub", "th-tub", "th-underside", "tray-floor", "floor-plate"].includes(solid.id)) underside = Math.min(underside, ...vertices.map((p) => p.y));
          if (vertices.some((p) => Math.abs(p.y) > 0.000001)) continue;
          const foot = centres.findIndex((centre) => vertices.every((p) => centre.distanceTo(p) <= 0.00521));
          expect(foot, `${view} ${solid.id} desk contact outside the four soles`).toBeGreaterThanOrEqual(0);
          areas[foot] = areas[foot]! + triangle.getArea();
        }
      }
      for (const area of areas) expect(area / (Math.PI * 0.0052 ** 2), view).toBeCloseTo(1, 1);
      expect(underside, view).toBeCloseTo(0.002, 4);
    }
  });
});

/** 🛞️ Where a wheel meets the ground in a band: the node (a prefix in the model frame), the position ahead and along the bone, the height of the ground, the lateral centre line and the width of its tread; a `roller` is one body across the centre line (no contact is asked on its two sides apart). */
type Station = { readonly node: string; readonly ahead: number; readonly y: number; readonly lateral: number; readonly width: number; readonly roller: boolean };
const pair = (node: string, y: number, lateral: number, width: number, ahead = 0): Station => ({ node, ahead, y, lateral, width, roller: false });
const roller = (node: string, y: number, width: number, ahead = 0): Station => ({ node, ahead, y, lateral: 0, width, roller: true });

/** 🚜️ What the body of each band of an asset on wheels resolves: the stations, the declarations that say so, and whether a wheel is one disc with its rim (no separate rim member joins the treads to the axle). */
type Rolling = { readonly stations: readonly Station[]; readonly said: readonly RegExp[]; readonly disc?: boolean };

describe("small equipment rolling contacts", () => {
  const scenarios = [
    { file: "../../🗂️catalog/🔌️appliances/vacuum-cleaner/🔣️.json", frame: "node" as const, bands: {
      50: { stations: [pair("wheel", -0.06, 0.1375, 0.025), pair("caster", -0.025, 0.018, 0.012)], said: [] },
      100: { stations: [pair("wheel", -0.06, 0.1375, 0.025), roller("caster", -0.025, 0.048)], said: [/twin swivel wheels \(12 mm each\) are drawn as one 48 mm roller/] },
      200: { stations: [pair("wheel", -0.06, 0.135, 0.03), roller("caster", -0.025, 0.048)], said: [/each rear wheel is one 120 mm disc, 30 mm wide/i, /swivel caster is one 50 mm roller/], disc: true },
      500: { stations: [roller("wheel", -0.06, 0.3), roller("caster", -0.025, 0.075)], said: [/one 120 mm roller across the full 0\.30 m track/, /caster is one 50 mm roller 75 mm wide/], disc: true },
    } satisfies Record<Scale, Rolling> },
    { file: "../../🗂️catalog/🪑️objects/shopping-trolley/🔣️.json", frame: "model" as const, bands: {
      500: { stations: [pair("wheel-rear-", 0, 0.25, 0.075, -0.295), pair("wheel-front-", 0, 0.198, 0.075, 0.265)], said: [/one o125 x 75 wheel on its axle bone/] },
    } satisfies Partial<Record<Scale, Rolling>> },
  ];

  for (const scenario of scenarios) {
    const asset: Asset = JSON.parse(readFileSync(new URL(scenario.file, import.meta.url), "utf8")).asset;
    for (const scale of Object.keys(scenario.bands).map(Number) as Scale[]) it(`keeps four separated rolling contacts on ${asset.id} at 1:${scale}`, async () => {
      const band = (scenario.bands as Partial<Record<Scale, Rolling>>)[scale]!;
      for (const pattern of band.said) said(asset, scale, pattern);
      for (const view of VIEWS) {
        const model = await modelOf(asset, { scale, view, poses: [], clips: [] });
        for (const station of band.stations) {
          const contacts = new Set<number>();
          const rims = new Set<number>();
          const lateralOf = (p: Vector3): number => (scenario.frame === "model" ? p.x : p.z);
          const aheadOf = (p: Vector3): number => (scenario.frame === "model" ? p.z : p.x);
          const solids = model.solids.filter((s) => (scenario.frame === "model" ? s.node.startsWith(station.node) : s.node === station.node) && !s.tags?.includes("part-mass"));
          expect(solids.length, `${view} ${station.node} solids exist`).toBeGreaterThan(0);
          for (const solid of solids) {
            const mesh = tessellateSolid(solid, { chordM: 0.0001, minSegments: 32, maxSegments: 64 });
            const to = scenario.frame === "model" ? worldOf(model, solid.node) : new Matrix4();
            if (station.node === "wheel" && view === "forces" && (solid.tags?.includes("part-member") || (band.disc === true && solid.tags?.includes("part-support")))) {
              for (const side of [-1, 1]) {
                let inner = Infinity, outer = 0;
                for (let j = 0; j < mesh.positions.length; j += 3) {
                  const vertex = new Vector3().fromArray(mesh.positions, j);
                  if (Math.abs(vertex.z - side * station.lateral) > station.width / 2) continue;
                  const radius = Math.hypot(vertex.x, vertex.y);
                  inner = Math.min(inner, radius);
                  outer = Math.max(outer, radius);
                }
                if (inner < 0.002 && outer >= 0.044) rims.add(side);
              }
            }
            for (let i = 0; i < mesh.indices.length; i += 3) {
              const points = [0, 1, 2].map((j) => new Vector3().fromArray(mesh.positions, 3 * mesh.indices[i + j]!).applyMatrix4(to));
              const low = points.filter((p) => Math.abs(p.y - station.y) < 0.0002 && Math.abs(aheadOf(p) - station.ahead) < 0.04);
              if (!low.length) continue;
              const mid = low.reduce((sum, p) => sum.add(p), new Vector3()).divideScalar(low.length);
              for (const p of [...low, mid]) {
                expect(Math.abs(Math.abs(lateralOf(p)) - station.lateral), `${view} ${solid.id}`).toBeLessThanOrEqual(station.width / 2 + 0.0002);
                contacts.add(Math.sign(lateralOf(p)));
              }
            }
          }
          if (station.node === "wheel" && view === "forces") expect([...rims].sort(), "rims connect both treads to the axle").toEqual([-1, 1]);
          expect([...contacts].filter((side) => !station.roller || side !== 0).sort(), `${view} ${station.node}`).toEqual([-1, 1]);
          if (!station.roller) expect(contacts.has(0), `${view} ${station.node} no body between the two treads`).toBe(false);
        }
      }
    });
  }
});

describe("street lamp access and spigot", () => {
  const lamp: Asset = JSON.parse(readFileSync(new URL("../../🗂️catalog/🪑️objects/street-lamp/🔣️.json", import.meta.url), "utf8")).asset;
  const CLOSURES: Readonly<Record<Scale, RegExp>> = {
    50: /A recessed square-drive lock secures the terminal door\. A galvanised spring catch spans the gear-cover joint/,
    100: /without fins, lock and catch/,
    200: /lock, catch and hatch seam are omitted/,
    500: /door, lock, catch and seams are omitted/,
  };
  it.each(SCALES)("seats the real arm, access lock and cover latch where the band of 1:%i keeps them", async (scale) => {
    said(lamp, scale, CLOSURES[scale]);
    if (scale === 50) said(lamp, scale, /Structure: the load path from the luminaire to the ground: the die-cast tray as head chassis held by the spigot clamp/);
    for (const view of VIEWS) {
      const model = await modelOf(lamp, { scale, view, poses: [], clips: [] });
      const measured = (name: string) => {
        const solid = model.solids.find((s) => s.id === name);
        expect(solid, `${scale} ${view} ${name}`).toBeDefined();
        const geometry = tessellateSolid(solid!, { chordM: 0.0001, minSegments: 32, maxSegments: 64 });
        return Array.from({ length: geometry.positions.length / 3 }, (_, i) => new Vector3().fromArray(geometry.positions, i * 3));
      };
      const id = (body: string, structure: string) => view === "forces" ? structure : (view === "thermal" ? "th-" : "") + body;
      const arm = measured(id("arm", "member-arm"));
      const sleeve = measured(id("head-spigot", "joint-head-spigot"));
      const radii = sleeve.map((p) => Math.hypot(p.y, p.z));
      expect(Math.max(...arm.map((p) => Math.hypot(p.y, p.z)))).toBeCloseTo(0.0302, 5);
      expect(Math.min(...radii)).toBeCloseTo(0.0303, 5);
      expect(Math.max(...radii)).toBeCloseTo(0.038, 5);
      const reach = new Box3().setFromPoints(sleeve);
      expect(reach.min.x).toBeCloseTo(-0.030, 5);
      expect(reach.max.x).toBeCloseTo(0.100, 5);
      const closures = scale === 50 && view !== "forces";
      for (const [body, structure] of [["door-lock", "joint-door-lock"], ["head-latch", "joint-head-latch"]] as const)
        expect(model.solids.some((s) => s.id === id(body, structure)), `${scale} ${view} ${body} in the body and thermal list of 1:50 only; the structure list carries the load path, which neither the lock nor the catch is`).toBe(closures);
      if (!closures) continue;
      const lock = new Box3().setFromPoints(measured(id("door-lock", "joint-door-lock")));
      expect(lock.getSize(new Vector3()).x).toBeCloseTo(0.010, 4);
      expect(lock.getCenter(new Vector3()).x).toBeCloseTo(0.99, 4);
      const latch = new Box3().setFromPoints(measured(id("head-latch", "joint-head-latch")));
      expect(latch.min.x).toBeLessThan(0.168);
      expect(latch.max.x).toBeGreaterThan(0.172);
      expect(latch.getSize(new Vector3()).z).toBeCloseTo(0.012, 5);
    }
  });
});

function triangles(solids: Awaited<ReturnType<typeof modelOf>>["solids"]) {
  const result: Triangle[] = [];
  for (const solid of solids) {
    const mesh = tessellateSolid(solid, { chordM: 0.0001, minSegments: 24, maxSegments: 48 });
    for (let i = 0; i < mesh.indices.length; i += 3) {
      const points = [0, 1, 2].map((j) => new Vector3().fromArray(mesh.positions, 3 * mesh.indices[i + j]!));
      result.push(new Triangle(points[0], points[1], points[2]));
    }
  }
  return result;
}

function hit(ray: Ray, faces: Triangle[], minimumX = -Infinity) {
  const point = new Vector3();
  return faces.some((t) => ray.intersectTriangle(t.a, t.b, t.c, false, point) && point.x >= minimumX);
}

/** 🔥️ What the body of each band of the heater resolves (D52). `volumes-feature` allows no solid dimension below the smallest feature of the band (7.5, 15, 30, 75 mm), and the bands state their choice in `volumes.<scale>.decisions`: the eight 1.5 mm slats at a pitch of 20 mm are kept at 1:50 (texture) and stand as four 15 mm slats at 1:100 (the thermal list keeps them at their gauge between the two 5 mm rails of the cassette, six bands on the ray); the 26 mm opening of the carry grip is kept down to 1:100 and is a solid lug from 1:200 on, where the outlet is one panel; the 44 mm dial leaves the body at 1:500 and stays in the thermal list. */
const HEATER_RESOLVES: Readonly<Record<50 | 100 | 200 | 500, { readonly gripOpen: boolean; readonly louvres: Readonly<Record<"light-solid" | "thermal", number>>; readonly dial: Readonly<Record<"light-solid" | "thermal", boolean>> }>> = {
  50: { gripOpen: true, louvres: { "light-solid": 8, thermal: 8 }, dial: { "light-solid": true, thermal: true } },
  100: { gripOpen: true, louvres: { "light-solid": 4, thermal: 6 }, dial: { "light-solid": true, thermal: true } },
  200: { gripOpen: false, louvres: { "light-solid": 1, thermal: 1 }, dial: { "light-solid": true, thermal: true } },
  500: { gripOpen: false, louvres: { "light-solid": 1, thermal: 1 }, dial: { "light-solid": false, thermal: true } },
};

describe("portable heater and slatted park bench", () => {
  const source = (file: string): Asset => JSON.parse(readFileSync(new URL(file, import.meta.url), "utf8")).asset;
  const heater = source("../../🗂️catalog/🔌️appliances/electric-fan-heater/🔣️.json");
  const bench = source("../../🗂️catalog/🛋️furniture/park-bench/🔣️.json");

  function bands(samples: boolean[]) {
    return samples.filter((inside, index) => inside && !samples[index - 1]).length;
  }

  it.each([50, 100, 200, 500] as const)("keeps the carry bar and resolves the grip opening, the outlet louvres and the dial of the heater as the band of 1:%i states", async (scale) => {
    const resolves = HEATER_RESOLVES[scale];
    for (const view of ["light-solid", "thermal", "forces"] as const) {
      const model = await modelOf(heater, { scale, view, poses: [], clips: [] });
      const face = triangles(model.solids.filter((s) => s.node === "body" && !s.tags?.includes("part-mass")));
      const across = (y: number, z: number) => new Ray(new Vector3(0.20, y, z), new Vector3(-1, 0, 0));
      for (const z of [-0.04, 0, 0.04]) {
        expect(hit(across(0.286, z), face), `${view} hand clearance ${z}`).toBe(!resolves.gripOpen);
        expect(hit(across(0.304, z), face), `${view} bar above hand ${z}`).toBe(true);
      }
      if (view !== "forces") {
        const knobs = model.solids.filter((s) => s.node === "knob");
        const gauge = triangles(knobs);
        expect(hit(new Ray(new Vector3(0, 0.017, -0.15), new Vector3(0, 0, 1)), gauge), `end of the 44 mm control dial in ${view}`).toBe(resolves.dial[view]);
        const aperture = Array.from({ length: 155 }, (_, i) => hit(across(0.0673 + i * 0.001, 0.076), face, 0.08));
        expect(bands(aperture), `${view} louvre bands across outlet`).toBe(resolves.louvres[view]);
      }
    }
  });

  it.each([50, 100, 200, 500] as const)("keeps three slats and both gaps in the park bench back at 1:%i", async (scale) => {
    for (const view of ["light-solid", "thermal", "forces"] as const) {
      const model = await modelOf(bench, { scale, view, poses: [], clips: [] });
      const face = triangles(model.solids.filter((s) => s.node === "backrest" && !s.tags?.includes("part-mass") && !s.tags?.includes("part-joint")));
      const back = Array.from({ length: 201 }, (_, i) => hit(new Ray(new Vector3(0.73, -0.211 + i * 0.002, 0.4), new Vector3(0, 0, -1)), face));
      expect(bands(back), view).toBe(3);
      expect(back.filter(Boolean).length * 0.002).toBeGreaterThan(0.235);
      expect(back.filter(Boolean).length * 0.002).toBeLessThan(0.265);
    }
  });
});

/** 🧱️ The glass balustrade: what the body of each band keeps of the drilled laminate (`laminate`: id and least dimension) and of its bolts and anchors (`fasteners`: id and the separate spans along the widest axis). */
const GLASS: Readonly<Record<Scale, { readonly laminate: readonly (readonly [string, number])[]; readonly fasteners: readonly (readonly [string, number])[]; readonly words?: RegExp }>> = {
  50: { laminate: [["pane-fixed", 0.01], ["interlayer", 0.0015], ["pane-clamp", 0.01]], fasteners: [["anchor-stud-fixed", 5], ["anchor-stud-clamp", 5], ["bolt-shank", 7]] },
  100: { laminate: [["laminate", 0.0215]], fasteners: [["anchor-fixed", 5], ["anchor-clamp", 5], ["bolt", 7]], words: /the two panes and the interlayer are one 21\.5 mm laminate/ },
  200: { laminate: [["laminate", 0.0215]], fasteners: [], words: /Bolts, anchors and nuts \(22 mm\) are below the feature and left out/ },
  500: { laminate: [["laminate", 0.0215]], fasteners: [], words: /the laminate as a declared sheet at its true 21\.5 mm/ },
};

/** 🚪️ The sliding door: per side of the leaf the pull handle (`grips`) and the wheel sets of the carriages (`wheelSets` solids, `spans` separate wheels along their widest axis) that the band keeps. */
type DoorSide = { readonly grips: number; readonly wheelSets: number; readonly spans?: number };
const DOOR: Readonly<Record<Scale, { readonly ext: DoorSide; readonly int: DoorSide; readonly words?: RegExp }>> = {
  50: { ext: { grips: 1, wheelSets: 2, spans: 4 }, int: { grips: 1, wheelSets: 2, spans: 4 } },
  100: { ext: { grips: 1, wheelSets: 2, spans: 4 }, int: { grips: 1, wheelSets: 2, spans: 4 } },
  200: { ext: { grips: 1, wheelSets: 2, spans: 4 }, int: { grips: 1, wheelSets: 0 }, words: /each wheel pair is one 30 mm through roller/ },
  500: { ext: { grips: 0, wheelSets: 0 }, int: { grips: 0, wheelSets: 0 }, words: /floor guide, floor seal, beads and handles are left out/ },
};

/** 🧱️ The bonded brick wall: the bands of its body per scale (twelve bed joints apart, twelve courses in two repeated bands that touch, six bond bands in two repeated bands that touch, one wall). */
const BRICK_WALL: Readonly<Record<Scale, Resolves>> = {
  50: { parts: /^bed-joint$/, count: 1, spans: 12, present: ["plaster", "render"] },
  100: { parts: /^course-[ab]$/, count: 2, copies: 12, spans: 1, present: ["render"], absent: [/^plaster$/, /^bed-joint$/], words: /every course is one band of 83 mm with its bed joint in it/ },
  200: { parts: /^bond-band-[ab]$/, count: 2, copies: 6, spans: 1, absent: [/^plaster$/, /^render$/, /^bed-joint$/, /^course-/], words: /the wall is six bond bands of 167 mm/ },
  500: { parts: /^wall$/, count: 1, spans: 1, absent: [/^plaster$/, /^render$/, /^bed-joint$/, /^course-/, /^bond-band/], words: /the wall is one homogeneous 400 mm body/ },
};

/** 🪝️ The tied canopy in the structure list: the end fittings of the tie (`joints` ids per end) and the studs behind the pads. */
type Canopy = { readonly joints: (end: string) => readonly string[]; readonly studs?: Resolves; readonly absent?: readonly RegExp[]; readonly words?: RegExp };
const CANOPY: Readonly<Record<Scale, Canopy>> = {
  50: { joints: (end) => ["j-clevis-" + end, "j-pin-" + end], studs: { parts: /^j-stud-/, count: 4, spans: 2 } },
  100: { joints: (end) => ["j-clevis-" + end, "j-pin-" + end], studs: { parts: /^j-stud-/, count: 4, spans: 2 } },
  200: { joints: (end) => ["j-node-" + end], absent: [/^j-(clevis|pin|gusset)/, /^j-stud-/], studs: { parts: /^j-studs-/, count: 2, spans: 1 }, words: /gusset, fork head, bridge and pin of each strut end are one 40 mm node lug[\s\S]*the structure list carries each stud group as one anchorage block behind its pad/ },
  500: { joints: () => [], absent: [/^j-/], words: /Left out: wall layers, anchor pads and plates, node lugs, joints and every fixing/ },
};

/** 🚪️ The sectional garage door: the hinge of the band is a pin on every joint or, at 1:500, one block across the joint that carries the roller stem. */
const GARAGE: Readonly<Record<Scale, Resolves>> = {
  50: { parts: /^hinge-pin-[1-3]$/, count: 3 },
  100: { parts: /^hinge-pin-[1-3]$/, count: 3 },
  200: { parts: /^hinge-pin-[1-3]$/, count: 3 },
  500: { parts: /^hinge-leaf-up-[1-3]$/, count: 3, absent: [/^hinge-pin-/], words: /each hinge is one 80 x 40 x 80 mm block across the joint that carries the roller stem/ },
};

const LINTEL_PIECES = ["lintel-outer", "lintel-inner", "cavity-wool", "reveal-plaster-left", "reveal-plaster-right"] as const;
const LINTEL_COURSING = [/^(bed|brick|head|piece|block|course)-/, /^wall-tie-/] as const;

/** 🧱️ The masonry wall with lintel: 33 bed joints, then 53 course bands, then one wall mass with the lintel band. */
const LINTEL_WALL: Readonly<Record<Scale, Resolves>> = {
  50: { parts: /^bed-/, spans: 33, present: LINTEL_PIECES },
  100: { parts: /^course-/, count: 53, present: LINTEL_PIECES, absent: [/^(bed|brick|head|piece|block)-/, /^wall-tie-/], words: /the facing is 53 course bands of 83 mm/ },
  200: { present: ["wall", "lintel"], absent: [...LINTEL_COURSING, /^lintel-(outer|inner)$/, /^cavity-wool$/, /^reveal-plaster/], words: /the four leaves are one 445 mm wall mass around the opening/ },
  500: { present: ["wall", "lintel"], absent: [...LINTEL_COURSING, /^lintel-(outer|inner)$/, /^cavity-wool$/, /^reveal-plaster/], words: /one wall mass around the opening, the lintel band with its bearings/ },
};

/** 🪟️ The glazing of the curtain wall bay: the panes with the 16 mm cavity at 1:50, one 28 mm strip at 1:100 and 1:200, one vision block at 1:500. */
type Glazing = { readonly ids: readonly string[]; readonly thickness: number; readonly gap?: number; readonly words?: RegExp };
const CURTAIN: Readonly<Record<Scale, Glazing>> = {
  50: { ids: ["igu-pane-in", "igu-pane-out"], thickness: 0.006, gap: 0.016 },
  100: { ids: ["igu"], thickness: 0.028, words: /panes, cavity and edge seal are one 28 mm glazing strip/ },
  200: { ids: ["igu"], thickness: 0.028, words: /the glazing is one strip of its true 28 mm/ },
  500: { ids: ["vision"], thickness: 0.12, words: /one vision block of 2\.45 m and one spandrel block of 0\.85 m, both 120 mm deep/ },
};
const TILT: Glazing = { ids: ["pane-inner", "pane-outer"], thickness: 0.004, gap: 0.016 };

/** 🔥️ The collar of the flue of the gas boiler in the structure list: the spigot with its 3 mm wall at 1:50, then one solid collar (two steps, then a 125 mm drum). */
const BOILER: Readonly<Record<Scale, { readonly id: string; readonly inner: number; readonly outer: number; readonly words?: RegExp }>> = {
  50: { id: "member-collar-spigot", inner: 0.037, outer: 0.04 },
  100: { id: "member-collar", inner: 0, outer: 0.0625, words: /the collar is two solid steps/ },
  200: { id: "member-collar", inner: 0, outer: 0.0625, words: /the collar is one 125 mm drum/ },
  500: { id: "member-collar", inner: 0, outer: 0.0625, words: /the 80\/125 flue adapter is one solid 125 mm drum/ },
};

/** 🧱️ The layers of the ETICS wall in the body: the plaster, the insulation (its depth beyond the board thickness: the adhesive that the band merges into it) and the render as `[pattern, depth beyond the nominal]` or none. */
type EticsBody = { readonly plaster?: number; readonly insulation?: RegExp; readonly extra: number; readonly render?: number; readonly absent: readonly RegExp[]; readonly words?: RegExp };
const ETICS: Readonly<Record<Scale, EticsBody>> = {
  50: { plaster: 0.015, insulation: /wool-board|insulation/, extra: 0, render: 0.02, absent: [] },
  100: { insulation: /wool-board|insulation/, extra: 0.01, render: 0.02, absent: [/plaster/], words: /the 15 mm plaster lies in the 380 mm leaf as in the drawing; the 10 mm adhesive lies in the \d+ mm insulation boards; base coat and finishing render are one 20 mm coat/ },
  200: { insulation: /insulation/, extra: 0.03, absent: [/plaster/, /render/], words: /adhesive, wool and render are one \d+ mm ETICS layer in its two board courses/ },
  500: { extra: 0, absent: [/plaster/, /wool-board|insulation/, /render/], words: /the wall is one homogeneous \d+ mm body drawn as one poché/ },
};
const ETICS_ANCHORS: Readonly<Record<Scale, { readonly anchors: number; readonly words?: RegExp }>> = {
  50: { anchors: 4 },
  100: { anchors: 4, words: /the structure list keeps the four anchors as joints of the bearing leaf/ },
  200: { anchors: 0, words: /The structure list keeps the bearing 365 mm leaf on its support, without the anchors \(8 mm shafts, 60 mm plates\)/ },
  500: { anchors: 0, words: /the structure list is one bearing 365 mm leaf, its support and its mass/ },
};

/** 🛡️ The balcony guard: the balusters (count, width, centres of the outermost) and the fixing of the railing in the structure list (four anchors 90 or 110 mm deep, or one fixing block). */
type Guard = { readonly balusters: number; readonly width: number; readonly centres: readonly [number, number]; readonly reach?: number; readonly anchors?: number; readonly words?: RegExp };
const BALCONY: Readonly<Record<Scale, Guard>> = {
  50: { balusters: 8, width: 0.012, centres: [0.12589, 0.44656], anchors: 0.11 },
  100: { balusters: 8, width: 0.015, centres: [0.12589, 0.44656], reach: 0.855, anchors: 0.09, words: /the eight balusters 15 mm at the true pitch of 106\.9 mm[\s\S]*its joint is the foot with four 15 mm anchors 90 mm deep/ },
  200: { balusters: 8, width: 0.03, centres: [0.12589, 0.44656], reach: 0.84, words: /the eight balusters thickened to 30 mm at their true pitch[\s\S]*The railing member stands on a 150 mm fixing block that reaches 90 mm into the slab/ },
  500: { balusters: 2, width: 0.075, centres: [0.33967, 0.33967], reach: 0.8493, words: /two 75 mm balusters a third of a metre either side of the post[\s\S]*the railing member stands on a 150 mm fixing block/ },
};

/** 🌡️ The radiator pipes of the connection and the plate of the top grille: two 15 mm pipes on 50 mm centres, one pair body 80 mm across, or the pipes inside one connection block; the slotted plate or the closed 30 mm top plate. */
type Radiator = { readonly pipes: readonly number[]; readonly slotted: boolean; readonly words?: RegExp };
const RADIATOR: Readonly<Record<Scale, Radiator>> = {
  50: { pipes: [0.015, 0.015], slotted: true },
  100: { pipes: [0.015, 0.015], slotted: true },
  200: { pipes: [0.08], slotted: true, words: /the two 15 mm pipes on 50 mm centres are one pair body of 30 x 80 mm/ },
  500: { pipes: [], slotted: false, words: /closed 30 mm top plate; no side covers and no slots[\s\S]*The connection with its union nuts, nipples and both pipes is one block below the panel end/ },
};

/** 🌡️ The fin stack and the tube rows of the finned heat exchanger: every foil, one block, or the sample plates of the structure list; the twelve tubes of both rows, or the hidden core. */
type Exchanger = { readonly rows: boolean; readonly foils?: number; readonly block?: boolean; readonly plates: number; readonly plate?: number; readonly words?: RegExp };
const EXCHANGER: Readonly<Record<Scale, Exchanger>> = {
  50: { rows: true, foils: 134, plates: 5, plate: 0.0002 },
  100: { rows: true, block: true, plates: 4, plate: 0.015, words: /The fin pack is therefore one block of 69 mm by 480 mm by 400 mm/ },
  200: { rows: false, plates: 2, plate: 0.03, words: /The tubes are a hidden core of 51 mm by 440 mm by 390 mm inside the coil block for the thermal and the structural model; it carries two sample fin plates of 30 mm/ },
  500: { rows: false, plates: 2, plate: 0.075, words: /The thermal model nests the tube core[\s\S]*two fin plates of 75 mm/ },
};

/** 🌀️ The louvred side of the heat pump: 25 louvres, twelve, six bars or one grille slab, with the declaration that says so. */
const PUMP_SIDE: Readonly<Record<Scale, { readonly parts: RegExp; readonly spans: number; readonly words?: RegExp }>> = {
  50: { parts: /side-louvres/, spans: 25, words: /25 louvres at 40 degrees guard the side/ },
  100: { parts: /side-louvres/, spans: 12, words: /twelve louvres of 100 mm pitch/ },
  200: { parts: /side-louvres/, spans: 6, words: /six louvre bars/ },
  500: { parts: /side-grille/, spans: 1, words: /the louvred side as one grille slab/ },
};

/** 🛡️ The frame of an intake of the generator in the structure list: its stock (the jamb of 1:50 to 1:200) at the height of the lower bed rail, or none. */
const GENERATOR_STOCK: Readonly<Record<Scale, { readonly forces: boolean; readonly words?: RegExp }>> = {
  50: { forces: true },
  100: { forces: true },
  200: { forces: true },
  500: { forces: false, words: /Structure: 80 mm rails and sills, three cross members, posts, top rails, eight castings as joints/ },
};

describe("guard stock and thermal equipment internals", () => {
  const read = (file: string): Asset => JSON.parse(readFileSync(new URL("../../🗂️catalog/" + file + "/🔣️.json", import.meta.url), "utf8")).asset;
  const balcony = read("🧱️components/cantilever-balcony");
  const radiator = read("🧱️components/panel-radiator");
  const exchanger = read("🏗️machinery/finned-heat-exchanger");
  const pump = read("🏗️machinery/heat-pump-outdoor-unit");
  const generator = read("🏗️machinery/diesel-generator");
  const insulatedWalls = [
    { asset: read("🧱️components/etics-brick-wall-12"), thickness: 0.12 },
    { asset: read("🧱️components/etics-brick-wall-30"), thickness: 0.30 },
  ];
  const blind = read("🧱️components/external-venetian-blind");
  const collector = read("🏗️machinery/solar-thermal-collector");
  const compressor = read("🏗️machinery/scroll-compressor");
  const glassBalustrade = read("🧱️components/glass-balustrade");
  const slidingDoor = read("🧱️components/sliding-door");
  const brickWall = read("🧱️components/solid-brick-wall-1960");
  const canopy = read("🧱️components/tied-canopy");
  const garageDoor = read("🧱️components/sectional-garage-door");
  const lintelWall = read("🧱️components/masonry-wall-with-lintel");
  const curtainWall = read("🧱️components/curtain-wall-bay");
  const tiltWindow = read("🧱️components/tilt-window");
  const boiler = read("🔌️appliances/gas-boiler");

  for (const scale of SCALES) {
    it(`retains the drilled laminate, door running gear and bonded masonry as the band of 1:${scale} resolves them`, async () => {
      const make = async (asset: Asset) => (await modelOf(asset, { scale, view: "light-solid", poses: [], clips: [] })).solids;
      const glass = await make(glassBalustrade);
      const glassWant = GLASS[scale];
      if (glassWant.words !== undefined) said(glassBalustrade, scale, glassWant.words);
      for (const [id, expected] of glassWant.laminate) {
        const pane = glass.filter((p) => p.id === id);
        expect(pane, `${id} solids`).toHaveLength(1);
        const widths = [0, 1, 2].map((axis) => { const [part] = occupied(pane, axis); return part![1]! - part![0]!; });
        expect(Math.min(...widths), id).toBeCloseTo(expected, 5);
      }
      for (const [id, spans] of glassWant.fasteners) expect(separate(glass.filter((p) => p.id === id)), id).toBe(spans);
      expect(glass.filter((p) => glassWant.fasteners.length === 0 && /^(anchor|bolt)/.test(p.id)).map((p) => p.id), "fasteners left out by the band").toEqual([]);
      const door = await make(slidingDoor);
      const doorWant = DOOR[scale];
      if (doorWant.words !== undefined) said(slidingDoor, scale, doorWant.words);
      expect(door.some((p) => p.id === "leaf-glass"), "leaf-glass").toBe(true);
      expect(door.some((p) => p.id === "trolley-chassis"), "trolley-chassis").toBe(true);
      for (const side of ["ext", "int"] as const) {
        const want = doorWant[side];
        expect(door.filter((p) => p.id === "handle-grip-" + side), `handle-grip-${side}`).toHaveLength(want.grips);
        const wheelsets = door.filter((p) => p.id.startsWith("trolley-") && p.id.endsWith("wheels-" + side));
        expect(wheelsets.map((p) => p.id), `wheel sets ${side}`).toHaveLength(want.wheelSets);
        if (want.spans !== undefined) expect(separate(wheelsets), `wheels ${side}`).toBe(want.spans);
      }
      resolved(brickWall, scale, await make(brickWall), BRICK_WALL[scale], "wall");
    });
    it(`retains the canopy pins, ten door rollers and lintel facing courses as the band of 1:${scale} resolves them`, async () => {
      const solids = async (asset: Asset, view: "light-solid" | "forces") => (await modelOf(asset, { scale, view, poses: [], clips: [] })).solids;
      const tie = await solids(canopy, "forces");
      const canopyWant = CANOPY[scale];
      if (canopyWant.words !== undefined) said(canopy, scale, canopyWant.words);
      for (const end of ["wall", "beam"]) for (const id of canopyWant.joints(end)) expect(tie.some((p) => p.id === id), id).toBe(true);
      resolved(canopy, scale, tie, { parts: canopyWant.studs?.parts, count: canopyWant.studs?.count, absent: canopyWant.absent }, "studs");
      if (canopyWant.studs?.parts !== undefined) for (const row of tie.filter((p) => canopyWant.studs!.parts!.test(p.id))) expect(separate([row]), row.id).toBe(canopyWant.studs.spans);
      const garage = await solids(garageDoor, "light-solid");
      expect(garage.filter((p) => /^roller-[0-4]-[lr]$/.test(p.id))).toHaveLength(10);
      resolved(garageDoor, scale, garage, GARAGE[scale], "hinges");
      resolved(lintelWall, scale, await solids(lintelWall, "light-solid"), LINTEL_WALL[scale], "wall");
    });
    it(`keeps the insulating glass cavity empty in the curtain wall and tilt sash as the band of 1:${scale} resolves it`, async () => {
      for (const [asset, want] of [[curtainWall, CURTAIN[scale]], [tiltWindow, TILT]] as const) {
        if (want.words !== undefined) said(asset, scale, want.words);
        const glass = (await modelOf(asset, { scale, view: "light-solid", poses: [], clips: [] })).solids.filter((p) => want.ids.some((id) => id === p.id));
        expect(glass.map((p) => p.id), `${asset.id} glazing`).toHaveLength(want.ids.length);
        const meshes = glass.map((p) => tessellateSolid(p, { chordM: 0.0001, minSegments: 16, maxSegments: 32 }));
        const positions = meshes[0]!.positions, indices = meshes[0]!.indices;
        let largest = 0; const normal = new Vector3();
        for (let i = 0; i < indices.length; i += 3) {
          const points = [0, 1, 2].map((j) => new Vector3().fromArray(positions, indices[i + j]! * 3));
          const axis = points[1]!.sub(points[0]!).cross(points[2]!.sub(points[0]!));
          if (axis.lengthSq() > largest) { largest = axis.lengthSq(); normal.copy(axis).normalize(); }
        }
        const ranges = meshes.map(({ positions }) => {
          const samples = Array.from({ length: positions.length / 3 }, (_, i) => new Vector3().fromArray(positions, 3 * i).dot(normal));
          return [Math.min(...samples), Math.max(...samples)];
        }).sort((lo, hi) => lo[0]! - hi[0]!);
        for (const [start, end] of ranges) expect(end! - start!, `${asset.id} thickness`).toBeCloseTo(want.thickness, 5);
        if (want.gap !== undefined) expect(ranges[1]![0]! - ranges[0]![1]!, `${asset.id} cavity`).toBeCloseTo(want.gap, 5);
      }
    });
    it(`keeps the boiler flue collar as the band of 1:${scale} resolves it`, async () => {
      const want = BOILER[scale];
      if (want.words !== undefined) said(boiler, scale, want.words);
      const solids = (await modelOf(boiler, { scale, view: "forces", poses: [], clips: [] })).solids;
      const collar = solids.find((p) => p.id === want.id);
      expect(collar, want.id).toBeDefined();
      const mesh = tessellateSolid(collar!, { chordM: 0.0001, minSegments: 24, maxSegments: 48 });
      const xyz = Array.from({ length: mesh.positions.length / 3 }, (_, i) => new Vector3().fromArray(mesh.positions, 3 * i));
      const centre = [0, 2].map((k) => { const a = xyz.map((p) => p.getComponent(k)); return (Math.min(...a) + Math.max(...a)) / 2; });
      const radii = xyz.map((p) => Math.hypot(p.x - centre[0]!, p.z - centre[1]!));
      expect(Math.min(...radii), `${want.id} bore`).toBeCloseTo(want.inner, 4);
      expect(Math.max(...radii), `${want.id} outside`).toBeCloseTo(want.outer, 4);
    });
    it(`keeps the layers and headed ETICS anchors as the band of 1:${scale} resolves them`, async () => {
      for (const { asset, thickness } of insulatedWalls) {
        const body = ETICS[scale];
        if (body.words !== undefined) said(asset, scale, body.words);
        const anchors = ETICS_ANCHORS[scale];
        if (anchors.words !== undefined) said(asset, scale, anchors.words);
        for (const view of VIEWS) {
          const { solids } = await modelOf(asset, { scale, view, poses: [], clips: [] });
          if (view === "forces") {
            const ties = solids.filter((s) => s.id.startsWith("joint-anchor-"));
            expect(ties, `${asset.id} anchors`).toHaveLength(anchors.anchors);
            for (const tie of ties) {
              const head = occupied([tie], 1)[0]!;
              expect(head[1]! - head[0]!).toBeCloseTo(0.06, 4);
            }
            continue;
          }
          const layers: readonly (readonly [RegExp, number])[] = view === "thermal"
            ? [[/plaster/, 0.015], [/wool-board|insulation/, thickness], [/render/, 0.02]]
            : [...(body.plaster === undefined ? [] : [[/plaster/, body.plaster] as const]), ...(body.insulation === undefined ? [] : [[body.insulation, thickness + body.extra] as const]), ...(body.render === undefined ? [] : [[/render/, body.render] as const])];
          for (const [pattern, depth] of layers) {
            const spans = occupied(solids.filter((s) => pattern.test(s.id)), 0);
            expect(spans, `${asset.id} ${view} ${pattern}`).toHaveLength(1);
            expect(spans[0]![1]! - spans[0]![0]!, `${asset.id} ${view} ${pattern}`).toBeCloseTo(depth, 5);
          }
          if (view !== "thermal") for (const pattern of body.absent) expect(solids.filter((s) => pattern.test(s.id)).map((s) => s.id), `${asset.id} ${view} ${pattern} merged by the band`).toEqual([]);
        }
      }
    });
    it(`suspends all eighteen blind slats on the lift tapes at 1:${scale}`, async () => {
      for (const view of VIEWS) {
        const { solids } = await modelOf(blind, { scale, view, poses: [], clips: [] });
        const slats = solids.filter((s) => /^(?:member-|th-)?slat-\d+$/.test(s.id));
        expect(slats).toHaveLength(18);
        for (const slat of slats) {
          const width = occupied([slat], 2)[0]!;
          expect(width[1]! - width[0]!).toBeCloseTo(1.13, 3);
        }
        if (view === "forces") {
          const tapes = solids.filter((s) => s.id.includes("lift-tape-top"));
          expect(occupied(tapes, 2)).toHaveLength(2);
          expect(solids.some((s) => s.id.includes("tape-bearing"))).toBe(true);
        }
      }
    });
    it(`carries the rear collector X on a pair of 2 mm straps at 1:${scale}`, async () => {
      for (const view of VIEWS) {
        const { solids } = await modelOf(collector, { scale, view, poses: [], clips: [] });
        const straps = solids.filter((s) => s.id.includes("brace-rising") && !s.id.includes("bolt"));
        expect(straps).toHaveLength(2);
        for (const strap of straps) {
          const width = occupied([strap], 0)[0]!;
          expect(width[1]! - width[0]!).toBeCloseTo(0.002, 4);
          const reach = occupied([strap], 2)[0]!;
          expect(reach[1]! - reach[0]!).toBeGreaterThan(0.85);
        }
      }
    });
    it(`has an open airflow passage through each generator intake, framed as the structure list of 1:${scale} frames it`, async () => {
      const stock = GENERATOR_STOCK[scale];
      if (stock.words !== undefined) said(generator, scale, stock.words);
      for (const view of VIEWS) {
        const { solids } = await modelOf(generator, { scale, view, poses: [], clips: [] });
        const wall = triangles(solids.filter((p) => p.node === "container" && !p.tags?.includes("part-mass")));
        for (const sign of [-1, 1]) {
          for (const x of [0.52, 3.12]) {
            const ray = (y: number) => new Ray(new Vector3(x, y, 1.3 * sign), new Vector3(0, 0, -sign));
            expect(hit(ray(0.65), wall), view + " free air channel").toBe(false);
            expect(hit(ray(0.525), wall), view + " frame stock").toBe(view === "forces" ? stock.forces : true);
          }
        }
      }
    });
    it(`seats the compressor on four pierced isolators and a bored foot plate at 1:${scale}`, async () => {
      for (const view of VIEWS) {
        const { solids } = await modelOf(compressor, { scale, view, poses: [], clips: [] });
        const isolators = solids.filter((p) => (view === "forces" ? p.tags?.includes("part-support") === true : /^(th-)?grommets$/.test(p.id)));
        expect(isolators.map((p) => p.id), `${view} rubber mounts of the structure list are the four supports, of the body and thermal list the grommets`).toHaveLength(view === "forces" ? 4 : 1);
        const mounts = triangles(isolators);
        const baseplate = solids.filter((p) => p.id.includes("base-plate") || p.id === "foot-frame");
        expect(baseplate.length, `${view} foot plate`).toBeGreaterThan(0);
        const plate = triangles(baseplate);
        const down = (x: number, z: number) => new Ray(new Vector3(x, 0.065, z), new Vector3(0, -1, 0));
        expect(hit(down(0, 0), mounts), view + " separated feet").toBe(false);
        for (const x of [-0.09525, 0.09525]) for (const z of [-0.09525, 0.09525]) {
          expect(hit(down(x, z), mounts), view + " rubber bore").toBe(false);
          expect(hit(down(x + 0.012, z), mounts), view + " annular rubber").toBe(true);
          expect(hit(down(x, z), plate), view + " plate bore").toBe(false);
          expect(hit(down(x - Math.sign(x) * 0.018, z), plate), view + " mounting ear").toBe(true);
        }
      }
    });
    it(`carries an open balcony guard down to its anchors as the band of 1:${scale} resolves it`, async () => {
      const want = BALCONY[scale];
      if (want.words !== undefined) said(balcony, scale, want.words);
      for (const view of VIEWS) {
        const { solids } = await modelOf(balcony, { scale, view, poses: [], clips: [] });
        const rods = solids.filter((s) => s.id.includes("balusters"));
        const sections = occupied(rods, 2);
        expect(sections).toHaveLength(want.balusters);
        const centers = sections.map(([lo, hi]) => {
          expect(hi! - lo!).toBeCloseTo(want.width, 4);
          return Math.abs((hi! + lo!) / 2);
        });
        const [reach] = occupied(rods, 0);
        if (want.reach === undefined) expect(reach![1]! - reach![0]!).toBeGreaterThan(0.86);
        else expect(reach![1]! - reach![0]!).toBeCloseTo(want.reach, 3);
        expect(Math.min(...centers)).toBeCloseTo(want.centres[0], 4);
        expect(Math.max(...centers)).toBeCloseTo(want.centres[1], 4);
        if (view === "forces") {
          const anchors = solids.filter((s) => s.id.includes("railing-anchors"));
          expect(anchors.map((s) => s.id), "railing anchors").toHaveLength(want.anchors === undefined ? 0 : 2);
          for (const row of anchors) expect(occupied([row], 2)).toHaveLength(2);
          const lengths = anchors.map((s) => {
            const { positions } = tessellateSolid(s, { chordM: 0.0001, minSegments: 24, maxSegments: 48 });
            const x = positions.filter((_, i) => i % 3 === 0);
            return Math.max(...x) - Math.min(...x);
          });
          for (const length of lengths) expect(length).toBeCloseTo(want.anchors!, 4);
          expect(solids.some((s) => s.id === "joint-railing-fixing"), "fixing block of the railing").toBe(want.anchors === undefined);
        }
      }
    });
    it(`retains the two radiator pressure panels and the outlet as the band of 1:${scale} resolves them`, async () => {
      const want = RADIATOR[scale];
      if (want.words !== undefined) said(radiator, scale, want.words);
      for (const view of VIEWS) {
        const { solids } = await modelOf(radiator, { scale, view, poses: [], clips: [] });
        if (view === "forces") {
          for (const side of ["front", "rear"]) {
            const s = solids.find((p) => p.id === "member-panel-" + side);
            expect(s).toBeDefined();
            const geo = tessellateSolid(s!, { chordM: 0.001, minSegments: 12, maxSegments: 32 });
            const z = geo.positions.filter((_, i) => i % 3 === 2);
            expect(Math.max(...z) - Math.min(...z)).toBeCloseTo(1, 3);
          }
        } else {
          const plate = solids.find((s) => s.id.endsWith("grille-plate"));
          expect(plate).toBeDefined();
          const geo = tessellateSolid(plate!, { chordM: 0.001, minSegments: 12, maxSegments: 32 });
          if (want.slotted) expect(geo.indices.length / 3).toBeGreaterThan(120);
          else expect(geo.indices.length / 3).toBe(12);
          const pipes = occupied(solids.filter((p) => p.id === "pipe" || p.id === "th-pipe"), 2);
          expect(pipes.map(([lo, hi]) => hi! - lo!), `${view} pipes`).toHaveLength(want.pipes.length);
          pipes.forEach(([lo, hi], i) => expect(hi! - lo!).toBeCloseTo(want.pipes[i]!, 4));
          if (pipes.length === 2) expect(pipes[1]![0]! - pipes[0]![0]!).toBeCloseTo(0.05, 4);
        }
      }
    });
    it(`carries both rows of twelve tubes and an open fin stack as the band of 1:${scale} resolves them`, async () => {
      const want = EXCHANGER[scale];
      if (want.words !== undefined) said(exchanger, scale, want.words);
      for (const view of VIEWS) {
        const { solids } = await modelOf(exchanger, { scale, view, poses: [], clips: [] });
        for (const row of ["entering", "leaving"]) {
          const tubes = solids.filter((s) => s.id.includes("tube-row-" + row));
          expect(occupied(tubes, 0), `${view} tube row ${row}`).toHaveLength(want.rows ? 12 : 0);
        }
        if (!want.rows) expect(solids.some((s) => s.id.includes("tube-core")), `${view} hidden tube core`).toBe(view !== "light-solid");
        const fins = solids.filter((s) => s.id.includes(view === "forces" ? "member-fin-plates" : "fin-pack") && !s.tags?.includes("part-mass"));
        const plates = occupied(fins, 2);
        if (view === "forces") {
          expect(plates, `${view} sample fin plates`).toHaveLength(want.plates);
          for (const [lo, hi] of plates) expect(hi! - lo!).toBeCloseTo(want.plate!, 5);
        } else if (want.foils !== undefined) {
          expect(plates).toHaveLength(want.foils);
          for (const [lo, hi] of plates) expect(hi! - lo!).toBeCloseTo(0.0002, 5);
          for (let i = 1; i < plates.length; i++) expect(plates[i]![0]! - plates[i - 1]![1]!).toBeCloseTo(0.0028, 5);
        } else expect(plates, `${view} fin pack as one block`).toHaveLength(want.block === true ? 1 : 0);
      }
    });
    it(`keeps the three fan blades, the louvred side and the motor support as the band of 1:${scale} resolves them`, async () => {
      const side = PUMP_SIDE[scale];
      if (side.words !== undefined) said(pump, scale, side.words);
      for (const view of VIEWS) {
        const { solids } = await modelOf(pump, { scale, view, poses: [], clips: [] });
        if (view === "forces") {
          expect(solids.some((s) => s.id.includes("motor-bracket"))).toBe(true);
          expect(solids.some((s) => s.id === "mounted-fan-motor")).toBe(true);
          expect(solids.some((s) => s.id.includes("compressor"))).toBe(true);
        } else {
          const blades = solids.filter((s) => s.id.includes("fan-blades"));
          expect(blades).toHaveLength(1);
          const louvres = solids.filter((s) => side.parts.test(s.id));
          expect(louvres.map((s) => s.id), `${view} louvred side`).toHaveLength(1);
          expect(occupied(louvres, 1), `${view} louvres across the side`).toHaveLength(side.spans);
          const intakeDepth = occupied(louvres, 2).at(-1)![1]!;
          const coilDepth = occupied(solids.filter((s) => s.id === "coil" || s.id === "th-coil"), 2).at(-1)![1]!;
          expect(intakeDepth - coilDepth).toBeGreaterThan(0.02);
          for (const blade of blades) {
            const geo = tessellateSolid(blade, { chordM: 0.001, minSegments: 12, maxSegments: 32 });
            const tips = Array.from({ length: geo.positions.length / 3 }, (_, i) => ({ radius: Math.hypot(geo.positions[3 * i + 1]!, geo.positions[3 * i + 2]!), yaw: Math.atan2(geo.positions[3 * i + 2]!, geo.positions[3 * i + 1]!) })).filter((p) => p.radius > 0.26);
            expect(tips.length, `${view} blade tips beyond 0.26 m about the axis of the fan bone`).toBeGreaterThan(0);
            const yaws = tips.map((p) => p.yaw).sort((a, b) => a - b);
            const gaps = yaws.map((yaw, i) => [i, (yaws[(i + 1) % yaws.length]! - yaw + 2 * Math.PI) % (2 * Math.PI)] as const).filter(([, gap]) => gap > 0.6);
            expect(gaps, `${view} blades about the fan bone`).toHaveLength(3);
            const centres = gaps.map(([end], k) => {
              const start = (gaps[(k + gaps.length - 1) % gaps.length]![0] + 1) % yaws.length;
              const members = Array.from({ length: (end - start + yaws.length) % yaws.length + 1 }, (_, n) => yaws[(start + n) % yaws.length]!);
              return Math.atan2(members.reduce((sum, a) => sum + Math.sin(a), 0), members.reduce((sum, a) => sum + Math.cos(a), 0));
            });
            for (let k = 0; k < 3; k++) expect(Math.abs(Math.sin((centres[(k + 1) % 3]! - centres[k]!) / 2 - Math.PI / 3)), `${view} blade ${k} stands 120 degrees from the next`).toBeLessThan(0.1);
          }
        }
      }
    });
  }
});
