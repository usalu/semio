// #region 🧲️Header
/** @emoji 🪟️ The host's half of the virtualised panel tree (🎫️ 26/09/16
 * ARTIFACT-TREE-VIRTUALISED-STREAMING, 📓️design-virtualised-tree.md §6.3): the refresh scheduler's
 * three timing rules, and `uiNodeToTreePanelConfig` handing host-owned expansion to the guest tree. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { afterEach, describe, expect, it } from "vitest";
import { createElement, Fragment } from "react";
import { cleanup, fireEvent, render } from "@semio-tech/ui-react/test";
import type { BuiltNode } from "@semio-tech/framework";
import { createTreeWindowSchedulerV1, TREE_WINDOW_DEFAULT_ROWS, uiNodeToTreePanelConfig, type TreeWindowHostV1 } from "../../🟦️.tsx";
// #endregion 🔌️Adapters

//#region 🧱️Fixtures
type AnyRecord = Record<string, any>;

const TEST_LAYOUT = { kind: "leaf", width: "hug", height: "hug" } as const;
const TEST_STYLE = { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" } as const;
const TEST_ACCESSIBILITY = { label: null, description: null, live: "off", shortcut: null, hidden: false } as const;

function builtNode(key: string, component: AnyRecord, children: readonly AnyRecord[] = []): BuiltNode {
  return { key, component, layout: TEST_LAYOUT, style: TEST_STYLE, activity: "idle", disabled: false, transition: null, accessibility: TEST_ACCESSIBILITY, bindings: [], menu: null, children: [...children] } as unknown as BuiltNode;
}

function outlinerBody(): BuiltNode {
  return builtNode("outliner", { type: "tree", interactionDomain: null }, [
    builtNode("outliner.objects", { type: "treeSection", label: "Objects", defaultOpen: true, window: { total: 3, offset: 0 } }, [
      builtNode("seed-left-001", { type: "treeItem", label: "Seed Left", description: null, icon: null, defaultOpen: null, draggable: null, dragData: null, dimmed: null, rowActions: [] }),
    ]),
  ]);
}

/** 🪟️ A deterministic clock for the trailing debounce — no timers, no `vi.useFakeTimers`. */
function manualTimers() {
  const queue: { handle: number; run: () => void }[] = [];
  let next = 1;
  return {
    setTimer: (run: () => void) => {
      const handle = next++;
      queue.push({ handle, run });
      return handle;
    },
    clearTimer: (handle: unknown) => {
      const index = queue.findIndex((entry) => entry.handle === handle);
      if (index >= 0) queue.splice(index, 1);
    },
    pending: () => queue.length,
    fire: () => {
      const entry = queue.shift();
      entry?.run();
    },
  };
}

function hostFor(openStates: Readonly<Record<string, boolean>>, sink: { readonly opens: [string, string, boolean][] }): TreeWindowHostV1 {
  return {
    openStatesFor: () => openStates,
    setOpen: (bodyKey, nodeKey, open) => sink.opens.push([bodyKey, nodeKey, open]),
    reportWindows: () => {},
  };
}
//#endregion 🧱️Fixtures

