/** 🧾️ Energy-model mutation payloads of the `shading` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
/** 🌳️ `create-shading-surface` payload. */
export interface CreateShadingSurface {
  readonly mutation: "createShadingSurface";
  readonly id: number;
  readonly name: string;
  readonly verticesM: readonly (readonly [number, number, number])[];
  readonly transmittanceScheduleId: number | null;
  readonly index: number | null;
}

/** 🪵️ `delete-shading-surface` payload. */
export interface DeleteShadingSurface {
  readonly mutation: "deleteShadingSurface";
  readonly id: number;
}

/** 🏕️ `rename-shading-surface` payload. */
export interface RenameShadingSurface {
  readonly mutation: "renameShadingSurface";
  readonly id: number;
  readonly newName: string;
}

/** 🗺️ `replace-shading-surface-vertices` payload. */
export interface ReplaceShadingSurfaceVertices {
  readonly mutation: "replaceShadingSurfaceVertices";
  readonly id: number;
  readonly newVerticesM: readonly (readonly [number, number, number])[];
}

/** ⛱️ `change-shading-surface-transmittance-schedule` payload. */
export interface ChangeShadingSurfaceTransmittanceSchedule {
  readonly mutation: "changeShadingSurfaceTransmittanceSchedule";
  readonly id: number;
  readonly newTransmittanceScheduleId: number | null;
}
