/** 📥️ The model definition catalog: manifests, registered asset modules, their caches and listeners, and asset path ownership. */
import { ephemeralBox, ephemeralMap } from "@semio-tech/framework";
import type { KernelTopologyKind } from "../🟦️.ts";

/** 🏷️ Parsed model-definition manifest (`spatial.modelDefinition/v1` on disk). */
export interface ModelDefinitionManifest {
  readonly schema: "spatial.modelDefinition";
  readonly id: string;
  readonly version: string;
  readonly label: string;
  readonly description?: string;
  readonly kinds: readonly string[];
  readonly default?: boolean;
  readonly baseObjectTypology?: string;
  readonly kernelTypologies?: Readonly<Partial<Record<KernelTopologyKind, string>>>;
}

// #region 📥️ModelDefinitionCatalog
/** 📥️ Model-definition contributions supplied by independent owners. */
export interface ModelDefinitionAssetModules {
  readonly typologies: Readonly<Record<string, unknown>>;
  readonly actions: Readonly<Record<string, unknown>>;
  readonly interactions: Readonly<Record<string, unknown>>;
  readonly manifests: Readonly<Record<string, unknown>>;
  readonly extensions: Readonly<Record<string, unknown>>;
  readonly attributes: Readonly<Record<string, unknown>>;
  readonly propertyDefinitions: Readonly<Record<string, unknown>>;
  readonly properties: Readonly<Record<string, unknown>>;
  readonly statDefinitions: Readonly<Record<string, unknown>>;
  readonly transformations: Readonly<Record<string, unknown>>;
}

const emptyModelDefinitionAssetModules = (): ModelDefinitionAssetModules => ({
  typologies: {},
  actions: {},
  interactions: {},
  manifests: {},
  extensions: {},
  attributes: {},
  propertyDefinitions: {},
  properties: {},
  statDefinitions: {},
  transformations: {},
});

export const modelDefinitionAssetModules = ephemeralBox<ModelDefinitionAssetModules>("s.spatial-kernel.geometry.modelDefinitionAssetModules", emptyModelDefinitionAssetModules());

const modelDefinitionFolderIdMapCache = ephemeralBox<ReadonlyMap<string, string> | null>("s.spatial-kernel.geometry.modelDefinitionFolderIdMapCache", null);

export const typologyOwnerByIdCache = ephemeralBox<ReadonlyMap<string, string> | null>("s.spatial-kernel.geometry.typologyOwnerByIdCache", null);

export const actionOwnerByIdCache = ephemeralBox<ReadonlyMap<string, string> | null>("s.spatial-kernel.geometry.actionOwnerByIdCache", null);

export const interactionOwnerByIdCache = ephemeralBox<ReadonlyMap<string, string> | null>("s.spatial-kernel.geometry.interactionOwnerByIdCache", null);

export const attributeOwnerByIdCache = ephemeralBox<ReadonlyMap<string, string> | null>("s.spatial-kernel.geometry.attributeOwnerByIdCache", null);

export const propertyOwnerByIdCache = ephemeralBox<ReadonlyMap<string, string> | null>("s.spatial-kernel.geometry.propertyOwnerByIdCache", null);

export const statOwnerByIdCache = ephemeralBox<ReadonlyMap<string, string> | null>("s.spatial-kernel.geometry.statOwnerByIdCache", null);

const defaultModelDefinitionIdCache = ephemeralBox<string | null>("s.spatial-kernel.geometry.defaultModelDefinitionIdCache", null);

export const typologyStyleCache = ephemeralBox<Map<string, ResolvedTypologyStyle> | null>("s.spatial-kernel.geometry.typologyStyleCache", null);

const modelDefinitionContributions = ephemeralMap<symbol, ModelDefinitionAssetModules>("s.spatial-kernel.geometry.modelDefinitionContributions");

const modelDefinitionAssetListeners = ephemeralMap<symbol, () => void>("s.spatial-kernel.geometry.modelDefinitionAssetListeners");

/** 📡️ Subscribes an independent cache owner to contribution changes. */
export function onModelDefinitionAssetsChanged(listener: () => void): () => void {
  const subscription = Symbol();
  modelDefinitionAssetListeners.set(subscription, listener);
  return () => { modelDefinitionAssetListeners.delete(subscription); };
}

