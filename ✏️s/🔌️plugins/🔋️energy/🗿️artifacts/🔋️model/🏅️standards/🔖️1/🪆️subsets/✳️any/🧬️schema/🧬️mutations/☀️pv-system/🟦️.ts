/** ☀️ Energy-model mutation payloads of the `pv-system` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
/** ✨️ `create-pv-system` payload. */
export interface CreatePvSystem {
  readonly mutation: "createPvSystem";
  readonly index: number;
  readonly id: number;
  readonly dcCapacityW: number;
  readonly areaM2: number;
  readonly tiltDeg: number;
  readonly azimuthDeg: number;
  readonly moduleEfficiency: number;
  readonly inverterEfficiency: number;
}

/** 🌒️ `delete-pv-system` payload. */
export interface DeletePvSystem {
  readonly mutation: "deletePvSystem";
  readonly id: number;
}

/** ⚛️ `change-pv-system-dc-capacity` payload. */
export interface ChangePvSystemDcCapacity {
  readonly mutation: "changePvSystemDcCapacity";
  readonly id: number;
  readonly newDcCapacityW: number;
}

/** 🟨️ `change-pv-system-area` payload. */
export interface ChangePvSystemArea {
  readonly mutation: "changePvSystemArea";
  readonly id: number;
  readonly newAreaM2: number;
}

/** 📈️ `change-pv-system-tilt` payload. */
export interface ChangePvSystemTilt {
  readonly mutation: "changePvSystemTilt";
  readonly id: number;
  readonly newTiltDeg: number;
}

/** 🧿️ `change-pv-system-azimuth` payload. */
export interface ChangePvSystemAzimuth {
  readonly mutation: "changePvSystemAzimuth";
  readonly id: number;
  readonly newAzimuthDeg: number;
}

/** 🎖️ `change-pv-system-module-efficiency` payload. */
export interface ChangePvSystemModuleEfficiency {
  readonly mutation: "changePvSystemModuleEfficiency";
  readonly id: number;
  readonly newModuleEfficiency: number;
}

/** ♌️ `change-pv-system-inverter-efficiency` payload. */
export interface ChangePvSystemInverterEfficiency {
  readonly mutation: "changePvSystemInverterEfficiency";
  readonly id: number;
  readonly newInverterEfficiency: number;
}
