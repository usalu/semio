import { windowUtility } from "../../👁️view-state-and-public-invocation/🟦️.ts";
/** 🧪️ Runs the shared window-context vectors against the TypeScript implementation. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { Database } from "bun:sqlite";
import { hostArmedViewContext, panelViewContext, windowViewContext, type PluginViewState } from "../../👁️view-state-and-public-invocation/🟦️.ts";
import fixture from "../🧫️fixtures/🔬️window-view-context/🔣️.json";

export function testWindowViewContext(): void {
  const native = readFileSync(new URL("../../🖼️view-model/🦀️.rs", import.meta.url), "utf8");
  assert.match(native, /pub fn window_utility\(&self, window_id: Option<&str>\) -> Option<&str>/, "the defining Rust ViewModel must implement the utility routing used by application callers");
  const oracle = new Database(":memory:");
  const utility = oracle.query<{ utility: string | null }, { $view: string; $requested: string | null }>(`
    WITH input AS (
      SELECT coalesce($requested, json_extract($view, '$.windowId')) AS requested,
             json_extract($view, '$.windowId') AS current_id,
             json_extract($view, '$.focusedWindowId') AS focused_id
    ), windows AS (
      SELECT json_extract(value, '$.id') AS id, json_extract(value, '$.windowKindId') AS kind
      FROM json_each($view, '$.windowInstances')
    ), candidates AS (
      SELECT id FROM windows, input WHERE kind = requested
    ), resolved AS (
      SELECT CASE
        WHEN EXISTS(SELECT 1 FROM windows WHERE id = requested) THEN requested
        WHEN EXISTS(SELECT 1 FROM candidates) THEN (
          SELECT id FROM candidates WHERE id = current_id OR id = focused_id OR (SELECT count(*) FROM candidates) = 1
          ORDER BY CASE WHEN id = current_id THEN 0 WHEN id = focused_id THEN 1 ELSE 2 END LIMIT 1
        )
        ELSE requested END AS id, requested, current_id FROM input
    ), utilities AS (
      SELECT key, value FROM json_each($view, '$.activeUtilityByWindowId')
    )
    SELECT nullif(CASE
      WHEN requested IS NULL THEN json_extract($view, '$.activeUtilityId')
      WHEN EXISTS(SELECT 1 FROM utilities WHERE key = id) THEN (SELECT value FROM utilities WHERE key = id)
      WHEN current_id = id THEN json_extract($view, '$.activeUtilityId')
      ELSE NULL END, '') AS utility FROM resolved
  `);
  try {
    const view: PluginViewState = fixture.view;
    const before = structuredClone(view);
    for (const test of fixture.cases) {
      const actual = windowViewContext(view, test.windowId);
      if ("absent" in test) {
        assert.equal(actual, undefined);
        continue;
      }
      assert(actual);
      assert.equal(actual.windowId, test.windowId);
      assert.equal(actual.activeWindowKindId, "graph");
      assert.equal(actual.activeUtilityId, test.activeUtilityId ?? undefined);
      assert.equal(actual.locale, view.locale);
      assert.equal(actual.terminology, view.terminology);
      assert.equal(actual.activeModeId, view.activeModeId);
    }
    assert.deepEqual(view, before);
    const panel = panelViewContext(view);
    assert.equal(panel.windowId, undefined);
    assert.equal(panel.activeWindowKindId, undefined);
    assert.equal(panel.activeUtilityId, undefined);
    assert.equal(panel.locale, view.locale);
    assert.equal(panel.terminology, view.terminology);
    assert.equal(panel.activeModeId, view.activeModeId);
    assert.deepEqual(panel.activeUtilityByWindowId, view.activeUtilityByWindowId);
    assert.equal(panel.focusedWindowId, view.focusedWindowId, "fails-before: an app-level panel that authors per-window settings loses the pane the user is looking at and retunes the roster's first entry");
    assert.equal(windowViewContext(view, "left")?.focusedWindowId, view.focusedWindowId, "a windowed projection carries the focused pane alongside its own render target");
    assert.deepEqual(view, before);
    const windowOnly = windowViewContext(view, "left");
    assert.equal(windowOnly?.activeToolId, undefined, "fails-before: windowed dispatch without host overlay drops the armed tool");
    for (const test of fixture.hostArmed) {
      const actual = hostArmedViewContext(view, test.hostActiveToolId, test.windowId ?? undefined);
      assert(actual);
      assert.equal(actual.activeToolId, test.activeToolId ?? undefined);
      assert.equal(actual.activeUtilityId, test.activeUtilityId ?? undefined);
    }
    assert.deepEqual(view, before);

    for (const test of fixture.utilityCases) {
      const context = { ...view, ...("currentWindowId" in test ? { windowId: test.currentWindowId ?? undefined } : {}), ...("focusedWindowId" in test ? { focusedWindowId: test.focusedWindowId ?? undefined } : {}), ...("utilities" in test ? { activeUtilityByWindowId: Object.fromEntries(Object.entries(test.utilities ?? {}).filter(([, value]) => value !== undefined)) } : {}), ...("windows" in test ? { windowInstances: test.windows } : {}) };
      const expected = test.activeUtilityId ?? undefined;
      assert.equal(windowUtility(context, test.windowId ?? undefined), expected, JSON.stringify(test));
      assert.equal(utility.get({ $view: JSON.stringify(context), $requested: test.windowId ?? null })?.utility ?? undefined, expected, `SQLite utility ownership: ${JSON.stringify(test)}`);
    }

    const windowedMenu = windowViewContext(view, "left");
    assert.equal(windowedMenu?.activeToolId, undefined, "windowViewContext alone drops the armed tool — requestContextMenu must not use it");
    const armedMenu = hostArmedViewContext(view, "fill", "left");
    assert.equal(armedMenu?.activeToolId, "fill", "windowed requestContextMenu must use hostArmedViewContext so the tool overlay is present");

    console.log(`[DEBUG] window-view-context cases=${fixture.cases.length} hostArmed=${fixture.hostArmed.length} utilities=${fixture.utilityCases.length} sqlite=matched isolation=valid preferences=preserved`);
  } finally {
    oracle.close();
  }
}
