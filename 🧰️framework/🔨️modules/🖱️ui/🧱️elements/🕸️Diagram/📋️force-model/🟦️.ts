import { DIAGRAM_UNIT } from "../📐️layout/📜️wire/🟦️.ts";

// #region 📐️Contract
/**
 * 📊️ Configuration interface for the owned force simulation parameters.
 **/
export interface DiagramForceConfig {
  enabled: boolean;
  chargeStrength?: number;
  linkDistance?: number;
  collideRadius?: number;
  centerStrength?: number;
  updateIntervalMs?: number;
}

/**
 * 📈️ Default owned force configuration values.
 **/
export const defaultDiagramForceConfig: DiagramForceConfig = {
  enabled: false,
  chargeStrength: -DIAGRAM_UNIT * 1.67,
  linkDistance: DIAGRAM_UNIT * 1.25,
  collideRadius: DIAGRAM_UNIT * 0.625,
  centerStrength: 0.15,
  updateIntervalMs: 50,
};

export interface DiagramForceNode {
  id: string;
  x?: number;
  y?: number;
  vx?: number;
  vy?: number;
  fx?: number | null;
  fy?: number | null;
}

export interface DiagramForceLink<NodeType extends DiagramForceNode> {
  id: string;
  source: string | NodeType;
  target: string | NodeType;
}

export interface DiagramForceFrame {
  readonly complete: boolean;
  readonly deadline: number;
  readonly generation: number;
  hasTime(): boolean;
  take(): boolean;
}

export interface DiagramForceWork {
  readonly deadline: number;
  readonly fuel: number;
}

export interface DiagramForceStepResult {
  readonly initialized: boolean;
  readonly remainingFuel: number;
  readonly tickComplete: boolean;
}

export interface DiagramForceSimulation<NodeType extends DiagramForceNode> {
  alphaDecay(): number;
  alphaMin(): number;
  alphaTarget(value: number): this;
  drag(kind: DiagramForceDragKind, node: DiagramForceDragNode, nodes: readonly DiagramForceDragNode[]): this;
  nodes(): NodeType[];
  on(event: "tick", listener: (frame: DiagramForceFrame) => boolean | void): this;
  restart(): this;
  step(work: DiagramForceWork): DiagramForceStepResult;
  stop(): this;
}

export type DiagramForceDragKind = "start" | "move" | "stop";

export interface DiagramForceDragNode {
  id: string;
  position: { x: number; y: number };
  selected?: boolean;
}

export interface DiagramForcePort {
  create<NodeType extends DiagramForceNode, LinkType extends DiagramForceLink<NodeType>>(nodes: NodeType[], links: LinkType[], config: DiagramForceConfig): DiagramForceSimulation<NodeType>;
}
// #endregion 📐️Contract

export interface DiagramForceIdentity {
  hashA: number;
  hashB: number;
  hashC: number;
  hashD: number;
  index: number;
  length: number;
  value: string;
}

export interface DiagramForceNodeRuntime<NodeType extends DiagramForceNode> {
  degree: number;
  fallbackX: number;
  fallbackY: number;
  identity: DiagramForceIdentity;
  node: NodeType;
  sourceIndex: number;
}

export interface ResolvedDiagramForceLink<NodeType extends DiagramForceNode> {
  strength: number;
  bias: number;
  id: string;
  source: DiagramForceNodeRuntime<NodeType>;
  target: DiagramForceNodeRuntime<NodeType>;
}

interface DiagramForceSourceLink {
  id: string;
  index: number;
  source: string;
  target: string;
}

export interface DiagramForceSource<NodeType extends DiagramForceNode> {
  link(index: number): DiagramForceSourceLink;
  readonly linkCount: number;
  linkResolved?(link: DiagramForceSourceLink, sourceIndex: number, targetIndex: number): void;
  node(index: number): NodeType;
  readonly nodeCount: number;
}

export interface DiagramForceMergeCursor<Value> {
  comparisonCursor: number;
  left: number;
  leftCursor: number;
  middle: number;
  right: number;
  rightCursor: number;
  source: Value[];
  target: Value[];
  width: number;
}

export interface DiagramForceHashCursor<NodeType extends DiagramForceNode> {
  cursor: number;
  hashA: number;
  hashB: number;
  hashC: number;
  hashD: number;
  node: NodeType;
  sourceIndex: number;
  value: string;
}

export interface DiagramForceLookupCursor<NodeType extends DiagramForceNode> {
  comparisonCursor: number;
  done: boolean;
  high: number;
  low: number;
  middle: number;
  result?: DiagramForceNodeRuntime<NodeType>;
  value: string;
}

export interface DiagramForcePendingLink<NodeType extends DiagramForceNode> extends DiagramForceSourceLink {
  sourceLookup: DiagramForceLookupCursor<NodeType>;
  sourceRuntime?: DiagramForceNodeRuntime<NodeType>;
  targetLookup?: DiagramForceLookupCursor<NodeType>;
}

export interface DiagramForceWorkCursor {
  readonly deadline: number;
  remaining: number;
}

export interface DiagramForceDragBatch<NodeType extends DiagramForceNode> {
  current?: DiagramForceDragNode;
  cursor: number;
  kind: Exclude<DiagramForceDragKind, "stop">;
  length: number;
  node: DiagramForceDragNode;
  nodes: readonly DiagramForceDragNode[];
  lookup?: DiagramForceLookupCursor<NodeType>;
  selected: boolean;
}

export interface DiagramForcePairCursor {
  left: number;
  right: number;
}

export type DiagramForceInitializationPhase = "nodes" | "sort-nodes" | "index-nodes" | "links" | "sort-links" | "degrees" | "resolve-links" | "ready";
export type DiagramForceTickPhase = "alpha" | "charge" | "links" | "collision" | "nodes";

export const diagramForceBudget = Object.freeze({ maxTicksPerFrame: 4, maxFrameMs: 6, maxUnitsPerFrame: 32_768, publicationReserveMs: 1, maxPairsPerTick: 2_048, maxLinksPerTick: 2_048, maxNodesPerTick: 2_048, maxProjectionNodesPerFrame: 2_048 });
export const diagramForceAlphaMin = 0.001;
export const diagramForceAlphaDecay = 1 - Math.pow(diagramForceAlphaMin, 1 / 300);
export const diagramForceVelocityRetention = 0.6;
