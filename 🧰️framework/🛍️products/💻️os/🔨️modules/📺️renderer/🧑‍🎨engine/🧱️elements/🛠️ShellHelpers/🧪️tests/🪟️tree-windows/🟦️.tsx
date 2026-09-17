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
import { TREE_WINDOW_PATH_SEPARATOR } from "@semio-tech/ui-react";
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

  /** 🧯️ 📓️s3-review-streaming-loop.md §3: an open container the reader has scrolled PAST used to look
   * exactly like one they had just opened, so it asked for a whole viewport of rows nobody was looking at —
   * out of the same body budget the visible containers were competing for — and forgot where it had been
   * scrolled to. The observer reports the whole body, so "off screen" arrives as `rows: 0` at the offset it
   * already holds, and the first-paint fallback speaks only for a container that is not in the body yet. */
  it("asks an off-screen container for no rows at the offset it already holds, and only a never-measured one for a viewport", () => {
    const timers = manualTimers();
    const scheduler = createTreeWindowSchedulerV1({ refresh: () => {}, setTimer: timers.setTimer, clearTimer: timers.clearTimer });
    scheduler.setOpen("outliner", "just-opened", true);
    scheduler.settled("outliner");
    scheduler.reportWindows(
      "outliner",
      [
        { nodeKey: "on-screen", offset: 40, rows: 26 },
        { nodeKey: "scrolled-past", offset: 96, rows: 0 },
      ],
      20,
    );
    timers.fire();

    expect(scheduler.viewStateFields().treeWindows).toEqual([
      // 🪟️ A measured container is a rendered container: the host records it open without being told.
      { bodyKey: "outliner", nodeKey: "just-opened", open: true, offset: 0, rows: 20 },
      { bodyKey: "outliner", nodeKey: "on-screen", open: true, offset: 40, rows: 26 },
      { bodyKey: "outliner", nodeKey: "scrolled-past", open: true, offset: 96, rows: 0 },
    ]);
  });

  it("never lets a report in flight re-open a container the reader has just folded", () => {
    const timers = manualTimers();
    const scheduler = createTreeWindowSchedulerV1({ refresh: () => {}, setTimer: timers.setTimer, clearTimer: timers.clearTimer });
    scheduler.setOpen("outliner", "objects", false);
    scheduler.settled("outliner");
    scheduler.reportWindows("outliner", [{ nodeKey: "objects", offset: 0, rows: 26 }], 20);
    timers.fire();

    expect(scheduler.viewStateFields().treeWindows).toEqual([{ bodyKey: "outliner", nodeKey: "objects", open: false, offset: 0, rows: 26 }]);
    expect(scheduler.openStatesFor("outliner")).toEqual({ objects: false });
  });

  it("forgets a container that has left the body, because the report is the whole body", () => {
    const timers = manualTimers();
    const scheduler = createTreeWindowSchedulerV1({ refresh: () => {}, setTimer: timers.setTimer, clearTimer: timers.clearTimer });
    scheduler.reportWindows("outliner", [{ nodeKey: "old-doc.a", offset: 12, rows: 20 }], 20);
    timers.fire();
    scheduler.settled("outliner");
    scheduler.reportWindows("outliner", [{ nodeKey: "new-doc.a", offset: 0, rows: 20 }], 20);
    timers.fire();

    expect(scheduler.viewStateFields().treeWindows?.map((request) => [request.nodeKey, request.rows])).toEqual([
      // 🪟️ The open flag outlives the measurement on purpose — a fold the reader chose must survive a
      // branch closing over it — but the container asks for nothing until it is in the body again.
      ["new-doc.a", 20],
      ["old-doc.a", 0],
    ]);

    // 🪟️ …and an explicit re-open is a request for rows again, whatever the observer last measured.
    scheduler.setOpen("outliner", "old-doc.a", false);
    scheduler.setOpen("outliner", "old-doc.a", true);
    expect(scheduler.viewStateFields().treeWindows?.find((request) => request.nodeKey === "old-doc.a")?.rows).toBe(20);
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

  const sectionState = (container: Element) => container.querySelector('[data-slot="tree-section-row"]')?.getAttribute("data-state");

  it("renders a host-closed container closed although the author defaulted it open", () => {
    const sink = { opens: [] as [string, string, boolean][] };
    const open = uiNodeToTreePanelConfig(outlinerBody(), () => {}, "outliner", hostFor({ "outliner.objects": true }, sink));
    const shown = render(createElement(Fragment, null, open.emptyState));
    expect(sectionState(shown.container)).toBe("open");
    expect(shown.container.textContent).toContain("Seed Left");
    cleanup();
    // 🪟️ The author's `defaultOpen: true` is unchanged — only the host's map says otherwise, and the
    // host wins. This is the expansion that has to survive the next body refresh.
    const closed = uiNodeToTreePanelConfig(outlinerBody(), () => {}, "outliner", hostFor({ "outliner.objects": false }, sink));
    const hidden = render(createElement(Fragment, null, closed.emptyState));
    expect(hidden.container.textContent).toContain("Objects");
    expect(sectionState(hidden.container)).toBe("closed");
  });

  it("reports a fold toggle back under the AUTHORED node key, not the surface-prefixed DOM id", () => {
    const sink = { opens: [] as [string, string, boolean][] };
    const config = uiNodeToTreePanelConfig(outlinerBody(), () => {}, "outliner", hostFor({ "outliner.objects": true }, sink));
    const rendered = render(createElement(Fragment, null, config.emptyState));
    const row = rendered.container.querySelector('[data-slot="tree-section-row"]');
    expect(row).toBeTruthy();
    fireEvent.click(row as Element);
    // 🔑️ `🌳️Tree` routes one fold through its provider twice under the role-prefixed state id
    // `tree-section-panel:outliner/outliner.objects`; every report arrives re-keyed on the authored key.
    expect(sink.opens.length).toBeGreaterThan(0);
    expect(new Set(sink.opens.map((entry) => JSON.stringify(entry)))).toEqual(new Set([JSON.stringify(["outliner", "outliner.objects", false])]));
  });

  /** 🔑️ The same node key under two parents is two windows — `📐️cad` builds one `object.id` under four pane
   * sections and that key is also the pick target id, so it cannot be namespaced
   * (📓️f2-sdk-body-node-ledger.md §10). Host-owned expansion therefore keys by PATH. */
  it("folds one of two containers that share a node key, by path, and leaves its twin open", () => {
    const sink = { opens: [] as [string, string, boolean][] };
    const shared = (parent: string) =>
      builtNode(parent, { type: "treeSection", label: parent, defaultOpen: true, window: { total: 2, offset: 0 } }, [
        builtNode("shared", { type: "treeItem", label: "Shared", description: null, icon: null, defaultOpen: true, draggable: null, dragData: null, dimmed: null, rowActions: [], window: { total: 1, offset: 0 } }, [
          builtNode(`${parent}.shared.0`, { type: "treeItem", label: `${parent} child`, description: null, icon: null, defaultOpen: null, draggable: null, dragData: null, dimmed: null, rowActions: [] }),
        ]),
      ]);
    const body = builtNode("outliner", { type: "tree", interactionDomain: null }, [shared("left"), shared("right")]);
    const closedLeft = { [`left${TREE_WINDOW_PATH_SEPARATOR}shared`]: false, [`right${TREE_WINDOW_PATH_SEPARATOR}shared`]: true };

    const config = uiNodeToTreePanelConfig(body, () => {}, "outliner", hostFor(closedLeft, sink));
    const rendered = render(createElement(Fragment, null, config.emptyState));

    // 🪟️ A closed group renders no branch content at all, so its window path is simply not in the DOM.
    const paths = Array.from(rendered.container.querySelectorAll("[data-tree-window-path]")).map((element) => element.getAttribute("data-tree-window-path"));
    expect(paths).toEqual(["left", "right", `right${TREE_WINDOW_PATH_SEPARATOR}shared`]);
    expect(rendered.container.textContent).not.toContain("left child");
    expect(rendered.container.textContent).toContain("right child");

    const groups = rendered.container.querySelectorAll('[data-tree-row-kind="group"]');
    expect(groups.length).toBe(2);
    // 🖱️ A group row folds from its chevron; the row shell itself carries the selection handler.
    fireEvent.click((groups[1] as Element).querySelector("button") as Element);
    // 🔑️ …and the fold comes back under the RIGHT branch's path, never the bare shared key.
    expect(new Set(sink.opens.map((entry) => entry[1]))).toEqual(new Set([`right${TREE_WINDOW_PATH_SEPARATOR}shared`]));
  });

  it("leaves the tree uncontrolled when no host channel is provided", () => {
    const config = uiNodeToTreePanelConfig(outlinerBody(), () => {}, "outliner");
    const rendered = render(createElement(Fragment, null, config.emptyState));
    expect(rendered.container.textContent).toContain("Seed Left");
    expect(sectionState(rendered.container)).toBe("open");
    fireEvent.click(rendered.container.querySelector('[data-slot="tree-section-row"]') as Element);
    expect(sectionState(rendered.container)).toBe("closed");
  });
});
//#endregion 🌲️PanelBody
