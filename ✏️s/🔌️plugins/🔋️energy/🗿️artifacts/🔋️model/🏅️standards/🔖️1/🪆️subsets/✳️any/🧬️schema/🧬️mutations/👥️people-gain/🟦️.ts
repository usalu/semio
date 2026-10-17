/** 🧾️ Energy-model mutation payloads of the `people-gain` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
/** 👤️ `create-people-gain` payload. */
export interface CreatePeopleGain {
  readonly mutation: "createPeopleGain";
  readonly index: number;
  readonly id: number;
  readonly zoneId: number;
  readonly scheduleId: number;
  readonly activityScheduleId: number;
  readonly peoplePerArea: number;
  readonly sensibleFraction: number;
  readonly latentFraction: number;
  readonly radiantFraction: number;
}

/** 🚷️ `delete-people-gain` payload. */
export interface DeletePeopleGain {
  readonly mutation: "deletePeopleGain";
  readonly id: number;
}

/** 🚶️ `change-people-gain-zone` payload. */
export interface ChangePeopleGainZone {
  readonly mutation: "changePeopleGainZone";
  readonly id: number;
  readonly newZoneId: number;
}

/** ⏰️ `change-people-gain-schedule` payload. */
export interface ChangePeopleGainSchedule {
  readonly mutation: "changePeopleGainSchedule";
  readonly id: number;
  readonly newScheduleId: number;
}

/** 🏃️ `change-people-gain-activity-schedule` payload. */
export interface ChangePeopleGainActivitySchedule {
  readonly mutation: "changePeopleGainActivitySchedule";
  readonly id: number;
  readonly newActivityScheduleId: number;
}

/** 👥️ `change-people-gain-people-per-area` payload. */
export interface ChangePeopleGainPeoplePerArea {
  readonly mutation: "changePeopleGainPeoplePerArea";
  readonly id: number;
  readonly newPeoplePerArea: number;
}

/** 🌞️ `change-people-gain-sensible-fraction` payload. */
export interface ChangePeopleGainSensibleFraction {
  readonly mutation: "changePeopleGainSensibleFraction";
  readonly id: number;
  readonly newSensibleFraction: number;
}

/** 💧️ `change-people-gain-latent-fraction` payload. */
export interface ChangePeopleGainLatentFraction {
  readonly mutation: "changePeopleGainLatentFraction";
  readonly id: number;
  readonly newLatentFraction: number;
}

/** 📡️ `change-people-gain-radiant-fraction` payload. */
export interface ChangePeopleGainRadiantFraction {
  readonly mutation: "changePeopleGainRadiantFraction";
  readonly id: number;
  readonly newRadiantFraction: number;
}
