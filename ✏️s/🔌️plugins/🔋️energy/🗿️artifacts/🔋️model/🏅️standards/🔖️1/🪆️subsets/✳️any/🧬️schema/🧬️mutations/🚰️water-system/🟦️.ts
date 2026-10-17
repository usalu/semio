/** 🚰️ Energy-model mutation payloads of the `water-system` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
/** 🚽️ `create-water-system` payload. */
export interface CreateWaterSystem {
  readonly mutation: "createWaterSystem";
  readonly index: number;
  readonly id: number;
  readonly fixtureCount: number;
  readonly peakFlowLS: number;
  readonly scheduleId: number;
}

/** 🧼️ `delete-water-system` payload. */
export interface DeleteWaterSystem {
  readonly mutation: "deleteWaterSystem";
  readonly id: number;
}

/** 🪣️ `change-water-system-fixture-count` payload. */
export interface ChangeWaterSystemFixtureCount {
  readonly mutation: "changeWaterSystemFixtureCount";
  readonly id: number;
  readonly newFixtureCount: number;
}

/** 🚾️ `change-water-system-peak-flow` payload. */
export interface ChangeWaterSystemPeakFlow {
  readonly mutation: "changeWaterSystemPeakFlow";
  readonly id: number;
  readonly newPeakFlowLS: number;
}

/** 🕒️ `change-water-system-schedule` payload. */
export interface ChangeWaterSystemSchedule {
  readonly mutation: "changeWaterSystemSchedule";
  readonly id: number;
  readonly newScheduleId: number;
}
