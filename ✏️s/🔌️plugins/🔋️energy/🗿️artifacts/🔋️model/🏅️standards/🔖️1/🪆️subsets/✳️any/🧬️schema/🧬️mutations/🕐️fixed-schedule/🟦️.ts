/** 🕐️ Energy-model mutation payloads of the `fixed-schedule` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
/** 🕜️ `create-constant-schedule` payload. */
export interface CreateConstantSchedule {
  readonly mutation: "createConstantSchedule";
  readonly index: number;
  readonly id: number;
  readonly value: number;
}

/** 📍️ `delete-constant-schedule` payload. */
export interface DeleteConstantSchedule {
  readonly mutation: "deleteConstantSchedule";
  readonly id: number;
}

/** 🕝️ `change-constant-schedule-value` payload. */
export interface ChangeConstantScheduleValue {
  readonly mutation: "changeConstantScheduleValue";
  readonly id: number;
  readonly newValue: number;
}
