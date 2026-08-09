/** 🧬️ SpaceConfig */
export interface SpaceConfig {
  /** @state local-ui */
  camera: Record<string, SpaceWindowCamera>;
  /** @state local-ui */
  selectedNodeIds: string[];
  /** @state local-ui */
  hoveredNodeId?: string;
  /** @state local-ui */
  collapsedNodeIds: string[];
  /** @state local-ui */
  previewOffNodeIds: string[];
  /** @state local-ui */
  activeNodeId?: string;
  /** @state local-ui */
  focusedNodeId?: string;
  /** @state local-ui */
  clipboardNodeIds: string[];
  /** @state local-ui */
  workflowEngagementInput: string;
  /** @state local-ui */
  compiledDagEngagementInput: string;
  /** @state local-ui */
  pendingImportNodeId?: string;
  /** @state local-ui */
  pendingImportFormat?: string;
  /** @state local-ui */
  activePanelTab: string;
  /** @state local-ui */
  spaceId?: string;
  /** @state local-ui */
  clientId?: string;
  /** @state local-ui */
  clientName?: string;
  /** @state local-ui */
  locale: string;
}

export interface SpaceWindowCamera {
  x: number;
  y: number;
  zoom: number;
}
