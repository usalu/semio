/** 🔁 Framework sync utilities, live measure dispatch, interval skipping, ui refresh coalescing and identity preservation. */
import { ChromeAwareWindowScrollSurface } from "@semio-tech/ui-react";
import canvasClearance from "../../🧱️elements/🗣️Interpreter/🧪️tests/🧫️fixtures/🪟️canvas-clearance/🔣️.json";
import { createElement } from "react";
import { uiI18n } from "@semio-tech/ui-react";
import { type UiDirtyScope } from "@semio-tech/framework";
import { panelViewContext } from "@semio-tech/framework/modules/manifest/view-state-and-public-invocation";
import { cleanup, render } from "@semio-tech/ui-react/test";
import { describe, expect, it, vi } from "vitest";
import { type UtilityNode } from "@semio-tech/framework";
import { InterpretedUiNode, UiDocumentStore, applyUiRefreshResponseToCache, buildActiveUtilityByWindowId, buildUiRefreshRequest, groupUtilityNodesByCategory, mergeRecordPreservingIdentity, preserveJsonIdentity, createLatestAsyncDispatcher, createDirectionalAsyncDispatcher, createInFlightSkippingInterval, createCoalescingActionDispatcher, type UiRefreshCache, SyncAttachCard } from "../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import { createUiRefreshCoalescerV1, mergeUiDirtyScopeV1 } from "../../🧱️elements/🛠️ShellHelpers/🔄️ui-refresh/🟦️.ts";
import { buildContractSnapshot, computeAccessibleName, noopAction } from "../🧰️engine-contract-helpers/🟦️.ts";

it("keeps the interpreted canvas visible beneath expanded Actions chrome", () => {
  for (const row of canvasClearance.cases) {
    if(row.kind!=="canvas-2d"&&row.kind!=="table")throw Error("Undeclared canvas clearance fixture surface kind");
    const bounds = vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockImplementation(function (this: HTMLElement) {
      const bottom = this.getAttribute("data-slot") === "window-engagement-overlay" ? row.chromeHeight : 720;
      return { x: 0, y: 0, top: 0, bottom, left: 0, right: 1280, width: 1280, height: bottom, toJSON: () => ({}) };
    });
    const store = new UiDocumentStore("canvas-clearance");
    store.loadSnapshot(buildContractSnapshot({ key: "canvas", component: { type: "surface", kind: row.kind, docSchema: `${row.kind}@1`, doc: { bytes: [] }, bindings: [] } }));
    const view = render(createElement("div", { "data-slot": "window-body" },
      createElement("div", { "data-slot": "window-engagement-overlay" }, "Actions"),
      createElement(ChromeAwareWindowScrollSurface, { ref: (el: HTMLDivElement | null) => { if (el) el.scrollTop = row.scrollTop; } },
        createElement(InterpretedUiNode, { store, onAction: noopAction, onIntent: noopAction, requestContextMenu: undefined }))));
    try {
      const surface = view.container.querySelector<HTMLElement>('[data-slot="window-dead-line-scroll"]')!;
      expect(Number.parseFloat(surface.style.paddingBlockStart) || 0, row.id).toBe(row.inset);
      expect(surface.scrollTop, row.id).toBe(row.scrollTop);
      expect(surface.querySelector('[role="application"]'), row.id).not.toBeNull();
    } finally { view.unmount(); bounds.mockRestore(); }
  }
});

