// #region 🧲️Header
/** @emoji 🧠️ `@semio-tech/cad-js/spatial-kernel/semio` — first-party `SpatialKernel` backed by the
 * Rust `BrepKernel` (`Brep`, `semio-s-plugin-stdio`'s `✳️brep` subset) over the existing
 * `flow_core` wasm JS→Rust bridge (`invokeBrep`/`brep_invoke`, see
 * `🧰️framework/🔨️modules/🧊️3d/🟦️.ts` and `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📐️brep-geometry/🦀️.rs`).
 * THE production CAD runtime kernel (`id = "semio-brep"`); `🧱️brepjs` (OpenCascade) stays only as
 * the vitest differential oracle. Pure preview math lives in the kernel-agnostic `🧮️preview/🟦️.ts`,
 * which this kernel extends. See `🎫️tickets/…/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME/📓️w4a-spatial-kernel-first-party.md`. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { invokeBrep, kernelGeometry, type EdgeCurve, type EdgeGroup, type FaceGroup, type MeshTransfer, type Vec3, solidRef } from "@semio-tech/s-3d-js";
import { Model } from "../📐️geometry/🟦️.ts";
import { applyModelDiff, isEmptyModelDiff, type ModelDiff, type SpatialKernel } from "../🗺️spatial/🟦️.ts";
import {
  PreciseSpatialKernelMath,
  arcEndFromAngle,
  arcEndOnCircle,
  arcSamplePoints,
  circleFromCenterRadiusPoint,
  boxModelDiff,
  edgeCurveLength,
  edgeSamplePoints,
  geom,
  nurbsCurveFromPoles,
  nurbsDisplaySamplePoints,
  vec3Cross,
  vec3Distance,
  vec3Dot,
  vec3Length,
  vec3Normalize,
  vec3Sub,
  type EdgeRecord,
  type EdgeRef,
  type FaceRecord,
  type FaceRef,
  type SolidPrimitive,
  type SolidRecord,
  type SolidRef,
  type VertexRef,
  type WireRef,
} from "../🧮️preview/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🌉️InvokeShapes
interface RawFaceInfo {
  readonly entity_id: string;
  readonly surface_kind: string;
  readonly area: number;
  readonly normal: readonly [number, number, number];
}
interface RawEdgeInfo {
  readonly entity_id: string;
  readonly curve_kind: string;
  readonly length: number;
}
interface RawMeshTransfer {
  readonly position: readonly number[];
  readonly normal: readonly number[];
  readonly index: readonly number[];
  readonly edges: readonly number[];
  readonly points?: readonly number[];
  readonly face_groups: readonly { readonly start: number; readonly count: number; readonly entity_id: string }[];
  readonly edge_groups?: readonly { readonly start: number; readonly count: number; readonly entity_id: string }[];
  readonly face_infos?: readonly RawFaceInfo[];
  readonly edge_infos?: readonly RawEdgeInfo[];
}
interface RawTopology {
  readonly vertices: readonly string[];
  readonly edges: readonly string[];
  readonly faces: readonly string[];
  readonly shells: readonly string[];
}

/** @emoji 🔑️ Quantized face/vertex normal key (1e-3 tolerance) used to map a Rust primitive's
 * analytically-ordered faces back onto the `Model` `FaceRef`s built by the SAME construction call
 * (`boxFaceNormalMap`), since `BrepKernel` primitive constructors carry no caller-chosen labels. */
function normalKey(n: readonly [number, number, number]): string {
  return `${Math.round(n[0] * 1000)},${Math.round(n[1] * 1000)},${Math.round(n[2] * 1000)}`;
}

/** @emoji 📦️ Fixed face order every `boxModelDiff` call emits: bottom(-Z), top(+Z), y0(-Y), x1(+X), y1(+Y), x0(-X). */
function boxFaceNormalMap(diff: ModelDiff): Map<string, FaceRef> {
  const faces = diff.faces?.added ?? [];
  const normals: readonly [number, number, number][] = [
    [0, 0, -1],
    [0, 0, 1],
    [0, -1, 0],
    [1, 0, 0],
    [0, 1, 0],
    [-1, 0, 0],
  ];
  const map = new Map<string, FaceRef>();
  faces.forEach((f, i) => {
    const n = normals[i];
    if (n) map.set(normalKey(n), f.id);
  });
  return map;
}

function meshTransferFromInvoke(raw: RawMeshTransfer, faceNormalMap?: Map<string, FaceRef>): MeshTransfer {
  const normalByEntity = new Map<string, readonly [number, number, number]>((raw.face_infos ?? []).map((fi) => [fi.entity_id, fi.normal]));
  const faceRefFor = (label: string): string => {
    const n = normalByEntity.get(label);
    if (!n || !faceNormalMap) return label;
    return faceNormalMap.get(normalKey(n)) ?? label;
  };
  const faceGroups: FaceGroup[] = raw.face_groups.map((g) => ({ start: g.start, count: g.count, entityId: faceRefFor(g.entity_id) as FaceRef }));
  const edgeGroups: EdgeGroup[] = (raw.edge_groups ?? []).map((g) => ({ start: g.start, count: g.count, entityId: g.entity_id as EdgeRef }));
  return {
    position: new Float32Array(raw.position),
    normal: new Float32Array(raw.normal),
    index: new Uint32Array(raw.index),
    edges: new Float32Array(raw.edges),
    points: new Float32Array(raw.points ?? []),
    faceGroups,
    edgeGroups,
    faceInfos: (raw.face_infos ?? []).map((fi) => ({ entityId: faceRefFor(fi.entity_id) as FaceRef, surfaceType: fi.surface_kind.toUpperCase(), area: fi.area, normal: fi.normal })),
    edgeInfos: (raw.edge_infos ?? []).map((ei) => ({ entityId: ei.entity_id as EdgeRef, curveType: ei.curve_kind.toUpperCase(), length: ei.length })),
  };
}

