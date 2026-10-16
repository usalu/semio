/** 📊️ Stat definitions and their computers over the objects of a model. */
import { ephemeralMap } from "@semio-tech/framework";
import type { Vec3 } from "@semio-tech/framework-3d-js";
import type { SpatialKernel } from "../../🗺️spatial/🟦️.ts";
import { modelDefinitionAssetModules, statOwnerByIdCache, modelDefinitionStatCatalog, modelDefinitionIdFromAssetPath } from "../📥️definition-catalog/🟦️.ts";
import { dedupeDefinitionCatalog } from "../🎨️typology/🟦️.ts";
import { objectPrimitiveRefs, Model, resolveStandalonePrimitiveKind, resolvePrimitiveRefKind } from "../🧊️model/🟦️.ts";
import { type ModelPrimitiveDocumentNode, buildModelPrimitiveDocument } from "../🧾️primitive-document/🟦️.ts";
import { listModelObjectsForModelDefinition } from "../🧭️definition-ownership/🟦️.ts";
import type { SpatialObjectRecord } from "../🟦️.ts";

// #region 📊️StatDefinitions
/** 📊️ Live stat output descriptor shipped with a model definition. */
export interface StatOutputSpec {
  readonly key: string;
  readonly label: string;
  readonly unit?: string;
  readonly format?: "integer" | "decimal" | "percent";
}

/** 📊️ Model-definition live stat declaration (`spatial.stat/v1`). */
export interface StatDefinitionSpec {
  readonly schema: "spatial.stat";
  readonly id: string;
  readonly version: string;
  readonly label: string;
  readonly description?: string;
  readonly scopes: readonly ("model" | "selection")[];
  readonly sources?: Readonly<{ readonly typologies?: readonly string[] }>;
  readonly outputs: readonly StatOutputSpec[];
}

/** 🧾️ Parses `spatial.stat/v1` JSON or returns `null`. */
export function parseStatDefinitionSpec(raw: unknown): StatDefinitionSpec | null {
  if (!raw || typeof raw !== "object") return null;
  const r = raw as Record<string, unknown>;
  if (r.schema !== "spatial.stat") return null;
  if (typeof r.id !== "string" || typeof r.version !== "string" || typeof r.label !== "string") return null;
  if (!Array.isArray(r.outputs) || r.outputs.length === 0) return null;
  const outputs: StatOutputSpec[] = [];
  for (const row of r.outputs) {
    if (!row || typeof row !== "object") return null;
    const o = row as Record<string, unknown>;
    if (typeof o.key !== "string" || typeof o.label !== "string") return null;
    outputs.push({
      key: o.key,
      label: o.label,
      unit: typeof o.unit === "string" ? o.unit : undefined,
      format: o.format === "integer" || o.format === "decimal" || o.format === "percent" ? o.format : undefined,
    });
  }
  const scopesRaw = r.scopes;
  const scopes: ("model" | "selection")[] = Array.isArray(scopesRaw) && scopesRaw.length > 0 ? scopesRaw.filter((scope): scope is "model" | "selection" => scope === "model" || scope === "selection") : ["model", "selection"];
  if (scopes.length === 0) return null;
  const sourcesRaw = r.sources;
  const sources =
    sourcesRaw && typeof sourcesRaw === "object"
      ? {
          typologies: Array.isArray((sourcesRaw as { typologies?: unknown }).typologies) ? (sourcesRaw as { typologies: string[] }).typologies.filter((id): id is string => typeof id === "string") : undefined,
        }
      : undefined;
  return {
    schema: "spatial.stat",
    id: r.id,
    version: r.version,
    label: r.label,
    description: typeof r.description === "string" ? r.description : undefined,
    scopes,
    sources: sources?.typologies?.length ? sources : undefined,
    outputs,
  };
}

function shippedStatDefinitionCatalog(): readonly StatDefinitionSpec[] {
  return dedupeDefinitionCatalog(
    modelDefinitionStatCatalog()
      .map((raw) => parseStatDefinitionSpec(raw))
      .filter((spec): spec is StatDefinitionSpec => spec !== null),
  );
}

