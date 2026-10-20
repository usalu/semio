/** 🟠️ The enqueues coalesces pins unpins tests of the owned Diagram force suite. */
import { it, expect, vi } from "vitest";
import { Diagram } from "../../../../🟦️.tsx";
import { type DiagramHandoffStatus } from "../../../../🤝️handoff/🟦️.ts";
import { act, render } from "@testing-library/react";
import * as React from "react";
import { diagramFrameStack, flowCapture, forceConfig, installBudgetClock, installFrames, installHandoffs, largeDiagramNodes, runUntil, trackArrayReads } from "../../🧰️harness/🟦️.ts";

/** 🟡 Registers the enqueues coalesces pins unpins tests of the owned Diagram force suite. */
export function registerEnqueuesCoalescesPinsUnpinsTests(): void {
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
}
