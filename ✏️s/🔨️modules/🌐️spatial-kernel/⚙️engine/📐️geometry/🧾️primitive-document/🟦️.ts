/** 🧾️ Primitive kind resolution, the primitive document of a model and the property computers that derive values from it. */
import { ephemeralMap } from "@semio-tech/framework";
import type { SpatialKernel } from "../../🗺️spatial/🟦️.ts";
import { kernelTypologyIds } from "../📥️definition-catalog/🟦️.ts";
import { type TypologySpec, typologyPrimitiveToEntityKind, loadTypology, type PropertyDefinitionSpec, shippedPropertyDefinitionCatalog } from "../🎨️typology/🟦️.ts";
import { objectPrimitiveRefs, Model, resolveStandalonePrimitiveKind, resolvePrimitiveRefKind } from "../🧊️model/🟦️.ts";
import { listTypologiesForModelDefinition, listPropertyDefinitionsForModelDefinition } from "../🧭️definition-ownership/🟦️.ts";
import type { ModelEntityKind, SpatialObjectRecord, TypologyPrimitiveKind, KernelTopologyKind } from "../🟦️.ts";

/** 🌳️ Nested primitive node under an object primitive (`solid` → `shell` → `face` → `wire` → `edge` → `vertex`). */
export interface ModelPrimitiveDocumentNode {
  readonly kind: KernelTopologyKind;
  readonly id: string;
  readonly children: readonly ModelPrimitiveDocumentNode[];
}

function sortedPrimitiveChildIds(ids: readonly string[]): string[] {
  return [...ids].sort((a, b) => a.localeCompare(b));
}

function buildModelPrimitiveDocumentNode(model: Model, kind: KernelTopologyKind, id: string): ModelPrimitiveDocumentNode | null {
  const children: ModelPrimitiveDocumentNode[] = [];
  switch (kind) {
    case "solid": {
      const solid = model.solids[id];
      if (!solid) return null;
      for (const shellId of sortedPrimitiveChildIds(solid.shellIds)) {
        const child = buildModelPrimitiveDocumentNode(model, "shell", shellId);
        if (child) children.push(child);
      }
      break;
    }
    case "shell": {
      const shell = model.shells[id];
      if (!shell) return null;
      for (const faceId of sortedPrimitiveChildIds(shell.faceIds)) {
        const child = buildModelPrimitiveDocumentNode(model, "face", faceId);
        if (child) children.push(child);
      }
      break;
    }
    case "face": {
      const face = model.faces[id];
      if (!face) return null;
      for (const wireId of sortedPrimitiveChildIds(face.wireIds)) {
        const child = buildModelPrimitiveDocumentNode(model, "wire", wireId);
        if (child) children.push(child);
      }
      break;
    }
    case "wire": {
      const wire = model.wires[id];
      if (!wire) return null;
      for (const edgeId of sortedPrimitiveChildIds(wire.edgeIds)) {
        const child = buildModelPrimitiveDocumentNode(model, "edge", edgeId);
        if (child) children.push(child);
      }
      break;
    }
    case "edge": {
      const edge = model.edges[id];
      if (!edge) return null;
      for (const vertexId of sortedPrimitiveChildIds(edge.vertexIds)) {
        const child = buildModelPrimitiveDocumentNode(model, "vertex", vertexId);
        if (child) children.push(child);
      }
      break;
    }
    case "vertex":
      if (!model.vertices[id]) return null;
      break;
    case "anchor":
      if (!model.anchors[id]) return null;
      break;
  }
  return { kind, id, children };
}

/** 🌳️ Builds nested primitive document under one object primitive ref in `model`. */
export function buildModelPrimitiveDocument(model: Model, primitiveRef: string): ModelPrimitiveDocumentNode | null {
  const kind = resolvePrimitiveRefKind(model, primitiveRef);
  if (!kind) return null;
  return buildModelPrimitiveDocumentNode(model, kind, primitiveRef);
}

/** ✅️ Whether `typology` allows objects whose geometry resolves to `primitiveKind`. */
export function typologyAllowsPrimitiveKind(typology: TypologySpec, primitiveKind: TypologyPrimitiveKind): boolean {
  return typology.primitiveKinds.includes(primitiveKind);
}

