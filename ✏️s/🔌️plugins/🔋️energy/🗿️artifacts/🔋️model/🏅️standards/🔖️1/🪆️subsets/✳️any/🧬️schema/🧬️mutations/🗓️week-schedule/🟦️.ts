/** 🧾️ Energy-model mutation payloads of the `week-schedule` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
/** 🗓️ `create-weekly-schedule` payload. */
export interface CreateWeeklySchedule {
  readonly mutation: "createWeeklySchedule";
  readonly index: number;
  readonly id: number;
  readonly dailyScheduleIds: readonly number[];
}

/** 🕖️ `delete-weekly-schedule` payload. */
export interface DeleteWeeklySchedule {
  readonly mutation: "deleteWeeklySchedule";
  readonly id: number;
}

/** 🕗️ `change-weekly-schedule-day` payload. */
export interface ChangeWeeklyScheduleDay {
  readonly mutation: "changeWeeklyScheduleDay";
  readonly id: number;
  readonly dayIndex: number;
  readonly newDailyScheduleId: number;
}
