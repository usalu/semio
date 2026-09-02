// #region 🧱️Header
/** @emoji 🧩 `@semio-tech/puzzle-5d-react` — compose flat+volume fixtures into a 5d model and flatten for sketchpad topology. */
// #endregion 🧱️Header

//#region 🔖️Types
/** ⚓️ Part root plane policy aligned with {@link https://github.com/usalu/semio/blob/main/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🦀️.rs} puzzle5d schema. */
export type Puzzle5dPartAnchor = "fixed" | "derived";

/** 🧱️ Merged 5d part for topology flatten (sketchpad `2d` / `3d` presentation keys). */
export type Puzzle5dComposePart = {
  readonly id: string;
  readonly anchor?: Puzzle5dPartAnchor;
  readonly partKind?: string;
  readonly "2d": Record<string, unknown>;
  readonly "3d": Record<string, unknown>;
  readonly grips?: readonly Record<string, unknown>[];
};

/** 🔗️ Fastener row with eight transform params + diagram offsets. */
export type Puzzle5dComposeFastener = {
  readonly id: string;
  readonly source: string;
  readonly target: string;
  readonly fastenerKind?: string;
  readonly gap?: number;
  readonly shift?: number;
  readonly rise?: number;
  readonly rotation?: number;
  readonly turn?: number;
  readonly tilt?: number;
  readonly x?: number;
  readonly y?: number;
};

/** 🧩 Unified compose model produced by {@link compose5d}. */
export type Puzzle5dComposeModel = {
  readonly schema?: string;
  readonly domain?: string;
  readonly parts: Puzzle5dComposePart[];
  readonly fasteners: Puzzle5dComposeFastener[];
};

/** 📐 Sketchpad topology diagram icon width (compose diagram u/v → board pixels). */
export const PUZZLE_5D_TOPOLOGY_ICON_WIDTH = 48;

type FlatRecord = Record<string, unknown>;
type VolumeRecord = Record<string, unknown>;
//#endregion 🔖️Types

//#region 🔖️Compose5d
function num(row: FlatRecord, key: string, fallback = 0): number {
  const value = row[key];
  return typeof value === "number" && Number.isFinite(value) ? value : fallback;
}

function str(row: FlatRecord, key: string): string | undefined {
  const value = row[key];
  return typeof value === "string" ? value : undefined;
}

function anchorFromRow(row: FlatRecord): Puzzle5dPartAnchor {
  const raw = row.anchor;
  if (raw === "fixed" || raw === "derived") return raw;
  return "derived";
}

function gripLocalId(rawId: string, partId: string): string {
  const colon = rawId.indexOf(":");
  if (colon < 0) return rawId;
  const prefix = rawId.slice(0, colon);
  if (prefix === partId) return rawId.slice(colon + 1);
  return rawId.slice(colon + 1);
}

function vec3(row: FlatRecord, key: string, fallback: [number, number, number]): [number, number, number] {
  const value = row[key];
  if (Array.isArray(value) && value.length >= 3) {
    return [Number(value[0]) || 0, Number(value[1]) || 0, Number(value[2]) || 0];
  }
  return fallback;
}

function quat(row: FlatRecord, key: string): [number, number, number, number] {
  const value = row[key];
  if (Array.isArray(value) && value.length >= 4) {
    return [num({ v: value[0] }, "v", 0), num({ v: value[1] }, "v", 0), num({ v: value[2] }, "v", 0), num({ v: value[3] }, "v", 1)];
  }
  return [0, 0, 0, 1];
}