/** ✅️ Whether `object` on `model` satisfies its typology `primitiveKinds`. */
export function objectMatchesTypologyPrimitives(model: Model, object: SpatialObjectRecord): boolean {
  const typology = loadTypology(object.typology);
  if (!typology || typology.primitiveKinds.length === 0) return false;
  const primitiveKinds = objectPrimitiveRefs(object)
    .map((primitiveRef) => resolveStandalonePrimitiveKind(model, primitiveRef))
    .filter((kind): kind is TypologyPrimitiveKind => kind !== null);
  return primitiveKinds.length > 0 && primitiveKinds.every((kind) => typologyAllowsPrimitiveKind(typology, kind));
}

/** 🧭️ Typology → entity kind map for one model definition (`ModelDefinition` includes kernel typology ids; AEC typologies map to `object`). */
export function buildTypologyToEntityKindMapForModelDefinition(modelDefinitionId: string): Readonly<Record<string, ModelEntityKind>> {
  const out: Record<string, ModelEntityKind> = {};
  const kernelTypologies = kernelTypologyIds(modelDefinitionId);
  if (kernelTypologies) {
    for (const [kind, id] of Object.entries(kernelTypologies)) {
      if (typeof id === "string" && id.length > 0) out[id] = kind as ModelEntityKind;
    }
    for (const spec of listTypologiesForModelDefinition(modelDefinitionId)) {
      if (spec.primitiveKinds.length !== 1) continue;
      const kind = typologyPrimitiveToEntityKind(spec.primitiveKinds[0]!);
      if (kind === "anchor" && !spec.id.includes("entity") && !spec.id.includes("measure")) continue;
      out[spec.id] = kind;
    }
    return out;
  }
  for (const spec of listTypologiesForModelDefinition(modelDefinitionId)) out[spec.id] = "object";
  return out;
}

/** ✅️ Whether a property definition applies to `object` on `model`. */
export function propertyDefinitionAppliesToObject(defn: PropertyDefinitionSpec, object: SpatialObjectRecord): boolean {
  const typologies = defn.sources?.typologies;
  if (Array.isArray(typologies) && typologies.length > 0) return typologies.includes(object.typology);
  return true;
}

/** 📐️ Context passed to registered property computers. */
export interface PropertyComputeContext {
  readonly model: Model;
  readonly kernel: SpatialKernel;
  readonly object: SpatialObjectRecord;
  readonly defn: PropertyDefinitionSpec;
}

/** 📐️ Registered property computer for one property definition id. */
export type PropertyComputer = (ctx: PropertyComputeContext) => Promise<Record<string, unknown>>;

const propertyComputers = ephemeralMap<string, PropertyComputer>("s.spatial-kernel.geometry.propertyComputers");

/** 📐️ Registers a TypeScript computer for one property definition id. */
export function registerPropertyComputer(propertyId: string, computer: PropertyComputer): void {
  propertyComputers.set(propertyId, computer);
}

/** 📐️ Derives property output for one model object from a property definition. */
export async function derivePropertyValue(defn: PropertyDefinitionSpec, ctx: { readonly model: Model; readonly kernel: SpatialKernel; readonly object: SpatialObjectRecord }): Promise<Record<string, unknown>> {
  if (!propertyDefinitionAppliesToObject(defn, ctx.object)) return {};
  const computer = propertyComputers.get(defn.id);
  if (computer) return computer({ ...ctx, defn });
  const output = defn.output ?? {};
  return { ...output };
}

/** 📚️ Property definitions for one model definition that apply to `object` on `model`. */
export function listApplicablePropertyDefinitionsForModelDefinition(modelDefinitionId: string, model: Model, object: SpatialObjectRecord): readonly PropertyDefinitionSpec[] {
  const scoped = new Set(listPropertyDefinitionsForModelDefinition(modelDefinitionId).map((row) => row.id));
  return shippedPropertyDefinitionCatalog().filter((defn) => scoped.has(defn.id) && propertyDefinitionAppliesToObject(defn, object));
}
