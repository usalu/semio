/** 🔁️ Applying a transformation to a model: appliers, derive matching, classification and the derive runner. */
import { ephemeralMap } from "@semio-tech/framework";
import type { Vec3 } from "@semio-tech/framework-3d-js";
import type { SpatialPreviewKernel } from "../../🗺️spatial/🟦️.ts";
import { objectPrimitiveEntries, Model } from "../🧊️model/🟦️.ts";
import { listModelObjectsForModelDefinition } from "../🧭️definition-ownership/🟦️.ts";
import { qualifiedTransformationId, type TransformationDeriveClassifyRule, type TransformationDeriveOpening, type TransformationDeriveSpec, type TransformationSpec } from "../🔄️transformation-spec/🟦️.ts";
import { type FaceRef, type SolidRef, type ObjectRef, type TypologyRef, sortedRecordValues, type TypologyPrimitiveKind } from "../🟦️.ts";

// #region 🔄️TransformationGeometry
/** 🔄️ Registered transformation applier for one qualified transformation id. */
export type TransformationApplier = (spec: TransformationSpec, source: Model) => Model;

const transformationAppliers = ephemeralMap<string, TransformationApplier>("s.spatial-kernel.geometry.transformationAppliers");

/** 🔄️ Registers a model-definition-specific transformation implementation. */
export function registerTransformationApplier(qualifiedTransformationId: string, applier: TransformationApplier): void {
  transformationAppliers.set(qualifiedTransformationId, applier);
}

function collectTransformationPrimitiveRefs(model: Model, sourceModelDefinition: string, primitiveKind: TypologyPrimitiveKind): readonly SolidRef[] {
  const refs = new Set<string>();
  for (const obj of listModelObjectsForModelDefinition(model, sourceModelDefinition)) {
    for (const [kind, primitiveRef] of objectPrimitiveEntries(obj)) {
      if (kind === primitiveKind && model.solids[primitiveRef]) refs.add(primitiveRef);
    }
  }
  if (!refs.size) {
    for (const id of Object.keys(model.solids)) refs.add(id);
  }
  return [...refs].sort().map((id) => id as SolidRef);
}

export function transformationObjectId(typology: string, index: number): ObjectRef {
  return (index === 0 ? typology : `${typology}#${index}`) as ObjectRef;
}

function deriveOpeningMatch(model: Model, faceId: FaceRef, opening: TransformationDeriveOpening): boolean {
  const fields = model.metadata.get(String(faceId));
  if (!fields) return false;
  for (const field of opening.fields) {
    const value = fields[field];
    if (opening.values.some((candidate) => candidate === value)) return true;
  }
  return false;
}

function deriveClassifyRuleMatches(rule: TransformationDeriveClassifyRule, normal: Vec3, centroid: Vec3, zMin: number, zMax: number, zTol: number): boolean {
  if (rule.fallback) return true;
  const ax = Math.abs(normal[0]);
  const ay = Math.abs(normal[1]);
  const az = Math.abs(normal[2]);
  if (rule.dominantAxis === "z" && rule.minDominantNormal != null) {
    if (!(az >= ax && az >= ay && az >= rule.minDominantNormal)) return false;
    if (rule.zBand === "max") return centroid[2] >= zMax - zTol;
    if (rule.zBand === "min") return centroid[2] <= zMin + zTol;
    return true;
  }
  if (rule.minAxisNormal != null) return ax >= rule.minAxisNormal || ay >= rule.minAxisNormal;
  return false;
}

function classifyFaceFromDeriveRules(derive: TransformationDeriveSpec, normal: Vec3, centroid: Vec3, zMin: number, zMax: number, zTol: number): TransformationDeriveClassifyRule {
  for (const rule of derive.classify.rules) {
    if (deriveClassifyRuleMatches(rule, normal, centroid, zMin, zMax, zTol)) return rule;
  }
  return derive.classify.rules[derive.classify.rules.length - 1]!;
}

export function cloneModelGeometryShell(source: Model): Model {
  const target = new Model();
  target.revision = source.revision;
  target.anchors = source.anchors;
  target.vertices = source.vertices;
  target.edges = source.edges;
  target.wires = source.wires;
  target.faces = source.faces;
  target.shells = { ...source.shells };
  target.solids = { ...source.solids };
  return target;
}

/** 🔄️ Copies geometry and keeps only object rows whose typology is listed on the transformation spec. */
function applyTransformationFallback(spec: TransformationSpec, source: Model): Model {
  const target = cloneModelGeometryShell(source);
  const allowedTypologies = new Set(spec.typologies);
  const objects: Model["objects"] = {};
  for (const row of Object.values(source.objects)) {
    if (!allowedTypologies.has(row.typology)) continue;
    objects[row.id] = row;
  }
  target.objects = objects;
  target.bump();
  return target;
}

