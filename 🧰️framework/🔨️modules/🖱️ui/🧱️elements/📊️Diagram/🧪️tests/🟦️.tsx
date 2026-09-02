// #region 🔌️Adapters
import * as React from "react";
import { act, render } from "@testing-library/react";
import { renderToString } from "react-dom/server";
import { afterEach, describe, expect, it, vi } from "vitest";
import { Diagram, createDiagramForceSimulation, useDiagramLayout, type DiagramForceConfig, type DiagramForceNode, type DiagramHandoffStatus } from "../🟦️.tsx";
import { DIAGRAM_LAYOUT_CODEC_KIND, DIAGRAM_LAYOUT_INGRESS_BYTES, DIAGRAM_LAYOUT_MAX_EDGE_BYTES, DIAGRAM_LAYOUT_MAX_NODE_BYTES, calculateDiagramLayoutForBatchTest, createDiagramLayoutBatchTestJob, createDiagramLayoutPublication, createDiagramLayoutWorkerJob, diagramLayoutCredits, diagramLayoutEdgeWireBytes, diagramLayoutNodeWireBytes, diagramLayoutUtf8Bytes, type DiagramLayoutDirection, type DiagramLayoutEdgeWire, type DiagramLayoutNodeWire } from "../🟦️.ts";
import { setInteractiveJobPort, type InteractiveJobPort } from "../../🔌️Ports/🟦️.ts";
// #endregion 🔌️Adapters

const flowCapture = vi.hoisted(() => ({ props: undefined as Record<string, any> | undefined }));
let diagramFrameStack = false;

vi.mock("../../🔌️Ports/🟦️.tsx", async (importOriginal) => {
  const actual = await importOriginal<typeof import("../../🔌️Ports/🟦️.tsx")>();
  const mocked = {
    ...actual,
    HostReactFlow: (props: Record<string, any>) => {
      flowCapture.props = props;
      return actual.reactHostPort.createElement(actual.HostReactFlow, props);
    },
  };
  Object.defineProperty(mocked, "interactiveJobPort", { configurable: true, enumerable: true, get: () => actual.interactiveJobPort });
  return mocked;
});

// #region 🧲️ForceFixtures
const forceConfig: DiagramForceConfig = { enabled: true, chargeStrength: -80, linkDistance: 60, collideRadius: 30, centerStrength: 0.15, updateIntervalMs: 0 };

function settle(nodes: DiagramForceNode[], links: { id: string; source: string | DiagramForceNode; target: string | DiagramForceNode }[], config: DiagramForceConfig = forceConfig, ticks = 30): Record<string, readonly [number, number]> {
  const simulation = createDiagramForceSimulation(nodes, links, config);
  for (let index = 0; index < ticks; index++) while (!simulation.step({ deadline: performance.now() + 1_000, fuel: 2_048 }).tickComplete) {}
  return Object.fromEntries(nodes.map((node) => [node.id, [node.x!, node.y!] as const]));
}

function installFrames(): { callbacks: Map<number, FrameRequestCallback>; run: (time: number) => void; flush: (time: number) => void } {
  let nextId = 0;
  const callbacks = new Map<number, FrameRequestCallback>();
  vi.stubGlobal(
    "requestAnimationFrame",
    vi.fn((callback: FrameRequestCallback) => {
      const id = ++nextId;
      callbacks.set(id, callback);
      return id;
    }),
  );
  vi.stubGlobal(
    "cancelAnimationFrame",
    vi.fn((id: number) => callbacks.delete(id)),
  );
  return {
    callbacks,
    run(time) {
      const entry = callbacks.entries().next().value as [number, FrameRequestCallback] | undefined;
      if (!entry) throw new Error("No scheduled Diagram frame");
      callbacks.delete(entry[0]);
      entry[1](time);
    },
    flush(time) {
      const scheduled = [...callbacks.entries()];
      callbacks.clear();
      for (const [, callback] of scheduled) callback(time);
    },
  };
}

function installBudgetClock(step = 0.01): { read: () => number } {
  let time = 0;
  vi.spyOn(performance, "now").mockImplementation(() => {
    time += step;
    return time;
  });
  return { read: () => time };
}

function installHandoffs(): () => void {
  vi.useFakeTimers({ toFake: ["setTimeout", "clearTimeout"] });
  return () =>
    act(() => {
      for (let index = 0; index < 8; index++) vi.runOnlyPendingTimers();
    });
}

function largeDiagramNodes(count: number): Array<{ id: string; position: { x: number; y: number }; data: Record<string, never>; selected?: boolean }> {
  return Array.from({ length: count }, (_, index) => ({ id: `node-${index.toString().padStart(5, "0")}`, position: { x: index + 1, y: index % 31 }, data: {} }));
}

function largeDiagramEdges(count: number): Array<{ id: string; source: string; target: string }> {
  return Array.from({ length: count }, (_, index) => ({ id: `edge-${index.toString().padStart(5, "0")}`, source: `node-${index.toString().padStart(5, "0")}`, target: `node-${((index + 1) % count).toString().padStart(5, "0")}` }));
}