/** 📚️ Lists stat definitions from model-definition assets. */
export function listModelDefinitionStatDefinitions(): readonly StatDefinitionSpec[] {
  return shippedStatDefinitionCatalog();
}

/** 📚️ Loads a stat definition by stable `id`. */
export function loadStatDefinition(statId: string): StatDefinitionSpec | null {
  return shippedStatDefinitionCatalog().find((row) => row.id === statId) ?? null;
}

function statOwnerById(): ReadonlyMap<string, string> {
  if (statOwnerByIdCache.current) return statOwnerByIdCache.current;
  const map = new Map<string, string>();
  for (const [path, raw] of Object.entries(modelDefinitionAssetModules.current.statDefinitions)) {
    const owner = modelDefinitionIdFromAssetPath(path);
    const spec = parseStatDefinitionSpec(raw);
    if (!owner || !spec) continue;
    map.set(spec.id, owner);
  }
  statOwnerByIdCache.current = map;
  return map;
}

/** 🧭️ Stat definitions owned by a model definition. */
export function listStatDefinitionsForModelDefinition(modelDefinitionId: string): readonly StatDefinitionSpec[] {
  return shippedStatDefinitionCatalog().filter((row) => statOwnerById().get(row.id) === modelDefinitionId);
}

/** ✅️ Whether a stat definition supports one compute scope. */
export function statDefinitionAppliesToScope(defn: StatDefinitionSpec, scope: "model" | "selection"): boolean {
  return defn.scopes.includes(scope);
}

/** 🧾️ Formats one stat output value for display. */
export function formatStatOutputValue(value: number, format?: StatOutputSpec["format"]): string {
  if (!Number.isFinite(value)) return "—";
  if (format === "integer") return String(Math.round(value));
  if (format === "percent") return `${(value * 100).toFixed(1)}%`;
  if (format === "decimal") return value.toLocaleString(undefined, { maximumFractionDigits: 3 });
  return String(value);
}

/** ✅️ Whether `object` is included in stat sources for one definition. */
export function statDefinitionAppliesToObject(defn: StatDefinitionSpec, object: SpatialObjectRecord): boolean {
  const typologies = defn.sources?.typologies;
  if (Array.isArray(typologies) && typologies.length > 0) return typologies.includes(object.typology);
  return true;
}

/** 📊️ Context passed to registered stat computers. */
export interface StatComputeContext {
  readonly model: Model;
  readonly kernel: SpatialKernel;
  readonly modelDefinitionId: string;
  readonly scope: "model" | "selection";
  readonly objects: readonly SpatialObjectRecord[];
}

/** 📊️ Registered stat computer for one stat definition id. */
export type StatComputer = (ctx: StatComputeContext) => Promise<Record<string, number>>;

const statComputers = ephemeralMap<string, StatComputer>("s.spatial-kernel.geometry.statComputers");

/** 📊️ Registers a TypeScript computer for one stat definition id. */
export function registerStatComputer(statId: string, computer: StatComputer): void {
  statComputers.set(statId, computer);
}

function zeroStatOutputs(defn: StatDefinitionSpec): Record<string, number> {
  const out: Record<string, number> = {};
  for (const row of defn.outputs) out[row.key] = 0;
  return out;
}

export function collectSolidRefsForObjects(model: Model, objects: readonly SpatialObjectRecord[]): string[] {
  const ids = new Set<string>();
  for (const object of objects) {
    for (const primitiveRef of objectPrimitiveRefs(object)) {
      if (resolvePrimitiveRefKind(model, primitiveRef) === "solid") ids.add(primitiveRef);
    }
  }
  return [...ids];
}

export function collectFaceRefsForObjects(model: Model, objects: readonly SpatialObjectRecord[]): string[] {
  const ids = new Set<string>();
  for (const object of objects) {
    for (const primitiveRef of objectPrimitiveRefs(object)) {
      const kind = resolveStandalonePrimitiveKind(model, primitiveRef);
      if (kind === "surface") {
        ids.add(primitiveRef);
        continue;
      }
      if (kind === "solid") {
        const solid = model.solids[primitiveRef];
        if (!solid) continue;
        for (const shellId of solid.shellIds) {
          const shell = model.shells[shellId];
          if (!shell) continue;
          for (const faceId of shell.faceIds) ids.add(faceId);
        }
      }
    }
  }
  return [...ids];
}

