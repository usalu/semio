// #region 🧲️Header
// 💻️ framework/ui/elements/📊️Diagram/layout.ts
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
// #endregion 🧲️Header

// #region 🔌️Adapters
import type { Edge, Node } from "@xyflow/react";
// #endregion 🔌️Adapters

/**
 * Base pixel unit for diagram node sizing.
 **/
export const DIAGRAM_UNIT = 48;

/**
 * Union type for diagram layout directions (TB/BT/LR/RL).
 **/
export type DiagramLayoutDirection = "TB" | "BT" | "LR" | "RL";

/**
 * Configuration for the owned deterministic directed layout.
 **/
export interface DiagramLayoutOptions {
  direction?: DiagramLayoutDirection;
  nodeWidth?: number;
  nodeHeight?: number;
  rankSep?: number;
  nodeSep?: number;
}

// #region 🧬️Codec
export const DIAGRAM_LAYOUT_CODEC_KIND = "diagram-directed-layout-v1";
export const DIAGRAM_LAYOUT_INGRESS_ITEMS = 64;
export const DIAGRAM_LAYOUT_INGRESS_BYTES = 16 * 1024;
export const DIAGRAM_LAYOUT_OUTPUT_ITEMS = 128;
export const DIAGRAM_LAYOUT_MAX_INPUT_ITEMS = 65_536;
export const DIAGRAM_LAYOUT_MAX_ID_CHARACTERS = 512;
export const DIAGRAM_LAYOUT_MAX_NODE_BYTES = 64 + DIAGRAM_LAYOUT_MAX_ID_CHARACTERS * 4;
export const DIAGRAM_LAYOUT_MAX_EDGE_BYTES = 64 + DIAGRAM_LAYOUT_MAX_ID_CHARACTERS * 4 * 3;
export const DIAGRAM_LAYOUT_MAX_RESERVED_BYTES = 256 * 1024 * 1024;

export interface DiagramLayoutNodeWire {
  readonly height?: number;
  readonly id: string;
  readonly index: number;
  readonly measuredHeight?: number;
  readonly measuredWidth?: number;
  readonly styleHeight?: number;
  readonly styleWidth?: number;
  readonly width?: number;
}

export interface DiagramLayoutEdgeWire {
  readonly id: string;
  readonly index: number;
  readonly source: string;
  readonly target: string;
}

export type DiagramLayoutIngressPage =
  | { readonly bytes: number; readonly complete?: boolean; readonly generation: number; readonly kind: "nodes"; readonly offset: number; readonly values: readonly DiagramLayoutNodeWire[] }
  | { readonly bytes: number; readonly complete?: boolean; readonly generation: number; readonly kind: "edges"; readonly offset: number; readonly values: readonly DiagramLayoutEdgeWire[] }
  | { readonly generation: number; readonly kind: "seal" };

export interface DiagramLayoutPositionPage {
  readonly complete: boolean;
  readonly generation: number;
  readonly kind: "positions";
  readonly sequence: number;
  readonly values: readonly DiagramLayoutPosition[];
}

export type DiagramLayoutTerminal = { readonly generation: number; readonly kind: "terminal"; readonly status: "complete" | "cancelled" } | { readonly generation: number; readonly kind: "terminal"; readonly reason: string; readonly status: "fault" };

export interface DiagramLayoutDescriptor {
  readonly edgeCount: number;
  readonly generation: number;
  readonly kind: typeof DIAGRAM_LAYOUT_CODEC_KIND;
  readonly nodeCount: number;
  readonly options: DiagramLayoutOptions;
}

export interface DiagramLayoutHostPage {
  readonly byteLength: number;
  readonly complete: boolean;
  readonly itemCount: number;
  readonly payload: unknown;
}

export interface DiagramLayoutPublicationResult {
  readonly edges: Edge[];
  readonly nodes: Node[];
  closeStep(): boolean;
}

export function diagramLayoutUtf8Bytes(value: string): number {
  let bytes = 0;
  let characters = 0;
  for (let index = 0; index < value.length; index++) {
    characters += 1;
    if (characters > DIAGRAM_LAYOUT_MAX_ID_CHARACTERS) throw new Error("Diagram layout id exceeds 512 Unicode characters");
    const code = value.charCodeAt(index);
    if (code <= 0x7f) bytes += 1;
    else if (code <= 0x7ff) bytes += 2;
    else if (code >= 0xd800 && code <= 0xdbff && index + 1 < value.length && value.charCodeAt(index + 1) >= 0xdc00 && value.charCodeAt(index + 1) <= 0xdfff) {
      bytes += 4;
      index += 1;
    } else bytes += 3;
  }
  return bytes;
}

export function diagramLayoutNodeWireBytes(value: DiagramLayoutNodeWire): number {
  return 64 + diagramLayoutUtf8Bytes(value.id);
}

export function diagramLayoutEdgeWireBytes(value: DiagramLayoutEdgeWire): number {
  return 64 + diagramLayoutUtf8Bytes(value.id) + diagramLayoutUtf8Bytes(value.source) + diagramLayoutUtf8Bytes(value.target);
}

function diagramLayoutIdentityAdmitted(value: unknown, allowEmpty = false): value is string {
  if (typeof value !== "string" || (!allowEmpty && value.length === 0)) return false;
  try {
    diagramLayoutUtf8Bytes(value);
    return true;
  } catch {
    return false;
  }
}

export type DiagramLayoutCredits =
  | { readonly admitted: true; readonly inputBytes: number; readonly inputItems: number; readonly outputBytes: number; readonly outputItems: number }
  | { readonly admitted: false; readonly reason: "bytes" | "items" };

/** 📏️ Declares exact worst-case credits from counts without reading either source array. */
export function diagramLayoutCredits(nodeCount: number, edgeCount: number): DiagramLayoutCredits {
  if (!Number.isSafeInteger(nodeCount) || !Number.isSafeInteger(edgeCount) || nodeCount < 0 || edgeCount < 0 || nodeCount + edgeCount > DIAGRAM_LAYOUT_MAX_INPUT_ITEMS) return { admitted: false, reason: "items" };
  const inputBytes = nodeCount * DIAGRAM_LAYOUT_MAX_NODE_BYTES + edgeCount * DIAGRAM_LAYOUT_MAX_EDGE_BYTES;
  const outputBytes = nodeCount * 32;
  if (!Number.isSafeInteger(inputBytes) || inputBytes + outputBytes > DIAGRAM_LAYOUT_MAX_RESERVED_BYTES) return { admitted: false, reason: "bytes" };
  return { admitted: true, inputBytes, inputItems: nodeCount + edgeCount, outputBytes, outputItems: nodeCount };
}
// #endregion 🧬️Codec

// #region 🧭️DirectedLayout
export interface DiagramLayoutWork {
  readonly deadline: number;
  readonly fuel: number;
  readonly generation: number;
}

export type DiagramLayoutJobStatus = "running" | "complete" | "cancelled" | "fault";

export interface DiagramLayoutPreview {
  readonly generation: number;
  readonly positions: readonly DiagramLayoutPosition[];
  readonly sequence: number;
}

export interface DiagramLayoutPosition {
  readonly index: number;
  readonly x: number;
  readonly y: number;
}

export interface DiagramLayoutStepResult {
  readonly consumed: number;
  readonly stage: DiagramLayoutStage;
  readonly status: DiagramLayoutJobStatus;
}

type DiagramLayoutStage = "admit-nodes" | "sort-nodes" | "index-nodes" | "admit-edges" | "sort-edges" | "build-graph" | "assign-ranks" | "crossing" | "sort-crossing" | "measure-ranks" | "position-ranks" | "coordinates" | "project" | "project-edges" | "complete";
type DiagramLayoutCloseStage = "previews" | "edges" | "nodes" | "spares" | "indices" | "captures" | "scalars" | "complete";

interface DiagramLayoutNodeRuntime {
  barycenterCount: number;
  barycenterSum: number;
  cross: number;
  depth: number;
  height: number;
  id: string;
  indegree: number;
  order: number;
  outgoingHead: number;
  outgoingTail: number;
  processed: boolean;
  rank: number;
  sourceIndex: number;
  width: number;
  x: number;
  y: number;
}

interface DiagramLayoutEdgeRuntime {
  id: string;
  source: number;
  sourceId: string;
  sourceIndex: number;
  target: number;
  targetId: string;
}

