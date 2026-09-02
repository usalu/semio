/** 📸️ DAG snapshot schema — artifact-lane fields only. */
export interface DagSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ nodes: DagNodeSpec[];
  /** @state artifact */ edges: DagFixtureEdge[];
}
export interface DagNodeSpec { id: string; [key: string]: unknown; }
export interface DagFixtureEdge { id: string; source: string; target: string; }
