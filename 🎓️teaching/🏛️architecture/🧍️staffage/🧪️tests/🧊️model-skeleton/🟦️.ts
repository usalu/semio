/** 🧊️ The rig as the neutral node tree: the nodes and the mechanism joints of the rig, every pose and every clip sampled through `evaluate` and replayed by the kernel against the evaluated bone frames, and the FBX of a clip replayed by three's `FBXLoader` and `AnimationMixer`.
 *
 * THE REFERENCES are `evaluate` (the single source of the bone frames, composed independently here from `(ahead, up, left)` to `(left, up, ahead)` with three.js matrices) and, for the FBX, three.js loading and animating the bytes.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧫️fixtures/🧊️model-skeleton/🔣️.json — the vectors
 */
import { AnimationMixer, Matrix4, Ray, Vector3 } from "three";
import { describe, expect, it } from "vitest";
import { poseModel, type ModelDocument } from "../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import type { Asset, VolumeSolid } from "../../🧬️schema/🟦️.ts";
import { VOLUME_LIMITS, featureOf, isSheet, sortedExtents } from "../../🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { writeFbx } from "../../🔨️modules/📤️export/🎬️fbx/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { frameTimes, modelOf } from "../../🔨️modules/🧊️volume/🟦️.ts";
import { catalogAsset } from "../../🧫️fixtures/🌡️thermal/🟦️.ts";
import { fixture } from "../🧰️support/🧰️.ts";
import { loadFbx, sampleFigure } from "../🧰️support/🧊️.ts";

type Joint = Record<string, unknown>;
type Vectors = {
  doeSuspension: { asset: string; massKg: number; skinAreaM2: number; poses: Record<string,number> };
  adultLongBones: { subjects: string[]; pairedShafts: string[]; scales: number[]; toleranceM: number };
  roadWheels: Record<string,{slots: number; style: string; diameterM: number}>;
  scissorBearings: { asset: string; stationsM: number[]; pinRadiusM: number; boreRadiusM: number; sleeveRadiusM: number };
  crawler: { asset: string; rollersPerSide: number; idlerClearanceM: number };
  industrialTyres: Record<string, number>;
  cycleWheels: Record<string, {spokes: number; nippleM: number}>;
  scooterCartridges: {asset: string; boreM: number; outsideM: number; widthM: number; windows: number};
  squirrelBrush: { asset: string; segments: number; lengthM: number; massKg: number; centra: number[]; guardLocks: number[] };
  craneCarriage: { asset: string; treadRadiusM: number; flangeRadiusM: number; grooveDepthM: number; wheelbaseM: number };
  skateboard: { asset: string; deckGaugeM: number; wheelRadiusM: number; trackM: number };
  canineLumbar: { asset: string; levels: number; canalHeightM: number; centrumHeightM: number };
  goatBarrel: { asset: string; parts: string[]; joints: string[]; jointToleranceM: number };
  hedgeClipping: { asset: string; plants: number; blades: number[]; crosswindTipM: number };
  gullWing: { asset: string; primaries: number; secondaries: number; minimumSpanM: number; maximumSpanM: number; foldDeg: number; glideDeg: number };
  passengers: { asset: string; root: string; neck: string; head: string; outlets: [string, string][]; segments: string[] }[];
  joints: Record<string, Joint>;
  variants: { name: string; pose: string; poseBones: Record<string, unknown>; jointPatch: Record<string, Record<string, unknown>> | null; bone: string; expected: Joint }[];
  clips: Record<string, { fps: number; durationS: number; keys: number }>;
  tolerances: { poseWorldM: number; clipKeyWorldM: number; clipBetweenKeysM: number; clipBetweenKeysRad: number };
};
const vectors = fixture<Vectors>("🧊️model-skeleton");
const OPTIONS = { scale: 50, view: "light-solid" } as const;
const near = (a: readonly number[], b: readonly number[], eps: number, what = ""): void => void a.forEach((v, i) => expect(Math.abs(v - b[i]!), `${what} ${a} vs ${b}`).toBeLessThanOrEqual(eps));

/** 🔬️ The authored 1:50 volume of an asset, the scale at which every pinned part is measured. */
const fineOf = (asset: Asset) => {
  const volume = asset.volumes?.["50"];
  if (volume?.status !== "authored") throw new Error(asset.id);
  return volume;
};

/** 📏️ Whether `volumes-feature` admits a part, measured on its 1:50 solid, at a scale: always at 1:50, coarser when it is at least one smallest feature of that scale in every dimension, a thin sheet, a texture or a thin role, or long enough over its largest cross extent to be declared `slender` (a structure `member` from the member ratio and two features of length). A part that is missing at 1:50 is demanded everywhere.
 * @see ../../🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts — `featureOf`, `isSheet`, `sortedExtents`, `VOLUME_LIMITS` */
const admits = (part: object | undefined, scale: string, member = false): boolean => {
  if (scale === "50" || part === undefined || !("kind" in part)) return true;
  const solid = part as VolumeSolid, feature = featureOf(Number(scale)), [thin, wide, long] = sortedExtents(solid);
  if (thin >= feature - 1e-9 || solid.texture === true || (VOLUME_LIMITS.thinRoles as readonly string[]).includes(solid.role) || isSheet(solid, feature)) return true;
  const bar = member && long >= VOLUME_LIMITS.slenderMemberFeatures * feature - 1e-9;
  return long >= (bar ? VOLUME_LIMITS.slenderMemberRatio : VOLUME_LIMITS.slenderRatio) * wide - 1e-9;
};

