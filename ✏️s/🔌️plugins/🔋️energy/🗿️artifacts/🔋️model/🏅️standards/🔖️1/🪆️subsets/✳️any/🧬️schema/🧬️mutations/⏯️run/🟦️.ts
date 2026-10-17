/** ⏯️ Energy-model mutation payloads of the `run` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
/** 🛫️ `change-run-start-month` payload. */
export interface ChangeRunStartMonth {
  readonly mutation: "changeRunStartMonth";
  readonly newStartMonth: number;
}

/** ▶️ `change-run-start-day` payload. */
export interface ChangeRunStartDay {
  readonly mutation: "changeRunStartDay";
  readonly newStartDay: number;
}

/** 🛬️ `change-run-end-month` payload. */
export interface ChangeRunEndMonth {
  readonly mutation: "changeRunEndMonth";
  readonly newEndMonth: number;
}

/** ⏹️ `change-run-end-day` payload. */
export interface ChangeRunEndDay {
  readonly mutation: "changeRunEndDay";
  readonly newEndDay: number;
}

/** 📅️ `change-run-year` payload. */
export interface ChangeRunYear {
  readonly mutation: "changeRunYear";
  readonly newYear: number;
}
