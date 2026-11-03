/** 🎞️ Per-frame state of one window must not re-render a sibling window: a 30 fps stream of node updates in window A commits A only (re-render storm class: per-frame state belongs in window transient, not in shared host state). */
import { Profiler, createElement } from "react";
import { cleanup, render, act } from "@semio-tech/ui-react/test";
import { afterEach, describe, expect, it } from "vitest";
import { InterpretedUiNode, UiDocumentStore } from "../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import { buildContractSnapshot, noopAction } from "../🧰️engine-contract-helpers/🟦️.ts";

const textSnapshot = (key: string, value: string, revision: number) => ({
  ...buildContractSnapshot({ key, component: { type: "text", value, emphasize: null, dataAttributes: null } as never }),
  revision,
});

const mountWindow = (store: UiDocumentStore, counts: Map<string, number>, id: string) => createElement(Profiler, { id, onRender: (profilerId: string) => { counts.set(profilerId, (counts.get(profilerId) ?? 0) + 1); } },
  createElement(InterpretedUiNode, { store, onAction: noopAction, onIntent: noopAction, requestContextMenu: undefined }));

afterEach(() => cleanup());

describe("window frame render isolation", () => {
  it("commits only the ticking window across a 30-frame stream", () => {
    const frames = new UiDocumentStore("frames");
    const quiet = new UiDocumentStore("quiet");
    frames.loadSnapshot(textSnapshot("frames", "tick 0", 0));
    quiet.loadSnapshot(textSnapshot("quiet", "static", 0));
    const counts = new Map<string, number>();
    const view = render(createElement("div", null, mountWindow(frames, counts, "frames"), mountWindow(quiet, counts, "quiet")));
    const mounted = { frames: counts.get("frames") ?? 0, quiet: counts.get("quiet") ?? 0 };
    expect(mounted.frames).toBeGreaterThan(0);
    expect(mounted.quiet).toBeGreaterThan(0);
    for (let frame = 1; frame <= 30; frame += 1) act(() => frames.loadSnapshot(textSnapshot("frames", `tick ${frame}`, frame)));
    expect(view.container.textContent).toContain("tick 30");
    expect(view.container.textContent).toContain("static");
    expect(counts.get("quiet")).toBe(mounted.quiet);
    expect(counts.get("frames")! - mounted.frames).toBeLessThanOrEqual(30);
    expect(counts.get("frames")! - mounted.frames).toBeGreaterThan(0);
  });

  it("an identical re-load of a window snapshot keeps every node reference and commits nothing", () => {
    const store = new UiDocumentStore("same");
    store.loadSnapshot(textSnapshot("same", "x", 0));
    const before = store.getState().nodes.get(0);
    const counts = new Map<string, number>();
    render(mountWindow(store, counts, "same"));
    const mounted = counts.get("same") ?? 0;
    for (let n = 0; n < 10; n += 1) act(() => store.loadSnapshot(textSnapshot("same", "x", 0)));
    expect(store.getState().nodes.get(0)).toBe(before);
    expect(counts.get("same")).toBe(mounted);
    act(() => store.loadSnapshot(textSnapshot("same", "y", 1)));
    expect(counts.get("same")).toBeGreaterThan(mounted);
  });
});
