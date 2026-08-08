/** 🧬️ Puzzle2d artifact schema — every field with its state class. */

export interface Puzzle2dArtifact {
  /** @state persistent */
  schema: string;
  /** @state persistent */
  camera: Puzzle2dCamera;
  /** @state persistent */
  nodes: Puzzle2dNode[];
  /** @state persistent */
  edges: Puzzle2dEdge[];
  /** @state persistent */
  meta: Puzzle2dMeta;
  /** @state shared-ui */
  selectedIds: string[];
  /** @state shared-ui */
  activeUtilityId: string;
  /** @state local-ui */
  cameraX: number;
  /** @state local-ui */
  cameraY: number;
  /** @state local-ui */
  cameraZoom: number;
  /** @state local-ui */
  selectionMethod: string;
  /** @state local-ui */
  gridSnapEnabled: boolean;
  /** @state local-ui */
  gridFactor: number;
  /** @state local-ui */
  suggestionOffset: number;
  /** @state local-ui */
  fillCount: number;
  /** @state local-ui */
  brushCandidateIndex: number;
  /** @state local-ui */
  brushCandidateSourceHandleId: string;
  /** @state local-ui */
  locale: string;
  /** @state local-ui */
  terminology: string;
  /** @state local-ui */
  lodModeByPaneJson: string;
  /** @state local-ui */
  engagementInputByPaneJson: string;
  /** @state local-ui */
  brushCandidatesJson: string;
  /** @state local-ui */
  nodeKindWeightsJson: string;
  /** @state local-ui */
  handleKindWeightsJson: string;
  /** @state local-ui */
  activeUtilityByWindowIdJson: string;
  /** @state preview */
  hoveredNodeId?: string;
  /** @state preview */
  previewSeq: number;
}