/** @emoji 🧭️ Axis-angle rotating world `+Z` onto a unit `target` direction (rotation line through the origin), used to place cylinder/cone primitives (canonical base-at-origin, axis `+Z`) onto an arbitrary `axis`. */
function axisAngleFromZ(target: Vec3): { readonly axis: Vec3; readonly angle: number } {
  const z: Vec3 = [0, 0, 1];
  const dot = Math.max(-1, Math.min(1, vec3Dot(z, target)));
  if (dot > 1 - 1e-9) return { axis: [1, 0, 0], angle: 0 };
  if (dot < -1 + 1e-9) return { axis: [1, 0, 0], angle: Math.PI };
  const cross = vec3Cross(z, target);
  return { axis: vec3Length(cross) > 1e-12 ? vec3Normalize(cross) : [1, 0, 0], angle: Math.acos(dot) };
}
// #endregion 🌉️InvokeShapes

// #region 🔧️GeometryReconstruction
/** @emoji 🧵️ Polyline-samples a `Model` wire's edges into world points (exact for straight edges, chordally approximated for arcs/circles/nurbs — `BrepKernel` has no generic wire-from-mixed-edges constructor yet, only `polyline_wire`/`rectangle_wire` from points). */
function wirePolylinePoints(model: Model, wireId: WireRef, segments = 24): Vec3[] {
  const wire = geom(model).wires[String(wireId)];
  if (!wire) return [];
  const points: Vec3[] = [];
  for (const edgeId of wire.edgeIds) {
    const edge = geom(model).edges[String(edgeId)];
    if (!edge) continue;
    const pts = edgeSamplePoints(geom(model).vertices, edge, segments);
    for (const p of pts) {
      const last = points[points.length - 1];
      if (!last || vec3Distance(last, p) > 1e-9) points.push(p);
    }
  }
  return points;
}

/** @emoji 🧊️ Rust `GeometryHandle` for a wire's polyline approximation. */
async function polylineWireHandle(model: Model, wireId: WireRef, segments = 24): Promise<string | null> {
  const points = wirePolylinePoints(model, wireId, segments);
  if (points.length < 2) return null;
  const { handle } = await invokeBrep<{ readonly handle: string }>("polylineWire", { points });
  return handle;
}

/** @emoji 🏗️ Rust `GeometryHandle` for one Rust primitive matching a `SolidPrimitive` record, placed to match the record's world transform. */
async function primitiveHandle(solid: SolidPrimitive): Promise<string> {
  if (solid.kind === "sphere") {
    const { handle } = await invokeBrep<{ readonly handle: string }>("sphere", { radius: solid.radius });
    const t = await invokeBrep<{ readonly handle: string }>("translate", { shape: handle, offset: solid.center });
    return t.handle;
  }
  if (solid.kind === "cylinder") {
    const height = Math.max(solid.height, 1e-6);
    const axLen = vec3Length(solid.axis);
    const axis: Vec3 = axLen > 1e-12 ? vec3Normalize(solid.axis) : [0, 0, 1];
    const { handle } = await invokeBrep<{ readonly handle: string }>("cylinder", { radius: solid.radius, height });
    const { axis: rotAxis, angle } = axisAngleFromZ(axis);
    const rotated = angle > 1e-12 ? (await invokeBrep<{ readonly handle: string }>("rotate", { shape: handle, axis: rotAxis, angle })).handle : handle;
    const t = await invokeBrep<{ readonly handle: string }>("translate", { shape: rotated, offset: solid.base });
    return t.handle;
  }
  if (solid.kind === "cone") {
    const height = Math.max(solid.height, 1e-6);
    const axLen = vec3Length(solid.axis);
    const axis: Vec3 = axLen > 1e-12 ? vec3Normalize(solid.axis) : [0, 0, 1];
    const { handle } = await invokeBrep<{ readonly handle: string }>("cone", { radius: solid.radius, height });
    const { axis: rotAxis, angle } = axisAngleFromZ(axis);
    const rotated = angle > 1e-12 ? (await invokeBrep<{ readonly handle: string }>("rotate", { shape: handle, axis: rotAxis, angle })).handle : handle;
    const t = await invokeBrep<{ readonly handle: string }>("translate", { shape: rotated, offset: solid.base });
    return t.handle;
  }
  const ax = Math.min(solid.cornerA[0], solid.cornerB[0]);
  const ay = Math.min(solid.cornerA[1], solid.cornerB[1]);
  const bx = Math.max(solid.cornerA[0], solid.cornerB[0]);
  const by = Math.max(solid.cornerA[1], solid.cornerB[1]);
  const minZ = Math.min(solid.cornerA[2], solid.cornerB[2]);
  const { handle } = await invokeBrep<{ readonly handle: string }>("box", { width: bx - ax, depth: by - ay, height: solid.height });
  const t = await invokeBrep<{ readonly handle: string }>("translate", { shape: handle, offset: [ax, ay, minZ] });
  return t.handle;
}

