// #region 🔌️Adapters
import { act, fireEvent, render, screen, within } from "@testing-library/react";
import * as React from "react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { centeredLastRowCells, followFactor, veilClip, veilClipPath, type LayeredCell } from "../../../../🔨️modules/🥞️layered-overview-geometry/🟦️.ts";
import { OverviewCard, OverviewCardAction } from "../../../🃏️OverviewCard/🟦️.tsx";
import { LayeredOverview, capturePosterFromCanvases, type LayeredCardState, type LayeredOverviewProps, type LayeredPane } from "../../🟦️.tsx";
// #endregion 🔌️Adapters

// #region 🥞️LayeredOverviewBehaviour
const HOME = ["learner", "physics", "intro", "heating", "board", "cooling", "badges", "demand", "prefs"] as const;
const LABEL = (id: string): string => id.charAt(0).toUpperCase() + id.slice(1);
const GRID: Readonly<Record<string, LayeredCell>> = Object.fromEntries(HOME.map((id, index) => [id, { column: index % 3, row: Math.floor(index / 3) }]));
const LABELS: LayeredOverviewProps["labels"] = { grid: "Quizzes", overview: "Overview", waiting: (pane) => `${pane.label} is waiting to start`, failed: (pane) => `${pane.label} could not be loaded.` };
const LATER = { warmStartMs: 1e9, warmIntervalMs: 1e9 } as const;

/** 🥞️ Pages that record when they are rendered. */
function pages(ids: readonly string[], rendered: string[] = [], extra: Partial<Record<string, Partial<LayeredPane>>> = {}): readonly LayeredPane[] {
  return ids.map((id) => ({ id, label: LABEL(id), icon: "award", render: ({ opened }) => (rendered.push(id), <div data-page={id}>{`${LABEL(id)} page${opened ? " (open)" : ""}`}</div>), ...extra[id] }));
}

/** 🃏️ A quiz-like card: a region named by a heading link, with a real Open button. */
function card(pane: LayeredPane, state: LayeredCardState): React.ReactNode {
  return (
    <OverviewCard as="section" slot="test-card" headingId={`card-${pane.id}`} icon={null} title={<a href={`#${pane.id}`}>{pane.label}</a>} className="pointer-events-auto" footerRight={<OverviewCardAction primary onClick={state.open}>{`Open ${pane.label}`}</OverviewCardAction>}>
      <p>{state.revealed ? "revealed" : "resting"}</p>
    </OverviewCard>
  );
}

function renderHome(props: Partial<LayeredOverviewProps> = {}, rendered: string[] = []) {
  const result = render(<LayeredOverview panes={pages(HOME, rendered)} cells={GRID} renderCard={card} labels={LABELS} lifecycle={LATER} {...props} />);
  const root = result.container.querySelector<HTMLElement>("[data-layered-overview]")!;
  return { ...result, root, strip: () => root.querySelector<HTMLElement>("[data-layered-strip]")!, veil: () => root.querySelector<HTMLElement>("[data-layered-veil]"), cardOf: (id: string) => root.querySelector<HTMLElement>(`[data-layered-card="${id}"] section`)! };
}

const fakeClock = () => vi.useFakeTimers({ toFake: ["setTimeout", "clearTimeout", "setInterval", "clearInterval", "requestAnimationFrame", "cancelAnimationFrame", "performance", "Date"] });
const pane = (root: HTMLElement, id: string) => root.querySelector<HTMLElement>(`[data-layered-pane="${id}"]`)!;
const openedPane = () => document.querySelector<HTMLElement>("[data-layered-pane][data-opened]");
const frames = (ms: number) => {
  for (let spent = 0; spent < ms; spent += 10) act(() => void vi.advanceTimersByTime(Math.min(10, ms - spent)));
};
const numbers = (text: string) => (text.match(/-?\d+(?:\.\d+)?/gu) ?? []).map(Number);
const offsetOf = (transform: string, columns: number, rows: number) => {
  const [x = 0, y = 0] = numbers(transform.replace(/^translate3d/u, ""));
  return { x: (-x * columns) / 100, y: (-y * rows) / 100 };
};
const sized = (root: HTMLElement) => vi.spyOn(root, "getBoundingClientRect").mockReturnValue({ left: 0, top: 0, width: 900, height: 900, right: 900, bottom: 900, x: 0, y: 0, toJSON: () => ({}) });
const moveMouse = (clientX: number, clientY: number, pointerType: string = "mouse") => act(() => void window.dispatchEvent(new PointerEvent("pointermove", { clientX, clientY, pointerType })));

