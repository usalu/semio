/** 🧪️ Runs the shared window-context vectors against the TypeScript implementation. */
import assert from "node:assert/strict";
import { hostArmedViewContext, panelViewContext, windowViewContext, type PluginViewState } from "../../🟦️.ts";
import fixture from "../../🧫️fixtures/🔬️window-view-context/🔣️.json";

export function testWindowViewContext(): void {
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

  const windowedMenu = windowViewContext(view, "left");
  assert.equal(windowedMenu?.activeToolId, undefined, "windowViewContext alone drops the armed tool — requestContextMenu must not use it");
  const armedMenu = hostArmedViewContext(view, "fill", "left");
  assert.equal(armedMenu?.activeToolId, "fill", "windowed requestContextMenu must use hostArmedViewContext so the tool overlay is present");

  console.log(`window-view-context cases=${fixture.cases.length} hostArmed=${fixture.hostArmed.length} isolation=valid preferences=preserved`);
}
