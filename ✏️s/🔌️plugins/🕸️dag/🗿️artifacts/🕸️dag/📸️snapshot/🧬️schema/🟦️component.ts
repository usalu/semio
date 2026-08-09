/** 📸️ DAG snapshot schema — persistent fields only. */
export interface DagSnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ nodes: DagNodeSpec[];
  /** @state persistent */ edges: DagFixtureEdge[];
}
export interface DagNodeSpec { id: string; [key: string]: unknown; }
export interface DagFixtureEdge { id: string; source: string; target: string; }
