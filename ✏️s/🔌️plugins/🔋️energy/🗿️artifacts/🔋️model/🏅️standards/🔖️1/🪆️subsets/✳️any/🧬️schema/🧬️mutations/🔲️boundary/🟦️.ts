/** 🔲️ Energy-model mutation payloads of the `boundary` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
/** 🚧️ `change-surface-boundary-condition` payload. */
export interface ChangeSurfaceBoundaryCondition {
  readonly mutation: "changeSurfaceBoundaryCondition";
  readonly id: number;
  readonly newBoundary: "OutdoorAir" | "Ground" | "OtherSideTemperature" | "Adiabatic" | "Interzone";
  readonly newInterzoneSurfaceId: number | null;
}

/** 🌅️ `change-surface-sun-exposed` payload. */
export interface ChangeSurfaceSunExposed {
  readonly mutation: "changeSurfaceSunExposed";
  readonly id: number;
  readonly newSunExposed: boolean;
}

/** 🍃️ `change-surface-wind-exposed` payload. */
export interface ChangeSurfaceWindExposed {
  readonly mutation: "changeSurfaceWindExposed";
  readonly id: number;
  readonly newWindExposed: boolean;
}

/** 🤝️ `connect-surfaces` payload. */
export interface ConnectSurfaces {
  readonly mutation: "connectSurfaces";
  readonly surfaceAId: number;
  readonly surfaceBId: number;
  readonly index: number | null;
}

/** 💔️ `disconnect-surfaces` payload. */
export interface DisconnectSurfaces {
  readonly mutation: "disconnectSurfaces";
  readonly surfaceAId: number;
  readonly surfaceBId: number;
}
