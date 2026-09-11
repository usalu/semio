type TestSource = { readonly url: string };

/** 🪪️ Every interpreted container carries its authored stable DOM id — including the two wrapper roles
 * (`section`/`group` and `field`) that used to render one without it, so an app's own authored section
 * and field rows were unreachable by id while the control inside them was not (ticket
 * 26/09/02/PUZZLE-3D-END-TO-END wave B12: puzzle 3d's whole Settings panel is authored exactly that
 * way — `ui::section(...).try_id("puzzle3d-play-settings")` over four
 * `ui::field(...).try_id("puzzle3d-play-settings.grid-spacing")` rows). */
export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { UiDocumentStore, UiNodeView } = dependencies;
  const { describe, expect, it, afterEach } = vitest;
  void source;

  const { cleanup, fireEvent, render } = await import("@semio-tech/ui-react/test");
  const { createElement } = await import("react");

  type AnyRecord = Record<string, any>;

  const TEST_STYLE = { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" };
  const TEST_ACCESSIBILITY = { label: null, description: null, live: "off", shortcut: null, hidden: false };
  const SURFACE = "panel:settings";

  function node(id: number, key: string, component: AnyRecord, children: readonly number[] = []): AnyRecord {
    return { id, key, component, layout: { kind: "leaf", width: "hug", height: "hug" }, style: TEST_STYLE, activity: "idle", disabled: false, transition: null, accessibility: TEST_ACCESSIBILITY, bindings: [], menu: null, children: [...children] };
  }

  function container(id: number, key: string, role: string, label: string | null, children: readonly number[]): AnyRecord {
    return node(id, key, { type: "container", role, label, description: null, required: null, error: null, defaultOpen: null, dropOverlay: null }, children);
  }

  function mount(nodes: readonly AnyRecord[], root: number) {
    const store = new UiDocumentStore(SURFACE);
    store.loadSnapshot({ surface: SURFACE, revision: 1, root, nodes: [...nodes] });
    return render(createElement(UiNodeView, { store, id: root, context: { store, onAction: () => {}, onIntent: () => {} } }));
  }

  function stepper(id: number, key: string, value: number, step: number, triggers: readonly string[]): AnyRecord {
    return {
      ...node(id, key, { type: "numberStepper", value, step, uniform: true }),
      bindings: triggers.map((trigger) => ({ trigger, action: { name: "setGridSpacing", scope: "puzzle3d-play", version: 1 }, args: {}, capability: null })),
    };
  }

  describe("🪜️ interpreted number steppers", () => {
    afterEach(() => cleanup());

    it("sends a +/− bump down the absolute `change` trigger a program that declares only `change` can actually receive", () => {
      const intents: any[] = [];
      const store = new UiDocumentStore(SURFACE);
      store.loadSnapshot({ surface: SURFACE, revision: 1, root: 1, nodes: [stepper(1, "spacing", 10, 0.5, ["change"])] });
      render(createElement(UiNodeView, { store, id: 1, context: { store, onAction: () => {}, onIntent: (intent: unknown) => intents.push(intent) } }));
      const plus = document.querySelector('[data-slot="stepper-plus"]') as HTMLElement;
      expect(plus).not.toBeNull();
      fireEvent.mouseDown(plus);
      fireEvent.mouseUp(plus);
      expect(intents.length).toBe(1);
      expect(intents[0].trigger).toBe("change");
      expect(intents[0].input).toBe(10.5);
    });

    it("keeps the relative `delta` trigger for a program that declares it", () => {
      const intents: any[] = [];
      const store = new UiDocumentStore(SURFACE);
      store.loadSnapshot({ surface: SURFACE, revision: 1, root: 1, nodes: [stepper(1, "spacing", 10, 0.5, ["change", "delta"])] });
      render(createElement(UiNodeView, { store, id: 1, context: { store, onAction: () => {}, onIntent: (intent: unknown) => intents.push(intent) } }));
      const plus = document.querySelector('[data-slot="stepper-plus"]') as HTMLElement;
      fireEvent.mouseDown(plus);
      fireEvent.mouseUp(plus);
      expect(intents.length).toBe(1);
      expect(intents[0].trigger).toBe("delta");
      expect(intents[0].input).toBe(0.5);
    });
  });

  describe("🪪️ interpreted container ids", () => {
    afterEach(() => cleanup());

    it("renders a section's own authored id, namespaced by its surface", () => {
      mount([container(1, "puzzle3d-play-settings", "section", "Settings", [2]), node(2, "puzzle3d-play-settings.control", { type: "text", value: "body", emphasize: null, dataAttributes: null })], 1);
      const section = document.getElementById(`${SURFACE}/puzzle3d-play-settings`);
      expect(section).not.toBeNull();
      expect(section!.tagName.toLowerCase()).toBe("section");
      expect(document.querySelectorAll(`[id="${SURFACE}/puzzle3d-play-settings"]`).length).toBe(1);
    });

    it("renders a field row's own authored id and associates its label with the control id the row implies", () => {
      mount([container(1, "puzzle3d-play-settings.grid-spacing", "field", "Spacing", [2]), node(2, "puzzle3d-play-settings.grid-spacing.control", { type: "text", value: "10", emphasize: null, dataAttributes: null })], 1);
      const field = document.getElementById(`${SURFACE}/puzzle3d-play-settings.grid-spacing`);
      expect(field).not.toBeNull();
      expect(field!.getAttribute("data-slot")).toBe("field");
      expect(field!.querySelector('[data-slot="field-label"]')!.getAttribute("for")).toBe(`${SURFACE}/puzzle3d-play-settings.grid-spacing.control`);
    });
  });
}
