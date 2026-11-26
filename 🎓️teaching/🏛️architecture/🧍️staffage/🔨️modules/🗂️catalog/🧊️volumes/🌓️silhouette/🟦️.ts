/** 🌓️ The silhouette agreement of the 2D drawings and the 3D volume of one asset at every authored scale, shared by `checkVolumes`, the catalog-wide guard and the authoring tools: the body of `volumes[scale]` is compared with the shapes of the representation of the same scale.
 *
 * One physical body: in a pose its side elevation, front elevation, plan and volume are projections of the same body. The authored side is what `evaluate` draws (filled `body`, `detail` and `glazing` shapes; open strokes and unfilled outlines at the true width of the member they draw), the other side the kernel tessellation of the solids carried by the bone frames of the pose and projected through the staffage cameras.
 * A view declared as a section (`representation.sections`) is compared with the volume cut by the plane: the cut faces of the bodies the plane crosses, which it must show; what remains visible beyond the plane, which it may show; and what lies in front of the plane, which it may only outline.
 * For a section the agreement is |(filled ∩ visible) ∪ (outlines ∩ full)| / |A ∪ cut|: the drawing may fill anything that remains visible beyond the plane and outline any part of the body (overhead and hidden lines), but must show every cut face and nothing that is not there. The area measure is an owned scanline rasteriser; the independent references live in `./oracles` (polygon-clipping exact areas, sharp's librsvg rasteriser).
 *
 * @see ../../🧊️model-silhouette/🥒️.feature
 * @see ../../🧊️catalog-silhouette/🥒️.feature
 */
import { invert4, multiply4, tessellateSolid, type M4 } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import type { Asset, AuthoredVolume, ScaleDenominator, ViewSection, VolumeList, VolumeSolid } from "../../../../🧬️schema/🟦️.ts";
import { solidsOf } from "../🧱️lists/🟦️.ts";
import { centreLineOf, clusterPoints } from "../🧵️centreline/🟦️.ts";
import { CAMERAS } from "../../../🎨️depiction/🔭️views/🟦️.ts";
import type { BoneFrame, Vec3 } from "../../../🔗️contracts/🟦️.ts";
import { restFrames } from "../../../🦴️rig/🟦️.ts";
import { worldFrameOf, type FrameLike } from "../../../🧊️volume/🦴️skeleton/🟦️.ts";
import { evaluate } from "../../../🧮️evaluation/🟦️.ts";
import { qualityOf } from "../../../🧊️volume/⏱️job/🟦️.ts";

/** 👁️ The three planar viewpoints. */
export const VIEWPOINTS = ["elevation", "front", "plan"] as const;
export type Viewpoint = (typeof VIEWPOINTS)[number];

/** 🚦️ The gate of the guard: the bounding box within 8 % per axis (the volume rules use one smallest feature instead), IoU of at least 0.92 per view, at the working scale 1:50 unless a scale is given. */
export const GATE = { bbox: 0.08, iou: 0.92, scale: 50, memberMaxPaperMm: 3, cells: 800, cellsMax: 4000, cellM: 0.02 } as const;

export type Point = { readonly x: number; readonly y: number };
export type Ring = readonly Point[];
/** 🧩️ Rings that fill together by the even-odd rule (a polygon with holes, a single triangle). */
export type Group = readonly Ring[];
/** 🧭️ The bone axes of a solid in the world, the frame that `sectionFaces` refer to. */
export type AxisFrame = { readonly u: { readonly x: number; readonly y: number; readonly z: number }; readonly v: { readonly x: number; readonly y: number; readonly z: number }; readonly w: { readonly x: number; readonly y: number; readonly z: number } };

type SeparateCopies = { readonly coordinatesPerCopy: number; readonly indices: Uint32Array };

/** 🧊️ A posed triangle mesh with its bone frame and authored presentation attributes. Exact body references may share the same geometry. */
export type World = { readonly geometrySource?: World; readonly separateCopies?: SeparateCopies; readonly id: string; readonly bone?: string; readonly material?: string; readonly positions: Float64Array; readonly indices: Uint32Array; readonly frame?: AxisFrame; readonly slender?: boolean; readonly penM?: number; readonly copyVertices?: number; readonly envelope?: boolean; readonly emitter?: boolean; readonly texture?: boolean; readonly soft?: boolean; readonly sectionFaces?: readonly string[] };
type Vec = readonly [number, number, number];

/** 🧭️ The cut axis (world x, y, z) and the sign of the retained side of the section of each viewpoint: the elevation is seen from -z (what lies at larger z remains), the front from +x (smaller x), the plan from +y (smaller y); a section with `retained: "near"` keeps the opposite side. */
export const CUT = { elevation: { axis: 2, retained: 1 }, front: { axis: 0, retained: -1 }, plan: { axis: 1, retained: -1 } } as const;

type CopyMesh = { readonly positions: Float64Array; readonly indices: Uint32Array };
const MESHES = new Map<string, CopyMesh>();
const MESH_CACHE_BYTES = 32 * 1024 * 1024;
let retainedMeshBytes = 0;
const meshBytes = (key: string, mesh: CopyMesh): number => 192 + key.length * 2 + mesh.positions.byteLength + mesh.indices.byteLength;

/** 🧊️ The mesh of one copy of a solid in its bone frame after `transform` (without `repeat`), tessellated once by the kernel per distinct geometry and quality and cached. */
export function copyMesh(solid: VolumeSolid, quality: { readonly chordM: number; readonly minSegments: number; readonly maxSegments: number }): { readonly positions: Float64Array; readonly indices: Uint32Array } {
  const { id: _id, bone, material: _material, role: _role, tone: _tone, weights: _weights, tags: _tags, repeat: _repeat, slender: _slender, sheet: _sheet, envelope: _envelope, texture: _texture, sectionFaces: _sectionFaces, ...geometry } = solid as VolumeSolid & Record<string, unknown>;
  const key = JSON.stringify([geometry, quality]);
  const hit = MESHES.get(key);
  if (hit) {
    MESHES.delete(key);
    MESHES.set(key, hit);
    return hit;
  }
  const g = tessellateSolid({ ...(geometry as object), id: "mesh", bone, material: "m", role: "body", tone: 0.5, node: bone } as never, quality);
  const mesh = { positions: Float64Array.from(g.positions), indices: Uint32Array.from(g.indices) };
  const bytes = meshBytes(key, mesh);
  if (bytes <= MESH_CACHE_BYTES) {
    for (const [oldKey, oldMesh] of MESHES) {
      if (retainedMeshBytes + bytes <= MESH_CACHE_BYTES) break;
      MESHES.delete(oldKey);
      retainedMeshBytes -= meshBytes(oldKey, oldMesh);
    }
    MESHES.set(key, mesh);
    retainedMeshBytes += bytes;
  }
  return mesh;
}

/** 🕰️ The affine maps of the copies of a `repeat`: copy k is p -> A_k p + b_k, the step (a rotation of `rotationRad` about the axis through `origin`, then `translation`) applied k times; the same placement as the kernel's. */
export function repeatMaps(repeat: NonNullable<VolumeSolid["repeat"]>): readonly { readonly a: readonly number[]; readonly b: readonly number[] }[] {
  const [t, o] = [repeat.translation ?? [0, 0, 0], repeat.origin ?? [0, 0, 0]];
  const axis = repeat.rotationAxis ?? [0, 1, 0];
  const n = Math.hypot(axis[0]!, axis[1]!, axis[2]!) || 1;
  const [x, y, z] = [axis[0]! / n, axis[1]! / n, axis[2]! / n];
  const [c, s] = [Math.cos(repeat.rotationRad ?? 0), Math.sin(repeat.rotationRad ?? 0)];
  const R = [c + x * x * (1 - c), x * y * (1 - c) - z * s, x * z * (1 - c) + y * s, y * x * (1 - c) + z * s, c + y * y * (1 - c), y * z * (1 - c) - x * s, z * x * (1 - c) - y * s, z * y * (1 - c) + x * s, c + z * z * (1 - c)];
  const mul = (m: readonly number[], v: readonly number[]): number[] => [m[0]! * v[0]! + m[1]! * v[1]! + m[2]! * v[2]!, m[3]! * v[0]! + m[4]! * v[1]! + m[5]! * v[2]!, m[6]! * v[0]! + m[7]! * v[1]! + m[8]! * v[2]!];
  const Ro = mul(R, o);
  const shift = [o[0]! - Ro[0]! + t[0]!, o[1]! - Ro[1]! + t[1]!, o[2]! - Ro[2]! + t[2]!];
  const out: { a: number[]; b: number[] }[] = [];
  let A = [1, 0, 0, 0, 1, 0, 0, 0, 1];
  let b = [0, 0, 0];
  for (let k = 0; k < repeat.count; k++) {
    out.push({ a: A, b });
    const cols = [0, 1, 2].map((j) => mul(R, [A[j]!, A[3 + j]!, A[6 + j]!]));
    A = [cols[0]![0]!, cols[1]![0]!, cols[2]![0]!, cols[0]![1]!, cols[1]![1]!, cols[2]![1]!, cols[0]![2]!, cols[1]![2]!, cols[2]![2]!];
    const rb = mul(R, b);
    b = [rb[0]! + shift[0]!, rb[1]! + shift[1]!, rb[2]! + shift[2]!];
  }
  return out;
}

const frameMatrix = (frame: FrameLike): M4 => {
  const { o, u, v, w } = worldFrameOf(frame);
  return [...u, 0, ...v, 0, ...w, 0, ...o, 1];
};

const REST_FRAME_MATRICES = new WeakMap<Asset["rig"], Record<string, M4>>();

/** 🧶 Poses uniformly weighted solids with the same rest-to-pose joint transforms as the exported skin. */
function posedFrames(asset: Asset, bones: Readonly<Record<string, { readonly frame: BoneFrame }>>) {
  const palette = new Map<string, M4>();
  return (solid: VolumeSolid): BoneFrame => {
    const joints = solid.weights?.filter(w => w.weight > 0).slice(0, 4);
    if (!joints?.length) return bones[solid.bone]!.frame;
    let rest = REST_FRAME_MATRICES.get(asset.rig);
    if (!rest) {
      rest = Object.fromEntries(Object.entries(restFrames(asset.rig)).map(([id, solved]) => [id, frameMatrix(solved.frame)]));
      REST_FRAME_MATRICES.set(asset.rig, rest);
    }
    const blend = Array<number>(16).fill(0);
    const sum = joints.reduce((total, joint) => total + joint.weight, 0);
    for (const joint of joints) {
      let delta = palette.get(joint.bone);
      if (!delta) {
        delta = multiply4(frameMatrix(bones[joint.bone]!.frame), invert4(rest[joint.bone]!));
        palette.set(joint.bone, delta);
      }
      for (let i = 0; i < 16; i++) blend[i]! += delta[i]! * joint.weight / sum;
    }
    const matrix = multiply4(blend, rest[solid.bone]!);
    const column = (i: number): Vec3 => ({ x: matrix[i]!, y: matrix[i + 1]!, z: matrix[i + 2]! });
    return { o: column(12), u: column(0), v: column(4), w: column(8) };
  };
}

/** 📐 The positive local section normals after a skin transform can shear or compress its tangents. */
function sectionAxes({ u, v, w }: BoneFrame): AxisFrame {
  const normal = (a: Vec3, b: Vec3, toward: Vec3): Vec3 => {
    const x = a.y * b.z - a.z * b.y, y = a.z * b.x - a.x * b.z, z = a.x * b.y - a.y * b.x;
    const length = Math.hypot(x, y, z);
    if (!length) return { x: 0, y: 0, z: 0 };
    const scale = (x * toward.x + y * toward.y + z * toward.z < 0 ? -1 : 1) / length;
    return { x: x * scale, y: y * scale, z: z * scale };
  };
  return { u: normal(v, w, u), v: normal(w, u, v), w: normal(u, v, w) };
}