interface DiagramLayoutLookup {
  done: boolean;
  high: number;
  low: number;
  result?: number;
  value: string;
}

interface DiagramLayoutPendingEdge {
  captured: Edge;
  inputIndex: number;
  sourceLookup: DiagramLayoutLookup;
  targetLookup?: DiagramLayoutLookup;
}

export interface DiagramLayoutOwnedSource<Value> {
  readonly length: number;
  get(index: number): Value | undefined;
}

function asDiagramLayoutSource<Value>(values: readonly Value[] | DiagramLayoutOwnedSource<Value>): DiagramLayoutOwnedSource<Value> {
  if ("get" in values && typeof values.get === "function") return values;
  const array = values as readonly Value[];
  return { get: (index) => array[index], length: array.length };
}

interface DiagramLayoutMerge<Value> {
  left: number;
  leftCursor: number;
  middle: number;
  right: number;
  rightCursor: number;
  source: DiagramPagedStore<Value>;
  target: DiagramPagedStore<Value>;
  width: number;
}

const diagramLayoutLimits = Object.freeze({ maxEdges: DIAGRAM_LAYOUT_MAX_INPUT_ITEMS, maxNodes: DIAGRAM_LAYOUT_MAX_INPUT_ITEMS, previewNodes: 128 });
const diagramLayoutFrame = Object.freeze({ fuel: 16_384, milliseconds: 6 });
const diagramLayoutPageSize = 128;

class DiagramPagedStore<Value> {
  private readonly directories = new Array<Array<Array<Value | undefined> | undefined> | undefined>(16);
  private count = 0;
  private pageHighWater = 0;

  constructor(readonly capacity: number) {}

  get length(): number {
    return this.count;
  }

  get(index: number): Value | undefined {
    if (index < 0 || index >= this.count) return undefined;
    const pageIndex = Math.floor(index / diagramLayoutPageSize);
    return this.directories[Math.floor(pageIndex / 32)]?.[pageIndex % 32]?.[index % diagramLayoutPageSize];
  }

  set(index: number, value: Value): void {
    if (index < 0 || index >= this.capacity) throw new Error("Diagram layout page capacity exceeded");
    const pageIndex = Math.floor(index / diagramLayoutPageSize);
    const directoryIndex = Math.floor(pageIndex / 32);
    const directory = this.directories[directoryIndex] ?? (this.directories[directoryIndex] = new Array(32));
    const page = directory[pageIndex % 32] ?? (directory[pageIndex % 32] = new Array(diagramLayoutPageSize));
    this.pageHighWater = Math.max(this.pageHighWater, pageIndex + 1);
    page[index % diagramLayoutPageSize] = value;
    if (index >= this.count) this.count = index + 1;
  }

  push(value: Value): number {
    const index = this.count;
    this.set(index, value);
    return index;
  }

  pop(): Value | undefined {
    if (this.count === 0) return undefined;
    const index = --this.count;
    const pageIndex = Math.floor(index / diagramLayoutPageSize);
    const directoryIndex = Math.floor(pageIndex / 32);
    const directory = this.directories[directoryIndex];
    const page = directory?.[pageIndex % 32];
    const value = page?.[index % diagramLayoutPageSize];
    if (page) page[index % diagramLayoutPageSize] = undefined;
    if (index % diagramLayoutPageSize === 0 && directory) directory[pageIndex % 32] = undefined;
    return value;
  }

  take(index: number): Value | undefined {
    if (index < 0 || index >= this.count) return undefined;
    const pageIndex = Math.floor(index / diagramLayoutPageSize);
    const page = this.directories[Math.floor(pageIndex / 32)]?.[pageIndex % 32];
    const offset = index % diagramLayoutPageSize;
    const value = page?.[offset];
    if (page) page[offset] = undefined;
    return value;
  }

  resetCleared(): void {
    this.count = 0;
  }

  releaseOnePage(): boolean {
    if (this.count > 0) {
      this.pop();
      return false;
    }
    if (this.pageHighWater === 0) return true;
    const pageIndex = --this.pageHighWater;
    const directoryIndex = Math.floor(pageIndex / 32);
    const directory = this.directories[directoryIndex];
    if (directory) {
      directory[pageIndex % 32] = undefined;
      if (pageIndex % 32 === 0) this.directories[directoryIndex] = undefined;
    }
    return this.pageHighWater === 0;
  }

  releasePageStep(): boolean {
    const retained = this.count;
    const limit = Math.max(0, retained - diagramLayoutPageSize);
    while (this.count > limit) this.pop();
    if (retained > 0) return false;
    return this.releaseOnePage();
  }
}

function finiteLayoutValue(value: unknown, fallback: number): number {
  return typeof value === "number" && Number.isFinite(value) ? value : fallback;
}

function optionalFiniteLayoutValue(value: unknown): boolean {
  return value === undefined || (typeof value === "number" && Number.isFinite(value));
}

function resolveLayoutOptions(options: DiagramLayoutOptions): Required<DiagramLayoutOptions> {
  return {
    direction: options.direction ?? "TB",
    nodeHeight: Math.max(1, finiteLayoutValue(options.nodeHeight, DIAGRAM_UNIT)),
    nodeSep: Math.max(0, finiteLayoutValue(options.nodeSep, DIAGRAM_UNIT * 1.04)),
    nodeWidth: Math.max(1, finiteLayoutValue(options.nodeWidth, DIAGRAM_UNIT)),
    rankSep: Math.max(0, finiteLayoutValue(options.rankSep, DIAGRAM_UNIT * 1.67)),
  };
}

function nodeLayoutDimension(node: Node, axis: "height" | "width", fallback: number): number {
  const measured = node.measured?.[axis];
  const direct = node[axis];
  const style = typeof node.style?.[axis] === "number" ? node.style[axis] : undefined;
  return Math.max(1, finiteLayoutValue(measured, finiteLayoutValue(direct, finiteLayoutValue(style, fallback))));
}

function createLayoutMerge<Value>(source: DiagramPagedStore<Value>): DiagramLayoutMerge<Value> {
  return { left: 0, leftCursor: 0, middle: Math.min(1, source.length), right: Math.min(2, source.length), rightCursor: Math.min(1, source.length), source, target: new DiagramPagedStore(source.capacity), width: 1 };
}

function stepLayoutMerge<Value>(merge: DiagramLayoutMerge<Value>, compare: (left: Value, right: Value) => number): boolean {
  if (merge.source.length < 2 || merge.width >= merge.source.length) return true;
  if (merge.left >= merge.source.length) {
    const cleared = merge.source;
    merge.source = merge.target;
    cleared.resetCleared();
    merge.target = cleared;
    merge.width *= 2;
    merge.left = 0;
    merge.leftCursor = 0;
    merge.middle = Math.min(merge.width, merge.source.length);
    merge.rightCursor = merge.middle;
    merge.right = Math.min(merge.width * 2, merge.source.length);
    return merge.width >= merge.source.length;
  }
  if (merge.leftCursor >= merge.middle && merge.rightCursor >= merge.right) {
    merge.left += merge.width * 2;
    merge.leftCursor = merge.left;
    merge.middle = Math.min(merge.left + merge.width, merge.source.length);
    merge.rightCursor = merge.middle;
    merge.right = Math.min(merge.left + merge.width * 2, merge.source.length);
    return false;
  }
  if (merge.rightCursor >= merge.right) merge.target.push(merge.source.take(merge.leftCursor++)!);
  else if (merge.leftCursor >= merge.middle) merge.target.push(merge.source.take(merge.rightCursor++)!);
  else merge.target.push(merge.source.take(compare(merge.source.get(merge.leftCursor)!, merge.source.get(merge.rightCursor)!) <= 0 ? merge.leftCursor++ : merge.rightCursor++)!);
  return false;
}

function compareLayoutText(left: string, right: string): number {
  return left < right ? -1 : left > right ? 1 : 0;
}

function projectLayoutNode(source: Node, x: number, y: number): Node {
  return {
    ariaLabel: source.ariaLabel,
    ariaRole: source.ariaRole,
    className: source.className,
    connectable: source.connectable,
    data: source.data,
    deletable: source.deletable,
    domAttributes: source.domAttributes,
    dragHandle: source.dragHandle,
    draggable: source.draggable,
    dragging: source.dragging,
    expandParent: source.expandParent,
    extent: source.extent,
    focusable: source.focusable,
    handles: source.handles,
    height: source.height,
    hidden: source.hidden,
    id: source.id,
    initialHeight: source.initialHeight,
    initialWidth: source.initialWidth,
    measured: source.measured,
    origin: source.origin,
    parentId: source.parentId,
    position: { x, y },
    resizing: source.resizing,
    selectable: source.selectable,
    selected: source.selected,
    sourcePosition: source.sourcePosition,
    style: source.style,
    targetPosition: source.targetPosition,
    type: source.type,
    width: source.width,
    zIndex: source.zIndex,
  };
}