describe("framework sync utilities", () => {
  it("renders the real SyncAttachCard popover as a dismissible nonmodal dialog", () => {
    const close = vi.fn();
    const { getByRole } = render(
      createElement(SyncAttachCard, {
        activeUri: null,
        cardKind: "file",
        draftPath: "/tmp/document.json",
        syncUtilities: [],
        status: null,
        quarantinedConflicts: [],
        onAction: vi.fn(),
        onDraftPathChange: vi.fn(),
        onClose: close,
        onAttach: vi.fn(),
        onDetach: vi.fn(),
        onBrowsePath: vi.fn(),
      }),
    );
    const dialog = getByRole("dialog");
    expect(dialog.getAttribute("aria-modal")).toBeNull();
    expect(document.activeElement).toBe(getByRole("textbox"));
    document.dispatchEvent(new KeyboardEvent("keydown", { key: "Escape", bubbles: true }));
    cleanup();
    expect(close).toHaveBeenCalledTimes(1);
  });

  it("offers localized file and folder picker affordances through the owned browse control", async () => {
    const browseFile = vi.fn();
    const browseFolder = vi.fn();
    await uiI18n.changeLanguage("en");
    const file = render(
      createElement(SyncAttachCard, {
        activeUri: null,
        cardKind: "file",
        draftPath: "",
        syncUtilities: [],
        status: null,
        quarantinedConflicts: [],
        onAction: vi.fn(),
        onDraftPathChange: vi.fn(),
        onClose: vi.fn(),
        onAttach: vi.fn(),
        onDetach: vi.fn(),
        onBrowsePath: browseFile,
      }),
    );
    const fileBrowse = document.querySelector<HTMLElement>("[data-semio-sync-browse]");
    expect(computeAccessibleName(fileBrowse!)).toBe("Browse");
    fileBrowse?.click();
    expect(browseFile).toHaveBeenCalledTimes(1);
    file.unmount();
    await uiI18n.changeLanguage("de");
    const folder = render(
      createElement(SyncAttachCard, {
        activeUri: null,
        cardKind: "folder",
        draftPath: "",
        syncUtilities: [],
        status: null,
        quarantinedConflicts: [],
        onAction: vi.fn(),
        onDraftPathChange: vi.fn(),
        onClose: vi.fn(),
        onAttach: vi.fn(),
        onDetach: vi.fn(),
        onBrowsePath: browseFolder,
      }),
    );
    const folderBrowse = document.querySelector<HTMLElement>("[data-semio-sync-browse]");
    expect(computeAccessibleName(folderBrowse!)).toBe("Durchsuchen");
    folderBrowse?.click();
    expect(browseFolder).toHaveBeenCalledTimes(1);
    folder.unmount();
    const remote = render(
      createElement(SyncAttachCard, {
        activeUri: null,
        cardKind: "remote",
        draftPath: "",
        syncUtilities: [],
        status: null,
        quarantinedConflicts: [],
        onAction: vi.fn(),
        onDraftPathChange: vi.fn(),
        onClose: vi.fn(),
        onAttach: vi.fn(),
        onDetach: vi.fn(),
        onBrowsePath: vi.fn(),
      }),
    );
    expect(remote.container.querySelector("[data-semio-sync-browse]")).toBeNull();
    remote.unmount();
    const { spacesUiLabel } = await import("@semio-tech/framework-renderer-react");
    expect(spacesUiLabel("ui.sync.browse", "en")).toBe("Browse…");
    expect(spacesUiLabel("ui.sync.browse", "de")).toBe("Durchsuchen…");
    await uiI18n.changeLanguage("en");
  });

  it("builds three sync backbone toggles", async () => {
    const { buildFrameworkSyncUtilities } = await import("@semio-tech/framework-os");
    const utilities = buildFrameworkSyncUtilities("file:///demo");
    expect(utilities).toHaveLength(3);
    expect(utilities.map((utility) => utility.id)).toEqual(["framework.sync.file", "framework.sync.folder", "framework.sync.remote"]);
    expect(utilities[0]?.pressed).toBe(true);
  });

  it("has no active toggle when detached", async () => {
    const { buildFrameworkSyncUtilities } = await import("@semio-tech/framework-os");
    const utilities = buildFrameworkSyncUtilities(null);
    expect(utilities.every((utility) => !utility.pressed)).toBe(true);
  });

  it("groups File, Folder, and Remote under a single Sync category collection", async () => {
    const { buildFrameworkSyncUtilities } = await import("@semio-tech/framework-os");
    const utilities = buildFrameworkSyncUtilities("file:///demo");
    const grouped = groupUtilityNodesByCategory(utilities as unknown as UtilityNode[], ["sync"]);
    expect(grouped).toHaveLength(1);
    expect(grouped[0]).toMatchObject({ id: "sync", kind: "collection" });
    expect(grouped[0].kind === "collection" ? grouped[0].children.map((child) => child.id) : []).toEqual(["framework.sync.file", "framework.sync.folder", "framework.sync.remote"]);
  });

  it("matches the sync backbone path picker fixture", async () => {
    const fixture = (await import("../../../../../🧪️tests/🧫️fixtures/🔗️sync-backbone-path-picker/🔣️.json")).default as {
      react: { helpers: string[]; hostIoOps?: string[] };
      wgpu: { hostIoOps: string[] };
    };
    const helpers = await import("../../🧱️elements/🛠️ShellHelpers/🔩️plumbing/📥️downloads-and-file-requests/🟦️.ts");
    for (const name of fixture.react.helpers) expect(typeof (helpers as Record<string, unknown>)[name]).toBe("function");
    const { default: hostIo } = await import("../../🎯️targets/🧊️wgpu/🚪️host-io/🟦️.ts?raw");
    for (const op of fixture.wgpu.hostIoOps) expect(hostIo).toContain(op);
  });
});

