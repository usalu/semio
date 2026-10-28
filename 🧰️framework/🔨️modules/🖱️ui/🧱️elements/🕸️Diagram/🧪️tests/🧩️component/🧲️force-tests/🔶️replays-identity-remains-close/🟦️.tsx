/** 🔶️ The replays identity remains close tests of the owned Diagram force suite. */
import { it, expect, vi } from "vitest";
import { type DiagramForceNode } from "../../../../📋️force-model/🟦️.ts";
import { createDiagramForceSimulation } from "../../../../🧮️force-simulation/🟦️.ts";
import { Diagram } from "../../../../🟦️.tsx";
import { act, render } from "@testing-library/react";
import { forceConfig, installBudgetClock, installFrames, installHandoffs, runUntil, settle } from "../../🧰️harness/🟦️.ts";

/** 🔷 Registers the replays identity remains close tests of the owned Diagram force suite. */
export function registerReplaysIdentityRemainsCloseTests(): void {
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
    installBudgetClock(0.0001);
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
}
