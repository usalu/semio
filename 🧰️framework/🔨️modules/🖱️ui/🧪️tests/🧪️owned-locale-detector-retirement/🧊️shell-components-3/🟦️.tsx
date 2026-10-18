import type { PanelTabNode, PanelTabRowDropTarget } from "@semio-tech/ui-react/source";
import { renderToStaticMarkup } from "react-dom/server";
import type { TestSource, Tests1Vitest, Tests1Dependencies } from "../🧰️harness/🟦️.ts";

export async function registerShellComponents3Tests(vitest: Tests1Vitest, dependencies: Tests1Dependencies, testSource: TestSource): Promise<void> {
  const { Button, Canvas, DEFAULT_UI_DRIVER, Label, Layout, Navbar, Panel, PanelTabBar, React, Toggle, TreeContext, UI_ELEMENT_REGISTRY, UiDriverProvider, WINDOW_SILHOUETTE_BORDER_KINDS, WINDOW_SILHOUETTE_GEOMETRY_SCHEMA, WINDOW_SILHOUETTE_PATH_INSET, Window, chromeStatusBorderClass, computeTabDockDropZone, createWindowSilhouetteGeometry, isWindowChromeIntroducedTarget, modeDockChromeGridPlacement, modeDockTabLabelClassName, normalizeWindowSilhouetteChips, normalizeWindowSilhouetteMetrics, renderToStaticMarkup, resolveWindowSilhouetteBorderKind, singleTreeLeaf, useFlow, windowSilhouetteBorderPaint, windowSilhouetteContains, windowSilhouetteOutline, windowSilhouetteOutlineViolations, windowSilhouettePath } = dependencies;
  type Anchor = any;
  type PanelTabNode = any;
  type PanelTabRowDropTarget = any;
  type WindowSilhouetteChip = any;
  type WindowSilhouetteEdge = any;
  type WindowSilhouetteMetrics = any;
  const { describe, expect, it } = vitest;
  const { render, screen } = await import("@testing-library/react");

describe("Shell components", () => {

    it("keeps a right-anchored panel's chrome mirrored and its tree rows left-to-right", () => {
      const StubIcon = (): null => null;
      const FlowProbe: React.FC = () => {
        const flow = useFlow();
        return <span data-testid="flow-probe">{flow.inline}</span>;
      };
      const tabs: PanelTabNode[] = [
        singleTreeLeaf({
          id: "tab-a",
          icon: StubIcon,
          name: "Tab A",
          tree: { sections: [{ id: "sec", label: "Section", defaultOpen: true, items: [{ id: "leaf", label: "Leaf row", control: <FlowProbe /> }] }] },
        }),
      ];
      const { container: leftContainer } = render(<Panel anchor="top-left" visible tabs={tabs} activeTabPath={["tab-a"]} />);
      expect(leftContainer.querySelector('[data-slot="panel-content"]')?.getAttribute("dir")).toBeNull();
      expect(leftContainer.querySelector('[data-testid="flow-probe"]')?.textContent).toBe("ltr");

      const { container: rightContainer } = render(<Panel anchor="top-right" visible tabs={tabs} activeTabPath={["tab-a"]} />);
      expect(rightContainer.querySelector('[data-slot="panel"]')?.getAttribute("dir")).toBe("rtl");
      expect(rightContainer.querySelector('[data-slot="panel-content"]')?.getAttribute("dir")).toBeNull();
      expect(rightContainer.querySelector('[data-slot="tree"]')?.getAttribute("dir")).toBe("auto");
      expect(rightContainer.querySelector('[data-testid="flow-probe"]')?.textContent).toBe("ltr");
    });

    it("Panel derives its tab bar's stacking direction from the corner's flow block axis", () => {
      const StubIcon = (): null => null;
      const tabs: PanelTabNode[] = [singleTreeLeaf({ id: "tab-a", icon: StubIcon, name: "Tab A", tree: { sections: [] } })];
      const topMarkup = renderToStaticMarkup(<Panel anchor="top-left" visible tabs={tabs} />);
      expect(topMarkup).toContain('data-direction="down"');
      const bottomMarkup = renderToStaticMarkup(<Panel anchor="bottom-left" visible tabs={tabs} />);
      expect(bottomMarkup).toContain('data-direction="up"');
    });

    it("Layout on mobile fills the mobile panel to the available space and hides (not unmounts) the canvas while it's open", () => {
      const StubIcon = (): null => null;
      const tabs: PanelTabNode[] = [
        singleTreeLeaf({
          id: "mobile-tab",
          icon: StubIcon,
          name: "Mobile Tab",
          tree: { sections: [{ id: "mobile-tab.section", label: "", items: [{ id: "mobile-tab.item", label: "", control: <div data-testid="mobile-tab-content">Tab body</div> }] }] },
        }),
      ];
      const { container, rerender } = render(<Layout mobile mobilePanel={{ visible: true, tabs, activeTabPath: ["mobile-tab"] }} canvas={<div data-testid="canvas">Canvas</div>} />);
      const mobilePanel = container.querySelector('[data-panel="mobilePanel"]');
      expect(mobilePanel).toBeTruthy();
      expect(mobilePanel?.className).toContain("flex-1");
      expect(screen.getByTestId("mobile-tab-content")).toBeTruthy();
      const canvasWrapper = screen.getByTestId("canvas").parentElement;
      expect(canvasWrapper?.className).toContain("hidden");

      rerender(<Layout mobile mobilePanel={{ visible: false, tabs, activeTabPath: ["mobile-tab"] }} canvas={<div data-testid="canvas">Canvas</div>} />);
      expect(container.querySelector('[data-panel="mobilePanel"]')).toBeNull();
      expect(screen.getByTestId("canvas").parentElement?.className).not.toContain("hidden");
    });

    it("computeTabDockDropZone keeps physical and model insertion order aligned at every anchor", () => {
      const makeRow = (anchor: Anchor, buttonRects: readonly { left: number; right: number }[]): PanelTabRowDropTarget => {
        const rowElement = document.createElement("div");
        rowElement.getBoundingClientRect = () => ({ left: 0, right: 200, top: 0, bottom: 20, width: 200, height: 20 }) as DOMRect;
        buttonRects.forEach((rect, index) => {
          const button = document.createElement("button");
          button.dataset.tabId = `tab-${index}`;
          button.dataset.tabKind = "leaf";
          button.getBoundingClientRect = () => ({ left: rect.left, right: rect.right, top: 0, bottom: 20, width: rect.right - rect.left, height: 20 }) as DOMRect;
          rowElement.appendChild(button);
        });
        return { anchor, parentPath: [], rowElement };
      };
      const ltrRow = makeRow("top-left", [{ left: 0, right: 100 }]);
      expect(computeTabDockDropZone(25, 10, [ltrRow], new Set())).toEqual({ kind: "insert", anchor: "top-left", parentPath: [], index: 0 });
      expect(computeTabDockDropZone(75, 10, [ltrRow], new Set())).toEqual({ kind: "insert", anchor: "top-left", parentPath: [], index: 1 });

      const rightRow = makeRow("top-right", [{ left: 0, right: 100 }]);
      expect(computeTabDockDropZone(25, 10, [rightRow], new Set())).toEqual({ kind: "insert", anchor: "top-right", parentPath: [], index: 0 });
      expect(computeTabDockDropZone(75, 10, [rightRow], new Set())).toEqual({ kind: "insert", anchor: "top-right", parentPath: [], index: 1 });

      const middleRow = makeRow("top-middle", [{ left: 0, right: 100 }]);
      expect(computeTabDockDropZone(25, 10, [middleRow], new Set())).toEqual({ kind: "insert", anchor: "top-middle", parentPath: [], index: 0 });
      expect(computeTabDockDropZone(75, 10, [middleRow], new Set())).toEqual({ kind: "insert", anchor: "top-middle", parentPath: [], index: 1 });
    });

    it("renders the label before the trailing icon for Button and Toggle", () => {
      const buttonMarkup = renderToStaticMarkup(
        <UiDriverProvider driver={DEFAULT_UI_DRIVER}>
          <Button id="tooltip.manual" text="Apply" icon="check" />
        </UiDriverProvider>,
      );
      expect(buttonMarkup.indexOf(">Apply<")).toBeGreaterThanOrEqual(0);
      expect(buttonMarkup.indexOf(">Apply<")).toBeLessThan(buttonMarkup.indexOf('data-icon="check"'));

      const toggleMarkup = renderToStaticMarkup(
        <UiDriverProvider driver={DEFAULT_UI_DRIVER}>
          <Toggle id="tooltip.manual" text="Focus" icon="crosshair" pressed={false} onPressedChange={() => undefined} />
        </UiDriverProvider>,
      );
      expect(toggleMarkup.indexOf(">Focus<")).toBeGreaterThanOrEqual(0);
      expect(toggleMarkup.indexOf(">Focus<")).toBeLessThan(toggleMarkup.indexOf('data-icon="crosshair"'));
    });

    it("keeps the panel tab icon leading (at the flow-start edge, so it sits at the anchor's own outer edge in every anchor)", () => {
      const StubIcon = (): null => null;
      const tabs: PanelTabNode[] = [singleTreeLeaf({ id: "tab-a", icon: StubIcon, name: "Tab A", tree: { sections: [] } })];
      const tabMarkup = renderToStaticMarkup(<PanelTabBar variant="panel" tabs={tabs} activePath={["tab-a"]} onActivePathChange={() => undefined} />);
      expect(tabMarkup).toContain(modeDockTabLabelClassName);
      expect(tabMarkup.indexOf("data-icon")).toBeLessThan(tabMarkup.indexOf(">Tab A<"));
    });

    it("mirrors flow-relative alignment onto logical classes (tree label, navbar trailing slot)", () => {
      const treeMarkup = renderToStaticMarkup(
        <TreeContext.Provider value={{ level: 0, isLastAtLevel: [true], showLines: false, isTree: false, indentMultiplier: 1 }}>
          <Label id="tooltip.manual" label="Group" />
        </TreeContext.Provider>,
      );
      expect(treeMarkup).toContain("text-start");
      const navbarMarkup = renderToStaticMarkup(<Navbar items={[]} showFullscreenToggle />);
      expect(navbarMarkup).toContain("ms-auto");
    });

    it("modeDockChromeGridPlacement keeps tabs left and controls right", () => {
      const grid = modeDockChromeGridPlacement(
        [
          { id: "a", title: "A" },
          { id: "b", title: "B" },
          { id: "c", title: "C" },
        ],
        "b",
      );
      expect(grid.templateColumns).toBe("max-content max-content max-content minmax(0, 1fr) max-content");
      expect(grid.tabCol(0)).toBe(1);
      expect(grid.tabCol(1)).toBe(2);
      expect(grid.tabCol(2)).toBe(3);
      expect(grid.activeCol).toBe(2);
      expect(grid.gapCol).toBe(4);
      expect(grid.controlsCol).toBe(5);
      expect(grid.bodyColumnSpan).toBe("2 / 5");
    });

    it("modeDockChromeGridPlacement keeps every tab left of the flex gap", () => {
      const grid = modeDockChromeGridPlacement(
        [
          { id: "a", title: "A" },
          { id: "b", title: "B" },
          { id: "c", title: "C" },
        ],
        "b",
      );
      expect(grid.tabCol(0)).toBeLessThan(grid.gapCol);
      expect(grid.tabCol(1)).toBeLessThan(grid.gapCol);
      expect(grid.tabCol(2)).toBeLessThan(grid.gapCol);
      expect(grid.gapCol).toBeLessThan(grid.controlsCol);
    });

    it("windowSilhouettePath follows tabs, gap cutout, and controls", () => {
      const dockTop = (titleRight = 60, controlsLeft = 160, depth = 24): WindowSilhouetteEdge => ({
        depth,
        chips: [
          { left: 0, right: titleRight },
          { left: controlsLeft, right: 200 },
        ],
      });
      const metrics = (top: WindowSilhouetteEdge, bottom: WindowSilhouetteEdge = { depth: 0, chips: [] }): WindowSilhouetteMetrics => ({
        width: 200,
        height: 100,
        top,
        bottom,
      });
      expect(windowSilhouettePath(metrics(dockTop()), 0)).toBe("M0,0 H60 V24 H160 V0 H200 V100 H0 Z");
      expect(windowSilhouettePath(metrics(dockTop()))).toBe("M1,1 H60 V25 H160 V1 H199 V99 H1 Z");
      expect(
        windowSilhouettePath(
          metrics(dockTop(), {
            depth: 24,
            chips: [
              { left: 0, right: 50 },
              { left: 140, right: 200 },
            ],
          }),
          0,
        ),
      ).toBe("M0,0 H60 V24 H160 V0 H200 V100 H140 V76 H50 V100 H0 Z");
      expect(
        windowSilhouettePath(
          metrics(dockTop(), {
            depth: 24,
            chips: [
              { left: 0, right: 50 },
              { left: 140, right: 200 },
            ],
          }),
        ),
      ).toBe("M1,1 H60 V25 H160 V1 H199 V99 H140 V75 H50 V99 H1 Z");
      expect(windowSilhouettePath(metrics(dockTop(), { depth: 24, chips: [{ left: 140, right: 200 }] }), 0)).toBe("M0,0 H60 V24 H160 V0 H200 V100 H140 V76 H0 Z");
      expect(windowSilhouettePath(metrics(dockTop(), { depth: 24, chips: [{ left: 0, right: 50 }] }), 0)).toBe("M0,0 H60 V24 H160 V0 H200 V76 H50 V100 H0 Z");
      expect(
        windowSilhouettePath(
          metrics(dockTop(), {
            depth: 24,
            chips: [
              { left: 0, right: 50 },
              { left: 80, right: 120 },
              { left: 140, right: 200 },
            ],
          }),
          0,
        ),
      ).toBe("M0,0 H60 V24 H160 V0 H200 V100 H140 V76 H120 V100 H80 V76 H50 V100 H0 Z");
      expect(
        windowSilhouettePath(
          metrics(dockTop(), {
            depth: 24,
            chips: [
              { left: 0, right: 50 },
              { left: 80, right: 120 },
            ],
          }),
          0,
        ),
      ).toBe("M0,0 H60 V24 H160 V0 H200 V76 H120 V100 H80 V76 H50 V100 H0 Z");
      expect(
        windowSilhouettePath(
          metrics(dockTop(), {
            depth: 24,
            chips: [
              { left: 80, right: 120 },
              { left: 140, right: 200 },
            ],
          }),
          0,
        ),
      ).toBe("M0,0 H60 V24 H160 V0 H200 V100 H140 V76 H120 V100 H80 V76 H0 Z");
      expect(windowSilhouettePath(metrics(dockTop(), { depth: 24, chips: [{ left: 80, right: 120 }] }), 0)).toBe("M0,0 H60 V24 H160 V0 H200 V76 H120 V100 H80 V76 H0 Z");
      expect(windowSilhouettePath(metrics({ depth: 24, chips: [{ left: 0, right: 60 }] }), 0)).toBe("M0,0 H60 V24 H200 V100 H0 Z");
      expect(windowSilhouettePath(metrics({ depth: 0, chips: [] }), 0)).toBe("M0,0 H200 V100 H0 Z");
      expect(WINDOW_SILHOUETTE_PATH_INSET).toBeGreaterThanOrEqual(1);
    });

    it("normalizes physical chip spans deterministically across RTL order and malformed input", () => {
      expect(
        normalizeWindowSilhouetteChips(
          [
            { left: 160, right: 220 },
            { left: 60.25, right: 90 },
            { left: 0, right: 60 },
            { left: 90.25, right: 120 },
            { left: 80, right: 70 },
            { left: Number.NaN, right: 20 },
          ],
          200,
          0,
        ),
      ).toEqual([
        { left: 0, right: 120 },
        { left: 160, right: 200 },
      ]);
      expect(
        normalizeWindowSilhouetteMetrics({
          width: 200,
          height: 30,
          top: { depth: 24, chips: [{ left: 160, right: 200 }] },
          bottom: { depth: 24, chips: [{ left: 0, right: 40 }] },
        }),
      ).toEqual({
        width: 200,
        height: 30,
        top: { depth: 24, chips: [{ left: 160, right: 200 }] },
        bottom: { depth: 6, chips: [{ left: 0, right: 40 }] },
      });
    });

    it("derives zero-inset content, inset border, glass, containment, and safe clearances from one schema", () => {
      const geometry = createWindowSilhouetteGeometry({
        width: 200,
        height: 100,
        top: {
          depth: 24,
          chips: [
            { left: 160, right: 200 },
            { left: 0, right: 60 },
          ],
        },
        bottom: {
          depth: 16,
          chips: [
            { left: 80, right: 120 },
            { left: 0, right: 40 },
          ],
        },
      });
      expect(geometry.schema).toBe(WINDOW_SILHOUETTE_GEOMETRY_SCHEMA);
      expect(geometry.state).toBe("ready");
      expect(geometry.contentPath).toBe("M0,0 H60 V24 H160 V0 H200 V84 H120 V100 H80 V84 H40 V100 H0 Z");
      expect(geometry.borderPath).toBe("M1,1 H60 V25 H160 V1 H199 V83 H120 V99 H80 V83 H40 V99 H1 Z");
      expect(geometry.contentClipPath).toBe("polygon(0px 0px, 60px 0px, 60px 24px, 160px 24px, 160px 0px, 200px 0px, 200px 84px, 120px 84px, 120px 100px, 80px 100px, 80px 84px, 40px 84px, 40px 100px, 0px 100px)");
      expect(geometry.bodyRegion).toEqual({ x: 0, y: 24, width: 200, height: 60, kind: "body" });
      expect(geometry.glassRegions).toEqual([
        { x: 0, y: 0, width: 60, height: 24, kind: "chip", dock: "top" },
        { x: 160, y: 0, width: 40, height: 24, kind: "chip", dock: "top" },
        { x: 0, y: 84, width: 40, height: 16, kind: "chip", dock: "bottom" },
        { x: 80, y: 84, width: 40, height: 16, kind: "chip", dock: "bottom" },
      ]);
      expect(geometry.safeClearances).toEqual({ top: 24, right: 0, bottom: 16, left: 0 });
      expect(windowSilhouetteContains(geometry.metrics, 20, 12)).toBe(true);
      expect(windowSilhouetteContains(geometry.metrics, 100, 12)).toBe(false);
      expect(windowSilhouetteContains(geometry.metrics, 100, 50)).toBe(true);
      expect(windowSilhouetteContains(geometry.metrics, 60, 92)).toBe(false);
      expect(windowSilhouetteContains(geometry.metrics, 100, 92)).toBe(true);
    });

    it("keeps pending and chipless chrome bands as conservative pure cutouts", () => {
      const pending = createWindowSilhouetteGeometry(null, { width: 200, height: 100, topClearance: 24, bottomClearance: 12 });
      expect(pending.state).toBe("pending");
      expect(pending.glassRegions).toEqual([]);
      expect(pending.bodyRegion).toEqual({ x: 0, y: 24, width: 200, height: 64, kind: "body" });
      expect(pending.contentRegions).toEqual([pending.bodyRegion]);
      expect(pending.contentPath).toBe("M0,24 H200 V88 H0 Z");
      expect(pending.contentClipPath).toBe("polygon(0px 24px, 200px 24px, 200px 88px, 0px 88px)");
      expect(windowSilhouetteContains(pending.metrics, 100, 12)).toBe(false);
      expect(windowSilhouetteContains(pending.metrics, 100, 50)).toBe(true);
      expect(windowSilhouetteContains(pending.metrics, 100, 94)).toBe(false);
      expect(windowSilhouettePath({ width: 200, height: 100, top: { depth: 24, chips: [] }, bottom: { depth: 0, chips: [] } }, 0)).toBe("M0,24 H200 V100 H0 Z");
      expect(createWindowSilhouetteGeometry(null).contentClipPath).toBe("inset(100%)");
    });

    it("UI_ELEMENT_REGISTRY lists chrome components with status axis coverage", () => {
      expect(UI_ELEMENT_REGISTRY).toContain("Window");
      expect(UI_ELEMENT_REGISTRY.length).toBeGreaterThan(4);
    });

    it("chromeStatusBorderClass maps loading and waiting to border utilities", () => {
      expect(chromeStatusBorderClass("loading")).toContain("border-loading");
      expect(chromeStatusBorderClass("waiting")).toContain("border-waiting");
      expect(chromeStatusBorderClass("idle")).toBe("");
      expect(chromeStatusBorderClass(undefined)).toBe("");
    });

    it("windowSilhouetteOutlineViolations stays empty across absent chip slots", () => {
      const footerSlots = [
        { left: false, center: false, right: false },
        { left: true, center: false, right: false },
        { left: false, center: true, right: false },
        { left: false, center: false, right: true },
        { left: true, center: true, right: false },
        { left: true, center: false, right: true },
        { left: false, center: true, right: true },
        { left: true, center: true, right: true },
      ] as const;
      const titleModes = [
        { title: true, controls: true },
        { title: true, controls: false },
        { title: false, controls: true },
      ] as const;
      for (const titleMode of titleModes) {
        for (const footer of footerSlots) {
          const topChips: WindowSilhouetteChip[] = [];
          if (titleMode.title) topChips.push({ left: 0, right: 60 });
          if (titleMode.controls) topChips.push({ left: 160, right: 200 });
          const bottomChips: WindowSilhouetteChip[] = [];
          if (footer.left) bottomChips.push({ left: 0, right: 50 });
          if (footer.center) bottomChips.push({ left: 80, right: 120 });
          if (footer.right) bottomChips.push({ left: 140, right: 200 });
          const outline = windowSilhouetteOutline({
            width: 200,
            height: 100,
            top: { depth: topChips.length > 0 ? 24 : 0, chips: topChips },
            bottom: { depth: bottomChips.length > 0 ? 24 : 0, chips: bottomChips },
          });
          expect(windowSilhouetteOutlineViolations(outline, { x0: 0, y0: 0, x1: 200, y1: 100 })).toEqual([]);
        }
      }
    });

    it("windowSilhouetteBorderPaint shares one path across every border kind", () => {
      const metrics: WindowSilhouetteMetrics = {
        width: 200,
        height: 100,
        top: {
          depth: 24,
          chips: [
            { left: 0, right: 60 },
            { left: 160, right: 200 },
          ],
        },
        bottom: {
          depth: 24,
          chips: [
            { left: 0, right: 50 },
            { left: 80, right: 120 },
          ],
        },
      };
      const path = windowSilhouettePath(metrics, 0);
      for (const kind of WINDOW_SILHOUETTE_BORDER_KINDS) {
        expect(windowSilhouetteBorderPaint(kind).className).toContain("window-silhouette-border");
        expect(windowSilhouettePath(metrics, 0)).toBe(path);
      }
    });

    it("resolveWindowSilhouetteBorderKind prefers introduced over loading/waiting and falls back to active/normal", () => {
      const el = document.createElement("div");
      el.className = "border-loading border-waiting";
      expect(resolveWindowSilhouetteBorderKind(el)).toBe("loading");
      el.setAttribute("data-introduced", "true");
      expect(resolveWindowSilhouetteBorderKind(el)).toBe("introduced");
      el.removeAttribute("data-introduced");
      el.className = "border-waiting-active";
      expect(resolveWindowSilhouetteBorderKind(el)).toBe("waiting");
      el.className = "";
      expect(resolveWindowSilhouetteBorderKind(el)).toBe("normal");
      expect(resolveWindowSilhouetteBorderKind(el, true)).toBe("active");
      const scroll = document.createElement("div");
      scroll.id = "framework.window.puzzle3dMain";
      scroll.setAttribute("data-introduced", "true");
      el.appendChild(scroll);
      expect(resolveWindowSilhouetteBorderKind(el)).toBe("introduced");
      const utility = document.createElement("button");
      utility.id = "transform";
      utility.setAttribute("data-introduced", "true");
      el.replaceChildren(utility);
      expect(isWindowChromeIntroducedTarget(utility)).toBe(false);
      expect(resolveWindowSilhouetteBorderKind(el)).toBe("normal");
    });

    it("resolveWindowSilhouetteBorderKind prefers celebrated over a lingering introduced stamp", () => {
      const el = document.createElement("div");
      el.setAttribute("data-introduced", "true");
      expect(resolveWindowSilhouetteBorderKind(el)).toBe("introduced");
      el.setAttribute("data-celebrated", "true");
      expect(resolveWindowSilhouetteBorderKind(el)).toBe("celebrated");
      el.removeAttribute("data-introduced");
      expect(resolveWindowSilhouetteBorderKind(el)).toBe("celebrated");
      const scroll = document.createElement("div");
      scroll.id = "framework.window.puzzle3dMain";
      scroll.setAttribute("data-celebrated", "true");
      const outer = document.createElement("div");
      outer.appendChild(scroll);
      expect(resolveWindowSilhouetteBorderKind(outer)).toBe("celebrated");
    });
});
}
