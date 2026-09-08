type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { App, Button, CELEBRATE_STAMP_DURATION_MS, COMPACT_UI_DRIVER, COMPOSE_WINDOW_TEMPLATE_MIME, Canvas, CanvasPickMenu, ContextMenu, ContextMenuController, DEFAULT_GUMBALL_CONFIG, DEFAULT_UI_DRIVER, Engagement, FlowProvider, Footer, GLASS_OVERLAY_BOX_CLASS, GUMBALL_DEFAULT_SHIFT_ROTATION_SNAP, GUMBALL_DEFAULT_SHIFT_SCALE_SNAP, GUMBALL_PLANE_OFFSET, GUMBALL_PLANE_SIZE, GUMBALL_PREVIEW_DISK_RADIUS, GUMBALL_PREVIEW_MIN_EXTENT, GUMBALL_PREVIEW_RING_RADIUS, GUMBALL_RING_RADIUS, ICONS, INTRODUCTION_DEMO_IDLE_THRESHOLD_MS, INTRODUCTION_INFO_BOX_GAP_PX, Icon, Input, LEVELS, Label, Layout, LevelProvider, MODE_CANVAS_INSET_CLASS, Mode, Navbar, NotFound, OrthographicCamera, Pane, PaneHost, Panel, PanelChromeTabBar, PanelDockProvider, PanelTabBar, PerspectiveCamera, Popover, PopoverContent, PopoverTrigger, React, RouteLink, Scrollable, Search, ShellScopeProvider, SortableTreeItems, Surface, THREE, TREE_SECTION_REORDER_MIME, TextSelectionContextMenuHost, Toggle, Tree, TreeContext, TreeItem, UIIntroduction, UI_CHROME_LOCALE_STORAGE_KEY, UI_ELEMENT_REGISTRY, Ui, UiDriverProvider, UiMobileProvider, WINDOW_SILHOUETTE_BORDER_KINDS, WINDOW_SILHOUETTE_GEOMETRY_SCHEMA, WINDOW_SILHOUETTE_PATH_INSET, Window, WindowChrome, WindowMeasureTreeGroup, WindowMeasureTreeLeaf, WindowMeasuresTree, applyAxisGroupLayoutDelta, applyModeDrop, applyModeJoinCornerResize, applySearchSpaceAction, assertUniqueIconConceptAssignments, beginWindowTemplateDrag, beginWindowTemplatePointerDrag, borderNormalClass, buildTextSelectionContextMenuItems, cancelWindowTemplatePointerDrag, celebrateAllElements, celebrateElement, celebrateElements, childElementId, chromeHostedOpenPanelPositionStyle, chromeStatusBorderClass, clampIntroductionInfoBoxPosition, clampSliderValuesToReady, classifyIconSelectorMode, cn, computeModeDropZone, computeModeSplitPreviewInBody, computeTabDockDropZone, computeTabInsertPreview, createDOMEventBinding, createDiagramForceSimulation, createEvenWindowLayout, createMemoryStoragePort, createShellScope, createWindowSilhouetteGeometry, decodeIcon, defaultDiagramForceConfig, detectShellLocale, elementIdSegment, elementIdSelector, encodeIcon, endWindowTemplateDrag, engagementActionTokenEquals, filterSearchPossibles, flowFromAnchor, formatNumber, glassClass, gumballApplyHandleVisualMaterial, gumballAxisRotateAngle, gumballAxisScaleFactor, gumballConfigVisible, gumballEffectiveSnapValue, gumballHandleAllowedByPlane, gumballHandleEnabled, gumballHandleKindToTransformMode, gumballHandleRaycast, gumballHandleVisualState, gumballKindFromRaycastObject, gumballPlaneScaleCorner, gumballPlaneScaleFactors, gumballPointerConsumesCanvasEventRef, gumballPreviewWorldExtent, gumballProjectRayOntoAxis, gumballRayAxisParameter, gumballRayFromNdc, gumballRayPlanePoint, gumballRaycastOwnedAtClientPoint, gumballResolveDragSnaps, gumballResolveHandleVisual, gumballScaleAxisOffset, gumballScalePlaneAxisIndices, gumballSnapScalar, iconShotFrameClass, iconShotFrameStyle, iconSvgMarkup, initUiLocaleSync, insertWindowAsTabAtCorner, insertWindowAtDropZone, installElementsSurfaceBrowserDefaultSuppression, introductionDemoArcPoint, introductionDemoResolveVisual, introductionPointRelativeToHost, introductionRectRelativeToHost, isContextMenuPointerTarget, isElementId, isPointerEventOnDomTextSelection, isSearchSuggestionActionTarget, isUiTypingTarget, isWindowChromeIntroducedTarget, measureWindowSilhouetteMetrics, mergeTreeSectionOrder, modeCollectWindowIds, modeDockChromeGridPlacement, modeDockOutLayout, modeDockTabLabelClassName, modeDockTabsWithInsertPreview, modeJoinCornerSpecsForCrossSeparator, modeJoinCornerSpecsForSeparator, modePerpendicularJoinSeparators, modeStackTabsByCorner, navigateOwnedRoute, ndcToViewportPoint, nearestAnchor, normalizeEngagementActionText, normalizeWindowSilhouetteChips, normalizeWindowSilhouetteMetrics, parseOwnedRouteTarget, parseUiTheme, polylinePointAt, progressPanelTabSelection, publishShellNavbarTrailingEndWidthPx, rankFuzzyItems, reactHostPort, readActiveWindowTemplateDragSession, readDomTextSelection, readResizableJoinCornerSpec, readScrollerContentOverflows, reconcileWindows, referenceMediaKindFromUrl, registerIntroductionSurfaceResolver, removeWindowFromLayout, renderToStaticMarkup, resolveCatalogIconSvg, resolveGumballConfig, resolveGumballVisualPalette, resolveIntroductionPlacement, resolveIntroductionPoint, resolveJoinCornerPeerCrossAxes, resolveModeSplitSideInBody, resolveSliderDraftClear, resolveTranslationLabel, resolveWindowSilhouetteBorderKind, routeWindowSearchEscape, routeWindowSearchSpace, sampleBezierSegments, searchActiveInlineCompletion, searchInlineCompletion, semioTheme, setActiveUiTheme, shellFloorFillClass, shellFloorPaints, shellNavbarTrailingEndWidthByRoot, shortcodeCatalogKey, shortcodeEmoji, shouldActivateSearchPossibleOnConfirm, shouldRouteKeysToWindowSearch, singleTreeLeaf, sliderValuesMatch, splitIntroductionBodyParagraphs, splitWithWindow, sunPositionFromAzimuthElevation, surfaceClass, uiDataLabel, uiI18n, uiSpacingPx, useFirstDraggableElementAlias, useFlow, useIntroductionPointerIdle, useLevel, usePaneSlot, useSurface, windowChromeTitleChipClass, windowMeasuresDefaultWidthPx, windowSilhouetteBorderPaint, windowSilhouetteContains, windowSilhouetteOutline, windowSilhouetteOutlineViolations, windowSilhouettePath, windowTemplatePaletteTreeDragController, windowTemplatePointerDragRef } = dependencies;
  type Anchor = any;
  type Camera = any;
  type CanvasPickTarget = any;
  type Edge = any;
  type FlowNode = any;
  type GumballVec3 = any;
  type IconName = any;
  type IntroductionStepDefinition = any;
  type PanelDock = any;
  type PanelTabNode = any;
  type PanelTabRowDropTarget = any;
  type PanelTreeUnit = any;
  type ResizableJoinCornerSpec = any;
  type SearchSpec = any;
  type TreeDataSection = any;
  type WindowLayoutNode = any;
  type WindowSilhouetteChip = any;
  type WindowSilhouetteEdge = any;
  type WindowSilhouetteMetrics = any;

  const { describe, expect, it, vi } = vitest;
  const { render, screen, fireEvent, waitFor, act } = await import("@testing-library/react");
  const { calculateDiagramLayoutForBatchTest } = await import("../../🧱️elements/🕸️Diagram/📐️layout.ts");

  describe("owned locale detector retirement", () => {
    it("normalizes the closed shell locale domain", () => {
      expect(detectShellLocale("de-AT")).toBe("de");
      expect(detectShellLocale("DE-de")).toBe("de");
      expect(detectShellLocale("fr-FR")).toBe("en");
      expect(detectShellLocale(undefined)).toBe("en");
    });

    it("initializes an explicit shell locale through the owned resolver", async () => {
      const previousStoredLocale = localStorage.getItem(UI_CHROME_LOCALE_STORAGE_KEY);
      const previousDocumentLocale = document.documentElement.lang;
      const previousI18nLocale = detectShellLocale(uiI18n.resolvedLanguage || uiI18n.language);
      try {
        initUiLocaleSync("de");
        expect(localStorage.getItem(UI_CHROME_LOCALE_STORAGE_KEY)).toBe("de");
        expect(document.documentElement.lang).toBe("de");
        await waitFor(() => expect(uiI18n.resolvedLanguage).toBe("de"));
        expect(resolveTranslationLabel(uiI18n.t("ui.nav.back"))).toBe("Zurück");
      } finally {
        if (previousStoredLocale === null) localStorage.removeItem(UI_CHROME_LOCALE_STORAGE_KEY);
        else localStorage.setItem(UI_CHROME_LOCALE_STORAGE_KEY, previousStoredLocale);
        document.documentElement.lang = previousDocumentLocale;
        await uiI18n.changeLanguage(previousI18nLocale);
      }
    });

    it("keeps source and public initialization free of the retired detector", async () => {
      const { readFileSync } = await import("node:fs");
      const { fileURLToPath } = await import("node:url");
      const source = readFileSync(fileURLToPath(source.url), "utf8");
      const retiredPackage = ["i18next", "browser", "languagedetector"].join("-");
      const retiredBinding = ["Language", "Detector"].join("");
      expect(source).not.toContain(retiredPackage);
      expect(source).not.toContain(retiredBinding);
      expect(source).not.toMatch(/\bdetection\s*:/u);
    });
  });

  describe("owned route navigation", () => {
    it("renders NotFound statically and ordinarily without a router provider", () => {
      expect(renderToStaticMarkup(<NotFound title="Missing" parentPath="/spaces" parentLabel="Back" />)).toContain("Missing");
      render(<NotFound title="Missing" parentPath="/spaces" parentLabel="Back" />);
      expect(screen.getByRole("button", { name: "Back" })).toBeTruthy();
    });

    it("preserves path, query, and fragment and emits exactly one popstate", () => {
      history.replaceState(null, "", "/known/start");
      let events = 0;
      const onPopState = () => events++;
      window.addEventListener("popstate", onPopState);
      expect(navigateOwnedRoute(parseOwnedRouteTarget("/spaces/a?tab=history#entry")!)).toEqual({ navigated: true });
      window.removeEventListener("popstate", onPopState);
      expect(location.pathname + location.search + location.hash).toBe("/spaces/a?tab=history#entry");
      expect(events).toBe(1);
    });

    it("routes only primary internal RouteLink clicks", () => {
      const cases: Array<{ props: React.AnchorHTMLAttributes<HTMLAnchorElement>; click?: MouseEventInit }> = [
        { props: { href: "/modified" }, click: { ctrlKey: true } },
        { props: { href: "/download", download: "file" } },
        { props: { href: "/blank", target: "_blank" } },
        { props: { href: "https://example.com/" } },
        { props: { href: "//example.com/path" } },
      ];
      for (const [index, fixture] of cases.entries()) {
        const { unmount } = render(<RouteLink {...fixture.props}>Native {index}</RouteLink>);
        let routePrevented = true;
        const stopNativeNavigation = (event: MouseEvent) => {
          routePrevented = event.defaultPrevented;
          event.preventDefault();
        };
        window.addEventListener("click", stopNativeNavigation, { once: true });
        const event = new MouseEvent("click", { bubbles: true, cancelable: true, button: 0, ...fixture.click });
        screen.getByText(`Native ${index}`).dispatchEvent(event);
        expect(routePrevented).toBe(false);
        unmount();
      }
      const { getByText } = render(<RouteLink href="/owned?tab=one#point">Owned</RouteLink>);
      const event = new MouseEvent("click", { bubbles: true, cancelable: true, button: 0 });
      getByText("Owned").dispatchEvent(event);
      expect(event.defaultPrevented).toBe(true);
      expect(location.pathname + location.search + location.hash).toBe("/owned?tab=one#point");
    });

    it("omits navigation for invalid NotFound parent paths", () => {
      const { rerender } = render(<NotFound title="Missing" parentPath="https://example.com/" parentLabel="Back" />);
      expect(screen.queryByRole("button", { name: "Back" })).toBeNull();
      rerender(<NotFound title="Missing" parentPath="" parentLabel="Back" />);
      expect(screen.queryByRole("button", { name: "Back" })).toBeNull();
      expect(parseOwnedRouteTarget("//example.com/path")).toBeNull();
      expect(parseOwnedRouteTarget("/\\example.com/path")).toBeNull();
      expect(parseOwnedRouteTarget("\\example.com/path")).toBeNull();
      expect(parseOwnedRouteTarget(`/spaces/${String.fromCharCode(0)}child`)).toBeNull();
      expect(parseOwnedRouteTarget("/spaces/\nchild")).toBeNull();
      expect(parseOwnedRouteTarget(" malformed ")).toBeNull();
    });

    it("does not mutate history or publish when pushState rejects", () => {
      history.replaceState(null, "", "/known/rejection-start?stable=1#before");
      let events = 0;
      const onPopState = () => events++;
      window.addEventListener("popstate", onPopState);
      const pushState = vi.spyOn(window.history, "pushState").mockImplementationOnce(() => {
        throw new DOMException("rejected", "SecurityError");
      });
      expect(navigateOwnedRoute(parseOwnedRouteTarget("/rejected?change=1#after")!)).toEqual({ navigated: false });
      pushState.mockRestore();
      window.removeEventListener("popstate", onPopState);
      expect(location.pathname + location.search + location.hash).toBe("/known/rejection-start?stable=1#before");
      expect(events).toBe(0);
    });

    it("keeps the source and public barrel free of the retired router boundary", async () => {
      const { readFileSync } = await import("node:fs");
      const { fileURLToPath } = await import("node:url");
      const source = readFileSync(fileURLToPath(source.url), "utf8");
      const retiredPackage = ["react", "router"].join("-");
      const retiredBindings = [
        ["Browser", "Router"],
        ["Memory", "Router"],
        ["use", "Navigate"],
        ["use", "Location"],
        ["use", "Params"],
        ["use", "Search", "Params"],
      ].map((parts) => parts.join(""));
      expect(source).not.toContain(`from "${retiredPackage}"`);
      expect(retiredBindings.filter((binding) => source.includes(binding))).toEqual([]);
    });
  });

  describe("owned diagram implementations", () => {
    it("lays out a directed graph through the owned structural boundary", () => {
      const nodes: FlowNode[] = [
        { id: "source", position: { x: 0, y: 0 }, data: {} },
        { id: "target", position: { x: 0, y: 0 }, data: {} },
      ];
      const edges: Edge[] = [{ id: "source-target", source: "source", target: "target" }];
      const result = calculateDiagramLayoutForBatchTest(nodes, edges, { direction: "LR", nodeWidth: 40, nodeHeight: 20, rankSep: 30, nodeSep: 10 });
      expect(result.nodes[0]!.position.x).toBeLessThan(result.nodes[1]!.position.x);
      expect(result.nodes.every((node) => Number.isFinite(node.position.x) && Number.isFinite(node.position.y))).toBe(true);
      expect(result.edges).not.toBe(edges);
      expect(result.edges.map(({ id, source, target }) => ({ id, source, target }))).toEqual(edges);
    });

    it("settles force nodes through an owned simulation handle", () => {
      const nodes = [
        { id: "source", x: -100, y: 0 },
        { id: "target", x: 100, y: 0 },
      ];
      const links = [{ id: "source-target", source: "source", target: "target" }];
      const simulation = createDiagramForceSimulation(nodes, links, { ...defaultDiagramForceConfig, enabled: true });
      const ticks = Math.ceil(Math.log(simulation.alphaMin()) / Math.log(1 - simulation.alphaDecay()));
      for (let index = 0; index < ticks; index++) while (!simulation.step({ deadline: performance.now() + 1_000, fuel: 2_048 }).tickComplete) {}
      expect(simulation.nodes()).toBe(nodes);
      expect(nodes.every((node) => Number.isFinite(node.x) && Number.isFinite(node.y))).toBe(true);
      expect(nodes[0]!.x).not.toBe(-100);
      simulation.stop();
    });
  });

  describe("owned fuzzy ranking", () => {
    const items = [
      { label: "Alpha", description: "First letter", category: "Letters" },
      { label: "Bravo", description: "Second letter", category: "Letters" },
      { label: "Chair", description: "A seat", category: "Furniture" },
    ] as const;
    const options = {
      fields: [
        { read: (item: (typeof items)[number]) => item.label, weight: 2 },
        { read: (item: (typeof items)[number]) => item.description, weight: 1 },
        { read: (item: (typeof items)[number]) => item.category, weight: 0.5 },
      ],
      threshold: 0.4,
      limit: 20,
    } as const;

    it("ranks prefixes and transposed typos deterministically", () => {
      expect(rankFuzzyItems(items, "alp", options).map((result) => result.item.label)).toEqual(["Alpha"]);
      expect(rankFuzzyItems(items, "ahlpa", options).map((result) => result.item.label)).toEqual(["Alpha"]);
    });

    it("matches query tokens across weighted fields and preserves source order for empty input", () => {
      expect(rankFuzzyItems(items, "chair furniture", options).map((result) => result.item.label)).toEqual(["Chair"]);
      expect(rankFuzzyItems(items, "", { ...options, limit: 2 }).map((result) => result.item.label)).toEqual(["Alpha", "Bravo"]);
    });
  });

  describe("element id grammar", () => {
    it("isElementId accepts dotted camelCase and rejects everything else", () => {
      expect(isElementId("framework.navbar")).toBe(true);
      expect(isElementId("ui.window.main.action.addLayer")).toBe(true);
      expect(isElementId("brush")).toBe(true);
      expect(isElementId("")).toBe(false);
      expect(isElementId("framework.display.save-label")).toBe(false);
      expect(isElementId("Framework.navbar")).toBe(false);
      expect(isElementId("framework..navbar")).toBe(false);
      expect(isElementId("framework.navbar.")).toBe(false);
    });

    it("elementIdSegment normalizes arbitrary input and is idempotent", () => {
      expect(elementIdSegment("world-orbit-projection")).toBe("worldOrbitProjection");
      expect(elementIdSegment("Some Name")).toBe("someName");
      expect(elementIdSegment("myUtilityId")).toBe("myUtilityId");
      expect(elementIdSegment(elementIdSegment("addLayer"))).toBe(elementIdSegment("addLayer"));
    });

    it("childElementId suffixes and normalizes segments", () => {
      expect(childElementId("ui.chat", "send")).toBe("ui.chat.send");
      expect(childElementId("ui.chat", "message-row")).toBe("ui.chat.messageRow");
      expect(childElementId("ui.tree", "row", 3)).toBe("ui.tree.row.3");
    });
  });

  describe("elementIdSelector", () => {
    it("matches a plain id or an aliased element", () => {
      expect(elementIdSelector("ui.navbar")).toBe('[id="ui.navbar"], [data-element-alias~="ui.navbar"]');
      const { container } = render(
        <div>
          <div id="ui.navbar" />
          <div data-element-alias="framework.panelTab.puzzle.catalogue.firstDraggable other.alias" />
        </div>,
      );
      expect(container.querySelectorAll(elementIdSelector("ui.navbar"))).toHaveLength(1);
      expect(container.querySelectorAll(elementIdSelector("framework.panelTab.puzzle.catalogue.firstDraggable"))).toHaveLength(1);
      expect(container.querySelectorAll(elementIdSelector("other.alias"))).toHaveLength(1);
      expect(container.querySelectorAll(elementIdSelector("nothing.here"))).toHaveLength(0);
    });
  });

  describe("celebrateElements", () => {
    it("stamps every match, auto-clears after durationMs, and cancel un-stamps immediately", () => {
      vi.useFakeTimers();
      try {
        const { container } = render(
          <div>
            <div id="a" />
            <div id="b" />
          </div>,
        );
        celebrateElements("#a, #b", 1000);
        expect(container.querySelector("#a")?.getAttribute("data-celebrated")).toBe("true");
        expect(container.querySelector("#b")?.getAttribute("data-celebrated")).toBe("true");
        act(() => {
          vi.advanceTimersByTime(999);
        });
        expect(container.querySelector("#a")?.getAttribute("data-celebrated")).toBe("true");
        act(() => {
          vi.advanceTimersByTime(1);
        });
        expect(container.querySelector("#a")?.getAttribute("data-celebrated")).toBeNull();
        expect(container.querySelector("#b")?.getAttribute("data-celebrated")).toBeNull();

        const cancel = celebrateElements("#a", 1000);
        expect(container.querySelector("#a")?.getAttribute("data-celebrated")).toBe("true");
        cancel();
        expect(container.querySelector("#a")?.getAttribute("data-celebrated")).toBeNull();
        act(() => {
          vi.advanceTimersByTime(1000);
        });
        expect(container.querySelector("#a")?.getAttribute("data-celebrated")).toBeNull();
      } finally {
        vi.useRealTimers();
      }
    });

    it("celebrateElement stamps a single target and cancel un-stamps immediately", () => {
      vi.useFakeTimers();
      try {
        const { container } = render(<div id="solo" />);
        const el = container.querySelector("#solo")!;
        const cancel = celebrateElement(el, 500);
        expect(el.getAttribute("data-celebrated")).toBe("true");
        cancel();
        expect(el.getAttribute("data-celebrated")).toBeNull();
        act(() => {
          vi.advanceTimersByTime(500);
        });
        expect(el.getAttribute("data-celebrated")).toBeNull();
      } finally {
        vi.useRealTimers();
      }
    });

    it("celebrating by a window-kind alias id stamps every pane of that kind, but celebrating by one pane's own element id stamps only that pane", () => {
      // 🪟️ Mirrors two open panes of the same window kind (e.g. a split-view aggregator viewport): each
      // pane's own `id` is unique (`windowElementId`/`childElementId` of its *instance* id), while both
      // share the kind-level `data-element-alias` (`windowElementId` of the *kind* id). A completed
      // pan/zoom/orbit interaction must celebrate only the pane that performed the gesture — never every
      // aliased pane — so the shell targets the specific pane's own id, not the shared kind alias.
      vi.useFakeTimers();
      try {
        const { container } = render(
          <div>
            <div id="framework.window.puzzle3dMainTop" data-element-alias="framework.window.puzzle3dMain" />
            <div id="framework.window.puzzle3dMainPerspective" data-element-alias="framework.window.puzzle3dMain" />
          </div>,
        );
        const top = container.querySelector("#framework\\.window\\.puzzle3dMainTop")!;
        const perspective = container.querySelector("#framework\\.window\\.puzzle3dMainPerspective")!;

        celebrateElements(elementIdSelector("framework.window.puzzle3dMain"), 1000);
        expect(top.getAttribute("data-celebrated")).toBe("true");
        expect(perspective.getAttribute("data-celebrated")).toBe("true");
        act(() => {
          vi.advanceTimersByTime(1000);
        });

        celebrateElements(elementIdSelector("framework.window.puzzle3dMainTop"), 1000);
        expect(top.getAttribute("data-celebrated")).toBe("true");
        expect(perspective.getAttribute("data-celebrated")).toBeNull();
        act(() => {
          vi.advanceTimersByTime(1000);
        });
      } finally {
        vi.useRealTimers();
      }
    });

    it("celebrateAllElements stamps every valid UI element id and alias, skips introduction chrome and non-grammar ids", () => {
      vi.useFakeTimers();
      try {
        const { container } = render(
          <div>
            <div id="framework.window.main" />
            <div id="framework.panel.catalogue" />
            <div id="ui.introduction.next" />
            <div id="not-a-valid-id" />
            <div data-element-alias="framework.panelTab.puzzle.catalogue.firstDraggable" />
          </div>,
        );
        const cancel = celebrateAllElements(1000);
        expect(container.querySelector('[id="framework.window.main"]')?.getAttribute("data-celebrated")).toBe("true");
        expect(container.querySelector('[id="framework.panel.catalogue"]')?.getAttribute("data-celebrated")).toBe("true");
        expect(container.querySelector('[id="ui.introduction.next"]')?.getAttribute("data-celebrated")).toBeNull();
        expect(container.querySelector('[id="not-a-valid-id"]')?.getAttribute("data-celebrated")).toBeNull();
        expect(container.querySelector('[data-element-alias="framework.panelTab.puzzle.catalogue.firstDraggable"]')?.getAttribute("data-celebrated")).toBe("true");
        cancel();
        expect(container.querySelector('[id="framework.window.main"]')?.getAttribute("data-celebrated")).toBeNull();
        expect(container.querySelector('[data-element-alias="framework.panelTab.puzzle.catalogue.firstDraggable"]')?.getAttribute("data-celebrated")).toBeNull();

        celebrateAllElements(1000);
        expect(container.querySelector('[id="framework.window.main"]')?.getAttribute("data-celebrated")).toBe("true");
        act(() => {
          vi.advanceTimersByTime(1000);
        });
        expect(container.querySelector('[id="framework.window.main"]')?.getAttribute("data-celebrated")).toBeNull();
      } finally {
        vi.useRealTimers();
      }
    });

    it("celebrate content paint shares --celebrate-conic on the host and paints leaf chrome, not window shells", async () => {
      const { readFileSync } = await import("node:fs");
      const { fileURLToPath } = await import("node:url");
      const { dirname, resolve } = await import("node:path");
      const css = readFileSync(resolve(dirname(fileURLToPath(source.url)), "../../../../🎨️styling/🖌️ui.css"), "utf8");
      expect(css).toMatch(/@property --celebrate-border-angle[\s\S]*?inherits:\s*true/);
      expect(css).toMatch(/\[data-celebrated="true"\][\s\S]*?--celebrate-conic:/);
      expect(css).not.toMatch(/\[data-celebrated="true"\][\s\S]*?animation:\s*celebrate-border-spin/);
      expect(css).toMatch(/\[data-celebrated="true"\]::after[\s\S]*?background:\s*var\(--celebrate-conic\)/);
      expect(css).toMatch(/\[data-celebrated="true"\]::after[\s\S]*?padding:\s*var\(--celebrate-border-padding\)/);
      expect(css).not.toMatch(/\[data-celebrated="true"\]::after[\s\S]*?animation:\s*celebrate-border-burst/);
      expect(css).toContain("#endregion 🎉️CelebrateContent");
      expect(css).toMatch(/\[data-celebrated="true"\]:is\([\s\S]*?\[data-slot="button-group-item"\]/);
      expect(css).toMatch(/\[data-celebrated="true"\]\[data-slot="introduction-interaction-label"\][\s\S]*?background-clip:\s*text/);
      expect(css).toMatch(/\.window-silhouette-border-celebrated-fill[\s\S]*?background:\s*var\(--celebrate-conic\)/);
      expect(css).not.toMatch(/\[data-celebrated="true"\]:is\([\s\S]*?\[data-slot="window"\]/);
      const celebrateContent = css.match(/\/\* #region 🎉️CelebrateContent[\s\S]*?\/\* #endregion 🎉️CelebrateContent \*\//)?.[0] ?? "";
      expect(celebrateContent).not.toMatch(/:is\(\[data-icon\], \[data-icon-kind\], \[data-slot="tree-icon"\], \[data-slot="drag-handle"\]\)/);
      expect(celebrateContent).not.toMatch(/mix-blend-mode:\s*destination-in/);
      expect(celebrateContent).not.toMatch(/inset:\s*-100%/);
      expect(celebrateContent).toMatch(/mask-image:\s*var\(--icon-mask, linear-gradient\(#0000 0 0\)/);
      expect(celebrateContent).toMatch(/\[data-icon-kind="themed"\]/);
      expect(celebrateContent).toMatch(/> svg[\s\S]*?visibility:\s*hidden/);
      expect(celebrateContent).toMatch(/:is\(\[data-slot="tree-label"\], \[data-slot="inline-label"\]\)/);
      expect(celebrateContent).toMatch(/-webkit-text-fill-color:\s*transparent/);
      expect(celebrateContent).toMatch(/:is\(\[data-tree-guide-line\], \[data-slot="tree-branch-elbow"\], \[data-slot="tree-branch-stem"\]\)/);
      expect(celebrateContent).toContain('[data-slot="tree-section-content"]');
      expect(celebrateContent).toContain('> [data-slot="tree-guide"] [data-tree-guide-line]');
      expect(celebrateContent).toMatch(/:has\([\s\S]*?\[data-celebrated="true"\]\s*\)/);
      expect(celebrateContent).not.toContain('[data-slot="tree-row-content"]');
    });

    it("border effect phase clocks run on unlayered :root and consumers paint inherited vars only", async () => {
      const { readFileSync } = await import("node:fs");
      const { fileURLToPath } = await import("node:url");
      const { dirname, resolve } = await import("node:path");
      const css = readFileSync(resolve(dirname(fileURLToPath(source.url)), "../../../../🎨️styling/🖌️ui.css"), "utf8");
      const unlayeredRoot = css.match(/^:root \{[\s\S]*?\n\}/m)?.[0] ?? "";
      expect(unlayeredRoot).toMatch(/animation:[\s\S]*?loading-border-spin/);
      expect(unlayeredRoot).toMatch(/animation:[\s\S]*?waiting-border-spin/);
      expect(unlayeredRoot).toMatch(/animation:[\s\S]*?introduced-border-pulse/);
      expect(unlayeredRoot).toMatch(/animation:[\s\S]*?celebrate-border-spin/);
      expect(css).toMatch(/@property --loading-border-angle[\s\S]*?inherits:\s*true/);
      expect(css).toMatch(/@property --waiting-border-angle[\s\S]*?inherits:\s*true/);
      expect(css).toMatch(/@property --introduced-border-width[\s\S]*?inherits:\s*true/);
      expect(css).toMatch(/@property --celebrate-border-padding[\s\S]*?inherits:\s*true/);
      expect(css).toMatch(/@utility border-loading[\s\S]*?opacity:\s*var\(--loading-border-pulse-opacity\)/);
      expect(css).toMatch(/@utility border-waiting[\s\S]*?opacity:\s*var\(--waiting-border-pulse-opacity\)/);
      expect(css).toMatch(/\[data-introduced="true"\][\s\S]*?box-shadow:\s*inset 0 0 0 var\(--introduced-border-width\)/);
      expect(css).not.toMatch(/\[data-introduced="true"\][\s\S]*?animation:\s*introduced-border-pulse/);
      expect(css).toMatch(/\.window-silhouette-border-introduced[\s\S]*?stroke-width:\s*var\(--introduced-border-width\)/);
      expect(css).toMatch(/\.window-silhouette-border-loading[\s\S]*?stroke-dashoffset:\s*var\(--loading-border-dashoffset\)/);
      expect(css).toMatch(/\.window-silhouette-border-celebrated-mask[\s\S]*?stroke-width:\s*var\(--celebrate-border-padding\)/);
      expect(css).not.toMatch(/\.window-silhouette-border-celebrated-fill[\s\S]*?animation:\s*celebrate-border-spin/);
      expect(css).toContain("[data-window-silhouette]");
      expect(css).toContain("[data-window-silhouette-border]");
      expect(css).toContain("[data-window-silhouette-gap]");
      expect(css).toContain("[data-window-silhouette-chip]");
      expect(css).not.toMatch(/\[data-slot="mode-dock-stack"\]\s*\[data-slot="window"\]\[data-introduced="true"\]/);
    });
  });

  describe("useFirstDraggableElementAlias", () => {
    function FirstDraggableHarness({ alias, rows }: { readonly alias: string | null; readonly rows: readonly string[] }) {
      const containerRef = reactHostPort.useRef<HTMLDivElement>(null);
      useFirstDraggableElementAlias(containerRef, alias);
      return (
        <div ref={containerRef}>
          {rows.map((id) => (
            <div key={id} id={id} data-slot="tree-item-row" data-draggable="true" />
          ))}
        </div>
      );
    }

    it("stamps the alias on the first draggable row in document order and moves it as rows change", async () => {
      const { container, rerender } = render(<FirstDraggableHarness alias="framework.panelTab.puzzle.catalogue.firstDraggable" rows={["a", "b"]} />);
      await waitFor(() => {
        expect(container.querySelector("#a")?.getAttribute("data-element-alias")).toBe("framework.panelTab.puzzle.catalogue.firstDraggable");
      });
      expect(container.querySelector("#b")?.getAttribute("data-element-alias")).toBeNull();
      rerender(<FirstDraggableHarness alias="framework.panelTab.puzzle.catalogue.firstDraggable" rows={["b"]} />);
      await waitFor(() => {
        expect(container.querySelector("#b")?.getAttribute("data-element-alias")).toBe("framework.panelTab.puzzle.catalogue.firstDraggable");
      });
    });

    it("cleans up the alias on unmount", async () => {
      const { container, unmount } = render(<FirstDraggableHarness alias="framework.panelTab.puzzle.catalogue.firstDraggable" rows={["a"]} />);
      await waitFor(() => {
        expect(container.querySelector("#a")?.getAttribute("data-element-alias")).toBe("framework.panelTab.puzzle.catalogue.firstDraggable");
      });
      const row = container.querySelector("#a")!;
      unmount();
      expect(row.getAttribute("data-element-alias")).toBeNull();
    });
  });

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
      // 🎓️ Kind-id stamps the inner scroll surface — not `[data-slot="window"]` — matching Aggregator/OS.
      expect(container.querySelector('[id="framework.window.puzzle3dMain"]')?.getAttribute("data-introduced")).toBe("true");
      expect(container.querySelector('[data-slot="window"]')?.getAttribute("data-introduced")).toBeNull();
      expect(resolveWindowSilhouetteBorderKind(container.querySelector('[data-slot="window"]'))).toBe("introduced");
      await waitFor(() => {
        expect(container.querySelector('[data-slot="mode-dock-silhouette-border"]')?.getAttribute("data-kind")).toBe("introduced");
      });
      // 🪟️ Stack owns `z-window` so silhouette `z-[40]` stays below floating Panels; elevation still overrides.
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

  describe("UIIntroduction appearance", () => {
    it("Done on the last step dismisses as completed; Skip dismisses as not completed", async () => {
      const onDismiss = vi.fn();
      const steps: IntroductionStepDefinition[] = [{ id: "only", title: "Only", body: "Last step.", introduce: null, show: [], placement: "center", interactions: [], ordered: false, logos: [], demonstrations: [] }];
      const { unmount } = render(<UIIntroduction introduction={{ title: "Welcome", steps }} stepIndex={0} onStepIndexChange={vi.fn()} onDismiss={onDismiss} />);
      fireEvent.click(screen.getByRole("button", { name: /done|fertig/i }));
      expect(onDismiss).toHaveBeenCalledWith(true);
      unmount();
      onDismiss.mockClear();
      render(<UIIntroduction introduction={{ title: "Welcome", steps }} stepIndex={0} onStepIndexChange={vi.fn()} onDismiss={onDismiss} />);
      fireEvent.click(screen.getByRole("button", { name: /skip|überspringen/i }));
      expect(onDismiss).toHaveBeenCalledWith(false);
    });

    // 🪜️ D2 regression: the title chip, close cell, body, and footer chips of one introduction step
    // are all `data-level="dialog"` and must render the exact same fill without piecewise borders —
    // the WindowChrome silhouette owns their one continuous outline.
    it("renders title, close, body, and footer chips as one borderless dialog-level glass silhouette", () => {
      const steps: IntroductionStepDefinition[] = [
        { id: "first", title: "First", body: "Step one.", introduce: null, show: [], placement: "center", interactions: [], ordered: false, logos: [], demonstrations: [] },
        { id: "second", title: "Second", body: "Step two.", introduce: null, show: [], placement: "center", interactions: [], ordered: false, logos: [], demonstrations: [] },
      ];
      const { container } = render(<UIIntroduction introduction={{ title: "Welcome", steps }} stepIndex={1} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />);
      const stack = container.querySelector('[data-slot="introduction-info-box"]') as HTMLElement;
      expect(stack.getAttribute("data-level")).toBe("dialog");

      const titleChip = container.querySelector('[data-slot="introduction-info-box-chip"]') as HTMLElement;
      const stepChip = container.querySelector('[data-slot="introduction-step-chip"]') as HTMLElement;
      expect(titleChip.className).not.toContain("ui-surface");
      expect(stepChip.className).not.toContain("ui-surface");
      expect(titleChip.className).toContain("border-0");
      expect(stepChip.className).toContain("border-0");

      const chipCap = container.querySelector('[data-slot="window-chrome-chip-cap"]') as HTMLElement;
      const controls = container.querySelector('[data-slot="window-chrome-controls"]') as HTMLElement;
      const body = container.querySelector('[data-slot="window-chrome-body"]') as HTMLElement;
      const footerLeft = container.querySelector('[data-slot="window-chrome-footer-left"]') as HTMLElement;
      const footerCenterChip = container.querySelector('[data-slot="window-chrome-footer-center-chip"]') as HTMLElement;
      const footerRight = container.querySelector('[data-slot="window-chrome-footer-right"]') as HTMLElement;
      for (const cell of [chipCap, controls, footerLeft, footerCenterChip, footerRight]) {
        expect(cell).toBeTruthy();
        expect(cell.className).toContain("ui-glass");
        expect(cell.className).not.toContain("ui-surface");
        expect(cell.className).not.toContain("bg-transparent");
      }
      expect(body.hasAttribute("data-window-silhouette-content")).toBe(true);
      expect(body.className).not.toContain("ui-glass");

      const backButtonGroup = container.querySelector('[id="ui.introduction.back"]')?.closest('[data-slot="button-group"]') as HTMLElement;
      const nextButtonGroup = container.querySelector('[id="ui.introduction.next"]')?.closest('[data-slot="button-group"]') as HTMLElement;
      const closeButton = container.querySelector('[data-slot="introduction-close"]') as HTMLElement;
      for (const flowingChip of [titleChip, stepChip, backButtonGroup, nextButtonGroup, closeButton]) {
        expect(flowingChip).toBeTruthy();
        expect(flowingChip.parentElement?.hasAttribute("data-window-silhouette-chip")).toBe(true);
        expect(flowingChip.getAttribute("data-level") ?? flowingChip.closest("[data-level]")?.getAttribute("data-level")).toBe("dialog");
      }

      const gap = container.querySelector('[data-slot="window-chrome-gap"]') as HTMLElement;
      expect(gap.className).not.toContain("ui-glass");
      expect(gap.className).not.toContain("ui-surface");
    });

    it("stamps an introduced element and clears it on step change", async () => {
      const steps: IntroductionStepDefinition[] = [
        { id: "footer", title: "Footer", body: "This is the footer.", introduce: "ui.footer", show: [], placement: "auto", interactions: [], ordered: false, logos: [], demonstrations: [] },
        { id: "welcome", title: "Welcome", body: "No introduce.", introduce: null, show: [], placement: "center", interactions: [], ordered: false, logos: [], demonstrations: [] },
      ];
      const Harness: React.FC = () => {
        const [stepIndex, setStepIndex] = reactHostPort.useState(0);
        return (
          <div>
            <footer id="ui.footer" />
            <UIIntroduction introduction={{ title: "Welcome", steps }} stepIndex={stepIndex} onStepIndexChange={setStepIndex} onDismiss={vi.fn()} />
          </div>
        );
      };
      const { container } = render(<Harness />);
      const footer = container.querySelector("#ui\\.footer")!;
      await waitFor(() => {
        expect(footer.getAttribute("data-introduced")).toBe("true");
      });
      fireEvent.click(screen.getByRole("button", { name: /next|weiter/i }));
      await waitFor(() => {
        expect(footer.getAttribute("data-introduced")).toBeNull();
      });
    });

    it("unmount clears an introduced stamp", async () => {
      const { container, unmount } = render(
        <div>
          <footer id="ui.footer" />
          <UIIntroduction
            introduction={{
              title: "Welcome",
              steps: [{ id: "footer", title: "Footer", body: "Cutout.", introduce: "ui.footer", show: [], placement: "auto", interactions: [], ordered: false, logos: [], demonstrations: [] }],
            }}
            stepIndex={0}
            onStepIndexChange={vi.fn()}
            onDismiss={vi.fn()}
          />
        </div>,
      );
      const footer = container.querySelector("#ui\\.footer")!;
      await waitFor(() => {
        expect(footer.getAttribute("data-introduced")).toBe("true");
      });
      unmount();
      expect(footer.getAttribute("data-introduced")).toBeNull();
    });

    it("a screen-style step (`introduce: null`) never stamps anything", () => {
      const { container } = render(
        <div>
          <footer id="ui.footer" />
          <UIIntroduction
            introduction={{
              title: "Welcome",
              steps: [{ id: "footer", title: "Footer", body: "Veiled only.", introduce: null, show: [], placement: "auto", interactions: [], ordered: false, logos: [], demonstrations: [] }],
            }}
            stepIndex={0}
            onStepIndexChange={vi.fn()}
            onDismiss={vi.fn()}
          />
        </div>,
      );
      const footer = container.querySelector("#ui\\.footer")!;
      expect(footer.getAttribute("data-introduced")).toBeNull();
    });

    it("uses adaptive foreground tokens for primary and secondary text", () => {
      const { container } = render(
        <UIIntroduction
          introduction={{
            title: "Welcome",
            steps: [
              {
                id: "welcome",
                title: "Welcome",
                body: "Introduction body",
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
      const box = container.querySelector('[data-slot="introduction-info-box"]');
      expect(box?.className).toContain("text-foreground");
      expect(box?.className).not.toMatch(/(?:^|\s)border(?:\s|$)/);
      expect(box?.className).not.toContain("border-emphasized");
      expect(box?.querySelector("p")?.className).toContain("text-muted-foreground");
      expect(box?.querySelector("p")?.className).not.toMatch(/(?:^|\s)text-muted(?:\s|$)/);
    });

    it("pulses the introduction info-box silhouette with the introduced border effect", () => {
      const rect = {
        x: 0,
        y: 0,
        top: 0,
        left: 0,
        bottom: 200,
        right: 320,
        width: 320,
        height: 200,
        toJSON() {
          return this;
        },
      };
      const rectSpy = vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockReturnValue(rect as DOMRect);
      try {
        const { container } = render(
          <UIIntroduction
            introduction={{
              title: "Welcome",
              steps: [
                {
                  id: "welcome",
                  title: "Welcome",
                  body: "Introduction body",
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
        const border = container.querySelector('[data-slot="introduction-info-box"] [data-slot="window-chrome-silhouette-border"]');
        expect(border?.getAttribute("data-kind")).toBe("introduced");
        expect(border?.hasAttribute("data-pending")).toBe(false);
        const path = border?.querySelector("path");
        expect(path).toBeTruthy();
        expect(path?.getAttribute("stroke")).toBe("var(--introduced-border-color, var(--color-secondary))");
        expect(path?.getAttribute("class") || "").toContain("window-silhouette-border-introduced");
      } finally {
        rectSpy.mockRestore();
      }
    });

    it("introduction step activates on click then returns to normal after a background click", async () => {
      const rectSpy = vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockReturnValue({
        x: 0,
        y: 0,
        top: 40,
        left: 40,
        bottom: 200,
        right: 360,
        width: 320,
        height: 160,
        toJSON: () => ({}),
      } as DOMRect);
      try {
        const { container } = render(
          <UIIntroduction
            introduction={{
              title: "Welcome",
              steps: [
                {
                  id: "welcome",
                  title: "Welcome",
                  body: "Introduction body",
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
        const stack = container.querySelector('[data-slot="introduction-info-box"]') as HTMLElement;
        const border = () => stack.querySelector('[data-slot="window-chrome-silhouette-border"]');
        expect(border()?.getAttribute("data-kind")).toBe("introduced");
        fireEvent.pointerDown(stack.querySelector('[data-slot="window-chrome-body"]')!);
        await waitFor(() => {
          expect(stack.getAttribute("data-active")).toBe("true");
          expect(border()?.getAttribute("data-kind")).toBe("active");
        });
        fireEvent.pointerDown(document.body);
        await waitFor(() => {
          expect(stack.getAttribute("data-active")).toBeNull();
          expect(border()?.getAttribute("data-kind")).toBe("normal");
        });
      } finally {
        rectSpy.mockRestore();
      }
    });

    it("introduction and dialog glass boxes emphasize their silhouette only while the pointer is inside", async () => {
      const { readFileSync } = await import("node:fs");
      const { fileURLToPath } = await import("node:url");
      const { dirname, resolve } = await import("node:path");
      const css = readFileSync(resolve(dirname(fileURLToPath(source.url)), "../../../../🎨️styling/🖌️ui.css"), "utf8");
      expect(css).toContain('[data-slot="introduction-info-box"]');
      expect(css).toContain('[data-slot="dialog-box"]');
      expect(css).toContain("data-window-silhouette-border");
      expect(css).toContain("[data-window-silhouette]");
      expect(css).toContain(':not([data-active="true"]):hover');
      expect(css).toContain("stroke: var(--border-emphasized-color)");
      expect(css).toContain("window-silhouette-border-introduced");
      expect(css).toContain("@keyframes introduced-border-pulse");
      expect(css).not.toContain("@keyframes window-silhouette-border-introduced-pulse");
      expect(css).not.toMatch(/\[data-slot="introduction-info-box"\]:focus-within/);
      expect(css).not.toMatch(/\[data-slot="dialog-box"\]:focus-within/);
      expect(GLASS_OVERLAY_BOX_CLASS).not.toMatch(/(?:^|\s)border(?:\s|$)/);
      expect(GLASS_OVERLAY_BOX_CLASS).not.toContain("border-emphasized");
      expect(GLASS_OVERLAY_BOX_CLASS).not.toContain("border-normal");
    });

    it("wraps the introduction silhouette around the step chip and Next when Back is absent", async () => {
      const steps: IntroductionStepDefinition[] = [{ id: "only", title: "Welcome", body: "Step one.", introduce: null, show: [], placement: "center", interactions: [], ordered: false, logos: [], demonstrations: [] }];
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
      const { container } = render(<UIIntroduction introduction={{ title: "Welcome", steps }} stepIndex={0} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />);
      const stack = container.querySelector('[data-slot="introduction-info-box"]') as HTMLElement;
      expect(stack.querySelector('[data-slot="window-chrome-footer-left"]')).toBeNull();
      expect(stack.querySelector('[data-slot="window-chrome-footer-center-chip"]')).toBeTruthy();
      expect(stack.querySelector('[data-slot="window-chrome-footer-right"]')).toBeTruthy();
      mockRect(stack, { left: 0, top: 0, width: 200, height: 100, right: 200, bottom: 100 });
      mockRect(stack.querySelector('[data-slot="window-chrome-chip-cap"]'), { left: 0, right: 60, width: 60, height: 24, bottom: 24 });
      mockRect(stack.querySelector('[data-slot="window-chrome-gap"]'), { left: 60, right: 160, width: 100, height: 24, bottom: 24 });
      mockRect(stack.querySelector('[data-slot="window-chrome-controls"]'), { left: 160, right: 200, width: 40, height: 24, bottom: 24 });
      mockRect(stack.querySelector('[data-slot="window-chrome-cap"]'), { left: 0, height: 24, bottom: 24, width: 200, right: 200 });
      mockRect(stack.querySelector('[data-slot="window-chrome-footer-center-chip"]'), { left: 80, right: 120, width: 40, height: 24, bottom: 100 });
      mockRect(stack.querySelector('[data-slot="window-chrome-footer-right"]'), { left: 140, right: 200, width: 60, height: 24, bottom: 100 });
      stack.setAttribute("data-silhouette-remeasure", "intro-center-right");
      await waitFor(() => {
        const path = stack.querySelector('[data-slot="window-chrome-silhouette-border"] path')?.getAttribute("d");
        expect(path).toBe(
          windowSilhouettePath({
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
                { left: 80, right: 120 },
                { left: 140, right: 200 },
              ],
            },
          }),
        );
        expect(path).toContain("V99 H140 V75 H120 V99 H80 V75");
        expect(path).not.toMatch(/V99 V\d+/);
      });
    });

    it("skips the bottom-right silhouette notch when the introduction next chip is absent", async () => {
      const steps: IntroductionStepDefinition[] = [
        { id: "first", title: "First", body: "Step one.", introduce: null, show: [], placement: "center", interactions: [], ordered: false, logos: [], demonstrations: [] },
        {
          id: "second",
          title: "Second",
          body: "Complete the step.",
          introduce: null,
          show: [],
          placement: "center",
          interactions: [],
          ordered: false,
          logos: [],
          demonstrations: [],
        },
      ];
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
      const { container } = render(<UIIntroduction introduction={{ title: "Welcome", steps }} stepIndex={1} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />);
      const stack = container.querySelector('[data-slot="introduction-info-box"]') as HTMLElement;
      mockRect(stack, { left: 0, top: 0, width: 200, height: 100, right: 200, bottom: 100 });
      mockRect(stack.querySelector('[data-slot="window-chrome-chip-cap"]'), { left: 0, right: 60, width: 60, height: 24, bottom: 24 });
      mockRect(stack.querySelector('[data-slot="window-chrome-gap"]'), { left: 60, right: 160, width: 100, height: 24, bottom: 24 });
      mockRect(stack.querySelector('[data-slot="window-chrome-controls"]'), { left: 160, right: 200, width: 40, height: 24, bottom: 24 });
      mockRect(stack.querySelector('[data-slot="window-chrome-cap"]'), { left: 0, height: 24, bottom: 24, width: 200, right: 200 });
      mockRect(stack.querySelector('[data-slot="window-chrome-footer-left"]'), { left: 0, right: 50, width: 50, height: 24, bottom: 100 });
      mockRect(stack.querySelector('[data-slot="window-chrome-footer-center-chip"]'), { left: 80, right: 120, width: 40, height: 24, bottom: 100 });
      stack.setAttribute("data-silhouette-remeasure", "intro");
      await waitFor(() => {
        const path = stack.querySelector('[data-slot="window-chrome-silhouette-border"] path')?.getAttribute("d");
        expect(path).toBe(
          windowSilhouettePath({
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
          }),
        );
        expect(path).toContain("V75 H120 V99 H80 V75 H50 V99");
        expect(path).not.toMatch(/V99 V\d+/);
      });
    });

    it("emphasizes only the introduction body paragraph under the pointer", async () => {
      const { readFileSync } = await import("node:fs");
      const { fileURLToPath } = await import("node:url");
      const { dirname, resolve } = await import("node:path");
      const css = readFileSync(resolve(dirname(fileURLToPath(source.url)), "../../../../🎨️styling/🖌️ui.css"), "utf8");
      expect(css).toMatch(/\[data-slot="introduction-info-box"\]\s*\[data-slot="introduction-body-paragraph"\]:hover\s*\{\s*color:\s*var\(--border-emphasized-color\);/);
      expect(css).not.toMatch(/\[data-slot="introduction-info-box"\]\s*\[data-slot="window-chrome-body"\]:hover/);
      expect(css).toMatch(
        /\[data-slot="introduction-info-box"\]:is\(\s*:has\(\[data-slot="window-chrome-body"\]:hover\)[\s\S]*?\)\s*\[data-slot="introduction-info-box-chip"\]:not\(\[data-handle-hovered="true"\]\)\s*\{\s*color:\s*var\(--border-emphasized-color\);/,
      );
      expect(css).toMatch(
        /\[data-slot="introduction-info-box"\]:is\([\s\S]*?:has\(\[data-slot="introduction-info-box-chip"\]:hover\)[\s\S]*?\)\s*\[data-slot="introduction-info-box-chip"\]\s*\[data-slot="drag-handle"\]\s*\{\s*color:\s*var\(--border-emphasized-color\);/,
      );
      expect(css).not.toMatch(/\[data-slot="introduction-info-box"\]:hover\s*\[data-slot="introduction-step-chip"\]/);
      expect(css).not.toMatch(/\[data-slot="introduction-body-paragraph"\]:focus-within/);
      expect(splitIntroductionBodyParagraphs("One.\n\nTwo.\n\n\nThree.")).toEqual(["One.", "Two.", "Three."]);
      expect(splitIntroductionBodyParagraphs("  single  ")).toEqual(["single"]);
      expect(splitIntroductionBodyParagraphs("")).toEqual([]);
      const steps: IntroductionStepDefinition[] = [
        {
          id: "body-hover",
          title: "Body Hover",
          body: "First paragraph.\n\nSecond paragraph.",
          introduce: null,
          show: [],
          placement: "center",
          interactions: [{ on: { kind: "action", id: "ui.footer" }, label: "Click footer" }],
          ordered: true,
          logos: [],
          demonstrations: [],
        },
      ];
      const { container } = render(<UIIntroduction introduction={{ title: "Welcome", steps }} stepIndex={0} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />);
      const paragraphs = container.querySelectorAll('[data-slot="introduction-body-paragraph"]');
      expect(paragraphs).toHaveLength(2);
      expect(paragraphs[0]?.textContent).toBe("First paragraph.");
      expect(paragraphs[1]?.textContent).toBe("Second paragraph.");
      expect(paragraphs[0]?.className).toContain("text-muted-foreground");
      expect(container.querySelector('[data-slot="introduction-interaction-label"]')?.textContent).toBe("Click footer");
    });

    it("renders step logos, wrapping only those with an href, and swaps light/dark srcs", () => {
      const { container } = render(
        <UIIntroduction
          introduction={{
            title: "Welcome",
            steps: [
              {
                id: "funding",
                title: "Funding",
                body: "Funded by",
                introduce: null,
                show: [],
                placement: "center",
                interactions: [],
                ordered: false,
                logos: [
                  { src: "/🖼️assets/logo/🖼️bbsr.png", darkSrc: "/🖼️assets/logo/🖼️bbsr-dark.png", alt: "BBSR", href: "https://www.bbsr.bund.de" },
                  { src: "/🖼️assets/logo/🖼️zukunft-bau.png", darkSrc: null, alt: "Zukunft Bau", href: null },
                ],
                demonstrations: [],
              },
            ],
          }}
          stepIndex={0}
          onStepIndexChange={vi.fn()}
          onDismiss={vi.fn()}
        />,
      );
      const box = container.querySelector('[data-slot="introduction-info-box"]');
      const links = box?.querySelectorAll("a[href]");
      expect(links).toHaveLength(1);
      expect(links?.[0].getAttribute("href")).toBe("https://www.bbsr.bund.de");
      const images = box?.querySelectorAll("img");
      expect(images).toHaveLength(3);
      expect(Array.from(images ?? []).map((img) => img.getAttribute("src"))).toEqual(["/🖼️assets/logo/🖼️bbsr.png", "/🖼️assets/logo/🖼️bbsr-dark.png", "/🖼️assets/logo/🖼️zukunft-bau.png"]);
    });

    it("renders an interaction checklist instead of the Next button, ticking off completed rows", () => {
      const steps: IntroductionStepDefinition[] = [
        {
          id: "viewport",
          title: "Viewport",
          body: "Navigate.",
          introduce: null,
          show: [],
          placement: "center",
          interactions: [
            { on: { kind: "zoom", id: "puzzle3d-main" }, label: "Zoom" },
            { on: { kind: "pan", id: "puzzle3d-main" }, label: "Pan" },
            { on: { kind: "orbit", id: "puzzle3d-main" }, label: "Orbit" },
          ],
          ordered: false,
          logos: [],
          demonstrations: [],
        },
      ];
      const { container } = render(<UIIntroduction introduction={{ title: "Welcome", steps }} stepIndex={0} completedInteractionIndices={[1]} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />);
      const rows = container.querySelectorAll('[data-slot="introduction-interactions"] li');
      expect(rows).toHaveLength(3);
      expect(Array.from(rows).map((row) => row.querySelector("[data-slot='introduction-interaction-label']")?.textContent)).toEqual(["Zoom", "Pan", "Orbit"]);
      expect(rows[0].getAttribute("data-completed")).toBeNull();
      expect(rows[1].getAttribute("data-completed")).toBe("true");
      expect(rows[2].getAttribute("data-completed")).toBeNull();
      expect(screen.queryByRole("button", { name: /next|done/i })).toBeNull();
    });

    it("celebrates a checklist row's own label text the instant its interaction flips to done, and only that row", () => {
      vi.useFakeTimers();
      try {
        const steps: IntroductionStepDefinition[] = [
          {
            id: "viewport",
            title: "Viewport",
            body: "Navigate.",
            introduce: null,
            show: [],
            placement: "center",
            interactions: [
              { on: { kind: "zoom", id: "puzzle3d-main" }, label: "Zoom" },
              { on: { kind: "pan", id: "puzzle3d-main" }, label: "Pan" },
            ],
            ordered: false,
            logos: [],
            demonstrations: [],
          },
        ];
        const { container, rerender } = render(<UIIntroduction introduction={{ title: "Welcome", steps }} stepIndex={0} completedInteractionIndices={[]} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />);
        const labels = () => container.querySelectorAll('[data-slot="introduction-interaction-label"]');
        expect(labels()[0]?.getAttribute("data-celebrated")).toBeNull();
        expect(labels()[1]?.getAttribute("data-celebrated")).toBeNull();

        rerender(<UIIntroduction introduction={{ title: "Welcome", steps }} stepIndex={0} completedInteractionIndices={[0]} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />);
        expect(labels()[0]?.getAttribute("data-celebrated")).toBe("true");
        expect(labels()[1]?.getAttribute("data-celebrated")).toBeNull();

        act(() => {
          vi.advanceTimersByTime(CELEBRATE_STAMP_DURATION_MS);
        });
        expect(labels()[0]?.getAttribute("data-celebrated")).toBeNull();
      } finally {
        vi.useRealTimers();
      }
    });

    it("numbers checklist rows when the step is ordered, and omits numbers when it isn't", () => {
      const orderedSteps: IntroductionStepDefinition[] = [
        {
          id: "viewport",
          title: "Viewport",
          body: "Navigate in order.",
          introduce: null,
          show: [],
          placement: "center",
          interactions: [
            { on: { kind: "zoom", id: "puzzle3d-main" }, label: "Zoom" },
            { on: { kind: "pan", id: "puzzle3d-main" }, label: "Pan" },
          ],
          ordered: true,
          logos: [],
          demonstrations: [],
        },
      ];
      const { container, rerender } = render(<UIIntroduction introduction={{ title: "Welcome", steps: orderedSteps }} stepIndex={0} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />);
      const orderedRows = container.querySelectorAll('[data-slot="introduction-interactions"] li');
      expect(orderedRows[0].querySelector("[data-slot='introduction-interaction-index']")?.textContent).toBe("1.");
      expect(orderedRows[0].querySelector("[data-slot='introduction-interaction-label']")?.textContent).toBe("Zoom");
      expect(orderedRows[1].querySelector("[data-slot='introduction-interaction-index']")?.textContent).toBe("2.");
      expect(orderedRows[1].querySelector("[data-slot='introduction-interaction-label']")?.textContent).toBe("Pan");

      const unorderedSteps: IntroductionStepDefinition[] = [{ ...orderedSteps[0], ordered: false }];
      rerender(<UIIntroduction introduction={{ title: "Welcome", steps: unorderedSteps }} stepIndex={0} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />);
      const unorderedRows = container.querySelectorAll('[data-slot="introduction-interactions"] li');
      expect(unorderedRows[0].querySelector("[data-slot='introduction-interaction-index']")).toBeNull();
      expect(unorderedRows[0].querySelector("[data-slot='introduction-interaction-label']")?.textContent).toBe("Zoom");
      expect(unorderedRows[1].querySelector("[data-slot='introduction-interaction-label']")?.textContent).toBe("Pan");
    });

    it("renders the Next button and no checklist when the step has no interactions", () => {
      const steps: IntroductionStepDefinition[] = [{ id: "welcome", title: "Welcome", body: "Hi.", introduce: null, show: [], placement: "center", interactions: [], ordered: false, logos: [], demonstrations: [] }];
      const { container } = render(<UIIntroduction introduction={{ title: "Welcome", steps }} stepIndex={0} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />);
      expect(container.querySelector('[data-slot="introduction-interactions"]')).toBeNull();
      expect(screen.getByRole("button", { name: /next|done|weiter|fertig/i })).toBeTruthy();
      expect(container.querySelector('[data-slot="window-chrome-footer-right"]')).toBeTruthy();
    });

    it("grows the introduction window for complete step titles and wraps only at the viewport boundary", () => {
      const title = "Willkommen bei Entwerfen mit Bestand";
      const steps: IntroductionStepDefinition[] = [{ id: "welcome", title, body: "Hi.", introduce: null, show: [], placement: "center", interactions: [], ordered: false, logos: [], demonstrations: [] }];
      const { container } = render(<UIIntroduction introduction={{ title: "Welcome", steps }} stepIndex={0} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />);
      const box = container.querySelector('[data-slot="introduction-info-box"]') as HTMLElement;
      const chip = container.querySelector('[data-slot="introduction-info-box-chip"]') as HTMLElement;
      const stepTitle = container.querySelector('[data-slot="introduction-info-box-title"]') as HTMLElement;
      const content = container.querySelector('[data-slot="introduction-info-box-content"]') as HTMLElement;
      expect(stepTitle.textContent).toBe(title);
      expect(stepTitle.className).not.toContain("truncate");
      expect(stepTitle.className).toContain("break-words");
      expect(stepTitle.className).toContain("whitespace-normal");
      expect(chip.className).toContain("max-w-none");
      expect(chip.className).not.toContain("max-w-[12rem]");
      expect(chip.className.split(" ")).toContain("shrink");
      expect(chip.className.split(" ")).not.toContain("shrink-0");
      expect(chip.className.split(" ")).toContain("!h-auto");
      expect(chip.className.split(" ")).toContain("overflow-visible");
      expect(chip.className.split(" ")).not.toContain("overflow-hidden");
      expect(box.className).toContain("w-fit");
      expect(box.className).toContain("max-w-[calc(100vw-2rem)]");
      expect(content.className).toContain("max-w-sm");
    });

    it("renders a header drag handle in the title chip and moves the info box", () => {
      const steps: IntroductionStepDefinition[] = [{ id: "welcome", title: "Welcome", body: "Hi.", introduce: null, show: [], placement: "center", interactions: [], ordered: false, logos: [], demonstrations: [] }];
      const { container } = render(<UIIntroduction introduction={{ title: "Welcome", steps }} stepIndex={0} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />);
      const box = container.querySelector('[data-slot="introduction-info-box"]') as HTMLElement;
      const chip = container.querySelector('[data-slot="introduction-info-box-chip"]') as HTMLElement;
      const dragRail = container.querySelector('[data-slot="introduction-info-box-drag"]');
      const handle = dragRail?.querySelector('[data-slot="drag-handle"]') as HTMLElement;
      expect(chip).toBeTruthy();
      expect(dragRail).toBeTruthy();
      expect(handle).toBeTruthy();
      expect(chip.contains(dragRail)).toBe(true);
      expect(chip.textContent).toContain("Welcome");
      expect(container.querySelector('[data-slot="introduction-close"]')).toBeTruthy();
      expect(container.textContent).toMatch(/1\s*\/\s*1/);
      const startTop = Number.parseFloat(box.style.top);
      const startLeft = Number.parseFloat(box.style.left);
      fireEvent.pointerDown(handle, { pointerId: 1, pointerType: "mouse", clientX: 100, clientY: 100 });
      fireEvent.pointerMove(handle, { pointerId: 1, pointerType: "mouse", clientX: 140, clientY: 160 });
      expect(box.getAttribute("data-dragging")).toBe("true");
      expect(Number.parseFloat(box.style.top)).toBe(startTop + 60);
      expect(Number.parseFloat(box.style.left)).toBe(startLeft + 40);
      fireEvent.pointerMove(handle, { pointerId: 1, pointerType: "mouse", clientX: -10_000, clientY: -10_000 });
      expect(Number.parseFloat(box.style.top)).toBe(0);
      expect(Number.parseFloat(box.style.left)).toBe(0);
      fireEvent.pointerMove(handle, { pointerId: 1, pointerType: "mouse", clientX: 10_000, clientY: 10_000 });
      expect(Number.parseFloat(box.style.top)).toBe(window.innerHeight - box.offsetHeight);
      expect(Number.parseFloat(box.style.left)).toBe(window.innerWidth - box.offsetWidth);
      fireEvent.pointerUp(handle, { pointerId: 1, pointerType: "mouse", clientX: 10_000, clientY: 10_000 });
      expect(box.getAttribute("data-dragging")).toBeNull();
      expect(Number.parseFloat(box.style.top)).toBe(window.innerHeight - box.offsetHeight);
      expect(Number.parseFloat(box.style.left)).toBe(window.innerWidth - box.offsetWidth);
    });

    it("places pressable Back and Next controls outside the transparent side gaps' hit-testing and accessibility suppression", () => {
      const steps: IntroductionStepDefinition[] = [
        { id: "first", title: "First", body: "Step one.", introduce: null, show: [], placement: "center", interactions: [], ordered: false, logos: [], demonstrations: [] },
        { id: "second", title: "Second", body: "Step two.", introduce: null, show: [], placement: "center", interactions: [], ordered: false, logos: [], demonstrations: [] },
        { id: "third", title: "Third", body: "Step three.", introduce: null, show: [], placement: "center", interactions: [], ordered: false, logos: [], demonstrations: [] },
      ];
      const onStepIndexChange = vi.fn();
      const { container } = render(<UIIntroduction introduction={{ title: "Welcome", steps }} stepIndex={1} onStepIndexChange={onStepIndexChange} onDismiss={vi.fn()} />);
      const footer = container.querySelector('[data-slot="window-chrome-footer"]');
      const footerGapLeft = container.querySelector('[data-slot="window-chrome-footer-gap-left"]');
      const footerGapRight = container.querySelector('[data-slot="window-chrome-footer-gap-right"]');
      const footerCenter = container.querySelector('[data-slot="window-chrome-footer-center"]');
      const footerLeft = container.querySelector('[data-slot="window-chrome-footer-left"]');
      const footerRight = container.querySelector('[data-slot="window-chrome-footer-right"]');
      expect(footer).toBeTruthy();
      expect(footerGapLeft).toBeTruthy();
      expect(footerGapRight).toBeTruthy();
      expect(footerCenter).toBeTruthy();
      expect(footerLeft).toBeTruthy();
      expect(footerRight).toBeTruthy();
      expect(footerCenter?.querySelector('[data-slot="window-chrome-footer-center-chip"]')?.textContent).toMatch(/2\s*\/\s*3/);
      expect(footerGapLeft?.getAttribute("aria-hidden")).toBeNull();
      expect(footerGapRight?.getAttribute("aria-hidden")).toBeNull();
      expect(footerLeft?.classList.contains("pointer-events-auto")).toBe(true);
      expect(footerRight?.classList.contains("pointer-events-auto")).toBe(true);
      expect(footer?.className).toContain("grid-cols-[minmax(0,1fr)_auto_minmax(0,1fr)]");
      const backButton = screen.getByRole("button", { name: /back|zurück/i });
      const nextButton = screen.getByRole("button", { name: /next|weiter/i });
      expect(backButton.className).toContain("hover:bg-hover-interactive-fill");
      expect(nextButton.className).toContain("hover:bg-hover-interactive-fill");
      fireEvent.click(backButton);
      fireEvent.click(nextButton);
      expect(onStepIndexChange).toHaveBeenNthCalledWith(1, 0);
      expect(onStepIndexChange).toHaveBeenNthCalledWith(2, 2);
      expect(container.querySelector('[data-slot="introduction-info-box"] [data-slot="window-chrome-body"]')?.textContent).not.toMatch(/next|done|back|zurück|\d\s*\/\s*\d/i);
    });

    it("centers the step chip when navigation occupies either side alone", () => {
      const rightOnlySteps: IntroductionStepDefinition[] = [{ id: "only", title: "Only", body: "Step one.", introduce: null, show: [], placement: "center", interactions: [], ordered: false, logos: [], demonstrations: [] }];
      const { container, rerender } = render(<UIIntroduction introduction={{ title: "Welcome", steps: rightOnlySteps }} stepIndex={0} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />);
      expect(container.querySelector('[data-slot="window-chrome-footer-gap-left"]')).toBeTruthy();
      expect(container.querySelector('[data-slot="window-chrome-footer-gap-right"]')).toBeTruthy();
      expect(container.querySelector('[data-slot="window-chrome-footer-center-chip"]')?.textContent).toMatch(/1\s*\/\s*1/);
      expect(container.querySelector('[data-slot="window-chrome-footer-left"]')).toBeNull();
      expect(container.querySelector('[data-slot="window-chrome-footer-right"]')).toBeTruthy();
      expect(container.querySelector('[data-slot="window-chrome-footer"]')?.className).toContain("grid-cols-[minmax(0,1fr)_auto_minmax(0,1fr)]");

      const leftOnlySteps: IntroductionStepDefinition[] = [
        { id: "first", title: "First", body: "Step one.", introduce: null, show: [], placement: "center", interactions: [], ordered: false, logos: [], demonstrations: [] },
        {
          id: "second",
          title: "Second",
          body: "Complete the step.",
          introduce: null,
          show: [],
          placement: "center",
          interactions: [{ on: { kind: "action", id: "ui.footer" }, label: "Click" }],
          ordered: false,
          logos: [],
          demonstrations: [],
        },
      ];
      rerender(<UIIntroduction introduction={{ title: "Welcome", steps: leftOnlySteps }} stepIndex={1} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />);
      expect(container.querySelector('[data-slot="window-chrome-footer-center-chip"]')?.textContent).toMatch(/2\s*\/\s*2/);
      expect(container.querySelector('[data-slot="window-chrome-footer-left"]')).toBeTruthy();
      expect(container.querySelector('[data-slot="window-chrome-footer-right"]')).toBeNull();
      expect(container.querySelector('[data-slot="window-chrome-footer"]')?.className).toContain("grid-cols-[minmax(0,1fr)_auto_minmax(0,1fr)]");
    });

    it("does not paint glass on the footer center rail when the next chip is absent", () => {
      const steps: IntroductionStepDefinition[] = [
        {
          id: "interact",
          title: "Interact",
          body: "Complete the step.",
          introduce: null,
          show: [],
          placement: "center",
          interactions: [{ on: { kind: "action", id: "ui.footer" }, label: "Click" }],
          ordered: false,
          logos: [],
          demonstrations: [],
        },
      ];
      const { container } = render(<UIIntroduction introduction={{ title: "Welcome", steps }} stepIndex={0} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />);
      const centerRail = container.querySelector('[data-slot="window-chrome-footer-center"]');
      const centerChip = container.querySelector('[data-slot="window-chrome-footer-center-chip"]');
      expect(centerRail?.className).not.toMatch(/ui-glass-chrome/);
      expect(centerChip?.className).toMatch(/ui-glass/);
      expect(container.querySelector('[data-slot="window-chrome-footer-right"]')).toBeNull();
      expect(container.querySelector('[data-slot="window-chrome-footer"]')?.className).toContain("grid-cols-[minmax(0,1fr)_auto_minmax(0,1fr)]");
    });
  });

  describe("introductionRectRelativeToHost", () => {
    it("subtracts the host origin from a viewport rect so absolute overlays survive CSS-transformed shells", () => {
      expect(introductionRectRelativeToHost({ top: 120, left: 80, width: 40, height: 20 }, { top: 100, left: 50, width: 400, height: 300 })).toEqual({
        top: 20,
        left: 30,
        width: 40,
        height: 20,
      });
    });
  });

  describe("introductionPointRelativeToHost", () => {
    it("subtracts the host origin from a viewport point", () => {
      expect(introductionPointRelativeToHost({ x: 250, y: 180 }, { top: 100, left: 50, width: 400, height: 300 })).toEqual({ x: 200, y: 80 });
    });
  });

  describe("clampIntroductionInfoBoxPosition", () => {
    const viewport = { width: 800, height: 600 };
    const boxSize = { width: 100, height: 50 };

    it("lets direct manipulation meet every viewport border exactly", () => {
      expect(clampIntroductionInfoBoxPosition({ top: -100, left: -100 }, boxSize, viewport, 0)).toEqual({ top: 0, left: 0 });
      expect(clampIntroductionInfoBoxPosition({ top: 1_000, left: 1_000 }, boxSize, viewport, 0)).toEqual({ top: 550, left: 700 });
    });

    it("preserves the authored placement inset", () => {
      expect(clampIntroductionInfoBoxPosition({ top: -100, left: -100 }, boxSize, viewport, INTRODUCTION_INFO_BOX_GAP_PX)).toEqual({ top: 16, left: 16 });
      expect(clampIntroductionInfoBoxPosition({ top: 1_000, left: 1_000 }, boxSize, viewport, INTRODUCTION_INFO_BOX_GAP_PX)).toEqual({ top: 534, left: 684 });
    });
  });

  describe("resolveIntroductionPlacement", () => {
    const viewport = { width: 800, height: 600 };
    const boxSize = { width: 100, height: 50 };

    it("centers when there is no anchor rect, or placement is center", () => {
      expect(resolveIntroductionPlacement("auto", null, boxSize, viewport)).toEqual({ top: 275, left: 350 });
      expect(resolveIntroductionPlacement("center", { top: 10, left: 10, width: 20, height: 20 }, boxSize, viewport)).toEqual({ top: 275, left: 350 });
    });

    it("auto picks the side with the most free space", () => {
      // 🎓️ Anchor near the top: most free space is below it, so auto should place the box below.
      const nearTop = { top: 0, left: 350, width: 100, height: 20 };
      const placedBelow = resolveIntroductionPlacement("auto", nearTop, boxSize, viewport);
      expect(placedBelow.top).toBeGreaterThan(nearTop.top + nearTop.height);
    });

    it("explicit placements position relative to the anchor and stay within the viewport", () => {
      const anchor = { top: 275, left: 350, width: 100, height: 20 };
      const right = resolveIntroductionPlacement("right", anchor, boxSize, viewport);
      expect(right.left).toBeGreaterThan(anchor.left + anchor.width);
      const left = resolveIntroductionPlacement("left", anchor, boxSize, viewport);
      expect(left.left).toBeLessThan(anchor.left);
      const nearEdge = resolveIntroductionPlacement("right", { top: 10, left: 780, width: 10, height: 10 }, boxSize, viewport);
      expect(nearEdge.left).toBeLessThanOrEqual(viewport.width - boxSize.width);
    });
  });

  describe("resolveIntroductionPoint", () => {
    it("resolves an element point to its rect center, or a normalized offset within it", () => {
      const { container } = render(<div id="demo.target" />);
      const el = container.querySelector("#demo\\.target")!;
      vi.spyOn(el, "getBoundingClientRect").mockReturnValue({ left: 100, top: 200, width: 100, height: 50, right: 200, bottom: 250, x: 100, y: 200, toJSON: () => ({}) } as DOMRect);
      expect(resolveIntroductionPoint({ kind: "element", id: "demo.target" })).toEqual({ x: 150, y: 225 });
      expect(resolveIntroductionPoint({ kind: "element", id: "demo.target", offset: [0, 0] })).toEqual({ x: 100, y: 200 });
    });

    it("resolves absolute and normalized screen points against the viewport", () => {
      const originalWidth = window.innerWidth;
      const originalHeight = window.innerHeight;
      Object.defineProperty(window, "innerWidth", { value: 800, configurable: true });
      Object.defineProperty(window, "innerHeight", { value: 600, configurable: true });
      expect(resolveIntroductionPoint({ kind: "screen", x: 10, y: 20 })).toEqual({ x: 10, y: 20 });
      expect(resolveIntroductionPoint({ kind: "screenNormalized", x: 0.5, y: 0.5 })).toEqual({ x: 400, y: 300 });
      Object.defineProperty(window, "innerWidth", { value: originalWidth, configurable: true });
      Object.defineProperty(window, "innerHeight", { value: originalHeight, configurable: true });
    });

    it("resolves window-local absolute and normalized points against the target's rect", () => {
      const { container } = render(<div id="demo.window" />);
      const el = container.querySelector("#demo\\.window")!;
      vi.spyOn(el, "getBoundingClientRect").mockReturnValue({ left: 50, top: 60, width: 200, height: 100, right: 250, bottom: 160, x: 50, y: 60, toJSON: () => ({}) } as DOMRect);
      expect(resolveIntroductionPoint({ kind: "window", id: "demo.window", x: 10, y: 20 })).toEqual({ x: 60, y: 80 });
      expect(resolveIntroductionPoint({ kind: "windowNormalized", id: "demo.window", x: 0.5, y: 0.5 })).toEqual({ x: 150, y: 110 });
    });

    it("resolves a scene point through its registered resolver, and null once off-camera or unregistered", () => {
      const unregister = registerIntroductionSurfaceResolver("✏️demo.scene", {
        scenePoint: (position) => (position[0] > 0 ? { x: 42, y: 84, visible: true } : { x: 0, y: 0, visible: false }),
      });
      expect(resolveIntroductionPoint({ kind: "scene", id: "✏️demo.scene", position: [1, 0, 0] })).toEqual({ x: 42, y: 84 });
      expect(resolveIntroductionPoint({ kind: "scene", id: "✏️demo.scene", position: [-1, 0, 0] })).toBeNull();
      unregister();
      expect(resolveIntroductionPoint({ kind: "scene", id: "✏️demo.scene", position: [1, 0, 0] })).toBeNull();
    });

    it("resolves a canvas (2D world) point through its registered resolver", () => {
      const unregister = registerIntroductionSurfaceResolver("demo.canvas", {
        canvasPoint: (x, y) => ({ x: x * 2, y: y * 2, visible: true }),
      });
      expect(resolveIntroductionPoint({ kind: "canvas", id: "demo.canvas", x: 10, y: 20 })).toEqual({ x: 20, y: 40 });
      unregister();
      expect(resolveIntroductionPoint({ kind: "canvas", id: "demo.canvas", x: 10, y: 20 })).toBeNull();
    });

    it("resolves an entity point centered, or at a normalized offset within its rect, and a wildcard entity id", () => {
      const unregister = registerIntroductionSurfaceResolver("demo.entities", {
        entity: (domain, entityId) => {
          if (domain !== "vortex") return null;
          if (entityId === "*") return { point: { x: 5, y: 5 }, visible: true };
          if (entityId !== "obj:v0") return null;
          return { point: { x: 100, y: 100 }, rect: { x: 80, y: 90, width: 40, height: 20 }, visible: true };
        },
      });
      expect(resolveIntroductionPoint({ kind: "entity", id: "demo.entities", domain: "vortex", entity: "obj:v0" })).toEqual({ x: 100, y: 100 });
      expect(resolveIntroductionPoint({ kind: "entity", id: "demo.entities", domain: "vortex", entity: "obj:v0", offset: [0, 0] })).toEqual({ x: 80, y: 90 });
      expect(resolveIntroductionPoint({ kind: "entity", id: "demo.entities", domain: "vortex", entity: "*" })).toEqual({ x: 5, y: 5 });
      expect(resolveIntroductionPoint({ kind: "entity", id: "demo.entities", domain: "vortex", entity: "missing" })).toBeNull();
      expect(resolveIntroductionPoint({ kind: "entity", id: "demo.entities", domain: "edge", entity: "*" })).toBeNull();
      unregister();
    });

    it("resolves a curve point by arc-length t along an entity's polyline", () => {
      const unregister = registerIntroductionSurfaceResolver("demo.curve", {
        entity: (domain, entityId) => {
          if (domain !== "attraction" || entityId !== "a1") return null;
          return {
            point: { x: 50, y: 0 },
            polyline: [
              { x: 0, y: 0 },
              { x: 100, y: 0 },
            ],
            visible: true,
          };
        },
      });
      expect(resolveIntroductionPoint({ kind: "curve", id: "demo.curve", domain: "attraction", entity: "a1", t: 0 })).toEqual({ x: 0, y: 0 });
      expect(resolveIntroductionPoint({ kind: "curve", id: "demo.curve", domain: "attraction", entity: "a1", t: 0.5 })).toEqual({ x: 50, y: 0 });
      expect(resolveIntroductionPoint({ kind: "curve", id: "demo.curve", domain: "attraction", entity: "missing", t: 0.5 })).toBeNull();
      unregister();
    });

    it("resolves a domain point by mapping value onto the entity's rect along its axis", () => {
      const unregister = registerIntroductionSurfaceResolver("demo.domain", {
        entity: (domain, entityId) => {
          if (domain !== "slider" || entityId !== "fillCount") return null;
          return { point: { x: 100, y: 50 }, rect: { x: 50, y: 40, width: 100, height: 20 }, domain: { min: 0, max: 10, axis: "x" }, visible: true };
        },
      });
      expect(resolveIntroductionPoint({ kind: "domain", id: "demo.domain", domain: "slider", entity: "fillCount", value: 0 })).toEqual({ x: 50, y: 50 });
      expect(resolveIntroductionPoint({ kind: "domain", id: "demo.domain", domain: "slider", entity: "fillCount", value: 10 })).toEqual({ x: 150, y: 50 });
      expect(resolveIntroductionPoint({ kind: "domain", id: "demo.domain", domain: "slider", entity: "fillCount", value: 5 })).toEqual({ x: 100, y: 50 });
      // 🎚️ Out-of-range values clamp into [min, max] rather than extrapolating past the rect.
      expect(resolveIntroductionPoint({ kind: "domain", id: "demo.domain", domain: "slider", entity: "fillCount", value: 999 })).toEqual({ x: 150, y: 50 });
      unregister();
    });

    it("returns null for an element/window point that hasn't mounted", () => {
      expect(resolveIntroductionPoint({ kind: "element", id: "nothing.here" })).toBeNull();
      expect(resolveIntroductionPoint({ kind: "window", id: "nothing.here", x: 0, y: 0 })).toBeNull();
    });
  });

  describe("polylinePointAt", () => {
    it("interpolates by arc length across a multi-segment polyline", () => {
      const points = [
        { x: 0, y: 0 },
        { x: 10, y: 0 },
        { x: 10, y: 10 },
      ];
      expect(polylinePointAt(points, 0)).toEqual({ x: 0, y: 0 });
      expect(polylinePointAt(points, 1)).toEqual({ x: 10, y: 10 });
      // 🪡️ Total length 20 (10 + 10); t=0.25 lands exactly at the corner (5 units along the first segment).
      expect(polylinePointAt(points, 0.25)).toEqual({ x: 5, y: 0 });
      // t=0.75 is 5 units into the second segment.
      expect(polylinePointAt(points, 0.75)).toEqual({ x: 10, y: 5 });
    });

    it("clamps t outside [0, 1] and degrades gracefully for degenerate polylines", () => {
      const points = [
        { x: 0, y: 0 },
        { x: 10, y: 0 },
      ];
      expect(polylinePointAt(points, -1)).toEqual({ x: 0, y: 0 });
      expect(polylinePointAt(points, 2)).toEqual({ x: 10, y: 0 });
      expect(polylinePointAt([], 0.5)).toEqual({ x: 0, y: 0 });
      expect(polylinePointAt([{ x: 3, y: 4 }], 0.5)).toEqual({ x: 3, y: 4 });
      expect(
        polylinePointAt(
          [
            { x: 1, y: 1 },
            { x: 1, y: 1 },
          ],
          0.5,
        ),
      ).toEqual({ x: 1, y: 1 });
    });
  });

  describe("sampleBezierSegments", () => {
    it("samples move/line segments verbatim and quad/cubic curves through their control points", () => {
      const linePoints = sampleBezierSegments([
        { kind: "move", to: [0, 0] },
        { kind: "line", to: [10, 0] },
      ]);
      expect(linePoints).toEqual([
        { x: 0, y: 0 },
        { x: 10, y: 0 },
      ]);

      const quadPoints = sampleBezierSegments(
        [
          { kind: "move", to: [0, 0] },
          { kind: "quad", ctrl: [5, 10], to: [10, 0] },
        ],
        2,
      );
      // 🪡️ 2 samples of a quad from (0,0) via ctrl (5,10) to (10,0): t=0.5 is the curve's own midpoint.
      expect(quadPoints).toHaveLength(3);
      expect(quadPoints[1].x).toBeCloseTo(5, 5);
      expect(quadPoints[1].y).toBeCloseTo(5, 5);
      expect(quadPoints[2]).toEqual({ x: 10, y: 0 });
    });

    it("ignores unsampleable segment kinds without throwing", () => {
      expect(sampleBezierSegments([{ kind: "close" }])).toEqual([]);
      expect(sampleBezierSegments([])).toEqual([]);
    });
  });

  describe("ndcToViewportPoint", () => {
    it("maps NDC corners and center to viewport pixels", () => {
      const rect = { left: 100, top: 50, width: 200, height: 100 };
      expect(ndcToViewportPoint({ x: 0, y: 0 }, rect)).toEqual({ x: 200, y: 100 });
      expect(ndcToViewportPoint({ x: -1, y: -1 }, rect)).toEqual({ x: 100, y: 150 });
      expect(ndcToViewportPoint({ x: 1, y: 1 }, rect)).toEqual({ x: 300, y: 50 });
    });
  });

  describe("introductionDemoResolveVisual", () => {
    it("maps each gesture kind to a unique button, modifier set, and feedback family", () => {
      const at = { kind: "screenNormalized" as const, x: 0.5, y: 0.5 };
      expect(introductionDemoResolveVisual({ kind: "leftClick", at })).toEqual({ button: "left", modifiers: [], feedback: "leftClick", showDoubleChip: false });
      expect(introductionDemoResolveVisual({ kind: "rightClick", at })).toEqual({ button: "right", modifiers: [], feedback: "rightClick", showDoubleChip: false });
      expect(introductionDemoResolveVisual({ kind: "doubleClick", at })).toEqual({ button: "left", modifiers: [], feedback: "doubleClick", showDoubleChip: true });
      expect(introductionDemoResolveVisual({ kind: "scroll", at, deltaY: -100 })).toEqual({ button: "wheel", modifiers: [], feedback: "scroll", showDoubleChip: false });
      expect(introductionDemoResolveVisual({ kind: "drag", from: at, to: at })).toEqual({ button: "left", modifiers: [], feedback: "dragLeft", showDoubleChip: false });
      expect(introductionDemoResolveVisual({ kind: "drag", from: at, to: at, button: "middle" })).toEqual({ button: "middle", modifiers: [], feedback: "dragMiddle", showDoubleChip: false });
      expect(introductionDemoResolveVisual({ kind: "orbit", from: at, to: at })).toEqual({ button: "right", modifiers: ["alt"], feedback: "orbit", showDoubleChip: false });
    });
  });

  describe("introductionDemoArcPoint", () => {
    it("starts and ends exactly at the endpoints, and bulges away from the straight line in between", () => {
      const from = { x: 0, y: 0 };
      const to = { x: 100, y: 0 };
      expect(introductionDemoArcPoint(from, to, 0)).toEqual(from);
      expect(introductionDemoArcPoint(from, to, 1)).toEqual(to);
      const mid = introductionDemoArcPoint(from, to, 0.5);
      // 🎬️ The straight-line midpoint is (50, 0) — an orbit arc must bulge perpendicular (off the x-axis).
      expect(mid.x).toBeCloseTo(50, 5);
      expect(Math.abs(mid.y)).toBeGreaterThan(5);
    });
  });

  describe("useIntroductionPointerIdle", () => {
    it("goes idle after the threshold, resets on coordinate or pointer-lock movement, and ignores stationary pointermoves", () => {
      vi.useFakeTimers();
      try {
        const IdleProbe: React.FC = () => {
          const { idle, lastPositionRef } = useIntroductionPointerIdle(true, 1000);
          return <div data-testid="idle-probe" data-idle={idle ? "true" : "false"} data-last={lastPositionRef.current ? `${lastPositionRef.current.x},${lastPositionRef.current.y}` : ""} />;
        };
        const { getByTestId } = render(<IdleProbe />);
        const probe = () => getByTestId("idle-probe").getAttribute("data-idle");

        expect(probe()).toBe("false");
        act(() => {
          vi.advanceTimersByTime(999);
        });
        expect(probe()).toBe("false");
        act(() => {
          vi.advanceTimersByTime(1);
        });
        expect(probe()).toBe("true");

        fireEvent(window, new MouseEvent("pointermove", { bubbles: true, clientX: 10, clientY: 20 }));
        expect(probe()).toBe("false");
        expect(getByTestId("idle-probe").getAttribute("data-last")).toBe("10,20");

        // 🎬️ Same coordinates as the last move: ignored, so the idle timer set by the move above must NOT
        // have been restarted — advancing exactly the threshold from that move flips idle back to true.
        fireEvent(window, new MouseEvent("pointermove", { bubbles: true, clientX: 10, clientY: 20 }));
        expect(probe()).toBe("false");
        act(() => {
          vi.advanceTimersByTime(1000);
        });
        expect(probe()).toBe("true");

        const pointerLockMove = new PointerEvent("pointermove", { bubbles: true, clientX: 10, clientY: 20 });
        Object.defineProperty(pointerLockMove, "movementX", { value: 4 });
        fireEvent(window, pointerLockMove);
        expect(probe()).toBe("false");
      } finally {
        vi.useRealTimers();
      }
    });
  });

  describe("UIIntroduction demonstration", () => {
    const demoSteps: IntroductionStepDefinition[] = [
      {
        id: "fill",
        title: "Füllen",
        body: "Click fill.",
        introduce: "tool.fill",
        show: [],
        placement: "auto",
        interactions: [{ on: { kind: "tool", id: "fill" }, label: "Activate Fill" }],
        ordered: false,
        logos: [],
        demonstrations: [{ gesture: { kind: "leftClick", at: { kind: "element", id: "tool.fill" } } }],
      },
    ];

    it("mutes the real cursor only once idle, and restores it the instant the pointer moves", () => {
      vi.useFakeTimers();
      try {
        render(
          <div>
            <button id="tool.fill">Fill</button>
            <UIIntroduction introduction={{ title: "Welcome", steps: demoSteps }} stepIndex={0} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />
          </div>,
        );
        expect(document.documentElement.hasAttribute("data-introduction-demonstrating")).toBe(false);

        act(() => {
          vi.advanceTimersByTime(INTRODUCTION_DEMO_IDLE_THRESHOLD_MS);
        });
        expect(document.documentElement.getAttribute("data-introduction-demonstrating")).toBe("true");
        const overlay = document.querySelector<HTMLElement>('[data-slot="introduction-demonstration"]');
        expect(overlay?.style.visibility).toBe("visible");

        fireEvent(window, new MouseEvent("pointermove", { bubbles: true, clientX: 5, clientY: 5 }));
        expect(document.documentElement.hasAttribute("data-introduction-demonstrating")).toBe(false);
        expect(overlay?.style.visibility).toBe("hidden");
      } finally {
        vi.useRealTimers();
      }
    });

    it("unmount clears the demonstrating attribute", () => {
      vi.useFakeTimers();
      try {
        const { unmount } = render(
          <div>
            <button id="tool.fill">Fill</button>
            <UIIntroduction introduction={{ title: "Welcome", steps: demoSteps }} stepIndex={0} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />
          </div>,
        );
        act(() => {
          vi.advanceTimersByTime(INTRODUCTION_DEMO_IDLE_THRESHOLD_MS);
        });
        expect(document.documentElement.getAttribute("data-introduction-demonstrating")).toBe("true");
        unmount();
        expect(document.documentElement.hasAttribute("data-introduction-demonstrating")).toBe(false);
      } finally {
        vi.useRealTimers();
      }
    });

    it("a purely informational step (Next is the only way forward) auto-demonstrates clicking Next, even without a declared demonstration", () => {
      vi.useFakeTimers();
      try {
        const steps: IntroductionStepDefinition[] = [{ id: "welcome", title: "Welcome", body: "No demo declared.", introduce: null, show: [], placement: "center", interactions: [], ordered: false, logos: [], demonstrations: [] }];
        render(<UIIntroduction introduction={{ title: "Welcome", steps }} stepIndex={0} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />);
        act(() => {
          vi.advanceTimersByTime(INTRODUCTION_DEMO_IDLE_THRESHOLD_MS);
        });
        expect(document.documentElement.getAttribute("data-introduction-demonstrating")).toBe("true");
      } finally {
        vi.useRealTimers();
      }
    });

    it("omitted demonstrations (Rust serde default) still auto-demonstrates Next on informational steps", () => {
      vi.useFakeTimers();
      try {
        const steps = [{ id: "welcome", title: "Welcome", body: "No demo field.", introduce: null, show: [], placement: "center" as const, interactions: [], ordered: false, logos: [] }] as IntroductionStepDefinition[];
        render(<UIIntroduction introduction={{ title: "Welcome", steps }} stepIndex={0} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />);
        expect(document.querySelector('[data-slot="introduction-info-box"]')).toBeTruthy();
        act(() => {
          vi.advanceTimersByTime(INTRODUCTION_DEMO_IDLE_THRESHOLD_MS);
        });
        expect(document.documentElement.getAttribute("data-introduction-demonstrating")).toBe("true");
      } finally {
        vi.useRealTimers();
      }
    });

    it("mounts the demonstration callout cluster while demonstrating", () => {
      vi.useFakeTimers();
      try {
        render(
          <div>
            <button id="tool.fill">Fill</button>
            <UIIntroduction introduction={{ title: "Welcome", steps: demoSteps }} stepIndex={0} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />
          </div>,
        );
        act(() => {
          vi.advanceTimersByTime(INTRODUCTION_DEMO_IDLE_THRESHOLD_MS);
        });
        expect(document.querySelector('[data-slot="introduction-demonstration-callout"]')).toBeTruthy();
        expect(document.querySelector(".introduction-demo-mouse")).toBeTruthy();
        expect(document.querySelector('.introduction-demo-mouse-button[data-part="left"]')).toBeTruthy();
      } finally {
        vi.useRealTimers();
      }
    });

    it("an interaction-gated step with no declared demonstration never mutes the cursor", () => {
      vi.useFakeTimers();
      try {
        const steps: IntroductionStepDefinition[] = [
          {
            id: "transform-utility",
            title: "Transform",
            body: "No demo declared.",
            introduce: "transform",
            show: [],
            placement: "auto",
            interactions: [{ on: { kind: "utility", id: "transform" }, label: "Activate Transform" }],
            ordered: false,
            logos: [],
            demonstrations: [],
          },
        ];
        render(<UIIntroduction introduction={{ title: "Welcome", steps }} stepIndex={0} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />);
        act(() => {
          vi.advanceTimersByTime(INTRODUCTION_DEMO_IDLE_THRESHOLD_MS);
        });
        expect(document.documentElement.hasAttribute("data-introduction-demonstrating")).toBe(false);
      } finally {
        vi.useRealTimers();
      }
    });

    const viewportDemoSteps: IntroductionStepDefinition[] = [
      {
        id: "viewport",
        title: "3D View",
        body: "Zoom, pan, orbit.",
        introduce: "framework.window.puzzle3dMain",
        show: [],
        placement: "auto",
        interactions: [
          { on: { kind: "zoom", id: "puzzle3d-main" }, label: "Zoom" },
          { on: { kind: "pan", id: "puzzle3d-main" }, label: "Pan" },
          { on: { kind: "orbit", id: "puzzle3d-main" }, label: "Orbit" },
        ],
        ordered: false,
        logos: [],
        demonstrations: [
          { gesture: { kind: "scroll", at: { kind: "element", id: "framework.window.puzzle3dMain" }, deltaY: -100 } },
          { gesture: { kind: "drag", from: { kind: "element", id: "framework.window.puzzle3dMain" }, to: { kind: "element", id: "framework.window.puzzle3dMain" }, button: "middle" } },
          { gesture: { kind: "orbit", from: { kind: "element", id: "framework.window.puzzle3dMain" }, to: { kind: "element", id: "framework.window.puzzle3dMain" } } },
        ],
      },
    ];

    it("still demonstrates remaining interactions when some are already completed", () => {
      vi.useFakeTimers();
      try {
        render(
          <>
            <div id="framework.window.puzzle3dMain" style={{ width: 400, height: 300 }} />
            <UIIntroduction introduction={{ title: "Welcome", steps: viewportDemoSteps }} stepIndex={0} completedInteractionIndices={[0]} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />
          </>,
        );
        act(() => {
          vi.advanceTimersByTime(INTRODUCTION_DEMO_IDLE_THRESHOLD_MS);
        });
        expect(document.documentElement.getAttribute("data-introduction-demonstrating")).toBe("true");
        expect(document.querySelector('[data-slot="introduction-demonstration-callout"]')).toBeTruthy();
      } finally {
        vi.useRealTimers();
      }
    });

    it("stops demonstrating when every interaction is completed", () => {
      vi.useFakeTimers();
      try {
        render(
          <>
            <div id="framework.window.puzzle3dMain" style={{ width: 400, height: 300 }} />
            <UIIntroduction introduction={{ title: "Welcome", steps: viewportDemoSteps }} stepIndex={0} completedInteractionIndices={[0, 1, 2]} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />
          </>,
        );
        act(() => {
          vi.advanceTimersByTime(INTRODUCTION_DEMO_IDLE_THRESHOLD_MS);
        });
        expect(document.documentElement.hasAttribute("data-introduction-demonstrating")).toBe(false);
        expect(document.querySelector('[data-slot="introduction-demonstration-callout"]')).toBeNull();
      } finally {
        vi.useRealTimers();
      }
    });

    it("demonstrates all gestures on gallery steps without interactions regardless of completion indices", () => {
      vi.useFakeTimers();
      try {
        const gallerySteps: IntroductionStepDefinition[] = [
          {
            id: "gallery",
            title: "Gallery",
            body: "All gestures.",
            introduce: null,
            show: [],
            placement: "center",
            interactions: [],
            ordered: false,
            logos: [],
            demonstrations: [{ gesture: { kind: "leftClick", at: { kind: "element", id: "tool.fill" } } }, { gesture: { kind: "scroll", at: { kind: "element", id: "tool.fill" }, deltaY: -100 } }],
          },
        ];
        render(
          <div>
            <button id="tool.fill">Fill</button>
            <UIIntroduction introduction={{ title: "Welcome", steps: gallerySteps }} stepIndex={0} completedInteractionIndices={[0, 1, 2]} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />,
          </div>,
        );
        act(() => {
          vi.advanceTimersByTime(INTRODUCTION_DEMO_IDLE_THRESHOLD_MS);
        });
        expect(document.documentElement.getAttribute("data-introduction-demonstrating")).toBe("true");
      } finally {
        vi.useRealTimers();
      }
    });

    it("emphasizes the step count chip when hovered", () => {
      const steps: IntroductionStepDefinition[] = [{ id: "welcome", title: "Welcome", body: "Hello", introduce: null, show: [], placement: "center", interactions: [], ordered: false, logos: [], demonstrations: [] }];
      const { container } = render(<UIIntroduction introduction={{ title: "Welcome", steps }} stepIndex={0} onStepIndexChange={vi.fn()} onDismiss={vi.fn()} />);
      const stepChip = container.querySelector('[data-slot="introduction-step-chip"]');
      expect(stepChip).toBeTruthy();
      const stepText = stepChip?.querySelector("span");
      expect(stepText).toBeTruthy();
      expect(stepText?.className).toContain("group-hover:text-emphasized");
      expect(stepText?.className).toContain("transition-colors");
    });
  });

  describe("formatNumber", () => {
    it("strips IEEE-754 float artifacts for display", () => {
      expect(formatNumber(-2.5999999999999996)).toBe("-2.6");
      expect(formatNumber(0.1 + 0.2)).toBe("0.3");
      expect(formatNumber(42)).toBe("42");
      expect(formatNumber(1e-7)).toBe("1e-7");
      expect(formatNumber(NaN)).toBe("");
      expect(formatNumber(Infinity)).toBe("");
      expect(formatNumber("not-a-number")).toBe("not-a-number");
    });
  });

  describe("slider draft confirmation", () => {
    it("keeps pending draft values until the external value catches up", () => {
      expect(resolveSliderDraftClear([42], [10], 1)).toEqual([42]);
      expect(resolveSliderDraftClear([42], [42], 1)).toBeNull();
      expect(sliderValuesMatch([0.5], [0.51], 0.1)).toBe(true);
      expect(sliderValuesMatch([0.5], [0.8], 0.1)).toBe(false);
    });
  });

  describe("clampSliderValuesToReady", () => {
    it("passes values through unchanged when ready is not set", () => {
      expect(clampSliderValuesToReady([500], undefined, 0)).toEqual([500]);
    });

    it("clamps every value down to the ready extent", () => {
      expect(clampSliderValuesToReady([500], 40, 0)).toEqual([40]);
      expect(clampSliderValuesToReady([10, 500], 40, 0)).toEqual([10, 40]);
    });

    it("leaves values at or below ready untouched", () => {
      expect(clampSliderValuesToReady([40], 40, 0)).toEqual([40]);
      expect(clampSliderValuesToReady([5], 40, 0)).toEqual([5]);
    });

    it("floors the clamp ceiling at min, even when ready is below it", () => {
      expect(clampSliderValuesToReady([50], 0, 10)).toEqual([10]);
    });
  });

  describe("referenceMediaKindFromUrl", () => {
    it("infers image, svg, and pdf kinds from paths", () => {
      expect(referenceMediaKindFromUrl("/infinite-fixture/🖼️sketch.png")).toBe("image");
      expect(referenceMediaKindFromUrl("/infinite-fixture/icon.svg")).toBe("svg");
      expect(referenceMediaKindFromUrl("/infinite-fixture/🗺️site.pdf")).toBe("pdf");
      expect(referenceMediaKindFromUrl("/unknown.bin")).toBeNull();
    });
  });

  describe("iconShotFrame", () => {
    it("sizes the frame from fixed width and height", () => {
      const landscape = iconShotFrameStyle(512, 256);
      expect(landscape.aspectRatio).toBe("512 / 256");
      expect(landscape.width).toBe("100%");
      expect(landscape.height).toBe("auto");
      const portrait = iconShotFrameStyle(256, 512);
      expect(portrait.width).toBe("auto");
      expect(portrait.height).toBe("100%");
    });

    it("masks ellipse shots with a rounded frame", () => {
      expect(iconShotFrameClass("ellipse")).toBe("rounded-full");
      expect(iconShotFrameClass("rectangle")).toBe("rounded-none");
    });
  });

  describe("sunPositionFromAzimuthElevation", () => {
    it("places the sun on the azimuth/elevation sphere", () => {
      const [x, y, z] = sunPositionFromAzimuthElevation(0, 90, 100);
      expect(x).toBeCloseTo(0, 5);
      expect(y).toBeCloseTo(0, 5);
      expect(z).toBeCloseTo(100, 5);
      const [ex, ey, ez] = sunPositionFromAzimuthElevation(90, 0, 100);
      expect(ex).toBeCloseTo(0, 5);
      expect(ey).toBeCloseTo(100, 5);
      expect(ez).toBeCloseTo(0, 5);
    });
  });

  describe("UnifiedGumball math", () => {
    it("computes axis translate, plane hit, rotate angle, scale factor, and snapping", () => {
      expect(gumballRayAxisParameter([0, 0, 0], [0, 0, 1], [1, 2, 3], [1, 0, 0])).toBeCloseTo(1, 5);
      const eye: readonly [number, number, number] = [0, 0, 1];
      const axis: readonly [number, number, number] = [1, 0, 0];
      const pivot: readonly [number, number, number] = [0, 0, 0];
      const norm = (v: readonly [number, number, number]): readonly [number, number, number] => {
        const l = Math.hypot(v[0], v[1], v[2]);
        return [v[0] / l, v[1] / l, v[2] / l];
      };
      const leftParam = gumballProjectRayOntoAxis([0, 0, 10], norm([-0.2, 0, -1]), pivot, axis, eye);
      const rightParam = gumballProjectRayOntoAxis([0, 0, 10], norm([0.2, 0, -1]), pivot, axis, eye);
      expect(leftParam).not.toBeNull();
      expect(rightParam).not.toBeNull();
      expect(rightParam! > leftParam!).toBe(true);
      expect(gumballRayPlanePoint([0, 0, 0], [0, 0, 1], [0, 0, 0], [0, 0, 1])).toEqual([0, 0, 0]);
      expect(gumballAxisRotateAngle([1, 0, 0], [0, 1, 0], [0, 0, 1])).toBeCloseTo(Math.PI / 2, 5);
      expect(gumballAxisScaleFactor(2, 4)).toBe(2);
      expect(gumballSnapScalar(1.05, 0.5)).toBe(1);
      expect(gumballEffectiveSnapValue(undefined, false, GUMBALL_DEFAULT_SHIFT_ROTATION_SNAP)).toBeUndefined();
      expect(gumballEffectiveSnapValue(undefined, true, GUMBALL_DEFAULT_SHIFT_ROTATION_SNAP)).toBe(GUMBALL_DEFAULT_SHIFT_ROTATION_SNAP);
      expect(gumballEffectiveSnapValue(0.25, true, GUMBALL_DEFAULT_SHIFT_ROTATION_SNAP)).toBe(0.25);
      const shiftRotate = gumballResolveDragSnaps({}, true);
      expect(shiftRotate.rotationSnap).toBe(GUMBALL_DEFAULT_SHIFT_ROTATION_SNAP);
      expect(shiftRotate.scaleSnap).toBe(GUMBALL_DEFAULT_SHIFT_SCALE_SNAP);
      expect(gumballResolveDragSnaps({}, false).rotationSnap).toBeUndefined();
      expect(gumballSnapScalar(Math.PI / 3, Math.PI / 4)).toBeCloseTo(Math.PI / 4, 6);
      expect(gumballSnapScalar(Math.PI / 6, GUMBALL_DEFAULT_SHIFT_ROTATION_SNAP)).toBeCloseTo(Math.PI / 6, 6);
      expect(gumballHandleKindToTransformMode("moveYZ")).toBe("translate");
      expect(gumballHandleKindToTransformMode("rotateX")).toBe("rotate");
      expect(gumballHandleKindToTransformMode("scaleUniform")).toBe("scale");
      expect(gumballConfigVisible(DEFAULT_GUMBALL_CONFIG)).toBe(true);
      expect(gumballConfigVisible({ moveAxes: false, movePlanes: false, rotate: false, scaleAxes: false, scalePlanes: false, scaleUniform: false })).toBe(false);
      expect(gumballHandleAllowedByPlane("moveZ", undefined)).toBe(true);
      expect(gumballHandleAllowedByPlane("moveZ", "xy")).toBe(false);
      expect(gumballHandleAllowedByPlane("moveX", "xy")).toBe(true);
      expect(gumballHandleAllowedByPlane("rotateZ", "xy")).toBe(true);
      expect(gumballHandleAllowedByPlane("rotateX", "xy")).toBe(false);
      expect(gumballHandleAllowedByPlane("moveYZ", "yz")).toBe(true);
      expect(gumballHandleAllowedByPlane("rotateY", "xz")).toBe(true);
      const xyMove = resolveGumballConfig({ moveAxes: true, movePlanes: true, rotate: true, scaleAxes: false, scalePlanes: false, scaleUniform: false, plane: "xy" });
      expect(gumballHandleEnabled("moveX", xyMove)).toBe(true);
      expect(gumballHandleEnabled("moveZ", xyMove)).toBe(false);
      expect(gumballHandleEnabled("moveXY", xyMove)).toBe(true);
      expect(gumballHandleEnabled("moveYZ", xyMove)).toBe(false);
      expect(gumballHandleEnabled("rotateZ", xyMove)).toBe(true);
      expect(gumballHandleEnabled("rotateX", xyMove)).toBe(false);
      expect(gumballConfigVisible(xyMove)).toBe(true);
      expect(resolveGumballConfig({ plane: "yz" }).plane).toBe("yz");
      expect(gumballHandleKindToTransformMode("scaleXY")).toBe("scale");
      expect(gumballHandleVisualState("moveX", "moveY", null)).toBe("dimmed");
      expect(gumballHandleVisualState("moveX", "moveX", null)).toBe("hover");
      expect(gumballHandleVisualState("moveX", null, "moveX")).toBe("active");
      expect(gumballHandleVisualState("moveY", "moveX", "moveX")).toBe("dimmed");
      const palette = resolveGumballVisualPalette();
      expect(palette.axisX).toBe("#ff344f");
      expect(palette.axisY).toBe("#34d1bf");
      expect(palette.axisZ).toBe("#fa9500");
      expect(gumballResolveHandleVisual("#ff0000", "hover", palette).color).toBe("#ff0000");
      expect(gumballResolveHandleVisual("#ff0000", "hover", palette).opacity).toBe(palette.hoverOpacity);
      expect(gumballResolveHandleVisual("#ff0000", "active", palette).color).toBe("#ff0000");
      expect(gumballResolveHandleVisual("#ff0000", "active", palette).opacity).toBe(palette.activeOpacity);
      expect(gumballResolveHandleVisual("#ff0000", "idle", palette).color).toBe("#ff0000");
      expect(gumballResolveHandleVisual(palette.axisX, "hover", palette).color).toBe(palette.axisX);
      expect(gumballResolveHandleVisual(palette.axisY, "hover", palette).color).toBe(palette.axisY);
      expect(gumballResolveHandleVisual(palette.axisZ, "active", palette).color).toBe(palette.axisZ);
      const tintRoot = new THREE.Group();
      const visibleMesh = new THREE.Mesh(new THREE.BoxGeometry(), new THREE.MeshBasicMaterial({ color: "#000000" }));
      const pickMesh = new THREE.Mesh(new THREE.BoxGeometry(), new THREE.MeshBasicMaterial({ color: "#000000" }));
      pickMesh.visible = false;
      pickMesh.userData = { gumballHandlePick: true };
      tintRoot.add(visibleMesh, pickMesh);
      const tintMaterial = new THREE.MeshBasicMaterial({ color: palette.axisX, transparent: true, opacity: palette.hoverOpacity });
      gumballApplyHandleVisualMaterial(tintRoot, tintMaterial);
      expect(visibleMesh.material).toBe(tintMaterial);
      expect(pickMesh.material).not.toBe(tintMaterial);
      tintMaterial.dispose();
      visibleMesh.geometry.dispose();
      pickMesh.geometry.dispose();
      (visibleMesh.material as THREE.Material).dispose();
      (pickMesh.material as THREE.Material).dispose();
      expect(gumballScaleAxisOffset()).toBeGreaterThan(GUMBALL_RING_RADIUS);
      const perspective = new THREE.PerspectiveCamera(50, 1.6, 0.1, 1000);
      perspective.position.set(0, 0, 20);
      perspective.updateProjectionMatrix();
      const previewPivot = new THREE.Vector3(0, 0, 0);
      expect(gumballPreviewWorldExtent(perspective, previewPivot)).toBeGreaterThan(GUMBALL_PREVIEW_MIN_EXTENT);
      const ortho = new THREE.OrthographicCamera(-10, 10, 10, -10, 0.1, 1000);
      ortho.position.set(0, 0, 20);
      ortho.updateProjectionMatrix();
      expect(gumballPreviewWorldExtent(ortho, previewPivot)).toBeGreaterThan(9);
      expect(GUMBALL_PREVIEW_DISK_RADIUS).toBe(GUMBALL_RING_RADIUS);
      expect(GUMBALL_PREVIEW_DISK_RADIUS).toBeLessThan(GUMBALL_PREVIEW_RING_RADIUS);
      expect(gumballScalePlaneAxisIndices("scaleXY")).toEqual([0, 1]);
      expect(gumballScalePlaneAxisIndices("scaleYZ")).toEqual([1, 2]);
      expect(gumballScalePlaneAxisIndices("scaleXZ")).toEqual([0, 2]);
      expect(gumballPlaneScaleCorner("xy")[0]).toBeGreaterThan(GUMBALL_PLANE_OFFSET + GUMBALL_PLANE_SIZE * 0.5);
      expect(gumballPlaneScaleFactors([1, 2], [2, 4])).toEqual([2, 2]);
      expect(gumballPlaneScaleFactors([3, 4], [6, 8], true)).toEqual([2, 2]);
      expect(gumballPlaneScaleFactors([1, 0], [2, 0], true)).toEqual([2, 2]);
      const gumballMesh = new THREE.Mesh();
      gumballMesh.userData = { gumballHandleKind: "moveX" };
      const gumballChild = new THREE.Mesh();
      gumballMesh.add(gumballChild);
      expect(gumballKindFromRaycastObject(gumballMesh)).toBe("moveX");
      expect(gumballKindFromRaycastObject(gumballChild)).toBe("moveX");
      expect(gumballKindFromRaycastObject(null)).toBeNull();
      gumballPointerConsumesCanvasEventRef.current = true;
      gumballPointerConsumesCanvasEventRef.current = false;
      const gumballRoot = new THREE.Group();
      const gumballHandle = new THREE.Mesh(new THREE.BoxGeometry(2, 2, 2), new THREE.MeshBasicMaterial());
      gumballHandle.userData = { gumballHandleKind: "moveX" };
      gumballHandle.raycast = gumballHandleRaycast;
      gumballRoot.add(gumballHandle);
      const ownedCamera = new THREE.OrthographicCamera(-2, 2, 2, -2, 0.1, 100);
      ownedCamera.position.set(0, 0, 5);
      ownedCamera.lookAt(0, 0, 0);
      ownedCamera.updateMatrixWorld(true);
      const ownedCanvas = { getBoundingClientRect: () => ({ left: 0, top: 0, width: 200, height: 200, right: 200, bottom: 200, x: 0, y: 0, toJSON: () => ({}) }) } as HTMLElement;
      expect(gumballRaycastOwnedAtClientPoint(ownedCamera, ownedCanvas, 100, 100, gumballRoot)?.kind).toBe("moveX");
      expect(gumballRaycastOwnedAtClientPoint(ownedCamera, ownedCanvas, 10, 10, gumballRoot)).toBeNull();
      gumballHandle.geometry.dispose();
      (gumballHandle.material as THREE.Material).dispose();
      const biasedPickMesh = new THREE.Mesh(new THREE.BoxGeometry(), new THREE.MeshBasicMaterial());
      biasedPickMesh.raycast = gumballHandleRaycast;
      const raycaster = new THREE.Raycaster();
      raycaster.set(new THREE.Vector3(0, 0, 10), new THREE.Vector3(0, 0, -1));
      const biased: THREE.Intersection[] = [];
      biasedPickMesh.raycast(raycaster, biased);
      const plain: THREE.Intersection[] = [];
      THREE.Mesh.prototype.raycast.call(biasedPickMesh, raycaster, plain);
      expect(biased.length).toBeGreaterThan(0);
      expect(plain.length).toBeGreaterThan(0);
      expect(biased[0]!.distance).toBeLessThan(plain[0]!.distance);
      biasedPickMesh.geometry.dispose();
      (biasedPickMesh.material as THREE.Material).dispose();
    });

    it("builds parallel orthographic rays via isOrthographicCamera duck-typing (not instanceof)", () => {
      const ortho = new THREE.OrthographicCamera(-100, 100, 50, -50, 0.1, 1000);
      ortho.position.set(0, 0, 100);
      ortho.up.set(0, 1, 0);
      ortho.lookAt(0, 0, 0);
      ortho.updateMatrixWorld(true);
      ortho.updateProjectionMatrix();

      const center = gumballRayFromNdc(0, 0, ortho);
      const right = gumballRayFromNdc(0.5, 0, ortho);
      expect(center.dir[0]).toBeCloseTo(right.dir[0], 5);
      expect(center.dir[1]).toBeCloseTo(right.dir[1], 5);
      expect(center.dir[2]).toBeCloseTo(right.dir[2], 5);
      expect(Math.abs(right.origin[0] - center.origin[0])).toBeCloseTo(50, 5);

      const planeHit = (ray: { origin: GumballVec3; dir: GumballVec3 }): GumballVec3 => {
        const hit = gumballRayPlanePoint(ray.origin, ray.dir, [0, 0, 0], [0, 0, 1]);
        expect(hit).not.toBeNull();
        return hit!;
      };
      const centerHit = planeHit(center);
      const rightHit = planeHit(right);
      expect(rightHit[0] - centerHit[0]).toBeCloseTo(50, 5);
      expect(rightHit[1] - centerHit[1]).toBeCloseTo(0, 5);

      // 🦆️ Duplicate-three failure mode: plain object with the flag but not `instanceof OrthographicCamera`.
      const duck = {
        isOrthographicCamera: true,
        position: ortho.position,
        matrixWorld: ortho.matrixWorld,
        projectionMatrix: ortho.projectionMatrix,
        projectionMatrixInverse: ortho.projectionMatrixInverse,
        left: ortho.left,
        right: ortho.right,
        top: ortho.top,
        bottom: ortho.bottom,
        zoom: ortho.zoom,
      } as unknown as THREE.Camera;
      expect(duck instanceof THREE.OrthographicCamera).toBe(false);
      const duckCenter = gumballRayFromNdc(0, 0, duck);
      const duckRight = gumballRayFromNdc(0.5, 0, duck);
      expect(duckCenter.dir[0]).toBeCloseTo(duckRight.dir[0], 5);
      expect(duckCenter.dir[2]).toBeCloseTo(duckRight.dir[2], 5);
      expect(Math.abs(duckRight.origin[0] - duckCenter.origin[0])).toBeCloseTo(50, 5);
      expect(gumballPreviewWorldExtent(duck, new THREE.Vector3(0, 0, 0))).toBeGreaterThan(50);
    });
  });

  describe("iconSvgMarkup", () => {
    it("returns vendored svg markup for a known icon", () => {
      expect(iconSvgMarkup("component")).toContain("<svg");
    });
  });

  describe("iconCodec", () => {
    it("round-trips canonical icon strings", () => {
      const samples = ["url:https://example.com/icon.png", ":smile:", "data:image/png;base64,iVBORw0KGgo=", "emoji:☺️", "typst:$x^2$", "text:Hi", "pen-tool"];
      for (const sample of samples) {
        const icon = decodeIcon(sample);
        expect(icon).toBeTruthy();
        const encoded = encodeIcon(icon!);
        expect(decodeIcon(encoded)).toEqual(icon);
      }
    });

    it("decodes metabolism stems as themed and catalog ids as catalog", () => {
      expect(decodeIcon("capsule_J")).toEqual({ kind: "themed", key: "capsule_J" });
      expect(decodeIcon("pen-tool")).toEqual({ kind: "catalog", key: "pen-tool" });
      expect(decodeIcon("definitely-not-a-known-icon-stem")).toBeUndefined();
    });

    it("classifies selector modes for all kinds", () => {
      expect(classifyIconSelectorMode("url:https://x.test/a.png")).toBe("url");
      expect(classifyIconSelectorMode(":plus:")).toBe("shortcode");
      expect(shortcodeEmoji("grinning")).toBeTruthy();
      expect(shortcodeCatalogKey("plus")).toBe("plus");
      expect(classifyIconSelectorMode("text:Hi")).toBe("text");
      expect(classifyIconSelectorMode("capsule_J")).toBe("vector");
    });

    it("resolveCatalogIconSvg applies theme variants without changing icon id", () => {
      const themed = parseUiTheme({
        ...semioTheme(),
        icons: { variants: { search: "<svg data-theme-variant></svg>" } },
      });
      setActiveUiTheme(themed);
      expect(resolveCatalogIconSvg("search")).toContain("data-theme-variant");
      setActiveUiTheme(semioTheme());
    });
  });

  describe("icon concept assignments", () => {
    it("keeps every canonical concept on a unique catalog icon", () => {
      expect(() => assertUniqueIconConceptAssignments()).not.toThrow();
    });
  });

  describe("icon hover animations", () => {
    const FULL_TURN_KEYFRAMES = new Set(["icon-rotate-ccw", "icon-rotate-cw", "icon-loader-2", "icon-flip-horizontal", "icon-flip-vertical", "icon-settings", "icon-globe"]);
    const NON_CATALOG_KINDS = ["emoji", "text", "typst", "image", "svg", "node", "shortcode", "missing"];
    // Icons whose SVG has cleanly separable moving parts get a mechanism-accurate per-part animation
    // instead (--icon-animation: none plus one or more --icon-part-<N>-anim assignments); see 🔧️IconPartAnim.
    // Every icon below — and every whole-icon icon outside this set — has its own uniquely-named,
    // individually-tuned keyframes; nothing is shared across icons (see the specificity/exclusivity tests).
    // Physical-correctness pass moved hammer/box/move/arrow-up/arrow-down/arrow-left/arrow-right/undo/
    // undo-2/redo/redo-2 OUT of this set: each is one rigid object (hammer head+handle, arrow head+shaft,
    // the whole "move" cross, an undo/redo hook+curve), so it now animates as a single whole-icon rigid
    // motion instead of independently-moving parts. Remaining whole-icon icons either have a single fused
    // SVG path (no separable part exists without redrawing the vendored asset — e.g.
    // folder/folder-open/message-square/message-circle, handled with a best-effort single-rigid-body
    // motion instead) or their rigid motion already is the mechanically correct animation (globe, search,
    // settings, rotate-cw/ccw, flip-horizontal/vertical, text-cursor).
    const PART_MECHANISM_ICONS = new Set([
      "alert-circle",
      "align-left",
      "app-window",
      "arrow-right-left",
      "award",
      "bar-chart-3",
      "bell",
      "book-open",
      "calendar-days",
      "camera",
      "folder",
      "folder-open",
      "check-circle-2",
      "chevrons-up-down",
      "circle-dot",
      "clipboard",
      "clipboard-list",
      "clock",
      "code",
      "combine",
      "component",
      "copy",
      "crosshair",
      "cylinder",
      "download",
      "edit-3",
      "eraser",
      "external-link",
      "eye",
      "eye-off",
      "file",
      "file-archive",
      "file-code",
      "file-image",
      "file-json",
      "file-spreadsheet",
      "file-text",
      "file-type",
      "file-video",
      "focus",
      "git-branch",
      "git-commit",
      "git-merge",
      "graduation-cap",
      "grid-3x3",
      "grip-vertical",
      "hand",
      "hard-drive",
      "hash",
      "home",
      "image",
      "image-plus",
      "image-up",
      "info",
      "landmark",
      "lasso",
      "layers",
      "layout",
      "layout-grid",
      "library",
      "lightbulb",
      "link",
      "link-2-off",
      "list",
      "list-ordered",
      "list-tree",
      "lock",
      "lock-open",
      "magnet",
      "maximize-2",
      "minimize-2",
      "monitor",
      "more-horizontal",
      "mouse-pointer",
      "move-3d",
      "network",
      "paint-bucket",
      "paintbrush",
      "panel-left",
      "panel-right",
      "panel-top",
      "pause",
      "pen-tool",
      "pipette",
      "plug",
      "plus",
      "save",
      "scaling",
      "scissors",
      "settings-2",
      "shapes",
      "skip-back",
      "skip-forward",
      "smartphone",
      "smile",
      "sparkles",
      "square-arrow-down-left",
      "square-arrow-down-right",
      "square-arrow-up-left",
      "square-arrow-up-right",
      "square-dashed",
      "sun",
      "table-2",
      "tablet",
      "tags",
      "text-search",
      "trash-2",
      "triangle-alert",
      "unlink",
      "user",
      "users",
      "x",
      "zoom-in",
      "zoom-out",
    ]);

    async function readUiCss(): Promise<string> {
      const { readFileSync } = await import("node:fs");
      const { fileURLToPath } = await import("node:url");
      const { dirname, resolve } = await import("node:path");
      return readFileSync(resolve(dirname(fileURLToPath(source.url)), "../../../../🎨️styling/🖌️ui.css"), "utf8");
    }

    it("gives every vendored icon id and non-catalog kind a hover keyframes block and animation assignment", async () => {
      const css = await readUiCss();
      for (const name of Object.keys(ICONS as Record<string, string>)) {
        if (PART_MECHANISM_ICONS.has(name)) {
          const ruleMatch = css.match(new RegExp(`\\[data-icon="${name}"\\] \\{([^}]*)\\}`));
          expect(ruleMatch, `missing [data-icon="${name}"] rule`).toBeTruthy();
          const rule = ruleMatch![1];
          expect(rule).toMatch(/--icon-animation:\s*none;/);
          const partAnims = [...rule.matchAll(/--icon-part-(\d+)-anim:\s*([\w-]+);/g)];
          expect(partAnims.length, `${name} should assign at least one --icon-part-N-anim`).toBeGreaterThan(0);
          for (const [, , kfName] of partAnims) {
            expect(css, `missing @keyframes ${kfName} referenced by ${name}`).toContain(`@keyframes ${kfName} {`);
          }
          continue;
        }
        // Whole-icon animations are named icon-<id> or, when a descriptive suffix reads better
        // (e.g. icon-folder-flap), icon-<id>-<suffix> — same convention as per-part icons.
        const ruleMatch = css.match(new RegExp(`\\[data-icon="${name}"\\] \\{([^}]*)\\}`));
        expect(ruleMatch, `missing [data-icon="${name}"] rule`).toBeTruthy();
        const animMatch = ruleMatch![1].match(/--icon-animation:\s*(icon-[\w-]+);/);
        expect(animMatch, `${name} has no --icon-animation assignment`).toBeTruthy();
        const animName = animMatch![1];
        expect(animName === `icon-${name}` || animName.startsWith(`icon-${name}-`), `${name} -> ${animName} does not match icon-${name}(-suffix)?`).toBe(true);
        expect(css, `missing @keyframes ${animName}`).toContain(`@keyframes ${animName} {`);
      }
      for (const kind of NON_CATALOG_KINDS) {
        expect(css).toContain(`@keyframes icon-kind-${kind} {`);
        expect(css).toMatch(new RegExp(`\\[data-icon-kind="${kind}"\\][^\\n]*--icon-animation:\\s*icon-kind-${kind};`));
      }
    });

    it("keeps every icon keyframes block closed (identical 0% and 100% frames), except declared full-turn spins", async () => {
      const css = await readUiCss();
      const blockRe = /@keyframes\s+(icon-[\w-]+)\s*\{([\s\S]*?)\n\}/g;
      let match: RegExpExecArray | null;
      let checked = 0;
      while ((match = blockRe.exec(css))) {
        const [, name, body] = match;
        checked++;
        if (FULL_TURN_KEYFRAMES.has(name)) {
          expect(body).toMatch(/0%\s*\{\s*transform:\s*(?:rotate|rotateX|rotateY)\(0deg\);/);
          expect(body).toMatch(/100%\s*\{\s*transform:\s*(?:rotate|rotateX|rotateY)\(-?360deg\);/);
          continue;
        }
        expect(body).toMatch(/0%,\s*\n\s*100%\s*\{/);
      }
      // Every icon has its own dedicated keyframes now, so this count roughly tracks icon count; kept as a
      // loose sanity floor (not exact) against an empty/broken scan rather than a precise assertion.
      expect(checked).toBeGreaterThan(100);
    });

    // Keyed by the required animation-name prefix ("icon-<id>" for data-icon, "icon-kind-<kind>" for
    // data-icon-kind), not the raw selector value, so specificity/exclusivity checks apply the right rule.
    function collectIconRules(css: string): Map<string, string> {
      const rules = new Map<string, string>();
      for (const m of css.matchAll(/\[data-icon="([a-z0-9-]+)"\] \{ ([^}]*)\}/g)) {
        rules.set(`icon-${m[1]}`, m[2]);
      }
      for (const m of css.matchAll(/\[data-icon-kind="([a-z0-9-]+)"\] \{ ([^}]*)\}/g)) {
        rules.set(`icon-kind-${m[1]}`, m[2]);
      }
      return rules;
    }

    function collectAnimNames(ruleBody: string): string[] {
      return [...ruleBody.matchAll(/--icon-(?:part-\d+-)?anim(?:ation)?:\s*([\w-]+);/g)].map((m) => m[1]).filter((n) => n !== "none");
    }

    it("names every icon's animation after that exact icon — no borrowed/generic names", async () => {
      const css = await readUiCss();
      const rules = collectIconRules(css);
      const violations: string[] = [];
      for (const [prefix, body] of rules) {
        for (const name of collectAnimNames(body)) {
          if (name !== prefix && !name.startsWith(`${prefix}-`)) {
            violations.push(`${prefix} -> ${name}`);
          }
        }
      }
      expect(violations, `every animation name must start with icon-<id>: ${violations.join(", ")}`).toEqual([]);
    });

    it("never reuses one icon's animation for a different icon (fully handcrafted, not shared)", async () => {
      const css = await readUiCss();
      const rules = collectIconRules(css);
      const owners = new Map<string, Set<string>>();
      for (const [id, body] of rules) {
        for (const name of collectAnimNames(body)) {
          if (!owners.has(name)) owners.set(name, new Set());
          owners.get(name)!.add(id);
        }
      }
      const shared = [...owners.entries()].filter(([, ids]) => ids.size > 1);
      expect(shared, `these animations are referenced by more than one icon: ${shared.map(([n, ids]) => `${n} -> [${[...ids].join(", ")}]`).join("; ")}`).toEqual([]);
    });

    it("has no orphaned keyframes — every defined icon-* animation is used by exactly one icon rule", async () => {
      const css = await readUiCss();
      const rules = collectIconRules(css);
      const referenced = new Set<string>();
      for (const [, body] of rules) {
        for (const name of collectAnimNames(body)) referenced.add(name);
      }
      const defined = [...css.matchAll(/^@keyframes (icon-[a-z0-9-]+)/gm)].map((m) => m[1]).filter((n) => !n.startsWith("icon-kind-"));
      const dead = defined.filter((n) => !referenced.has(n));
      expect(dead, `dead keyframes with no icon referencing them: ${dead.join(", ")}`).toEqual([]);
    });
  });

  describe("Icon hover animation attributes", () => {
    it("stamps data-icon and data-icon-kind for catalog icons", () => {
      const markup = renderToStaticMarkup(<Icon icon="bell" />);
      expect(markup).toMatch(/data-icon="bell"/);
      expect(markup).toMatch(/data-icon-kind="catalog"/);
      expect(markup).toMatch(/--icon-mask:url\(&quot;data:image\/svg\+xml,/);
    });

    it("stamps data-icon-kind for every non-catalog icon kind", () => {
      expect(renderToStaticMarkup(<Icon icon={{ kind: "emoji", emoji: "🙂️" }} />)).toMatch(/data-icon-kind="emoji"/);
      expect(renderToStaticMarkup(<Icon icon={{ kind: "emoji", emoji: "🙂️" }} />)).not.toMatch(/--icon-mask:/);
      expect(renderToStaticMarkup(<Icon icon={{ kind: "text", text: "Hi" }} />)).toMatch(/data-icon-kind="text"/);
      expect(renderToStaticMarkup(<Icon icon={{ kind: "text", text: "Hi" }} />)).not.toMatch(/--icon-mask:/);
      expect(renderToStaticMarkup(<Icon icon={{ kind: "typst", src: "$x^2$" }} />)).toMatch(/data-icon-kind="typst"/);
      expect(renderToStaticMarkup(<Icon icon={{ kind: "url", url: "https://example.com/a.png" }} />)).toMatch(/data-icon-kind="image"/);
      expect(renderToStaticMarkup(<Icon icon={{ kind: "svg", svg: "<svg></svg>" }} />)).toMatch(/data-icon-kind="svg"/);
      expect(renderToStaticMarkup(<Icon icon={{ kind: "node", node: <span>x</span> }} />)).toMatch(/data-icon-kind="node"/);
      expect(renderToStaticMarkup(<Icon icon={"definitely-not-a-vendored-icon" as IconName} />)).toMatch(/data-icon-kind="text"/);
    });
  });

  describe("Level Context", () => {
    it("LEVELS is ordered base..menu", () => {
      expect(LEVELS).toEqual(["base", "window", "pane", "panel", "dialog", "menu"]);
    });

    it("useLevel resolves the nearest LevelProvider when nested, defaulting to base with none", () => {
      const ProbeLevel: React.FC = () => {
        const level = useLevel();
        return <span data-testid="probe-level" data-level={level} />;
      };
      const { container, rerender } = render(<ProbeLevel />);
      expect(container.querySelector('[data-testid="probe-level"]')?.getAttribute("data-level")).toBe("base");

      rerender(
        <LevelProvider level="window">
          <LevelProvider level="dialog">
            <ProbeLevel />
          </LevelProvider>
        </LevelProvider>,
      );
      expect(container.querySelector('[data-testid="probe-level"]')?.getAttribute("data-level")).toBe("dialog");
    });

    it("a transient (Popover) always resolves menu level on its own portaled content root, regardless of the ambient LevelProvider", () => {
      render(
        <LevelProvider level="dialog">
          <Popover open>
            <PopoverTrigger>Open</PopoverTrigger>
            <PopoverContent>Body</PopoverContent>
          </Popover>
        </LevelProvider>,
      );
      const content = document.querySelector('[data-slot="popover-content"]');
      expect(content?.getAttribute("data-level")).toBe("menu");
      expect(content?.className).toContain("z-menu");
      expect(content?.className).toContain("ui-glass");
      expect(content?.className).not.toContain("ui-glass-panel");
      expect(content?.className).not.toContain("ui-glass-menu");
    });

    it("WindowChrome with level=dialog stamps its level, keeps payload transparent, and paints chip glass", () => {
      const { container } = render(<WindowChrome level="dialog" stackSlot="level-dialog-stack" titleChips={<span>Title</span>} body={<div data-testid="dialog-body">Body</div>} />);
      const stack = container.querySelector('[data-slot="level-dialog-stack"]') as HTMLElement;
      expect(stack.getAttribute("data-level")).toBe("dialog");
      const body = container.querySelector('[data-slot="window-chrome-body"]') as HTMLElement;
      expect(body.hasAttribute("data-window-silhouette-content")).toBe(true);
      expect(body.className).not.toContain("ui-glass");
      const chip = container.querySelector('[data-slot="window-chrome-chip-cap"]') as HTMLElement;
      expect(chip.className).toContain("ui-glass");
      expect(screen.getByTestId("dialog-body")).toBeTruthy();
    });

    // 🪜️ D1 regression: `ui-surface`/`ui-glass`/`ui-veil` are the only per-level fills — cn() must
    // merge them like any other bg-color utility (mutually exclusive, last-in-cn wins, both
    // directions), never silently keeping a stray `bg-transparent`/`bg-active-base` alongside one.
    it("cn() treats ui-surface/ui-glass/ui-veil as bg-color-group utilities: last one wins, both directions", () => {
      expect(cn("bg-transparent", glassClass)).toBe(glassClass);
      expect(cn(glassClass, "bg-transparent")).toBe("bg-transparent");
      expect(cn(surfaceClass, "bg-active-base")).toBe("bg-active-base");
      expect(cn("bg-active-base", surfaceClass)).toBe(surfaceClass);
      expect(cn(surfaceClass, glassClass)).toBe(glassClass);
      // modifier'd utilities (hover:/data-[...]:) are a distinct conflict key — never dropped by a fill
      expect(cn(surfaceClass, "hover:bg-hover-interactive-fill")).toBe(`${surfaceClass} hover:bg-hover-interactive-fill`);
    });

    // 🪟️ D2 regression: the introduction/context-menu title chip must never re-paint an opaque fill
    // or rectangular border over the already-painted chip-cap cell it sits inside.
    it("windowChromeTitleChipClass never carries an opaque fill or piecewise border of its own", () => {
      expect(windowChromeTitleChipClass).not.toContain("ui-surface");
      expect(windowChromeTitleChipClass).toContain("bg-transparent");
      expect(windowChromeTitleChipClass).toContain("border-0");
      expect(windowChromeTitleChipClass).not.toMatch(/(?:^|\s)border(?:\s|$)/);
      expect(windowChromeTitleChipClass).not.toContain(borderNormalClass);
    });

    it("useSurface resolves the nearest Surface/SurfaceScope, or null outside any", () => {
      const ProbeSurface: React.FC = () => {
        const surface = useSurface();
        return <span data-testid="probe-surface" data-level={surface?.level ?? "none"} data-fill={surface?.fill ?? "none"} />;
      };
      const { container: outside } = render(<ProbeSurface />);
      expect(outside.querySelector('[data-testid="probe-surface"]')?.getAttribute("data-level")).toBe("none");

      const { container: inside } = render(
        <Surface level="panel" fill="glass">
          <ProbeSurface />
        </Surface>,
      );
      const probe = inside.querySelector('[data-testid="probe-surface"]');
      expect(probe?.getAttribute("data-level")).toBe("panel");
      expect(probe?.getAttribute("data-fill")).toBe("glass");
    });

    it("Surface warns (dev-only) when nested inside an ancestor Surface painting the same level, never when levels differ or fill is none", () => {
      const warnSpy = vi.spyOn(console, "warn").mockImplementation(() => undefined);
      render(
        <Surface level="panel" fill="glass">
          <Surface level="panel" fill="surface">
            <span />
          </Surface>
        </Surface>,
      );
      expect(warnSpy).toHaveBeenCalledTimes(1);
      warnSpy.mockClear();

      render(
        <Surface level="panel" fill="glass">
          <Surface level="pane" fill="glass">
            <span />
          </Surface>
        </Surface>,
      );
      expect(warnSpy).not.toHaveBeenCalled();

      render(
        <Surface level="panel" fill="glass">
          <Surface level="panel" fill="none">
            <span />
          </Surface>
        </Surface>,
      );
      expect(warnSpy).not.toHaveBeenCalled();
      warnSpy.mockRestore();
    });

    it("shellFloorPaints/shellFloorFillClass defer opaque fill only on an already-painted base floor", () => {
      expect(shellFloorPaints(null)).toBe(true);
      expect(shellFloorFillClass(null)).toBe(surfaceClass);
      expect(shellFloorPaints({ level: "base", fill: "surface" })).toBe(false);
      expect(shellFloorFillClass({ level: "base", fill: "surface" })).toBe("bg-transparent");
      expect(shellFloorPaints({ level: "base", fill: "none" })).toBe(true);
      expect(shellFloorPaints({ level: "window", fill: "surface" })).toBe(true);
    });

    it("Layout paints one continuous base floor; navbar, footer, canvas, and mode-body stay transparent over it", () => {
      const { container } = render(
        <Layout
          navbar={<Navbar items={[{ key: "n", content: "Nav" }]} showFullscreenToggle={false} />}
          footer={<Footer items={[{ key: "f", content: "Foot" }]} />}
          canvas={<Mode windows={[{ id: "w", title: uiDataLabel("W"), iconId: "app-window", children: <div>Body</div> }]} activeWindowId="w" onActiveWindowChange={() => {}} />}
        />,
      );
      const layout = container.querySelector('[data-slot="layout"]');
      expect(layout?.getAttribute("data-level")).toBe("base");
      expect(layout?.className).toContain("ui-surface");
      const navbar = container.querySelector('[data-slot="navbar"]');
      expect(navbar?.getAttribute("data-level")).toBe("base");
      expect(navbar?.className).toContain("bg-transparent");
      expect(navbar?.className).not.toContain("ui-surface");
      const footer = container.querySelector('[data-slot="footer"]');
      expect(footer?.getAttribute("data-level")).toBe("base");
      expect(footer?.className).toContain("bg-transparent");
      expect(footer?.className).not.toContain("ui-surface");
      const modeBody = container.querySelector('[data-slot="mode-body"]');
      expect(modeBody?.getAttribute("data-level")).toBe("base");
      expect(modeBody?.className).toContain("bg-transparent");
      expect(modeBody?.className).not.toContain("ui-surface");
    });

    it("Navbar, Footer, Canvas, and Mode-body paint ui-surface at base when used standalone outside Layout", () => {
      expect(renderToStaticMarkup(<Navbar items={[{ key: "n", content: "Nav" }]} showFullscreenToggle={false} />)).toContain("ui-surface");
      expect(renderToStaticMarkup(<Footer items={[{ key: "f", content: "Foot" }]} />)).toContain("ui-surface");
      expect(renderToStaticMarkup(<Canvas>c</Canvas>)).toContain("ui-surface");
      const { container } = render(<Mode windows={[{ id: "w", title: uiDataLabel("W"), iconId: "app-window", children: <div>Body</div> }]} activeWindowId="w" onActiveWindowChange={() => {}} />);
      expect(container.querySelector('[data-slot="mode-body"]')?.className).toContain("ui-surface");
    });
  });

  describe("ContextMenu", () => {
    it("prevents the native context menu when no items are registered", () => {
      render(
        <ContextMenu title={uiDataLabel("Menu")}>
          <button type="button">Target</button>
        </ContextMenu>,
      );
      const target = screen.getByRole("button", { name: "Target" });
      const event = new MouseEvent("contextmenu", { bubbles: true, cancelable: true });
      const preventDefaultSpy = vi.spyOn(event, "preventDefault");
      target.dispatchEvent(event);
      expect(preventDefaultSpy).toHaveBeenCalled();
      expect(screen.queryByRole("menu")).toBeNull();
    });

    it("buildTextSelectionContextMenuItems exposes copy with a mod shortcut and gates cut on selection", () => {
      const cut = vi.fn();
      const copy = vi.fn();
      const paste = vi.fn();
      const selectAll = vi.fn();
      const items = buildTextSelectionContextMenuItems(
        { editable: true, hasSelection: false },
        { cut: uiDataLabel("Cut"), copy: uiDataLabel("Copy"), paste: uiDataLabel("Paste"), selectAll: uiDataLabel("Select All") },
        { cut, copy, paste, selectAll },
      );
      const copyItem = items.find((item) => item.id === "text-copy");
      const cutItem = items.find((item) => item.id === "text-cut");
      expect(copyItem?.shortcut === "⌘️C" || copyItem?.shortcut === "Ctrl+C").toBe(true);
      expect(cutItem?.disabled).toBe(true);
      copyItem?.onSelect?.(new Event("select"));
      expect(copy).toHaveBeenCalled();
    });

    it("isPointerEventOnDomTextSelection is true only for nodes inside the live selection", () => {
      const host = document.createElement("div");
      host.textContent = "Selectable label";
      document.body.appendChild(host);
      const outside = document.createElement("div");
      outside.textContent = "Outside";
      document.body.appendChild(outside);
      const range = document.createRange();
      range.selectNodeContents(host);
      const selection = window.getSelection();
      selection?.removeAllRanges();
      selection?.addRange(range);
      expect(readDomTextSelection()).toBe("Selectable label");
      expect(isPointerEventOnDomTextSelection(host.firstChild)).toBe(true);
      expect(isPointerEventOnDomTextSelection(outside)).toBe(false);
      selection?.removeAllRanges();
      host.remove();
      outside.remove();
    });

    it("TextSelectionContextMenuHost opens copy on right-click over selected text", async () => {
      const writeText = vi.fn().mockResolvedValue(undefined);
      Object.defineProperty(navigator, "clipboard", { configurable: true, value: { writeText, readText: vi.fn() } });
      render(
        <>
          <TextSelectionContextMenuHost />
          <span data-testid="selectable">Copy me please</span>
        </>,
      );
      const node = screen.getByTestId("selectable");
      const range = document.createRange();
      range.selectNodeContents(node);
      const selection = window.getSelection();
      selection?.removeAllRanges();
      selection?.addRange(range);
      fireEvent.contextMenu(node, { clientX: 40, clientY: 60 });
      const copyItem = await waitFor(() => screen.getByRole("menuitem", { name: "Copy" }));
      expect(copyItem).toBeTruthy();
      expect(screen.getByRole("menuitem", { name: "Select all" })).toBeTruthy();
      expect(screen.queryByRole("menuitem", { name: "Cut" })).toBeNull();
      fireEvent.click(copyItem);
      await waitFor(() => expect(writeText).toHaveBeenCalledWith("Copy me please"));
      selection?.removeAllRanges();
    });

    it("opens the custom menu when items are registered", async () => {
      render(
        <ContextMenu title={uiDataLabel("Menu")} items={[{ id: "demo", label: uiDataLabel("Demo action") }]}>
          <button type="button">Target</button>
        </ContextMenu>,
      );
      fireEvent.contextMenu(screen.getByRole("button", { name: "Target" }));
      await waitFor(() => {
        expect(screen.getByRole("menuitem", { name: "Demo action" })).toBeTruthy();
      });
    });

    it("anchors the menu title chip bottom-left at the pointer coordinates", async () => {
      render(
        <ContextMenu title={uiDataLabel("Menu")} items={[{ id: "demo", label: uiDataLabel("Demo action") }]}>
          <button type="button">Target</button>
        </ContextMenu>,
      );
      fireEvent.contextMenu(screen.getByRole("button", { name: "Target" }), { clientX: 123, clientY: 87 });
      const menu = await waitFor(() => screen.getByRole("menu"));
      const chrome = menu.closest<HTMLElement>('[data-window-silhouette][data-slot="context-menu-content"]');
      expect(chrome?.parentElement).toBe(document.body);
      expect(chrome?.style.position).toBe("fixed");
      expect(chrome?.style.left).toBe("123px");
      expect(chrome?.style.top).toBe("calc(87px - var(--size-medium))");
      expect(document.querySelector('[data-slot="context-menu-title-chip"] [data-icon="list"]')).toBeTruthy();
    });

    it("wraps context menu rows in window chrome with a title chip and no enlarge control", async () => {
      render(
        <ContextMenu items={[{ id: "demo", label: uiDataLabel("Demo action") }]} title={uiDataLabel("Actions")}>
          <button type="button">Target</button>
        </ContextMenu>,
      );
      fireEvent.contextMenu(screen.getByRole("button", { name: "Target" }));
      await waitFor(() => {
        expect(document.querySelector('[data-slot="context-menu-content"]')).toBeTruthy();
        expect(document.querySelector('[data-slot="context-menu-title-chip"]')?.textContent).toContain("Actions");
        expect(document.querySelector('[data-slot="context-menu-title-chip"] [data-icon="list"]')).toBeTruthy();
        expect(document.querySelector('[data-slot="window-chrome-silhouette-border"]')).toBeTruthy();
        expect(document.querySelector('[data-slot="mode-dock-maximize"]')).toBeNull();
        expect(document.querySelector('[data-slot="window-chrome-controls"]')).toBeNull();
      });
    });

    it("keeps the U-gap transparent so the cutout shows the background instead of a filled rectangle", async () => {
      const { readFileSync } = await import("node:fs");
      const { fileURLToPath } = await import("node:url");
      const { dirname, resolve } = await import("node:path");
      const css = readFileSync(resolve(dirname(fileURLToPath(source.url)), "../../../../🎨️styling/🖌️ui.css"), "utf8");
      expect(css).toContain("[data-window-silhouette-gap]");
      expect(css).toMatch(/\[data-window-silhouette-gap\][\s\S]*backdrop-filter:\s*none/);
      render(
        <ContextMenu items={[{ id: "demo", label: uiDataLabel("Demo action") }]} title={uiDataLabel("Actions")}>
          <button type="button">Target</button>
        </ContextMenu>,
      );
      fireEvent.contextMenu(screen.getByRole("button", { name: "Target" }));
      await waitFor(() => {
        const stack = document.querySelector('[data-slot="context-menu-content"]') as HTMLElement;
        const gap = stack?.querySelector('[data-slot="window-chrome-gap"]') as HTMLElement;
        const chip = stack?.querySelector('[data-slot="window-chrome-chip-cap"]') as HTMLElement;
        expect(gap).toBeTruthy();
        expect(gap.className).toContain("bg-transparent");
        expect(gap.className).not.toContain("ui-glass");
        expect(stack.querySelector('[class*="ui-glass"][class*="absolute"][class*="inset-0"]')).toBeNull();
        expect(chip?.className).toContain("ui-glass");
        // 🪜️ D2 regression: the interior title chip must be transparent, not a second opaque fill.
        const titleChip = stack?.querySelector('[data-slot="context-menu-title-chip"]') as HTMLElement;
        expect(titleChip?.className).not.toContain("ui-surface");
      });
    });

    it("renders catalog and shortcode menu icons instead of raw labels", async () => {
      render(
        <ContextMenu
          title={uiDataLabel("Menu")}
          items={[
            { id: "delete", label: uiDataLabel("Delete"), icon: "trash" },
            { id: "suggest", label: uiDataLabel("Suggest"), icon: "sparkles" },
            { id: "copy", label: uiDataLabel("Copy"), icon: "copy" },
          ]}
        >
          <button type="button">Target</button>
        </ContextMenu>,
      );
      fireEvent.contextMenu(screen.getByRole("button", { name: "Target" }));
      await waitFor(() => {
        expect(screen.getByRole("menuitem", { name: "Delete" }).querySelector("[data-icon='trash-2'] svg")).toBeTruthy();
        expect(screen.getByRole("menuitem", { name: "Suggest" }).querySelector("[data-icon='sparkles'] svg")).toBeTruthy();
        expect(screen.getByRole("menuitem", { name: "Copy" }).querySelector("[data-icon='copy'] svg")).toBeTruthy();
      });
      expect(screen.queryByText("trash")).toBeNull();
      expect(screen.queryByText("sparkles")).toBeNull();
    });

    it("renders a color swatch and fires hover callbacks on controller items", async () => {
      const onHover = vi.fn();
      const onHoverEnd = vi.fn();
      render(<ContextMenuController title={uiDataLabel("Menu")} open position={{ x: 12, y: 24 }} items={[{ id: "kind", label: uiDataLabel("Capsule"), icon: "box", color: "#aabbcc", onHover, onHoverEnd }]} onOpenChange={vi.fn()} />);
      const item = await waitFor(() => screen.getByRole("menuitem", { name: "Capsule" }));
      const swatch = Array.from(item.querySelectorAll("[aria-hidden]")).find((node) => {
        const style = (node as HTMLElement).getAttribute("style") ?? "";
        return style.includes("background");
      }) as HTMLElement | undefined;
      expect(swatch?.getAttribute("style") ?? "").toMatch(/#aabbcc|rgb\(170,\s*187,\s*204\)/);
      fireEvent.pointerEnter(item);
      expect(onHover).toHaveBeenCalled();
      fireEvent.pointerLeave(item);
      expect(onHoverEnd).toHaveBeenCalled();
    });

    it("highlights checked items without a tick or checkmark", async () => {
      render(
        <ContextMenuController
          title={uiDataLabel("Menu")}
          open
          position={{ x: 8, y: 16 }}
          items={[
            { id: "a", label: uiDataLabel("Capsule · port"), icon: "box", color: "#112233", checked: true },
            { id: "b", label: uiDataLabel("Box · port"), icon: "box", color: "#445566", checked: false },
          ]}
          onOpenChange={vi.fn()}
        />,
      );
      const active = await waitFor(() => screen.getByRole("menuitemcheckbox", { name: "Capsule · port" }));
      const idle = screen.getByRole("menuitemcheckbox", { name: "Box · port" });
      expect(active.getAttribute("aria-checked")).toBe("true");
      expect(active.getAttribute("data-selected")).toBe("true");
      expect(active.className.split(/\s+/)).toContain("bg-active-base");
      expect(active.className.split(/\s+/)).toContain("hover:bg-active-base/90");
      expect(active.textContent).not.toContain("✓️");
      expect(idle.getAttribute("aria-checked")).toBe("false");
      expect(idle.getAttribute("data-selected")).toBeNull();
      expect(idle.className.split(/\s+/)).not.toContain("bg-active-base");
      expect(idle.textContent).not.toContain("✓️");
    });

    it("omits the color swatch when suggestion-style rows have icon only", async () => {
      render(
        <ContextMenuController
          title={uiDataLabel("Menu")}
          open
          position={{ x: 8, y: 16 }}
          items={[
            { id: "suggestion-0", label: uiDataLabel("Capsule · vortex 0"), icon: "box", checked: true },
            { id: "suggestion-1", label: uiDataLabel("Box · vortex 1"), icon: "box", checked: false },
          ]}
          onOpenChange={vi.fn()}
        />,
      );
      const active = await waitFor(() => screen.getByRole("menuitemcheckbox", { name: "Capsule · vortex 0" }));
      const idle = screen.getByRole("menuitemcheckbox", { name: "Box · vortex 1" });
      const swatch = (item: HTMLElement) =>
        Array.from(item.querySelectorAll("[aria-hidden]")).find((node) => {
          const style = (node as HTMLElement).getAttribute("style") ?? "";
          return style.includes("background") || /#|rgb\(/i.test(style);
        });
      expect(swatch(active)).toBeUndefined();
      expect(swatch(idle)).toBeUndefined();
      expect(active.querySelector("[data-icon]")).toBeTruthy();
      expect(active.className.split(/\s+/)).toContain("gap-single");
      expect(active.className.split(/\s+/)).toContain("px-single");
      expect(active.className.split(/\s+/)).toContain("py-half");
    });

    it("ignores outside dismiss when pointerdown targets a sibling menu", async () => {
      const onOpenChange = vi.fn();
      render(
        <>
          <ContextMenuController title={uiDataLabel("Menu")} open position={{ x: 10, y: 10 }} items={[{ id: "a", label: uiDataLabel("Alpha"), onSelect: vi.fn() }]} onOpenChange={onOpenChange} />
          <div role="menu" data-testid="sibling-menu">
            <button type="button">Sibling</button>
          </div>
        </>,
      );
      await waitFor(() => screen.getByRole("menuitem", { name: "Alpha" }));
      await new Promise((resolve) => setTimeout(resolve, 0));
      fireEvent.pointerDown(screen.getByTestId("sibling-menu"));
      expect(onOpenChange).not.toHaveBeenCalled();
      fireEvent.pointerDown(document.body);
      expect(onOpenChange).toHaveBeenCalledWith(false);
    });

    it("previews numbered rows on digit keys and accepts the active row on Enter", async () => {
      const onHover = vi.fn();
      const onSelect = vi.fn();
      const onOpenChange = vi.fn();
      render(
        <ContextMenuController
          title={uiDataLabel("Menu")}
          open
          closeOnSelect={false}
          position={{ x: 8, y: 16 }}
          items={[
            { id: "suggestion-0", label: uiDataLabel("Capsule · port"), icon: "box", checked: true, onSelect, onHover },
            { id: "suggestion-1", label: uiDataLabel("Box · port"), icon: "box", checked: false, onSelect, onHover },
          ]}
          onOpenChange={onOpenChange}
        />,
      );
      await waitFor(() => screen.getByRole("menuitemcheckbox", { name: "Capsule · port" }));
      fireEvent.keyDown(window, { key: "2" });
      expect(onHover).toHaveBeenCalledTimes(1);
      expect(onSelect).not.toHaveBeenCalled();
      fireEvent.keyDown(window, { key: "Enter" });
      expect(onSelect).toHaveBeenCalledTimes(1);
      expect(onOpenChange).not.toHaveBeenCalled();
    });

    it("renders ordinal badges for the first nine enabled rows", async () => {
      render(
        <ContextMenuController
          title={uiDataLabel("Menu")}
          open
          position={{ x: 4, y: 4 }}
          items={[
            { id: "a", label: uiDataLabel("Alpha") },
            { id: "b", label: uiDataLabel("Beta") },
            { id: "sep", label: uiDataLabel(""), separator: true },
            { id: "c", label: uiDataLabel("Gamma") },
          ]}
          onOpenChange={vi.fn()}
        />,
      );
      const alpha = await waitFor(() => screen.getByRole("menuitem", { name: "Alpha" }));
      expect(alpha.textContent?.trim().startsWith("1")).toBe(true);
      expect(screen.getByRole("menuitem", { name: "Beta" }).textContent?.trim().startsWith("2")).toBe(true);
      expect(screen.getByRole("menuitem", { name: "Gamma" }).textContent?.trim().startsWith("3")).toBe(true);
    });

    it("moves the active row with arrow keys and wasd", async () => {
      const onHover = vi.fn();
      const onHoverEnd = vi.fn();
      render(
        <ContextMenuController
          title={uiDataLabel("Menu")}
          open
          position={{ x: 4, y: 4 }}
          items={[
            { id: "a", label: uiDataLabel("Alpha"), onHover, onHoverEnd },
            { id: "b", label: uiDataLabel("Beta"), onHover, onHoverEnd },
          ]}
          onOpenChange={vi.fn()}
        />,
      );
      await waitFor(() => screen.getByRole("menuitem", { name: /Alpha/ }));
      fireEvent.keyDown(window, { key: "ArrowDown" });
      expect(onHover).toHaveBeenCalledTimes(1);
      expect(screen.getByRole("menuitem", { name: /Alpha/ }).getAttribute("data-active")).toBe("true");
      fireEvent.keyDown(window, { key: "ArrowDown" });
      expect(screen.getByRole("menuitem", { name: /Beta/ }).getAttribute("data-active")).toBe("true");
      fireEvent.keyDown(window, { key: "w" });
      expect(onHoverEnd).toHaveBeenCalled();
      expect(screen.getByRole("menuitem", { name: /Alpha/ }).getAttribute("data-active")).toBe("true");
    });

    it("opens nested submenus with ArrowRight and closes with ArrowLeft", async () => {
      render(
        <ContextMenuController
          title={uiDataLabel("Menu")}
          open
          position={{ x: 4, y: 4 }}
          items={[
            {
              id: "transform",
              label: uiDataLabel("Transform"),
              children: [
                { id: "move", label: uiDataLabel("Move") },
                { id: "rotate", label: uiDataLabel("Rotate") },
              ],
            },
            { id: "delete", label: uiDataLabel("Delete") },
          ]}
          onOpenChange={vi.fn()}
        />,
      );
      await waitFor(() => screen.getByRole("menuitem", { name: "Transform" }));
      fireEvent.keyDown(window, { key: "ArrowDown" });
      fireEvent.keyDown(window, { key: "ArrowRight" });
      await waitFor(() => screen.getByRole("menuitem", { name: "Move" }));
      fireEvent.keyDown(window, { key: "ArrowLeft" });
      await waitFor(() => expect(screen.queryByRole("menuitem", { name: "Move" })).toBeNull());
    });

    it("does not close on select when closeOnSelect is false", async () => {
      const onOpenChange = vi.fn();
      const onSelect = vi.fn();
      render(<ContextMenuController title={uiDataLabel("Menu")} open closeOnSelect={false} position={{ x: 4, y: 4 }} items={[{ id: "place", label: uiDataLabel("Place"), onSelect }]} onOpenChange={onOpenChange} />);
      fireEvent.click(await waitFor(() => screen.getByRole("menuitem", { name: "Place" })));
      expect(onSelect).toHaveBeenCalled();
      expect(onOpenChange).not.toHaveBeenCalled();
    });

    it("isContextMenuPointerTarget detects menu surfaces", () => {
      const menu = document.createElement("div");
      menu.setAttribute("role", "menu");
      const child = document.createElement("button");
      menu.appendChild(child);
      document.body.appendChild(menu);
      expect(isContextMenuPointerTarget(child)).toBe(true);
      expect(isContextMenuPointerTarget(document.body)).toBe(false);
      menu.remove();
    });

    it("renders a non-interactive header row for a labeled separator, leaving a bare separator unlabeled", async () => {
      render(
        <ContextMenuController
          title={uiDataLabel("Menu")}
          open
          position={{ x: 4, y: 4 }}
          items={[
            { id: "recent-header", label: uiDataLabel("Recent"), separator: true },
            { id: "a", label: uiDataLabel("Alpha") },
            { id: "sep", label: uiDataLabel(""), separator: true },
            { id: "b", label: uiDataLabel("Beta") },
          ]}
          onOpenChange={vi.fn()}
        />,
      );
      const header = await waitFor(() => screen.getByText("Recent"));
      expect(header.getAttribute("role")).toBe("separator");
      expect(header.getAttribute("aria-label")).toBe("Recent");
      expect(header.tagName).not.toBe("BUTTON");
      const bareSeparators = document.querySelectorAll('[role="separator"]:not([aria-label])');
      expect(bareSeparators.length).toBe(1);
    });

    it("toggles a parent row's submenu open and closed on click", async () => {
      render(<ContextMenuController title={uiDataLabel("Menu")} open position={{ x: 4, y: 4 }} items={[{ id: "transform", label: uiDataLabel("Transform"), children: [{ id: "move", label: uiDataLabel("Move") }] }]} onOpenChange={vi.fn()} />);
      const parent = await waitFor(() => screen.getByRole("menuitem", { name: "Transform" }));
      expect(screen.queryByRole("menuitem", { name: "Move" })).toBeNull();
      fireEvent.click(parent);
      await waitFor(() => expect(screen.getByRole("menuitem", { name: "Move" })).toBeTruthy());
      fireEvent.click(parent);
      await waitFor(() => expect(screen.queryByRole("menuitem", { name: "Move" })).toBeNull());
    });

    it("renders a parent row's shortcut before the submenu chevron", async () => {
      render(
        <ContextMenuController
          title={uiDataLabel("Menu")}
          open
          position={{ x: 4, y: 4 }}
          items={[{ id: "transform", label: uiDataLabel("Transform"), shortcut: "⌘️T", children: [{ id: "move", label: uiDataLabel("Move") }] }]}
          onOpenChange={vi.fn()}
        />,
      );
      const parent = await waitFor(() => screen.getByRole("menuitem", { name: "Transform" }));
      expect(parent.textContent).toContain("⌘️T");
    });

    it("caps the menu surface height with a scrolling overflow class", async () => {
      render(<ContextMenuController title={uiDataLabel("Menu")} open position={{ x: 4, y: 4 }} items={[{ id: "a", label: uiDataLabel("Alpha") }]} onOpenChange={vi.fn()} />);
      const menu = await waitFor(() => screen.getByRole("menu"));
      const chrome = menu.closest<HTMLElement>('[data-slot="context-menu-content"]');
      expect(chrome?.className).toContain("max-h-layout-command");
      expect(chrome?.className).toContain("overflow-y-auto");
    });
  });

  describe("CanvasPickMenu", () => {
    const targets = [
      { domain: "group", id: "g1", generality: 0, label: "Group 1" },
      { domain: "path", id: "p1", generality: 2, label: "Path 1" },
    ] satisfies readonly CanvasPickTarget[];

    it("renders general-first rows and highlights hovered item", async () => {
      const onHoverKey = vi.fn();
      render(<CanvasPickMenu request={{ targets, client: { x: 10, y: 20 } }} hoveredKey="path:p1" onHoverKey={onHoverKey} onPick={vi.fn()} onDismiss={vi.fn()} />);
      const items = await waitFor(() => screen.getAllByRole("menuitem"));
      expect(items[0]?.textContent).toContain("Group 1");
      expect(items[1]?.getAttribute("aria-selected")).toBe("true");
    });

    it("dismisses on outside pointerdown", async () => {
      const onDismiss = vi.fn();
      render(<CanvasPickMenu request={{ targets, client: { x: 10, y: 20 } }} hoveredKey={null} onHoverKey={vi.fn()} onPick={vi.fn()} onDismiss={onDismiss} />);
      await waitFor(() => expect(screen.getAllByRole("menuitem").length).toBe(2));
      window.dispatchEvent(new PointerEvent("pointerdown", { bubbles: true }));
      expect(onDismiss).toHaveBeenCalled();
    });
  });

  /** 🐚️ Test-only stand-in for `FrameworkOsShell`'s own scope wiring — wraps a render-prop's output in
   * a `ShellScopeProvider` whose root points at the wrapper div, so `useShellKeydown` consumers
   * (`Window`, `Mode`) route document keydowns the same way they do inside a real mounted shell.
   * `renderChildren` is a FUNCTION, not a plain `ReactNode`: `rootRef.current` is still `null` on the
   * initial render (refs attach during commit, after render finishes), so this needs a forced second
   * render — mirroring `FrameworkOsShell`'s own `bumpAfterRootAttach` — to ever see the populated root.
   * A plain `children` prop would defeat that: React treats an unchanged child *element* (the common
   * "pass `children` through" optimization) as a bailout and never re-invokes it, so a component like
   * `Mode` with no unrelated state of its own to force a second render would never re-read the ref.
   * Calling a function instead produces a fresh element every render, matching how `FrameworkOsShell`
   * really works — its inner tree is authored directly in its own body, not threaded through as an
   * opaque prop. */
  function TestShellRoot({ children }: { readonly children: () => React.ReactNode }): React.ReactElement {
    const [scope] = reactHostPort.useState(() => createShellScope({ storage: createMemoryStoragePort() }));
    const [, bump] = reactHostPort.useState(0);
    // 🐚️ Memoized: an inline (re-created every render) ref callback makes React detach+reattach the
    // ref on every commit (identity changed → old callback fires with `null`, new one fires with the
    // node), and since each call bumps state, that becomes an infinite render loop.
    const setRoot = reactHostPort.useCallback(
      (node: HTMLDivElement | null) => {
        scope.rootRef.current = node;
        bump((n) => n + 1);
      },
      [scope],
    );
    return (
      <div ref={setRoot}>
        <ShellScopeProvider scope={scope}>{children()}</ShellScopeProvider>
      </div>
    );
  }

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
      expect(activeButton?.className).toContain("text-emphasized");
      expect(container.querySelector('[data-slot="panel-tabs"]')?.className).toContain("overflow-x-auto");
      expect(container.querySelector('[data-slot="panel-tabs"]')?.className).toContain("ui-scrollbar-hidden");
      expect(container.querySelector('[data-slot="panel-tabs"]')?.className).toContain("z-40");
      expect(container.querySelector('[data-slot="panel-tabs"]')?.className?.split(" ")).not.toContain("px-single");
      expect(container.querySelector('[data-slot="panel-tabs"]')?.className?.split(" ")).not.toContain("w-full");
      expect(container.querySelector('[data-slot="window-chrome-gap"]')).toBeTruthy();
      expect(container.querySelector('[data-slot="window-chrome-silhouette-border"]')).toBeTruthy();
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
      expect(panel.style.top).toContain("size-large");
      expect(panel.style.top).toContain("size-medium");
      expect(panel.style.top).not.toContain("spacing-single");
      expect(container.querySelector('[data-slot="window-chrome-stack"]')).toBeTruthy();
      expect(container.querySelector('[data-slot="window-chrome-cap"]')).toBeTruthy();
      expect(container.querySelector('[data-slot="window-chrome-gap"]')).toBeTruthy();
      expect(screen.getByText("Root A")).toBeTruthy();
      expect(screen.getByText("Leaf A")).toBeTruthy();
      expect(screen.getByText("Leaf B")).toBeTruthy();
      expect(container.querySelector('[data-slot="panel-fold"]')).toBeTruthy();
    });

    it("chromeHostedOpenPanelPositionStyle pulls top/bottom caps into the shell chrome band and leaves side-middle canvas insets alone", () => {
      const top = chromeHostedOpenPanelPositionStyle("top-left");
      expect(top.top).toBe("calc(-1 * (var(--size-large) + var(--size-medium)) / 2)");
      expect(top.left).toBe("var(--spacing-single)");
      expect(top.maxHeight).toBe("calc(100% + (var(--size-large) + var(--size-medium)) / 2)");

      const bottom = chromeHostedOpenPanelPositionStyle("bottom-right");
      expect(bottom.bottom).toBe("calc(-1 * (var(--size-large) + var(--size-medium)) / 2)");
      expect(bottom.right).toBe("var(--spacing-single)");

      const side = chromeHostedOpenPanelPositionStyle("left-middle");
      expect(side.top).toBe("50%");
      expect(side.left).toBe("var(--spacing-single)");
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

    it("navbar fullscreen toggle parks its width so inline labels do not collapse", async () => {
      const { render } = await import("@testing-library/react");
      const rectSpy = vi.spyOn(Element.prototype, "getBoundingClientRect").mockImplementation(function (this: Element) {
        if (this.getAttribute("data-slot") === "navbar-fullscreen-toggle") {
          return { width: 112, height: 24, top: 0, left: 0, right: 112, bottom: 24, x: 0, y: 0, toJSON: () => ({}) } as DOMRect;
        }
        return { width: 0, height: 0, top: 0, left: 0, right: 0, bottom: 0, x: 0, y: 0, toJSON: () => ({}) } as DOMRect;
      });
      const { container } = render(
        <UiDriverProvider driver={DEFAULT_UI_DRIVER}>
          <Navbar items={[]} showFullscreenToggle />
        </UiDriverProvider>,
      );
      const slot = container.querySelector('[data-slot="navbar-fullscreen-toggle"]') as HTMLElement;
      expect(slot.style.minWidth).toBe("112px");
      expect(shellNavbarTrailingEndWidthByRoot.get(document.documentElement)).toBe(112);
      rectSpy.mockRestore();
    });

    it('Panel with tabBarHost="chrome" at top-middle and bottom-middle expands with glass panel level and root chips on the chrome cap', () => {
      const StubIcon = (): null => null;
      const tabs: PanelTabNode[] = [{ kind: "branch", id: "root", icon: StubIcon, name: "Command", children: [singleTreeLeaf({ id: "leaf", icon: StubIcon, name: "Palette", tree: { sections: [] } })] }];
      for (const anchor of ["top-middle", "bottom-middle"] as const) {
        const { container, unmount } = render(<Panel anchor={anchor} tabBarHost="chrome" visible tabs={tabs} activeTabPath={["root", "leaf"]} size={360} />);
        const panel = container.querySelector('[data-slot="panel"]');
        expect(panel?.getAttribute("data-level")).toBe("panel");
        expect(panel?.querySelector(".ui-glass")).toBeTruthy();
        expect(screen.getByText("Command")).toBeTruthy();
        expect(screen.getByText("Palette")).toBeTruthy();
        unmount();
      }
    });

    it("flowFromAnchor mirrors right anchors inline and bottom anchors in block; middle anchors never mirror", () => {
      expect(flowFromAnchor("top-left")).toEqual({ inline: "ltr", block: "down" });
      expect(flowFromAnchor("top-right")).toEqual({ inline: "rtl", block: "down" });
      expect(flowFromAnchor("bottom-left")).toEqual({ inline: "ltr", block: "up" });
      expect(flowFromAnchor("bottom-right")).toEqual({ inline: "rtl", block: "up" });
      expect(flowFromAnchor("top-middle")).toEqual({ inline: "ltr", block: "down" });
      expect(flowFromAnchor("bottom-middle")).toEqual({ inline: "ltr", block: "up" });
    });

    it("Tree direction=up renders a group's content before its own header row, with reversed sibling order", () => {
      const sections: TreeDataSection[] = [
        { id: "sec-a", label: "Section A", defaultOpen: true, items: [{ id: "item-a", label: "Item A" }] },
        { id: "sec-b", label: "Section B", defaultOpen: true, items: [{ id: "item-b", label: "Item B" }] },
      ];
      const downMarkup = renderToStaticMarkup(<Tree sections={sections} />);
      const upMarkup = renderToStaticMarkup(<Tree sections={sections} direction="up" />);

      // Default (down): sections render in declared order, each header before its content.
      expect(downMarkup.indexOf("Section A")).toBeLessThan(downMarkup.indexOf("Section B"));
      expect(downMarkup.indexOf("Section A")).toBeLessThan(downMarkup.indexOf("Item A"));

      // Up: sibling sections reverse (B before A), and within each section the header comes after its own content.
      expect(upMarkup.indexOf("Section B")).toBeLessThan(upMarkup.indexOf("Section A"));
      expect(upMarkup.indexOf("Item A")).toBeLessThan(upMarkup.indexOf("Section A"));
    });

    it("Tree direction=up uses chevron-left (collapsed) / chevron-up (expanded); direction=down keeps chevron-right / chevron-down", () => {
      const sections: TreeDataSection[] = [{ id: "sec", label: "Section", defaultOpen: true, items: [{ id: "item", label: "Item" }] }];
      const downMarkup = renderToStaticMarkup(<Tree sections={sections} />);
      const upMarkup = renderToStaticMarkup(<Tree sections={sections} direction="up" />);
      expect(downMarkup).toContain('data-icon="chevron-down"');
      expect(downMarkup).not.toContain('data-icon="chevron-up"');
      expect(upMarkup).toContain('data-icon="chevron-up"');
      expect(upMarkup).not.toContain('data-icon="chevron-down"');
    });

    it("Tree direction=up collapses to chevron-left instead of chevron-right", () => {
      const sections: TreeDataSection[] = [{ id: "sec", label: "Section", defaultOpen: false, items: [{ id: "item", label: "Item" }] }];
      const upMarkup = renderToStaticMarkup(<Tree sections={sections} direction="up" />);
      expect(upMarkup).toContain('data-icon="chevron-left"');
      expect(upMarkup).not.toContain('data-icon="chevron-right"');
    });

    it("Tree direction=up extends open-group branch stems toward children (top) instead of toward the bottom", () => {
      const sections: TreeDataSection[] = [
        {
          id: "sec",
          label: "Section",
          defaultOpen: true,
          items: [
            {
              id: "group",
              label: "Group",
              defaultOpen: true,
              items: [
                { id: "leaf-a", label: "Leaf A" },
                { id: "leaf-b", label: "Leaf B" },
              ],
            },
          ],
        },
      ];
      const downMarkup = renderToStaticMarkup(<Tree sections={sections} />);
      const upMarkup = renderToStaticMarkup(<Tree sections={sections} direction="up" />);
      const stemStyle = (markup: string) => markup.match(/data-slot="tree-branch-stem"[^>]*style="([^"]*)"/)?.[1] ?? "";
      const downStem = stemStyle(downMarkup);
      const upStem = stemStyle(upMarkup);
      expect(downStem).toContain("top:calc(var(--size-workbench) / 2)");
      expect(downStem).toContain("bottom:0");
      expect(downStem).not.toContain("top:0");
      expect(upStem).toContain("top:0");
      expect(upStem).toContain("bottom:calc(var(--size-workbench) / 2)");
      expect(upStem).not.toContain("bottom:0");
    });

    it("progressPanelTabSelection reveals one level per press, records/restores drill-down memory, collapses, and prunes stale memory", () => {
      const StubIcon = (): null => null;
      const leaf = (id: string, name: string) => singleTreeLeaf({ id, icon: StubIcon, name, tree: { sections: [] } });
      const tabs: PanelTabNode[] = [
        {
          kind: "branch",
          id: "root-a",
          icon: StubIcon,
          name: "Root A",
          children: [{ kind: "branch", id: "child-a", icon: StubIcon, name: "Child A", children: [leaf("grandchild-a", "Grandchild A")] }, leaf("child-b", "Child B")],
        },
        { kind: "branch", id: "root-b", icon: StubIcon, name: "Root B", children: [leaf("child-c", "Child C")] },
      ];

      // Fresh branch press reveals only its own row — no auto-descend.
      const revealed = progressPanelTabSelection(tabs, [], ["root-a"], {});
      expect(revealed).toEqual({ path: ["root-a"], memory: {}, fold: false });

      // Selecting a leaf records a memory hop for every step of the final path.
      const leafSelected = progressPanelTabSelection(tabs, ["root-a"], ["root-a", "child-b"], {});
      expect(leafSelected.path).toEqual(["root-a", "child-b"]);
      expect(leafSelected.memory).toEqual({ "root-a": "child-b" });

      // Re-visiting a branch with remembered drill-down restores it, chained through nested branches.
      const withDeepMemory = { "root-a": "child-a", "child-a": "grandchild-a" };
      const restored = progressPanelTabSelection(tabs, [], ["root-a"], withDeepMemory);
      expect(restored.path).toEqual(["root-a", "child-a", "grandchild-a"]);
      expect(restored.memory).toEqual(withDeepMemory);

      // Collapsing a branch (re-pressing it while active) truncates before it and clears its subtree's memory, but keeps the parent's own hop.
      const collapsed = progressPanelTabSelection(tabs, ["root-a", "child-a", "grandchild-a"], ["root-a", "child-a"], withDeepMemory);
      expect(collapsed).toEqual({ path: ["root-a"], memory: { "root-a": "child-a" }, fold: false });

      // Root re-press always folds the hosting panel while preserving the selected child path and tree-memory for reopening.
      const folded = progressPanelTabSelection(tabs, ["root-a", "child-a", "grandchild-a"], ["root-a"], withDeepMemory);
      expect(folded).toEqual({ path: ["root-a", "child-a", "grandchild-a"], memory: withDeepMemory, fold: true });

      // A stale memory entry (no longer a valid child) is pruned rather than followed.
      const staleMemory = { "root-a": "no-longer-exists" };
      const pruned = progressPanelTabSelection(tabs, [], ["root-a"], staleMemory);
      expect(pruned.path).toEqual(["root-a"]);
      expect(pruned.memory).toEqual({});

      // Selecting a sibling at a deeper row keeps ancestor segments intact.
      const siblingPick = progressPanelTabSelection(tabs, ["root-a", "child-a"], ["root-a", "child-b"], {});
      expect(siblingPick.path).toEqual(["root-a", "child-b"]);
    });

    it("Panel: unfolding restores the stored path, and re-pressing a nested active leaf deselects instead of folding", () => {
      const StubIcon = (): null => null;
      const tabs: PanelTabNode[] = [
        {
          kind: "branch",
          id: "root-a",
          icon: StubIcon,
          name: "Root A",
          children: [singleTreeLeaf({ id: "leaf-a", icon: StubIcon, name: "Leaf A", tree: { sections: [{ id: "sec", label: "Section", defaultOpen: true, items: [{ id: "row", label: "Row" }] }] } })],
        },
      ];
      const onVisibleChange = vi.fn();
      let path: readonly string[] = ["root-a", "leaf-a"];
      const onActiveTabPathChange = vi.fn((next: readonly string[]) => {
        path = next;
      });
      const { container, rerender } = render(<Panel anchor="top-left" visible={false} onVisibleChange={onVisibleChange} tabs={tabs} activeTabPath={path} onActiveTabPathChange={onActiveTabPathChange} />);
      fireEvent.click(container.querySelector('button[id="root-a"]') as HTMLElement);
      expect(onVisibleChange).toHaveBeenCalledWith(true);
      expect(onActiveTabPathChange).not.toHaveBeenCalled();

      onVisibleChange.mockClear();
      rerender(<Panel anchor="top-left" visible onVisibleChange={onVisibleChange} tabs={tabs} activeTabPath={path} onActiveTabPathChange={onActiveTabPathChange} />);
      expect(screen.getByText("Row")).toBeTruthy();
      fireEvent.click(container.querySelector('button[id="leaf-a"]') as HTMLElement);
      expect(path).toEqual(["root-a"]);
      expect(onVisibleChange).not.toHaveBeenCalled();
    });

    it("Panel forwards drill-down memory through pathMemory/onPathMemoryChange when controlled", () => {
      const StubIcon = (): null => null;
      const tabs: PanelTabNode[] = [{ kind: "branch", id: "root-a", icon: StubIcon, name: "Root A", children: [singleTreeLeaf({ id: "leaf-a", icon: StubIcon, name: "Leaf A", tree: { sections: [] } })] }];
      let memory: Readonly<Record<string, string>> = {};
      const onPathMemoryChange = vi.fn((next: Readonly<Record<string, string>>) => {
        memory = next;
      });
      const { container } = render(<Panel anchor="top-left" visible tabs={tabs} pathMemory={memory} onPathMemoryChange={onPathMemoryChange} />);
      fireEvent.click(container.querySelector('button[id="root-a"]') as HTMLElement);
      fireEvent.click(container.querySelector('button[id="leaf-a"]') as HTMLElement);
      expect(memory).toEqual({ "root-a": "leaf-a" });
    });

    it("Tree's controlled openStates/onOpenStateChange survives what would otherwise be a fresh remount", () => {
      const sections: TreeDataSection[] = [{ id: "sec", label: "Section", defaultOpen: false, items: [{ id: "item", label: "Item" }] }];
      let openStates: Readonly<Record<string, boolean>> = {};
      const onOpenStateChange = vi.fn((id: string, open: boolean) => {
        openStates = { ...openStates, [id]: open };
      });
      const { container, unmount } = render(<Tree sections={sections} openStates={openStates} onOpenStateChange={onOpenStateChange} />);
      fireEvent.click(container.querySelector('[data-slot="tree-section-row"]') as HTMLElement);
      expect(openStates["tree-section-sec"]).toBe(true);
      unmount();
      const { container: remounted } = render(<Tree sections={sections} openStates={openStates} onOpenStateChange={onOpenStateChange} />);
      expect(remounted.querySelector('[data-slot="tree-section-row"]')?.getAttribute("data-state")).toBe("open");
    });

    it("PanelTreeUnitsPane namespaces controlled tree open-state per unit so keys never collide across units", () => {
      const StubIcon = (): null => null;
      const units: PanelTreeUnit[] = [
        { id: "unit-a", tree: { sections: [{ id: "sec", label: "Section A", defaultOpen: false, items: [{ id: "item", label: "Item A" }] }] } },
        { id: "unit-b", tree: { sections: [{ id: "sec", label: "Section B", defaultOpen: false, items: [{ id: "item", label: "Item B" }] }] } },
      ];
      const tabs: PanelTabNode[] = [{ kind: "leaf", id: "tab-a", icon: StubIcon, name: "Tab A", trees: units }];
      let treeOpenStates: Readonly<Record<string, boolean>> = {};
      const onTreeOpenStateChange = vi.fn((id: string, open: boolean) => {
        treeOpenStates = { ...treeOpenStates, [id]: open };
      });
      const { container } = render(<Panel anchor="top-left" visible tabs={tabs} activeTabPath={["tab-a"]} treeOpenStates={treeOpenStates} onTreeOpenStateChange={onTreeOpenStateChange} />);
      const rows = container.querySelectorAll('[data-slot="tree-section-row"]');
      fireEvent.click(rows[0]);
      expect(treeOpenStates).toEqual({ "unit-a:tree-section-sec": true });
      expect(treeOpenStates["unit-b:tree-section-sec"]).toBeUndefined();
    });

    it("PanelTreeUnitsPane re-resolves lazy trees when treeContentRevision changes", () => {
      const StubIcon = (): null => null;
      let sectionLabel = "before";
      const resolveTree = vi.fn(() => ({
        sections: [{ id: "sec", label: sectionLabel, items: [{ id: "item", label: "Item" }] }],
        sortableSections: false as const,
      }));
      const tabs: PanelTabNode[] = [singleTreeLeaf({ id: "tool.fill", icon: StubIcon, name: "Fill", tree: { resolveTree } })];
      const { container, rerender } = render(<Panel anchor="bottom-middle" visible tabs={tabs} activeTabPath={["tool.fill"]} treeContentRevision={0} />);
      expect(container.textContent).toContain("before");
      expect(resolveTree).toHaveBeenCalledTimes(1);
      sectionLabel = "after";
      rerender(<Panel anchor="bottom-middle" visible tabs={tabs} activeTabPath={["tool.fill"]} treeContentRevision={0} />);
      expect(resolveTree).toHaveBeenCalledTimes(1);
      expect(container.textContent).toContain("before");
      rerender(<Panel anchor="bottom-middle" visible tabs={tabs} activeTabPath={["tool.fill"]} treeContentRevision={1} />);
      expect(resolveTree).toHaveBeenCalledTimes(2);
      expect(container.textContent).toContain("after");
    });

    it("PanelTreeUnitsPane omits unlabeled single-unit headers under PanelDockProvider (no lonely top grip)", () => {
      const StubIcon = (): null => null;
      const emptyDock: PanelDock = {
        anchors: {
          "top-left": [],
          "top-middle": [],
          "top-right": [],
          "right-middle": [],
          "bottom-right": [],
          "bottom-middle": [],
          "bottom-left": [],
          "left-middle": [],
        },
      };
      const tabs: PanelTabNode[] = [
        singleTreeLeaf({
          id: "framework.panel.artifact",
          icon: StubIcon,
          name: "Artifact",
          tree: {
            sections: [
              { id: "objects", label: "Building Components", defaultOpen: false, items: [] },
              { id: "references", label: "References", defaultOpen: false, items: [] },
            ],
          },
        }),
      ];
      const { container } = render(
        <PanelDockProvider dock={emptyDock} onTabDockDrop={() => undefined} onTreeUnitDockDrop={() => undefined}>
          <Panel anchor="top-left" visible tabs={tabs} activeTabPath={["framework.panel.artifact"]} />
        </PanelDockProvider>,
      );
      expect(container.querySelector('[data-slot="panel-tree-unit-header"]')).toBeNull();
      const sectionHandles = container.querySelectorAll('[data-slot="tree-section-row"] [data-slot="drag-handle"]');
      expect(sectionHandles.length).toBe(2);
    });

    it("Tree section reorder grips fire onSectionsReorder with the new id order", () => {
      const onSectionsReorder = vi.fn();
      const sections: TreeDataSection[] = [
        { id: "a", label: "A", defaultOpen: false, items: [] },
        { id: "b", label: "B", defaultOpen: false, items: [] },
        { id: "c", label: "C", defaultOpen: false, items: [] },
      ];
      const { container } = render(<Tree sections={sections} sortableSections onSectionsReorder={onSectionsReorder} />);
      const rows = container.querySelectorAll('[data-slot="tree-section-row"]');
      expect(rows.length).toBe(3);
      expect(container.querySelectorAll('[data-slot="tree-section-row"] [data-slot="drag-handle"]').length).toBe(3);
      const source = rows[0] as HTMLElement;
      const target = rows[2] as HTMLElement;
      const dataTransfer = {
        effectAllowed: "none",
        dropEffect: "none",
        types: [TREE_SECTION_REORDER_MIME],
        setData: vi.fn(),
        getData: vi.fn(() => "a"),
      };
      fireEvent.dragStart(source, { dataTransfer });
      fireEvent.dragOver(target, { dataTransfer });
      fireEvent.drop(target, { dataTransfer });
      expect(onSectionsReorder).toHaveBeenCalledWith(["b", "c", "a"]);
    });

    it("mergeTreeSectionOrder keeps remembered order and appends new sections", () => {
      const sections: TreeDataSection[] = [
        { id: "a", label: "A" },
        { id: "b", label: "B" },
        { id: "c", label: "C" },
      ];
      expect(mergeTreeSectionOrder(["c", "a"], sections).map((section) => section.id)).toEqual(["c", "a", "b"]);
    });

    it("stretches every data-tree wrapper and nested row to the full host width", () => {
      const { container } = render(
        <Tree
          sections={[
            {
              id: "section",
              label: "Section",
              defaultOpen: true,
              items: [
                {
                  id: "group",
                  label: "Group",
                  defaultOpen: true,
                  items: [{ id: "leaf", label: "Leaf" }],
                },
              ],
            },
          ]}
        />,
      );
      const sectionWrapper = container.querySelector<HTMLElement>('[data-slot="tree-section-wrapper"]');
      const sectionBranch = container.querySelector<HTMLElement>('[data-slot="tree-section-content"]');
      const itemBranch = container.querySelector<HTMLElement>('[data-slot="tree-item-content"]');
      const rows = container.querySelectorAll<HTMLElement>('[data-slot="tree-section-row"], [data-slot="tree-item-row"]');

      expect(sectionWrapper?.classList.contains("w-full")).toBe(true);
      expect(sectionBranch?.classList.contains("w-full")).toBe(true);
      expect(sectionBranch?.parentElement?.classList.contains("w-full")).toBe(true);
      expect(itemBranch?.classList.contains("w-full")).toBe(true);
      expect(rows.length).toBe(3);
      expect(Array.from(rows).every((row) => row.classList.contains("w-full"))).toBe(true);
    });

    it("sortable TreeItem always renders a drag handle", () => {
      const markup = renderToStaticMarkup(
        <TreeContext.Provider value={{ level: 0, isLastAtLevel: [], showLines: true, isTree: true, indentMultiplier: 1 }}>
          <SortableTreeItems items={[{ id: "row-1" }]} onReorder={() => undefined}>
            {(item) => <TreeItem id={item.id} label="Row" sortable sortableId={item.id} />}
          </SortableTreeItems>
        </TreeContext.Provider>,
      );
      expect(markup).toContain('data-slot="drag-handle"');
    });

    it("Panel wires bottom corners' trees to direction=up (content above header) while top corners stay direction=down", () => {
      const StubIcon = (): null => null;
      const tabs: PanelTabNode[] = [
        singleTreeLeaf({
          id: "tab-a",
          icon: StubIcon,
          name: "Tab A",
          tree: {
            sections: [
              { id: "sec-a", label: "Section A", defaultOpen: true, items: [{ id: "item-a", label: "Item A" }] },
              { id: "sec-b", label: "Section B", defaultOpen: true, items: [{ id: "item-b", label: "Item B" }] },
            ],
          },
        }),
      ];
      const { container: topContainer } = render(<Panel anchor="top-left" visible tabs={tabs} activeTabPath={["tab-a"]} />);
      const topMarkup = topContainer.querySelector('[data-slot="panel-content"]')!.innerHTML;
      expect(topMarkup.indexOf("Section A")).toBeLessThan(topMarkup.indexOf("Section B"));
      expect(topContainer.querySelector('[data-slot="scroll-area-viewport"]')?.classList.contains("justify-end")).toBe(false);

      const { container: bottomContainer } = render(<Panel anchor="bottom-left" visible tabs={tabs} activeTabPath={["tab-a"]} />);
      const bottomMarkup = bottomContainer.querySelector('[data-slot="panel-content"]')!.innerHTML;
      expect(bottomMarkup.indexOf("Section B")).toBeLessThan(bottomMarkup.indexOf("Section A"));
      const bottomTree = bottomContainer.querySelector('[data-slot="tree"]');
      const bottomViewport = bottomContainer.querySelector('[data-slot="scroll-area-viewport"]');
      expect(bottomTree?.getAttribute("dir")).toBe("auto");
      expect(bottomTree?.getAttribute("role")).toBe("tree");
      expect(bottomTree?.classList.contains("overflow-y-auto")).toBe(false);
      expect(bottomViewport?.classList.contains("min-h-full")).toBe(true);
      expect(bottomViewport?.classList.contains("flex-col")).toBe(true);
      expect(bottomViewport?.classList.contains("justify-end")).toBe(true);
    });

    it("keeps right-anchored panel tree rows left-to-right inside mirrored panel chrome", () => {
      const StubIcon = (): null => null;
      const tab = singleTreeLeaf({
        id: "history",
        icon: StubIcon,
        name: "History",
        tree: { sections: [{ id: "commands", label: "Commands", defaultOpen: true, items: [{ id: "command", label: "Toggle Panel" }] }] },
      });
      const { container } = render(<Panel anchor="bottom-right" visible tabs={[tab]} activeTabPath={[tab.id]} />);
      expect(container.querySelector('[data-slot="panel"]')?.getAttribute("dir")).toBe("rtl");
      expect(container.querySelector('[data-slot="tree"]')?.getAttribute("dir")).toBe("auto");
      expect(container.querySelector('[data-slot="tree-item-row"]')?.closest('[data-slot="tree"]')?.getAttribute("dir")).toBe("auto");
    });

    it("FlowProvider defaults to ltr/down and lets nested providers override only what they pass", () => {
      const Probe: React.FC = () => {
        const flow = useFlow();
        return (
          <span>
            {flow.inline}/{flow.block}
          </span>
        );
      };
      expect(renderToStaticMarkup(<Probe />)).toContain("ltr/down");
      expect(
        renderToStaticMarkup(
          <FlowProvider inline="rtl" block="up">
            <Probe />
          </FlowProvider>,
        ),
      ).toContain("rtl/up");
      expect(
        renderToStaticMarkup(
          <FlowProvider inline="rtl" block="up">
            <FlowProvider block="down">
              <Probe />
            </FlowProvider>
          </FlowProvider>,
        ),
      ).toContain("rtl/down");
    });

    it("Panel sets dir=rtl only for right anchors (corners and right-middle), never for left or middle anchors", () => {
      const StubIcon = (): null => null;
      const tabs: PanelTabNode[] = [singleTreeLeaf({ id: "tab-a", icon: StubIcon, name: "Tab A", tree: { sections: [] } })];
      const dirOf = (anchor: Anchor) =>
        render(<Panel anchor={anchor} visible tabs={tabs} />)
          .container.querySelector('[data-slot="panel"]')
          ?.getAttribute("dir");
      expect(dirOf("top-left")).toBeNull();
      expect(dirOf("bottom-left")).toBeNull();
      expect(dirOf("left-middle")).toBeNull();
      expect(dirOf("top-middle")).toBeNull();
      expect(dirOf("bottom-middle")).toBeNull();
      expect(dirOf("top-right")).toBe("rtl");
      expect(dirOf("bottom-right")).toBe("rtl");
      expect(dirOf("right-middle")).toBe("rtl");
    });

    it("Panel body follows the corner's flow — trees, labels, and their controls mirror same as the chrome", () => {
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
      expect(rightContainer.querySelector('[data-testid="flow-probe"]')?.textContent).toBe("rtl");
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

    it("Mode dock-stack celebrated silhouette paints a spinning conic fill, not a solid stroke cycle", async () => {
      const { container } = render(
        <div className="h-layout-story w-layout-story-md">
          <Mode
            windows={[{ id: "main", title: uiDataLabel("Main"), iconId: "app-window", children: <div>Main Body</div> }]}
            layout={{ kind: "stack", children: [{ kind: "window", id: "main" }], activeId: "main" }}
            activeWindowId="main"
            onActiveWindowChange={() => {}}
          />
        </div>,
      );
      const stack = container.querySelector('[data-slot="mode-dock-stack"]') as HTMLElement;
      const windowEl = stack.querySelector('[data-slot="window"]') as HTMLElement;
      windowEl.setAttribute("data-celebrated", "true");
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
      stack.setAttribute("data-silhouette-remeasure", "celebrate");
      await waitFor(() => {
        const border = container.querySelector('[data-slot="mode-dock-silhouette-border"]');
        expect(border?.getAttribute("data-kind")).toBe("celebrated");
        expect(border?.querySelector(".window-silhouette-border-celebrated-fill")).toBeTruthy();
        expect(border?.querySelector(".window-silhouette-border-celebrated-mask")).toBeTruthy();
        expect(border?.querySelector("path")?.getAttribute("stroke")).toBe("white");
        expect(border?.querySelector("path.window-silhouette-border-celebrated")).toBeNull();
      });
    });

    it("Mode lays out all windows and marks the active one", async () => {
      const { container } = render(
        <div className="h-layout-story w-layout-story-md">
          <Mode
            windows={[
              { id: "left", title: uiDataLabel("Left"), iconId: "app-window", children: <div>Left Pane</div> },
              { id: "right", title: uiDataLabel("Right"), iconId: "app-window", children: <div>Right Pane</div> },
            ]}
            layout={{
              kind: "row",
              children: [
                { kind: "stack", children: [{ kind: "window", id: "left" }], activeId: "left" },
                { kind: "stack", children: [{ kind: "window", id: "right" }], activeId: "right" },
              ],
            }}
            activeWindowId="right"
            onActiveWindowChange={() => {}}
          />
        </div>,
      );
      expect(screen.getByText("Left Pane")).toBeTruthy();
      expect(screen.getByText("Right Pane")).toBeTruthy();
      expect(container.querySelector('[data-slot="window"][data-active="true"]')).toBeTruthy();
      expect(container.querySelector('[data-slot="window"][data-active="true"]')?.className).not.toContain("border-active-base");
      expect(container.querySelector('[data-slot="mode-dock-tab"][data-window-id="right"][data-active="true"]')).toBeTruthy();
      expect(container.querySelector('[data-slot="mode-dock-tab"][data-window-id="left"] [data-icon-kind]')).toBeTruthy();
      expect(container.querySelector('[data-slot="mode-dock-tab"][data-window-id="left"][data-active="true"]')).toBeNull();
      expect(screen.getByText("Left")).toBeTruthy();
      expect(screen.getByText("Right")).toBeTruthy();
      expect(container.querySelector('[data-slot="mode-body"]')?.getAttribute("data-level")).toBe("base");
      expect(container.querySelector('[data-slot="mode-body"]')?.className).toContain("ui-surface");
      expect(container.querySelector('[data-slot="mode-dock-canvas-label"]')).toBeNull();
      expect(container.querySelector('[data-slot="mode-dock-tab-gap"]')?.className).toContain("flex-1");
      expect(container.querySelector('[data-slot="mode-dock-tabbar"]')?.className).not.toContain("ui-glass");
      expect(container.querySelector('[data-slot="mode-dock-tab-cap"]')?.className).toContain("ui-glass");
      expect(container.querySelector('[data-slot="mode-dock-controls-cap"]')).toBeNull();
      expect(container.querySelector('[data-slot="mode-dock-tab-gap"]')?.className).not.toContain("ui-glass");
      expect(container.querySelector('[data-slot="mode-dock-tab-gap"]')?.className).not.toContain("ui-glass-chrome");
      expect(container.querySelector('[data-slot="mode-dock-tab-gap"]')?.className).not.toContain("ui-surface");
      expect(container.querySelector('[data-slot="mode-dock-tab-cap"]')?.className).not.toContain("ml-auto");
      expect(container.querySelector('[data-slot="mode-dock-tab-cap"]')?.className).not.toContain("justify-end");
      const tabbar = container.querySelector('[data-slot="mode-dock-tabbar"]');
      expect(tabbar?.querySelector('[data-slot="mode-dock-tab-cap"]')).toBeTruthy();
      expect(tabbar?.querySelector('[data-slot="mode-dock-controls-cap"]')).toBeNull();
      expect([...(tabbar?.children ?? [])].map((child) => child.getAttribute("data-slot")).filter(Boolean)).toEqual(["mode-dock-tab-cap", "mode-dock-tab-gap"]);
      expect(container.querySelector('[data-slot="mode-dock-tab-cap"]')?.className).not.toContain("ui-glass-chrome");
      expect(container.querySelector('[data-slot="mode-dock-tab-focus"]')?.className).toContain("hover:text-foreground");
      expect(container.querySelector('[data-slot="mode-dock-tab-close"]')?.className).toContain("hover:text-foreground");
      expect(container.querySelector('[data-slot="mode-dock-tab"]')?.className).toContain("text-element");
      const layoutActiveStack = container.querySelector('[data-slot="window"][data-active="true"]')?.closest('[data-slot="mode-dock-stack"]') as HTMLElement;
      const layoutInactiveStack = [...container.querySelectorAll('[data-slot="mode-dock-stack"]')].find((stack) => !stack.querySelector('[data-slot="window"][data-active="true"]')) as HTMLElement;
      expect(layoutActiveStack?.className).toContain("z-window");
      expect(layoutInactiveStack?.className).toContain("z-window");
      expect(layoutActiveStack?.querySelector('[data-slot="mode-dock-silhouette-border"]')?.getAttribute("data-kind")).toBe("normal");
      expect(layoutInactiveStack?.querySelector('[data-slot="mode-dock-silhouette-border"]')?.getAttribute("data-kind")).toBe("normal");
      fireEvent.pointerDown(layoutActiveStack.querySelector('[data-slot="mode-dock-stack-body"]')!);
      await waitFor(() => {
        expect(layoutActiveStack.getAttribute("data-active")).toBe("true");
        expect(layoutActiveStack.querySelector('[data-slot="mode-dock-silhouette-border"]')?.getAttribute("data-kind")).toBe("active");
      });
      expect(layoutInactiveStack?.querySelector('[data-slot="mode-dock-silhouette-border"]')?.getAttribute("data-kind")).toBe("normal");
      expect(layoutActiveStack?.querySelector('[data-slot="mode-dock-silhouette-border"]')?.className).toContain("z-[40]");
      expect(layoutActiveStack?.querySelector('[data-slot="mode-dock-tab-cap"]')?.className).toContain("border-0");
      expect(layoutActiveStack?.querySelector('[data-slot="mode-dock-tab-focus"]')?.className).toContain("border-0");
      expect(layoutActiveStack?.querySelector('[data-slot="mode-dock-stack-body"]')?.className).toContain("border-0");
      expect(layoutActiveStack?.querySelector('[data-slot="mode-dock-tab-gap"]')?.className).toContain("border-0");
      expect(layoutActiveStack?.querySelector('[data-slot="mode-dock-tab-cap-corner"]')).toBeNull();
      expect(layoutActiveStack?.querySelector('[data-slot="mode-dock-controls-cap"]')).toBeNull();
      expect(container.querySelector('[data-slot="mode-dock-stack"]')?.className).not.toContain("border-emphasized");
    });

    it("Mode exposes its top cutout to the canvas background from pointer down", async () => {
      const onCanvasPointerDown = vi.fn();
      const ControlledMode = () => {
        const [activeWindowId, setActiveWindowId] = reactHostPort.useState<string | null>("right");
        return (
          <Mode
            windows={[
              { id: "left", title: uiDataLabel("Left"), iconId: "app-window", children: <div>Left Body</div> },
              {
                id: "right",
                title: uiDataLabel("Right"),
                iconId: "app-window",
                children: (
                  <div data-testid="under-cutout-canvas" onPointerDown={onCanvasPointerDown}>
                    Right Body
                  </div>
                ),
              },
            ]}
            layout={{
              kind: "stack",
              children: [
                { kind: "window", id: "left" },
                { kind: "window", id: "right" },
              ],
              activeId: "right",
            }}
            activeWindowId={activeWindowId}
            onActiveWindowChange={setActiveWindowId}
          />
        );
      };
      const { container } = render(<ControlledMode />);
      expect(container.querySelector('[data-slot="mode-dock-tab"][data-active="true"]')?.getAttribute("data-window-id")).toBe("right");
      fireEvent.pointerDown(container.querySelector('[data-slot="mode-dock-stack-body"]')!);
      await waitFor(() => expect(container.querySelector('[data-slot="mode-dock-silhouette-border"]')?.getAttribute("data-kind")).toBe("active"));
      const gap = container.querySelector('[data-slot="mode-dock-tab-gap"]')!;
      expect(container.querySelector('[data-slot="mode-dock-stack"]')?.className).toContain("pointer-events-none");
      expect(container.querySelector('[data-slot="mode-dock-stack-body"]')?.className).toContain("pointer-events-auto");
      expect(container.querySelector('[data-slot="mode-dock-tab"]')?.className).toContain("pointer-events-auto");
      expect(container.querySelector('[data-slot="mode-dock-tab"]')?.className).toContain("pointer-events-auto");
      expect(gap.className).toContain("pointer-events-none");
      expect(gap.className).not.toContain("cursor-grab");
      vi.spyOn(gap, "getBoundingClientRect").mockReturnValue({ left: 60, right: 160, top: 0, bottom: 24, width: 100, height: 24, x: 60, y: 0, toJSON: () => ({}) } as DOMRect);
      fireEvent.pointerDown(screen.getByTestId("under-cutout-canvas"), { clientX: 100, clientY: 12 });
      expect(onCanvasPointerDown).toHaveBeenCalledOnce();
      expect(container.querySelector('[data-slot="mode-dock-tab"][data-active="true"]')).toBeNull();
      expect(container.querySelector('[data-slot="window"][data-active="true"]')).toBeNull();
      expect(screen.getByText("Right Body")).toBeTruthy();
      await waitFor(() => expect(container.querySelector('[data-slot="mode-dock-silhouette-border"]')?.getAttribute("data-kind")).toBe("normal"));
    });

    it("Mode dock-stack silhouette stays below Layout panels via z-window stacking context", () => {
      const StubIcon = (): null => null;
      const { container } = render(
        <div className="h-layout-story w-layout-story-lg">
          <Layout
            canvas={
              <Mode
                windows={[{ id: "main", title: uiDataLabel("Main"), iconId: "app-window", children: <div>Main Body</div> }]}
                layout={{ kind: "stack", children: [{ kind: "window", id: "main" }], activeId: "main" }}
                activeWindowId="main"
                onActiveWindowChange={() => {}}
              />
            }
            panels={{
              "top-left": {
                visible: true,
                size: 240,
                tabs: [{ kind: "leaf", id: "workbench", icon: StubIcon, name: "Workbench", trees: [] }],
              },
            }}
          />
        </div>,
      );
      const stack = container.querySelector('[data-slot="mode-dock-stack"]');
      const panel = container.querySelector('[data-slot="panel"]') as HTMLElement | null;
      expect(stack?.className).toContain("z-window");
      expect(stack?.querySelector('[data-slot="mode-dock-silhouette-border"]')?.className).toContain("z-[40]");
      expect(panel?.className).toContain("z-panel");
      expect(panel?.style.zIndex).toBe("");
    });

    it("Mode clears multi-tab active chrome on inactive stacks", async () => {
      const { container } = render(
        <div className="h-layout-story w-layout-story-lg">
          <Mode
            windows={[
              { id: "a1", title: uiDataLabel("A1"), iconId: "app-window", children: <div>A1 Body</div> },
              { id: "a2", title: uiDataLabel("A2"), iconId: "app-window", children: <div>A2 Body</div> },
              { id: "b1", title: uiDataLabel("B1"), iconId: "app-window", children: <div>B1 Body</div> },
              { id: "b2", title: uiDataLabel("B2"), iconId: "app-window", children: <div>B2 Body</div> },
            ]}
            layout={{
              kind: "row",
              children: [
                {
                  kind: "stack",
                  children: [
                    { kind: "window", id: "a1" },
                    { kind: "window", id: "a2" },
                  ],
                  activeId: "a1",
                },
                {
                  kind: "stack",
                  children: [
                    { kind: "window", id: "b1" },
                    { kind: "window", id: "b2" },
                  ],
                  activeId: "b2",
                },
              ],
            }}
            activeWindowId="b2"
            onActiveWindowChange={() => {}}
          />
        </div>,
      );
      const activeStack = container.querySelector('[data-slot="window"][data-active="true"]')?.closest('[data-slot="mode-dock-stack"]') as HTMLElement;
      const inactiveStack = [...container.querySelectorAll('[data-slot="mode-dock-stack"]')].find((stack) => !stack.querySelector('[data-slot="window"][data-active="true"]')) as HTMLElement;
      const inactiveStackTab = inactiveStack?.querySelector('[data-slot="mode-dock-tab"][data-stack-active="true"]');
      const activeStackTab = activeStack?.querySelector('[data-slot="mode-dock-tab"][data-stack-active="true"]');
      expect(inactiveStackTab?.className).not.toContain("border-normal");
      expect(inactiveStackTab?.className).not.toContain("!border-normal");
      expect(inactiveStackTab?.className).not.toContain("border-emphasized");
      expect(inactiveStackTab?.className).not.toContain("border-active-base");
      expect(activeStackTab?.className).toContain("bg-active-base");
      expect(activeStackTab?.className).toContain("text-emphasized");
      expect(activeStackTab?.className).toContain("border-0");
      expect(inactiveStackTab?.className).toContain("text-element");
      expect(inactiveStackTab?.className).not.toContain("text-foreground");
      expect(inactiveStack?.querySelector('[data-slot="mode-dock-tabbar"]')?.className).not.toContain("ui-glass");
      expect(inactiveStack?.querySelector('[data-slot="mode-dock-tab-cap"]')?.className).toContain("ui-glass");
      expect(inactiveStack?.querySelector('[data-slot="mode-dock-controls-cap"]')).toBeNull();
      expect(inactiveStack?.querySelector('[data-slot="mode-dock-tab-close"]')?.className).toContain("border-0");
      expect(activeStack?.querySelector('[data-slot="mode-dock-tabbar"]')?.className).not.toContain("ui-glass");
      expect(activeStack?.querySelector('[data-slot="mode-dock-tab-cap"]')?.className).toContain("ui-glass");
      expect(activeStack?.querySelector('[data-slot="mode-dock-controls-cap"]')).toBeNull();
      expect(activeStack?.querySelector('[data-slot="mode-dock-tab-focus"]')).toBeTruthy();
      expect(activeStack?.querySelector('[data-slot="mode-dock-silhouette-border"]')?.getAttribute("data-kind")).toBe("normal");
      expect(inactiveStack?.querySelector('[data-slot="mode-dock-silhouette-border"]')?.getAttribute("data-kind")).toBe("normal");
      fireEvent.pointerDown(activeStack.querySelector('[data-slot="mode-dock-stack-body"]')!);
      await waitFor(() => {
        expect(activeStack.getAttribute("data-active")).toBe("true");
        expect(activeStack.querySelector('[data-slot="mode-dock-silhouette-border"]')?.getAttribute("data-kind")).toBe("active");
      });
      expect(inactiveStack?.querySelector('[data-slot="mode-dock-silhouette-border"]')?.getAttribute("data-kind")).toBe("normal");
      expect(inactiveStack?.querySelector('[data-slot="mode-dock-tab-active-cell"]')).toBeNull();
      expect(activeStack?.querySelector('[data-slot="mode-dock-tab-active-cell"]')).toBeNull();
      expect(activeStack?.querySelector('[data-slot="mode-dock-tab"][data-stack-active="true"]')).toBeTruthy();
    });

    it("Mode keeps one canvas inset and one gutter between adjacent stacks", () => {
      const { container } = render(
        <div className="h-layout-story w-layout-story-md">
          <Mode
            windows={[
              { id: "left", title: uiDataLabel("Left"), iconId: "app-window", children: <div>Left Pane</div> },
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
      const modeBody = container.querySelector('[data-slot="mode-body"]');
      expect(modeBody?.className).toContain(MODE_CANVAS_INSET_CLASS);
      const panelGroup = container.querySelector('[data-slot="resizable-panel-group"]');
      expect(panelGroup?.getAttribute("data-panel-group-direction")).toBe("horizontal");
      const panels = [...container.querySelectorAll('[data-slot="resizable-panel"]')];
      expect(panels.length).toBeGreaterThanOrEqual(2);
      for (const panel of panels) {
        expect(panel.className).not.toContain("p-single");
        expect(panel.className).not.toContain("p-double");
      }
      const horizontalHandle = container.querySelector('[data-slot="resizable-handle"]');
      expect(horizontalHandle).toBeTruthy();
      expect(horizontalHandle!.className).toContain("w-single");
      expect(horizontalHandle!.className).not.toContain("data-[panel-group-direction=horizontal]:w-single");
      expect((horizontalHandle as HTMLElement).style.width).toBe("var(--spacing-single)");
    });

    it("Mode uses the same gutter for vertical splits as canvas inset", () => {
      const { container } = render(
        <div className="h-layout-story w-layout-story-md">
          <Mode
            windows={[
              { id: "top", title: uiDataLabel("Top"), iconId: "app-window", children: <div>Top Pane</div> },
              { id: "bottom", title: uiDataLabel("Bottom"), iconId: "app-window", children: <div>Bottom Pane</div> },
            ]}
            layout={{
              kind: "column",
              children: [
                { kind: "stack", children: [{ kind: "window", id: "top" }], activeId: "top" },
                { kind: "stack", children: [{ kind: "window", id: "bottom" }], activeId: "bottom" },
              ],
            }}
            activeWindowId="top"
            onActiveWindowChange={() => {}}
          />
        </div>,
      );
      const modeBody = container.querySelector('[data-slot="mode-body"]');
      expect(modeBody?.className).toContain(MODE_CANVAS_INSET_CLASS);
      const panelGroup = container.querySelector('[data-slot="resizable-panel-group"]');
      expect(panelGroup?.getAttribute("data-panel-group-direction")).toBe("vertical");
      const verticalHandle = container.querySelector('[data-slot="resizable-handle"]') as HTMLElement | null;
      expect(verticalHandle).toBeTruthy();
      expect(verticalHandle!.getAttribute("data-resize-orientation")).toBe("vertical");
      expect(verticalHandle!.className).toContain("h-single");
      expect(verticalHandle!.style.height).toBe("var(--spacing-single)");
    });

    it("Mode renders corner grabs at perpendicular split intersections", () => {
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
          />
        </div>,
      );
      const corners = [...container.querySelectorAll('[data-slot="resizable-corner"]')];
      expect(corners.length).toBeGreaterThanOrEqual(4);
      expect(corners.every((node) => node.className.includes("cursor-move"))).toBe(true);
    });

    it("Mode tab stack shows only the active window body", async () => {
      const { container } = render(
        <div className="h-layout-story w-layout-story-md">
          <Mode
            windows={[
              { id: "a", title: uiDataLabel("Alpha"), iconId: "app-window", children: <div>Alpha Body</div> },
              { id: "b", title: uiDataLabel("Beta"), iconId: "app-window", children: <div>Beta Body</div> },
            ]}
            layout={{
              kind: "stack",
              children: [
                { kind: "window", id: "a" },
                { kind: "window", id: "b" },
              ],
              activeId: "a",
            }}
            activeWindowId="a"
            onActiveWindowChange={() => {}}
          />
        </div>,
      );
      expect(container.querySelector('[data-slot="mode-dock-stack-body"]')?.getAttribute("data-level")).toBe("base");
      expect(container.querySelector('[data-slot="mode-dock-stack-body"]')?.className).toContain("ui-surface");
      expect(container.querySelector('[data-slot="mode-dock-stack-body"]')?.className).toContain("p-single");
      expect(screen.getByText("Alpha Body")).toBeTruthy();
      expect(screen.queryByText("Beta Body")).toBeNull();
      expect(container.querySelector('[data-slot="mode-dock-chrome-column"]')).toBeNull();
      expect(container.querySelector('[data-slot="mode-dock-tab-active-cell"]')).toBeNull();
      expect(container.querySelector('[data-slot="mode-dock-tab-cap"]')?.className).toContain("ui-glass");
      expect(container.querySelector('[data-slot="mode-dock-tab"][data-stack-active="true"]')?.className).toContain("bg-active-base");
      expect(container.querySelector('[data-slot="mode-dock-tab"][data-stack-active="true"]')?.className).toContain("text-emphasized");
      expect(container.querySelector('[data-slot="mode-dock-tab"][data-stack-active="true"]')?.className).toContain("border-0");
      expect(container.querySelector('[data-slot="mode-dock-tab"][data-stack-active="true"]')?.className).toContain("z-20");
      expect(container.querySelector('[data-slot="mode-dock-tab"][data-window-id="b"]')?.className).toContain("z-30");
      expect(container.querySelector('[data-slot="mode-dock-tab"][data-window-id="b"]')?.className).not.toContain("border-normal");
      expect(container.querySelector('[data-slot="mode-dock-tab"][data-window-id="b"]')?.className).not.toContain("!border-normal");
      expect(container.querySelector('[data-slot="mode-dock-tab"][data-window-id="b"]')?.className).not.toContain("border-emphasized");
      expect(container.querySelector('[data-slot="mode-dock-chrome-column"]')).toBeNull();
      expect(container.querySelector('[data-slot="mode-dock-stack"] [data-slot="mode-dock-stack-body"]')).toBeTruthy();
      const multiTabBar = container.querySelector('[data-slot="mode-dock-tabbar"]');
      expect(multiTabBar?.querySelector('[data-slot="mode-dock-controls-cap"]')).toBeNull();
      expect(multiTabBar?.querySelectorAll('[data-slot="mode-dock-tab-focus"]')).toHaveLength(2);
      expect(container.querySelector('[data-slot="mode-dock-stack"]')?.className).not.toContain("grid");
      expect(container.querySelector('[data-slot="mode-dock-tabbar"]')?.className).not.toContain("ui-glass");
      expect(container.querySelector('[data-slot="mode-dock-controls-cap"]')).toBeNull();
      expect(container.querySelector('[data-slot="mode-dock-tab-cap"]')?.className).toContain("ui-glass");
      expect(container.querySelector('[data-slot="mode-dock-tab-gap"]')?.className).not.toContain("ui-glass");
      expect(container.querySelector('[data-slot="mode-dock-tab-gap"]')?.className).not.toContain("ui-glass-chrome");
      expect(container.querySelector('[data-slot="mode-dock-tab-gap"]')?.className).toContain("border-0");
      expect(container.querySelector('[data-slot="mode-dock-silhouette-border"]')?.getAttribute("data-kind")).toBe("normal");
      fireEvent.pointerDown(container.querySelector('[data-slot="mode-dock-stack-body"]')!);
      await waitFor(() => {
        expect(container.querySelector('[data-slot="mode-dock-stack"]')?.getAttribute("data-active")).toBe("true");
        expect(container.querySelector('[data-slot="mode-dock-silhouette-border"]')?.getAttribute("data-kind")).toBe("active");
      });
      const tabOrder = () => [...container.querySelectorAll('[data-slot="mode-dock-tab"]')].map((tab) => tab.getAttribute("data-window-id"));
      expect(tabOrder()).toEqual(["a", "b"]);
      fireEvent.click(screen.getByText("Beta"));
      expect(screen.getByText("Beta Body")).toBeTruthy();
      expect(screen.queryByText("Alpha Body")).toBeNull();
      expect(container.querySelector('[data-slot="mode-dock-tab"][data-stack-active="true"]')?.getAttribute("data-window-id")).toBe("b");
      expect(tabOrder()).toEqual(["a", "b"]);
    });

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

    it("computeModeDropZone treats tab bar hits as tab drops not body splits", () => {
      const tabBar = { left: 0, top: 0, right: 200, bottom: 24, width: 200, height: 24 } as DOMRect;
      const body = { left: 0, top: 24, right: 200, bottom: 224, width: 200, height: 200 } as DOMRect;
      const targets = new Map([
        [
          "1",
          {
            corners: { topLeft: { rect: tabBar, element: null as unknown as HTMLElement } },
            body,
          },
        ],
      ]);
      expect(computeModeDropZone(100, 12, targets, null)).toEqual({ kind: "tab", stackPath: "1", corner: "topLeft", index: 0 });
      expect(computeModeDropZone(100, 30, targets, null)).toEqual({ kind: "split", stackPath: "1", side: "top" });
      expect(computeModeDropZone(100, 200, targets, null)).toEqual({ kind: "split", stackPath: "1", side: "bottom" });
      expect(computeModeDropZone(50, 120, targets, null)).toEqual({ kind: "split", stackPath: "1", side: "left" });
      expect(computeModeDropZone(150, 120, targets, null)).toEqual({ kind: "split", stackPath: "1", side: "right" });
    });
    it("modeStackTabsByCorner groups windows by corner defaulting to topLeft", () => {
      const groups = modeStackTabsByCorner([
        { kind: "window", id: "a" },
        { kind: "window", id: "b", corner: "topRight" },
        { kind: "window", id: "c", corner: "bottomLeft" },
        { kind: "window", id: "d", corner: "bottomRight" },
        { kind: "window", id: "e", corner: "topLeft" },
      ]);
      expect(groups.topLeft.map((child) => child.id)).toEqual(["a", "e"]);
      expect(groups.topRight.map((child) => child.id)).toEqual(["b"]);
      expect(groups.bottomLeft.map((child) => child.id)).toEqual(["c"]);
      expect(groups.bottomRight.map((child) => child.id)).toEqual(["d"]);
    });

    it("applyModeDrop moves a tab to another corner of the same stack", () => {
      const layout: WindowLayoutNode = {
        kind: "stack",
        activeId: "a",
        children: [
          { kind: "window", id: "a", corner: "topLeft" },
          { kind: "window", id: "b", corner: "topLeft" },
        ],
      };
      const drag = {
        dragKind: "tab" as const,
        windowId: "b",
        stackPath: "",
        tabIndex: 1,
        pointerId: 1,
        ghostLabel: "B",
        x: 0,
        y: 0,
      };
      const next = applyModeDrop(layout, drag, { kind: "tab", stackPath: "", corner: "bottomRight", index: 0 });
      expect(next.kind).toBe("stack");
      if (next.kind !== "stack") return;
      expect(next.activeId).toBe("b");
      expect(next.children.map((child) => ({ id: child.id, corner: child.corner }))).toEqual([
        { id: "a", corner: "topLeft" },
        { id: "b", corner: "bottomRight" },
      ]);
    });

    it("insertWindowAsTabAtCorner preserves one activeId across corners", () => {
      const layout: WindowLayoutNode = {
        kind: "stack",
        activeId: "a",
        children: [{ kind: "window", id: "a", corner: "topLeft" }],
      };
      const next = insertWindowAsTabAtCorner(layout, "", "b", "topRight");
      expect(next.kind).toBe("stack");
      if (next.kind !== "stack") return;
      expect(next.activeId).toBe("b");
      expect(modeStackTabsByCorner(next.children).topRight.map((child) => child.id)).toEqual(["b"]);
      expect(modeStackTabsByCorner(next.children).topLeft.map((child) => child.id)).toEqual(["a"]);
    });

    it("computeModeDropZone resolves distinct corner tab bars", () => {
      const topLeft = { left: 0, top: 0, right: 80, bottom: 24, width: 80, height: 24 } as DOMRect;
      const topRight = { left: 120, top: 0, right: 200, bottom: 24, width: 80, height: 24 } as DOMRect;
      const body = { left: 0, top: 24, right: 200, bottom: 224, width: 200, height: 200 } as DOMRect;
      const targets = new Map([
        [
          "0",
          {
            corners: {
              topLeft: { rect: topLeft, element: null as unknown as HTMLElement },
              topRight: { rect: topRight, element: null as unknown as HTMLElement },
            },
            body,
          },
        ],
      ]);
      expect(computeModeDropZone(40, 12, targets, null)).toEqual({ kind: "tab", stackPath: "0", corner: "topLeft", index: 0 });
      expect(computeModeDropZone(160, 12, targets, null)).toEqual({ kind: "tab", stackPath: "0", corner: "topRight", index: 0 });
    });

    it("beginWindowTemplateDrag records a session for mode dock preview", () => {
      beginWindowTemplateDrag({ payload: { windowKindId: "main", templateId: "top" }, label: "Top" });
      expect(readActiveWindowTemplateDragSession()?.label).toBe("Top");
      endWindowTemplateDrag();
      expect(readActiveWindowTemplateDragSession()).toBeNull();
    });

    it("windowTemplatePaletteTreeDragController starts pointer and native template drags", () => {
      const controller = windowTemplatePaletteTreeDragController();
      const encoded = JSON.stringify({ windowKindId: "puzzle-3d-main", templateId: "perspective" });
      controller.pointerPaletteDrag?.begin(encoded);
      expect(windowTemplatePointerDragRef.active).toBe(true);
      controller.onDragStart?.({
        items: [],
        sourceItem: {
          id: "framework.display.windows.puzzle-3d-main.perspective",
          label: "Perspective",
          dragData: { [COMPOSE_WINDOW_TEMPLATE_MIME]: encoded },
        },
        section: { id: "framework.display.windows.puzzle-3d-main", items: [] },
      });
      expect(readActiveWindowTemplateDragSession()?.label).toBe("Perspective");
      cancelWindowTemplatePointerDrag();
      expect(windowTemplatePointerDragRef.active).toBe(false);
      expect(readActiveWindowTemplateDragSession()).toBeNull();
    });

    it("cancelWindowTemplatePointerDrag clears an active template drag session", () => {
      beginWindowTemplatePointerDrag(JSON.stringify({ windowKindId: "gis-map-main" }));
      beginWindowTemplateDrag({ payload: { windowKindId: "gis-map-main" }, label: "Map" });
      cancelWindowTemplatePointerDrag();
      expect(windowTemplatePointerDragRef.active).toBe(false);
      expect(readActiveWindowTemplateDragSession()).toBeNull();
    });

    it("reconcileWindows drops closed windows instead of re-adding them", () => {
      const layout: WindowLayoutNode = {
        kind: "stack",
        children: [
          { kind: "window", id: "a" },
          { kind: "window", id: "b" },
        ],
        activeId: "a",
      };
      const next = reconcileWindows(layout, ["a"]);
      expect(modeCollectWindowIds(next)).toEqual(["a"]);
    });

    it("resolveTranslationLabel exposes the empty shell notice", () => {
      const label = resolveTranslationLabel(uiI18n.t("ui.display.emptyShell"));
      expect(label).toBeTruthy();
      expect(label).toContain("Display");
    });

    it("insertWindowAtDropZone adds a window on root-split", () => {
      const layout: WindowLayoutNode = {
        kind: "stack",
        children: [{ kind: "window", id: "a" }],
      };
      const next = insertWindowAtDropZone(layout, "b", { kind: "root-split", side: "right" });
      expect(next.kind).toBe("row");
      const ids = next.kind === "row" ? next.children.flatMap((child) => (child.kind === "stack" ? child.children.map((leaf) => leaf.id) : [])) : [];
      expect(ids.sort()).toEqual(["a", "b"]);
    });

    it("computeModeDropZone root-split uses half of the mode when pointer is outside stack bodies", () => {
      const modeRect = { left: 0, top: 0, right: 400, bottom: 300, width: 400, height: 300 } as DOMRect;
      expect(computeModeDropZone(80, 150, new Map(), modeRect)).toEqual({ kind: "root-split", side: "left" });
      expect(computeModeDropZone(320, 150, new Map(), modeRect)).toEqual({ kind: "root-split", side: "right" });
      expect(computeModeDropZone(200, 40, new Map(), modeRect)).toEqual({ kind: "root-split", side: "top" });
      expect(computeModeDropZone(200, 260, new Map(), modeRect)).toEqual({ kind: "root-split", side: "bottom" });
    });

    it("modeDockOutLayout removes the dragged window without mutating drop targets", () => {
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
      const dockedOut = modeDockOutLayout(layout, { dragKind: "tab", windowId: "b", stackPath: "0" });
      expect(modeCollectWindowIds(dockedOut)).toEqual(["a", "c"]);
      expect(modeCollectWindowIds(layout)).toEqual(["a", "b", "c"]);
    });

    it("removeWindowFromLayout and splitWithWindow mutate the layout tree", () => {
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
      const removed = removeWindowFromLayout(layout, "b");
      expect(removed?.kind).toBe("row");
      const split = splitWithWindow(layout, "1", "b", "left");
      expect(split.kind).toBe("row");
      if (split.kind === "row") {
        const target = split.children[1];
        expect(target?.kind === "row" || target?.kind === "column").toBe(true);
      }
    });

    it("modeDockOutLayout removes an entire stack for stack drag", () => {
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
      const dockedOut = modeDockOutLayout(layout, { dragKind: "stack", windowId: "a", stackPath: "0" });
      expect(modeCollectWindowIds(dockedOut)).toEqual(["c"]);
      expect(modeCollectWindowIds(layout)).toEqual(["a", "b", "c"]);
    });

    it("applyModeDrop relocates a stack when dropping on another stack body edge", () => {
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
      const next = applyModeDrop(layout, drag, { kind: "split", stackPath: "1", side: "left" });
      expect(next.kind).toBe("row");
      if (next.kind !== "row") return;
      expect(next.children).toHaveLength(2);
      const left = next.children[0];
      const right = next.children[1];
      expect(left?.kind).toBe("stack");
      expect(right?.kind).toBe("stack");
      if (left?.kind === "stack") expect(left.children.map((child) => child.id)).toEqual(["a", "b"]);
      if (right?.kind === "stack") expect(right.children.map((child) => child.id)).toEqual(["c"]);
    });

    it("applyModeDrop splits within the same stack when dropping on a body edge zone", () => {
      const layout: WindowLayoutNode = {
        kind: "stack",
        children: [
          { kind: "window", id: "a" },
          { kind: "window", id: "b" },
        ],
        activeId: "a",
      };
      const drag = {
        dragKind: "tab" as const,
        windowId: "a",
        stackPath: "",
        tabIndex: 0,
        pointerId: 1,
        ghostLabel: "A",
        x: 0,
        y: 0,
      };
      const zone = { kind: "split" as const, stackPath: "", side: "right" as const };
      const next = applyModeDrop(layout, drag, zone);
      expect(next.kind).toBe("row");
      if (next.kind !== "row") return;
      expect(next.children).toHaveLength(2);
      const leftStack = next.children[0];
      const rightStack = next.children[1];
      expect(leftStack?.kind).toBe("stack");
      expect(rightStack?.kind).toBe("stack");
      if (leftStack?.kind === "stack") expect(leftStack.children.map((c) => c.id)).toEqual(["b"]);
      if (rightStack?.kind === "stack") expect(rightStack.children.map((c) => c.id)).toEqual(["a"]);
    });

    it("Mode maximize shows only one stack", () => {
      const { container } = render(
        <div className="h-layout-story w-layout-story-md">
          <Mode
            windows={[
              { id: "a", title: uiDataLabel("A"), iconId: "app-window", children: <div>A Body</div> },
              { id: "b", title: uiDataLabel("B"), iconId: "app-window", children: <div>B Body</div> },
            ]}
            layout={{
              kind: "row",
              children: [
                { kind: "stack", children: [{ kind: "window", id: "a" }], activeId: "a" },
                { kind: "stack", children: [{ kind: "window", id: "b" }], activeId: "b" },
              ],
            }}
            activeWindowId="a"
            onActiveWindowChange={() => {}}
          />
        </div>,
      );
      fireEvent.click(container.querySelector("[data-stack-path='0'] [data-slot='mode-dock-tab-focus']")!);
      expect(container.querySelector('[data-slot="mode"]')?.getAttribute("data-maximized-path")).toBe("0");
      expect(screen.getByText("A Body")).toBeTruthy();
      expect(screen.queryByText("B Body")).toBeNull();
    });

    it("Mode hides the Focus enlarge control when only one window is on the canvas", () => {
      const { container, rerender } = render(
        <div className="h-layout-story w-layout-story-md">
          <Mode windows={[{ id: "solo", title: uiDataLabel("Solo"), iconId: "app-window", children: <div>Solo Body</div> }]} activeWindowId="solo" onActiveWindowChange={() => {}} />
        </div>,
      );
      expect(container.querySelector('[data-slot="mode-dock-tab-focus"]')).toBeNull();
      expect(container.querySelector('[data-slot="mode-dock-tab-close"]')).toBeTruthy();
      rerender(
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
      expect(container.querySelectorAll('[data-slot="mode-dock-tab-focus"]').length).toBeGreaterThan(0);
    });

    it("Engagement renders options and status lines; Search renders the input", () => {
      const { container } = render(
        <>
          <Engagement options={[{ id: "opt-a", label: uiDataLabel("Option A"), icon: "circle-dot", onPress: () => {} }]} status={[{ id: "status-a", content: "Ready" }]} />
          <Search input={{ placeholder: uiDataLabel("Type here") }} />
        </>,
      );
      expect(screen.getByRole("button", { name: "OptionA" })).toBeTruthy();
      expect(screen.getByPlaceholderText("Type here")).toBeTruthy();
      expect(screen.getByText("Ready")).toBeTruthy();
      expect(container.querySelector('[data-slot="engagement"]')).toBeTruthy();
      expect(container.querySelector('[data-slot="search"]')).toBeTruthy();
    });

    it("Engagement option buttons size to label text without clipping", () => {
      const longLabel = uiDataLabel("C Confirm selection");
      const { container } = render(<Engagement options={[{ id: "engagement-transition-confirm-c", label: longLabel, icon: "circle-dot", onPress: () => {} }]} />);
      const item = container.querySelector('[data-slot="button-group-item"]') as HTMLElement;
      expect(item?.textContent).toContain("CConfirmSelection");
      expect(item?.className).toContain("aspect-auto");
      expect(item?.className).not.toContain("aspect-square");
    });

    it("Search focuses its input when active", async () => {
      const { rerender } = render(<Search active={false} input={{ id: "search-input", placeholder: uiDataLabel("Action") }} />);
      const field = () => screen.getByPlaceholderText("Action") as HTMLInputElement;
      expect(document.activeElement).not.toBe(field());
      rerender(<Search active input={{ id: "search-input", placeholder: uiDataLabel("Action") }} />);
      await waitFor(() => expect(document.activeElement).toBe(field()));
      expect(field().tabIndex).toBe(0);
    });

    it("Search input is removed from tab order when inactive", () => {
      render(<Search active={false} input={{ placeholder: uiDataLabel("Action") }} />);
      expect((screen.getByPlaceholderText("Action") as HTMLInputElement).tabIndex).toBe(-1);
    });

    it("filterSearchPossibles matches label, detail, and id", () => {
      const items = [
        { id: "primitive.box", label: "Box", detail: "b" },
        { id: "primitive.sphere", label: "Sphere", detail: "s" },
      ];
      expect(filterSearchPossibles("", items)).toHaveLength(2);
      expect(filterSearchPossibles("sph", items).map((row) => row.id)).toEqual(["primitive.sphere"]);
    });

    it("filterSearchPossibles ranks shorter label prefix matches first", () => {
      const items = [
        { id: "feature.extrudeWire", label: "ExtrudeWire", detail: "e" },
        { id: "surface.extrudeCrv", label: "ExtrudeCrv", detail: "e" },
      ];
      expect(filterSearchPossibles("Extr", items).map((row) => row.id)).toEqual(["surface.extrudeCrv", "feature.extrudeWire"]);
    });

    it("isSearchSuggestionActionTarget accepts text nodes inside action rows", () => {
      const row = document.createElement("div");
      row.setAttribute("data-slot", "command-item");
      const label = document.createTextNode("ExtrudeCrv");
      row.appendChild(label);
      document.body.appendChild(row);
      expect(isSearchSuggestionActionTarget({ target: label })).toBe(true);
      row.remove();
    });

    it("searchInlineCompletion uses label casing for matched name prefix", () => {
      const box = { id: "primitive.box", label: "Box", detail: "b" };
      const sphere = { id: "primitive.sphere", label: "Sphere", detail: "s" };
      expect(searchInlineCompletion("b", box)).toEqual({ prefix: "B", suffix: "ox" });
      expect(searchInlineCompletion("Sp", sphere)).toEqual({ prefix: "Sp", suffix: "here" });
      expect(searchInlineCompletion("sph", sphere)).toEqual({ prefix: "Sph", suffix: "ere" });
      expect(searchActiveInlineCompletion("Sp", [sphere], 0)).toEqual({ prefix: "Sp", suffix: "here" });
    });

    it("shouldActivateSearchPossibleOnConfirm accepts typed draft or expanded list", () => {
      expect(shouldActivateSearchPossibleOnConfirm("", false, 2)).toBe(false);
      expect(shouldActivateSearchPossibleOnConfirm("", true, 2)).toBe(true);
      expect(shouldActivateSearchPossibleOnConfirm("f", false, 2)).toBe(true);
      expect(shouldActivateSearchPossibleOnConfirm("f", false, 0)).toBe(false);
    });

    it("Search Space and Enter activate inline suggestion without opening possibles list", async () => {
      const scrollIntoView = Element.prototype.scrollIntoView;
      Element.prototype.scrollIntoView = () => undefined;
      const selected: string[] = [];
      render(
        <Search
          active
          input={{ placeholder: uiDataLabel("Action") }}
          possibles={[
            { id: "primitive.box", label: "Box", detail: "b", onSelect: () => selected.push("primitive.box") },
            { id: "primitive.sphere", label: "Sphere", detail: "s", onSelect: () => selected.push("primitive.sphere") },
            { id: "puzzle3d.utility.fill", label: "Fill", detail: "f", onSelect: () => selected.push("puzzle3d.utility.fill") },
          ]}
        />,
      );
      const field = screen.getByPlaceholderText("Action");
      expect(document.querySelector('[data-slot="search-autocomplete"]')).toBeNull();
      fireEvent.change(field, { target: { value: "f" } });
      await waitFor(() => expect(document.querySelector('[data-slot="search-inline-suffix"]')?.textContent).toBe("ill"));
      fireEvent.keyDown(field, { key: " " });
      await waitFor(() => expect(selected).toEqual(["puzzle3d.utility.fill"]));
      fireEvent.change(field, { target: { value: "Sp" } });
      fireEvent.keyDown(field, { key: "Enter" });
      await waitFor(() => expect(selected).toEqual(["puzzle3d.utility.fill", "primitive.sphere"]));
      Element.prototype.scrollIntoView = scrollIntoView;
    });

    it("Search shows inline completion while typing and possibles list only on chevron", async () => {
      const scrollIntoView = Element.prototype.scrollIntoView;
      Element.prototype.scrollIntoView = () => undefined;
      const selected: string[] = [];
      render(
        <Search
          active
          input={{ placeholder: uiDataLabel("Action") }}
          possibles={[
            { id: "primitive.box", label: "Box", detail: "b", onSelect: () => selected.push("primitive.box") },
            { id: "primitive.sphere", label: "Sphere", detail: "s", onSelect: () => selected.push("primitive.sphere") },
          ]}
        />,
      );
      const field = screen.getByPlaceholderText("Action");
      expect(document.querySelector('[data-slot="search-autocomplete"]')).toBeNull();
      fireEvent.change(field, { target: { value: "b" } });
      await waitFor(() => {
        expect(document.querySelector('[data-slot="search-inline-suffix"]')?.textContent).toBe("ox");
        expect(document.querySelector('[data-slot="search-inline-completion"]')?.querySelector(".font-semibold")?.textContent).toBe("B");
      });
      fireEvent.change(field, { target: { value: "Sp" } });
      await waitFor(() => {
        expect(document.querySelector('[data-slot="search-inline-suffix"]')?.textContent).toBe("here");
        expect(document.querySelector('[data-slot="search-inline-completion"]')?.textContent).toContain("Sphere");
      });
      expect(document.querySelector('[data-slot="search-autocomplete"]')).toBeNull();
      fireEvent.click(document.querySelector('[data-slot="search-possibles-toggle"]')!);
      await waitFor(() => expect(document.querySelector('[data-slot="search-autocomplete"]')).toBeTruthy());
      expect(document.querySelector('[data-value="primitive.sphere"]')).toBeTruthy();
      fireEvent.change(field, { target: { value: "sph" } });
      fireEvent.keyDown(field, { key: "Enter" });
      await waitFor(() => expect(selected).toEqual(["primitive.sphere"]));
      fireEvent.change(field, { target: { value: "" } });
      fireEvent.click(document.querySelector('[data-slot="search-possibles-toggle"]')!);
      fireEvent.keyDown(field, { key: " " });
      await waitFor(() => expect(selected).toEqual(["primitive.sphere", "primitive.box"]));
      fireEvent.change(field, { target: { value: "" } });
      fireEvent.click(document.querySelector('[data-slot="search-possibles-toggle"]')!);
      const sphereRow = document.querySelector('[data-slot="command-item"][data-value="primitive.sphere"]');
      expect(sphereRow).toBeTruthy();
      fireEvent.click(sphereRow!);
      await waitFor(() => expect(selected).toEqual(["primitive.sphere", "primitive.box", "primitive.sphere"]));
      Element.prototype.scrollIntoView = scrollIntoView;
    });

    it("Search suggestion click selects the picked row when popover keeps input focus", async () => {
      const scrollIntoView = Element.prototype.scrollIntoView;
      Element.prototype.scrollIntoView = () => undefined;
      const selected: string[] = [];
      render(
        <Search
          active
          input={{ placeholder: uiDataLabel("Action"), value: "Extr", onChange: () => {} }}
          possibles={[
            { id: "feature.extrudeWire", label: "ExtrudeWire", detail: "e", onSelect: () => selected.push("feature.extrudeWire") },
            { id: "surface.extrudeCrv", label: "ExtrudeCrv", detail: "e", onSelect: () => selected.push("surface.extrudeCrv") },
          ]}
        />,
      );
      fireEvent.click(document.querySelector('[data-slot="search-possibles-toggle"]')!);
      const popover = await waitFor(() => document.querySelector('[data-slot="search-autocomplete"]')!);
      const crvRow = document.querySelector('[data-slot="command-item"][data-value="surface.extrudeCrv"]')!;
      const crvLabel = crvRow.querySelector("span")?.firstChild;
      expect(crvLabel).toBeTruthy();
      fireEvent.pointerDown(popover, { pointerId: 1, pointerType: "mouse", buttons: 1 });
      fireEvent.pointerDown(crvLabel!, { pointerId: 1, pointerType: "mouse", buttons: 1 });
      fireEvent.click(crvRow);
      await waitFor(() => expect(selected).toEqual(["surface.extrudeCrv"]));
      Element.prototype.scrollIntoView = scrollIntoView;
    });

    it("Engagement renders toggleGroup and select controls", () => {
      const { container } = render(
        <Engagement
          sessionActive
          control={{
            kind: "toggleGroup",
            id: "engagement-utility-group",
            label: uiDataLabel("Utility"),
            value: "brush",
            options: [
              { id: "select", label: "Select", icon: "mouse-pointer" },
              { id: "brush", label: "Brush", icon: "paintbrush" },
            ],
          }}
          controls={[
            {
              kind: "select",
              id: "engagement-placement",
              label: uiDataLabel("Placement"),
              value: "a",
              items: [
                { id: "a", value: "a", label: "A" },
                { id: "b", value: "b", label: "B" },
              ],
            },
          ]}
        />,
      );
      expect(container.querySelector('[data-control-kind="toggleGroup"]')).toBeTruthy();
      expect(container.querySelector('[data-control-kind="select"]')).toBeTruthy();
    });

    it("applySearchSpaceAction submits empty draft during sessionActive", () => {
      const submitted: string[] = [];
      const repeated: string[] = [];
      const input = {
        onSubmit: (value: string) => submitted.push(value),
        onRepeatLast: () => repeated.push("last"),
      };
      expect(applySearchSpaceAction(input, "", true)).toBe(true);
      expect(submitted).toEqual([""]);
      expect(repeated).toEqual([]);
      submitted.length = 0;
      expect(applySearchSpaceAction(input, "", false)).toBe(true);
      expect(repeated).toEqual(["last"]);
      expect(submitted).toEqual([]);
    });

    it("routeWindowSearchSpace calls onRepeatLast for empty action", () => {
      const repeated: string[] = [];
      const search = { input: { value: "", onRepeatLast: () => repeated.push("last") } };
      const body = document.createElement("div");
      expect(
        routeWindowSearchSpace(search, {
          key: " ",
          ctrlKey: false,
          metaKey: false,
          altKey: false,
          defaultPrevented: false,
          isComposing: false,
          target: body,
        }),
      ).toBe(true);
      expect(repeated).toEqual(["last"]);
    });

    it("routeWindowSearchSpace calls onSubmit for non-empty action", () => {
      const submitted: string[] = [];
      const search = {
        input: {
          value: "Box",
          onSubmit: (value: string) => submitted.push(value),
          onRepeatLast: () => submitted.push("last"),
        },
      };
      const body = document.createElement("div");
      expect(
        routeWindowSearchSpace(search, {
          key: " ",
          ctrlKey: false,
          metaKey: false,
          altKey: false,
          defaultPrevented: false,
          isComposing: false,
          target: body,
        }),
      ).toBe(true);
      expect(submitted).toEqual(["Box"]);
    });

    it("Search Space with draft calls onSubmit instead of onRepeatLast", async () => {
      const submitted: string[] = [];
      const repeated: string[] = [];
      const Harness = () => {
        const [value, setValue] = reactHostPort.useState("");
        return (
          <Search
            active
            input={{
              value,
              placeholder: uiDataLabel("Action"),
              onChange: setValue,
              onSubmit: (draft) => submitted.push(draft),
              onRepeatLast: () => repeated.push("last"),
            }}
          />
        );
      };
      render(<Harness />);
      const field = await screen.findByPlaceholderText("Action");
      fireEvent.change(field, { target: { value: "box" } });
      fireEvent.keyDown(field, { key: " " });
      expect(submitted).toEqual(["Box"]);
      expect(repeated).toEqual([]);
    });

    it("Search Space with empty draft calls onRepeatLast instead of onSubmit", async () => {
      const submitted: string[] = [];
      const repeated: string[] = [];
      render(
        <Search
          active
          input={{
            placeholder: uiDataLabel("Action"),
            onSubmit: () => submitted.push("submit"),
            onRepeatLast: () => repeated.push("last"),
          }}
        />,
      );
      const field = await screen.findByPlaceholderText("Action");
      fireEvent.keyDown(field, { key: " " });
      expect(repeated).toEqual(["last"]);
      expect(submitted).toEqual([]);
    });

    it("Mode routes printable keys to the active window search", async () => {
      const Harness = () => {
        const [value, setValue] = reactHostPort.useState("");
        return (
          <TestShellRoot>
            {() => (
              <div className="h-layout-preview-md w-layout-floating-menu-lg">
                <Mode
                  activeWindowId="engagement-window"
                  windows={[
                    {
                      id: "engagement-window",
                      iconId: "search",
                      title: uiDataLabel("Viewport"),
                      active: true,
                      search: { input: { id: "search-input", value, placeholder: uiDataLabel("Action"), onChange: setValue } },
                      children: <div data-testid="window-body">Body</div>,
                    },
                  ]}
                  layout={{ kind: "stack", children: [{ kind: "window", id: "engagement-window" }] }}
                />
              </div>
            )}
          </TestShellRoot>
        );
      };
      const { container } = render(<Harness />);
      expect(screen.queryByPlaceholderText("Action")).toBeNull();
      fireEvent.keyDown(container.querySelector('[data-slot="mode"]')!, { key: "b", bubbles: true });
      await waitFor(() => {
        const typedField = screen.getByPlaceholderText("Action") as HTMLInputElement;
        expect(typedField.value).toBe("B");
        expect(typedField.tabIndex).toBe(0);
        expect(document.querySelector('[data-slot="search"]')?.getAttribute("data-active")).toBe("true");
      });
    });

    it("installElementsSurfaceBrowserDefaultSuppression blocks native context menu and Tab outside typing targets", () => {
      const bindings = createDOMEventBinding();
      installElementsSurfaceBrowserDefaultSuppression(bindings);
      const panel = document.createElement("div");
      document.body.appendChild(panel);
      const contextEvent = new MouseEvent("contextmenu", { bubbles: true, cancelable: true });
      const contextPrevent = vi.spyOn(contextEvent, "preventDefault");
      panel.dispatchEvent(contextEvent);
      expect(contextPrevent).toHaveBeenCalled();
      const tabEvent = new KeyboardEvent("keydown", { key: "Tab", bubbles: true, cancelable: true });
      const tabPrevent = vi.spyOn(tabEvent, "preventDefault");
      panel.dispatchEvent(tabEvent);
      expect(tabPrevent).toHaveBeenCalled();
      const input = document.createElement("input");
      document.body.appendChild(input);
      input.focus();
      const tabInField = new KeyboardEvent("keydown", { key: "Tab", bubbles: true, cancelable: true });
      const tabInFieldPrevent = vi.spyOn(tabInField, "preventDefault");
      input.dispatchEvent(tabInField);
      expect(tabInFieldPrevent).not.toHaveBeenCalled();
      bindings.dispose();
      panel.remove();
      input.remove();
    });

    it("installElementsSurfaceBrowserDefaultSuppression disables browser defaults on focused form controls", () => {
      const bindings = createDOMEventBinding();
      installElementsSurfaceBrowserDefaultSuppression(bindings);
      const input = document.createElement("input");
      document.body.appendChild(input);
      input.focus();
      expect(input.autocomplete).toBe("off");
      expect(input.spellcheck).toBe(false);
      expect(input.getAttribute("autocorrect")).toBe("off");
      expect(input.dataset.uiBrowserDefaults).toBe("true");
      bindings.dispose();
      input.remove();
    });

    it("isUiTypingTarget treats text inputs, collapsed fields, and command inputs as typing targets", () => {
      const text = document.createElement("input");
      text.type = "text";
      expect(isUiTypingTarget(text)).toBe(true);
      const checkbox = document.createElement("input");
      checkbox.type = "checkbox";
      expect(isUiTypingTarget(checkbox)).toBe(false);
      const collapsed = document.createElement("div");
      collapsed.setAttribute("data-slot", "input");
      collapsed.setAttribute("data-collapsed", "true");
      expect(isUiTypingTarget(collapsed)).toBe(true);
      const command = document.createElement("input");
      command.setAttribute("data-slot", "command-input");
      expect(isUiTypingTarget(command)).toBe(true);
      const writer = document.createElement("div");
      writer.setAttribute("role", "textbox");
      writer.tabIndex = 0;
      expect(isUiTypingTarget(writer)).toBe(true);
      expect(shouldRouteKeysToWindowSearch(writer)).toBe(false);
      expect(shouldRouteKeysToWindowSearch(text)).toBe(false);
    });

    it("Window keeps bodies edgeless and chrome-aware scroll hosts start below the dead line", () => {
      const { container } = render(
        <Window id="chrome-aware-window" active search={{ input: { placeholder: uiDataLabel("Action") } }} measures={<div>LOD</div>}>
          <div data-window-content-layout="chrome-aware" className="flex min-h-0 flex-1 flex-col">
            <Scrollable className="h-40">
              <div style={{ height: 240 }}>Line one</div>
            </Scrollable>
          </div>
        </Window>,
      );
      const body = container.querySelector('[data-slot="window-body"]');
      expect(body?.className).not.toContain("has-[[data-window-content-layout=chrome-aware]]:pt-");
      const scroller = container.querySelector('[data-slot="scroll-area"]') as HTMLDivElement;
      expect(scroller?.className).toContain("scroll-padding-top:var(--window-content-dead-line)");
      expect(scroller.scrollTop).toBeGreaterThan(0);
    });

    it("Window edgeless bodies do not apply a dead-line scroll offset", () => {
      const { container } = render(
        <Window id="edgeless-window" active search={{ input: { placeholder: uiDataLabel("Action") } }}>
          <div data-window-content-layout="edgeless" className="h-40">
            <Scrollable className="h-full">
              <div style={{ height: 240 }}>Canvas</div>
            </Scrollable>
          </div>
        </Window>,
      );
      const scroller = container.querySelector('[data-slot="scroll-area"]') as HTMLDivElement;
      expect(scroller.scrollTop).toBe(0);
    });

    it("Window dead-line hosts skip offset when content fits", () => {
      const { container } = render(
        <Window id="fits-window" active search={{ input: { placeholder: uiDataLabel("Action") } }}>
          <Scrollable className="h-40">
            <div style={{ height: 40 }}>Short</div>
          </Scrollable>
        </Window>,
      );
      const scroller = container.querySelector('[data-slot="scroll-area"]') as HTMLDivElement;
      Object.defineProperty(scroller, "clientHeight", { value: 160, configurable: true });
      Object.defineProperty(scroller, "scrollHeight", { value: 160, configurable: true });
      expect(readScrollerContentOverflows(scroller)).toBe(false);
    });

    it("Search does not steal focus from another input when it becomes active", async () => {
      const Harness = ({ active }: { active: boolean }) => (
        <>
          <Input id="other-input" placeholder="Other" />
          <Search active={active} input={{ placeholder: uiDataLabel("Action") }} />
        </>
      );
      const { rerender } = render(<Harness active={false} />);
      const other = screen.getByPlaceholderText("Other") as HTMLInputElement;
      other.focus();
      rerender(<Harness active />);
      await waitFor(() => expect(document.activeElement).toBe(other));
    });

    it("Search Escape calls onAbort when possibles list is closed", () => {
      const aborted: string[] = [];
      render(<Search active input={{ placeholder: uiDataLabel("Action"), onAbort: () => aborted.push("abort") }} possibles={[{ id: "a", label: "A", onSelect: () => {} }]} />);
      const field = screen.getByPlaceholderText("Action");
      fireEvent.keyDown(field, { key: "Escape" });
      expect(aborted).toEqual(["abort"]);
    });

    it("Search Escape closes possibles list before onAbort", () => {
      const scrollIntoView = Element.prototype.scrollIntoView;
      Element.prototype.scrollIntoView = () => undefined;
      const aborted: string[] = [];
      render(<Search active input={{ placeholder: uiDataLabel("Action"), onAbort: () => aborted.push("abort") }} possibles={[{ id: "a", label: "A", onSelect: () => {} }]} />);
      fireEvent.click(document.querySelector('[data-slot="search-possibles-toggle"]')!);
      const field = screen.getByPlaceholderText("Action");
      fireEvent.keyDown(field, { key: "Escape" });
      expect(aborted).toEqual([]);
      expect(document.querySelector('[data-slot="search-autocomplete"]')).toBeNull();
      fireEvent.keyDown(field, { key: "Escape" });
      expect(aborted).toEqual(["abort"]);
      Element.prototype.scrollIntoView = scrollIntoView;
    });

    it("routeWindowSearchEscape calls onAbort when sessionActive without visible chrome", () => {
      const aborted: string[] = [];
      const search: SearchSpec = {
        sessionActive: true,
        input: { value: "", placeholder: uiDataLabel("Brush"), onAbort: () => aborted.push("abort") },
      };
      const handled = routeWindowSearchEscape(search, { key: "Escape", defaultPrevented: false, isComposing: false, target: document.body }, { chromeVisible: false, actionActive: false });
      expect(handled).toBe(true);
      expect(aborted).toEqual(["abort"]);
    });

    it("Mode Escape aborts active window search", async () => {
      const aborted: string[] = [];
      const Harness = () => (
        <TestShellRoot>
          {() => (
            <div className="h-layout-preview-md w-layout-floating-menu-lg">
              <Mode
                activeWindowId="engagement-window"
                windows={[
                  {
                    id: "engagement-window",
                    iconId: "search",
                    title: uiDataLabel("Viewport"),
                    active: true,
                    search: {
                      input: { value: "Box", placeholder: uiDataLabel("Action"), onChange: () => {}, onAbort: () => aborted.push("abort") },
                    },
                    children: <div data-testid="window-body">Body</div>,
                  },
                ]}
                layout={{ kind: "stack", children: [{ kind: "window", id: "engagement-window" }] }}
              />
            </div>
          )}
        </TestShellRoot>
      );
      const { container } = render(<Harness />);
      fireEvent.click(container.querySelector('[id="framework.window.engagementWindow.search.toggle"]')!);
      await waitFor(() => expect(screen.getByPlaceholderText("Action")).toBeTruthy());
      fireEvent.keyDown(container.querySelector('[data-slot="mode"]')!, { key: "Escape", bubbles: true });
      expect(aborted).toEqual(["abort"]);
    });

    it("Window shows engagement and search as folded strips by default, same U-cutout surface as window options", () => {
      const { container } = render(
        <Window id="engagement-window" active engagement={{ sessionActive: true, status: [{ id: "s", content: "Idle" }] }} search={{ sessionActive: true, input: { value: "Box", placeholder: uiDataLabel("Action") } }}>
          <div>Body</div>
        </Window>,
      );
      const engagementZone = container.querySelector('[data-slot="window-engagement-zone"]') as HTMLElement;
      const searchZone = container.querySelector('[data-slot="window-search-zone"]') as HTMLElement;
      expect(engagementZone).toBeTruthy();
      expect(engagementZone.getAttribute("data-level")).toBe("pane");
      expect(engagementZone.querySelector('[data-slot="window-chrome-silhouette-border"]')).toBeTruthy();
      expect(searchZone).toBeTruthy();
      expect(searchZone.getAttribute("data-level")).toBe("pane");
      expect(searchZone.querySelector('[data-slot="window-chrome-silhouette-border"]')).toBeTruthy();
      expect(screen.queryByPlaceholderText("Action")).toBeNull();
      expect(screen.queryByText("Idle")).toBeNull();
      fireEvent.click(container.querySelector('[id="framework.window.engagementWindow.engagement.toggle"]')!);
      expect(screen.getByText("Idle")).toBeTruthy();
      fireEvent.click(container.querySelector('[id="framework.window.engagementWindow.search.toggle"]')!);
      expect(screen.getByPlaceholderText("Action")).toBeTruthy();
      expect(container.querySelector('[data-slot="window-engagement-body"]')).toBeTruthy();
      expect(container.querySelector('[data-slot="window-search-body"]')).toBeTruthy();
    });

    it("Window merges the ad-hoc actionPane into the top-left Actions pane below the active engagement, sharing one toggle", () => {
      const { container } = render(
        <Window id="merged-window" active engagement={{ status: [{ id: "s", content: "Idle" }] }} actionPane={<div data-testid="adhoc-actions">Extrude…</div>}>
          <div>Body</div>
        </Window>,
      );
      expect(container.querySelector('[data-slot="window-action-pane-overlay"]')).toBeNull();
      expect(container.querySelector('[data-testid="adhoc-actions"]')).toBeNull();
      const toggle = container.querySelector('[id="framework.window.mergedWindow.engagement.toggle"]') as HTMLElement;
      expect(toggle).toBeTruthy();
      fireEvent.click(toggle);
      expect(screen.getByText("Idle")).toBeTruthy();
      const actions = screen.getByTestId("adhoc-actions");
      expect(actions).toBeTruthy();
      const body = container.querySelector('[data-slot="window-engagement-body"]');
      expect(body?.contains(actions)).toBe(true);
      fireEvent.click(container.querySelector('[data-slot="pane-fold"]')!);
      expect(container.querySelector('[data-testid="adhoc-actions"]')).toBeNull();
    });

    it("Window shows the merged Actions pane for an actionPane-only window with no active engagement", () => {
      const { container } = render(
        <Window id="actions-only-window" active actionPane={<div data-testid="adhoc-actions">Flatten</div>}>
          <div>Body</div>
        </Window>,
      );
      const overlay = container.querySelector('[data-slot="window-engagement-overlay"]');
      expect(overlay).toBeTruthy();
      fireEvent.click(container.querySelector('[id="framework.window.actionsOnlyWindow.engagement.toggle"]')!);
      expect(screen.getByTestId("adhoc-actions")).toBeTruthy();
    });

    it("Window reveals search on button click and activates on click", async () => {
      const Harness = () => {
        const [value, setValue] = reactHostPort.useState("");
        return (
          <Window id="engagement-window" active search={{ input: { id: "search-input", value, placeholder: uiDataLabel("Action"), onChange: setValue } }}>
            <div data-testid="window-body">Body</div>
          </Window>
        );
      };
      const { container } = render(<Harness />);
      const toggleBtn = container.querySelector('[id="framework.window.engagementWindow.search.toggle"]') as HTMLElement;
      expect(toggleBtn).toBeTruthy();
      expect(screen.queryByPlaceholderText("Action")).toBeNull();
      fireEvent.click(toggleBtn);
      const activeField = await waitFor(() => {
        const next = screen.getByPlaceholderText("Action") as HTMLInputElement;
        expect(document.activeElement).toBe(next);
        return next;
      });
      expect(activeField.tabIndex).toBe(0);
      expect(document.querySelector('[data-slot="search"]')?.getAttribute("data-active")).toBe("true");
      fireEvent.change(activeField, { target: { value: "b" } });
      await waitFor(() => {
        expect(activeField.value).toBe("B");
      });
    });

    it("Search onChange PascalCases spaced action without window routing", () => {
      const changed: string[] = [];
      render(<Search input={{ value: "", onChange: (next) => changed.push(next), placeholder: uiDataLabel("Action") }} />);
      fireEvent.change(screen.getByPlaceholderText("Action"), { target: { value: "set height" } });
      expect(changed).toEqual(["SetHeight"]);
    });

    it("normalizeEngagementActionText strips separators and PascalCases tokens", () => {
      expect(normalizeEngagementActionText("set height 5")).toBe("SetHeight5");
      expect(normalizeEngagementActionText("b ")).toBe("B");
      expect(normalizeEngagementActionText("box")).toBe("Box");
      expect(normalizeEngagementActionText("SetHeight")).toBe("SetHeight");
    });

    it("normalizeEngagementActionText preserves decimal points inside numbers", () => {
      expect(normalizeEngagementActionText("set height 3.5")).toBe("SetHeight3.5");
      expect(normalizeEngagementActionText("3.5")).toBe("3.5");
      expect(normalizeEngagementActionText("0.25")).toBe("0.25");
      expect(normalizeEngagementActionText("dist 12.75")).toBe("Dist12.75");
    });

    it("engagementActionTokenEquals matches tokens regardless of casing", () => {
      expect(engagementActionTokenEquals("brush", "Brush")).toBe(true);
      expect(engagementActionTokenEquals("SELECT", "select")).toBe(true);
      expect(engagementActionTokenEquals("box", "sphere")).toBe(false);
    });

    it("Search input PascalCases action text and space confirms like enter", async () => {
      const submitted: string[] = [];
      const Harness = () => {
        const [value, setValue] = reactHostPort.useState("SetHeight");
        return (
          <Search
            active
            input={{
              id: "search-input",
              value,
              placeholder: uiDataLabel("Action"),
              onChange: setValue,
              onSubmit: (next) => submitted.push(next),
            }}
          />
        );
      };
      render(<Harness />);
      const field = screen.getByPlaceholderText("Action") as HTMLInputElement;
      expect(field.value).toBe("SetHeight");
      fireEvent.keyDown(field, { key: " " });
      await waitFor(() => expect(submitted).toEqual(["SetHeight"]));
      fireEvent.keyDown(field, { key: "Enter" });
      await waitFor(() => expect(submitted).toEqual(["SetHeight", "SetHeight"]));
    });

    it("Window anchors engagement in a top overlay when active", () => {
      const { container } = render(
        <Window id="engagement-window" active engagement={{ status: [{ id: "s", content: "Idle" }] }}>
          <div>Body</div>
        </Window>,
      );
      const overlay = container.querySelector('[data-slot="window-engagement-overlay"]') as HTMLElement;
      const toggleBtn = container.querySelector('[id="framework.window.engagementWindow.engagement.toggle"]') as HTMLElement;
      expect(overlay).toBeTruthy();
      expect(overlay.style.top).toBe("var(--spacing-single)");
      expect(overlay.style.left).toBe("var(--spacing-single)");
      expect(overlay.querySelector('[data-slot="window-chrome-silhouette-border"]')).toBeTruthy();
      expect(overlay.getAttribute("data-folded")).toBe("true");
      expect(screen.queryByText("Idle")).toBeNull();
      fireEvent.click(toggleBtn);
      expect(overlay.getAttribute("data-folded")).toBeNull();
      expect(overlay.style.width).toBe(`${windowMeasuresDefaultWidthPx}px`);
      expect(screen.getByText("Idle")).toBeTruthy();
    });

    it("Window search action row spans the sized search zone", () => {
      const { container } = render(
        <Window id="engagement-window" active search={{ input: { placeholder: uiDataLabel("Action") } }}>
          <div>Body</div>
        </Window>,
      );
      const overlay = container.querySelector('[data-slot="window-search-overlay"]') as HTMLElement;
      const zone = container.querySelector('[data-slot="window-search-zone"]') as HTMLElement;
      const toggleBtn = container.querySelector('[id="framework.window.engagementWindow.search.toggle"]') as HTMLElement;
      fireEvent.click(toggleBtn);
      expect(zone.className).toContain("w-full");
      expect(overlay.style.width).toBe(`${windowMeasuresDefaultWidthPx}px`);
      const row = container.querySelector('[data-slot="search-row"]');
      const inputRoot = container.querySelector('[data-slot="search-input"] [data-slot="input-root"]');
      expect(row?.className).toContain("w-full");
      expect(inputRoot?.className).toContain("w-full");
    });

    it("Window and ModeDock controls render with labels", () => {
      const { container } = render(
        <Window id="labeled-window" onOpenInNewWindow={() => {}} onMaximize={() => {}} onClose={() => {}} measures={<div data-testid="measure-slot">LOD</div>}>
          <div>Body</div>
        </Window>,
      );

      const newWindowBtn = container.querySelector('[id="framework.window.labeledWindow.windowControls.external"]');
      expect(newWindowBtn?.textContent?.trim()).toBe("New Window");

      const closeBtn = container.querySelector('[id="framework.window.labeledWindow.windowControls.close"]');
      expect(closeBtn?.textContent?.trim()).toBe("Close");

      const maximizeBtn = container.querySelector('[id="framework.window.labeledWindow.windowControls.maximize"]');
      expect(maximizeBtn?.textContent?.trim()).toBe("Focus");

      // Verify folded state options label
      const unfoldBtn = container.querySelector('[id="framework.window.labeledWindow.measures.unfold"]');
      expect(unfoldBtn?.textContent?.trim()).toBe("Window Options");

      // Unfold it
      if (unfoldBtn) fireEvent.click(unfoldBtn);

      // Verify unfolded state option labels (Enlarge/Span button & Fold button)
      const spanBtn = container.querySelector('[id="framework.window.labeledWindow.measures.span"]');
      expect(spanBtn?.textContent?.trim()).toBe("Focus");

      const foldBtn = container.querySelector('[id="framework.window.labeledWindow.measures.fold"]');
      expect(foldBtn?.textContent?.trim()).toBe("Window Options");
    });

    it("Window maximize renders as Unfocus when onMinimize is provided", () => {
      const { container } = render(
        <Window id="labeled-window-min" onMinimize={() => {}}>
          <div>Body</div>
        </Window>,
      );
      const maximizeBtn = container.querySelector('[id="framework.window.labeledWindowMin.windowControls.maximize"]');
      expect(maximizeBtn?.textContent?.trim()).toBe("Unfocus");
    });

    it("Window drops the Focus control on mobile — windows always take the full space — but keeps Close", () => {
      const { container } = render(
        <UiMobileProvider mobile>
          <Window id="mobile-window" onOpenInNewWindow={() => {}} onMaximize={() => {}} onClose={() => {}}>
            <div>Body</div>
          </Window>
        </UiMobileProvider>,
      );
      expect(container.querySelector('[id="framework.window.mobileWindow.windowControls.maximize"]')).toBeNull();
      expect(container.querySelector('[id="framework.window.mobileWindow.windowControls.external"]')).toBeTruthy();
      expect(container.querySelector('[id="framework.window.mobileWindow.windowControls.close"]')).toBeTruthy();
    });

    it("Window search pane uses the same default width as panels when unfolded", () => {
      const { container } = render(
        <Window id="layout-window" active search={{ input: { placeholder: uiDataLabel("Action") } }} measures={<div data-testid="measure-slot">LOD</div>}>
          <div data-testid="window-body">Body</div>
        </Window>,
      );
      const overlay = container.querySelector('[data-slot="window-search-overlay"]') as HTMLElement;
      fireEvent.click(container.querySelector('[id="framework.window.layoutWindow.search.toggle"]')!);
      expect(overlay.style.width).toBe(`${windowMeasuresDefaultWidthPx}px`);
      expect(windowMeasuresDefaultWidthPx).toBe(300);
    });

    it("Window engagement and measures overlays pass pointer hits through to the canvas body", () => {
      const bodyDown = vi.fn();
      const { container } = render(
        <Window id="canvas-window" active fill engagement={{ options: [{ id: "opt-a", label: uiDataLabel("Alpha"), icon: "circle-dot", onPress: () => {} }] }} measures={<div data-testid="measure-slot">LOD</div>}>
          <div data-testid="window-body" className="size-full" onPointerDown={bodyDown}>
            Body
          </div>
        </Window>,
      );
      const body = container.querySelector('[data-testid="window-body"]') as HTMLElement;
      const bodyRect = body.getBoundingClientRect();
      fireEvent.pointerDown(body, { clientX: bodyRect.right - 8, clientY: bodyRect.top + bodyRect.height * 0.55, bubbles: true });
      expect(bodyDown).toHaveBeenCalledTimes(1);
      fireEvent.pointerDown(body, { clientX: bodyRect.left + 12, clientY: bodyRect.top + 12, bubbles: true });
      expect(bodyDown).toHaveBeenCalledTimes(2);
    });

    it("Window hides search overlay when measures are fullscreen", () => {
      const { container } = render(
        <Window id="measures-engagement-window" active search={{ input: { placeholder: uiDataLabel("Action") } }} measures={<div data-testid="measure-slot">LOD</div>}>
          <div>Body</div>
        </Window>,
      );
      fireEvent.click(container.querySelector('[id="framework.window.measuresEngagementWindow.search.toggle"]')!);
      fireEvent.click(container.querySelector('[id="framework.window.measuresEngagementWindow.measures.unfold"]')!);
      expect(screen.getByPlaceholderText("Action")).toBeTruthy();
      fireEvent.click(container.querySelector('[id="framework.window.measuresEngagementWindow.measures.span"]')!);
      expect(container.querySelector('[data-slot="window-measures-overlay"]')?.getAttribute("data-expanded")).toBe("true");
      expect(container.querySelector('[data-slot="window-search-overlay"]')).toBeNull();
      expect(screen.queryByPlaceholderText("Action")).toBeNull();
      fireEvent.click(container.querySelector('[id="framework.window.measuresEngagementWindow.measures.span"]')!);
      expect(container.querySelector('[data-slot="window-search-overlay"]')).toBeTruthy();
    });

    it("Window keeps engagement and search pane toggles visible while inactive, like window options", () => {
      const { container } = render(
        <Window id="engagement-window" engagement={{ status: [{ id: "s", content: "Idle" }] }} search={{ input: { placeholder: uiDataLabel("Action") } }} measures={<div data-testid="measure-slot">LOD</div>}>
          <div>Body</div>
        </Window>,
      );
      expect(container.querySelector('[data-slot="window"]')?.getAttribute("data-active")).toBeNull();
      expect(container.querySelector('[data-slot="window-engagement-overlay"]')).toBeTruthy();
      expect(container.querySelector('[data-slot="window-search-overlay"]')).toBeTruthy();
      expect(container.querySelector('[data-slot="window-measures-overlay"]')).toBeTruthy();
      expect(container.querySelector('[data-slot="utility-bar-overlay"]')).toBeTruthy();
      expect(container.querySelector('[id="framework.window.engagementWindow.engagement.toggle"]')).toBeTruthy();
      expect(container.querySelector('[id="framework.window.engagementWindow.search.toggle"]')).toBeTruthy();
      expect(container.querySelector('[id="framework.window.engagementWindow.measures.unfold"]')).toBeTruthy();
      expect(screen.queryByPlaceholderText("Action")).toBeNull();
      expect(screen.queryByText("Idle")).toBeNull();
      fireEvent.click(container.querySelector('[id="framework.window.engagementWindow.engagement.toggle"]')!);
      expect(screen.getByText("Idle")).toBeTruthy();
      fireEvent.click(container.querySelector('[id="framework.window.engagementWindow.search.toggle"]')!);
      expect(screen.getByPlaceholderText("Action")).toBeTruthy();
    });

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
      const span = container.querySelector('[id="framework.window.measuresFoldWindow.measures.span"]');
      const fold = container.querySelector('[id="framework.window.measuresFoldWindow.measures.fold"]');
      expect(span).toBeTruthy();
      expect(fold).toBeTruthy();
      expect(container.querySelector('[data-slot="window-measures-body"]')).toBeTruthy();
      fireEvent.click(container.querySelector('[id="framework.window.measuresFoldWindow.measures.fold"]')!);
      expect(container.querySelector('[data-slot="window-measures-body"]')).toBeNull();
      expect(container.querySelector('[id="framework.window.measuresFoldWindow.measures.span"]')).toBeNull();
      expect(container.querySelector('[data-slot="window-measures-stack"]')?.getAttribute("data-folded")).toBe("true");
      expect(overlay.getAttribute("data-folded")).toBe("true");
      fireEvent.click(container.querySelector('[id="framework.window.measuresFoldWindow.measures.unfold"]')!);
      expect(container.querySelector('[data-slot="window-measures-body"]')).toBeTruthy();
      expect(container.querySelector('[data-slot="window-measures-stack"]')?.getAttribute("data-folded")).toBeNull();
      expect(overlay.style.width).toBe(`${windowMeasuresDefaultWidthPx}px`);
      expect(container.querySelector('[data-slot="pane-resize-handle"]')).toBeTruthy();
    });

    it("Window measures chrome span expands the overlay across the window body", () => {
      const { container } = render(
        <Window id="measures-span-window" measures={<div data-testid="measure-slot">LOD</div>}>
          <div>Body</div>
        </Window>,
      );
      fireEvent.click(container.querySelector('[id="framework.window.measuresSpanWindow.measures.unfold"]')!);
      const overlay = container.querySelector('[data-slot="window-measures-overlay"]') as HTMLElement;
      expect(overlay.getAttribute("data-expanded")).toBeNull();
      expect(container.querySelector('[data-slot="pane-resize-handle"]')).toBeTruthy();
      fireEvent.click(container.querySelector('[id="framework.window.measuresSpanWindow.measures.span"]')!);
      expect(overlay.getAttribute("data-expanded")).toBe("true");
      expect(overlay.style.inset).toBe("0");
      expect(container.querySelector('[data-slot="pane-resize-handle"]')).toBeNull();
      fireEvent.click(container.querySelector('[id="framework.window.measuresSpanWindow.measures.span"]')!);
      expect(overlay.getAttribute("data-expanded")).toBeNull();
      expect(overlay.style.right).toBe("var(--spacing-single)");
      expect(container.querySelector('[data-slot="pane-resize-handle"]')).toBeTruthy();
    });

    it("Window measures chrome fold collapses span and expanded overlay", () => {
      const { container } = render(
        <Window id="measures-fold-expanded-window" measures={<div data-testid="measure-slot">LOD</div>}>
          <div>Body</div>
        </Window>,
      );
      fireEvent.click(container.querySelector('[id="framework.window.measuresFoldExpandedWindow.measures.unfold"]')!);
      const overlay = container.querySelector('[data-slot="window-measures-overlay"]') as HTMLElement;
      fireEvent.click(container.querySelector('[id="framework.window.measuresFoldExpandedWindow.measures.span"]')!);
      expect(overlay.getAttribute("data-expanded")).toBe("true");
      fireEvent.click(container.querySelector('[id="framework.window.measuresFoldExpandedWindow.measures.fold"]')!);
      expect(container.querySelector('[id="framework.window.measuresFoldExpandedWindow.measures.span"]')).toBeNull();
      expect(overlay.getAttribute("data-expanded")).toBeNull();
      expect(overlay.getAttribute("data-folded")).toBe("true");
      expect(overlay.className).toContain("w-fit");
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

    it("Window utility bar anchors bottom-left and hides when measures span the window", () => {
      const { container } = render(
        <Window id="utility-bar-measures-window" utilityBar={<button type="button">Utility</button>} measures={<div data-testid="measure-slot">LOD</div>}>
          <div>Body</div>
        </Window>,
      );
      const overlay = container.querySelector('[data-slot="utility-bar-overlay"]') as HTMLElement;
      expect(overlay.style.bottom).toBe("var(--spacing-single)");
      expect(overlay.style.left).toBe("var(--spacing-single)");
      fireEvent.click(container.querySelector('[id="framework.window.utilityBarMeasuresWindow.measures.unfold"]')!);
      fireEvent.click(container.querySelector('[id="framework.window.utilityBarMeasuresWindow.measures.span"]')!);
      expect(container.querySelector('[data-slot="utility-bar-overlay"]')).toBeNull();
    });

    it("Window utility bar body caps its height below the top-left Actions chrome and scrolls the overflow, instead of overlapping it (regression guard for a tall active-utility options tree, e.g. puzzle 3D's Füllen tree)", () => {
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
        HTMLElement.prototype.getBoundingClientRect = originalRect;
      }
    });

    describe("Pane", () => {
      it("nearestAnchor resolves each of the eight zones, incl. the dead-center zone by distance from the nearer midline", () => {
        const hostRect = { left: 0, top: 0, width: 300, height: 300 };
        expect(nearestAnchor(10, 10, hostRect)).toBe("top-left");
        expect(nearestAnchor(150, 10, hostRect)).toBe("top-middle");
        expect(nearestAnchor(290, 10, hostRect)).toBe("top-right");
        expect(nearestAnchor(10, 150, hostRect)).toBe("left-middle");
        expect(nearestAnchor(290, 150, hostRect)).toBe("right-middle");
        expect(nearestAnchor(10, 290, hostRect)).toBe("bottom-left");
        expect(nearestAnchor(150, 290, hostRect)).toBe("bottom-middle");
        expect(nearestAnchor(290, 290, hostRect)).toBe("bottom-right");
        // Dead center, nudged slightly toward the vertical midline (closer horizontally than vertically) -> the nearer side-middle.
        expect(nearestAnchor(160, 155, hostRect)).toBe("right-middle");
        // Dead center, nudged slightly toward the horizontal midline -> the nearer top/bottom-middle.
        expect(nearestAnchor(155, 160, hostRect)).toBe("bottom-middle");
      });

      it("Pane defaults to a folded chip when folded is omitted", () => {
        const { container } = render(
          <PaneHost>
            <Pane id="default-folded-pane" anchor="top-left" icon="box" label={uiDataLabel("Pane")}>
              <div data-testid="default-folded-content">Content</div>
            </Pane>
          </PaneHost>,
        );
        expect(container.querySelector('[data-slot="pane"]')?.getAttribute("data-folded")).toBe("true");
        expect(screen.queryByTestId("default-folded-content")).toBeNull();
      });

      it("Pane defaults open width to the panel default (300px) when unfolded", () => {
        const { container } = render(
          <PaneHost>
            <Pane id="default-width-pane" anchor="top-left" icon="box" label={uiDataLabel("Pane")} folded={false}>
              <div>Content</div>
            </Pane>
          </PaneHost>,
        );
        const pane = container.querySelector('[data-slot="pane"]') as HTMLElement;
        expect(pane.style.width).toBe("300px");
        expect(container.querySelector('[data-slot="window-chrome-gap"]')).toBeTruthy();
      });

      it("Pane positions itself via the same anchorPositionStyle math as Panel and folds to a chip", () => {
        const { container, rerender } = render(
          <PaneHost>
            <Pane id="test-pane" anchor="top-right" icon="box" label={uiDataLabel("Test")} folded={false}>
              <div data-testid="pane-content">Content</div>
            </Pane>
          </PaneHost>,
        );
        const pane = container.querySelector('[data-slot="pane"]') as HTMLElement;
        expect(pane.getAttribute("data-anchor")).toBe("top-right");
        expect(pane.style.right).toBe("var(--spacing-single)");
        expect(pane.style.top).toBe("var(--spacing-single)");
        expect(screen.getByTestId("pane-content")).toBeTruthy();
        rerender(
          <PaneHost>
            <Pane id="test-pane" anchor="top-right" icon="box" label={uiDataLabel("Test")} folded>
              <div data-testid="pane-content">Content</div>
            </Pane>
          </PaneHost>,
        );
        expect(screen.queryByTestId("pane-content")).toBeNull();
      });

      it("Pane opens its own pane level with chip glass above a transparent clipped payload", () => {
        const { container } = render(
          <PaneHost>
            <Pane id="level-pane" anchor="top-right" icon="box" label={uiDataLabel("Test")} folded={false}>
              <div data-testid="pane-level-content">{"placeholder"}</div>
            </Pane>
          </PaneHost>,
        );
        const pane = container.querySelector('[data-slot="pane"]') as HTMLElement;
        expect(pane.getAttribute("data-level")).toBe("pane");
        const chromeStack = container.querySelector('[data-slot="window-chrome-stack"]') as HTMLElement;
        expect(chromeStack.getAttribute("data-level")).toBe("pane");
        const chip = container.querySelector('[data-slot="window-chrome-chip-cap"]') as HTMLElement;
        expect(chip.className).toContain("ui-glass");
        const body = container.querySelector('[data-slot="pane-body"]') as HTMLElement;
        expect(body.hasAttribute("data-window-silhouette-content")).toBe(true);
        expect(body.className).not.toContain("ui-glass");
      });

      it("Pane grows down from top anchors, up from bottom anchors, and symmetrically around middle anchors within responsive bounds", () => {
        const { container, rerender } = render(
          <PaneHost>
            <Pane id="direction-pane" anchor="top-left" icon="box" label={uiDataLabel("Direction")} folded={false}>
              <div>Content</div>
            </Pane>
          </PaneHost>,
        );
        const pane = () => container.querySelector('[data-slot="pane"]') as HTMLElement;
        expect(pane().className).toContain("flex-col");
        expect(pane().className).not.toContain("flex-col-reverse");
        expect(pane().style.top).toBe("var(--spacing-single)");
        expect(pane().style.maxWidth).toBe("min(100% - (var(--spacing-single) * 2), 300px)");
        expect(pane().style.maxHeight).toBe("calc(100% - (var(--spacing-single) * 2))");

        rerender(
          <PaneHost>
            <Pane id="direction-pane" anchor="bottom-left" icon="box" label={uiDataLabel("Direction")} folded={false}>
              <div>Content</div>
            </Pane>
          </PaneHost>,
        );
        expect(pane().className).toContain("flex-col-reverse");
        expect(pane().style.bottom).toBe("var(--spacing-single)");

        rerender(
          <PaneHost>
            <Pane id="direction-pane" anchor="bottom-middle" icon="box" label={uiDataLabel("Direction")} folded={false}>
              <div>Content</div>
            </Pane>
          </PaneHost>,
        );
        expect(pane().className).toContain("flex-col-reverse");
        expect(pane().className).toContain("items-center");
        expect(pane().style.bottom).toBe("var(--spacing-single)");
        expect(pane().style.left).toBe("50%");
        expect(pane().style.transform).toBe("translateX(-50%)");

        rerender(
          <PaneHost>
            <Pane id="direction-pane" anchor="left-middle" icon="box" label={uiDataLabel("Direction")} folded={false}>
              <div>Content</div>
            </Pane>
          </PaneHost>,
        );
        expect(pane().style.top).toBe("50%");
        expect(pane().style.transform).toBe("translateY(-50%)");
      });

      it("Pane chrome toggle folds and unfolds when onFoldToggle is wired", () => {
        let folded = true;
        const onFoldToggle = vi.fn(() => {
          folded = !folded;
        });
        const { container, rerender } = render(
          <PaneHost>
            <Pane id="fold-pane" anchor="bottom-right" icon="camera" label={uiDataLabel("Projection")} folded={folded} onFoldToggle={onFoldToggle}>
              <div data-testid="fold-pane-content">Modes</div>
            </Pane>
          </PaneHost>,
        );
        expect(screen.queryByTestId("fold-pane-content")).toBeNull();
        expect(container.querySelector('[data-slot="pane"]')?.getAttribute("data-anchor")).toBe("bottom-right");
        fireEvent.click(container.querySelector('[data-slot="window-pane-chrome-toggle"]')!);
        expect(onFoldToggle).toHaveBeenCalledTimes(1);
        rerender(
          <PaneHost>
            <Pane id="fold-pane" anchor="bottom-right" icon="camera" label={uiDataLabel("Projection")} folded={folded} onFoldToggle={onFoldToggle}>
              <div data-testid="fold-pane-content">Modes</div>
            </Pane>
          </PaneHost>,
        );
        expect(screen.getByTestId("fold-pane-content")).toBeTruthy();
        fireEvent.click(container.querySelector('[data-slot="window-pane-chrome-toggle"]')!);
        expect(onFoldToggle).toHaveBeenCalledTimes(2);
        rerender(
          <PaneHost>
            <Pane id="fold-pane" anchor="bottom-right" icon="camera" label={uiDataLabel("Projection")} folded={folded} onFoldToggle={onFoldToggle}>
              <div data-testid="fold-pane-content">Modes</div>
            </Pane>
          </PaneHost>,
        );
        expect(screen.queryByTestId("fold-pane-content")).toBeNull();
      });

      it("Pane isolates toggle pointer events from a pointer-capturing canvas host", () => {
        const hostPointerDown = vi.fn();
        const onFoldToggle = vi.fn();
        const { container } = render(
          <UiDriverProvider driver={COMPACT_UI_DRIVER}>
            <div onPointerDown={hostPointerDown}>
              <PaneHost>
                <Pane id="projection-pane" anchor="bottom-right" icon="camera" label={uiDataLabel("Projection")} folded onFoldToggle={onFoldToggle}>
                  <div>Modes</div>
                </Pane>
              </PaneHost>
            </div>
          </UiDriverProvider>,
        );
        const toggle = container.querySelector('[data-slot="window-pane-chrome-toggle"]') as HTMLElement;
        fireEvent.pointerDown(toggle, { pointerId: 1, clientX: 10, clientY: 10 });
        fireEvent.pointerUp(toggle, { pointerId: 1, clientX: 10, clientY: 10 });
        fireEvent.click(toggle);
        expect(hostPointerDown).not.toHaveBeenCalled();
        expect(onFoldToggle).toHaveBeenCalledTimes(1);
      });

      it("Pane drag handle calls onAnchorChange once per anchor crossed, resolved against the PaneHost's bounds", () => {
        let anchor: Anchor = "top-left";
        const onAnchorChange = vi.fn((next: Anchor) => {
          anchor = next;
        });
        const { container, rerender } = render(
          <PaneHost>
            <Pane id="drag-pane" anchor={anchor} onAnchorChange={onAnchorChange} icon="box" label={uiDataLabel("Drag")} folded={false}>
              <div>Content</div>
            </Pane>
          </PaneHost>,
        );
        const host = container.querySelector('[data-slot="pane-host"]') as HTMLElement;
        host.getBoundingClientRect = () => ({ left: 0, top: 0, width: 300, height: 300, right: 300, bottom: 300, x: 0, y: 0, toJSON: () => ({}) }) as DOMRect;
        const handle = container.querySelector('[data-slot="window-pane-chrome-toggle"] [data-slot="drag-handle"]') as HTMLElement;
        fireEvent.pointerDown(handle, { pointerId: 1, clientX: 10, clientY: 10 });
        fireEvent.pointerMove(handle, { pointerId: 1, clientX: 290, clientY: 290 });
        expect(onAnchorChange).toHaveBeenCalledWith("bottom-right");
        fireEvent.pointerUp(handle, { pointerId: 1, clientX: 290, clientY: 290 });
        rerender(
          <PaneHost>
            <Pane id="drag-pane" anchor={anchor} onAnchorChange={onAnchorChange} icon="box" label={uiDataLabel("Drag")} folded={false}>
              <div>Content</div>
            </Pane>
          </PaneHost>,
        );
        expect(container.querySelector('[data-slot="pane"]')?.getAttribute("data-anchor")).toBe("bottom-right");
      });

      it("Pane without onAnchorChange still renders a drag-handle affordance like panel toggles", () => {
        const { container } = render(
          <PaneHost>
            <Pane id="fixed-pane" anchor="bottom-left" icon="box" label={uiDataLabel("Fixed")} folded={false}>
              <div>Content</div>
            </Pane>
          </PaneHost>,
        );
        expect(container.querySelector('[data-slot="window-chrome-cap"] [data-slot="drag-handle"]')).toBeTruthy();
        expect(container.querySelector('[data-slot="window-chrome-cap"] [data-icon="box"]')).toBeTruthy();
      });

      it("under the compact driver, Pane renders no drag-handle and no label, but the whole toggle stays draggable", () => {
        let anchor: Anchor = "top-left";
        const onAnchorChange = vi.fn((next: Anchor) => {
          anchor = next;
        });
        const { container } = render(
          <UiDriverProvider driver={COMPACT_UI_DRIVER}>
            <PaneHost>
              <Pane id="drag-pane" anchor={anchor} onAnchorChange={onAnchorChange} icon="box" label={uiDataLabel("Drag")} folded={false}>
                <div>Content</div>
              </Pane>
            </PaneHost>
          </UiDriverProvider>,
        );
        const host = container.querySelector('[data-slot="pane-host"]') as HTMLElement;
        host.getBoundingClientRect = () => ({ left: 0, top: 0, width: 300, height: 300, right: 300, bottom: 300, x: 0, y: 0, toJSON: () => ({}) }) as DOMRect;
        expect(container.querySelector('[data-slot="window-pane-chrome-toggle"] [data-slot="drag-handle"]')).toBeNull();
        expect(container.querySelector('[data-slot="window-pane-chrome-toggle"]')?.textContent).not.toContain("Drag");
        const toggle = container.querySelector('[data-slot="window-pane-chrome-toggle"]') as HTMLElement;
        fireEvent.pointerDown(toggle, { pointerId: 1, clientX: 10, clientY: 10 });
        fireEvent.pointerMove(toggle, { pointerId: 1, clientX: 290, clientY: 290 });
        expect(onAnchorChange).toHaveBeenCalledWith("bottom-right");
        fireEvent.pointerUp(toggle, { pointerId: 1, clientX: 290, clientY: 290 });
      });

      it("Pane resize handle grows the pane on its inner edge and respects min/max size", () => {
        let size = 300;
        const onSizeChange = vi.fn((next: number) => {
          size = next;
        });
        const { container, rerender } = render(
          <PaneHost>
            <Pane id="resize-pane" anchor="top-left" icon="box" label={uiDataLabel("Resize")} resizable size={size} onSizeChange={onSizeChange} minSize={200} maxSize={600} folded={false}>
              <div>Content</div>
            </Pane>
          </PaneHost>,
        );
        const handle = container.querySelector('[data-slot="pane-resize-handle"]') as HTMLElement;
        expect(handle.className).toContain("right-0");
        fireEvent.pointerDown(handle, { pointerId: 2, clientX: 100, clientY: 0 });
        fireEvent.pointerMove(handle, { pointerId: 2, clientX: 150, clientY: 0 });
        expect(onSizeChange).toHaveBeenCalledWith(350);
        rerender(
          <PaneHost>
            <Pane id="resize-pane" anchor="top-left" icon="box" label={uiDataLabel("Resize")} resizable size={size} onSizeChange={onSizeChange} minSize={200} maxSize={600} folded={false}>
              <div>Content</div>
            </Pane>
          </PaneHost>,
        );
        expect(container.querySelector('[data-slot="pane"]')?.getAttribute("style")).toContain("350px");
      });

      it("usePaneSlot portals its pane into the nearest PaneHost and renders nothing outside one", () => {
        function DeepChild() {
          return usePaneSlot(
            <Pane id="slotted-pane" anchor="bottom-right" icon="box" label={uiDataLabel("Slotted")} folded={false}>
              <div data-testid="slotted-content">Slotted</div>
            </Pane>,
          );
        }
        const { container: withHost } = render(
          <PaneHost>
            <DeepChild />
          </PaneHost>,
        );
        expect(withHost.querySelector('[data-slot="pane"][data-anchor="bottom-right"]')).toBeTruthy();
        expect(screen.getByTestId("slotted-content")).toBeTruthy();

        function OrphanDeepChild() {
          return usePaneSlot(
            <Pane id="orphan-pane" anchor="top-left" icon="box">
              {null}
            </Pane>,
          );
        }
        const { container: withoutHost } = render(<OrphanDeepChild />);
        expect(withoutHost.querySelector('[data-slot="pane"]')).toBeNull();
      });

      it("on mobile respects folded state — no drag or resize handles, fold toggle still works", () => {
        const onFoldToggle = vi.fn();
        const { container } = render(
          <UiMobileProvider mobile>
            <PaneHost>
              <Pane id="mobile-pane" anchor="bottom-right" icon="box" label={uiDataLabel("Mobile")} onAnchorChange={vi.fn()} resizable size={300} onSizeChange={vi.fn()} folded onFoldToggle={onFoldToggle}>
                <div data-testid="mobile-pane-content">Content</div>
              </Pane>
            </PaneHost>
          </UiMobileProvider>,
        );
        expect(container.querySelector('[data-slot="window-chrome-cap"] [data-slot="drag-handle"]')).toBeNull();
        expect(container.querySelector('[data-slot="pane-resize-handle"]')).toBeNull();
        expect(screen.queryByTestId("mobile-pane-content")).toBeNull();
        fireEvent.click(container.querySelector('[data-slot="window-pane-chrome-toggle"]')!);
        expect(onFoldToggle).toHaveBeenCalledTimes(1);
      });

      it("mirrors dir=rtl for right anchors with a fixed semantic icon and trailing drag handle, while the body carries no dir override", () => {
        const { container: leftContainer } = render(
          <PaneHost>
            <Pane id="left-pane" anchor="top-left" icon="box" label={uiDataLabel("Left")} folded={false}>
              <div data-testid="left-pane-content">Content</div>
            </Pane>
          </PaneHost>,
        );
        expect(leftContainer.querySelector('[data-slot="pane"]')?.getAttribute("dir")).toBeNull();
        expect(leftContainer.querySelector('[data-slot="pane-body"]')?.getAttribute("dir")).toBeNull();
        expect(leftContainer.querySelector('[data-slot="pane"]')?.className).toContain("items-start");
        expect(leftContainer.querySelector('[data-slot="pane"]')?.className).not.toContain("items-end");
        expect(leftContainer.querySelector('[data-icon="box"]')).toBeTruthy();
        expect(leftContainer.querySelector('[data-slot="window-chrome-cap"] [data-slot="drag-handle"]')).toBeTruthy();

        const { container: rightContainer } = render(
          <PaneHost>
            <Pane id="right-pane" anchor="top-right" icon="camera" label={uiDataLabel("Right")} folded={false}>
              <div data-testid="right-pane-content">Content</div>
            </Pane>
          </PaneHost>,
        );
        expect(rightContainer.querySelector('[data-slot="pane"]')?.getAttribute("dir")).toBe("rtl");
        expect(rightContainer.querySelector('[data-slot="pane-body"]')?.getAttribute("dir")).toBeNull();
        expect(rightContainer.querySelector('[data-slot="pane"]')?.className).toContain("items-start");
        expect(rightContainer.querySelector('[data-slot="pane"]')?.className).not.toContain("items-end");
        expect(rightContainer.querySelector('[data-icon="camera"]')).toBeTruthy();
        expect(rightContainer.querySelector('[data-slot="window-chrome-cap"] [data-slot="drag-handle"]')).toBeTruthy();
      });

      it("centers a middle anchor with items-center instead of items-start", () => {
        const { container } = render(
          <PaneHost>
            <Pane id="middle-pane" anchor="top-middle" icon="box" label={uiDataLabel("Middle")} folded={false}>
              <div>Content</div>
            </Pane>
          </PaneHost>,
        );
        expect(container.querySelector('[data-slot="pane"]')?.className).toContain("items-center");
      });
    });

    it("createEvenWindowLayout builds a row of stacks", () => {
      const layout = createEvenWindowLayout(["a", "b"]);
      expect(layout.kind).toBe("row");
      if (layout.kind === "row") {
        expect(layout.children).toHaveLength(2);
      }
    });
  });

}
