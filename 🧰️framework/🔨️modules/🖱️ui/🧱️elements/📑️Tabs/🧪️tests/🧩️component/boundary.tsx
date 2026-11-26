import * as React from "react";
import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "../../🟦️.tsx";
import { SurfaceScope, useSurface } from "../../../🌈️Surface/🟦️.tsx";
import { Badge, StatusChip } from "../../../../🎯️targets/⚛️react/🚦️status-surface/🟦️.tsx";

function SurfaceProbe() { const surface = useSurface(); return <span data-testid="surface-probe">{surface?.level}:{surface?.fill}</span>; }

describe("tabs painted boundary discovery", () => {
  it("publishes the next glass level to tab descendants", () => {
    const { getByRole, getByTestId } = render(<SurfaceScope level="window" fill="surface"><Tabs defaultValue="alpha"><TabsList><TabsTrigger value="alpha"><SurfaceProbe /></TabsTrigger></TabsList></Tabs></SurfaceScope>);
    expect(getByRole("tablist").getAttribute("data-level")).toBe("pane");
    expect(getByTestId("surface-probe").textContent).toBe("pane:glass");
  });

  it("paints badges and status chips at the next glass level with semantic ink", () => {
    const { container, getByTestId } = render(<SurfaceScope level="window" fill="surface"><Badge tone="neutral" text="Ready" icon={<SurfaceProbe />} /><Badge tone="danger" text="Error" /><StatusChip status="ok" label="Online" /></SurfaceScope>);
    const chips = [...container.querySelectorAll("[data-glass-chip]")];
    expect(chips).toHaveLength(3);
    expect(chips.every(chip => chip.getAttribute("data-level") === "pane" && chip.classList.contains("ui-glass"))).toBe(true);
    expect(chips[0].className).not.toContain("bg-muted");
    expect(chips[1].className).toContain("text-destructive-foreground");
    expect(getByTestId("surface-probe").textContent).toBe("pane:glass");
  });
  it.each(["horizontal", "vertical"] as const)("registers the list as a cells chip whose tabs are its cells in a %s group", (orientation) => {
    const { getByRole } = render(<Tabs defaultValue="alpha" orientation={orientation}><TabsList><TabsTrigger value="alpha">Alpha</TabsTrigger><TabsTrigger value="beta">Beta</TabsTrigger></TabsList></Tabs>);
    const list = getByRole("tablist");
    const alpha = getByRole("tab", { name: "Alpha" });
    const beta = getByRole("tab", { name: "Beta" });
    expect(list.getAttribute("data-orientation")).toBe(orientation);
    expect(list.hasAttribute("data-shared-border-cells")).toBe(true);
    expect(list.getAttribute("data-shared-border-edges")).toBe("right bottom");
    expect([alpha, beta].every((tab) => tab.hasAttribute("data-shared-border-frame") && tab.getAttribute("data-shared-border-edges") === "")).toBe(true);
    expect(alpha.getAttribute("data-shared-border-state")).toBe("active");
    expect(beta.hasAttribute("data-shared-border-state")).toBe(false);
    expect(alpha.className.split(" ")).toContain("relative");
    alpha.focus();
    fireEvent.keyDown(alpha, { key: orientation === "horizontal" ? "ArrowRight" : "ArrowDown" });
    expect(document.activeElement).toBe(beta);
    expect(beta.getAttribute("data-shared-border-state")).toBe("active");
    expect(alpha.hasAttribute("data-shared-border-state")).toBe(false);
  });

  it("draws the inner chip edges on the end side in right-to-left flow", () => {
    const { getByRole } = render(<Tabs defaultValue="alpha" dir="rtl"><TabsList><TabsTrigger value="alpha">Alpha</TabsTrigger></TabsList></Tabs>);
    expect(getByRole("tablist").getAttribute("data-shared-border-edges")).toBe("left bottom");
  });

  it("publishes the list extent as the clearance of the panel, however the parts are attached", () => {
    const rect = vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockImplementation(function (this: HTMLElement) { return this.getAttribute("role") === "tablist" ? new DOMRect(0, 0, 120, 28) : new DOMRect(); });
    try {
      const { container } = render(<Tabs defaultValue="alpha"><TabsList><TabsTrigger value="alpha">Alpha</TabsTrigger></TabsList><TabsContent value="alpha">Body</TabsContent></Tabs>);
      expect(container.querySelector<HTMLElement>('[data-slot="tabs"]')!.style.getPropertyValue("--tabs-list-clearance")).toBe("28px");
    } finally { rect.mockRestore(); }
  });

  it("is one rectangle: the tab list is a glass chip inside it and the panel spans the whole frame under the list", () => {
    const { getByRole, container } = render(<Tabs defaultValue="alpha"><TabsList><TabsTrigger value="alpha">Alpha</TabsTrigger></TabsList><TabsContent value="alpha">Body</TabsContent></Tabs>);
    const root = container.querySelector<HTMLElement>('[data-slot="tabs"]')!;
    const list = getByRole("tablist");
    const panel = getByRole("tabpanel");
    expect(root.getAttribute("data-shared-border-edges")).toBe("top right bottom left");
    expect(root.contains(list)).toBe(true);
    expect(list.className.split(" ")).toContain("absolute");
    expect(list.hasAttribute("data-tabs-chip")).toBe(true);
    expect(list.getAttribute("data-glass-chip")).toBe("");
    expect(list.className.split(" ")).toContain("ui-glass");
    expect(panel.parentElement).toBe(root);
    expect(panel.style.paddingBlockStart).toBe("var(--tabs-list-clearance, 0px)");
  });

  it("keeps the active chip flush with its list and the list flush with the panel", () => {
    const { getByRole, container } = render(<Tabs defaultValue="alpha"><TabsList><TabsTrigger value="alpha">Alpha</TabsTrigger><TabsTrigger value="beta">Beta</TabsTrigger></TabsList></Tabs>);
    const list = getByRole("tablist");
    const alpha = getByRole("tab", { name: "Alpha" });
    expect(list.className.split(" ")).toContain("p-0");
    expect(list.className).not.toMatch(/(^|\s)p-single(\s|$)/);
    expect(alpha.className).not.toContain("calc(100%");
    expect(container.querySelector('[data-slot="tabs"]')!.className.split(" ")).toContain("gap-0");
  });
});
