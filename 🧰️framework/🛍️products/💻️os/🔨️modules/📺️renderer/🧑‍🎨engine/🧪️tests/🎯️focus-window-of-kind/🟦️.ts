// @vitest-environment jsdom
/** 🎯️ The React half of the `focusWindow` law. Open a closed window kind, raise an open one, announce it in the interface language, and keep a kind the presented device layout lacks off the screen: mapped to its stand-in or refused with a localized notice.
 *
 * 🧫️ Read from `🐚️Shell/🧪️tests/🧫️fixtures/🎯️focus-window-of-kind/🔣️.json` — the SAME file the wgpu laws read (`🔬️wgpu-shell-chrome-parity/🎯️focus-window-of-kind/🦀️.rs`), so desktop, mobile and tablet rows and both languages cannot drift between the implementations. */
import { describe, expect, it } from "vitest";
import { FOCUS_WINDOW_ACTION_ID, FOCUS_WINDOW_ARG_FALLBACK, FOCUS_WINDOW_ARG_KIND, type ShellLocale } from "@semio-tech/framework";
import { deviceLayoutWindowKindsV1, focusWindowArgsV1, focusWindowBodyV1, focusWindowNoticeV1, planFocusWindowV1, windowKindIdsOfLayoutV1, type FocusWindowDeviceV1, type FocusWindowPlanV1 } from "../../🧱️elements/🏛️ShellHost/🎯️focus-window/🟦️.ts";
import fixture from "../../🧱️elements/🐚️Shell/🧪️tests/🧫️fixtures/🎯️focus-window-of-kind/🔣️.json" with { type: "json" };

const declared = fixture.windowKinds.map((kind) => kind.id);
const layoutsOf = (withDeviceLayouts: boolean) => (withDeviceLayouts ? fixture.deviceLayouts.map((layout) => ({ id: layout.id, windowKindIds: layout.kinds })) : []);
const nameOf = (windowKindId: string, locale: ShellLocale): string => fixture.windowKinds.find((kind) => kind.id === windowKindId)?.label[locale] ?? windowKindId;
const plan = (input: Partial<Parameters<typeof planFocusWindowV1>[0]> & { requested: string }): FocusWindowPlanV1 => planFocusWindowV1({ declaredKindIds: declared, layouts: layoutsOf(true), device: "desktop", fallback: undefined, mounted: fixture.initiallyOpen.map((id) => ({ windowId: id, windowKindId: id })), activeWindowId: fixture.initiallyOpen[0] ?? null, ...input });

describe("🎯️ focusWindow", () => {
  it("shares its action and argument names with the Rust shell", () => {
    expect(fixture.action).toBe(FOCUS_WINDOW_ACTION_ID);
    expect(fixture.argument).toBe(FOCUS_WINDOW_ARG_KIND);
    expect(fixture.fallbackArgument).toBe(FOCUS_WINDOW_ARG_FALLBACK);
    expect(focusWindowArgsV1({ [FOCUS_WINDOW_ARG_KIND]: "a", [FOCUS_WINDOW_ARG_FALLBACK]: "c" })).toEqual({ windowKindId: "a", fallback: "c" });
    expect(focusWindowArgsV1({ [FOCUS_WINDOW_ARG_KIND]: "a" })).toEqual({ windowKindId: "a", fallback: undefined });
    expect(focusWindowArgsV1({})).toBeNull();
    expect(focusWindowArgsV1(undefined)).toBeNull();
  });

  it("opens a closed window, raises an open one and refuses an undeclared kind", () => {
    expect(plan({ requested: "b" })).toEqual({ outcome: "opened", windowId: "b", windowKindId: "b" });
    const both = [{ windowId: "a", windowKindId: "a" }, { windowId: "b", windowKindId: "b" }];
    expect(plan({ requested: "b", mounted: both })).toEqual({ outcome: "raised", windowId: "b", windowKindId: "b" });
    expect(plan({ requested: "b", mounted: [...both, { windowId: "b-2", windowKindId: "b" }], activeWindowId: "b-2" })).toEqual({ outcome: "raised", windowId: "b-2", windowKindId: "b" });
    expect(plan({ requested: "undeclared" })).toMatchObject({ outcome: "refused", reason: "undeclared" });
  });

  for (const row of fixture.deviceRows) {
    it(`${row.id}`, () => {
      const outcome = plan({ requested: row.requested, fallback: "fallback" in row ? row.fallback : undefined, device: row.device as FocusWindowDeviceV1, layouts: layoutsOf(!("withoutDeviceLayouts" in row)) });
      if (row.target === null) expect(outcome).toMatchObject({ outcome: "refused", reason: "unavailable", device: row.device });
      else expect(outcome).toMatchObject({ windowKindId: row.target });
    });
  }

  it("reads the window kinds of a manifest layout tree and matches a device layout by its id suffix", () => {
    const tree = { root: { kind: "row", children: [{ kind: "stack", children: [{ kind: "window", windowKindId: "x" }] }, { kind: "column", children: [{ kind: "stack", children: [{ kind: "window", windowKindId: "y" }, { kind: "window", windowKindId: "z" }] }] }] } };
    expect(windowKindIdsOfLayoutV1(tree)).toEqual(["x", "y", "z"]);
    const layouts = [{ id: "norm-visual-desktop", windowKindIds: ["x"] }, { id: "norm-visual-mobile", windowKindIds: ["y"] }];
    expect(deviceLayoutWindowKindsV1(layouts, "mobile")).toEqual(["y"]);
    expect(deviceLayoutWindowKindsV1(layouts, "tablet")).toBeNull();
  });

  for (const row of fixture.announcements) {
    it(`announces ${row.outcome} in ${row.locale}`, () => {
      const outcome = row.outcome === "refused" ? planFocusWindowV1({ declaredKindIds: ["a"], layouts: [], device: "desktop", requested: row.kind, fallback: undefined, mounted: [], activeWindowId: null }) : plan({ requested: row.kind, mounted: row.outcome === "raised" ? [{ windowId: "a", windowKindId: "a" }, { windowId: "b", windowKindId: "b" }] : [{ windowId: "a", windowKindId: "a" }] });
      expect(outcome.outcome).toBe(row.outcome);
      const names = row.outcome === "refused" ? (windowKindId: string, locale: ShellLocale) => (windowKindId === "a" ? nameOf("a", locale) : windowKindId) : nameOf;
      expect(focusWindowNoticeV1(outcome, names, row.locale as ShellLocale)).toMatchObject({ text: row.message, code: row.code, severity: row.outcome === "refused" ? "warning" : "info" });
    });
  }

  for (const row of fixture.unavailable) {
    it(`announces an unavailable window on ${row.device} in ${row.locale}`, () => {
      const outcome = plan({ requested: row.kind, device: row.device as FocusWindowDeviceV1 });
      expect(outcome).toMatchObject({ outcome: "refused", reason: "unavailable" });
      expect(focusWindowNoticeV1(outcome, nameOf, row.locale as ShellLocale)).toEqual({ text: row.message, code: row.code, severity: "warning" });
    });
  }

  it("moves the keyboard focus into the body of the stack-active window", async () => {
    document.body.innerHTML = `<div data-slot="mode-dock-tab" data-window-id="b" data-stack-active="true"><button role="tab" aria-controls="panel-b"></button></div><div id="panel-b" role="tabpanel" tabindex="0"><span>text</span><button id="first">first</button></div>`;
    expect(await focusWindowBodyV1("b", document)).toBe(true);
    expect(document.activeElement?.id).toBe("first");
    document.body.innerHTML = "";
    expect(await focusWindowBodyV1("b", document, 1)).toBe(false);
  });
});
