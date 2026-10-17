/** 📍️ Energy-model mutation payloads of the `site` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
/** 🌍️ `change-site-latitude` payload. */
export interface ChangeSiteLatitude {
  readonly mutation: "changeSiteLatitude";
  readonly newLatitudeDeg: number;
}

/** 🌏️ `change-site-longitude` payload. */
export interface ChangeSiteLongitude {
  readonly mutation: "changeSiteLongitude";
  readonly newLongitudeDeg: number;
}

/** ⛰️ `change-site-elevation` payload. */
export interface ChangeSiteElevation {
  readonly mutation: "changeSiteElevation";
  readonly newElevationM: number;
}

/** 🕰️ `change-site-time-zone` payload. */
export interface ChangeSiteTimeZone {
  readonly mutation: "changeSiteTimeZone";
  readonly newTimeZoneHours: number;
}

/** 🔝️ `change-site-north-axis` payload. */
export interface ChangeSiteNorthAxis {
  readonly mutation: "changeSiteNorthAxis";
  readonly newNorthAxisDeg: number;
}

/** 🌡️ `change-ground-building` payload. */
export interface ChangeGroundBuilding {
  readonly mutation: "changeGroundBuilding";
  readonly month: number;
  readonly newTemperatureC: number;
}

/** 🌱️ `change-ground-shallow` payload. */
export interface ChangeGroundShallow {
  readonly mutation: "changeGroundShallow";
  readonly month: number;
  readonly newTemperatureC: number;
}

/** ⛏️ `change-ground-deep` payload. */
export interface ChangeGroundDeep {
  readonly mutation: "changeGroundDeep";
  readonly newTemperatureC: number;
}
