// #region 🧲️Header
// 💻️ framework/ui/elements/📊️Diagram/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import type { Connection, ConnectionLineComponentProps, Edge, EdgeProps, EdgeTypes, MiniMapNodeProps, Node, NodeProps, NodeTypes, OnNodeDrag, OnSelectionChangeParams, ReactFlowInstance } from "@xyflow/react";
import {
  applyNodeChanges,
  Background,
  BackgroundVariant,
  BaseEdge,
  ConnectionMode,
  getBezierPath,
  Handle,
  MiniMap,
  Position,
  ReactFlow,
  ReactFlowProvider,
  SelectionMode,
  useInternalNode,
  useReactFlow,
  useStoreApi,
  ViewportPortal,
} from "@xyflow/react";
import { interactiveJobPort, reactHostPort } from "../🔌️Ports/🟦️component.tsx";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️component.ts";
import { surfaceClass } from "../../🔨️modules/🌈️surface-presentation/🟦️component.ts";
import { loadingBorderClass } from "../../🔨️modules/🌀️status-border-presentation/🟦️component.ts";
import { HostReactFlow, HostReactFlowProvider } from "../🔌️Ports/🟦️component.tsx";
import { createDiagramLayoutPublication, diagramLayoutCredits, DIAGRAM_LAYOUT_CODEC_KIND, DIAGRAM_UNIT, type DiagramLayoutOptions, type DiagramLayoutPublicationResult } from "./🟦️layout.ts";
export { DIAGRAM_UNIT, DIAGRAM_LAYOUT_CODEC_KIND, DIAGRAM_LAYOUT_INGRESS_BYTES, DIAGRAM_LAYOUT_INGRESS_ITEMS, DIAGRAM_LAYOUT_MAX_EDGE_BYTES, DIAGRAM_LAYOUT_MAX_ID_CHARACTERS, DIAGRAM_LAYOUT_MAX_INPUT_ITEMS, DIAGRAM_LAYOUT_MAX_NODE_BYTES, DIAGRAM_LAYOUT_MAX_RESERVED_BYTES, DIAGRAM_LAYOUT_OUTPUT_ITEMS } from "./🟦️layout.ts";
export type { DiagramLayoutDescriptor, DiagramLayoutDirection, DiagramLayoutEdgeWire, DiagramLayoutIngressPage, DiagramLayoutNodeWire, DiagramLayoutOptions, DiagramLayoutPosition, DiagramLayoutPositionPage, DiagramLayoutTerminal } from "./🟦️layout.ts";
// #endregion 🔌️Adapters

// #region 🧫️Diagram
// Interactive node-edge diagram built on ReactFlow and owned layout ports.
// Consumers MUST provide nodes and edges arrays.

export { applyNodeChanges, Background, BackgroundVariant, BaseEdge, getBezierPath, Handle, Position, ReactFlow, ReactFlowProvider, useInternalNode, useReactFlow, useStoreApi, ViewportPortal };
export type { Connection, ConnectionLineComponentProps, Edge, EdgeProps, EdgeTypes, MiniMapNodeProps, Node, NodeProps, NodeTypes, ReactFlowInstance, Connection as RFConnection };


// #region 🧲️ForceSimulation
// #region 📐️Contract
/**
 * Configuration interface for the owned force simulation parameters.
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
 * Default owned force configuration values.
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

interface DiagramForcePort {
  create<NodeType extends DiagramForceNode, LinkType extends DiagramForceLink<NodeType>>(nodes: NodeType[], links: LinkType[], config: DiagramForceConfig): DiagramForceSimulation<NodeType>;
}
// #endregion 📐️Contract

// #region ⚙️Runtime
interface DiagramForceIdentity {
  hashA: number;
  hashB: number;
  hashC: number;
  hashD: number;
  index: number;
  length: number;
  value: string;
}

interface DiagramForceNodeRuntime<NodeType extends DiagramForceNode> {
  degree: number;
  fallbackX: number;
  fallbackY: number;
  identity: DiagramForceIdentity;
  node: NodeType;
  sourceIndex: number;
}

interface ResolvedDiagramForceLink<NodeType extends DiagramForceNode> {
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

interface DiagramForceSource<NodeType extends DiagramForceNode> {
  link(index: number): DiagramForceSourceLink;
  readonly linkCount: number;
  linkResolved?(link: DiagramForceSourceLink, sourceIndex: number, targetIndex: number): void;
  node(index: number): NodeType;
  readonly nodeCount: number;
}

interface DiagramForceMergeCursor<Value> {
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

interface DiagramForceHashCursor<NodeType extends DiagramForceNode> {
  cursor: number;
  hashA: number;
  hashB: number;
  hashC: number;
  hashD: number;
  node: NodeType;
  sourceIndex: number;
  value: string;
}

interface DiagramForceLookupCursor<NodeType extends DiagramForceNode> {
  comparisonCursor: number;
  done: boolean;
  high: number;
  low: number;
  middle: number;
  result?: DiagramForceNodeRuntime<NodeType>;
  value: string;
}

interface DiagramForcePendingLink<NodeType extends DiagramForceNode> extends DiagramForceSourceLink {
  sourceLookup: DiagramForceLookupCursor<NodeType>;
  sourceRuntime?: DiagramForceNodeRuntime<NodeType>;
  targetLookup?: DiagramForceLookupCursor<NodeType>;
}

interface DiagramForceWorkCursor {
  readonly deadline: number;
  remaining: number;
}

interface DiagramForceDragBatch<NodeType extends DiagramForceNode> {
  current?: DiagramForceDragNode;
  cursor: number;
  kind: Exclude<DiagramForceDragKind, "stop">;
  length: number;
  node: DiagramForceDragNode;
  nodes: readonly DiagramForceDragNode[];
  lookup?: DiagramForceLookupCursor<NodeType>;
  selected: boolean;
}

interface DiagramForcePairCursor {
  left: number;
  right: number;
}

type DiagramForceInitializationPhase = "nodes" | "sort-nodes" | "index-nodes" | "links" | "sort-links" | "degrees" | "resolve-links" | "ready";
type DiagramForceTickPhase = "alpha" | "charge" | "links" | "collision" | "nodes";

const diagramForceBudget = Object.freeze({ maxTicksPerFrame: 4, maxFrameMs: 6, maxUnitsPerFrame: 32_768, publicationReserveMs: 1, maxPairsPerTick: 2_048, maxLinksPerTick: 2_048, maxNodesPerTick: 2_048, maxProjectionNodesPerFrame: 2_048 });
const diagramForceAlphaMin = 0.001;
const diagramForceAlphaDecay = 1 - Math.pow(diagramForceAlphaMin, 1 / 300);
const diagramForceVelocityRetention = 0.6;

function fallbackForcePosition(identity: DiagramForceIdentity): readonly [number, number] {
  const angle = (identity.hashA / 0x1_0000_0000) * Math.PI * 2;
  const radius = 10 + ((identity.hashB >>> 8) % 17);
  return [Math.cos(angle) * radius, Math.sin(angle) * radius];
}

function forceJiggle(left: DiagramForceIdentity, right: DiagramForceIdentity): readonly [number, number] {
  const [first, second] = left.index <= right.index ? [left, right] : [right, left];
  const hash = Math.imul(first.hashA ^ second.hashC ^ first.index, 16777619) ^ Math.imul(first.hashB ^ second.hashD ^ second.index, 668265263);
  const angle = (hash / 0x1_0000_0000) * Math.PI * 2;
  return [Math.cos(angle) * 1e-6, Math.sin(angle) * 1e-6];
}

function createForceMergeCursor<Value>(source: Value[]): DiagramForceMergeCursor<Value> {
  return { comparisonCursor: 0, left: 0, leftCursor: 0, middle: Math.min(1, source.length), right: Math.min(2, source.length), rightCursor: Math.min(1, source.length), source, target: [], width: 1 };
}

function takeForceFuel(work: DiagramForceWorkCursor): boolean {
  if (work.remaining <= 0) return false;
  work.remaining -= 1;
  return true;
}

function stepForceTextComparison(left: string, right: string, cursor: DiagramForceMergeCursor<any> | DiagramForceLookupCursor<any>, work: DiagramForceWorkCursor): number | undefined {
  if (!takeForceFuel(work)) return undefined;
  const length = Math.min(left.length, right.length);
  if (cursor.comparisonCursor >= length) return left.length - right.length;
  const result = left.charCodeAt(cursor.comparisonCursor) - right.charCodeAt(cursor.comparisonCursor);
  cursor.comparisonCursor += 1;
  return result || (cursor.comparisonCursor >= length ? left.length - right.length : undefined);
}

function stepForceMerge<Value>(cursor: DiagramForceMergeCursor<Value>, work: DiagramForceWorkCursor, text: (value: Value) => string, tie: (left: Value, right: Value) => number, duplicateFault: boolean): boolean {
  if (cursor.source.length < 2 || cursor.width >= cursor.source.length) return true;
  if (cursor.left >= cursor.source.length) {
    cursor.source = cursor.target;
    cursor.target = [];
    cursor.width *= 2;
    cursor.comparisonCursor = 0;
    cursor.left = 0;
    cursor.leftCursor = 0;
    cursor.middle = Math.min(cursor.width, cursor.source.length);
    cursor.rightCursor = cursor.middle;
    cursor.right = Math.min(cursor.width * 2, cursor.source.length);
    return cursor.width >= cursor.source.length;
  }
  if (cursor.leftCursor >= cursor.middle && cursor.rightCursor >= cursor.right) {
    cursor.left += cursor.width * 2;
    cursor.leftCursor = cursor.left;
    cursor.middle = Math.min(cursor.left + cursor.width, cursor.source.length);
    cursor.rightCursor = cursor.middle;
    cursor.right = Math.min(cursor.left + cursor.width * 2, cursor.source.length);
    cursor.comparisonCursor = 0;
    return false;
  }
  if (!takeForceFuel(work)) return false;
  if (cursor.rightCursor >= cursor.right || cursor.leftCursor >= cursor.middle) {
    cursor.target.push(cursor.source[cursor.rightCursor >= cursor.right ? cursor.leftCursor++ : cursor.rightCursor++]!);
    cursor.comparisonCursor = 0;
    return false;
  }
  work.remaining += 1;
  const left = cursor.source[cursor.leftCursor]!;
  const right = cursor.source[cursor.rightCursor]!;
  const textResult = stepForceTextComparison(text(left), text(right), cursor, work);
  if (textResult === undefined) return false;
  if (duplicateFault && textResult === 0) throw new Error("Duplicate Diagram force node id");
  const result = textResult || tie(left, right);
  const takeLeft = result <= 0;
  cursor.target.push(cursor.source[takeLeft ? cursor.leftCursor++ : cursor.rightCursor++]!);
  cursor.comparisonCursor = 0;
  return false;
}

function finiteForceValue(value: number | null | undefined, fallback: number): number {
  return typeof value === "number" && Number.isFinite(value) ? value : fallback;
}

/** 🧲️ Deterministic force subset with finite per-frame and per-tick work. */
class OwnedDiagramForceSimulation<NodeType extends DiagramForceNode> implements DiagramForceSimulation<NodeType> {
  private orderedNodes: DiagramForceNodeRuntime<NodeType>[] = [];
  private resolvedLinks: ResolvedDiagramForceLink<NodeType>[] = [];
  private readonly chargeStrength: number;
  private readonly linkDistance: number;
  private readonly collideRadius: number;
  private readonly centerStrength: number;
  private readonly updateIntervalMs: number;
  private alpha = 1;
  private targetAlpha = 0;
  private listener?: (frame: DiagramForceFrame) => boolean | void;
  private running = false;
  private frameHandle?: number;
  private frameKind?: "animation" | "timeout";
  private lastNotification = Number.NEGATIVE_INFINITY;
  private notificationComplete = false;
  private notificationGeneration = 0;
  private notificationPending = false;
  private generation = 0;
  private chargeCursor: DiagramForcePairCursor = { left: 0, right: 1 };
  private collideCursor: DiagramForcePairCursor = { left: 0, right: 1 };
  private linkCursor = 0;
  private nodeCursor = 0;
  private tickPhase: DiagramForceTickPhase = "alpha";
  private tickPhaseProgress = 0;
  private initializationPhase: DiagramForceInitializationPhase = "nodes";
  private initializationCursor = 0;
  private initializationGeneration = 0;
  private pendingNode?: DiagramForceHashCursor<NodeType>;
  private pendingLink?: DiagramForcePendingLink<NodeType>;
  private nodeSort?: DiagramForceMergeCursor<DiagramForceNodeRuntime<NodeType>>;
  private linkSort?: DiagramForceMergeCursor<ResolvedDiagramForceLink<NodeType>>;
  private pendingDrag?: DiagramForceDragBatch<NodeType>;
  private releasePinned = false;
  private releasePinnedIterator?: IterableIterator<DiagramForceNodeRuntime<NodeType>>;
  private readonly pinnedNodes = new Set<DiagramForceNodeRuntime<NodeType>>();

