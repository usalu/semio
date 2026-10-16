/** 🧵️ Backbone uris and workflow planning: classifying backbone uris and planning os workflows over media contracts.
 *
 * @see ../🛠️backbone-wire-helpers/🟦️.ts — the shared helpers
 * @see ../../🟦️.ts — the package entry the suite reads fixtures relative to
 */
import { describe, expect, it } from "vitest";
import { backboneKindFromUri, buildFileBackboneUri, buildFolderBackboneUri, buildFrameworkSyncUtilities, buildRemoteBackboneUri, decodeBackboneMessage, decodeDocumentPackSnapshot, encodeBackboneMessage, encodeDocumentPackBundle, parseRemoteBackboneUri, planWorkflow } from "@semio-tech/framework/backbone";
import { source } from "../🛠️backbone-wire-helpers/🟦️.ts";

type BinaryBackboneMessage = import("@semio-tech/framework/backbone").BinaryBackboneMessage;

type MediaContract = import("@semio-tech/framework/backbone").MediaContract;

type OsWorkflow = import("@semio-tech/framework/backbone").OsWorkflow;

type OsWorkflowNode = import("@semio-tech/framework/backbone").OsWorkflowNode;

describe("@semio-tech/framework-os backbone", () => {
  it("classifies backbone uri kinds", () => {
    expect(backboneKindFromUri("file:///tmp/a.json")).toBe("file");
    expect(backboneKindFromUri("folder:///tmp")).toBe("folder");
    expect(backboneKindFromUri("remote://host:1234/doc-1")).toBe("remote");
    expect(backboneKindFromUri("other://x")).toBe("unknown");
  });

  it("builds and parses backbone uris", () => {
    expect(buildFileBackboneUri("tmp/a.json")).toBe("file:///tmp/a.json");
    expect(buildFolderBackboneUri("tmp")).toBe("folder:///tmp");
    expect(buildRemoteBackboneUri("localhost:1234", "studio-1", "doc-1")).toBe("remote://localhost:1234/studio-1/doc-1");
    expect(parseRemoteBackboneUri("remote://localhost:1234/studio-1/doc-1")).toEqual({ hostPort: "localhost:1234", spaceId: "studio-1", documentId: "doc-1" });
    expect(parseRemoteBackboneUri("remote://localhost:1234/doc-1")).toBeNull();
    expect(parseRemoteBackboneUri("file:///tmp/a.json")).toBeNull();
  });

  it("packs and unpacks document bundles", () => {
    const bundle = encodeDocumentPackBundle({ nodes: [] });
    expect(decodeDocumentPackSnapshot(bundle)).toEqual({ nodes: [] });
  });

  it("round-trips backbone genesis messages", () => {
    const message: BinaryBackboneMessage = { kind: "genesis", pack: new Uint8Array([1, 2]) };
    const round = decodeBackboneMessage(encodeBackboneMessage(message));
    expect(round.kind).toBe("genesis");
    if (round.kind !== "genesis") return;
    expect(Array.from(round.pack)).toEqual([1, 2]);
  });

  it("encodes genesis exactly like the Rust store's OpBinary", () => {
    expect(Array.from(encodeBackboneMessage({ kind: "genesis", pack: Uint8Array.of(0xaa) }))).toEqual([1, 0, 0, 1, 0, 8, 1, 0xaa]);
  });

  it("throws on an unknown backbone message tag", () => {
    expect(() => decodeBackboneMessage(new Uint8Array([99]))).toThrow("invalid format");
  });

  it("builds sync utilities reflecting the active backbone kind", () => {
    const utilities = buildFrameworkSyncUtilities("folder:///tmp");
    expect(utilities.map((utility) => utility.id)).toEqual(["framework.sync.file", "framework.sync.folder", "framework.sync.remote"]);
    expect(utilities.find((utility) => utility.id === "framework.sync.folder")?.pressed).toBe(true);
    expect(utilities.find((utility) => utility.id === "framework.sync.file")?.pressed).toBe(false);
  });
});