//#region ⏱️Scheduling
describe("🪟️ tree window scheduler", () => {
  it("sends an open toggle immediately and coalesces window reports onto the trailing edge", () => {
    const refreshed: string[] = [];
    const timers = manualTimers();
    const scheduler = createTreeWindowSchedulerV1({ refresh: (bodyKey) => refreshed.push(bodyKey), setTimer: timers.setTimer, clearTimer: timers.clearTimer });

    scheduler.setOpen("outliner", "outliner.objects", true);
    expect(refreshed).toEqual(["outliner"]);
    expect(timers.pending()).toBe(0);
    scheduler.settled("outliner");

    scheduler.reportWindows("outliner", [{ nodeKey: "outliner.objects", offset: 0, rows: 40 }], 32);
    scheduler.reportWindows("outliner", [{ nodeKey: "outliner.objects", offset: 8, rows: 40 }], 32);
    scheduler.reportWindows("outliner", [{ nodeKey: "outliner.objects", offset: 96, rows: 40 }], 32);
    expect(refreshed).toEqual(["outliner"]);
    expect(timers.pending()).toBe(1);
    timers.fire();
    // 🪟️ One refresh for the whole gesture, carrying the LAST offset the scroll settled on.
    expect(refreshed).toEqual(["outliner", "outliner"]);
    expect(scheduler.viewStateFields().treeWindows).toEqual([{ bodyKey: "outliner", nodeKey: "outliner.objects", open: true, offset: 96, rows: 40 }]);
    expect(scheduler.viewStateFields().treeViewportRows).toBe(32);
  });

  it("re-reports a report that changed nothing not at all, and never schedules for it", () => {
    const refreshed: string[] = [];
    const timers = manualTimers();
    const scheduler = createTreeWindowSchedulerV1({ refresh: (bodyKey) => refreshed.push(bodyKey), setTimer: timers.setTimer, clearTimer: timers.clearTimer });
    scheduler.reportWindows("outliner", [{ nodeKey: "objects", offset: 0, rows: 40 }], 32);
    timers.fire();
    scheduler.settled("outliner");
    scheduler.reportWindows("outliner", [{ nodeKey: "objects", offset: 0, rows: 40 }], 32);
    expect(timers.pending()).toBe(0);
    expect(refreshed).toEqual(["outliner"]);
  });

  it("keeps one refresh per body in flight and re-sends the LATEST state when it settles", () => {
    const refreshed: string[] = [];
    const timers = manualTimers();
    const scheduler = createTreeWindowSchedulerV1({ refresh: (bodyKey) => refreshed.push(bodyKey), setTimer: timers.setTimer, clearTimer: timers.clearTimer });
    scheduler.setOpen("outliner", "a", true);
    expect(refreshed).toEqual(["outliner"]);
    scheduler.setOpen("outliner", "b", true);
    scheduler.setOpen("outliner", "c", true);
    expect(refreshed).toEqual(["outliner"]);
    scheduler.settled("outliner");
    expect(refreshed).toEqual(["outliner", "outliner"]);
    expect(scheduler.viewStateFields().treeWindows?.map((request) => request.nodeKey)).toEqual(["a", "b", "c"]);
    // 🪟️ A container the observer has not measured asks for one viewport of rows, never zero.
    expect(scheduler.viewStateFields().treeWindows?.every((request) => request.rows === TREE_WINDOW_DEFAULT_ROWS)).toBe(true);
    scheduler.settled("outliner");
    expect(refreshed).toEqual(["outliner", "outliner"]);
  });

  it("drops every body's state on reset, so a session switch never asks a new guest about old containers", () => {
    const scheduler = createTreeWindowSchedulerV1({ refresh: () => {}, setTimer: manualTimers().setTimer, clearTimer: () => {} });
    scheduler.setOpen("outliner", "a", true);
    scheduler.settled("outliner");
    scheduler.reset();
    expect(scheduler.viewStateFields()).toEqual({});
    expect(scheduler.openStatesFor("outliner")).toEqual({});
  });
});
//#endregion ⏱️Scheduling

//#region 🌲️PanelBody
describe("🪟️ panel body tree window context", () => {
  afterEach(() => cleanup());

  it("renders a host-closed container closed although the author defaulted it open", () => {
    const sink = { opens: [] as [string, string, boolean][] };
    const open = uiNodeToTreePanelConfig(outlinerBody(), () => {}, "outliner", hostFor({ "outliner.objects": true }, sink));
    const shown = render(createElement(Fragment, null, open.emptyState));
    expect(shown.container.textContent).toContain("Seed Left");
    cleanup();
    const closed = uiNodeToTreePanelConfig(outlinerBody(), () => {}, "outliner", hostFor({ "outliner.objects": false }, sink));
    const hidden = render(createElement(Fragment, null, closed.emptyState));
    expect(hidden.container.textContent).toContain("Objects");
    expect(hidden.container.textContent).not.toContain("Seed Left");
  });

  it("reports a fold toggle back under the AUTHORED node key, not the surface-prefixed DOM id", () => {
    const sink = { opens: [] as [string, string, boolean][] };
    const config = uiNodeToTreePanelConfig(outlinerBody(), () => {}, "outliner", hostFor({ "outliner.objects": true }, sink));
    const rendered = render(createElement(Fragment, null, config.emptyState));
    const row = rendered.container.querySelector('[data-slot="tree-section-row"]');
    expect(row).toBeTruthy();
    fireEvent.click(row as Element);
    expect(sink.opens).toEqual([["outliner", "outliner.objects", false]]);
  });

  it("leaves the tree uncontrolled when no host channel is provided", () => {
    const config = uiNodeToTreePanelConfig(outlinerBody(), () => {}, "outliner");
    const rendered = render(createElement(Fragment, null, config.emptyState));
    expect(rendered.container.textContent).toContain("Seed Left");
    const row = rendered.container.querySelector('[data-slot="tree-section-row"]');
    fireEvent.click(row as Element);
    expect(rendered.container.textContent).not.toContain("Seed Left");
  });
});
//#endregion 🌲️PanelBody
