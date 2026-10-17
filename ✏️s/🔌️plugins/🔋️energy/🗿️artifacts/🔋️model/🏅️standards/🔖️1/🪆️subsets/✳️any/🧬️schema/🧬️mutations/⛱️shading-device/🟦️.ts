/** ⛱️ Energy-model mutation payloads of the `shading-device` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
/** 🧢️ `change-fenestration-overhang-depth` payload. */
export interface ChangeFenestrationOverhangDepth {
  readonly mutation: "changeFenestrationOverhangDepth";
  readonly id: number;
  readonly newOverhangDepthM: number;
}

/** 🎩️ `change-fenestration-overhang-offset` payload. */
export interface ChangeFenestrationOverhangOffset {
  readonly mutation: "changeFenestrationOverhangOffset";
  readonly id: number;
  readonly newOverhangOffsetM: number;
}

/** 🐬️ `change-fenestration-fin-depth` payload. */
export interface ChangeFenestrationFinDepth {
  readonly mutation: "changeFenestrationFinDepth";
  readonly id: number;
  readonly newFinDepthM: number;
}

/** 🐋️ `change-fenestration-fin-offset` payload. */
export interface ChangeFenestrationFinOffset {
  readonly mutation: "changeFenestrationFinOffset";
  readonly id: number;
  readonly newFinOffsetM: number;
}
