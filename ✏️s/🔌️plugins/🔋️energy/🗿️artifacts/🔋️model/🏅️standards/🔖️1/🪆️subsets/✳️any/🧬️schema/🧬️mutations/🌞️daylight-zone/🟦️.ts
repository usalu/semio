/** 🌞️ Energy-model mutation payloads of the `daylight-zone` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
/** 🔭️ `create-daylight-zone` payload. */
export interface CreateDaylightZone {
  readonly mutation: "createDaylightZone";
  readonly id: number;
  readonly zoneId: number;
  readonly illuminanceTargetLux: number;
  readonly glareLimit: number;
  readonly windowTransmittance: number;
  readonly index: number | null;
}

/** 🌗️ `delete-daylight-zone` payload. */
export interface DeleteDaylightZone {
  readonly mutation: "deleteDaylightZone";
  readonly id: number;
}

/** 🏫️ `change-daylight-zone-zone` payload. */
export interface ChangeDaylightZoneZone {
  readonly mutation: "changeDaylightZoneZone";
  readonly id: number;
  readonly newZoneId: number;
}

/** 🪔️ `change-daylight-zone-illuminance-target` payload. */
export interface ChangeDaylightZoneIlluminanceTarget {
  readonly mutation: "changeDaylightZoneIlluminanceTarget";
  readonly id: number;
  readonly newIlluminanceTargetLux: number;
}

/** 🕶️ `change-daylight-zone-glare-limit` payload. */
export interface ChangeDaylightZoneGlareLimit {
  readonly mutation: "changeDaylightZoneGlareLimit";
  readonly id: number;
  readonly newGlareLimit: number;
}

/** 🥃️ `change-daylight-zone-window-transmittance` payload. */
export interface ChangeDaylightZoneWindowTransmittance {
  readonly mutation: "changeDaylightZoneWindowTransmittance";
  readonly id: number;
  readonly newWindowTransmittance: number;
}