/** 🔀 Merges puzzle 2d flat + 3d volume fixtures into one 5d compose model (no compose runtime adapters). */
export function compose5d(flatFixture: FlatRecord, volumeFixture: FlatRecord): Puzzle5dComposeModel {
  const nodes = Array.isArray(flatFixture.nodes) ? (flatFixture.nodes as FlatRecord[]) : [];
  const edges = Array.isArray(flatFixture.edges) ? (flatFixture.edges as FlatRecord[]) : [];
  const objects = Array.isArray(volumeFixture.objects) ? (volumeFixture.objects as FlatRecord[]) : [];
  const attractions = Array.isArray(volumeFixture.attractions) ? (volumeFixture.attractions as FlatRecord[]) : [];
  const objectById = new Map(objects.map((object) => [str(object, "id") ?? "", object]));

  const parts: Puzzle5dComposePart[] = nodes.map((node) => {
    const id = str(node, "id") ?? "";
    const object = objectById.get(id);
    const handles = Array.isArray(node.handles) ? (node.handles as FlatRecord[]) : [];
    const vortices = object && Array.isArray(object.vortices) ? (object.vortices as FlatRecord[]) : [];
    const vortexById = new Map(vortices.map((v) => [str(v, "id") ?? "", v]));

    const grips = handles.map((handle) => {
      const handleId = str(handle, "id") ?? "";
      const vortex = vortexById.get(handleId) ?? vortexById.get(gripLocalId(handleId, id));
      const angle = num(handle, "angle", num(handle, "t", 0) * 2 * Math.PI);
      return {
        id: gripLocalId(handleId, id),
        gripKind: str(handle, "handleKind") ?? str(vortex ?? {}, "vortexKind"),
        grip_2d: { angle, radius: num(handle, "radius", undefined as unknown as number) },
        grip_3d: {
          position: vortex ? vec3(vortex, "position", [0, 0, 0]) : [0, 0, 0],
          direction: vortex ? vec3(vortex, "direction", [0, 0, 1]) : [0, 0, 1],
          radius: vortex ? num(vortex, "radius", undefined as unknown as number) : undefined,
          label: vortex ? str(vortex, "label") : undefined,
        },
      };
    });

    const part2d: FlatRecord = {
      x: num(node, "x", 0),
      y: num(node, "y", 0),
      shape: str(node, "shape"),
      radius: num(node, "radius", undefined as unknown as number),
      width: num(node, "width", undefined as unknown as number),
      height: num(node, "height", undefined as unknown as number),
      text: str(node, "text"),
      iconKind: str(node, "iconKind"),
      hidden: node.hidden,
      locked: node.locked,
    };

    const part3d: FlatRecord = object
      ? {
          origin: vec3(object, "origin", [0, 0, 0]),
          orientation: quat(object, "orientation"),
          meshUrl: str(object, "meshUrl"),
          scale: object.scale,
          label: str(object, "label") ?? str(node, "text"),
        }
      : { origin: [0, 0, 0], orientation: [0, 0, 0, 1] };

    const anchor = object ? anchorFromRow(object) : anchorFromRow(node);
    return {
      id,
      anchor,
      partKind: str(node, "nodeKind") ?? str(object ?? {}, "objectKind"),
      "2d": part2d,
      "3d": part3d,
      grips,
    };
  });

  const attractionById = new Map(attractions.map((row) => [str(row, "id") ?? "", row]));
  const fasteners: Puzzle5dComposeFastener[] = edges.map((edge) => {
    const id = str(edge, "id") ?? "";
    const attraction = attractionById.get(id);
    const pickNum = (key: string) => num(edge, key, attraction ? num(attraction, key, 0) : 0);
    return {
      id,
      source: str(edge, "source") ?? str(attraction ?? {}, "attracting") ?? "",
      target: str(edge, "target") ?? str(attraction ?? {}, "attracted") ?? "",
      fastenerKind: str(edge, "edgeKind") ?? str(attraction ?? {}, "attractionKind"),
      gap: pickNum("gap"),
      shift: pickNum("shift"),
      rise: pickNum("rise"),
      rotation: pickNum("rotation"),
      turn: pickNum("turn"),
      tilt: pickNum("tilt"),
      x: pickNum("x"),
      y: pickNum("y"),
    };
  });

  return {
    schema: "puzzle.5d.compose",
    domain: str(volumeFixture, "domain") ?? "architecture",
    parts,
    fasteners,
  };
}
//#endregion 🔖️Compose5d

//#region 🔖️Flatten
const TOLERANCE = 0.01;
const DIAGRAM_RADIUS = 2.697;
const DIAGRAM_VERTICAL_V_EXTRA = 1.0;
const DIAGRAM_HORIZONTAL_SCALE = 3.0633;

type FlattenPlane = { origin: [number, number, number]; xAxis: [number, number, number]; yAxis: [number, number, number] };
type FlattenPose = { plane: FlattenPlane; center: [number, number, number]; orientation: [number, number, number, number] };

