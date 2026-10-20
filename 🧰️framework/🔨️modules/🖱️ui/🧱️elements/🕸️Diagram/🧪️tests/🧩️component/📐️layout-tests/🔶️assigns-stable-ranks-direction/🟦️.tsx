/** 🔶️ The assigns stable ranks direction tests of the owned Diagram directed layout suite. */
import { it, expect } from "vitest";
import { type DiagramLayoutDirection } from "../../../../📐️layout/📜️wire/🟦️.ts";
import { calculateDiagramLayoutForBatchTest, createDiagramLayoutBatchTestJob } from "../../../../📐️layout/🟦️.ts";
import { useDiagramLayout } from "../../../../🪢️layout-hook/🟦️.ts";
import { render } from "@testing-library/react";
import { installFrames, largeDiagramEdges, largeDiagramNodes, layoutNodes, layoutPositions, runLayoutJob, trackArrayReads } from "../../🧰️harness/🟦️.ts";

/** 🔷 Registers the assigns stable ranks direction tests of the owned Diagram directed layout suite. */
export function registerAssignsStableRanksDirectionTests(): void {
  it("assigns stable ranks in every direction and respects variable node dimensions", () => {
    const nodes = [
      { data: {}, height: 40, id: "a", position: { x: 0, y: 0 }, width: 120 },
      { data: {}, height: 80, id: "b", position: { x: 0, y: 0 }, width: 60 },
      { data: {}, height: 30, id: "c", position: { x: 0, y: 0 }, width: 90 },
    ];
    const edges = [
      { id: "ab", source: "a", target: "b" },
      { id: "bc", source: "b", target: "c" },
    ];
    for (const direction of ["TB", "BT", "LR", "RL"] satisfies DiagramLayoutDirection[]) {
      const result = calculateDiagramLayoutForBatchTest(nodes, edges, { direction, nodeSep: 25, rankSep: 30 });
      const [a, b, c] = result.nodes;
      if (direction === "TB") expect(a!.position.y).toBeLessThan(b!.position.y);
      if (direction === "BT") expect(a!.position.y).toBeGreaterThan(b!.position.y);
      if (direction === "LR") expect(a!.position.x).toBeLessThan(b!.position.x);
      if (direction === "RL") expect(a!.position.x).toBeGreaterThan(b!.position.x);
      expect(new Set(result.nodes.map((node) => `${node.position.x}:${node.position.y}`)).size).toBe(3);
      expect(c!.position).not.toEqual(b!.position);
    }
    const siblings = calculateDiagramLayoutForBatchTest(
      [
        { data: {}, height: 50, id: "wide", position: { x: 0, y: 0 }, width: 200 },
        { data: {}, height: 50, id: "narrow", position: { x: 0, y: 0 }, width: 40 },
      ],
      [],
      { nodeSep: 20 },
    ).nodes;
    const wide = siblings.find((node) => node.id === "wide")!;
    const narrow = siblings.find((node) => node.id === "narrow")!;
    expect(narrow.position.x + 40 + 20).toBeLessThanOrEqual(wide.position.x);
  });
  it("handles cycles, parallel and self edges, and disconnected components without losing nodes", () => {
    const nodes = layoutNodes(["a", "b", "c", "island"]);
    const edges = [
      { id: "ab-1", source: "a", target: "b" },
      { id: "ab-2", source: "a", target: "b" },
      { id: "ba", source: "b", target: "a" },
      { id: "bc", source: "b", target: "c" },
      { id: "self", source: "c", target: "c" },
    ];
    const result = calculateDiagramLayoutForBatchTest(nodes, edges);
    expect(result.nodes.map((node) => node.id)).toEqual(nodes.map((node) => node.id));
    expect(result.edges.map(({ id, source, target }) => ({ id, source, target }))).toEqual(edges);
    expect(result.nodes.every((node) => Number.isFinite(node.position.x) && Number.isFinite(node.position.y))).toBe(true);
    expect(new Set(result.nodes.map((node) => `${node.position.x}:${node.position.y}`)).size).toBe(nodes.length);
  });
  it("is deterministic by identity under reversed node and edge input order", () => {
    const nodes = layoutNodes(["delta", "alpha", "charlie", "bravo"]);
    const edges = [
      { id: "ad", source: "alpha", target: "delta" },
      { id: "ab", source: "alpha", target: "bravo" },
      { id: "bc", source: "bravo", target: "charlie" },
    ];
    const forward = calculateDiagramLayoutForBatchTest(nodes, edges, { direction: "LR" });
    const reverse = calculateDiagramLayoutForBatchTest([...nodes].reverse(), [...edges].reverse(), { direction: "LR" });
    expect(layoutPositions(reverse.nodes)).toEqual(layoutPositions(forward.nodes));
  });
  it("uses the exact persistent job for batch results and emits only bounded replaceable previews", () => {
    const nodes = layoutNodes(Array.from({ length: 400 }, (_, index) => `n-${index.toString().padStart(4, "0")}`));
    const edges = nodes.slice(1).map((node, index) => ({ id: `e-${index}`, source: nodes[index]!.id, target: node.id }));
    const job = createDiagramLayoutBatchTestJob(nodes, edges, { direction: "RL" }, 19);
    let maxPreview = 0;
    let step = job.step({ deadline: performance.now() + 1_000, fuel: 11, generation: 19 });
    for (let index = 0; step.status === "running" && index < 1_000_000; index++) {
      maxPreview = Math.max(maxPreview, job.takePreview()?.positions.length ?? 0);
      step = job.step({ deadline: performance.now() + 1_000, fuel: 11, generation: 19 });
    }
    maxPreview = Math.max(maxPreview, job.takePreview()?.positions.length ?? 0);
    expect(step.status).toBe("complete");
    expect(maxPreview).toBeLessThanOrEqual(128);
    const owned = job.takeResult()!;
    const ownedNodes = Array.from({ length: owned.nodeCount }, (_, index) => owned.takeNode(index)!);
    while (!owned.closeStep()) {}
    expect(layoutPositions(ownedNodes)).toEqual(layoutPositions(calculateDiagramLayoutForBatchTest(nodes, edges, { direction: "RL" }).nodes));
  });
  it("retains the latest bounded preview window after more than two full publications", () => {
    const nodes = layoutNodes(Array.from({ length: 400 }, (_, index) => `n-${index.toString().padStart(4, "0")}`));
    const job = createDiagramLayoutBatchTestJob(nodes, [], {}, 21);
    expect(runLayoutJob(job, 127).status).toBe("complete");
    const preview = job.takePreview()!;
    expect(preview.positions).toHaveLength(128);
    expect(preview.positions.map(({ index }) => index)).toEqual(Array.from({ length: 128 }, (_, index) => index + 272));
    while (!job.close({ deadline: performance.now() + 1_000, fuel: 257 })) {}
  });
  it("captures 20k sources in O(1), respects fuel and deadlines, and cancels in O(1)", () => {
    const trackedNodes = trackArrayReads(largeDiagramNodes(20_000));
    const trackedEdges = trackArrayReads(largeDiagramEdges(20_000));
    const job = createDiagramLayoutBatchTestJob(trackedNodes.values, trackedEdges.values, {}, 23);
    expect(trackedNodes.read()).toBe(0);
    expect(trackedEdges.read()).toBe(0);
    expect(job.step({ deadline: performance.now() - 1, fuel: 100, generation: 23 }).consumed).toBe(0);
    expect(job.step({ deadline: performance.now() + 1_000, fuel: 1, generation: 23 }).consumed).toBe(1);
    expect(trackedNodes.read()).toBe(1);
    expect(trackedEdges.read()).toBe(0);
    job.cancel(23);
    const cancelled = job.step({ deadline: performance.now() + 1_000, fuel: 10_000, generation: 23 });
    expect(cancelled.status).toBe("cancelled");
    expect(cancelled.consumed).toBe(0);
  });
  it("copies each admitted source once and rejects stale generations before further admission", () => {
    const nodes = layoutNodes(["a", "b"]);
    const job = createDiagramLayoutBatchTestJob(nodes, [{ id: "ab", source: "a", target: "b" }], {}, 27);
    expect(job.step({ deadline: performance.now() + 1_000, fuel: 1, generation: 27 }).consumed).toBe(1);
    nodes[0]!.id = "mutated-after-admission";
    expect(runLayoutJob(job).status).toBe("complete");
    const result = job.takeResult()!;
    expect(result.takeNode(0)?.id).toBe("a");
    while (!result.closeStep()) {}

    const stale = createDiagramLayoutBatchTestJob(layoutNodes(["a", "b"]), [], {}, 28);
    expect(stale.step({ deadline: performance.now() + 1_000, fuel: 1, generation: 29 }).status).toBe("cancelled");
    expect(stale.step({ deadline: performance.now() + 1_000, fuel: 1, generation: 28 }).consumed).toBe(0);
  });
  it("keeps React render source capture O(1) and runs no layout work without the shared host port", () => {
    const frames = installFrames();
    const trackedNodes = trackArrayReads(largeDiagramNodes(20_000));
    const trackedEdges = trackArrayReads(largeDiagramEdges(20_000));
    const Probe = () => {
      const result = useDiagramLayout(trackedNodes.values, trackedEdges.values);
      return <span>{result.nodes.length}</span>;
    };
    const view = render(<Probe />);
    expect(trackedNodes.read()).toBe(0);
    expect(trackedEdges.read()).toBe(0);
    expect(frames.callbacks.size).toBe(0);
    expect(trackedNodes.read()).toBe(0);
    expect(trackedEdges.read()).toBe(0);
    view.unmount();
    expect(frames.callbacks.size).toBe(0);
  });
}