/** 📏️ Separate bounding intervals prevent vertices of translated copies from welding across copies. */
function disjointTranslations(positions: Float64Array, stride: number): boolean {
  if (!stride || positions.length <= stride) return false;
  for (let axis = 0; axis < 3; axis++) {
    const sign = positions[stride + axis]! >= positions[axis]! ? 1 : -1;
    let previousMax = -Infinity;
    let separated = true;
    for (let start = 0; start < positions.length && separated; start += stride) {
      let low = Infinity, high = -Infinity;
      for (let i = start + axis; i < start + stride; i += 3) {
        const value = sign * positions[i]!;
        low = Math.min(low, value);
        high = Math.max(high, value);
      }
      separated = low - previousMax > 1e-6;
      previousMax = high;
    }
    if (separated) return true;
  }
  return false;
}

/** 🧊️ Poses the tessellated solids of one authored volume list. Passing the body worlds from the same asset, scale and pose shares exact geometry for references to body solids. */
export function worldSolids(asset: Asset, bones: Readonly<Record<string, { readonly frame: BoneFrame }>>, scale: number, list: VolumeList = "body", bodyWorlds?: readonly World[]): readonly World[] {
  const volume = (asset.volumes as Record<string, { readonly status: string }>)[String(scale)];
  if (volume === undefined || volume.status !== "authored") return [];
  const quality = qualityOf(asset);
  const emitters = new Set(asset.thermal.emitters.map((e) => e.id));
  const pens = (asset.representations[String(scale) as unknown as keyof Asset["representations"]] as unknown as { readonly pens?: Record<string, number | null> }).pens ?? {};
  const posed = posedFrames(asset, bones);
  const bodyById = bodyWorlds && new Map(bodyWorlds.map(solid => [solid.id, solid]));
  const references = new Map<string, World>();
  if (bodyById && list !== "body") {
    for (const entry of (volume as AuthoredVolume)[list]) {
      if (!("of" in entry)) continue;
      const source = bodyById.get(entry.of);
      if (source) references.set(entry.id, source);
    }
  }
  return solidsOf(volume as never, list).map((solid: VolumeSolid) => {
    const f = posed(solid);
    const source = references.get(solid.id);
    let positions: Float64Array, indices: Uint32Array, separateCopies: SeparateCopies | undefined, copyVertices: number | undefined;
    if (source) {
      ({ positions, indices, separateCopies, copyVertices } = source);
    } else {
      const base = copyMesh(solid, quality);
      const maps = solid.repeat === undefined ? [{ a: [1, 0, 0, 0, 1, 0, 0, 0, 1], b: [0, 0, 0] }] : repeatMaps(solid.repeat);
      const per = base.positions.length;
      positions = new Float64Array(per * maps.length);
      indices = maps.length === 1 ? base.indices.slice() : new Uint32Array(base.indices.length * maps.length);
      maps.forEach(({ a: A, b }, k) => {
        for (let i = 0; i < per; i += 3) {
          const [px, py, pz] = [base.positions[i]!, base.positions[i + 1]!, base.positions[i + 2]!];
          const [a, bb, c] = [A[0]! * px + A[1]! * py + A[2]! * pz + b[0]!, A[3]! * px + A[4]! * py + A[5]! * pz + b[1]!, A[6]! * px + A[7]! * py + A[8]! * pz + b[2]!];
          positions[k * per + i] = f.o.x + a * f.u.x + bb * f.v.x + c * f.w.x;
          positions[k * per + i + 1] = f.o.y + a * f.u.y + bb * f.v.y + c * f.w.y;
          positions[k * per + i + 2] = f.o.z + a * f.u.z + bb * f.v.z + c * f.w.z;
        }
        if (maps.length > 1) for (let i = 0; i < base.indices.length; i++) indices[k * base.indices.length + i] = base.indices[i]! + (k * per) / 3;
      });
      if (maps.length > 1 && !solid.repeat?.rotationRad && disjointTranslations(positions, per)) separateCopies = { coordinatesPerCopy: per, indices: base.indices };
      copyVertices = per / 3;
    }
    return { ...(source ? { geometrySource: source } : {}), ...(separateCopies ? { separateCopies } : {}), id: solid.id, bone: solid.bone, material: solid.material, positions, indices, frame: solid.weights?.length && solid.sectionFaces?.length ? sectionAxes(f) : { u: f.u, v: f.v, w: f.w }, ...(solid.slender === true ? { slender: true, copyVertices, ...(((pens[solid.role] ?? pens.body) ?? 0) > 0 ? { penM: (((pens[solid.role] ?? pens.body) as number) * scale) / 1000 } : {}) } : {}), ...(solid.envelope === true ? { envelope: true } : {}), ...(solid.tags?.some((tag) => tag === "emitter" || emitters.has(tag)) === true ? { emitter: true } : {}), ...(solid.texture === true ? { texture: true } : {}), ...(solid.sectionFaces === undefined ? {} : { sectionFaces: solid.sectionFaces }) };
  });
}

/** 📦️ The extent of the solids along world x, y and z. */
export function extentOf(solids: readonly World[]): { readonly min: Vec; readonly max: Vec } {
  const min: [number, number, number] = [Infinity, Infinity, Infinity];
  const max: [number, number, number] = [-Infinity, -Infinity, -Infinity];
  for (const s of solids) for (let i = 0; i < s.positions.length; i += 3) for (let k = 0; k < 3; k++) {
    const v = s.positions[i + k]!;
    if (v < min[k]!) min[k] = v;
    if (v > max[k]!) max[k] = v;
  }
  return { min, max };
}

/** 📐️ Orthographic projection of a world point through the camera of a viewpoint. */
export function project(view: Viewpoint, x: number, y: number, z: number): Point {
  const c = CAMERAS[view];
  const pick = (axis: "x" | "y" | "z"): number => (axis === "x" ? x : axis === "y" ? y : z);
  return { x: c.paperX.sign * pick(c.paperX.axis), y: c.paperUp.sign * pick(c.paperUp.axis) };
}

const KEY = 1e6;

/** ✂️ The cut face of a closed triangle mesh by the plane `coordinate[axis] = plane`: the loops of the intersection, assembled from the triangle segments by their end points (rounded to a micrometre). */
export function cutLoops(solid: World, axis: number, plane: number): readonly (readonly Vec[])[] {
  const { positions: p, indices } = solid;
  const segments: [Vec, Vec][] = [];
  const at = (i: number): Vec => [p[i * 3]!, p[i * 3 + 1]!, p[i * 3 + 2]!];
  const cross = (a: Vec, b: Vec): Vec => {
    const [da, db] = [a[axis]! - plane, b[axis]! - plane];
    const lexicographic = a[0]! < b[0]! || (a[0] === b[0] && (a[1]! < b[1]! || (a[1] === b[1] && a[2]! < b[2]!)));
    const [lo, hi] = lexicographic ? [a, b] : [b, a];
    const [dlo, dhi] = lexicographic ? [da, db] : [db, da];
    const t = dlo / (dlo - dhi);
    return [lo[0]! + (hi[0]! - lo[0]!) * t, lo[1]! + (hi[1]! - lo[1]!) * t, lo[2]! + (hi[2]! - lo[2]!) * t];
  };
  for (let t = 0; t < indices.length; t += 3) {
    const v = [at(indices[t]!), at(indices[t + 1]!), at(indices[t + 2]!)];
    const above = v.map((q) => q[axis]! >= plane);
    if (above[0] === above[1] && above[1] === above[2]) continue;
    const ends: Vec[] = [];
    for (let k = 0; k < 3; k++) if (above[k] !== above[(k + 1) % 3]) ends.push(cross(v[k]!, v[(k + 1) % 3]!));
    if (ends.length === 2) segments.push([ends[0]!, ends[1]!]);
  }
  const key = (q: Vec): string => `${Math.round(q[0]! * KEY)},${Math.round(q[1]! * KEY)},${Math.round(q[2]! * KEY)}`;
  const byEnd = new Map<string, number[]>();
  segments.forEach(([a, b], i) => {
    for (const q of [a, b]) {
      const list = byEnd.get(key(q));
      if (list === undefined) byEnd.set(key(q), [i]);
      else list.push(i);
    }
  });
  const used = new Uint8Array(segments.length);
  const loops: Vec[][] = [];
  for (let s = 0; s < segments.length; s++) {
    if (used[s]) continue;
    used[s] = 1;
    const loop: Vec[] = [segments[s]![0]];
    let tail = segments[s]![1];
    const first = key(loop[0]!);
    while (key(tail) !== first) {
      loop.push(tail);
      const next = (byEnd.get(key(tail)) ?? []).find((i) => !used[i]);
      if (next === undefined) break;
      used[next] = 1;
      const [a, b] = segments[next]!;
      tail = key(a) === key(tail) ? b : a;
    }
    if (key(tail) === first && loop.length >= 3) loops.push(loop);
  }
  return loops;
}

/** 🧱️ The connected pieces of a mesh: groups of triangle indices that share a vertex (positions rounded to a micrometre), so that two bodies of one solid (the two mullions of a bay) count apart. */
export function pieces(solid: World): readonly World[] {
  const { positions: p, indices } = solid;
  const parent = new Int32Array(indices.length / 3).map((_, i) => i);
  const find = (i: number): number => {
    while (parent[i] !== i) {
      parent[i] = parent[parent[i]!]!;
      i = parent[i]!;
    }
    return i;
  };
  const seen = new Map<string, number>();
  for (let t = 0; t < indices.length / 3; t++) for (let k = 0; k < 3; k++) {
    const v = indices[t * 3 + k]!;
    const key = `${Math.round(p[v * 3]! * KEY)},${Math.round(p[v * 3 + 1]! * KEY)},${Math.round(p[v * 3 + 2]! * KEY)}`;
    const other = seen.get(key);
    if (other === undefined) seen.set(key, t);
    else parent[find(t)] = find(other);
  }
  const groups = new Map<number, number[]>();
  for (let t = 0; t < indices.length / 3; t++) {
    const root = find(t);
    const list = groups.get(root);
    if (list === undefined) groups.set(root, [t]);
    else list.push(t);
  }
  return [...groups.values()].map((triangles) => ({ id: solid.id, positions: p, indices: Uint32Array.from(triangles.flatMap((t) => [indices[t * 3]!, indices[t * 3 + 1]!, indices[t * 3 + 2]!])) }));
}

/** 🔪️ The part of a triangle on one side of the plane `coordinate[axis] = plane` (Sutherland-Hodgman), as a polygon of 3D points. */
export function clipTriangle(t: readonly Vec[], axis: number, plane: number, keepHigh: boolean): readonly Vec[] {
  const inside = (q: Vec): boolean => (keepHigh ? q[axis]! >= plane : q[axis]! < plane);
  const out: Vec[] = [];
  for (let i = 0; i < t.length; i++) {
    const [a, b] = [t[i]!, t[(i + 1) % t.length]!];
    if (inside(a)) out.push(a);
    if (inside(a) !== inside(b)) {
      const f = (plane - a[axis]!) / (b[axis]! - a[axis]!);
      out.push([a[0]! + (b[0]! - a[0]!) * f, a[1]! + (b[1]! - a[1]!) * f, a[2]! + (b[2]! - a[2]!) * f]);
    }
  }
  return out;
}

/** ✏️ The 2D shapes of one viewpoint as the slender solids follow them: the bone, whether the shape is closed and filled, its rings and the pen width in metres at the scale (the shape's own `penMm`, else the pen of its role). */
export type Drawn = readonly { readonly bone: string; readonly material?: string; readonly body?: boolean; readonly closed: boolean; readonly filled: boolean; readonly rings: readonly Ring[]; readonly penM: number }[];

const DRAWN = new WeakMap<object, Drawn>();

