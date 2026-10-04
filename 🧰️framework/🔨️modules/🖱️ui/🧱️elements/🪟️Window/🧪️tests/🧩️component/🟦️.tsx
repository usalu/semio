// #region 🔌️Adapters
import { fireEvent, render, within } from "@testing-library/react";
import * as React from "react";
import { describe, expect, it, vi } from "vitest";
import { Window } from "../../🟦️.tsx";
import { uiDataLabel } from "../../../🎗️UiLabel/🟦️.tsx";
import chromeStacking from "../../🧫️fixtures/🪜️chrome-stacking.json";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import Ajv from "ajv";
import { I18nextProvider } from "react-i18next";
import { Mode } from "../../../🎨️Canvas/🟦️.tsx";
import { createShellI18nInstance, disposeShellI18nInstance } from "../../../../🎯️targets/⚛️react/🟦️.tsx";
import dockNames from "../../../../🧫️fixtures/🪟️dock-accessible-names/🔣️.json";
import dockNamesSchema from "../../../../🧬️schema/🪟️dock-accessible-names/🔣️.json";
import Ajv2020 from "ajv/dist/2020.js";
import searchRouting from "../../🧫️fixtures/🔎️search-fold-routing/🔣️.json";
import searchRoutingSchema from "../../🧬️schema/🔎️search-fold-routing/🔣️.json";
import paneFolds from "../../🧫️fixtures/🔀️pane-fold-independence/🔣️.json";
import paneFoldsSchema from "../../🧬️schema/🔀️pane-fold-independence/🔣️.json";
import { createMemoryStoragePort } from "@semio-tech/framework";
import { ShellScopeProvider, createShellScope } from "../../../🐚️ShellScope/🟦️.tsx";
// #endregion 🔌️Adapters

it("keeps Actions and Search pane folds independent in both explicit locales", () => {
  const validate = new Ajv2020({ strict: true }).compile(paneFoldsSchema);
  expect(validate(paneFolds)).toBe(true);
  expect(validate({ ...paneFolds, sharedToggle: true })).toBe(false);
  for (const locale of searchRouting.locales) for (const entry of paneFolds.cases) {
    const root = document.createElement("div"), app = document.createElement("div"), portal = document.createElement("div");
    root.append(app, portal);
    document.body.append(root);
    const scope = createShellScope({ shellId: "pane-fold-" + locale.locale + "-" + entry.id, storage: createMemoryStoragePort(), initialLocale: locale.locale as "en" | "de" });
    scope.rootRef.current = root;
    scope.portalLayerRef.current = portal;
    const view = render(<ShellScopeProvider scope={scope}><Window id="fold-window" active actionPane={<div data-testid="fold-actions">Actions</div>} search={{ input: { placeholder: uiDataLabel(locale.placeholder) } }}>Body</Window></ShellScopeProvider>, { container: app });
    try {
      for (const kind of entry.clicks) fireEvent.click(app.querySelector('[id="framework.window.foldWindow.' + kind + '.toggle"]')!);
      expect(view.queryByTestId("fold-actions") !== null, locale.locale + ":" + entry.id).toBe(entry.actions);
      expect(view.queryByPlaceholderText(locale.placeholder) !== null, locale.locale + ":" + entry.id).toBe(entry.search);
    } finally {
      view.unmount();
      root.remove();
    }
  }
  console.info("[DEBUG] Window independent pane folds: twelve bilingual native DOM toggle sequences");
});

it("unfolds only Search for admitted printable keys in either explicit locale", () => {
  const validate = new Ajv2020({ strict: true }).compile(searchRoutingSchema);
  expect(validate(searchRouting)).toBe(true);
  expect(validate({ ...searchRouting, ambientLocale: "en" })).toBe(false);
  expect(validate({ ...searchRouting, cases: searchRouting.cases.slice(1) })).toBe(false);
  for (const locale of searchRouting.locales) for (const entry of searchRouting.cases) {
    const root = document.createElement("div"), app = document.createElement("div"), portal = document.createElement("div");
    root.append(app, portal);
    document.body.append(root);
    const scope = createShellScope({ shellId: "search-routing-" + locale.locale + "-" + entry.id, storage: createMemoryStoragePort(), initialLocale: locale.locale as "en" | "de" });
    scope.rootRef.current = root;
    scope.portalLayerRef.current = portal;
    const view = render(
      <ShellScopeProvider scope={scope}>
        <Window id="routing-window" active actionPane={<div data-testid="action-content">Actions</div>} search={{ input: { placeholder: uiDataLabel(locale.placeholder) } }}>Body</Window>
      </ShellScopeProvider>,
      { container: app },
    );
    try {
      expect(view.queryByPlaceholderText(locale.placeholder)).toBeNull();
      expect(view.queryByTestId("action-content")).toBeNull();
      fireEvent.keyDown(app, entry);
      expect(view.queryByPlaceholderText(locale.placeholder) !== null, locale.locale + ":" + entry.id).toBe(entry.searchVisible);
      expect(view.queryByTestId("action-content") !== null, locale.locale + ":" + entry.id).toBe(entry.actionsVisible);
    } finally {
      view.unmount();
      root.remove();
    }
  }
  console.info("[DEBUG] Search fold routing: sixteen explicit-locale native keyboard event cases");
});