type FlatObject = {
  id: string;
  anchor: Puzzle5dPartAnchor;
  origin: [number, number, number];
  orientation: [number, number, number, number];
  vortices: Array<{ id: string; position: [number, number, number]; direction: [number, number, number]; t: number }>;
};

type FlatAttraction = {
  attracting: string;
  attracted: string;
  gap: number;
  shift: number;
  rise: number;
  rotation: number;
  turn: number;
  tilt: number;
  x: number;
  y: number;
};

function normalize(v: [number, number, number]): [number, number, number] {
  const len = Math.hypot(v[0], v[1], v[2]);
  if (len <= 0) return [0, 0, 1];
  return [v[0] / len, v[1] / len, v[2] / len];
}

function dot(a: [number, number, number], b: [number, number, number]): number {
  return a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
}

function cross(a: [number, number, number], b: [number, number, number]): [number, number, number] {
  return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
}

function degToRad(deg: number): number {
  return (deg * Math.PI) / 180;
}

function roundF(v: number): number {
  return Math.round(v * 1_000_000) / 1_000_000;
}

function planeToMatrix(p: FlattenPlane): number[] {
  const x = p.xAxis;
  const y = p.yAxis;
  const z = cross(x, y);
  return [x[0], x[1], x[2], 0, y[0], y[1], y[2], 0, z[0], z[1], z[2], 0, p.origin[0], p.origin[1], p.origin[2], 1];
}

function matrixToPlane(m: number[]): FlattenPlane {
  return { origin: [m[12], m[13], m[14]], xAxis: [m[0], m[1], m[2]], yAxis: [m[4], m[5], m[6]] };
}

function mulMat(a: number[], b: number[]): number[] {
  const out = new Array<number>(16).fill(0);
  for (let col = 0; col < 4; col += 1) {
    for (let row = 0; row < 4; row += 1) {
      out[col * 4 + row] =
        a[row] * b[col * 4] + a[4 + row] * b[col * 4 + 1] + a[8 + row] * b[col * 4 + 2] + a[12 + row] * b[col * 4 + 3];
    }
  }
  return out;
}

function translation(x: number, y: number, z: number): number[] {
  return [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, x, y, z, 1];
}

function rotationAxis(axis: [number, number, number], angle: number): number[] {
  const [x, y, z] = axis;
  const c = Math.cos(angle);
  const s = Math.sin(angle);
  const t = 1 - c;
  return [
    t * x * x + c,
    t * x * y + s * z,
    t * x * z - s * y,
    0,
    t * x * y - s * z,
    t * y * y + c,
    t * y * z + s * x,
    0,
    t * x * z + s * y,
    t * y * z - s * x,
    t * z * z + c,
    0,
    0,
    0,
    0,
    1,
  ];
}

function applyMatVec3(m: number[], v: [number, number, number]): [number, number, number] {
  return [m[0] * v[0] + m[4] * v[1] + m[8] * v[2], m[1] * v[0] + m[5] * v[1] + m[9] * v[2], m[2] * v[0] + m[6] * v[1] + m[10] * v[2]];
}

function quaternionFromUnitVectors(from: [number, number, number], to: [number, number, number]): [number, number, number, number] {
  const r = dot(from, to) + 1;
  let quat: [number, number, number, number];
  if (r < 0.000001) {
    if (Math.abs(from[0]) > Math.abs(from[2])) quat = [-from[1], from[0], 0, 0];
    else quat = [0, -from[2], from[1], 0];
  } else {
    const c = cross(from, to);
    quat = [c[0], c[1], c[2], r];
  }
  const len = Math.hypot(quat[0], quat[1], quat[2], quat[3]);
  return [quat[0] / len, quat[1] / len, quat[2] / len, quat[3] / len];
}

function quaternionToMatrix(q: [number, number, number, number]): number[] {
  const [x, y, z, w] = q;
  const x2 = x + x;
  const y2 = y + y;
  const z2 = z + z;
  const xx = x * x2;
  const xy = x * y2;
  const xz = x * z2;
  const yy = y * y2;
  const yz = y * z2;
  const zz = z * z2;
  const wx = w * x2;
  const wy = w * y2;
  const wz = w * z2;
  return [
    1 - (yy + zz),
    xy + wz,
    xz - wy,
    0,
    xy - wz,
    1 - (xx + zz),
    yz + wx,
    0,
    xz + wy,
    yz - wx,
    1 - (xx + yy),
    0,
    0,
    0,
    0,
    1,
  ];
}