/** ✏️ {@link Drawn} of the evaluated shapes of one viewpoint at 1:`scale`, memoised per array. */
export function drawnOf(evaluated: readonly { readonly shape: { readonly kind?: string; readonly bone?: string; readonly material?: string; readonly role: string; readonly penMm?: number; readonly filled?: boolean }; readonly rings: readonly Ring[]; readonly closed: boolean }[], scale: number, pens: Readonly<Record<string, number | null>>): Drawn {
  let hit = DRAWN.get(evaluated);
  if (hit === undefined) DRAWN.set(evaluated, (hit = evaluated.map((s) => ({ bone: s.shape.bone ?? "", ...(s.shape.material === undefined ? {} : { material: s.shape.material }), ...(s.shape.kind === "loft" ? { body: true } : {}), closed: s.closed, filled: s.shape.filled !== false, rings: s.rings, penM: (((s.shape.penMm ?? pens[s.shape.role]) ?? 0) * scale) / 1000 }))));
  return hit;
}

const polylines = (d: Drawn[number]): readonly (readonly Point[])[] => d.rings.map((ring) => (d.closed && ring.length > 0 ? [...ring, ring[0]!] : ring));

function distanceToPolylines(p: Point, lines: readonly (readonly Point[])[]): number {
  let best = Infinity;
  for (const line of lines) {
    if (line.length === 1) best = Math.min(best, Math.hypot(p.x - line[0]!.x, p.y - line[0]!.y));
    for (let i = 0; i + 1 < line.length; i++) {
      const [a, b] = [line[i]!, line[i + 1]!];
      const [dx, dy] = [b.x - a.x, b.y - a.y];
      const l2 = dx * dx + dy * dy;
      const f = l2 === 0 ? 0 : Math.max(0, Math.min(1, ((p.x - a.x) * dx + (p.y - a.y) * dy) / l2));
      best = Math.min(best, Math.hypot(p.x - a.x - f * dx, p.y - a.y - f * dy));
    }
  }
  return best;
}

/** 🧭️ The principal axis of a point set as the centre line of a strip: the segment between the extreme points along the axis and the width across it. */
function stripOf(points: readonly Point[]): { readonly line: readonly Point[]; readonly length: number; readonly width: number } {
  const [mx, my] = [points.reduce((t, p) => t + p.x, 0) / points.length, points.reduce((t, p) => t + p.y, 0) / points.length];
  let [sxx, sxy, syy] = [0, 0, 0];
  for (const p of points) [sxx, sxy, syy] = [sxx + (p.x - mx) ** 2, sxy + (p.x - mx) * (p.y - my), syy + (p.y - my) ** 2];
  const angle = 0.5 * Math.atan2(2 * sxy, sxx - syy);
  const [ux, uy] = [Math.cos(angle), Math.sin(angle)];
  let [lo, hi, left, right] = [Infinity, -Infinity, Infinity, -Infinity];
  for (const p of points) {
    const [t, n] = [(p.x - mx) * ux + (p.y - my) * uy, -(p.x - mx) * uy + (p.y - my) * ux];
    [lo, hi, left, right] = [Math.min(lo, t), Math.max(hi, t), Math.min(left, n), Math.max(right, n)];
  }
  const [cx, cy] = [mx - ((left + right) / 2) * uy, my + ((left + right) / 2) * ux];
  return { line: [{ x: cx + ux * lo, y: cy + uy * lo }, { x: cx + ux * hi, y: cy + uy * hi }], length: hi - lo, width: right - left };
}

/** 🪡️ The width a slender solid is drawn with (C24 refined): of the 2D shape it follows, found on the same bone and material where the projected centre line lies within half the drawn width on average and within half the physical width of a filled strip: an open stroke or an unfilled outline draws its own pen width (the caller leaves out the outlines that are the edge of a body, {@link strokeDrawings}), a filled strip (at least three times as long as wide) its own width; `undefined` when the viewpoint draws no such shape for the bone. */
export function followedWidth(bone: string | undefined, centre: readonly Point[], drawn: Drawn, material?: string, projectedWidthM = Infinity): number | undefined {
  let [best, width] = [Infinity, undefined as number | undefined];
  for (const d of drawn) {
    if (d.bone !== bone || d.rings.length === 0 || (material !== undefined && d.material !== undefined && material !== d.material)) continue;
    let [lines, w] = [polylines(d), d.penM];
    if (d.closed && d.filled) {
      const strip = stripOf(d.rings.flat());
      if (!(strip.width > 0) || strip.length / strip.width < 3) continue;
      [lines, w] = [[strip.line], strip.width];
    }
    if (!(w > 0)) continue;
    const mean = centre.reduce((t, p) => t + distanceToPolylines(p, lines), 0) / centre.length;
    const reach = d.closed && d.filled ? Math.min(w, projectedWidthM) : w;
    if (mean <= reach / 2 && mean < best) [best, width] = [mean, w];
  }
  return width;
}

/** 🫧️ Whether a `loft` shape of the bone (and material), filled and closed, is the body of a slender solid itself (a drawing that carries the sections of the solid; any other filled shape is compared at the width it is drawn with): nine tenths of the points of its projected centre line lie inside the shape or within half the true width of its outline. A `loft` or `member` drawn from the sections of the solid is such a shape: the reference is then the solid as it is (its projected triangles, tapering included), not a stroke of one width. */
export function drawsBody(bone: string | undefined, centre: readonly Point[], drawn: Drawn, material: string | undefined, truthM: number): boolean {
  const inside = (p: Point, ring: readonly Point[]): boolean => {
    let odd = false;
    for (let i = 0, k = ring.length - 1; i < ring.length; k = i++) {
      const [a, b] = [ring[k]!, ring[i]!];
      if (a.y > p.y !== b.y > p.y && p.x < ((b.x - a.x) * (p.y - a.y)) / (b.y - a.y) + a.x) odd = !odd;
    }
    return odd;
  };
  return drawn.some((d) => d.bone === bone && d.body === true && d.closed && d.filled && d.rings.length > 0 && (material === undefined || d.material === undefined || d.material === material) && centre.filter((p) => d.rings.some((ring) => inside(p, ring)) || distanceToPolylines(p, polylines(d)) <= truthM / 2).length >= 0.9 * centre.length);
}

/** 🧵️ The strokes a slender solid is drawn as (C24 refined): per copy of its `repeat`, its centre line (curved or closed as the solid is, see {@link centreLineOf}; a straight principal axis where the mesh gives none) widened to the width of the 2D shape it follows ({@link followedWidth}), else to its true projected width, and where no drawing is given to the larger of the pen of its role and that true width. One quadrilateral per segment of the centre line, like the strokes of the drawing. */
export function strokesOf(view: Viewpoint, solid: World, drawn?: Drawn): readonly Ring[] {
  const per = solid.copyVertices ?? solid.positions.length / 3;
  const copies = Math.max(1, Math.round(solid.positions.length / 3 / per));
  const local = solid.indices.subarray(0, solid.indices.length / copies);
  const line = centreLineOf(solid.positions, solid.positions.subarray(0, per * 3), local);
  const out: Ring[] = [];
  for (let c = 0; c < copies; c++) {
    const vertices = Array.from({ length: per }, (_, i) => project(view, solid.positions[(c * per + i) * 3]!, solid.positions[(c * per + i) * 3 + 1]!, solid.positions[(c * per + i) * 3 + 2]!));
    if (line === undefined) {
      out.push(...axisStroke(vertices, drawn === undefined ? solid.penM ?? 0 : followedWidth(solid.bone, vertices, drawn, solid.material, stripOf(vertices).width) ?? 0));
      continue;
    }
    const centre = clusterPoints(line, solid.positions, c * per).map((q) => project(view, q[0], q[1], q[2]));
    const segments = line.edges.length === 0 ? centre.map((p) => [p, p] as const) : line.edges.map(([i, j]) => [centre[i]!, centre[j]!] as const);
    const step = Math.max(1, Math.ceil(vertices.length / 300));
    const offsets = vertices.filter((_, i) => i % step === 0).map((v) => Math.min(...segments.map(([a, b]) => distanceToPolylines(v, [[a, b]])))).sort((x, y) => x - y);
    const truth = Math.max(1e-4, 2 * (offsets[Math.floor(0.9 * (offsets.length - 1))] ?? 0));
    if (drawn !== undefined && drawsBody(solid.bone, centre, drawn, solid.material, truth)) {
      for (let t = 0; t + 2 < local.length; t += 3) out.push([0, 1, 2].map((k) => vertices[local[t + k]!]!));
      continue;
    }
    const followed = drawn === undefined ? undefined : followedWidth(solid.bone, centre, drawn, solid.material, truth);
    const width = followed ?? (drawn === undefined ? Math.max(solid.penM ?? 0, truth) : truth);
    for (const [a, b] of segments) out.push(strokeQuad(a, b, width));
  }
  return out;
}

/** 🧵️ The stroke of the principal axis of the projected vertices, widened to at least `pen` and the true width: the centre line of a solid whose mesh gives none. */
function axisStroke(points: readonly Point[], pen: number): readonly Ring[] {
  const [mx, my] = [points.reduce((t, p) => t + p.x, 0) / points.length, points.reduce((t, p) => t + p.y, 0) / points.length];
  let [sxx, sxy, syy] = [0, 0, 0];
  for (const p of points) [sxx, sxy, syy] = [sxx + (p.x - mx) ** 2, sxy + (p.x - mx) * (p.y - my), syy + (p.y - my) ** 2];
  const angle = 0.5 * Math.atan2(2 * sxy, sxx - syy);
  const [ux, uy] = [Math.cos(angle), Math.sin(angle)];
  let [lo, hi, side] = [Infinity, -Infinity, 0];
  for (const p of points) {
    const [t, n] = [(p.x - mx) * ux + (p.y - my) * uy, Math.abs(-(p.x - mx) * uy + (p.y - my) * ux)];
    [lo, hi, side] = [Math.min(lo, t), Math.max(hi, t), Math.max(side, n)];
  }
  return [strokeQuad({ x: mx + ux * lo, y: my + uy * lo }, { x: mx + ux * hi, y: my + uy * hi }, Math.max(pen, 2 * side))];
}

/** 📚️ What a view is compared with. `cut` is what the drawing has to show: every projected triangle of the solids, or for a declared section the cut faces of the bodies the plane crosses. `visible` is all the drawing may fill: every projected triangle, or for a section the projection of everything that remains beyond the plane (the far faces of a cut body included, clipped at the plane). A body in front of the plane is removed from `cut` and `visible`; `full` is the projection of the whole volume, where a section may still show a removed or hidden body as a thin outline (the overhead and hidden-line convention). */
export function referenceOf(view: Viewpoint, solids: readonly World[], section: ViewSection | undefined, drawn?: Drawn, geometry: "drawing" | "mesh" = "drawing"): { readonly cut: Group[]; readonly visible: Group[]; readonly full: Group[] } {
  const cut: Group[] = [];
  const visible: Group[] = section === undefined ? cut : [];
  const full: Group[] = section === undefined ? cut : [];
  const side = { axis: CUT[view].axis, retained: section?.retained === "near" ? -CUT[view].retained : CUT[view].retained };
  const followable = geometry === "drawing" && section === undefined && drawn !== undefined ? strokeDrawings(view, solids, drawn) : drawn;
  for (const whole of solids) {
    if (geometry === "drawing" && section === undefined && whole.slender === true) {
      for (const ring of strokesOf(view, whole, followable)) cut.push([ring]);
      continue;
    }
    const p = whole.positions;
    const projected: Point[] = [];
    for (let i = 0; i < p.length; i += 3) projected.push(project(view, p[i]!, p[i + 1]!, p[i + 2]!));
    const triangle = (indices: Uint32Array, t: number): Group => [[projected[indices[t]!]!, projected[indices[t + 1]!]!, projected[indices[t + 2]!]!]];
    for (let t = 0; t < whole.indices.length; t += 3) full.push(triangle(whole.indices, t));
    if (section === undefined) continue;
    const at = (i: number): Vec => [p[i * 3]!, p[i * 3 + 1]!, p[i * 3 + 2]!];
    for (const piece of pieces(whole)) {
      const indices = piece.indices;
      let low = false, high = false;
      for (const i of indices) {
        if (p[i * 3 + side.axis]! < section.plane) low = true;
        else high = true;
        if (low && high) break;
      }
      if (side.retained > 0 ? !high : !low) continue;
      for (let t = 0; t < indices.length; t += 3) {
        if (low && high) {
          const kept = clipTriangle([at(indices[t]!), at(indices[t + 1]!), at(indices[t + 2]!)], side.axis, section.plane, side.retained > 0);
          if (kept.length >= 3) visible.push([kept.map(q => project(view, q[0], q[1], q[2]))]);
        } else visible.push(triangle(indices, t));
      }
      if (low && high) cut.push(cutLoops(piece, side.axis, section.plane).map(loop => loop.map(q => project(view, q[0], q[1], q[2]))));
    }
  }
  return { cut, visible, full };
}

