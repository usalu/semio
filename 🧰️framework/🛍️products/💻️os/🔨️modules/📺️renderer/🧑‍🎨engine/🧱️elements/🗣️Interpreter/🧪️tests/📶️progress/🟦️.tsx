type TestSource = { readonly url: string };

/** 📶️ The interpreted `progress` component is a real, accessible progressbar that answers the shared
 * `🧬️contract/🧫️fixtures/📶️progress.json` law the Rust contract, the wgpu projection and the TypeScript
 * twin answer: value attributes while determinate, only `aria-busy` while indeterminate, and a fill
 * sized by the same fraction. jsdom's own ARIA role query (`getByRole`, dom-testing-library) is the
 * third-party oracle that the element is exposed as a progressbar at all. Ticket
 * 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS. */
export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { UiDocumentStore, UiNodeView } = dependencies;
  const { describe, expect, it, afterEach } = vitest;
  void source;

  const { cleanup, render, screen } = await import("@semio-tech/ui-react/test");
  const { createElement } = await import("react");
  const { default: fixture } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/📶️progress.json");

  const SURFACE = "panel:tool-run";
  const TEST_STYLE = { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" };

  function mount(component: Record<string, unknown>, activity = "idle") {
    const store = new UiDocumentStore(SURFACE);
    const record = { id: 1, key: "fill.progress", component, layout: { kind: "leaf", width: "fill", height: "hug" }, style: TEST_STYLE, activity, disabled: false, transition: null, accessibility: { label: "Fill progress", description: null, live: "off", shortcut: null, hidden: false }, bindings: [], menu: null, children: [] };
    store.loadSnapshot({ surface: SURFACE, revision: 1, root: 1, nodes: [record] });
    return render(createElement(UiNodeView, { store, id: 1, context: { store, onAction: () => {}, onIntent: () => {} } }));
  }

  describe("📶️ interpreted progress", () => {
    afterEach(() => cleanup());

    for (const row of fixture.cases) {
      it(`${row.id}: exposes the value attributes the shared fixture declares`, () => {
        mount(row.component);
        const bar = screen.getByRole("progressbar", { name: "Fill progress" });
        const attribute = (name: string) => bar.getAttribute(name);
        const expected = row.accessibility;
        expect(attribute("aria-valuemin")).toBe(expected.valueMin === null ? null : String(expected.valueMin));
        expect(attribute("aria-valuemax")).toBe(expected.valueMax === null ? null : String(expected.valueMax));
        expect(attribute("aria-valuenow")).toBe(expected.valueNow === null ? null : String(expected.valueNow));
        expect(attribute("aria-valuetext")).toBe(expected.valueText);
        expect(attribute("aria-busy")).toBe(expected.busy ? "true" : null);
        expect(bar.hasAttribute("tabindex")).toBe(expected.focusable);
        const fill = bar.querySelector('[data-slot="progress-fill"]') as HTMLElement;
        expect(fill).not.toBeNull();
        if (row.fraction === null) {
          expect(fill.style.width).toBe("");
          expect(fill.className).toContain("motion-safe:animate-pulse");
        } else {
          expect(fill.style.width).toBe(`${row.fraction * 100}%`);
          expect(fill.className).not.toContain("animate-pulse");
        }
      });
    }

    it("stays a progressbar while its record is loading instead of collapsing into a skeleton", () => {
      mount(fixture.cases[0].component, "loading");
      expect(screen.getByRole("progressbar", { name: "Fill progress" }).getAttribute("aria-valuenow")).toBe("12");
    });
  });
}
