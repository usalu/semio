import { describe, it, expect, vi } from "vitest";
import { type DiagramForceNode } from "../../../📋️force-model/🟦️.ts";
import { createDiagramForceSimulation } from "../../../🧮️force-simulation/🟦️.ts";
import { Diagram } from "../../../🟦️.tsx";
import { type DiagramHandoffStatus } from "../../../🤝️handoff/🟦️.ts";
import { act, render } from "@testing-library/react";
import * as React from "react";
import { renderToString } from "react-dom/server";
import { diagramFrameStack, flowCapture, forceConfig, installBudgetClock, installFrames, installHandoffs, largeDiagramEdges, largeDiagramNodes, runUntil, settle, trackArrayReads } from "../🧰️harness/🟦️.ts";

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
    installBudgetClock(0.001);
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
      const node: DiagramForceNode = {
        get id() {
          identityReads += 1;
          return id;
        },
        x: index === 0 ? -100 : index === 1 ? 100 : 0,
        y: index === 2 ? 100 : 0,
      };
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
