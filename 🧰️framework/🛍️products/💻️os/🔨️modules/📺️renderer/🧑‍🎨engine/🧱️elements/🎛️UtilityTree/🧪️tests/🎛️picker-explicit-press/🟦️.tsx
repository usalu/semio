/** 🎛️ `UtilityTree`'s picker and toggle runs emit a utility (de)activation ONLY from an explicit press on
 * the item the user pressed (ticket 26/09/16/INPUT-CAUSALITY-LEDGER, design §2 E). What this replaced: the
 * single picker turned any `onValueChange("")` — the shape a controlled-value resync takes as much as a
 * toggle-off click — into `setActiveUtility { utilityId: "" }`, and the toggle run diffed EVERY entry's
 * `pressed` against the group's next value set, so a resync that differed for several entries dispatched
 * several actions. The shell used to swallow the phantom with an 8 s wall-clock echo-off, which also
 * swallowed the user's real toggle-off; the versioned register that replaces it needs the picker to send
 * the `expectedGeneration` (and `windowId`) it rendered, unchanged, on every dispatch.
 *   1. a controlled resync (the tree re-publishes; the pressed chip's value goes to `""`) dispatches nothing;
 *   2. an explicit press on the pressed chip dispatches exactly ONE `setActiveUtility` whose args carry the
 *      template's `windowId` and `expectedGeneration` with `utilityId: ""`;
 *   3. a press on an unpressed chip only drills the ribbon; folding a sibling chip that does not own the
 *      pressed leaf dispatches nothing;
 *   4. in a toggle run, a press dispatches that toggle's own descriptor (args untouched) and nothing else;
 *      a re-render whose pressed set differs for several entries dispatches nothing.
 */
import { cleanup, fireEvent, render } from "@semio-tech/ui-react/test";
import { createElement } from "react";
import { afterEach, describe, expect, it } from "vitest";
import type { ActionDescriptor, UtilityNode } from "@semio-tech/framework";
import { UtilityTree } from "../../🟦️.tsx";
import Ajv from "ajv";
import paletteFixture from "../../🧫️fixtures/📐️bounded-palette/🔣️.json";
import { createShellScope, ShellScopeProvider } from "@semio-tech/ui-react";
import { createMemoryStoragePort } from "@semio-tech/framework";
import { I18nextProvider } from "react-i18next";
import { SelectionUtilityOptions } from "../../../🛠️ShellHelpers/🟦️.tsx";
import { within } from "@testing-library/react";

const WINDOW_ID = "fem2d-window-1";
const TREE_ID = `ui.utilities.${WINDOW_ID}`;

const setActive = (utilityId: string, expectedGeneration: number): ActionDescriptor => ({
  controllerId: "fem2d",
  action: "setActiveUtility",
  args: { utilityId, windowId: WINDOW_ID, expectedGeneration },
});

describe("🎛️ Bounded utility palette", () => {
  afterEach(() => cleanup());

  it("validates the language-neutral end-user palette contract independently", () => {
  });

  for (const locale of paletteFixture.selection.locales) {
    it(`exposes mutually exclusive, named selection choices in ${locale}`, () => {
      const dispatched: ActionDescriptor[] = [];
      const scope = createShellScope({ storage: createMemoryStoragePort(), initialLocale: locale as "en" | "de" });
      const view = render(createElement(ShellScopeProvider, { scope, children: createElement(I18nextProvider, { i18n: scope.i18n }, createElement(SelectionUtilityOptions, {
        activeUtilityId: "selectMarquee", windowId: WINDOW_ID, generation: 13, onAction: action => dispatched.push(action),
      })) }));
      const groups = within(view.container).getAllByRole("radiogroup");
      expect(groups).toHaveLength(2);
      for (const group of groups) expect(group.getAttribute("aria-label")?.length).toBeGreaterThan(0);
      const choices = within(view.container).getAllByRole("radio");
      expect(choices).toHaveLength(paletteFixture.selection.methods.length + paletteFixture.selection.merges.length);
      fireEvent.click(choices[1]!);
      expect(dispatched).toEqual([{ controllerId: "window", action: "setActiveUtility", args: { windowId: WINDOW_ID, utilityId: "selectLasso", expectedGeneration: 13 } }]);
      for (const [index, mode] of paletteFixture.selection.merges.entries()) {
        fireEvent.click(choices[2 + index]!);
        expect(scope.selection.get()).toBe(mode);
        expect(choices[2 + index]!.getAttribute("aria-checked")).toBe("true");
      }
      expect(dispatched).toHaveLength(paletteFixture.expected.activationCount);
    });
  }

  for (const direction of paletteFixture.directions) {
    for (const width of paletteFixture.widths) {
      it(`preserves accessible tool activation and disabled tools in a ${width}px ${direction} pane`, () => {
        const dispatched: ActionDescriptor[] = [];
        const utilities = paletteFixture.utilities.map((entry, order) => ({
          id: entry.id, kind: "toggle" as const, iconId: "mouse-pointer", text: entry.label,
          order, pressed: false, disabled: entry.disabled, onChange: setActive(entry.id, 13),
        })) as readonly UtilityNode[];
        const view = render(createElement("div", { style: { width } }, createElement(UtilityTree, {
          utilities, id: TREE_ID, direction: direction as "up" | "down", onAction: action => dispatched.push(action),
        })));
        for (const entry of paletteFixture.utilities) {
          const control = view.getByRole("button", { name: entry.label, exact: true }) as HTMLButtonElement;
          expect(control.disabled).toBe(entry.disabled ?? false);
          fireEvent.click(control);
          const expected = entry.disabled ? [] : [setActive(entry.id, 13)];
          expect(dispatched).toEqual(expected);
          dispatched.length = 0;
        }
      });
    }
  }
});

