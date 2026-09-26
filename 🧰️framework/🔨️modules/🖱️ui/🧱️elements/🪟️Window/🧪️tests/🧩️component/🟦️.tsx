// #region 🔌️Adapters
import { fireEvent, render, within } from "@testing-library/react";
import * as React from "react";
import { describe, expect, it } from "vitest";
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
// #endregion 🔌️Adapters

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