/** 🦴️ Whether a bone still has a `member` solid in a structure list: its own, or one that spans it and names it in `members`. */
const carried = (structure: readonly object[], bone: string): boolean => structure.some((entry) => {
  const solid = entry as { part?: string; member?: string; members?: readonly string[] };
  return solid.part === "member" && (solid.member === bone || (solid.members ?? []).includes(bone));
});

const variantOf = (v: Vectors["variants"][number]): Asset => {
  const asset = structuredClone(sampleFigure()) as unknown as { rig: { bones: { id: string; joint: unknown }[] }; poses: { id: string; bones: Record<string, unknown> }[] };
  for (const [bone, joint] of Object.entries(v.jointPatch ?? {})) asset.rig.bones.find((b) => b.id === bone)!.joint = joint;
  const pose = asset.poses.find((p) => p.id === v.pose)!;
  pose.bones = { ...pose.bones, ...v.poseBones };
  return asset as unknown as Asset;
};

/** 🧮️ The neutral world matrix of an evaluated bone, composed here independently: columns (u, v, -w) mapped (x, y, z) -> (z, y, x). */
const expectedWorld = (frame: { o: { x: number; y: number; z: number }; u: { x: number; y: number; z: number }; v: { x: number; y: number; z: number }; w: { x: number; y: number; z: number } }): Matrix4 =>
  new Matrix4().makeBasis(new Vector3(frame.u.z, frame.u.y, frame.u.x), new Vector3(frame.v.z, frame.v.y, frame.v.x), new Vector3(-frame.w.z, -frame.w.y, -frame.w.x)).setPosition(frame.o.z, frame.o.y, frame.o.x);

const worldOf = (document: ModelDocument, pose: Parameters<typeof poseModel>[1]) => poseModel(document, pose);

