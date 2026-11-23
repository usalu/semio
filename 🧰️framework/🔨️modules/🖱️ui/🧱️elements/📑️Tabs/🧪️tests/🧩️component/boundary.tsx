import * as React from "react";
import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "../../🟦️.tsx";

describe("tabs painted boundary discovery", () => {
  it.each(["horizontal", "vertical"] as const)("registers only painted chips in a %s group", (orientation) => {
    const { getByRole } = render(<Tabs defaultValue="alpha" orientation={orientation}><TabsList><TabsTrigger value="alpha">Alpha</TabsTrigger><TabsTrigger value="beta">Beta</TabsTrigger></TabsList></Tabs>);
    const list = getByRole("tablist");
    const alpha = getByRole("tab", { name: "Alpha" });
    const beta = getByRole("tab", { name: "Beta" });
    expect(list.getAttribute("data-shared-border-group")).toBe("");
    expect(list.getAttribute("data-orientation")).toBe(orientation);
    expect(list.hasAttribute("data-shared-border-frame")).toBe(false);
    expect(alpha.getAttribute("data-shared-border-edges")).toBe(orientation === "horizontal" ? "right bottom left" : "top right bottom");
    expect(alpha.getAttribute("data-shared-border-frame")).toBe("");
    expect(alpha.getAttribute("data-shared-border-state")).toBe("active");
    expect(alpha.className.split(" ")).toContain("relative");
    expect(beta.hasAttribute("data-shared-border-frame")).toBe(false);
    expect(list.querySelector("[data-shared-border-cell], [data-shared-border-attached]")).toBeNull();
    alpha.focus();
    fireEvent.keyDown(alpha, { key: orientation === "horizontal" ? "ArrowRight" : "ArrowDown" });
    expect(document.activeElement).toBe(beta);
    expect(beta.getAttribute("data-shared-border-frame")).toBe("");
    expect(alpha.hasAttribute("data-shared-border-frame")).toBe(false);
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
