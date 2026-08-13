/** 💡️ Puzzle3d inference schema — flatPosition (plane + center) per object. */

export interface FlattenPlane {
  origin: [number, number, number];
  xAxis: [number, number, number];
  yAxis: [number, number, number];
}

export interface FlattenPose {
  plane: FlattenPlane;
  center: [number, number];
  orientation: [number, number, number, number];
}

export interface Puzzle3dInference {
  /** @derived */
  flatPositions: Record<string, FlattenPose>;
}
