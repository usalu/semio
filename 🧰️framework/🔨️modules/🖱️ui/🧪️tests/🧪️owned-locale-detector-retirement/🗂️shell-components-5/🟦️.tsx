import type { WindowLayoutNode } from "../../../🔨️modules/🗺️window-layout/🟦️.ts";
import * as ui from "@semio-tech/ui-react/source";
import type { TestSource, Tests1Vitest, Tests1Dependencies } from "../🧰️harness/🟦️.ts";

export async function registerShellComponents5Tests(vitest: Tests1Vitest, dependencies: Tests1Dependencies, testSource: TestSource): Promise<void> {
  const { Mode, applyAxisGroupLayoutDelta, applyModeDrop, applyModeJoinCornerResize, computeModeSplitPreviewInBody, computeTabInsertPreview, modeDockTabsWithInsertPreview, modeJoinCornerSpecsForCrossSeparator, modeJoinCornerSpecsForSeparator, modePerpendicularJoinSeparators, readResizableJoinCornerSpec, resolveJoinCornerPeerCrossAxes, resolveModeSplitSideInBody, uiDataLabel } = dependencies;
  type ResizableJoinCornerSpec = any;
  type WindowLayoutNode = any;
  const { describe, expect, it, vi } = vitest;
  const { render, screen, fireEvent, waitFor } = await import("@testing-library/react");

describe("Shell components", () => {

    it("Mode tab stack clips one active payload beneath every chip while preserving the gap cutout", async () => {
      const { container } = render(
        <div className="h-layout-story w-layout-story-md">
          <Mode
            windows={[
              { id: "shape", title: uiDataLabel("Shape"), iconId: "app-window", children: <div>Shape Body</div> },
              { id: "energy", title: uiDataLabel("Energy"), iconId: "app-window", children: <div>Energy Body</div> },
            ]}
            layout={{
              kind: "stack",
              children: [
                { kind: "window", id: "shape" },
                { kind: "window", id: "energy" },
              ],
              activeId: "energy",
            }}
            activeWindowId="energy"
            onActiveWindowChange={() => {}}
          />
        </div>,
      );
      const stack = container.querySelector('[data-slot="mode-dock-stack"]') as HTMLElement;
      const stackBody = stack.querySelector('[data-slot="mode-dock-stack-body"]') as HTMLElement;
      expect(stack.querySelectorAll('[data-slot="mode-dock-tab"]')).toHaveLength(2);
      expect(stack.querySelector('[data-slot="mode-dock-tab-cap"]')?.className).toContain("ui-glass");
      expect(stack.querySelector('[data-slot="mode-dock-controls-cap"]')).toBeNull();
      expect(stack.querySelector('[data-slot="mode-dock-tab-gap"]')?.className).not.toContain("ui-glass");
      expect(stackBody.hasAttribute("data-window-silhouette-content")).toBe(true);
      expect(stackBody.style.clipPath).toBe("inset(100%)");
      expect(screen.getByText("Energy Body")).toBeTruthy();
      expect(screen.queryByText("Shape Body")).toBeNull();
      const inactiveTab = stack.querySelector('[data-slot="mode-dock-tab"][data-window-id="shape"]');
      const activeTab = stack.querySelector('[data-slot="mode-dock-tab"][data-window-id="energy"]');
      expect(inactiveTab?.className).not.toContain("border-active-base");
      expect(activeTab?.className).toContain("bg-active-base");
      const inactiveTabButton = inactiveTab?.querySelector<HTMLElement>('[role="tab"]');
      const activeTabButton = activeTab?.querySelector<HTMLElement>('[role="tab"]');
      expect(activeTabButton?.getAttribute("aria-selected")).toBe("true");
      expect(activeTabButton?.getAttribute("aria-controls")).toBe(stack.querySelector('[role="tabpanel"]')?.id);
      expect(stack.querySelector('[role="tabpanel"]')?.getAttribute("aria-labelledby")).toBe(activeTabButton?.id);
      expect(stackBody.getAttribute("data-level")).toBe("base");
      fireEvent.keyDown(activeTabButton!, { key: "ArrowLeft" });
      expect(document.activeElement).toBe(inactiveTabButton);
      fireEvent.keyDown(inactiveTabButton!, { key: "Enter" });
      expect(screen.getByText("Shape Body")).toBeTruthy();
      expect(screen.queryByText("Energy Body")).toBeNull();
      fireEvent.pointerDown(stackBody);
      await waitFor(() => {
        expect(stack.getAttribute("data-active")).toBe("true");
        expect(stack.querySelector('[data-slot="mode-dock-silhouette-border"]')?.getAttribute("data-kind")).toBe("active");
      });
    });

    it("Mode close removes a tab and collapses an emptied stack", () => {
      const { container } = render(
        <div className="h-layout-story w-layout-story-md">
          <Mode
            windows={[
              { id: "solo", title: uiDataLabel("Solo"), iconId: "app-window", children: <div>Solo Body</div> },
              { id: "peer", title: uiDataLabel("Peer"), iconId: "app-window", children: <div>Peer Body</div> },
            ]}
            layout={{
              kind: "row",
              children: [
                { kind: "stack", children: [{ kind: "window", id: "solo" }], activeId: "solo" },
                { kind: "stack", children: [{ kind: "window", id: "peer" }], activeId: "peer" },
              ],
            }}
            activeWindowId="solo"
            onActiveWindowChange={() => {}}
          />
        </div>,
      );
      const soloClose = container.querySelector("[data-stack-path='0'] [data-slot='mode-dock-tab-close']");
      expect(soloClose).toBeTruthy();
      fireEvent.click(soloClose!);
      expect(screen.queryByText("Solo Body")).toBeNull();
      expect(screen.getByText("Peer Body")).toBeTruthy();
    });

    it("modeDockTabsWithInsertPreview inserts a ghost tab at the drop index for that stack", () => {
      const tabs = [
        { id: "a", title: "A", iconId: "app-window" as const },
        { id: "b", title: "B", iconId: "app-window" as const },
      ];
      const ghost = [{ id: "drag", title: "Drag", iconId: "app-window" as const }];
      const row = modeDockTabsWithInsertPreview(tabs, { stackPath: "1", corner: "topLeft", index: 1 }, "1", "topLeft", ghost);
      expect(row.map((tab) => tab.id)).toEqual(["a", "drag", "b"]);
      expect(row[1]?.preview).toBe("ghost");
      expect(modeDockTabsWithInsertPreview(tabs, { stackPath: "2", corner: "topLeft", index: 1 }, "1", "topLeft", ghost).map((tab) => tab.id)).toEqual(["a", "b"]);
      expect(modeDockTabsWithInsertPreview(tabs, { stackPath: "1", corner: "topRight", index: 1 }, "1", "topLeft", ghost).map((tab) => tab.id)).toEqual(["a", "b"]);
    });

    it("modeDockTabsWithInsertPreview inserts ghost tabs for every window in a dragged stack", () => {
      const tabs = [
        { id: "a", title: "A", iconId: "app-window" as const },
        { id: "b", title: "B", iconId: "app-window" as const },
      ];
      const row = modeDockTabsWithInsertPreview(tabs, { stackPath: "1", corner: "topLeft", index: 0 }, "1", "topLeft", [
        { id: "x", title: "X", iconId: "app-window" as const },
        { id: "y", title: "Y", iconId: "app-window" as const },
      ]);
      expect(row.map((tab) => tab.id)).toEqual(["x", "y", "a", "b"]);
      expect(row[0]?.preview).toBe("ghost");
      expect(row[1]?.preview).toBe("ghost");
    });

    it("applyModeDrop merges a dragged stack into another stack tab bar", () => {
      const layout: WindowLayoutNode = {
        kind: "row",
        children: [
          {
            kind: "stack",
            children: [
              { kind: "window", id: "a" },
              { kind: "window", id: "b" },
            ],
            activeId: "a",
          },
          { kind: "stack", children: [{ kind: "window", id: "c" }], activeId: "c" },
        ],
      };
      const drag = {
        dragKind: "stack" as const,
        windowId: "a",
        stackPath: "0",
        tabIndex: -1,
        pointerId: 1,
        ghostLabel: "Stack",
        x: 0,
        y: 0,
      };
      const next = applyModeDrop(layout, drag, { kind: "tab", stackPath: "1", corner: "topLeft", index: 0 });
      const merged = next.kind === "stack" ? next : next.kind === "row" || next.kind === "column" ? next.children.find((child) => child.kind === "stack" && child.children.some((window) => window.id === "c")) : null;
      expect(merged?.kind).toBe("stack");
      if (merged?.kind === "stack") expect(merged.children.map((child) => child.id)).toEqual(["a", "b", "c"]);
    });

    it("computeTabInsertPreview resolves slot geometry at tab boundaries", () => {
      const tabBar = document.createElement("div");
      tabBar.setAttribute("data-slot", "mode-dock-tabbar");
      const tabA = document.createElement("div");
      tabA.setAttribute("data-slot", "mode-dock-tab");
      tabA.getBoundingClientRect = () => ({ left: 0, right: 80, top: 0, bottom: 24, width: 80, height: 24 }) as DOMRect;
      const tabB = document.createElement("div");
      tabB.setAttribute("data-slot", "mode-dock-tab");
      tabB.getBoundingClientRect = () => ({ left: 80, right: 160, top: 0, bottom: 24, width: 80, height: 24 }) as DOMRect;
      tabBar.appendChild(tabA);
      tabBar.appendChild(tabB);
      tabBar.getBoundingClientRect = () => ({ left: 0, right: 160, top: 0, bottom: 24, width: 160, height: 24 }) as DOMRect;
      const between = computeTabInsertPreview(tabBar, 1);
      expect(between?.insertX).toBe(80);
      const end = computeTabInsertPreview(tabBar, 2);
      expect(end?.insertX).toBe(160);
    });

    it("computeModeSplitPreviewInBody covers half the stack body on each side", () => {
      expect(computeModeSplitPreviewInBody(400, 300, "left")).toEqual({ left: 0, top: 0, width: 200, height: 300 });
      expect(computeModeSplitPreviewInBody(400, 300, "right")).toEqual({ left: 200, top: 0, width: 200, height: 300 });
      expect(computeModeSplitPreviewInBody(400, 300, "top")).toEqual({ left: 0, top: 0, width: 400, height: 150 });
      expect(computeModeSplitPreviewInBody(400, 300, "bottom")).toEqual({ left: 0, top: 150, width: 400, height: 150 });
    });

    it("resolveModeSplitSideInBody uses half-panel zones with dominant axis at corners", () => {
      expect(resolveModeSplitSideInBody(50, 100, 200, 200)).toBe("left");
      expect(resolveModeSplitSideInBody(150, 100, 200, 200)).toBe("right");
      expect(resolveModeSplitSideInBody(100, 50, 200, 200)).toBe("top");
      expect(resolveModeSplitSideInBody(100, 150, 200, 200)).toBe("bottom");
      expect(resolveModeSplitSideInBody(40, 40, 200, 200)).toBe("left");
      expect(resolveModeSplitSideInBody(160, 40, 200, 200)).toBe("right");
    });

    it("modeJoinCornerSpecsForSeparator wires perpendicular child splits to corner grabs", () => {
      const layout: WindowLayoutNode = {
        kind: "row",
        children: [
          {
            kind: "column",
            children: [
              { kind: "stack", children: [{ kind: "window", id: "lt" }], activeId: "lt" },
              { kind: "stack", children: [{ kind: "window", id: "lb" }], activeId: "lb" },
            ],
          },
          { kind: "stack", children: [{ kind: "window", id: "r" }], activeId: "r" },
        ],
      };
      const specs = modeJoinCornerSpecsForSeparator("", layout.kind, 1, layout.children[0]!, layout.children[1]!);
      expect(specs.some((spec) => spec.edgeSide === "leading" && spec.crossAxisPath === "0")).toBe(true);
      expect(specs).toHaveLength(1);
    });

    it("modeJoinCornerSpecsForCrossSeparator wires parent splits on inner separators", () => {
      const specs = modeJoinCornerSpecsForCrossSeparator("0", "column", 1, { path: "", kind: "row", panelIndex: 0 });
      expect(specs).toHaveLength(1);
      expect(specs[0]?.edgeSide).toBe("trailing");
      expect(specs[0]?.alongFraction).toBe(1);
      expect(specs[0]?.mainAxisPath).toBe("");
      expect(specs[0]?.crossAxisPath).toBe("0");
      const rightSpecs = modeJoinCornerSpecsForCrossSeparator("1", "column", 1, { path: "", kind: "row", panelIndex: 1 });
      expect(rightSpecs[0]?.edgeSide).toBe("leading");
      expect(rightSpecs[0]?.alongFraction).toBe(0);
    });

    it("resolveJoinCornerPeerCrossAxes includes every aligned perpendicular axis at a plus junction", () => {
      const layout: WindowLayoutNode = {
        kind: "row",
        children: [
          {
            kind: "column",
            children: [
              { kind: "stack", size: 50, children: [{ kind: "window", id: "lt" }], activeId: "lt" },
              { kind: "stack", size: 50, children: [{ kind: "window", id: "lb" }], activeId: "lb" },
            ],
          },
          {
            kind: "column",
            children: [
              { kind: "stack", size: 50, children: [{ kind: "window", id: "rt" }], activeId: "rt" },
              { kind: "stack", size: 50, children: [{ kind: "window", id: "rb" }], activeId: "rb" },
            ],
          },
        ],
      };
      const spec: ResizableJoinCornerSpec = {
        parentKind: "row",
        mainAxisPath: "",
        mainSeparatorIndex: 1,
        crossAxisPath: "1",
        crossSeparatorIndex: 1,
        edgeSide: "trailing",
        alongFraction: 0.5,
      };
      expect(resolveJoinCornerPeerCrossAxes(layout, spec)).toEqual([
        { path: "0", separatorIndex: 1 },
        { path: "1", separatorIndex: 1 },
      ]);
    });

    it("resolveJoinCornerPeerCrossAxes ignores perpendicular joins that no longer touch", () => {
      const layout: WindowLayoutNode = {
        kind: "row",
        children: [
          {
            kind: "column",
            children: [
              { kind: "stack", size: 30, children: [{ kind: "window", id: "lt" }], activeId: "lt" },
              { kind: "stack", size: 70, children: [{ kind: "window", id: "lb" }], activeId: "lb" },
            ],
          },
          {
            kind: "column",
            children: [
              { kind: "stack", size: 50, children: [{ kind: "window", id: "rt" }], activeId: "rt" },
              { kind: "stack", size: 50, children: [{ kind: "window", id: "rb" }], activeId: "rb" },
            ],
          },
        ],
      };
      expect(
        resolveJoinCornerPeerCrossAxes(layout, {
          parentKind: "row",
          mainAxisPath: "",
          mainSeparatorIndex: 1,
          crossAxisPath: "1",
          crossSeparatorIndex: 1,
          edgeSide: "trailing",
          alongFraction: 0.5,
        }),
      ).toEqual([{ path: "1", separatorIndex: 1 }]);
      expect(
        resolveJoinCornerPeerCrossAxes(layout, {
          parentKind: "row",
          mainAxisPath: "",
          mainSeparatorIndex: 1,
          crossAxisPath: "0",
          crossSeparatorIndex: 1,
          edgeSide: "leading",
          alongFraction: 0.3,
        }),
      ).toEqual([{ path: "0", separatorIndex: 1 }]);
      expect(modePerpendicularJoinSeparators(layout.children[0]!)).toEqual([{ index: 1, fraction: 0.3 }]);
      expect(modePerpendicularJoinSeparators(layout.children[1]!)).toEqual([{ index: 1, fraction: 0.5 }]);
    });

    it("applyModeJoinCornerResize updates only touching cross axes when joins are misaligned", () => {
      const layout: WindowLayoutNode = {
        kind: "row",
        children: [
          {
            kind: "column",
            size: 50,
            children: [
              { kind: "stack", size: 30, children: [{ kind: "window", id: "lt" }], activeId: "lt" },
              { kind: "stack", size: 70, children: [{ kind: "window", id: "lb" }], activeId: "lb" },
            ],
          },
          {
            kind: "column",
            size: 50,
            children: [
              { kind: "stack", size: 50, children: [{ kind: "window", id: "rt" }], activeId: "rt" },
              { kind: "stack", size: 50, children: [{ kind: "window", id: "rb" }], activeId: "rb" },
            ],
          },
        ],
      };
      const next = applyModeJoinCornerResize(
        layout,
        {
          parentKind: "row",
          mainAxisPath: "",
          mainSeparatorIndex: 1,
          crossAxisPath: "1",
          crossSeparatorIndex: 1,
          edgeSide: "trailing",
          alongFraction: 0.5,
        },
        20,
        10,
        400,
        300,
      );
      expect(next.kind).toBe("row");
      if (next.kind !== "row") return;
      const left = next.children[0];
      const right = next.children[1];
      expect(left?.kind).toBe("column");
      expect(right?.kind).toBe("column");
      if (left?.kind !== "column" || right?.kind !== "column") return;
      expect(left.children[0]?.size).toBe(30);
      expect(left.children[1]?.size).toBe(70);
      expect(right.children[0]?.size).toBeGreaterThan(50);
      expect(right.children[1]?.size).toBeLessThan(50);
    });

    it("applyModeJoinCornerResize updates both main and cross axis sizes", () => {
      const layout: WindowLayoutNode = {
        kind: "row",
        children: [
          {
            kind: "column",
            size: 50,
            children: [
              { kind: "stack", size: 50, children: [{ kind: "window", id: "lt" }], activeId: "lt" },
              { kind: "stack", size: 50, children: [{ kind: "window", id: "lb" }], activeId: "lb" },
            ],
          },
          {
            kind: "column",
            size: 50,
            children: [
              { kind: "stack", size: 50, children: [{ kind: "window", id: "rt" }], activeId: "rt" },
              { kind: "stack", size: 50, children: [{ kind: "window", id: "rb" }], activeId: "rb" },
            ],
          },
        ],
      };
      const spec: ResizableJoinCornerSpec = {
        parentKind: "row",
        mainAxisPath: "",
        mainSeparatorIndex: 1,
        crossAxisPath: "1",
        crossSeparatorIndex: 1,
        edgeSide: "trailing",
        alongFraction: 0.5,
      };
      const next = applyModeJoinCornerResize(layout, spec, 20, 10, 400, 300);
      expect(next.kind).toBe("row");
      if (next.kind === "row") {
        expect(next.children[0]?.size).toBeGreaterThan(50);
        expect(next.children[1]?.size).toBeLessThan(50);
        for (const column of next.children) {
          expect(column.kind).toBe("column");
          if (column.kind !== "column") continue;
          expect(column.children[0]?.size).toBeGreaterThan(50);
          expect(column.children[1]?.size).toBeLessThan(50);
        }
      }
    });

    it("applyAxisGroupLayoutDelta resizes a live panel pair without changing its total", () => {
      const next = applyAxisGroupLayoutDelta({ "0": 50, "1": 50 }, "", 1, 5);
      expect(next).toEqual({ "0": 55, "1": 45 });
      expect(applyAxisGroupLayoutDelta(next, "", 1, 100)).toEqual({ "0": 92, "1": 8 });
    });

    it("Mode corner pointer drag resizes both mounted panel groups", async () => {
      const onLayoutChange = vi.fn();
      const { container } = render(
        <div className="h-layout-story w-layout-story-md">
          <Mode
            windows={[
              { id: "lt", title: uiDataLabel("Left Top"), iconId: "app-window", children: <div>Left Top</div> },
              { id: "lb", title: uiDataLabel("Left Bottom"), iconId: "app-window", children: <div>Left Bottom</div> },
              { id: "rt", title: uiDataLabel("Right Top"), iconId: "app-window", children: <div>Right Top</div> },
              { id: "rb", title: uiDataLabel("Right Bottom"), iconId: "app-window", children: <div>Right Bottom</div> },
            ]}
            layout={{
              kind: "row",
              children: [
                {
                  kind: "column",
                  children: [
                    { kind: "stack", children: [{ kind: "window", id: "lt" }], activeId: "lt" },
                    { kind: "stack", children: [{ kind: "window", id: "lb" }], activeId: "lb" },
                  ],
                },
                {
                  kind: "column",
                  children: [
                    { kind: "stack", children: [{ kind: "window", id: "rt" }], activeId: "rt" },
                    { kind: "stack", children: [{ kind: "window", id: "rb" }], activeId: "rb" },
                  ],
                },
              ],
            }}
            activeWindowId="lt"
            onActiveWindowChange={() => {}}
            onLayoutChange={onLayoutChange}
          />
        </div>,
      );
      const rootGroup = container.querySelector<HTMLElement>('[id^="mode-axis-root-"]')!;
      const leftGroup = container.querySelector<HTMLElement>('[id^="mode-axis-0-"]')!;
      const rightGroup = container.querySelector<HTMLElement>('[id^="mode-axis-1-"]')!;
      rootGroup.getBoundingClientRect = () => ({ width: 400, height: 300 }) as DOMRect;
      leftGroup.getBoundingClientRect = () => ({ width: 200, height: 300 }) as DOMRect;
      rightGroup.getBoundingClientRect = () => ({ width: 200, height: 300 }) as DOMRect;
      const corner = [...container.querySelectorAll<HTMLElement>('[data-slot="resizable-corner"]')].find((element) => {
        const spec = readResizableJoinCornerSpec(element);
        return spec?.mainAxisPath === "" && spec.crossAxisPath === "1";
      })!;
      fireEvent(corner, new MouseEvent("pointerdown", { bubbles: true, button: 0, clientX: 200, clientY: 150 }));
      fireEvent(window, new MouseEvent("pointermove", { bubbles: true, clientX: 220, clientY: 165 }));
      fireEvent(window, new MouseEvent("pointerup", { bubbles: true, clientX: 220, clientY: 165 }));
      await waitFor(() => expect(onLayoutChange).toHaveBeenCalled());
      const resized = onLayoutChange.mock.lastCall?.[0] as WindowLayoutNode;
      expect(resized.kind).toBe("row");
      if (resized.kind !== "row") return;
      expect(resized.children[0]?.size).toBe(55);
      expect(resized.children[1]?.size).toBe(45);
      for (const column of resized.children) {
        expect(column.kind).toBe("column");
        if (column.kind !== "column") return;
        expect(column.children[0]?.size).toBe(55);
        expect(column.children[1]?.size).toBe(45);
      }
    });
});
}
