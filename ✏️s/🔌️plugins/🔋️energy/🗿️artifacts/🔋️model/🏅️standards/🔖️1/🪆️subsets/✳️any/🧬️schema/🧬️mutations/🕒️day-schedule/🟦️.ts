/** 🕒️ Energy-model mutation payloads of the `day-schedule` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
/** 🕞️ `create-daily-schedule` payload. */
export interface CreateDailySchedule {
  readonly mutation: "createDailySchedule";
  readonly index: number;
  readonly id: number;
  readonly hourlyValues: readonly number[];
  readonly interpolation: "Continuous" | "Discrete";
  readonly limitsMin: number | null;
  readonly limitsMax: number | null;
}

/** 🌓️ `delete-daily-schedule` payload. */
export interface DeleteDailySchedule {
  readonly mutation: "deleteDailySchedule";
  readonly id: number;
}

/** 🕔️ `replace-daily-schedule-hourly-values` payload. */
export interface ReplaceDailyScheduleHourlyValues {
  readonly mutation: "replaceDailyScheduleHourlyValues";
  readonly id: number;
  readonly newHourlyValues: readonly number[];
}

/** 🕕️ `change-daily-schedule-interpolation` payload. */
export interface ChangeDailyScheduleInterpolation {
  readonly mutation: "changeDailyScheduleInterpolation";
  readonly id: number;
  readonly newInterpolation: "Continuous" | "Discrete";
}

/** 🕟️ `change-daily-schedule-limits` payload. */
export interface ChangeDailyScheduleLimits {
  readonly mutation: "changeDailyScheduleLimits";
  readonly id: number;
  readonly newLimitsMin: number | null;
  readonly newLimitsMax: number | null;
}
