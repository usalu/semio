/** 🟠️ The retains committed proxy through tests of the owned Diagram directed layout suite. */
import { it, expect } from "vitest";
import { useDiagramLayout } from "../../../../🪢️layout-hook/🟦️.ts";
import { render, act } from "@testing-library/react";
import { setInteractiveJobPort } from "../../../../../🔌️Ports/📡️interactive-jobs/🟦️.ts";
import * as React from "react";
import { createTestInteractivePort, flushPortNotifications, layoutNodes } from "../../🧰️harness/🟦️.ts";

/** 🟡 Registers the retains committed proxy through tests of the owned Diagram directed layout suite. */
export function registerRetainsCommittedProxyThroughTests(): void {
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
}
