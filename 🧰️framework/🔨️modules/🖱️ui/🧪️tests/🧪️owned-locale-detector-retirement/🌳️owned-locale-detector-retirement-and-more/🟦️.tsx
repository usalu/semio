import { colorOracle } from "@semio-tech/ui-styling/verification/testing/color-oracle";
import * as ui from "@semio-tech/ui-react/source";
import { renderToStaticMarkup } from "react-dom/server";
import { createDiagramForceSimulation } from "../../../🧱️elements/🕸️Diagram/🧮️force-simulation/🟦️.ts";
import type { TestSource, Tests1Vitest, Tests1Dependencies } from "../🧰️harness/🟦️.ts";

export async function registerOwnedLocaleDetectorRetirementAndMoreTests(vitest: Tests1Vitest, dependencies: Tests1Dependencies, testSource: TestSource): Promise<void> {
  const { NotFound, React, RouteLink, Search, UI_CHROME_LOCALE_STORAGE_KEY, celebrateAllElements, celebrateElement, celebrateElements, childElementId, createDiagramForceSimulation, defaultDiagramForceConfig, detectShellLocale, elementIdSegment, elementIdSelector, initUiLocaleSync, isElementId, navigateOwnedRoute, parseOwnedRouteTarget, rankFuzzyItems, reactHostPort, renderToStaticMarkup, resolveTranslationLabel, uiI18n, useFirstDraggableElementAlias } = dependencies;
  type Edge = any;
  type FlowNode = any;
  const { describe, expect, it, vi } = vitest;
  const { render, screen, waitFor, act } = await import("@testing-library/react");
  const { calculateDiagramLayoutForBatchTest } = await import("../../../🧱️elements/🕸️Diagram/📐️layout/🟦️.ts");

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
      const source = readFileSync(fileURLToPath(testSource.url), "utf8");
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
      const source = readFileSync(fileURLToPath(testSource.url), "utf8");
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
      const css = readFileSync(resolve(dirname(fileURLToPath(testSource.url)), "../../🎨️styling/🖌️ui/🎨️.css"), "utf8");
      expect(css).toMatch(/@property --celebrate-border-angle[\s\S]*?inherits:\s*false/);
      expect(css).toMatch(/\[data-celebrated="true"\][\s\S]*?--celebrate-conic:/);
      expect(css).toMatch(/\[data-celebrated="true"\]::after[\s\S]*?background:\s*var\(--celebrate-conic\)/);
      expect(css).toMatch(/\[data-celebrated="true"\]::after[\s\S]*?padding:\s*var\(--celebrate-border-padding\)/);
      expect(css).toMatch(/\[data-celebrated="true"\]::after[\s\S]*?animation:[\s\S]*?celebrate-border-spin[\s\S]*?celebrate-border-burst/);
      expect(css).toContain("#endregion 🎉️CelebrateContent");
      expect(css).toMatch(/\[data-celebrated="true"\]:is\([\s\S]*?\[data-slot="button-group-item"\]/);
      expect(css).toMatch(/\[data-celebrated="true"\]\[data-slot="introduction-interaction-label"\][\s\S]*?background-clip:\s*text/);
      expect(css).toMatch(/\.window-silhouette-border-celebrated-fill[\s\S]*?background:\s*var\(--celebrate-conic\)/);
      expect(css).not.toMatch(/\[data-celebrated="true"\]:is\([\s\S]*?\[data-slot="window"\]/);
      const celebrateContent = css.match(/\/\* #region 🎉️CelebrateContent[\s\S]*?\/\* #endregion 🎉️CelebrateContent \*\//)?.[0] ?? "";
      expect(celebrateContent).not.toMatch(/:is\(\[data-icon\], \[data-icon-kind\], \[data-slot="tree-icon"\], \[data-slot="drag-handle"\]\)/);
      expect(celebrateContent).not.toMatch(/mix-blend-mode:\s*destination-in/);
      expect(celebrateContent).not.toMatch(/inset:\s*-100%/);
      expect(celebrateContent).toMatch(/mask-image:\s*var\(--icon-mask, linear-gradient\(transparent 0 0\)/);
      expect(colorOracle.get("transparent")).toEqual(colorOracle.get("#0000"));
      expect(colorOracle.get("transparent")?.value).toEqual([0, 0, 0, 0]);
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

    it("border effect phase clocks run on the painting element, never on the document root", async () => {
      const { readFileSync } = await import("node:fs");
      const { fileURLToPath } = await import("node:url");
      const { dirname, resolve } = await import("node:path");
      const css = readFileSync(resolve(dirname(fileURLToPath(testSource.url)), "../../🎨️styling/🖌️ui/🎨️.css"), "utf8");
      const unlayeredRoot = css.match(/^:root \{[\s\S]*?\n\}/m)?.[0] ?? "";
      expect(unlayeredRoot).not.toMatch(/animation:/);
      expect(css).toMatch(/@property --loading-border-angle[\s\S]*?inherits:\s*false/);
      expect(css).toMatch(/@property --waiting-border-angle[\s\S]*?inherits:\s*false/);
      expect(css).toMatch(/@property --introduced-border-width[\s\S]*?inherits:\s*false/);
      expect(css).toMatch(/@property --celebrate-border-padding[\s\S]*?inherits:\s*false/);
      expect(css).toMatch(/@utility border-loading[\s\S]*?opacity:\s*var\(--loading-border-pulse-opacity\)/);
      expect(css).toMatch(/@utility border-loading[\s\S]*?animation:[\s\S]*?loading-border-spin[\s\S]*?loading-border-pulse/);
      expect(css).toMatch(/@utility border-waiting[\s\S]*?opacity:\s*var\(--waiting-border-pulse-opacity\)/);
      expect(css).toMatch(/@utility border-waiting[\s\S]*?animation:[\s\S]*?waiting-border-spin[\s\S]*?waiting-border-pulse/);
      expect(css).toMatch(/\[data-introduced="true"\][\s\S]*?box-shadow:\s*inset 0 0 0 var\(--introduced-border-width\)/);
      expect(css).toMatch(/\[data-introduced="true"\] \{[\s\S]*?animation:\s*introduced-border-pulse/);
      expect(css).toMatch(/\.window-silhouette-border-introduced[\s\S]*?stroke-width:\s*var\(--introduced-border-width\)/);
      expect(css).toMatch(/\.window-silhouette-border-introduced[\s\S]*?animation:\s*introduced-border-pulse/);
      expect(css).toMatch(/\.window-silhouette-border-loading[\s\S]*?stroke-dashoffset:\s*var\(--loading-border-dashoffset\)/);
      expect(css).toMatch(/\.window-silhouette-border-loading[\s\S]*?animation:[\s\S]*?window-silhouette-border-loading-dash/);
      expect(css).toMatch(/\.window-silhouette-border-celebrated-mask[\s\S]*?stroke-width:\s*var\(--celebrate-border-padding\)/);
      expect(css).toMatch(/\.window-silhouette-border-celebrated-mask[\s\S]*?animation:\s*celebrate-border-burst/);
      expect(css).toMatch(/\.window-silhouette-border-celebrated-fill[\s\S]*?animation:\s*celebrate-border-spin/);
      expect(css).toContain("[data-window-silhouette]");
      expect(css).toContain("[data-window-silhouette-border]");
      expect(css).toContain("[data-window-silhouette-gap]");
      expect(css).toContain("[data-window-silhouette-chip]");
      expect(css).not.toMatch(/\[data-slot="mode-dock-stack"\]\s*\[data-slot="window"\]\[data-introduced="true"\]/);
      expect(css).toMatch(/\[data-slot="button-group"\] > \[data-slot="button-group-item"\]:not\(:last-child\)/);
      expect(css).toMatch(/\[data-slot="panel-tabs"\] > \[data-slot="panel-tab-button"\]:not\(:last-child\)/);
      expect(css).toContain("box-shadow: inset calc(-1 * var(--stroke-hairline)) 0 0 0 var(--border-normal-color)");
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
}
