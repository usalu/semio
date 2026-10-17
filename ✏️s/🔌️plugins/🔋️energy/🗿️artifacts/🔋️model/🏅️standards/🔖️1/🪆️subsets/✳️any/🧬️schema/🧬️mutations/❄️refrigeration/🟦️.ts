/** 🧾️ Energy-model mutation payloads of the `refrigeration` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
/** ❄️ `create-refrigeration-system` payload. */
export interface CreateRefrigerationSystem {
  readonly mutation: "createRefrigerationSystem";
  readonly index: number;
  readonly id: number;
  readonly caseCount: number;
  readonly designLoadW: number;
  readonly defrostScheduleId: number;
}

/** 🫠️ `delete-refrigeration-system` payload. */
export interface DeleteRefrigerationSystem {
  readonly mutation: "deleteRefrigerationSystem";
  readonly id: number;
}

/** 🗄️ `change-refrigeration-system-case-count` payload. */
export interface ChangeRefrigerationSystemCaseCount {
  readonly mutation: "changeRefrigerationSystemCaseCount";
  readonly id: number;
  readonly newCaseCount: number;
}

/** 🏋️ `change-refrigeration-system-design-load` payload. */
export interface ChangeRefrigerationSystemDesignLoad {
  readonly mutation: "changeRefrigerationSystemDesignLoad";
  readonly id: number;
  readonly newDesignLoadW: number;
}

/** 🕑️ `change-refrigeration-system-defrost-schedule` payload. */
export interface ChangeRefrigerationSystemDefrostSchedule {
  readonly mutation: "changeRefrigerationSystemDefrostSchedule";
  readonly id: number;
  readonly newDefrostScheduleId: number;
}
