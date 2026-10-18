// #region 🧲️Header
// 💻️ framework/ui/elements/📊️Diagram/layout.ts
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
// #endregion 🧲️Header

import type { Edge, Node } from "@xyflow/react";
import { DIAGRAM_LAYOUT_CODEC_KIND, DIAGRAM_LAYOUT_INGRESS_BYTES, DIAGRAM_LAYOUT_INGRESS_ITEMS, DIAGRAM_LAYOUT_OUTPUT_ITEMS, type DiagramLayoutDescriptor, type DiagramLayoutEdgeWire, type DiagramLayoutNodeWire, type DiagramLayoutOptions, type DiagramLayoutPositionPage, type DiagramLayoutTerminal, diagramLayoutCredits, diagramLayoutEdgeWireBytes, diagramLayoutNodeWireBytes } from "./📜️wire/🟦️.ts";
import { type DiagramLayoutJobStatus, type DiagramLayoutPosition, type DiagramLayoutStepResult, type DiagramLayoutWork, diagramLayoutFrame, diagramLayoutLimits } from "./📊️job-types/🟦️.ts";
import { DiagramPagedStore, finiteLayoutValue, optionalFiniteLayoutValue } from "./🗃️paged-store/🟦️.ts";
import type { DiagramLayoutOwnedResult } from "./📢️publication/🟦️.ts";
import { DiagramLayoutJob } from "./⚙️job/🟦️.ts";

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