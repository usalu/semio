/** 🧾️ Energy-model mutation payloads of the `construction` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
/** 🏗️ `create-construction` payload. */
export interface CreateConstruction {
  readonly mutation: "createConstruction";
  readonly index: number;
  readonly id: number;
  readonly name: string;
  readonly layerMaterialIds: readonly number[];
}

/** 🧨️ `delete-construction` payload. */
export interface DeleteConstruction {
  readonly mutation: "deleteConstruction";
  readonly id: number;
}

/** 🪪️ `rename-construction` payload. */
export interface RenameConstruction {
  readonly mutation: "renameConstruction";
  readonly id: number;
  readonly newName: string;
}

/** ➕️ `add-construction-layer` payload. */
export interface AddConstructionLayer {
  readonly mutation: "addConstructionLayer";
  readonly id: number;
  readonly index: number;
  readonly materialId: number;
}

/** ➖️ `remove-construction-layer` payload. */
export interface RemoveConstructionLayer {
  readonly mutation: "removeConstructionLayer";
  readonly id: number;
  readonly index: number;
}

/** 🔀️ `reorder-construction-layers` payload. */
export interface ReorderConstructionLayers {
  readonly mutation: "reorderConstructionLayers";
  readonly id: number;
  readonly newLayerMaterialIds: readonly number[];
}