describe("@semio-tech/framework-os workflow", () => {
  const mediaContract = (): MediaContract => ({ kindId: "2d.drawing", mediaType: { class: "data", form: "value" }, wire: { kind: "document", schema: "2d.drawing" } });
  const mediaNode = (id: string, instanceId: string): OsWorkflowNode => ({
    id,
    instanceId,
    x: 0,
    y: 0,
    width: 160,
    height: 72,
    inputs: [{ id: `${instanceId}:in`, artifactKind: "2d.drawing", direction: "in" }],
    outputs: [{ id: `${instanceId}:out`, artifactKind: "2d.drawing", direction: "out" }],
  });

  it("plans a single delivery across one dirty edge", () => {
    const graph: OsWorkflow = {
      schema: "os.workflow",
      nodes: [mediaNode("node-1", "app-1"), mediaNode("node-2", "app-2")],
      edges: [{ id: "edge-1", sourceNodeId: "node-1", sourcePortId: "app-1:out", targetNodeId: "node-2", targetPortId: "app-2:in", contract: mediaContract() }],
    };
    const deliveries = planWorkflow(graph, new Set(["app-1"]));
    expect(deliveries).toEqual([{ edgeId: "edge-1", producerInstanceId: "app-1", producerPortId: "app-1:out", consumerInstanceId: "app-2", consumerPortId: "app-2:in" }]);
  });

  it("plans a chain in topological order when only the root is dirty", () => {
    const graph: OsWorkflow = {
      schema: "os.workflow",
      nodes: [mediaNode("node-1", "app-1"), mediaNode("node-2", "app-2"), mediaNode("node-3", "app-3")],
      edges: [
        { id: "edge-ab", sourceNodeId: "node-1", sourcePortId: "app-1:out", targetNodeId: "node-2", targetPortId: "app-2:in", contract: mediaContract() },
        { id: "edge-bc", sourceNodeId: "node-2", sourcePortId: "app-2:out", targetNodeId: "node-3", targetPortId: "app-3:in", contract: mediaContract() },
      ],
    };
    const deliveries = planWorkflow(graph, new Set(["app-1"]));
    expect(deliveries.map((delivery) => delivery.edgeId)).toEqual(["edge-ab", "edge-bc"]);
  });

  it("plans a diamond with one delivery per incoming edge", () => {
    const graph: OsWorkflow = {
      schema: "os.workflow",
      nodes: [mediaNode("node-1", "app-a"), mediaNode("node-2", "app-b"), mediaNode("node-3", "app-c"), mediaNode("node-4", "app-d")],
      edges: [
        { id: "edge-ab", sourceNodeId: "node-1", sourcePortId: "app-a:out", targetNodeId: "node-2", targetPortId: "app-b:in", contract: mediaContract() },
        { id: "edge-ac", sourceNodeId: "node-1", sourcePortId: "app-a:out", targetNodeId: "node-3", targetPortId: "app-c:in", contract: mediaContract() },
        { id: "edge-bd", sourceNodeId: "node-2", sourcePortId: "app-b:out", targetNodeId: "node-4", targetPortId: "app-d:in", contract: mediaContract() },
        { id: "edge-cd", sourceNodeId: "node-3", sourcePortId: "app-c:out", targetNodeId: "node-4", targetPortId: "app-d:in", contract: mediaContract() },
      ],
    };
    const deliveries = planWorkflow(graph, new Set(["app-a"]));
    const edgeIds = deliveries.map((delivery) => delivery.edgeId);
    expect(edgeIds).toHaveLength(4);
    expect(edgeIds.indexOf("edge-bd")).toBeGreaterThan(edgeIds.indexOf("edge-ab"));
    expect(edgeIds.indexOf("edge-cd")).toBeGreaterThan(edgeIds.indexOf("edge-ac"));
  });

  it("plans nothing when no instance is dirty", () => {
    const graph: OsWorkflow = {
      schema: "os.workflow",
      nodes: [mediaNode("node-1", "app-1"), mediaNode("node-2", "app-2")],
      edges: [{ id: "edge-1", sourceNodeId: "node-1", sourcePortId: "app-1:out", targetNodeId: "node-2", targetPortId: "app-2:in", contract: mediaContract() }],
    };
    expect(planWorkflow(graph, new Set())).toEqual([]);
  });

  it("plans nothing for a dirty node with no outgoing edges", () => {
    const graph: OsWorkflow = { schema: "os.workflow", nodes: [mediaNode("node-1", "app-1")], edges: [] };
    expect(planWorkflow(graph, new Set(["app-1"]))).toEqual([]);
  });

  it("replays shared plain planner witnesses against the graphology reachability oracle", async () => {
    const { readdirSync, readFileSync } = await import("node:fs");
    const { fileURLToPath } = await import("node:url");
    const { dirname, join } = await import("node:path");
    const { DirectedGraph } = await import("graphology");
    const fixturesDir = join(dirname(fileURLToPath(source.url)), "🧪️tests/🧫️fixtures");
    const paths = readdirSync(fixturesDir, { withFileTypes: true }).filter((entry) => entry.isDirectory()).map((entry) => join(fixturesDir, entry.name, "🔣️.json")).filter((path) => { try { return JSON.parse(readFileSync(path, "utf8")).expectedDeliveries !== undefined; } catch { return false; } });
    expect(paths).toHaveLength(5);
    for (const path of paths) {
      const witness = JSON.parse(readFileSync(path, "utf8")) as { graph: { schema: string; nodes: { id: string; x: number; y: number; width: number; height: number }[]; edges: OsWorkflow["edges"] }; dirtyNodeIds: string[]; expectedDeliveries: { edgeId: string; producerNodeId: string; producerPortId: string; consumerNodeId: string; consumerPortId: string }[] };
      const graph: OsWorkflow = { ...witness.graph, nodes: witness.graph.nodes.map((node) => ({ ...node, instanceId: node.id, inputs: [], outputs: [] })) };
      const expected = witness.expectedDeliveries.map(({ producerNodeId, consumerNodeId, ...delivery }) => ({ ...delivery, producerInstanceId: producerNodeId, consumerInstanceId: consumerNodeId }));
      const actual = planWorkflow(graph, new Set(witness.dirtyNodeIds));
      expect(actual).toEqual(expected);
      const oracle = new DirectedGraph();
      for (const node of graph.nodes) oracle.addNode(node.id);
      for (const edge of graph.edges) oracle.addDirectedEdgeWithKey(edge.id, edge.sourceNodeId, edge.targetNodeId);
      const reached = new Set(witness.dirtyNodeIds);
      const queue = [...reached];
      for (let i = 0; i < queue.length; i++) for (const target of oracle.outNeighbors(queue[i]!)) if (!reached.has(target)) { reached.add(target); queue.push(target); }
      const oracleEdges = oracle.edges().filter((id) => reached.has(oracle.source(id)));
      expect(actual.map((delivery) => delivery.edgeId).sort()).toEqual(oracleEdges.sort());
    }
  });
});