/** 🌳️ Two collections; the pressed leaf lives under `pick`. `pickDirect` deliberately does not start with
 * `select`, which keeps `SelectionUtilityOptions` (a shell-owned row) out of this fixture. */
function tree(options: { readonly pressed?: string | null; readonly generation?: number; readonly pickDisabled?: boolean } = {}): readonly UtilityNode[] {
  const pressed = options.pressed === undefined ? "pickDirect" : options.pressed;
  const generation = options.generation ?? 3;
  return [
    {
      id: "pick",
      kind: "collection",
      iconId: "mouse-pointer",
      text: "Pick",
      order: 0,
      disabled: options.pickDisabled,
      children: [
        { id: "pickDirect", kind: "toggle", iconId: "mouse-pointer", text: "Direct", order: 0, pressed: pressed === "pickDirect", onChange: setActive("pickDirect", generation) },
        { id: "pickLasso", kind: "toggle", iconId: "lasso", text: "Lasso", order: 1, pressed: pressed === "pickLasso", onChange: setActive("pickLasso", generation) },
      ],
    },
    {
      id: "draw",
      kind: "collection",
      iconId: "pencil",
      text: "Draw",
      order: 1,
      children: [{ id: "drawLine", kind: "toggle", iconId: "pencil", text: "Line", order: 0, pressed: pressed === "drawLine", onChange: setActive("drawLine", generation) }],
    },
  ] as unknown as readonly UtilityNode[];
}

const chip = (collectionId: string): HTMLButtonElement => document.getElementById(`${TREE_ID}.group.${collectionId}`) as HTMLButtonElement;
const toggle = (leafId: string): HTMLButtonElement => document.getElementById(leafId) as HTMLButtonElement;

function mount(utilities: readonly UtilityNode[]) {
  const dispatched: ActionDescriptor[] = [];
  const onAction = (action: ActionDescriptor) => {
    dispatched.push(action);
  };
  const view = render(createElement(UtilityTree, { utilities, onAction, id: TREE_ID, direction: "up" }));
  return { dispatched, view, rerender: (next: readonly UtilityNode[]) => view.rerender(createElement(UtilityTree, { utilities: next, onAction, id: TREE_ID, direction: "up" })) };
}

describe("🎛️ UtilityTree dispatches only from an explicit press", () => {
  afterEach(() => cleanup());

  it("dispatches nothing on a controlled resync that takes the pressed chip's value to empty", () => {
    const { dispatched, rerender } = mount(tree());
    expect(chip("pick").getAttribute("aria-pressed")).toBe("true");
    // 🔁️ The guest re-publishes the same tree under a new register generation: no dispatch.
    rerender(tree({ generation: 4 }));
    expect(dispatched.length).toBe(0);
    // 🔁️ The pressed leaf's collection goes away: the picker's value resyncs to "" — still no dispatch.
    rerender(tree({ pickDisabled: true }));
    expect(chip("pick")).toBeNull();
    expect(dispatched.length).toBe(0);
    // 🔁️ The guest clears the pressed leaf: no dispatch either.
    rerender(tree({ pressed: null }));
    expect(dispatched.length).toBe(0);
  });

  it("dispatches exactly one setActiveUtility carrying the template's windowId and expectedGeneration on an explicit press of the pressed chip", () => {
    const { dispatched } = mount(tree({ generation: 7 }));
    expect(chip("pick").getAttribute("aria-pressed")).toBe("true");
    fireEvent.click(chip("pick"));
    expect(dispatched.length).toBe(1);
    expect(dispatched[0]).toEqual({ controllerId: "fem2d", action: "setActiveUtility", args: { utilityId: "", windowId: WINDOW_ID, expectedGeneration: 7 } });
    // 🎀️ The ribbon folded with it, and the fold did not echo a second deactivation.
    expect(chip("pick").getAttribute("aria-pressed")).toBe("false");
    expect(dispatched.length).toBe(1);
  });

  it("only drills the ribbon on an unpressed chip, and folding a sibling that does not own the pressed leaf dispatches nothing", () => {
    const { dispatched } = mount(tree());
    fireEvent.click(chip("draw"));
    expect(chip("draw").getAttribute("aria-pressed")).toBe("true");
    expect(chip("pick").getAttribute("aria-pressed")).toBe("false");
    expect(dispatched.length).toBe(0);
    fireEvent.click(chip("draw"));
    expect(chip("draw").getAttribute("aria-pressed")).toBe("false");
    expect(dispatched.length).toBe(0);
  });

  it("dispatches one descriptor, args untouched, for the toggle the user pressed and nothing for a resync", () => {
    const { dispatched, rerender } = mount(tree({ generation: 11 }));
    expect(toggle("pickDirect").getAttribute("aria-pressed")).toBe("true");
    fireEvent.click(toggle("pickLasso"));
    expect(dispatched).toEqual([setActive("pickLasso", 11)]);
    // 🔁️ The guest answers with a pressed set that differs for TWO entries at once: nothing is dispatched.
    rerender(tree({ pressed: "pickLasso", generation: 12 }));
    expect(toggle("pickLasso").getAttribute("aria-pressed")).toBe("true");
    expect(toggle("pickDirect").getAttribute("aria-pressed")).toBe("false");
    expect(dispatched.length).toBe(1);
    // 🖱️ Pressing the pressed toggle dispatches its own descriptor once (the shell resolves it to a
    // deactivation against the register) — never a synthesized one.
    fireEvent.click(toggle("pickLasso"));
    expect(dispatched.length).toBe(2);
    expect(dispatched[1]).toEqual(setActive("pickLasso", 12));
  });
});
