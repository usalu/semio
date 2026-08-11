/** 🧬️ StlArtifact schema — full `stdio.stl` artifact state (mirrors `StlSnapshot`). */
export interface StlTriangle {
  normal: [number, number, number];
  vertices: [[number, number, number], [number, number, number], [number, number, number]];
}
export interface StlArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ solidName: string;
  /** @state persistent */ triangles: StlTriangle[];
}
