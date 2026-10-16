/** 🔄️ Transformation specs: the derive rules, their parsers and the shipped transformation catalog. */
import { modelDefinitionTransformationModules, modelDefinitionIdFromAssetPath } from "../📥️definition-catalog/🟦️.ts";
import type { TypologyPrimitiveKind } from "../🟦️.ts";

/** 🪪️ Qualified transformation id (`modelDefinitionId.transformationId`). */
export function qualifiedTransformationId(modelDefinitionId: string, transformationId: string): string {
  return `${modelDefinitionId}.${transformationId}`;
}

/** 🔄️ Z-band selector for surface classification rules. */
export type TransformationDeriveZBand = "min" | "max" | "mid";

/** 🔄️ One surface-classification rule in a transformation `derive` block. */
export interface TransformationDeriveClassifyRule {
  readonly role: string;
  readonly typology: string;
  readonly dominantAxis?: "x" | "y" | "z";
  readonly minDominantNormal?: number;
  readonly minAxisNormal?: number;
  readonly zBand?: TransformationDeriveZBand;
  readonly fallback?: boolean;
}

/** 🔄️ Opening metadata → typology mapping in a transformation `derive` block. */
export interface TransformationDeriveOpening {
  readonly fields: readonly string[];
  readonly values: readonly (string | boolean)[];
  readonly typology: string;
  readonly role: string;
}

/** 🔄️ Solid-fuse options for a transformation `derive` block. */
export interface TransformationDeriveFuse {
  readonly hullSolidId?: string;
  readonly contactPairs?: readonly (readonly [string, string])[];
  readonly maxSeparation?: number;
}

/** 🔄️ Source primitive collection for a transformation `derive` block. */
export interface TransformationDeriveCollect {
  readonly sourceModelDefinition: string;
  readonly primitiveKind: TypologyPrimitiveKind;
}

/** 🔄️ Hull object row for a transformation `derive` block. */
export interface TransformationDeriveHull {
  readonly typology: string;
  readonly primitiveKind: string;
}

/** 🔄️ Ensures typology rows exist after derive. */
export interface TransformationDeriveEnsure {
  readonly typology: string;
  readonly empty?: boolean;
}

/** 🔄️ Declarative surface-classification derive spec on `spatial.transformation/v1`. */
export interface TransformationDeriveSpec {
  readonly collect: TransformationDeriveCollect;
  readonly fuse?: TransformationDeriveFuse;
  readonly hull: TransformationDeriveHull;
  readonly classify: {
    readonly zTolRatio?: number;
    readonly zTolMin?: number;
    readonly rules: readonly TransformationDeriveClassifyRule[];
    readonly opening?: TransformationDeriveOpening;
    readonly mergeRoles?: readonly string[];
    readonly mergeByPlane?: boolean;
  };
  readonly ensure?: readonly TransformationDeriveEnsure[];
}

/** 🔄️ Parsed transformation (`spatial.transformation/v1`). */
export interface TransformationSpec {
  readonly schema: "spatial.transformation";
  readonly id: string;
  readonly version: string;
  readonly label: string;
  readonly description?: string;
  readonly modelDefinitionId: string;
  readonly source: { readonly modelDefinition: string };
  readonly target: { readonly modelDefinition: string };
  readonly typologies: readonly string[];
  readonly derive?: TransformationDeriveSpec;
}

function parseTransformationDeriveClassifyRule(raw: unknown): TransformationDeriveClassifyRule | null {
  if (!raw || typeof raw !== "object") return null;
  const r = raw as Record<string, unknown>;
  if (typeof r.role !== "string" || typeof r.typology !== "string") return null;
  const dominantAxis = r.dominantAxis === "x" || r.dominantAxis === "y" || r.dominantAxis === "z" ? r.dominantAxis : undefined;
  const zBand = r.zBand === "min" || r.zBand === "max" || r.zBand === "mid" ? r.zBand : undefined;
  return {
    role: r.role,
    typology: r.typology,
    ...(dominantAxis ? { dominantAxis } : {}),
    ...(typeof r.minDominantNormal === "number" ? { minDominantNormal: r.minDominantNormal } : {}),
    ...(typeof r.minAxisNormal === "number" ? { minAxisNormal: r.minAxisNormal } : {}),
    ...(zBand ? { zBand } : {}),
    ...(r.fallback === true ? { fallback: true } : {}),
  };
}

