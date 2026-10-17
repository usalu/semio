/** 🧱️ Mesh parsing and residency: the kind geometry, the mesh record resolution and JSON array splitting, the residency stepping and the surface sample accessors. */
import { BoxGeometry, BufferGeometry, ConeGeometry, CylinderGeometry, IcosahedronGeometry, PlaneGeometry, SphereGeometry, TorusGeometry } from "@semio-tech/ui-react";
import { type WorldMeshAttribute, type WorldMeshData, type WorldMeshRecord } from "../../🔭️scene-schema/🟦️.ts";

//#region 🔖️World3dHost
//#region WorldSceneParsing
/** 🥽️ One built-in mesh kind's geometry, built from the engine's own primitives. The placement and
 * extent are pinned against `🧰️framework/🔨️modules/🏗️mesh-engine/🧫️fixtures/🥽️scene-mesh-kinds/🔣️.json`,
 * the same fixture Rust's `mesh_from_kind` answers to — the two tessellate differently on purpose, so the
 * local bounding box, not the triangle list, is the contract. `cone` and `plane`/`torus` are re-placed
 * because the engine's primitives are centred / lie in a different plane than the scene convention. */
function worldMeshKindGeometry(kind: string): BufferGeometry {
  switch (kind) {
    case "plane":
      return new PlaneGeometry(1, 1).rotateX(-Math.PI / 2);
    case "sphere":
    case "uvSphere":
      return new SphereGeometry(0.5, 16, 12);
    case "icoSphere":
      return new IcosahedronGeometry(0.5, 1);
    case "vortex-marker":
      return new IcosahedronGeometry(0.12, 1);
    case "vertex-marker":
      return new IcosahedronGeometry(1, 1);
    case "cylinder":
      return new CylinderGeometry(0.5, 0.5, 1, 16);
    case "cone":
      return new ConeGeometry(0.5, 1, 16).translate(0, 0.5, 0);
    case "torus":
      return new TorusGeometry(0.5, 0.15, 16, 12).rotateX(-Math.PI / 2);
    default:
      return new BoxGeometry(1, 1, 1);
  }
}

const worldMeshKindData = new Map<string, WorldMeshData>();

/** 🥽️ Resolves a `{ id, kind }` scene mesh reference into the buffers every downstream path
 * (shading, edge outlines, marquee bounds, component overlays) already expects. Memoized per kind: the
 * set is closed and tiny, and a scene refresh must not re-tessellate. */
export function meshDataFromKind(kind: string): WorldMeshData {
  const cached = worldMeshKindData.get(kind);
  if (cached) return cached;
  const geometry = worldMeshKindGeometry(kind);
  if (!geometry.getAttribute("normal")) geometry.computeVertexNormals();
  const index = geometry.getIndex();
  const positions = Array.from(geometry.getAttribute("position").array as ArrayLike<number>);
  const data: WorldMeshData = {
    positions,
    normals: Array.from(geometry.getAttribute("normal").array as ArrayLike<number>),
    // 🔺️ A non-indexed engine primitive (the polyhedra) still owes this record a triangle list: every
    // downstream reader gates shaded rendering, face overlays and pick on `indices.length > 0`, and the
    // Rust generator emits the same trivial sequential run for its own non-indexed triangles.
    indices: index ? Array.from(index.array as ArrayLike<number>) : Array.from({ length: positions.length / 3 }, (_unused, vertex) => vertex),
  };
  geometry.dispose();
  worldMeshKindData.set(kind, data);
  return data;
}

function resolveMeshRecord(record: WorldMeshRecord): WorldMeshRecord {
  return record.data || !record.kind ? record : { ...record, data: meshDataFromKind(record.kind) };
}

function parseMeshes(meshesJson: string): WorldMeshRecord[] {
  try {
    const parsed = JSON.parse(meshesJson);
    if (!Array.isArray(parsed)) return [];
    return (parsed as WorldMeshRecord[]).map(resolveMeshRecord);
  } catch {
    return [];
  }
}

/** 🔪️ Splits a JSON array's TEXT into its element texts without parsing any of them — a
 * balanced scan over strings/escapes/nesting. `null` means "this is not a flat JSON array I can
 * account for", which every caller answers by falling back to a whole-document parse.
 *
 * 🪪️ This exists so mesh identity can be decided on the wire text: `meshesJson` is one string, and
 * re-parsing it hands back brand-new record objects for meshes whose bytes never moved. */
