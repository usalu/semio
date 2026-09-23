import { describe, expect, it } from "vitest";
import { canvas2dGumballTransformDelta, canvas2dGumballTransformStep, type Canvas2dGumballTransformPayload } from "../../🟦️GumballOverlay.tsx";

const camera = { x: 0, y: 0, zoom: 1 };
const drag = {
  kind: "moveX" as const,
  startScreen: { x: 100, y: 100 },
  startModelPivot: [0, 0] as const,
};

describe("canvas2dGumballTransformStep", () => {
  it("dispatches incremental translate deltas across pointer moves", () => {
    const first = canvas2dGumballTransformStep("moveX", drag, 120, 100, camera, 800, 600, ["n1"], null);
    expect(first?.dispatch.action).toBe("translateSelection");
    const dx1 = first?.dispatch.args.dx as number;
    expect(dx1).not.toBe(0);

    const second = canvas2dGumballTransformStep("moveX", drag, 140, 100, camera, 800, 600, ["n1"], first?.total ?? null);
    expect(second?.dispatch.args.dx).toBeCloseTo((second?.total.args.dx as number) - dx1, 9);

    const totalOnly = canvas2dGumballTransformDelta("moveX", drag, 140, 100, camera, 800, 600, ["n1"]);
    expect(second?.total.args.dx).toBeCloseTo(totalOnly?.args.dx as number, 9);
  });

  it("world space translate is the raw screen delta, not the fem2d scale", () => {
    const world = canvas2dGumballTransformDelta("moveX", drag, 120, 100, camera, 800, 600, ["f1"], "world");
    const fem = canvas2dGumballTransformDelta("moveX", drag, 120, 100, camera, 800, 600, ["f1"]);
    expect(world?.args.dy).toBe(0);
    expect(world?.args.dx).toBeCloseTo((fem?.args.dx as number) * 20, 6);
    const worldY = canvas2dGumballTransformDelta("moveY", { ...drag, kind: "moveY" }, 100, 120, camera, 800, 600, ["f1"], "world");
    const femY = canvas2dGumballTransformDelta("moveY", { ...drag, kind: "moveY" }, 100, 120, camera, 800, 600, ["f1"]);
    expect(worldY?.args.dy).toBeCloseTo(-((femY?.args.dy as number) * 20), 6);
  });

  it("turns cumulative rotation into per-step angle deltas", () => {
    const rotateDrag = { kind: "rotate" as const, startScreen: { x: 200, y: 100 }, startModelPivot: [0, 0] as const };
    const first = canvas2dGumballTransformStep("rotate", rotateDrag, 200, 120, camera, 800, 600, ["n1"], null);
    expect(Math.abs(first?.dispatch.args.angle as number)).toBeGreaterThan(0);
    const prev: Canvas2dGumballTransformPayload = first!.total;
    const second = canvas2dGumballTransformStep("rotate", rotateDrag, 200, 140, camera, 800, 600, ["n1"], prev);
    expect(Math.abs(second?.dispatch.args.angle as number)).toBeGreaterThan(0);
    expect((second?.total.args.angle as number) - (prev.args.angle as number)).toBeCloseTo(second?.dispatch.args.angle as number, 9);
  });
});
