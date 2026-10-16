/** 🧭️ Which model definition owns a typology, action, interaction, attribute or property, which actions a definition makes available, and its objects. */
import { ephemeralMap } from "@semio-tech/framework";
import { type InteractionSpec, isCallableOnlyInteraction, parseInteractionSpec, compileInteraction, parseActionSpec } from "../🎰️interaction-spec/🟦️.ts";
import { modelDefinitionAssetModules, typologyOwnerByIdCache, actionOwnerByIdCache, interactionOwnerByIdCache, attributeOwnerByIdCache, propertyOwnerByIdCache, modelDefinitionInteractionCatalog, defaultModelDefinitionId, kernelTypologyIds, modelDefinitionIdFromAssetPath } from "../📥️definition-catalog/🟦️.ts";
import { type TypologySpec, parseTypologySpec, shippedTypologyCatalog, typologyConstructKitByInteraction, type AttributeDefinitionSpec, parseAttributeDefinitionSpec, type PropertyDefinitionSpec, parsePropertyDefinitionSpec, shippedAttributeDefinitionCatalog, shippedPropertyDefinitionCatalog, loadPropertyDefinition } from "../🎨️typology/🟦️.ts";
import { Model } from "../🧊️model/🟦️.ts";
import { loadTransformation } from "../🔄️transformation-spec/🟦️.ts";
import type { ModelEntityKind, SpatialObjectRecord } from "../🟦️.ts";

/** 🧭️ Throws when `actionId` is outside the active model definition catalog. */
export function assertActionAvailableInModelDefinition(actionId: string, activeModelDefinitionId?: string | null): void {
  const mdId = activeModelDefinitionId ?? defaultModelDefinitionId();
  if (!actionAvailableInModelDefinition(actionId, mdId)) {
    throw new Error(`action ${actionId} is not available in model definition ${mdId}`);
  }
}

/** 🧱️ Primitive entity kinds selectable on factory geometry (excludes typology `object` rows). */
export const PRIMITIVE_MODEL_ENTITY_KINDS: readonly ModelEntityKind[] = ["anchor", "vertex", "edge", "wire", "face", "shell", "solid"];

/** ✅️ True when `defn` applies to a model entity kind under the active model definition. */
export function attributeDefinitionAppliesToEntity(defn: AttributeDefinitionSpec, entityKind: ModelEntityKind): boolean {
  if (!defn.targets.includes(entityKind)) return false;
  const selector = defn.geometrySelector?.kinds;
  if (selector && selector.length > 0 && !selector.includes(entityKind)) return false;
  return true;
}

/** 📚️ Attribute definitions for one model definition and entity kind. */
export function listAttributeDefinitionsForModelDefinitionEntity(modelDefinitionId: string, entityKind: ModelEntityKind): readonly AttributeDefinitionSpec[] {
  return listAttributeDefinitionsForModelDefinition(modelDefinitionId).filter((defn) => attributeDefinitionAppliesToEntity(defn, entityKind));
}

/** 🧲️ True when the active model definition exposes factory-geometry pick targets (all definitions). */
export function modelDefinitionUsesGeometryPicking(_modelDefinitionId: string): boolean {
  return true;
}

/** 📋️ String/number/boolean/record options from an attribute value schema. */
export function attributeDefinitionValueOptions(defn: AttributeDefinitionSpec): readonly string[] | null {
  const schema = defn.value;
  if (!schema || typeof schema !== "object") return null;
  const row = schema as Record<string, unknown>;
  if (row.kind === "string" && Array.isArray(row.options)) {
    return row.options.filter((option): option is string => typeof option === "string");
  }
  if (row.kind === "oneOf" && Array.isArray(row.variants)) {
    for (const variant of row.variants) {
      if (!variant || typeof variant !== "object") continue;
      const v = variant as Record<string, unknown>;
      if (v.kind === "string" && Array.isArray(v.options)) {
        return v.options.filter((option): option is string => typeof option === "string");
      }
    }
  }
  return null;
}

/** 🧾️ Value editor kind inferred from an attribute definition schema. */
export function attributeDefinitionEditorKind(defn: AttributeDefinitionSpec): "string" | "enum" | "number" | "boolean" | "text" {
  if (attributeDefinitionValueOptions(defn)) return "enum";
  const schema = defn.value;
  if (!schema || typeof schema !== "object") return "text";
  const row = schema as Record<string, unknown>;
  if (row.kind === "number") return "number";
  if (row.kind === "boolean") return "boolean";
  if (row.kind === "string") return "string";
  if (row.kind === "oneOf") return "text";
  return "text";
}

