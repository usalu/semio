/** 🧬️ SpaceConfig */
export interface SpaceConfig {
  /** @state config */
  camera: Record<string, SpaceWindowCamera>;
  /** @state config */
  collapsedNodeIds: string[];
  /** @state config */
  previewOffNodeIds: string[];
  /** @state config */
  activeNodeId?: string;
  /** @state config */
  focusedNodeId?: string;
  /** @state config */
  clipboardNodeIds: string[];
  /** @state config */
  workflowEngagementInput: string;
  /** @state config */
  compiledDagEngagementInput: string;
  /** @state config */
  pendingImportNodeId?: string;
  /** @state config */
  pendingImportFormat?: string;
  /** @state config */
  activePanelTab: string;
  /** @state config */
  spaceId?: string;
  /** @state config */
  clientId?: string;
  /** @state config */
  clientName?: string;
  /** @state config */
  locale: string;
}

export interface SpaceWindowCamera {
  x: number;
  y: number;
  zoom: number;
}
