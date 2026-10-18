import type { TestSource, Tests1Vitest, Tests1Dependencies } from "../🧰️harness/🟦️.ts";

export async function registerShellComponents9Tests(vitest: Tests1Vitest, dependencies: Tests1Dependencies, testSource: TestSource): Promise<void> {
  const { React, Window, WindowChrome, WindowMeasureTreeGroup, WindowMeasureTreeLeaf, WindowMeasuresTree, reactHostPort, uiDataLabel, uiSpacingPx, windowMeasuresDefaultWidthPx, windowMeasuresMinWidthPx, publishShellChromePanelBox, chromePanelSafeArea, chromePanelSafeAreaStyle, safeAreaBoxFromRect, useChromePanelSafeArea } = dependencies;
  const { describe, expect, it } = vitest;
  const { render, screen, fireEvent, waitFor, act } = await import("@testing-library/react");

describe("Shell components", () => {

    it("Window pane chrome uses semantic icons in U-cutout chips, never fold chevrons", () => {
      const { container } = render(
        <Window
          id="pane-chrome-window"
          engagement={{ status: [{ id: "s", content: "Idle" }] }}
          search={{ input: { placeholder: uiDataLabel("Action") } }}
          measures={<div data-testid="measure-slot">LOD</div>}
          utilityBar={<button type="button">Utility</button>}
        >
          <div>Body</div>
        </Window>,
      );
      const toggles = Array.from(container.querySelectorAll('[data-slot="window-pane-chrome-toggle"]'));
      expect(toggles.length).toBeGreaterThanOrEqual(4);
      expect(container.querySelector('[data-slot="window-measures-overlay"] [data-icon="settings-2"]')).toBeTruthy();
      expect(container.querySelector('[data-slot="window-engagement-overlay"] [data-icon="play"]')).toBeTruthy();
      expect(container.querySelector('[data-slot="window-search-overlay"] [data-icon="search"]')).toBeTruthy();
      expect(container.querySelector('[data-slot="utility-bar-overlay"] [data-icon="hammer"]')).toBeTruthy();
      expect(container.querySelectorAll('[data-slot="window-chrome-silhouette-border"]').length).toBeGreaterThanOrEqual(4);
      expect(container.querySelector('[data-slot="window-measures-overlay"] [data-icon="chevron-left"]')).toBeNull();
      expect(container.querySelector('[data-slot="window-engagement-overlay"] [data-icon="chevron-right"]')).toBeNull();
      expect(container.querySelector('[data-slot="window-search-overlay"] [data-icon="chevron-down"]')).toBeNull();
    });

    it("Window built-in panes anchor with frame inset via anchorPositionStyle, not extra padding", () => {
      const { container } = render(
        <Window id="inset-window" measures={<div>LOD</div>} engagement={{ status: [{ id: "s", content: "Idle" }] }}>
          <div>Body</div>
        </Window>,
      );
      const measures = container.querySelector('[data-slot="window-measures-overlay"]') as HTMLElement;
      expect(measures.style.right).toBe("var(--spacing-single)");
      expect(measures.style.top).toBe("var(--spacing-single)");
      const engagement = container.querySelector('[data-slot="window-engagement-overlay"]') as HTMLElement;
      expect(engagement.style.left).toBe("var(--spacing-single)");
      expect(engagement.style.top).toBe("var(--spacing-single)");
    });

    it("reserves an in-window affordance against the box an anchored chrome panel occupies", () => {
      // 🛟️ B47 §1.3 measured the Inspection panel body at x 1137‑1437 over the perspective window's
      // measures rail at x 1130‑1430, and the generation3d world pane measured the Tool runs panel at
      // (1137, 3) 300×120 over its own top-right overlay rail at (1355, 83) 75×16 — `Frame visible`
      // and the compute `Cancel` unreachable at every press. The reserve is the overlap and nothing
      // more; an affordance a panel does not cover keeps its flush inset exactly as authored.
      const worldPane = { left: 493, top: 26, right: 1440, bottom: 840 };
      const toolRunPanel = { left: 1137, top: 3, right: 1437, bottom: 123 };
      const overlayRail = { left: 1355, top: 83, right: 1430, bottom: 99 };
      const windowRightChromeColumn = { left: 1440 - windowMeasuresMinWidthPx, top: 26, right: 1440, bottom: 840 };

      expect(chromePanelSafeArea(overlayRail, worldPane, "top-right", [], "either", 8)).toEqual({ inlinePx: 0, blockPx: 0 });
      expect(chromePanelSafeArea(overlayRail, worldPane, "top-right", [{ left: 0, top: 3, right: 300, bottom: 123 }], "either", 8)).toEqual({ inlinePx: 0, blockPx: 0 });
      // 🛟️ Dropping below the panel costs 48 px, moving in past its left edge 301 — `"either"` takes the cheaper axis.
      expect(chromePanelSafeArea(overlayRail, worldPane, "top-right", [toolRunPanel], "either", 8)).toEqual({ inlinePx: 0, blockPx: 48 });
      expect(chromePanelSafeArea(overlayRail, worldPane, "top-right", [toolRunPanel], "block", 8)).toEqual({ inlinePx: 0, blockPx: 48 });
      expect(chromePanelSafeArea(overlayRail, worldPane, "top-right", [toolRunPanel], "inline", 8)).toEqual({ inlinePx: 301, blockPx: 0 });
      // 🛟️ A full-height right column has no bottom edge to drop under, so the block axis is not viable.
      expect(chromePanelSafeArea(overlayRail, worldPane, "top-right", [{ left: 1137, top: 3, right: 1437, bottom: 900 }], "either", 8)).toEqual({ inlinePx: 301, blockPx: 0 });
      // 🛟️ Two panels sharing the corner reserve their union, not one of them.
      expect(chromePanelSafeArea(overlayRail, worldPane, "top-right", [toolRunPanel, { left: 1300, top: 3, right: 1437, bottom: 240 }], "either", 8)).toEqual({ inlinePx: 0, blockPx: 165 });
      // 🛟️ The window's own right-edge chrome is pinned to its window's top and only yields inline.
      expect(chromePanelSafeArea(windowRightChromeColumn, worldPane, "top-right", [toolRunPanel], "inline", 0)).toEqual({ inlinePx: 303, blockPx: 0 });
      expect(chromePanelSafeArea(windowRightChromeColumn, worldPane, "top-right", [{ left: 300, top: 26, right: 470, bottom: 840 }], "inline", 0)).toEqual({ inlinePx: 0, blockPx: 0 });
      // 🛟️ An affordance a panel covers with no room to clear inside its host does not move at all —
      // a half-step frees nothing, and the answer there is a re-anchoring, not a clamp.
      expect(chromePanelSafeArea(windowRightChromeColumn, { left: 1300, top: 26, right: 1440, bottom: 840 }, "top-right", [toolRunPanel], "inline", 0)).toEqual({ inlinePx: 0, blockPx: 0 });
      // 🛟️ A middle anchor has no edge on that axis to reserve against.
      expect(chromePanelSafeArea(overlayRail, worldPane, "top-middle", [toolRunPanel], "either", 8)).toEqual({ inlinePx: 0, blockPx: 48 });
      expect(chromePanelSafeArea(overlayRail, worldPane, "right-middle", [toolRunPanel], "either", 8)).toEqual({ inlinePx: 301, blockPx: 0 });
      // 🛟️ A bottom-left affordance clears a bottom-left panel on its own two edges.
      expect(chromePanelSafeArea({ left: 500, top: 800, right: 620, bottom: 830 }, worldPane, "bottom-left", [{ left: 493, top: 700, right: 793, bottom: 840 }], "either", 8)).toEqual({ inlinePx: 0, blockPx: 138 });
      expect(chromePanelSafeArea({ left: 500, top: 800, right: 620, bottom: 830 }, worldPane, "bottom-left", [{ left: 493, top: 700, right: 793, bottom: 840 }], "inline", 8)).toEqual({ inlinePx: 301, blockPx: 0 });

      expect(chromePanelSafeAreaStyle("top-right", { inlinePx: 303, blockPx: 0 })).toEqual({ top: "var(--spacing-single)", right: "calc(var(--spacing-single) + 303px)" });
      expect(chromePanelSafeAreaStyle("top-right", { inlinePx: 0, blockPx: 48 }, { block: "var(--chrome)" })).toEqual({ top: "calc(var(--chrome) + 48px)", right: "var(--spacing-single)" });
      expect(chromePanelSafeAreaStyle("top-left", { inlinePx: 303, blockPx: 0 })).toEqual({ top: "var(--spacing-single)", left: "calc(var(--spacing-single) + 303px)" });
      expect(chromePanelSafeAreaStyle("top-middle", { inlinePx: 303, blockPx: 0 })).toEqual({ top: "var(--spacing-single)" });
      expect(safeAreaBoxFromRect({ left: 1136.6, top: 3.4, right: 1437.2, bottom: 122.8 } as DOMRect)).toEqual({ left: 1137, top: 3, right: 1437, bottom: 123 });
    });

    it("keeps window pane toggles flush when an anchored chrome panel opens over them", () => {
      const originalRect = Element.prototype.getBoundingClientRect;
      Element.prototype.getBoundingClientRect = function stubbedRect(this: Element): DOMRect {
        if (this.getAttribute("data-slot") !== "window-body") return originalRect.call(this);
        return { x: 493, y: 26, width: 947, height: 814, top: 26, right: 1440, bottom: 840, left: 493, toJSON: () => ({}) } as DOMRect;
      };
      try {
        const { container } = render(
          <Window id="behind-panel-window" measures={<div>LOD</div>}>
            <div>Body</div>
          </Window>,
        );
        const overlay = container.querySelector('[data-slot="window-measures-overlay"]') as HTMLElement;
        expect(overlay.style.right).toBe("var(--spacing-single)");
        act(() => {
          publishShellChromePanelBox(undefined, "panel:top-right", "top-right", { left: 1137, top: 26, right: 1437, bottom: 840 });
        });
        expect(overlay.style.right).toBe("var(--spacing-single)");
        act(() => {
          publishShellChromePanelBox(undefined, "panel:top-right", "top-right", null);
        });
        expect(overlay.style.right).toBe("var(--spacing-single)");
      } finally {
        Element.prototype.getBoundingClientRect = originalRect;
        publishShellChromePanelBox(undefined, "panel:top-right", "top-right", null);
      }
    });

    it("keeps a window content overlay rail hit-testable under an anchored chrome panel", async () => {
      // 🛟️ The generation3d defect as its own law. A `z-panel` chrome panel in the app root's stacking
      // context always paints above `z-window` window content, so while the two boxes overlap the rail can
      // never take a press — `document.elementFromPoint` on `Frame visible`'s own centre returns the panel,
      // which is what made `fit` 0/8 and the preview `Cancel` unreachable. jsdom runs no layout, so the
      // geometry is stubbed and that stacking premise is modelled directly: the point resolves to the
      // chrome panel wherever it covers it, and to the rail's button otherwise. Runtime `elementFromPoint`
      // evidence is the ticket's own browser battery on :6022.
      const panelBox = { x: 1137, y: 3, width: 300, height: 120, top: 3, right: 1437, bottom: 123, left: 1137 };
      const hostBox = { x: 493, y: 26, width: 947, height: 814, top: 26, right: 1440, bottom: 840, left: 493 };
      const railTopBase = 57;
      const railTopBaseVar = "var(--safe-area-rail-base)";
      let rail: HTMLElement | null = null;
      const railBox = () => {
        const top = railTopBase + Number(rail?.getAttribute("data-safe-area-block") ?? 0);
        return { x: 1355, y: top, width: 75, height: 16, top, right: 1430, bottom: top + 16, left: 1355 };
      };
      const originalRect = Element.prototype.getBoundingClientRect;
      Element.prototype.getBoundingClientRect = function stubbedRect(this: Element): DOMRect {
        const slot = this.getAttribute("data-slot");
        if (slot === "safe-area-panel") return { ...panelBox, toJSON: () => ({}) } as DOMRect;
        if (slot === "safe-area-host") return { ...hostBox, toJSON: () => ({}) } as DOMRect;
        if (slot === "world-view-overlay-rail" || slot === "world-frame-instances") return { ...railBox(), toJSON: () => ({}) } as DOMRect;
        return originalRect.call(this);
      };
      const covers = (box: { left: number; right: number; top: number; bottom: number }, x: number, y: number) => x >= box.left && x <= box.right && y >= box.top && y <= box.bottom;
      const elementFromPoint = (x: number, y: number): Element | null => {
        const panel = document.querySelector('[data-slot="safe-area-panel"]');
        if (panel && covers(panelBox, x, y)) return panel;
        const button = document.querySelector('[data-slot="world-frame-instances"]');
        if (button && covers(railBox(), x, y)) return button;
        return null;
      };
      const SafeAreaRail: React.FC = () => {
        const hostRef = reactHostPort.useRef<HTMLDivElement | null>(null);
        const railRef = reactHostPort.useRef<HTMLDivElement | null>(null);
        const safeArea = useChromePanelSafeArea({ hostRef, affordanceRef: railRef, anchor: "top-right", yieldAxis: "either", gapPx: uiSpacingPx(1) });
        return (
          <div ref={hostRef} data-slot="safe-area-host">
            <div data-slot="safe-area-panel">Tool runs</div>
            <div ref={railRef} data-slot="world-view-overlay-rail" data-safe-area-block={safeArea.blockPx || undefined} data-safe-area-inline={safeArea.inlinePx || undefined} style={chromePanelSafeAreaStyle("top-right", safeArea, { block: railTopBaseVar })}>
              <button type="button" data-slot="world-frame-instances">
                Frame visible
              </button>
            </div>
          </div>
        );
      };
      try {
        const { container } = render(
          <Window id="safe-area-window">
            <SafeAreaRail />
          </Window>,
        );
        rail = container.querySelector('[data-slot="world-view-overlay-rail"]') as HTMLElement;
        const panel = container.querySelector('[data-slot="safe-area-panel"]') as HTMLElement;
        const button = container.querySelector('[data-slot="world-frame-instances"]') as HTMLElement;
        const unreserved = railBox();
        expect(rail.style.top).toBe(railTopBaseVar);
        expect(elementFromPoint((unreserved.left + unreserved.right) / 2, (unreserved.top + unreserved.bottom) / 2)).toBe(panel);
        act(() => {
          publishShellChromePanelBox(undefined, "panel:top-right", "top-right", { left: panelBox.left, top: panelBox.top, right: panelBox.right, bottom: panelBox.bottom });
        });
        const reserve = Math.ceil(panelBox.bottom + uiSpacingPx(1) - railTopBase);
        await waitFor(() => expect(rail!.getAttribute("data-safe-area-block")).toBe(String(reserve)));
        expect(rail.style.top).toBe(`calc(${railTopBaseVar} + ${reserve}px)`);
        expect(rail.getAttribute("data-safe-area-inline")).toBeNull();
        const reserved = railBox();
        expect(reserved.top).toBeGreaterThan(panelBox.bottom);
        expect(elementFromPoint((reserved.left + reserved.right) / 2, (reserved.top + reserved.bottom) / 2)).toBe(button);
        act(() => {
          publishShellChromePanelBox(undefined, "panel:top-right", "top-right", null);
        });
        await waitFor(() => expect(rail!.getAttribute("data-safe-area-block")).toBeNull());
        expect(rail.style.top).toBe(railTopBaseVar);
        expect(elementFromPoint((unreserved.left + unreserved.right) / 2, (unreserved.top + unreserved.bottom) / 2)).toBe(panel);
      } finally {
        Element.prototype.getBoundingClientRect = originalRect;
        publishShellChromePanelBox(undefined, "panel:top-right", "top-right", null);
      }
    });

    it("Window measures overlay uses a fixed right rail without clipping overflow", () => {
      const { container } = render(
        <Window id="measures-window" measures={<div data-testid="measure-slot">LOD</div>}>
          <div>Body</div>
        </Window>,
      );
      fireEvent.click(container.querySelector('[id="framework.window.measuresWindow.measures.unfold"]')!);
      const overlay = container.querySelector('[data-slot="window-measures-overlay"]') as HTMLElement;
      expect(overlay.style.width).toBe(`${windowMeasuresDefaultWidthPx}px`);
      expect(overlay.style.right).toBe("var(--spacing-single)");
      expect(container.querySelector('[data-testid="measure-slot"]')).toBeTruthy();
      expect(container.querySelector('[data-slot="window-chrome-gap"]')).toBeTruthy();
      expect(container.querySelector('[data-slot="pane-resize-handle"]')).toBeTruthy();
      expect(screen.getByText("Window Options")).toBeTruthy();
    });

    it("Window measures chrome fold hides the options body and unfold restores it", () => {
      const { container } = render(
        <Window id="measures-fold-window" measures={<div data-testid="measure-slot">LOD</div>}>
          <div>Body</div>
        </Window>,
      );
      fireEvent.click(container.querySelector('[id="framework.window.measuresFoldWindow.measures.unfold"]')!);
      const overlay = container.querySelector('[data-slot="window-measures-overlay"]') as HTMLElement;
      expect(container.querySelector('[id="framework.window.measuresFoldWindow.measures.fold"]')).toBeTruthy();
      expect(container.querySelector('[data-slot="window-measures-body"]')).toBeTruthy();
      fireEvent.click(container.querySelector('[id="framework.window.measuresFoldWindow.measures.fold"]')!);
      expect(container.querySelector('[data-slot="window-measures-body"]')).toBeNull();
      expect(container.querySelector('[data-slot="window-measures-stack"]')?.getAttribute("data-folded")).toBe("true");
      expect(overlay.getAttribute("data-folded")).toBe("true");
      fireEvent.click(container.querySelector('[id="framework.window.measuresFoldWindow.measures.unfold"]')!);
      expect(container.querySelector('[data-slot="window-measures-body"]')).toBeTruthy();
      expect(container.querySelector('[data-slot="window-measures-stack"]')?.getAttribute("data-folded")).toBeNull();
      expect(overlay.style.width).toBe(`${windowMeasuresDefaultWidthPx}px`);
      expect(container.querySelector('[data-slot="pane-resize-handle"]')).toBeTruthy();
    });

    it("Window measures rail resizes from the left edge when unfolded", () => {
      const { container } = render(
        <Window id="measures-resize-window" measures={<div data-testid="measure-slot">LOD</div>}>
          <div className="h-64 w-96">Body</div>
        </Window>,
      );
      fireEvent.click(container.querySelector('[id="framework.window.measuresResizeWindow.measures.unfold"]')!);
      const overlay = container.querySelector('[data-slot="window-measures-overlay"]') as HTMLElement;
      const stack = container.querySelector('[data-slot="window-measures-stack"]') as HTMLElement;
      const leftHandle = container.querySelector('[data-slot="pane-resize-handle"]') as HTMLElement;
      expect(overlay.style.width).toBe(`${windowMeasuresDefaultWidthPx}px`);
      expect(stack.style.height).toBe("");
      fireEvent.pointerDown(leftHandle, { clientX: 300, pointerId: 1 });
      fireEvent.pointerMove(leftHandle, { clientX: 260, pointerId: 1 });
      fireEvent.pointerUp(leftHandle, { pointerId: 1 });
      expect(Number.parseInt(overlay.style.width, 10)).toBeGreaterThan(windowMeasuresDefaultWidthPx);
    });

    it("Window measures rail height follows tree content and scrolls at the window bottom", () => {
      const { container } = render(
        <Window
          id="measures-content-window"
          measures={
            <WindowMeasuresTree>
              <WindowMeasureTreeGroup id="group-a" label="Group A" defaultOpen={false}>
                <WindowMeasureTreeLeaf label={uiDataLabel("Alpha")}>
                  <span>Alpha value</span>
                </WindowMeasureTreeLeaf>
              </WindowMeasureTreeGroup>
            </WindowMeasuresTree>
          }
        >
          <div className="h-32 w-96">Body</div>
        </Window>,
      );
      fireEvent.click(container.querySelector('[id="framework.window.measuresContentWindow.measures.unfold"]')!);
      const overlay = container.querySelector('[data-slot="window-measures-overlay"]') as HTMLElement;
      const body = container.querySelector('[data-slot="window-measures-body"]') as HTMLElement;
      expect(body.className).toContain("overflow-y-auto");
      expect(body.className).toContain("flex-auto");
      expect(screen.queryByText("Alpha value")).toBeNull();
      fireEvent.click(container.querySelector('[data-slot="window-measures-tree"] button')!);
      expect(screen.getByText("Alpha value")).toBeTruthy();
      expect(overlay.style.maxWidth).toContain("300px");
    });

    it("Window utility bar folds and unfolds on WindowChrome, on the same surface as panels", () => {
      const { container, rerender } = render(
        <Window id="utility-bar-window" utilityBar={<button type="button">Utility</button>}>
          <div>Body</div>
        </Window>,
      );
      const stack = container.querySelector('[data-slot="utility-bar"]') as HTMLElement;
      expect(stack).toBeTruthy();
      expect(stack.getAttribute("data-level")).toBe("pane");
      expect(stack.querySelector('[data-slot="window-chrome-silhouette-border"]')).toBeTruthy();
      expect(screen.queryByText("Utility")).toBeNull();
      fireEvent.click(container.querySelector('[id="framework.window.utilityBarWindow.utilityBar.unfold"]')!);
      expect(screen.getByText("Utility")).toBeTruthy();
      expect(container.querySelector('[data-slot="utility-bar-body"]')).toBeTruthy();
      fireEvent.click(container.querySelector('[id="framework.window.utilityBarWindow.utilityBar.fold"]')!);
      expect(screen.queryByText("Utility")).toBeNull();
      rerender(
        <Window id="utility-bar-window">
          <div>Body</div>
        </Window>,
      );
      expect(container.querySelector('[data-slot="utility-bar"]')).toBeTruthy();
      expect((container.querySelector('[id="framework.window.utilityBarWindow.utilityBar.unfold"]') as HTMLButtonElement).disabled).toBe(true);
    });

    it("Window utility bar anchors bottom-left alongside measures", () => {
      const { container } = render(
        <Window id="utility-bar-measures-window" utilityBar={<button type="button">Utility</button>} measures={<div data-testid="measure-slot">LOD</div>}>
          <div>Body</div>
        </Window>,
      );
      const overlay = container.querySelector('[data-slot="utility-bar-overlay"]') as HTMLElement;
      expect(overlay.style.bottom).toBe("var(--spacing-single)");
      expect(overlay.style.left).toBe("var(--spacing-single)");
      fireEvent.click(container.querySelector('[id="framework.window.utilityBarMeasuresWindow.measures.unfold"]')!);
      expect(container.querySelector('[data-slot="utility-bar-overlay"]')).toBeTruthy();
    });

    it("Window utility bar body caps its height below the top-left Actions chrome and scrolls the overflow, instead of overlapping it (regression guard for a tall active-utility options tree, e.g. puzzle 3D's Füllen tree)", () => {
      const originalDescriptor = Object.getOwnPropertyDescriptor(HTMLElement.prototype, "getBoundingClientRect");
      const originalRect = HTMLElement.prototype.getBoundingClientRect;
      HTMLElement.prototype.getBoundingClientRect = function (this: HTMLElement) {
        if (this.getAttribute("data-slot") === "window-body") return { top: 0, bottom: 400, height: 400, left: 0, right: 800, width: 800, x: 0, y: 0, toJSON: () => ({}) } as DOMRect;
        if (this.getAttribute("data-slot") === "window-engagement-overlay") return { top: 0, bottom: 120, height: 120, left: 0, right: 200, width: 200, x: 0, y: 0, toJSON: () => ({}) } as DOMRect;
        return originalRect.call(this);
      };
      try {
        const { container } = render(
          <Window id="utility-bar-clearance-window" active actionPane={<div data-testid="adhoc-actions">Füllen</div>} utilityBar={<button type="button">Utility</button>}>
            <div>Body</div>
          </Window>,
        );
        fireEvent.click(container.querySelector('[id="framework.window.utilityBarClearanceWindow.engagement.toggle"]')!);
        fireEvent.click(container.querySelector('[id="framework.window.utilityBarClearanceWindow.utilityBar.unfold"]')!);
        const body = container.querySelector('[data-slot="utility-bar-body"]') as HTMLElement;
        expect(body.className).toContain("overflow-y-auto");
        expect(Number.parseFloat(body.style.maxHeight)).toBe(400 - 120 - uiSpacingPx(1));
      } finally {
        if (originalDescriptor) Object.defineProperty(HTMLElement.prototype, "getBoundingClientRect", originalDescriptor);
        else Reflect.deleteProperty(HTMLElement.prototype, "getBoundingClientRect");
      }
    });
});
}