  constructor(
    private readonly forceNodes: NodeType[],
    private readonly source: DiagramForceSource<NodeType>,
    private readonly ownsForceNodes: boolean,
    config: DiagramForceConfig,
  ) {
    this.chargeStrength = finiteForceValue(config.chargeStrength, -100);
    this.linkDistance = Math.max(0, finiteForceValue(config.linkDistance, 100));
    this.collideRadius = Math.max(0, finiteForceValue(config.collideRadius, 50));
    this.centerStrength = Math.max(0, finiteForceValue(config.centerStrength, 0.1));
    this.updateIntervalMs = Math.max(0, finiteForceValue(config.updateIntervalMs, 50));
  }

  alphaDecay(): number {
    return diagramForceAlphaDecay;
  }

  alphaMin(): number {
    return diagramForceAlphaMin;
  }

  alphaTarget(value: number): this {
    this.targetAlpha = finiteForceValue(value, 0);
    return this;
  }

  drag(kind: DiagramForceDragKind, node: DiagramForceDragNode, nodes: readonly DiagramForceDragNode[]): this {
    if (kind === "start" || kind === "stop") this.releasePinned = true;
    this.pendingDrag = kind === "stop" ? undefined : { cursor: 0, kind, length: node.selected && nodes.length > 0 ? nodes.length : 1, node, nodes, selected: Boolean(node.selected && nodes.length > 0) };
    this.generation += 1;
    if (this.initializationPhase !== "ready") this.initializationGeneration = this.generation;
    this.lastNotification = Number.NEGATIVE_INFINITY;
    this.notificationPending = false;
    if (this.running) {
      this.cancelFrame();
      if (!this.scheduleFrame()) this.running = false;
    }
    return this;
  }

  nodes(): NodeType[] {
    return this.forceNodes;
  }

  on(_event: "tick", listener: (frame: DiagramForceFrame) => boolean | void): this {
    this.listener = listener;
    return this;
  }

  restart(): this {
    this.generation += 1;
    if (this.initializationPhase !== "ready") this.initializationGeneration = this.generation;
    this.lastNotification = Number.NEGATIVE_INFINITY;
    this.notificationPending = false;
    if (this.running) this.cancelFrame();
    this.running = true;
    if (!this.scheduleFrame()) this.running = false;
    return this;
  }

  stop(): this {
    this.running = false;
    this.generation += 1;
    if (this.initializationPhase !== "ready") this.initializationGeneration = this.generation;
    this.notificationPending = false;
    this.cancelFrame();
    return this;
  }

  step(work: DiagramForceWork): DiagramForceStepResult {
    const cursor = { deadline: finiteForceValue(work.deadline, this.now()), remaining: Math.max(0, Math.floor(finiteForceValue(work.fuel, 0))) };
    const initialized = this.stepInitialization(cursor);
    const dragged = initialized && this.stepDrag(cursor);
    const tickComplete = dragged && this.stepTick(cursor);
    return { initialized, remainingFuel: cursor.remaining, tickComplete };
  }

  private recover(runtime: DiagramForceNodeRuntime<NodeType>): void {
    const node = runtime.node;
    node.x = finiteForceValue(node.x, runtime.fallbackX);
    node.y = finiteForceValue(node.y, runtime.fallbackY);
    node.vx = finiteForceValue(node.vx, 0);
    node.vy = finiteForceValue(node.vy, 0);
    if (node.fx !== undefined && node.fx !== null && !Number.isFinite(node.fx)) node.fx = null;
    if (node.fy !== undefined && node.fy !== null && !Number.isFinite(node.fy)) node.fy = null;
  }

  private advancePair(cursor: DiagramForcePairCursor): void {
    cursor.right += 1;
    if (cursor.right < this.orderedNodes.length) return;
    cursor.left += 1;
    if (cursor.left >= this.orderedNodes.length - 1) cursor.left = 0;
    cursor.right = cursor.left + 1;
  }

  private applyChargePair(): void {
    const left = this.orderedNodes[this.chargeCursor.left]!;
    const right = this.orderedNodes[this.chargeCursor.right]!;
    this.recover(left);
    this.recover(right);
    let deltaX = right.node.x! - left.node.x!;
    let deltaY = right.node.y! - left.node.y!;
    let distanceSquared = deltaX * deltaX + deltaY * deltaY;
    if (distanceSquared < 1e-12) {
      [deltaX, deltaY] = forceJiggle(left.identity, right.identity);
      distanceSquared = deltaX * deltaX + deltaY * deltaY;
    }
    const scale = (this.chargeStrength * this.alpha) / distanceSquared;
    left.node.vx! += deltaX * scale;
    left.node.vy! += deltaY * scale;
    right.node.vx! -= deltaX * scale;
    right.node.vy! -= deltaY * scale;
    this.advancePair(this.chargeCursor);
  }

  private applyLink(): void {
    const link = this.resolvedLinks[this.linkCursor]!;
    this.linkCursor = (this.linkCursor + 1) % this.resolvedLinks.length;
    this.recover(link.source);
    this.recover(link.target);
    let deltaX = link.target.node.x! + link.target.node.vx! - link.source.node.x! - link.source.node.vx!;
    let deltaY = link.target.node.y! + link.target.node.vy! - link.source.node.y! - link.source.node.vy!;
    let distance = Math.hypot(deltaX, deltaY);
    if (distance < 1e-6) {
      [deltaX, deltaY] = forceJiggle(link.source.identity, link.target.identity);
      distance = Math.hypot(deltaX, deltaY);
    }
    const scale = ((distance - this.linkDistance) / distance) * this.alpha * link.strength;
    deltaX *= scale;
    deltaY *= scale;
    link.target.node.vx! -= deltaX * link.bias;
    link.target.node.vy! -= deltaY * link.bias;
    link.source.node.vx! += deltaX * (1 - link.bias);
    link.source.node.vy! += deltaY * (1 - link.bias);
  }