export function splitJsonArrayElements(json: string): readonly string[] | null {
  const text = json.trim();
  if (!text.startsWith("[") || !text.endsWith("]")) return null;
  const elements: string[] = [];
  let depth = 0;
  let inString = false;
  let escaped = false;
  let start = -1;
  for (let index = 1; index < text.length - 1; index += 1) {
    const character = text[index]!;
    if (inString) {
      if (escaped) escaped = false;
      else if (character === "\\") escaped = true;
      else if (character === '"') inString = false;
      continue;
    }
    if (character === '"') {
      if (start < 0) start = index;
      inString = true;
      continue;
    }
    if (character === "{" || character === "[") {
      if (start < 0) start = index;
      depth += 1;
      continue;
    }
    if (character === "}" || character === "]") {
      depth -= 1;
      if (depth < 0) return null;
      continue;
    }
    if (character === "," && depth === 0) {
      if (start < 0) return null;
      elements.push(text.slice(start, index).trim());
      start = -1;
      continue;
    }
    if (start < 0 && character.trim() !== "") start = index;
  }
  if (inString || depth !== 0) return null;
  if (start >= 0) elements.push(text.slice(start, text.length - 1).trim());
  else if (elements.length > 0) return null;
  return elements;
}

/** 🧾️ One consumer's retained mesh set and the element texts it was decoded from — the texts are the
 * identity key, so a refresh that republishes an unchanged mesh hands back the SAME record object. */
export type WorldMeshResidencyV1 = {
  readonly sources: readonly string[];
  readonly records: readonly WorldMeshRecord[];
};

/** 🧊️ Advances a retained mesh set to `meshesJson`, keeping the object IDENTITY of every mesh
 * whose wire text is unchanged. That identity is what lets the instanced-mesh layer rebuild the
 * `BufferGeometry` of exactly the meshes that moved instead of all of them: before this, a 3-mesh
 * surface re-tessellated all three whenever one upstream node re-evaluated, because the memo boundary
 * was the whole `meshesJson` string (`📓️react-perf-ceilings-audit-2026-09-14.md` §2c).
 *
 * 🪃️ A payload this cannot split is not an error — it falls back to the whole-document parse, which
 * is always correct and merely rebuilds everything, exactly as the instance-delta lane's own
 * `advanceWorldInstanceResidency` fallback does. */
export function advanceWorldMeshResidency(previous: WorldMeshResidencyV1 | null, meshesJson: string): WorldMeshResidencyV1 {
  const sources = splitJsonArrayElements(meshesJson);
  if (sources === null) return { sources: [], records: parseMeshes(meshesJson) };
  const retained = new Map<string, WorldMeshRecord>();
  const previousSources = previous?.sources ?? [];
  for (let index = 0; index < previousSources.length; index += 1) {
    const source = previousSources[index]!;
    const record = previous?.records[index];
    if (record && !retained.has(source)) retained.set(source, record);
  }
  try {
    return { sources, records: sources.map((source) => retained.get(source) ?? resolveMeshRecord(JSON.parse(source) as WorldMeshRecord)) };
  } catch {
    return { sources: [], records: parseMeshes(meshesJson) };
  }
}

export function surfaceSample(attribute: WorldMeshAttribute, mesh: WorldMeshData, corner: number): unknown {
  const vertex = mesh.indices[corner]!;
  const domain = attribute.domain === "vertex" ? vertex : attribute.domain === "corner" ? corner : attribute.domain === "face" ? Math.floor(corner / 3) : -1;
  if (domain < 0) throw new Error("a directed edge attribute cannot shade a triangle");
  const sample = attribute.indices?.[domain] ?? (attribute.indices ? -1 : domain);
  if (!Number.isInteger(sample) || sample < 0 || sample >= attribute.values.length) throw new Error("surface attribute references an invalid sample");
  return attribute.values[sample];
}

export function surfaceTuple(value: unknown, width: number, name: string): number[] {
  if (!Array.isArray(value) || value.length !== width || value.some(value => typeof value !== "number" || !Number.isFinite(value))) throw new Error(`${name} requires a finite ${width}-component tuple`);
  return value as number[];
}

export function surfaceAttributes(mesh: WorldMeshData, semantic: WorldMeshAttribute["semantic"]): [string, WorldMeshAttribute][] {
  return Object.entries(mesh.attributes ?? {}).filter(([, attribute]) => attribute.semantic === semantic).sort(([a],[b]) => a < b ? -1 : a > b ? 1 : 0);
}
//#endregion WorldSceneParsing
//#endregion 🔖️World3dHost