function projectLayoutEdge(source: Edge): Edge {
  return {
    animated: source.animated,
    ariaLabel: source.ariaLabel,
    ariaRole: source.ariaRole,
    className: source.className,
    data: source.data,
    deletable: source.deletable,
    domAttributes: source.domAttributes,
    focusable: source.focusable,
    hidden: source.hidden,
    id: source.id,
    interactionWidth: source.interactionWidth,
    label: source.label,
    labelBgBorderRadius: source.labelBgBorderRadius,
    labelBgPadding: source.labelBgPadding,
    labelBgStyle: source.labelBgStyle,
    labelShowBg: source.labelShowBg,
    labelStyle: source.labelStyle,
    markerEnd: source.markerEnd,
    markerStart: source.markerStart,
    reconnectable: source.reconnectable,
    selectable: source.selectable,
    selected: source.selected,
    source: source.source,
    sourceHandle: source.sourceHandle,
    style: source.style,
    target: source.target,
    targetHandle: source.targetHandle,
    type: source.type,
    zIndex: source.zIndex,
  };
}

function pagedLayoutArray<Value>(store: DiagramPagedStore<Value>, length: number): Value[] {
  const target: Value[] = [];
  const numericIndex = (property: PropertyKey): number | undefined => {
    if (typeof property !== "string" || !/^(0|[1-9]\d*)$/.test(property)) return undefined;
    const index = Number(property);
    return Number.isSafeInteger(index) && index < length ? index : undefined;
  };
  return new Proxy(target, {
    get(array, property, receiver) {
      if (property === "length") return length;
      const index = numericIndex(property);
      return index === undefined ? Reflect.get(array, property, receiver) : store.get(index);
    },
    getOwnPropertyDescriptor(array, property) {
      const index = numericIndex(property);
      return index === undefined ? Reflect.getOwnPropertyDescriptor(array, property) : { configurable: true, enumerable: true, value: store.get(index), writable: false };
    },
    has(array, property) {
      return numericIndex(property) !== undefined || Reflect.has(array, property);
    },
  });
}

export class DiagramLayoutPublication {
  private capturedNodes: DiagramPagedStore<Node> | undefined = new DiagramPagedStore<Node>(diagramLayoutLimits.maxNodes);
  private capturedEdges: DiagramPagedStore<Edge> | undefined = new DiagramPagedStore<Edge>(diagramLayoutLimits.maxEdges);
  private readonly positions = new DiagramPagedStore<DiagramLayoutPosition>(diagramLayoutLimits.maxNodes);
  private closeStage: "positions" | "edges" | "nodes" | "terminal" | "complete" = "positions";
  private expectedPosition = 0;
  private expectedSequence = 1;
  private outputComplete = false;
  private faulted = false;
  private terminalRetained = false;

  constructor(
    private readonly sourceNodes: readonly Node[],
    private readonly sourceEdges: readonly Edge[],
    readonly descriptor: DiagramLayoutDescriptor,
  ) {}

  readInputPage(cursor: number, maxItems: number): DiagramLayoutHostPage {
    try {
      if (this.faulted || !Number.isSafeInteger(cursor) || cursor < 0 || cursor > this.sourceNodes.length + this.sourceEdges.length) return this.faultPage();
      const limit = Math.max(1, Math.min(DIAGRAM_LAYOUT_INGRESS_ITEMS, Math.floor(finiteLayoutValue(maxItems, 1))));
      if (cursor < this.sourceNodes.length) return this.readNodePage(cursor, limit);
      return this.readEdgePage(cursor - this.sourceNodes.length, limit);
    } catch {
      return this.faultPage();
    }
  }

  acceptOutputPage(page: DiagramLayoutHostPage): boolean {
    try {
      if (this.faulted || this.outputComplete || !Number.isSafeInteger(page.itemCount) || !Number.isSafeInteger(page.byteLength) || page.itemCount < 0 || page.itemCount > DIAGRAM_LAYOUT_OUTPUT_ITEMS || page.byteLength < 0 || page.byteLength > DIAGRAM_LAYOUT_INGRESS_BYTES) return this.rejectOutput();
      const payload = page.payload as DiagramLayoutPositionPage;
      if (!payload || payload.kind !== "positions" || payload.generation !== this.descriptor.generation || payload.sequence !== this.expectedSequence || !Array.isArray(payload.values) || payload.values.length !== page.itemCount || page.byteLength !== page.itemCount * 32 || page.complete !== payload.complete || (page.itemCount === 0 && this.sourceNodes.length > this.expectedPosition)) return this.rejectOutput();
      for (let index = 0; index < payload.values.length; index++) {
        const position = payload.values[index]!;
        if (!position || position.index !== this.expectedPosition || position.index >= this.sourceNodes.length || !Number.isFinite(position.x) || !Number.isFinite(position.y)) return this.rejectOutput();
        this.positions.set(position.index, { index: position.index, x: position.x, y: position.y });
        const node = this.capturedNodes?.get(position.index);
        if (node) node.position = { x: position.x, y: position.y };
        this.expectedPosition += 1;
      }
      const exactComplete = this.expectedPosition === this.sourceNodes.length;
      if (payload.complete !== exactComplete) return this.rejectOutput();
      this.expectedSequence += 1;
      this.outputComplete = payload.complete;
      return true;
    } catch {
      return this.rejectOutput();
    }
  }

  acceptTerminal(terminal: DiagramLayoutTerminal): DiagramLayoutPublicationResult | undefined {
    this.terminalRetained = true;
    if (terminal.generation !== this.descriptor.generation || terminal.status !== "complete" || this.faulted || !this.outputComplete || this.expectedPosition !== this.sourceNodes.length || this.capturedNodes?.length !== this.sourceNodes.length || this.capturedEdges?.length !== this.sourceEdges.length) {
      this.faulted = true;
      return undefined;
    }
    const nodes = this.capturedNodes;
    const edges = this.capturedEdges;
    this.capturedNodes = undefined;
    this.capturedEdges = undefined;
    return new DiagramLayoutPublishedResult(nodes, edges, this.sourceNodes.length, this.sourceEdges.length);
  }

  closeStep(): boolean {
    if (this.closeStage === "positions") {
      if (!this.positions.releasePageStep()) return false;
      this.closeStage = "edges";
      return false;
    }
    if (this.closeStage === "edges") {
      if (this.capturedEdges && !this.capturedEdges.releasePageStep()) return false;
      this.capturedEdges = undefined;
      this.closeStage = "nodes";
      return false;
    }
    if (this.closeStage === "nodes") {
      if (this.capturedNodes && !this.capturedNodes.releasePageStep()) return false;
      this.capturedNodes = undefined;
      this.closeStage = "terminal";
      return false;
    }
    if (this.closeStage === "terminal") {
      this.terminalRetained = false;
      this.closeStage = "complete";
    }
    return true;
  }

  terminalIsEmpty(): boolean {
    return !this.terminalRetained;
  }

  private readNodePage(offset: number, limit: number): DiagramLayoutHostPage {
    const values: DiagramLayoutNodeWire[] = [];
    let bytes = 0;
    while (values.length < limit && offset + values.length < this.sourceNodes.length) {
      const index = offset + values.length;
      const source = this.sourceNodes[index];
      if (!source || typeof source.id !== "string") return this.faultPage();
      const value: DiagramLayoutNodeWire = {
        height: source.height,
        id: source.id,
        index,
        measuredHeight: source.measured?.height,
        measuredWidth: source.measured?.width,
        styleHeight: typeof source.style?.height === "number" ? source.style.height : undefined,
        styleWidth: typeof source.style?.width === "number" ? source.style.width : undefined,
        width: source.width,
      };
      let valueBytes: number;
      try {
        valueBytes = diagramLayoutNodeWireBytes(value);
      } catch {
        return this.faultPage();
      }
      if (values.length > 0 && bytes + valueBytes > DIAGRAM_LAYOUT_INGRESS_BYTES) break;
      if (valueBytes > DIAGRAM_LAYOUT_INGRESS_BYTES) return this.faultPage();
      this.capturedNodes!.set(index, projectLayoutNode(source, source.position.x, source.position.y));
      values.push(value);
      bytes += valueBytes;
    }
    const next = offset + values.length;
    const complete = next === this.sourceNodes.length && this.sourceEdges.length === 0;
    return { byteLength: bytes, complete, itemCount: values.length, payload: { bytes, complete, generation: this.descriptor.generation, kind: "nodes", offset, values } satisfies DiagramLayoutIngressPage };
  }

