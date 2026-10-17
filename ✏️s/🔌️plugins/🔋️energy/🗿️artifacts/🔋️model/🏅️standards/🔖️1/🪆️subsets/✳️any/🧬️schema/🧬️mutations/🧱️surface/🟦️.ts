/** 🧱️ Energy-model mutation payloads of the `surface` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
/** 🟫️ `create-surface` payload. */
export interface CreateSurface {
  readonly mutation: "createSurface";
  readonly id: number;
  readonly name: string;
  readonly zoneId: number;
  readonly class: "ExteriorWall" | "InteriorWall" | "Roof" | "Ceiling" | "Floor" | "Interzone" | "Adiabatic" | "Ground";
  readonly verticesM: readonly (readonly [number, number, number])[];
  readonly constructionId: number;
  readonly boundary: "OutdoorAir" | "Ground" | "OtherSideTemperature" | "Adiabatic" | "Interzone";
  readonly interzoneSurfaceId: number | null;
  readonly sunExposed: boolean;
  readonly windExposed: boolean;
  readonly multiplier: number;
  readonly index: number | null;
}

/** 🪚️ `delete-surface` payload. */
export interface DeleteSurface {
  readonly mutation: "deleteSurface";
  readonly id: number;
}

/** 🏳️ `rename-surface` payload. */
export interface RenameSurface {
  readonly mutation: "renameSurface";
  readonly id: number;
  readonly newName: string;
}

/** 🗜️ `change-surface-zone` payload. */
export interface ChangeSurfaceZone {
  readonly mutation: "changeSurfaceZone";
  readonly id: number;
  readonly newZoneId: number;
}

/** 🧩️ `change-surface-class` payload. */
export interface ChangeSurfaceClass {
  readonly mutation: "changeSurfaceClass";
  readonly id: number;
  readonly newClass: "ExteriorWall" | "InteriorWall" | "Roof" | "Ceiling" | "Floor" | "Interzone" | "Adiabatic" | "Ground";
}

/** 🔺️ `replace-surface-vertices` payload. */
export interface ReplaceSurfaceVertices {
  readonly mutation: "replaceSurfaceVertices";
  readonly id: number;
  readonly newVerticesM: readonly (readonly [number, number, number])[];
}

/** 🧰️ `change-surface-construction` payload. */
export interface ChangeSurfaceConstruction {
  readonly mutation: "changeSurfaceConstruction";
  readonly id: number;
  readonly newConstructionId: number;
}

/** 🔁️ `change-surface-multiplier` payload. */
export interface ChangeSurfaceMultiplier {
  readonly mutation: "changeSurfaceMultiplier";
  readonly id: number;
  readonly newMultiplier: number;
}