function planeToOrientation(plane: FlattenPlane): [number, number, number, number] {
  const xx = plane.xAxis[0];
  const xy = plane.xAxis[1];
  const xz = plane.xAxis[2];
  const yx = plane.yAxis[0];
  const yy = plane.yAxis[1];
  const yz = plane.yAxis[2];
  const zx = xy * yz - xz * yy;
  const zy = xz * yx - xx * yz;
  const zz = xx * yy - xy * yx;
  const m00 = xx;
  const m01 = yx;
  const m02 = zx;
  const m10 = xy;
  const m11 = yy;
  const m12 = zy;
  const m20 = xz;
  const m21 = yz;
  const m22 = zz;
  const trace = m00 + m11 + m22;
  if (trace > 0) {
    const s = Math.sqrt(trace + 1) * 2;
    return [(m21 - m12) / s, (m02 - m20) / s, (m10 - m01) / s, 0.25 * s];
  }
  if (m00 > m11 && m00 > m22) {
    const s = Math.sqrt(1 + m00 - m11 - m22) * 2;
    return [0.25 * s, (m01 + m10) / s, (m02 + m20) / s, (m21 - m12) / s];
  }
  if (m11 > m22) {
    const s = Math.sqrt(1 + m11 - m00 - m22) * 2;
    return [(m01 + m10) / s, 0.25 * s, (m12 + m21) / s, (m02 - m20) / s];
  }
  const s = Math.sqrt(1 + m22 - m00 - m11) * 2;
  return [(m02 + m20) / s, (m12 + m21) / s, 0.25 * s, (m10 - m01) / s];
}

function orientationToPlane(origin: [number, number, number], orientation: [number, number, number, number]): FlattenPlane {
  const m = quaternionToMatrix(orientation);
  return { origin, xAxis: [m[0], m[1], m[2]], yAxis: [m[4], m[5], m[6]] };
}

function parseEndpoint(endpoint: string): [string, string] | null {
  const colon = endpoint.indexOf(":");
  if (colon < 0) return null;
  return [endpoint.slice(0, colon), endpoint.slice(colon + 1)];
}

function diagramCenter(parentCenter: [number, number], parentDirection: [number, number, number], parentT: number, attraction: FlatAttraction): [number, number] {
  const connectionX = attraction.x;
  const connectionY = attraction.y;
  let childX: number;
  let childY: number;
  if (parentCenter[0] === 0 && parentCenter[1] === 0) {
    const angle = 2 * Math.PI * parentT;
    childX = DIAGRAM_RADIUS * Math.sin(angle);
    childY = DIAGRAM_RADIUS * Math.cos(angle);
  } else if (Math.abs(parentDirection[2]) > 0.5) {
    childX = parentCenter[0] + connectionX;
    childY = parentCenter[1] + connectionY + DIAGRAM_VERTICAL_V_EXTRA;
  } else {
    childX = parentCenter[0] + connectionX * DIAGRAM_HORIZONTAL_SCALE;
    childY = parentCenter[1] + connectionY * DIAGRAM_HORIZONTAL_SCALE;
  }
  return [roundF(childX), roundF(childY)];
}