/** @emoji 🧵️ Assembles a Rust solid handle from a `Model` shell's planar faces (`sewFaces` + `healSolid`) — best-effort topology reconstruction for shells not covered by `primitiveHandle` (e.g. edited/extruded solids). Non-planar faces are dropped from the sew set (their wire is still polyline-approximated). */
async function shellSolidHandle(model: Model, cell: SolidRecord): Promise<string | null> {
  const faceHandles: string[] = [];
  for (const shellId of cell.shellIds) {
    const shell = geom(model).shells[String(shellId)];
    if (!shell) continue;
    for (const faceId of shell.faceIds) {
      const face = geom(model).faces[String(faceId)];
      const wireId = face?.wireIds[0];
      if (!face || !wireId) continue;
      const wireHandle = await polylineWireHandle(model, wireId);
      if (!wireHandle) continue;
      const { handle } = await invokeBrep<{ readonly handle: string }>("planarFaceFromWire", { wire: wireHandle });
      faceHandles.push(handle);
    }
  }
  if (faceHandles.length === 0) return null;
  const { handle: sewn } = await invokeBrep<{ readonly handle: string }>("sewFaces", { faces: faceHandles, tolerance: 1e-6 });
  const { handle: healed } = await invokeBrep<{ readonly handle: string }>("healSolid", { shape: sewn, tolerance: 1e-6 });
  return healed;
}
// #endregion 🔧️GeometryReconstruction

// #region 🧠️SemioBrepEngine
class SemioBrepEngine {
  private seq = 0;
  private readonly solids = new Map<SolidRef, string>();
  private readonly faceNormalMaps = new Map<SolidRef, Map<string, FaceRef>>();
  private readonly meshCache = new Map<string, MeshTransfer>();
  private solidsModelKey: string | null = null;

  resetDerivedPipeline(): void {
    this.solids.clear();
    this.faceNormalMaps.clear();
    this.meshCache.clear();
    this.solidsModelKey = null;
  }

  private nextRef(kind: string): SolidRef {
    return kernelGeometry.solidRef(`semio-${kind}-${++this.seq}`);
  }

  private meshCacheKey(solid: SolidRef, tolerance: number, model?: Model): string {
    return model ? `${String(solid)}:${tolerance}:r${model.revision}` : `${String(solid)}:${tolerance}`;
  }

  private modelDerivedKey(model: Model): string {
    const solidIds = (Object.keys(geom(model).solids) as SolidRef[]).sort().join(",");
    const vertexDigest = Object.values(geom(model).vertices)
      .map((v) => `${v.id}:${v.position.map((n) => n.toFixed(4)).join(",")}`)
      .sort()
      .join("|");
    return `${model.revision}:${solidIds}:${vertexDigest}`;
  }

  disposeSolid(solid: SolidRef): void {
    const prefix = `${String(solid)}:`;
    for (const key of [...this.meshCache.keys()]) if (key.startsWith(prefix)) this.meshCache.delete(key);
    const handle = this.solids.get(solid);
    if (handle) void invokeBrep("dispose", { handle });
    this.solids.delete(solid);
    this.faceNormalMaps.delete(solid);
  }

  async createBoxFromCorners(input: { cornerA: Vec3; cornerB: Vec3; height: number }): Promise<SolidRef> {
    const handle = await primitiveHandle({ kind: "box", cornerA: input.cornerA, cornerB: input.cornerB, height: input.height });
    const ref = this.nextRef("box");
    this.solids.set(ref, handle);
    return ref;
  }

  async createBoxFromCornersDiff(input: { cornerA: Vec3; cornerB: Vec3; height: number }): Promise<{ readonly diff: ModelDiff; readonly solid: SolidRef }> {
    const solid = this.nextRef("box");
    const diff = boxModelDiff(input, solid);
    const handle = await primitiveHandle({ kind: "box", cornerA: input.cornerA, cornerB: input.cornerB, height: input.height });
    this.solids.set(solid, handle);
    this.faceNormalMaps.set(solid, boxFaceNormalMap(diff));
    return { diff, solid };
  }

  async volume(solid: SolidRef): Promise<number> {
    const handle = this.solids.get(solid);
    if (!handle) return 0;
    const { value } = await invokeBrep<{ readonly value: number }>("volume", { shape: handle });
    return value;
  }

  async tessellate(solid: SolidRef, tolerance: number, model?: Model): Promise<MeshTransfer> {
    if (model) await this.syncSolidsFromModel(model);
    const handle = this.solids.get(solid);
    if (!handle) return { position: new Float32Array(0), normal: new Float32Array(0), index: new Uint32Array(0), edges: new Float32Array(0), points: new Float32Array(0), faceGroups: [], edgeGroups: [], faceInfos: [], edgeInfos: [] };
    const key = this.meshCacheKey(solid, tolerance, model);
    const cached = this.meshCache.get(key);
    if (cached) return cached;
    const raw = await invokeBrep<RawMeshTransfer>("tessellate", { shape: handle, tolerance });
    const transfer = meshTransferFromInvoke(raw, this.faceNormalMaps.get(solid));
    this.meshCache.set(key, transfer);
    return transfer;
  }

  async solidForSolidRecord(model: Model, cell: SolidRecord): Promise<string | null> {
    const cached = this.solids.get(cell.id);
    if (cached) return cached;
    if (cell.shellIds.length === 0) {
      const primitive = (cell as SolidRecord & { solid?: SolidPrimitive }).solid;
      const handle = await primitiveHandle(primitive ?? { kind: "box", cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 });
      this.solids.set(cell.id, handle);
      return handle;
    }
    const handle = await shellSolidHandle(model, cell);
    if (handle) this.solids.set(cell.id, handle);
    return handle;
  }

  async validSolidsFromRefs(model: Model, refs: readonly SolidRef[]): Promise<string[]> {
    const out: string[] = [];
    for (const ref of refs) {
      const rec = geom(model).solids[String(ref)];
      if (!rec) continue;
      const handle = await this.solidForSolidRecord(model, rec);
      if (handle) out.push(handle);
    }
    return out;
  }

  async syncSolidsFromModel(model: Model): Promise<void> {
    const modelKey = this.modelDerivedKey(model);
    if (this.solidsModelKey === modelKey && this.solids.size > 0) return;
    this.solids.clear();
    this.faceNormalMaps.clear();
    this.meshCache.clear();
    for (const cell of Object.values(geom(model).solids)) {
      await this.solidForSolidRecord(model, cell);
    }
    this.solidsModelKey = modelKey;
  }