/** ✅️ Validates a value against an attribute definition schema. */
export function validateAttributeValue(defn: AttributeDefinitionSpec, value: unknown): boolean {
  const schema = defn.value;
  if (!schema || typeof schema !== "object") return false;
  const row = schema as Record<string, unknown>;
  const kind = row.kind;
  if (kind === "string") {
    if (typeof value !== "string") return false;
    const options = row.options;
    if (Array.isArray(options) && options.length > 0) return options.includes(value);
    return true;
  }
  if (kind === "number") return typeof value === "number" && Number.isFinite(value);
  if (kind === "boolean") return typeof value === "boolean";
  if (kind === "record") {
    if (!value || typeof value !== "object" || Array.isArray(value)) return false;
    return true;
  }
  if (kind === "oneOf" && Array.isArray(row.variants)) {
    return row.variants.some((variant) => validateAttributeValue({ ...defn, value: variant }, value));
  }
  return false;
}

function typologyOwnerById(): ReadonlyMap<string, string> {
  if (typologyOwnerByIdCache.current) return typologyOwnerByIdCache.current;
  const map = new Map<string, string>();
  for (const [path, raw] of Object.entries(modelDefinitionAssetModules.current.typologies)) {
    const owner = modelDefinitionIdFromAssetPath(path);
    const spec = parseTypologySpec(raw);
    if (!owner || !spec) continue;
    map.set(spec.id, owner);
  }
  typologyOwnerByIdCache.current = map;
  return map;
}

/** 🧭️ Typologies owned by a model-definition folder manifest. */
export function listTypologiesForModelDefinition(modelDefinitionId: string): readonly TypologySpec[] {
  const owners = typologyOwnerById();
  return shippedTypologyCatalog().filter((row) => owners.get(row.id) === modelDefinitionId);
}

function actionOwnerById(): ReadonlyMap<string, string> {
  if (actionOwnerByIdCache.current) return actionOwnerByIdCache.current;
  const map = new Map<string, string>();
  for (const [path, raw] of Object.entries(modelDefinitionAssetModules.current.actions)) {
    const owner = modelDefinitionIdFromAssetPath(path);
    const spec = parseActionSpec(raw);
    if (!owner || !spec) continue;
    map.set(spec.id, owner);
  }
  actionOwnerByIdCache.current = map;
  return map;
}

/** 🧭️ True when an action asset file lives under `modelDefinitionId`. */
export function actionOwnedByModelDefinition(actionId: string, modelDefinitionId: string): boolean {
  return actionOwnerById().get(actionId) === modelDefinitionId;
}

/** 🧭️ Model definition that owns a typology asset. */
export function modelDefinitionIdForTypology(typologyId: string): string | null {
  return typologyOwnerById().get(typologyId) ?? null;
}

/** 📚️ Host-facing interaction row from model-definition interaction JSON. */
export interface SpatialInteraction {
  readonly id: string;
  readonly label: string;
  /** ⌨️ Host interaction key; must stay unique and appear in `label`. */
  readonly key: string;
}

type ModelDefinitionInteractionRecord = InteractionSpec & { readonly key?: string };

function shippedInteractionJsons(): readonly ModelDefinitionInteractionRecord[] {
  return modelDefinitionInteractionCatalog() as readonly ModelDefinitionInteractionRecord[];
}

function interactionCatalogRow(spec: ModelDefinitionInteractionRecord): SpatialInteraction {
  return { id: spec.id, label: spec.label ?? spec.id, key: typeof spec.key === "string" ? spec.key : (spec.id[0] ?? "?") };
}

/** 🧭️ Interaction rows shipped from model-definition interaction assets (id/label/key catalog view). */
function shippedSpatialInteractionCatalog(): readonly SpatialInteraction[] {
  return shippedInteractionJsons().map(interactionCatalogRow);
}

const COMPILED_INTERACTION_BY_ID = ephemeralMap<string, InteractionSpec>("s.spatial-kernel.geometry.COMPILED_INTERACTION_BY_ID");

