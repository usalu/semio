/** 🧬️ DAG artifact schema — every field with its state class. */
export interface DagArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ nodes: DagNodeSpec[];
  /** @state artifact */ edges: DagFixtureEdge[];
  /** @state presence */ selectedNodeIds: string[];
  /** @state config */ camera: DagCamera;
  /** @state config */ locale: string;
}
export interface DagNodeSpec { id: string; [key: string]: unknown; }
export interface DagFixtureEdge { id: string; source: string; target: string; }
export interface DagCamera { x: number; y: number; zoom: number; }