describe("live measure dispatch", () => {
  it("serializes document updates and skips stale slider values", async () => {
    const values: number[] = [];
    let finishFirst = () => {};
    const dispatch = createLatestAsyncDispatcher((value: number) => {
      values.push(value);
      if (value === 1) return new Promise<void>((resolve) => (finishFirst = resolve));
    });

    dispatch(1);
    dispatch(2);
    dispatch(19);
    dispatch(24);
    expect(values).toEqual([1]);

    finishFirst();
    await vi.waitFor(() => expect(values).toEqual([1, 24]));
  });

  it("coalesces straight slider movement but preserves a down-up reversal", async () => {
    const values: number[] = [];
    const finishes: Array<() => void> = [];
    const dispatch = createDirectionalAsyncDispatcher(
      (value) =>
        new Promise<void>((resolve) => {
          values.push(value);
          finishes.push(resolve);
        }),
    );

    dispatch(20);
    dispatch(18);
    dispatch(12);
    dispatch(8);
    dispatch(14);
    dispatch(20);
    expect(values).toEqual([20]);

    finishes.shift()?.();
    await vi.waitFor(() => expect(values).toEqual([20, 8]));
    finishes.shift()?.();
    await vi.waitFor(() => expect(values).toEqual([20, 8, 20]));
    finishes.shift()?.();
  });

  it("caps queued direction reversals so a jittery drag cannot grow the queue unbounded", async () => {
    const values: number[] = [];
    const finishes: Array<() => void> = [];
    const dispatch = createDirectionalAsyncDispatcher(
      (value) =>
        new Promise<void>((resolve) => {
          values.push(value);
          finishes.push(resolve);
        }),
    );

    dispatch(0);
    for (let i = 1; i <= 40; i += 1) {
      dispatch(i % 2 === 0 ? 100 + i : -100 - i);
    }
    expect(values).toEqual([0]);

    finishes.shift()?.();
    await vi.waitFor(() => expect(values).toHaveLength(2));
    finishes.shift()?.();
    await vi.waitFor(() => expect(values).toHaveLength(3));
    finishes.shift()?.();
    expect(values).toHaveLength(3);
  });
});

describe("in-flight skipping interval", () => {
  it("drops overlapping ticks instead of queueing them behind a slow run", async () => {
    const runs: number[] = [];
    let finishFirst = () => {};
    const timers: Array<() => void> = [];
    const stop = createInFlightSkippingInterval(
      () => {
        runs.push(runs.length + 1);
        if (runs.length === 1) return new Promise<void>((resolve) => (finishFirst = resolve));
      },
      10,
      (fn) => {
        timers.push(fn);
        return 1;
      },
      () => {},
    );

    expect(timers).toHaveLength(1);
    timers[0]!();
    timers[0]!();
    timers[0]!();
    expect(runs).toEqual([1]);

    finishFirst();
    await Promise.resolve();
    timers[0]!();
    expect(runs).toEqual([1, 2]);
    stop();
  });

  it("gates on exactly what run returns — a discarded dispatch promise gates nothing", async () => {
    const started: number[] = [];
    const timers: Array<() => void> = [];
    const never = new Promise<void>(() => {});
    const drive = async (run: () => unknown) => {
      timers.length = 0;
      const stop = createInFlightSkippingInterval(
        run,
        10,
        (fn) => (timers.push(fn), 1),
        () => {},
      );
      for (let tick = 0; tick < 3; tick += 1) {
        timers[0]!();
        await Promise.resolve();
        await Promise.resolve();
      }
      stop();
    };

    await drive(() => {
      started.push(started.length + 1);
      void never;
    });
    expect(started, "a run that swallows its dispatch cannot be gated — every tick fires").toHaveLength(3);

    started.length = 0;
    await drive(() => {
      started.push(started.length + 1);
      return never;
    });
    expect(started, "returning the dispatch is what keeps exactly one tick outstanding").toHaveLength(1);
  });
});

describe("coalescing action dispatcher", () => {
  it("dedupes unchanged values and keeps at most one in-flight dispatch", async () => {
    const calls: string[] = [];
    let finishFirst = () => {};
    const send = createCoalescingActionDispatcher<string | null>((value) => {
      calls.push(value ?? "null");
      if (calls.length === 1) return new Promise<void>((resolve) => (finishFirst = resolve));
    });
    send("a");
    send("a");
    send("b");
    expect(calls).toEqual(["a"]);
    finishFirst();
    await Promise.resolve();
    expect(calls).toEqual(["a", "b"]);
    send("b");
    expect(calls).toEqual(["a", "b"]);
  });

  it("gates on exactly what the dispatch returns — a swallowed round trip coalesces nothing", async () => {
    const storm = ["a", "b", "c", "d", "e"] as const;
    const drive = async (shape: (value: string, sent: readonly string[]) => unknown) => {
      const sent: string[] = [];
      const send = createCoalescingActionDispatcher<string>((value) => shape(value, (sent.push(value), sent)));
      for (const value of storm) {
        send(value);
        await Promise.resolve();
        await Promise.resolve();
      }
      return sent;
    };

    const pending = new Promise<void>(() => {});
    const swallowed = await drive(() => {
      void pending;
    });
    expect(swallowed, "a hover dispatcher whose callback drops the round trip sends one turn per pointer move").toEqual([...storm]);

    const awaited = await drive(() => pending);
    expect(awaited, "returning the round trip is what keeps exactly ONE hover outstanding while the guest is busy").toEqual(["a"]);
  });

  it("keeps flushing after a rejected round trip", async () => {
    const calls: string[] = [];
    const send = createCoalescingActionDispatcher<string>((value) => {
      calls.push(value);
      return calls.length === 1 ? Promise.reject(new Error("refused hover")) : Promise.resolve();
    });
    send("a");
    expect(calls).toEqual(["a"]);
    await Promise.resolve();
    await Promise.resolve();
    send("b");
    await Promise.resolve();
    expect(calls, "a refusal clears the in-flight flag, so the next hover still reaches the guest").toEqual(["a", "b"]);
  });
});