  private readEdgePage(offset: number, limit: number): DiagramLayoutHostPage {
    const values: DiagramLayoutEdgeWire[] = [];
    let bytes = 0;
    while (values.length < limit && offset + values.length < this.sourceEdges.length) {
      const index = offset + values.length;
      const source = this.sourceEdges[index];
      if (!source || typeof source.id !== "string" || typeof source.source !== "string" || typeof source.target !== "string") return this.faultPage();
      const value: DiagramLayoutEdgeWire = { id: source.id, index, source: source.source, target: source.target };
      let valueBytes: number;
      try {
        valueBytes = diagramLayoutEdgeWireBytes(value);
      } catch {
        return this.faultPage();
      }
      if (values.length > 0 && bytes + valueBytes > DIAGRAM_LAYOUT_INGRESS_BYTES) break;
      if (valueBytes > DIAGRAM_LAYOUT_INGRESS_BYTES) return this.faultPage();
      this.capturedEdges!.set(index, projectLayoutEdge(source));
      values.push(value);
      bytes += valueBytes;
    }
    const next = offset + values.length;
    const complete = next === this.sourceEdges.length;
    return { byteLength: bytes, complete, itemCount: values.length, payload: { bytes, complete, generation: this.descriptor.generation, kind: "edges", offset, values } satisfies DiagramLayoutIngressPage };
  }

  private faultPage(): DiagramLayoutHostPage {
    this.faulted = true;
    return { byteLength: 0, complete: true, itemCount: 0, payload: { generation: this.descriptor.generation, kind: "seal" } satisfies DiagramLayoutIngressPage };
  }

  private rejectOutput(): false {
    this.faulted = true;
    return false;
  }
}

class DiagramLayoutPublishedResult implements DiagramLayoutPublicationResult {
  readonly nodes: Node[];
  readonly edges: Edge[];

  constructor(
    private readonly nodeStore: DiagramPagedStore<Node>,
    private readonly edgeStore: DiagramPagedStore<Edge>,
    nodeCount: number,
    edgeCount: number,
  ) {
    this.nodes = pagedLayoutArray(nodeStore, nodeCount);
    this.edges = pagedLayoutArray(edgeStore, edgeCount);
  }

  closeStep(): boolean {
    if (!this.nodeStore.releasePageStep()) return false;
    return this.edgeStore.releasePageStep();
  }
}

/** 📡️ Creates the bounded browser-host publication authority without reading either source array. */
export function createDiagramLayoutPublication(nodes: readonly Node[], edges: readonly Edge[], options: DiagramLayoutOptions, generation: number): DiagramLayoutPublication {
  return new DiagramLayoutPublication(nodes, edges, { edgeCount: edges.length, generation, kind: DIAGRAM_LAYOUT_CODEC_KIND, nodeCount: nodes.length, options });
}

class DiagramLayoutOwnedResult {
  constructor(
    private readonly nodes: DiagramPagedStore<Node>,
    private readonly edges: DiagramPagedStore<Edge>,
    readonly nodeCount: number,
    readonly edgeCount: number,
  ) {}

  takeNode(index: number): Node | undefined {
    return this.nodes.take(index);
  }

  takeEdge(index: number): Edge | undefined {
    return this.edges.take(index);
  }

  closeStep(): boolean {
    if (this.nodes.length > 0) this.nodes.pop();
    else if (this.edges.length > 0) this.edges.pop();
    return this.nodes.length === 0 && this.edges.length === 0;
  }
}

/** 🧭️ Persistent deterministic directed-layout authority used by both batch and React paths. */
class DiagramLayoutJob {
  private sourceNodes: DiagramLayoutOwnedSource<Node> | undefined;
  private sourceEdges: DiagramLayoutOwnedSource<Edge> | undefined;
  private readonly options: Required<DiagramLayoutOptions>;
  private nodes = new DiagramPagedStore<DiagramLayoutNodeRuntime>(diagramLayoutLimits.maxNodes);
  private edges = new DiagramPagedStore<DiagramLayoutEdgeRuntime>(diagramLayoutLimits.maxEdges);
  private capturedNodes = new DiagramPagedStore<Node>(diagramLayoutLimits.maxNodes);
  private capturedEdges = new DiagramPagedStore<Edge>(diagramLayoutLimits.maxEdges);
  private queue = new DiagramPagedStore<number>(diagramLayoutLimits.maxNodes);
  private rankCross = new DiagramPagedStore<number>(diagramLayoutLimits.maxNodes);
  private rankDepth = new DiagramPagedStore<number>(diagramLayoutLimits.maxNodes);
  private rankOffset = new DiagramPagedStore<number>(diagramLayoutLimits.maxNodes);
  private rankSpan = new DiagramPagedStore<number>(diagramLayoutLimits.maxNodes);
  private edgeNext = new DiagramPagedStore<number>(diagramLayoutLimits.maxEdges);
  private previewPositions = new DiagramPagedStore<DiagramLayoutPosition>(diagramLayoutLimits.previewNodes);
  private layoutX = new DiagramPagedStore<number>(diagramLayoutLimits.maxNodes);
  private layoutY = new DiagramPagedStore<number>(diagramLayoutLimits.maxNodes);
  private admittedEdgeCount = 0;
  private queueLength = 0;
  private previewLength = 0;
  private previewWriteCursor = 0;
  private resultTaken = false;
  private pendingEdge?: DiagramLayoutPendingEdge;
  private nodeMerge?: DiagramLayoutMerge<DiagramLayoutNodeRuntime>;
  private edgeMerge?: DiagramLayoutMerge<DiagramLayoutEdgeRuntime>;
  private crossingMerge?: DiagramLayoutMerge<DiagramLayoutNodeRuntime>;
  private readonly mergeSpares = new Array<DiagramPagedStore<unknown>>(9);
  private mergeSpareLength = 0;
  private stage: DiagramLayoutStage = "admit-nodes";
  private status: DiagramLayoutJobStatus = "running";
  private cursor = 0;
  private secondaryCursor = 0;
  private queueCursor = 0;
  private activeRankNode = -1;
  private unresolvedCursor = 0;
  private maxRank = 0;
  private totalDepth = 0;
  private previewSequence = 0;
  private faultReason?: string;
  private closeStage: DiagramLayoutCloseStage = "previews";
  private closeCursor = 0;
  private closeArray = 0;
  private closePrepared = false;
  private readonly sourceNodeCount: number;
  private readonly sourceEdgeCount: number;

  private constructor(nodes: readonly Node[] | DiagramLayoutOwnedSource<Node>, edges: readonly Edge[] | DiagramLayoutOwnedSource<Edge>, options: DiagramLayoutOptions = {}, readonly generation = 1) {
    this.sourceNodes = asDiagramLayoutSource(nodes);
    this.sourceEdges = asDiagramLayoutSource(edges);
    this.sourceNodeCount = nodes.length;
    this.sourceEdgeCount = edges.length;
    this.options = resolveLayoutOptions(options);
    if (nodes.length > diagramLayoutLimits.maxNodes || edges.length > diagramLayoutLimits.maxEdges) this.fail("Diagram layout capacity exceeded");
  }

  static fromBatchTest(nodes: readonly Node[], edges: readonly Edge[], options: DiagramLayoutOptions = {}, generation = 1): DiagramLayoutJob {
    return new DiagramLayoutJob(nodes, edges, options, generation);
  }

  static fromOwnedPagedSources(nodes: DiagramLayoutOwnedSource<Node>, edges: DiagramLayoutOwnedSource<Edge>, options: DiagramLayoutOptions = {}, generation = 1): DiagramLayoutJob {
    return new DiagramLayoutJob(nodes, edges, options, generation);
  }