export function edt1(f: Float64Array, n: number, d: Float64Array, v: Int32Array, z: Float64Array): void {
  let k = 0;
  v[0] = 0;
  z[0] = -Infinity;
  z[1] = Infinity;
  for (let q = 1; q < n; q++) {
    let s = 0;
    for (;;) {
      const p = v[k]!;
      s = (f[q]! + q * q - (f[p]! + p * p)) / (2 * q - 2 * p);
      if (s > z[k]!) break;
      k--;
    }
    k++;
    v[k] = q;
    z[k] = s;
    z[k + 1] = Infinity;
  }
  k = 0;
  for (let q = 0; q < n; q++) {
    while (z[k + 1]! < q) k++;
    d[q] = (q - v[k]!) * (q - v[k]!) + f[v[k]!]!;
  }
}

/** 📏️ The squared Euclidean distance in cells of every cell to the nearest set cell of `mask` (Felzenszwalb-Huttenlocher), `Infinity` everywhere when no cell is set. */
export function distanceField(mask: Uint8Array, width: number, height: number): Float64Array {
  const big = 1e20;
  const grid = new Float64Array(width * height);
  for (let i = 0; i < grid.length; i++) grid[i] = mask[i] ? 0 : big;
  const n = Math.max(width, height);
  const [f, d, z] = [new Float64Array(n), new Float64Array(n), new Float64Array(n + 1)];
  const v = new Int32Array(n);
  for (let x = 0; x < width; x++) {
    for (let y = 0; y < height; y++) f[y] = grid[y * width + x]!;
    edt1(f, height, d, v, z);
    for (let y = 0; y < height; y++) grid[y * width + x] = d[y]!;
  }
  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) f[x] = grid[y * width + x]!;
    edt1(f, width, d, v, z);
    for (let x = 0; x < width; x++) grid[y * width + x] = d[x]!;
  }
  return grid;
}

const OFFSETS = new Map<number, { readonly dx: Int32Array; readonly dy: Int32Array; readonly d2: Int32Array }>();

/** 🔎️ The cell offsets within `radius` cells, nearest first (the order of squared distance), cached per radius. */
function offsetsWithin(radius: number): { readonly dx: Int32Array; readonly dy: Int32Array; readonly d2: Int32Array } {
  let hit = OFFSETS.get(radius);
  if (hit === undefined) {
    const list: [number, number, number][] = [];
    for (let y = -radius; y <= radius; y++) for (let x = -radius; x <= radius; x++) if (x * x + y * y <= radius * radius) list.push([x, y, x * x + y * y]);
    list.sort((p, q) => p[2] - q[2]);
    hit = { dx: Int32Array.from(list.map((o) => o[0])), dy: Int32Array.from(list.map((o) => o[1])), d2: Int32Array.from(list.map((o) => o[2])) };
    OFFSETS.set(radius, hit);
  }
  return hit;
}

/** 🔎️ The squared distance in cells from cell `k` to the nearest set cell of `target` when it is at most `maxD2`, else `Infinity`: an exact nearest-neighbour search over the offsets within the radius, the cheap answer for the few boundary cells of a tile where {@link distanceField} would transform the whole window. */
export function nearWithin(target: Uint8Array, width: number, height: number, k: number, maxD2: number): number {
  const radius = Math.floor(Math.sqrt(maxD2));
  const { dx, dy, d2 } = offsetsWithin(radius);
  const [i, j] = [k % width, Math.floor(k / width)];
  const inside = i >= radius && j >= radius && i < width - radius && j < height - radius;
  for (let n = 0; n < d2.length; n++) {
    if (d2[n]! > maxD2) break;
    if (inside) {
      if (target[k + dy[n]! * width + dx[n]!]) return d2[n]!;
    } else {
      const [x, y] = [i + dx[n]!, j + dy[n]!];
      if (x >= 0 && y >= 0 && x < width && y < height && target[y * width + x]) return d2[n]!;
    }
  }
  return Infinity;
}

/** 🧭️ The symmetric Hausdorff distance in metres between two binary masks of the same grid, to the precision of one cell (the largest distance of a set cell of one mask to the nearest set cell of the other); 0 when both are empty, `Infinity` when exactly one is. */
export function hausdorff(a: Uint8Array, b: Uint8Array, width: number, height: number, cell: number): number {
  const [na, nb] = [a.reduce((s, x) => s + x, 0), b.reduce((s, x) => s + x, 0)];
  if (na === 0 && nb === 0) return 0;
  if (na === 0 || nb === 0) return Infinity;
  let worst = 0;
  for (const [from, to] of [[a, b], [b, a]] as const) {
    const field = distanceField(to, width, height);
    for (let i = 0; i < from.length; i++) if (from[i]! && field[i]! > worst) worst = field[i]!;
  }
  return Math.sqrt(worst) * cell;
}

export type Box = { readonly minX: number; readonly minY: number; readonly maxX: number; readonly maxY: number };

const BOUNDS = new WeakMap<object, readonly [number, number, number, number]>();

/** 📦️ The bounding box [minX, maxX, minY, maxY] of the rings of a group, computed once per group. */
export function boundsOf(rings: Group): readonly [number, number, number, number] {
  let hit = BOUNDS.get(rings);
  if (hit === undefined) {
    let [minX, maxX, minY, maxY] = [Infinity, -Infinity, Infinity, -Infinity];
    for (const ring of rings) for (const p of ring) {
      if (p.x < minX) minX = p.x;
      if (p.x > maxX) maxX = p.x;
      if (p.y < minY) minY = p.y;
      if (p.y > maxY) maxY = p.y;
    }
    hit = [minX, maxX, minY, maxY];
    BOUNDS.set(rings, hit);
  }
  return hit;
}

/** 📏️ The distance of a point to a triangle (0 inside). */
export function distanceToTriangle(p: Point, t: readonly [Point, Point, Point]): number {
  const inside = (a: Point, b: Point): number => (b.x - a.x) * (p.y - a.y) - (b.y - a.y) * (p.x - a.x);
  const [d0, d1, d2] = [inside(t[0], t[1]), inside(t[1], t[2]), inside(t[2], t[0])];
  if ((d0 !== 0 || d1 !== 0 || d2 !== 0) && ((d0 >= 0 && d1 >= 0 && d2 >= 0) || (d0 <= 0 && d1 <= 0 && d2 <= 0))) return 0;
  let best = Infinity;
  for (let k = 0; k < 3; k++) {
    const [a, b] = [t[k]!, t[(k + 1) % 3]!];
    const [dx, dy] = [b.x - a.x, b.y - a.y];
    const l2 = dx * dx + dy * dy;
    const f = l2 === 0 ? 0 : Math.max(0, Math.min(1, ((p.x - a.x) * dx + (p.y - a.y) * dy) / l2));
    best = Math.min(best, Math.hypot(p.x - a.x - f * dx, p.y - a.y - f * dy));
  }
  return best;
}

/** 🪶 Distance to the nearest projected mesh triangle, capped at the supplied search radius. */
export function projectedBodyDistance(faces: readonly Group[], frame: Box): (point: Point, radius: number) => number {
  const nearby = bucketIndex(faces, boundsOf, frame, mayTouch, 32);
  return (point, radius) => {
    const window = { minX: point.x - radius, minY: point.y - radius, maxX: point.x + radius, maxY: point.y + radius };
    let distance = radius;
    for (const group of nearby(window)) {
      const triangle = group[0];
      if (triangle?.length === 3) distance = Math.min(distance, distanceToTriangle(point, triangle as readonly [Point, Point, Point]));
      if (distance === 0) break;
    }
    return distance;
  };
}

/** 🖼️ A binary coverage grid sampled at cell centres, filled by the even-odd scanline rule. */
export class Mask {
  readonly width: number;
  readonly height: number;
  readonly cells: Uint8Array;
  constructor(readonly box: Box, readonly cell: number) {
    this.width = Math.max(1, Math.ceil((box.maxX - box.minX) / cell));
    this.height = Math.max(1, Math.ceil((box.maxY - box.minY) / cell));
    this.cells = new Uint8Array(this.width * this.height);
  }

  private triangles: (readonly [Point, Point, Point])[] | undefined;

  /** 🧷️ Closes the hairlines between solids that touch exactly (a silencer tangent on a roof, tessellated a hair inside its true outline), the morphological closing by an eighth of a cell of the triangles filled with `seal`: a cell that no triangle covers, that is covered on both sides along a row, a column or a diagonal, and whose disc of an eighth of a cell lies entirely within an eighth of a cell of the triangles, is covered. A gap wider than a quarter of a cell, a hole and the outer boundary are never changed. */
  seal(): void {
    const triangles = this.triangles;
    if (triangles === undefined) return;
    this.triangles = undefined;
    const { width, height, cells, box, cell } = this;
    const reach = cell / 8;
    const candidates = new Map<number, number[]>();
    let count = 0;
    for (let j = 1; j < height - 1; j++)
      for (let i = 1; i < width - 1; i++) {
        const k = j * width + i;
        if (cells[k]) continue;
        if (!((cells[k - 1] && cells[k + 1]) || (cells[k - width] && cells[k + width]) || (cells[k - width - 1] && cells[k + width + 1]) || (cells[k - width + 1] && cells[k + width - 1]))) continue;
        const bucket = (j >> 4) * 65536 + (i >> 4);
        const list = candidates.get(bucket);
        if (list === undefined) candidates.set(bucket, [k]);
        else list.push(k);
        count++;
      }
    if (count === 0) return;
    const centre = (k: number): Point => ({ x: box.minX + ((k % width) + 0.5) * cell, y: box.minY + (Math.floor(k / width) + 0.5) * cell });
    const near = new Map<number, (readonly [Point, Point, Point])[]>();
    for (const t of triangles) {
      const [x0, x1] = [Math.min(t[0].x, t[1].x, t[2].x) - 2 * reach, Math.max(t[0].x, t[1].x, t[2].x) + 2 * reach];
      const [y0, y1] = [Math.min(t[0].y, t[1].y, t[2].y) - 2 * reach, Math.max(t[0].y, t[1].y, t[2].y) + 2 * reach];
      const [i0, i1] = [Math.max(0, Math.floor((x0 - box.minX) / cell)) >> 4, Math.min(width - 1, Math.floor((x1 - box.minX) / cell)) >> 4];
      const [j0, j1] = [Math.max(0, Math.floor((y0 - box.minY) / cell)) >> 4, Math.min(height - 1, Math.floor((y1 - box.minY) / cell)) >> 4];
      for (let bj = j0; bj <= j1; bj++)
        for (let bi = i0; bi <= i1; bi++)
          for (const k of candidates.get(bj * 65536 + bi) ?? []) {
            const c = centre(k);
            if (c.x < x0 || c.x > x1 || c.y < y0 || c.y > y1 || distanceToTriangle(c, t) > 2 * reach) continue;
            const list = near.get(k);
            if (list === undefined) near.set(k, [t]);
            else list.push(t);
          }
    }
    const probes = [[0, 0], ...[0.5, 1].flatMap((r) => [[r, 0], [-r, 0], [0, r], [0, -r], [0.7071 * r, 0.7071 * r], [-0.7071 * r, 0.7071 * r], [0.7071 * r, -0.7071 * r], [-0.7071 * r, -0.7071 * r]])] as const;
    for (const [k, list] of near) {
      const c = centre(k);
      if (probes.every(([dx, dy]) => list.some((t) => distanceToTriangle({ x: c.x + dx * reach, y: c.y + dy * reach }, t) <= reach))) cells[k] = 1;
    }
  }

