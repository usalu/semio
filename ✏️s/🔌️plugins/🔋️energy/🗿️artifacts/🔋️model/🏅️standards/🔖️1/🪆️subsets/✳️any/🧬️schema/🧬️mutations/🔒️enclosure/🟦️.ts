/** 🧾️ Energy-model mutation payloads of the `enclosure` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
/** 🏟️ `create-thermal-enclosure` payload. */
export interface CreateThermalEnclosure {
  readonly mutation: "createThermalEnclosure";
  readonly index: number;
  readonly id: number;
  readonly name: string;
  readonly zoneIds: readonly number[];
}

/** 🏯️ `delete-thermal-enclosure` payload. */
export interface DeleteThermalEnclosure {
  readonly mutation: "deleteThermalEnclosure";
  readonly id: number;
}

/** 🖋️ `rename-thermal-enclosure` payload. */
export interface RenameThermalEnclosure {
  readonly mutation: "renameThermalEnclosure";
  readonly id: number;
  readonly newName: string;
}

/** 🔒️ `add-thermal-enclosure-zone` payload. */
export interface AddThermalEnclosureZone {
  readonly mutation: "addThermalEnclosureZone";
  readonly id: number;
  readonly index: number;
  readonly zoneId: number;
}

/** 🔓️ `remove-thermal-enclosure-zone` payload. */
export interface RemoveThermalEnclosureZone {
  readonly mutation: "removeThermalEnclosureZone";
  readonly id: number;
  readonly zoneId: number;
}
