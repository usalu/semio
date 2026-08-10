/** 🧬️ DAG artifact schema — every field with its state class. */
export interface DagArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ nodes: DagNodeSpec[];
  /** @state persistent */ edges: DagFixtureEdge[];
  /** @state shared-ui */ selectedNodeIds: string[];
  /** @state local-ui */ camera: DagCamera;
  /** @state local-ui */ locale: string;
}
export interface DagNodeSpec { id: string; [key: string]: unknown; }
export interface DagFixtureEdge { id: string; source: string; target: string; }
export interface DagCamera { x: number; y: number; zoom: number; }