function refreshModelDefinitionAssets(): void {
  const modules = emptyModelDefinitionAssetModules();
  for (const contribution of modelDefinitionContributions.values()) {
    for (const key of Object.keys(modules) as (keyof ModelDefinitionAssetModules)[]) Object.assign(modules[key], contribution[key]);
  }
  modelDefinitionAssetModules.current = modules;
  modelDefinitionFolderIdMapCache.current = null;
  typologyOwnerByIdCache.current = null;
  actionOwnerByIdCache.current = null;
  interactionOwnerByIdCache.current = null;
  attributeOwnerByIdCache.current = null;
  propertyOwnerByIdCache.current = null;
  statOwnerByIdCache.current = null;
  defaultModelDefinitionIdCache.current = null;
  typologyStyleCache.current = null;
  typologyConstructKitByInteractionCache.current = null;
  for (const listener of [...modelDefinitionAssetListeners.values()]) listener();
}

/** 🔌️ Registers an independent contribution and returns its removal operation. */
export function registerModelDefinitionAssets(modules: ModelDefinitionAssetModules): () => void {
  const contribution = Symbol();
  modelDefinitionContributions.set(contribution, modules);
  refreshModelDefinitionAssets();
  return () => {
    if (modelDefinitionContributions.delete(contribution)) refreshModelDefinitionAssets();
  };
}

/** 🎬️ Registered action asset rows. */
export function modelDefinitionActionCatalog(): readonly unknown[] {
  return Object.values(modelDefinitionAssetModules.current.actions);
}

export function modelDefinitionTypologyCatalog(): readonly unknown[] {
  return Object.values(modelDefinitionAssetModules.current.typologies);
}

function modelDefinitionManifestCatalog(): readonly unknown[] {
  return [...Object.values(modelDefinitionAssetModules.current.manifests), ...Object.values(modelDefinitionAssetModules.current.extensions)];
}

export function modelDefinitionAttributeCatalog(): readonly unknown[] {
  return Object.values(modelDefinitionAssetModules.current.attributes);
}

export function modelDefinitionPropertyCatalog(): readonly unknown[] {
  return [...Object.values(modelDefinitionAssetModules.current.propertyDefinitions), ...Object.values(modelDefinitionAssetModules.current.properties)];
}

export function modelDefinitionStatCatalog(): readonly unknown[] {
  return Object.values(modelDefinitionAssetModules.current.statDefinitions);
}

/** 📥️ Registered interaction asset rows. */
export function modelDefinitionInteractionCatalog(): readonly unknown[] {
  return Object.values(modelDefinitionAssetModules.current.interactions);
}

export function modelDefinitionTransformationModules(): Readonly<Record<string, unknown>> {
  return modelDefinitionAssetModules.current.transformations;
}
// #endregion 📥️ModelDefinitionCatalog

/** 🧭️ Default geometry-edit model definition id (manifest `default: true`). */
export function defaultModelDefinitionId(): string {
  if (defaultModelDefinitionIdCache.current) return defaultModelDefinitionIdCache.current;
  const manifests = listModelDefinitionManifests();
  const row = manifests.find((m) => m.default) ?? manifests[0];
  defaultModelDefinitionIdCache.current = row?.id ?? "";
  return defaultModelDefinitionIdCache.current;
}

/** 🧭️ True when the active definition is geometry edit (`ModelDefinition`) rather than typology objects. */
export function isShapeModelDefinition(modelDefinitionId: string | null | undefined): boolean {
  if (modelDefinitionId == null) return true;
  return kernelTypologyIds(modelDefinitionId) !== null;
}

/** 🪪️ Kernel typology ids per primitive kind on a model-definition manifest. */
export function kernelTypologyIds(modelDefinitionId: string): Readonly<Partial<Record<KernelTopologyKind, string>>> | null {
  const manifest = listModelDefinitionManifests().find((row) => row.id === modelDefinitionId);
  const map = manifest?.kernelTypologies;
  if (!map || Object.keys(map).length === 0) return null;
  return map;
}

/** 🧾️ Parses a model-definition manifest JSON or returns `null`. */
export function parseModelDefinitionManifest(raw: unknown): ModelDefinitionManifest | null {
  if (!raw || typeof raw !== "object") return null;
  const r = raw as Record<string, unknown>;
  if (r.schema !== "spatial.modelDefinition" && r.schema !== "spatial.extension") return null;
  if (typeof r.id !== "string" || typeof r.version !== "string" || typeof r.label !== "string") return null;
  if (!Array.isArray(r.kinds) || r.kinds.length === 0) return null;
  const kernelTypologies = parseKernelTypologies(r.kernelTypologies);
  return {
    schema: "spatial.modelDefinition",
    id: r.id,
    version: r.version,
    label: r.label,
    description: typeof r.description === "string" ? r.description : undefined,
    kinds: r.kinds as string[],
    ...(r.default === true ? { default: true } : {}),
    ...(typeof r.baseObjectTypology === "string" && r.baseObjectTypology.length > 0 ? { baseObjectTypology: r.baseObjectTypology } : {}),
    ...(kernelTypologies ? { kernelTypologies } : {}),
  };
}

