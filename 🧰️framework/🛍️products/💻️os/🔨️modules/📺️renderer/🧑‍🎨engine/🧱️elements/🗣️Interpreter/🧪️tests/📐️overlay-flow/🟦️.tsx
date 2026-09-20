type TestSource = { readonly url: string };

/** 📐️ Proves the renderer-neutral Overlay/Absolute positioning contract against the shared geometry
 * fixture. The browser's `CSSStyleDeclaration` is the independent CSS parser: the law observes the
 * mounted element rather than trusting the projection helper's returned object. */
export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { UiDocumentStore, UiNodeView, layoutSpecStyle } = dependencies;
  const { afterEach, describe, expect, it } = vitest;
  void source;

  const { cleanup, render } = await import("@semio-tech/ui-react/test");
  const { createElement } = await import("react");
  const { default: fixture } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧫️fixtures/📐️overlay-flow/🔣️.json");

  const SURFACE = "panel:overlay-flow";
  const TEST_STYLE = { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" };

  function mount(layout: Record<string, unknown>): HTMLElement {
    const store = new UiDocumentStore(SURFACE);
    const record = {
      id: 1,
      key: "overlay.flow",
      component: { type: "container", role: "plain", label: null, description: null, required: null, error: null, defaultOpen: null, dropOverlay: null },
      layout,
      style: TEST_STYLE,
      activity: "idle",
      disabled: false,
      transition: null,
      accessibility: { label: null, description: null, live: "off", shortcut: null, hidden: false },
      bindings: [],
      menu: null,
      children: [],
    };
    store.loadSnapshot({ surface: SURFACE, revision: 1, root: 1, nodes: [record] });
    const mounted = render(createElement(UiNodeView, { store, id: 1, context: { store, onAction: () => {}, onIntent: () => {} } }));
    return mounted.container.querySelector('[data-ui-node-id="1"]') as HTMLElement;
  }

  describe("📐️ Overlay layout flow", () => {
    afterEach(() => cleanup());

    it("projects Overlay as an in-flow positioning context whose inset is padding", () => {
      const projected = layoutSpecStyle(fixture.overlay.layout);
      expect(projected).toMatchObject({ position: fixture.overlay.expectedCss.position, padding: fixture.overlay.expectedCss.padding });
      expect(projected.inset).toBeUndefined();
      const element = mount(fixture.overlay.layout);
      expect(element.style.position).toBe(fixture.overlay.expectedCss.position);
      expect(element.style.padding).toBe(fixture.overlay.expectedCss.padding);
      expect(element.style.inset).toBe(fixture.overlay.expectedCss.inset);
    });

    it("keeps Absolute out of flow through the mounted ContainerView", () => {
      const projected = layoutSpecStyle(fixture.absolute.layout);
      expect(projected).toMatchObject(fixture.absolute.expectedCss);
      const element = mount(fixture.absolute.layout);
      expect(element.style.position).toBe(fixture.absolute.expectedCss.position);
      expect(element.style.width).toBe(fixture.absolute.expectedCss.width);
      expect(element.style.height).toBe(fixture.absolute.expectedCss.height);
    });
  });
}
