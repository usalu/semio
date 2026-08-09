/** 🧬️ SpacePresence */
export interface SpacePresence {
  /** @state shared-ui */
  selectedNodeIds: string[];
  /** @state shared-ui */
  hoveredNodeId?: string;
  /** @state shared-ui */
  camera: Record<string, SpaceWindowCamera>;
  /** @state shared-ui */
  activeNodeId?: string;
  /** @state shared-ui */
  focusedNodeId?: string;
  /** @state shared-ui */
  collapsedNodeIds: string[];
  /** @state shared-ui */
  previewOffNodeIds: string[];
}

export interface SpaceWindowCamera {
  x: number;
  y: number;
  zoom: number;
}
