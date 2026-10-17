/** 🌬️ Energy-model mutation payloads of the `air-loop` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
/** 🛞️ `create-air-loop` payload. */
export interface CreateAirLoop {
  readonly mutation: "createAirLoop";
  readonly id: number;
  readonly name: string;
  readonly supplyNodeId: number;
  readonly returnNodeId: number;
  readonly designSupplyAirFlowM3S: number;
  readonly terminalZoneIds: readonly number[];
  readonly index: number | null;
}

/** 🥀️ `delete-air-loop` payload. */
export interface DeleteAirLoop {
  readonly mutation: "deleteAirLoop";
  readonly id: number;
}

/** 📇️ `rename-air-loop` payload. */
export interface RenameAirLoop {
  readonly mutation: "renameAirLoop";
  readonly id: number;
  readonly newName: string;
}

/** ↗️ `change-air-loop-supply-node` payload. */
export interface ChangeAirLoopSupplyNode {
  readonly mutation: "changeAirLoopSupplyNode";
  readonly id: number;
  readonly newSupplyNodeId: number;
}

/** ↘️ `change-air-loop-return-node` payload. */
export interface ChangeAirLoopReturnNode {
  readonly mutation: "changeAirLoopReturnNode";
  readonly id: number;
  readonly newReturnNodeId: number;
}

/** 🍥️ `change-air-loop-design-supply-air-flow` payload. */
export interface ChangeAirLoopDesignSupplyAirFlow {
  readonly mutation: "changeAirLoopDesignSupplyAirFlow";
  readonly id: number;
  readonly newDesignSupplyAirFlowM3S: number;
}

/** 🪺️ `add-air-loop-terminal-zone` payload. */
export interface AddAirLoopTerminalZone {
  readonly mutation: "addAirLoopTerminalZone";
  readonly id: number;
  readonly zoneId: number;
  readonly index: number | null;
}

/** 🪹️ `remove-air-loop-terminal-zone` payload. */
export interface RemoveAirLoopTerminalZone {
  readonly mutation: "removeAirLoopTerminalZone";
  readonly id: number;
  readonly zoneId: number;
}
