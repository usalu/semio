// #region 🔌️Adapters
import { fireEvent, render, screen, within } from "@testing-library/react";
import * as React from "react";
import { describe, expect, it, vi } from "vitest";
import { Surface, useSurface } from "../../../🌈️Surface/🟦️.tsx";
import { OverviewCard, OverviewCardAction, OverviewCardOpenChip } from "../../🟦️.tsx";
// #endregion 🔌️Adapters

// #region 🃏️OverviewCardAnatomy
/** 🃏️ The card's DOM contract: an owner-level, never-active window chrome whose stack stays transparent, the title in the
 * cap chip, the body in `<slot>-content`, and — by shape — either a named region with real footer buttons or one button. */
describe("OverviewCard", () => {
  it("makes a flush card's content a vertical shared-border group without body padding", () => {
    render(
      <OverviewCard as="section" slot="quiz-card" headingId="flush-title" icon={null} title="Flush" flush>
        <div data-shared-border-cell="">one</div>
        <div data-shared-border-cell="">two</div>
      </OverviewCard>,
    );
    const region = screen.getByRole("region", { name: "Flush" });
    const content = region.querySelector('[data-slot="quiz-card-content"]')!;
    expect(content.getAttribute("data-shared-border-group")).toBe("");
    expect(content.getAttribute("data-orientation")).toBe("vertical");
    expect(region.querySelector('[data-slot="window-chrome-body"]')?.className ?? "").not.toContain("p-double");
  });

  it("leaves an ordinary card padded and ungrouped", () => {
    render(
      <OverviewCard as="section" slot="quiz-card" headingId="plain-title" icon={null} title="Plain">
        <p>text</p>
      </OverviewCard>,
    );
    const content = screen.getByRole("region", { name: "Plain" }).querySelector('[data-slot="quiz-card-content"]')!;
    expect(content.hasAttribute("data-shared-border-group")).toBe(false);
    expect(content.hasAttribute("data-orientation")).toBe(false);
  });

  it("renders a section named by its heading, with the chrome at the page level and real footer buttons", () => {
    const onStart = vi.fn();
    const onLast = vi.fn();
    render(
      <OverviewCard
        as="section"
        slot="quiz-card"
        headingId="heating-title"
        headingLevel={3}
        icon={<span aria-hidden="true">🔥</span>}
        title="Heating"
        data={{ "data-card": "heating" }}
        footerLeft={<OverviewCardAction onClick={onLast}>Last result</OverviewCardAction>}
        footerRight={
          <OverviewCardAction primary onClick={onStart}>
            Play again
          </OverviewCardAction>
        }
      >
        <p>U-values from single glazing to the passive-house roof.</p>
      </OverviewCard>,
    );
    const region = screen.getByRole("region", { name: "Heating" });
    expect(region.tagName).toBe("SECTION");
    expect(region.getAttribute("data-card")).toBe("heating");
    expect(region.hasAttribute("data-overview-card")).toBe(true);
    const heading = within(region).getByRole("heading", { level: 3, name: "Heating" });
    expect(heading.id).toBe("heating-title");
    const chip = heading.closest('[data-slot="quiz-card-title-chip"]')!;
    expect(chip.className.split(" ")).toContain("max-w-full");
    expect(chip.className).not.toContain("max-w-[12rem]");
    const stack = region.querySelector('[data-slot="quiz-card-stack"]');
    expect(stack?.hasAttribute("data-overview-card-stack")).toBe(true);
    expect(stack?.hasAttribute("data-shared-border-hover")).toBe(true);
    expect(stack?.getAttribute("data-level")).toBe("base");
    expect(stack?.hasAttribute("data-active")).toBe(false);
    expect(region.querySelector('[data-slot="quiz-card-content"]')?.textContent).toContain("passive-house roof");
    const buttons = within(region).getAllByRole("button");
    expect(buttons.map((button) => button.textContent?.trim())).toEqual(["Last result", "Play again"]);
    expect(within(region).getByRole("button", { name: "Play again" })).toBe(buttons[1]);
    for (const button of buttons) expect((button as HTMLButtonElement).type).toBe("button");
    expect(buttons[0]!.closest('[data-slot="window-chrome-footer-left"]')).not.toBeNull();
    expect(buttons[1]!.closest('[data-slot="window-chrome-footer-right"]')).not.toBeNull();
    for (const button of buttons) {
      const chipOverlay = button.closest("[data-window-frame-chip]")!;
      expect(chipOverlay.closest("[data-window-frame]")).toBe(stack);
    }
    expect(heading.closest("[data-window-frame-chip]")?.closest("[data-window-frame]")).toBe(stack);
    expect(stack?.querySelector("[data-window-frame-content]")?.contains(region.querySelector('[data-slot="quiz-card-content"]'))).toBe(true);
    expect(buttons[1]!.getAttribute("data-overview-card-action")).toBe("primary");
    expect(buttons[1]!.querySelector('[data-icon="chevron-right"]')).not.toBeNull();
    expect(buttons[0]!.querySelector('[data-icon="chevron-right"]')).toBeNull();
    expect(buttons[1]!.style.minHeight).toBe("var(--chip-row-height)");
    fireEvent.click(buttons[1]!);
    fireEvent.click(buttons[0]!);
    expect(onStart).toHaveBeenCalledTimes(1);
    expect(onLast).toHaveBeenCalledTimes(1);
  });

  it("inherits the painted window for in-flow cards and raises only its edge chips", () => {
    function Probe() {
      const surface = useSurface();
      return <output>{surface?.level}/{surface?.fill}</output>;
    }
    const { container } = render(
      <Surface level="window" fill="glass">
        <OverviewCard as="section" slot="unit" headingId="task" icon={null} title="Task">
          <Probe />
        </OverviewCard>
      </Surface>,
    );
    expect(container.querySelector("output")?.textContent).toBe("window/glass");
    expect(container.querySelector('[data-slot="window-chrome-body-surface"]')).toBeNull();
    expect(container.querySelector("[data-window-frame-chip]")?.getAttribute("data-level")).toBe("pane");
  });

  it("defaults the section heading to level two", () => {
    render(
      <OverviewCard as="section" slot="card" headingId="intro" icon={null} title="How it works">
        <p>Classify, sort and match.</p>
      </OverviewCard>,
    );
    expect(screen.getByRole("heading", { level: 2, name: "How it works" }).id).toBe("intro");
  });

  it("renders a whole-card button whose name is its label, with a non-interactive open chip", () => {
    const onClick = vi.fn();
    render(
      <OverviewCard
        as="button"
        slot="play-pane-card"
        label="Open Home: every app in one place"
        data={{ "data-play-pane-card": "", "data-pane-id": "home" }}
        onClick={onClick}
        icon={<span aria-hidden="true">🏠</span>}
        title="Home"
        footerRight={<OverviewCardOpenChip slot="play-pane-card">Open</OverviewCardOpenChip>}
      >
        <span data-slot="play-pane-card-tagline">Every app in one place</span>
      </OverviewCard>,
    );
    const button = screen.getByRole("button", { name: "Open Home: every app in one place" });
    expect(screen.getAllByRole("button")).toHaveLength(1);
    expect(button.getAttribute("data-pane-id")).toBe("home");
    expect(button.hasAttribute("data-play-pane-card")).toBe(true);
    expect(screen.queryByRole("heading")).toBeNull();
    expect(button.querySelector('[data-slot="play-pane-card-open-chip"]')?.textContent?.trim()).toBe("Open");
    expect(button.querySelector("[data-overview-card-paint]")?.className).not.toMatch(/translate|transition-transform/u);
    fireEvent.click(button);
    expect(onClick).toHaveBeenCalledTimes(1);
  });
});
// #endregion 🃏️OverviewCardAnatomy
