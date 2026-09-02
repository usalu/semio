/** 🔺️ DAG diff schema — sparse field delta. */
export interface DagDiff {
  /** @state artifact */ artifact?: DagArtifact;
  /** @state artifact */ schema?: string;
  /** @state artifact */ nodes?: DagNodesDelta;
  /** @state artifact */ edges?: DagEdgesDelta;
    /** @state artifact */ setNodes?: DagNodeSpecList;
    /** @state artifact */ setEdges?: DagFixtureEdgeList;
  /** @state presence */ selectedNodeIds?: DagStringList;
  /** @state config */ camera?: DagCamera;
  /** @state config */ locale?: string;
}
export interface DagStringList { values: string[]; }
export interface DagNodesDelta { added: DagNodeSpec[]; removed: string[]; patched: DagNodePatchEntry[]; reordered?: string[]; }
export interface DagEdgesDelta { added: DagFixtureEdge[]; removed: string[]; patched: DagEdgePatchEntry[]; reordered?: string[]; }
export interface DagNodePatchEntry { id: string; patch: DagNodePatch; }
export interface DagEdgePatchEntry { id: string; patch: DagEdgePatch; }
export interface DagNodePatch { name?: string; x?: number; y?: number; }
export interface DagEdgePatch { source?: string; target?: string; }
export interface DagNodeSpecList { values: DagNodeSpec[]; }
export interface DagFixtureEdgeList { values: DagFixtureEdge[]; }
export interface DagNodeSpec { id: string; [key: string]: unknown; }
export interface DagFixtureEdge { id: string; source: string; target: string; }
export interface DagCamera { x: number; y: number; zoom: number; }
export interface DagArtifact {
  schema: string; nodes: DagNodeSpec[]; edges: DagFixtureEdge[];
  selectedNodeIds: string[]; camera: DagCamera; locale: string;
}
