/** 🧬️ StlArtifact schema — full `stdio.stl` artifact state (mirrors `StlSnapshot`). */
export interface StlTriangle {
  normal: [number, number, number];
  vertices: [[number, number, number], [number, number, number], [number, number, number]];
}
export interface StlArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ solidName: string;
  /** @state artifact */ triangles: StlTriangle[];
}
