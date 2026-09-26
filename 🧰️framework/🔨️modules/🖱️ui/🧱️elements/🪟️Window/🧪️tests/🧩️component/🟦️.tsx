// #region 🔌️Adapters
import { render } from "@testing-library/react";
import * as React from "react";
import { describe, expect, it } from "vitest";
import { Window } from "../../🟦️.tsx";
import { uiDataLabel } from "../../../🎗️UiLabel/🟦️.tsx";
import chromeStacking from "../../🧫️fixtures/🪜️chrome-stacking.json";
import { readFileSync } from "node:fs";
import { join } from "node:path";
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
