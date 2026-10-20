/** 🔸️ The cursorizes live controlled 20 tests of the owned Diagram force suite. */
import { it, expect, vi } from "vitest";
import { Diagram } from "../../../../🟦️.tsx";
import { type DiagramHandoffStatus } from "../../../../🤝️handoff/🟦️.ts";
import { act, render } from "@testing-library/react";
import * as React from "react";
import { diagramFrameStack, flowCapture, forceConfig, installBudgetClock, installFrames, installHandoffs, largeDiagramEdges, largeDiagramNodes, runUntil, trackArrayReads } from "../../🧰️harness/🟦️.ts";

/** 🔹 Registers the cursorizes live controlled 20 tests of the owned Diagram force suite. */
export function registerCursorizesLiveControlled20Tests(): void {
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
    expect(onNodesChange.mock.calls[0]![0].at(-1)!.id).toBe(rawNodes.at(-1)!.id);
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
}