// #region 🪜️ChromeStacking
/** 🪜️ The chrome-stacking law, replayed from `🧫️fixtures/🪜️chrome-stacking.json`. jsdom computes no stacking, so this
 * suite asserts the STRUCTURE that makes the law hold (isolated body root, chrome panes as its siblings at the pane
 * level); the live `s` matrix probe replays the same fixture's intent against Chromium's hit-testing. */
type ChromeStackingFixture = {
  readonly bodyContentRoot: { readonly slot: string; readonly class: string };
  readonly chromeSlots: readonly string[];
  readonly chromeLevelClass: string;
  readonly cases: readonly { readonly name: string; readonly bodyLayerClass: string }[];
};
const fixture = chromeStacking as ChromeStackingFixture;

describe("Window chrome stacking", () => {
  for (const testCase of fixture.cases) {
    it(`keeps every chrome pane outside the isolated body root: ${testCase.name}`, () => {
      const { container } = render(
        <Window id="stacking-window" active fill actionPane={<div>Rows</div>} utilityBar={<div>Tools</div>} measures={<div>LOD</div>} search={{ input: { placeholder: uiDataLabel("Action") } }}>
          <div data-testid="body-layer" className={testCase.bodyLayerClass} />
        </Window>,
      );
      const body = container.querySelector('[data-slot="window-body"]');
      const root = body?.querySelector(`:scope > [data-slot="${fixture.bodyContentRoot.slot}"]`);
      expect(root).not.toBeNull();
      expect(root!.className.split(/\s+/u)).toContain(fixture.bodyContentRoot.class);
      expect(root!.contains(container.querySelector('[data-testid="body-layer"]'))).toBe(true);
      for (const slot of fixture.chromeSlots) {
        const pane = body!.querySelector(`[data-slot="${slot}"]`);
        expect(pane, slot).not.toBeNull();
        expect(root!.contains(pane), `${slot} must not live inside the isolated body root`).toBe(false);
        expect(pane!.parentElement, `${slot} is a sibling of the body root`).toBe(body);
        expect(pane!.className.split(/\s+/u), `${slot} paints at the chrome level`).toContain(fixture.chromeLevelClass);
      }
    });
  }
});

