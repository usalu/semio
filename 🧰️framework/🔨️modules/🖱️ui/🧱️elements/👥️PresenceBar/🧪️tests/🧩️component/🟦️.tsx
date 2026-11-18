import * as React from "react";
import { cleanup, render } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { PresenceBar } from "../../🟦️.tsx";
import spacing from "../../../📻️TableAvatar/🧪️tests/🧫️fixtures/📐️spacing/🔣️.json";

vi.mock("../../../🏷️Label/🟦️.tsx", () => ({ useLabel: (key: string) => key }));
vi.mock("@semio-tech/ui-styling", () => ({ currentStylingAppearanceName: () => "light" }));
afterEach(cleanup);

describe("PresenceBar spacing", () => {
  it("separates readable peers and equally sized overflow without losing semantics", () => {
    const { container, getByRole } = render(<PresenceBar max={1} peers={[
      { actor: "ada", label: "Ada Lovelace", isAgent: true, activity: { text: "Editing", badge: "E" } },
      { actor: "grace", label: "Grace Hopper" },
    ]} />);
    const roster = getByRole("list");
    expect(spacing.rosterGapClass).toBeNull();
    expect(roster.className).not.toMatch(/\bgap-|-space-x/);
    expect(roster.hasAttribute("data-shared-border-cells")).toBe(true);
    expect(roster.hasAttribute("data-shared-border-frame")).toBe(true);
    const avatar = container.querySelector<HTMLElement>('[data-slot="avatar"]')!;
    const peer = container.querySelector<HTMLElement>('[data-row-id="peer:ada"]')!;
    const overflow = container.querySelector<HTMLElement>('[data-row-id="peer:overflow"]')!;
    expect(avatar.classList.contains(spacing.sizeClass)).toBe(true);
    expect(avatar.classList.contains("border-0")).toBe(true);
    expect(avatar.classList.contains("border")).toBe(false);
    expect(avatar.style.boxShadow).toContain("2px");
    expect(peer.hasAttribute("data-shared-border-cell")).toBe(true);
    expect(overflow.hasAttribute("data-shared-border-cell")).toBe(true);
    expect(overflow.className).not.toMatch(/\bborder\b|rounded-full/);
    for (const badge of container.querySelectorAll<HTMLElement>('[data-row-id^="peer-agent-badge"], [data-row-id^="peer-activity-badge"]')) expect(badge.className).not.toMatch(/\bborder\b/);
    expect(roster.querySelectorAll(":scope > :not([data-shared-border-cell])").length).toBe(0);
    expect(overflow.classList.contains(spacing.sizeClass)).toBe(true);
    expect(overflow.classList.contains(spacing.textClass)).toBe(true);
    expect(overflow.textContent).toBe("+1");
    expect(peer.tabIndex).toBe(0);
    expect(overflow.tabIndex).toBe(0);
    expect(peer.dataset.presenceKind).toBe("agent");
    expect(peer.getAttribute("aria-label")).toContain("Editing");
    expect(container.querySelector('[data-row-id="peer-agent-badge:ada"]')).not.toBeNull();
    expect(container.querySelector('[data-row-id="peer-activity-badge:ada"]')).not.toBeNull();
  });
});