function trackArrayReads<Value>(values: Value[]): { readonly values: Value[]; read(): number } {
  let reads = 0;
  return {
    read: () => reads,
    values: new Proxy(values, {
      get(target, property, receiver) {
        if (typeof property === "string" && /^(0|[1-9]\d*)$/.test(property)) reads += 1;
        return Reflect.get(target, property, receiver);
      },
    }),
  };
}

function runUntil(frames: ReturnType<typeof installFrames>, predicate: () => boolean, limit = 200, afterFrame?: () => void): number[] {
  const elapsed: number[] = [];
  for (let index = 0; index < limit && !predicate(); index++) {
    const started = performance.now();
    diagramFrameStack = true;
    act(() => frames.run(index * 16));
    diagramFrameStack = false;
    elapsed.push(performance.now() - started);
    afterFrame?.();
  }
  return elapsed;
}

afterEach(() => {
  setInteractiveJobPort({ status: "unavailable", getSnapshot: () => ({ revision: 0, status: "unavailable" }), observeConsumerTurn: () => true, subscribe: () => () => {}, submit: () => undefined });
  diagramFrameStack = false;
  flowCapture.props = undefined;
  vi.restoreAllMocks();
  vi.unstubAllGlobals();
  vi.useRealTimers();
});
// #endregion 🧲️ForceFixtures

// #region 🧪️OwnedDirectedLayout
function layoutNodes(ids: readonly string[]) {
  return ids.map((id) => ({ data: {}, id, position: { x: 0, y: 0 } }));
}

function layoutPositions(nodes: ReturnType<typeof layoutNodes>): Record<string, readonly [number, number]> {
  return Object.fromEntries(nodes.map((node) => [node.id, [node.position.x, node.position.y] as const]));
}

function runLayoutJob(job: ReturnType<typeof createDiagramLayoutBatchTestJob>, fuel = 17): ReturnType<ReturnType<typeof createDiagramLayoutBatchTestJob>["step"]> {
  let step = job.step({ deadline: performance.now() + 1_000, fuel, generation: job.generation });
  for (let index = 0; step.status === "running" && index < 1_000_000; index++) step = job.step({ deadline: performance.now() + 1_000, fuel, generation: job.generation });
  return step;
}

function captureLayoutPublication(publication: ReturnType<typeof createDiagramLayoutPublication>, count: number): void {
  let cursor = 0;
  for (let turn = 0; cursor < count && turn < count + 1; turn++) {
    const page = publication.readInputPage(cursor, 64);
    expect(page.itemCount).toBeGreaterThan(0);
    cursor += page.itemCount;
  }
  expect(cursor).toBe(count);
}

async function flushPortNotifications(turns = 3): Promise<void> {
  await act(async () => {
    for (let index = 0; index < turns; index++) await new Promise((resolve) => setTimeout(resolve, 0));
  });
}

function createTestInteractivePort(initialStatus: InteractiveJobPort["status"]): InteractiveJobPort & {
  readonly consumers: Array<Parameters<InteractiveJobPort["submit"]>[1]>;
  readonly descriptors: Array<Parameters<InteractiveJobPort["submit"]>[0]>;
  readonly cancellations: { count: number };
  transition(status: InteractiveJobPort["status"]): void;
} {
  let status = initialStatus;
  let revision = 0;
  let snapshot = { revision, status };
  const observers = new Set<() => void>();
  const consumers: Array<Parameters<InteractiveJobPort["submit"]>[1]> = [];
  const descriptors: Array<Parameters<InteractiveJobPort["submit"]>[0]> = [];
  const cancellations = { count: 0 };
  return {
    cancellations,
    consumers,
    descriptors,
    get status() { return status; },
    getSnapshot: () => snapshot,
    observeConsumerTurn: () => true,
    subscribe(listener) {
      observers.add(listener);
      return () => observers.delete(listener);
    },
    submit(descriptor, consumer) {
      descriptors.push(descriptor);
      consumers.push(consumer);
      return { generation: descriptor.generation, operation: descriptor.operation, cancel: () => { cancellations.count += 1; return true; } };
    },
    transition(nextStatus) {
      status = nextStatus;
      revision += 1;
      snapshot = { revision, status };
      for (const observer of observers) observer();
    },
  };
}

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
      observeConsumerTurn: () => true,
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
      diagramFrameStack = false;
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
    for (const descriptor of [
      { edgeCount: 0, generation: Number.NaN, kind: DIAGRAM_LAYOUT_CODEC_KIND, nodeCount: 0, options: {} },
      { edgeCount: 0, generation: 1, kind: DIAGRAM_LAYOUT_CODEC_KIND, nodeCount: 0.5, options: {} },
      { edgeCount: Number.POSITIVE_INFINITY, generation: 1, kind: DIAGRAM_LAYOUT_CODEC_KIND, nodeCount: 0, options: {} },
    ])
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
    const product = await import("../🟦️.tsx");
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
// #endregion 🧪️OwnedDirectedLayout