function parseKernelTypologies(raw: unknown): Readonly<Partial<Record<KernelTopologyKind, string>>> | undefined {
  if (!raw || typeof raw !== "object" || Array.isArray(raw)) return undefined;
  const allowed = new Set<KernelTopologyKind>(["anchor", "vertex", "edge", "wire", "face", "shell", "solid"]);
  const out: Partial<Record<KernelTopologyKind, string>> = {};
  for (const [key, value] of Object.entries(raw as Record<string, unknown>)) {
    if (!allowed.has(key as KernelTopologyKind) || typeof value !== "string" || value.length === 0) continue;
    out[key as KernelTopologyKind] = value;
  }
  return Object.keys(out).length > 0 ? out : undefined;
}

/** 📚️ Lists model-definition manifests under spatial/asset/modelDefinition. */
export function listModelDefinitionManifests(): readonly ModelDefinitionManifest[] {
  return modelDefinitionManifestCatalog()
    .map((raw) => parseModelDefinitionManifest(raw))
    .filter((m): m is ModelDefinitionManifest => m !== null);
}

/** 🎨️ Surface pattern kind for typology display styling. */
export type TypologyStylePatternKind = "none" | "hatch" | "crosshatch" | "dots";

/** 🎨️ Authored surface pattern on a typology. */
export interface TypologyStylePatternSpec {
  readonly kind: TypologyStylePatternKind;
  readonly direction?: number;
  readonly spacing?: number;
  readonly lineWidth?: number;
  readonly color?: string;
}

/** 🎨️ Fully resolved display style for one typology (auto fallback + authored overrides). */
export interface ResolvedTypologyStyle {
  readonly color: string;
  readonly edgeColor: string;
  readonly opacity: number;
  readonly pattern: Required<Pick<TypologyStylePatternSpec, "kind" | "direction" | "spacing" | "lineWidth" | "color">>;
}

/** 🧭️ Per-typology construct kit: three mode actions + one interaction id. */
export interface TypologyConstructKit {
  readonly typology: string;
  readonly interaction: string;
  readonly constructFrom2PointsAndHeight: string;
  readonly constructFromCurveAndHeight: string;
  readonly constructFromSurface: string;
}

export const typologyConstructKitByInteractionCache = ephemeralBox<ReadonlyMap<string, TypologyConstructKit> | null>("s.spatial-kernel.geometry.typologyConstructKitByInteractionCache", null);

// #region 🧭️ModelDefinitionScope
function modelDefinitionAssetPathRest(assetPath: string): string | null {
  const normalized = assetPath.replace(/\\/g, "/");
  for (const marker of ["/asset/modelDefinition/", "assets/modelDefinition/", "assets/modelDefinitions/", "🖼️assets/🏗️modelDefinitions/"]) {
    const idx = normalized.indexOf(marker);
    if (idx >= 0) return normalized.slice(idx + marker.length);
  }
  return null;
}

function modelDefinitionFolderFromAssetPath(assetPath: string): string | null {
  const rest = modelDefinitionAssetPathRest(assetPath);
  if (!rest) return null;
  const folder = rest.split("/")[0];
  return folder || null;
}

function modelDefinitionFolderIdMap(): ReadonlyMap<string, string> {
  if (modelDefinitionFolderIdMapCache.current) return modelDefinitionFolderIdMapCache.current;
  const map = new Map<string, string>();
  const modules = {
    ...modelDefinitionAssetModules.current.manifests,
    ...modelDefinitionAssetModules.current.extensions,
  };
  for (const [path, raw] of Object.entries(modules)) {
    const folder = modelDefinitionFolderFromAssetPath(path);
    const manifest = parseModelDefinitionManifest(raw);
    if (!folder || !manifest) continue;
    map.set(folder, manifest.id);
  }
  modelDefinitionFolderIdMapCache.current = map;
  return map;
}

/** 🧭️ Resolves manifest `id` from an asset path under `spatial/asset/modelDefinition`. */
export function modelDefinitionIdFromAssetPath(assetPath: string): string | null {
  const folder = modelDefinitionFolderFromAssetPath(assetPath);
  if (!folder) return null;
  return modelDefinitionFolderIdMap().get(folder) ?? null;
}
//#endregion 🧭️ModelDefinitionScope