export function collectVertexPositionsForObjects(model: Model, objects: readonly SpatialObjectRecord[]): Vec3[] {
  const positions: Vec3[] = [];
  const seen = new Set<string>();
  const visitPrimitive = (primitiveRef: string): void => {
    if (seen.has(primitiveRef)) return;
    seen.add(primitiveRef);
    const kind = resolvePrimitiveRefKind(model, primitiveRef);
    if (kind === "vertex") {
      const vertex = model.vertices[primitiveRef];
      if (vertex) positions.push(vertex.position);
      return;
    }
    if (kind === "anchor") {
      const anchor = model.anchors[primitiveRef];
      if (anchor) positions.push(anchor.position);
      return;
    }
    const tree = buildModelPrimitiveDocument(model, primitiveRef);
    if (!tree) return;
    const walk = (node: ModelPrimitiveDocumentNode): void => {
      if (node.kind === "vertex") {
        const vertex = model.vertices[node.id];
        if (vertex) positions.push(vertex.position);
        return;
      }
      if (node.kind === "anchor") {
        const anchor = model.anchors[node.id];
        if (anchor) positions.push(anchor.position);
        return;
      }
      for (const child of node.children) walk(child);
    };
    walk(tree);
  };
  for (const object of objects) {
    for (const primitiveRef of objectPrimitiveRefs(object)) visitPrimitive(primitiveRef);
  }
  return positions;
}

export function bboxSizesFromPositions(positions: readonly Vec3[]): { readonly sizeX: number; readonly sizeY: number; readonly sizeZ: number } {
  if (positions.length === 0) return { sizeX: 0, sizeY: 0, sizeZ: 0 };
  let minX = positions[0]![0];
  let minY = positions[0]![1];
  let minZ = positions[0]![2];
  let maxX = minX;
  let maxY = minY;
  let maxZ = minZ;
  for (const [x, y, z] of positions) {
    minX = Math.min(minX, x);
    minY = Math.min(minY, y);
    minZ = Math.min(minZ, z);
    maxX = Math.max(maxX, x);
    maxY = Math.max(maxY, y);
    maxZ = Math.max(maxZ, z);
  }
  return { sizeX: maxX - minX, sizeY: maxY - minY, sizeZ: maxZ - minZ };
}

/** 📊️ Derives live stat output for one definition and scope. */
export async function computeStat(defn: StatDefinitionSpec, ctx: StatComputeContext): Promise<Record<string, number>> {
  if (!statDefinitionAppliesToScope(defn, ctx.scope)) return zeroStatOutputs(defn);
  const objects = ctx.objects.filter((object) => statDefinitionAppliesToObject(defn, object));
  if (objects.length === 0) return zeroStatOutputs(defn);
  const computer = statComputers.get(defn.id);
  if (!computer) return zeroStatOutputs(defn);
  const values = await computer({ ...ctx, objects });
  const out = zeroStatOutputs(defn);
  for (const row of defn.outputs) {
    const value = values[row.key];
    out[row.key] = typeof value === "number" && Number.isFinite(value) ? value : 0;
  }
  return out;
}

/** 📊️ Objects included when computing stats for one model definition scope. */
export function objectsForStatCompute(model: Model, modelDefinitionId: string, defn: StatDefinitionSpec, scope: "model" | "selection", selectionObjects: readonly SpatialObjectRecord[]): readonly SpatialObjectRecord[] {
  const typologyFilter = defn.sources?.typologies;
  let base: readonly SpatialObjectRecord[];
  if (scope === "selection") {
    base = selectionObjects.filter((object) => model.objects[object.id]);
  } else if (typologyFilter && typologyFilter.length > 0) {
    base = Object.values(model.objects).filter((object) => typologyFilter.includes(object.typology));
  } else {
    base = listModelObjectsForModelDefinition(model, modelDefinitionId);
  }
  return base.filter((object) => statDefinitionAppliesToObject(defn, object));
}

// #endregion 📊️StatDefinitions
