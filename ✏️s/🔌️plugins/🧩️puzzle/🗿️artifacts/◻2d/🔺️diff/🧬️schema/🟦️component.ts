/** 🧬️ Puzzle2d diff schema — sparse field delta. */

export interface Puzzle2dDiff {
  /** @state persistent */
  artifact?: Puzzle2dArtifact;
  /** @state persistent */
  schema?: string;
  /** @state persistent */
  camera?: Puzzle2dCamera;
  /** @state persistent */
  nodes?: Puzzle2dNodesDelta;
  /** @state persistent */
  edges?: Puzzle2dEdgesDelta;
  /** @state persistent */
  meta?: Puzzle2dMeta;
  /** @state shared-ui */
  selectedIds?: Puzzle2dStringList;
  /** @state shared-ui */
  activeUtilityId?: string;
  /** @state local-ui */
  cameraX?: number;
  /** @state local-ui */
  cameraY?: number;
  /** @state local-ui */
  cameraZoom?: number;
  /** @state local-ui */
  selectionMethod?: string;
  /** @state local-ui */
  gridSnapEnabled?: boolean;
  /** @state local-ui */
  gridFactor?: number;
  /** @state local-ui */
  suggestionOffset?: number;
  /** @state local-ui */
  fillCount?: number;
  /** @state local-ui */
  brushCandidateIndex?: number;
  /** @state local-ui */
  brushCandidateSourceHandleId?: string;
  /** @state local-ui */
  locale?: string;
  /** @state local-ui */
  terminology?: string;
  /** @state local-ui */
  lodModeByPaneJson?: string;
  /** @state local-ui */
  engagementInputByPaneJson?: string;
  /** @state local-ui */
  brushCandidatesJson?: string;
  /** @state local-ui */
  nodeKindWeightsJson?: string;
  /** @state local-ui */
  handleKindWeightsJson?: string;
  /** @state local-ui */
  activeUtilityByWindowIdJson?: string;
  /** @state preview */
  hoveredNodeId?: string | null;
  /** @state preview */
  previewSeq?: number;
}

export interface Puzzle2dStringList { values: string[]; }
export interface Puzzle2dNodesDelta { added: Puzzle2dNode[]; removed: string[]; patched: Puzzle2dNodePatchEntry[]; reordered?: string[]; }
export interface Puzzle2dNodePatchEntry { id: string; patch: Puzzle2dNodePatch; }
export interface Puzzle2dNodePatch { replacement?: Puzzle2dNode; }
export interface Puzzle2dNode { id: string; [key: string]: unknown; }
export interface Puzzle2dEdgesDelta { added: Puzzle2dEdge[]; removed: string[]; patched: Puzzle2dEdgePatchEntry[]; reordered?: string[]; }
export interface Puzzle2dEdgePatchEntry { id: string; patch: Puzzle2dEdgePatch; }
export interface Puzzle2dEdgePatch { replacement?: Puzzle2dEdge; }
export interface Puzzle2dEdge { id: string; [key: string]: unknown; }
export interface Puzzle2dArtifact { [key: string]: unknown; }
export interface Puzzle2dCamera { [key: string]: unknown; }
export interface Puzzle2dMeta { [key: string]: unknown; }