  takeResult(): DiagramLayoutOwnedResult | undefined {
    if (this.status !== "complete" || this.resultTaken) return undefined;
    this.resultTaken = true;
    const result = new DiagramLayoutOwnedResult(this.capturedNodes, this.capturedEdges, this.sourceNodeCount, this.sourceEdgeCount);
    this.capturedNodes = new DiagramPagedStore(diagramLayoutLimits.maxNodes);
    this.capturedEdges = new DiagramPagedStore(diagramLayoutLimits.maxEdges);
    return result;
  }

  get reason(): string | undefined {
    return this.faultReason;
  }

  cancel(generation = this.generation): void {
    if (generation === this.generation && this.status === "running") this.status = "cancelled";
  }

  takePreview(): DiagramLayoutPreview | undefined {
    if (this.previewLength === 0) return undefined;
    const positions = new Array<DiagramLayoutPosition>(this.previewLength);
    for (let index = 0; index < this.previewLength; index++) {
      const sourceIndex = (this.previewWriteCursor - this.previewLength + index + diagramLayoutLimits.previewNodes) % diagramLayoutLimits.previewNodes;
      positions[index] = this.previewPositions.take(sourceIndex)!;
    }
    this.previewPositions.resetCleared();
    this.previewLength = 0;
    this.previewWriteCursor = 0;
    return { generation: this.generation, positions, sequence: this.previewSequence };
  }

  step(work: DiagramLayoutWork): DiagramLayoutStepResult {
    const fuel = Math.max(0, Math.floor(finiteLayoutValue(work.fuel, 0)));
    if (work.generation !== this.generation) this.cancel();
    if (this.status !== "running" || fuel === 0) return { consumed: 0, stage: this.stage, status: this.status };
    let remaining = fuel;
    while (remaining > 0 && this.now() < work.deadline && this.status === "running") {
      remaining -= 1;
      this.stepUnit();
    }
    return { consumed: fuel - remaining, stage: this.stage, status: this.status };
  }

  close(work: Omit<DiagramLayoutWork, "generation">): boolean {
    this.prepareClose();
    let remaining = Math.max(0, Math.floor(finiteLayoutValue(work.fuel, 0)));
    while (remaining > 0 && this.now() < work.deadline && this.closeStage !== "complete") {
      remaining -= 1;
      this.closeUnit();
    }
    return this.closeStage === "complete";
  }

  private now(): number {
    return typeof performance === "undefined" ? Date.now() : performance.now();
  }

  private fail(reason: string): void {
    this.faultReason = reason;
    this.status = "fault";
  }

  private prepareClose(): void {
    if (this.closePrepared) return;
    this.closePrepared = true;
    for (const merge of [this.nodeMerge, this.edgeMerge, this.crossingMerge]) {
      if (!merge) continue;
      this.mergeSpares[this.mergeSpareLength++] = merge.source;
      this.mergeSpares[this.mergeSpareLength++] = merge.target;
    }
  }

  private advance(stage: DiagramLayoutStage): void {
    this.stage = stage;
    this.cursor = 0;
    this.secondaryCursor = 0;
  }

  private stepUnit(): void {
    if (this.stage === "admit-nodes") this.admitNode();
    else if (this.stage === "sort-nodes") this.sortNode();
    else if (this.stage === "index-nodes") this.indexNode();
    else if (this.stage === "admit-edges") this.admitEdge();
    else if (this.stage === "sort-edges") this.sortEdge();
    else if (this.stage === "build-graph") this.buildGraph();
    else if (this.stage === "assign-ranks") this.assignRank();
    else if (this.stage === "crossing") this.accumulateCrossing();
    else if (this.stage === "sort-crossing") this.sortCrossing();
    else if (this.stage === "measure-ranks") this.measureRank();
    else if (this.stage === "position-ranks") this.positionRank();
    else if (this.stage === "coordinates") this.coordinateNode();
    else if (this.stage === "project") this.projectNode();
    else if (this.stage === "project-edges") this.projectEdge();
  }

  private admitNode(): void {
    const source = this.sourceNodes!;
    if (this.cursor >= source.length) {
      this.nodeMerge = createLayoutMerge(this.nodes);
      this.sourceNodes = undefined;
      this.advance("sort-nodes");
      return;
    }
    const node = source.get(this.cursor)!;
    if (!diagramLayoutIdentityAdmitted(node.id)) {
      this.fail("Diagram layout node id is invalid");
      return;
    }
    const sourceIndex = this.cursor++;
    const captured = projectLayoutNode(node, node.position.x, node.position.y);
    this.capturedNodes.set(sourceIndex, captured);
    this.nodes.push({
      barycenterCount: 0,
      barycenterSum: 0,
      cross: 0,
      depth: 0,
      height: nodeLayoutDimension(captured, "height", this.options.nodeHeight),
      id: captured.id,
      indegree: 0,
      order: 0,
      outgoingHead: -1,
      outgoingTail: -1,
      processed: false,
      rank: 0,
      sourceIndex,
      width: nodeLayoutDimension(captured, "width", this.options.nodeWidth),
      x: 0,
      y: 0,
    });
  }

  private sortNode(): void {
    if (!this.nodeMerge || stepLayoutMerge(this.nodeMerge, (left, right) => compareLayoutText(left.id, right.id))) {
      if (this.nodeMerge) {
        this.nodes = this.nodeMerge.source;
        this.mergeSpares[this.mergeSpareLength++] = this.nodeMerge.target;
      }
      this.nodeMerge = undefined;
      this.advance("index-nodes");
    }
  }

  private indexNode(): void {
    if (this.cursor >= this.nodes.length) {
      this.advance("admit-edges");
      return;
    }
    const node = this.nodes.get(this.cursor)!;
    if (this.cursor > 0 && this.nodes.get(this.cursor - 1)!.id === node.id) {
      this.fail("Duplicate Diagram layout node id");
      return;
    }
    node.order = this.cursor;
    this.cursor += 1;
  }

  private admitEdge(): void {
    const source = this.sourceEdges!;
    if (this.cursor >= source.length && !this.pendingEdge) {
      this.edgeMerge = createLayoutMerge(this.edges);
      this.sourceEdges = undefined;
      this.advance("sort-edges");
      return;
    }
    if (!this.pendingEdge) {
      const edge = source.get(this.cursor);
      const inputIndex = this.cursor++;
      if (!edge) return;
      if (!diagramLayoutIdentityAdmitted(edge.id, true) || !diagramLayoutIdentityAdmitted(edge.source) || !diagramLayoutIdentityAdmitted(edge.target)) {
        this.fail("Diagram layout edge identity is invalid");
        return;
      }
      const captured = projectLayoutEdge(edge);
      this.capturedEdges.set(inputIndex, captured);
      this.pendingEdge = { captured, inputIndex, sourceLookup: { done: false, high: this.nodes.length - 1, low: 0, value: captured.source } };
    }
    const pending = this.pendingEdge;
    if (!pending.sourceLookup.done) {
      this.stepLayoutLookup(pending.sourceLookup);
      return;
    }
    pending.targetLookup ??= { done: false, high: this.nodes.length - 1, low: 0, value: pending.captured.target };
    if (!pending.targetLookup.done) {
      this.stepLayoutLookup(pending.targetLookup);
      return;
    }
    const sourceIndex = pending.sourceLookup.result;
    const targetIndex = pending.targetLookup.result;
    if (sourceIndex !== undefined && targetIndex !== undefined) {
      const id = typeof pending.captured.id === "string" ? pending.captured.id : `${pending.captured.source}:${pending.captured.target}:${pending.inputIndex}`;
      this.edges.push({ id, source: sourceIndex, sourceId: pending.captured.source, sourceIndex: pending.inputIndex, target: targetIndex, targetId: pending.captured.target });
      this.admittedEdgeCount += 1;
    }
    this.pendingEdge = undefined;
  }

  private stepLayoutLookup(lookup: DiagramLayoutLookup): void {
    if (lookup.low > lookup.high) {
      lookup.done = true;
      return;
    }
    const middle = Math.floor((lookup.low + lookup.high) / 2);
    const comparison = compareLayoutText(lookup.value, this.nodes.get(middle)!.id);
    if (comparison === 0) {
      lookup.done = true;
      lookup.result = middle;
    } else if (comparison < 0) lookup.high = middle - 1;
    else lookup.low = middle + 1;
  }