  private applyCollisionPair(): void {
    const diameter = this.collideRadius * 2;
    const left = this.orderedNodes[this.collideCursor.left]!;
    const right = this.orderedNodes[this.collideCursor.right]!;
    this.recover(left);
    this.recover(right);
    let deltaX = right.node.x! + right.node.vx! - left.node.x! - left.node.vx!;
    let deltaY = right.node.y! + right.node.vy! - left.node.y! - left.node.vy!;
    let distance = Math.hypot(deltaX, deltaY);
    if (distance < 1e-6) {
      [deltaX, deltaY] = forceJiggle(left.identity, right.identity);
      distance = Math.hypot(deltaX, deltaY);
    }
    if (distance < diameter) {
      const scale = ((diameter - distance) / distance) * 0.5;
      left.node.vx! -= deltaX * scale;
      left.node.vy! -= deltaY * scale;
      right.node.vx! += deltaX * scale;
      right.node.vy! += deltaY * scale;
    }
    this.advancePair(this.collideCursor);
  }

  private applyNode(): void {
    const runtime = this.orderedNodes[this.nodeCursor]!;
    const node = runtime.node;
    this.nodeCursor = (this.nodeCursor + 1) % this.orderedNodes.length;
    this.recover(runtime);
    node.vx! += -node.x! * this.centerStrength * this.alpha;
    node.vy! += -node.y! * this.centerStrength * this.alpha;
    if (node.fx === undefined || node.fx === null) node.x! += node.vx! *= diagramForceVelocityRetention;
    else {
      node.x = node.fx;
      node.vx = 0;
    }
    if (node.fy === undefined || node.fy === null) node.y! += node.vy! *= diagramForceVelocityRetention;
    else {
      node.y = node.fy;
      node.vy = 0;
    }
  }

  private pairBudget(): number {
    return Math.min((this.orderedNodes.length * (this.orderedNodes.length - 1)) / 2, diagramForceBudget.maxPairsPerTick);
  }

  private hasWork(work: DiagramForceWorkCursor): boolean {
    return work.remaining > 0 && this.now() < work.deadline;
  }

  private createLookup(value: string): DiagramForceLookupCursor<NodeType> {
    return { comparisonCursor: 0, done: false, high: this.orderedNodes.length - 1, low: 0, middle: -1, value };
  }

  private stepLookup(lookup: DiagramForceLookupCursor<NodeType>, work: DiagramForceWorkCursor): boolean {
    while (this.hasWork(work) && !lookup.done) {
      if (lookup.low > lookup.high) {
        lookup.done = true;
        return true;
      }
      if (lookup.middle < 0) lookup.middle = Math.floor((lookup.low + lookup.high) / 2);
      const candidate = this.orderedNodes[lookup.middle]!;
      const comparison = stepForceTextComparison(lookup.value, candidate.identity.value, lookup, work);
      if (comparison === undefined) continue;
      if (comparison === 0) {
        lookup.done = true;
        lookup.result = candidate;
        return true;
      }
      if (comparison < 0) lookup.high = lookup.middle - 1;
      else lookup.low = lookup.middle + 1;
      lookup.comparisonCursor = 0;
      lookup.middle = -1;
    }
    return lookup.done;
  }

  private stepNodeInitialization(work: DiagramForceWorkCursor): void {
    if (!this.pendingNode) {
      if (this.initializationCursor >= this.source.nodeCount) return;
      if (!takeForceFuel(work)) return;
      const sourceIndex = this.initializationCursor++;
      const node = this.source.node(sourceIndex);
      const value = node.id;
      this.pendingNode = { cursor: 0, hashA: (2166136261 ^ value.length) >>> 0, hashB: (2246822507 ^ value.length) >>> 0, hashC: (3266489909 ^ value.length) >>> 0, hashD: (668265263 ^ value.length) >>> 0, node, sourceIndex, value };
    }
    const pending = this.pendingNode;
    if (pending.cursor < pending.value.length) {
      if (!takeForceFuel(work)) return;
      const code = pending.value.charCodeAt(pending.cursor++);
      pending.hashA = Math.imul(pending.hashA ^ code, 16777619) >>> 0;
      pending.hashB = Math.imul(pending.hashB ^ code, 3266489917) >>> 0;
      pending.hashC = Math.imul(pending.hashC ^ code, 668265263) >>> 0;
      pending.hashD = Math.imul(pending.hashD ^ code, 374761393) >>> 0;
      if (pending.cursor < pending.value.length) return;
    }
    const identity = { hashA: pending.hashA, hashB: pending.hashB, hashC: pending.hashC, hashD: pending.hashD, index: -1, length: pending.value.length, value: pending.value };
    const [fallbackX, fallbackY] = fallbackForcePosition(identity);
    const runtime = { degree: 0, fallbackX, fallbackY, identity, node: pending.node, sourceIndex: pending.sourceIndex };
    this.recover(runtime);
    this.orderedNodes.push(runtime);
    if (this.ownsForceNodes) this.forceNodes.push(pending.node);
    this.pendingNode = undefined;
  }

  private stepInitialization(work: DiagramForceWorkCursor): boolean {
    while (this.hasWork(work)) {
      if (this.initializationPhase === "nodes") {
        if (this.initializationCursor < this.source.nodeCount || this.pendingNode) this.stepNodeInitialization(work);
        else {
          this.nodeSort = createForceMergeCursor(this.orderedNodes);
          this.initializationPhase = "sort-nodes";
        }
      } else if (this.initializationPhase === "sort-nodes") {
        if (
          stepForceMerge(
            this.nodeSort!,
            work,
            (runtime) => runtime.identity.value,
            () => 0,
            true,
          )
        ) {
          this.orderedNodes = this.nodeSort!.source;
          this.initializationCursor = 0;
          this.initializationPhase = "index-nodes";
        }
      } else if (this.initializationPhase === "index-nodes") {
        if (this.initializationCursor < this.orderedNodes.length) {
          if (!takeForceFuel(work)) break;
          this.orderedNodes[this.initializationCursor]!.identity.index = this.initializationCursor++;
        } else {
          this.initializationCursor = 0;
          this.initializationPhase = "links";
        }
      } else if (this.initializationPhase === "links") {
        if (!this.pendingLink && this.initializationCursor < this.source.linkCount) {
          if (!takeForceFuel(work)) break;
          const link = this.source.link(this.initializationCursor++);
          this.pendingLink = { ...link, sourceLookup: this.createLookup(link.source) };
        }
        const pending = this.pendingLink;
        if (pending) {
          if (!pending.sourceLookup.done && !this.stepLookup(pending.sourceLookup, work)) break;
          pending.sourceRuntime = pending.sourceLookup.result;
          if (!pending.sourceRuntime) {
            this.pendingLink = undefined;
            continue;
          }
          pending.targetLookup ??= this.createLookup(pending.target);
          if (!pending.targetLookup.done && !this.stepLookup(pending.targetLookup, work)) break;
          if (pending.targetLookup.result) {
            this.resolvedLinks.push({ bias: 0, id: pending.id, source: pending.sourceRuntime, strength: 0, target: pending.targetLookup.result });
            this.source.linkResolved?.(pending, pending.sourceRuntime.sourceIndex, pending.targetLookup.result.sourceIndex);
          }
          this.pendingLink = undefined;
        } else if (this.initializationCursor >= this.source.linkCount) {
          this.linkSort = createForceMergeCursor(this.resolvedLinks);
          this.initializationPhase = "sort-links";
        }
      } else if (this.initializationPhase === "sort-links") {
        if (
          stepForceMerge(
            this.linkSort!,
            work,
            (link) => link.id,
            (left, right) => left.source.identity.index - right.source.identity.index || left.target.identity.index - right.target.identity.index,
            false,
          )
        ) {
          this.resolvedLinks = this.linkSort!.source;
          this.initializationCursor = 0;
          this.initializationPhase = "degrees";
        }
      } else if (this.initializationPhase === "degrees") {
        if (this.initializationCursor < this.resolvedLinks.length) {
          if (!takeForceFuel(work)) break;
          const link = this.resolvedLinks[this.initializationCursor++]!;
          link.source.degree += 1;
          link.target.degree += 1;
        } else {
          this.initializationCursor = 0;
          this.initializationPhase = "resolve-links";
        }
      } else if (this.initializationPhase === "resolve-links") {
        if (this.initializationCursor < this.resolvedLinks.length) {
          if (!takeForceFuel(work)) break;
          const link = this.resolvedLinks[this.initializationCursor++]!;
          const sourceDegree = Math.max(1, link.source.degree);
          const targetDegree = Math.max(1, link.target.degree);
          link.strength = 1 / Math.min(sourceDegree, targetDegree);
          link.bias = sourceDegree / (sourceDegree + targetDegree);
        } else this.initializationPhase = "ready";
      } else return true;
    }
    return this.initializationPhase === "ready";
  }

