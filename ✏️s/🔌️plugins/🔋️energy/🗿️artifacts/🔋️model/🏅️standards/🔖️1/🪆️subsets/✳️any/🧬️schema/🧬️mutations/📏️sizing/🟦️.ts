/** 📏️ Energy-model mutation payloads of the `sizing` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
/** 📶️ `create-sizing-object` payload. */
export interface CreateSizingObject {
  readonly mutation: "createSizingObject";
  readonly id: number;
  readonly zoneId: number;
  readonly sizingType: "Heating" | "Cooling" | "OutdoorAir";
  readonly designDayType: "Heating" | "Cooling";
  readonly index: number | null;
}

/** 🪒️ `delete-sizing-object` payload. */
export interface DeleteSizingObject {
  readonly mutation: "deleteSizingObject";
  readonly id: number;
}

/** 🏨️ `change-sizing-object-zone` payload. */
export interface ChangeSizingObjectZone {
  readonly mutation: "changeSizingObjectZone";
  readonly id: number;
  readonly newZoneId: number;
}

/** 🧾️ `change-sizing-object-sizing-type` payload. */
export interface ChangeSizingObjectSizingType {
  readonly mutation: "changeSizingObjectSizingType";
  readonly id: number;
  readonly newSizingType: "Heating" | "Cooling" | "OutdoorAir";
}

/** 🌥️ `change-sizing-object-design-day-type` payload. */
export interface ChangeSizingObjectDesignDayType {
  readonly mutation: "changeSizingObjectDesignDayType";
  readonly id: number;
  readonly newDesignDayType: "Heating" | "Cooling";
}
