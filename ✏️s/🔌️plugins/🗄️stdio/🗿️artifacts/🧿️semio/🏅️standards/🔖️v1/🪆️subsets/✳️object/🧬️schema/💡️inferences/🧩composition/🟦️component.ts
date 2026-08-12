/** 🧩 `composition` — the semio object's own child-presence census + placement. */

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
