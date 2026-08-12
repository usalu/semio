/** 💡️ Semio object inference schema — composition census + own placement. */

export interface SemioPoint3 {
  x: number;
  y: number;
  z: number;
}

export interface SemioObjectComposition {
  hasBrep: boolean;
  hasMesh: boolean;
  hasProperties: boolean;
  position: SemioPoint3;
}

export interface SemioObjectInference {
  /** @state inferred */
  composition: SemioObjectComposition;
}
