/** 🎚️ A measures-rail control must show the value the USER just asked for for as long as its own
 * dispatch is in flight, and hand authority back to the program the moment the program answers.
 *
 * 🕰️ Measured on the live `:6013` shell (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B12): a per-window
 * option's round trip — dispatch, guest, `WindowConfig` publication, `refreshUi`'s `measures` section —
 * takes 0.7 s on an idle app and several seconds on a busy one. A bare controlled control renders the
 * pre-click value for that whole window, so the gesture reads as ignored and the next click is aimed at
 * a value the program has already left behind. `WindowMeasureSlider` has always held a draft for
 * exactly this reason; these are the other two control kinds. */
import { cleanup, fireEvent, render } from "@semio-tech/ui-react/test";
import { createElement } from "react";
import { afterEach, describe, expect, it } from "vitest";
import { WindowMeasureSelect, WindowMeasureToggle } from "../../🧱️elements/🛠️ShellHelpers/🎚️measure-controls/🟦️.tsx";

type Dispatched = { readonly args?: Record<string, unknown> };

const toggleAction = { controllerId: "puzzle3d-play", action: "setGridVisible" } as const;
const selectAction = { controllerId: "puzzle3d-play", action: "setVortexShow" } as const;

const toggleMeasure = (pressed: boolean) => ({ kind: "toggle", id: "puzzle3d-play-grid-visible", iconId: "layout-grid", label: "Visible", pressed, onChange: toggleAction }) as never;
const selectMeasure = (value: string) =>
  ({
    kind: "select",
    id: "puzzle3d-play-vortex-show",
    label: "Vortex Show",
    value,
    items: [
      { id: "always", value: "always", label: "Always" },
      { id: "selected", value: "selected", label: "Selected" },
    ],
    onChange: selectAction,
  }) as never;

const checkbox = (): HTMLInputElement => document.getElementById("puzzle3d-play-grid-visible") as HTMLInputElement;
const trigger = (): HTMLElement => document.getElementById("puzzle3d-play-vortex-show") as HTMLElement;

export function testWindowMeasureControls(): void {
  describe("🎚️ window measure controls while their dispatch is in flight", () => {
    afterEach(() => cleanup());

    it("shows the toggle state the user asked for before the program has published it", () => {
      const dispatched: Dispatched[] = [];
      const onAction = (action: unknown) => dispatched.push(action as Dispatched);
      const view = render(createElement(WindowMeasureToggle, { measure: toggleMeasure(true), onAction }));
      expect(checkbox().checked).toBe(true);
      fireEvent.click(checkbox());
      expect(dispatched.length).toBe(1);
      expect(dispatched[0]!.args!.pressed).toBe(false);
      expect(checkbox().checked).toBe(false);
      // 🔁️ The program has still not answered: a re-render with the SAME published value must not snap
      // the control back, or the user watches their own click undo itself.
      view.rerender(createElement(WindowMeasureToggle, { measure: toggleMeasure(true), onAction }));
      expect(checkbox().checked).toBe(false);
      // 🏛️ The published value arrives and equals the draft — the draft retires, the program is the only
      // authority again, and the next click dispatches from the published value.
      view.rerender(createElement(WindowMeasureToggle, { measure: toggleMeasure(false), onAction }));
      expect(checkbox().checked).toBe(false);
      fireEvent.click(checkbox());
      expect(dispatched[1]!.args!.pressed).toBe(true);
    });

    it("lets the program overrule a draft by publishing a different value", () => {
      const dispatched: Dispatched[] = [];
      const onAction = (action: unknown) => dispatched.push(action as Dispatched);
      const view = render(createElement(WindowMeasureToggle, { measure: toggleMeasure(false), onAction }));
      fireEvent.click(checkbox());
      expect(checkbox().checked).toBe(true);
      view.rerender(createElement(WindowMeasureToggle, { measure: toggleMeasure(false), onAction }));
      expect(checkbox().checked).toBe(true);
      // 🧯️ A refusal: the program answers with a value that is not the draft. A draft is only ever the
      // gap-filler, never a second source of truth, so the program wins.
      view.rerender(createElement(WindowMeasureToggle, { measure: toggleMeasure(true), onAction }));
      view.rerender(createElement(WindowMeasureToggle, { measure: toggleMeasure(false), onAction }));
      expect(checkbox().checked).toBe(false);
    });

    it("shows the select option the user picked before the program has published it", () => {
      const dispatched: Dispatched[] = [];
      const onAction = (action: unknown) => dispatched.push(action as Dispatched);
      const view = render(createElement(WindowMeasureSelect, { measure: selectMeasure("selected"), onAction }));
      expect(trigger().textContent).toContain("Selected");
      fireEvent.click(trigger());
      const option = [...document.querySelectorAll('[data-value="always"]')].at(-1) as HTMLElement | undefined;
      expect(option).toBeTruthy();
      fireEvent.click(option!);
      expect(dispatched.length).toBe(1);
      expect(dispatched[0]!.args!.value).toBe("always");
      expect(trigger().textContent).toContain("Always");
      view.rerender(createElement(WindowMeasureSelect, { measure: selectMeasure("selected"), onAction }));
      expect(trigger().textContent).toContain("Always");
      view.rerender(createElement(WindowMeasureSelect, { measure: selectMeasure("always"), onAction }));
      expect(trigger().textContent).toContain("Always");
    });
  });
}

testWindowMeasureControls();
