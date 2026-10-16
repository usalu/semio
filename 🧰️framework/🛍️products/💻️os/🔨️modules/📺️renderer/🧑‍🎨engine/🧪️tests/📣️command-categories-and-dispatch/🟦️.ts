/** 📣 Command category trees and host effect dispatch. */
import { treeDataActivation } from "@semio-tech/ui-react";
import { createContinuationScheduler } from "../../../../../../../🔨️modules/⏳️async/🪃️continuation/🟦️.ts";
import { createVirtualContinuationHost } from "../../../../../../../🔨️modules/⏳️async/🪃️continuation/🧪️tests/⚖️oracle/🟦️.ts";
import { afterEach, describe, expect, it, vi } from "vitest";
import { actionSemanticsForKind } from "@semio-tech/framework";
import { buildCommandCategoryTree, buildCommandCategoryTabs, encodeEffectActionInvocation, encodeEffectCommandInvocation, type ResolvedCommand, type ResolvedActionArgDef, dispatchOpenedFiles, IMPORT_CHUNK_BYTES, importPayloadChunks, scheduleDispatchAction, sampleMediaFrameTimestampsMs, runTier2VideoFrames, requestMediaFramesSourceV1, runMediaFramesV1 } from "../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import "../🧰️engine-contract-helpers/🟦️.ts";

