/** 🖊️ The interpreted text fields a history editor is built from — a blur-committed single-line field and a multi-line one
 * (`kind: "longText"`, what a descriptor declared `multiline` becomes) — answer every row of the shared text-control corpus
 * (`🖱️ui/🧬️contract/🧫️fixtures/🧫️text-controls`, design §22.7 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING) through
 * its physical key: Enter inserts a line in a multi-line field and commits a single-line one, Ctrl/⌘+Enter commits the
 * multi-line draft, Escape drops it — and the two choice kinds §22.7 makes first-class editor controls: a segmented select (a
 * radio group) and an icon select (the icon picker, named and lockable). Two oracles: the dispatched intents (what the guest receives is what the law says) and
 * `@testing-library/user-event`, whose own model of a browser types the line break a multi-line Enter means and none into a
 * single-line field. A segmented select renders as a radio group that dispatches the picked value. */
import userEvent from "@testing-library/user-event";
import textControls from "../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧫️text-controls/🔣️.json";
import { uiTextInputKey, type TextInputKey, type TextInputKeyAction } from "../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧩️component/🟦️.ts";

type TestSource = { readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: Pick<typeof import("../../../📃️UiDocumentStore/🟦️.tsx"), "UiDocumentStore"> & Pick<typeof import("../../🟦️.tsx"), "UiNodeView">, _source: TestSource): Promise<void> {
  const { UiDocumentStore, UiNodeView } = dependencies;
  type Component = import("../../../../../../../../../🔨️modules/🛂️manifest/🟦️.ts").Component;
  type InputKind = import("../../../../../../../../../🔨️modules/🛂️manifest/🟦️.ts").InputKind;
  type UiIntent = import("../../../../../../../../../🔨️modules/🛂️manifest/🟦️.ts").UiIntent;
  type UiNodeRecord = import("../../../../../../../../../🔨️modules/🛂️manifest/🟦️.ts").UiNodeRecord;
  type KeyRow = { readonly case: string; readonly kind: InputKind; readonly key: TextInputKey; readonly primary: boolean; readonly shift: boolean; readonly alt: boolean; readonly action: TextInputKeyAction | null };

  const { describe, expect, it } = vitest;
  const rows = textControls.keys as readonly KeyRow[];
  const PUBLISHED = "7";
  const DRAFT = "42";
  const BASE = { layout: { kind: "leaf", width: "hug", height: "hug" }, style: { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" }, activity: "idle", disabled: false, transition: null, menu: null, children: [] };

  /** 🧱️ One bound draft field of `kind` publishing {@link PUBLISHED}, committed on blur — the shape of a history editor's text row. */
  function field(kind: InputKind): UiNodeRecord {
    const component = { type: "input", kind, value: PUBLISHED, placeholder: null, commit: "blur", accept: null, min: null, max: null, step: null, precision: null, displayFactor: null, snaps: [], limits: null };
    return { ...BASE, id: 0, key: "field", component: component as unknown as Component, accessibility: { label: "Text", description: null, live: "off", shortcut: null, hidden: false }, bindings: [{ trigger: "commit", action: "setValue", args: null, capability: null }] } as unknown as UiNodeRecord;
  }

  /** 🖼️ Mounts `node` and answers its DOM field with what the guest received. */
  async function mount(node: UiNodeRecord, selector: string) {
    const { render } = await import("@semio-tech/ui-react/test");
    const store = new UiDocumentStore("text-keyboard-law");
    store.loadSnapshot({ surface: "text-keyboard-law", revision: 0, root: 0, nodes: [node], layoutEpoch: 0n });
    const intents: UiIntent[] = [];
    const view = render(<UiNodeView store={store} id={0} context={{ store, onAction: () => {}, onIntent: (intent) => void intents.push(intent) }} />);
    return { element: view.container.querySelector<HTMLElement>(selector)!, view, received: () => intents.map((intent) => intent.input), unmount: view.unmount };
  }

  describe("🖊️ interpreted text fields answer the shared text-control law (design §22.7)", () => {
    it("renders a multi-line kind as a textarea and every other draft kind as a one-line field", async () => {
      const long = await mount(field("longText"), "textarea");
      expect([long.element?.tagName, long.element?.getAttribute("aria-label"), (long.element as HTMLTextAreaElement).value]).toEqual(["TEXTAREA", "Text", PUBLISHED]);
      long.unmount();
      const short = await mount(field("text"), "input");
      expect([short.element?.tagName, short.view.container.querySelector("textarea")]).toEqual(["INPUT", null]);
      short.unmount();
    });

    it("answers every keys row of the corpus through its physical key: a line, a commit, a revert, or nothing", async () => {
      const { act, fireEvent } = await import("@semio-tech/ui-react/test");
      const user = userEvent.setup();
      for (const row of rows) {
        const { element, received, unmount } = await mount(field(row.kind), row.kind === "longText" ? "textarea" : "input");
        const input = element as HTMLInputElement | HTMLTextAreaElement;
        expect(uiTextInputKey(row.kind, row.key, { primary: row.primary, shift: row.shift, alt: row.alt }), row.case).toBe(row.action);
        await act(async () => {
          await user.click(input);
        });
        fireEvent.change(input, { target: { value: DRAFT } });
        if (input.tagName === "TEXTAREA") input.setSelectionRange(DRAFT.length, DRAFT.length);
        const chord = `${row.alt ? "{Alt>}" : ""}${row.primary ? "{Control>}" : ""}${row.shift ? "{Shift>}" : ""}${row.key === "enter" ? "{Enter}" : "{Escape}"}${row.shift ? "{/Shift}" : ""}${row.primary ? "{/Control}" : ""}${row.alt ? "{/Alt}" : ""}`;
        await act(async () => {
          await user.keyboard(chord);
        });
        const typed = String(DRAFT);
        const expected = { newline: [[], `${typed}\n`], commit: [[row.kind === "number" ? Number(typed) : typed], null], revert: [[], PUBLISHED], none: [[], typed] }[row.action ?? "none"];
        expect([received(), row.action === "commit" ? null : input.value], `${row.case}: ${chord}`).toEqual(expected);
        if (row.action === "commit") expect(document.activeElement === input, `${row.case}: a committed field gives its focus back`).toBe(false);
        unmount();
      }
    }, 60_000);

    it("commits a multi-line draft with ⌘+Enter as with Ctrl+Enter, and leaves a composing Enter to the input method", async () => {
      const { fireEvent } = await import("@semio-tech/ui-react/test");
      const meta = await mount(field("longText"), "textarea");
      fireEvent.change(meta.element, { target: { value: "one\ntwo" } });
      fireEvent.keyDown(meta.element, { key: "Enter", metaKey: true });
      expect(meta.received()).toEqual(["one\ntwo"]);
      meta.unmount();
      const composing = await mount(field("text"), "input");
      fireEvent.change(composing.element, { target: { value: DRAFT } });
      fireEvent.keyDown(composing.element, { key: "Enter", isComposing: true });
      expect(composing.received(), "Enter while an input method composes commits nothing").toEqual([]);
      composing.unmount();
    });

    it("renders a segmented select as a radio group that dispatches the picked value and never presses the chosen one off", async () => {
      const { act, fireEvent } = await import("@semio-tech/ui-react/test");
      const items = [{ value: "x", label: "X axis" }, { value: "y", label: "Y axis" }];
      const select = (appearance: "menu" | "segmented") => ({ ...BASE, id: 0, key: "axis", component: { type: "select", value: "x", items, placeholder: null, appearance } as unknown as Component, accessibility: { label: "Axis", description: null, live: "off", shortcut: null, hidden: false }, bindings: [{ trigger: "change", action: "setValue", args: null, capability: null }] }) as unknown as UiNodeRecord;
      const segmented = await mount(select("segmented"), '[role="radiogroup"]');
      const radios = [...segmented.element.querySelectorAll<HTMLElement>('[role="radio"]')];
      expect([segmented.element.getAttribute("aria-label"), radios.map((radio) => [radio.getAttribute("aria-checked"), (radio.getAttribute("aria-label") ?? radio.textContent ?? "").trim()])]).toEqual(["Axis", [["true", "X axis"], ["false", "Y axis"]]]);
      fireEvent.click(radios[0]!);
      expect(segmented.received(), "the chosen option dispatches nothing").toEqual([]);
      fireEvent.click(radios[1]!);
      act(() => radios[0]!.focus());
      fireEvent.keyDown(radios[0]!, { key: "ArrowRight" });
      expect(segmented.received(), "a press and an arrow both dispatch the picked value").toEqual(["y", "y"]);
      segmented.unmount();
      const menu = await mount(select("menu"), '[role="combobox"]');
      expect([menu.element !== null, menu.view.container.querySelector('[role="radiogroup"]')], "a menu select stays a combobox").toEqual([true, null]);
      menu.unmount();
    });

    it("renders an icon select as the icon picker, named by its label, dispatching the picked icon and locked while disabled", async () => {
      const { fireEvent } = await import("@semio-tech/ui-react/test");
      const picker = (disabled: boolean) => ({ ...BASE, disabled, id: 0, key: "icon", component: { type: "iconSelect", value: "", uniform: true, classifierKind: "icon" } as unknown as Component, accessibility: { label: "Icon", description: null, live: "off", shortcut: null, hidden: false }, bindings: [{ trigger: "change", action: "setValue", args: null, capability: null }] }) as unknown as UiNodeRecord;
      const open = await mount(picker(false), '[data-slot="icon-selector"]');
      const field = open.element.querySelector<HTMLTextAreaElement>("textarea")!;
      const named = (element: Element) => (element.getAttribute("aria-labelledby") ?? "").split(/\s+/u).filter(Boolean).map((id) => document.getElementById(id)?.textContent ?? "").join(" ");
      expect([field.value, named(field), named(open.element.querySelector('[role="combobox"]')!), field.readOnly]).toEqual(["", "Icon", "Icon", false]);
      fireEvent.change(field, { target: { value: "B" } });
      expect(open.received().map((picked) => [typeof picked, String(picked).includes("B")]), "one change, carrying the icon the picker emits for the typed text").toEqual([["string", true]]);
      open.unmount();
      const locked = await mount(picker(true), '[data-slot="icon-selector"]');
      expect(locked.element.querySelector<HTMLTextAreaElement>("textarea")!.readOnly, "a disabled record locks the picker").toBe(true);
      locked.unmount();
    });
  });
}