  private stepDrag(work: DiagramForceWorkCursor): boolean {
    while (this.hasWork(work)) {
      if (this.releasePinned) {
        this.releasePinnedIterator ??= this.pinnedNodes.values();
        if (!takeForceFuel(work)) break;
        const next = this.releasePinnedIterator.next();
        if (!next.done) {
          next.value.node.fx = null;
          next.value.node.fy = null;
          this.pinnedNodes.delete(next.value);
          continue;
        }
        this.releasePinned = false;
        this.releasePinnedIterator = undefined;
      }
      const batch = this.pendingDrag;
      if (!batch) return true;
      if (!batch.current) {
        if (batch.cursor >= batch.length) {
          this.pendingDrag = undefined;
          return true;
        }
        if (!takeForceFuel(work)) break;
        batch.current = batch.selected ? batch.nodes[batch.cursor] : batch.node;
        batch.cursor += 1;
        if (!batch.current) continue;
        batch.lookup = this.createLookup(batch.current.id);
      }
      if (!this.stepLookup(batch.lookup!, work)) break;
      const runtime = batch.lookup!.result;
      if (runtime) {
        runtime.node.x = batch.current.position.x;
        runtime.node.y = batch.current.position.y;
        runtime.node.fx = batch.current.position.x;
        runtime.node.fy = batch.current.position.y;
        this.pinnedNodes.add(runtime);
      }
      batch.current = undefined;
      batch.lookup = undefined;
    }
    return !this.releasePinned && !this.pendingDrag;
  }

  private stepTick(work: DiagramForceWorkCursor): boolean {
    while (this.hasWork(work)) {
      if (this.tickPhase === "alpha") {
        if (!takeForceFuel(work)) break;
        this.alpha += (this.targetAlpha - this.alpha) * diagramForceAlphaDecay;
        this.tickPhase = "charge";
        this.tickPhaseProgress = 0;
      } else if (this.tickPhase === "charge") {
        const budget = this.chargeStrength === 0 ? 0 : this.pairBudget();
        if (this.tickPhaseProgress < budget) {
          if (!takeForceFuel(work)) break;
          this.applyChargePair();
          this.tickPhaseProgress += 1;
        } else {
          this.tickPhase = "links";
          this.tickPhaseProgress = 0;
        }
      } else if (this.tickPhase === "links") {
        const budget = Math.min(this.resolvedLinks.length, diagramForceBudget.maxLinksPerTick);
        if (this.tickPhaseProgress < budget) {
          if (!takeForceFuel(work)) break;
          this.applyLink();
          this.tickPhaseProgress += 1;
        } else {
          this.tickPhase = "collision";
          this.tickPhaseProgress = 0;
        }
      } else if (this.tickPhase === "collision") {
        const budget = this.collideRadius === 0 ? 0 : this.pairBudget();
        if (this.tickPhaseProgress < budget) {
          if (!takeForceFuel(work)) break;
          this.applyCollisionPair();
          this.tickPhaseProgress += 1;
        } else {
          this.tickPhase = "nodes";
          this.tickPhaseProgress = 0;
        }
      } else {
        const budget = Math.min(this.orderedNodes.length, diagramForceBudget.maxNodesPerTick);
        if (this.tickPhaseProgress < budget) {
          if (!takeForceFuel(work)) break;
          this.applyNode();
          this.tickPhaseProgress += 1;
        } else {
          this.tickPhase = "alpha";
          this.tickPhaseProgress = 0;
          return true;
        }
      }
    }
    return false;
  }

  private now(): number {
    return typeof globalThis.performance?.now === "function" ? globalThis.performance.now() : Date.now();
  }

  private scheduleFrame(): boolean {
    if (typeof window === "undefined") return false;
    if (this.frameHandle !== undefined) return true;
    const generation = this.generation;
    if (typeof window.requestAnimationFrame === "function") {
      this.frameKind = "animation";
      this.frameHandle = window.requestAnimationFrame((time) => this.advanceFrame(time, generation));
    } else {
      this.frameKind = "timeout";
      this.frameHandle = window.setTimeout(() => this.advanceFrame(this.now(), generation), 16);
    }
    return true;
  }

  private cancelFrame(): void {
    if (this.frameHandle !== undefined && typeof window !== "undefined") {
      if (this.frameKind === "animation" && typeof window.cancelAnimationFrame === "function") window.cancelAnimationFrame(this.frameHandle);
      else window.clearTimeout(this.frameHandle);
    }
    this.frameHandle = undefined;
    this.frameKind = undefined;
  }

  private complete(): boolean {
    return this.initializationPhase === "ready" && !this.releasePinned && !this.pendingDrag && this.tickPhase === "alpha" && this.alpha < diagramForceAlphaMin && this.targetAlpha === 0;
  }

  private notify(work: DiagramForceWorkCursor, time: number): boolean {
    if (!this.notificationPending || !this.hasWork(work)) return false;
    const listener = this.listener;
    const finished =
      !listener ||
      listener({
        complete: this.notificationComplete,
        deadline: work.deadline,
        generation: this.notificationGeneration,
        hasTime: () => this.hasWork(work),
        take: () => this.hasWork(work) && takeForceFuel(work),
      }) !== false;
    if (!finished) return false;
    this.notificationPending = false;
    this.lastNotification = time;
    return true;
  }

  private advanceFrame(time: number, generation: number): void {
    if (!this.running || generation !== this.generation || (this.initializationPhase !== "ready" && this.initializationGeneration !== generation)) return;
    this.frameHandle = undefined;
    this.frameKind = undefined;
    const deadline = this.now() + diagramForceBudget.maxFrameMs - diagramForceBudget.publicationReserveMs;
    const work = { deadline, remaining: diagramForceBudget.maxUnitsPerFrame };
    if (!this.stepInitialization(work) || !this.stepDrag(work)) {
      if (!this.scheduleFrame()) this.running = false;
      return;
    }
    const notified = this.notify(work, time);
    if (this.notificationPending) {
      if (!this.scheduleFrame()) this.running = false;
      return;
    }
    let ticks = 0;
    while (ticks < diagramForceBudget.maxTicksPerFrame && !this.complete() && this.hasWork(work)) {
      if (this.stepTick(work)) ticks += 1;
    }
    const complete = this.complete();
    if (!notified && this.listener && (complete || time - this.lastNotification >= this.updateIntervalMs)) {
      this.notificationComplete = complete;
      this.notificationGeneration = this.generation;
      this.notificationPending = true;
      this.notify(work, time);
    }
    if (complete && !this.notificationPending) this.running = false;
    else if (!this.scheduleFrame()) this.running = false;
  }
}

const diagramForcePort: DiagramForcePort = {
  create(nodes, links, config) {
    const source: DiagramForceSource<(typeof nodes)[number]> = {
      link: (index) => {
        const link = links[index]!;
        return { id: link.id, index, source: typeof link.source === "string" ? link.source : link.source.id, target: typeof link.target === "string" ? link.target : link.target.id };
      },
      linkCount: links.length,
      node: (index) => nodes[index]!,
      nodeCount: nodes.length,
    };
    return new OwnedDiagramForceSimulation(nodes, source, false, config).stop();
  },
};

/** @emoji 🧲️ Creates the owned force-simulation handle used by the Diagram interaction loop. */
export function createDiagramForceSimulation<NodeType extends DiagramForceNode, LinkType extends DiagramForceLink<NodeType>>(nodes: NodeType[], links: LinkType[], config: DiagramForceConfig): DiagramForceSimulation<NodeType> {
  return diagramForcePort.create(nodes, links, config);
}
// #endregion ⚙️Runtime
// #endregion 🧲️ForceSimulation

/**
 * ForceNode holds the data fields for a ForceNode record.
 **/
interface ForceNode extends DiagramForceNode {
  id: string;
  data: any;
}

interface DiagramForceViewportPage {
  maximumX: number;
  maximumY: number;
  minimumX: number;
  minimumY: number;
  readonly resolvedEdges: Array<{ edge: Edge; sourceIndex: number; targetIndex: number }>;
  readonly selectedNodes: Node[];
}

const diagramForceHostPage = Object.freeze({ extent: 2_048, maxEdges: 256, maxNodes: 128 });

function queryDiagramElement<ElementType extends Element>(selector: string): ElementType | null {
  return typeof document === "undefined" ? null : document.querySelector<ElementType>(selector);
}

function setDiagramForceViewport(page: DiagramForceViewportPage, viewport?: { x: number; y: number; zoom: number }): void {
  const zoom = Math.max(0.01, finiteForceValue(viewport?.zoom, 1));
  page.minimumX = -finiteForceValue(viewport?.x, 0) / zoom - DIAGRAM_UNIT;
  page.minimumY = -finiteForceValue(viewport?.y, 0) / zoom - DIAGRAM_UNIT;
  page.maximumX = page.minimumX + diagramForceHostPage.extent / zoom;
  page.maximumY = page.minimumY + diagramForceHostPage.extent / zoom;
}

function offsetDiagramForceSelection(nodes: Node[], deltaX: number, deltaY: number): Node[] {
  if (deltaX === 0 && deltaY === 0) return nodes;
  return new Proxy(nodes, {
    get(target, property, receiver) {
      const value = Reflect.get(target, property, receiver);
      if (typeof property !== "string" || !/^(0|[1-9]\d*)$/.test(property) || !value) return value;
      const node = value as Node;
      return { ...node, position: { x: node.position.x + deltaX, y: node.position.y + deltaY } };
    },
  });
}

