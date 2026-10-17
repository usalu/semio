/** 📆️ Energy-model mutation payloads of the `year-schedule` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
/** 📚️ `create-annual-schedule` payload. */
export interface CreateAnnualSchedule {
  readonly mutation: "createAnnualSchedule";
  readonly index: number;
  readonly id: number;
  readonly defaultDailyScheduleId: number;
  readonly holidayDailyScheduleId: number | null;
}

/** 📕️ `delete-annual-schedule` payload. */
export interface DeleteAnnualSchedule {
  readonly mutation: "deleteAnnualSchedule";
  readonly id: number;
}

/** 📗️ `insert-annual-schedule-rule` payload. */
export interface InsertAnnualScheduleRule {
  readonly mutation: "insertAnnualScheduleRule";
  readonly id: number;
  readonly index: number;
  readonly startMonth: number;
  readonly startDay: number;
  readonly endMonth: number;
  readonly endDay: number;
  readonly dailyScheduleId: number;
}

/** 📙️ `remove-annual-schedule-rule` payload. */
export interface RemoveAnnualScheduleRule {
  readonly mutation: "removeAnnualScheduleRule";
  readonly id: number;
  readonly index: number;
}

/** 🗂️ `reorder-annual-schedule-rules` payload. */
export interface ReorderAnnualScheduleRules {
  readonly mutation: "reorderAnnualScheduleRules";
  readonly id: number;
  readonly from: number;
  readonly to: number;
}

/** 🎌️ `change-annual-schedule-default-daily-schedule` payload. */
export interface ChangeAnnualScheduleDefaultDailySchedule {
  readonly mutation: "changeAnnualScheduleDefaultDailySchedule";
  readonly id: number;
  readonly newDefaultDailyScheduleId: number;
}

/** 🎄️ `change-annual-schedule-holiday-daily-schedule` payload. */
export interface ChangeAnnualScheduleHolidayDailySchedule {
  readonly mutation: "changeAnnualScheduleHolidayDailySchedule";
  readonly id: number;
  readonly newHolidayDailyScheduleId: number | null;
}

/** 🎉️ `add-annual-schedule-holiday` payload. */
export interface AddAnnualScheduleHoliday {
  readonly mutation: "addAnnualScheduleHoliday";
  readonly id: number;
  readonly index: number;
  readonly year: number;
  readonly month: number;
  readonly day: number;
}

/** 🎊️ `remove-annual-schedule-holiday` payload. */
export interface RemoveAnnualScheduleHoliday {
  readonly mutation: "removeAnnualScheduleHoliday";
  readonly id: number;
  readonly year: number;
  readonly month: number;
  readonly day: number;
}