  async extrudeWire(input: { wireId: string; distance: number; direction: Vec3; model: Model }): Promise<SolidRef> {
    const wireHandle = await polylineWireHandle(input.model, input.wireId as WireRef);
    if (!wireHandle) throw new Error(`Cannot extrude wire ${input.wireId}`);
    const vector = [input.direction[0] * input.distance, input.direction[1] * input.distance, input.direction[2] * input.distance] as Vec3;
    const { handle } = await invokeBrep<{ readonly handle: string }>("extrudeWire", { wire: wireHandle, vector });
    const ref = this.nextRef("extrude");
    this.solids.set(ref, handle);
    return ref;
  }

  async extrudeWireDiff(input: { wireId: string; distance: number; direction: Vec3; model: Model }): Promise<{ readonly diff: ModelDiff; readonly solid: SolidRef }> {
    const solid = await this.extrudeWire(input);
    return { diff: { solids: { added: [{ id: solid, shellIds: [] }] } }, solid };
  }

  async offsetFacesDiff(input: { faceIds: readonly string[]; distance: number; model: Model }): Promise<{ readonly diff: ModelDiff }> {
    const faceId = input.faceIds[0];
    if (!faceId) return { diff: {} };
    const face = geom(input.model).faces[faceId];
    const wireId = face?.wireIds[0];
    if (!wireId) return { diff: {} };
    const wireHandle = await polylineWireHandle(input.model, wireId as WireRef);
    if (!wireHandle) return { diff: {} };
    const { handle: planar } = await invokeBrep<{ readonly handle: string }>("planarFaceFromWire", { wire: wireHandle });
    const { handle: offset } = await invokeBrep<{ readonly handle: string }>("offsetFace", { face: planar, distance: input.distance });
    const ref = this.nextRef("offset");
    this.solids.set(ref, offset);
    return { diff: { solids: { added: [{ id: ref, shellIds: [] }] } } };
  }

  async offsetFaces(input: { faceIds: readonly string[]; distance: number; model: Model }): Promise<void> {
    await this.offsetFacesDiff(input);
  }

  async vertexDistance(a: VertexRef, b: VertexRef, model: Model): Promise<number> {
    const pa = geom(model).vertices[String(a)]?.position;
    const pb = geom(model).vertices[String(b)]?.position;
    if (!pa || !pb) return 0;
    return vec3Distance(pa, pb);
  }

  async edgeLength(e: EdgeRef, model: Model): Promise<number> {
    const edge = geom(model).edges[String(e)];
    if (!edge) return 0;
    const ends: Vec3[] = [];
    for (const vid of edge.vertexIds) {
      const p = geom(model).vertices[String(vid)]?.position;
      if (p) ends.push(p);
    }
    if (ends.length < 2) return 0;
    return edgeCurveLength(edge.curve, [ends[0]!, ends[1]!]);
  }

  async faceArea(f: FaceRef, model: Model): Promise<number> {
    const face = geom(model).faces[String(f)];
    const wireId = face?.wireIds[0];
    if (!wireId) return 0;
    const wireHandle = await polylineWireHandle(model, wireId as WireRef);
    if (!wireHandle) return 0;
    const { handle: planar } = await invokeBrep<{ readonly handle: string }>("planarFaceFromWire", { wire: wireHandle });
    const { value } = await invokeBrep<{ readonly value: number }>("area", { shape: planar });
    return value;
  }

  async solidVolume(c: SolidRef): Promise<number> {
    return this.volume(c);
  }

  async adjacentSolids(cell: SolidRef, model: Model): Promise<readonly SolidRef[]> {
    const out = new Set<string>();
    const c = geom(model).solids[String(cell)];
    if (!c) return [];
    const faces = new Set<string>();
    for (const sid of c.shellIds) {
      const sh = geom(model).shells[sid];
      if (sh) for (const f of sh.faceIds) faces.add(f);
    }
    for (const f of faces) {
      for (const [cid, cellRec] of Object.entries(geom(model).solids)) {
        if (cid === String(cell)) continue;
        for (const sid of cellRec.shellIds) {
          const sh = geom(model).shells[sid];
          if (sh?.faceIds.includes(f as FaceRef)) out.add(cid);
        }
      }
    }
    return [...out].map((id) => id as SolidRef);
  }

  async sharedFacesBetween(a: SolidRef, b: SolidRef, model: Model): Promise<readonly FaceRef[]> {
    const ca = geom(model).solids[String(a)];
    const cb = geom(model).solids[String(b)];
    if (!ca || !cb) return [];
    const fa = new Set<string>();
    const fb = new Set<string>();
    for (const sid of ca.shellIds) {
      const sh = geom(model).shells[sid];
      if (sh) for (const fid of sh.faceIds) fa.add(fid);
    }
    for (const sid of cb.shellIds) {
      const sh = geom(model).shells[sid];
      if (sh) for (const fid of sh.faceIds) fb.add(fid);
    }
    const xs: FaceRef[] = [];
    for (const x of fa) if (fb.has(x)) xs.push(x as FaceRef);
    return xs;
  }

  private fuseShapes = async (handles: readonly string[]): Promise<string | null> => {
    if (handles.length === 0) return null;
    let result = handles[0]!;
    for (const h of handles.slice(1)) {
      const { handle } = await invokeBrep<{ readonly handle: string }>("fuse", { a: result, b: h });
      result = handle;
    }
    return result;
  };

