/** 🪫️ Energy-model mutation payloads of the `battery` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
/** 🪙️ `create-battery` payload. */
export interface CreateBattery {
  readonly mutation: "createBattery";
  readonly index: number;
  readonly id: number;
  readonly capacityKwh: number;
  readonly maxChargeW: number;
  readonly maxDischargeW: number;
  readonly roundTripEfficiency: number;
}

/** ♒️ `delete-battery` payload. */
export interface DeleteBattery {
  readonly mutation: "deleteBattery";
  readonly id: number;
}

/** 🥫️ `change-battery-capacity` payload. */
export interface ChangeBatteryCapacity {
  readonly mutation: "changeBatteryCapacity";
  readonly id: number;
  readonly newCapacityKwh: number;
}

/** ⏫️ `change-battery-max-charge` payload. */
export interface ChangeBatteryMaxCharge {
  readonly mutation: "changeBatteryMaxCharge";
  readonly id: number;
  readonly newMaxChargeW: number;
}

/** ⏬️ `change-battery-max-discharge` payload. */
export interface ChangeBatteryMaxDischarge {
  readonly mutation: "changeBatteryMaxDischarge";
  readonly id: number;
  readonly newMaxDischargeW: number;
}

/** 🥉️ `change-battery-round-trip-efficiency` payload. */
export interface ChangeBatteryRoundTripEfficiency {
  readonly mutation: "changeBatteryRoundTripEfficiency";
  readonly id: number;
  readonly newRoundTripEfficiency: number;
}
