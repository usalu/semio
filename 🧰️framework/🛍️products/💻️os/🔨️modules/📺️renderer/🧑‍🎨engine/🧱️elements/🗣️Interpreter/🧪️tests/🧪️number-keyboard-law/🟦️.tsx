/** 🎹️ The interpreted number controls a history editor is built from — a blur-committed number field (every vector axis is
 * one) and a number stepper — answer every row of the shared number-control corpus (`🖱️ui/🧬️contract/🧫️fixtures/🧫️number-controls`,
 * design §18 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING): each `keys` row through its physical key (a Home/End without
 * its bound stays the caret's), each `typed` row read back to its stored value, each `limits` row refused visibly or dispatched.
 * The dispatched intents are the oracle: what the guest receives is what the law says. */
import numberControls from "../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧫️number-controls/🔣️.json";
import { uiNumberFieldKey, type SliderKey } from "../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧩️component/🟦️.ts";

type TestSource = { readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: Pick<typeof import("../../../📃️UiDocumentStore/🟦️.tsx"), "UiDocumentStore"> & Pick<typeof import("../../🟦️.tsx"), "UiNodeView">, _source: TestSource): Promise<void> {
  const { UiDocumentStore, UiNodeView } = dependencies;
  type Component = import("../../../../../../../../../🔨️modules/🛂️manifest/🟦️.ts").Component;
  type UiIntent = import("../../../../../../../../../🔨️modules/🛂️manifest/🟦️.ts").UiIntent;
  type UiNodeRecord = import("../../../../../../../../../🔨️modules/🛂️manifest/🟦️.ts").UiNodeRecord;
  type KeyRow = { readonly case: string; readonly current: number; readonly min: number | null; readonly max: number | null; readonly step: number; readonly snaps: readonly number[]; readonly key: SliderKey; readonly large: boolean; readonly precision?: number | null; readonly factor?: number | null; readonly expected: number };
  type TypedRow = { readonly case: string; readonly typed: number; readonly factor: number | null; readonly precision: number | null; readonly candidates: readonly number[]; readonly expected: number };
  type LimitRow = { readonly case: string; readonly value: number; readonly min: number | null; readonly max: number | null; readonly limits: Record<string, unknown> | null; readonly crossed: "min" | "max" | null };
  type Facets = { readonly min?: number | null; readonly max?: number | null; readonly step?: number | null; readonly precision?: number | null; readonly displayFactor?: number | null; readonly snaps?: readonly number[]; readonly limits?: Record<string, unknown> | null };

  const { describe, expect, it } = vitest;
  const PHYSICAL: Readonly<Record<SliderKey, string>> = { increment: "ArrowUp", decrement: "ArrowDown", pageUp: "PageUp", pageDown: "PageDown", home: "Home", end: "End" };
  const keyRows = numberControls.keys as readonly KeyRow[];
  const typedRows = numberControls.typed as readonly TypedRow[];
  const limitRows = numberControls.limits as readonly LimitRow[];
  const CORPUS_TIMEOUT_MS = 60_000;

  /** 🧱️ One bound node: `field` is a blur-committed number input (the history editor's number and vector-axis rows), `stepper`
   * a uniform number stepper riding the continuous lane. */
  function record(kind: "field" | "stepper", current: number, facets: Facets): UiNodeRecord {
    const numeric = { min: facets.min ?? null, max: facets.max ?? null, step: facets.step ?? null, precision: facets.precision ?? null, displayFactor: facets.displayFactor ?? null, snaps: [...(facets.snaps ?? [])], limits: facets.limits ?? null, unit: null, displayUnit: null };
    const component = kind === "field" ? { type: "input", kind: "number", value: String(current), placeholder: null, commit: "blur", accept: null, ...numeric } : { type: "numberStepper", value: current, uniform: true, ...numeric, step: facets.step ?? 0 };
    return { id: 0, key: kind, component: component as unknown as Component, layout: { kind: "leaf", width: "hug", height: "hug" }, style: { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" }, activity: "idle", disabled: false, transition: null, accessibility: { label: "Value", description: null, live: "off", shortcut: null, hidden: false }, bindings: [{ trigger: kind === "field" ? "commit" : "change", action: "setValue", args: null, capability: null }], menu: null, children: [] } as unknown as UiNodeRecord;
  }

  /** 🖼️ Mounts `node` and answers its value field and the values the guest received (a continuous press's `value`). */
  async function mount(node: UiNodeRecord) {
    const { render } = await import("@semio-tech/ui-react/test");
    const store = new UiDocumentStore("number-keyboard-law");
    store.loadSnapshot({ surface: "number-keyboard-law", revision: 0, root: 0, nodes: [node], layoutEpoch: 0n });
    const intents: UiIntent[] = [];
    const view = render(<UiNodeView store={store} id={0} context={{ store, onAction: () => {}, onIntent: (intent) => void intents.push(intent) }} />);
    const input = view.container.querySelector<HTMLInputElement>(node.key === "field" ? 'input[type="number"]' : '[data-stepper-input="true"]')!;
    const received = () => intents.filter((intent) => intent.input !== null && !(typeof intent.input === "object" && "abort" in (intent.input as object))).map((intent) => (typeof intent.input === "number" ? intent.input : ((intent.input as { readonly value: number }).value)));
    return { input, received, unmount: view.unmount };
  }

  const facetsOfKeyRow = (row: KeyRow): Facets => ({ min: row.min, max: row.max, step: row.step > 0 ? row.step : null, precision: row.precision ?? null, displayFactor: row.factor ?? null, snaps: row.snaps });

  describe("🎹️ interpreted number controls answer the shared number-control law (design §18)", () => {
    it("names a law key for exactly the arrow, page, Home and End keys, Home/End only toward a bound the field has", () => {
      expect(Object.entries(PHYSICAL).map(([key, physical]) => [key, uiNumberFieldKey(physical, false, 0, 1)?.key ?? null])).toEqual(Object.keys(PHYSICAL).map((key) => [key, key]));
      expect([uiNumberFieldKey("ArrowUp", true, null, null), uiNumberFieldKey("PageUp", true, null, null)]).toEqual([{ key: "increment", large: true }, { key: "pageUp", large: false }]);
      expect(["ArrowLeft", "ArrowRight", "Enter", "a"].map((key) => uiNumberFieldKey(key, false, 0, 1))).toEqual([null, null, null, null]);
      expect([uiNumberFieldKey("Home", false, null, 1), uiNumberFieldKey("End", false, 0, undefined), uiNumberFieldKey("Home", false, Number.NEGATIVE_INFINITY, 1)]).toEqual([null, null, null]);
      expect(new Set(keyRows.map((row) => row.key))).toEqual(new Set(Object.keys(PHYSICAL)));
    });

    for (const kind of ["field", "stepper"] as const) {
      it(`walks a ${kind} by every keys row of the corpus`, async () => {
        const { fireEvent } = await import("@semio-tech/ui-react/test");
        for (const row of keyRows) {
          const { input, received, unmount } = await mount(record(kind, row.current, facetsOfKeyRow(row)));
          const caret = (row.key === "home" && row.min === null) || (row.key === "end" && row.max === null);
          const pressed = fireEvent.keyDown(input, { key: PHYSICAL[row.key], shiftKey: row.large });
          if (kind === "field") fireEvent.blur(input);
          expect(pressed, `${row.case}: ${caret ? "the caret keeps the key" : "the law takes the key"}`).toBe(caret);
          if (caret) expect(received(), row.case).toEqual([]);
          expect(received().at(-1) ?? row.current, row.case).toBeCloseTo(row.expected, 9);
          unmount();
        }
      }, CORPUS_TIMEOUT_MS);

      it(`reads every typed row of the corpus back to its stored value on a ${kind}`, async () => {
        const { fireEvent } = await import("@semio-tech/ui-react/test");
        for (const row of typedRows) {
          const { input, received, unmount } = await mount(record(kind, row.candidates[0] ?? 0, { precision: row.precision, displayFactor: row.factor, snaps: row.candidates, step: 1 }));
          fireEvent.change(input, { target: { value: String(row.typed) } });
          if (kind === "field") fireEvent.blur(input);
          expect(received().length, row.case).toBe(1);
          expect(received()[0], row.case).toBeCloseTo(row.expected, 12);
          unmount();
        }
      }, CORPUS_TIMEOUT_MS);

      it(`refuses exactly the typed values the corpus's hard-bound rows cross on a ${kind}, keeping the draft and dispatching nothing`, async () => {
        const { fireEvent } = await import("@semio-tech/ui-react/test");
        for (const row of limitRows) {
          const { input, received, unmount } = await mount(record(kind, row.value + 0.5, { min: row.min, max: row.max, limits: row.limits, step: 1 }));
          fireEvent.change(input, { target: { value: String(row.value) } });
          if (kind === "field") fireEvent.blur(input);
          expect([received(), input.getAttribute("aria-invalid"), input.value], row.case).toEqual(row.crossed === null ? [[row.value], null, input.value] : [[], "true", String(row.value)]);
          unmount();
        }
      }, CORPUS_TIMEOUT_MS);
    }
  });
}
