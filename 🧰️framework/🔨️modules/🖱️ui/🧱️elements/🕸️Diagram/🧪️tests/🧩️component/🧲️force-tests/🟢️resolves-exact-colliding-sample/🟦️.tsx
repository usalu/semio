/** 🟢️ The resolves exact colliding sample tests of the owned Diagram force suite. */
import { it, expect } from "vitest";
import { type DiagramForceNode } from "../../../../📋️force-model/🟦️.ts";
import { createDiagramForceSimulation } from "../../../../🧮️force-simulation/🟦️.ts";
import { Diagram } from "../../../../🟦️.tsx";
import { act } from "@testing-library/react";
import { renderToString } from "react-dom/server";
import { forceConfig, installBudgetClock, installFrames, runUntil } from "../../🧰️harness/🟦️.ts";

/** 🔵 Registers the resolves exact colliding sample tests of the owned Diagram force suite. */
export function registerResolvesExactCollidingSampleTests(): void {
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
}
