// #region 🔌️Adapters
import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { LevelProvider } from "../../../🌈️Surface/🟦️.tsx";
import { CanvasSkeleton } from "../../🟦️.tsx";
// #endregion 🔌️Adapters

// #region 🦴️CanvasSkeleton
describe("CanvasSkeleton", () => {
  it("renders two even window-chrome silhouettes instead of a single rectangle", () => {
    const { container } = render(
      <LevelProvider level="base">
        <div className="h-64 w-full">
          <CanvasSkeleton label="Loading" />
        </div>
      </LevelProvider>,
    );
    expect(container.querySelectorAll('[data-slot="canvas-skeleton-stack"]')).toHaveLength(2);
    expect(container.querySelectorAll('[data-window-silhouette-border]')).toHaveLength(2);
    expect(container.querySelectorAll('[data-window-silhouette-chip]')).toHaveLength(2);
    expect(container.querySelector('[data-slot="canvas-skeleton-stack"][data-active="true"]')).toBeTruthy();
    expect(container.querySelector(".flex-row.gap-single")?.children).toHaveLength(2);
    expect(container.querySelector('[role="status"][aria-busy="true"]')).toBeTruthy();
  });
});
// #endregion 🦴️CanvasSkeleton