// #region 🧪️OwnedDiagramForce
describe("owned Diagram force", () => {
  it("replays by identity and remains close to the retired three-node reference fixture", () => {
    const makeNodes = () => [
      { id: "a", x: -80, y: 0 },
      { id: "b", x: 80, y: 0 },
      { id: "c", x: 0, y: 80 },
    ];
    const links = [
      { id: "ab", source: "a", target: "b" },
      { id: "bc", source: "b", target: "c" },
    ];
    const forward = settle(makeNodes(), links);
    const reverse = settle(makeNodes().reverse(), links.slice().reverse());
    expect(reverse).toEqual(forward);
    const retiredReference: Record<string, readonly [number, number]> = { a: [-11.574003, -31.790995], b: [28.684336, 7.358646], c: [-24.941755, 22.616164] };
    for (const id of Object.keys(retiredReference)) expect(Math.hypot(forward[id]![0] - retiredReference[id]![0], forward[id]![1] - retiredReference[id]![1])).toBeLessThan(35);
  });

  it("applies link distance, repulsion, collision, and center springs independently", () => {
    const linked = [
      { id: "a", x: -100, y: 0 },
      { id: "b", x: 100, y: 0 },
    ];
    settle(linked, [{ id: "ab", source: "a", target: "b" }], { enabled: true, chargeStrength: 0, linkDistance: 20, collideRadius: 0, centerStrength: 0 }, 1);
    expect(Math.abs(linked[1]!.x! - linked[0]!.x!)).toBeLessThan(200);

    const charged = [
      { id: "a", x: -100, y: 0 },
      { id: "b", x: 100, y: 0 },
    ];
    settle(charged, [], { enabled: true, chargeStrength: -100, linkDistance: 0, collideRadius: 0, centerStrength: 0 }, 1);
    expect(Math.abs(charged[1]!.x! - charged[0]!.x!)).toBeGreaterThan(200);

    const collided = [
      { id: "a", x: 0, y: 0 },
      { id: "b", x: 0, y: 0 },
    ];
    settle(collided, [], { enabled: true, chargeStrength: 0, linkDistance: 0, collideRadius: 30, centerStrength: 0 }, 1);
    expect(Math.hypot(collided[1]!.x! - collided[0]!.x!, collided[1]!.y! - collided[0]!.y!)).toBeGreaterThan(0);

    const centered = [{ id: "center", x: 100, y: 50 }];
    settle(centered, [], { enabled: true, chargeStrength: 0, linkDistance: 0, collideRadius: 0, centerStrength: 0.2 }, 1);
    expect(Math.abs(centered[0]!.x!)).toBeLessThan(100);
    expect(Math.abs(centered[0]!.y!)).toBeLessThan(50);
  });

  it("recovers finite positions and keeps every drag-pinned node fixed until unpinned", () => {
    const nodes: DiagramForceNode[] = [
      { id: "selected-a", x: Number.NaN, y: Number.POSITIVE_INFINITY, vx: Number.NaN, fx: 10, fy: 20 },
      { id: "selected-b", x: 0, y: 0, fx: -10, fy: -20 },
      { id: "free", x: 0, y: 0 },
    ];
    const simulation = createDiagramForceSimulation(nodes, [], forceConfig);
    for (let index = 0; index < 10; index++) while (!simulation.step({ deadline: performance.now() + 1_000, fuel: 2_048 }).tickComplete) {}
    expect(nodes.slice(0, 2).map((node) => [node.x, node.y, node.vx, node.vy])).toEqual([
      [10, 20, 0, 0],
      [-10, -20, 0, 0],
    ]);
    expect(nodes.every((node) => Number.isFinite(node.x) && Number.isFinite(node.y))).toBe(true);
    nodes[0]!.fx = null;
    nodes[0]!.fy = null;
    simulation.alphaTarget(0.3);
    while (!simulation.step({ deadline: performance.now() + 1_000, fuel: 2_048 }).tickComplete) {}
    expect([nodes[0]!.x, nodes[0]!.y]).not.toEqual([10, 20]);
  });

  it("schedules once, emits an initial notification after every restart, throttles, and cancels stale frames exactly", () => {
    const frames = installFrames();
    const listener = vi.fn();
    const nodes = [
      { id: "a", x: -10, y: 0 },
      { id: "b", x: 10, y: 0 },
    ];
    const simulation = createDiagramForceSimulation(nodes, [], { ...forceConfig, updateIntervalMs: 50 }).on("tick", listener);
    while (!simulation.step({ deadline: performance.now() + 1_000, fuel: 2_048 }).tickComplete) {}
    expect(listener).not.toHaveBeenCalled();
    simulation.restart().restart();
    expect(frames.callbacks.size).toBe(1);
    frames.run(0);
    expect(listener).toHaveBeenCalledTimes(1);
    frames.run(10);
    expect(listener).toHaveBeenCalledTimes(1);
    const restartStale = frames.callbacks.values().next().value!;
    simulation.stop().restart();
    restartStale(20);
    expect(listener).toHaveBeenCalledTimes(1);
    expect(frames.callbacks.size).toBe(1);
    frames.run(20);
    expect(listener).toHaveBeenCalledTimes(2);
    const stale = frames.callbacks.values().next().value!;
    simulation.stop();
    expect(frames.callbacks.size).toBe(0);
    stale(60);
    expect(listener).toHaveBeenCalledTimes(2);
    expect(frames.callbacks.size).toBe(0);
  });

  it("bounds a large graph to a cooperative subset of nodes per frame", () => {
    const frames = installFrames();
    installBudgetClock(0.001);
    const listener = vi.fn();
    const nodes = Array.from({ length: 20_000 }, (_, index) => ({ id: `node-${index.toString().padStart(5, "0")}`, x: index, y: 0, vx: 1, vy: 0 }));
    const simulation = createDiagramForceSimulation(nodes, [], { enabled: true, chargeStrength: 0, linkDistance: 0, collideRadius: 0, centerStrength: 0, updateIntervalMs: 0 }).on("tick", listener).restart();
    expect(nodes.every((node, index) => node.x === index)).toBe(true);
    const elapsed = runUntil(frames, () => nodes.some((node, index) => node.x !== index), 3_000);
    const changed = nodes.filter((node, index) => node.x !== index).length;
    expect(elapsed.length).toBeGreaterThan(20);
    expect(Math.max(...elapsed)).toBeLessThanOrEqual(6.1);
    expect(changed).toBeGreaterThan(0);
    expect(changed).toBeLessThan(nodes.length / 2);
    if (listener.mock.calls.length === 0) frames.run(16);
    expect(listener).toHaveBeenCalledTimes(1);
    expect(frames.callbacks.size).toBe(1);
    simulation.stop();
  });

  it("resumes every full-force tick phase before its six-millisecond deadline", () => {
    const frames = installFrames();
    const clock = installBudgetClock(0.0001);
    const nodes = Array.from({ length: 5_000 }, (_, index) => ({ id: `node-${index.toString().padStart(5, "0")}`, x: index + 1, y: index % 29, vx: 0, vy: 0 }));
    const links = nodes.map((node, index) => ({ id: `link-${node.id}`, source: node.id, target: nodes[(index + 1) % nodes.length]!.id }));
    const listener = vi.fn();
    const simulation = createDiagramForceSimulation(nodes, links, { enabled: true, chargeStrength: -80, linkDistance: 60, collideRadius: 30, centerStrength: 0.15, updateIntervalMs: 1_000_000 }).on("tick", listener).restart();
    const initial = nodes.map((node) => [node.x, node.y]);
    const elapsed = runUntil(frames, () => nodes.some((node, index) => node.x !== initial[index]![0] || node.y !== initial[index]![1]), 6_000);
    expect(elapsed.length).toBeGreaterThan(20);
    expect(Math.max(...elapsed)).toBeLessThanOrEqual(6.1);
    expect(clock.read()).toBeGreaterThan(0);
    expect(nodes.some((node, index) => node.x !== initial[index]![0] || node.y !== initial[index]![1])).toBe(true);
    if (listener.mock.calls.length === 0) act(() => frames.run(elapsed.length * 16));
    expect(listener).toHaveBeenCalledTimes(1);
    simulation.stop();
  });

  it("keeps controlled input positions stable while emitting cooperative proposals", () => {
    const frames = installFrames();
    const flushHandoffs = installHandoffs();
    const onNodesChange = vi.fn();
    const nodes = [{ id: "controlled", position: { x: 100, y: 50 }, data: {} }];
    const view = render(
      <div style={{ width: 500, height: 500 }}>
        <Diagram nodeTypes={{}} nodes={nodes} edges={[]} onNodesChange={onNodesChange} forceConfig={{ enabled: true, chargeStrength: 0, collideRadius: 0, centerStrength: 0.2, updateIntervalMs: 0 }} />
      </div>,
    );
    expect(onNodesChange).not.toHaveBeenCalled();
    act(() => frames.flush(0));
    expect(onNodesChange).not.toHaveBeenCalled();
    flushHandoffs();
    expect(onNodesChange).toHaveBeenCalled();
    const proposal = onNodesChange.mock.calls.at(-1)![0][0];
    expect(proposal.position.x).toBeLessThan(100);
    expect(proposal.position.y).toBeLessThan(50);
    expect(nodes[0]!.position).toEqual({ x: 100, y: 50 });
    const stale = [...frames.callbacks.values()];
    const calls = onNodesChange.mock.calls.length;
    view.unmount();
    for (const callback of stale) callback(16);
    expect(onNodesChange).toHaveBeenCalledTimes(calls);
  });

  it("cursorizes live controlled 20,000-node and edge setup before cooperative projection", () => {
    const frames = installFrames();
    const flushHandoffs = installHandoffs();
    const rawNodes = largeDiagramNodes(20_000);
    const rawEdges = largeDiagramEdges(20_000);
    const handoffStatusRef = React.createRef<DiagramHandoffStatus | null>();
    let subscriberReads = 0;
    let subscriberShouldBlock = false;
    const onNodesChange = vi.fn((proposal: typeof rawNodes) => {
      expect(diagramFrameStack).toBe(false);
      for (let index = 0; index < proposal.length; index++) if (proposal[index]) subscriberReads += 1;
      if (subscriberShouldBlock) {
        const until = Date.now() + 10;
        while (Date.now() < until) {}
      }
    });
    const nodes = trackArrayReads(rawNodes);
    const edges = trackArrayReads(rawEdges);
    const originalLastPosition = { ...rawNodes.at(-1)!.position };
    const view = render(<Diagram nodeTypes={{}} nodes={[]} edges={[]} onNodesChange={onNodesChange} forceConfig={{ enabled: false }} />);
    installBudgetClock(0.0005);
    const setupStarted = performance.now();
    view.rerender(
      <Diagram
        nodeTypes={{}}
        nodes={nodes.values}
        edges={edges.values}
        handoffStatusRef={handoffStatusRef}
        onNodesChange={onNodesChange}
        forceConfig={{ enabled: true, chargeStrength: 0, linkDistance: 60, collideRadius: 0, centerStrength: 0.2, updateIntervalMs: 0 }}
      />,
    );
    const setupElapsed = performance.now() - setupStarted;
    expect(setupElapsed).toBeLessThan(8);
    expect(nodes.read()).toBe(0);
    expect(edges.read()).toBe(0);
    act(() => frames.run(0));
    expect(onNodesChange).not.toHaveBeenCalled();
    expect(nodes.read()).toBeGreaterThan(0);
    expect(nodes.read()).toBeLessThan(1_200);
    expect(edges.read()).toBe(0);
    const elapsed = runUntil(frames, () => onNodesChange.mock.calls.length > 0, 10_000, flushHandoffs);
    expect(elapsed.length).toBeGreaterThan(1);
    expect(Math.max(...elapsed)).toBeLessThanOrEqual(6.1);
    expect(onNodesChange).toHaveBeenCalledTimes(1);
    expect(onNodesChange.mock.calls[0]![0]).toHaveLength(20_000);
    expect(onNodesChange.mock.calls[0]![0].at(-1).id).toBe(rawNodes.at(-1)!.id);
    expect(subscriberReads).toBe(20_000);
    expect(rawNodes.at(-1)!.position).toEqual(originalLastPosition);
    expect(edges.read()).toBeGreaterThan(0);
    expect(flowCapture.props?.nodes.length).toBeGreaterThan(0);
    expect(flowCapture.props?.nodes.length).toBeLessThanOrEqual(128);
    expect(flowCapture.props?.edges.length).toBeGreaterThan(0);
    expect(flowCapture.props?.edges.length).toBeLessThanOrEqual(256);
    expect(view.container.querySelector(".react-flow")).not.toBeNull();
    const lastValidGeneration = handoffStatusRef.current?.lastValidPublicationGeneration();
    expect(lastValidGeneration).toBeTypeOf("number");
    subscriberShouldBlock = true;
    act(() => flowCapture.props?.onNodeDragStart({} as MouseEvent, rawNodes[0], rawNodes));
    act(() => flowCapture.props?.onNodeDragStop({} as MouseEvent, rawNodes[0], rawNodes));
    runUntil(frames, () => onNodesChange.mock.calls.length === 2, 1_000, flushHandoffs);
    expect(onNodesChange).toHaveBeenCalledTimes(2);
    expect(subscriberReads).toBe(40_000);
    expect(handoffStatusRef.current?.violations().filter((violation) => violation.kind === "consumer-publication")).toHaveLength(1);
    expect(handoffStatusRef.current?.lastValidPublicationGeneration()).toBe(lastValidGeneration);
    act(() => flowCapture.props?.onNodeDragStart({} as MouseEvent, rawNodes[0], rawNodes));
    act(() => flowCapture.props?.onNodeDragStop({} as MouseEvent, rawNodes[0], rawNodes));
    for (let index = 0; index < 200 && frames.callbacks.size > 0; index++) {
      act(() => frames.run(index * 16));
      flushHandoffs();
    }
    expect(onNodesChange).toHaveBeenCalledTimes(2);
    expect(subscriberReads).toBe(40_000);
    view.unmount();
  });

  it("projects and commits a real uncontrolled 20,000-node Diagram cooperatively", () => {
    const frames = installFrames();
    const flushHandoffs = installHandoffs();
    const onNodesChange = vi.fn();
    const nodes = trackArrayReads(largeDiagramNodes(20_000));
    const view = render(<Diagram nodeTypes={{}} initialNodes={[]} initialEdges={[]} onNodesChange={onNodesChange} forceConfig={{ enabled: false }} />);
    installBudgetClock(0.0001);
    const setupStarted = performance.now();
    view.rerender(<Diagram nodeTypes={{}} initialNodes={nodes.values} initialEdges={[]} onNodesChange={onNodesChange} forceConfig={{ enabled: true, chargeStrength: 0, collideRadius: 0, centerStrength: 0.2, updateIntervalMs: 0 }} />);
    const setupElapsed = performance.now() - setupStarted;
    expect(setupElapsed).toBeLessThan(8);
    expect(nodes.read()).toBe(0);
    flushHandoffs();
    onNodesChange.mockClear();
    act(() => frames.run(0));
    expect(onNodesChange).not.toHaveBeenCalled();
    const elapsed = runUntil(frames, () => onNodesChange.mock.calls.length > 0, 4_000, flushHandoffs);
    expect(elapsed.length).toBeGreaterThan(1);
    expect(Math.max(...elapsed)).toBeLessThanOrEqual(6.1);
    expect(onNodesChange).toHaveBeenCalledTimes(1);
    expect(onNodesChange.mock.calls[0]![0]).toHaveLength(20_000);
    expect(flowCapture.props?.nodes.length).toBeGreaterThan(0);
    expect(flowCapture.props?.nodes.length).toBeLessThanOrEqual(128);
    expect(view.container.querySelector(".react-flow")).not.toBeNull();
    view.unmount();
  });

  it("enqueues, coalesces, pins, and unpins a real large multi-selection outside pointer callbacks", () => {
    const frames = installFrames();
    const flushHandoffs = installHandoffs();
    const events: string[] = [];
    const onNodesChange = vi.fn();
    const handoffStatusRef = React.createRef<DiagramHandoffStatus | null>();
    let pointerStack = false;
    const onNodeDrag = vi.fn((_event: MouseEvent | TouchEvent, _node: ReturnType<typeof largeDiagramNodes>[number], selection: ReturnType<typeof largeDiagramNodes>) => {
      expect(pointerStack).toBe(false);
      for (let index = 0; index < selection.length; index++) selection[index];
      const until = Date.now() + 10;
      while (Date.now() < until) {}
      events.push("drag");
    });
    const nodes = largeDiagramNodes(3_001).map((node) => ({ ...node, selected: true }));
    const view = render(
      <Diagram
        nodeTypes={{}}
        nodes={nodes}
        edges={[]}
        handoffStatusRef={handoffStatusRef}
        onNodesChange={onNodesChange}
        onNodeDragStart={() => events.push("start")}
        onNodeDrag={onNodeDrag}
        onNodeDragStop={() => events.push("stop")}
        forceConfig={{ enabled: true, chargeStrength: 0, collideRadius: 0, centerStrength: 0.2, updateIntervalMs: 0 }}
      />,
    );
    const startedNodes = nodes.map((node) => ({ ...node }));
    const movedNodes = nodes.map((node, index) => ({ ...node, position: { x: 10_000 + index * 2, y: -2_000 - index } }));
    const startedLead = startedNodes[0]!;
    const movedLead = movedNodes[0]!;
    const started = trackArrayReads(startedNodes);
    const moved = trackArrayReads(movedNodes);
    installBudgetClock(0.0001);
    const callbacksStarted = performance.now();
    pointerStack = true;
    act(() => flowCapture.props?.onNodeDragStart({} as MouseEvent, startedLead, started.values));
    act(() => flowCapture.props?.onNodeDrag({} as MouseEvent, movedLead, moved.values));
    pointerStack = false;
    const callbacksElapsed = performance.now() - callbacksStarted;
    expect(callbacksElapsed).toBeLessThan(8);
    expect(started.read()).toBe(0);
    expect(moved.read()).toBe(0);
    expect(events).toEqual([]);
    flushHandoffs();
    expect(onNodeDrag).toHaveBeenCalledTimes(1);
    expect(moved.read()).toBe(3_001);
    expect(handoffStatusRef.current?.violations().filter((violation) => violation.kind === "drag-move")).toHaveLength(1);
    pointerStack = true;
    act(() => flowCapture.props?.onNodeDrag({} as MouseEvent, movedLead, moved.values));
    pointerStack = false;
    flushHandoffs();
    expect(onNodeDrag).toHaveBeenCalledTimes(1);
    expect(moved.read()).toBe(3_001);
    const pinnedElapsed = runUntil(frames, () => onNodesChange.mock.calls.length > 0, 4_000, flushHandoffs);
    expect(pinnedElapsed.length).toBeGreaterThan(0);
    const pinnedProposal = onNodesChange.mock.calls.at(-1)![0];
    expect(pinnedProposal[0].position).toEqual(moved.values[0]!.position);
    expect(pinnedProposal[2_500].position).toEqual(moved.values[2_500]!.position);
    const proposalCount = onNodesChange.mock.calls.length;
    const readsBeforeStop = moved.read();
    const stopStarted = performance.now();
    act(() => flowCapture.props?.onNodeDragStop({} as MouseEvent, movedLead, moved.values));
    const stopElapsed = performance.now() - stopStarted;
    expect(stopElapsed).toBeLessThan(8);
    expect(moved.read() - readsBeforeStop).toBe(0);
    expect(events).toEqual(["start", "drag"]);
    flushHandoffs();
    runUntil(frames, () => onNodesChange.mock.calls.slice(proposalCount).some(([proposal]) => proposal[2_500].position.x !== moved.values[2_500]!.position.x || proposal[2_500].position.y !== moved.values[2_500]!.position.y), 500, flushHandoffs);
    expect(onNodesChange.mock.calls.slice(proposalCount).some(([proposal]) => proposal[2_500].position.x !== moved.values[2_500]!.position.x || proposal[2_500].position.y !== moved.values[2_500]!.position.y)).toBe(true);
    expect(events).toEqual(["start", "drag", "stop"]);
    expect(nodes[2_500]!.position).not.toEqual(moved.values[2_500]!.position);
    view.unmount();
  });

  it("expands a real virtualized host-page drag to the complete selected graph cooperatively", () => {
    const frames = installFrames();
    const flushHandoffs = installHandoffs();
    const nodes = largeDiagramNodes(3_001).map((node) => ({ ...node, selected: true }));
    const onNodesChange = vi.fn();
    const onNodeDrag = vi.fn();
    const view = render(<Diagram nodeTypes={{}} nodes={nodes} edges={[]} onNodesChange={onNodesChange} onNodeDrag={onNodeDrag} forceConfig={{ enabled: true, chargeStrength: 0, collideRadius: 0, centerStrength: 0, updateIntervalMs: 0 }} />);
    installBudgetClock(0.0001);
    runUntil(frames, () => onNodesChange.mock.calls.length > 0, 2_000, flushHandoffs);
    const hostSelection = flowCapture.props?.nodes as typeof nodes;
    expect(hostSelection.length).toBeGreaterThan(1);
    expect(hostSelection.length).toBeLessThanOrEqual(128);
    const lead = hostSelection[0]!;
    const delta = { x: 700, y: -300 };
    const movedLead = { ...lead, position: { x: lead.position.x + delta.x, y: lead.position.y + delta.y } };
    const callbackStarted = performance.now();
    act(() => flowCapture.props?.onNodeDragStart({} as MouseEvent, lead, hostSelection));
    act(() => flowCapture.props?.onNodeDrag({} as MouseEvent, movedLead, [movedLead, ...hostSelection.slice(1)]));
    expect(performance.now() - callbackStarted).toBeLessThan(8);
    flushHandoffs();
    expect(onNodeDrag).toHaveBeenCalledTimes(1);
    const semanticSelection = onNodeDrag.mock.calls[0]![2] as typeof nodes;
    expect(semanticSelection).toHaveLength(3_001);
    expect(semanticSelection[2_500]!.position).toEqual({ x: nodes[2_500]!.position.x + delta.x, y: nodes[2_500]!.position.y + delta.y });
    onNodesChange.mockClear();
    runUntil(frames, () => onNodesChange.mock.calls.some(([proposal]) => proposal[2_500].position.x === nodes[2_500]!.position.x + delta.x && proposal[2_500].position.y === nodes[2_500]!.position.y + delta.y), 2_000, flushHandoffs);
    expect(onNodesChange.mock.calls.some(([proposal]) => proposal[2_500].position.x === nodes[2_500]!.position.x + delta.x && proposal[2_500].position.y === nodes[2_500]!.position.y + delta.y)).toBe(true);
    act(() => flowCapture.props?.onNodeDragStop({} as MouseEvent, movedLead, hostSelection));
    flushHandoffs();
    view.unmount();
  });

  it("quarantines a throwing publication consumer, retains the last snapshot, and continues draining", () => {
    const frames = installFrames();
    const flushHandoffs = installHandoffs();
    const handoffStatusRef = React.createRef<DiagramHandoffStatus | null>();
    const nodes = [
      { id: "source", position: { x: -100, y: 0 }, data: {} },
      { id: "target", position: { x: 100, y: 0 }, data: {} },
    ];
    let shouldThrow = false;
    const onNodesChange = vi.fn(() => {
      expect(diagramFrameStack).toBe(false);
      if (shouldThrow) throw new Error("blocked Diagram consumer");
    });
    const onNodeDragStop = vi.fn();
    const view = render(
      <Diagram
        nodeTypes={{}}
        nodes={nodes}
        edges={[]}
        handoffStatusRef={handoffStatusRef}
        onNodesChange={onNodesChange}
        onNodeDragStop={onNodeDragStop}
        forceConfig={{ enabled: true, chargeStrength: 0, collideRadius: 0, centerStrength: 0.2, updateIntervalMs: 0 }}
      />,
    );
    runUntil(frames, () => onNodesChange.mock.calls.length === 1, 500, flushHandoffs);
    expect(onNodesChange).toHaveBeenCalledTimes(1);
    const lastValidGeneration = handoffStatusRef.current?.lastValidPublicationGeneration();
    expect(lastValidGeneration).toBeTypeOf("number");
    shouldThrow = true;
    act(() => flowCapture.props?.onNodeDragStart({} as MouseEvent, nodes[0], nodes));
    act(() => flowCapture.props?.onNodeDragStop({} as MouseEvent, nodes[0], nodes));
    runUntil(frames, () => onNodesChange.mock.calls.length === 2, 500, flushHandoffs);
    expect(onNodesChange).toHaveBeenCalledTimes(2);
    expect(handoffStatusRef.current?.violations().filter((violation) => violation.kind === "consumer-publication")).toEqual([expect.objectContaining({ fault: "blocked Diagram consumer" })]);
    expect(handoffStatusRef.current?.lastValidPublicationGeneration()).toBe(lastValidGeneration);
    act(() => flowCapture.props?.onNodeDragStop({} as MouseEvent, nodes[0], nodes));
    flushHandoffs();
    expect(onNodeDragStop).toHaveBeenCalledTimes(2);
    act(() => flowCapture.props?.onNodeDragStart({} as MouseEvent, nodes[0], nodes));
    act(() => flowCapture.props?.onNodeDragStop({} as MouseEvent, nodes[0], nodes));
    for (let index = 0; index < 100 && frames.callbacks.size > 0; index++) {
      act(() => frames.run(index * 16));
      flushHandoffs();
    }
    expect(onNodesChange).toHaveBeenCalledTimes(2);
    expect(handoffStatusRef.current?.lastValidPublicationGeneration()).toBe(lastValidGeneration);
    view.unmount();
  });

  it("resolves exact colliding-sample oversized identifiers and never rereads them in force hot paths", () => {
    const frames = installFrames();
    const tail = "x".repeat(99_998);
    const ids = [`xA${tail}`, `xB${tail}`, "target"];
    let identityReads = 0;
    const nodes = ids.map((id, index) => {
      const node: DiagramForceNode = { x: index === 0 ? -100 : index === 1 ? 100 : 0, y: index === 2 ? 100 : 0 };
      Object.defineProperty(node, "id", { configurable: true, get: () => ((identityReads += 1), id) });
      return node;
    });
    installBudgetClock(0.0001);
    const setupStarted = performance.now();
    const simulation = createDiagramForceSimulation(nodes, [{ id: "exact-link", source: ids[0]!, target: ids[2]! }], { enabled: true, chargeStrength: 0, linkDistance: 20, collideRadius: 0, centerStrength: 0, updateIntervalMs: 0 }).restart();
    expect(performance.now() - setupStarted).toBeLessThan(8);
    expect(identityReads).toBe(0);
    expect("tick" in simulation).toBe(false);
    expect(simulation.step({ deadline: performance.now() + 1, fuel: 0 })).toEqual({ initialized: false, remainingFuel: 0, tickComplete: false });
    expect(identityReads).toBe(0);
    const initial = nodes.map((node) => [node.x, node.y]);
    const elapsed = runUntil(frames, () => nodes[0]!.x !== initial[0]![0], 2_000);
    expect(Math.max(...elapsed)).toBeLessThanOrEqual(6.1);
    expect(identityReads).toBe(3);
    expect(nodes[0]!.x).not.toBe(initial[0]![0]);
    expect(nodes[1]!.x).toBe(initial[1]![0]);
    const initializedReads = identityReads;
    for (let index = 0; index < 10; index++) act(() => frames.run(index * 16));
    expect(identityReads).toBe(initializedReads);
    expect(nodes.every((node) => Number.isFinite(node.x) && Number.isFinite(node.y))).toBe(true);
    simulation.stop();
  });

  it("faults duplicate exact node identities deterministically through the bounded public step", () => {
    const make = () =>
      createDiagramForceSimulation(
        [
          { id: "duplicate", x: 0, y: 0 },
          { id: "duplicate", x: 1, y: 0 },
        ],
        [],
        forceConfig,
      );
    const drive = (simulation: ReturnType<typeof make>) => {
      for (let index = 0; index < 100; index++) simulation.step({ deadline: performance.now() + 8, fuel: 1 });
    };
    expect(() => drive(make())).toThrowError("Duplicate Diagram force node id");
    expect(() => drive(make())).toThrowError("Duplicate Diagram force node id");
  });

  it("renders the real Diagram surface statically without browser scheduling", () => {
    const frames = installFrames();
    const html = renderToString(<Diagram nodeTypes={{}} initialNodes={[{ id: "ssr", position: { x: 10, y: 20 }, data: {} }]} initialEdges={[]} forceConfig={{ enabled: true }} />);
    expect(html).toContain("react-flow");
    expect(frames.callbacks.size).toBe(0);
  });
});
// #endregion 🧪️OwnedDiagramForce
