/** 🚦️ On an overview grid only the previews near the reader follow the ambient clock, and all live previews together stay within a weight budget; every other preview is a still figure without a drawing lease. A live preview replaces its whole drawing on every frame, so a grid of many assets that are all live holds gigabytes.
 * @see ./🥒️.feature — the scenarios
 * @see ../../🔨️modules/🖼️previews/🚦️live/🟦️.ts — `PreviewLiveSet`, `PREVIEW_LIVE_BUDGET_POINTS`
 * @see ../../🔨️modules/🗃️browse/🟦️.tsx — `AssetCard` */
import { act, cleanup, fireEvent, render } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { VIEWS, type ManifestEntry } from "@teaching/staffage";
import { StaffageClock } from "@teaching/staffage-react";
import { AssetCard } from "../../🔨️modules/🗃️browse/🟦️.tsx";
import { CatalogStore } from "../../🔨️modules/📚️catalog/🟦️.ts";
import { ArchiveProvider, useArchive } from "../../🔨️modules/🧭️context/🟦️.tsx";
import { translator } from "../../🔨️modules/🌐️i18n/🟦️.ts";
import { ArchiveStore, memoryStorage } from "../../🔨️modules/💾️persistence/🟦️.ts";
import { PREVIEW_LIVE_BUDGET_POINTS, PreviewLiveSet } from "../../🔨️modules/🖼️previews/🚦️live/🟦️.ts";
import { defaultQuery, evaluateState, statePoints } from "../../🔨️modules/🧮️pipeline/🟦️.ts";
import { drawingBench, type DrawingReply, type DrawingRequest } from "../../🔨️modules/🧮️pipeline/🏁️drawing/🟦️.ts";
import type { Row } from "../../🔨️modules/🔎️search/🟦️.ts";
import { walker } from "../🧰️support/🧰️.ts";

type Watch = { readonly callback: IntersectionObserverCallback; readonly margin: boolean; readonly elements: Set<Element> };
const watches: Watch[] = [];

function observeIntersections(): void {
  vi.stubGlobal("IntersectionObserver", class {
    readonly watch: Watch;
    constructor(callback: IntersectionObserverCallback, options?: IntersectionObserverInit) {
      this.watch = { callback, margin: options?.rootMargin !== undefined, elements: new Set() };
      watches.push(this.watch);
    }
    observe(element: Element): void { this.watch.elements.add(element); }
    unobserve(element: Element): void { this.watch.elements.delete(element); }
    disconnect(): void { this.watch.elements.clear(); }
  });
}

type Place = "inside" | "near" | "far";

function scrollTo(card: Element, place: Place): void {
  act(() => {
    for (const watch of watches) for (const element of watch.elements) if (card.contains(element)) watch.callback([{ target: element, isIntersecting: watch.margin ? place !== "far" : place === "inside" } as IntersectionObserverEntry], watch as never);
  });
}

const text = (value: string) => ({ en: value, de: value });
const entry = (id: string): ManifestEntry => ({ id, category: "humans", title: text(id), heightM: 1.7, widthM: 0.5, powerW: 0, poses: [], clips: [], representations: { "500": "unavailable", "200": "unavailable", "100": "authored", "50": "unavailable" }, symbols: [], analytics: [], exports: ["svg"], distinction: text(id), completion: "authored" }) as unknown as ManifestEntry;

const each = statePoints(evaluateState(walker, defaultQuery(walker, 0), { kind: "scale", scale: { denominator: 100 } }));
const seats = Math.floor(PREVIEW_LIVE_BUDGET_POINTS / each);

type Grid = { readonly cards: readonly HTMLElement[]; readonly clock: StaffageClock; readonly step: () => readonly HTMLElement[] };

