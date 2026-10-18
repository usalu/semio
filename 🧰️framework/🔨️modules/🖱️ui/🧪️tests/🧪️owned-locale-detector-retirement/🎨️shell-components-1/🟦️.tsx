import type { PanelTabNode } from "@semio-tech/ui-react/source";
import * as ui from "@semio-tech/ui-react/source";
import { renderToStaticMarkup } from "react-dom/server";
import type { TestSource, Tests1Vitest, Tests1Dependencies } from "../🧰️harness/🟦️.ts";

export async function registerShellComponents1Tests(vitest: Tests1Vitest, dependencies: Tests1Dependencies, testSource: TestSource): Promise<void> {
  const { App, DEFAULT_UI_DRIVER, Footer, LEVELS, Mode, Navbar, Pane, Panel, PanelChromeTabBar, PanelTabBar, Toggle, Ui, UiDriverProvider, WindowChrome, chromeHostedOpenPanelPositionStyle, publishShellNavbarTrailingEndWidthPx, reactHostPort, renderToStaticMarkup, shellNavbarTrailingEndWidthByRoot, singleTreeLeaf, uiDataLabel, uiSpacingPx } = dependencies;
  type PanelTabNode = any;
  const { describe, expect, it, vi } = vitest;
  const { render, screen, fireEvent } = await import("@testing-library/react");

describe("Shell components", () => {

    it("Ui renders the active app body", () => {
      render(
        <Ui
          apps={[
            { id: "editor", label: uiDataLabel("Editor"), children: <div>Editor Body</div> },
            { id: "dashboard", label: uiDataLabel("Dashboard"), children: <div>Dashboard Body</div> },
          ]}
          activeAppId="dashboard"
          onActiveAppChange={() => {}}
        />,
      );
      expect(screen.getByText("Dashboard Body")).toBeTruthy();
    });

    it("App renders the active mode body", () => {
      render(
        <App
          modes={[
            { id: "edit", label: uiDataLabel("Edit"), children: <div>Edit Mode</div> },
            { id: "review", label: uiDataLabel("Review"), children: <div>Review Mode</div> },
          ]}
          activeModeId="review"
          onActiveModeChange={() => {}}
        />,
      );
      expect(screen.getByText("Review Mode")).toBeTruthy();
    });

    it("PanelTabBar keeps the first tab label fully visible inside the inset strip", () => {
      const StubIcon = (): null => null;
      const { container } = render(
        <PanelTabBar
          variant="panel"
          tabs={[singleTreeLeaf({ id: "framework.panel.artifact", icon: StubIcon, name: "Artifact", tree: { sections: [] } }), singleTreeLeaf({ id: "framework.panel.catalogue", icon: StubIcon, name: "Catalogue", tree: { sections: [] } })]}
          activePath={["framework.panel.artifact"]}
          onActivePathChange={() => {}}
        />,
      );
      expect(screen.getByText("Artifact")).toBeTruthy();
      expect(screen.getByText("Catalogue")).toBeTruthy();
      const tabBar = container.querySelector('[data-slot="panel-tabs"]');
      expect(tabBar?.className).toContain("px-single");
      expect(tabBar?.className).toContain("z-40");
    });

    it("PanelTabBar reveals a second row for a nested branch's children (progressive — no auto-descend), and clicking a sibling category reveals only its own immediate children, unhighlighted", () => {
      const StubIcon = (): null => null;
      const tabs: PanelTabNode[] = [
        {
          kind: "branch",
          id: "framework.category.workbench",
          icon: StubIcon,
          name: "Workbench",
          children: [singleTreeLeaf({ id: "framework.panel.artifact", icon: StubIcon, name: "Artifact", tree: { sections: [] } }), singleTreeLeaf({ id: "framework.panel.catalogue", icon: StubIcon, name: "Catalogue", tree: { sections: [] } })],
        },
        {
          kind: "branch",
          id: "framework.category.display",
          icon: StubIcon,
          name: "Display",
          children: [singleTreeLeaf({ id: "framework.display.windows", icon: StubIcon, name: "Windows", tree: { sections: [] } })],
        },
      ];
      let path: readonly string[] = ["framework.category.workbench", "framework.panel.catalogue"];
      const { container, rerender } = render(<PanelTabBar variant="panel" tabs={tabs} activePath={path} onActivePathChange={(next) => (path = next)} />);
      expect(container.querySelectorAll('[data-slot="ribbon-row"]').length).toBe(2);
      expect(screen.getByText("Workbench")).toBeTruthy();
      expect(screen.getByText("Display")).toBeTruthy();
      expect(screen.getByText("Artifact")).toBeTruthy();
      expect(screen.getByText("Catalogue")).toBeTruthy();
      expect(screen.queryByText("Windows")).toBeNull();

      fireEvent.click(screen.getByText("Display"));
      expect(path).toEqual(["framework.category.display"]);
      rerender(<PanelTabBar variant="panel" tabs={tabs} activePath={path} onActivePathChange={(next) => (path = next)} />);
      expect(screen.getByText("Windows")).toBeTruthy();
      expect(screen.queryByText("Artifact")).toBeNull();
      expect(screen.getByText("Windows").closest('[data-active="true"]')).toBeNull();
    });

    it("Panel keeps only the root tab row in the silhouette cutout and renders deeper rows as full-width lines before the tree", () => {
      const StubIcon = (): null => null;
      const tabs: PanelTabNode[] = [
        {
          kind: "branch",
          id: "framework.category.app",
          icon: StubIcon,
          name: "App",
          children: [singleTreeLeaf({ id: "framework.panel.command", icon: StubIcon, name: "Command", tree: { sections: [] } })],
        },
      ];
      const { container } = render(<Panel anchor="top-left" visible tabs={tabs} activeTabPath={["framework.category.app", "framework.panel.command"]} />);
      const chipCap = container.querySelector('[data-slot="window-chrome-chip-cap"]');
      const body = container.querySelector('[data-slot="panel-content"]');
      const bodyStack = body?.firstElementChild;
      const bodyTabs = bodyStack?.querySelector('[data-slot="panel-tabs"]');

      expect(bodyStack?.getAttribute("data-slot")).toBe("panel-body-stack");
      expect(bodyStack?.className).toContain("min-w-0");
      expect(bodyStack?.className).toContain("w-full");
      expect(chipCap?.querySelectorAll('[data-slot="panel-tabs"]')).toHaveLength(1);
      expect(chipCap?.textContent).toContain("App");
      expect(chipCap?.textContent).not.toContain("Command");
      expect(bodyTabs?.textContent).toContain("Command");
      expect(bodyTabs?.className).toContain("w-full");
      expect(bodyTabs?.closest("[data-window-silhouette-chip]")).toBeNull();
      expect(bodyStack?.firstElementChild?.getAttribute("data-slot")).toBe("ribbon");
      expect(bodyStack?.lastElementChild?.getAttribute("data-slot")).toBe("scroll-area");
      expect(bodyStack?.querySelector('[data-slot="scroll-area-viewport"]')?.className).toContain("w-full");
    });

    it("Panel marks the active tab with hover fill and emphasized icon above the emphasized frame", () => {
      const StubIcon = (): null => null;
      const tabs: PanelTabNode[] = [singleTreeLeaf({ id: "tab-a", icon: StubIcon, name: "Tab A", tree: { sections: [] } }), singleTreeLeaf({ id: "tab-b", icon: StubIcon, name: "Tab B", tree: { sections: [] } })];
      const { container } = render(<Panel anchor="top-left" visible tabs={tabs} activeTabPath={["tab-b"]} />);
      expect(screen.getByText("Tab A")).toBeTruthy();
      expect(screen.getByText("Tab B")).toBeTruthy();
      const activeButton = container.querySelector('[data-slot="panel-tab-button"][data-active="true"]');
      expect(activeButton?.className).toContain("bg-active-base");
      expect(activeButton?.className).toContain("text-active-foreground");
      expect(container.querySelector('[data-slot="panel-tabs"]')?.className).toContain("overflow-x-auto");
      expect(container.querySelector('[data-slot="panel-tabs"]')?.className).toContain("ui-scrollbar-hidden");
      expect(container.querySelector('[data-slot="panel-tabs"]')?.className).toContain("z-40");
      expect(container.querySelector('[data-slot="panel-tabs"]')?.className?.split(" ")).not.toContain("px-single");
      expect(container.querySelector('[data-slot="panel-tabs"]')?.className?.split(" ")).not.toContain("w-full");
      expect(container.querySelector('[data-slot="window-chrome-gap"]')).toBeTruthy();
      expect(container.querySelector('[data-slot="window-chrome-silhouette-border"]')).toBeTruthy();
    });

    it("Panel announces which tab is open through aria-pressed, not only through its styling data attributes", () => {
      // ♿️ `data-active`/`data-state` are CSS hooks; assistive technology reads neither, so every tab in a
      // bar announced identically and "Fill is armed" / "the Document panel is open" was invisible to a
      // screen reader — and unassertable for any consumer that is not reading class names.
      const StubIcon = (): null => null;
      const tabs: PanelTabNode[] = [singleTreeLeaf({ id: "tab-a", icon: StubIcon, name: "Tab A", tree: { sections: [] } }), singleTreeLeaf({ id: "tab-b", icon: StubIcon, name: "Tab B", tree: { sections: [] } })];
      const { container, rerender } = render(<Panel anchor="top-left" visible tabs={tabs} activeTabPath={["tab-b"]} />);
      const pressed = (id: string): string | null => container.querySelector(`[data-slot="panel-tab-button"][id="${id}"]`)?.getAttribute("aria-pressed") ?? null;
      expect(pressed("tab-b")).toBe("true");
      expect(pressed("tab-a")).toBe("false");
      rerender(<Panel anchor="top-left" visible tabs={tabs} activeTabPath={["tab-a"]} />);
      expect(pressed("tab-a")).toBe("true");
      expect(pressed("tab-b")).toBe("false");
    });

    it("Panel only paints the active tab's fill/border while expanded — a folded button group shouldn't claim a tab is active", () => {
      const StubIcon = (): null => null;
      const tabs: PanelTabNode[] = [singleTreeLeaf({ id: "tab-a", icon: StubIcon, name: "Tab A", tree: { sections: [] } }), singleTreeLeaf({ id: "tab-b", icon: StubIcon, name: "Tab B", tree: { sections: [] } })];
      const { container, rerender } = render(<Panel anchor="top-left" visible={false} tabs={tabs} activeTabPath={["tab-a"]} />);
      const foldedActiveButton = container.querySelector('[data-slot="panel-tab-button"][data-active="true"]');
      expect(foldedActiveButton).toBeTruthy();
      expect(foldedActiveButton?.className).not.toContain("bg-active-base");
      rerender(<Panel anchor="top-left" visible tabs={tabs} activeTabPath={["tab-a"]} />);
      const visibleActiveButton = container.querySelector('[data-slot="panel-tab-button"][data-active="true"]');
      expect(visibleActiveButton?.className).toContain("bg-active-base");
    });

    it("Panel restores the tab strip's normal content-facing border only while uncollapsed", () => {
      const StubIcon = (): null => null;
      const tabs: PanelTabNode[] = [singleTreeLeaf({ id: "tab-a", icon: StubIcon, name: "Tab A", tree: { sections: [] } })];
      const { container: topContainer, rerender: rerenderTop } = render(<Panel anchor="top-left" visible={false} tabs={tabs} />);
      const foldedTopTabs = topContainer.querySelector('[data-slot="panel-tabs"]');
      expect(foldedTopTabs?.className).not.toContain("border-b");
      expect(foldedTopTabs?.className).not.toContain("border-t");
      rerenderTop(<Panel anchor="top-left" visible tabs={tabs} />);
      const expandedTopTabs = topContainer.querySelector('[data-slot="panel-tabs"]');
      expect(expandedTopTabs?.className).toContain("border-b");
      expect(expandedTopTabs?.className).toContain("!border-normal");
      const chromeMarkup = renderToStaticMarkup(<PanelChromeTabBar anchor="top-middle" tabs={tabs} visible={false} activeTabPath={["tab-a"]} onActiveTabPathChange={() => {}} />);
      expect(chromeMarkup).toContain('data-slot="window-chrome-chip-cap"');
      expect(chromeMarkup).toContain("ui-glass");
      expect(chromeMarkup).not.toContain("rounded-sm");
      const { container: bottomContainer, rerender: rerenderBottom } = render(<Panel anchor="bottom-left" visible={false} tabs={tabs} />);
      const foldedBottomTabs = bottomContainer.querySelector('[data-slot="panel-tabs"]');
      expect(foldedBottomTabs?.className).not.toContain("border-b");
      expect(foldedBottomTabs?.className).not.toContain("border-t");
      rerenderBottom(<Panel anchor="bottom-left" visible tabs={tabs} />);
      const expandedBottomTabs = bottomContainer.querySelector('[data-slot="panel-tabs"]');
      expect(expandedBottomTabs?.className).toContain("border-t");
      expect(expandedBottomTabs?.className).toContain("!border-normal");
    });

    it("Panel keeps its tab button group mounted when folded, but drops content — the tabs are the panel's only chrome", () => {
      const StubIcon = (): null => null;
      const tabs: PanelTabNode[] = [
        singleTreeLeaf({
          id: "tab-a",
          icon: StubIcon,
          name: "Tab A",
          tree: {
            sections: [{ id: "sec", label: "Section", defaultOpen: true, items: [{ id: "leaf", label: "Leaf row" }] }],
          },
        }),
      ];
      const { rerender } = render(<Panel anchor="top-left" visible tabs={tabs} activeTabPath={["tab-a"]} />);
      expect(screen.getByText("Leaf row")).toBeTruthy();
      rerender(<Panel anchor="top-left" visible={false} tabs={tabs} activeTabPath={["tab-a"]} />);
      expect(screen.queryByText("Leaf row")).toBeNull();
      expect(document.querySelector('[data-panel-visible="false"]')).toBeTruthy();
      expect(screen.getByText("Tab A")).toBeTruthy();
    });

    it("Panel's tabs double as the fold toggle: picking a tab opens a folded panel, picking the active tab folds it", () => {
      const StubIcon = (): null => null;
      const tabs: PanelTabNode[] = [singleTreeLeaf({ id: "tab-a", icon: StubIcon, name: "Tab A", tree: { sections: [] } }), singleTreeLeaf({ id: "tab-b", icon: StubIcon, name: "Tab B", tree: { sections: [] } })];
      const onVisibleChange = vi.fn();
      const { container, rerender } = render(<Panel anchor="top-right" visible={false} onVisibleChange={onVisibleChange} tabs={tabs} activeTabPath={["tab-a"]} />);
      const tabAButton = container.querySelector('button[id="tab-a"]') as HTMLElement;
      expect(tabAButton).toBeTruthy();
      fireEvent.click(tabAButton);
      expect(onVisibleChange).toHaveBeenCalledWith(true);

      onVisibleChange.mockClear();
      rerender(<Panel anchor="top-right" visible onVisibleChange={onVisibleChange} tabs={tabs} activeTabPath={["tab-a"]} />);
      fireEvent.click(container.querySelector('button[id="tab-a"]') as HTMLElement);
      expect(onVisibleChange).toHaveBeenCalledWith(false);
    });

    it("bottom-right panel tabs preserve declared visual and keyboard order when a folded toggle opens the panel", () => {
      const StubIcon = (): null => null;
      const tabs: PanelTabNode[] = [
        singleTreeLeaf({ id: "general", icon: StubIcon, name: "General", order: 0, tree: { sections: [] } }),
        singleTreeLeaf({ id: "theme", icon: StubIcon, name: "Theme", order: 1, tree: { sections: [] } }),
        singleTreeLeaf({ id: "history", icon: StubIcon, name: "History", order: 2, tree: { sections: [] } }),
      ];
      const Harness = () => {
        const [visible, setVisible] = reactHostPort.useState(false);
        const [activePath, setActivePath] = reactHostPort.useState<readonly string[]>(["general"]);
        const selection = { tabs, visible, onVisibleChange: setVisible, activeTabPath: activePath, onActiveTabPathChange: setActivePath };
        return (
          <>
            <PanelChromeTabBar anchor="bottom-right" {...selection} />
            <Panel anchor="bottom-right" tabBarHost="chrome" {...selection} />
          </>
        );
      };
      const { container } = render(<Harness />);
      const tabIds = () => [...container.querySelectorAll<HTMLElement>('[data-slot="panel-tab-button"]')].map((tab) => tab.dataset.tabId);
      expect(container.querySelector('[data-slot="panel-tabs"]')?.getAttribute("dir")).toBe("ltr");
      expect(tabIds()).toEqual(["general", "theme", "history"]);

      fireEvent.click(container.querySelector('button[id="general"]')!);

      expect(container.querySelector('[data-slot="panel"]')?.getAttribute("dir")).toBe("rtl");
      expect(container.querySelector('[data-slot="panel-tabs"]')?.getAttribute("dir")).toBe("ltr");
      expect(tabIds()).toEqual(["general", "theme", "history"]);
    });

    it("Panel folds immediately from a root tab while preserving its selected child path", () => {
      const StubIcon = (): null => null;
      const tabs: PanelTabNode[] = [{ kind: "branch", id: "root-a", icon: StubIcon, name: "Root A", children: [singleTreeLeaf({ id: "leaf-a", icon: StubIcon, name: "Leaf A", tree: { sections: [] } })] }];
      const onVisibleChange = vi.fn();
      const onActiveTabPathChange = vi.fn();
      const { container } = render(<Panel anchor="bottom-right" visible onVisibleChange={onVisibleChange} tabs={tabs} activeTabPath={["root-a", "leaf-a"]} onActiveTabPathChange={onActiveTabPathChange} />);

      fireEvent.click(container.querySelector('button[id="root-a"]') as HTMLElement);

      expect(onVisibleChange).toHaveBeenCalledWith(false);
      expect(onActiveTabPathChange).not.toHaveBeenCalled();
    });

    it('Panel with tabBarHost="chrome" renders nothing while closed and hosts the full tab strip on WindowChrome once open', () => {
      const StubIcon = (): null => null;
      const tabs: PanelTabNode[] = [
        {
          kind: "branch",
          id: "root-a",
          icon: StubIcon,
          name: "Root A",
          children: [singleTreeLeaf({ id: "leaf-a", icon: StubIcon, name: "Leaf A", tree: { sections: [] } }), singleTreeLeaf({ id: "leaf-b", icon: StubIcon, name: "Leaf B", tree: { sections: [] } })],
        },
      ];
      const { container, rerender } = render(<Panel anchor="top-left" tabBarHost="chrome" visible={false} tabs={tabs} activeTabPath={["root-a", "leaf-a"]} onVisibleChange={() => undefined} />);
      expect(container.querySelector('[data-slot="panel"]')).toBeNull();

      rerender(<Panel anchor="top-left" tabBarHost="chrome" visible tabs={tabs} activeTabPath={["root-a", "leaf-a"]} onVisibleChange={() => undefined} />);
      const panel = container.querySelector('[data-slot="panel"]') as HTMLElement;
      expect(panel).toBeTruthy();
      expect(panel.getAttribute("data-panel-chrome-hosted")).toBe("true");
      expect(panel.className).toContain("z-panel");
      expect(panel.className).not.toContain("z-navbar");
      expect(panel.style.top).toContain("navbar-height");
      expect(panel.style.top).toContain("panel-header-height");
      expect(panel.style.top).not.toContain("spacing-single");
      expect(container.querySelector('[data-slot="window-chrome-stack"]')).toBeTruthy();
      expect(container.querySelector('[data-slot="window-chrome-cap"]')).toBeTruthy();
      expect(container.querySelector('[data-slot="window-chrome-gap"]')).toBeTruthy();
      expect(screen.getByText("Root A")).toBeTruthy();
      expect(screen.getByText("Leaf A")).toBeTruthy();
      expect(screen.getByText("Leaf B")).toBeTruthy();
    });

    it("chromeHostedOpenPanelPositionStyle pulls top/bottom caps into the shell chrome band and leaves side-middle canvas insets alone", () => {
      const top = chromeHostedOpenPanelPositionStyle("top-left");
      expect(top.top).toBe("calc(-1 * (var(--navbar-height) + var(--panel-header-height)) / 2)");
      expect(top.left).toBe("var(--panel-inset)");
      expect(top.maxHeight).toBe("calc(100% + (var(--navbar-height) + var(--panel-header-height)) / 2)");

      const bottom = chromeHostedOpenPanelPositionStyle("bottom-right");
      expect(bottom.bottom).toBe("calc(-1 * (var(--footer-height) + var(--panel-header-height)) / 2)");
      expect(bottom.right).toBe("var(--panel-inset)");

      const side = chromeHostedOpenPanelPositionStyle("left-middle");
      expect(side.top).toBe("50%");
      expect(side.left).toBe("var(--panel-inset)");
    });

    // 🪜️ Ticket 26/09/02 wave B27 §1. A chrome-hosted panel deliberately unfolds INTO the navbar/footer
    // band, so "does the chrome cover the dock?" is a question about two numbers and one stacking level,
    // both of which this pins from the SAME tokens the CSS is generated from:
    //   • the open panel's cap must land exactly on the band's own centered control row — the slot
    //     `PanelChromeTabBar` empties while the panel is open — so the panel's BODY begins at or below
    //     that row's end and no dock row is ever geometrically under a chrome control;
    //   • the band must stack at the BASE level, below `z-panel`, or it paints over that cap regardless.
    // The navbar carried a `z-navbar` class that `🎨️ui.css` has always overridden with
    // `z-index: var(--z-base) !important`; three waves read the catalogue's folded (zero-box) rows as
    // "the navbar covers them" partly because the source said something the browser never did.
    it("keeps shell chrome at the base stacking level and lands an open chrome-hosted panel's cap exactly on the chrome control row", async () => {
      const { render } = await import("@testing-library/react");
      const { STYLING_METRICS, uiSpacingPx: spacingPx } = await import("@semio-tech/ui-styling");
      const chrome = STYLING_METRICS.chrome;
      expect(chrome.navbarHeightUiSpacing).toBe(chrome.footerHeightUiSpacing);
      const band = chrome.navbarHeightUiSpacing;
      const control = chrome.controlHeightUiSpacing;
      const pad = chrome.paddingStandardUiSpacing;
      const chromeControlRow = [pad, band - pad] as const;
      const overhang = (band + control) / 2;
      const capRow = [band - overhang, band - overhang + control] as const;
      expect(capRow).toEqual(chromeControlRow);
      expect(spacingPx(capRow[1])).toBe(spacingPx(band - pad));
      expect(capRow[1]).toBeLessThanOrEqual(band);

      const navbar = render(<Navbar items={[]} showFullscreenToggle={false} />);
      const nav = navbar.container.querySelector('[data-slot="navbar"]') as HTMLElement;
      expect(nav.className).toContain("z-base");
      expect(nav.className).not.toContain("z-navbar");
      navbar.unmount();

      const footer = render(<Footer items={[]} />);
      const foot = footer.container.querySelector('[data-slot="footer"]') as HTMLElement;
      expect(foot.className).toContain("z-base");
      expect(foot.className).not.toContain("z-navbar");
      footer.unmount();

      const StubIcon = (): null => null;
      const tabs: PanelTabNode[] = [singleTreeLeaf({ id: "kinds", icon: StubIcon, name: "Catalogue", tree: { sections: [] } })];
      const dock = render(<Panel anchor="top-left" tabBarHost="chrome" visible tabs={tabs} activeTabPath={["kinds"]} onVisibleChange={() => undefined} />);
      const panel = dock.container.querySelector('[data-slot="panel"]') as HTMLElement;
      expect(panel.className).toContain("z-panel");
      expect(LEVELS.indexOf("panel")).toBeGreaterThan(LEVELS.indexOf("base"));
      dock.unmount();
    });

    it("keeps the mode dock tab bar band clear of the hosted window's own chrome control row", async () => {
      const { render } = await import("@testing-library/react");
      const { STYLING_METRICS, uiSpacingPx: spacingPx } = await import("@semio-tech/ui-styling");
      const chrome = STYLING_METRICS.chrome;
      const control = chrome.controlHeightUiSpacing;
      const pad = chrome.paddingStandardUiSpacing;
      // 🚧️ The dock's cap row is `min-h-medium`, so its band is [0, control]. The dock body is a
      // silhouette content plane pulled up by exactly that clearance, and its compensating padding is
      // what puts the hosted window back below the band: the window's own top pane row then occupies
      // [clearance + pad, clearance + pad + control]. Drop the padding (the `edgeless`/dead-line opt-out
      // leaking through a nested window) and the same row occupies [pad, pad + control] — inside the tab
      // bar's own band, which is how `mode-dock-tab-*` came to answer `elementFromPoint` at every window
      // pane toggle's centre.
      const dockCapBand = [0, control] as const;
      const clearance = control;
      const chromeRowCleared = [clearance + pad, clearance + pad + control] as const;
      const chromeRowHoisted = [pad, pad + control] as const;
      expect(chromeRowCleared[0]).toBeGreaterThanOrEqual(dockCapBand[1]);
      expect(chromeRowHoisted[0]).toBeLessThan(dockCapBand[1]);
      expect(spacingPx(chromeRowCleared[0])).toBe(spacingPx(clearance) + spacingPx(pad));

      const stacked = render(
        <div className="h-layout-story w-layout-story-md">
          <Mode
            windows={[
              {
                id: "left",
                title: uiDataLabel("Left"),
                iconId: "app-window",
                children: <div data-window-content-layout="edgeless">Left Pane</div>,
                search: { input: { id: "left-engagement", value: "", placeholder: uiDataLabel("brush, fill <n>"), onChange: () => undefined, onSubmit: () => undefined } },
              },
              { id: "right", title: uiDataLabel("Right"), iconId: "app-window", children: <div>Right Pane</div> },
            ]}
            layout={{
              kind: "row",
              children: [
                { kind: "stack", children: [{ kind: "window", id: "left" }], activeId: "left" },
                { kind: "stack", children: [{ kind: "window", id: "right" }], activeId: "right" },
              ],
            }}
            activeWindowId="left"
            onActiveWindowChange={() => {}}
          />
        </div>,
      );
      const stack = stacked.container.querySelector('[data-slot="mode-dock-stack"]') as HTMLElement;
      const capRow = stack.querySelector('[data-slot="mode-dock-tabbar"]') as HTMLElement;
      const plane = stack.querySelector('[data-slot="mode-dock-stack-body"]') as HTMLElement;
      expect(capRow.querySelector('[data-slot="mode-dock-tab"]')).toBeTruthy();
      expect(plane.className).toContain("window-silhouette-content-plane");
      // 🪟️ The plane HOSTS a whole window, which is the case `🖌️ui.css`'s opt-out guard has to exclude —
      // the window's chrome toggles live in that window, not in this plane's own content.
      expect(plane.querySelector('[data-slot="window"]')).toBeTruthy();
      expect(plane.querySelector('[data-slot="window-search-overlay"]')).toBeTruthy();
      expect(capRow.compareDocumentPosition(plane) & Node.DOCUMENT_POSITION_FOLLOWING).toBeTruthy();
      stacked.unmount();
    });

    it("PanelChromeTabBar renders a width placeholder without tab chips while the panel is open", async () => {
      const { render } = await import("@testing-library/react");
      const StubIcon = (): null => null;
      const tabs: PanelTabNode[] = [singleTreeLeaf({ id: "tab-a", icon: StubIcon, name: "Inspector", tree: { sections: [] } })];
      const rectSpy = vi.spyOn(Element.prototype, "getBoundingClientRect").mockImplementation(function (this: Element) {
        if (this.getAttribute("data-slot") === "panel-chrome-tab-bar" && !this.getAttribute("data-panel-chrome-tab-bar-placeholder")) {
          return { width: 128, height: 24, top: 0, left: 0, right: 128, bottom: 24, x: 0, y: 0, toJSON: () => ({}) } as DOMRect;
        }
        return { width: 0, height: 0, top: 0, left: 0, right: 0, bottom: 0, x: 0, y: 0, toJSON: () => ({}) } as DOMRect;
      });
      const { container, rerender } = render(<PanelChromeTabBar anchor="top-right" tabs={tabs} visible={false} activeTabPath={["tab-a"]} onActiveTabPathChange={() => undefined} />);
      rerender(<PanelChromeTabBar anchor="top-right" tabs={tabs} visible={true} activeTabPath={["tab-a"]} onActiveTabPathChange={() => undefined} />);
      const placeholder = container.querySelector('[data-slot="panel-chrome-tab-bar"][data-panel-chrome-tab-bar-placeholder="true"]') as HTMLElement;
      expect(placeholder).toBeTruthy();
      expect(placeholder.querySelector('[data-slot="panel-tabs"]')).toBeNull();
      expect(placeholder.style.width).toBe("128px");
      rectSpy.mockRestore();
    });

    it("open chrome-hosted top-right panel reserves trailing navbar end space on the cap row only when trailing elements exist", async () => {
      const { render } = await import("@testing-library/react");
      const StubIcon = (): null => null;
      const tabs: PanelTabNode[] = [singleTreeLeaf({ id: "tab-a", icon: StubIcon, name: "Inspector", tree: { sections: [] } })];
      publishShellNavbarTrailingEndWidthPx(undefined, 96);
      const { container, rerender } = render(<Panel anchor="top-right" tabBarHost="chrome" visible tabs={tabs} activeTabPath={["tab-a"]} size={360} />);
      const cap = container.querySelector('[data-slot="panel"][data-anchor="top-right"] [data-slot="window-chrome-cap"]') as HTMLElement;
      expect(cap.style.paddingInlineStart).toBe(`${96 + uiSpacingPx(1)}px`);

      publishShellNavbarTrailingEndWidthPx(undefined, 0);
      rerender(<Panel anchor="top-right" tabBarHost="chrome" visible tabs={tabs} activeTabPath={["tab-a"]} size={360} />);
      const capNoReserve = container.querySelector('[data-slot="panel"][data-anchor="top-right"] [data-slot="window-chrome-cap"]') as HTMLElement;
      expect(capNoReserve.style.paddingInlineStart).toBe("");
    });

    it("open chrome-hosted bottom-right panel does not reserve trailing navbar end space", async () => {
      const { render } = await import("@testing-library/react");
      const StubIcon = (): null => null;
      const tabs: PanelTabNode[] = [singleTreeLeaf({ id: "tab-a", icon: StubIcon, name: "History", tree: { sections: [] } })];
      publishShellNavbarTrailingEndWidthPx(undefined, 96);
      const { container } = render(<Panel anchor="bottom-right" tabBarHost="chrome" visible tabs={tabs} activeTabPath={["tab-a"]} size={360} />);
      const cap = container.querySelector('[data-slot="panel"][data-anchor="bottom-right"] [data-slot="window-chrome-cap"]') as HTMLElement;
      expect(cap.style.paddingInlineStart).toBe("");
    });

    it("navbar trailing chrome parks its width so inline labels do not collapse", async () => {
      const { render } = await import("@testing-library/react");
      const rectSpy = vi.spyOn(Element.prototype, "getBoundingClientRect").mockImplementation(function (this: Element) {
        if (this.getAttribute("data-slot") === "navbar-trailing-chrome") {
          return { width: 112, height: 24, top: 0, left: 0, right: 112, bottom: 24, x: 0, y: 0, toJSON: () => ({}) } as DOMRect;
        }
        return { width: 0, height: 0, top: 0, left: 0, right: 0, bottom: 0, x: 0, y: 0, toJSON: () => ({}) } as DOMRect;
      });
      const { container } = render(
        <UiDriverProvider driver={DEFAULT_UI_DRIVER}>
          <Navbar
            items={[]}
            trailingBeforeFullscreen={<Toggle id="ui.panelToggle.chat" pressed={false} onPressedChange={() => undefined} icon="message-square" />}
            showFullscreenToggle
          />
        </UiDriverProvider>,
      );
      const slot = container.querySelector('[data-slot="navbar-trailing-chrome"]') as HTMLElement;
      expect(slot.style.minWidth).toBe("112px");
      expect(container.querySelector("#ui\\.panelToggle\\.chat")).toBeTruthy();
      expect(shellNavbarTrailingEndWidthByRoot.get(document.documentElement)).toBe(112);
      rectSpy.mockRestore();
    });
});
}
