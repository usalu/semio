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
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import { afterEach, describe, expect, it } from "vitest";
import { WindowMeasureNumber, WindowMeasureSelect, WindowMeasureToggle } from "../../🧱️elements/🛠️ShellHelpers/🎚️measure-controls/🟦️.tsx";
import { renderWindowMeasuresTree } from "../../🧱️elements/🛠️ShellHelpers/🟦️.tsx";
import { UiDocumentStore } from "../../🧱️elements/📃️UiDocumentStore/🟦️.tsx";
import { UiNodeView } from "../../🧱️elements/🗣️Interpreter/🟦️.tsx";
import { FlowProvider } from "@semio-tech/ui-react";

it("interprets the neutral compact Tree and preserves checkbox identity, authority, and disability", () => {
  const law = JSON.parse(readFileSync(resolve(dirname(fileURLToPath(import.meta.url)), "../../🧫️fixtures/📏️window-measures/🔣️.json"), "utf8"));
  const root = law.windows[0].expectedOverlay;
  for (const disabled of [false, true]) {
    const nodes: any[] = [];
    const visit = (outline: any): number => {
      const id = nodes.length;
      const node = { ...outline, id, children: [] as number[], layout: { kind: "leaf", width: "hug", height: "hug" }, style: { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" }, accessibility: { label: outline.label, description: null, live: "off", shortcut: null, hidden: false }, menu: null, transition: null };
      if (outline.component.type === "toggle") node.disabled = disabled;
      nodes.push(node);
      node.children = outline.children.map(visit);
      return id;
    };
    const rootId = visit(root);
    const store = new UiDocumentStore("compact-tree-law");
    store.loadSnapshot({ surface: "compact-tree-law", revision: 1, root: rootId, nodes, layoutEpoch: 0n } as any);
    const intents: any[] = [];
    const view = render(createElement(FlowProvider, { inline: "rtl", children: createElement(UiNodeView, { store, id: rootId, context: { store, onAction: () => {}, onIntent: (intent: unknown) => intents.push(intent) } }) }));
    const tree = view.container.querySelector("[role='tree']")!;
    expect(tree.getAttribute("data-tree-presentation")).toBe("compact");
    expect(tree.getAttribute("dir")).toBe("rtl");
    const input = view.container.querySelector<HTMLInputElement>("input[type='checkbox'][aria-label='Grid']")!;
    expect(input).toBeTruthy();
    expect(input.checked).toBe(true);
    expect(input.disabled).toBe(disabled);
    expect(input.id).toBe("compact-tree-law/puzzle3d-main/grid-visible");
    fireEvent.click(input);
    expect(intents).toHaveLength(disabled ? 0 : 1);
    if (!disabled) {
      expect(intents[0].nodeKey).toBe("puzzle3d-main/grid-visible");
      expect(intents[0].trigger).toBe("change");
      expect(intents[0].input).toBe(false);
    }
    cleanup();
  }
});

it("renders the neutral Window Options tree with compact rows and closed child retirement", () => {
  const law = JSON.parse(readFileSync(resolve(dirname(fileURLToPath(import.meta.url)), "../../🧫️fixtures/🌳️window-measures-tree-parity/🔣️.json"), "utf8"));
  const schema = JSON.parse(readFileSync(resolve(dirname(fileURLToPath(import.meta.url)), "../../🧬️schema/🌳️window-measures-tree-parity/🔣️.json"), "utf8"));
  const validate = new Ajv({ strict: false, allErrors: true }).compile(schema);
  expect(validate(law), JSON.stringify(validate.errors)).toBe(true);
  const view = render(renderWindowMeasuresTree(law.measures, () => undefined));
  const rows = Array.from(view.container.querySelectorAll("[data-slot='window-measure-tree-row']"));
  expect(rows.map(row => row.querySelector("[data-slot='tree-label']")?.textContent)).toEqual(law.expected.labels);
  const hasClosedLabel = () => Array.from(view.container.querySelectorAll("[data-slot='tree-label']")).some(label => label.textContent === law.expected.closedLabel);
  expect(hasClosedLabel()).toBe(false);
  const toggle = view.container.querySelector<HTMLInputElement>("input[type='checkbox'][aria-label='Visible']")!;
  expect(toggle).toBeTruthy();
  expect(toggle.checked).toBe(true);
  expect(toggle.classList.contains("size-tiny")).toBe(true);
  expect(toggle.parentElement?.classList.contains("min-w-tiny")).toBe(true);
  expect(view.container.querySelector("[role='combobox']")).toBeTruthy();
  expect(rows.every(row => row.classList.contains("min-h-tiny"))).toBe(true);
  const lastGroup = rows.at(-1)!;
  fireEvent.click(lastGroup.querySelector("button")!);
  expect(hasClosedLabel()).toBe(true);
  fireEvent.click(lastGroup.querySelector("button")!);
  expect(hasClosedLabel()).toBe(false);
  cleanup();
});

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

const countAction = { controllerId: "puzzle3d-play", action: "setFillCount" } as const;

/** @emoji 🔢️ An unbounded count: a floor but deliberately NO ceiling, which is the whole point of `number`. */
const numberMeasure = (value: number) => ({ kind: "number", id: "puzzle3d-fill-count", label: "Count", value, min: 0, step: 1, ready: 42, loading: true, onChange: countAction }) as never;

const numberInput = (): HTMLInputElement => document.getElementById("puzzle3d-fill-count") as HTMLInputElement;
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

    // 🕰️ Wave B41: the draft is invisible from outside React — a combobox trigger carries no value of its
    // own, and the rendered text is the DRAFT's — so `🔍️browser-probe.ts` scored a pending draft as the
    // program's answer. Every draft-bearing control now also exposes the AUTHORITY's value, which is what
    // lets a reader (assistive technology, a tutorial, a probe) tell a pending gesture from a landed one.
    it("exposes the value the program published beside the draft it renders", () => {
      const onAction = () => undefined;
      const view = render(createElement(WindowMeasureSelect, { measure: selectMeasure("selected"), onAction }));
      expect(trigger().getAttribute("data-published-value")).toBe("selected");
      fireEvent.click(trigger());
      fireEvent.click([...document.querySelectorAll('[data-value="always"]')].at(-1) as HTMLElement);
      // 🏛️ Draft in flight: the trigger SHOWS "Always" while the program still publishes "selected".
      expect(trigger().textContent).toContain("Always");
      expect(trigger().getAttribute("data-published-value")).toBe("selected");
      view.rerender(createElement(WindowMeasureSelect, { measure: selectMeasure("always"), onAction }));
      expect(trigger().getAttribute("data-published-value")).toBe("always");
      cleanup();
      const toggleView = render(createElement(WindowMeasureToggle, { measure: toggleMeasure(true), onAction }));
      expect(checkbox().getAttribute("data-published-value")).toBe("true");
      fireEvent.click(checkbox());
      expect(checkbox().checked).toBe(false);
      expect(checkbox().getAttribute("data-published-value")).toBe("true");
      toggleView.rerender(createElement(WindowMeasureToggle, { measure: toggleMeasure(false), onAction }));
      expect(checkbox().getAttribute("data-published-value")).toBe("false");
    });
  });

  // 🔢️ Ticket 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS wave F: an UNBOUNDED count entry (a slider cannot
  // express "no ceiling"; its `min`/`max` are mandatory). Run progress is the framework tool run panel.
  describe("🔢️ the unbounded number measure", () => {
    afterEach(() => cleanup());

    it("commits once, on blur, rather than on every keystroke of a multi-digit count", () => {
      const dispatched: Dispatched[] = [];
      const onAction = (action: unknown) => dispatched.push(action as Dispatched);
      render(createElement(WindowMeasureNumber, { measure: numberMeasure(100), onAction }));
      const input = numberInput();
      fireEvent.focus(input);
      fireEvent.change(input, { target: { value: "2" } });
      fireEvent.change(input, { target: { value: "25" } });
      fireEvent.change(input, { target: { value: "250" } });
      expect(dispatched.length).toBe(0);
      fireEvent.blur(input);
      expect(dispatched.length).toBe(1);
      expect(dispatched[0]!.args!.value).toBe(250);
    });

    it("accepts a value far above any slider ceiling and declares no maximum to assistive technology", () => {
      const onAction = () => undefined;
      render(createElement(WindowMeasureNumber, { measure: numberMeasure(100), onAction }));
      const input = numberInput();
      expect(input.getAttribute("max")).toBe(null);
      fireEvent.change(input, { target: { value: "100000" } });
      expect(input.value).toBe("100000");
    });

    it("exposes the value the program published beside the draft it renders", () => {
      const onAction = () => undefined;
      const view = render(createElement(WindowMeasureNumber, { measure: numberMeasure(100), onAction }));
      expect(document.querySelector("[data-slot='window-measure-number']")!.getAttribute("data-published-value")).toBe("100");
      fireEvent.change(numberInput(), { target: { value: "250" } });
      expect(document.querySelector("[data-slot='window-measure-number']")!.getAttribute("data-published-value")).toBe("100");
      view.rerender(createElement(WindowMeasureNumber, { measure: numberMeasure(250), onAction }));
      expect(document.querySelector("[data-slot='window-measure-number']")!.getAttribute("data-published-value")).toBe("250");
    });
  });
}

testWindowMeasureControls();