function computeChildPlane(
  parentPlane: FlattenPlane,
  parentPoint: [number, number, number],
  parentDir: [number, number, number],
  childPoint: [number, number, number],
  childDir: [number, number, number],
  attraction: FlatAttraction,
): FlattenPlane {
  const parentMatrix = planeToMatrix(parentPlane);
  let pDir = normalize(parentDir);
  let cDir = normalize(childDir);
  const gap = attraction.gap;
  const shift = attraction.shift;
  const rise = attraction.rise;
  const rotationRad = degToRad(attraction.rotation);
  const turnRad = degToRad(attraction.turn);
  const tiltRad = degToRad(attraction.tilt);
  const reverseChild: [number, number, number] = [-cDir[0], -cDir[1], -cDir[2]];
  const crossVec = cross(pDir, reverseChild);
  const crossLen = Math.hypot(crossVec[0], crossVec[1], crossVec[2]);
  let alignQuat: [number, number, number, number];
  if (crossLen < TOLERANCE) {
    if (dot(pDir, reverseChild) > 0) alignQuat = [0, 0, 0, 1];
    else if (Math.abs(pDir[2]) < TOLERANCE) alignQuat = quaternionFromUnitVectors([0, 1, 0], [0, 0, -1]);
    else alignQuat = [1, 0, 0, 0];
  } else {
    alignQuat = quaternionFromUnitVectors(reverseChild, pDir);
  }
  const directionT = quaternionToMatrix(alignQuat);
  const yAxis: [number, number, number] = [0, 1, 0];
  const parentRotationT = quaternionToMatrix(quaternionFromUnitVectors(yAxis, pDir));
  const gapDirection = applyMatVec3(parentRotationT, [0, 1, 0]);
  const shiftDirection = applyMatVec3(parentRotationT, [1, 0, 0]);
  const raiseDirection = applyMatVec3(parentRotationT, [0, 0, 1]);
  let turnAxis = applyMatVec3(parentRotationT, [0, 0, 1]);
  let tiltAxis = applyMatVec3(parentRotationT, [1, 0, 0]);
  let orientationT = directionT;
  const rotateT = rotationAxis(pDir, -rotationRad);
  orientationT = mulMat(rotateT, orientationT);
  turnAxis = applyMatVec3(rotateT, turnAxis);
  tiltAxis = applyMatVec3(rotateT, tiltAxis);
  orientationT = mulMat(rotationAxis(turnAxis, turnRad), orientationT);
  orientationT = mulMat(rotationAxis(tiltAxis, tiltRad), orientationT);
  const centerChildT = translation(-childPoint[0], -childPoint[1], -childPoint[2]);
  let transform = mulMat(orientationT, centerChildT);
  const gapTransform = translation(gapDirection[0] * gap, gapDirection[1] * gap, gapDirection[2] * gap);
  const shiftTransform = translation(shiftDirection[0] * shift, shiftDirection[1] * shift, shiftDirection[2] * shift);
  const raiseTransform = translation(raiseDirection[0] * rise, raiseDirection[1] * rise, raiseDirection[2] * rise);
  transform = mulMat(mulMat(raiseTransform, mulMat(shiftTransform, gapTransform)), transform);
  transform = mulMat(translation(parentPoint[0], parentPoint[1], parentPoint[2]), transform);
  return matrixToPlane(mulMat(parentMatrix, transform));
}

