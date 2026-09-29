// #region 🔌️Adapters
import { fireEvent, render, screen, within } from "@testing-library/react";
import * as React from "react";
import { describe, expect, it, vi } from "vitest";
import { OverviewCard, OverviewCardAction, OverviewCardOpenChip } from "../../🟦️.tsx";
// #endregion 🔌️Adapters

// #region 🃏️OverviewCardAnatomy
/** 🃏️ The card's DOM contract: a dialog-level, never-active window chrome whose stack stays transparent, the title in the
 * cap chip, the body in `<slot>-content`, and — by shape — either a named region with real footer buttons or one button. */
describe("OverviewCard", () => {
  it("renders a section named by its heading, with the chrome at dialog level and real footer buttons", () => {
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
    expect(stack?.getAttribute("data-level")).toBe("dialog");
    expect(stack?.hasAttribute("data-active")).toBe(false);
    expect(region.querySelector('[data-slot="quiz-card-content"]')?.textContent).toContain("passive-house roof");
    const buttons = within(region).getAllByRole("button");
    expect(buttons.map((button) => button.textContent?.trim())).toEqual(["Last result", "Play again"]);
    expect(within(region).getByRole("button", { name: "Play again" })).toBe(buttons[1]);
    for (const button of buttons) expect((button as HTMLButtonElement).type).toBe("button");
    expect(buttons[0]!.closest('[data-slot="window-chrome-footer-left"]')).not.toBeNull();
    expect(buttons[1]!.closest('[data-slot="window-chrome-footer-right"]')).not.toBeNull();
    expect(buttons[1]!.getAttribute("data-overview-card-action")).toBe("primary");
    expect(buttons[1]!.querySelector('[data-icon="chevron-right"]')).not.toBeNull();
    expect(buttons[0]!.querySelector('[data-icon="chevron-right"]')).toBeNull();
    expect(buttons[1]!.style.minHeight).toContain("1.5rem");
    fireEvent.click(buttons[1]!);
    fireEvent.click(buttons[0]!);
    expect(onStart).toHaveBeenCalledTimes(1);
    expect(onLast).toHaveBeenCalledTimes(1);
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
    const { rerender } = render(
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
    expect(button.className).not.toContain(" -translate-y-0.5");
    fireEvent.click(button);
    expect(onClick).toHaveBeenCalledTimes(1);
    rerender(
      <OverviewCard as="button" slot="play-pane-card" lifted onClick={onClick} icon={null} title="Home">
        <span>Every app in one place</span>
      </OverviewCard>,
    );
    expect(screen.getByRole("button").className.split(" ")).toContain("-translate-y-0.5");
  });
});
// #endregion 🃏️OverviewCardAnatomy