/** 📚️ Loads a model-definition interaction by stable `id` (compiled once per id for stable React runtime identity). */
export function loadSpatialInteraction(interactionId: string): InteractionSpec | null {
  const cached = COMPILED_INTERACTION_BY_ID.get(interactionId);
  if (cached) return cached;
  const raw = shippedInteractionJsons().find((spec) => spec.id === interactionId);
  const spec = raw ? parseInteractionSpec(raw) : null;
  if (!spec) return null;
  const compiled = compileInteraction(spec);
  COMPILED_INTERACTION_BY_ID.set(interactionId, compiled);
  return compiled;
}

function interactionOwnerById(): ReadonlyMap<string, string> {
  if (interactionOwnerByIdCache.current) return interactionOwnerByIdCache.current;
  const map = new Map<string, string>();
  for (const [path, raw] of Object.entries(modelDefinitionAssetModules.current.interactions)) {
    const owner = modelDefinitionIdFromAssetPath(path);
    const spec = parseInteractionSpec(raw);
    if (!owner || !spec) continue;
    map.set(spec.id, owner);
  }
  interactionOwnerByIdCache.current = map;
  return map;
}

/** 🧭️ Model definition that owns an interaction asset. */
export function modelDefinitionIdForInteraction(interactionId: string): string | null {
  return interactionOwnerById().get(interactionId) ?? null;
}

/** 🧭️ Interactions shipped for a model definition (folder assets + typology references). */
export function listSpatialInteractionsForModelDefinition(modelDefinitionId: string, options?: { readonly includeCallable?: boolean }): readonly SpatialInteraction[] {
  const ids = new Set<string>();
  for (const [id, owner] of interactionOwnerById()) {
    if (owner === modelDefinitionId) ids.add(id);
  }
  for (const typology of listTypologiesForModelDefinition(modelDefinitionId)) {
    for (const interactionId of typology.interactions) ids.add(interactionId);
  }
  const catalog = new Map(shippedSpatialInteractionCatalog().map((row) => [row.id, row] as const));
  return [...ids]
    .map((id) => catalog.get(id))
    .filter((row): row is SpatialInteraction => row !== undefined)
    .filter((row) => {
      if (options?.includeCallable) return true;
      const spec = loadSpatialInteraction(row.id);
      return spec ? !isCallableOnlyInteraction(spec) : true;
    })
    .sort((a, b) => a.id.localeCompare(b.id));
}

function attributeOwnerById(): ReadonlyMap<string, string> {
  if (attributeOwnerByIdCache.current) return attributeOwnerByIdCache.current;
  const map = new Map<string, string>();
  for (const [path, raw] of Object.entries(modelDefinitionAssetModules.current.attributes)) {
    const owner = modelDefinitionIdFromAssetPath(path);
    const spec = parseAttributeDefinitionSpec(raw);
    if (!owner || !spec) continue;
    map.set(spec.id, owner);
  }
  attributeOwnerByIdCache.current = map;
  return map;
}

/** 🧭️ Attribute definitions owned by a model definition. */
export function listAttributeDefinitionsForModelDefinition(modelDefinitionId: string): readonly AttributeDefinitionSpec[] {
  const owners = attributeOwnerById();
  return shippedAttributeDefinitionCatalog().filter((row) => owners.get(row.id) === modelDefinitionId);
}

function propertyOwnerById(): ReadonlyMap<string, string> {
  if (propertyOwnerByIdCache.current) return propertyOwnerByIdCache.current;
  const map = new Map<string, string>();
  for (const [path, raw] of Object.entries({
    ...modelDefinitionAssetModules.current.propertyDefinitions,
    ...modelDefinitionAssetModules.current.properties,
  })) {
    const owner = modelDefinitionIdFromAssetPath(path);
    const spec = parsePropertyDefinitionSpec(raw);
    if (!owner || !spec) continue;
    map.set(spec.id, owner);
  }
  propertyOwnerByIdCache.current = map;
  return map;
}

/** 🧭️ Property definitions referenced by typologies in a model definition. */
export function listPropertyDefinitionsForModelDefinition(modelDefinitionId: string): readonly PropertyDefinitionSpec[] {
  const ids = new Set<string>();
  for (const row of shippedPropertyDefinitionCatalog()) {
    if (propertyOwnerById().get(row.id) === modelDefinitionId) ids.add(row.id);
  }
  for (const typology of listTypologiesForModelDefinition(modelDefinitionId)) {
    for (const propertyId of typology.properties ?? []) ids.add(propertyId);
  }
  return [...ids].map((id) => loadPropertyDefinition(id)).filter((row): row is PropertyDefinitionSpec => row !== null);
}

