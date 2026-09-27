// #region 🧲️Header
// framework/ui/elements/💡️ChromeControlHint/tests/component.tsx
// #endregion 🧲️Header

import { describe, expect, it, vi } from "vitest";
import { renderToStaticMarkup } from "react-dom/server";
import { act } from "react";
import { fireEvent, render } from "@testing-library/react";
import Ajv2020 from "ajv/dist/2020";
import tooltipFixture from "../../../../🧫️fixtures/💡️retained-control-tooltip/🔣️.json" with { type: "json" };
import tooltipSchema from "../../../../🧬️schema/💡️retained-control-tooltip/🔣️.json" with { type: "json" };
import { UiKeybindingsProvider } from "../../../../🔨️modules/🕹️control-keybinding-context/🟦️.tsx";
import { DEFAULT_UI_DRIVER, UiDriverProvider } from "../../../🚗️UiDriver/🟦️.tsx";
import { ChromeControlHint } from "../../🟦️.tsx";

describe("ChromeControlHint", () => {
  it("reveals the shared accepted label after dwell and dismisses immediately", () => {
    const validate = new Ajv2020({ strict: true }).compile(tooltipSchema);
    expect(validate(tooltipFixture), JSON.stringify(validate.errors)).toBe(true);
    vi.useFakeTimers();
    const placement = tooltipFixture.placements[0]!;
    const rect = (values: readonly number[]): DOMRect => ({ x: values[0]!, y: values[1]!, width: values[2]!, height: values[3]!, top: values[1]!, right: values[0]! + values[2]!, bottom: values[1]! + values[3]!, left: values[0]!, toJSON: () => ({}) });
    const bounds = vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockImplementation(function (this: HTMLElement): DOMRect {
      if (this.dataset.slot === "chrome-control-hint") return rect(placement.anchor);
      if (this.dataset.slot === "tooltip-content") return rect([0, 0, ...placement.content]);
      return rect([0, 0, 0, 0]);
    });
    const view = render(
      <UiDriverProvider driver={DEFAULT_UI_DRIVER}>
        <UiKeybindingsProvider bindings={new Map([["fixture.tooltip", "ctrl+w"]])}>
          <ChromeControlHint id="fixture.tooltip" text={tooltipFixture.text.label} always>
            <button type="button">Trigger</button>
          </ChromeControlHint>
        </UiKeybindingsProvider>
      </UiDriverProvider>,
    );
    try {
      const trigger = view.container.querySelector<HTMLElement>('[data-slot="chrome-control-hint"]')!;
      const button = view.getByRole("button", { name: tooltipFixture.text.label });
      fireEvent.pointerEnter(trigger);
      act(() => vi.advanceTimersByTime(tooltipFixture.timing.dwellMs - 1));
      expect(document.querySelector('[role="tooltip"]')).toBeNull();
      act(() => vi.advanceTimersByTime(1));
      const tooltip = document.querySelector<HTMLElement>('[role="tooltip"]');
      expect(tooltip?.textContent).toBe(tooltipFixture.text.formatted);
      expect(tooltip?.style.left).toBe(`${placement.expected.left}px`);
      expect(tooltip?.style.top).toBe(`${placement.expected.top}px`);
      expect(button.getAttribute("aria-describedby")).toBe(tooltip?.id);
      fireEvent.pointerLeave(trigger);
      expect(document.querySelector('[role="tooltip"]')).toBeNull();
      expect(button.hasAttribute("aria-describedby")).toBe(false);
    } finally {
      view.unmount();
      bounds.mockRestore();
      vi.useRealTimers();
    }
  });

  it("omits native title on the trigger when tooltip text resolves", () => {
    const markup = renderToStaticMarkup(
      <UiDriverProvider driver={DEFAULT_UI_DRIVER}>
        <ChromeControlHint id="ui.tree.drag.sort" always>
          <span data-slot="drag-handle">Grip</span>
        </ChromeControlHint>
      </UiDriverProvider>,
    );
    expect(markup).toContain('data-slot="chrome-control-hint"');
    expect(markup).not.toMatch(/data-slot="drag-handle"[^>]*\btitle=/);
  });
});