function runDeriveTransformation(spec: TransformationSpec, source: Model, preview: SpatialPreviewKernel): Model {
  const derive = spec.derive;
  if (!derive) throw new Error(`transformation ${qualifiedTransformationId(spec.modelDefinitionId, spec.id)} is missing derive`);
  const target = cloneModelGeometryShell(source);
  const solidRefs = collectTransformationPrimitiveRefs(source, derive.collect.sourceModelDefinition, derive.collect.primitiveKind);
  if (!solidRefs.length) {
    target.bump();
    return target;
  }
  const sourceObjectIds = sortedRecordValues(source.objects).map((row) => String(row.id));
  const { hullSolid, externalFaces } = preview.fuseSolidsToExternalFaces(target, solidRefs, {
    hullSolidId: derive.fuse?.hullSolidId,
    contactPairs: derive.fuse?.contactPairs,
    maxSeparation: derive.fuse?.maxSeparation,
  });
  if (solidRefs.length > 1) {
    target.metadata.setField(String(hullSolid), "fuseSourceSolidIds", solidRefs.map(String));
    target.solids[hullSolid] = { id: hullSolid, shellIds: [] };
  }
  const centroids: Vec3[] = [];
  const faceMeta: { readonly face: FaceRef; readonly normal: Vec3; readonly centroid: Vec3 }[] = [];
  for (const faceId of externalFaces) {
    const face = target.faces[faceId];
    if (!face) continue;
    const normal = preview.faceNormal(target, face);
    const centroid = preview.faceCentroid(target, face);
    if (!normal || !centroid) continue;
    faceMeta.push({ face: faceId, normal, centroid });
    centroids.push(centroid);
  }
  let zMin = Infinity;
  let zMax = -Infinity;
  for (const c of centroids) {
    zMin = Math.min(zMin, c[2]);
    zMax = Math.max(zMax, c[2]);
  }
  if (!Number.isFinite(zMin)) {
    zMin = 0;
    zMax = 0;
  }
  const zTolRatio = derive.classify.zTolRatio ?? 0.02;
  const zTolMin = derive.classify.zTolMin ?? 1e-3;
  const zTol = Math.max((zMax - zMin) * zTolRatio, zTolMin);
  const roleCounts = new Map<string, number>();
  const hullTypology = derive.hull.typology as TypologyRef;
  target.objects[transformationObjectId(hullTypology, 0)] = {
    id: transformationObjectId(hullTypology, 0),
    typology: hullTypology,
    primitives: { [derive.hull.primitiveKind]: String(hullSolid) },
    attributes: { sourceObjectIds, fusedSolidIds: solidRefs.map(String) },
  };
  const mergeRoles = new Set(derive.classify.mergeRoles ?? []);
  const mergeByPlane = derive.classify.mergeByPlane === true;
  const groupedFaces = new Map<string, { readonly role: string; readonly typology: TypologyRef; readonly faces: FaceRef[] }>();
  for (const row of faceMeta) {
    const opening = derive.classify.opening;
    const classified =
      opening && deriveOpeningMatch(source, row.face, opening)
        ? { role: opening.role, typology: opening.typology as TypologyRef }
        : (() => {
            const rule = classifyFaceFromDeriveRules(derive, row.normal, row.centroid, zMin, zMax, zTol);
            return { role: rule.role, typology: rule.typology as TypologyRef };
          })();
    const mergeGroup = mergeRoles.has(classified.role);
    const groupKey = mergeGroup && mergeByPlane ? `${classified.typology}:${preview.facePlaneGroupKey(row.normal, row.centroid)}` : `${classified.typology}:${String(row.face)}`;
    const existing = groupedFaces.get(groupKey);
    if (existing) existing.faces.push(row.face);
    else groupedFaces.set(groupKey, { role: classified.role, typology: classified.typology, faces: [row.face] });
  }
  for (const group of groupedFaces.values()) {
    const index = roleCounts.get(group.typology) ?? 0;
    roleCounts.set(group.typology, index + 1);
    const objectId = transformationObjectId(group.typology, index);
    target.objects[objectId] = {
      id: objectId,
      typology: group.typology,
      primitives: { surface: String(group.faces[0]!) },
      attributes: { sourceObjectIds, surfaceRole: group.role, faceIds: group.faces.map(String) },
    };
  }
  for (const ensure of derive.ensure ?? []) {
    if (!roleCounts.has(ensure.typology)) {
      const objectId = transformationObjectId(ensure.typology, 0);
      target.objects[objectId] = {
        id: objectId,
        typology: ensure.typology as TypologyRef,
        primitives: ensure.empty ? {} : { surface: String(externalFaces[0] ?? "") },
        attributes: { sourceObjectIds },
      };
    }
  }
  target.bump();
  return target;
}

// #endregion 🔄️TransformationGeometry

/** 🔄️ Derives a target-definition model from a source model (shared geometry, new object rows). */
export function applyTransformation(spec: TransformationSpec, source: Model, preview: SpatialPreviewKernel): Model {
  const qualified = qualifiedTransformationId(spec.modelDefinitionId, spec.id);
  const applier = transformationAppliers.get(qualified);
  if (applier) return applier(spec, source);
  if (spec.derive) return runDeriveTransformation(spec, source, preview);
  return applyTransformationFallback(spec, source);
}