function createLiveDiagramForceSimulation(
  nodes: readonly Node[],
  edges: readonly Edge[],
  config: DiagramForceConfig,
  viewport?: { x: number; y: number; zoom: number },
): { page: DiagramForceViewportPage; simulation: DiagramForceSimulation<ForceNode> } {
  const forceNodes: ForceNode[] = [];
  const page: DiagramForceViewportPage = { maximumX: 0, maximumY: 0, minimumX: 0, minimumY: 0, resolvedEdges: [], selectedNodes: [] };
  setDiagramForceViewport(page, viewport);
  const source: DiagramForceSource<ForceNode> = {
    link: (index) => {
      const edge = edges[index]!;
      return { id: edge.id, index, source: edge.source, target: edge.target };
    },
    linkCount: edges.length,
    linkResolved: (link, sourceIndex, targetIndex) => {
      page.resolvedEdges.push({ edge: edges[link.index]!, sourceIndex, targetIndex });
    },
    node: (index) => {
      const node = nodes[index]!;
      if (node.selected) page.selectedNodes.push(node);
      return { data: node.data, id: node.id, x: node.position.x, y: node.position.y };
    },
    nodeCount: nodes.length,
  };
  return { page, simulation: new OwnedDiagramForceSimulation(forceNodes, source, true, config).stop() };
}

interface DiagramForceProjection {
  cursor: number;
  edgeCursor: number;
  generation: number;
  hostEdges: Edge[];
  hostNodeIndices: Set<number>;
  hostNodes: Node[];
  nodes: Node[];
  phase: "edges" | "nodes";
}

export type DiagramHandoffKind = "consumer-publication" | "drag-move" | "drag-start" | "drag-stop" | "edge-publication" | "host-publication" | "state-publication";

export interface DiagramHandoffViolation {
  readonly elapsedMs: number;
  readonly fault?: string;
  readonly generation: number;
  readonly kind: DiagramHandoffKind;
}

export interface DiagramHandoffStatus {
  lastValidPublicationGeneration(): number | undefined;
  violations(): readonly DiagramHandoffViolation[];
}

interface DiagramHandoffTask {
  readonly consumer?: object;
  readonly generation: number;
  readonly sequence: number;
  readonly valid: () => boolean;
  readonly run: () => void;
}

const diagramHandoffKinds: readonly DiagramHandoffKind[] = ["host-publication", "state-publication", "consumer-publication", "edge-publication", "drag-start", "drag-move", "drag-stop"];

class DiagramHandoffQueue implements DiagramHandoffStatus {
  private generation = 0;
  private handle?: number;
  private lastValidPublication?: DiagramHandoffTask;
  private sequence = 0;
  private readonly quarantined = new WeakSet<object>();
  private readonly tasks: Partial<Record<DiagramHandoffKind, DiagramHandoffTask>> = {};
  private readonly violationLog: DiagramHandoffViolation[] = [];

  lastValidPublicationGeneration(): number | undefined {
    return this.lastValidPublication?.generation;
  }

  violations(): readonly DiagramHandoffViolation[] {
    return this.violationLog;
  }

  enqueue(kind: DiagramHandoffKind, consumer: object | undefined, valid: () => boolean, run: () => void): number | undefined {
    if (consumer && this.quarantined.has(consumer)) return undefined;
    const generation = ++this.generation;
    this.tasks[kind] = Object.freeze({ consumer, generation, run, sequence: ++this.sequence, valid });
    this.schedule();
    return generation;
  }

  invalidate(...kinds: DiagramHandoffKind[]): void {
    for (const kind of kinds) this.tasks[kind] = undefined;
  }

  stop(): void {
    this.invalidate(...diagramHandoffKinds);
    if (this.handle !== undefined && typeof window !== "undefined") window.clearTimeout(this.handle);
    this.handle = undefined;
  }

  private schedule(): void {
    if (this.handle !== undefined || typeof window === "undefined") return;
    this.handle = window.setTimeout(() => this.consume(), 0);
  }

  private consume(): void {
    this.handle = undefined;
    let kind: DiagramHandoffKind | undefined;
    let task: DiagramHandoffTask | undefined;
    for (const candidate of diagramHandoffKinds) {
      const current = this.tasks[candidate];
      if (current && (!task || current.sequence < task.sequence)) {
        kind = candidate;
        task = current;
      }
    }
    if (!kind || !task) return;
    this.tasks[kind] = undefined;
    if (diagramHandoffKinds.some((candidate) => this.tasks[candidate])) this.schedule();
    if (!task.valid() || (task.consumer && this.quarantined.has(task.consumer))) return;
    let failed = false;
    let fault: string | undefined;
    const started = Date.now();
    try {
      task.run();
    } catch (error) {
      failed = true;
      fault = error instanceof Error ? error.message : typeof error === "string" ? error : "Unknown Diagram handoff fault";
    }
    const elapsedMs = Date.now() - started;
    if (failed || elapsedMs >= 8) {
      if (task.consumer) this.quarantined.add(task.consumer);
      this.violationLog.push({ elapsedMs, fault, generation: task.generation, kind });
      if (this.violationLog.length > 16) this.violationLog.shift();
    } else if (kind === "consumer-publication") this.lastValidPublication = task;
  }
}

/**
 * Props interface for the Diagram component.
 **/
export interface DiagramProps {
  nodeTypes: NodeTypes;
  edgeTypes?: EdgeTypes;
  initialNodes?: Node[];
  initialEdges?: Edge[];
  nodes?: Node[];
  edges?: Edge[];
  onNodesChange?: (nodes: Node[]) => void;
  onEdgesChange?: (edges: Edge[]) => void;
  onNodesChangeReactFlow?: (changes: any[]) => void;
  onEdgesChangeReactFlow?: (changes: any[]) => void;
  onConnect?: (connection: any) => void;
  onNodeClick?: (event: React.MouseEvent, node: Node) => void;
  onNodeDoubleClick?: (event: React.MouseEvent, node: Node) => void;
  onNodeMouseEnter?: (event: React.MouseEvent, node: Node) => void;
  onNodeMouseLeave?: (event: React.MouseEvent, node: Node) => void;
  onNodeDragStart?: OnNodeDrag<Node>;
  onNodeDrag?: OnNodeDrag<Node>;
  onNodeDragStop?: OnNodeDrag<Node>;
  onEdgeClick?: (event: React.MouseEvent, edge: Edge) => void;
  onEdgeMouseEnter?: (event: React.MouseEvent, edge: Edge) => void;
  onEdgeMouseLeave?: (event: React.MouseEvent, edge: Edge) => void;
  onPaneClick?: (event: React.MouseEvent) => void;
  onPaneDoubleClick?: (event: React.MouseEvent) => void;
  onMoveStart?: () => void;
  onMoveEnd?: () => void;
  reactFlowInstanceRef?: React.RefObject<ReactFlowInstance | null>;
  onInit?: (instance: ReactFlowInstance) => void;
  wrapperRef?: React.RefObject<HTMLDivElement> | ((node: HTMLDivElement | null) => void);
  showBackground?: boolean;
  backgroundVariant?: BackgroundVariant;
  showControls?: boolean;
  showMinimap?: boolean;
  panels?: React.ReactNode;
  className?: string;
  fitView?: boolean;
  minZoom?: number;
  maxZoom?: number;
  defaultZoom?: number;
  connectionMode?: "strict" | "loose";
  connectionLineComponent?: any;
  deleteKeyCode?: string | string[];
  panOnDrag?: boolean | number[];
  selectionOnDrag?: boolean;
  zoomOnScroll?: boolean;
  zoomOnPinch?: boolean;
  zoomOnDoubleClick?: boolean;
  elementsSelectable?: boolean;
  nodesFocusable?: boolean;
  edgesFocusable?: boolean;
  nodesDraggable?: boolean;
  nodesConnectable?: boolean;
  edgesReconnectable?: boolean;
  miniMapNodeComponent?: any;
  focusedItemId?: string;
  onFocusComplete?: () => void;
  forceConfig?: Partial<DiagramForceConfig>;
  handoffStatusRef?: React.RefObject<DiagramHandoffStatus | null>;
  selectionMode?: SelectionMode;
  panOnScroll?: boolean;
  proOptions?: { hideAttribution: boolean };
  onSelectionChange?: (selection: OnSelectionChangeParams) => void;
  onSelectionStart?: (event: React.MouseEvent) => void;
  onSelectionEnd?: (event: React.MouseEvent) => void;
  defaultViewport?: { x: number; y: number; zoom: number };
  autoPanOnNodeDrag?: boolean;
  selectNodesOnDrag?: boolean;
}

/**
 * DiagramInner holds the data fields for a DiagramInner record.
 **/
