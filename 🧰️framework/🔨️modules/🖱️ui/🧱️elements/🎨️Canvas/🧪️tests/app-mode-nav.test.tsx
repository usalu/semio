import * as React from "react";
import { cleanup, render } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { App } from "@semio-tech/ui-react";

afterEach(cleanup);

describe("App mode navigation band", () => {
  it("owns its bottom edge without padding or a gap around the framed mode selector", () => {
    const { container } = render(<App modes={[{ id: "a", label: "A", children: <div /> }, { id: "b", label: "B", children: <div /> }]} activeModeId="a" onActiveModeChange={vi.fn()} />);
    const nav = container.querySelector<HTMLElement>('[data-slot="app-mode-nav"]')!;
    expect(nav.dataset.sharedBorderEdges).toBe("bottom");
    expect(nav.hasAttribute("data-shared-border-frame")).toBe(true);
    expect(nav.hasAttribute("data-shared-border-cells")).toBe(true);
    expect(nav.className).not.toMatch(/(^|\s)(p|px|py|gap)-/);
    expect(nav.querySelector(":scope > [data-shared-border-frame]")).not.toBeNull();
  });
});