function grid(count: number): Grid {
  const rows = Array.from({ length: count }, (_, index): Row => ({ entry: entry(`asset-${index}`), status: "ready", asset: walker }) as Row);
  const catalog = new CatalogStore({ manifest: async () => { throw new Error("unused"); }, category: async () => { throw new Error("unused"); } });
  const store = new ArchiveStore(memoryStorage(), VIEWS);
  store.dispatch({ type: "preferences-changed", patch: { motion: "reduced" } });
  let clock!: StaffageClock;
  function Cards() {
    clock = useArchive().ambient;
    return <ul>{rows.map((row) => <li key={row.entry.id}><AssetCard row={row} /></li>)}</ul>;
  }
  const { container } = render(<ArchiveProvider catalog={catalog} store={store} language="en" t={translator("en")}><Cards /></ArchiveProvider>);
  const cards = [...container.querySelectorAll<HTMLElement>("[data-card]")];
  const drawing = (card: HTMLElement): string => card.querySelector("[data-preview] svg")?.outerHTML ?? "";
  return {
    cards,
    clock,
    step: () => {
      const before = cards.map(drawing);
      act(() => clock.step(3));
      return cards.filter((card, index) => drawing(card) !== before[index]);
    },
  };
}

const state = (card: HTMLElement): string | undefined => card.querySelector<HTMLElement>("[data-preview-state]")?.dataset.previewState;
const weight = (cards: readonly HTMLElement[]): number => cards.reduce((sum, card) => sum + Number(card.querySelector<HTMLElement>("[data-preview-weight]")?.dataset.previewWeight ?? Number.NaN), 0);

afterEach(() => { cleanup(); watches.length = 0; vi.restoreAllMocks(); vi.unstubAllGlobals(); });

