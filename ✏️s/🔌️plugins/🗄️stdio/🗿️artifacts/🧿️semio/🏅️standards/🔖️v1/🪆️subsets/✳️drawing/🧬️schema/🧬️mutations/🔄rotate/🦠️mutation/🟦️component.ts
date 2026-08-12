/** mutation payload — mirrors `Rotate`. No-op on `Path`/`Text`/`Image` (no rotation field). */
export interface Rotate {
  at: { layer: number; path: number[] };
  newRotation: { x: number; y: number; z: number; w: number };
}