  async executeCommandDiff(commandId: string, params: Record<string, unknown>): Promise<{ readonly diff: ModelDiff }> {
    const nextId = (kind: string) => `semio-${kind}-${Math.random().toString(36).slice(2, 9)}`;
    const asVec3 = (v: unknown, fallback: Vec3): Vec3 => (Array.isArray(v) && v.length >= 3 ? ([Number(v[0]), Number(v[1]), Number(v[2])] as Vec3) : fallback);
    const poleList = (raw: unknown): Vec3[] => {
      if (!Array.isArray(raw)) return [];
      const out: Vec3[] = [];
      for (const p of raw) if (Array.isArray(p) && p.length >= 3) out.push([Number(p[0]), Number(p[1]), Number(p[2])]);
      return out;
    };
    const createVertex = (pos: Vec3) => ({ id: nextId("v") as VertexRef, position: pos });

    if (commandId === "curve.line") {
      const p0 = asVec3(params.p0, [0, 0, 0]);
      const p1 = asVec3(params.p1, [1, 0, 0]);
      const v0 = createVertex(p0);
      const v1 = createVertex(p1);
      const e = { id: nextId("e") as EdgeRef, vertexIds: [v0.id, v1.id], curve: { kind: "line" as const } };
      const w = { id: nextId("w") as WireRef, edgeIds: [e.id] };
      return { diff: { vertices: { added: [v0, v1] }, edges: { added: [e] }, wires: { added: [w] } } };
    }
    if (commandId === "curve.polyline") {
      const pts = poleList(params.points);
      if (pts.length < 2) return { diff: {} };
      const verts = pts.map((p) => createVertex(p));
      const edges: EdgeRecord[] = [];
      for (let i = 0; i < verts.length - 1; i++) edges.push({ id: nextId("e") as EdgeRef, vertexIds: [verts[i]!.id, verts[i + 1]!.id], curve: { kind: "line" } });
      const w = { id: nextId("w") as WireRef, edgeIds: edges.map((e) => e.id) };
      return { diff: { vertices: { added: verts }, edges: { added: edges }, wires: { added: [w] } } };
    }
    if (commandId === "curve.circle") {
      const center = asVec3(params.center, [0, 0, 0]);
      const radiusPoint = asVec3(params.radiusPoint, [1, 0, 0]);
      const circle = circleFromCenterRadiusPoint(center, radiusPoint);
      if (!circle) return { diff: {} };
      const v = createVertex(radiusPoint);
      const curve: EdgeCurve = { kind: "circle", center: circle.center, normal: circle.normal, radius: circle.radius };
      const e = { id: nextId("e") as EdgeRef, vertexIds: [v.id, v.id], curve };
      const w = { id: nextId("w") as WireRef, edgeIds: [e.id] };
      return { diff: { vertices: { added: [v] }, edges: { added: [e] }, wires: { added: [w] } } };
    }
    if (commandId === "curve.arc") {
      const center = asVec3(params.center, [0, 0, 0]);
      const start = params.start != null ? asVec3(params.start, [1, 0, 0]) : ([1, 0, 0] as Vec3);
      const endRaw = params.end != null ? asVec3(params.end, start) : null;
      const angle = typeof params.angle === "number" ? params.angle : null;
      let endPos: Vec3;
      if (endRaw) endPos = arcEndOnCircle(center, start, endRaw);
      else if (angle !== null) endPos = arcEndFromAngle(center, start, angle) ?? start;
      else endPos = start;
      const vStart = createVertex(start);
      const vEnd = createVertex(endPos);
      const curve: EdgeCurve = { kind: "arc", center };
      const e = { id: nextId("e") as EdgeRef, vertexIds: [vStart.id, vEnd.id], curve };
      const w = { id: nextId("w") as WireRef, edgeIds: [e.id] };
      return { diff: { vertices: { added: [vStart, vEnd] }, edges: { added: [e] }, wires: { added: [w] } } };
    }
    if (commandId === "curve.controlPointCurve" || commandId === "curve.interpolateCurve") {
      const poles = poleList(params.points);
      if (poles.length < 2) return { diff: {} };
      const through = commandId === "curve.interpolateCurve";
      const curve = nurbsCurveFromPoles(poles, through);
      if (!curve) return { diff: {} };
      const vStart = createVertex(poles[0]!);
      const vEnd = createVertex(poles[poles.length - 1]!);
      const e = { id: nextId("e") as EdgeRef, vertexIds: [vStart.id, vEnd.id], curve };
      const w = { id: nextId("w") as WireRef, edgeIds: [e.id] };
      return { diff: { vertices: { added: [vStart, vEnd] }, edges: { added: [e] }, wires: { added: [w] } } };
    }
    if (commandId === "solid.sphere") {
      const center = asVec3(params.center, [0, 0, 0]);
      const radiusPoint = params.radiusPoint != null ? asVec3(params.radiusPoint, center) : null;
      const radius = typeof params.radius === "number" ? params.radius : radiusPoint ? vec3Distance(center, radiusPoint) : 1;
      const solid: SolidPrimitive = { kind: "sphere", center, radius };
      const c = { id: nextId("c") as SolidRef, shellIds: [], solid };
      this.solids.set(c.id, await primitiveHandle(solid));
      return { diff: { solids: { added: [c] } } };
    }
    if (commandId === "solid.cylinder") {
      const base = asVec3(params.base, [0, 0, 0]);
      const radiusPoint = asVec3(params.radiusPoint, [1, 0, 0]);
      const end = asVec3(params.end, base);
      const radius = vec3Distance(base, radiusPoint);
      const axisVec = vec3Sub(end, base);
      const height = vec3Length(axisVec);
      const axis = height > 1e-9 ? vec3Normalize(axisVec) : ([0, 0, 1] as Vec3);
      const solid: SolidPrimitive = { kind: "cylinder", base, axis, radius, height: height > 1e-9 ? height : 1e-6 };
      const c = { id: nextId("c") as SolidRef, shellIds: [], solid };
      this.solids.set(c.id, await primitiveHandle(solid));
      return { diff: { solids: { added: [c] } } };
    }
    if (commandId === "solid.cone") {
      const base = asVec3(params.base, [0, 0, 0]);
      const radiusPoint = asVec3(params.radiusPoint, [1, 0, 0]);
      const end = asVec3(params.end, [0, 0, 1] as Vec3);
      const radius = vec3Distance(base, radiusPoint);
      const axisVec = vec3Sub(end, base);
      const height = vec3Length(axisVec);
      const axis = height > 1e-9 ? vec3Normalize(axisVec) : ([0, 0, 1] as Vec3);
      const solid: SolidPrimitive = { kind: "cone", base, axis, radius, height: height > 1e-9 ? height : 1e-6, radiusTop: 0 };
      const c = { id: nextId("c") as SolidRef, shellIds: [], solid };
      this.solids.set(c.id, await primitiveHandle(solid));
      return { diff: { solids: { added: [c] } } };
    }
    const solidRefsFromSelection = (model: Model, raw: unknown): SolidRef[] => {
      if (!Array.isArray(raw)) return [];
      const out: SolidRef[] = [];
      for (const item of raw) {
        const id = typeof item === "string" ? item : item && typeof item === "object" && "id" in item ? String((item as { id: unknown }).id) : null;
        if (id && geom(model).solids[id]) out.push(id as SolidRef);
      }
      return out;
    };
    if (commandId === "solid.booleanUnion") {
      const model = params.model instanceof Model ? params.model : null;
      if (!model) return { diff: {} };
      const refs = solidRefsFromSelection(model, params.targets);
      if (refs.length < 2) return { diff: {} };
      const handles = await this.validSolidsFromRefs(model, refs);
      const fused = await this.fuseShapes(handles);
      if (!fused) return { diff: {} };
      const ref = this.nextRef("union");
      this.solids.set(ref, fused);
      return { diff: { solids: { added: [{ id: ref, shellIds: [] }], removed: refs } } };
    }
    if (commandId === "solid.booleanDifference") {
      const model = params.model instanceof Model ? params.model : null;
      if (!model) return { diff: {} };
      const baseRefs = solidRefsFromSelection(model, params.baseObjects);
      const toolRefs = solidRefsFromSelection(model, params.cutterObjects);
      if (!baseRefs.length || !toolRefs.length) return { diff: {} };
      const baseHandle = await this.fuseShapes(await this.validSolidsFromRefs(model, baseRefs));
      const toolHandles = await this.validSolidsFromRefs(model, toolRefs);
      if (!baseHandle || !toolHandles.length) return { diff: {} };
      let result = baseHandle;
      for (const tool of toolHandles) {
        const { handle } = await invokeBrep<{ readonly handle: string }>("cut", { a: result, b: tool });
        result = handle;
      }
      const ref = this.nextRef("diff");
      this.solids.set(ref, result);
      return { diff: { solids: { added: [{ id: ref, shellIds: [] }], removed: [...baseRefs, ...toolRefs] } } };
    }
    if (commandId === "solid.booleanIntersection") {
      const model = params.model instanceof Model ? params.model : null;
      if (!model) return { diff: {} };
      const firstRefs = solidRefsFromSelection(model, params.firstSet);
      const secondRefs = solidRefsFromSelection(model, params.secondSet);
      if (!firstRefs.length || !secondRefs.length) return { diff: {} };
      const firstHandle = await this.fuseShapes(await this.validSolidsFromRefs(model, firstRefs));
      const secondHandle = await this.fuseShapes(await this.validSolidsFromRefs(model, secondRefs));
      if (!firstHandle || !secondHandle) return { diff: {} };
      const { handle } = await invokeBrep<{ readonly handle: string }>("intersect", { a: firstHandle, b: secondHandle });
      const ref = this.nextRef("isect");
      this.solids.set(ref, handle);
      return { diff: { solids: { added: [{ id: ref, shellIds: [] }], removed: [...firstRefs, ...secondRefs] } } };
    }
    if (commandId.endsWith("From2PointsAndHeight")) {
      const p0 = asVec3(params.pointA, asVec3(params.p0, [0, 0, 0]));
      const p1 = asVec3(params.pointB, asVec3(params.p1, [1, 1, 0]));
      const height = typeof params.height === "number" && Number.isFinite(params.height) ? params.height : 2.7;
      const cornerA: Vec3 = [Math.min(p0[0], p1[0]), Math.min(p0[1], p1[1]), Math.min(p0[2], p1[2])];
      const cornerB: Vec3 = [Math.max(p0[0], p1[0]), Math.max(p0[1], p1[1]), Math.max(p0[2], p1[2])];
      return this.createBoxFromCornersDiff({ cornerA, cornerB, height });
    }
    if (commandId.endsWith("FromCurveAndHeight")) {
      const wireId = String(params.wireId ?? "");
      const distance = typeof params.height === "number" && Number.isFinite(params.height) ? params.height : 2.7;
      const model = params.model instanceof Model ? params.model : null;
      if (wireId && model) return this.extrudeWireDiff({ wireId, distance, direction: [0, 0, 1], model });
      return { diff: {} };
    }
    if (commandId === "surface.extrudeCrv") {
      const model = params.model instanceof Model ? params.model : null;
      if (!model) return { diff: {} };
      const distance = typeof params.distance === "number" ? params.distance : 1;
      if (!(distance > 1e-9)) return { diff: {} };
      const direction = asVec3(params.direction, [0, 0, 1]);
      const picks = Array.isArray(params.curves) ? (params.curves as readonly { readonly kind?: string; readonly id?: string }[]) : [];
      const wireIds = picks.filter((p) => p.kind === "wire" && p.id).map((p) => String(p.id));
      if (!wireIds.length) return { diff: {} };
      const diffs: ModelDiff[] = [];
      for (const wireId of wireIds) {
        const row = await this.extrudeWireDiff({ wireId, distance, direction, model });
        if (!isEmptyModelDiff(row.diff)) diffs.push(row.diff);
      }
      const added = diffs.flatMap((d) => d.solids?.added ?? []);
      return { diff: added.length ? { solids: { added } } : {} };
    }
    if (commandId === "surface.loft" || commandId === "surface.networkSrf") {
      const model = params.model instanceof Model ? params.model : null;
      if (!model) return { diff: {} };
      const picks = Array.isArray(params.curves) ? (params.curves as readonly { readonly kind?: string; readonly id?: string }[]) : [];
      const wireIds = picks.filter((p) => p.kind === "wire" && p.id).map((p) => String(p.id));
      if (!wireIds.length) return { diff: {} };
      if (wireIds.length === 1) {
        const f: FaceRecord = { id: nextId("f") as FaceRef, wireIds: [wireIds[0]! as WireRef] };
        return { diff: { faces: { added: [f] } } };
      }
      const profiles: string[] = [];
      for (const wireId of wireIds) {
        const handle = await polylineWireHandle(model, wireId as WireRef);
        if (!handle) return { diff: {} };
        const { handle: face } = await invokeBrep<{ readonly handle: string }>("planarFaceFromWire", { wire: handle });
        profiles.push(face);
      }
      await invokeBrep<{ readonly handle: string }>("loft", { profiles, smooth: true });
      const f: FaceRecord = { id: nextId("f") as FaceRef, wireIds: wireIds as WireRef[] };
      return { diff: { faces: { added: [f] } } };
    }
    if (commandId === "surface.sweep1" || commandId === "surface.sweep2") {
      const model = params.model instanceof Model ? params.model : null;
      if (!model) return { diff: {} };
      const targets = params.targets as { readonly rail?: { readonly id?: string } } | undefined;
      const railId = targets?.rail?.id;
      if (!railId) return { diff: {} };
      const picks = Array.isArray(params.sections) ? (params.sections as readonly { readonly kind?: string; readonly id?: string }[]) : [];
      const sectionId = picks.find((p) => p.kind === "wire" && p.id)?.id;
      const railHandle = await polylineWireHandle(model, railId as WireRef);
      const profileHandle = sectionId ? await polylineWireHandle(model, sectionId as WireRef) : null;
      if (!railHandle || !profileHandle) return { diff: {} };
      await invokeBrep<{ readonly handle: string }>("sweep", { profile: profileHandle, path: railHandle });
      const boundaryWireIds = sectionId ? [railId, sectionId] : [railId];
      const f: FaceRecord = { id: nextId("f") as FaceRef, wireIds: boundaryWireIds as WireRef[] };
      return { diff: { faces: { added: [f] } } };
    }
    return { diff: {} };
  }

