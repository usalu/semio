import type { Node, Edge } from "@xyflow/react";
import { DiagramPagedStore, finiteLayoutValue, pagedLayoutArray, projectLayoutEdge, projectLayoutNode } from "../🗃️paged-store/🟦️.ts";
import { diagramLayoutLimits } from "../📊️job-types/🟦️.ts";
import type { DiagramLayoutPosition } from "../📜️wire/🟦️.ts";
import { DIAGRAM_LAYOUT_CODEC_KIND, DIAGRAM_LAYOUT_INGRESS_BYTES, DIAGRAM_LAYOUT_INGRESS_ITEMS, DIAGRAM_LAYOUT_OUTPUT_ITEMS, type DiagramLayoutDescriptor, type DiagramLayoutEdgeWire, type DiagramLayoutHostPage, type DiagramLayoutIngressPage, type DiagramLayoutNodeWire, type DiagramLayoutOptions, type DiagramLayoutPositionPage, type DiagramLayoutPublicationResult, type DiagramLayoutTerminal, diagramLayoutEdgeWireBytes, diagramLayoutNodeWireBytes } from "../📜️wire/🟦️.ts";

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

export class DiagramLayoutOwnedResult {
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