function parseTransformationDeriveSpec(raw: unknown): TransformationDeriveSpec | null {
  if (!raw || typeof raw !== "object") return null;
  const r = raw as Record<string, unknown>;
  const collect = r.collect as Record<string, unknown> | undefined;
  const hull = r.hull as Record<string, unknown> | undefined;
  const classify = r.classify as Record<string, unknown> | undefined;
  if (typeof collect?.sourceModelDefinition !== "string") return null;
  const primitiveKind = collect.primitiveKind;
  const allowed = new Set<TypologyPrimitiveKind>(["anchor", "solid", "surface", "curve"]);
  if (typeof primitiveKind !== "string" || !allowed.has(primitiveKind as TypologyPrimitiveKind)) return null;
  if (typeof hull?.typology !== "string" || typeof hull?.primitiveKind !== "string") return null;
  if (!classify || !Array.isArray(classify.rules) || classify.rules.length === 0) return null;
  const rules = classify.rules.map(parseTransformationDeriveClassifyRule).filter((row): row is TransformationDeriveClassifyRule => row !== null);
  if (rules.length !== classify.rules.length) return null;
  const fuseRaw = r.fuse as Record<string, unknown> | undefined;
  const fuse: TransformationDeriveFuse | undefined =
    fuseRaw && typeof fuseRaw === "object"
      ? {
          ...(typeof fuseRaw.hullSolidId === "string" ? { hullSolidId: fuseRaw.hullSolidId } : {}),
          ...(Array.isArray(fuseRaw.contactPairs)
            ? {
                contactPairs: fuseRaw.contactPairs.filter((pair): pair is [string, string] => Array.isArray(pair) && pair.length === 2 && typeof pair[0] === "string" && typeof pair[1] === "string").map((pair) => [pair[0], pair[1]] as const),
              }
            : {}),
          ...(typeof fuseRaw.maxSeparation === "number" ? { maxSeparation: fuseRaw.maxSeparation } : {}),
        }
      : undefined;
  const openingRaw = classify.opening as Record<string, unknown> | undefined;
  const opening: TransformationDeriveOpening | undefined =
    openingRaw && Array.isArray(openingRaw.fields) && openingRaw.fields.every((f) => typeof f === "string") && Array.isArray(openingRaw.values) && typeof openingRaw.typology === "string" && typeof openingRaw.role === "string"
      ? {
          fields: openingRaw.fields as string[],
          values: openingRaw.values as (string | boolean)[],
          typology: openingRaw.typology,
          role: openingRaw.role,
        }
      : undefined;
  const ensure: TransformationDeriveEnsure[] | undefined = Array.isArray(r.ensure)
    ? r.ensure
        .map((row) => {
          if (!row || typeof row !== "object") return null;
          const e = row as Record<string, unknown>;
          if (typeof e.typology !== "string") return null;
          return { typology: e.typology, ...(e.empty === true ? { empty: true } : {}) };
        })
        .filter((row): row is TransformationDeriveEnsure => row !== null)
    : undefined;
  return {
    collect: { sourceModelDefinition: collect.sourceModelDefinition, primitiveKind: primitiveKind as TypologyPrimitiveKind },
    ...(fuse ? { fuse } : {}),
    hull: { typology: hull.typology, primitiveKind: hull.primitiveKind },
    classify: {
      rules,
      ...(typeof classify.zTolRatio === "number" ? { zTolRatio: classify.zTolRatio } : {}),
      ...(typeof classify.zTolMin === "number" ? { zTolMin: classify.zTolMin } : {}),
      ...(opening ? { opening } : {}),
      ...(Array.isArray(classify.mergeRoles) ? { mergeRoles: classify.mergeRoles.filter((x): x is string => typeof x === "string") } : {}),
      ...(classify.mergeByPlane === true ? { mergeByPlane: true } : {}),
    },
    ...(ensure?.length ? { ensure } : {}),
  };
}