describe("🚦️ preview liveness", () => {
  it("measures the reference walker and a budget that seats some but not many of it", () => {
    expect(each).toBeGreaterThan(100);
    expect(seats).toBeGreaterThanOrEqual(2);
    expect(seats).toBeLessThan(200);
  });

  it("keeps the previews that move within the live budget when every card is inside the viewport", () => {
    vi.stubGlobal("Worker", undefined);
    const { cards, step } = grid(seats + 6);
    expect(cards).toHaveLength(seats + 6);
    const moved = step();
    expect(moved.length).toBe(seats);
    expect(weight(moved)).toBeLessThanOrEqual(PREVIEW_LIVE_BUDGET_POINTS);
    expect(moved.every((card) => state(card) === "live")).toBe(true);
    const rest = cards.filter((card) => !moved.includes(card));
    expect(rest).toHaveLength(6);
    for (const card of rest) {
      expect(state(card)).toBe("still");
      expect(card.querySelector("[data-preview] svg")).not.toBeNull();
    }
    expect([...step()].sort()).toEqual([...moved].sort());
  });

  it("keeps a preview further away than one row still and makes it live when it is scrolled near", () => {
    vi.stubGlobal("Worker", undefined);
    observeIntersections();
    const { cards, step } = grid(6);
    const places: readonly Place[] = ["inside", "inside", "near", "near", "far", "far"];
    cards.forEach((card, index) => scrollTo(card, places[index]!));
    const wanted = Math.min(4, seats);
    expect(cards.map(state)).toEqual(cards.map((_, index) => (index < wanted ? "live" : "still")));
    for (const card of cards) expect(card.querySelector("[data-preview] svg")).not.toBeNull();
    const moved = step();
    expect(moved).toEqual(cards.slice(0, Math.min(2, seats)));
    scrollTo(cards[0]!, "far");
    scrollTo(cards[1]!, "far");
    scrollTo(cards[4]!, "inside");
    expect(state(cards[0]!)).toBe("still");
    expect(state(cards[4]!)).toBe("live");
    expect(step()).toContain(cards[4]);
    expect(step()).not.toContain(cards[0]);
  });

  it("makes a hovered or focused still preview live within the budget", () => {
    vi.stubGlobal("Worker", undefined);
    const { cards, step } = grid(seats + 3);
    const [first, second] = cards.filter((card) => state(card) === "still");
    expect(first).toBeDefined();
    expect(second).toBeDefined();
    act(() => void fireEvent.pointerOver(first!.querySelector("a")!));
    expect(state(first!)).toBe("live");
    let moved = step();
    expect(moved).toContain(first);
    expect(weight(moved)).toBeLessThanOrEqual(PREVIEW_LIVE_BUDGET_POINTS);
    expect(cards.filter((card) => state(card) === "live")).toHaveLength(seats);
    act(() => second!.querySelector("a")!.focus());
    expect(state(second!)).toBe("live");
    expect(state(first!)).toBe("live");
    moved = step();
    expect(moved).toContain(second);
    expect(weight(moved)).toBeLessThanOrEqual(PREVIEW_LIVE_BUDGET_POINTS);
    act(() => second!.querySelector("a")!.blur());
    act(() => void fireEvent.pointerOut(first!.querySelector("a")!, { relatedTarget: null }));
    expect(cards.filter((card) => state(card) === "live")).toHaveLength(seats);
  });

  it("serves seats by nearness and weight", () => {
    const set = new PreviewLiveSet(100);
    const live = new Map<string, boolean>();
    const seat = (name: string) => set.seat({ element: document.createElement("div"), live: (next) => void live.set(name, next) });
    const a = seat("a"), b = seat("b"), c = seat("c");
    a.weigh(60);
    b.weigh(60);
    c.weigh(30);
    expect(Object.fromEntries(live)).toEqual({ a: true, c: true });
    expect(set.load).toEqual({ seats: 3, live: 2, points: 90 });
    a.release();
    expect(live.get("b")).toBe(true);
    expect(set.load).toEqual({ seats: 2, live: 2, points: 90 });
    b.release();
    c.release();
    const heavy = seat("heavy");
    heavy.weigh(1000);
    expect(live.get("heavy")).toBe(true);
    const next = seat("next");
    next.weigh(10);
    expect(live.get("next")).toBeUndefined();
    heavy.release();
    expect(live.get("next")).toBe(true);
    next.release();
    expect(set.load).toEqual({ seats: 0, live: 0, points: 0 });
  });

  it("ends the drawing worker once every still figure has arrived and starts it again for a live preview", async () => {
    observeIntersections();
    const workers: { readonly requests: DrawingRequest[]; ended: boolean }[] = [];
    vi.stubGlobal("Worker", class {
      onmessage: ((event: MessageEvent<DrawingReply>) => void) | null = null;
      onerror: ((event: ErrorEvent) => void) | null = null;
      readonly record = { requests: [] as DrawingRequest[], ended: false };
      readonly answer = drawingBench();
      constructor() { workers.push(this.record); }
      postMessage(request: DrawingRequest): void {
        this.record.requests.push(request);
        const reply = this.answer(request);
        if (reply !== null) setTimeout(() => { if (!this.record.ended) act(() => this.onmessage?.({ data: reply } as MessageEvent<DrawingReply>)); }, 0);
      }
      terminate(): void { this.record.ended = true; }
    });
    const { cards } = grid(3);
    cards.forEach((card) => scrollTo(card, "far"));
    await vi.waitFor(() => expect(cards.every((card) => card.querySelector("[data-preview] svg") !== null)).toBe(true));
    expect(cards.map(state)).toEqual(["still", "still", "still"]);
    expect(workers).toHaveLength(1);
    expect(workers[0]!.ended).toBe(true);
    expect(workers[0]!.requests.filter((request) => request.kind === "draw")).toHaveLength(3);
    scrollTo(cards[1]!, "inside");
    expect(state(cards[1]!)).toBe("live");
    expect(workers).toHaveLength(2);
    expect(workers[1]!.ended).toBe(false);
    scrollTo(cards[1]!, "far");
    expect(state(cards[1]!)).toBe("still");
    expect(workers[1]!.ended).toBe(true);
  });
});
