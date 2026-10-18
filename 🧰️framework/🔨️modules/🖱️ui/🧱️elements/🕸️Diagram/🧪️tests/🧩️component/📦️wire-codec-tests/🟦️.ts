import { describe, it, expect } from "vitest";
import { diagramLayoutUtf8Bytes, diagramLayoutNodeWireBytes, DIAGRAM_LAYOUT_MAX_NODE_BYTES, diagramLayoutEdgeWireBytes, DIAGRAM_LAYOUT_MAX_EDGE_BYTES, DIAGRAM_LAYOUT_CODEC_KIND, type DiagramLayoutNodeWire, DIAGRAM_LAYOUT_INGRESS_BYTES, diagramLayoutCredits, type DiagramLayoutDescriptor, type DiagramLayoutEdgeWire } from "../../../📐️layout/📜️wire/🟦️.ts";
import { createDiagramLayoutWorkerJob } from "../../../📐️layout/🟦️.ts";
import { createDiagramLayoutPublication } from "../../../📐️layout/📢️publication/🟦️.ts";
import { captureLayoutPublication, layoutNodes } from "../🧰️harness/🟦️.ts";

describe("owned Diagram layout wire codec", () => {
  it("accounts UTF-8 exactly and rejects overlong ids before ingress", () => {
    expect(diagramLayoutUtf8Bytes("aä😀")).toBe(7);
    expect(diagramLayoutUtf8Bytes("\ud800")).toBe(3);
    expect(diagramLayoutUtf8Bytes("😀".repeat(512))).toBe(2_048);
    expect(() => diagramLayoutUtf8Bytes("😀".repeat(513))).toThrow("512 Unicode");
    expect(diagramLayoutNodeWireBytes({ id: "😀".repeat(512), index: 0 })).toBe(DIAGRAM_LAYOUT_MAX_NODE_BYTES);
    expect(diagramLayoutEdgeWireBytes({ id: "😀".repeat(512), index: 0, source: "😀".repeat(512), target: "😀".repeat(512) })).toBe(DIAGRAM_LAYOUT_MAX_EDGE_BYTES);
    const job = createDiagramLayoutWorkerJob({ edgeCount: 0, generation: 41, kind: DIAGRAM_LAYOUT_CODEC_KIND, nodeCount: 1, options: {} });
    const value: DiagramLayoutNodeWire = { id: "x".repeat(512), index: 0 };
    expect(job.ingest({ bytes: DIAGRAM_LAYOUT_INGRESS_BYTES + 1, generation: 41, kind: "nodes", offset: 0, values: [value] })).toBe(false);
    expect(job.status).toBe("fault");
  });

  it("admits exact count and reservation boundaries without scanning source identities", () => {
    expect(diagramLayoutCredits(65_536, 0)).toEqual({ admitted: true, inputBytes: 65_536 * DIAGRAM_LAYOUT_MAX_NODE_BYTES, inputItems: 65_536, outputBytes: 65_536 * 32, outputItems: 65_536 });
    expect(diagramLayoutCredits(65_536, 1)).toEqual({ admitted: false, reason: "items" });
    expect(diagramLayoutCredits(65_537, 0)).toEqual({ admitted: false, reason: "items" });
    const edgeBoundary = Math.floor((256 * 1024 * 1024) / DIAGRAM_LAYOUT_MAX_EDGE_BYTES);
    expect(diagramLayoutCredits(0, edgeBoundary).admitted).toBe(true);
    expect(diagramLayoutCredits(0, edgeBoundary + 1)).toEqual({ admitted: false, reason: "bytes" });
    expect(createDiagramLayoutWorkerJob({ edgeCount: 65_536, generation: 72, kind: DIAGRAM_LAYOUT_CODEC_KIND, nodeCount: 1, options: {} }).status).toBe("fault");
  });

  it("rejects duplicate, hole, out-of-order, and early-complete output pages", () => {
    const nodes = layoutNodes(["a", "b", "c"]);
    const malformed = [
      { complete: false, sequence: 1, values: [{ index: 0, x: 1, y: 1 }, { index: 0, x: 2, y: 2 }] },
      { complete: false, sequence: 1, values: [{ index: 0, x: 1, y: 1 }, { index: 2, x: 2, y: 2 }] },
      { complete: false, sequence: 2, values: [{ index: 0, x: 1, y: 1 }] },
      { complete: true, sequence: 1, values: [{ index: 0, x: 1, y: 1 }] },
    ] as const;
    for (const payload of malformed) {
      const publication = createDiagramLayoutPublication(nodes, [], {}, 71);
      captureLayoutPublication(publication, nodes.length);
      expect(publication.acceptOutputPage({ byteLength: payload.values.length * 32, complete: payload.complete, itemCount: payload.values.length, payload: { generation: 71, kind: "positions", ...payload } })).toBe(false);
      expect(publication.acceptTerminal({ generation: 71, kind: "terminal", status: "complete" })).toBeUndefined();
      let turns = 0;
      while (!publication.closeStep() && turns < 100) turns += 1;
      expect(turns).toBeGreaterThan(0);
      expect(publication.terminalIsEmpty()).toBe(true);
    }
    for (const page of [
      { byteLength: 0, complete: false, itemCount: 0, payload: { complete: false, generation: 71, kind: "positions", sequence: 1, values: [] } },
      { byteLength: 0, complete: false, itemCount: 0, payload: null },
    ]) {
      const publication = createDiagramLayoutPublication(nodes, [], {}, 71);
      captureLayoutPublication(publication, nodes.length);
      expect(() => publication.acceptOutputPage(page)).not.toThrow();
      expect(publication.acceptOutputPage(page)).toBe(false);
    }
  });

  it("closes cancelled captured publication pages in finite cursor turns", () => {
    const nodes = layoutNodes(Array.from({ length: 400 }, (_, index) => `n-${index}`));
    const publication = createDiagramLayoutPublication(nodes, [], {}, 72);
    captureLayoutPublication(publication, nodes.length);
    expect(publication.acceptTerminal({ generation: 72, kind: "terminal", status: "cancelled" })).toBeUndefined();
    let turns = 0;
    while (!publication.closeStep() && turns < 100) turns += 1;
    expect(turns).toBeGreaterThan(4);
    expect(turns).toBeLessThan(100);
    expect(publication.terminalIsEmpty()).toBe(true);
  });

  it("emits and accepts one terminal-complete empty position page for zero nodes", () => {
    const job = createDiagramLayoutWorkerJob({ edgeCount: 0, generation: 73, kind: DIAGRAM_LAYOUT_CODEC_KIND, nodeCount: 0, options: {} });
    expect(job.ingest({ generation: 73, kind: "seal" })).toBe(true);
    let step = job.step({ deadline: performance.now() + 1_000, fuel: 1, generation: 73 });
    for (let turn = 0; step.status === "running" && turn < 100; turn++) step = job.step({ deadline: performance.now() + 1_000, fuel: 1, generation: 73 });
    expect(step.status).toBe("complete");
    expect(job.terminal()).toBeUndefined();
    expect(job.takeResultPage()).toEqual({ complete: true, generation: 73, kind: "positions", sequence: 1, values: [] });
    expect(job.takeResultPage()).toBeUndefined();
    expect(job.terminal()).toEqual({ generation: 73, kind: "terminal", status: "complete" });

    const publication = createDiagramLayoutPublication([], [], {}, 73);
    expect(publication.readInputPage(0, 64).complete).toBe(true);
    expect(publication.acceptOutputPage({ byteLength: 0, complete: true, itemCount: 0, payload: { complete: true, generation: 73, kind: "positions", sequence: 1, values: [] } })).toBe(true);
    const result = publication.acceptTerminal({ generation: 73, kind: "terminal", status: "complete" })!;
    expect(result.nodes).toHaveLength(0);
    expect(result.edges).toHaveLength(0);
    expect(result.closeStep()).toBe(true);
    while (!publication.closeStep()) {}
    expect(publication.terminalIsEmpty()).toBe(true);
  });

  it("admits maximal UTF-8 node and edge records under exact page and aggregate credits", () => {
    const maximal = "😀".repeat(512);
    const nodes = [{ data: {}, id: maximal, position: { x: 0, y: 0 } }, { data: {}, id: `${maximal.slice(0, -2)}😁`, position: { x: 0, y: 0 } }];
    const edges = [{ id: maximal, source: nodes[0]!.id, target: nodes[1]!.id }];
    const credits = diagramLayoutCredits(nodes.length, edges.length);
    expect(credits).toEqual({ admitted: true, inputBytes: DIAGRAM_LAYOUT_MAX_NODE_BYTES * 2 + DIAGRAM_LAYOUT_MAX_EDGE_BYTES, inputItems: 3, outputBytes: 64, outputItems: 2 });
    const publication = createDiagramLayoutPublication(nodes, edges, {}, 74);
    const nodePage = publication.readInputPage(0, 64);
    const edgePage = publication.readInputPage(2, 64);
    expect(nodePage.byteLength).toBe(DIAGRAM_LAYOUT_MAX_NODE_BYTES * 2);
    expect(edgePage.byteLength).toBe(DIAGRAM_LAYOUT_MAX_EDGE_BYTES);
    expect(nodePage.byteLength).toBeLessThanOrEqual(DIAGRAM_LAYOUT_INGRESS_BYTES);
    expect(edgePage.byteLength).toBeLessThanOrEqual(DIAGRAM_LAYOUT_INGRESS_BYTES);
  });

  it("owns malformed numeric and identity faults without throwing after partial ingress", () => {
    const malformed: DiagramLayoutDescriptor[] = [
      { edgeCount: 0, generation: Number.NaN, kind: DIAGRAM_LAYOUT_CODEC_KIND, nodeCount: 0, options: {} },
      { edgeCount: 0, generation: 1, kind: DIAGRAM_LAYOUT_CODEC_KIND, nodeCount: 0.5, options: {} },
      { edgeCount: Number.POSITIVE_INFINITY, generation: 1, kind: DIAGRAM_LAYOUT_CODEC_KIND, nodeCount: 0, options: {} },
    ];
    for (const descriptor of malformed)
      expect(createDiagramLayoutWorkerJob(descriptor).status).toBe("fault");

    const generation = 42;
    const nodeJob = createDiagramLayoutWorkerJob({ edgeCount: 0, generation, kind: DIAGRAM_LAYOUT_CODEC_KIND, nodeCount: 2, options: {} });
    const valid: DiagramLayoutNodeWire = { id: "valid", index: 0 };
    expect(nodeJob.ingest({ bytes: diagramLayoutNodeWireBytes(valid), generation, kind: "nodes", offset: 0, values: [valid] })).toBe(true);
    expect(() => nodeJob.ingest({ bytes: 64, generation, kind: "nodes", offset: 1, values: [{ id: "x".repeat(513), index: 1 }] })).not.toThrow();
    expect(nodeJob.status).toBe("fault");
    expect(nodeJob.close({ deadline: performance.now() + 1_000, fuel: 0 })).toBe(false);
    while (!nodeJob.close({ deadline: performance.now() + 1_000, fuel: 1 })) {}

    const edgeJob = createDiagramLayoutWorkerJob({ edgeCount: 1, generation, kind: DIAGRAM_LAYOUT_CODEC_KIND, nodeCount: 0, options: {} });
    expect(() => edgeJob.ingest({ bytes: 64, generation, kind: "edges", offset: 0, values: [{ id: "edge", index: Number.NaN, source: "a", target: "b" }] })).not.toThrow();
    expect(edgeJob.status).toBe("fault");
    const numericPageJob = createDiagramLayoutWorkerJob({ edgeCount: 0, generation, kind: DIAGRAM_LAYOUT_CODEC_KIND, nodeCount: 0, options: {} });
    expect(numericPageJob.ingest({ bytes: 0.5, generation, kind: "nodes", offset: 0, values: [] })).toBe(false);
    expect(numericPageJob.status).toBe("fault");
  });

  it("faults every hostile page shape without throwing, constructing, or partially committing", () => {
    const throwingGeneration = Object.defineProperty({}, "generation", { get: () => { throw new Error("generation"); } });
    const throwingValues = Object.defineProperties({}, {
      generation: { value: 75 },
      kind: { value: "nodes" },
      offset: { value: 0 },
      bytes: { value: 0 },
      values: { get: () => { throw new Error("values"); } },
    });
    const throwingLength = new Proxy([], { get: (target, property, receiver) => { if (property === "length") throw new Error("length"); return Reflect.get(target, property, receiver); } });
    const invalidZeroPages: unknown[] = [
      null,
      undefined,
      false,
      0,
      "page",
      Symbol("page"),
      [],
      () => {},
      Object.create(null),
      { generation: 75, kind: "nodes", offset: 0, bytes: 0 },
      { generation: 75, kind: "nodes", offset: 0, bytes: 0, values: null },
      { generation: 75, kind: "nodes", offset: 0, bytes: 0, values: {} },
      { generation: 75, kind: "nodes", offset: 0, bytes: 0, values: "" },
      { generation: 75, kind: "nodes", offset: 0, bytes: 0, values: new Uint8Array() },
      { generation: 75, kind: "nodes", offset: 0, bytes: 0, complete: null, values: [] },
      { generation: 75, kind: "unknown", offset: 0, bytes: 0, values: [] },
      { generation: 75, kind: "nodes", offset: 0, bytes: 0, values: throwingLength },
      throwingGeneration,
      throwingValues,
    ];
    for (const page of invalidZeroPages) {
      const job = createDiagramLayoutWorkerJob({ edgeCount: 0, generation: 75, kind: DIAGRAM_LAYOUT_CODEC_KIND, nodeCount: 0, options: {} });
      let accepted = true;
      expect(() => { accepted = job.ingest(page); }).not.toThrow();
      expect(accepted).toBe(false);
      expect(job.status).toBe("fault");
      expect(job.takeResultPage()).toBeUndefined();
      expect(job.close({ deadline: performance.now() + 1_000, fuel: 1 })).toBe(true);
    }

    for (const value of [null, undefined, false, 1, "node", []]) {
      const job = createDiagramLayoutWorkerJob({ edgeCount: 0, generation: 75, kind: DIAGRAM_LAYOUT_CODEC_KIND, nodeCount: 1, options: {} });
      expect(() => job.ingest({ bytes: 0, generation: 75, kind: "nodes", offset: 0, values: [value] })).not.toThrow();
      expect(job.status).toBe("fault");
      expect(job.close({ deadline: performance.now() + 1_000, fuel: 1 })).toBe(true);
    }

    const premature = createDiagramLayoutWorkerJob({ edgeCount: 0, generation: 75, kind: DIAGRAM_LAYOUT_CODEC_KIND, nodeCount: 2, options: {} });
    expect(premature.ingest({ bytes: 65, complete: true, generation: 75, kind: "nodes", offset: 0, values: [{ id: "a", index: 0 }] })).toBe(false);
    expect(premature.status).toBe("fault");
    expect(premature.close({ deadline: performance.now() + 1_000, fuel: 1 })).toBe(true);

    const throwingNode = new Proxy({ id: "b", index: 1 }, { get: (target, property, receiver) => { if (property === "id") throw new Error("node id"); return Reflect.get(target, property, receiver); } });
    const nodeJob = createDiagramLayoutWorkerJob({ edgeCount: 0, generation: 76, kind: DIAGRAM_LAYOUT_CODEC_KIND, nodeCount: 2, options: {} });
    expect(() => nodeJob.ingest({ bytes: 130, generation: 76, kind: "nodes", offset: 0, values: [{ id: "a", index: 0 }, throwingNode] })).not.toThrow();
    expect(nodeJob.status).toBe("fault");
    expect(nodeJob.close({ deadline: performance.now() + 1_000, fuel: 1 })).toBe(true);

    const throwingEdge = new Proxy({ id: "e-1", index: 1, source: "a", target: "b" }, { get: (target, property, receiver) => { if (property === "target") throw new Error("edge target"); return Reflect.get(target, property, receiver); } });
    const edgeJob = createDiagramLayoutWorkerJob({ edgeCount: 2, generation: 77, kind: DIAGRAM_LAYOUT_CODEC_KIND, nodeCount: 0, options: {} });
    expect(() => edgeJob.ingest({ bytes: 138, generation: 77, kind: "edges", offset: 0, values: [{ id: "e-0", index: 0, source: "a", target: "b" }, throwingEdge] })).not.toThrow();
    expect(edgeJob.status).toBe("fault");
    expect(edgeJob.close({ deadline: performance.now() + 1_000, fuel: 1 })).toBe(true);
  });

  it("keeps concrete jobs and batch adapters absent from the Diagram product barrel", async () => {
    const product = await import("../../../🟦️.tsx");
    expect("DiagramLayoutJob" in product).toBe(false);
    expect("DiagramLayoutWireJob" in product).toBe(false);
    expect("createDiagramLayoutWorkerJob" in product).toBe(false);
    expect("calculateDiagramLayoutForBatchTest" in product).toBe(false);
  });

  it("runs bounded pages through the same job and emits fixed-width result pages", () => {
    const count = 400;
    const job = createDiagramLayoutWorkerJob({ edgeCount: count - 1, generation: 43, kind: DIAGRAM_LAYOUT_CODEC_KIND, nodeCount: count, options: { direction: "LR" } });
    for (let offset = 0; offset < count; offset += 64) {
      const values: DiagramLayoutNodeWire[] = Array.from({ length: Math.min(64, count - offset) }, (_, index) => ({ height: 30 + ((offset + index) % 3), id: `n-${(offset + index).toString().padStart(4, "0")}`, index: offset + index, width: 60 }));
      expect(job.ingest({ bytes: values.reduce((sum, value) => sum + diagramLayoutNodeWireBytes(value), 0), generation: 43, kind: "nodes", offset, values })).toBe(true);
    }
    for (let offset = 0; offset < count - 1; offset += 64) {
      const values: DiagramLayoutEdgeWire[] = Array.from({ length: Math.min(64, count - 1 - offset) }, (_, index) => {
        const source = offset + index;
        return { id: `e-${source}`, index: source, source: `n-${source.toString().padStart(4, "0")}`, target: `n-${(source + 1).toString().padStart(4, "0")}` };
      });
      expect(job.ingest({ bytes: values.reduce((sum, value) => sum + diagramLayoutEdgeWireBytes(value), 0), generation: 43, kind: "edges", offset, values })).toBe(true);
    }
    expect(job.ingest({ generation: 43, kind: "seal" })).toBe(true);
    let step = job.step({ deadline: performance.now() + 1_000, fuel: 97, generation: 43 });
    const positions: Array<{ index: number; x: number; y: number }> = [];
    for (let turn = 0; step.status === "running" && turn < 1_000_000; turn++) {
      const page = job.takeResultPage();
      if (page) {
        expect(page.values.length).toBeLessThanOrEqual(128);
        positions.push(...page.values);
      }
      step = job.step({ deadline: performance.now() + 1_000, fuel: 97, generation: 43 });
    }
    expect(step.status).toBe("complete");
    expect(positions).toHaveLength(count);
    expect(positions[0]!.x).toBeLessThan(positions.at(-1)!.x);
    expect(job.close({ deadline: performance.now() + 1_000, fuel: 0 })).toBe(false);
    while (!job.close({ deadline: performance.now() + 1_000, fuel: 4_096 })) {}
    expect(job.terminal()?.status).toBe("complete");
  });

  it("admits and cancels a 20k/20k graph under aggregate item and byte caps", () => {
    const count = 20_000;
    const job = createDiagramLayoutWorkerJob({ edgeCount: count, generation: 47, kind: DIAGRAM_LAYOUT_CODEC_KIND, nodeCount: count, options: {} });
    for (let offset = 0; offset < count; offset += 64) {
      const values: DiagramLayoutNodeWire[] = Array.from({ length: Math.min(64, count - offset) }, (_, index) => ({ id: `n-${(offset + index).toString().padStart(5, "0")}`, index: offset + index }));
      const bytes = values.reduce((sum, value) => sum + diagramLayoutNodeWireBytes(value), 0);
      expect(bytes).toBeLessThanOrEqual(DIAGRAM_LAYOUT_INGRESS_BYTES);
      expect(job.ingest({ bytes, generation: 47, kind: "nodes", offset, values })).toBe(true);
    }
    for (let offset = 0; offset < count; offset += 64) {
      const values: DiagramLayoutEdgeWire[] = Array.from({ length: Math.min(64, count - offset) }, (_, index) => {
        const source = offset + index;
        return { id: `e-${source}`, index: source, source: `n-${source.toString().padStart(5, "0")}`, target: `n-${((source + 1) % count).toString().padStart(5, "0")}` };
      });
      const bytes = values.reduce((sum, value) => sum + diagramLayoutEdgeWireBytes(value), 0);
      expect(bytes).toBeLessThanOrEqual(DIAGRAM_LAYOUT_INGRESS_BYTES);
      expect(job.ingest({ bytes, generation: 47, kind: "edges", offset, values })).toBe(true);
    }
    expect(job.ingest({ generation: 47, kind: "seal" })).toBe(true);
    expect(job.step({ deadline: performance.now() + 1_000, fuel: 1, generation: 47 }).consumed).toBe(1);
    job.cancel(47);
    expect(job.step({ deadline: performance.now() + 1_000, fuel: 10, generation: 47 }).status).toBe("cancelled");
    while (!job.close({ deadline: performance.now() + 1_000, fuel: 16_384 })) {}
  });
});
