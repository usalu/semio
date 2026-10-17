/** 🌡️ Energy-model mutation payloads of the `thermostat` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
/** 🩺️ `create-thermostat` payload. */
export interface CreateThermostat {
  readonly mutation: "createThermostat";
  readonly id: number;
  readonly zoneId: number;
  readonly heatingSetpointScheduleId: number;
  readonly coolingSetpointScheduleId: number;
  readonly heatingThrottleRangeK: number;
  readonly coolingThrottleRangeK: number;
  readonly index: number | null;
}

/** 🛑️ `delete-thermostat` payload. */
export interface DeleteThermostat {
  readonly mutation: "deleteThermostat";
  readonly id: number;
}

/** 🛖️ `change-thermostat-zone` payload. */
export interface ChangeThermostatZone {
  readonly mutation: "changeThermostatZone";
  readonly id: number;
  readonly newZoneId: number;
}

/** 🥵️ `change-thermostat-heating-setpoint-schedule` payload. */
export interface ChangeThermostatHeatingSetpointSchedule {
  readonly mutation: "changeThermostatHeatingSetpointSchedule";
  readonly id: number;
  readonly newHeatingSetpointScheduleId: number;
}

/** 🐧️ `change-thermostat-cooling-setpoint-schedule` payload. */
export interface ChangeThermostatCoolingSetpointSchedule {
  readonly mutation: "changeThermostatCoolingSetpointSchedule";
  readonly id: number;
  readonly newCoolingSetpointScheduleId: number;
}

/** 🎚️ `change-thermostat-heating-throttle-range` payload. */
export interface ChangeThermostatHeatingThrottleRange {
  readonly mutation: "changeThermostatHeatingThrottleRange";
  readonly id: number;
  readonly newHeatingThrottleRangeK: number;
}

/** 🎛️ `change-thermostat-cooling-throttle-range` payload. */
export interface ChangeThermostatCoolingThrottleRange {
  readonly mutation: "changeThermostatCoolingThrottleRange";
  readonly id: number;
  readonly newCoolingThrottleRangeK: number;
}
