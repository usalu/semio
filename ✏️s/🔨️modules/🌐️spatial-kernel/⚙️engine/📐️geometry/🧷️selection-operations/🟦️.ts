/** 🧷️ Selection operations of a model definition and its resolved scope of entity kinds and objects. */
import type { TypologySpec, AttributeDefinitionSpec, PropertyDefinitionSpec } from "../🎨️typology/🟦️.ts";
import { PRIMITIVE_MODEL_ENTITY_KINDS, listTypologiesForModelDefinition, type SpatialInteraction, listSpatialInteractionsForModelDefinition, listAttributeDefinitionsForModelDefinition, listPropertyDefinitionsForModelDefinition, listActionsForModelDefinition } from "../🧭️definition-ownership/🟦️.ts";
import { type StatDefinitionSpec, listStatDefinitionsForModelDefinition } from "../📊️stats/🟦️.ts";
import type { ModelEntityKind } from "../🟦️.ts";

//#region 🧭️ModelDefinitionScope
/** 🪪️ model-definition selection command operation id (`selection.apply` param). */
export type SelectionApplyOperation = "selectAll" | "deselectAll" | "invert" | "selectKinds";

/** 🪪️ model-definition selection command interaction row (`selection.*` registry). */
export interface SelectionOperationInteractionDef {
  readonly id: string;
  readonly label: string;
  readonly key: string;
  readonly operation: SelectionApplyOperation;
  readonly kinds?: readonly ModelEntityKind[];
}

const SELECTION_INTERACTION_KEYS: Readonly<Record<string, string>> = {
  "selection.selectAll": "sa",
  "selection.deselectAll": "ds",
  "selection.invert": "iv",
  "selection.selectAnchors": "xa",
  "selection.selectVertices": "xv",
  "selection.selectEdges": "xe",
  "selection.selectWires": "xw",
  "selection.selectFaces": "xf",
  "selection.selectSolids": "xc",
  "selection.selectGeometries": "xg",
  "selection.selectObjects": "xo",
};

const SELECTION_ACTION_META: Readonly<Record<string, { readonly operation: SelectionApplyOperation; readonly kinds?: readonly ModelEntityKind[] }>> = {
  "selection.selectAll": { operation: "selectAll" },
  "selection.deselectAll": { operation: "deselectAll" },
  "selection.invert": { operation: "invert" },
  "selection.selectAnchors": { operation: "selectKinds", kinds: ["anchor"] },
  "selection.selectVertices": { operation: "selectKinds", kinds: ["vertex"] },
  "selection.selectEdges": { operation: "selectKinds", kinds: ["edge"] },
  "selection.selectWires": { operation: "selectKinds", kinds: ["wire"] },
  "selection.selectFaces": { operation: "selectKinds", kinds: ["face"] },
  "selection.selectSolids": { operation: "selectKinds", kinds: ["solid"] },
  "selection.selectGeometries": { operation: "selectKinds", kinds: ["geometry"] },
  "selection.selectObjects": { operation: "selectKinds", kinds: ["object"] },
};

function selectionOperationDefForActionId(actionId: string, label?: string): SelectionOperationInteractionDef | null {
  if (!actionId.startsWith("selection.") || actionId === "selection.apply") return null;
  const meta = SELECTION_ACTION_META[actionId];
  const key = SELECTION_INTERACTION_KEYS[actionId];
  if (!meta || !key) return null;
  return {
    id: actionId,
    label: label ?? actionId.slice("selection.".length),
    key,
    operation: meta.operation,
    ...(meta.kinds ? { kinds: [...meta.kinds] } : {}),
  };
}

/** 🧭️ Selection command fixtures whose action assets belong to a model definition. */
export function listSelectionOperationsForModelDefinition(modelDefinitionId: string): readonly SelectionOperationInteractionDef[] {
  const out: SelectionOperationInteractionDef[] = [];
  for (const actionId of listActionsForModelDefinition(modelDefinitionId)) {
    const defn = selectionOperationDefForActionId(actionId);
    if (defn) out.push(defn);
  }
  return out.sort((a, b) => a.id.localeCompare(b.id));
}

/** 🧭️ Selection entity kinds available while a model definition is active (factory primitives + objects). */
export function modelDefinitionSelectionEntityKinds(modelDefinitionId: string): readonly ModelEntityKind[] {
  const entityKindIds = new Set<string>([...PRIMITIVE_MODEL_ENTITY_KINDS, "object", "geometry", "attribute"]);
  const kinds = new Set<ModelEntityKind>([...PRIMITIVE_MODEL_ENTITY_KINDS, "object"]);
  for (const defn of listAttributeDefinitionsForModelDefinition(modelDefinitionId)) {
    for (const kind of defn.targets) {
      if (entityKindIds.has(kind)) kinds.add(kind as ModelEntityKind);
    }
    for (const kind of defn.geometrySelector?.kinds ?? []) {
      if (entityKindIds.has(kind)) kinds.add(kind as ModelEntityKind);
    }
  }
  const ordered: ModelEntityKind[] = [];
  for (const kind of PRIMITIVE_MODEL_ENTITY_KINDS) {
    if (kinds.has(kind)) ordered.push(kind);
  }
  for (const kind of kinds) {
    if (!(PRIMITIVE_MODEL_ENTITY_KINDS as readonly string[]).includes(kind)) ordered.push(kind);
  }
  return ordered;
}

/** 🧭️ Summarizes scoped catalogs for the active model definition (hosts + REPL). */
export interface ModelDefinitionScope {
  readonly modelDefinitionId: string;
  readonly typologies: readonly TypologySpec[];
  readonly interactions: readonly SpatialInteraction[];
  readonly selectionOperations: readonly SelectionOperationInteractionDef[];
  readonly attributeDefinitions: readonly AttributeDefinitionSpec[];
  readonly propertyDefinitions: readonly PropertyDefinitionSpec[];
  readonly statDefinitions: readonly StatDefinitionSpec[];
  readonly actions: readonly string[];
  readonly selectionEntityKinds: readonly ModelEntityKind[];
}

/** 🧭️ Resolves everything available under one model definition manifest id. */
export function resolveModelDefinitionScope(modelDefinitionId: string): ModelDefinitionScope {
  return {
    modelDefinitionId,
    typologies: listTypologiesForModelDefinition(modelDefinitionId),
    interactions: listSpatialInteractionsForModelDefinition(modelDefinitionId),
    selectionOperations: listSelectionOperationsForModelDefinition(modelDefinitionId),
    attributeDefinitions: listAttributeDefinitionsForModelDefinition(modelDefinitionId),
    propertyDefinitions: listPropertyDefinitionsForModelDefinition(modelDefinitionId),
    statDefinitions: listStatDefinitionsForModelDefinition(modelDefinitionId),
    actions: listActionsForModelDefinition(modelDefinitionId),
    selectionEntityKinds: modelDefinitionSelectionEntityKinds(modelDefinitionId),
  };
}
// #endregion 🧭️ModelDefinitionScope
