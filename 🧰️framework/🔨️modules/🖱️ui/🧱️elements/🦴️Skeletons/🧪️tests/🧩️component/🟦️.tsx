// #region 🔌️Adapters
import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { LevelProvider } from "../../../🌈️Surface/🟦️.tsx";
import { CanvasSkeleton, PanelTreeSkeleton, elementSkeleton } from "../../🟦️.tsx";
// #endregion 🔌️Adapters

// #region 🦴️CanvasSkeleton
describe("CanvasSkeleton", () => {
  it("renders two even window frames, each one rectangle with its chip inside", () => {
    const { container } = render(
      <LevelProvider level="base">
        <div className="h-64 w-full">
          <CanvasSkeleton label="Loading" />
        </div>
      </LevelProvider>,
    );
    expect(container.querySelectorAll('[data-slot="canvas-skeleton-stack"]')).toHaveLength(2);
    expect(container.querySelectorAll('[data-window-frame-border][data-kind="loading"]')).toHaveLength(2);
    expect(container.querySelectorAll('[data-window-frame-chip]')).toHaveLength(2);
    for (const stack of container.querySelectorAll('[data-slot="canvas-skeleton-stack"]')) {
      expect(stack.hasAttribute("data-window-frame")).toBe(true);
      expect(stack.querySelector("[data-window-frame-content]")).not.toBeNull();
      expect(stack.querySelector("[data-window-frame-chip]")?.closest("[data-window-frame]")).toBe(stack);
      expect(stack.querySelector('[data-slot$="gap"]')).toBeNull();
    }
    expect(container.querySelector('[data-slot="canvas-skeleton-stack"][data-active="true"]')).toBeTruthy();
    const row = container.querySelector('[data-slot="canvas-skeleton-stack"]')!.closest<HTMLElement>(".flex-row")!;
    expect(row.className).not.toMatch(/\bgap-/);
    expect(row.children).toHaveLength(2);
    expect(container.querySelector('[role="status"][aria-busy="true"]')).toBeTruthy();
  });

  it("stacks placeholder rows and fields as gapless shared-border cells", () => {
    const tree = render(<PanelTreeSkeleton />).container.firstElementChild as HTMLElement;
    expect(tree.hasAttribute("data-shared-border-cells")).toBe(true);
    expect(tree.className).not.toMatch(/\b(gap|p)-/);
    expect(tree.querySelectorAll(":scope > [data-shared-border-cell]")).toHaveLength(3);
    for (const kind of ["field", "group"] as const) {
      const row = render(elementSkeleton(kind)).container.firstElementChild as HTMLElement;
      expect(row.hasAttribute("data-shared-border-cells")).toBe(true);
      expect(row.className).not.toMatch(/\bgap-/);
      expect(row.querySelectorAll(":scope > [data-shared-border-cell]").length).toBeGreaterThanOrEqual(2);
    }
  });
});
// #endregion 🦴️CanvasSkeleton
