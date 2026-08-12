/** mutation payload — mirrors `Scale`. No-op on `Path`/`Text`/`Image` (no scale field). */
export interface Scale {
  at: { layer: number; path: number[] };
  newScale: { x: number; y: number; z: number };
}