  private sortEdge(): void {
    if (!this.edgeMerge || stepLayoutMerge(this.edgeMerge, (left, right) => compareLayoutText(left.sourceId, right.sourceId) || compareLayoutText(left.targetId, right.targetId) || compareLayoutText(left.id, right.id))) {
      if (this.edgeMerge) {
        this.edges = this.edgeMerge.source;
        this.mergeSpares[this.mergeSpareLength++] = this.edgeMerge.target;
      }
      this.edgeMerge = undefined;
      this.advance("build-graph");
    }
  }

  private buildGraph(): void {
    if (this.cursor >= this.edges.length) {
      if (this.secondaryCursor < this.nodes.length) {
        const node = this.nodes.get(this.secondaryCursor);
        if (node && node.indegree === 0) this.queue.set(this.queueLength++, this.secondaryCursor);
        this.secondaryCursor += 1;
        return;
      }
      this.queueCursor = 0;
      this.activeRankNode = -1;
      this.unresolvedCursor = 0;
      this.advance("assign-ranks");
      return;
    }
    const edgeIndex = this.cursor++;
    const edge = this.edges.get(edgeIndex)!;
    if (edge.source === edge.target) return;
    const source = this.nodes.get(edge.source)!;
    const target = this.nodes.get(edge.target)!;
    this.edgeNext.set(edgeIndex, -1);
    if (source.outgoingTail < 0) source.outgoingHead = edgeIndex;
    else this.edgeNext.set(source.outgoingTail, edgeIndex);
    source.outgoingTail = edgeIndex;
    target.indegree += 1;
  }

  private assignRank(): void {
    if (this.activeRankNode >= 0) {
      const active = this.nodes.get(this.activeRankNode)!;
      if (this.secondaryCursor >= 0) {
        const edgeIndex = this.secondaryCursor;
        this.secondaryCursor = this.edgeNext.get(edgeIndex) ?? -1;
        const edge = this.edges.get(edgeIndex)!;
        const target = this.nodes.get(edge.target)!;
        if (!target.processed) {
          target.rank = Math.max(target.rank, active.rank + 1);
          target.indegree = Math.max(0, target.indegree - 1);
          if (target.indegree === 0) this.queue.set(this.queueLength++, edge.target);
        }
        return;
      }
      active.processed = true;
      this.activeRankNode = -1;
      this.secondaryCursor = -1;
      return;
    }
    if (this.queueCursor < this.queueLength) {
      const candidate = this.queue.get(this.queueCursor++)!;
      if (this.nodes.get(candidate)!.processed) return;
      this.activeRankNode = candidate;
      const active = this.nodes.get(candidate)!;
      this.secondaryCursor = active.outgoingHead;
      this.maxRank = Math.max(this.maxRank, active.rank);
      return;
    }
    if (this.unresolvedCursor < this.nodes.length) {
      const candidate = this.unresolvedCursor++;
      if (this.nodes.get(candidate)!.processed) return;
      this.nodes.get(candidate)!.indegree = 0;
      this.queue.set(this.queueLength++, candidate);
      return;
    }
    this.advance("crossing");
  }

  private accumulateCrossing(): void {
    if (this.cursor >= this.edges.length) {
      this.crossingMerge = createLayoutMerge(this.nodes);
      this.advance("sort-crossing");
      return;
    }
    const edge = this.edges.get(this.cursor++)!;
    const source = this.nodes.get(edge.source)!;
    const target = this.nodes.get(edge.target)!;
    if (source.rank < target.rank) {
      target.barycenterCount += 1;
      target.barycenterSum += source.order;
    }
  }

  private sortCrossing(): void {
    const compare = (left: DiagramLayoutNodeRuntime, right: DiagramLayoutNodeRuntime) => {
      if (left.rank !== right.rank) return left.rank - right.rank;
      const leftBarycenter = left.barycenterCount === 0 ? left.order : left.barycenterSum / left.barycenterCount;
      const rightBarycenter = right.barycenterCount === 0 ? right.order : right.barycenterSum / right.barycenterCount;
      return leftBarycenter - rightBarycenter || compareLayoutText(left.id, right.id);
    };
    if (!this.crossingMerge || stepLayoutMerge(this.crossingMerge, compare)) {
      if (this.crossingMerge) {
        this.nodes = this.crossingMerge.source;
        this.mergeSpares[this.mergeSpareLength++] = this.crossingMerge.target;
      }
      this.crossingMerge = undefined;
      this.advance("measure-ranks");
    }
  }

  private measureRank(): void {
    if (this.cursor >= this.nodes.length) {
      this.advance("position-ranks");
      return;
    }
    const node = this.nodes.get(this.cursor++)!;
    const horizontal = this.options.direction === "LR" || this.options.direction === "RL";
    const crossSize = horizontal ? node.height : node.width;
    const depthSize = horizontal ? node.width : node.height;
    const rank = node.rank;
    const span = this.rankSpan.get(rank);
    this.rankSpan.set(rank, (span ?? 0) + (span === undefined ? 0 : this.options.nodeSep) + crossSize);
    this.rankDepth.set(rank, Math.max(this.rankDepth.get(rank) ?? 0, depthSize));
  }

  private positionRank(): void {
    if (this.cursor > this.maxRank) {
      this.totalDepth = this.maxRank < 0 ? 0 : (this.rankOffset.get(this.maxRank) ?? 0) + (this.rankDepth.get(this.maxRank) ?? 0);
      this.advance("coordinates");
      return;
    }
    const rank = this.cursor++;
    this.rankOffset.set(rank, rank === 0 ? 0 : (this.rankOffset.get(rank - 1) ?? 0) + (this.rankDepth.get(rank - 1) ?? 0) + this.options.rankSep);
    this.rankCross.set(rank, -(this.rankSpan.get(rank) ?? 0) / 2);
  }

  private coordinateNode(): void {
    if (this.cursor >= this.nodes.length) {
      this.advance("project");
      return;
    }
    const node = this.nodes.get(this.cursor++)!;
    const horizontal = this.options.direction === "LR" || this.options.direction === "RL";
    const crossSize = horizontal ? node.height : node.width;
    const depthSize = horizontal ? node.width : node.height;
    const cross = (this.rankCross.get(node.rank) ?? 0) + crossSize / 2;
    const forwardDepth = (this.rankOffset.get(node.rank) ?? 0) + depthSize / 2;
    const depth = this.options.direction === "BT" || this.options.direction === "RL" ? this.totalDepth - forwardDepth : forwardDepth;
    this.rankCross.set(node.rank, cross + crossSize / 2 + this.options.nodeSep);
    node.cross = cross;
    node.depth = depth;
    node.x = horizontal ? depth - node.width / 2 : cross - node.width / 2;
    node.y = horizontal ? cross - node.height / 2 : depth - node.height / 2;
    this.layoutX.set(node.sourceIndex, node.x);
    this.layoutY.set(node.sourceIndex, node.y);
  }

  private projectNode(): void {
    if (this.cursor >= this.sourceNodeCount) {
      this.advance("project-edges");
      return;
    }
    const sourceNode = this.capturedNodes.get(this.cursor++);
    if (!sourceNode) return;
    const sourceIndex = this.cursor - 1;
    const x = this.layoutX.get(sourceIndex);
    const y = this.layoutY.get(sourceIndex);
    if (x === undefined || y === undefined) return;
    sourceNode.position = { x, y };
    this.previewPositions.set(this.previewWriteCursor, { index: sourceIndex, x, y });
    this.previewWriteCursor = (this.previewWriteCursor + 1) % diagramLayoutLimits.previewNodes;
    this.previewLength = Math.min(diagramLayoutLimits.previewNodes, this.previewLength + 1);
    this.previewSequence += 1;
  }

  private projectEdge(): void {
    if (this.cursor >= this.sourceEdgeCount) {
      this.stage = "complete";
      this.status = "complete";
      return;
    }
    this.capturedEdges.get(this.cursor++);
  }

