import { vi, afterEach, expect } from "vitest";
import { type DiagramForceConfig, type DiagramForceNode } from "../../../📋️force-model/🟦️.ts";
import { createDiagramForceSimulation } from "../../../🧮️force-simulation/🟦️.ts";
import { type Node } from "../../../🟦️.tsx";
import { act } from "@testing-library/react";
import { setInteractiveJobPort, type InteractiveJobPort } from "../../../../🔌️Ports/📡️interactive-jobs/🟦️.ts";
import { createDiagramLayoutBatchTestJob } from "../../../📐️layout/🟦️.ts";
import { createDiagramLayoutPublication } from "../../../📐️layout/📢️publication/🟦️.ts";

export const flowCapture = vi.hoisted(() => ({ props: undefined as Record<string, any> | undefined }));
export let diagramFrameStack = false;
/** 🎞️ Lets a test module reset the frame-stack witness, because an imported binding cannot be assigned from outside its module. */
export function setDiagramFrameStack(value: boolean): void {
  diagramFrameStack = value;
}

vi.mock("../../../../🔌️Ports/🟦️.tsx", async (importOriginal) => {
  const actual = await importOriginal<typeof import("../../../../🔌️Ports/🟦️.tsx")>();
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
export const forceConfig: DiagramForceConfig = { enabled: true, chargeStrength: -80, linkDistance: 60, collideRadius: 30, centerStrength: 0.15, updateIntervalMs: 0 };

export function settle(nodes: DiagramForceNode[], links: { id: string; source: string | DiagramForceNode; target: string | DiagramForceNode }[], config: DiagramForceConfig = forceConfig, ticks = 30): Record<string, readonly [number, number]> {
  const simulation = createDiagramForceSimulation(nodes, links, config);
  for (let index = 0; index < ticks; index++) while (!simulation.step({ deadline: performance.now() + 1_000, fuel: 2_048 }).tickComplete) {}
  return Object.fromEntries(nodes.map((node) => [node.id, [node.x!, node.y!] as const]));
}

export function installFrames(): { callbacks: Map<number, FrameRequestCallback>; run: (time: number) => void; flush: (time: number) => void } {
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

export function installBudgetClock(step = 0.01): { read: () => number } {
  let time = 0;
  vi.spyOn(performance, "now").mockImplementation(() => {
    time += step;
    return time;
  });
  return { read: () => time };
}

export function installHandoffs(): () => void {
  vi.useFakeTimers({ toFake: ["setTimeout", "clearTimeout"] });
  return () =>
    act(() => {
      for (let index = 0; index < 8; index++) vi.runOnlyPendingTimers();
    });
}

export function largeDiagramNodes(count: number): Node[] {
  return Array.from({ length: count }, (_, index) => ({ id: `node-${index.toString().padStart(5, "0")}`, position: { x: index + 1, y: index % 31 }, data: {} }));
}

export function largeDiagramEdges(count: number): Array<{ id: string; source: string; target: string }> {
  return Array.from({ length: count }, (_, index) => ({ id: `edge-${index.toString().padStart(5, "0")}`, source: `node-${index.toString().padStart(5, "0")}`, target: `node-${((index + 1) % count).toString().padStart(5, "0")}` }));
}

export function trackArrayReads<Value>(values: Value[]): { readonly values: Value[]; read(): number } {
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

export function runUntil(frames: ReturnType<typeof installFrames>, predicate: () => boolean, limit = 200, afterFrame?: () => void): number[] {
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
  setInteractiveJobPort({ status: "unavailable", getSnapshot: () => ({ revision: 0, status: "unavailable" }), observeConsumerTurn: () => true, reportConsumerFault: () => {}, subscribe: () => () => {}, submit: () => undefined });
  diagramFrameStack = false;
  flowCapture.props = undefined;
  vi.restoreAllMocks();
  vi.unstubAllGlobals();
  vi.useRealTimers();
});
// #endregion 🧲️ForceFixtures

export function layoutNodes(ids: readonly string[]) {
  return ids.map((id) => ({ data: {}, id, position: { x: 0, y: 0 } }));
}

export function layoutPositions(nodes: ReturnType<typeof layoutNodes>): Record<string, readonly [number, number]> {
  return Object.fromEntries(nodes.map((node) => [node.id, [node.position.x, node.position.y] as const]));
}

export function runLayoutJob(job: ReturnType<typeof createDiagramLayoutBatchTestJob>, fuel = 17): ReturnType<ReturnType<typeof createDiagramLayoutBatchTestJob>["step"]> {
  let step = job.step({ deadline: performance.now() + 1_000, fuel, generation: job.generation });
  for (let index = 0; step.status === "running" && index < 1_000_000; index++) step = job.step({ deadline: performance.now() + 1_000, fuel, generation: job.generation });
  return step;
}

export function captureLayoutPublication(publication: ReturnType<typeof createDiagramLayoutPublication>, count: number): void {
  let cursor = 0;
  for (let turn = 0; cursor < count && turn < count + 1; turn++) {
    const page = publication.readInputPage(cursor, 64);
    expect(page.itemCount).toBeGreaterThan(0);
    cursor += page.itemCount;
  }
  expect(cursor).toBe(count);
}

export async function flushPortNotifications(turns = 3): Promise<void> {
  await act(async () => {
    for (let index = 0; index < turns; index++) await new Promise((resolve) => setTimeout(resolve, 0));
  });
}

export function createTestInteractivePort(initialStatus: InteractiveJobPort["status"]): InteractiveJobPort & {
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
    observeConsumerTurn: () => true, reportConsumerFault: () => {},
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
