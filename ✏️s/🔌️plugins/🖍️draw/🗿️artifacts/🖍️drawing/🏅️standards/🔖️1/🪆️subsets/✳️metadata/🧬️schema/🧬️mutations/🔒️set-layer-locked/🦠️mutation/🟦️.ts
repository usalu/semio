/** 🔒️ Drawing mutation — `SetLayerLocked` payload mirror: flips one layer's `locked` flag. */
export interface SetLayerLocked {
  layerId: string;
  locked: boolean;
}
