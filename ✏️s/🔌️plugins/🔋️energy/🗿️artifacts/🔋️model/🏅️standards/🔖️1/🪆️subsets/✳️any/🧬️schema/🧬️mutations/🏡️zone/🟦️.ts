/** 🏡️ Energy-model mutation payloads of the `zone` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
/** 🏠️ `rename-zone` payload. */
export interface RenameZone {
  readonly mutation: "renameZone";
  readonly id: number;
  readonly newName: string;
}

/** 📦️ `change-zone-volume` payload. */
export interface ChangeZoneVolume {
  readonly mutation: "changeZoneVolume";
  readonly id: number;
  readonly newVolumeM3: number;
}

/** ✖️ `change-zone-multiplier` payload. */
export interface ChangeZoneMultiplier {
  readonly mutation: "changeZoneMultiplier";
  readonly id: number;
  readonly newMultiplier: number;
}

/** 🌬️ `change-zone-conditioned` payload. */
export interface ChangeZoneConditioned {
  readonly mutation: "changeZoneConditioned";
  readonly id: number;
  readonly newConditioned: boolean;
}

/** 📐️ `change-zone-floor-area-participation` payload. */
export interface ChangeZoneFloorAreaParticipation {
  readonly mutation: "changeZoneFloorAreaParticipation";
  readonly id: number;
  readonly newPartOfTotalFloorArea: boolean;
}

/** 🏘️ `create-zone` payload. */
export interface CreateZone {
  readonly mutation: "createZone";
  readonly id: number;
  readonly name: string;
  readonly volumeM3: number;
  readonly multiplier: number;
  readonly conditioned: boolean;
  readonly partOfTotalFloorArea: boolean;
  readonly index: number | null;
}

/** 🏚️ `delete-zone` payload. */
export interface DeleteZone {
  readonly mutation: "deleteZone";
  readonly id: number;
}
