import { describe, it, expect } from "vitest";
import { type DiagramLayoutDirection, DIAGRAM_LAYOUT_CODEC_KIND, DIAGRAM_LAYOUT_MAX_EDGE_BYTES } from "../../../📐️layout/📜️wire/🟦️.ts";
import { calculateDiagramLayoutForBatchTest, createDiagramLayoutBatchTestJob } from "../../../📐️layout/🟦️.ts";
import { useDiagramLayout } from "../../../🪢️layout-hook/🟦️.ts";
import { render, act } from "@testing-library/react";
import { type InteractiveJobPort, setInteractiveJobPort } from "../../../../🔌️Ports/📡️interactive-jobs/🟦️.ts";
import * as React from "react";
import { createTestInteractivePort, diagramFrameStack, flushPortNotifications, installFrames, largeDiagramEdges, largeDiagramNodes, layoutNodes, layoutPositions, runLayoutJob, trackArrayReads, setDiagramFrameStack } from "../🧰️harness/🟦️.ts";

describe("owned Diagram directed layout", () => {
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

  it("admits and publishes only through bounded shared-worker callbacks outside animation-frame stacks", () => {
    let consumer: Parameters<InteractiveJobPort["submit"]>[1] | undefined;
    let descriptor: Parameters<InteractiveJobPort["submit"]>[0] | undefined;
    let callbackInAnimationFrame = false;
    setInteractiveJobPort({
      status: "ready",
      getSnapshot: () => ({ revision: 1, status: "ready" }),
      observeConsumerTurn: () => true, reportConsumerFault: () => {},
      subscribe: () => () => {},
      submit(nextDescriptor, nextConsumer) {
        descriptor = nextDescriptor;
        consumer = nextConsumer;
        return { cancel: () => true, generation: nextDescriptor.generation, operation: nextDescriptor.operation };
      },
    });
    const nodes = layoutNodes(["a", "b"]);
    const edges = [{ id: "ab", source: "a", target: "b" }];
    const trackedNodes = trackArrayReads(nodes);
    const trackedEdges = trackArrayReads(edges);
    const Probe = () => {
      const result = useDiagramLayout(trackedNodes.values, trackedEdges.values, { direction: "LR" });
      return <span>{`${result.nodes[0]!.position.x}:${result.nodes[1]!.position.x}`}</span>;
    };
    const view = render(<Probe />);
    expect(trackedNodes.read()).toBe(2);
    expect(trackedEdges.read()).toBe(0);
    expect(descriptor?.kind).toBe(DIAGRAM_LAYOUT_CODEC_KIND);
    act(() => {
      setDiagramFrameStack(false);
      const first = consumer!.readInputPage(0, 64);
      callbackInAnimationFrame ||= diagramFrameStack;
      expect(first.itemCount).toBe(2);
      const second = consumer!.readInputPage(2, 64);
      callbackInAnimationFrame ||= diagramFrameStack;
      expect(second.itemCount).toBe(1);
      consumer!.onOutputPage({
        byteLength: 64,
        complete: true,
        itemCount: 2,
        payload: { complete: true, generation: descriptor!.generation, kind: "positions", sequence: 1, values: [{ index: 0, x: 10, y: 0 }, { index: 1, x: 90, y: 0 }] },
      });
      callbackInAnimationFrame ||= diagramFrameStack;
      consumer!.onTerminal({ generation: descriptor!.generation, operation: descriptor!.operation, status: "complete" });
      callbackInAnimationFrame ||= diagramFrameStack;
    });
    expect(callbackInAnimationFrame).toBe(false);
    expect(view.getByText("10:90")).toBeTruthy();
    view.unmount();
  });

  it("reacts to unavailable, ready, ready-port replacement, quarantine, and close", async () => {
    const unavailable = createTestInteractivePort("unavailable");
    setInteractiveJobPort(unavailable);
    const nodes = layoutNodes(["a"]);
    const Probe = () => <span>{useDiagramLayout(nodes, []).layoutStatus}</span>;
    const view = render(<Probe />);
    expect(view.getByText("source")).toBeTruthy();
    expect(unavailable.descriptors).toHaveLength(0);

    unavailable.transition("ready");
    await flushPortNotifications();
    expect(unavailable.descriptors).toHaveLength(1);
    expect(view.getByText("pending")).toBeTruthy();

    const replacement = createTestInteractivePort("ready");
    setInteractiveJobPort(replacement);
    await flushPortNotifications();
    expect(unavailable.cancellations.count).toBe(1);
    expect(replacement.descriptors).toHaveLength(1);

    replacement.transition("quarantined");
    await flushPortNotifications();
    expect(replacement.cancellations.count).toBe(1);
    expect(view.getByText("source")).toBeTruthy();

    const closing = createTestInteractivePort("ready");
    setInteractiveJobPort(closing);
    await flushPortNotifications();
    expect(closing.descriptors).toHaveLength(1);
    closing.transition("closed");
    await flushPortNotifications();
    expect(closing.cancellations.count).toBe(1);
    expect(view.getByText("source")).toBeTruthy();
    view.unmount();
  });

  it("publishes only after exact terminal-complete coverage and exposes explicit rejection", () => {
    const port = createTestInteractivePort("ready");
    setInteractiveJobPort(port);
    const nodes = layoutNodes(["a", "b"]);
    const emptyEdges: Array<{ id: string; source: string; target: string }> = [];
    const rejectedEdges = Array.from({ length: 65_535 }, (_, index) => ({ id: `e-${index}`, source: "a", target: "b" }));
    const byteRejectedEdges = rejectedEdges.slice(0, Math.floor((256 * 1024 * 1024) / DIAGRAM_LAYOUT_MAX_EDGE_BYTES) + 1);
    const Probe = ({ values = nodes, edges = emptyEdges }: { values?: ReturnType<typeof layoutNodes>; edges?: Array<{ id: string; source: string; target: string }> }) => {
      const result = useDiagramLayout(values, edges);
      return <span>{`${result.layoutStatus}:${result.layoutRejection ?? "none"}:${result.nodes[0]?.position.x ?? "empty"}`}</span>;
    };
    const view = render(<Probe />);
    const descriptor = port.descriptors[0]!;
    const consumer = port.consumers[0]!;
    consumer.readInputPage(0, 64);
    act(() => {
      consumer.onOutputPage({ byteLength: 64, complete: true, itemCount: 2, payload: { complete: true, generation: descriptor.generation, kind: "positions", sequence: 1, values: [{ index: 0, x: 11, y: 0 }, { index: 1, x: 22, y: 0 }] } });
    });
    expect(view.getByText("pending:none:0")).toBeTruthy();
    act(() => consumer.onTerminal({ generation: descriptor.generation, operation: descriptor.operation, status: "complete" }));
    expect(view.getByText("complete:none:11")).toBeTruthy();
    let closeTurns = 0;
    while (!consumer.closeStep() && closeTurns < 100) closeTurns += 1;
    expect(closeTurns).toBeGreaterThan(0);
    expect(consumer.terminalIsEmpty()).toBe(true);
    expect(view.getByText("complete:none:11")).toBeTruthy();
    view.rerender(<Probe edges={rejectedEdges} />);
    expect(view.getByText("rejected:items:0")).toBeTruthy();
    view.rerender(<Probe edges={byteRejectedEdges} values={[]} />);
    expect(view.getByText("rejected:bytes:empty")).toBeTruthy();
    view.unmount();
  });

  it("retains the committed proxy through a suspended concurrent replacement and retires it only after commit", async () => {
    const firstPort = createTestInteractivePort("ready");
    setInteractiveJobPort(firstPort);
    const nodes = layoutNodes(["a"]);
    const edges: Array<{ id: string; source: string; target: string }> = [];
    let releaseReplacement = () => {};
    let replacementReleased = false;
    const replacementGate = new Promise<void>((resolve) => { releaseReplacement = resolve; });
    let firstPublishedNodes: ReturnType<typeof layoutNodes> | undefined;
    const Probe = () => {
      const result = useDiagramLayout(nodes, edges);
      const x = result.nodes[0]?.position.x;
      if (result.layoutStatus === "complete" && x === 10) firstPublishedNodes = result.nodes as ReturnType<typeof layoutNodes>;
      if (result.layoutStatus === "complete" && x === 20 && !replacementReleased) throw replacementGate;
      return <span>{`${result.layoutStatus}:${x ?? "empty"}`}</span>;
    };
    const view = render(<React.Suspense fallback={<span>deferred</span>}><Probe /></React.Suspense>);
    const firstDescriptor = firstPort.descriptors[0]!;
    const firstConsumer = firstPort.consumers[0]!;
    firstConsumer.readInputPage(0, 64);
    act(() => {
      firstConsumer.onOutputPage({ byteLength: 32, complete: true, itemCount: 1, payload: { complete: true, generation: firstDescriptor.generation, kind: "positions", sequence: 1, values: [{ index: 0, x: 10, y: 0 }] } });
      firstConsumer.onTerminal({ generation: firstDescriptor.generation, operation: firstDescriptor.operation, status: "complete" });
    });
    expect(view.getByText("complete:10")).toBeTruthy();
    expect(firstPublishedNodes?.[0]?.position.x).toBe(10);
    while (!firstConsumer.closeStep()) {}

    const secondPort = createTestInteractivePort("ready");
    setInteractiveJobPort(secondPort);
    await flushPortNotifications();
    const secondDescriptor = secondPort.descriptors[0]!;
    const secondConsumer = secondPort.consumers[0]!;
    secondConsumer.readInputPage(0, 64);
    act(() => secondConsumer.onOutputPage({ byteLength: 32, complete: true, itemCount: 1, payload: { complete: true, generation: secondDescriptor.generation, kind: "positions", sequence: 1, values: [{ index: 0, x: 20, y: 0 }] } }));
    act(() => React.startTransition(() => secondConsumer.onTerminal({ generation: secondDescriptor.generation, operation: secondDescriptor.operation, status: "complete" })));
    expect(view.getByText("complete:10")).toBeTruthy();
    await flushPortNotifications(8);
    expect(firstPublishedNodes?.[0]?.position.x).toBe(10);

    replacementReleased = true;
    await act(async () => {
      releaseReplacement();
      await replacementGate;
    });
    expect(view.getByText("complete:20")).toBeTruthy();
    await flushPortNotifications(8);
    expect(firstPublishedNodes?.[0]).toBeUndefined();
    while (!secondConsumer.closeStep()) {}
    view.unmount();
  });

  it("retires an abandoned suspended successor after a newer generation commits its lifecycle", async () => {
    const firstPort = createTestInteractivePort("ready");
    setInteractiveJobPort(firstPort);
    const nodes = layoutNodes(["a"]);
    const edges: Array<{ id: string; source: string; target: string }> = [];
    let releaseAbandoned = () => {};
    let abandonedReleased = false;
    const abandonedGate = new Promise<void>((resolve) => { releaseAbandoned = resolve; });
    let committedNodes: ReturnType<typeof layoutNodes> | undefined;
    let abandonedNodes: ReturnType<typeof layoutNodes> | undefined;
    const Probe = () => {
      const result = useDiagramLayout(nodes, edges);
      const x = result.nodes[0]?.position.x;
      if (result.layoutStatus === "complete" && x === 10) committedNodes = result.nodes as ReturnType<typeof layoutNodes>;
      if (result.layoutStatus === "complete" && x === 20) {
        abandonedNodes = result.nodes as ReturnType<typeof layoutNodes>;
        if (!abandonedReleased) throw abandonedGate;
      }
      return <span>{`${result.layoutStatus}:${x ?? "empty"}`}</span>;
    };
    const view = render(<React.Suspense fallback={<span>deferred</span>}><Probe /></React.Suspense>);
    const firstDescriptor = firstPort.descriptors[0]!;
    const firstConsumer = firstPort.consumers[0]!;
    firstConsumer.readInputPage(0, 64);
    act(() => {
      firstConsumer.onOutputPage({ byteLength: 32, complete: true, itemCount: 1, payload: { complete: true, generation: firstDescriptor.generation, kind: "positions", sequence: 1, values: [{ index: 0, x: 10, y: 0 }] } });
      firstConsumer.onTerminal({ generation: firstDescriptor.generation, operation: firstDescriptor.operation, status: "complete" });
    });
    expect(view.getByText("complete:10")).toBeTruthy();
    while (!firstConsumer.closeStep()) {}
    expect(firstConsumer.terminalIsEmpty()).toBe(true);

    const abandonedPort = createTestInteractivePort("ready");
    setInteractiveJobPort(abandonedPort);
    await flushPortNotifications();
    const abandonedDescriptor = abandonedPort.descriptors[0]!;
    const abandonedConsumer = abandonedPort.consumers[0]!;
    abandonedConsumer.readInputPage(0, 64);
    act(() => abandonedConsumer.onOutputPage({ byteLength: 32, complete: true, itemCount: 1, payload: { complete: true, generation: abandonedDescriptor.generation, kind: "positions", sequence: 1, values: [{ index: 0, x: 20, y: 0 }] } }));
    act(() => React.startTransition(() => abandonedConsumer.onTerminal({ generation: abandonedDescriptor.generation, operation: abandonedDescriptor.operation, status: "complete" })));
    expect(view.getByText("complete:10")).toBeTruthy();
    await flushPortNotifications(4);
    expect(committedNodes?.[0]?.position.x).toBe(10);
    expect(abandonedNodes?.[0]?.position.x).toBe(20);

    const successorPort = createTestInteractivePort("ready");
    setInteractiveJobPort(successorPort);
    await flushPortNotifications(8);
    await flushPortNotifications(8);
    expect(view.getByText("complete:10")).toBeTruthy();
    expect(committedNodes?.[0]?.position.x).toBe(10);
    expect(abandonedNodes?.[0]).toBeUndefined();
    expect(successorPort.descriptors).toHaveLength(1);

    act(() => abandonedConsumer.onTerminal({ generation: abandonedDescriptor.generation, operation: abandonedDescriptor.operation, status: "complete" }));
    while (!abandonedConsumer.closeStep()) {}
    expect(abandonedConsumer.terminalIsEmpty()).toBe(true);
    abandonedReleased = true;
    await act(async () => {
      releaseAbandoned();
      await abandonedGate;
    });
    expect(view.getByText("complete:10")).toBeTruthy();

    const successorDescriptor = successorPort.descriptors[0]!;
    const successorConsumer = successorPort.consumers[0]!;
    successorConsumer.readInputPage(0, 64);
    act(() => {
      successorConsumer.onOutputPage({ byteLength: 32, complete: true, itemCount: 1, payload: { complete: true, generation: successorDescriptor.generation, kind: "positions", sequence: 1, values: [{ index: 0, x: 30, y: 0 }] } });
      successorConsumer.onTerminal({ generation: successorDescriptor.generation, operation: successorDescriptor.operation, status: "complete" });
    });
    expect(view.getByText("complete:30")).toBeTruthy();
    await flushPortNotifications(8);
    expect(committedNodes?.[0]).toBeUndefined();
    while (!successorConsumer.closeStep()) {}
    expect(successorConsumer.terminalIsEmpty()).toBe(true);
    view.unmount();
  });

  it("retires displayed source fallback and a suspended successor on unmount", async () => {
    const port = createTestInteractivePort("ready");
    setInteractiveJobPort(port);
    const firstNodes = layoutNodes(["a"]);
    const fallbackNodes = [{ data: {}, id: "a", position: { x: 5, y: 0 } }];
    const edges: Array<{ id: string; source: string; target: string }> = [];
    let releasePending = () => {};
    const pendingGate = new Promise<void>((resolve) => { releasePending = resolve; });
    let committedNodes: ReturnType<typeof layoutNodes> | undefined;
    let pendingNodes: ReturnType<typeof layoutNodes> | undefined;
    const Probe = ({ values }: { values: ReturnType<typeof layoutNodes> }) => {
      const result = useDiagramLayout(values, edges);
      const x = result.nodes[0]?.position.x;
      if (result.layoutStatus === "complete" && x === 10) committedNodes = result.nodes as ReturnType<typeof layoutNodes>;
      if (result.layoutStatus === "complete" && x === 20) {
        pendingNodes = result.nodes as ReturnType<typeof layoutNodes>;
        throw pendingGate;
      }
      return <span>{`${result.layoutStatus}:${x ?? "empty"}`}</span>;
    };
    const view = render(<React.Suspense fallback={<span>deferred</span>}><Probe values={firstNodes} /></React.Suspense>);
    const firstDescriptor = port.descriptors[0]!;
    const firstConsumer = port.consumers[0]!;
    firstConsumer.readInputPage(0, 64);
    act(() => {
      firstConsumer.onOutputPage({ byteLength: 32, complete: true, itemCount: 1, payload: { complete: true, generation: firstDescriptor.generation, kind: "positions", sequence: 1, values: [{ index: 0, x: 10, y: 0 }] } });
      firstConsumer.onTerminal({ generation: firstDescriptor.generation, operation: firstDescriptor.operation, status: "complete" });
    });
    expect(view.getByText("complete:10")).toBeTruthy();
    while (!firstConsumer.closeStep()) {}

    view.rerender(<React.Suspense fallback={<span>deferred</span>}><Probe values={fallbackNodes} /></React.Suspense>);
    expect(view.getByText("pending:5")).toBeTruthy();
    await flushPortNotifications(8);
    await flushPortNotifications(8);
    expect(committedNodes?.[0]).toBeUndefined();
    const pendingDescriptor = port.descriptors[1]!;
    const pendingConsumer = port.consumers[1]!;
    pendingConsumer.readInputPage(0, 64);
    act(() => pendingConsumer.onOutputPage({ byteLength: 32, complete: true, itemCount: 1, payload: { complete: true, generation: pendingDescriptor.generation, kind: "positions", sequence: 1, values: [{ index: 0, x: 20, y: 0 }] } }));
    act(() => React.startTransition(() => pendingConsumer.onTerminal({ generation: pendingDescriptor.generation, operation: pendingDescriptor.operation, status: "complete" })));
    expect(view.getByText("pending:5")).toBeTruthy();
    expect(pendingNodes?.[0]?.position.x).toBe(20);
    view.unmount();
    await flushPortNotifications(8);
    expect(pendingNodes?.[0]).toBeUndefined();
    while (!pendingConsumer.closeStep()) {}
    expect(pendingConsumer.terminalIsEmpty()).toBe(true);
    releasePending();
  });

  it("rejects stale generations and closes incrementally under zero and finite budgets", () => {
    const job = createDiagramLayoutBatchTestJob(layoutNodes(["a", "b", "c"]), [{ id: "ab", source: "a", target: "b" }], {}, 29);
    expect(job.step({ deadline: performance.now() + 1_000, fuel: 1, generation: 30 }).status).toBe("cancelled");
    expect(job.close({ deadline: performance.now() + 1_000, fuel: 0 })).toBe(false);
    let closed = false;
    for (let index = 0; !closed && index < 100; index++) closed = job.close({ deadline: performance.now() + 1_000, fuel: 1 });
    expect(closed).toBe(true);

    const completed = createDiagramLayoutBatchTestJob(layoutNodes(["a", "b"]), [{ id: "ab", source: "a", target: "b" }], {}, 31);
    expect(runLayoutJob(completed).status).toBe("complete");
    const retained = completed.takeResult()!;
    while (!completed.close({ deadline: performance.now() + 1_000, fuel: 2 })) {}
    expect(retained.nodeCount).toBe(2);
    expect(retained.takeNode(0)?.id).toBe("a");
    while (!retained.closeStep()) {}
  });

  it("retires populated partial merge storage one finite close unit at a time", () => {
    const nodes = layoutNodes(Array.from({ length: 400 }, (_, index) => `n-${index.toString().padStart(4, "0")}`));
    const job = createDiagramLayoutBatchTestJob(nodes, [], {}, 33);
    expect(job.step({ deadline: performance.now() + 1_000, fuel: 560, generation: 33 }).status).toBe("running");
    job.cancel(33);
    expect(job.close({ deadline: performance.now() + 1_000, fuel: 0 })).toBe(false);
    let calls = 0;
    while (!job.close({ deadline: performance.now() + 1_000, fuel: 1 }) && calls < 5_000) calls += 1;
    expect(calls).toBeGreaterThan(128);
    expect(calls).toBeLessThan(5_000);
  });
});
