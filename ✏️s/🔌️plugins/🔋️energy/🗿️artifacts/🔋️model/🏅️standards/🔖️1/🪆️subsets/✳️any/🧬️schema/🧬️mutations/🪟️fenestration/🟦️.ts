/** 🧾️ Energy-model mutation payloads of the `fenestration` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
/** 🪟️ `create-fenestration` payload. */
export interface CreateFenestration {
  readonly mutation: "createFenestration";
  readonly id: number;
  readonly name: string;
  readonly surfaceId: number;
  readonly uValueWM2k: number;
  readonly shgc: number;
  readonly vlt: number;
  readonly areaM2: number;
  readonly heightM: number;
  readonly sillHeightM: number;
  readonly frameConductanceWK: number;
  readonly dividerConductanceWK: number;
  readonly overhangDepthM: number;
  readonly overhangOffsetM: number;
  readonly finDepthM: number;
  readonly finOffsetM: number;
  readonly glazingConstructionId: number | null;
  readonly index: number | null;
}

/** 🚪️ `delete-fenestration` payload. */
export interface DeleteFenestration {
  readonly mutation: "deleteFenestration";
  readonly id: number;
}

/** 🏁️ `rename-fenestration` payload. */
export interface RenameFenestration {
  readonly mutation: "renameFenestration";
  readonly id: number;
  readonly newName: string;
}

/** 🧲️ `change-fenestration-surface` payload. */
export interface ChangeFenestrationSurface {
  readonly mutation: "changeFenestrationSurface";
  readonly id: number;
  readonly newSurfaceId: number;
}

/** 🟥️ `change-fenestration-area` payload. */
export interface ChangeFenestrationArea {
  readonly mutation: "changeFenestrationArea";
  readonly id: number;
  readonly newAreaM2: number;
}

/** ⬆️ `change-fenestration-height` payload. */
export interface ChangeFenestrationHeight {
  readonly mutation: "changeFenestrationHeight";
  readonly id: number;
  readonly newHeightM: number;
}

/** ⬇️ `change-fenestration-sill-height` payload. */
export interface ChangeFenestrationSillHeight {
  readonly mutation: "changeFenestrationSillHeight";
  readonly id: number;
  readonly newSillHeightM: number;
}

/** 🔶️ `replace-fenestration-vertices` payload. */
export interface ReplaceFenestrationVertices {
  readonly mutation: "replaceFenestrationVertices";
  readonly id: number;
  readonly newVerticesM: readonly (readonly [number, number, number])[];
}
