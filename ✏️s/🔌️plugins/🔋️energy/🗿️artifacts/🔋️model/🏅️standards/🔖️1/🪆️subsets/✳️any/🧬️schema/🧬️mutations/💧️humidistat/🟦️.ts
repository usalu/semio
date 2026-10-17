/** 💧️ Energy-model mutation payloads of the `humidistat` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
/** 🌂️ `create-humidistat` payload. */
export interface CreateHumidistat {
  readonly mutation: "createHumidistat";
  readonly id: number;
  readonly zoneId: number;
  readonly humidifyingSetpointScheduleId: number;
  readonly dehumidifyingSetpointScheduleId: number;
  readonly humidifyingThrottleRange: number;
  readonly dehumidifyingThrottleRange: number;
  readonly index: number | null;
}

/** 🏜️ `delete-humidistat` payload. */
export interface DeleteHumidistat {
  readonly mutation: "deleteHumidistat";
  readonly id: number;
}

/** 🏙️ `change-humidistat-zone` payload. */
export interface ChangeHumidistatZone {
  readonly mutation: "changeHumidistatZone";
  readonly id: number;
  readonly newZoneId: number;
}

/** ☔️ `change-humidistat-humidifying-setpoint-schedule` payload. */
export interface ChangeHumidistatHumidifyingSetpointSchedule {
  readonly mutation: "changeHumidistatHumidifyingSetpointSchedule";
  readonly id: number;
  readonly newHumidifyingSetpointScheduleId: number;
}

/** 🏝️ `change-humidistat-dehumidifying-setpoint-schedule` payload. */
export interface ChangeHumidistatDehumidifyingSetpointSchedule {
  readonly mutation: "changeHumidistatDehumidifyingSetpointSchedule";
  readonly id: number;
  readonly newDehumidifyingSetpointScheduleId: number;
}

/** 🌧️ `change-humidistat-humidifying-throttle-range` payload. */
export interface ChangeHumidistatHumidifyingThrottleRange {
  readonly mutation: "changeHumidistatHumidifyingThrottleRange";
  readonly id: number;
  readonly newHumidifyingThrottleRange: number;
}

/** 🧻️ `change-humidistat-dehumidifying-throttle-range` payload. */
export interface ChangeHumidistatDehumidifyingThrottleRange {
  readonly mutation: "changeHumidistatDehumidifyingThrottleRange";
  readonly id: number;
  readonly newDehumidifyingThrottleRange: number;
}
