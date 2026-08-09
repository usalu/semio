/** 🧬️ DagPresence */
export interface DagPresence {
  /** @state shared-ui */
  selectedNodeIds: string[];
  /** @state shared-ui */
  cameraX: number;
  /** @state shared-ui */
  cameraY: number;
  /** @state shared-ui */
  cameraZoom: number;
  /** @state shared-ui */
  hoveredNodeId?: string;
  /** @state shared-ui */
  hoveredEdgeId?: string;
}