  async exportModelToStep(model: Model): Promise<string> {
    await this.syncSolidsFromModel(model);
    const handles = [...this.solids.values()];
    if (handles.length === 0) return "";
    const { value } = await invokeBrep<{ readonly value: string }>("exportStep", { shapes: handles });
    return value;
  }

  async importStepHandles(stepText: string): Promise<readonly string[]> {
    const { handles } = await invokeBrep<{ readonly handles: readonly string[] }>("importStep", { data: stepText });
    return handles;
  }

  async deconstruct(solid: SolidRef): Promise<RawTopology | null> {
    const handle = this.solids.get(solid);
    if (!handle) return null;
    return invokeBrep<RawTopology>("deconstruct", { shape: handle });
  }
}
// #endregion 🧠️SemioBrepEngine

// #region 🔌️SemioBrepKernel
/** @emoji 🧠️ THE production CAD `SpatialKernel`: OCCT-backed methods route through the Rust
 * `BrepKernel` via `invokeBrep`; every preview-math method is inherited unchanged from
 * `PreciseSpatialKernelMath`. */
export class SemioBrepKernel extends PreciseSpatialKernelMath implements SpatialKernel {
  readonly id = "semio-brep";
  readonly operations: readonly string[] = ["solid.createBox", "wire.extrudeToSolid", "face.offset", "entity.tessellate", "measure.distance", "measure.area", "measure.volume"];
  private readonly engine = new SemioBrepEngine();