describe("ui refresh coalescing lane", () => {
  type LaneRequest = { readonly scope: UiDirtyScope; readonly utilities: Readonly<Record<string, string>>; readonly replaceBodies: boolean };
  const mergeLane = (owed: LaneRequest, next: LaneRequest): LaneRequest => ({ scope: mergeUiDirtyScopeV1(owed.scope, next.scope), utilities: next.utilities, replaceBodies: owed.replaceBodies || next.replaceBodies });
  /** 🧪️ One lane plus a hand-held pass: `settle()` finishes the pass the lane is currently running, so a
   * law can hold a pass open for exactly as long as it needs and read what the FOLLOW-UP carried. */
  const lane = () => {
    const taken: LaneRequest[] = [];
    let finish: { resolve: () => void; reject: (error: unknown) => void } | null = null;
    const coalescer = createUiRefreshCoalescerV1<LaneRequest>((request) => {
      taken.push(request);
      return new Promise<void>((resolve, reject) => {
        finish = { resolve, reject };
      });
    }, mergeLane);
    const settle = async (error?: unknown) => {
      const pass = finish;
      finish = null;
      if (error === undefined) pass?.resolve();
      else pass?.reject(error);
      await Promise.resolve();
      await Promise.resolve();
      await Promise.resolve();
    };
    return { coalescer, taken, settle };
  };
  const partial = (windowBodies: readonly string[], flags: Partial<UiDirtyScope & { readonly utilities: boolean }> = {}): UiDirtyScope => ({ kind: "partial", windowBodies: [...windowBodies], ...flags }) as UiDirtyScope;

  it("turns every refresh asked for during an in-flight pass into exactly ONE follow-up carrying their union", async () => {
    const { coalescer, taken, settle } = lane();
    const first = coalescer.request({ scope: partial(["world"]), utilities: {}, replaceBodies: false });
    expect(
      taken.map((request) => request.scope),
      "the starter's own request is what the first pass runs",
    ).toEqual([partial(["world"])]);
    let firstSettled = false;
    void first.then(() => {
      firstSettled = true;
    });

    const second = coalescer.request({ scope: partial(["panel"], { utilities: true }), utilities: { "world-1": "brush" }, replaceBodies: true });
    let secondSettled = false;
    void second.then(() => {
      secondSettled = true;
    });
    void coalescer.request({ scope: partial(["measures-only"], { measures: true }), utilities: { "world-1": "brush" }, replaceBodies: false });
    expect(taken, "neither request may start a second pass while one is crossing into the guest").toHaveLength(1);

    await settle();
    expect(firstSettled, "the starter's promise settles on the pass that covered it, not on the whole drain").toBe(true);
    expect(secondSettled, "a joiner is only covered by the FOLLOW-UP, so it may not settle on the pass that was already running").toBe(false);
    expect(taken, "two requests during one pass owe exactly one follow-up").toHaveLength(2);
    expect(taken[1]!.scope, "the follow-up carries the union of everything asked for while the pass ran — a dropped field is a section that never repaints").toEqual({
      kind: "partial",
      windowBodies: ["panel", "measures-only"],
      panelBodies: [],
      utilities: true,
      tools: false,
      engagements: false,
      measures: true,
      labels: false,
    });
    expect(taken[1]!.replaceBodies, "a replace-bodies request may not be downgraded by merging").toBe(true);

    await settle();
    expect(secondSettled).toBe(true);
    expect(coalescer.passes(), "three requests, two passes — never one per request").toBe(2);
    expect(coalescer.busy()).toBe(false);
  });

  it("carries a utility armed by a host effect while a pass was in flight into the very next pass", async () => {
    const { coalescer, taken, settle } = lane();
    const armed: Record<string, string> = {};
    void coalescer.request({ scope: { kind: "full" }, utilities: { ...armed }, replaceBodies: false });
    expect(taken[0]!.utilities, "the pass that was already running predates the arm").toEqual({});

    armed["puzzle3d-main-perspective"] = "brush";
    void coalescer.request({ scope: { kind: "full" }, utilities: { ...armed }, replaceBodies: false });
    await settle();
    expect(taken, "the effect's own refresh is owed a pass — losing it leaves the armed pane publishing `select`").toHaveLength(2);
    expect(taken[1]!.utilities).toEqual({ "puzzle3d-main-perspective": "brush" });
    await settle();
    expect(coalescer.passes()).toBe(2);
  });

  it("answers a 50-request hover storm during one in-flight pass with exactly one follow-up", async () => {
    const { coalescer, taken, settle } = lane();
    void coalescer.request({ scope: partial(["world"]), utilities: {}, replaceBodies: false });
    for (let index = 0; index < 50; index += 1) void coalescer.request({ scope: partial(["world"]), utilities: {}, replaceBodies: false });
    expect(taken, "a storm behind a running pass costs nothing while it runs").toHaveLength(1);
    await settle();
    expect(taken, "50 requests during one pass owe ONE follow-up, never 50").toHaveLength(2);
    await settle();
    expect(coalescer.passes()).toBe(2);
    expect(coalescer.owedScope()).toBeNull();
  });

  it("runs the owed follow-up after a pass that REJECTED, and rejects only the requests that pass covered", async () => {
    const { coalescer, taken, settle } = lane();
    const starter = coalescer.request({ scope: partial(["world"]), utilities: {}, replaceBodies: false });
    const starterOutcome = starter.then(() => "resolved" as const).catch((error: unknown) => `rejected:${String(error)}`);
    const joiner = coalescer.request({ scope: partial(["panel"]), utilities: {}, replaceBodies: false });
    const joinerOutcome = joiner.then(() => "resolved" as const).catch((error: unknown) => `rejected:${String(error)}`);

    await settle(new Error("guest refused the refresh"));
    expect(taken, "a failed pass may not take the lane down with it").toHaveLength(2);
    expect(taken[1]!.scope).toEqual(partial(["panel"]));
    expect(await starterOutcome, "the boot refresh turns its own rejection into the session fault card — it must still arrive").toBe("rejected:Error: guest refused the refresh");
    await settle();
    expect(await joinerOutcome).toBe("resolved");
  });

  it("serves a request made from INSIDE a pass with the next pass instead of wedging the lane", async () => {
    const taken: UiDirtyScope[] = [];
    const finishers: Array<() => void> = [];
    let reentrant: Promise<void> | null = null;
    const coalescer = createUiRefreshCoalescerV1<LaneRequest>((request) => {
      taken.push(request.scope);
      return new Promise<void>((resolve) => {
        finishers.push(resolve);
        if (taken.length === 1) reentrant = coalescer.request({ scope: partial(["world"]), utilities: {}, replaceBodies: false });
      });
    }, mergeLane);
    void coalescer.request({ scope: { kind: "full" }, utilities: {}, replaceBodies: false });
    expect(taken).toHaveLength(1);
    let reentrantSettled = false;
    void reentrant!.then(() => {
      reentrantSettled = true;
    });

    finishers[0]!();
    await Promise.resolve();
    await Promise.resolve();
    await Promise.resolve();
    expect(taken, "the pass's own request is owed the next pass").toEqual([{ kind: "full" }, partial(["world"])]);
    expect(reentrantSettled, "a re-entrant request is never settled by the pass that made it").toBe(false);
    finishers[1]!();
    await Promise.resolve();
    await Promise.resolve();
    expect(reentrantSettled).toBe(true);
    expect(coalescer.busy()).toBe(false);
  });

  it("asks for nothing at all on a none scope", async () => {
    const { coalescer, taken } = lane();
    await coalescer.request({ scope: { kind: "none" }, utilities: {}, replaceBodies: false });
    expect(taken).toHaveLength(0);
    expect(coalescer.passes()).toBe(0);
    expect(coalescer.busy()).toBe(false);
  });
});

