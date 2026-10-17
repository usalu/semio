/** 🧾️ Energy-model mutation payloads of the `lighting-gain` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
/** 💡️ `create-lighting-gain` payload. */
export interface CreateLightingGain {
  readonly mutation: "createLightingGain";
  readonly index: number;
  readonly id: number;
  readonly zoneId: number;
  readonly scheduleId: number;
  readonly wattsPerArea: number;
  readonly radiantFraction: number;
  readonly visibleFraction: number;
  readonly returnAirFraction: number;
}

/** 🕯️ `delete-lighting-gain` payload. */
export interface DeleteLightingGain {
  readonly mutation: "deleteLightingGain";
  readonly id: number;
}

/** 🔦️ `change-lighting-gain-zone` payload. */
export interface ChangeLightingGainZone {
  readonly mutation: "changeLightingGainZone";
  readonly id: number;
  readonly newZoneId: number;
}

/** ⏱️ `change-lighting-gain-schedule` payload. */
export interface ChangeLightingGainSchedule {
  readonly mutation: "changeLightingGainSchedule";
  readonly id: number;
  readonly newScheduleId: number;
}

/** 🔌️ `change-lighting-gain-watts-per-area` payload. */
export interface ChangeLightingGainWattsPerArea {
  readonly mutation: "changeLightingGainWattsPerArea";
  readonly id: number;
  readonly newWattsPerArea: number;
}

/** 🌟️ `change-lighting-gain-radiant-fraction` payload. */
export interface ChangeLightingGainRadiantFraction {
  readonly mutation: "changeLightingGainRadiantFraction";
  readonly id: number;
  readonly newRadiantFraction: number;
}

/** 🔅️ `change-lighting-gain-visible-fraction` payload. */
export interface ChangeLightingGainVisibleFraction {
  readonly mutation: "changeLightingGainVisibleFraction";
  readonly id: number;
  readonly newVisibleFraction: number;
}

/** 🎐️ `change-lighting-gain-return-air-fraction` payload. */
export interface ChangeLightingGainReturnAirFraction {
  readonly mutation: "changeLightingGainReturnAirFraction";
  readonly id: number;
  readonly newReturnAirFraction: number;
}
