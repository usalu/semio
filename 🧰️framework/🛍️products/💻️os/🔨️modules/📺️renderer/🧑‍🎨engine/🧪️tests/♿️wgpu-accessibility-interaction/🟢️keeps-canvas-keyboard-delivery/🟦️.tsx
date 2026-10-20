/** 🟢️ The keeps canvas keyboard delivery tests of the wgpu accessibility interaction contract suite. */
import { act } from "react";
import { expect, it } from "vitest";
import { fixture, mountOracle, staleReason } from "../🟣️shared/🟦️.tsx";

/** 🔵 Registers the keeps canvas keyboard delivery tests of the wgpu accessibility interaction contract suite. */
export function registerKeepsCanvasKeyboardDeliveryTests(): void {
  it("keeps canvas keyboard delivery independent from mirror activation", () => {
    const { container, canvas, sent } = mountOracle();
    let canvasKeys = 0;
    canvas.addEventListener("keydown", () => canvasKeys++);
    const button = container.querySelector<HTMLButtonElement>('[data-node-key="#apply"]')!;
    act(() => button.dispatchEvent(new KeyboardEvent("keydown", { key: "Enter", bubbles: true })));
    expect(canvasKeys).toBe(fixture.exactOnce.canvasKeyEvents);
    expect(sent.filter((event) => event.kind === "accessibility-activate")).toHaveLength(0);
    act(() => button.click());
    expect(sent.filter((event) => event.kind === "accessibility-activate")).toHaveLength(fixture.exactOnce.retainedActivateActions);
    act(() => canvas.dispatchEvent(new KeyboardEvent("keydown", { key: "f", bubbles: true })));
    expect(canvasKeys).toBe(1);
    expect(sent.filter((event) => event.kind === "accessibility-activate")).toHaveLength(1);
  });
  it("rejects retired windows generations and reused node ids before dispatch", () => {
    for (const row of fixture.rejections) expect(staleReason(row.wire), row.id).toBe(row.reason);
    for (const row of fixture.events) expect(staleReason(row.wire), row.id).toBeNull();
    expect(fixture.exactOnce.staleActivateActions).toBe(0);
  });
}
