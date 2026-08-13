/** 🧬️ DagPresence */
export interface DagPresence {
  /** @state presence */
  selectedNodeIds: string[];
  /** @state presence */
  cameraX: number;
  /** @state presence */
  cameraY: number;
  /** @state presence */
  cameraZoom: number;
  /** @state presence */
  hoveredNodeId?: string;
  /** @state presence */
  hoveredEdgeId?: string;
}
