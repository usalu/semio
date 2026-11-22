/** 🧊️ The extent of the 3D model: at rest against the physical `dimensions` of the asset, and for every named pose against the elevation, front and plan extents of the same evaluated state (decision D18: the larger of 2 % and 15 mm).
 *
 * THE REFERENCES are three.js (`Box3.expandByObject(object, true)`, the precise skinned extent of the GLB loaded by `GLTFLoader`), the 2D shapes of `evaluate`, and the hand-computed extent of the sample figure's rest frames in `🧫️fixtures/🧊️volume-schema`.
 *
 * The ahead and up extents are compared with the elevation (the view that shows both), the lateral extent with the front and the plan; the sample figure's authored front shapes stay on the ground line in a stride, so their vertical extent is not a reference.
 *
 * @see ./🥒️.feature — the scenarios
 */
import { readFileSync } from "node:fs";
import { Box3, Ray, Triangle, Vector3 } from "three";
import { tessellateSolid } from "../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import type { Asset } from "../../🧬️schema/🟦️.ts";
import { describe, expect, it } from "vitest";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { writeGlb } from "../../🔨️modules/📤️export/🧊️glb/🟦️.ts";
import { modelOf } from "../../🔨️modules/🧊️volume/🟦️.ts";
import { extentTolerance, loadGlb, restExtent, sampleFigure, sceneExtent, shapeExtent } from "../🧰️support/🧊️.ts";

const SCALE = 50;
const within = (got: number, want: number, what: string): void => void expect(Math.abs(got - want), `${what}: ${got} vs ${want}`).toBeLessThanOrEqual(extentTolerance(want));

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

describe("small equipment rolling contacts", () => {
  const scenarios = [
    { file: "../../🗂️catalog/🔌️appliances/vacuum-cleaner/🔣️.json", scales: [50, 100, 200, 500], stations: [
      { node: "wheel", x: 0, y: -0.06, z: 0.1375, width: 0.025 },
      { node: "caster", x: 0, y: -0.025, z: 0.018, width: 0.012 },
    ] },
    { file: "../../🗂️catalog/🪑️objects/shopping-trolley/🔣️.json", scales: [500], stations: [
      { node: "chassis", x: 0, y: -0.1675, z: 0.25, width: 0.032 },
      { node: "chassis", x: 0.56, y: -0.1675, z: 0.198, width: 0.032 },
    ] },
  ] as const;

  for (const scenario of scenarios) {
    const asset: Asset = JSON.parse(readFileSync(new URL(scenario.file, import.meta.url), "utf8")).asset;
    for (const scale of scenario.scales) it(`keeps four separated rolling contacts on ${asset.id} at 1:${scale}`, async () => {
      for (const view of ["light-solid", "thermal", "forces"] as const) {
        const model = await modelOf(asset, { scale, view, poses: [], clips: [] });
        for (const station of scenario.stations) {
          const contacts = new Set<number>();
          const rims = new Set<number>();
          for (const solid of model.solids.filter((s) => s.node === station.node && !s.tags?.includes("part-mass"))) {
            const mesh = tessellateSolid(solid, { chordM: 0.0001, minSegments: 32, maxSegments: 64 });
            if (station.node === "wheel" && view === "forces" && solid.tags?.includes("part-member")) {
              for (const side of [-1, 1]) {
                let inner = Infinity, outer = 0;
                for (let j = 0; j < mesh.positions.length; j += 3) {
                  const vertex = new Vector3().fromArray(mesh.positions, j);
                  if (Math.abs(vertex.z - side * station.z) > station.width / 2) continue;
                  const radius = Math.hypot(vertex.x, vertex.y);
                  inner = Math.min(inner, radius);
                  outer = Math.max(outer, radius);
                }
                if (inner < 0.002 && outer >= 0.044) rims.add(side);
              }
            }
            for (let i = 0; i < mesh.indices.length; i += 3) {
              const points = [0, 1, 2].map((j) => new Vector3().fromArray(mesh.positions, 3 * mesh.indices[i + j]!));
              const low = points.filter((p) => Math.abs(p.y - station.y) < 0.0002 && Math.abs(p.x - station.x) < 0.04);
              if (!low.length) continue;
              const mid = low.reduce((sum, p) => sum.add(p), new Vector3()).divideScalar(low.length);
              for (const p of [...low, mid]) {
                expect(Math.abs(Math.abs(p.z) - station.z), `${view} ${solid.id}`).toBeLessThanOrEqual(station.width / 2 + 0.0002);
                contacts.add(Math.sign(p.z));
              }
            }
          }
          if (station.node === "wheel" && view === "forces") expect([...rims].sort(), "rims connect both treads to the axle").toEqual([-1, 1]);
          expect([...contacts].sort(), `${view} ${station.node}`).toEqual([-1, 1]);
        }
      }
    });
  }
});

