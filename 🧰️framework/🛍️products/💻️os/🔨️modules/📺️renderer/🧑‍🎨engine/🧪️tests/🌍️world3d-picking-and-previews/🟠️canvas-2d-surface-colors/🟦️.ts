/** 🟠️ The canvas 2d surface colors tests of the framework renderer hosts suite. */
import { expect, it } from "vitest";
import { readCanvas2dSurfaceColors } from "../../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";

/** 🟡 Registers the canvas 2d surface colors tests of the framework renderer hosts suite. */
export function registerCanvas2dSurfaceColorsTests(): void {
  it("canvas-2d surface colors follow the light theme canvas token instead of a hardcoded dark fill", () => {
    document.documentElement.classList.remove("dark");
    const colors = readCanvas2dSurfaceColors();
    expect(colors.clear).toBe("rgba(240, 236, 221, 1)");
    expect(colors.clear.toLowerCase()).not.toContain("17, 19, 24");
    expect(colors.grid).toMatch(/^rgba\(/);
  });
}
