/** 🧬️ Puzzle2dConfig */
export interface Puzzle2dConfig {
  /** @state local-ui */
  selectedIds: string[];
  /** @state local-ui */
  cameraX: number;
  /** @state local-ui */
  cameraY: number;
  /** @state local-ui */
  cameraZoom: number;
  /** @state local-ui */
  lodModeByPane: Record<string, string>;
  /** @state local-ui */
  engagementInputByPane: Record<string, string>;
  /** @state local-ui */
  brushCandidateIndex: number;
  /** @state local-ui */
  brushCandidates: unknown[];
  /** @state local-ui */
  brushCandidateSourceHandleId: string;
  /** @state local-ui */
  fillCount: number;
  /** @state local-ui */
  selectionMethod: string;
  /** @state local-ui */
  gridSnapEnabled: boolean;
  /** @state local-ui */
  gridFactor: number;
  /** @state local-ui */
  suggestionOffset: number;
  /** @state local-ui */
  nodeKindWeights: Record<string, number>;
  /** @state local-ui */
  handleKindWeights: Record<string, number>;
  /** @state local-ui */
  activeUtilityByWindowId: Record<string, string>;
  /** @state local-ui */
  locale: string;
  /** @state local-ui */
  terminology: string;
}
