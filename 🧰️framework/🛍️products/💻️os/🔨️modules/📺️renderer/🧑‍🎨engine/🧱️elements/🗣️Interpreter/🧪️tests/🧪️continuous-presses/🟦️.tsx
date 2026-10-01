/** 🎚️ The Interpreter's continuous controls speak the scrub protocol (design §13.1 of ticket
 * 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING): every move of one press carries the same `gesture` with `commit: false`, the
 * release carries `commit: true`, a host cancel `{gesture, abort}` carries no value, and two presses never share an
 * identity. A colour picker's moves arrive as `input` events and its commit as the native `change` event. */
type TestSource = { readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: Pick<typeof import("../../../📃️UiDocumentStore/🟦️.tsx"), "UiDocumentStore"> & Pick<typeof import("../../🟦️.tsx"), "UiNodeView">, _source: TestSource): Promise<void> {
  const { UiDocumentStore, UiNodeView } = dependencies;
  type AccessibilitySpec = import("../../../../../../../../../🔨️modules/🛂️manifest/🟦️.ts").AccessibilitySpec;
  type Component = import("../../../../../../../../../🔨️modules/🛂️manifest/🟦️.ts").Component;
  type StyleSpec = import("../../../../../../../../../🔨️modules/🛂️manifest/🟦️.ts").StyleSpec;
  type UiIntent = import("../../../../../../../../../🔨️modules/🛂️manifest/🟦️.ts").UiIntent;
  type UiNodeRecord = import("../../../../../../../../../🔨️modules/🛂️manifest/🟦️.ts").UiNodeRecord;

  const { describe, expect, it } = vitest;
  const STYLE: StyleSpec = { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" };
  const ACCESSIBILITY: AccessibilitySpec = { label: "Field", description: null, live: "off", shortcut: null, hidden: false };

  /** 🧱️ One input node whose `change` trigger is bound to `setValue`. */
  function field(kind: "color" | "number", value: string): UiNodeRecord {
    return bound({ type: "input", kind, value, placeholder: null, commit: null, min: null, max: null, step: null, accept: null, precision: null, snaps: [] } as unknown as Component);
  }

  function bound(component: Component): UiNodeRecord {
    return { id: 0, key: "field", component, layout: { kind: "leaf", width: "hug", height: "hug" }, style: STYLE, activity: "idle", disabled: false, transition: null, accessibility: ACCESSIBILITY, bindings: [{ trigger: "change", action: "setValue", args: null, capability: null }], menu: null, children: [] } as unknown as UiNodeRecord;
  }

  async function mount(record: UiNodeRecord) {
    const { render, cleanup } = await import("@semio-tech/ui-react/test");
    const store = new UiDocumentStore("s");
    store.loadSnapshot({ surface: "s", revision: 0, root: 0, nodes: [record], layoutEpoch: 0n });
    const intents: UiIntent[] = [];
    const { container } = render(<UiNodeView store={store} id={0} context={{ store, onAction: () => {}, onIntent: (intent) => void intents.push(intent) }} />);
    const input = container.querySelector("input") as HTMLInputElement;
    const payloads = () => intents.map((intent) => intent.input as Record<string, unknown>);
    return { container, input, payloads, cleanup };
  }

  /** ⌨️ A browser's `input` event after the value moved: React's value tracker sees the new value, so `onChange` fires. */
  function move(input: HTMLInputElement, value: string): void {
    Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, "value")!.set!.call(input, value);
    input.dispatchEvent(new Event("input", { bubbles: true }));
  }

  describe("continuous presses", () => {
    it("a colour picker's moves are ONE press, released by its native change; a later blur adds nothing", async () => {
      const { act, fireEvent } = await import("@semio-tech/ui-react/test");
      const { input, payloads, cleanup } = await mount(field("color", "#336699"));
      await act(async () => {
        move(input, "#112233");
        move(input, "#445566");
        input.dispatchEvent(new Event("change", { bubbles: true }));
      });
      const press = payloads();
      expect(press.map(({ value, commit }) => [value, commit])).toEqual([["#112233", false], ["#445566", false], ["#445566", true]]);
      expect(new Set(press.map(({ gesture }) => gesture)).size).toBe(1);
      act(() => {
        fireEvent.blur(input);
      });
      expect(payloads().length, "a value-less release of a closed press sends nothing").toBe(3);
      await act(async () => {
        Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, "value")!.set!.call(input, "#778899");
        input.dispatchEvent(new Event("change", { bubbles: true }));
      });
      const second = payloads().slice(3);
      expect(second.map(({ value, commit }) => [value, commit]), "a picker that reports only its commit is a one-move press").toEqual([["#778899", false], ["#778899", true]]);
      expect(second[1]!.gesture).toBe(second[0]!.gesture);
      expect(second[0]!.gesture, "two presses never share an identity").not.toBe(press[0]!.gesture);
      cleanup();
    });

    it("a number field's keystrokes are ONE press released by its blur; unmounting an open press cancels it", async () => {
      const { act, fireEvent } = await import("@semio-tech/ui-react/test");
      const { input, payloads, cleanup } = await mount(field("number", "1"));
      act(() => {
        move(input, "12");
        move(input, "125");
        fireEvent.blur(input);
      });
      const press = payloads();
      expect(press.map(({ value, commit }) => [value, commit])).toEqual([[12, false], [125, false], [125, true]]);
      expect(new Set(press.map(({ gesture }) => gesture)).size).toBe(1);
      act(() => {
        move(input, "7");
      });
      cleanup();
      const cancel = payloads().at(-1)!;
      expect(cancel.abort, "an open press dies with its control").toBe("retired");
      expect(cancel.value).toBeUndefined();
      expect(cancel.gesture).toBe(payloads().at(-2)!.gesture);
    });

    it("a dragged ring orb is ONE press released by the pointer, and a cancelled pointer drops it", async () => {
      const { act, fireEvent } = await import("@semio-tech/ui-react/test");
      const { container, payloads, cleanup } = await mount(bound({ type: "ring", orbId: "orb-1", t: 0.25 } as unknown as Component));
      const orb = () => container.querySelector('[data-slot="orb"]') as Element;
      await act(async () => {
        fireEvent.pointerDown(orb(), { clientX: 10, clientY: 0 });
        fireEvent.pointerMove(window as never, { clientX: 0, clientY: 10 });
        fireEvent.pointerUp(window as never, { clientX: -10, clientY: 0 });
      });
      const press = payloads();
      expect(press.length).toBeGreaterThanOrEqual(2);
      expect(press.at(-1)!.commit, "the pointer's release ends the press").toBe(true);
      expect(press.slice(0, -1).every(({ commit }) => commit === false)).toBe(true);
      expect(new Set(press.map(({ gesture }) => gesture)).size).toBe(1);
      await act(async () => {
        fireEvent.pointerDown(orb(), { clientX: 10, clientY: 0 });
        fireEvent.pointerMove(window as never, { clientX: 0, clientY: -10 });
        await new Promise((resolve) => requestAnimationFrame(() => resolve(undefined)));
        fireEvent.pointerCancel(window as never);
      });
      const cancelled = payloads().slice(press.length);
      expect(cancelled.at(-1)!.abort, "a cancelled pointer leaves zero trace").toBe("captureLost");
      expect(cancelled.some(({ commit }) => commit === true)).toBe(false);
      expect(cancelled.at(-1)!.gesture).not.toBe(press[0]!.gesture);
      cleanup();
    });
  });
}
