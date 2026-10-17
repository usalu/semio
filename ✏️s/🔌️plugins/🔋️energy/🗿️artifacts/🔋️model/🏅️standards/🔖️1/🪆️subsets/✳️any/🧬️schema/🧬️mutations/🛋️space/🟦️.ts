/** 🛋️ Energy-model mutation payloads of the `space` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
/** 🪑️ `create-space` payload. */
export interface CreateSpace {
  readonly mutation: "createSpace";
  readonly id: number;
  readonly name: string;
  readonly zoneId: number;
  readonly floorAreaM2: number;
  readonly index: number | null;
}

/** 🧹️ `delete-space` payload. */
export interface DeleteSpace {
  readonly mutation: "deleteSpace";
  readonly id: number;
}

/** 🔤️ `rename-space` payload. */
export interface RenameSpace {
  readonly mutation: "renameSpace";
  readonly id: number;
  readonly newName: string;
}

/** 🧮️ `change-space-floor-area` payload. */
export interface ChangeSpaceFloorArea {
  readonly mutation: "changeSpaceFloorArea";
  readonly id: number;
  readonly newFloorAreaM2: number;
}

/** 🚚️ `change-space-zone` payload. */
export interface ChangeSpaceZone {
  readonly mutation: "changeSpaceZone";
  readonly id: number;
  readonly newZoneId: number;
}