describe("buildCommandCategoryTree / buildCommandCategoryTabs (command palette as a real bottom-middle Panel)", () => {
  const definition = (id: string, label: string, category: string, args: ResolvedActionArgDef[] = []): ResolvedCommand["definition"] => ({
    id,
    label,
    category,
    args,
    iconId: "wrench",
    semantics: actionSemanticsForKind("shell"),
    kind: "shell",
    keybindings: [],
    inPalette: true,
  });
  const zeroArgCommand: ResolvedCommand = { definition: definition("os.resetDock", "Reset Dock", "layout"), address: { owner: "os", commandId: "os.resetDock" } };
  const argCommand: ResolvedCommand = {
    definition: definition("os.setThemeId", "Set Theme", "appearance", [{ id: "themeId", label: "Theme", schema: { kind: "string", options: [] }, required: true }]),
    address: { owner: "os", commandId: "os.setThemeId" },
  };
  const secondArgCommand: ResolvedCommand = {
    definition: definition("os.setAppearance", "Set Appearance", "appearance", [{ id: "appearance", label: "Appearance", schema: { kind: "string", options: [] }, required: true }]),
    address: { owner: "os", commandId: "os.setAppearance" },
  };
  const singletonArgCommand: ResolvedCommand = {
    definition: definition("os.setDriver", "Set Driver", "general", [{ id: "driver", label: "Driver", schema: { kind: "string", options: [] }, required: true }]),
    address: { owner: "os", commandId: "os.setDriver" },
  };

  it("a zero-arg command row fires onExecute directly on click; only one command-list section is present when nothing is expanded", () => {
    const onExecute = vi.fn();
    const tree = buildCommandCategoryTree([zeroArgCommand], null, {}, onExecute, vi.fn(), vi.fn(), vi.fn());
    expect(tree.sections).toHaveLength(1);
    const row = tree.sections[0]!.items!.find((item) => item.id === "command.os.os.resetDock")!;
    expect(row.label).toBe("Reset Dock");
    row.onClick?.(...treeDataActivation());
    expect(onExecute).toHaveBeenCalledWith(zeroArgCommand);
  });

  it("auto-expands a singleton arg-carrying category into a flat form with section actions and no disclosure list", () => {
    const tree = buildCommandCategoryTree([singletonArgCommand], null, {}, vi.fn(), vi.fn(), vi.fn(), vi.fn());
    expect(tree.sections).toHaveLength(1);
    expect(tree.sections[0]!.id).toBe("command.category.general.form");
    expect(tree.sections[0]!.items?.map((item) => item.id)).toEqual(["command.os.os.setDriver.arg.driver"]);
    expect(tree.sections[0]!.actions?.map((action) => action.id)).toEqual(["command-os.os.setDriver-execute", "command-os.os.setDriver-reset"]);
  });

  it("an arg-carrying command row toggles expansion instead of executing, and a synthetic arg-form section only appears while expanded", () => {
    const onToggleExpanded = vi.fn();
    const collapsedTree = buildCommandCategoryTree([argCommand, secondArgCommand], null, {}, vi.fn(), onToggleExpanded, vi.fn(), vi.fn());
    expect(collapsedTree.sections).toHaveLength(1);
    const collapsedRow = collapsedTree.sections[0]!.items!.find((item) => item.id === "command.os.os.setThemeId")!;
    expect(collapsedRow.label).toBe("Set Theme…");
    collapsedRow.onClick?.(...treeDataActivation());
    expect(onToggleExpanded).toHaveBeenCalledWith("os:os.setThemeId");

    const expandedTree = buildCommandCategoryTree([argCommand, secondArgCommand], "os:os.setThemeId", {}, vi.fn(), vi.fn(), vi.fn(), vi.fn());
    expect(expandedTree.sections).toHaveLength(2);
    const formItems = expandedTree.sections[0]!.items!;
    expect(formItems.find((item) => item.id === "command.os.os.setThemeId.arg.themeId")?.label).toBe("Theme");
    expect(expandedTree.sections[0]!.actions?.map((action) => action.id)).toEqual(["command-os.os.setThemeId-execute", "command-os.os.setThemeId-reset"]);
    expect(expandedTree.sections[1]!.items?.map((item) => item.id)).toEqual(["command.os.os.setAppearance"]);
  });

  it("Execute is disabled until the required arg is staged, and calling it passes the effective (staged) args; Reset dispatches onResetArgs", () => {
    const onExecute = vi.fn();
    const onStageArg = vi.fn();
    const onResetArgs = vi.fn();

    const missingTree = buildCommandCategoryTree([argCommand, secondArgCommand], "os:os.setThemeId", {}, onExecute, vi.fn(), onStageArg, onResetArgs);
    const missingExecute = missingTree.sections[0]!.actions!.find((action) => action.id === "command-os.os.setThemeId-execute")!;
    expect(missingExecute.disabled).toBe(true);

    const stagedTree = buildCommandCategoryTree([argCommand, secondArgCommand], "os:os.setThemeId", { "os:os.setThemeId": { themeId: "semio" } }, onExecute, vi.fn(), onStageArg, onResetArgs);
    const stagedExecute = stagedTree.sections[0]!.actions!.find((action) => action.id === "command-os.os.setThemeId-execute")!;
    const stagedReset = stagedTree.sections[0]!.actions!.find((action) => action.id === "command-os.os.setThemeId-reset")!;
    expect(stagedExecute.disabled).toBe(false);
    if (stagedExecute.kind === "checkbox" || stagedReset.kind === "checkbox") throw new Error("Command fixture requires execute and reset buttons");
    stagedExecute.onClick();
    expect(onExecute).toHaveBeenCalledWith(argCommand, { themeId: "semio" });
    stagedReset.onClick();
    expect(onResetArgs).toHaveBeenCalledWith("os:os.setThemeId");
  });

  it("buildCommandCategoryTabs builds one namespaced PanelTabLeaf per category, whose lazily-resolved tree only contains that category's commands", () => {
    const categories = [
      { id: "layout", label: "Layout" },
      { id: "appearance", label: "Appearance" },
    ];
    const expandedRef = { current: null as string | null };
    const stagedRef = { current: {} as Readonly<Record<string, Readonly<Record<string, unknown>>>> };
    const onCommand = vi.fn();
    const dispatch = vi.fn();
    const tabs = buildCommandCategoryTabs([zeroArgCommand, argCommand], categories, expandedRef, stagedRef, onCommand, dispatch);
    expect(tabs.map((tab) => tab.id)).toEqual(["command.category.layout", "command.category.appearance"]);
    expect(tabs.every((tab) => tab.kind === "leaf")).toBe(true);

    const layoutLeaf = tabs[0]!;
    expect(layoutLeaf.kind).toBe("leaf");
    const resolved = layoutLeaf.kind === "leaf" ? (layoutLeaf.trees[0]!.tree as { resolveTree: () => { sections: { items?: { id: string }[] }[] } }).resolveTree() : { sections: [] };
    expect(resolved.sections[0]!.items?.map((item) => item.id)).toEqual(["command.os.os.resetDock"]);

    // Executing routes through the injected onCommand with the command's own source.
    const executeRow = resolved.sections[0]!.items!.find((item: { id: string }) => item.id === "command.os.os.resetDock") as unknown as { onClick: (event: never, context: never) => void };
    executeRow.onClick({} as never, {} as never);
    expect(onCommand).toHaveBeenCalledWith({ owner: "os", commandId: "os.resetDock" }, undefined);
  });
});