  /** 🧽️ The hairline closing of {@link seal} for filled rings (the drawn shapes): a cell that no ring covers, that is covered on both sides along a row, a column or a diagonal, and whose disc of an eighth of a cell lies entirely within an eighth of a cell of the filled rings, is covered. One ring is filled by the non-zero winding rule, several by the even-odd rule, as {@link fill} fills them. A gap wider than a quarter of a cell, a hole and the outer boundary are never changed. */
  sealRings(groups: readonly Group[]): void {
    const { width, height, cells, box, cell } = this;
    const reach = cell / 8;
    const candidates: number[] = [];
    for (let j = 1; j < height - 1; j++)
      for (let i = 1; i < width - 1; i++) {
        const k = j * width + i;
        if (cells[k]) continue;
        if ((cells[k - 1] && cells[k + 1]) || (cells[k - width] && cells[k + width]) || (cells[k - width - 1] && cells[k + width + 1]) || (cells[k - width + 1] && cells[k + width - 1])) candidates.push(k);
      }
    if (candidates.length === 0) return;
    const boxes = groups.map((g) => boundsOf(g));
    const inside = (g: Group, p: Point): boolean => {
      let [winding, crossings] = [0, 0];
      for (const ring of g)
        for (let i = 0, k = ring.length - 1; i < ring.length; k = i++) {
          const [a, b] = [ring[k]!, ring[i]!];
          if (a.y <= p.y === b.y <= p.y) continue;
          if (a.x + ((p.y - a.y) / (b.y - a.y)) * (b.x - a.x) <= p.x) continue;
          crossings++;
          winding += b.y > a.y ? 1 : -1;
        }
      return g.length === 1 ? winding !== 0 : crossings % 2 === 1;
    };
    const toEdge = (g: Group, p: Point): number => {
      let best = Infinity;
      for (const ring of g)
        for (let i = 0, k = ring.length - 1; i < ring.length; k = i++) {
          const [a, b] = [ring[k]!, ring[i]!];
          const [dx, dy] = [b.x - a.x, b.y - a.y];
          const t = Math.max(0, Math.min(1, ((p.x - a.x) * dx + (p.y - a.y) * dy) / (dx * dx + dy * dy || 1)));
          best = Math.min(best, Math.hypot(p.x - a.x - t * dx, p.y - a.y - t * dy));
        }
      return best;
    };
    const probes = [[0, 0], ...[0.5, 1].flatMap((r) => [[r, 0], [-r, 0], [0, r], [0, -r], [0.7071 * r, 0.7071 * r], [-0.7071 * r, 0.7071 * r], [0.7071 * r, -0.7071 * r], [-0.7071 * r, -0.7071 * r]])] as const;
    for (const k of candidates) {
      const c: Point = { x: box.minX + ((k % width) + 0.5) * cell, y: box.minY + (Math.floor(k / width) + 0.5) * cell };
      const near = groups.filter((_, n) => c.x >= boxes[n]![0] - 2 * reach && c.x <= boxes[n]![1] + 2 * reach && c.y >= boxes[n]![2] - 2 * reach && c.y <= boxes[n]![3] + 2 * reach);
      if (near.length === 0) continue;
      if (probes.every(([dx, dy]) => near.some((g) => inside(g, { x: c.x + dx * reach, y: c.y + dy * reach }) || toEdge(g, { x: c.x + dx * reach, y: c.y + dy * reach }) <= reach))) cells[k] = 1;
    }
  }

  /** 🖌️ Fills the rings: one ring by the non-zero winding rule (a self-crossing polygon is filled where it winds around), several rings by the even-odd rule (a polygon with holes); `seal` marks a triangle for {@link seal}. */
  fill(rings: Group, seal = false): void {
    const { box, cell, width, height, cells } = this;
    const bounds = boundsOf(rings);
    const [lo, hi] = [bounds[2], bounds[3]];
    if (!Number.isFinite(lo) || hi < box.minY - cell || lo > box.maxY + cell || bounds[1] < box.minX - cell || bounds[0] > box.maxX + cell) return;
    if (rings.length === 1 && rings[0]!.length === 3) {
      const [p0, p1, p2] = rings[0] as readonly [Point, Point, Point];
      if (seal) (this.triangles ??= []).push([p0, p1, p2]);
      for (let j = Math.max(0, Math.ceil((lo - box.minY) / cell - 0.5)); j <= Math.min(height - 1, Math.floor((hi - box.minY) / cell - 0.5)); j++) {
        const y = box.minY + (j + 0.5) * cell;
        let [x0, x1, n] = [Infinity, -Infinity, 0];
        for (const [a, b] of [[p0, p1], [p1, p2], [p2, p0]] as const)
          if (a.y <= y !== b.y <= y) {
            const x = a.x + ((y - a.y) / (b.y - a.y)) * (b.x - a.x);
            if (x < x0) x0 = x;
            if (x > x1) x1 = x;
            n++;
          }
        if (n < 2) continue;
        for (let i = Math.max(0, Math.ceil((x0 - box.minX) / cell - 0.5)); i <= Math.min(width - 1, Math.ceil((x1 - box.minX) / cell - 0.5) - 1); i++) cells[j * width + i] = 1;
      }
      return;
    }
    const winding = rings.length === 1;
    const [cx, cd]: [number[], number[]] = [[], []];
    for (let j = Math.max(0, Math.ceil((lo - box.minY) / cell - 0.5)); j <= Math.min(height - 1, Math.floor((hi - box.minY) / cell - 0.5)); j++) {
      const y = box.minY + (j + 0.5) * cell;
      cx.length = 0;
      cd.length = 0;
      for (const ring of rings) for (let i = 0, k = ring.length - 1; i < ring.length; k = i++) {
        const [a, b] = [ring[k]!, ring[i]!];
        if (a.y <= y !== b.y <= y) {
          const x = a.x + ((y - a.y) / (b.y - a.y)) * (b.x - a.x);
          let n = cx.length;
          cx.push(x);
          cd.push(b.y > a.y ? 1 : -1);
          while (n > 0 && cx[n - 1]! > x) {
            [cx[n], cx[n - 1]] = [cx[n - 1]!, cx[n]!];
            [cd[n], cd[n - 1]] = [cd[n - 1]!, cd[n]!];
            n--;
          }
        }
      }
      const span = (from: number, to: number): void => {
        for (let i = Math.max(0, Math.ceil((from - box.minX) / cell - 0.5)); i <= Math.min(width - 1, Math.ceil((to - box.minX) / cell - 0.5) - 1); i++) cells[j * width + i] = 1;
      };
      if (winding) {
        let w = 0;
        for (let n = 0; n + 1 < cx.length; n++) {
          w += cd[n]!;
          if (w !== 0) span(cx[n]!, cx[n + 1]!);
        }
      } else for (let n = 0; n + 1 < cx.length; n += 2) span(cx[n]!, cx[n + 1]!);
    }
  }

  has(x: number, y: number): boolean {
    const i = Math.floor((x - this.box.minX) / this.cell);
    const j = Math.floor((y - this.box.minY) / this.cell);
    return i >= 0 && j >= 0 && i < this.width && j < this.height && this.cells[j * this.width + i] === 1;
  }
}

/** 🧵️ The quadrilateral of a stroke segment of the given width, its ends extended by half the width. */
export function strokeQuad(a: Point, b: Point, width: number): Ring {
  const h = width / 2;
  const l = Math.hypot(b.x - a.x, b.y - a.y);
  const [ex, ey] = l < 1e-12 ? [h, 0] : [((b.x - a.x) / l) * h, ((b.y - a.y) / l) * h];
  const [nx, ny] = [-ey, ex];
  return [{ x: a.x - ex + nx, y: a.y - ey + ny }, { x: b.x + ex + nx, y: b.y + ey + ny }, { x: b.x + ex - nx, y: b.y + ey - ny }, { x: a.x - ex - nx, y: a.y - ey - ny }];
}

const RING: readonly (readonly [number, number])[] = [[0, 0], [1, 0], [-1, 0], [0, 1], [0, -1], [0.7, 0.7], [-0.7, 0.7], [0.7, -0.7], [-0.7, -0.7]];
const beside = (mask: Mask, x: number, y: number, radius: number): boolean => RING.some(([dx, dy]) => mask.has(x + dx! * radius, y + dy! * radius));

/** 🪡️ The member a stroke segment draws: at samples along the segment, the largest stripe centred on the stroke that the body fills (twice the shorter of the two distances to its boundary along the normal, measured on the first of `bodies` that holds the sample, the cut faces before the whole projection), the median of the samples, capped at {@link GATE.memberMaxPaperMm} of paper. A stroke on the boundary of a body is the edge of that body, not a member: it counts two cells wide, shifted into the body. One that lies within a pen width of a closed shape of the drawing (a seam or detail line on a body) is a line of the drawing and counts two cells wide; one that lies on no body keeps its pen width, so a line drawn where nothing is costs its ink. Returns the width and the signed shift along the normal of the segment. */
export function memberOf(a: Point, b: Point, bodies: readonly Mask[], drawn: Mask, penM: number, capM: number): { readonly width: number; readonly shift: number } {
  const l = Math.hypot(b.x - a.x, b.y - a.y);
  if (l < 1e-12) return { width: penM, shift: 0 };
  const [nx, ny] = [-(b.y - a.y) / l, (b.x - a.x) / l];
  const samples = Math.max(1, Math.min(9, Math.round(l / Math.max(bodies[0]!.cell * 2, penM))));
  const found: { width: number; shift: number }[] = [];
  for (let s = 0; s < samples; s++) {
    const t = (s + 0.5) / samples;
    const [x, y] = [a.x + (b.x - a.x) * t, a.y + (b.y - a.y) * t];
    const body = bodies.find((m) => beside(m, x, y, m.cell));
    if (body === undefined) {
      found.push({ width: penM, shift: 0 });
      continue;
    }
    if (beside(drawn, x, y, penM)) {
      found.push({ width: 2 * body.cell, shift: 0 });
      continue;
    }
    const steps = Math.ceil(capM / 2 / body.cell);
    let up = 0;
    while (up < steps && body.has(x + nx * (up + 1) * body.cell, y + ny * (up + 1) * body.cell)) up++;
    let down = 0;
    while (down < steps && body.has(x - nx * (down + 1) * body.cell, y - ny * (down + 1) * body.cell)) down++;
    if (Math.min(up, down) <= 1 && Math.max(up, down) > 2) found.push({ width: 2 * body.cell, shift: (up > down ? 1 : -1) * body.cell });
    else found.push({ width: Math.min((2 * Math.min(up, down) + 1) * body.cell, capM), shift: 0 });
  }
  found.sort((p, q) => p.width - q.width);
  return found[found.length >> 1]!;
}