function flattenObjects(
  objects: FlatObject[],
  attractions: FlatAttraction[],
  seedCenters: Map<string, [number, number]> | undefined,
): Map<string, FlattenPose> {
  if (objects.length === 0) return new Map();
  const objectMap = new Map(objects.map((object) => [object.id, object]));
  const adjacency = new Map<string, Array<[string, number]>>();
  for (let index = 0; index < attractions.length; index += 1) {
    const attraction = attractions[index]!;
    const parent = parseEndpoint(attraction.attracting);
    const child = parseEndpoint(attraction.attracted);
    if (!parent || !child) continue;
    if (!objectMap.has(parent[0]) || !objectMap.has(child[0])) continue;
    const parentId = parent[0];
    const childId = child[0];
    if (!adjacency.has(parentId)) adjacency.set(parentId, []);
    if (!adjacency.has(childId)) adjacency.set(childId, []);
    adjacency.get(parentId)!.push([childId, index]);
    adjacency.get(childId)!.push([parentId, index]);
  }

  const piecePlanes = new Map<string, FlattenPlane>();
  const pieceCenters = new Map<string, [number, number]>();
  const visited = new Set<string>();

  const bfsRoot = (rootId: string) => {
    const queue: string[] = [rootId];
    visited.add(rootId);
    const root = objectMap.get(rootId)!;
    const storedPlane = orientationToPlane(root.origin, root.orientation);
    const storedCenter = seedCenters?.get(rootId) ?? [0, 0];
    if (root.anchor === "fixed") {
      piecePlanes.set(rootId, storedPlane);
      pieceCenters.set(rootId, storedCenter);
    } else {
      piecePlanes.set(rootId, { origin: [0, 0, 0], xAxis: [1, 0, 0], yAxis: [0, 1, 0] });
      pieceCenters.set(rootId, storedCenter);
    }
    while (queue.length > 0) {
      const currentId = queue.shift()!;
      const currentPlane = piecePlanes.get(currentId) ?? { origin: [0, 0, 0], xAxis: [1, 0, 0], yAxis: [0, 1, 0] };
      const parentCenter = pieceCenters.get(currentId) ?? [0, 0];
      const neighbors = adjacency.get(currentId) ?? [];
      for (const [neighborId, attractionIndex] of neighbors) {
        if (visited.has(neighborId)) continue;
        visited.add(neighborId);
        const attraction = attractions[attractionIndex]!;
        const parentEp = parseEndpoint(attraction.attracting);
        const childEp = parseEndpoint(attraction.attracted);
        if (!parentEp || !childEp) {
          piecePlanes.set(neighborId, { origin: [0, 0, 0], xAxis: [1, 0, 0], yAxis: [0, 1, 0] });
          pieceCenters.set(neighborId, [0, 0]);
          queue.push(neighborId);
          continue;
        }
        const currentVortexId = parentEp[0] === currentId ? parentEp[1] : childEp[1];
        const neighborVortexId = parentEp[0] === currentId ? childEp[1] : parentEp[1];
        const currentObject = objectMap.get(currentId)!;
        const neighborObject = objectMap.get(neighborId)!;
        const parentVortex = currentObject.vortices.find((v) => v.id === currentVortexId);
        const childVortex = neighborObject.vortices.find((v) => v.id === neighborVortexId);
        if (!parentVortex || !childVortex) {
          piecePlanes.set(neighborId, { origin: [0, 0, 0], xAxis: [1, 0, 0], yAxis: [0, 1, 0] });
          pieceCenters.set(neighborId, [0, 0]);
          queue.push(neighborId);
          continue;
        }
        const parentPoint = parentVortex.position;
        const parentDirection = normalize(parentVortex.direction);
        const childPoint = childVortex.position;
        const childDirection = normalize(childVortex.direction);
        const childPlane = computeChildPlane(currentPlane, parentPoint, parentDirection, childPoint, childDirection, attraction);
        piecePlanes.set(neighborId, childPlane);
        pieceCenters.set(neighborId, diagramCenter(parentCenter, parentDirection, parentVortex.t, attraction));
        queue.push(neighborId);
      }
    }
  };

  for (const object of objects) {
    if (object.anchor === "fixed" && !visited.has(object.id)) bfsRoot(object.id);
  }
  for (const object of objects) {
    if (!visited.has(object.id)) bfsRoot(object.id);
  }

  const out = new Map<string, FlattenPose>();
  for (const object of objects) {
    const plane = piecePlanes.get(object.id) ?? { origin: [0, 0, 0], xAxis: [1, 0, 0], yAxis: [0, 1, 0] };
    const center = pieceCenters.get(object.id) ?? [0, 0];
    out.set(object.id, { plane, center, orientation: planeToOrientation(plane) });
  }
  return out;
}

function flattenComposeModel(model: Puzzle5dComposeModel): Puzzle5dComposeModel {
  const objects: FlatObject[] = model.parts.map((part) => {
    const part3d = part["3d"];
    const grips = part.grips ?? [];
    return {
      id: part.id,
      anchor: part.anchor ?? "derived",
      origin: vec3(part3d, "origin", [0, 0, 0]),
      orientation: quat(part3d, "orientation"),
      vortices: grips.map((grip) => {
        const grip3d = (grip.grip_3d as FlatRecord) ?? {};
        const grip2d = (grip.grip_2d as FlatRecord) ?? {};
        const angle = num(grip2d, "angle", 0);
        return {
          id: str(grip, "id") ?? "",
          position: vec3(grip3d, "position", [0, 0, 0]),
          direction: normalize(vec3(grip3d, "direction", [0, 0, 1])),
          t: angle / (2 * Math.PI),
        };
      }),
    };
  });

  const seedCenters = new Map<string, [number, number]>();
  for (const part of model.parts) {
    const part2d = part["2d"];
    seedCenters.set(part.id, [num(part2d, "x", 0), num(part2d, "y", 0)]);
  }

  const attractions: FlatAttraction[] = model.fasteners.map((fastener) => ({
    attracting: fastener.source,
    attracted: fastener.target,
    gap: fastener.gap ?? 0,
    shift: fastener.shift ?? 0,
    rise: fastener.rise ?? 0,
    rotation: fastener.rotation ?? 0,
    turn: fastener.turn ?? 0,
    tilt: fastener.tilt ?? 0,
    x: fastener.x ?? 0,
    y: fastener.y ?? 0,
  }));

  const flatObjects: FlatObject[] = objects;

  const poses = flattenObjects(flatObjects, attractions, seedCenters);

  const parts = model.parts.map((part) => {
    const pose = poses.get(part.id);
    if (!pose) return part;
    const part2d = { ...part["2d"], x: pose.center[0], y: pose.center[1] };
    const part3d = { ...part["3d"], origin: pose.plane.origin, orientation: pose.orientation };
    return { ...part, "2d": part2d, "3d": part3d };
  });

  return { ...model, parts };
}