  private closeUnit(): void {
    if (this.closeStage === "previews") {
      if (this.previewLength > 0) this.previewPositions.take(--this.previewLength);
      else if (!this.resultTaken && this.capturedNodes.length > 0) this.capturedNodes.pop();
      else if (!this.resultTaken && this.capturedEdges.length > 0) this.capturedEdges.pop();
      else this.closeStage = "edges";
      return;
    }
    if (this.closeStage === "edges") {
      if (this.edges.length > 0) this.edges.pop();
      else this.closeStage = "nodes";
      return;
    }
    if (this.closeStage === "nodes") {
      const node = this.nodes.get(this.nodes.length - 1);
      if (!node) {
        this.closeStage = "spares";
        return;
      }
      this.nodes.pop();
      return;
    }
    if (this.closeStage === "spares") {
      if (this.closeCursor >= this.mergeSpareLength) {
        this.closeCursor = 0;
        this.closeStage = "indices";
        return;
      }
      if (this.mergeSpares[this.closeCursor]!.releaseOnePage()) this.closeCursor += 1;
      return;
    }
    if (this.closeStage === "indices") {
      const store = this.closeIndexStore();
      if (!store) {
        this.closeCursor = 0;
        this.closeArray = 0;
        this.closeStage = "captures";
        return;
      }
      if (store.length > 0) store.pop();
      else this.closeArray += 1;
      return;
    }
    if (this.closeStage === "captures") {
      this.closeStage = "scalars";
      return;
    }
    if (this.closeStage === "scalars") {
      this.sourceNodes = undefined;
      this.sourceEdges = undefined;
      this.nodeMerge = undefined;
      this.edgeMerge = undefined;
      this.crossingMerge = undefined;
      this.closeStage = "complete";
    }
  }

  private closeIndexStore(): DiagramPagedStore<number> | undefined {
    if (this.closeArray === 0) return this.queue;
    if (this.closeArray === 1) return this.rankCross;
    if (this.closeArray === 2) return this.rankDepth;
    if (this.closeArray === 3) return this.rankOffset;
    if (this.closeArray === 4) return this.rankSpan;
    if (this.closeArray === 5) return this.edgeNext;
    if (this.closeArray === 6) return this.layoutX;
    if (this.closeArray === 7) return this.layoutY;
    return undefined;
  }
}

/** 🧬️ Worker-registry adapter that owns bounded wire ingress and the exact layout job. */
class DiagramLayoutWireJob implements DiagramLayoutWorkerJob {
  private readonly nodes = new DiagramPagedStore<Node>(diagramLayoutLimits.maxNodes);
  private readonly edges = new DiagramPagedStore<Edge>(diagramLayoutLimits.maxEdges);
  private nodeReceived = 0;
  private edgeReceived = 0;
  private job?: DiagramLayoutJob;
  private owned?: DiagramLayoutOwnedResult;
  private resultCursor = 0;
  private sequence = 0;
  private emptyResultPublished = false;
  private cancelled = false;
  private ingesting = false;
  private faultReason?: string;

  constructor(readonly descriptor: DiagramLayoutDescriptor) {
    const credits = diagramLayoutCredits(descriptor.nodeCount, descriptor.edgeCount);
    if (
      descriptor.kind !== DIAGRAM_LAYOUT_CODEC_KIND ||
      !Number.isSafeInteger(descriptor.generation) ||
      descriptor.generation < 0 ||
      !Number.isSafeInteger(descriptor.nodeCount) ||
      !Number.isSafeInteger(descriptor.edgeCount) ||
      descriptor.nodeCount < 0 ||
      descriptor.edgeCount < 0 ||
      descriptor.nodeCount > diagramLayoutLimits.maxNodes ||
      descriptor.edgeCount > diagramLayoutLimits.maxEdges ||
      !credits.admitted
    )
      this.faultReason = "Diagram layout descriptor capacity is invalid";
  }

  get status(): DiagramLayoutJobStatus {
    if (this.faultReason) return "fault";
    if (this.cancelled) return "cancelled";
    if (!this.job) return "running";
    return this.job.step({ deadline: 0, fuel: 0, generation: this.descriptor.generation }).status;
  }

  get reason(): string | undefined {
    return this.faultReason ?? this.job?.reason;
  }

  ingest(page: unknown): boolean {
    if (this.cancelled || this.faultReason || this.ingesting) return false;
    this.ingesting = true;
    try {
      if (!page || typeof page !== "object" || Array.isArray(page)) return this.failIngress("Diagram layout ingress page is invalid");
      const candidate = page as Record<string, unknown>;
      const generation = candidate.generation;
      const kind = candidate.kind;
      if (!Number.isSafeInteger(generation) || generation !== this.descriptor.generation) return this.failIngress("Diagram layout ingress generation is invalid");
      if (kind === "seal") return this.sealIngress();
      if (kind !== "nodes" && kind !== "edges") return this.failIngress("Diagram layout ingress kind is invalid");
      if (this.job) return false;
      const offset = candidate.offset;
      const bytes = candidate.bytes;
      const complete = candidate.complete;
      const values = candidate.values;
      if (!Number.isSafeInteger(offset) || (offset as number) < 0 || !Number.isSafeInteger(bytes) || (bytes as number) < 0 || (bytes as number) > DIAGRAM_LAYOUT_INGRESS_BYTES || (complete !== undefined && typeof complete !== "boolean") || !Array.isArray(values) || values.length > DIAGRAM_LAYOUT_INGRESS_ITEMS)
        return this.failIngress("Diagram layout ingress page exceeds its item or byte cap");
      const capturedNodes = kind === "nodes" ? this.captureNodes(values, offset as number, bytes as number) : undefined;
      const capturedEdges = kind === "edges" ? this.captureEdges(values, offset as number, bytes as number) : undefined;
      if ((kind === "nodes" && !capturedNodes) || (kind === "edges" && !capturedEdges)) return false;
      const nextNodeReceived = this.nodeReceived + (capturedNodes?.length ?? 0);
      const nextEdgeReceived = this.edgeReceived + (capturedEdges?.length ?? 0);
      const ingressComplete = nextNodeReceived === this.descriptor.nodeCount && nextEdgeReceived === this.descriptor.edgeCount;
      if (complete && !ingressComplete) return this.failIngress("Diagram layout ingress completed before its declared counts");
      if (this.cancelled || this.faultReason) return false;
      for (let index = 0; index < (capturedNodes?.length ?? 0); index++) this.nodes.set(this.nodeReceived + index, capturedNodes![index]!);
      for (let index = 0; index < (capturedEdges?.length ?? 0); index++) this.edges.set(this.edgeReceived + index, capturedEdges![index]!);
      this.nodeReceived = nextNodeReceived;
      this.edgeReceived = nextEdgeReceived;
      if (ingressComplete) this.job = DiagramLayoutJob.fromOwnedPagedSources(this.nodes, this.edges, this.descriptor.options, this.descriptor.generation);
      return true;
    } catch {
      return this.failIngress("Diagram layout ingress value is invalid");
    } finally {
      this.ingesting = false;
    }
  }

  cancel(generation = this.descriptor.generation): void {
    if (generation !== this.descriptor.generation) return;
    this.cancelled = true;
    this.job?.cancel(generation);
  }

  step(work: DiagramLayoutWork): DiagramLayoutStepResult {
    if (this.faultReason) return { consumed: 0, stage: "complete", status: "fault" };
    if (this.cancelled && !this.job) return { consumed: 0, stage: "complete", status: "cancelled" };
    if (!this.job) return { consumed: 0, stage: "admit-nodes", status: "running" };
    const result = this.job.step(work);
    if (result.status === "fault") this.faultReason = this.job.reason;
    if (result.status === "complete" && this.resultCursor < this.descriptor.nodeCount) return { ...result, status: "running" };
    return result;
  }

  takePreviewPage(): DiagramLayoutPositionPage | undefined {
    return undefined;
  }

  takeResultPage(): DiagramLayoutPositionPage | undefined {
    if (!this.job || this.status !== "complete") return undefined;
    this.owned ??= this.job.takeResult();
    if (!this.owned) return undefined;
    if (this.owned.nodeCount === 0) {
      if (this.emptyResultPublished) return undefined;
      this.emptyResultPublished = true;
      this.sequence += 1;
      return { complete: true, generation: this.descriptor.generation, kind: "positions", sequence: this.sequence, values: [] };
    }
    if (this.resultCursor >= this.owned.nodeCount) return undefined;
    const count = Math.min(DIAGRAM_LAYOUT_OUTPUT_ITEMS, this.owned.nodeCount - this.resultCursor);
    const values = new Array<DiagramLayoutPosition>(count);
    for (let index = 0; index < count; index++) {
      const sourceIndex = this.resultCursor + index;
      const node = this.owned.takeNode(sourceIndex)!;
      values[index] = { index: sourceIndex, x: node.position.x, y: node.position.y };
    }
    this.resultCursor += count;
    this.sequence += 1;
    return { complete: this.resultCursor === this.owned.nodeCount, generation: this.descriptor.generation, kind: "positions", sequence: this.sequence, values };
  }