const DiagramInner: React.FC<DiagramProps> = ({
  nodeTypes,
  edgeTypes,
  initialNodes = [],
  initialEdges = [],
  nodes: controlledNodes,
  edges: controlledEdges,
  onNodesChange: onNodesChangeProp,
  onEdgesChange: onEdgesChangeProp,
  onNodesChangeReactFlow,
  onEdgesChangeReactFlow,
  onConnect,
  onNodeClick,
  onNodeDoubleClick,
  onNodeMouseEnter,
  onNodeMouseLeave,
  onNodeDragStart: onNodeDragStartProp,
  onNodeDrag: onNodeDragProp,
  onNodeDragStop: onNodeDragStopProp,
  onEdgeClick,
  onEdgeMouseEnter,
  onEdgeMouseLeave,
  onPaneClick,
  onPaneDoubleClick,
  onMoveStart,
  onMoveEnd,
  reactFlowInstanceRef,
  onInit: onInitProp,
  wrapperRef,
  showMinimap = false,
  panels,
  className = "",
  fitView = true,
  minZoom = 0.1,
  maxZoom = 12,
  connectionMode = "loose",
  connectionLineComponent,
  deleteKeyCode = "Delete",
  panOnDrag = [0],
  selectionOnDrag = false,
  zoomOnScroll = true,
  zoomOnPinch = true,
  zoomOnDoubleClick = false,
  elementsSelectable = false,
  nodesFocusable = false,
  edgesFocusable = false,
  nodesDraggable = true,
  nodesConnectable = true,
  edgesReconnectable = true,
  miniMapNodeComponent,
  focusedItemId,
  onFocusComplete,
  forceConfig: forceConfigProp,
  handoffStatusRef,
  selectionMode = SelectionMode.Partial,
  panOnScroll = false,
  proOptions = { hideAttribution: true },
  onSelectionChange,
  onSelectionStart,
  onSelectionEnd,
  defaultViewport,
  autoPanOnNodeDrag,
  selectNodesOnDrag,
}) => {
  const forceConfig = reactHostPort.useMemo(() => ({ ...defaultDiagramForceConfig, ...forceConfigProp }), [forceConfigProp]);
  const simulationRef = reactHostPort.useRef<DiagramForceSimulation<ForceNode> | null>(null);
  const forcePageRef = reactHostPort.useRef<DiagramForceViewportPage | null>(null);
  const draggingNodeRef = reactHostPort.useRef(false);
  const dragOriginRef = reactHostPort.useRef({ x: 0, y: 0 });
  const dragOffsetRef = reactHostPort.useRef({ x: 0, y: 0 });
  const dragHandoffGenerationRef = reactHostPort.useRef(0);
  const handoffRef = reactHostPort.useRef<DiagramHandoffQueue | null>(null);
  handoffRef.current ??= new DiagramHandoffQueue();
  const handoff = handoffRef.current;
  if (handoffStatusRef) (handoffStatusRef as React.MutableRefObject<DiagramHandoffStatus | null>).current = handoff;
  reactHostPort.useEffect(() => () => handoff.stop(), [handoff]);
  const isControlled = controlledNodes !== undefined && controlledEdges !== undefined;
  const rfStoreApi = useStoreApi();
  reactHostPort.useEffect(() => {
    const original = rfStoreApi.setState;
    const api = rfStoreApi as any;
    api.__suppressTransform = false;
    api.__pendingTransform = null;
    api.__original = original;
    rfStoreApi.setState = ((partial: any, replace: any) => {
      if (typeof partial === "object" && partial !== null && !replace) {
        const state = rfStoreApi.getState();
        const keys = Object.keys(partial);
        if (keys.length > 0 && keys.every((k) => Object.is((state as any)[k], partial[k]))) return;
        if (api.__suppressTransform && keys.length === 1 && keys[0] === "transform") {
          const t = partial.transform;
          const el = queryDiagramElement<HTMLElement>(".react-flow__viewport");
          if (el) el.style.transform = `translate(${t[0]}px, ${t[1]}px) scale(${t[2]})`;
          api.__pendingTransform = t;
          return;
        }
      }
      return original(partial, replace);
    }) as typeof original;
    return () => {
      rfStoreApi.setState = original;
    };
  }, [rfStoreApi]);

  const [internalNodes, setInternalNodes] = reactHostPort.useState<Node[]>(initialNodes);
  const [internalEdges, setInternalEdges] = reactHostPort.useState<Edge[]>(initialEdges);

  const finalNodes = isControlled ? controlledNodes : internalNodes;
  const finalEdges = isControlled ? controlledEdges : internalEdges;
  const [hostNodes, setHostNodes] = reactHostPort.useState<Node[]>([]);
  const [hostEdges, setHostEdges] = reactHostPort.useState<Edge[]>([]);
  const virtualizedHost = forceConfig.enabled && finalNodes.length > diagramForceHostPage.maxNodes;
  const renderedNodes = virtualizedHost ? hostNodes : finalNodes;
  const renderedEdges = virtualizedHost ? hostEdges : finalEdges;

  const onNodesChangeReactFlowRef = reactHostPort.useRef(onNodesChangeReactFlow);
  onNodesChangeReactFlowRef.current = onNodesChangeReactFlow;
  const onNodeDragStartPropRef = reactHostPort.useRef(onNodeDragStartProp);
  onNodeDragStartPropRef.current = onNodeDragStartProp;
  const onNodeDragPropRef = reactHostPort.useRef(onNodeDragProp);
  onNodeDragPropRef.current = onNodeDragProp;
  const onNodeDragStopPropRef = reactHostPort.useRef(onNodeDragStopProp);
  onNodeDragStopPropRef.current = onNodeDragStopProp;
  const onInitPropRef = reactHostPort.useRef(onInitProp);
  onInitPropRef.current = onInitProp;
  const onConnectRef = reactHostPort.useRef(onConnect);
  onConnectRef.current = onConnect;
  const onMoveStartRef = reactHostPort.useRef(onMoveStart);
  onMoveStartRef.current = onMoveStart;
  const onMoveEndRef = reactHostPort.useRef(onMoveEnd);
  onMoveEndRef.current = onMoveEnd;
  const onSelectionChangeRef = reactHostPort.useRef(onSelectionChange);
  onSelectionChangeRef.current = onSelectionChange;
  const handleNodesChange = reactHostPort.useCallback(
    (changes: any[]) => {
      onNodesChangeReactFlowRef.current?.(changes);
      if (!isControlled) {
        setInternalNodes((nds) => applyNodeChanges(changes, nds));
      }
    },
    [isControlled],
  );

  const handleEdgesChange = reactHostPort.useCallback(
    (changes: any[]) => {
      if (!isControlled) {
        setInternalEdges((eds) => {
          const updated = [...eds];
          for (const change of changes) {
            if (change.type === "remove") {
              const idx = updated.findIndex((e) => e.id === change.id);
              if (idx !== -1) updated.splice(idx, 1);
            }
          }
          return updated;
        });
      }
    },
    [isControlled],
  );

  const handleInit = reactHostPort.useCallback(
    (instance: ReactFlowInstance) => {
      if (reactFlowInstanceRef) {
        (reactFlowInstanceRef as any).current = instance;
      }
      onInitPropRef.current?.(instance);
    },
    [reactFlowInstanceRef],
  );

  const handleNodeDragStart = reactHostPort.useCallback(
    (event: MouseEvent | TouchEvent, node: Node, nodes: Node[]) => {
      draggingNodeRef.current = true;
      dragOriginRef.current = node.position;
      dragOffsetRef.current = { x: 0, y: 0 };
      const generation = ++dragHandoffGenerationRef.current;
      handoff.invalidate("drag-start", "drag-move", "drag-stop", "consumer-publication", "host-publication", "state-publication");
      const semanticNodes = virtualizedHost && node.selected && forcePageRef.current?.selectedNodes.length ? forcePageRef.current.selectedNodes : nodes;
      if (forceConfig.enabled && simulationRef.current) {
        simulationRef.current.drag("start", node, semanticNodes).alphaTarget(0.3).restart();
      }
      const consumer = onNodeDragStartPropRef.current;
      if (consumer)
        handoff.enqueue(
          "drag-start",
          consumer,
          () => dragHandoffGenerationRef.current === generation,
          () => consumer(event, node, semanticNodes),
        );
    },
    [forceConfig.enabled, handoff, virtualizedHost],
  );

  const handleNodeDrag = reactHostPort.useCallback(
    (event: MouseEvent | TouchEvent, node: Node, nodes: Node[]) => {
      if (!draggingNodeRef.current) return;
      const generation = dragHandoffGenerationRef.current;
      const delta = { x: node.position.x - dragOriginRef.current.x, y: node.position.y - dragOriginRef.current.y };
      dragOffsetRef.current = delta;
      const fullSelection = virtualizedHost && node.selected && forcePageRef.current?.selectedNodes.length ? forcePageRef.current.selectedNodes : undefined;
      const semanticNodes = fullSelection ? offsetDiagramForceSelection(fullSelection, delta.x, delta.y) : nodes;
      if (forceConfig.enabled && simulationRef.current) simulationRef.current.drag("move", node, semanticNodes);
      const consumer = onNodeDragPropRef.current;
      if (consumer)
        handoff.enqueue(
          "drag-move",
          consumer,
          () => dragHandoffGenerationRef.current === generation,
          () => consumer(event, node, semanticNodes),
        );
    },
    [forceConfig.enabled, handoff, virtualizedHost],
  );

  const handleNodeDragStop = reactHostPort.useCallback(
    (event: MouseEvent | TouchEvent, node: Node, nodes: Node[]) => {
      if (forceConfig.enabled && simulationRef.current) {
        simulationRef.current.drag("stop", node, nodes).alphaTarget(0);
      }
      draggingNodeRef.current = false;
      const generation = dragHandoffGenerationRef.current;
      const fullSelection = virtualizedHost && node.selected && forcePageRef.current?.selectedNodes.length ? forcePageRef.current.selectedNodes : undefined;
      const semanticNodes = fullSelection ? offsetDiagramForceSelection(fullSelection, dragOffsetRef.current.x, dragOffsetRef.current.y) : nodes;
      const consumer = onNodeDragStopPropRef.current;
      if (consumer)
        handoff.enqueue(
          "drag-stop",
          consumer,
          () => dragHandoffGenerationRef.current === generation,
          () => consumer(event, node, semanticNodes),
        );
    },
    [forceConfig.enabled, handoff, virtualizedHost],
  );

  const stableOnConnect = reactHostPort.useCallback((connection: any) => {
    onConnectRef.current?.(connection);
  }, []);
  const stableOnMoveStart = reactHostPort.useCallback(() => {
    onMoveStartRef.current?.();
  }, []);
  const stableOnMoveEnd = reactHostPort.useCallback((_event?: MouseEvent | TouchEvent | null, viewport?: { x: number; y: number; zoom: number }) => {
    if (viewport && forcePageRef.current) setDiagramForceViewport(forcePageRef.current, viewport);
    onMoveEndRef.current?.();
  }, []);
  const stableOnSelectionChange = reactHostPort.useCallback((selection: OnSelectionChangeParams) => {
    onSelectionChangeRef.current?.(selection);
  }, []);

  reactHostPort.useEffect(() => {
    if (!forceConfig.enabled || finalNodes.length === 0) {
      simulationRef.current = null;
      return;
    }

    const { page, simulation } = createLiveDiagramForceSimulation(finalNodes, finalEdges, forceConfig, defaultViewport);
    forcePageRef.current = page;
    const simulationNodes = simulation.nodes();
    let projection: DiagramForceProjection | undefined;
    let active = true;
    let publicationGeneration = -1;

    simulation.on("tick", (frame) => {
      if (!projection || projection.generation !== frame.generation) projection = { cursor: 0, edgeCursor: 0, generation: frame.generation, hostEdges: [], hostNodeIndices: new Set<number>(), hostNodes: [], nodes: [], phase: "nodes" };
      let projected = 0;
      while (projection.phase === "nodes" && projection.cursor < simulationNodes.length && projected < diagramForceBudget.maxProjectionNodesPerFrame && frame.take()) {
        const simulationNode = simulationNodes[projection.cursor]!;
        const original = finalNodes[projection.cursor];
        if (original) {
          const positionedNode = {
            ...original,
            position: {
              x: finiteForceValue(simulationNode.x, original.position.x),
              y: finiteForceValue(simulationNode.y, original.position.y),
            },
          };
          projection.nodes.push(positionedNode);
          if (
            projection.hostNodes.length < diagramForceHostPage.maxNodes &&
            positionedNode.position.x >= page.minimumX &&
            positionedNode.position.x <= page.maximumX &&
            positionedNode.position.y >= page.minimumY &&
            positionedNode.position.y <= page.maximumY
          ) {
            projection.hostNodeIndices.add(projection.cursor);
            projection.hostNodes.push(positionedNode);
          }
        }
        projection.cursor += 1;
        projected += 1;
      }
      if (projection.cursor < simulationNodes.length) return false;
      projection.phase = "edges";
      while (projection.edgeCursor < page.resolvedEdges.length && projected < diagramForceBudget.maxProjectionNodesPerFrame && frame.take()) {
        const resolved = page.resolvedEdges[projection.edgeCursor++]!;
        if (projection.hostEdges.length < diagramForceHostPage.maxEdges && projection.hostNodeIndices.has(resolved.sourceIndex) && projection.hostNodeIndices.has(resolved.targetIndex)) projection.hostEdges.push(resolved.edge);
        projected += 1;
      }
      if (projection.edgeCursor < page.resolvedEdges.length) return false;
      const positionedNodes = projection.nodes;
      publicationGeneration = frame.generation;
      const snapshot = Object.freeze({ generation: frame.generation, hostEdges: projection.hostEdges, hostNodes: projection.hostNodes, nodes: positionedNodes });
      projection = undefined;
      handoff.enqueue(
        "host-publication",
        setHostNodes,
        () => active && publicationGeneration === snapshot.generation && simulationRef.current === simulation,
        () => {
          setHostNodes(snapshot.hostNodes);
          setHostEdges(snapshot.hostEdges);
        },
      );
      const consumer = isControlled ? onNodesChangeProp : setInternalNodes;
      if (consumer)
        handoff.enqueue(
          isControlled ? "consumer-publication" : "state-publication",
          consumer,
          () => active && publicationGeneration === snapshot.generation && simulationRef.current === simulation,
          () => consumer(snapshot.nodes),
        );
      return true;
    });

    simulationRef.current = simulation;
    simulation.restart();

    return () => {
      active = false;
      publicationGeneration += 1;
      handoff.invalidate("consumer-publication", "host-publication", "state-publication");
      simulation.stop();
      if (forcePageRef.current === page) forcePageRef.current = null;
      simulationRef.current = null;
    };
  }, [
    defaultViewport,
    forceConfig.enabled,
    forceConfig.chargeStrength,
    forceConfig.linkDistance,
    forceConfig.collideRadius,
    forceConfig.centerStrength,
    forceConfig.updateIntervalMs,
    finalNodes.length,
    finalEdges.length,
    handoff,
    isControlled,
    onNodesChangeProp,
  ]);

  reactHostPort.useEffect(() => {
    if (focusedItemId && reactFlowInstanceRef?.current) {
      const node = finalNodes.find((n) => n.id === focusedItemId);
      const edge = finalEdges.find((e) => e.id === focusedItemId);

      if (node) {
        reactFlowInstanceRef.current.fitView({
          padding: 0.5,
          duration: 600,
          nodes: [node],
        });
      } else if (edge) {
        const sourceNode = finalNodes.find((n) => n.id === edge.source);
        const targetNode = finalNodes.find((n) => n.id === edge.target);
        const nodesToFit = [sourceNode, targetNode].filter(Boolean) as Node[];
        if (nodesToFit.length > 0) {
          reactFlowInstanceRef.current.fitView({
            padding: 0.5,
            duration: 600,
            nodes: nodesToFit,
          });
        }
      }

      if (onFocusComplete) {
        setTimeout(() => onFocusComplete(), 600);
      }
    }
  }, [focusedItemId, finalNodes, finalEdges, reactFlowInstanceRef, onFocusComplete]);

  reactHostPort.useEffect(() => {
    if (!isControlled) {
      setInternalNodes(initialNodes);
      setInternalEdges(initialEdges);
    }
  }, [initialNodes, initialEdges, isControlled]);

  reactHostPort.useEffect(() => {
    if (isControlled || !onNodesChangeProp) return;
    let active = true;
    const snapshot = internalNodes;
    handoff.enqueue(
      "consumer-publication",
      onNodesChangeProp,
      () => active,
      () => onNodesChangeProp(snapshot),
    );
    return () => {
      active = false;
      handoff.invalidate("consumer-publication");
    };
  }, [handoff, internalNodes, onNodesChangeProp, isControlled]);

  reactHostPort.useEffect(() => {
    if (isControlled || !onEdgesChangeProp) return;
    let active = true;
    const snapshot = internalEdges;
    handoff.enqueue(
      "edge-publication",
      onEdgesChangeProp,
      () => active,
      () => onEdgesChangeProp(snapshot),
    );
    return () => {
      active = false;
      handoff.invalidate("edge-publication");
    };
  }, [handoff, internalEdges, onEdgesChangeProp, isControlled]);

  return (
    <div ref={wrapperRef as any} className={`relative w-full h-full ${className}`}>
      <HostReactFlow
        nodes={renderedNodes}
        edges={renderedEdges}
        onNodesChange={handleNodesChange}
        onEdgesChange={handleEdgesChange}
        onConnect={stableOnConnect}
        onInit={handleInit}
        onNodeClick={onNodeClick}
        onNodeDoubleClick={onNodeDoubleClick}
        onNodeMouseEnter={onNodeMouseEnter}
        onNodeMouseLeave={onNodeMouseLeave}
        onNodeDragStart={handleNodeDragStart}
        onNodeDrag={handleNodeDrag}
        onNodeDragStop={handleNodeDragStop}
        onEdgeClick={onEdgeClick}
        onEdgeMouseEnter={onEdgeMouseEnter}
        onEdgeMouseLeave={onEdgeMouseLeave}
        onPaneClick={onPaneClick}
        onDoubleClick={onPaneDoubleClick}
        onMoveStart={stableOnMoveStart}
        onMoveEnd={stableOnMoveEnd}
        onSelectionChange={stableOnSelectionChange}
        onSelectionStart={onSelectionStart}
        onSelectionEnd={onSelectionEnd}
        nodeTypes={nodeTypes}
        edgeTypes={edgeTypes}
        connectionLineComponent={connectionLineComponent}
        fitView={fitView}
        minZoom={minZoom}
        maxZoom={maxZoom}
        defaultViewport={defaultViewport}
        connectionMode={connectionMode === "loose" ? ConnectionMode.Loose : ConnectionMode.Strict}
        deleteKeyCode={deleteKeyCode}
        panOnDrag={panOnDrag}
        panOnScroll={panOnScroll}
        preventScrolling={true}
        selectionOnDrag={selectionOnDrag}
        selectionMode={selectionMode}
        zoomOnScroll={zoomOnScroll}
        zoomOnPinch={zoomOnPinch}
        zoomOnDoubleClick={zoomOnDoubleClick}
        elementsSelectable={elementsSelectable}
        nodesFocusable={nodesFocusable}
        edgesFocusable={edgesFocusable}
        nodesDraggable={nodesDraggable}
        nodesConnectable={nodesConnectable}
        edgesReconnectable={edgesReconnectable}
        autoPanOnNodeDrag={autoPanOnNodeDrag}
        selectNodesOnDrag={selectNodesOnDrag}
        onlyRenderVisibleElements={true}
        proOptions={proOptions}
        className={surfaceClass}
      >
        {showMinimap && <MiniMap className="border" maskColor="var(--accent)" bgColor="var(--background)" nodeStrokeWidth={3} zoomable pannable nodeComponent={miniMapNodeComponent} />}
        {panels}
      </HostReactFlow>
    </div>
  );
};

