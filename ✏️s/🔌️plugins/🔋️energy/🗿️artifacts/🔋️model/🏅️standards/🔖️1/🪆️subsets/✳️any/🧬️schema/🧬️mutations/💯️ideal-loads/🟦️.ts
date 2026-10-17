/** 💯️ Energy-model mutation payloads of the `ideal-loads` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
/** 🫁️ `create-ideal-loads-system` payload. */
export interface CreateIdealLoadsSystem {
  readonly mutation: "createIdealLoadsSystem";
  readonly id: number;
  readonly zoneId: number;
  readonly maxHeatingSupplyAirTempC: number;
  readonly minCoolingSupplyAirTempC: number;
  readonly maxHeatingCapacityPresent: boolean;
  readonly maxHeatingCapacityW: number;
  readonly maxCoolingCapacityPresent: boolean;
  readonly maxCoolingCapacityW: number;
  readonly outdoorAirPerPersonM3S: number;
  readonly outdoorAirPerAreaM3SM2: number;
  readonly index: number | null;
}

/** 🫥️ `delete-ideal-loads-system` payload. */
export interface DeleteIdealLoadsSystem {
  readonly mutation: "deleteIdealLoadsSystem";
  readonly id: number;
}

/** 🏢️ `change-ideal-loads-system-zone` payload. */
export interface ChangeIdealLoadsSystemZone {
  readonly mutation: "changeIdealLoadsSystemZone";
  readonly id: number;
  readonly newZoneId: number;
}

/** 🔴️ `change-ideal-loads-system-max-heating-supply-air-temp` payload. */
export interface ChangeIdealLoadsSystemMaxHeatingSupplyAirTemp {
  readonly mutation: "changeIdealLoadsSystemMaxHeatingSupplyAirTemp";
  readonly id: number;
  readonly newMaxHeatingSupplyAirTempC: number;
}

/** 🔵️ `change-ideal-loads-system-min-cooling-supply-air-temp` payload. */
export interface ChangeIdealLoadsSystemMinCoolingSupplyAirTemp {
  readonly mutation: "changeIdealLoadsSystemMinCoolingSupplyAirTemp";
  readonly id: number;
  readonly newMinCoolingSupplyAirTempC: number;
}

/** ⛽️ `change-ideal-loads-system-max-heating-capacity` payload. */
export interface ChangeIdealLoadsSystemMaxHeatingCapacity {
  readonly mutation: "changeIdealLoadsSystemMaxHeatingCapacity";
  readonly id: number;
  readonly newCapacityPresent: boolean;
  readonly newMaxHeatingCapacityW: number;
}

/** 🟧️ `change-ideal-loads-system-max-cooling-capacity` payload. */
export interface ChangeIdealLoadsSystemMaxCoolingCapacity {
  readonly mutation: "changeIdealLoadsSystemMaxCoolingCapacity";
  readonly id: number;
  readonly newCapacityPresent: boolean;
  readonly newMaxCoolingCapacityW: number;
}

/** 🧍️ `change-ideal-loads-system-outdoor-air-per-person` payload. */
export interface ChangeIdealLoadsSystemOutdoorAirPerPerson {
  readonly mutation: "changeIdealLoadsSystemOutdoorAirPerPerson";
  readonly id: number;
  readonly newOutdoorAirPerPersonM3S: number;
}

/** 🔳️ `change-ideal-loads-system-outdoor-air-per-area` payload. */
export interface ChangeIdealLoadsSystemOutdoorAirPerArea {
  readonly mutation: "changeIdealLoadsSystemOutdoorAirPerArea";
  readonly id: number;
  readonly newOutdoorAirPerAreaM3SM2: number;
}
