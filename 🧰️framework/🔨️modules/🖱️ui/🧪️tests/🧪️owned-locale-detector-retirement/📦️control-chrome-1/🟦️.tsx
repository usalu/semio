import type { ControlIcon, PanelTabNode, UiRibbonParentCategory, UiTranslationKey } from "@semio-tech/ui-react/source";
import * as ui from "@semio-tech/ui-react/source";
import { renderToStaticMarkup } from "react-dom/server";
import type { TestSource, Tests2Vitest, Tests2Dependencies } from "../🧰️harness/🟦️.ts";

export async function registerControlChrome1Tests(vitest: Tests2Vitest, dependencies: Tests2Dependencies, testSource: TestSource): Promise<void> {
  const { borderNormalBottomClass, borderNormalClass, borderNormalTopClass, Button, ButtonGroup, ButtonGroupItem, CheckIcon, Command, CommandItem, CommandList, COMPACT_UI_DRIVER, DEFAULT_UI_DRIVER, FindInViewIcon, Footer, humanizeControlId, humanizeControlSegment, isInternalChromeControlId, LevelProvider, Mode, modeDockTabClassName, Navbar, navbarFillItem, Pane, PaneHost, Panel, PanelChromeTabBar, panelKindFromPanelToggleControlId, PanelRightIcon, panelTabButtonDividerClass, renderToStaticMarkup, resolveControlLabelId, resolveTranslationLabel, resolveWindowSilhouetteBorderKind, RibbonItem, RibbonZone, Search, SearchIcon, Select, SelectContent, SelectItem, SelectTrigger, SelectValue, shellChromeBorderClass, shellChromeFrameLayerClass, singleTreeLeaf, Toggle, ToggleGroup, uiDataLabel, UiDriverProvider, uiI18n, Window } = dependencies;
  const { describe, expect, it } = vitest;

describe("control chrome", () => {

      it("shows inline labels on buttons for the default driver", () => {
        const markup = renderToStaticMarkup(
          <UiDriverProvider driver={DEFAULT_UI_DRIVER}>
            <Button id="settings.driver.compact" icon={<CheckIcon />} />
          </UiDriverProvider>,
        );
        expect(markup).toContain("Compact");
        expect(markup).toContain("aspect-auto");
      });
  
      it("hides inline labels on buttons for the compact driver", () => {
        const markup = renderToStaticMarkup(
          <UiDriverProvider driver={COMPACT_UI_DRIVER}>
            <Button id="settings.driver.compact" icon={<CheckIcon />} />
          </UiDriverProvider>,
        );
        expect(markup).not.toContain(">Compact<");
      });
  
      it("renders panel toggles with icons including display", () => {
        const markup = renderToStaticMarkup(<Toggle id="ui.panelToggle.display" pressed={false} onPressedChange={() => undefined} icon="layout-grid" />);
        expect(markup).toContain('id="ui.panelToggle.display"');
        expect(markup).toContain('data-icon="layout-grid"');
        expect(markup).toContain('width="7"');
        expect(markup).not.toContain("data-missing-icon");
      });
  
      it("PanelChromeTabBar renders only the root row for a nested tree, and a press on a closed host opens it without changing the active path", async () => {
        const { render, screen, fireEvent } = await import("@testing-library/react");
        const StubIcon = (): null => null;
        const tabs: PanelTabNode[] = [
          {
            kind: "branch",
            id: "framework.category.workbench",
            icon: StubIcon,
            name: "Workbench",
            children: [singleTreeLeaf({ id: "framework.panel.artifact", icon: StubIcon, name: "Artifact", tree: { sections: [] } }), singleTreeLeaf({ id: "framework.panel.catalogue", icon: StubIcon, name: "Catalogue", tree: { sections: [] } })],
          },
        ];
        let visible = false;
        let path: readonly string[] = ["framework.category.workbench", "framework.panel.artifact"];
        const { container, rerender } = render(<PanelChromeTabBar anchor="top-left" tabs={tabs} visible={visible} onVisibleChange={(next) => (visible = next)} activeTabPath={path} onActiveTabPathChange={(next) => (path = next)} />);
        expect(container.querySelectorAll('[data-slot="panel-tabs"]').length).toBe(1);
        expect(screen.getByText("Workbench")).toBeTruthy();
        expect(screen.queryByText("Artifact")).toBeNull();
        fireEvent.click(screen.getByText("Workbench"));
        expect(visible).toBe(true);
        expect(path).toEqual(["framework.category.workbench", "framework.panel.artifact"]);
        rerender(<PanelChromeTabBar anchor="top-left" tabs={tabs} visible={visible} onVisibleChange={(next) => (visible = next)} activeTabPath={path} onActiveTabPathChange={(next) => (path = next)} />);
        const placeholder = container.querySelector('[data-slot="panel-chrome-tab-bar"]');
        expect(placeholder?.getAttribute("data-panel-chrome-tab-bar-placeholder")).toBe("true");
        expect(placeholder?.querySelector('[data-slot="panel-tabs"]')).toBeNull();
      });
  
      it("PanelChromeTabBar at middle anchors uses panel level and window-chrome chip cap glass", () => {
        const StubIcon = (): null => null;
        const tabs: PanelTabNode[] = [singleTreeLeaf({ id: "search", icon: StubIcon, name: "Search", tree: { sections: [] } })];
        for (const anchor of ["top-middle", "bottom-middle"] as const) {
          const markup = renderToStaticMarkup(<PanelChromeTabBar anchor={anchor} tabs={tabs} visible={false} />);
          expect(markup).toContain('data-level="panel"');
          expect(markup).toContain('data-slot="window-chrome-chip-cap"');
          expect(markup).toContain("ui-glass");
        }
      });
  
      it("folded PanelChromeTabBar matches folded Panel chip tab button styling", async () => {
        const { render } = await import("@testing-library/react");
        const StubIcon = (): null => null;
        const tabs: PanelTabNode[] = [singleTreeLeaf({ id: "tab-a", icon: StubIcon, name: "Tab A", tree: { sections: [] } })];
        const { container: chromeBarContainer } = render(<PanelChromeTabBar anchor="top-left" tabs={tabs} visible={false} activeTabPath={["tab-a"]} onActiveTabPathChange={() => undefined} />);
        const { container: foldedPanelContainer } = render(<Panel anchor="top-left" visible={false} tabs={tabs} activeTabPath={["tab-a"]} />);
        const chromeBarButton = chromeBarContainer.querySelector('[data-slot="panel-tab-button"]') as HTMLElement;
        const foldedPanelButton = foldedPanelContainer.querySelector('[data-slot="panel-tab-button"]') as HTMLElement;
        expect(chromeBarButton.className).toBe(foldedPanelButton.className);
        expect(chromeBarContainer.querySelector('[data-slot="window-chrome-chip-cap"]')).toBeTruthy();
        expect(foldedPanelContainer.querySelector('[data-slot="window-chrome-chip-cap"]')).toBeTruthy();
        expect(chromeBarContainer.querySelector('[data-slot="window-chrome-silhouette-border"]')).toBeTruthy();
        expect(foldedPanelContainer.querySelector('[data-slot="window-chrome-silhouette-border"]')).toBeTruthy();
      });
  
      it("button and toggle groups share chrome control group glass styling", () => {
        const buttonMarkup = renderToStaticMarkup(
          <ButtonGroup>
            <Button id="test.button" icon="x" />
          </ButtonGroup>,
        );
        const toggleMarkup = renderToStaticMarkup(<Toggle id="ui.search.toggle" pressed={false} onPressedChange={() => undefined} icon="search" text="Search" />);
        expect(buttonMarkup).toContain("ui-glass");
        expect(toggleMarkup).toContain("ui-glass");
      });
  
      it("PanelChromeTabBar does not celebrate an unrelated tab press outside introduction completion", async () => {
        const { render, screen, fireEvent } = await import("@testing-library/react");
        const StubIcon = (): null => null;
        const tabs: PanelTabNode[] = [singleTreeLeaf({ id: "framework.panel.catalogue", icon: StubIcon, name: "Katalog", tree: { sections: [] } })];
        let visible = false;
        const { container } = render(<PanelChromeTabBar anchor="top-left" tabs={tabs} visible={visible} onVisibleChange={(next) => (visible = next)} activeTabPath={["framework.panel.catalogue"]} onActiveTabPathChange={() => undefined} />);
        fireEvent.click(screen.getByText("Katalog"));
        expect(visible).toBe(true);
        expect(container.querySelector('[id="framework.panel.catalogue"]')?.getAttribute("data-celebrated")).toBeNull();
      });
  
      it("Toggle does not celebrate on an ordinary press outside introduction completion", async () => {
        const { render, fireEvent } = await import("@testing-library/react");
        const { container } = render(<Toggle id="ui.panelToggle.display" pressed={false} onPressedChange={() => undefined} icon="layout-grid" text="Display" />);
        const item = container.querySelector('[data-slot="toggle-group-item"]');
        expect(item).toBeTruthy();
        fireEvent.click(item!);
        expect(item?.getAttribute("data-celebrated")).toBeNull();
      });
  
      it("exposes navbar fill helper for trailing chrome alignment", () => {
        const markup = renderToStaticMarkup(<Navbar items={[navbarFillItem(), { key: "trailing", content: <span>Trailing</span> }]} />);
        expect(markup).toContain("flex-1 min-w-0");
        expect(markup).toContain("Trailing");
      });
  
      it("renders fullscreen toggle on the trailing navbar edge by default", () => {
        const markup = renderToStaticMarkup(<Navbar items={[{ key: "title", content: <span>App</span> }]} />);
        expect(markup).toContain('data-slot="navbar-trailing-chrome"');
        expect(markup).toContain('id="ui.fullscreen.toggle"');
      });
  
      it("maps fullscreen toggle id to ui i18n key", () => {
        expect(resolveControlLabelId("ui.fullscreen.toggle")).toBe("ui.fullscreen.toggle");
      });
  
      it("uses the active foreground on pressed navbar toggles", () => {
        const markup = renderToStaticMarkup(<Toggle id="ui.panelToggle.display" pressed={true} onPressedChange={() => undefined} icon="layout-grid" text="Display" />);
        expect(markup).toContain('data-state="on"');
        expect(markup).toContain("data-[state=on]:bg-active-base");
        expect(markup).toContain("data-[state=on]:border-active-base");
        expect(markup).toContain("data-[state=on]:text-active-foreground");
        expect(markup).not.toContain("data-[state=on]:bg-hover-interactive-fill");
      });
  
      it("surfaces missing icons at runtime when control icon is absent", () => {
        const markup = renderToStaticMarkup(
          <ToggleGroup
            items={[
              {
                value: "on",
                id: "ui.panelToggle.display",
                icon: undefined as unknown as ControlIcon,
              },
            ]}
          />,
        );
        expect(markup).toContain("data-missing-icon");
      });
  
      it("maps ui shell ids to domain-neutral ui i18n keys by default", () => {
        expect(resolveControlLabelId("ui.nav.back")).toBe("ui.nav.back");
        expect(resolveControlLabelId("ui.panelToggle.workbench")).toBe("ui.panelToggle.workbench");
        expect(resolveControlLabelId("playground.panel.details")).toBe("ui.panelToggle.details");
        expect(panelKindFromPanelToggleControlId("playground.panel.workbench")).toBe("workbench");
      });
  
      it("resolves ribbon collection ids in en and de", () => {
        const categories: readonly UiRibbonParentCategory[] = [
          "history",
          "hand",
          "selection",
          "lasso",
          "filter",
          "open",
          "save",
          "transfer",
          "transform",
          "create",
          "view",
          "actions",
          "settings",
          "methods",
          "mode",
          "targets",
          "export",
          "utilities",
          "sync",
        ];
        for (const locale of ["en", "de"] as const) {
          void uiI18n.changeLanguage(locale);
          for (const category of categories) {
            const key = `ui.ribbon.parent.${category}` as UiTranslationKey;
            const label = resolveTranslationLabel(uiI18n.t(key as UiTranslationKey));
            expect(label, `${locale}:${key}`).toBeTruthy();
            expect(label).not.toBe(key);
          }
          expect(resolveControlLabelId(`ui.ribbon.group.${categories[0]}`)).toBe(`ui.ribbon.parent.${categories[0]}`);
          expect(resolveControlLabelId(`ui.ribbon.example-window.group.${categories[0]}`)).toBe(`ui.ribbon.parent.${categories[0]}`);
        }
        void uiI18n.changeLanguage("en");
      });
  
      it("renders navbar navigation buttons with inline labels when compact is off", () => {
        const markup = renderToStaticMarkup(
          <UiDriverProvider driver={DEFAULT_UI_DRIVER}>
            <ButtonGroup id="ui.nav.back">
              <ButtonGroupItem id="ui.nav.back" icon="arrow-left" />
            </ButtonGroup>
          </UiDriverProvider>,
        );
        expect(markup).toContain("Go back");
        expect(markup).toContain("aspect-auto");
      });
  
      it("threads ButtonGroup level into ButtonGroupItem hover tokens", () => {
        const markup = renderToStaticMarkup(
          <LevelProvider level="window">
            <ButtonGroup>
              <ButtonGroupItem id="ui.engagement.actions" text="Next" icon="arrow-right" />
            </ButtonGroup>
          </LevelProvider>,
        );
        expect(markup).toContain("hover:bg-hover-interactive-fill");
        expect(markup).not.toContain("hover:bg-hover-base");
        expect(markup).not.toContain("hover:bg-hover-window");
      });
  
      it("renders command menu rows with hover feedback", () => {
        const commandMarkup = renderToStaticMarkup(
          <Command>
            <CommandList>
              <CommandItem value="alpha">Alpha</CommandItem>
            </CommandList>
          </Command>,
        );
        expect(commandMarkup).toContain("hover:bg-hover-interactive-fill");
      });
  
      it("renders select items with hover feedback", async () => {
        const { render } = await import("@testing-library/react");
        render(
          <Select id="compute-mode.select" open defaultValue="fast">
            <SelectTrigger id="compute-mode">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="fast">Fast</SelectItem>
            </SelectContent>
          </Select>,
        );
        const item = document.querySelector('[data-slot="select-item"]');
        expect(item?.className).toContain("hover:bg-hover-interactive-fill");
      });
  
      it("renders select menus with popover surface tokens", async () => {
        const { render } = await import("@testing-library/react");
        render(
          <Select id="compute-mode.select" open defaultValue="fast">
            <SelectTrigger id="compute-mode">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="fast">Fast</SelectItem>
            </SelectContent>
          </Select>,
        );
        const content = document.querySelector('[data-slot="select-content"]');
        expect(content?.getAttribute("data-level")).toBe("menu");
        expect(content?.className).toContain("z-menu");
        expect(content?.className).toContain("ui-glass");
        expect(content?.className).toContain("text-popover-foreground");
        expect(content?.className).not.toContain("bg-transparent");
      });
  
      it("renders toggles with inline labels from the toggle group id when compact is off", () => {
        const markup = renderToStaticMarkup(
          <UiDriverProvider driver={DEFAULT_UI_DRIVER}>
            <Toggle id="ui.search.toggle" pressed={false} onPressedChange={() => undefined} icon={<SearchIcon className="size-small" />} />
          </UiDriverProvider>,
        );
        expect(markup).toContain("Search");
        expect(markup).toContain("aspect-auto");
      });
  
      it("renders search and find toggles with distinct labels", () => {
        const markup = renderToStaticMarkup(
          <UiDriverProvider driver={DEFAULT_UI_DRIVER}>
            <Toggle id="ui.search.toggle" pressed={false} onPressedChange={() => undefined} icon={<SearchIcon className="size-small" />} />
            <Toggle id="ui.find.toggle" pressed={false} onPressedChange={() => undefined} icon={<FindInViewIcon className="size-small" />} />
          </UiDriverProvider>,
        );
        expect(markup).toContain("Search");
        expect(markup).toContain("Find");
        expect(resolveControlLabelId("ui.search.toggle")).not.toBe(resolveControlLabelId("ui.find.toggle"));
      });
  
      it("humanizes unknown control ids when no i18n entry exists", () => {
        expect(humanizeControlId("ui.panelToggle.details")).toBe("Details");
        expect(humanizeControlSegment("puzzle2dGridSnap")).toBe("Puzzle2d Grid Snap");
      });
  
      it("maps internal engagement/search control ids to their ui.* i18n keys", () => {
        expect(isInternalChromeControlId("search-possibles-toggle")).toBe(true);
        expect(resolveControlLabelId("ui.windowSearch.suggestions")).toBe("ui.windowSearch.suggestions");
        expect(resolveControlLabelId("ui.engagement.actions")).toBe("ui.engagement.actions");
        expect(resolveControlLabelId("search-input")).toBe("ui.windowSearch.action");
      });
  
      it("uses normal shell edges on panel/pane frame, navbar bottom, and footer top with CSS hover emphasis", () => {
        expect(shellChromeBorderClass).toContain("border-normal");
        expect(shellChromeBorderClass).not.toContain("border-emphasized");
        expect(shellChromeFrameLayerClass).not.toContain("border-normal");
        expect(shellChromeFrameLayerClass).not.toContain("border-emphasized");
        const navbarMarkup = renderToStaticMarkup(<Navbar items={[{ key: "a", content: "Nav" }]} />);
        expect(navbarMarkup).toContain('data-slot="navbar"');
        expect(navbarMarkup).not.toContain(borderNormalBottomClass);
        expect(navbarMarkup).not.toContain("border-emphasized");
        expect(navbarMarkup).not.toContain("border-border");
        const footerMarkup = renderToStaticMarkup(<Footer items={[{ key: "a", content: "Footer" }]} />);
        expect(footerMarkup).toContain('data-slot="footer"');
        expect(footerMarkup).not.toContain(borderNormalTopClass);
        expect(footerMarkup).not.toContain("border-emphasized");
        const ribbonMarkup = renderToStaticMarkup(
          <RibbonZone>
            <RibbonItem>
              <ToggleGroup kind="single" value="tool" items={[{ value: "tool", id: "ui.ribbon.group.tool", icon: "save", text: "Tool" }]} />
            </RibbonItem>
          </RibbonZone>,
        );
        expect(ribbonMarkup).toMatch(/\bborder\b/);
        expect(ribbonMarkup).toContain(borderNormalClass);
        expect(ribbonMarkup).not.toContain("border-emphasized");
        const panelMarkup = renderToStaticMarkup(<Panel anchor="top-left" visible tabs={[singleTreeLeaf({ id: "tab-a", icon: PanelRightIcon, name: "Tab A", tree: { sections: [] } })]} />);
        expect(panelMarkup).toContain('data-slot="panel"');
        expect(panelMarkup).toContain('data-slot="window-chrome-silhouette-border"');
        expect(panelMarkup).toContain('data-slot="window-chrome-cap"');
        expect(panelMarkup).not.toContain("border-emphasized");
        expect(panelMarkup).not.toContain("border-b-current");
        expect(panelMarkup).not.toMatch(/panel-tabs[^>]*border-emphasized/);
        expect(panelMarkup).toMatch(/panel-tabs[^>]*z-40/);
        const paneMarkup = renderToStaticMarkup(
          <PaneHost>
            <Pane id="hover-pane" anchor="top-left" icon="box" label={uiDataLabel("Pane")} folded={false}>
              <div>Content</div>
            </Pane>
          </PaneHost>,
        );
        expect(paneMarkup).toContain('data-slot="pane"');
        expect(paneMarkup).toContain('data-slot="window-chrome-silhouette-border"');
        expect(paneMarkup).toContain('data-slot="window-chrome-cap"');
        expect(paneMarkup).not.toContain("border-emphasized");
        const measuresMarkup = renderToStaticMarkup(
          <Window id="measures-markup-window" measures={<div>LOD</div>} measuresFolded={false}>
            <div>Body</div>
          </Window>,
        );
        expect(measuresMarkup).toContain('data-slot="window-chrome-silhouette-border"');
        expect(measuresMarkup).toContain('data-slot="window-chrome-gap"');
        expect(measuresMarkup).not.toContain("border-emphasized");
      });
  
      it("keeps inactive window chrome neutral while controls emphasize only on direct hover", async () => {
        const { readFileSync } = await import("node:fs");
        const { fileURLToPath } = await import("node:url");
        const { dirname, resolve } = await import("node:path");
        const css = readFileSync(resolve(dirname(fileURLToPath(testSource.url)), "../../🎨️styling/🖌️ui/🎨️.css"), "utf8");
        expect(css).toContain('[data-slot="navbar"]::after');
        expect(css).toContain('[data-slot="footer"]::before');
        expect(css).toMatch(/\[data-slot="navbar"\]:hover::after/);
        expect(css).toMatch(/\[data-slot="footer"\]:hover::before/);
        expect(css).not.toMatch(/\[data-slot="navbar"\]:focus-within::after/);
        expect(css).not.toMatch(/\[data-slot="footer"\]:focus-within::before/);
        expect(css).toContain("background-color: var(--border-emphasized-color)");
        expect(css).toMatch(/\[data-window-silhouette\]:hover > \[data-window-silhouette-border\]\[data-kind="normal"\]:not\(\[data-pending\]\) path/);
        expect(css).not.toMatch(/\[data-window-silhouette\]:hover \[data-window-silhouette-border\]\[data-kind="normal"\]:not\(\[data-pending\]\) path/);
        expect(css).toMatch(/:is\(\[data-slot="panel"\],\s*\[data-slot="pane"\]\):hover\s*\[data-slot="chrome-frame"\]/);
        expect(css).not.toMatch(/:is\(\[data-slot="panel"\],\s*\[data-slot="pane"\]\):focus-within\s*\[data-slot="chrome-frame"\]/);
        expect(css).toContain("@utility bg-hover-interactive-fill");
        expect(css).toContain("--hover-ink: var(--color-emphasized)");
        expect(css).toContain("--muted-foreground: var(--color-emphasized)");
        expect(css).toContain("color: var(--hover-ink, var(--border-element-color))");
        expect(css).toContain('[data-hover-scope]:hover [data-slot="drag-handle"]');
        expect(css).toContain('[data-hover-scope]:hover :is([data-slot="mode-dock-tab-focus"], [data-slot="mode-dock-tab-close"])');
        expect(css).toMatch(/\[data-hover-scope\]:hover\s*\[data-slot="drag-handle"\]\s*,\s*\n\[data-hover-scope\]:hover\s*:is\(\[data-slot="mode-dock-tab-focus"\]/);
        expect(css).toContain("/* #region 🫨️IconAnimChromeAccessory");
        expect(css).toMatch(
          /\[role="combobox"\]\):hover\s*:is\(\s*\[data-slot="drag-handle"\][\s\S]*?\):not\(:hover\)\s*:where\(\[data-icon\], \[data-icon-kind\]\)\s*\{\s*animation: none !important;/,
        );
        expect(css).toMatch(
          /\[role="combobox"\]\):hover\s*:is\(\s*\[data-slot="drag-handle"\][\s\S]*?\):not\(:hover\)\s*:where\(\[data-icon\], \[data-icon-kind\]\)\s*svg > \*\s*\{\s*animation: none !important;/,
        );
        expect(css).not.toContain(':has([data-slot="mode-dock-stack-body"]:hover)');
        expect(css).not.toContain(') [data-slot="mode-dock-tab"][data-stack-active="true"]:not([data-handle-hovered="true"])');
        expect(css).not.toContain(') [data-slot="mode-dock-tab"][data-stack-active="true"] [data-slot="drag-handle"]');
        expect(css).not.toContain(') [data-slot$="-tab-button"][data-active="true"]:not([data-handle-hovered="true"])');
        expect(css).not.toContain(') [data-slot$="-tab-button"][data-active="true"] [data-slot="drag-handle"]');
        expect(css).not.toContain(') [data-slot="window-pane-chrome-toggle"]:not([data-handle-hovered="true"])');
        expect(css).not.toContain(') [data-slot="window-pane-chrome-toggle"] [data-slot="drag-handle"]');
        expect(modeDockTabClassName).toContain("hover:not-data-[handle-hovered=true]:text-emphasized");
      });
  
      it("panel and pane silhouettes stay normal until the surface receives focus", async () => {
        const { fireEvent, render, waitFor } = await import("@testing-library/react");
        const tabs = [singleTreeLeaf({ id: "tab-a", icon: PanelRightIcon, name: "Tab A", tree: { sections: [] } }), singleTreeLeaf({ id: "tab-b", icon: PanelRightIcon, name: "Tab B", tree: { sections: [] } })];
        const { container: panelContainer } = render(<Panel anchor="top-left" visible onVisibleChange={() => undefined} tabs={tabs} activeTabPath={["tab-a"]} />);
        const panelStack = panelContainer.querySelector('[data-slot="panel"] [data-slot="window-chrome-stack"]') as HTMLElement;
        expect(panelStack.querySelector('[data-slot="window-chrome-silhouette-border"]')?.getAttribute("data-kind")).toBe("normal");
        fireEvent.pointerDown(panelStack.querySelector('[data-slot="panel-tab-button"]')!);
        await waitFor(() => {
          expect(panelStack.getAttribute("data-active")).toBe("true");
          expect(panelStack.querySelector('[data-slot="window-chrome-silhouette-border"]')?.getAttribute("data-kind")).toBe("active");
        });

        const { container: paneContainer } = render(
          <PaneHost>
            <Pane id="focus-pane" anchor="top-right" icon="box" label={uiDataLabel("Pane")} folded={false} onFoldToggle={() => undefined}>
              <div data-slot="pane-body">Pane body</div>
            </Pane>
          </PaneHost>,
        );
        const paneStack = paneContainer.querySelector('[data-slot="pane"] [data-slot="window-chrome-stack"]') as HTMLElement;
        expect(paneStack.querySelector('[data-slot="window-chrome-silhouette-border"]')?.getAttribute("data-kind")).toBe("normal");
        fireEvent.pointerDown(paneStack.querySelector('[data-slot="pane-body"]')!);
        await waitFor(() => {
          expect(paneStack.getAttribute("data-active")).toBe("true");
          expect(paneStack.querySelector('[data-slot="window-chrome-silhouette-border"]')?.getAttribute("data-kind")).toBe("active");
        });
      });
  
      it("activating a surface clears its introduced stamps so the active stroke can win", async () => {
        const { fireEvent, render, waitFor } = await import("@testing-library/react");
        const { container } = render(
          <div className="h-layout-story w-layout-story-md">
            <Mode
              windows={[{ id: "main", title: uiDataLabel("Main"), iconId: "app-window", children: <div id="framework.window.main">Main Body</div> }]}
              layout={{ kind: "stack", children: [{ kind: "window", id: "main" }], activeId: "main" }}
              activeWindowId="main"
              onActiveWindowChange={() => {}}
            />
          </div>,
        );
        const stack = container.querySelector('[data-slot="mode-dock-stack"]') as HTMLElement;
        const scroll = container.querySelector("#framework\\.window\\.main") as HTMLElement;
        scroll.setAttribute("data-introduced", "true");
        expect(resolveWindowSilhouetteBorderKind(stack.querySelector('[data-slot="window"]'))).toBe("introduced");
        stack.setAttribute("data-silhouette-remeasure", "introduced");
        await waitFor(() => {
          expect(stack.querySelector('[data-slot="mode-dock-silhouette-border"]')?.getAttribute("data-kind")).toBe("introduced");
        });
        fireEvent.pointerDown(stack.querySelector('[data-slot="mode-dock-stack-body"]')!);
        await waitFor(() => {
          expect(scroll.getAttribute("data-introduced")).toBeNull();
          expect(stack.getAttribute("data-active")).toBe("true");
          expect(stack.querySelector('[data-slot="mode-dock-silhouette-border"]')?.getAttribute("data-kind")).toBe("active");
        });
      });
  
      it("panel window-variant tabs use mode-dock pill chrome, normal toggle dividers, and a U-gap", () => {
        const tabs = [singleTreeLeaf({ id: "tab-a", icon: PanelRightIcon, name: "Tab A", tree: { sections: [] } }), singleTreeLeaf({ id: "tab-b", icon: PanelRightIcon, name: "Tab B", tree: { sections: [] } })];
        const markup = renderToStaticMarkup(<Panel anchor="top-left" visible onVisibleChange={() => {}} tabs={tabs} activeTabPath={["tab-a"]} />);
        expect(markup).toContain("bg-active-base");
        expect(markup).toContain("border-0");
        expect(markup).toContain("data-window-silhouette");
        expect(markup).toContain('data-slot="window-chrome-gap"');
        expect(markup).toContain(panelTabButtonDividerClass);
        expect(markup).toContain("ui-glass");
        expect(markup).not.toContain("ui-glass-chrome");
      });
});
}
