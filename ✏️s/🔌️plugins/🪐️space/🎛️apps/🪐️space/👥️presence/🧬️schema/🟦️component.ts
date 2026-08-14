/** 🧬️ SpacePresence */
export interface SpacePresence {
  /** @state presence */
  camera: Record<string, SpaceWindowCamera>;
  /** @state presence */
  activeNodeId?: string;
  /** @state presence */
  focusedNodeId?: string;
  /** @state presence */
  collapsedNodeIds: string[];
  /** @state presence */
  previewOffNodeIds: string[];
}

export interface SpaceWindowCamera {
  x: number;
  y: number;
  zoom: number;
}