/** 🧾️ Parses `spatial.transformation/v1` JSON; `modelDefinitionId` comes from the asset folder. */
export function parseTransformationSpec(raw: unknown, modelDefinitionId: string): TransformationSpec | null {
  if (!raw || typeof raw !== "object") return null;
  const r = raw as Record<string, unknown>;
  if (r.schema !== "spatial.transformation") return null;
  if (typeof r.id !== "string" || typeof r.version !== "string" || typeof r.label !== "string") return null;
  const source = r.source as { modelDefinition?: unknown } | undefined;
  const target = r.target as { modelDefinition?: unknown } | undefined;
  if (typeof source?.modelDefinition !== "string" || typeof target?.modelDefinition !== "string") return null;
  if (!Array.isArray(r.typologies) || r.typologies.length === 0) return null;
  const typologies = r.typologies.filter((row): row is string => typeof row === "string");
  if (typologies.length !== r.typologies.length) return null;
  const derive = r.derive !== undefined ? parseTransformationDeriveSpec(r.derive) : undefined;
  if (r.derive !== undefined && !derive) return null;
  return {
    schema: "spatial.transformation",
    id: r.id,
    version: r.version,
    label: r.label,
    description: typeof r.description === "string" ? r.description : undefined,
    modelDefinitionId,
    source: { modelDefinition: source.modelDefinition },
    target: { modelDefinition: target.modelDefinition },
    typologies,
    ...(derive ? { derive } : {}),
  };
}

function modelDefinitionIdFromTransformationAssetPath(assetPath: string): string | null {
  return modelDefinitionIdFromAssetPath(assetPath);
}

function shippedTransformationCatalog(): readonly TransformationSpec[] {
  const seen = new Set<string>();
  const out: TransformationSpec[] = [];
  for (const [path, raw] of Object.entries(modelDefinitionTransformationModules())) {
    const modelDefinitionId = modelDefinitionIdFromTransformationAssetPath(path);
    if (!modelDefinitionId) continue;
    const spec = parseTransformationSpec(raw, modelDefinitionId);
    if (!spec) continue;
    const qid = qualifiedTransformationId(spec.modelDefinitionId, spec.id);
    if (seen.has(qid)) continue;
    seen.add(qid);
    out.push(spec);
  }
  return out;
}

/** 📚️ Lists transformation assets under spatial/asset/modelDefinition. */
export function listModelDefinitionTransformations(): readonly TransformationSpec[] {
  return shippedTransformationCatalog();
}

/** 📚️ Loads a transformation by qualified id (`aec.building.energy.from_geometry`). */
export function loadTransformation(qualifiedId: string): TransformationSpec | null {
  return shippedTransformationCatalog().find((row) => qualifiedTransformationId(row.modelDefinitionId, row.id) === qualifiedId) ?? null;
}

/** 🔄️ Lists transformations whose target is `modelDefinitionId` (derive current definition from source). */
export function listTransformationsIntoModelDefinition(modelDefinitionId: string): readonly TransformationSpec[] {
  return listModelDefinitionTransformations().filter((row) => row.target.modelDefinition === modelDefinitionId);
}

/** 🔄️ Lists transformations whose source is `modelDefinitionId` (derive another definition from current). */
export function listTransformationsFromModelDefinition(modelDefinitionId: string): readonly TransformationSpec[] {
  return listModelDefinitionTransformations().filter((row) => row.source.modelDefinition === modelDefinitionId);
}
