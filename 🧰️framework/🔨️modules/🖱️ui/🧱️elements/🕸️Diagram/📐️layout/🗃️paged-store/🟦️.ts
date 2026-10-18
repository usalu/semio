import type { Node, Edge } from "@xyflow/react";
import { type DiagramLayoutMerge, diagramLayoutPageSize } from "../📊️job-types/🟦️.ts";
import { DIAGRAM_UNIT, type DiagramLayoutOptions } from "../📜️wire/🟦️.ts";

export class DiagramPagedStore<Value> {
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

export function finiteLayoutValue(value: unknown, fallback: number): number {
  return typeof value === "number" && Number.isFinite(value) ? value : fallback;
}

export function optionalFiniteLayoutValue(value: unknown): boolean {
  return value === undefined || (typeof value === "number" && Number.isFinite(value));
}

export function resolveLayoutOptions(options: DiagramLayoutOptions): Required<DiagramLayoutOptions> {
  return {
    direction: options.direction ?? "TB",
    nodeHeight: Math.max(1, finiteLayoutValue(options.nodeHeight, DIAGRAM_UNIT)),
    nodeSep: Math.max(0, finiteLayoutValue(options.nodeSep, DIAGRAM_UNIT * 1.04)),
    nodeWidth: Math.max(1, finiteLayoutValue(options.nodeWidth, DIAGRAM_UNIT)),
    rankSep: Math.max(0, finiteLayoutValue(options.rankSep, DIAGRAM_UNIT * 1.67)),
  };
}

export function nodeLayoutDimension(node: Node, axis: "height" | "width", fallback: number): number {
  const measured = node.measured?.[axis];
  const direct = node[axis];
  const style = typeof node.style?.[axis] === "number" ? node.style[axis] : undefined;
  return Math.max(1, finiteLayoutValue(measured, finiteLayoutValue(direct, finiteLayoutValue(style, fallback))));
}

export function createLayoutMerge<Value>(source: DiagramPagedStore<Value>): DiagramLayoutMerge<Value> {
  return { left: 0, leftCursor: 0, middle: Math.min(1, source.length), right: Math.min(2, source.length), rightCursor: Math.min(1, source.length), source, target: new DiagramPagedStore(source.capacity), width: 1 };
}

export function stepLayoutMerge<Value>(merge: DiagramLayoutMerge<Value>, compare: (left: Value, right: Value) => number): boolean {
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

export function compareLayoutText(left: string, right: string): number {
  return left < right ? -1 : left > right ? 1 : 0;
}

export function projectLayoutNode(source: Node, x: number, y: number): Node {
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

export function projectLayoutEdge(source: Edge): Edge {
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

export function pagedLayoutArray<Value>(store: DiagramPagedStore<Value>, length: number): Value[] {
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
