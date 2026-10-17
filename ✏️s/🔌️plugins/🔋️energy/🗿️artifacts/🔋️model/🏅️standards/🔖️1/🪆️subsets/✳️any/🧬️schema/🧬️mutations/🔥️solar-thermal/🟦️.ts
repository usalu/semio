/** 🔥️ Energy-model mutation payloads of the `solar-thermal` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
/** 🌄️ `create-solar-thermal-system` payload. */
export interface CreateSolarThermalSystem {
  readonly mutation: "createSolarThermalSystem";
  readonly index: number;
  readonly id: number;
  readonly collectorAreaM2: number;
  readonly efficiency: number;
  readonly storageVolumeM3: number;
  readonly tiltDeg: number;
  readonly azimuthDeg: number;
}

/** 🌆️ `delete-solar-thermal-system` payload. */
export interface DeleteSolarThermalSystem {
  readonly mutation: "deleteSolarThermalSystem";
  readonly id: number;
}

/** 🟩️ `change-solar-thermal-system-collector-area` payload. */
export interface ChangeSolarThermalSystemCollectorArea {
  readonly mutation: "changeSolarThermalSystemCollectorArea";
  readonly id: number;
  readonly newCollectorAreaM2: number;
}

/** 🏅️ `change-solar-thermal-system-efficiency` payload. */
export interface ChangeSolarThermalSystemEfficiency {
  readonly mutation: "changeSolarThermalSystemEfficiency";
  readonly id: number;
  readonly newEfficiency: number;
}

/** 🧃️ `change-solar-thermal-system-storage-volume` payload. */
export interface ChangeSolarThermalSystemStorageVolume {
  readonly mutation: "changeSolarThermalSystemStorageVolume";
  readonly id: number;
  readonly newStorageVolumeM3: number;
}

/** 🔼️ `change-solar-thermal-system-tilt` payload. */
export interface ChangeSolarThermalSystemTilt {
  readonly mutation: "changeSolarThermalSystemTilt";
  readonly id: number;
  readonly newTiltDeg: number;
}

/** ⛵️ `change-solar-thermal-system-azimuth` payload. */
export interface ChangeSolarThermalSystemAzimuth {
  readonly mutation: "changeSolarThermalSystemAzimuth";
  readonly id: number;
  readonly newAzimuthDeg: number;
}