describe("Window chrome focus indicator", () => {
  const uiCss = readFileSync(join(import.meta.dirname, "../../../../🎨️styling/🖌️ui/🎨️.css"), "utf8");
  it("keeps a visible focus ring on docked chips although the chip frame reset drops every other shadow", () => {
    expect(uiCss).toMatch(/\[data-window-silhouette-chip\] > \*\s*\{[^}]*box-shadow: none;/u);
    expect(uiCss).toMatch(/\[data-window-silhouette-chip\] > :focus-visible[^{]*\{\s*box-shadow: inset 0 0 0 var\(--stroke-default\) var\(--active-base\);/u);
  });
});
// #endregion 🪜️ChromeStacking

describe("Dock accessible names", () => {
  for (const locale of dockNames.locales) for (const stacked of [false, true]) {
    it(`announces authored titles and localized dock actions in ${locale.id}, stacked=${stacked}`, () => {
      expect(new Ajv().validate(dockNamesSchema, dockNames)).toBe(true);
      const i18n = createShellI18nInstance(locale.id as "en" | "de");
      const view = render(
        <I18nextProvider i18n={i18n}>
          <Mode
            windows={dockNames.windows.map(window => ({ id: window.id, title: uiDataLabel(window.title), iconId: "app-window", children: <div>{window.title} body</div> }))}
            layout={stacked ? { kind: "stack", children: dockNames.windows.map(window => ({ kind: "window", id: window.id })), activeId: dockNames.stackedActiveId } : { kind: "row", children: dockNames.windows.map(window => ({ kind: "stack", children: [{ kind: "window", id: window.id }], activeId: window.id })) }}
            activeWindowId={dockNames.windows[0].id}
            onActiveWindowChange={() => {}}
          />
        </I18nextProvider>,
      );
      try {
        for (const window of dockNames.windows) {
          const tab = view.container.querySelector(`[data-slot="mode-dock-tab"][data-window-id="${window.id}"]`)!;
          const selector = within(tab as HTMLElement).getByRole(dockNames.semantics.tabRole, { name: window.title });
          const active = dockNames.windows.find(candidate => candidate.id === dockNames.stackedActiveId)!;
          const panel = view.getByRole(dockNames.semantics.panelRole, { name: stacked ? active.title : window.title });
          expect(selector.getAttribute("aria-selected")).toBe(String(stacked ? window.id === active.id : dockNames.semantics.selected));
          expect(selector.getAttribute("aria-controls")).toBe(panel.id);
          if (!stacked) expect(within(tab as HTMLElement).getByRole("button", { name: locale.focus })).toBeTruthy();
          expect(within(tab as HTMLElement).getByRole("button", { name: locale.close })).toBeTruthy();
          expect(tab.querySelector('[data-slot="drag-handle"]')?.getAttribute("aria-label")).toBe(locale.drag.replace("{{target}}", window.title));
        }
        if (!stacked) {
          fireEvent.click(view.container.querySelector('[data-slot="mode-dock-tab-focus"]')!);
          expect(view.getByRole("button", { name: locale.unfocus })).toBeTruthy();
        }
      } finally {
        view.unmount();
        disposeShellI18nInstance(i18n);
      }
    });
  }
});

import contentClearance from "../../🧫️fixtures/🚧️content-clearance/🔣️.json";
import contentClearanceSchema from "../../🧬️schema/🚧️content-clearance/🔣️.json";
import { ChromeAwareWindowScrollSurface, Scrollable } from "../../../../🎯️targets/⚛️react/🟦️.tsx";

it("keeps the first chrome-aware content control reachable without changing scroll position", () => {
  expect(new Ajv2020({ strict: true }).validate(contentClearanceSchema, contentClearance)).toBe(true);
  for (const row of contentClearance.cases) {
    const bounds = vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockImplementation(function (this: HTMLElement) {
      const slot = this.getAttribute("data-slot");
      const top = slot === "window-dead-line-scroll" ? row.surfaceTop : slot === "scroll-area" ? row.chromeBottom : row.bodyTop;
      const bottom = slot === "window-engagement-overlay" ? Number(this.getAttribute("data-test-bottom") ?? row.chromeBottom) : top + 400;
      return { x: 0, y: top, top, bottom, left: 0, right: 600, width: 600, height: bottom - top, toJSON: () => ({}) };
    });
    const view = render(<div data-slot="window-body">
      {row.overlay ? <div data-slot="window-engagement-overlay">Actions</div> : null}
      <ChromeAwareWindowScrollSurface ref={(el) => { if (el) el.scrollTop = row.scrollTop; }}>
        <div data-window-content-layout={row.edgeless ? "edgeless" : "chrome-aware"}><button>Add row</button>{"nestedEdgeless" in row && row.nestedEdgeless ? <div data-window-content-layout="edgeless">Canvas</div> : null}{"nestedChromeBottom" in row ? <div data-slot="window-body"><div data-slot="window-engagement-overlay" data-test-bottom={row.nestedChromeBottom}>Nested actions</div></div> : null}</div>
        {"nestedScroller" in row && row.nestedScroller ? <Scrollable><div style={{ height: 1200 }}>Long details</div></Scrollable> : null}
      </ChromeAwareWindowScrollSurface>
    </div>);
    try {
      const surface = view.container.querySelector<HTMLElement>('[data-slot="window-dead-line-scroll"]')!;
      expect(Number.parseFloat(surface.style.paddingBlockStart) || 0, row.id).toBe(row.inset);
      expect(surface.scrollTop, row.id).toBe(row.scrollTop);
      const nestedScroller = surface.querySelector<HTMLElement>('[data-slot="scroll-area"]');
      if (nestedScroller) { expect(nestedScroller.scrollTop, row.id).toBe(0); expect(Number.parseFloat(nestedScroller.style.paddingBlockStart) || 0, row.id).toBe(0); }
      expect(within(surface).getByRole("button", { name: "Add row" })).toBeTruthy();
    } finally { view.unmount(); bounds.mockRestore(); }
  }
});