beforeEach(() => window.history.replaceState(null, "", "/"));
afterEach(() => {
  vi.useRealTimers();
  vi.restoreAllMocks();
  vi.unstubAllGlobals();
  window.history.replaceState(null, "", "/");
});

describe("LayeredOverview", () => {
  it("stacks an inert strip, one dialog-level glass and nine card regions in reading order", () => {
    const { root, strip, veil } = renderHome();
    expect(root.dataset.mode).toBe("strip");
    expect(strip().style.width).toBe("300%");
    expect(strip().style.height).toBe("300%");
    expect(strip().style.transform).toBe("translate3d(0%, 0%, 0)");
    const panes = [...root.querySelectorAll<HTMLElement>("[data-layered-pane]")];
    expect(panes.map((element) => element.dataset.layeredPane)).toEqual([...HOME]);
    for (const element of panes) {
      expect(element.hasAttribute("inert")).toBe(true);
      expect(element.getAttribute("aria-hidden")).toBe("true");
      expect(element.getAttribute("role")).toBeNull();
    }
    expect(root.querySelectorAll(".ui-veil")).toHaveLength(1);
    expect(veil()!.getAttribute("data-level")).toBe("dialog");
    expect(veil()!.dataset.veil).toBe("whole");
    const group = screen.getByRole("group", { name: "Quizzes" });
    const regions = within(group).getAllByRole("region");
    expect(regions.map((region) => region.getAttribute("aria-labelledby"))).toEqual(HOME.map((id) => `card-${id}`));
    expect(regions.map((region) => region.textContent?.replace(/\s+/gu, ""))).toEqual(HOME.map((id) => `${LABEL(id)}restingOpen${LABEL(id)}`));
  });

  it("reveals a page on mouse hover (glass hidden, card lifted, chrome told) and restores the glass on leave", () => {
    const chrome = vi.fn(() => null);
    const revealed = vi.fn();
    const { strip, veil, cardOf } = renderHome({ reducedMotion: "always", renderChrome: chrome, onRevealedIdChange: revealed });
    fireEvent.pointerEnter(cardOf("board"), { pointerType: "mouse" });
    expect(strip().style.transform).toBe("translate3d(-33.3333%, -33.3333%, 0)");
    expect(veil()!.dataset.veil).toBe("clear");
    expect(veil()!.style.visibility).toBe("hidden");
    expect(cardOf("board").closest("[data-layered-card]")!.hasAttribute("data-revealed")).toBe(true);
    expect(within(cardOf("board")).getByText("revealed")).toBeTruthy();
    expect(chrome).toHaveBeenLastCalledWith({ revealed: true, opened: false });
    expect(revealed).toHaveBeenLastCalledWith("board");
    fireEvent.pointerLeave(cardOf("board"), { pointerType: "mouse" });
    expect(veil()!.dataset.veil).toBe("whole");
    expect(veil()!.style.visibility).toBe("visible");
    expect(chrome).toHaveBeenLastCalledWith({ revealed: false, opened: false });
    expect(revealed).toHaveBeenLastCalledWith(null);
  });

  it("re-renders only the cards whose reveal changed", () => {
    const rendered: string[] = [];
    const counting = (pane: LayeredPane, state: LayeredCardState) => (rendered.push(pane.id), card(pane, state));
    const { cardOf } = renderHome({ reducedMotion: "always", renderCard: counting });
    rendered.length = 0;
    fireEvent.pointerEnter(cardOf("board"), { pointerType: "mouse" });
    expect(rendered).toEqual(["board"]);
    rendered.length = 0;
    fireEvent.pointerEnter(cardOf("cooling"), { pointerType: "mouse" });
    expect(rendered.sort()).toEqual(["board", "cooling"]);
  });

  it("never reveals on a touch pointer", () => {
    const { veil, cardOf } = renderHome({ reducedMotion: "always" });
    fireEvent.pointerEnter(cardOf("board"), { pointerType: "touch" });
    expect(cardOf("board").closest("[data-layered-card]")!.hasAttribute("data-revealed")).toBe(false);
    expect(veil()!.dataset.veil).toBe("whole");
  });

  it("glides 500 ms from the CURRENT offset and cuts the hole from it every frame", () => {
    fakeClock();
    const { strip, veil, cardOf } = renderHome({ reducedMotion: "never" });
    fireEvent.pointerEnter(cardOf("cooling"), { pointerType: "mouse" });
    expect(strip().style.transform).toBe("translate3d(0%, 0%, 0)");
    expect(veil()!.dataset.veil).toBe("whole");
    frames(400);
    const middle = offsetOf(strip().style.transform, 3, 3);
    expect(middle.x).toBeGreaterThan(1.5);
    expect(middle.x).toBeLessThan(2);
    expect(middle.y).toBeCloseTo(middle.x / 2, 3);
    const expected = veilClip({ column: 2, row: 1 }, middle);
    expect(expected.kind).toBe("hole");
    expect(veil()!.dataset.veil).toBe("hole");
    const painted = numbers(veil()!.style.clipPath);
    const wanted = numbers(veilClipPath(expected));
    expect(painted).toHaveLength(wanted.length);
    painted.forEach((value, index) => expect(value).toBeCloseTo(wanted[index]!, 2));
    frames(200);
    expect(strip().style.transform).toBe("translate3d(-66.6667%, -33.3333%, 0)");
    expect(veil()!.dataset.veil).toBe("clear");
  });

  it("keeps its warm queue and a running glide when the app re-renders with fresh but equal panes and cells", () => {
    fakeClock();
    const view = () => <LayeredOverview panes={pages(HOME)} cells={{ ...GRID }} renderCard={card} labels={LABELS} lifecycle={{ budget: 9, warmStartMs: 100, warmIntervalMs: 100 }} reducedMotion="never" />;
    const { rerender, container } = render(view());
    for (let step = 0; step < 6; step += 1) {
      frames(40);
      rerender(view());
    }
    expect(container.querySelectorAll("[data-page]").length).toBeGreaterThan(0);
    fireEvent.pointerEnter(container.querySelector<HTMLElement>('[data-layered-card="cooling"] section')!, { pointerType: "mouse" });
    frames(100);
    rerender(view());
    frames(100);
    const middle = offsetOf(container.querySelector<HTMLElement>("[data-layered-strip]")!.style.transform, 3, 3);
    expect(middle.x).toBeGreaterThan(0);
    expect(middle.x).toBeLessThan(2);
  });

  it("follows the mouse across the strip, closing 12 % of the gap per 60 Hz frame by elapsed time, and settles exactly", () => {
    fakeClock();
    const { root, strip } = renderHome({ reducedMotion: "never" });
    vi.spyOn(root, "getBoundingClientRect").mockReturnValue({ left: 0, top: 0, width: 900, height: 900, right: 900, bottom: 900, x: 0, y: 0, toJSON: () => ({}) });
    act(() => void window.dispatchEvent(new PointerEvent("pointermove", { clientX: 900, clientY: 450, pointerType: "mouse" })));
    frames(16);
    expect(offsetOf(strip().style.transform, 3, 3).x).toBeCloseTo(2 * followFactor(16), 3);
    frames(3000);
    expect(strip().style.transform).toBe("translate3d(-66.6667%, -33.3333%, 0)");
  });

  it("pans as far in the same time whether the browser paints sixty frames a second or ten", () => {
    const after = (frameMs: number): number => {
      vi.useFakeTimers({ toFake: ["setTimeout", "clearTimeout", "setInterval", "clearInterval", "performance", "Date"] });
      let next = 0;
      const queued = new Map<number, FrameRequestCallback>();
      vi.stubGlobal("requestAnimationFrame", (callback: FrameRequestCallback) => (queued.set((next += 1), callback), next));
      vi.stubGlobal("cancelAnimationFrame", (handle: number) => void queued.delete(handle));
      const { root, strip, unmount } = renderHome({ reducedMotion: "never" });
      sized(root);
      moveMouse(900, 450);
      for (let spent = 0; spent < 300; spent += frameMs) {
        act(() => void vi.advanceTimersByTime(frameMs));
        const due = [...queued.values()];
        queued.clear();
        act(() => due.forEach((callback) => callback(performance.now())));
      }
      const x = offsetOf(strip().style.transform, 3, 3).x;
      unmount();
      vi.useRealTimers();
      vi.unstubAllGlobals();
      return x;
    };
    const [smooth, slow] = [after(1000 / 60), after(100)];
    expect(smooth).toBeCloseTo(2 * followFactor(300), 2);
    expect(slow).toBeCloseTo(smooth, 2);
  });

  it("pans and glides by default whatever the device says about motion, and gives way to a device that asks for less only for an app that says `auto`", () => {
    fakeClock();
    vi.stubGlobal("matchMedia", (query: string) => ({ matches: query.includes("reduce"), media: query, onchange: null, addEventListener: () => undefined, removeEventListener: () => undefined, addListener: () => undefined, removeListener: () => undefined, dispatchEvent: () => false }));
    const moving = renderHome();
    sized(moving.root);
    expect(moving.root.dataset.pan).toBe("pointer");
    moveMouse(900, 900);
    frames(3000);
    expect(moving.strip().style.transform).toBe("translate3d(-66.6667%, -66.6667%, 0)");
    fireEvent.pointerEnter(moving.cardOf("learner"), { pointerType: "mouse" });
    expect(moving.strip().style.transform).toBe("translate3d(-66.6667%, -66.6667%, 0)");
    frames(250);
    const middle = offsetOf(moving.strip().style.transform, 3, 3);
    expect(middle.x).toBeGreaterThan(0);
    expect(middle.x).toBeLessThan(2);
    frames(300);
    expect(moving.strip().style.transform).toBe("translate3d(0%, 0%, 0)");
    moving.unmount();
    const still = renderHome({ reducedMotion: "auto" });
    sized(still.root);
    expect(still.root.dataset.pan).toBe("none");
    moveMouse(900, 900);
    frames(3000);
    expect(still.strip().style.transform).toBe("translate3d(0%, 0%, 0)");
    fireEvent.pointerEnter(still.cardOf("prefs"), { pointerType: "mouse" });
    expect(still.strip().style.transform).toBe("translate3d(-66.6667%, -66.6667%, 0)");
  });

  it("reveals on keyboard focus and conceals when focus leaves the card", () => {
    const { veil, cardOf } = renderHome({ reducedMotion: "always" });
    const link = within(cardOf("badges")).getByRole("link", { name: "Badges" });
    act(() => link.focus());
    expect(veil()!.dataset.veil).toBe("clear");
    act(() => within(cardOf("badges")).getByRole("button").focus());
    expect(veil()!.dataset.veil).toBe("clear");
    act(() => within(cardOf("badges")).getByRole("button").blur());
    expect(veil()!.dataset.veil).toBe("whole");
  });

  it("opens a card as a focused, labelled region and returns focus to the card on Escape", () => {
    const opened = vi.fn();
    const { root, veil } = renderHome({ reducedMotion: "always", onOpenedIdChange: opened });
    fireEvent.click(screen.getByRole("button", { name: "Open Heating" }));
    expect(window.location.hash).toBe("#heating");
    expect(opened).toHaveBeenLastCalledWith("heating");
    expect(screen.queryByRole("group", { name: "Quizzes" })).toBeNull();
    expect(veil()).toBeNull();
    const region = screen.getByRole("region", { name: "Heating" });
    expect(region).toBe(pane(root, "heating"));
    expect(region.hasAttribute("inert")).toBe(false);
    expect(region.hasAttribute("data-opened")).toBe(true);
    expect(document.activeElement).toBe(region);
    expect(pane(root, "cooling").hasAttribute("inert")).toBe(true);
    expect(root.querySelector("[data-layered-overview-button]")!.textContent?.trim()).toBe("Overview");
    fireEvent.keyDown(window, { key: "Escape" });
    expect(window.location.hash).toBe("");
    expect(opened).toHaveBeenLastCalledWith(null);
    expect(screen.getByRole("group", { name: "Quizzes" })).toBeTruthy();
    expect(document.activeElement).toBe(screen.getByRole("link", { name: "Heating" }));
    expect(root.querySelector("[data-layered-overview-button]")).toBeNull();
  });

  it("closes with the Overview button", () => {
    renderHome({ reducedMotion: "always" });
    fireEvent.click(screen.getByRole("button", { name: "Open Demand" }));
    fireEvent.click(screen.getByRole("button", { name: "Overview" }));
    expect(openedPane()).toBeNull();
    expect(document.activeElement).toBe(screen.getByRole("link", { name: "Demand" }));
  });

  it("shows no Overview button of its own for an app that names none, whose chrome closes the page and keeps the focus", () => {
    const { overview: _, ...named } = LABELS;
    function App(): React.ReactElement {
      const [openedId, setOpenedId] = React.useState<string | null>("demand");
      return (
        <>
          <button type="button" onClick={() => setOpenedId(null)}>
            Back to all
          </button>
          <LayeredOverview panes={pages(HOME)} cells={GRID} renderCard={card} labels={named} lifecycle={LATER} reducedMotion="always" routing="none" openedId={openedId} onOpenedIdChange={setOpenedId} />
        </>
      );
    }
    const { container } = render(<App />);
    expect(openedPane()?.dataset.layeredPane).toBe("demand");
    expect(container.querySelector("[data-layered-overview-button]")).toBeNull();
    const chrome = screen.getByRole("button", { name: "Back to all" });
    act(() => chrome.focus());
    fireEvent.click(chrome);
    expect(openedPane()).toBeNull();
    expect(document.activeElement).toBe(chrome);
    fireEvent.click(screen.getByRole("button", { name: "Open Board" }));
    expect(openedPane()?.dataset.layeredPane).toBe("board");
    expect(container.querySelector("[data-layered-overview-button]")).toBeNull();
    fireEvent.keyDown(window, { key: "Escape" });
    expect(openedPane()).toBeNull();
    expect(window.location.hash).toBe("");
  });

  it("follows hash changes and leaves a hash naming no pane alone", () => {
    renderHome({ reducedMotion: "always" });
    act(() => {
      window.location.hash = "#intro";
      window.dispatchEvent(new HashChangeEvent("hashchange"));
    });
    expect(screen.getByRole("region", { name: "Intro" })).toBeTruthy();
    act(() => {
      window.history.replaceState(null, "", "/");
      window.dispatchEvent(new HashChangeEvent("hashchange"));
    });
    expect(openedPane()).toBeNull();
    act(() => {
      window.location.hash = "#main";
      window.dispatchEvent(new HashChangeEvent("hashchange"));
    });
    expect(window.location.hash).toBe("#main");
    expect(screen.getByRole("group", { name: "Quizzes" })).toBeTruthy();
  });

  it("deep-links a page: only that page mounts, at its cell, without cards or glass", () => {
    window.history.replaceState(null, "", "/#badges");
    const rendered: string[] = [];
    const opened = vi.fn();
    const { root, strip, veil } = renderHome({ onOpenedIdChange: opened }, rendered);
    expect(new Set(rendered)).toEqual(new Set(["badges"]));
    expect(strip().style.transform).toBe("translate3d(0%, -66.6667%, 0)");
    expect(veil()).toBeNull();
    expect(root.querySelector("[data-layered-overlay]")).toBeNull();
    expect(screen.getByRole("region", { name: "Badges" }).textContent).toBe("Badges page (open)");
    expect(opened).toHaveBeenCalledWith("badges");
    expect(window.location.hash).toBe("#badges");
  });

  it("stays controlled: it requests changes and shows what it is given", () => {
    const requested = vi.fn();
    const { rerender } = render(<LayeredOverview panes={pages(HOME)} cells={GRID} renderCard={card} labels={LABELS} lifecycle={LATER} reducedMotion="always" openedId="physics" onOpenedIdChange={requested} />);
    expect(screen.getByRole("region", { name: "Physics" })).toBeTruthy();
    expect(window.location.hash).toBe("#physics");
    fireEvent.click(screen.getByRole("button", { name: "Overview" }));
    expect(requested).toHaveBeenLastCalledWith(null);
    expect(screen.getByRole("region", { name: "Physics" })).toBeTruthy();
    rerender(<LayeredOverview panes={pages(HOME)} cells={GRID} renderCard={card} labels={LABELS} lifecycle={LATER} reducedMotion="always" openedId={null} onOpenedIdChange={requested} />);
    expect(openedPane()).toBeNull();
    expect(window.location.hash).toBe("");
  });

  it("keeps at most `budget` pages mounted and releases the least recently touched pristine one", () => {
    const { root, cardOf } = renderHome({ reducedMotion: "always", lifecycle: { ...LATER, budget: 2 }, windowing: { radius: 9 } });
    const mounted = () => [...root.querySelectorAll<HTMLElement>("[data-page]")].map((element) => element.dataset.page).sort();
    for (const id of ["learner", "physics", "intro"]) {
      fireEvent.pointerEnter(cardOf(id), { pointerType: "mouse" });
      fireEvent.pointerLeave(cardOf(id), { pointerType: "mouse" });
      expect(mounted().length).toBeLessThanOrEqual(2);
    }
    expect(mounted()).toEqual(["intro", "physics"]);
    expect(within(pane(root, "learner")).getByRole("status", { name: "Learner is waiting to start", hidden: true })).toBeTruthy();
  });

  it("warms pages in cell order and releases pristine ones by time, keeping the one behind the overview", () => {
    fakeClock();
    const { root } = renderHome({ reducedMotion: "always", lifecycle: { budget: 3, warmStartMs: 100, warmIntervalMs: 100, suspendIdleMs: 1_000, sweepMs: 100 } });
    const mounted = () => [...root.querySelectorAll<HTMLElement>("[data-page]")].map((element) => element.dataset.page);
    frames(99);
    expect(mounted()).toEqual([]);
    frames(1);
    expect(mounted()).toEqual(["learner"]);
    frames(200);
    expect(mounted()).toEqual(["learner", "physics", "intro"]);
    frames(5_000);
    expect(mounted()).toEqual(["learner"]);
  });

  it("boots the pages at once, in cell order and within the budget, when the warm pace is zero", () => {
    const all = renderHome({ reducedMotion: "always", lifecycle: { budget: 9, warmStartMs: 0, warmIntervalMs: 0 } });
    expect([...all.root.querySelectorAll<HTMLElement>("[data-page]")].map((element) => element.dataset.page)).toEqual([...HOME]);
    all.unmount();
    const some = renderHome({ reducedMotion: "always", lifecycle: { budget: 3, warmStartMs: 0, warmIntervalMs: 0 } });
    expect([...some.root.querySelectorAll<HTMLElement>("[data-page]")].map((element) => element.dataset.page)).toEqual(["learner", "physics", "intro"]);
  });

  it("gives a DOM-only page no poster but shows a poster function's still once released", () => {
    const poster = "data:image/png;base64,AAAA";
    const { container } = render(
      <LayeredOverview
        panes={pages(["dom", "canvas"], [], { dom: { capturePoster: capturePosterFromCanvases }, canvas: { capturePoster: () => poster } })}
        cells={{ dom: { column: 0, row: 0 }, canvas: { column: 1, row: 0 } }}
        renderCard={card}
        labels={LABELS}
        lifecycle={{ ...LATER, budget: 1 }}
        reducedMotion="always"
      />,
    );
    const root = container.querySelector<HTMLElement>("[data-layered-overview]")!;
    const cardOf = (id: string) => root.querySelector<HTMLElement>(`[data-layered-card="${id}"] section`)!;
    for (const id of ["dom", "canvas", "dom"]) {
      fireEvent.pointerEnter(cardOf(id), { pointerType: "mouse" });
      fireEvent.pointerLeave(cardOf(id), { pointerType: "mouse" });
    }
    expect(pane(root, "canvas").querySelector("img")!.getAttribute("src")).toBe(poster);
    fireEvent.pointerEnter(cardOf("canvas"), { pointerType: "mouse" });
    expect(pane(root, "dom").querySelector("img")).toBeNull();
    expect(within(pane(root, "dom")).getByRole("status", { name: "Dom is waiting to start", hidden: true })).toBeTruthy();
  });

  it("isolates a crashing page behind its failed label", () => {
    vi.spyOn(console, "error").mockImplementation(() => undefined);
    const broken: Partial<LayeredPane> = {
      render: () => {
        throw new Error("boom");
      },
    };
    render(<LayeredOverview panes={pages(HOME, [], { board: broken })} cells={GRID} renderCard={card} labels={LABELS} lifecycle={LATER} reducedMotion="always" />);
    fireEvent.pointerEnter(document.querySelector<HTMLElement>('[data-layered-card="board"] section')!, { pointerType: "mouse" });
    fireEvent.pointerEnter(document.querySelector<HTMLElement>('[data-layered-card="cooling"] section')!, { pointerType: "mouse" });
    expect(document.querySelector('[data-layered-pane="board"] [role="alert"]')!.textContent).toBe("Board could not be loaded.");
    expect(document.querySelector('[data-layered-pane="cooling"] [data-page]')!.textContent).toBe("Cooling page");
  });

  it("keeps placeholders only near the view, so 148 panes cost four placeholders at rest", () => {
    const ids = Array.from({ length: 148 }, (_, index) => `app${index}`);
    const cells = Object.fromEntries(centeredLastRowCells(148).map((cell, index) => [ids[index]!, cell]));
    const { container } = render(<LayeredOverview panes={pages(ids)} cells={cells} renderCard={card} labels={LABELS} lifecycle={LATER} reducedMotion="always" />);
    expect(container.querySelectorAll("[data-layered-pane]")).toHaveLength(148);
    expect(container.querySelectorAll('[data-layered-pane] [role="status"]')).toHaveLength(4);
    expect(container.querySelector<HTMLElement>("[data-layered-strip]")!.style.width).toBe("1300%");
  });

  it("never pans under reduced motion, with `pan=\"none\"` or for a touch pointer, and says on its root whether the mouse pans", () => {
    fakeClock();
    for (const [props, pan, pointerType] of [[{ reducedMotion: "always" }, "none", "mouse"], [{ reducedMotion: "never", pan: "none" }, "none", "mouse"], [{ reducedMotion: "never" }, "pointer", "touch"], [{ reducedMotion: "never" }, "pointer", "mouse"]] as const) {
      const { root, strip, unmount } = renderHome(props);
      sized(root);
      expect(root.dataset.pan).toBe(pan);
      moveMouse(900, 900, pointerType);
      frames(3000);
      expect(strip().style.transform).toBe(pan === "pointer" && pointerType === "mouse" ? "translate3d(-66.6667%, -66.6667%, 0)" : "translate3d(0%, 0%, 0)");
      unmount();
    }
  });

  it("holds the revealed page while the mouse moves over its card, and pans again once the mouse is between the cards", () => {
    fakeClock();
    const { root, strip, veil, cardOf } = renderHome({ reducedMotion: "never" });
    sized(root);
    fireEvent.pointerEnter(cardOf("board"), { pointerType: "mouse" });
    frames(600);
    expect(strip().style.transform).toBe("translate3d(-33.3333%, -33.3333%, 0)");
    expect(veil()!.dataset.veil).toBe("clear");
    moveMouse(900, 900);
    frames(600);
    expect(strip().style.transform).toBe("translate3d(-33.3333%, -33.3333%, 0)");
    fireEvent.pointerLeave(cardOf("board"), { pointerType: "mouse" });
    expect(veil()!.dataset.veil).toBe("whole");
    expect(strip().style.transform).toBe("translate3d(-33.3333%, -33.3333%, 0)");
    moveMouse(0, 900);
    frames(3000);
    expect(strip().style.transform).toBe("translate3d(0%, -66.6667%, 0)");
    expect(veil()!.dataset.veil).toBe("whole");
  });

  it("lists one snap section per pane under its own glass (near the view only), and locks the list while a page is open", () => {
    const wide = renderHome({ mode: "list", reducedMotion: "always", windowing: { radius: 9 } });
    expect(wide.root.querySelectorAll("[data-layered-veil][data-level='dialog']")).toHaveLength(9);
    wide.unmount();
    const { root } = renderHome({ mode: "list", reducedMotion: "always" });
    expect(root.dataset.mode).toBe("list");
    const list = screen.getByRole("group", { name: "Quizzes" });
    expect(list.className).toContain("snap-mandatory");
    expect(list.querySelectorAll("[data-layered-section]")).toHaveLength(9);
    expect([...list.querySelectorAll<HTMLElement>("[data-layered-section]")].map((section) => section.querySelector("[data-layered-veil]") !== null)).toEqual([true, true, false, false, false, false, false, false, false]);
    expect(within(list).getAllByRole("region")).toHaveLength(9);
    fireEvent.click(screen.getByRole("button", { name: "Open Board" }));
    expect(list.className).toContain("overflow-hidden");
    expect(root.querySelectorAll("[data-layered-veil]")).toHaveLength(0);
    expect(screen.getByRole("region", { name: "Board" })).toBe(pane(root, "board"));
  });
});
// #endregion 🥞️LayeredOverviewBehaviour