describe("🧊️ the rig as the neutral node tree", () => {
  it("suspends the doe’s paired glands and exposes the bare skin at every scale whose feature admits them", () => {
    const f = vectors.doeSuspension, asset = catalogAsset(f.asset), id = "mammary-suspension";
    const suspension = asset.rig.bones.find(b => b.id === id)!;
    expect(suspension.parent).toBe("pelvis");
    expect(suspension.mass!.kg).toBe(f.massKg);
    const skin = asset.thermal.regions.find(r => r.id === "mammary-skin")!;
    expect(skin.areaM2).toBe(f.skinAreaM2);
    expect(skin.layers).toHaveLength(0);
    for (const [pose, degrees] of Object.entries(f.poses)) {
      const {u} = evaluate(asset, {kind:"pose",pose}).bones[id]!.frame;
      expect(new Vector3(u.x,u.y,u.z).angleTo(new Vector3(0,-1,0))*180/Math.PI).toBeCloseTo(degrees,2);
    }
    const high = fineOf(asset);
    for (const [scale, vol] of Object.entries(asset.volumes ?? {})) {
      if (vol.status !== "authored") throw new Error(asset.id);
      for (const part of ["udder-near","udder-far","teat-near","teat-far"]) {
        if (!admits(high.body.find(s => s.id === part), scale)) continue;
        const body = vol.body.find(s => s.id === part);
        expect(body?.bone, scale+" "+part).toBe(id);
        expect(body?.material).toBe("bare-skin");
        expect(vol.thermal.some(s => "of" in s && s.of === part && s.region === skin.id)).toBe(true);
      }
      const glands = vol.body.filter(s => s.bone === id);
      expect(glands.length, scale).toBeGreaterThan(0);
      for (const gland of glands) {
        expect(gland.material).toBe("bare-skin");
        expect(vol.thermal.some(s => "of" in s && s.of === gland.id && s.region === skin.id), scale+" "+gland.id).toBe(true);
      }
    }
  });

  it("preserves each adult's two forearm bones and measured shaft radii at every scale", () => {
    const f = vectors.adultLongBones;
    for (const name of f.subjects) {
      const asset = catalogAsset(name), high = asset.volumes?.["50"];
      if (high?.status !== "authored") throw new Error(name);
      const section = (s: typeof high.structure[number] | undefined) => {
        if (s && "kind" in s && s.kind === "capsule") return new Vector3(s.r0,s.r1,0);
        if (s && "kind" in s && s.kind === "loft") {
          const narrow = s.sections.reduce((a,b) => a.rx*a.ry<b.rx*b.ry?a:b);
          return new Vector3(narrow.rx,narrow.ry,0);
        }
        throw new Error("Missing anatomical shaft on "+name);
      };
      for (const v of Object.values(asset.volumes ?? {})) {
        if (v.status !== "authored") throw new Error(name);
        for (const shaft of f.pairedShafts) for (const side of ["l","r"]) {
          const id = shaft+"-"+side;
          expect(section(v.structure.find(s=>s.id===id)).distanceTo(section(high.structure.find(s=>s.id===id))),name+" "+id).toBeLessThan(f.toleranceM);
        }
      }
    }
  });

  it("keeps four distinct wheel centres coaxial with the tyres and binds metal and rubber separately at every scale whose feature admits them", () => {
    for (const [name,f] of Object.entries(vectors.roadWheels)) {
      const asset = catalogAsset(name), high = fineOf(asset);
      for (const [scale,v] of Object.entries(asset.volumes ?? {})) {
        if (v.status !== "authored") throw new Error(name);
        for (const corner of ["fl","fr","rl","rr"]) {
          const id = (part:string) => f.style==="spoked"?part+"-"+corner:corner+"-"+part;
          const kept = (part:string) => admits(high.body.find(q=>q.id===id(part)), scale);
          const tyre = v.body.find(q=>q.id===id("tyre"));
          if (tyre?.kind!=="revolve") throw new Error(name+" "+corner+" 1:"+scale);
          expect(Math.max(...tyre.profile.map(p=>p[1]))*2).toBeCloseTo(f.diameterM,4);
          expect(Math.min(...tyre.profile.map(p=>p[1]))).toBeGreaterThan(.16);
          expect(v.thermal.some(s=>"of" in s && s.of===tyre.id && s.region==="tyres")).toBe(true);
          const metal = ["rim","hub"].filter(kept).map(id);
          if (kept("rim")) {
            const rim = v.body.find(q=>q.id===id("rim"));
            if (rim?.kind!=="revolve") throw new Error(name+" "+corner+" 1:"+scale);
            expect(new Vector3(...tyre.origin!).distanceTo(new Vector3(...rim.origin!))).toBeLessThan(.00001);
          }
          const web = f.style==="spoked"?"spokes":f.style==="cast"?"spoke":"disc-wheel";
          if (!kept(web)) {
            for (const part of metal) {
              expect(v.structure.some(s=>"of" in s && s.of===part), part+" 1:"+scale).toBe(true);
              expect(v.thermal.some(s=>"of" in s && s.of===part && s.region==="wheel-metal"), part+" 1:"+scale).toBe(true);
            }
            continue;
          }
          const centre = v.body.find(q=>q.id===id(web));
          if (!centre) throw new Error("Missing wheel centre "+name+" "+corner+" 1:"+scale);
          if (f.style==="pressed") {
            if (centre.kind!=="polyhedron") throw new Error("Missing pressed web");
            const vertices = centre.vertices.map(p=>new Vector3(...p));
            const hits = (r:number,angle:number) => {
              const ray = new Ray(new Vector3(r*Math.cos(angle),r*Math.sin(angle),-2),new Vector3(0,0,1));
              return centre.triangles.some(([a,b,c])=>ray.intersectTriangle(vertices[a]!,vertices[b]!,vertices[c]!,false,new Vector3())!==null);
            };
            for (let i=0;i<f.slots;i++) {
              expect(hits(.124,2*Math.PI*i/f.slots)).toBe(false);
              expect(hits(.124,2*Math.PI*(i+.5)/f.slots)).toBe(true);
            }
          } else expect(centre.repeat?.count).toBe(f.slots);
          for (const part of [centre.id,...metal]) {
            expect(v.structure.some(s=>"of" in s && s.of===part), part+" 1:"+scale).toBe(true);
            expect(v.thermal.some(s=>"of" in s && s.of===part && s.region==="wheel-metal"), part+" 1:"+scale).toBe(true);
          }
        }
      }
    }
  });

  it("keeps the squirrel's tapered caudal chain and articulated brush together at every scale whose feature admits the centra", () => {
    const f = vectors.squirrelBrush, asset = catalogAsset(f.asset), high = fineOf(asset);
    const bones = asset.rig.bones.filter(b => /^tail-\d+$/.test(b.id));
    expect(bones).toHaveLength(f.segments);
    expect(bones.reduce((sum,b) => sum+b.lengthM,0)).toBeCloseTo(f.lengthM,6);
    expect(bones.reduce((sum,b) => sum+b.mass!.kg,0)).toBeCloseTo(f.massKg,6);
    const v = (point: {x:number;y:number;z:number}) => new Vector3(point.x,point.y,point.z);
    for (const pose of asset.poses) {
      const state = evaluate(asset,{kind:"pose",pose:pose.id});
      for (const [i,bone] of bones.slice(0,-1).entries()) {
        const parent = state.bones[bone.id]!.frame, next = bones[i+1]!;
        expect(next.parent).toBe(bone.id);
        expect(v(parent.o).addScaledVector(v(parent.u),bone.lengthM).distanceTo(v(state.bones[next.id]!.frame.o))).toBeLessThan(.00001);
      }
    }
    for (const [i,scale] of (["50","100","200","500"] as const).entries()) {
      const vol = asset.volumes?.[scale];
      if (vol?.status !== "authored") throw new Error(scale);
      const locks = vol.body.filter(s => s.id.startsWith("brush-lock-"));
      expect(locks).toHaveLength(f.guardLocks[i]!);
      for (const lock of locks) expect(vol.thermal.some(s => "of" in s && s.of === lock.id && s.region === "tail-fur")).toBe(true);
      for (const [j,bone] of bones.entries()) {
        const centra = (list: typeof vol.structure) => list.filter(s => s.member === bone.id && s.id.startsWith("caudal-centrum-"));
        if (centra(high.structure).every(s => admits(s, scale, true))) expect(centra(vol.structure), scale+" "+bone.id).toHaveLength(f.centra[j]!);
        expect(carried(vol.structure, bone.id), scale+" "+bone.id).toBe(true);
      }
    }
  });

  it("laces both bicycle wheels through open nipple seats", () => {
    for (const [name, spec] of Object.entries(vectors.cycleWheels)) {
      for (const volume of Object.values(catalogAsset(name).volumes)) {
        if (volume.status !== "authored") throw new Error("Missing bicycle");
        for (const end of ["f", "r"]) {
          const wheel = volume.body.filter(s => s.bone === "wheel-" + end);
          const spokes = wheel.filter(s => s.id.startsWith("spoke-"));
          expect(spokes.reduce((n, s) => n + (s.repeat?.count ?? 1), 0)).toBe(spec.spokes);
          for (const spoke of spokes) {
            const nipple = wheel.find(s => s.id === spoke.id.replace("spoke-", "nipple-"));
            if (nipple?.kind !== "polyhedron") throw new Error("Missing nipple");
            const vertices = nipple.vertices.map(p => new Vector3(...p));
            const center = (points: Vector3[]) => points.reduce((sum, p) => sum.add(p), new Vector3()).divideScalar(points.length);
            const start = center(vertices.slice(0,8)), end = center(vertices.slice(-8));
            const axis = end.clone().sub(start).normalize();
            const ray = new Ray(start.clone().addScaledVector(axis,-.01),axis);
            expect(start.distanceTo(end)).toBeCloseTo(spec.nippleM,5);
            expect(nipple.triangles.some(([i,j,k]) => ray.intersectTriangle(vertices[i]!,vertices[j]!,vertices[k]!,false,new Vector3()))).toBe(false);
          }
        }
      }
    }
  });

  it("seats two annular cartridges in the electric scooter rear hub at every scale whose feature admits them", () => {
    const f = vectors.scooterCartridges, asset = catalogAsset(f.asset), high = fineOf(asset);
    for (const [scale, volume] of Object.entries(asset.volumes ?? {})) {
      if (volume.status !== "authored") throw new Error("Missing scooter");
      expect(volume.body.some(s => s.bone === "wheel-r"), scale).toBe(true);
      const rim = volume.body.find(s => s.id === "web-r");
      if (admits(high.body.find(s => s.id === "web-r"), scale)) expect(rim?.kind === "extrude" && rim.holes?.length, scale).toBe(f.windows);
      if (!high.body.filter(s => s.id.startsWith("bearing-r-")).every(s => admits(s, scale))) continue;
      const bearings = volume.body.filter(s => s.id.startsWith("bearing-r-"));
      expect(bearings, scale).toHaveLength(2);
      for (const bearing of bearings) {
        if (bearing.kind !== "revolve") throw new Error("Missing annular bearing");
        const radii = bearing.profile.map(p => p[1]);
        expect(2 * Math.min(...radii)).toBeCloseTo(f.boreM,7);
        expect(2 * Math.max(...radii)).toBeCloseTo(f.outsideM,7);
        expect(Math.max(...bearing.profile.map(p => p[0]))).toBeCloseTo(f.widthM,7);
      }
    }
  });

  it("bores every scissor arm for its cross pins at every scale whose feature admits the arm", () => {
    const f = vectors.scissorBearings, asset = catalogAsset(f.asset), high = fineOf(asset);
    for (const [scale, volume] of Object.entries(asset.volumes ?? {})) {
      if (volume.status !== "authored") throw new Error("Missing scissor volume");
      for (const side of ["a","b"]) for (const index of [1,2,3,4]) {
        expect(volume.body.some(s => s.bone === side+index), scale+" "+side+index).toBe(true);
        if (!admits(high.body.find(s => s.id === `arm-${side}${index}`), scale)) continue;
        const arm = volume.body.find(s => s.id === `arm-${side}${index}`);
        if (arm?.kind !== "polyhedron") throw new Error("Missing bored link");
        const vertices = arm.vertices.map(p => new Vector3(...p));
        for (const [station, x] of f.stationsM.entries()) {
          const rayHits = (y: number) => {
            const ray = new Ray(new Vector3(x,y,-1), new Vector3(0,0,1));
            return arm.triangles.some(([i,j,k]) => ray.intersectTriangle(vertices[i]!,vertices[j]!,vertices[k]!,false,new Vector3()) !== null);
          };
          expect(rayHits(f.boreRadiusM)).toBe(false);
          expect(rayHits(f.sleeveRadiusM + 0.0025)).toBe(true);
          if (!admits(high.body.find(s => s.id === `sleeve-${side}${index}-${station}`), scale)) continue;
          const sleeve = volume.body.find(s => s.id === `sleeve-${side}${index}-${station}`);
          if (sleeve?.kind !== "revolve") throw new Error("Missing sleeve");
          expect(Math.min(...sleeve.profile.map(p => p[1]))).toBeCloseTo(f.boreRadiusM,6);
          expect(f.boreRadiusM).toBeGreaterThan(f.pinRadiusM);
        }
      }
    }
  });

  it("passes each idler pin through its yoke and seats eight rollers per track at every scale whose feature admits them", () => {
    const f = vectors.crawler, asset = catalogAsset(f.asset), high = fineOf(asset);
    const kept = (scale: string, prefix: string) => high.body.filter(s => s.id.startsWith(prefix)).every(s => admits(s, scale));
    for (const [scale, volume] of Object.entries(asset.volumes ?? {})) {
      if (volume.status !== "authored") throw new Error("Missing crawler volume");
      const cheeks = volume.body.filter(s => s.id.startsWith("idler-yoke-"));
      expect(cheeks, scale).toHaveLength(2);
      if (kept(scale, "idler-cross-pin")) {
        const pin = volume.body.find(s => s.id === "idler-cross-pin");
        if (pin?.kind !== "revolve") throw new Error("Missing idler cross pin 1:"+scale);
        if (kept(scale, "idler-yoke-")) for (const cheek of cheeks) {
          if (cheek.kind !== "extrude" || !cheek.holes?.[0]) throw new Error("Missing yoke bore 1:"+scale);
          for (const [x,y] of cheek.holes[0]) expect(Math.hypot(x-pin.origin[0],y-pin.origin[1])-pin.profile[0]![1], scale).toBeCloseTo(f.idlerClearanceM,4);
        }
      }
      for (const side of ["l","r"]) {
        if (!kept(scale, "roller-"+side)) continue;
        const roller = volume.body.find(s => s.id === "roller-"+side);
        expect(roller?.repeat?.count, scale).toBe(f.rollersPerSide);
        expect(volume.structure.some(s => "of" in s && s.of === roller?.id)).toBe(true);
      }
    }
  });

  it("carries the industrial tyres in the temperature and structural scenes", () => {
    for (const [name,count] of Object.entries(vectors.industrialTyres)) for (const volume of Object.values(catalogAsset(name).volumes)) {
      if (volume.status !== "authored") throw new Error("Missing wheel volume");
      const tyres = volume.body.filter(s => s.id.startsWith("tyre-") && s.bone.startsWith("wheel-"));
      expect(tyres).toHaveLength(count);
      for (const tyre of tyres) {
        expect(volume.structure.some(s => "of" in s && s.of === tyre.id)).toBe(true);
        expect(volume.thermal.some(s => "of" in s && s.of === tyre.id && s.region === "tyres")).toBe(true);
      }
    }
  });

  it("runs four flanged trolley wheels on the crane rails and keeps two reeving grooves open at every scale whose feature admits them", () => {
    const f = vectors.craneCarriage, asset = catalogAsset(f.asset), high = fineOf(asset);
    const joint = asset.rig.bones.find(b => b.id === "trolley")!;
    for (const [scale, vol] of Object.entries(asset.volumes ?? {})) {
      if (vol.status !== "authored") throw new Error(asset.id);
      for (const side of ["r","l"]) {
        const rail = vol.body.find(s => s.id === "jib-rail-"+side);
        if (rail?.kind !== "box") throw new Error("Missing jib rail");
        const railTop = rail.transform!.translation![1]+rail.size[1]/2;
        expect(vol.body.some(s => s.bone === "trolley"), scale).toBe(true);
        if (![1,2].every(i => admits(high.body.find(s => s.id === "trolley-wheel-"+side+"-"+i), scale))) continue;
        const points = [1,2].map(i => {
          const wheel = vol.body.find(s => s.id === "trolley-wheel-"+side+"-"+i);
          if (wheel?.kind !== "revolve") throw new Error("Missing trolley wheel");
          const origin = new Vector3(...wheel.origin);
          expect(origin.y+joint.y-f.treadRadiusM).toBeCloseTo(railTop,4);
          expect(Math.max(...wheel.profile.map(p => p[1]))).toBeCloseTo(f.flangeRadiusM,4);
          expect(vol.structure.some(q => "of" in q && q.of === wheel.id && q.part === "joint")).toBe(true);
          expect(vol.thermal.some(q => "of" in q && q.of === wheel.id && q.region === "trolley")).toBe(true);
          return origin;
        });
        expect(points[0]!.distanceTo(points[1]!)).toBeCloseTo(f.wheelbaseM,4);
      }
      for (const i of [1,2]) {
        if (!admits(high.body.find(s => s.id === "trolley-sheave-"+i), scale)) continue;
        const sheave = vol.body.find(s => s.id === "trolley-sheave-"+i);
        if (sheave?.kind !== "revolve") throw new Error("Missing trolley sheave");
        const centre = sheave.profile.find(p => Math.abs(p[0]-.03)<1e-5)!;
        expect(Math.max(...sheave.profile.map(p => p[1]))-centre[1]).toBeCloseTo(f.grooveDepthM,4);
      }
    }
  });

  it("carries the same maple deck stock and four ground-contact wheels at every scale whose feature admits them", () => {
    const f = vectors.skateboard, asset = catalogAsset(f.asset), high = fineOf(asset);
    for (const [scale, vol] of Object.entries(asset.volumes ?? {})) {
      if (vol.status !== "authored") throw new Error(asset.id);
      expect(vol.thermal.some(s => s.region === "deck"), scale).toBe(true);
      expect(vol.thermal.some(s => s.region === "wheels"), scale).toBe(true);
      for (const side of ["l","r"]) {
        if (!admits(high.body.find(s => s.id === "deck-"+side), scale)) continue;
        const deck = vol.body.find(s => s.id === "deck-"+side);
        if (deck?.kind !== "loft") throw new Error("Missing concave deck half");
        for (const section of deck.sections) expect(section.ry*2).toBeCloseTo(f.deckGaugeM,5);
      }
      for (const end of ["f","r"]) {
        if (!["l","r"].every(side => admits(high.body.find(q => q.id === "wheel-"+end+"-"+side), scale))) continue;
        const centres = ["l","r"].map(side => {
          const wheel = vol.body.find(q => q.id === "wheel-"+end+"-"+side);
          if (wheel?.kind !== "revolve") throw new Error("Missing road wheel");
          expect(wheel.bone).toBe("wheel-"+end);
          expect(Math.max(...wheel.profile.map(p => p[1]))).toBeCloseTo(f.wheelRadiusM,5);
          expect(vol.thermal.some(s => "of" in s && s.of === wheel.id && s.region === "wheels")).toBe(true);
          expect(vol.structure.some(s => "of" in s && s.of === wheel.id && s.member === wheel.bone)).toBe(true);
          return new Vector3(...wheel.origin);
        });
        expect(centres[0]!.distanceTo(centres[1]!)).toBeCloseTo(f.trackM,4);
      }
    }
  });

  it("keeps a patent spinal canal through seven distinct canine lumbar centra at every scale whose feature admits them", () => {
    const f = vectors.canineLumbar, asset = catalogAsset(f.asset), high = fineOf(asset);
    const levels = Array.from({length:f.levels}, (_,i) => high.structure.find(s => s.id === "lumbar-l"+(i+1)));
    for (const [scale, vol] of Object.entries(asset.volumes ?? {})) {
      if (vol.status !== "authored") throw new Error(asset.id);
      expect(carried(vol.structure, "lumbar"), scale).toBe(true);
      if (!levels.every(level => admits(level, scale, true))) continue;
      let previous = Infinity;
      for (let level=1; level<=f.levels; level++) {
        const body = vol.structure.find(s => s.id === "lumbar-l"+level);
        if (!body || !("kind" in body) || body.kind !== "polyhedron") throw new Error("Missing lumbar vertebra L"+level);
        expect(body.bone).toBe("lumbar");
        const points = body.vertices.map(p => new Vector3(...p));
        const center = points.reduce((x,p) => x+p.x,0)/points.length;
        expect(center).toBeLessThan(previous);
        previous = center;
        const hits = (y:number) => {
          const ray = new Ray(new Vector3(-1,y,0),new Vector3(1,0,0)), point = new Vector3();
          return body.triangles.some(([a,b,c]) => ray.intersectTriangle(points[a]!,points[b]!,points[c]!,false,point) !== null);
        };
        expect(hits(f.canalHeightM)).toBe(false);
        expect(hits(f.centrumHeightM)).toBe(true);
      }
    }
  });

  it("keeps both sections of each doe torso junction coincident through every named pose", () => {
    const f = vectors.goatBarrel, asset = catalogAsset(f.asset);
    for (const pose of asset.poses) {
      const state = evaluate(asset,{kind:"pose",pose:pose.id});
      for (const vol of Object.values(asset.volumes ?? {})) {
        if (vol.status !== "authored") throw new Error(pose.id);
        const ends = f.parts.map((id,i) => {
          const segment = vol.body.find(q => q.id === id);
          if (segment?.kind !== "loft") throw new Error(id);
          expect(segment.bone).toBe(f.joints[i]);
          const frame = state.bones[segment.bone]!.frame;
          const v = (p: {x:number;y:number;z:number}) => new Vector3(p.x,p.y,p.z);
          const world = (point: readonly number[]) => v(frame.o).addScaledVector(v(frame.u),point[0]!).addScaledVector(v(frame.v),point[1]!).addScaledVector(v(frame.w),point[2]!);
          const start = segment.sections[0]!, end = segment.sections[segment.sections.length-1]!;
          return { start:world(start.at), end:world(end.at), first:[start.rx,start.ry], last:[end.rx,end.ry] };
        });
        for (let i=1; i<ends.length; i++) {
          expect(ends[i-1]!.end.distanceTo(ends[i]!.start)).toBeLessThan(f.jointToleranceM);
          expect(ends[i-1]!.last).toEqual(ends[i]!.first);
        }
      }
    }
  });

  it("keeps foliage on each of six moving hornbeam crowns at every representation scale", () => {
    const f = vectors.hedgeClipping, asset = catalogAsset(f.asset);
    const calm = evaluate(asset,{kind:"pose",pose:"calm"}), windy = evaluate(asset,{kind:"pose",pose:"windy"});
    for (const [index,scale] of (["50","100","200","500"] as const).entries()) {
      const vol = asset.volumes?.[scale];
      if (vol?.status !== "authored") throw new Error(scale);
      const leaves = vol.body.filter(q => q.id.startsWith("leaf-spray-") && !q.id.endsWith("-shoot"));
      expect(leaves.length).toBe(f.blades[index]);
      const exposed = new Set(vol.thermal.flatMap(q => "of" in q && q.region === "leaf-face" ? [q.of] : []));
      for (const leaf of leaves) expect(exposed.has(leaf.id)).toBe(true);
      for (let i=1; i<=f.plants; i++) {
        const id = "block-"+i, bone = asset.rig.bones.find(q => q.id === id)!;
        expect(leaves.some(leaf => leaf.bone === id)).toBe(true);
        const tip = (state: typeof calm) => {
          const {o,u} = state.bones[id]!.frame;
          return new Vector3(o.x,o.y,o.z).addScaledVector(new Vector3(u.x,u.y,u.z),bone.lengthM);
        };
        const delta = tip(windy).sub(tip(calm));
        expect(Math.abs(delta.z)).toBeGreaterThan(Math.max(f.crosswindTipM,Math.abs(delta.x)));
      }
    }
  });

  it("folds and opens the gull’s remiges from their anatomical attachments at all four scales and carries each vane at every scale whose feature admits it", () => {
    const f = vectors.gullWing, asset = catalogAsset(f.asset), high = fineOf(asset);
    const bones = new Map(asset.rig.bones.map(b => [b.id,b]));
    const vector = (p: { x: number; y: number; z: number }) => new Vector3(p.x,p.y,p.z);
    for (const [pose, degrees] of [["standing",f.foldDeg],["gliding",f.glideDeg]] as const) {
      const state = evaluate(asset, { kind:"pose", pose }), tips: number[] = [];
      for (const side of ["n","f"]) {
        for (const [series,count,parent] of [["p",f.primaries,"hand"],["s",f.secondaries,"forearm"]] as const) {
          for (let i = 1; i <= count; i++) {
            const id = series+i+"-"+side;
            expect(bones.get(id)?.parent).toBe(parent+"-"+side);
            for (const [scale, vol] of Object.entries(asset.volumes ?? {})) {
              if (vol.status !== "authored") throw new Error(id);
              expect(vol.structure.some(q => q.member === id), id).toBe(true);
              if (!admits(high.body.find(q => q.bone === id), scale)) continue;
              const solid = vol.body.find(q => q.bone === id);
              expect(solid, id+" 1:"+scale).toBeDefined();
              expect(vol.thermal.some(q => "of" in q && q.of === solid!.id), id).toBe(true);
            }
          }
        }
        const axis = (i: number) => vector(state.bones["p"+i+"-"+side]!.frame.u);
        expect(axis(1).angleTo(axis(f.primaries))*180/Math.PI).toBeCloseTo(degrees,5);
        const id = "p9-"+side, frame = state.bones[id]!.frame;
        tips.push(vector(frame.o).addScaledVector(vector(frame.u),bones.get(id)!.lengthM).z);
      }
      if (pose === "gliding") {
        const span = Math.abs(tips[1]!-tips[0]!);
        expect(span).toBeGreaterThan(f.minimumSpanM);
        expect(span).toBeLessThan(f.maximumSpanM);
      }
    }
  });

  it.each(vectors.passengers)("keeps the $asset passenger’s joints attached at every scale whose feature admits them and the respiratory landmarks at all scales", (p) => {
    const asset = catalogAsset(p.asset), high = fineOf(asset);
    const parentOf = (id: string) => asset.rig.bones.find(b => b.id === id)?.parent;
    expect(parentOf(p.head)).toBe(p.neck);
    expect(parentOf(p.neck)).toBe(p.root);
    for (const [scale, volume] of Object.entries(asset.volumes ?? {})) {
      expect(volume.status).toBe("authored");
      if (volume.status !== "authored") throw new Error(p.asset);
      for (const id of [p.neck, p.head, ...p.segments]) {
        const flesh = high.body.filter(s => s.bone === id), bones = high.structure.filter(s => s.member === id);
        if (flesh.length === 0 || flesh.some(s => admits(s, scale))) expect(volume.body.some(s => s.bone === id), id+" 1:"+scale).toBe(true);
        if (bones.length === 0 || bones.some(s => admits(s, scale, s.part === "member"))) expect(volume.structure.some(s => s.member === id), id+" 1:"+scale).toBe(true);
      }
      for (const [landmark, id] of p.outlets) {
        const anchor = asset.anchors.find(q => q.id === landmark)!;
        const outlet = volume.thermal.find(q => q.id === id);
        expect(anchor.bone).toBe(p.head);
        if (!outlet || !("bone" in outlet) || !outlet.transform?.translation) throw new Error(id);
        expect(outlet.bone).toBe(p.head);
        const distance = new Vector3(...outlet.transform.translation).distanceTo(new Vector3(anchor.x, anchor.y, anchor.z ?? 0));
        expect(distance, id).toBeLessThan(0.001);
      }
    }
  });

  it("makes one node per bone, parents first, with the hand-computed joints and a free root", async () => {
    const asset = sampleFigure();
    const document = await modelOf(asset, OPTIONS);
    const bones = document.nodes.filter((n) => n.tags?.includes("bone"));
    expect(bones.map((n) => n.id).sort()).toEqual(asset.rig.bones.map((b) => b.id).sort());
    for (const bone of asset.rig.bones) {
      const node = bones.find((n) => n.id === bone.id)!;
      expect(node.parent).toBe(bone.parent);
      expect(node.joint?.type).toBe(vectors.joints[bone.id]!.type);
      near(node.joint!.axis, vectors.joints[bone.id]!.axis as number[], 1e-12, bone.id);
      for (const key of ["min", "max"] as const) {
        const want = vectors.joints[bone.id]![key] as number | undefined;
        if (want === undefined) expect(node.joint?.[key], `${bone.id}.${key}`).toBeUndefined();
        else expect(node.joint?.[key], `${bone.id}.${key}`).toBeCloseTo(want, 12);
      }
    }
    document.nodes.forEach((n, i) => {
      if (n.parent !== null) expect(document.nodes.findIndex((p) => p.id === n.parent)).toBeLessThan(i);
    });
  });

  it.each(vectors.variants)("maps $name to the mechanism joint, and every pose still replays through the kernel without a clamp", async (variant) => {
    const asset = variantOf(variant);
    const document = await modelOf(asset, { ...OPTIONS, poses: asset.poses.map((p) => p.id), clips: [] });
    const joint = document.nodes.find((n) => n.id === variant.bone)!.joint!;
    expect(joint.type).toBe(variant.expected.type);
    near(joint.axis, variant.expected.axis as number[], 1e-12);
    for (const key of ["min", "max", "twistMin", "twistMax"] as const) if (variant.expected[key] !== undefined) expect(joint[key], key).toBeCloseTo(variant.expected[key] as number, 12);
    for (const pose of document.poses) {
      const replay = worldOf(document, pose.id);
      expect(replay.issues, pose.id).toEqual([]);
      const state = evaluate(asset, { kind: "pose", pose: pose.id });
      for (const bone of asset.rig.bones) near(replay.world[bone.id]!, expectedWorld(state.bones[bone.id]!.frame).toArray(), vectors.tolerances.poseWorldM, `${pose.id}/${bone.id}`);
    }
  });

  it("replays every pose, named or key, of the sample figure to the evaluated bone frames", async () => {
    const asset = sampleFigure();
    const document = await modelOf(asset, { ...OPTIONS, poses: asset.poses.map((p) => p.id), clips: [] });
    expect(document.poses.map((p) => p.id)).toEqual(asset.poses.map((p) => p.id));
    for (const pose of document.poses) {
      const replay = worldOf(document, pose.id);
      expect(replay.issues, pose.id).toEqual([]);
      const state = evaluate(asset, { kind: "pose", pose: pose.id });
      for (const bone of asset.rig.bones) near(replay.world[bone.id]!, expectedWorld(state.bones[bone.id]!.frame).toArray(), vectors.tolerances.poseWorldM, `${pose.id}/${bone.id}`);
    }
  });

  it("samples every clip at its fps: the count, the key times and the evaluated frames at the keys", async () => {
    const asset = sampleFigure();
    const document = await modelOf(asset, { ...OPTIONS, poses: [] });
    expect(document.animations.map((a) => a.id)).toEqual(asset.clips.map((c) => c.id));
    for (const clip of asset.clips) {
      const expected = vectors.clips[clip.id]!;
      const animation = document.animations.find((a) => a.id === clip.id)!;
      expect(frameTimes(clip)).toHaveLength(expected.keys);
      expect(animation.durationS).toBe(expected.durationS);
      expect(animation.loop).toBe(clip.loop);
      for (const channel of animation.channels.filter((c) => "node" in c.target)) {
        expect(channel.times).toHaveLength(expected.keys);
        expect(channel.times[0]).toBe(0);
        expect(channel.times[channel.times.length - 1]).toBe(expected.durationS);
        channel.times.forEach((t, i) => i > 0 && expect(t).toBeGreaterThan(channel.times[i - 1]!));
      }
      const keys = frameTimes(clip).slice(0, -1);
      for (const timeS of keys.filter((_, i) => i % 7 === 0)) {
        const replay = worldOf(document, { animation: clip.id, timeS });
        expect(replay.issues).toEqual([]);
        const state = evaluate(asset, { kind: "clip", clip: clip.id, timeS });
        for (const bone of asset.rig.bones) near(replay.world[bone.id]!, expectedWorld(state.bones[bone.id]!.frame).toArray(), vectors.tolerances.clipKeyWorldM, `${clip.id}@${timeS}/${bone.id}`);
      }
      expect(animation.events.map((e) => e.t)).toEqual(clip.events.map((e) => e.t));
    }
  });

  it("samples between the keys within the tolerance of the evaluated frames, at arbitrary times", async () => {
    const asset = sampleFigure();
    const document = await modelOf(asset, { ...OPTIONS, poses: [] });
    for (const clip of asset.clips) {
      for (const fraction of [0.013, 0.211, 0.5003, 0.77, 0.9917]) {
        const timeS = fraction * clip.durationS;
        const replay = worldOf(document, { animation: clip.id, timeS });
        const state = evaluate(asset, { kind: "clip", clip: clip.id, timeS });
        for (const bone of asset.rig.bones) {
          const want = expectedWorld(state.bones[bone.id]!.frame).toArray();
          near(replay.world[bone.id]!.slice(12, 15), want.slice(12, 15), vectors.tolerances.clipBetweenKeysM, `${clip.id}@${timeS}/${bone.id} origin`);
          near(replay.world[bone.id]!.slice(0, 12), want.slice(0, 12), vectors.tolerances.clipBetweenKeysRad, `${clip.id}@${timeS}/${bone.id} axes`);
        }
      }
    }
  });

  it("carries root motion: a looping planted clip ends one cycle further ahead, and rotation keys never flip hemisphere", async () => {
    const asset = sampleFigure();
    const document = await modelOf(asset, { ...OPTIONS, poses: [] });
    const walk = asset.clips.find((c) => c.id === "walk")!;
    const root = document.animations.find((a) => a.id === "walk")!.channels.find((c) => "node" in c.target && c.target.node === asset.rig.root && c.target.path === "translation")!;
    const n = root.times.length;
    const first = root.values.slice(0, 3);
    const last = root.values.slice((n - 1) * 3, n * 3);
    expect(last[2]! - first[2]!).toBeCloseTo(walk.advanceM!, 3);
    for (const animation of document.animations) for (const c of animation.channels.filter((k) => "node" in k.target && k.target.path === "rotation"))
      for (let i = 1; i < c.times.length; i++) expect(c.values.slice(i * 4, i * 4 + 4).reduce((s, v, k) => s + v * c.values[(i - 1) * 4 + k]!, 0)).toBeGreaterThanOrEqual(0);
  });

  it("replays the FBX of a clip through three's FBXLoader and AnimationMixer to the evaluated bone positions", async () => {
    const asset = sampleFigure();
    for (const clip of asset.clips) {
      const scene = loadFbx(await writeFbx(asset, { clip: clip.id, scale: 50 }));
      expect(scene.animations).toHaveLength(1);
      expect(scene.animations[0]!.duration).toBeCloseTo(clip.durationS, 6);
      const mixer = new AnimationMixer(scene);
      mixer.clipAction(scene.animations[0]!).play();
      for (const fraction of [0, 0.25, 0.5, 0.75, 0.9]) {
        const timeS = fraction * clip.durationS;
        mixer.setTime(timeS);
        scene.updateMatrixWorld(true);
        const state = evaluate(asset, { kind: "clip", clip: clip.id, timeS });
        for (const bone of asset.rig.bones) {
          const at = scene.getObjectByName(bone.id)!.getWorldPosition(new Vector3());
          const f = state.bones[bone.id]!.frame.o;
          near([at.x, at.y, at.z], [f.z, f.y, f.x], vectors.tolerances.clipBetweenKeysM, `${clip.id}@${timeS}/${bone.id}`);
        }
      }
    }
  });
});
