/** ⏱️ A running clock costs no React render per frame: one subscription, visible figures only, direct DOM writes.
 * @see ./🥒️.feature — the scenario */
import { act, cleanup, render } from "@testing-library/react";
import { Profiler } from "react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { VIEWS, depict, evaluate, type Asset } from "@teaching/staffage";
import { StaffageClock } from "@teaching/staffage-react";
import type { ClockScheduler } from "@teaching/staffage-react";
import { DrawingQueue } from "../../🔨️modules/🧮️pipeline/🏁️drawing/queue.ts";
import { drawFigure, type DrawingReply, type DrawingRequest } from "../../🔨️modules/🧮️pipeline/🏁️drawing/🟦️.ts";
import { driveFigure } from "../../🔨️modules/⏱️clock/🏁️frames/🟦️.ts";
import { PlacedFigure } from "../../🔨️modules/📍️placed/🟦️.tsx";
import * as pipeline from "../../🔨️modules/🧮️pipeline/🟦️.ts";
import { RowPreview } from "../../🔨️modules/🖼️previews/🟦️.tsx";
import { translator } from "../../🔨️modules/🌐️i18n/🟦️.ts";
import { ArchiveProvider, useArchive } from "../../🔨️modules/🧭️context/🟦️.tsx";
import { ArchiveStore, memoryStorage } from "../../🔨️modules/💾️persistence/🟦️.ts";
import { CatalogStore } from "../../🔨️modules/📚️catalog/🟦️.ts";
import type { Row } from "../../🔨️modules/🔎️search/🟦️.ts";
import { walker } from "../🧰️support/🧰️.ts";

type Observer = { callback: IntersectionObserverCallback; elements: Set<Element> };
const observers: Observer[] = [];

function manual(): ClockScheduler & { tick: (nowMs: number) => void } {
  let next: ((nowMs: number) => void) | null = null;
  return { request: (tick) => ((next = tick), 1), cancel: () => void (next = null), tick: (nowMs) => { const run = next; next = null; run?.(nowMs); } };
}

afterEach(() => { cleanup(); observers.length = 0; vi.restoreAllMocks(); vi.unstubAllGlobals(); });

