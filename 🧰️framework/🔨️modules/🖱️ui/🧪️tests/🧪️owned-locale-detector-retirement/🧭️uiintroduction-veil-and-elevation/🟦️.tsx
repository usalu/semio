import * as ui from "@semio-tech/ui-react/source";
import type { TestSource, Tests1Vitest, Tests1Dependencies } from "../🧰️harness/🟦️.ts";

export async function registerUiintroductionVeilAndElevationTests(vitest: Tests1Vitest, dependencies: Tests1Dependencies, testSource: TestSource): Promise<void> {
  const { Mode, Pane, UIIntroduction, isWindowChromeIntroducedTarget, measureWindowSilhouetteMetrics, resolveWindowSilhouetteBorderKind, uiDataLabel, windowSilhouettePath } = dependencies;
  const { describe, expect, it, vi } = vitest;
  const { render, screen, fireEvent, waitFor } = await import("@testing-library/react");

  describe("UIIntroduction veil and elevation", () => {
    it("renders a single fullscreen veil and elevates the introduced panel above it", async () => {
      const { container } = render(
        <div>
          <div id="framework.panelTab.framework.panel.catalogue" data-slot="panel" data-elevation-root="" data-panel-visible="true" data-active-tab-id="framework.panel.catalogue" />
          <UIIntroduction
            introduction={{
              title: "Welcome",
              steps: [
                {
                  id: "catalogue",
                  title: "Der Katalog",
                  body: "Browse kinds.",
                  introduce: "framework.panelTab.framework.panel.catalogue",
                  show: [],
                  placement: "right",
                  interactions: [],
                  ordered: false,
                  logos: [],
                  demonstrations: [],
                },
              ],
            }}
            stepIndex={0}
            onStepIndexChange={vi.fn()}
            onDismiss={vi.fn()}
          />
        </div>,
      );
      await waitFor(() => {
        expect(container.querySelector('[data-slot="panel"]')?.getAttribute("data-introduced")).toBe("true");
      });
      expect(container.querySelector('[data-slot="panel"]')?.getAttribute("data-introduction-elevated")).toBe("true");
      expect(container.querySelectorAll(".ui-veil")).toHaveLength(1);
      expect(document.documentElement.getAttribute("data-introduction-active")).toBe("true");
    });

    it("clears data-introduction-active when the introduction unmounts so portaled overlays leave the tutorial layer", () => {
      const { unmount } = render(
        <UIIntroduction
          introduction={{
            title: "Welcome",
            steps: [
              {
                id: "welcome",
                title: "Hi",
                body: "Body",
                introduce: null,
                show: [],
                placement: "center",
                interactions: [],
                ordered: false,
                logos: [],
                demonstrations: [],
              },
            ],
          }}
          stepIndex={0}
          onStepIndexChange={vi.fn()}
          onDismiss={vi.fn()}
        />,
      );
      expect(document.documentElement.getAttribute("data-introduction-active")).toBe("true");
      unmount();
      expect(document.documentElement.getAttribute("data-introduction-active")).toBeNull();
    });

    it("introduces the first draggable tree item via its stamped alias, elevating the panel (not the row) and not pulsing siblings", async () => {
      const { container } = render(
        <div>
          <div data-slot="panel" data-elevation-root="" data-panel-visible="true">
            <div data-slot="tree-item-row" data-draggable="true" data-element-alias="framework.panelTab.framework.panel.catalogue.firstDraggable" id="puzzle3d-kind:first" />
            <div data-slot="tree-item-row" data-draggable="true" id="puzzle3d-kind:second" />
          </div>
          <UIIntroduction
            introduction={{
              title: "Welcome",
              steps: [
                {
                  id: "add-object",
                  title: "Baukomponente hinzufügen",
                  body: "Drag the first kind.",
                  introduce: "framework.panelTab.framework.panel.catalogue.firstDraggable",
                  show: [],
                  placement: "right",
                  interactions: [{ on: { kind: "action", id: "addObjectKind" }, label: "Add object" }],
                  ordered: false,
                  logos: [],
                  demonstrations: [],
                },
              ],
            }}
            stepIndex={0}
            onStepIndexChange={vi.fn()}
            onDismiss={vi.fn()}
          />
        </div>,
      );
      await waitFor(() => {
        expect(container.querySelector('[id="puzzle3d-kind:first"]')?.getAttribute("data-introduced")).toBe("true");
      });
      expect(container.querySelector('[id="puzzle3d-kind:second"]')?.getAttribute("data-introduced")).toBeNull();
      expect(container.querySelector('[data-slot="panel"]')?.getAttribute("data-introduced")).toBeNull();
      expect(container.querySelector('[data-slot="panel"]')?.getAttribute("data-introduction-elevated")).toBe("true");
      expect(container.querySelector('[id="puzzle3d-kind:first"]')?.getAttribute("data-introduction-elevated")).toBeNull();
    });

    it("elevates every open window instance aliased to a window-kind introduce id", async () => {
      const { container } = render(
        <div>
          <div data-slot="window" data-elevation-root="" id="puzzle3d-main-top">
            <div id="framework.window.puzzle3dMainTop" data-element-alias="framework.window.puzzle3dMain" />
          </div>
          <div data-slot="window" data-elevation-root="" id="puzzle3d-main-perspective">
            <div id="framework.window.puzzle3dMainPerspective" data-element-alias="framework.window.puzzle3dMain" />
          </div>
          <UIIntroduction
            introduction={{
              title: "Welcome",
              steps: [
                {
                  id: "viewport",
                  title: "Die 3D-Ansicht",
                  body: "Orbit, pan, and zoom.",
                  introduce: "framework.window.puzzle3dMain",
                  show: [],
                  placement: "auto",
                  interactions: [],
                  ordered: false,
                  logos: [],
                  demonstrations: [],
                },
              ],
            }}
            stepIndex={0}
            onStepIndexChange={vi.fn()}
            onDismiss={vi.fn()}
          />
        </div>,
      );
      await waitFor(() => {
        expect(container.querySelector('[id="puzzle3d-main-top"]')?.getAttribute("data-introduction-elevated")).toBe("true");
        expect(container.querySelector('[id="puzzle3d-main-perspective"]')?.getAttribute("data-introduction-elevated")).toBe("true");
      });
      expect(container.querySelector('[id="framework.window.puzzle3dMainTop"]')?.getAttribute("data-introduced")).toBe("true");
      expect(container.querySelector('[id="framework.window.puzzle3dMainPerspective"]')?.getAttribute("data-introduced")).toBe("true");
    });

    it("pulses an introduced utility toggle without promoting the dock-stack window silhouette", async () => {
      const { container } = render(
        <div className="h-layout-story w-layout-story-md">
          <Mode
            windows={[
              {
                id: "puzzle3d-main-top",
                iconId: "scene-3d",
                title: uiDataLabel("Top"),
                utilityBar: (
                  <button id="transform" type="button">
                    Transform
                  </button>
                ),
                utilityBarFolded: false,
                children: (
                  <div id="framework.window.puzzle3dMainTop" data-element-alias="framework.window.puzzle3dMain">
                    Main Pane
                  </div>
                ),
              },
            ]}
            layout={{ kind: "stack", children: [{ kind: "window", id: "puzzle3d-main-top" }], activeId: "puzzle3d-main-top" }}
            activeWindowId="puzzle3d-main-top"
            onActiveWindowChange={() => {}}
          />
          <UIIntroduction
            introduction={{
              title: "Welcome",
              steps: [
                {
                  id: "transform-utility",
                  title: "Transform",
                  body: "Activate transform.",
                  introduce: "transform",
                  show: ["framework.window.puzzle3dMain"],
                  placement: "auto",
                  interactions: [{ on: { kind: "utility", id: "transform" }, label: "Activate Transform" }],
                  ordered: false,
                  logos: [],
                  demonstrations: [],
                },
              ],
            }}
            stepIndex={0}
            onStepIndexChange={vi.fn()}
            onDismiss={vi.fn()}
          />
        </div>,
      );
      await waitFor(() => {
        expect(container.querySelector("#transform")?.getAttribute("data-introduced")).toBe("true");
      });
      expect(container.querySelector('[id="framework.window.puzzle3dMainTop"]')?.getAttribute("data-introduced")).toBeNull();
      expect(isWindowChromeIntroducedTarget(container.querySelector("#transform")!)).toBe(false);
      expect(resolveWindowSilhouetteBorderKind(container.querySelector('[data-slot="window"]'))).toBe("normal");
      await waitFor(() => {
        expect(container.querySelector('[data-slot="mode-dock-silhouette-border"]')?.getAttribute("data-kind")).not.toBe("introduced");
      });
    });

    it("elevates the mode-dock-stack silhouette and paints the SVG border when introducing a docked window", async () => {
      const { container } = render(
        <div className="h-layout-story w-layout-story-md">
          <Mode
            windows={[
              {
                id: "puzzle3d-main-top",
                iconId: "scene-3d",
                title: uiDataLabel("Top"),
                children: <div id="framework.window.puzzle3dMain">Main Pane</div>,
              },
            ]}
            layout={{ kind: "stack", children: [{ kind: "window", id: "puzzle3d-main-top" }], activeId: "puzzle3d-main-top" }}
            activeWindowId="puzzle3d-main-top"
            onActiveWindowChange={() => {}}
          />
          <UIIntroduction
            introduction={{
              title: "Welcome",
              steps: [
                {
                  id: "viewport",
                  title: "Viewport",
                  body: "All windows.",
                  introduce: "framework.window.puzzle3dMain",
                  show: [],
                  placement: "auto",
                  interactions: [],
                  ordered: false,
                  logos: [],
                  demonstrations: [],
                },
              ],
            }}
            stepIndex={0}
            onStepIndexChange={vi.fn()}
            onDismiss={vi.fn()}
          />
        </div>,
      );
      await waitFor(() => {
        expect(container.querySelector('[data-slot="mode-dock-stack"]')?.getAttribute("data-introduction-elevated")).toBe("true");
      });
      expect(container.querySelector('[data-slot="window"]')?.getAttribute("data-introduction-elevated")).toBeNull();
      expect(container.querySelector('[id="framework.window.puzzle3dMain"]')?.getAttribute("data-introduced")).toBe("true");
      expect(container.querySelector('[data-slot="window"]')?.getAttribute("data-introduced")).toBeNull();
      expect(resolveWindowSilhouetteBorderKind(container.querySelector('[data-slot="window"]'))).toBe("introduced");
      await waitFor(() => {
        expect(container.querySelector('[data-slot="mode-dock-silhouette-border"]')?.getAttribute("data-kind")).toBe("introduced");
      });
      expect(container.querySelector('[data-slot="mode-dock-stack"]')?.className).toContain("z-window");
      expect(container.querySelector('[data-slot="mode-dock-silhouette-border"]')?.className).toContain("z-[40]");
      const stack = container.querySelector('[data-slot="mode-dock-stack"]') as HTMLElement;
      const mockRect = (el: Element | null, rect: Partial<DOMRect>) => {
        if (!(el instanceof HTMLElement)) return;
        vi.spyOn(el, "getBoundingClientRect").mockReturnValue({
          x: 0,
          y: 0,
          top: 0,
          left: 0,
          bottom: 0,
          right: 0,
          width: 0,
          height: 0,
          toJSON: () => ({}),
          ...rect,
        } as DOMRect);
      };
      mockRect(stack, { width: 200, height: 100, right: 200, bottom: 100 });
      mockRect(stack.querySelector('[data-slot="mode-dock-tab-cap"]'), { left: 0, right: 60, width: 60, height: 24, bottom: 24 });
      mockRect(stack.querySelector('[data-slot="mode-dock-tab-gap"]'), { left: 60, right: 160, width: 100, height: 24, bottom: 24 });
      mockRect(stack.querySelector('[data-slot="mode-dock-controls-cap"]'), { left: 160, right: 200, width: 40, height: 24, bottom: 24 });
      mockRect(stack.querySelector('[data-slot="mode-dock-tabbar"]'), { height: 24, bottom: 24, width: 200, right: 200 });
      expect(measureWindowSilhouetteMetrics(stack)).toEqual({
        width: 200,
        height: 100,
        top: { depth: 24, chips: [{ left: 0, right: 60 }] },
        bottom: { depth: 0, chips: [] },
      });
      Object.defineProperty(stack, "offsetWidth", { configurable: true, value: 400 });
      Object.defineProperty(stack, "offsetHeight", { configurable: true, value: 200 });
      expect(measureWindowSilhouetteMetrics(stack)).toEqual({
        width: 400,
        height: 200,
        top: { depth: 48, chips: [{ left: 0, right: 120 }] },
        bottom: { depth: 0, chips: [] },
      });
      Reflect.deleteProperty(stack, "offsetWidth");
      Reflect.deleteProperty(stack, "offsetHeight");
      stack.setAttribute("data-silhouette-remeasure", "1");
      await waitFor(() => {
        const border = container.querySelector('[data-slot="mode-dock-silhouette-border"]');
        expect(border?.hasAttribute("data-pending")).toBe(false);
        expect(border?.getAttribute("data-kind")).toBe("introduced");
        expect(border?.querySelector("path")?.getAttribute("d")).toBe(
          windowSilhouettePath({
            width: 200,
            height: 100,
            top: { depth: 24, chips: [{ left: 0, right: 60 }] },
            bottom: { depth: 0, chips: [] },
          }),
        );
      });
    });

    it("keeps show elements interactive without pulsing them, self-elevating when there is no enclosing chrome unit", async () => {
      const { container } = render(
        <div>
          <div id="framework.window.puzzle3dMain" />
          <UIIntroduction
            introduction={{
              title: "Welcome",
              steps: [
                {
                  id: "add-object",
                  title: "Baukomponente hinzufügen",
                  body: "Drag the first kind.",
                  introduce: null,
                  show: ["framework.window.puzzle3dMain"],
                  placement: "right",
                  interactions: [],
                  ordered: false,
                  logos: [],
                  demonstrations: [],
                },
              ],
            }}
            stepIndex={0}
            onStepIndexChange={vi.fn()}
            onDismiss={vi.fn()}
          />
        </div>,
      );
      await waitFor(() => {
        expect(container.querySelector('[id="framework.window.puzzle3dMain"]')?.getAttribute("data-introduction-elevated")).toBe("true");
      });
      expect(container.querySelectorAll(".ui-veil")).toHaveLength(1);
      expect(container.querySelector('[id="framework.window.puzzle3dMain"]')?.getAttribute("data-introduced")).toBeNull();
    });

    it("blocks the veil's pointer-events only once a target resolves", async () => {
      const { container, rerender } = render(
        <div>
          <UIIntroduction
            introduction={{
              title: "Welcome",
              steps: [
                {
                  id: "add-object",
                  title: "Baukomponente hinzufügen",
                  body: "Drag the first kind.",
                  introduce: "framework.window.puzzle3dMain",
                  show: [],
                  placement: "right",
                  interactions: [],
                  ordered: false,
                  logos: [],
                  demonstrations: [],
                },
              ],
            }}
            stepIndex={0}
            onStepIndexChange={vi.fn()}
            onDismiss={vi.fn()}
          />
        </div>,
      );
      expect(container.querySelector(".ui-veil")?.className).toContain("pointer-events-none");
      rerender(
        <div>
          <div id="framework.window.puzzle3dMain" />
          <UIIntroduction
            introduction={{
              title: "Welcome",
              steps: [
                {
                  id: "add-object",
                  title: "Baukomponente hinzufügen",
                  body: "Drag the first kind.",
                  introduce: "framework.window.puzzle3dMain",
                  show: [],
                  placement: "right",
                  interactions: [],
                  ordered: false,
                  logos: [],
                  demonstrations: [],
                },
              ],
            }}
            stepIndex={0}
            onStepIndexChange={vi.fn()}
            onDismiss={vi.fn()}
          />
        </div>,
      );
      await waitFor(() => {
        expect(container.querySelector(".ui-veil")?.className).toContain("pointer-events-auto");
      });
    });

    it("ends the tour when its blocking veil is pressed, so a screen step can never leave the app inert", () => {
      const onDismiss = vi.fn();
      const { container } = render(
        <div>
          <UIIntroduction
            introduction={{
              title: "Welcome",
              steps: [{ id: "welcome", title: "Willkommen", body: "A quick tour.", introduce: null, show: [], placement: "center", interactions: [], ordered: false, logos: [], demonstrations: [] }],
            }}
            stepIndex={0}
            onStepIndexChange={vi.fn()}
            onDismiss={onDismiss}
          />
        </div>,
      );
      const veil = container.querySelector('[data-slot="introduction-veil"]')!;
      expect(veil.className).toContain("pointer-events-auto");
      expect(veil.getAttribute("aria-hidden")).toBe("true");
      fireEvent.pointerDown(veil);
      expect(onDismiss).toHaveBeenCalledWith(false);
    });

    it("never captures pointer events with no step to show", () => {
      const { container } = render(
        <div>
          <UIIntroduction
            introduction={{
              title: "Welcome",
              steps: [{ id: "welcome", title: "Willkommen", body: "A quick tour.", introduce: null, show: [], placement: "center", interactions: [], ordered: false, logos: [], demonstrations: [] }],
            }}
            stepIndex={1}
            onStepIndexChange={vi.fn()}
            onDismiss={vi.fn()}
          />
        </div>,
      );
      expect(container.querySelectorAll(".ui-veil")).toHaveLength(0);
      expect(container.querySelectorAll('[data-slot="introduction-veil"]')).toHaveLength(0);
    });

    it("clears elevation on step change and on unmount", async () => {
      const { container, rerender, unmount } = render(
        <div>
          <div id="framework.window.puzzle3dMain" data-elevation-root="" />
          <div id="framework.panelTab.framework.panel.catalogue" data-elevation-root="" />
          <UIIntroduction
            introduction={{
              title: "Welcome",
              steps: [
                { id: "a", title: "A", body: "A", introduce: "framework.window.puzzle3dMain", show: [], placement: "right", interactions: [], ordered: false, logos: [], demonstrations: [] },
                { id: "b", title: "B", body: "B", introduce: "framework.panelTab.framework.panel.catalogue", show: [], placement: "right", interactions: [], ordered: false, logos: [], demonstrations: [] },
              ],
            }}
            stepIndex={0}
            onStepIndexChange={vi.fn()}
            onDismiss={vi.fn()}
          />
        </div>,
      );
      const stepAWindow = container.querySelector('[id="framework.window.puzzle3dMain"]')!;
      const stepBPanel = container.querySelector('[id="framework.panelTab.framework.panel.catalogue"]')!;
      await waitFor(() => {
        expect(stepAWindow.getAttribute("data-introduction-elevated")).toBe("true");
      });
      rerender(
        <div>
          <div id="framework.window.puzzle3dMain" data-elevation-root="" />
          <div id="framework.panelTab.framework.panel.catalogue" data-elevation-root="" />
          <UIIntroduction
            introduction={{
              title: "Welcome",
              steps: [
                { id: "a", title: "A", body: "A", introduce: "framework.window.puzzle3dMain", show: [], placement: "right", interactions: [], ordered: false, logos: [], demonstrations: [] },
                { id: "b", title: "B", body: "B", introduce: "framework.panelTab.framework.panel.catalogue", show: [], placement: "right", interactions: [], ordered: false, logos: [], demonstrations: [] },
              ],
            }}
            stepIndex={1}
            onStepIndexChange={vi.fn()}
            onDismiss={vi.fn()}
          />
        </div>,
      );
      await waitFor(() => {
        expect(stepBPanel.getAttribute("data-introduction-elevated")).toBe("true");
      });
      expect(stepAWindow.getAttribute("data-introduction-elevated")).toBeNull();
      unmount();
      expect(stepBPanel.getAttribute("data-introduction-elevated")).toBeNull();
    });
  });
}
