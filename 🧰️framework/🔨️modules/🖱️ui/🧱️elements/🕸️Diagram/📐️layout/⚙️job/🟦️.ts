import type { Node, Edge } from "@xyflow/react";
import { type DiagramLayoutCloseStage, type DiagramLayoutEdgeRuntime, type DiagramLayoutJobStatus, type DiagramLayoutLookup, type DiagramLayoutMerge, type DiagramLayoutNodeRuntime, type DiagramLayoutOwnedSource, type DiagramLayoutPendingEdge, type DiagramLayoutPosition, type DiagramLayoutPreview, type DiagramLayoutStage, type DiagramLayoutStepResult, type DiagramLayoutWork, asDiagramLayoutSource, diagramLayoutLimits } from "../📊️job-types/🟦️.ts";
import { type DiagramLayoutOptions, diagramLayoutIdentityAdmitted } from "../📜️wire/🟦️.ts";
import { DiagramPagedStore, compareLayoutText, createLayoutMerge, finiteLayoutValue, nodeLayoutDimension, projectLayoutEdge, projectLayoutNode, resolveLayoutOptions, stepLayoutMerge } from "../🗃️paged-store/🟦️.ts";
import { DiagramLayoutOwnedResult } from "../📢️publication/🟦️.ts";

/** 🧭️ Persistent deterministic directed-layout authority used by both batch and React paths. */
export class DiagramLayoutJob {
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
