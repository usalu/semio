/** 📈️ Energy-model mutation payloads of the `time-series` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
/** 🪗️ `create-time-series-schedule` payload. */
export interface CreateTimeSeriesSchedule {
  readonly mutation: "createTimeSeriesSchedule";
  readonly index: number;
  readonly id: number;
  readonly values: readonly number[];
  readonly timestepSeconds: number;
}

/** 🎞️ `delete-time-series-schedule` payload. */
export interface DeleteTimeSeriesSchedule {
  readonly mutation: "deleteTimeSeriesSchedule";
  readonly id: number;
}

/** 🕘️ `replace-time-series-schedule-values` payload. */
export interface ReplaceTimeSeriesScheduleValues {
  readonly mutation: "replaceTimeSeriesScheduleValues";
  readonly id: number;
  readonly newValues: readonly number[];
}

/** 🕙️ `change-time-series-schedule-timestep` payload. */
export interface ChangeTimeSeriesScheduleTimestep {
  readonly mutation: "changeTimeSeriesScheduleTimestep";
  readonly id: number;
  readonly newTimestepSeconds: number;
}