describe("street lamp access and spigot", () => {
  const lamp: Asset = JSON.parse(readFileSync(new URL("../../🗂️catalog/🪑️objects/street-lamp/🔣️.json", import.meta.url), "utf8")).asset;
  it.each([50, 100, 200, 500] as const)("seats the real arm, access lock and cover latch at 1:%i", async (scale) => {
    for (const view of ["light-solid", "thermal", "forces"] as const) {
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

describe("portable heater and slatted park bench", () => {
  const source = (file: string): Asset => JSON.parse(readFileSync(new URL(file, import.meta.url), "utf8")).asset;
  const heater = source("../../🗂️catalog/🔌️appliances/electric-fan-heater/🔣️.json");
  const bench = source("../../🗂️catalog/🛋️furniture/park-bench/🔣️.json");

  function bands(samples: boolean[]) {
    return samples.filter((inside, index) => inside && !samples[index - 1]).length;
  }

  it.each([50, 100, 200, 500] as const)("keeps the carry grip open and resolves the heater outlet at 1:%i", async (scale) => {
    for (const view of ["light-solid", "thermal", "forces"] as const) {
      const model = await modelOf(heater, { scale, view, poses: [], clips: [] });
      const face = triangles(model.solids.filter((s) => s.node === "body" && !s.tags?.includes("part-mass")));
      const across = (y: number, z: number) => new Ray(new Vector3(0.20, y, z), new Vector3(-1, 0, 0));
      for (const z of [-0.04, 0, 0.04]) {
        expect(hit(across(0.286, z), face), `${view} hand clearance ${z}`).toBe(false);
        expect(hit(across(0.304, z), face), `${view} bar above hand ${z}`).toBe(true);
      }
      if (view !== "forces") {
        const knobs = model.solids.filter((s) => s.node === "knob");
        const gauge = triangles(knobs);
        expect(hit(new Ray(new Vector3(0, 0.017, -0.15), new Vector3(0, 0, 1)), gauge), `end of the 44 mm control dial in ${view}`).toBe(true);
        const aperture = Array.from({ length: 155 }, (_, i) => hit(across(0.0673 + i * 0.001, 0.076), face, 0.08));
        expect(bands(aperture), `${view} louvre bands across outlet`).toBe(8);
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

describe("guard stock and thermal equipment internals", () => {
  const read = (file: string): Asset => JSON.parse(readFileSync(new URL("../../🗂️catalog/" + file + "/🔣️.json", import.meta.url), "utf8")).asset;
  const balcony = read("🧱️components/cantilever-balcony");
  const radiator = read("🧱️components/panel-radiator");
  const exchanger = read("🏗️machinery/finned-heat-exchanger");
  const pump = read("🏗️machinery/heat-pump-outdoor-unit");
  const generator = read("🏗️machinery/diesel-generator");
  const insulatedWalls = [
    {asset: read("🧱️components/etics-brick-wall-12"), thickness: .12},
    {asset: read("🧱️components/etics-brick-wall-30"), thickness: .30},
  ];
  const blind = read("🧱️components/external-venetian-blind");
  const collector = read("🏗️machinery/solar-thermal-collector");
  const compressor = read("🏗️machinery/scroll-compressor");

  function occupied(solids: Awaited<ReturnType<typeof modelOf>>["solids"], axis: number) {
    const spans: number[][] = [];
    for (const solid of solids) {
      const mesh = tessellateSolid(solid, {chordM: .0001, minSegments: 24, maxSegments: 48});
      for (let i = 0; i < mesh.indices.length; i += 3) {
        const points = [0, 1, 2].map(j => mesh.positions[3 * mesh.indices[i + j]! + axis]!);
        spans.push([Math.min(...points), Math.max(...points)]);
      }
    }
    const merged: number[][] = [];
    for (const [lo, hi] of spans.sort((a,b) => a[0]! - b[0]!)) {
      const last = merged.at(-1);
      if (last && lo! <= last[1]! + .000001) last[1] = Math.max(last[1]!, hi!);
      else merged.push([lo!, hi!]);
    }
    return merged;
  }
  for (const scale of [50, 100, 200, 500] as const) {
    it(`retains the drilled laminate, door running gear and bonded masonry at 1:${scale}`, async () => {
      const make = async (slug: string) => (await modelOf(read("🧱️components/"+slug), {scale,view:"light-solid",poses:[],clips:[]})).solids;
      const glass = await make("glass-balustrade");
      for (const [id, expected] of [["pane-fixed",.01],["interlayer",.0015],["pane-clamp",.01]] as const) {
        const pane=glass.filter(p=>p.id===id);
        expect(pane).toHaveLength(1);
        const widths=[0,1,2].map(axis => {const [part]=occupied(pane,axis);return part![1]!-part![0]!;});
        expect(Math.min(...widths)).toBeCloseTo(expected,5);
      }
      for(const id of ["anchor-stud-fixed","anchor-stud-clamp","bolt-shank"]) {
        const parts=glass.filter(p=>p.id===id);
        expect(Math.max(...[0,1,2].map(axis=>occupied(parts,axis).length))).toBe(id==="bolt-shank"?7:5);
      }
      const door=await make("sliding-door");
      expect(door.some(p=>p.id==="leaf-glass")).toBe(true);
      for(const side of ["ext","int"]) {
        const grip=door.filter(p=>p.id==="handle-grip-"+side);
        expect(grip).toHaveLength(1);
        const wheelsets = door.filter(p=>p.id.startsWith("trolley-")&&p.id.endsWith("wheels-"+side));
        expect(wheelsets).toHaveLength(2);
        expect(Math.max(...[0,1,2].map(axis=>occupied(wheelsets,axis).length))).toBe(4);
      }
      const wall=await make("solid-brick-wall-1960");
      const beds=wall.filter(p=>p.id==="bed-joint");
      expect(Math.max(...[0,1,2].map(axis=>occupied(beds,axis).length))).toBe(12);
      for(const face of ["plaster","render"]) expect(wall.some(p=>p.id===face)).toBe(true);
    });
    it(`retains the canopy pins, ten door rollers and lintel facing courses at 1:${scale}`,async()=>{
      const solids=async(slug:string,view:"light-solid"|"forces")=>(await modelOf(read("🧱️components/"+slug),{scale,view,poses:[],clips:[]})).solids;
      const canopy=await solids("tied-canopy","forces");
      for(const joint of ["wall","beam"]) {
        expect(canopy.some(p=>p.id==="j-clevis-"+joint)).toBe(true);
        expect(canopy.some(p=>p.id==="j-pin-"+joint)).toBe(true);
      }
      for(const row of canopy.filter(p=>p.id.startsWith("j-stud-"))) {
        expect(Math.max(...[0,1,2].map(axis=>occupied([row],axis).length))).toBe(2);
      }
      expect(canopy.filter(p=>p.id.startsWith("j-stud-"))).toHaveLength(4);
      const garage=await solids("sectional-garage-door","light-solid");
      expect(garage.filter(p=>/^roller-[0-4]-[lr]$/.test(p.id))).toHaveLength(10);
      expect(garage.filter(p=>/^hinge-pin-[1-3]$/.test(p.id))).toHaveLength(3);
      const wall=await solids("masonry-wall-with-lintel","light-solid");
      const beds=wall.filter(p=>p.id.startsWith("bed-"));
      expect(Math.max(...[0,1,2].map(axis=>occupied(beds,axis).length))).toBe(33);
      for(const piece of ["lintel-outer","lintel-inner","cavity-wool","reveal-plaster-left","reveal-plaster-right"]) expect(wall.some(p=>p.id===piece)).toBe(true);
    });
    it(`keeps the insulating glass cavity empty in the curtain wall and tilt sash at 1:${scale}`,async()=>{
      for(const [slug,ids,thickness] of [["curtain-wall-bay",["igu-pane-in","igu-pane-out"],.006],["tilt-window",["pane-inner","pane-outer"],.004]] as const) {
        const glass=(await modelOf(read("🧱️components/"+slug),{scale,view:"light-solid",poses:[],clips:[]})).solids.filter(p=>ids.some(id=>id===p.id));
        expect(glass).toHaveLength(2);
        const meshes=glass.map(p=>tessellateSolid(p,{chordM:.0001,minSegments:16,maxSegments:32}));
        const positions=meshes[0]!.positions, indices=meshes[0]!.indices;
        let largest=0;const normal=new Vector3();
        for(let i=0;i<indices.length;i+=3) {
          const points=[0,1,2].map(j=>new Vector3().fromArray(positions,indices[i+j]!*3));
          const axis=points[1]!.sub(points[0]!).cross(points[2]!.sub(points[0]!));
          if(axis.lengthSq()>largest){largest=axis.lengthSq();normal.copy(axis).normalize();}
        }
        const ranges=meshes.map(({positions})=>{
          const samples=Array.from({length:positions.length/3},(_,i)=>new Vector3().fromArray(positions,3*i).dot(normal));
          return [Math.min(...samples),Math.max(...samples)];
        }).sort((lo,hi)=>lo[0]!-hi[0]!);
        for(const [start,end] of ranges)expect(end!-start!).toBeCloseTo(thickness,5);
        expect(ranges[1]![0]!-ranges[0]![1]!).toBeCloseTo(.016,5);
      }
    });
    it(`keeps the boiler flue spigot open through its support at 1:${scale}`,async()=>{
      const solids=(await modelOf(read("🔌️appliances/gas-boiler"),{scale,view:"forces",poses:[],clips:[]})).solids;
      const spigot=solids.find(p=>p.id==="member-collar-spigot");
      expect(spigot).toBeDefined();
      const mesh=tessellateSolid(spigot!,{chordM:.0001,minSegments:24,maxSegments:48});
      const xyz=Array.from({length:mesh.positions.length/3},(_,i)=>new Vector3().fromArray(mesh.positions,3*i));
      const widths=[0,1,2].map(k=>{const a=xyz.map(p=>p.getComponent(k));return [Math.min(...a),Math.max(...a)];});
      const radial=[0,1,2].filter(i=>widths[i]![1]!-widths[i]![0]!>.071);
      expect(radial).toHaveLength(2);
      const radii=xyz.map(p=>Math.hypot(...radial.map(i=>p.getComponent(i)-(widths[i]![0]!+widths[i]![1]!)/2)));
      expect(Math.min(...radii)).toBeCloseTo(.037,4);
      expect(Math.max(...radii)).toBeCloseTo(.040,4);
    });
    it(`keeps the four material layers and headed ETICS anchors at 1:${scale}`, async () => {
      for (const {asset,thickness} of insulatedWalls) {
        for (const view of ["light-solid","thermal","forces"] as const) {
          const {solids} = await modelOf(asset,{scale,view,poses:[],clips:[]});
          if (view === "forces") {
            const ties=solids.filter(s=>s.id.startsWith("joint-anchor-"));
            expect(ties).toHaveLength(4);
            for(const tie of ties) {
              const head=occupied([tie],1)[0]!;
              expect(head[1]!-head[0]!).toBeCloseTo(.06,4);
            }
          } else {
            for(const [pattern,depth] of [[/plaster/,.015],[/wool-board|insulation/,thickness],[/render/,.02]] as const) {
              const spans=occupied(solids.filter(s=>pattern.test(s.id)),0);
              expect(spans).toHaveLength(1);
              expect(spans[0]![1]!-spans[0]![0]!).toBeCloseTo(depth,5);
            }
          }
        }
      }
    });
    it(`suspends all eighteen blind slats on the lift tapes at 1:${scale}`,async()=>{
      for (const view of ["light-solid","thermal","forces"] as const) {
        const {solids}=await modelOf(blind,{scale,view,poses:[],clips:[]});
        const slats=solids.filter(s=>/^(?:member-|th-)?slat-\d+$/.test(s.id));
        expect(slats).toHaveLength(18);
        for(const slat of slats) {
          const width=occupied([slat],2)[0]!;
          expect(width[1]!-width[0]!).toBeCloseTo(1.13,3);
        }
        if(view==="forces") {
          const tapes=solids.filter(s=>s.id.includes("lift-tape-top"));
          expect(occupied(tapes,2)).toHaveLength(2);
          expect(solids.some(s=>s.id.includes("tape-bearing"))).toBe(true);
        }
      }
    });
    it(`carries the rear collector X on a pair of 2 mm straps at 1:${scale}`,async()=>{
      for (const view of ["light-solid","thermal","forces"] as const) {
        const {solids}=await modelOf(collector,{scale,view,poses:[],clips:[]});
        const straps=solids.filter(s=>s.id.includes("brace-rising")&&!s.id.includes("bolt"));
        expect(straps).toHaveLength(2);
        for(const strap of straps) {
          const width=occupied([strap],0)[0]!;
          expect(width[1]!-width[0]!).toBeCloseTo(.002,4);
          const reach=occupied([strap],2)[0]!;
          expect(reach[1]!-reach[0]!).toBeGreaterThan(.85);
        }
      }
    });
    it(`has an open airflow passage through each generator intake at 1:${scale}`, async () => {
      for (const view of ["light-solid", "thermal", "forces"] as const) {
        const {solids} = await modelOf(generator, {scale, view, poses: [], clips: []});
        const wall = triangles(solids.filter(p => p.node === "container" && !p.tags?.includes("part-mass")));
        for (const sign of [-1, 1]) {
          for (const x of [.52, 3.12]) {
            const ray = (y: number) => new Ray(new Vector3(x, y, 1.3 * sign), new Vector3(0, 0, -sign));
            expect(hit(ray(.65), wall), view + " free air channel").toBe(false);
            expect(hit(ray(.525), wall), view + " frame stock").toBe(true);
          }
        }
      }
    });
    it(`seats the compressor on four pierced isolators and a bored foot plate at 1:${scale}`, async () => {
      for (const view of ["light-solid", "thermal", "forces"] as const) {
        const {solids} = await modelOf(compressor, {scale, view, poses: [], clips: []});
        const mounts = triangles(solids.filter(p => /grommets|contact-pad|^feet$/.test(p.id)));
        const plate = triangles(solids.filter(p => p.id.includes("base-plate") || p.id === "foot-frame"));
        const down = (x: number, z: number) => new Ray(new Vector3(x, .065, z), new Vector3(0,-1,0));
        expect(hit(down(0, 0), mounts), view + " separated feet").toBe(false);
        for (const x of [-.09525, .09525]) for (const z of [-.09525, .09525]) {
          expect(hit(down(x,z), mounts), view + " rubber bore").toBe(false);
          expect(hit(down(x+.012,z), mounts), view + " annular rubber").toBe(true);
          expect(hit(down(x,z), plate), view + " plate bore").toBe(false);
          expect(hit(down(x-Math.sign(x)*.018,z), plate), view + " mounting ear").toBe(true);
        }
      }
    });
    it(`carries an open balcony guard down to four anchors at 1:${scale}`, async () => {
      for (const view of ["light-solid", "thermal", "forces"] as const) {
        const {solids} = await modelOf(balcony, { scale, view, poses: [], clips: [] });
        const rods = solids.filter(s => s.id.includes("balusters"));
        const sections = occupied(rods, 2);
        expect(sections).toHaveLength(8);
        const centers = sections.map(([lo,hi]) => {
          expect(hi! - lo!).toBeCloseTo(.012, 4);
          return Math.abs((hi! + lo!) / 2);
        });
        const [reach] = occupied(rods, 0);
        expect(reach![1]! - reach![0]!).toBeGreaterThan(.86);
        expect(Math.min(...centers)).toBeCloseTo(.12589, 4);
        expect(Math.max(...centers)).toBeCloseTo(.44656, 4);
        if (view === "forces") {
          const anchors = solids.filter(s => s.id.includes("railing-anchors"));
          expect(anchors).toHaveLength(2);
          for(const row of anchors) expect(occupied([row], 2)).toHaveLength(2);
          const lengths = anchors.map(s => {
            const {positions} = tessellateSolid(s, {chordM: .0001, minSegments: 24, maxSegments: 48});
            const x = positions.filter((_, i) => i % 3 === 0);
            return Math.max(...x) - Math.min(...x);
          });
          for (const length of lengths) expect(length).toBeCloseTo(.11, 4);
        }
      }
    });
    it(`retains the two radiator pressure panels and the slotted outlet at 1:${scale}`, async () => {
      for (const view of ["light-solid", "thermal", "forces"] as const) {
        const {solids} = await modelOf(radiator, {scale, view, poses: [], clips: []});
        if (view === "forces") {
          for (const side of ["front","rear"]) {
            const s = solids.find(p => p.id === "member-panel-"+side);
            expect(s).toBeDefined();
            const geo = tessellateSolid(s!, {chordM: .001, minSegments: 12, maxSegments: 32});
            const z = geo.positions.filter((_, i) => i % 3 === 2);
            expect(Math.max(...z)-Math.min(...z)).toBeCloseTo(1, 3);
          }
        } else {
          const plate = solids.find(s => s.id.endsWith("grille-plate"));
          expect(plate).toBeDefined();
          const geo = tessellateSolid(plate!, {chordM: .001, minSegments: 12, maxSegments: 32});
          expect(geo.indices.length / 3).toBeGreaterThan(120);
          const pipes = occupied(solids.filter(p => p.id === "pipe" || p.id === "th-pipe"), 2);
          expect(pipes).toHaveLength(2);
          for (const [lo, hi] of pipes) expect(hi! - lo!).toBeCloseTo(.015, 4);
          expect(pipes[1]![0]! - pipes[0]![0]!).toBeCloseTo(.05, 4);
        }
      }
    });
    it(`carries both rows of twelve tubes and an open fin stack at 1:${scale}`, async () => {
      for (const view of ["light-solid", "thermal", "forces"] as const) {
        const {solids} = await modelOf(exchanger, {scale, view, poses: [], clips: []});
        for (const row of ["entering","leaving"]) {
          const tubes = solids.filter(s => s.id.includes("tube-row-" + row));
          expect(occupied(tubes, 0)).toHaveLength(12);
        }
        const fins = solids.filter(s => s.id.includes(view === "forces" ? "member-fin-plates" : "fin-pack") && !s.tags?.includes("part-mass"));
        const plates = occupied(fins, 2);
        expect(plates).toHaveLength(view === "forces" ? ({50:5,100:4,200:3,500:2})[scale] : 134);
        for (const [lo,hi] of plates) expect(hi! - lo!).toBeCloseTo(.0002, 5);
        if (view !== "forces") {
          for (let i = 1; i < plates.length; i++) expect(plates[i]![0]! - plates[i-1]![1]!).toBeCloseTo(.0028, 5);
        }

      }
    });
    it(`keeps the three fan blades, intake louvres and motor support at 1:${scale}`, async () => {
      for (const view of ["light-solid", "thermal", "forces"] as const) {
        const {solids} = await modelOf(pump, {scale, view, poses: [], clips: []});
        if (view === "forces") {
          expect(solids.some(s => s.id.includes("motor-bracket"))).toBe(true);
          expect(solids.some(s => s.id === "mounted-fan-motor")).toBe(true);
          expect(solids.some(s => s.id.includes("compressor"))).toBe(true);
        } else {
          const blades = solids.filter(s => s.id.includes("fan-blades"));
          expect(blades).toHaveLength(1);
          const louvres = solids.filter(s => s.id.includes("side-louvres"));
          expect(occupied(louvres, 1)).toHaveLength(25);
          const intakeDepth = occupied(louvres, 2).at(-1)![1]!;
          const coilDepth = occupied(solids.filter(s => s.id === "coil" || s.id === "th-coil"), 2).at(-1)![1]!;
          expect(intakeDepth - coilDepth).toBeGreaterThan(.02);
          for (const blade of blades) {
            const geo = tessellateSolid(blade, {chordM: .001, minSegments: 12, maxSegments: 32});
            const sizes = Array.from({length: geo.positions.length/3}, (_, i) => Math.hypot(geo.positions[3*i+1]! - .8, geo.positions[3*i+2]! - .15));
            expect(Math.max(...sizes)).toBeGreaterThan(.26);
            const thirds = new Set<number>();
            for(let i=0;i<sizes.length;i++) if(sizes[i]!>.26) {
              const yaw=Math.atan2(geo.positions[3*i+2]!-.15,geo.positions[3*i+1]!-.8);
              thirds.add((Math.round(yaw * 3 / (2*Math.PI))+3)%3);
            }
            expect(thirds.size).toBe(3);
          }
        }
      }
    });
  }
});