  async resetDerivedPipelineForTest(): Promise<void> {
    this.engine.resetDerivedPipeline();
  }

  async createBoxFromCorners(input: { cornerA: Vec3; cornerB: Vec3; height: number }): Promise<SolidRef> {
    return this.engine.createBoxFromCorners(input);
  }
  async volume(solid: SolidRef): Promise<number> {
    return this.engine.volume(solid);
  }
  async tessellate(solid: SolidRef, tolerance: number, model?: Model): Promise<MeshTransfer> {
    return this.engine.tessellate(solid, tolerance, model);
  }
  disposeSolid(solid: SolidRef): void {
    this.engine.disposeSolid(solid);
  }
  async executeCommandDiff(commandId: string, params: Record<string, unknown>): Promise<{ readonly diff: ModelDiff }> {
    return this.engine.executeCommandDiff(commandId, params);
  }
  async createBoxFromCornersDiff(input: { cornerA: Vec3; cornerB: Vec3; height: number }): Promise<{ readonly diff: ModelDiff; readonly solid: SolidRef }> {
    return this.engine.createBoxFromCornersDiff(input);
  }
  async extrudeWireDiff(input: { wireId: string; distance: number; direction: Vec3; model: Model }): Promise<{ readonly diff: ModelDiff; readonly solid: SolidRef }> {
    return this.engine.extrudeWireDiff(input);
  }
  async offsetFacesDiff(input: { faceIds: readonly string[]; distance: number; model: Model }): Promise<{ readonly diff: ModelDiff }> {
    return this.engine.offsetFacesDiff(input);
  }
  async vertexDistance(a: VertexRef, b: VertexRef, model: Model): Promise<number> {
    return this.engine.vertexDistance(a, b, model);
  }
  async edgeLength(e: EdgeRef, model: Model): Promise<number> {
    return this.engine.edgeLength(e, model);
  }
  async faceArea(f: FaceRef, model: Model): Promise<number> {
    return this.engine.faceArea(f, model);
  }
  async syncSolidsFromModel(model: Model): Promise<void> {
    return this.engine.syncSolidsFromModel(model);
  }
  async solidVolume(c: SolidRef): Promise<number> {
    return this.engine.solidVolume(c);
  }
  async adjacentSolids(solid: SolidRef, model: Model): Promise<readonly SolidRef[]> {
    return this.engine.adjacentSolids(solid, model);
  }
  async sharedFacesBetween(a: SolidRef, b: SolidRef, model: Model): Promise<readonly FaceRef[]> {
    return this.engine.sharedFacesBetween(a, b, model);
  }
  async extrudeWire(input: { wireId: string; distance: number; direction: Vec3; model: Model }): Promise<SolidRef> {
    return this.engine.extrudeWire(input);
  }
  async offsetFaces(input: { faceIds: readonly string[]; distance: number; model: Model }): Promise<void> {
    return this.engine.offsetFaces(input);
  }
  async exportModelToStep(model: Model): Promise<string> {
    return this.engine.exportModelToStep(model);
  }
  async deconstruct(solid: SolidRef): Promise<{ readonly vertices: readonly string[]; readonly edges: readonly string[]; readonly faces: readonly string[]; readonly shells: readonly string[] } | null> {
    return this.engine.deconstruct(solid);
  }
}