/**
 * Diagram holds the data fields for a Diagram record.
 **/
const Diagram: React.FC<DiagramProps> = (props) => {
  return (
    <HostReactFlowProvider>
      <DiagramInner {...props} />
    </HostReactFlowProvider>
  );
};

export { Diagram, SelectionMode };
export type { OnSelectionChangeParams };

let diagramLayoutGeneration = 0;

function nextDiagramLayoutGeneration(): number {
  diagramLayoutGeneration = diagramLayoutGeneration >= Number.MAX_SAFE_INTEGER ? 1 : diagramLayoutGeneration + 1;
  return diagramLayoutGeneration;
}

type DiagramLayoutHookResult = {
  readonly edges: Edge[];
  readonly layoutRejection?: "bytes" | "items";
  readonly layoutStatus: "complete" | "pending" | "rejected" | "source";
  readonly nodes: Node[];
};

type DiagramLayoutPublishedState = {
  readonly authority: DiagramLayoutPublicationResult;
  readonly edges: Edge[];
  readonly generation: number;
  readonly nodes: Node[];
  readonly sourceEdges: Edge[];
  readonly sourceNodes: Node[];
};

type DiagramLayoutOwnershipState = {
  readonly activeGeneration: number;
  readonly candidate?: DiagramLayoutPublishedState;
  readonly committed?: DiagramLayoutPublishedState;
};