/** 🫙️ Whether the projected volume (`groups`: the cut faces and everything that remains visible) fills at least half of the area an unfilled closed outline encloses (the outline of a solid part, drawn as its boundary) rather than less (a hoop or a frame around a void). Measured on a probe grid of the outline itself (about 200000 cells, from the groups as a whole, never from a tile or a raster of the measurement), so that the answer does not depend on the resolution, the tiling or the window. */
export function encloses(rings: Group, groups: readonly Group[]): boolean {
  let lo = Infinity, hi = -Infinity, left = Infinity, right = -Infinity;
  for (const ring of rings) for (const p of ring) {
    lo = Math.min(lo, p.y);
    hi = Math.max(hi, p.y);
    left = Math.min(left, p.x);
    right = Math.max(right, p.x);
  }
  const cell = Math.max(1e-4, Math.sqrt(((right - left) * (hi - lo)) / 200000));
  const box = { minX: left, minY: lo, maxX: right, maxY: hi };
  const probe = new Mask(box, cell);
  probe.fill(rings);
  const volume = new Mask(box, cell);
  for (const g of groups) {
    const b = boundsOf(g);
    if (b[1] >= left && b[0] <= right && b[3] >= lo && b[2] <= hi) volume.fill(g);
  }
  let inside = 0;
  let filled = 0;
  for (let k = 0; k < probe.cells.length; k++) {
    if (!probe.cells[k]) continue;
    inside++;
    if (volume.cells[k]) filled++;
  }
  return inside > 0 && filled >= inside / 2;
}

export type Measured = { readonly edge?: { readonly mask: Uint8Array; readonly width: number; readonly height: number; readonly minX: number; readonly minY: number; readonly cell: number }; readonly spread: Spread; readonly p95M: number; readonly p95SoftM: number; readonly softExtentM: number; readonly counts: { readonly both: number; readonly either: number; readonly drawn: number; readonly volume: number; readonly drawnCut: number }; readonly hausdorffM: number; readonly iou: number; readonly recall: number; readonly precision: number; readonly filled: readonly Group[]; readonly outlines: readonly Group[]; readonly reference: readonly Group[]; readonly visible: readonly Group[]; readonly full: readonly Group[]; readonly section: boolean; readonly box: Box; readonly cell: number; readonly openStrokes: number; readonly shapes: number };

type Evaluated = ReturnType<typeof evaluate>;

/** 📊️ The distances of the boundary cells of two masks to the boundary of the other, both directions: the maximum in metres and the histograms (cells of distance, the last bin collects everything farther) of the hard cells and of the soft cells (those near a soft mass such as foliage, fur or hair). */
export type Spread = { readonly maxM: number; readonly hard: readonly number[]; readonly soft: readonly number[]; readonly cellM: number };

/** 🔲️ The boundary cells of a mask: set cells with an unset 4-neighbour or on the edge of the grid. */
export function boundaryOf(mask: Uint8Array, width: number, height: number): Uint8Array {
  const out = new Uint8Array(mask.length);
  for (let j = 0; j < height; j++)
    for (let i = 0; i < width; i++) {
      const k = j * width + i;
      if (!mask[k]) continue;
      if (i === 0 || j === 0 || i === width - 1 || j === height - 1 || !mask[k - 1] || !mask[k + 1] || !mask[k - width] || !mask[k + width]) out[k] = 1;
    }
  return out;
}

let HOLE_SEEN = new Uint8Array(0);
let HOLE_QUEUE = new Int32Array(0);

/** 🕳️ A copy of the mask with every enclosed hole of at most `maxCells` cells filled (a gap between slats, a seam between overlapping shapes): such voids are not part of the outline of a silhouette. Holes open to the edge of the grid are the outside and stay: the outside is flooded once from the edge, so only the enclosed components are walked again. */
export function fillSmallHoles(mask: Uint8Array, width: number, height: number, maxCells: number): Uint8Array {
  const out = Uint8Array.from(mask);
  if (HOLE_SEEN.length < mask.length) [HOLE_SEEN, HOLE_QUEUE] = [new Uint8Array(mask.length), new Int32Array(mask.length)];
  const [seen, queue] = [HOLE_SEEN, HOLE_QUEUE];
  seen.fill(0, 0, mask.length);
  const flood = (head0: number, tail0: number): number => {
    let [head, tail] = [head0, tail0];
    while (head < tail) {
      const k = queue[head++]!;
      const i = k % width;
      if (i > 0 && !seen[k - 1] && !mask[k - 1]) { seen[k - 1] = 1; queue[tail++] = k - 1; }
      if (i < width - 1 && !seen[k + 1] && !mask[k + 1]) { seen[k + 1] = 1; queue[tail++] = k + 1; }
      if (k >= width && !seen[k - width] && !mask[k - width]) { seen[k - width] = 1; queue[tail++] = k - width; }
      if (k + width < mask.length && !seen[k + width] && !mask[k + width]) { seen[k + width] = 1; queue[tail++] = k + width; }
    }
    return tail;
  };
  let tail = 0;
  const edge = (k: number): void => {
    if (!mask[k] && !seen[k]) { seen[k] = 1; queue[tail++] = k; }
  };
  for (let i = 0; i < width; i++) {
    edge(i);
    edge((height - 1) * width + i);
  }
  for (let j = 0; j < height; j++) {
    edge(j * width);
    edge(j * width + width - 1);
  }
  flood(0, tail);
  for (let start = 0; start < mask.length; start++) {
    if (mask[start] || seen[start]) continue;
    seen[start] = 1;
    queue[0] = start;
    const size = flood(0, 1);
    if (size <= maxCells) for (let q = 0; q < size; q++) out[queue[q]!] = 1;
  }
  return out;
}

/** 🤝️ Both masks with their small enclosed holes filled by ONE decision per cell: a cell of an enclosed hole of at most `maxCells` cells of one mask is filled when the other mask is covered there or has such a hole there too (a seam between overlapping shapes, a gap between slats, a void both agree on); where the other mask is open (the outside, or a void too large to be a pocket) the cell stays empty in both, so the rim of that pocket is outline in both masks and is compared like every other outline. A pocket that one mask encloses and the other leaves open by a slit, and a hole one cell above the limit in one mask and at it in the other, are therefore never filled in one mask alone. */
export function agreedPockets(a: Uint8Array, b: Uint8Array, width: number, height: number, maxCells: number): readonly [Uint8Array, Uint8Array] {
  const [fa, fb] = [fillSmallHoles(a, width, height, maxCells), fillSmallHoles(b, width, height, maxCells)];
  const [oa, ob] = [Uint8Array.from(a), Uint8Array.from(b)];
  for (let k = 0; k < a.length; k++) {
    if (fa[k] && !a[k] && fb[k]) oa[k] = 1;
    if (fb[k] && !b[k] && fa[k]) ob[k] = 1;
  }
  return [oa, ob];
}

/** 📈️ The spread of the boundary distances of `a` and `b` over the cells of `core` (enclosed holes of at most `holeCells` cells that both masks agree on are filled first, {@link agreedPockets}: they are not outline); cells within three cells of `soft` count as soft; distances are limited to `bins - 1` cells. */
export function boundarySpread(a: Uint8Array, b: Uint8Array, soft: Uint8Array | undefined, width: number, height: number, cell: number, core: { readonly i0: number; readonly i1: number; readonly j0: number; readonly j1: number }, bins: number, holeCells = 64): Spread {
  const hard = new Array<number>(bins).fill(0);
  const softer = new Array<number>(bins).fill(0);
  let worst = 0;
  const holes = Math.max(1, Math.round(holeCells));
  const [fa, fb] = agreedPockets(a, b, width, height, holes);
  const [ba, bb] = [boundaryOf(fa, width, height), boundaryOf(fb, width, height)];
  const anyB = bb.some((x) => x === 1);
  const anyA = ba.some((x) => x === 1);
  const reach = Math.min(SEARCH_CELLS, bins - 1);
  for (const [from, to, has] of [[ba, bb, anyB], [bb, ba, anyA]] as const) {
    const far: number[] = [];
    const count = (k: number, d: number): void => {
      worst = Math.max(worst, d);
      const bin = Math.min(bins - 1, Math.round(d));
      if (soft !== undefined && nearWithin(soft, width, height, k, 9) !== Infinity) softer[bin]!++;
      else hard[bin]!++;
    };
    for (let j = core.j0; j < core.j1; j++)
      for (let i = core.i0; i < core.i1; i++) {
        const k = j * width + i;
        if (!from[k]) continue;
        const near = has ? nearWithin(to, width, height, k, reach * reach) : Infinity;
        if (near !== Infinity) count(k, Math.sqrt(near));
        else far.push(k);
      }
    if (far.length === 0) continue;
    const field = has ? distanceField(to, width, height) : undefined;
    for (const k of far) count(k, field === undefined ? bins - 1 : Math.sqrt(field[k]!));
  }
  return { maxM: worst * cell, hard, soft: softer, cellM: cell };
}

/** 📏️ The `fraction` quantile of a spread histogram in metres (the smallest distance below which that fraction of the cells lie, to the precision of one cell); 0 for no cells. */
export function quantileOf(hist: readonly number[], fraction: number, cell: number): number {
  const total = hist.reduce((sum, x) => sum + x, 0);
  if (total === 0) return 0;
  let run = 0;
  for (let k = 0; k < hist.length; k++) {
    run += hist[k]!;
    if (run >= fraction * total) return k * cell;
  }
  return (hist.length - 1) * cell;
}

/** 🪟️ The margin of a tile window in smallest features: a boundary distance beyond it counts as the margin, so it must exceed every limit of the rules on a boundary distance (at most four features for a silhouette) or a tiled measurement would pass what the whole raster fails. */
export const WINDOW_FEATURES = 6;

/** 🔎️ How far in cells {@link boundarySpread} searches around a boundary cell before it transforms the whole window for the cells that found nothing. */
export const SEARCH_CELLS = 8;

/** 🧱️ The most cells per side of one tile of a measurement; larger objects are measured in tiles with a margin (tile and margin never change a pass or fail). */
export const TILE_CELLS = 512;

/** 🧭️ The Hausdorff distance in metres of the cells of `core` (index ranges) of one mask to the other mask, and of the other mask's cells of the core to this one, both within the window of the masks; a distance beyond the margin of the window (the distance to its edge) counts as that margin, which is above every limit of the rules. */
export function hausdorffIn(a: Uint8Array, b: Uint8Array, width: number, height: number, cell: number, core: { readonly i0: number; readonly i1: number; readonly j0: number; readonly j1: number }, margin: number): number {
  let worst = 0;
  for (const [from, to] of [[a, b], [b, a]] as const) {
    let any = false;
    for (let j = core.j0; j < core.j1 && !any; j++) for (let i = core.i0; i < core.i1; i++) if (from[j * width + i]) { any = true; break; }
    if (!any) continue;
    let target = false;
    for (let i = 0; i < to.length; i++) if (to[i]) { target = true; break; }
    if (!target) return Math.max(worst, margin || cell * 1000);
    const field = distanceField(to, width, height);
    for (let j = core.j0; j < core.j1; j++) for (let i = core.i0; i < core.i1; i++) if (from[j * width + i] && field[j * width + i]! > worst) worst = field[j * width + i]!;
    worst = Math.min(worst, ((margin || Infinity) / cell) ** 2);
  }
  return Math.sqrt(worst) * cell;
}

