import type { PanelTabNode, PanelTabRowDropTarget } from "@semio-tech/ui-react/source";
import { renderToStaticMarkup } from "react-dom/server";
import type { TestSource, Tests1Vitest, Tests1Dependencies } from "../🧰️harness/🟦️.ts";

export async function registerShellComponents3Tests(vitest: Tests1Vitest, dependencies: Tests1Dependencies, testSource: TestSource): Promise<void> {
  const { Button, Canvas, DEFAULT_UI_DRIVER, Label, Layout, Navbar, Panel, PanelTabBar, React, Toggle, TreeContext, UI_ELEMENT_REGISTRY, UiDriverProvider, WINDOW_FRAME_BORDER_KINDS, WINDOW_FRAME_GEOMETRY_SCHEMA, WINDOW_FRAME_PATH_INSET, Window, chromeStatusBorderClass, computeTabDockDropZone, createWindowFrameGeometry, isWindowChromeIntroducedTarget, modeDockChromeGridPlacement, modeDockTabLabelClassName, normalizeWindowFrameChips, normalizeWindowFrameMetrics, renderToStaticMarkup, resolveWindowFrameBorderKind, singleTreeLeaf, useFlow, windowFrameBorderPaint, windowFrameContains, windowFrameBorderPath } = dependencies;
  type Anchor = any;
  type PanelTabNode = any;
  type PanelTabRowDropTarget = any;
  type WindowFrameChip = any;
  type WindowFrameEdge = any;
  type WindowFrameMetrics = any;
  const { describe, expect, it } = vitest;
  const { render, screen } = await import("@testing-library/react");
  const clippingModule = await import("polygon-clipping");
  const clipping = "default" in clippingModule ? (clippingModule.default as typeof clippingModule) : clippingModule;

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

    it("windowFrameBorderPath is the inset rectangle whatever the chips are", () => {
      const chipSets: WindowFrameChip[][] = [[], [{ left: 0, right: 60 }], [{ left: 0, right: 60 }, { left: 160, right: 200 }], [{ left: 0, right: 50 }, { left: 80, right: 120 }, { left: 140, right: 200 }]];
      for (const top of chipSets) {
        for (const bottom of chipSets) {
          const metrics: WindowFrameMetrics = { width: 200, height: 100, top: { depth: top.length > 0 ? 24 : 0, chips: top }, bottom: { depth: bottom.length > 0 ? 24 : 0, chips: bottom } };
          expect(windowFrameBorderPath(metrics, 0)).toBe("M0,0 H200 V100 H0 Z");
          expect(windowFrameBorderPath(metrics)).toBe("M1,1 H199 V99 H1 Z");
        }
      }
      expect(WINDOW_FRAME_PATH_INSET).toBeGreaterThanOrEqual(1);
    });

    it("normalizes physical chip spans deterministically across RTL order and malformed input", () => {
      expect(
        normalizeWindowFrameChips(
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
        normalizeWindowFrameMetrics({
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

    it("derives frame, inset border, chip overlays, containment, and safe clearances from one schema", () => {
      const geometry = createWindowFrameGeometry({
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
      expect(geometry.schema).toBe(WINDOW_FRAME_GEOMETRY_SCHEMA);
      expect(geometry.state).toBe("ready");
      expect(geometry.frame).toEqual({ x: 0, y: 0, width: 200, height: 100 });
      expect(geometry.borderPath).toBe("M1,1 H199 V99 H1 Z");
      expect(geometry.chipRegions).toEqual([
        { x: 0, y: 0, width: 60, height: 24, dock: "top" },
        { x: 160, y: 0, width: 40, height: 24, dock: "top" },
        { x: 0, y: 84, width: 40, height: 16, dock: "bottom" },
        { x: 80, y: 84, width: 40, height: 16, dock: "bottom" },
      ]);
      expect(geometry.safeClearances).toEqual({ top: 24, right: 0, bottom: 16, left: 0 });
      for (const removed of ["contentClipPath", "contentPath", "bodyRegion", "contentRegions", "glassRegions", "gapRegions"]) expect(geometry).not.toHaveProperty(removed);
      for (const [x, y] of [[20, 12], [100, 12], [100, 50], [60, 92], [100, 92], [0, 0], [200, 100]] as const) expect(windowFrameContains(geometry.metrics, x, y)).toBe(true);
      for (const [x, y] of [[-1, 50], [201, 50], [100, -1], [100, 101]] as const) expect(windowFrameContains(geometry.metrics, x, y)).toBe(false);
    });

    it("keeps chip overlays inside the frame, pairwise disjoint, and the uncovered chip row as content (independent polygon-clipping oracle)", () => {
      const chipSets: WindowFrameChip[][] = [[], [{ left: 0, right: 60 }], [{ left: 140, right: 200 }], [{ left: 0, right: 50 }, { left: 80, right: 120 }, { left: 140, right: 200 }]];
      const toRing = (r: { x: number; y: number; width: number; height: number }) => [[r.x, r.y], [r.x + r.width, r.y], [r.x + r.width, r.y + r.height], [r.x, r.y + r.height], [r.x, r.y]] as [number, number][];
      const area = (multi: ReturnType<typeof clipping.union>) => multi.reduce((sum, polygon) => sum + polygon.reduce((acc, ring, index) => acc + (index === 0 ? 1 : -1) * Math.abs(ring.slice(1).reduce((a, [x, y], i) => a + ring[i]![0] * y - x * ring[i]![1], 0) / 2), 0), 0);
      for (const top of chipSets) {
        for (const bottom of chipSets) {
          const geometry = createWindowFrameGeometry({ width: 200, height: 100, top: { depth: top.length > 0 ? 24 : 0, chips: top }, bottom: { depth: bottom.length > 0 ? 24 : 0, chips: bottom } });
          const frame = [[toRing(geometry.frame)]] as [number, number][][][];
          const regions = geometry.chipRegions.map((region) => [[toRing(region)]] as [number, number][][][]);
          expect(regions).toHaveLength(top.length + bottom.length);
          for (const region of regions) expect(area(clipping.difference(region, frame))).toBe(0);
          for (let i = 0; i < regions.length; i += 1) for (let j = i + 1; j < regions.length; j += 1) expect(area(clipping.intersection(regions[i]!, regions[j]!))).toBe(0);
          const chipArea = regions.reduce((sum, region) => sum + area(region), 0);
          const uncovered = regions.length === 0 ? area(frame) : area(clipping.difference(frame, clipping.union(regions[0]!, ...regions.slice(1))));
          expect(uncovered).toBe(20000 - chipArea);
          expect(windowFrameBorderPath(geometry.metrics, 0)).toBe("M0,0 H200 V100 H0 Z");
        }
      }
    });

    it("pending and chipless chrome keep the full frame and report conservative clearances", () => {
      const pending = createWindowFrameGeometry(null, { width: 200, height: 100, topClearance: 24, bottomClearance: 12 });
      expect(pending.state).toBe("pending");
      expect(pending.frame).toEqual({ x: 0, y: 0, width: 200, height: 100 });
      expect(pending.chipRegions).toEqual([]);
      expect(pending.safeClearances).toEqual({ top: 24, right: 0, bottom: 12, left: 0 });
      expect(pending.borderPath).toBe("M1,1 H199 V99 H1 Z");
      for (const removed of ["contentClipPath", "contentPath", "bodyRegion", "contentRegions"]) expect(pending).not.toHaveProperty(removed);
      for (const y of [12, 50, 94]) expect(windowFrameContains(pending.metrics, 100, y)).toBe(true);
      const chipless = createWindowFrameGeometry({ width: 200, height: 100, top: { depth: 24, chips: [] }, bottom: { depth: 0, chips: [] } });
      expect(chipless.chipRegions).toEqual([]);
      expect(chipless.frame).toEqual({ x: 0, y: 0, width: 200, height: 100 });
      expect(createWindowFrameGeometry(null).chipRegions).toEqual([]);
      expect(createWindowFrameGeometry(null)).not.toHaveProperty("contentClipPath");
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

    it("window frame exports no outline polygon, violation or gap API", () => {
      for (const removed of ["windowSilhouetteOutline", "windowSilhouetteOutlineViolations", "windowSilhouettePath", "createWindowSilhouetteGeometry"]) expect(dependencies).not.toHaveProperty(removed);
    });

    it("windowFrameBorderPaint shares one rectangular path across every border kind", () => {
      const metrics: WindowFrameMetrics = {
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
      const path = windowFrameBorderPath(metrics, 0);
      for (const kind of WINDOW_FRAME_BORDER_KINDS) {
        expect(windowFrameBorderPaint(kind).className).toContain("window-frame-border");
        expect(windowFrameBorderPath(metrics, 0)).toBe(path);
        expect(path).toBe("M0,0 H200 V100 H0 Z");
      }
    });

    it("resolveWindowFrameBorderKind prefers introduced over loading/waiting and falls back to active/normal", () => {
      const el = document.createElement("div");
      el.className = "border-loading border-waiting";
      expect(resolveWindowFrameBorderKind(el)).toBe("loading");
      el.setAttribute("data-introduced", "true");
      expect(resolveWindowFrameBorderKind(el)).toBe("introduced");
      el.removeAttribute("data-introduced");
      el.className = "border-waiting-active";
      expect(resolveWindowFrameBorderKind(el)).toBe("waiting");
      el.className = "";
      expect(resolveWindowFrameBorderKind(el)).toBe("normal");
      expect(resolveWindowFrameBorderKind(el, true)).toBe("active");
      const scroll = document.createElement("div");
      scroll.id = "framework.window.puzzle3dMain";
      scroll.setAttribute("data-introduced", "true");
      el.appendChild(scroll);
      expect(resolveWindowFrameBorderKind(el)).toBe("introduced");
      const utility = document.createElement("button");
      utility.id = "transform";
      utility.setAttribute("data-introduced", "true");
      el.replaceChildren(utility);
      expect(isWindowChromeIntroducedTarget(utility)).toBe(false);
      expect(resolveWindowFrameBorderKind(el)).toBe("normal");
    });

    it("resolveWindowFrameBorderKind prefers celebrated over a lingering introduced stamp", () => {
      const el = document.createElement("div");
      el.setAttribute("data-introduced", "true");
      expect(resolveWindowFrameBorderKind(el)).toBe("introduced");
      el.setAttribute("data-celebrated", "true");
      expect(resolveWindowFrameBorderKind(el)).toBe("celebrated");
      el.removeAttribute("data-introduced");
      expect(resolveWindowFrameBorderKind(el)).toBe("celebrated");
      const scroll = document.createElement("div");
      scroll.id = "framework.window.puzzle3dMain";
      scroll.setAttribute("data-celebrated", "true");
      const outer = document.createElement("div");
      outer.appendChild(scroll);
      expect(resolveWindowFrameBorderKind(outer)).toBe("celebrated");
    });
});
}