/** 🌤️ Flattens a compose model and scales diagram `2d.x` / `2d.y` into sketchpad topology pixels. */
export function prepareTopologyModel(model: Puzzle5dComposeModel): Puzzle5dComposeModel {
  const flattened = flattenComposeModel(model);
  const scale = PUZZLE_5D_TOPOLOGY_ICON_WIDTH;
  const parts = flattened.parts.map((part) => {
    const part2d = part["2d"];
    return {
      ...part,
      "2d": {
        ...part2d,
        x: num(part2d, "x", 0) * scale,
        y: num(part2d, "y", 0) * scale,
      },
    };
  });
  return { ...flattened, parts };
}
//#endregion 🔖️Flatten

//#region 🔖️Vitest
if (import.meta.vitest) {
  const { describe, it, expect } = import.meta.vitest;

  describe("compose5d + prepareTopologyModel", () => {
    it("flattens a fixed root and derived child with fastener x/y", () => {
      const flat = {
        schema: "puzzle.2d.fixture",
        camera: { x: 0, y: 0, zoom: 1 },
        nodes: [
          {
            id: "root",
            anchor: "fixed",
            x: 1,
            y: 2,
            handles: [{ id: "root:conn-a", handleKind: "compose.connector", angle: 0, t: 0 }],
          },
          { id: "child", anchor: "derived", x: 0, y: 0, handles: [{ id: "child:conn-a", handleKind: "compose.connector", angle: 0, t: 0 }] },
        ],
        edges: [
          {
            id: "link-1",
            source: "root:conn-a",
            target: "child:conn-a",
            x: 0.5,
            y: 0.25,
            gap: 0,
            shift: 0,
            rise: 0,
            rotation: 0,
            turn: 0,
            tilt: 0,
          },
        ],
      };
      const volume = {
        schema: "puzzle.3d.fixture",
        domain: "architecture",
        camera: { position: [8, 8, 8], target: [0, 0, 0], zoom: 1 },
        objects: [
          {
            id: "root",
            anchor: "fixed",
            origin: [1, 2, 3],
            orientation: [0, 0, 0, 1],
            vortices: [{ id: "root:conn-a", position: [0, 0, 0], direction: [0, 0, 1], vortexKind: "compose.connector" }],
          },
          {
            id: "child",
            anchor: "derived",
            origin: [0, 0, 0],
            orientation: [0, 0, 0, 1],
            vortices: [{ id: "child:conn-a", position: [0, 0, 0], direction: [0, 0, 1], vortexKind: "compose.connector" }],
          },
        ],
        attractions: [
          {
            id: "link-1",
            attracting: "root:conn-a",
            attracted: "child:conn-a",
            x: 0.5,
            y: 0.25,
          },
        ],
      };
      const prepared = prepareTopologyModel(compose5d(flat, volume));
      const child = prepared.parts.find((part) => part.id === "child");
      expect(child?.["3d"]?.origin).toBeDefined();
      expect((child?.["3d"]?.origin as number[])[0]).toBeCloseTo(1, 2);
      expect(child?.["2d"]?.x).toBeCloseTo(1.5 * PUZZLE_5D_TOPOLOGY_ICON_WIDTH, 2);
    });
  });
}
//#endregion 🔖️Vitest
