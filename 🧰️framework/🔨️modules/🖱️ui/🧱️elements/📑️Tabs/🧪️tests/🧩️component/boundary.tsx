import * as React from "react";
import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { Tabs, TabsList, TabsTrigger } from "../../🟦️.tsx";

describe("tabs painted boundary discovery", () => {
  it.each(["horizontal", "vertical"] as const)("registers only painted chips in a %s group", (orientation) => {
    const { getByRole } = render(<Tabs defaultValue="alpha" orientation={orientation}><TabsList><TabsTrigger value="alpha">Alpha</TabsTrigger><TabsTrigger value="beta">Beta</TabsTrigger></TabsList></Tabs>);
    const list = getByRole("tablist");
    const alpha = getByRole("tab", { name: "Alpha" });
    const beta = getByRole("tab", { name: "Beta" });
    expect(list.getAttribute("data-shared-border-group")).toBe("");
    expect(list.getAttribute("data-orientation")).toBe(orientation);
    expect(list.hasAttribute("data-shared-border-frame")).toBe(false);
    expect(alpha.getAttribute("data-shared-border-edges")).toBe("top right bottom left");
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
});
