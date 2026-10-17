/** 🔌️ Energy-model mutation payloads of the `load-center` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
/** 🏦️ `create-electrical-load-center` payload. */
export interface CreateElectricalLoadCenter {
  readonly mutation: "createElectricalLoadCenter";
  readonly index: number;
  readonly id: number;
  readonly name: string;
  readonly generatorIds: readonly number[];
  readonly pvIds: readonly number[];
  readonly batteryIds: readonly number[];
}

/** 🔻️ `delete-electrical-load-center` payload. */
export interface DeleteElectricalLoadCenter {
  readonly mutation: "deleteElectricalLoadCenter";
  readonly id: number;
}

/** 🖊️ `rename-electrical-load-center` payload. */
export interface RenameElectricalLoadCenter {
  readonly mutation: "renameElectricalLoadCenter";
  readonly id: number;
  readonly newName: string;
}

/** ☄️ `add-electrical-load-center-pv` payload. */
export interface AddElectricalLoadCenterPv {
  readonly mutation: "addElectricalLoadCenterPv";
  readonly id: number;
  readonly index: number;
  readonly pvId: number;
}

/** 🌘️ `remove-electrical-load-center-pv` payload. */
export interface RemoveElectricalLoadCenterPv {
  readonly mutation: "removeElectricalLoadCenterPv";
  readonly id: number;
  readonly pvId: number;
}

/** 🔋️ `add-electrical-load-center-battery` payload. */
export interface AddElectricalLoadCenterBattery {
  readonly mutation: "addElectricalLoadCenterBattery";
  readonly id: number;
  readonly index: number;
  readonly batteryId: number;
}

/** 🪝️ `remove-electrical-load-center-battery` payload. */
export interface RemoveElectricalLoadCenterBattery {
  readonly mutation: "removeElectricalLoadCenterBattery";
  readonly id: number;
  readonly batteryId: number;
}
