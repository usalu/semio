/** 🟢️ The retires displayed source fallback tests of the owned Diagram directed layout suite. */
import { it, expect } from "vitest";
import { createDiagramLayoutBatchTestJob } from "../../../../📐️layout/🟦️.ts";
import { useDiagramLayout } from "../../../../🪢️layout-hook/🟦️.ts";
import { render, act } from "@testing-library/react";
import { setInteractiveJobPort } from "../../../../../🔌️Ports/📡️interactive-jobs/🟦️.ts";
import * as React from "react";
import { createTestInteractivePort, flushPortNotifications, layoutNodes, runLayoutJob } from "../../🧰️harness/🟦️.ts";

/** 🔵 Registers the retires displayed source fallback tests of the owned Diagram directed layout suite. */
export function registerRetiresDisplayedSourceFallbackTests(): void {
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
}
