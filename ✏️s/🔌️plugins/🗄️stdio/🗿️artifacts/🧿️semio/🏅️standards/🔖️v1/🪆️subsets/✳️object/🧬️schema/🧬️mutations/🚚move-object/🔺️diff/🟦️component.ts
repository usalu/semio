/** 🔺️ `move-object` diff construction — real mirror (documents the shape `diff()` builds). */
export interface MoveObjectDiff {
  transform: { translation: { x: number; y: number; z: number }; rotation: { x: number; y: number; z: number; w: number }; scale: { x: number; y: number; z: number } };
}
