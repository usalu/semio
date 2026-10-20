/** 🔸️ The admits publishes only through tests of the owned Diagram directed layout suite. */
import { it, expect } from "vitest";
import { DIAGRAM_LAYOUT_CODEC_KIND, DIAGRAM_LAYOUT_MAX_EDGE_BYTES } from "../../../../📐️layout/📜️wire/🟦️.ts";
import { useDiagramLayout } from "../../../../🪢️layout-hook/🟦️.ts";
import { render, act } from "@testing-library/react";
import { type InteractiveJobPort, setInteractiveJobPort } from "../../../../../🔌️Ports/📡️interactive-jobs/🟦️.ts";
import { createTestInteractivePort, diagramFrameStack, flushPortNotifications, layoutNodes, trackArrayReads, setDiagramFrameStack } from "../../🧰️harness/🟦️.ts";

/** 🔹 Registers the admits publishes only through tests of the owned Diagram directed layout suite. */
export function registerAdmitsPublishesOnlyThroughTests(): void {
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
}