describe("host effect dispatch (D2 DispatchAction, D3 RequestFileOpen.multiple, D5 RequestMediaFrames)", () => {
  afterEach(() => {
    vi.useRealTimers();
  });

  it("encodes recursive host-effect actions as fully scoped JSON at the runtime boundary", () => {
    const baseSession = {
      pluginId: "demonstrator",
      app: { id: "cad-play", defaultModeId: "edit", modes: [{ id: "edit" }], windowKinds: [{ id: "shape" }] },
      viewState: { activeModeId: "edit", activeWindowKindId: "shape", windowId: "shape-2" },
    } as unknown as Parameters<typeof encodeEffectActionInvocation>[0];
    expect(JSON.parse(encodeEffectActionInvocation(baseSession, "dispatchNext", { jobId: "job-1" }))).toEqual({
      address: {
        pluginId: "demonstrator",
        appId: "cad-play",
        modeId: "edit",
        windowKindId: "shape",
        windowInstanceId: "shape-2",
        actionId: "dispatchNext",
      },
      arguments: { jobId: "job-1", windowId: "shape-2" },
    });
    expect(JSON.parse(encodeEffectCommandInvocation(baseSession, "flowEvalTick"))).toEqual({
      address: { owner: { app: { pluginId: "demonstrator", appId: "cad-play" } }, commandId: "flowEvalTick" },
      arguments: {},
    });
  });

  it("scheduleDispatchAction (D2): a positive delayMs is a real deadline, honoured to the millisecond", () => {
    const host = createVirtualContinuationHost();
    const dispatchOne = vi.fn().mockResolvedValue(undefined);
    scheduleDispatchAction("advanceReconstruction", { jobId: "job-1" }, 250, dispatchOne, createContinuationScheduler(host.ports));
    expect(dispatchOne).not.toHaveBeenCalled();
    host.drain(249);
    expect(dispatchOne).not.toHaveBeenCalled();
    host.drain(250);
    expect(dispatchOne).toHaveBeenCalledExactlyOnceWith("advanceReconstruction", { jobId: "job-1" });
  });

  it("scheduleDispatchAction (D2): delayMs 0 defers to a continuation and never arms a host timer", () => {
    const host = createVirtualContinuationHost();
    const armed: number[] = [];
    const dispatchOne = vi.fn().mockResolvedValue(undefined);
    scheduleDispatchAction(
      "tick",
      undefined,
      0,
      dispatchOne,
      createContinuationScheduler({
        ...host.ports,
        setTimer: (run, delayMs) => {
          armed.push(delayMs);
          return host.ports.setTimer(run, delayMs);
        },
      }),
    );
    expect(dispatchOne).not.toHaveBeenCalled();
    host.drain();
    expect(dispatchOne).toHaveBeenCalledExactlyOnceWith("tick", undefined);
    expect(armed).toEqual([]);
    expect(host.nowMs()).toBe(0);
  });

  // 🪃️ THE regression law for ticket 26/09/09/PROCEDURAL-3D-END-TO-END's ~24 s extension hop. The guest
  // re-arms with `Effect::DispatchAction { delay_ms: 0 }` from inside the previous dispatch's own
  // callback; on a nested `setTimeout` chain a hidden/unfocused/headless Chrome renderer clamps that to
  // ~1 tick/s, which is what turned a 0.58 s native chain into 112 s in the browser. This simulates the
  // clamp by making EVERY `globalThis.setTimeout` a second late and then asserts the ten-hop chain still
  // finishes in well under one clamped tick — impossible unless the re-arm avoids timers entirely.
  it("scheduleDispatchAction (D2): a ten-hop re-arm chain beats a throttled setTimeout by two orders of magnitude", async () => {
    vi.useRealTimers();
    const realSetTimeout = globalThis.setTimeout;
    const THROTTLE_MS = 1_000;
    const HOPS = 10;
    let throttledCalls = 0;
    globalThis.setTimeout = ((handler: TimerHandler, timeout?: number, ...args: unknown[]) => {
      throttledCalls += 1;
      return realSetTimeout(handler as () => void, (timeout ?? 0) + THROTTLE_MS, ...(args as []));
    }) as unknown as typeof globalThis.setTimeout;
    try {
      const patchProbe = throttledCalls;
      globalThis.setTimeout(() => {}, 0);
      expect(throttledCalls).toBe(patchProbe + 1);
      const before = throttledCalls;
      const startedAtMs = Date.now();
      const hops: string[] = [];
      await new Promise<void>((resolve) => {
        const dispatchOne = async (action: string): Promise<void> => {
          hops.push(action);
          if (hops.length >= HOPS) return resolve();
          scheduleDispatchAction(`hop#${hops.length + 1}`, undefined, 0, dispatchOne);
        };
        scheduleDispatchAction("hop#1", undefined, 0, dispatchOne);
      });
      const elapsedMs = Date.now() - startedAtMs;
      expect(hops).toEqual(Array.from({ length: HOPS }, (_, index) => `hop#${index + 1}`));
      expect(throttledCalls).toBe(before);
      expect(elapsedMs).toBeLessThan(THROTTLE_MS / 4);
    } finally {
      globalThis.setTimeout = realSetTimeout;
    }
  });

  it("dispatchOpenedFiles (D3): single-file (multiple=false) makes exactly one call with {payload, name} plus the one-chunk envelope and no index/total", async () => {
    const dispatchOne = vi.fn().mockResolvedValue(undefined);
    await dispatchOpenedFiles([{ contents: "abc", name: "a.png" }], "importFramePayload", false, dispatchOne);
    expect(dispatchOne).toHaveBeenCalledExactlyOnceWith("importFramePayload", { payload: "abc", name: "a.png", chunk: 0, chunkCount: 1 });
  });

  it("dispatchOpenedFiles (B59): a file above one chunk is dispatched in order as chunks that each stay inside the guest contiguous ceiling", async () => {
    const calls: { readonly payload: string; readonly chunk: number; readonly chunkCount: number; readonly name: string }[] = [];
    const dispatchOne = vi.fn().mockImplementation(async (_action: string, args: unknown) => {
      calls.push(args as (typeof calls)[number]);
    });
    // 📏️ The product's own payload class: the Nakagin Capsule Tower export is 145 714 B (wave B53/B57).
    const contents = `{"schema":"puzzle.3d.fixture","objects":[${"x".repeat(145_000)}]}`;
    await dispatchOpenedFiles([{ contents, name: "nakagin-capsule-tower.json" }], "importSnapshot", false, dispatchOne);
    expect(calls.length).toBe(Math.ceil(contents.length / IMPORT_CHUNK_BYTES));
    expect(calls.map((call) => call.chunk)).toEqual(calls.map((_, index) => index));
    expect(new Set(calls.map((call) => call.chunkCount))).toEqual(new Set([calls.length]));
    expect(new Set(calls.map((call) => call.name))).toEqual(new Set(["nakagin-capsule-tower.json"]));
    for (const call of calls) expect(new TextEncoder().encode(call.payload).length).toBeLessThanOrEqual(IMPORT_CHUNK_BYTES);
    expect(calls.map((call) => call.payload).join("")).toBe(contents);
  });

  it("importPayloadChunks (B59): slices by UTF-8 extent so a non-ASCII label can never overrun the chunk cap, and never splits a code point", () => {
    const label = "Distinct Capsule J · cs_sl1 ōō";
    const contents = label.repeat(4_000);
    const chunks = importPayloadChunks(contents);
    expect(chunks.length).toBeGreaterThan(1);
    for (const chunk of chunks) expect(new TextEncoder().encode(chunk.payload).length).toBeLessThanOrEqual(IMPORT_CHUNK_BYTES);
    expect(chunks.map((chunk) => chunk.payload).join("")).toBe(contents);
    expect(importPayloadChunks("")).toEqual([{ payload: "", chunk: 0, chunkCount: 1 }]);
  });

  it("dispatchOpenedFiles (D3): multiple=true dispatches once per file, in order, each extended with {index, total}", async () => {
    const calls: unknown[][] = [];
    const dispatchOne = vi.fn().mockImplementation(async (action: string, args: unknown) => {
      calls.push([action, args]);
    });
    const opened = [
      { contents: "a", name: "a.png" },
      { contents: "b", name: "b.png" },
      { contents: "c", name: "c.png" },
    ];
    await dispatchOpenedFiles(opened, "importFramePayload", true, dispatchOne);
    expect(dispatchOne).toHaveBeenCalledTimes(3);
    expect(calls).toEqual([
      ["importFramePayload", { payload: "a", name: "a.png", chunk: 0, chunkCount: 1, index: 0, total: 3 }],
      ["importFramePayload", { payload: "b", name: "b.png", chunk: 0, chunkCount: 1, index: 1, total: 3 }],
      ["importFramePayload", { payload: "c", name: "c.png", chunk: 0, chunkCount: 1, index: 2, total: 3 }],
    ]);
  });

  it("sampleMediaFrameTimestampsMs (D5): steps by sampleStride/fpsHint seconds, capped at maxFrames", () => {
    // Expected timestamps mirror the implementation's own `k * stepMs` computation exactly (bit-for-bit)
    // rather than independently-derived literals — plain float division/multiplication isn't perfectly
    // associative, so e.g. `k * (5 / 30 * 1000)` and `(k * 5000) / 30` can differ by a ULP.
    const stepAt5_30 = (5 / 30) * 1000;
    expect(sampleMediaFrameTimestampsMs(1000, 5, 0, 30)).toEqual([0, 1, 2, 3, 4, 5].map((k) => k * stepAt5_30));
    expect(sampleMediaFrameTimestampsMs(1000, 5, 2, 30)).toEqual([0, stepAt5_30]);
    // sampleStride 0 floors to 1, fpsHint 0 falls back to 30 — never divides by zero.
    const stepAt1_30 = (1 / 30) * 1000;
    expect(sampleMediaFrameTimestampsMs(100, 0, 0, 0)).toEqual([0, 1, 2].map((k) => k * stepAt1_30));
    expect(sampleMediaFrameTimestampsMs(0, 5, 10, 30)).toEqual([]);
  });

  //#region 🔌️jsdom media mocks
  /** 🎞️ jsdom has no real media decoder — `<video>`'s `duration`/`videoWidth`/`videoHeight` are
   * read-only getters that never change from a `src` assignment, and `currentTime` is a no-operation setter
   * that never fires `seeked`. This stubs both to the minimum needed for `runTier2VideoFrames`'s
   * seek-and-capture loop: `currentTime` synchronously (via microtask) fires `seeked`, mirroring how a
   * real browser resolves a seek asynchronously without needing fake timers in these tests. */
  function mockVideoElement(durationMs: number, width: number, height: number): HTMLVideoElement {
    const video = document.createElement("video");
    Object.defineProperty(video, "duration", { value: durationMs / 1000, configurable: true });
    Object.defineProperty(video, "videoWidth", { value: width, configurable: true });
    Object.defineProperty(video, "videoHeight", { value: height, configurable: true });
    Object.defineProperty(video, "readyState", { value: 1, configurable: true });
    Object.defineProperty(video, "currentTime", {
      configurable: true,
      get() {
        return 0;
      },
      set() {
        queueMicrotask(() => video.dispatchEvent(new Event("seeked")));
      },
    });
    return video;
  }

  function mockCanvasCapture(): void {
    HTMLCanvasElement.prototype.getContext = vi.fn().mockReturnValue({ drawImage: vi.fn() }) as unknown as typeof HTMLCanvasElement.prototype.getContext;
    HTMLCanvasElement.prototype.toDataURL = vi.fn().mockReturnValue("data:image/jpeg;base64,ZmFrZQ==");
  }
  //#endregion 🔌️jsdom media mocks

  it("runTier2VideoFrames (D5): dispatches frameAction once per sampled timestamp, in order, then doneAction exactly once", async () => {
    mockCanvasCapture();
    const video = mockVideoElement(200, 64, 48);
    const calls: { action: string; args: Record<string, unknown> }[] = [];
    const dispatchOne = vi.fn().mockImplementation(async (action: string, args: Record<string, unknown>) => {
      calls.push({ action, args });
    });
    await runTier2VideoFrames(video, { frameAction: "frame", doneAction: "done", fallbackAction: "fallback", sampleStride: 5, maxFrames: 0, maxLongEdgePx: 0, fpsHint: 30, args: { streamId: "s1" } }, "clip.mp4", dispatchOne);
    const frameCalls = calls.filter((call) => call.action === "frame");
    const doneCalls = calls.filter((call) => call.action === "done");
    expect(frameCalls.length).toBeGreaterThan(0);
    expect(doneCalls).toHaveLength(1);
    // frame/done ordering: every frame dispatch precedes the single done dispatch.
    expect(calls.at(-1)!.action).toBe("done");
    expect(frameCalls.map((call) => call.args.index)).toEqual(frameCalls.map((_, index) => index));
    expect(frameCalls[0]!.args).toMatchObject({ payload: "data:image/jpeg;base64,ZmFrZQ==", name: "clip.mp4", streamId: "s1" });
    expect(doneCalls[0]!.args).toMatchObject({ name: "clip.mp4", frameCount: frameCalls.length, sampledCount: frameCalls.length, streamId: "s1" });
  });

  it("runMediaFramesV1 (D5): Tier 2 failure (video element throws mid-seek) ⇒ dispatches fallbackAction exactly once with raw bytes as a data URL, no frame/done calls", async () => {
    const dispatchOne = vi.fn().mockResolvedValue(undefined);
    const payload = "data:video/mp4;base64," + btoa("not a real mp4 but bytes exist");
    const throwingVideo = mockVideoElement(1000, 16, 16);
    Object.defineProperty(throwingVideo, "currentTime", {
      configurable: true,
      get() {
        return 0;
      },
      set() {
        throw new Error("decode failed");
      },
    });
    const source = await requestMediaFramesSourceV1("video/mp4", payload);
    expect(await runMediaFramesV1({ frameAction: "frame", doneAction: "done", fallbackAction: "fallback", sampleStride: 1, maxFrames: 2, maxLongEdgePx: 0, fpsHint: 30 }, source!, dispatchOne, { actions: [] }, new AbortController().signal, undefined, () => throwingVideo)).toBe("done");
    expect(dispatchOne).toHaveBeenCalledTimes(1);
    const [action, args] = dispatchOne.mock.calls[0]! as [string, Record<string, unknown>];
    expect(action).toBe("fallback");
    expect(args.name).toBe("video");
    expect(String(args.payload)).toMatch(/^data:video\/mp4;base64,/);
  });

  it("runMediaFramesV1 (D5): payload bytes in hand ⇒ Tier 2 seek-capture runs, reporting every frame, ending in doneAction (no picker needed)", async () => {
    mockCanvasCapture();
    const dispatchOne = vi.fn().mockResolvedValue(undefined);
    const payload = "data:video/mp4;base64," + btoa("not a real mp4 but bytes exist");
    const progress: [number, number][] = [];
    const source = await requestMediaFramesSourceV1("video/mp4", payload);
    expect(source?.name).toBe("video");
    expect(await runMediaFramesV1({ frameAction: "frame", doneAction: "done", fallbackAction: "fallback", sampleStride: 1, maxFrames: 2, maxLongEdgePx: 0, fpsHint: 30 }, source!, dispatchOne, { actions: [] }, new AbortController().signal, (completed, total) => progress.push([completed, total]), () => mockVideoElement(1000, 16, 16))).toBe("done");
    const actions = dispatchOne.mock.calls.map((call) => call[0] as string);
    expect(actions).toEqual(["frame", "frame", "done"]);
    expect(progress).toEqual([[1, 2], [2, 2]]);
  });

  it("runMediaFramesV1 (D5): a host cancel stops before the next frame — no done, no fallback; `importAbort` only when a frame reached a declaring guest", async () => {
    mockCanvasCapture();
    const payload = "data:video/mp4;base64," + btoa("not a real mp4 but bytes exist");
    const args = { frameAction: "frame", doneAction: "done", fallbackAction: "fallback", sampleStride: 1, maxFrames: 4, maxLongEdgePx: 0, fpsHint: 30 } as const;
    const run = async (declares: boolean, cancelAfter: number | "before") => {
      const controller = new AbortController();
      if (cancelAfter === "before") controller.abort();
      const dispatched: string[] = [];
      const dispatchOne = vi.fn().mockImplementation(async (action: string) => {
        dispatched.push(action);
        if (dispatched.filter((entry) => entry === "frame").length === cancelAfter) controller.abort();
      });
      const outcome = await runMediaFramesV1(args, (await requestMediaFramesSourceV1("video/mp4", payload))!, dispatchOne, { actions: declares ? [{ id: "importAbort" }] : [] } as unknown as Parameters<typeof runMediaFramesV1>[3], controller.signal, undefined, () => mockVideoElement(1000, 16, 16));
      return { outcome, dispatched };
    };
    expect(await run(true, 1)).toEqual({ outcome: "cancelled", dispatched: ["frame", "importAbort"] });
    expect(await run(true, 2)).toEqual({ outcome: "cancelled", dispatched: ["frame", "frame", "importAbort"] });
    expect(await run(false, 1)).toEqual({ outcome: "cancelled", dispatched: ["frame"] });
    expect(await run(true, "before")).toEqual({ outcome: "cancelled", dispatched: [] });
  });
});