/** 🧭️ Interaction ids invoked via `interaction.call` in one spec. */
export function interactionIdsReferencedByInteractionSpec(spec: InteractionSpec): readonly string[] {
  const ids = new Set<string>();
  for (const st of spec.machine.states) {
    for (const h of st.on ?? []) {
      for (const tr of h.transitions) {
        for (const fx of tr.effects ?? []) {
          if (fx.mutation === "interaction.call") ids.add(fx.interaction);
        }
      }
    }
  }
  return [...ids];
}

/** 🧭️ Action ids referenced by one interaction spec (transition effects + commit + nested interactions). */
export function actionIdsReferencedByInteractionSpec(spec: InteractionSpec): readonly string[] {
  const ids = new Set<string>();
  if (spec.commit.operation.kind === "action") {
    const kit = typologyConstructKitByInteraction().get(spec.id);
    if (kit) {
      ids.add(kit.constructFrom2PointsAndHeight);
      ids.add(kit.constructFromCurveAndHeight);
      ids.add(kit.constructFromSurface);
    } else {
      ids.add(spec.commit.operation.action);
    }
  }
  for (const st of spec.machine.states) {
    for (const h of st.on ?? []) {
      for (const tr of h.transitions) {
        for (const fx of tr.effects ?? []) {
          if (fx.mutation === "action") ids.add(fx.action);
        }
      }
    }
  }
  for (const nestedId of interactionIdsReferencedByInteractionSpec(spec)) {
    const nested = loadSpatialInteraction(nestedId);
    if (nested) {
      for (const actionId of actionIdsReferencedByInteractionSpec(nested)) ids.add(actionId);
    }
  }
  return [...ids];
}

/** 🧭️ Action ids declared on typologies, action assets, or owned interactions. */
export function listActionsForModelDefinition(modelDefinitionId: string): readonly string[] {
  const ids = new Set<string>();
  for (const typology of listTypologiesForModelDefinition(modelDefinitionId)) {
    for (const actionId of typology.actions) {
      if (actionOwnedByModelDefinition(actionId, modelDefinitionId)) ids.add(actionId);
    }
  }
  for (const [path, raw] of Object.entries(modelDefinitionAssetModules.current.actions)) {
    if (modelDefinitionIdFromAssetPath(path) !== modelDefinitionId) continue;
    const spec = parseActionSpec(raw);
    if (spec) ids.add(spec.id);
  }
  for (const row of listSpatialInteractionsForModelDefinition(modelDefinitionId)) {
    const interaction = loadSpatialInteraction(row.id);
    if (interaction) {
      for (const actionId of actionIdsReferencedByInteractionSpec(interaction)) ids.add(actionId);
    }
  }
  return [...ids].sort((a, b) => a.localeCompare(b));
}

/** 🧭️ True when `actionId` is declared in the active model definition (or is `selection.apply`). */
export function actionAvailableInModelDefinition(actionId: string, modelDefinitionId: string): boolean {
  if (actionId === "selection.apply") return true;
  if (actionId.startsWith("command.")) return true;
  const transformation = loadTransformation(actionId);
  if (transformation) {
    return transformation.source.modelDefinition === modelDefinitionId || transformation.target.modelDefinition === modelDefinitionId;
  }
  return listActionsForModelDefinition(modelDefinitionId).includes(actionId);
}

/** 🧭️ Object rows owned by typologies declared under a model definition. */
export function listModelObjectsForModelDefinition(model: Model, modelDefinitionId: string): readonly SpatialObjectRecord[] {
  const typologyIds = new Set(listTypologiesForModelDefinition(modelDefinitionId).map((row) => row.id));
  const kernelTypologyMap = kernelTypologyIds(modelDefinitionId);
  if (kernelTypologyMap) {
    for (const typologyId of Object.values(kernelTypologyMap)) {
      if (typeof typologyId === "string" && typologyId.length > 0) typologyIds.add(typologyId);
    }
  }
  return Object.values(model.objects).filter((row) => {
    if (typologyIds.has(row.typology)) return true;
    return modelDefinitionIdForTypology(row.typology) === modelDefinitionId;
  });
}

/** 🧭️ Counts in-view typology objects for a model definition (renderer scope). */
export function countViewObjectsForModelDefinition(model: Model, modelDefinitionId: string): number {
  return listModelObjectsForModelDefinition(model, modelDefinitionId).length;
}
