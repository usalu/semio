/** 🍃️ Energy-model mutation payloads of the `outdoor-air` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
/** 🌲️ `create-outdoor-air-system` payload. */
export interface CreateOutdoorAirSystem {
  readonly mutation: "createOutdoorAirSystem";
  readonly id: number;
  readonly airLoopId: number;
  readonly minOaFlowM3S: number;
  readonly economizerEnabled: boolean;
  readonly index: number | null;
}

/** 🍂️ `delete-outdoor-air-system` payload. */
export interface DeleteOutdoorAirSystem {
  readonly mutation: "deleteOutdoorAirSystem";
  readonly id: number;
}

/** ⛓️ `change-outdoor-air-system-air-loop` payload. */
export interface ChangeOutdoorAirSystemAirLoop {
  readonly mutation: "changeOutdoorAirSystemAirLoop";
  readonly id: number;
  readonly newAirLoopId: number;
}

/** 🦋️ `change-outdoor-air-system-min-oa-flow` payload. */
export interface ChangeOutdoorAirSystemMinOaFlow {
  readonly mutation: "changeOutdoorAirSystemMinOaFlow";
  readonly id: number;
  readonly newMinOaFlowM3S: number;
}

/** 💰️ `change-outdoor-air-system-economizer-enabled` payload. */
export interface ChangeOutdoorAirSystemEconomizerEnabled {
  readonly mutation: "changeOutdoorAirSystemEconomizerEnabled";
  readonly id: number;
  readonly newEconomizerEnabled: boolean;
}