  close(work: Omit<DiagramLayoutWork, "generation">): boolean {
    let remaining = Math.max(0, Math.floor(finiteLayoutValue(work.fuel, 0)));
    while (remaining > 0 && (typeof performance === "undefined" ? Date.now() : performance.now()) < work.deadline) {
      remaining -= 1;
      if (this.job && !this.job.close({ deadline: work.deadline, fuel: 1 })) continue;
      if (this.owned && !this.owned.closeStep()) continue;
      if (this.nodes.length > 0) {
        this.nodes.pop();
        continue;
      }
      if (this.edges.length > 0) {
        this.edges.pop();
        continue;
      }
      return true;
    }
    return false;
  }

  terminal(): DiagramLayoutTerminal | undefined {
    const status = this.status;
    if (status === "running") return undefined;
    if (status === "complete" && (this.resultCursor < this.descriptor.nodeCount || (this.descriptor.nodeCount === 0 && !this.emptyResultPublished))) return undefined;
    if (status === "fault") return { generation: this.descriptor.generation, kind: "terminal", reason: this.reason ?? "Diagram layout fault", status };
    return { generation: this.descriptor.generation, kind: "terminal", status };
  }

  private captureNodes(values: readonly unknown[], offset: number, declaredBytes: number): Node[] | undefined {
    if (offset !== this.nodeReceived || offset + values.length > this.descriptor.nodeCount) return this.failCapture("Diagram node ingress offset is invalid");
    let bytes = 0;
    const captured = new Array<Node>(values.length);
    for (let index = 0; index < values.length; index++) {
      const source = values[index];
      if (!source || typeof source !== "object" || Array.isArray(source)) return this.failCapture("Diagram node ingress value is invalid");
      const candidate = source as Record<string, unknown>;
      const value: DiagramLayoutNodeWire = {
        height: candidate.height as number | undefined,
        id: candidate.id as string,
        index: candidate.index as number,
        measuredHeight: candidate.measuredHeight as number | undefined,
        measuredWidth: candidate.measuredWidth as number | undefined,
        styleHeight: candidate.styleHeight as number | undefined,
        styleWidth: candidate.styleWidth as number | undefined,
        width: candidate.width as number | undefined,
      };
      if (
        !Number.isSafeInteger(value.index) ||
        value.index !== offset + index ||
        typeof value.id !== "string" ||
        value.id.length === 0 ||
        !optionalFiniteLayoutValue(value.height) ||
        !optionalFiniteLayoutValue(value.measuredHeight) ||
        !optionalFiniteLayoutValue(value.measuredWidth) ||
        !optionalFiniteLayoutValue(value.styleHeight) ||
        !optionalFiniteLayoutValue(value.styleWidth) ||
        !optionalFiniteLayoutValue(value.width)
      )
        return this.failCapture("Diagram node ingress value is invalid");
      bytes += diagramLayoutNodeWireBytes(value);
      if (bytes > DIAGRAM_LAYOUT_INGRESS_BYTES) return this.failCapture("Diagram node ingress exceeds its byte cap");
      if (!value.id) return this.failCapture("Diagram node id is empty");
      captured[index] = {
        data: {},
        height: value.height,
        id: value.id,
        measured: { height: value.measuredHeight, width: value.measuredWidth },
        position: { x: 0, y: 0 },
        style: { height: value.styleHeight, width: value.styleWidth },
        width: value.width,
      };
    }
    if (bytes !== declaredBytes) return this.failCapture("Diagram node ingress byte accounting is invalid");
    return captured;
  }

  private captureEdges(values: readonly unknown[], offset: number, declaredBytes: number): Edge[] | undefined {
    if (offset !== this.edgeReceived || offset + values.length > this.descriptor.edgeCount) return this.failCapture("Diagram edge ingress offset is invalid");
    let bytes = 0;
    const captured = new Array<Edge>(values.length);
    for (let index = 0; index < values.length; index++) {
      const source = values[index];
      if (!source || typeof source !== "object" || Array.isArray(source)) return this.failCapture("Diagram edge ingress value is invalid");
      const candidate = source as Record<string, unknown>;
      const value: DiagramLayoutEdgeWire = { id: candidate.id as string, index: candidate.index as number, source: candidate.source as string, target: candidate.target as string };
      if (!Number.isSafeInteger(value.index) || value.index !== offset + index || typeof value.id !== "string" || typeof value.source !== "string" || typeof value.target !== "string" || value.source.length === 0 || value.target.length === 0)
        return this.failCapture("Diagram edge ingress value is invalid");
      bytes += diagramLayoutEdgeWireBytes(value);
      if (bytes > DIAGRAM_LAYOUT_INGRESS_BYTES) return this.failCapture("Diagram edge ingress exceeds its byte cap");
      captured[index] = { id: value.id, source: value.source, target: value.target };
    }
    if (bytes !== declaredBytes) return this.failCapture("Diagram edge ingress byte accounting is invalid");
    return captured;
  }

  private sealIngress(): boolean {
    if (this.job) return true;
    if (this.nodeReceived !== this.descriptor.nodeCount || this.edgeReceived !== this.descriptor.edgeCount) return this.failIngress("Diagram layout ingress was not complete");
    this.job = DiagramLayoutJob.fromOwnedPagedSources(this.nodes, this.edges, this.descriptor.options, this.descriptor.generation);
    return true;
  }

  private failCapture(reason: string): undefined {
    this.faultReason = reason;
    return undefined;
  }

  private failIngress(reason: string): false {
    this.faultReason = reason;
    return false;
  }
}

/** 🧬️ Bounded worker-registry authority for the owned directed-layout codec. */
export interface DiagramLayoutWorkerJob {
  readonly descriptor: DiagramLayoutDescriptor;
  readonly reason: string | undefined;
  readonly status: DiagramLayoutJobStatus;
  cancel(generation?: number): void;
  close(work: Omit<DiagramLayoutWork, "generation">): boolean;
  ingest(page: unknown): boolean;
  step(work: DiagramLayoutWork): DiagramLayoutStepResult;
  takePreviewPage(): DiagramLayoutPositionPage | undefined;
  takeResultPage(): DiagramLayoutPositionPage | undefined;
  terminal(): DiagramLayoutTerminal | undefined;
}

/** 🧬️ Creates the sole registry-owned concrete layout authority. */
export function createDiagramLayoutWorkerJob(descriptor: DiagramLayoutDescriptor): DiagramLayoutWorkerJob {
  return new DiagramLayoutWireJob(descriptor);
}

/** 🧪️ Creates an exact persistent layout job for colocated tests only. */
export function createDiagramLayoutBatchTestJob(nodes: readonly Node[], edges: readonly Edge[], options: DiagramLayoutOptions = {}, generation = 1): DiagramLayoutJob {
  return DiagramLayoutJob.fromBatchTest(nodes, edges, options, generation);
}

/** 🧪️ Batch adapter that drives the exact persistent job used by the React hook. */
export function calculateDiagramLayoutForBatchTest(nodes: Node[], edges: Edge[], options: DiagramLayoutOptions = {}): { nodes: Node[]; edges: Edge[] } {
  const job = DiagramLayoutJob.fromBatchTest(nodes, edges, options);
  let status: DiagramLayoutJobStatus = "running";
  while (status === "running") status = job.step({ deadline: (typeof performance === "undefined" ? Date.now() : performance.now()) + diagramLayoutFrame.milliseconds, fuel: diagramLayoutFrame.fuel, generation: job.generation }).status;
  if (job.reason) throw new Error(job.reason);
  const owned = job.takeResult();
  if (!owned) return { edges, nodes };
  while (!job.close({ deadline: (typeof performance === "undefined" ? Date.now() : performance.now()) + diagramLayoutFrame.milliseconds, fuel: diagramLayoutFrame.fuel })) {}
  const resultNodes = new Array<Node>(owned.nodeCount);
  const resultEdges = new Array<Edge>(owned.edgeCount);
  for (let index = 0; index < resultNodes.length; index++) resultNodes[index] = owned.takeNode(index)!;
  for (let index = 0; index < resultEdges.length; index++) resultEdges[index] = owned.takeEdge(index)!;
  while (!owned.closeStep()) {}
  return { edges: resultEdges, nodes: resultNodes };
}

// #endregion 🧭️DirectedLayout