/** 🧵️ A large object: the IoU, recall and precision come from one coarse raster (about 2000 cells across, whose area error is the boundary length times a cell, far below the limits), the boundary distances from fine tiles (a quarter feature) only where the coarse raster has a boundary within the margin: tiles without a boundary have no distance to measure, so skipping them changes nothing. */
function measureBanded(asset: Asset, state: Evaluated, solids: readonly World[], view: Viewpoint, scale: number, featureM: number | undefined, tileCells: number, frame: Box, cell: number): Measured {
  const coarse = measureView(asset, state, solids, view, scale, 2000, featureM, Infinity, true);
  const edge = coarse.edge!;
  const side = tileCells * cell;
  const margin = Math.ceil(((featureM ?? 10 * cell) * WINDOW_FEATURES) / cell) * cell;
  const [nx, ny] = [Math.ceil((frame.maxX - frame.minX) / side), Math.ceil((frame.maxY - frame.minY) / side)];
  let worst = 0;
  let hard: number[] | undefined;
  let softHist: number[] | undefined;
  const section = (asset.representations[String(scale) as unknown as keyof Asset["representations"]] as unknown as { readonly sections?: Partial<Record<Viewpoint, ViewSection>> }).sections?.[view];
  const prepared = referenceCached(view, solids, section, drawnFor(asset, state, view, scale));
  const queryReference = bucketIndex(prepared.cut, boundsOf, frame, mayTouch);
  const queryVisible = section === undefined ? undefined : bucketIndex(prepared.visible, boundsOf, frame, mayTouch);
  const queryFull = section === undefined ? undefined : bucketIndex(prepared.full, boundsOf, frame, mayTouch);
  const queryDrawn = bucketIndex(evaluatedOf(state, view), (shape) => boundsOf(shape.rings), frame);
  const hasEdge = (box: Box): boolean => {
    const [i0, i1] = [Math.max(0, Math.floor((box.minX - edge.minX) / edge.cell)), Math.min(edge.width - 1, Math.floor((box.maxX - edge.minX) / edge.cell))];
    const [j0, j1] = [Math.max(0, Math.floor((box.minY - edge.minY) / edge.cell)), Math.min(edge.height - 1, Math.floor((box.maxY - edge.minY) / edge.cell))];
    for (let j = j0; j <= j1; j++) for (let i = i0; i <= i1; i++) if (edge.mask[j * edge.width + i]) return true;
    return false;
  };
  for (let j = 0; j < ny; j++)
    for (let i = 0; i < nx; i++) {
      const [minX, minY] = [frame.minX + i * side, frame.minY + j * side];
      const core: Box = { minX, minY, maxX: minX + Math.ceil((Math.min(frame.maxX, minX + side) - minX) / cell) * cell, maxY: minY + Math.ceil((Math.min(frame.maxY, minY + side) - minY) / cell) * cell };
      const box: Box = { minX: core.minX - margin, minY: core.minY - margin, maxX: core.maxX + margin, maxY: core.maxY + margin };
      if (!hasEdge({ minX: core.minX - edge.cell, minY: core.minY - edge.cell, maxX: core.maxX + edge.cell, maxY: core.maxY + edge.cell })) continue;
      const reference = queryReference(box);
      const tile = measureView(asset, state, solids, view, scale, undefined, featureM, Infinity, false, { core, box, cell, prepared: { reference, visible: queryVisible?.(box) ?? reference, full: queryFull?.(box) ?? reference, evaluated: queryDrawn(box) } });
      worst = Math.max(worst, tile.hausdorffM);
      hard = hard === undefined ? [...tile.spread.hard] : hard.map((x, k) => x + (tile.spread.hard[k] ?? 0));
      softHist = softHist === undefined ? [...tile.spread.soft] : softHist.map((x, k) => x + (tile.spread.soft[k] ?? 0));
    }
  const spread = { maxM: worst, hard: hard ?? [], soft: softHist ?? [], cellM: cell };
  const { edge: _edge, ...rest } = coarse;
  return { ...rest, spread, p95M: quantileOf(spread.hard, 0.95, cell), p95SoftM: quantileOf(spread.soft, 0.95, cell), hausdorffM: worst };
}

type EvaluatedShape = { readonly shape: { readonly id: string; readonly bone?: string; readonly role: string; readonly penMm?: number; readonly filled?: boolean; readonly region?: string; readonly regions?: readonly string[] }; readonly rings: readonly Ring[]; readonly closed: boolean };

/** ✏️ The drawn shapes of one viewpoint that take part in the comparison (filled body, detail and glazing shapes, open strokes, unfilled outlines). */
function evaluatedOf(state: Evaluated, view: Viewpoint): readonly EvaluatedShape[] {
  const source = state.shapes[view] as readonly EvaluatedShape[];
  let hit = EVALUATED.get(source);
  if (hit === undefined) EVALUATED.set(source, (hit = source.filter((s) => ["body", "detail", "glazing"].includes(s.shape.role) && s.rings.length > 0)));
  return hit;
}

const EVALUATED = new WeakMap<object, readonly EvaluatedShape[]>();

/** ✏️ {@link Drawn} of one viewpoint of an evaluated pose at 1:`scale`. */
export function drawnFor(asset: Asset, state: Evaluated, view: Viewpoint, scale: number): Drawn {
  const rep = asset.representations[String(scale) as unknown as keyof Asset["representations"]] as unknown as { readonly pens: Record<string, number | null> };
  return drawnOf(evaluatedOf(state, view), scale, rep.pens);
}

/** 🔺️ Whether a group of one triangle (or any group, conservatively) can touch the box: the three edge half-planes must not all exclude the box. */
function mayTouch(g: Group, box: Box): boolean {
  if (g.length !== 1 || g[0]!.length !== 3) return true;
  const [a, b, c] = g[0] as readonly [Point, Point, Point];
  const area = (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);
  if (area === 0) return true;
  const sign = area > 0 ? 1 : -1;
  return !(excludes(a, b, sign, box) || excludes(b, c, sign, box) || excludes(c, a, sign, box));
}

/** 📐️ Whether all four corners of the box lie strictly on the outer side of the edge p-q of a triangle of orientation `sign`. */
function excludes(p: Point, q: Point, sign: number, box: Box): boolean {
  const [dx, dy] = [q.x - p.x, q.y - p.y];
  return sign * (dx * (box.minY - p.y) - dy * (box.minX - p.x)) < 0 && sign * (dx * (box.minY - p.y) - dy * (box.maxX - p.x)) < 0 && sign * (dx * (box.maxY - p.y) - dy * (box.maxX - p.x)) < 0 && sign * (dx * (box.maxY - p.y) - dy * (box.minX - p.x)) < 0;
}

/** 🧺️ A uniform grid of buckets over a frame that answers which items lie near a box (items are indexed by their bounding boxes). */
function bucketIndex<T>(items: readonly T[], boxOf: (item: T) => readonly [number, number, number, number], frame: Box, touch?: (item: T, box: Box) => boolean, n = 128): (box: Box) => readonly T[] {
  const [w, h] = [Math.max(frame.maxX - frame.minX, 1e-9), Math.max(frame.maxY - frame.minY, 1e-9)];
  const at = (v: number, lo: number, span: number): number => Math.max(0, Math.min(n - 1, Math.floor(((v - lo) / span) * n)));
  const buckets: number[][] = Array.from({ length: n * n }, () => []);
  items.forEach((item, k) => {
    const b = boxOf(item);
    if (!Number.isFinite(b[0])) return;
    for (let j = at(b[2], frame.minY, h); j <= at(b[3], frame.minY, h); j++) for (let i = at(b[0], frame.minX, w); i <= at(b[1], frame.minX, w); i++) buckets[j * n + i]!.push(k);
  });
  const stamp = new Int32Array(items.length);
  const boxes = items.map(boxOf);
  let current = 0;
  return (box) => {
    current++;
    const out: T[] = [];
    for (let j = at(box.minY, frame.minY, h); j <= at(box.maxY, frame.minY, h); j++)
      for (let i = at(box.minX, frame.minX, w); i <= at(box.maxX, frame.minX, w); i++)
        for (const k of buckets[j * n + i]!) {
          if (stamp[k] === current) continue;
          stamp[k] = current;
          const b = boxes[k]!;
          if (b[1] >= box.minX && b[0] <= box.maxX && b[3] >= box.minY && b[2] <= box.maxY && (touch === undefined || touch(items[k]!, box))) out.push(items[k]!);
        }
    return out;
  };
}

const REFERENCES = new WeakMap<readonly World[], WeakMap<object, Map<string, ReturnType<typeof referenceOf>>>>();
const SOFT = new WeakMap<readonly World[], readonly World[]>();
const NO_DRAWN: object = {};

const MESHES_PROJECTED = new WeakMap<readonly World[], Map<Viewpoint, readonly Group[]>>();

/** 🗃️ The projected triangles of the solids as they are (a slender solid at its true section, never widened to a pen), memoised per solid list and view: what an unfilled closed outline is asked about in {@link bodyOutline}. */
function meshProjected(view: Viewpoint, solids: readonly World[]): readonly Group[] {
  let byView = MESHES_PROJECTED.get(solids);
  if (byView === undefined) MESHES_PROJECTED.set(solids, (byView = new Map()));
  let hit = byView.get(view);
  if (hit === undefined) byView.set(view, (hit = referenceOf(view, solids, undefined, undefined, "mesh").cut));
  return hit;
}

/** 🫙️ Whether an unfilled closed outline is the edge of a body: the solids as they are fill at least half of what it encloses ({@link encloses}). Its centre line is then the body edge and the pen is drawing convention on both sides of the comparison: the drawing counts as filled up to the centre line, and a slender solid that runs along it (a rim wire, an edge band) keeps its true width instead of the pen. An outline around a void (a hoop, a frame) is geometry at pen width (C24). */
export function bodyOutline(view: Viewpoint, solids: readonly World[], rings: Group): boolean {
  return enclosesCached(rings, meshProjected(view, solids), solids);
}

/** 🖊️ The drawn shapes a slender solid may follow ({@link followedWidth}): all but the unfilled closed outlines that are the edge of a body ({@link bodyOutline}). */
export function strokeDrawings(view: Viewpoint, solids: readonly World[], drawn: Drawn): Drawn {
  if (!solids.some((solid) => solid.slender === true) || !drawn.some((d) => d.closed && !d.filled && d.penM > 0)) return drawn;
  return drawn.filter((d) => !(d.closed && !d.filled && bodyOutline(view, solids, d.rings)));
}

/** 🗃️ {@link referenceOf} memoised per solid list, drawing, view and section plane, so that the tiles of one measurement project the triangles once. */
function referenceCached(view: Viewpoint, solids: readonly World[], section: ViewSection | undefined, drawn?: Drawn): ReturnType<typeof referenceOf> {
  let byDrawn = REFERENCES.get(solids);
  if (byDrawn === undefined) REFERENCES.set(solids, (byDrawn = new WeakMap()));
  let byKey = byDrawn.get(drawn ?? NO_DRAWN);
  if (byKey === undefined) byDrawn.set(drawn ?? NO_DRAWN, (byKey = new Map()));
  const key = `${view}|${section?.plane ?? ""}|${section?.retained ?? ""}`;
  let hit = byKey.get(key);
  if (hit === undefined) byKey.set(key, (hit = referenceOf(view, solids, section, drawn)));
  return hit;
}

const softOf = (solids: readonly World[]): readonly World[] => {
  let hit = SOFT.get(solids);
  if (hit === undefined) SOFT.set(solids, (hit = solids.filter((w) => w.soft === true)));
  return hit;
};

const ENCLOSES = new WeakMap<object, WeakMap<object, boolean>>();

/** 🗃️ {@link encloses} memoised per outline and per projected volume, so that every tile of a measurement and every resolution asks the same question once and gets the same answer. */
function enclosesCached(rings: Group, groups: readonly Group[], key: object): boolean {
  let byGroups = ENCLOSES.get(rings);
  if (byGroups === undefined) ENCLOSES.set(rings, (byGroups = new WeakMap()));
  let hit = byGroups.get(key);
  if (hit === undefined) byGroups.set(key, (hit = encloses(rings, groups)));
  return hit;
}
/** 🔎️ How many cells the raster of a rule comparison has at least along the long side of what it compares: an object of a few features (a valve of 60 mm at 1:50, most assets at 1:500) is otherwise drawn with a few dozen cells, and the cells along its outline, half of each in doubt, weigh several percent of its area. */
export const SMALL_OBJECT_CELLS = 256;

/** 📏️ The cell of the raster on which a rule compares a drawing with a volume: a quarter of the smallest feature, finer for an object whose long side `longM` would have fewer than {@link SMALL_OBJECT_CELLS} cells, never below 0.05 mm. The limits of the rules stay in smallest features; only the error of the raster shrinks with the object. */
export const comparisonCell = (featureM: number, longM: number): number => Math.max(5e-5, Math.min(featureM / 4, Number.isFinite(longM) && longM > 0 ? longM / SMALL_OBJECT_CELLS : Infinity));