// 🐢️ Puzzle 2D performance round 2: the per-interaction full-shell refresh cascade was dominated by
// React reconciling freshly-parsed-but-structurally-identical UiNode/engagement/measure trees on every
// action (select/camera/nodeMove). These helpers let unchanged bodies keep their object identity across
// a `refreshUi` so `InterpretedUiNode`'s `React.memo` (ui-interpreter.tsx) and `modeWindows`'s
// `useMemo` (os-shell.tsx) can bail instead of reconciling the whole shell every time.
describe("ui identity preservation (puzzle 2d perf)", () => {
  it("preserveJsonIdentity reuses the previous reference for structurally-equal values", () => {
    const previous = { type: "text", value: "hello" };
    const next = { type: "text", value: "hello" };
    expect(preserveJsonIdentity(previous, next)).toBe(previous);
  });

  it("preserveJsonIdentity returns the new reference when content actually differs", () => {
    const previous = { type: "text", value: "hello" };
    const next = { type: "text", value: "goodbye" };
    expect(preserveJsonIdentity(previous, next)).toBe(next);
  });

  it("preserveJsonIdentity treats nested arrays/objects structurally, not just top-level fields", () => {
    const previous = {
      nodes: [
        { id: "a", x: 1 },
        { id: "b", x: 2 },
      ],
    };
    const next = {
      nodes: [
        { id: "a", x: 1 },
        { id: "b", x: 2 },
      ],
    };
    expect(preserveJsonIdentity(previous, next)).toBe(previous);
    const moved = {
      nodes: [
        { id: "a", x: 1 },
        { id: "b", x: 3 },
      ],
    };
    expect(preserveJsonIdentity(previous, moved)).toBe(moved);
  });

  it("preserveJsonIdentity treats undefined previous as always-changed", () => {
    const next = { type: "text", value: "hello" };
    expect(preserveJsonIdentity(undefined, next)).toBe(next);
  });

  it("mergeRecordPreservingIdentity reuses the whole previous record when every key is unchanged", () => {
    const prev = { overview: { type: "text", value: "a" }, detail: { type: "text", value: "b" } };
    const merged = mergeRecordPreservingIdentity(prev, [
      ["overview", { type: "text", value: "a" }],
      ["detail", { type: "text", value: "b" }],
    ]);
    expect(merged).toBe(prev);
  });

  it("mergeRecordPreservingIdentity reuses per-key references, replacing only the changed key", () => {
    const prev = { overview: { type: "text", value: "a" }, detail: { type: "text", value: "b" } };
    const merged = mergeRecordPreservingIdentity(prev, [
      ["overview", { type: "text", value: "a" }],
      ["detail", { type: "text", value: "changed" }],
    ]);
    expect(merged).not.toBe(prev);
    expect(merged.overview).toBe(prev.overview);
    expect(merged.detail).not.toBe(prev.detail);
  });

  it("mergeRecordPreservingIdentity treats a key being added or removed as a change", () => {
    const prev = { overview: { type: "text", value: "a" } };
    const withNewKey = mergeRecordPreservingIdentity(prev, [
      ["overview", { type: "text", value: "a" }],
      ["detail", { type: "text", value: "b" }],
    ]);
    expect(withNewKey).not.toBe(prev);
    expect(withNewKey.overview).toBe(prev.overview);
  });
});