function retireDiagramLayoutResult(result: DiagramLayoutPublicationResult): void {
  setTimeout(function closeResultStep() {
    const closed = result.closeStep();
    if (!closed) setTimeout(closeResultStep, 0);
  }, 0);
}

/**
 * 📡️ Submits directed layout only through the process-wide interactive worker port.
 **/
export function useDiagramLayout(initialNodes: Node[], initialEdges: Edge[], layoutOptions?: DiagramLayoutOptions): DiagramLayoutHookResult {
  const portSnapshot = reactHostPort.useSyncExternalStore(interactiveJobPort.subscribe, interactiveJobPort.getSnapshot, interactiveJobPort.getSnapshot);
  const credits = diagramLayoutCredits(initialNodes.length, initialEdges.length);
  const [ownership, setOwnership] = reactHostPort.useState<DiagramLayoutOwnershipState>({ activeGeneration: 0 });
  const activeGenerationRef = reactHostPort.useRef(0);
  const ownedResultsRef = reactHostPort.useRef(new Set<DiagramLayoutPublicationResult>());
  const selected = ownership.candidate ?? ownership.committed;
  const displayed = selected?.sourceNodes === initialNodes && selected.sourceEdges === initialEdges ? selected : undefined;
  reactHostPort.useEffect(() => {
    const generation = nextDiagramLayoutGeneration();
    activeGenerationRef.current = generation;
    let abandonsAuthority = false;
    for (const authority of ownedResultsRef.current) {
      if (authority !== displayed?.authority) {
        abandonsAuthority = true;
        break;
      }
    }
    if (abandonsAuthority) setOwnership({ activeGeneration: generation, committed: displayed });
    if (portSnapshot.status !== "ready" || !credits.admitted) return;
    const publication = createDiagramLayoutPublication(initialNodes, initialEdges, layoutOptions ?? {}, generation);
    let live = true;
    let lease = interactiveJobPort.submit(
      {
        generation,
        inputBytes: credits.inputBytes,
        inputItems: credits.inputItems,
        inputPageItems: 64,
        kind: DIAGRAM_LAYOUT_CODEC_KIND,
        operation: generation,
        outputBytes: credits.outputBytes,
        outputItems: credits.outputItems,
        outputPageItems: 128,
        pageBytes: 16 * 1024,
        payload: publication.descriptor,
      },
      {
        readInputPage: (cursor, maxItems) => publication.readInputPage(cursor, maxItems),
        onOutputPage: (page) => {
          if (!live || !publication.acceptOutputPage(page)) lease?.cancel();
        },
        onTerminal: (terminal) => {
          const validIdentity = terminal.generation === generation && terminal.operation === generation;
          const result = publication.acceptTerminal(
            validIdentity
              ? terminal.status === "fault"
                ? { generation, kind: "terminal", reason: terminal.detail ?? "Diagram layout fault", status: "fault" }
                : { generation, kind: "terminal", status: terminal.status }
              : { generation: -1, kind: "terminal", reason: "Diagram layout terminal identity mismatch", status: "fault" },
          );
          if (!live || !result) {
            if (result) retireDiagramLayoutResult(result);
            return;
          }
          const next = { authority: result, edges: result.edges, generation, nodes: result.nodes, sourceEdges: initialEdges, sourceNodes: initialNodes };
          ownedResultsRef.current.add(result);
          setOwnership((retained) => (activeGenerationRef.current === generation ? { ...retained, candidate: next } : retained));
        },
        closeStep: () => publication.closeStep(),
        terminalIsEmpty: () => publication.terminalIsEmpty(),
      },
    );
    if (!lease) {
      setTimeout(function closeRejectedPublication() {
        if (!publication.closeStep()) setTimeout(closeRejectedPublication, 0);
      }, 0);
    }
    return () => {
      live = false;
      lease?.cancel();
      lease = undefined;
    };
  }, [
    initialNodes,
    initialEdges,
    layoutOptions?.direction,
    layoutOptions?.nodeHeight,
    layoutOptions?.nodeSep,
    layoutOptions?.nodeWidth,
    layoutOptions?.rankSep,
    portSnapshot,
    credits.admitted,
    credits.admitted ? credits.inputBytes : credits.reason,
  ]);
  reactHostPort.useLayoutEffect(() => {
    const candidate = ownership.candidate;
    if (!candidate || displayed?.authority !== candidate.authority) return;
    setOwnership((retained) => (retained.candidate?.authority === candidate.authority ? { activeGeneration: retained.activeGeneration, committed: candidate } : retained));
  }, [displayed?.authority, ownership.candidate]);
  reactHostPort.useEffect(() => {
    const committed = displayed?.authority;
    for (const authority of ownedResultsRef.current) {
      if (authority === committed) continue;
      ownedResultsRef.current.delete(authority);
      retireDiagramLayoutResult(authority);
    }
  }, [displayed?.authority, ownership.activeGeneration]);
  reactHostPort.useEffect(
    () => () => {
      for (const authority of ownedResultsRef.current) retireDiagramLayoutResult(authority);
      ownedResultsRef.current.clear();
    },
    [],
  );
  if (displayed) return { edges: displayed.edges, layoutStatus: "complete", nodes: displayed.nodes };
  if (!credits.admitted) return { edges: initialEdges, layoutRejection: credits.reason, layoutStatus: "rejected", nodes: initialNodes };
  return { edges: initialEdges, layoutStatus: portSnapshot.status === "ready" ? "pending" : "source", nodes: initialNodes };
}

/**
 * DiagramSkeletonProps holds the data fields for a DiagramSkeletonProps record.
 **/
interface DiagramSkeletonProps {
  nodeCount?: number;
  edgeCount?: number;
  className?: string;
}

/**
 * Skeleton loading placeholder for a diagram.
 **/
export const DiagramSkeleton: React.FC<DiagramSkeletonProps> = ({ nodeCount = 5, edgeCount = 4, className = "" }) => {
  const skeletonNodes: Node[] = reactHostPort.useMemo(
    () =>
      Array.from({ length: nodeCount }).map((_, i) => ({
        id: `skeleton-node-${i}`,
        type: "default",
        position: { x: (i % 3) * 150 + 50, y: Math.floor(i / 3) * 150 + 50 },
        data: { label: " " },
        draggable: false,
      })),
    [nodeCount],
  );
  const skeletonEdges: Edge[] = reactHostPort.useMemo(
    () =>
      Array.from({ length: edgeCount }).map((_, i) => ({
        id: `skeleton-edge-${i}`,
        source: `skeleton-node-${i}`,
        target: `skeleton-node-${Math.min(i + 1, nodeCount - 1)}`,
        animated: false,
      })),
    [edgeCount, nodeCount],
  );
  return (
    <div className={cn("relative w-full h-full", loadingBorderClass, className)}>
      <HostReactFlow
        nodes={skeletonNodes}
        edges={skeletonEdges}
        nodeTypes={{}}
        edgeTypes={{}}
        nodesDraggable={false}
        elementsSelectable={false}
        panOnDrag={false}
        zoomOnScroll={false}
        zoomOnPinch={false}
        proOptions={{ hideAttribution: true }}
        className={cn(surfaceClass, "animate-pulse opacity-50")}
      ></HostReactFlow>
    </div>
  );
};

// #endregion 🧫️Diagram
