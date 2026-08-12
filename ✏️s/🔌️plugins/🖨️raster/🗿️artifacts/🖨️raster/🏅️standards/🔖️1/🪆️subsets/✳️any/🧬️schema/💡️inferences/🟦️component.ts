/** 💡️ Raster inference schema — layer-tree topology (pre-order + nesting depth). */

export interface RasterTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}

export interface RasterInference {
  /** @state inferred */
  topology: RasterTopology;
}