// 🐢️ Puzzle 2D performance round 3: the batched, hash-conditional `refresh-ui` protocol that replaces
// ~12 sequential per-section WASM calls with one round trip. `buildUiRefreshRequest` restricts what's
// asked for by scope and attaches known hashes; `applyUiRefreshResponseToCache` writes back only the
// sections the plugin actually says changed.
describe("batched ui refresh request/response (puzzle 2d perf round 3)", () => {
  const windowKinds = [
    { id: "overview", bodyKey: "puzzle2d.play.overview" },
    { id: "detail", bodyKey: "puzzle2d.play.detail" },
  ];
  const panelTabLeaves = [{ kind: { kind: "app" as const, id: "framework.panel.artifact" }, bodyKey: "puzzle2d.play.layers" }];

  it("buildActiveUtilityByWindowId preserves explicit clears for batched refresh", () => {
    expect(buildActiveUtilityByWindowId({ top: "transform", perspective: null, brush: "brush" })).toEqual({ top: "transform", perspective: null, brush: "brush" });
  });

  
  it("buildActiveUtilityByWindowId retains an explicit utility clear beside active siblings", () => {
    const utilities={top:"transform",perspective:null,brush:"brush"};
    expect(buildActiveUtilityByWindowId(utilities)).toEqual({top:"transform",perspective:null,brush:"brush"});
    expect(utilities).toEqual({top:"transform",perspective:null,brush:"brush"});
  });
it("buildUiRefreshRequest forwards per-window utility map on viewState without a focused-window singular leak", () => {
    const viewState = { activeUtilityByWindowId: { top: "transform", perspective: "brush" }, activeUtilityId: undefined };
    const request = buildUiRefreshRequest({ kind: "full" }, windowKinds, panelTabLeaves, viewState, new Map());
    expect(request?.viewState.activeUtilityByWindowId).toEqual({ top: "transform", perspective: "brush" });
    expect(request?.viewState.activeUtilityId).toBeUndefined();
  });

  it("buildActiveUtilityByWindowId makes a just-activated transform visible to refresh before the next React render", () => {
    const map: Record<string, string | null> = { "puzzle3d-main-top": null };
    map["puzzle3d-main-top"] = "transform";
    const activeUtilityByWindowId = buildActiveUtilityByWindowId(map);
    expect(activeUtilityByWindowId).toEqual({ "puzzle3d-main-top": "transform" });
    const request = buildUiRefreshRequest(
      { kind: "full" },
      [
        { id: "puzzle3d-main-top", bodyKey: "puzzle3d.play.composite" },
        { id: "puzzle3d-main-perspective", bodyKey: "puzzle3d.play.composite" },
      ],
      [],
      { activeUtilityByWindowId, activeUtilityId: undefined },
      new Map(),
    );
    expect(request?.viewState.activeUtilityByWindowId).toEqual({ "puzzle3d-main-top": "transform" });
    expect(request?.viewState.activeUtilityId).toBeUndefined();
  });

  it("buildUiRefreshRequest for a full scope requests every window/panel/engagements/measures/labels section (utility bars are now registry-derived, not a plugin section)", () => {
    const request = buildUiRefreshRequest({ kind: "full" }, windowKinds, panelTabLeaves, {}, new Map());
    expect(request?.windows?.map((w) => w.key)).toEqual(["overview", "detail"]);
    expect(request?.panels?.map((p) => p.key)).toEqual(["framework.panel.artifact"]);
    expect(request?.engagements).toBeDefined();
    expect(request?.measures).toBeDefined();
    expect(request?.labels).toBeDefined();
  });

  const generateRoster = [
    { id: "generation3d-generations", windowKindId: "generation3d-generations" },
    { id: "generation3d-generate-form", windowKindId: "generation3d-generate-form" },
    { id: "generation3d-generate-preview", windowKindId: "generation3d-generate-preview" },
  ];

  it("panelViewContext unbinds a panel from every window-scoped field", () => {
    const projected = panelViewContext({
      activeModeId: "generate",
      windowId: "generation3d-generations",
      activeWindowKindId: "generation3d-generations",
      activeUtilityId: "move",
      focusedWindowId: "generation3d-generations",
      windowInstances: generateRoster,
    });
    expect(projected.windowId).toBeUndefined();
    expect(projected.activeWindowKindId).toBeUndefined();
    expect(projected.activeUtilityId).toBeUndefined();
    expect(projected.activeModeId).toBe("generate");
  });

  it("panelViewContext keeps a focused pane the roster carries, so per-window panel controls still address it", () => {
    const projected = panelViewContext({ activeModeId: "generate", focusedWindowId: "generation3d-generate-form", windowInstances: generateRoster });
    expect(projected.focusedWindowId).toBe("generation3d-generate-form");
  });

  it("panelViewContext drops a focused pane the active mode's roster does not carry, so a mode-switch race cannot unpublish every app panel", () => {
    const projected = panelViewContext({ activeModeId: "generate", focusedWindowId: "procedural-main", windowInstances: generateRoster });
    expect(projected.focusedWindowId).toBeUndefined();
    expect(projected.windowInstances).toEqual(generateRoster);
  });

  it("panelViewContext drops a focused pane when the host carries no roster at all", () => {
    expect(panelViewContext({ focusedWindowId: "procedural-main" }).focusedWindowId).toBeUndefined();
  });

  it("buildUiRefreshRequest for none returns null", () => {
    expect(buildUiRefreshRequest({ kind: "none" }, windowKinds, panelTabLeaves, {}, new Map())).toBeNull();
  });

  it("buildUiRefreshRequest for a partial scope requests only the listed window/panel bodies and flags", () => {
    const scope = { kind: "partial" as const, windowBodies: ["puzzle2d.play.overview"], panelBodies: [], engagements: true };
    const request = buildUiRefreshRequest(scope, windowKinds, panelTabLeaves, {}, new Map());
    expect(request?.windows?.map((w) => w.key)).toEqual(["overview"]);
    expect(request?.panels).toEqual([]);
    expect(request?.engagements).toBeDefined();
    expect(request?.measures).toBeUndefined();
    expect(request?.labels).toBeUndefined();
  });

  it("buildUiRefreshRequest returns null for a partial scope that matches nothing in this app", () => {
    const scope = { kind: "partial" as const, windowBodies: ["some-other-app.body"] };
    expect(buildUiRefreshRequest(scope, windowKinds, panelTabLeaves, {}, new Map())).toBeNull();
  });

  it("buildUiRefreshRequest attaches the cached hash for a section that was already fetched once", () => {
    const cache: UiRefreshCache = new Map([["window:overview", { hash: "abc123", value: { type: "text", value: "x" } }]]);
    const request = buildUiRefreshRequest({ kind: "full" }, windowKinds, panelTabLeaves, {}, cache);
    expect(request?.windows?.find((w) => w.key === "overview")?.hash).toBe("abc123");
    expect(request?.windows?.find((w) => w.key === "detail")?.hash).toBeUndefined();
  });

  it("buildUiRefreshRequest gives two instances of the same window kind distinct keys and independent cached hashes", () => {
    const splitInstances = [
      { id: "puzzle3d-main", bodyKey: "puzzle3d.play.main" },
      { id: "puzzle3d-main-2", bodyKey: "puzzle3d.play.main" },
    ];
    const cache: UiRefreshCache = new Map([["window:puzzle3d-main", { hash: "base-hash", value: { type: "text", value: "base" } }]]);
    const request = buildUiRefreshRequest({ kind: "full" }, splitInstances, [], {}, cache);
    expect(request?.windows?.map((w) => w.key)).toEqual(["puzzle3d-main", "puzzle3d-main-2"]);
    expect(request?.windows?.find((w) => w.key === "puzzle3d-main")?.hash).toBe("base-hash");
    expect(request?.windows?.find((w) => w.key === "puzzle3d-main-2")?.hash).toBeUndefined();
  });

  it("applyUiRefreshResponseToCache writes changed sections and ignores hash-only (unchanged) ones", () => {
    const cache: UiRefreshCache = new Map([["window:detail", { hash: "old-hash", value: { type: "text", value: "stale-should-not-be-touched" } }]]);
    applyUiRefreshResponseToCache(cache, {
      windows: [
        { key: "overview", hash: "new-hash", value: { type: "text", value: "fresh" } },
        { key: "detail", hash: "old-hash" },
      ],
      engagements: { key: "engagements", hash: "eng-hash", value: { overview: {} } },
    });
    expect(cache.get("window:overview")).toEqual({ hash: "new-hash", value: { type: "text", value: "fresh" } });
    expect(cache.get("window:detail")).toEqual({ hash: "old-hash", value: { type: "text", value: "stale-should-not-be-touched" } });
    expect(cache.get("engagements")).toEqual({ hash: "eng-hash", value: { overview: {} } });
  });

  it("buildUiRefreshRequest for a full scope also requests the mode-level tools section (keyed by tool id, not a window)", () => {
    const request = buildUiRefreshRequest({ kind: "full" }, windowKinds, panelTabLeaves, {}, new Map());
    expect(request?.tools).toBeDefined();
  });

  it("buildUiRefreshRequest for a partial scope requests tools only when the scope's `tools` flag is set", () => {
    const withTools = buildUiRefreshRequest({ kind: "partial" as const, tools: true }, windowKinds, panelTabLeaves, {}, new Map());
    expect(withTools?.tools).toBeDefined();
    const withoutTools = buildUiRefreshRequest({ kind: "partial" as const, engagements: true }, windowKinds, panelTabLeaves, {}, new Map());
    expect(withoutTools?.tools).toBeUndefined();
  });

  it("applyUiRefreshResponseToCache caches the tools section same as measures/engagements/labels", () => {
    const cache: UiRefreshCache = new Map();
    applyUiRefreshResponseToCache(cache, { tools: { key: "tools", hash: "tools-hash", value: { fill: [] } } });
    expect(cache.get("tools")).toEqual({ hash: "tools-hash", value: { fill: [] } });
  });

  it("buildUiRefreshRequest asks for the app catalogue on a full scope only, carrying its cached hash", () => {
    const full = buildUiRefreshRequest({ kind: "full" }, windowKinds, panelTabLeaves, {}, new Map());
    expect(full?.catalogue).toBeDefined();
    expect(full?.catalogue?.hash).toBeUndefined();
    const cache: UiRefreshCache = new Map([["catalogue", { hash: "cat-hash", value: { operators: [] } }]]);
    expect(buildUiRefreshRequest({ kind: "full" }, windowKinds, panelTabLeaves, {}, cache)?.catalogue?.hash).toBe("cat-hash");
    const partial = buildUiRefreshRequest({ kind: "partial" as const, engagements: true }, windowKinds, panelTabLeaves, {}, new Map());
    expect(partial?.catalogue).toBeUndefined();
  });

  it("applyUiRefreshResponseToCache caches the app catalogue and leaves it untouched on an unchanged hash", () => {
    const cache: UiRefreshCache = new Map();
    const value = { operators: [{ id: "math.add", extension: "math", name: "Add", abbreviation: "Add", icon: "emoji:+", summary: "Adds", inputs: [], outputs: [] }], sections: [{ id: "math", title: "Math", items: [] }] };
    applyUiRefreshResponseToCache(cache, { catalogue: { key: "catalogue", hash: "cat-hash", value } });
    expect(cache.get("catalogue")).toEqual({ hash: "cat-hash", value });
    applyUiRefreshResponseToCache(cache, { catalogue: { key: "catalogue", hash: "cat-hash" } });
    expect(cache.get("catalogue")).toEqual({ hash: "cat-hash", value });
  });
});