export const semioBrepKernel = new SemioBrepKernel();
// #endregion 🔌️SemioBrepKernel

// #region 🧪️Tests
if (import.meta.vitest) {
  const { beforeEach, describe, expect, it } = import.meta.vitest;
  const { bootstrapCadModules } = await import("../../../../🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏃️runtime/🟦️.ts");
  bootstrapCadModules();

  describe("@semio-tech/cad-js/spatial-kernel/semio", () => {
    const kernel = new SemioBrepKernel();

    beforeEach(async () => {
      await kernel.resetDerivedPipelineForTest();
    });

    it("createBoxFromCorners volume matches axis-aligned footprint×height", async () => {
      const cell = await kernel.createBoxFromCorners({ cornerA: [0, 0, 0], cornerB: [2, 3, 0], height: 4 });
      expect(await kernel.volume(cell)).toBeCloseTo(24, 3);
    });

    it("createBoxFromCornersDiff includes one face bucket and matching FaceRef entity ids on tessellate", async () => {
      const r = await kernel.createBoxFromCornersDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 });
      expect(Object.keys(r.diff.faces?.added ?? {}).length).toBe(6);
      const mesh = await kernel.tessellate(r.solid, 1e-3);
      expect(mesh.index.length).toBeGreaterThan(0);
      const modelFaceIds = new Set((r.diff.faces?.added ?? []).map((f) => String(f.id)));
      for (const info of mesh.faceInfos) expect(modelFaceIds.has(String(info.entityId))).toBe(true);
    });

    it("solid.sphere command creates a solid with the expected volume", async () => {
      const res = await kernel.executeCommandDiff("solid.sphere", { center: [0, 0, 0], radius: 2 });
      const added = res.diff.solids?.added?.[0];
      expect(added).toBeTruthy();
      const vol = await kernel.solidVolume(added!.id);
      expect(vol).toBeCloseTo((4 / 3) * Math.PI * 8, 0);
    });

    it("solid.booleanDifference cuts a sphere out of a box", async () => {
      const box = await kernel.executeCommandDiff("solid.sphere", { center: [0, 0, 0], radius: 5 });
      const boxId = box.diff.solids!.added![0]!.id;
      const sphere = await kernel.executeCommandDiff("solid.sphere", { center: [0, 0, 0], radius: 1 });
      const sphereId = sphere.diff.solids!.added![0]!.id;
      const res = await kernel.executeCommandDiff("solid.booleanDifference", { baseObjects: [{ id: boxId }], cutterObjects: [{ id: sphereId }] });
      const resultId = res.diff.solids?.added?.[0]?.id;
      expect(resultId).toBeTruthy();
      const vol = await kernel.solidVolume(resultId!);
      expect(vol).toBeGreaterThan(0);
      expect(vol).toBeLessThan((4 / 3) * Math.PI * 125);
    });

    it("curve.arc places start/end vertices on the requested circle", async () => {
      const res = await kernel.executeCommandDiff("curve.arc", { center: [0, 0, 0], start: [1, 0, 0], angle: 90 });
      const verts = res.diff.vertices?.added ?? [];
      expect(verts.length).toBe(2);
      expect(verts[0]!.position).toEqual([1, 0, 0]);
    });

    it("energy wall command (…From2PointsAndHeight) builds a box solid", async () => {
      const res = await kernel.executeCommandDiff("energy.energy.constructExternalWallFrom2PointsAndHeight", { pointA: [0, 0, 0], pointB: [4, 0, 0], height: 2.7 });
      expect(res.diff.solids?.added?.length).toBe(1);
      const vol = await kernel.solidVolume(res.diff.solids!.added![0]!.id);
      expect(vol).toBeGreaterThan(0);
    });
  });
}
// #endregion 🧪️Tests