/** 🔬️ Compares one view of an evaluated pose with the reference of the volume on one raster of `cells` cells across the long side (by default 800, more where a cell would be coarser than 20 mm, at most 4000). */
export function measureView(asset: Asset, state: Evaluated, solids: readonly World[], view: Viewpoint, scale: number, cells?: number, featureM?: number, tileCells: number = TILE_CELLS, wantEdge = false, window?: { readonly core: Box; readonly box: Box; readonly cell: number; readonly prepared?: { readonly reference: readonly Group[]; readonly visible: readonly Group[]; readonly full: readonly Group[]; readonly evaluated: readonly EvaluatedShape[] } }): Measured {
  const rep = asset.representations[String(scale) as unknown as keyof Asset["representations"]] as unknown as { readonly pens: Record<string, number | null>; readonly sections?: Partial<Record<Viewpoint, ViewSection>> };
  const section = rep.sections?.[view];
  const { cut: reference, visible, full } = window?.prepared ? { cut: window.prepared.reference, visible: window.prepared.visible, full: window.prepared.full } : referenceCached(view, solids, section, drawnFor(asset, state, view, scale));
  const evaluated = window?.prepared?.evaluated ?? evaluatedOf(state, view);
  const bounds = { minX: Infinity, minY: Infinity, maxX: -Infinity, maxY: -Infinity };
  const grow = (p: Point): void => {
    if (p.x < bounds.minX) bounds.minX = p.x;
    if (p.x > bounds.maxX) bounds.maxX = p.x;
    if (p.y < bounds.minY) bounds.minY = p.y;
    if (p.y > bounds.maxY) bounds.maxY = p.y;
  };
  const growBox = (b: readonly [number, number, number, number]): void => {
    if (b[0] < bounds.minX) bounds.minX = b[0];
    if (b[1] > bounds.maxX) bounds.maxX = b[1];
    if (b[2] < bounds.minY) bounds.minY = b[2];
    if (b[3] > bounds.maxY) bounds.maxY = b[3];
  };
  if (window === undefined) {
    for (const g of full) growBox(boundsOf(g));
    for (const g of reference) growBox(boundsOf(g));
    for (const s of evaluated) growBox(boundsOf(s.rings));
  }
  void grow;
  const long = Math.max(bounds.maxX - bounds.minX, bounds.maxY - bounds.minY);
  const cell = window?.cell ?? (featureM !== undefined && cells === undefined ? comparisonCell(featureM, long) : Math.max(2e-4, long / (cells ?? Math.min(GATE.cellsMax, Math.max(GATE.cells, Math.round(long / GATE.cellM))))));
  const frame: Box = { minX: bounds.minX - 2 * cell, minY: bounds.minY - 2 * cell, maxX: bounds.maxX + 2 * cell, maxY: bounds.maxY + 2 * cell };
  if (window === undefined && Math.max(frame.maxX - frame.minX, frame.maxY - frame.minY) / cell > tileCells) return measureBanded(asset, state, solids, view, scale, featureM, tileCells, frame, cell);
  const box: Box = window?.box ?? frame;
  const volume = new Mask(box, cell);
  for (const g of reference) volume.fill(g, true);
  volume.seal();
  const upper = section === undefined ? volume : new Mask(box, cell);
  const whole = section === undefined ? volume : new Mask(box, cell);
  if (section !== undefined) {
    for (const g of visible) upper.fill(g, true);
    for (const g of full) whole.fill(g, true);
    upper.seal();
    whole.seal();
  }
  const fine = volume;
  const pen = (s: (typeof evaluated)[number]): number => ((s.shape.penMm ?? rep.pens[s.shape.role] ?? 0) * scale) / 1000;
  const solid = new Map<(typeof evaluated)[number], boolean>();
  const fills = (s: (typeof evaluated)[number]): boolean => {
    if (!s.closed) return false;
    if (s.shape.filled !== false) return true;
    let known = solid.get(s);
    if (known === undefined) {
      if (section === undefined) known = bodyOutline(view, solids, s.rings);
      else {
        const whole = referenceCached(view, solids, section, drawnFor(asset, state, view, scale));
        known = enclosesCached(s.rings, [...whole.cut, ...whole.visible], whole.cut);
      }
      solid.set(s, known);
    }
    return known;
  };
  const strokes = evaluated.filter((s) => !fills(s));
  const filled: Group[] = [];
  const outlines: Group[] = [];
  const drawn = new Mask(box, fine.cell);
  for (const s of evaluated) if (fills(s)) drawn.fill(s.rings);
  for (const s of evaluated) {
    if (fills(s)) filled.push(s.rings);
    else for (const ring of s.rings) {
      const r = s.closed ? [...ring, ring[0]!] : ring;
      for (let i = 0; i + 1 < r.length; i++) {
        const [p, q] = [r[i]!, r[i + 1]!];
        const [width, shift] = [Math.max(pen(s), 1e-4), 0];
        const l = Math.hypot(q.x - p.x, q.y - p.y) || 1;
        const [dx, dy] = [(-(q.y - p.y) / l) * shift, ((q.x - p.x) / l) * shift];
        outlines.push([strokeQuad({ x: p.x + dx, y: p.y + dy }, { x: q.x + dx, y: q.y + dy }, width)]);
      }
    }
  }
  const ink = new Mask(box, cell);
  const area = new Mask(box, cell);
  for (const g of filled) area.fill(g);
  area.sealRings(filled);
  for (const g of outlines) ink.fill(g);
  let both = 0;
  let either = 0;
  let a = 0;
  let v = 0;
  let drawnCut = 0;
  const core = window === undefined ? { i0: 0, i1: area.width, j0: 0, j1: area.height } : { i0: Math.round((window.core.minX - box.minX) / cell), i1: Math.round((window.core.maxX - box.minX) / cell), j0: Math.round((window.core.minY - box.minY) / cell), j1: Math.round((window.core.maxY - box.minY) / cell) };
  for (let j = core.j0; j < core.j1; j++)
    for (let i = core.i0; i < core.i1; i++) {
      const k = j * area.width + i;
      const [f, o, y, u, w] = [area.cells[k]!, ink.cells[k]!, volume.cells[k]!, upper.cells[k]!, whole.cells[k]!];
      const x = f | o;
      a += x;
      v += y;
      drawnCut += x & y;
      both += (f & u) | (o & w);
      either += x | y;
    }
  const softWorlds = softOf(solids);
  const exchange = new Set(asset.thermal.regions.filter((r) => r.areaBasis === "exchange").map((r) => r.id));
  const softDrawn = evaluated.filter((s) => (s.shape.region !== undefined && exchange.has(s.shape.region)) || (s.shape.regions ?? []).some((r) => exchange.has(r))).map((s) => s.rings as Group);
  let softMask: Mask | undefined;
  let softExtentM = 0;
  if (softWorlds.length > 0 || softDrawn.length > 0) {
    softMask = new Mask(box, cell);
    const projected = softWorlds.length > 0 ? referenceCached(view, softWorlds, undefined, drawnFor(asset, state, view, scale)).cut : [];
    let minX = Infinity, maxX = -Infinity, minY = Infinity, maxY = -Infinity;
    for (const groups of [projected, softDrawn]) for (const g of groups) {
      softMask.fill(g);
      const b = boundsOf(g);
      minX = Math.min(minX, b[0]);
      maxX = Math.max(maxX, b[1]);
      minY = Math.min(minY, b[2]);
      maxY = Math.max(maxY, b[3]);
    }
    softExtentM = Math.max(0, maxX - minX, maxY - minY);
  }
  const drawnMask = new Uint8Array(area.cells.length);
  for (let i = 0; i < drawnMask.length; i++) drawnMask[i] = area.cells[i]! | ink.cells[i]!;
  const bins = window === undefined ? Math.ceil(Math.hypot(volume.width, volume.height)) + 2 : Math.ceil((window.core.minX - box.minX) / cell) + 2;
  const reference3d = section === undefined ? volume.cells : volume.cells.map((x, i) => x | upper.cells[i]!);
  const spread = boundarySpread(drawnMask, reference3d, softMask?.cells, volume.width, volume.height, cell, core, bins, featureM === undefined ? 64 : (4 * featureM / cell) ** 2);
  const hausdorffM = spread.maxM;
  const edgeMask = wantEdge ? (() => { const [x, y] = [boundaryOf(drawnMask, volume.width, volume.height), boundaryOf(reference3d, volume.width, volume.height)]; return x.map((v, k) => v | y[k]!); })() : undefined;
  const edge = edgeMask !== undefined ? { mask: edgeMask, width: volume.width, height: volume.height, minX: box.minX, minY: box.minY, cell } : undefined;
  return { ...(edge === undefined ? {} : { edge }), spread, p95M: quantileOf(spread.hard, 0.95, cell), p95SoftM: quantileOf(spread.soft, 0.95, cell), softExtentM, counts: { both, either, drawn: a, volume: v, drawnCut }, hausdorffM, iou: either === 0 ? 1 : both / either, recall: a === 0 ? 1 : both / a, precision: v === 0 ? 1 : drawnCut / v, filled, outlines, reference, visible, full, section: section !== undefined, box, cell, openStrokes: strokes.length, shapes: evaluated.length };
}

/** 📐️ Computes the rule measurements for a single viewpoint. Projected triangles and tile caches belong to this view and can be released as soon as its scalar measurements are returned. */
export function measureBodyView(asset: Asset, state: Evaluated, body: readonly World[], view: Viewpoint, scale: number, featureM: number, softIds: ReadonlySet<string>): Pick<Measured, "iou" | "p95M" | "p95SoftM" | "softExtentM" | "hausdorffM"> {
  const solids = body.filter(w => w.texture !== true).map(w => softIds.has(w.id) ? { ...w, soft: true } : w);
  const { iou, p95M, p95SoftM, softExtentM, hausdorffM } = measureView(asset, state, solids, view, scale, undefined, featureM);
  return { iou, p95M, p95SoftM, softExtentM, hausdorffM };
}

/** 📏️ Width, height and depth of the physical body in metres. */
export function bodyDimensions(worlds: readonly World[]): readonly [number, number, number] {
  const e = extentOf(worlds.filter((solid) => solid.texture !== true));
  return [e.max[0]! - e.min[0]!, e.max[1]! - e.min[1]!, e.max[2]! - e.min[2]!];
}

export type AssetMeasure = { readonly asset: string; readonly pose: string; readonly bbox: readonly [number, number, number]; readonly bboxError: readonly [number, number, number]; readonly views: Record<Viewpoint, Measured>; readonly sections: Partial<Record<Viewpoint, ViewSection>> };

/** 🧪️ The bounding box against `dimensions` and the three views of an asset in a pose at 1:`scale` for the body of `volumes[scale]`; `featureM` refines the raster to half the smallest feature so that the Hausdorff distance resolves it. */
export function measureAsset(asset: Asset, scale: ScaleDenominator | number = GATE.scale, pose: string = asset.presentation.defaultPose, cells?: number, featureM?: number, softIds: ReadonlySet<string> = new Set()): AssetMeasure {
  const state = evaluate(asset, { kind: "pose", pose }, { kind: "scale", scale: { denominator: scale } });
  const solids = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true).map((w) => (softIds.has(w.id) ? { ...w, soft: true } : w));
  const d = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
  const bbox = bodyDimensions(solids);
  const rep = asset.representations[String(scale) as unknown as keyof Asset["representations"]] as unknown as { readonly sections?: Partial<Record<Viewpoint, ViewSection>> };
  const views = Object.fromEntries(VIEWPOINTS.map((v) => [v, measureView(asset, state, solids, v, scale, cells, featureM)])) as Record<Viewpoint, Measured>;
  return { asset: asset.id, pose, bbox, bboxError: bbox.map((b, k) => (b - d[k]!) / d[k]!) as [number, number, number], views, sections: rep.sections ?? {} };
}