describe("⏱️ frame budget", () => {
  it("draws an elevation without projecting unused views and preserves its SVG", () => {
    const request = { view: "dark-line", viewpoint: "elevation", scale: { denominator: 100 }, language: "en" } as const;
    const options = pipeline.depictionOptions(request);
    const query = { kind: "clip", clip: walker.presentation.defaultClip, timeS: 1 / 12 } as const;
    const expected = pipeline.inlineSvg(depict(evaluate(walker, query, options.representation), options));
    const unusedReads = vi.fn(() => { throw new Error("unused front view was projected"); });
    const representation = walker.representations["100"];
    if (representation.status !== "authored" || representation.geometry.front === undefined) throw new Error("The reference walker needs an authored 1:100 front view.");
    const front = new Proxy(representation.geometry.front, {
      get(target, key, receiver) {
        if (key === "map") unusedReads();
        return Reflect.get(target, key, receiver);
      },
    });
    const asset: Asset = { ...walker, representations: {
      ...walker.representations,
      "100": { ...representation, geometry: { ...representation.geometry, front } },
    } };
    const scene = pipeline.figureFrame(asset, query, request).scene;
    expect(pipeline.inlineSvg(scene)).toBe(expected);
    expect(unusedReads).not.toHaveBeenCalled();
  });

  it("reuses evaluated drawings when a fractional frame recurs in a later loop", () => {
    const clock = new StaffageClock({ scheduler: manual(), fps: 12, playing: false });
    const scenes = vi.spyOn(pipeline, "sceneOf");
    render(<PlacedFigure asset={walker} selection={null} clock={clock} view="dark-line" viewpoint="elevation" scale={100} language="en" pxPerMm={1} xPx={50} groundPx={100} />);
    act(() => clock.step(1));
    const first = scenes.mock.results.at(-1)?.value;
    expect(first).toBeDefined();
    act(() => clock.step(12 * 50));
    expect(scenes.mock.results.at(-1)?.value).toBe(first);
  });

  it("paints ambient row previews without React commits or accessible label churn", () => {
    vi.stubGlobal("Worker", undefined);
    const row = { entry: { id: walker.id }, status: "ready", asset: walker } as Row;
    const catalog = new CatalogStore({ manifest: async () => { throw new Error("unused"); }, category: async () => { throw new Error("unused"); } });
    const store = new ArchiveStore(memoryStorage(), VIEWS);
    store.dispatch({ type: "preferences-changed", patch: { motion: "reduced" } });
    let clock!: StaffageClock;
    function Preview() {
      clock = useArchive().ambient;
      return <RowPreview row={row} viewpoint="elevation" view="dark-line" />;
    }
    let commits = 0;
    const { container } = render(<ArchiveProvider catalog={catalog} store={store} language="en" t={translator("en")}><Profiler id="preview" onRender={() => commits++}><Preview /></Profiler></ArchiveProvider>);
    const preview = container.querySelector("[data-preview]")!;
    const first = preview.querySelector("svg")!.outerHTML;
    const count = commits;
    act(() => clock.step(4));
    const expected = pipeline.figureFrame(walker, { kind: "clip", clip: walker.presentation.defaultClip, timeS: 4 / clock.state.fps }, {
      view: "dark-line", viewpoint: "elevation", scale: { denominator: 100 }, language: "en", transparent: true,
      annotations: { dimensions: false, scaleBar: false, legend: false, labels: false, ground: true },
    });
    const reference = document.createElement("template");
    reference.innerHTML = pipeline.inlineSvg(expected.scene);
    expect(preview.querySelector("svg")!.outerHTML).not.toBe(first);
    expect(preview.querySelector("svg")!.outerHTML).toBe(reference.content.querySelector("svg")!.outerHTML);
    expect(commits).toBe(count);
    expect(preview.querySelector("[aria-label]")).toBeNull();
  });

  it("coalesces the worker queue while painting ordered results and cancelling a released figure", () => {
    const requests: DrawingRequest[] = [];
    const port = {
      postMessage: (request: DrawingRequest) => { requests.push(request); }, terminate: vi.fn(),
      onmessage: null as ((event: MessageEvent<DrawingReply>) => void) | null,
      onerror: null as ((event: ErrorEvent) => void) | null,
    };
    const queue = new DrawingQueue(port, vi.fn());
    const options = pipeline.depictionOptions({ view: "dark-line", viewpoint: "elevation", scale: { denominator: 100 }, language: "en" });
    const painted: [string, number][] = [];
    const fail = vi.fn();
    const first = queue.retain(walker, options, false, (_, query) => { painted.push(["a", query.timeS ?? 0]); }, fail);
    const second = queue.retain(walker, options, false, (_, query) => { painted.push(["b", query.timeS ?? 0]); }, fail);
    const clip = (timeS: number) => ({ kind: "clip", clip: walker.presentation.defaultClip, timeS } as const);
    const sent = () => requests.filter(request => request.kind === "draw");
    const reply = () => {
      const request = sent().at(-1)!;
      port.onmessage!({ data: { lease: request.lease, drawing: drawFigure(walker, options, request.query) } } as MessageEvent<DrawingReply>);
    };
    first.request(clip(0));
    second.request(clip(0));
    first.request(clip(1 / 12));
    first.request(clip(2 / 12));
    expect(requests.map(request => request.kind)).toEqual(["asset", "draw"]);
    reply();
    reply();
    expect(painted).toEqual([["a", 0], ["b", 0]]);
    expect(sent().map(request => request.query.timeS)).toEqual([0, 0, 2 / 12]);
    first.release();
    reply();
    expect(painted).toHaveLength(2);
    second.release();
    expect(requests.at(-1)?.kind).toBe("release");
    expect(port.terminate).toHaveBeenCalledOnce();
    expect(fail).not.toHaveBeenCalled();
  });

  it("budgets ambient drawing across display frames and drops stale work fairly", () => {
    const scheduler = manual();
    const drawing = manual();
    vi.stubGlobal("requestAnimationFrame", drawing.request);
    vi.stubGlobal("cancelAnimationFrame", drawing.cancel);
    let now = 0;
    vi.spyOn(performance, "now").mockImplementation(() => now);
    const clock = new StaffageClock({ scheduler, fps: 12, playing: true, drawBudgetMs: 5 });
    const steps: [string, number][] = [];
    const remove = ["a", "b", "c"].map(id => driveFigure(clock, {
      element: document.createElement("div"), animated: () => true, step: time => { steps.push([id, time]); now += 6; },
    }));
    clock.step(1);
    expect(steps).toEqual([]);
    drawing.tick(16);
    expect(steps).toEqual([["a", 1 / 12]]);
    clock.step(1);
    drawing.tick(32);
    drawing.tick(48);
    expect(steps).toEqual([["a", 1 / 12], ["b", 2 / 12], ["c", 2 / 12]]);
    remove.forEach(off => off());
    drawing.tick(64);
    expect(steps).toHaveLength(3);
    clock.dispose();
  });

  it("shares one subscription, never re-renders and steps only visible figures", () => {
    vi.stubGlobal("IntersectionObserver", class { readonly record: Observer; constructor(callback: IntersectionObserverCallback) { this.record = { callback, elements: new Set() }; observers.push(this.record); } observe(element: Element) { this.record.elements.add(element); } unobserve(element: Element) { this.record.elements.delete(element); } disconnect() { this.record.elements.clear(); } });
    const scheduler = manual();
    const clock = new StaffageClock({ scheduler, fps: 12, durationS: 1, playing: true, reducedMotion: false });
    const subscribe = vi.spyOn(clock, "subscribe");
    let commits = 0;
    const figures = Array.from({ length: 40 }, (_, index) => <PlacedFigure key={index} asset={walker} selection={null} clock={clock} offsetS={index / 40} view="dark-line" viewpoint="elevation" scale={100} language="en" pxPerMm={1} xPx={50 + index} groundPx={100} />);
    const { container } = render(<Profiler id="figures" onRender={() => commits++}><div>{figures}</div></Profiler>);
    expect(subscribe).toHaveBeenCalledTimes(1);
    const placed = [...container.querySelectorAll<HTMLElement>("[data-placed]")];
    expect(placed).toHaveLength(40);
    const first = placed[0]!, last = placed[39]!;
    const base = commits;
    const frames = new Set<string>();
    let now = 0;
    for (let frame = 0; frame < 10; frame++) { now += 1000 / 12; act(() => scheduler.tick(now)); frames.add(first.innerHTML); }
    expect(commits).toBe(base);
    expect(frames.size).toBeGreaterThan(3);
    const hide = (element: Element, isIntersecting: boolean) => act(() => observers[0]!.callback([{ target: element, isIntersecting } as IntersectionObserverEntry], observers[0] as never));
    hide(last, false);
    const frozen = last.innerHTML;
    for (let frame = 0; frame < 4; frame++) { now += 1000 / 12; act(() => scheduler.tick(now)); }
    expect(last.innerHTML).toBe(frozen);
    expect(first.innerHTML).not.toBe(frozen);
    hide(last, true);
    expect(last.innerHTML).not.toBe(frozen);
    expect(commits).toBe(base);
  });
});
