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

  //#region 🪪️RetainedSurfaceHost
  const { default: retention } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🪪️surface-host-retention.json");
  const { uiSiblingReactKeys, uiChildReactKeys } = dependencies;

  /** 🪪️ Projects one fixture body onto the authored `BuiltNode` tree the shell loads into a window's
   * store. `tree`/`treeSection`/`treeItem` carry their real contract payloads so the fixture's node
   * numbering is the numbering the running app mints, not an approximation of it. */
  function builtFromFixture(spec: AnyRecord): AnyRecord {
    const base = { key: spec.key, layout: { kind: "leaf", width: "hug", height: "hug" }, style: TEST_STYLE, activity: "idle", disabled: false, accessibility: TEST_ACCESSIBILITY, bindings: [], menu: null, children: (spec.children ?? []).map(builtFromFixture) };
    switch (spec.component) {
      case "container":
        return { ...base, component: { type: "container", role: "plain", label: null, description: null, required: null, error: null, defaultOpen: null, dropOverlay: null } };
      case "tree":
        return { ...base, component: { type: "tree", interactionDomain: null } };
      case "treeSection":
        return { ...base, component: { type: "treeSection", label: spec.key, defaultOpen: true } };
      case "treeItem":
        return { ...base, component: { type: "treeItem", label: spec.key, description: null, icon: null, defaultOpen: true, draggable: null, dragData: null, dimmed: null, rowActions: [] } };
      case "surface":
        return { ...base, component: { type: "surface", kind: "node-graph", docSchema: "flow.graph@1", doc: { bytes: [] }, bindings: [] } };
      default:
        throw new Error(`unmapped fixture component ${String(spec.component)}`);
    }
  }

  describe("🪪️ sibling reconciliation keys", () => {
    for (const sample of retention.siblingKeyCases) {
      it(`${sample.name} — ${sample.why}`, () => {
        expect(uiSiblingReactKeys(sample.siblings)).toEqual(sample.expected);
      });
    }

    it("resolves a child-id run against the live document state, and keeps a dangling id addressable", () => {
      const store = new UiDocumentStore(retention.surface);
      store.loadSnapshot({ surface: retention.surface, revision: 1, root: 1, nodes: [container(1, "body", "plain", null, [2]), node(2, "kept", { type: "text", value: "x", emphasize: null, dataAttributes: null })] });
      expect(uiChildReactKeys(store.getState(), [2, 404])).toEqual(["k:kept", "#404"]);
    });
  });

  /** 🪪️ The retention law this whole fixture exists for. `builtNodeToSnapshot` renumbers by pre-order
   * DFS, so a refresh that grows the outline tree moves the node-graph surface — measured on
   * `window:procedural-main`, id 30 → 34 and its container 29 → 33. Reconciliation keyed on that
   * number unmounted the surface subtree and React built a NEW DOM element for it, which on the live
   * page meant a second wasm flow session, a second canvas and a second wasm-side surface. DOM node
   * identity is the proof: a remount cannot preserve it, and a re-render cannot lose it. */
  describe("🪪️ retained surface host across refreshes", () => {
    afterEach(() => cleanup());

    it("renumbers the surface exactly as the running app does — the hazard the law defends against is real", async () => {
      const { builtNodeToSnapshot } = await import("../../../📃️UiDocumentStore/🟦️.tsx");
      const mintedIds: number[] = [];
      const nodeCounts: number[] = [];
      for (const refresh of retention.refreshes) {
        const snapshot = builtNodeToSnapshot(retention.surface, builtFromFixture(refresh.body) as any);
        nodeCounts.push(snapshot.nodes.length);
        mintedIds.push(snapshot.nodes.find((record: AnyRecord) => record.key === retention.surfaceKey)!.id);
      }
      expect(nodeCounts).toEqual(retention.expected.bodyNodeCountByRefresh);
      expect(mintedIds, "a fixture whose surface never renumbers would make the retention law below vacuous").toEqual(retention.expected.surfaceNodeIdByRefresh);
      expect(new Set(mintedIds).size).toBeGreaterThan(1);
    });

    it(`keeps ONE surface host across ${retention.refreshes.length} refreshes with changing status maps`, async () => {
      const { builtNodeToSnapshot } = await import("../../../📃️UiDocumentStore/🟦️.tsx");
      const { act } = await import("react");
      const store = new UiDocumentStore(retention.surface);
      const load = (index: number) => act(() => { store.loadSnapshot(builtNodeToSnapshot(retention.surface, builtFromFixture(retention.refreshes[index].body) as any)); });
      load(0);
      const view = render(createElement(UiNodeView, { store, id: store.getState().root, context: { store, onAction: () => {}, onIntent: () => {} } }));
      const canvasAt = () => document.getElementById(retention.expected.canvasDomId);
      // 🕸️ A `node-graph` surface whose `doc.bytes` are empty decodes to no scene, so `NodeGraphHost`
      // renders its empty-scene body and never boots a wasm flow session — the retained-identity law
      // is about the React subtree, and a jsdom suite must not need a 43 MB engine to state it.
      const hostAt = () => view.container.querySelector(".semio-node-graph-empty");
      const firstCanvas = canvasAt();
      const firstHost = hostAt();
      expect(firstCanvas, "the canvas container must render under its authored DOM id").not.toBeNull();
      expect(firstHost, "the node-graph host must render under the surface node").not.toBeNull();
      const mintedAt = (index: number) => String(retention.expected.surfaceNodeIdByRefresh[index] - 1);
      expect(firstCanvas!.getAttribute("data-ui-node-id")).toBe(mintedAt(0));
      for (let index = 1; index < retention.refreshes.length; index += 1) {
        const refresh = retention.refreshes[index];
        load(index);
        // 🪪️ The refresh REACHED the DOM — the minted id on the very element under test moved. Without
        // this the identity assertions below would pass on a render that never happened.
        expect(canvasAt()!.getAttribute("data-ui-node-id"), `refresh "${refresh.name}" never reached the DOM`).toBe(mintedAt(index));
        expect(canvasAt(), `refresh "${refresh.name}" replaced the canvas container's DOM element — the surface host was unmounted and rebuilt`).toBe(firstCanvas);
        expect(hostAt(), `refresh "${refresh.name}" replaced the node-graph host's DOM element`).toBe(firstHost);
        expect(canvasAt()!.id, "the authored DOM id must survive every renumbering").toBe(retention.expected.canvasDomId);
      }
      expect(view.container.querySelectorAll(".semio-node-graph-empty").length, "exactly one node-graph host, never a second one left beside the first").toBe(retention.expected.surfaceHostMounts);
      view.unmount();
    });
  });
  //#endregion 🪪️RetainedSurfaceHost
}
