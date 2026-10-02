/** 🚏️ Finding the way (shared vectors): the trail behind and ahead of the step in front, the place above it and the
 * address that names a page — and the navbar that offers them: the overview, back, forward and up on its left in that
 * order, what the quizzes are about in its middle, the connection and the language on its right. Every trail that
 * closes no run is replayed on a session history of `jsdom` (`pushState`, `replaceState`, `back`, `forward`) as the
 * third-party oracle: the trail keeps and forgets steps exactly as a browser's own history does.
 *
 * @see ../../🧫️fixtures/🚏️navigation/🔣️.json
 * @see https://html.spec.whatwg.org/multipage/nav-history-apis.html#the-history-interface
 */

import { act, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { JSDOM } from "jsdom";
import { afterEach, describe, expect, it } from "vitest";
import { decodeQueryEnvelope, encodeQueryResult, type HttpResponse, type HttpTransport } from "@semio-tech/framework-server";
import type { CatalogView, LearnerView, Query, RunStatus, RunView } from "@semio-tech/quiz";
import {
  EMPTY_TRAIL,
  NAVIGATION_WAYS,
  QuizApp,
  TRAIL_LIMIT,
  addressPage,
  evolveQuizState,
  homePages,
  initialQuizState,
  localStore,
  memoryStorageOrigin,
  navigationWays,
  placeName,
  quizText,
  sameStep,
  stepAbove,
  stepAddress,
  stepAfter,
  stepBefore,
  type PresenceConnect,
  type QuizState,
  type QuizStep,
  type StorageArea,
} from "@semio-tech/quiz-react";
import navigation from "../../🧫️fixtures/🚏️navigation/🔣️.json";

//#region 🧫️Vectors
type Step = { readonly screen: string; readonly page?: string; readonly run?: string };

interface Move {
  readonly open?: Step;
  readonly enter?: Step;
  readonly go?: string;
  readonly submit?: string;
  readonly close?: string;
  readonly as?: string;
  readonly void?: string;
  readonly step: Step;
  readonly back: Step | null;
  readonly forward: Step | null;
  readonly up: Step | null;
}

interface Fixture {
  readonly limit: number;
  readonly quizzes: readonly string[];
  readonly runs: Readonly<Record<string, { readonly quiz: string; readonly status: string }>>;
  readonly trails: readonly { readonly id: string; readonly start: Step; readonly moves: readonly Move[] }[];
  readonly addresses: readonly { readonly step: Step; readonly hash: string }[];
  readonly named: readonly { readonly hash: string; readonly page: string | null }[];
}

const fixture: Fixture = navigation;
const LEARNER = "a".repeat(32);
const IDENTITY = { kind: "pseudonym", handle: "Ada" } as const;
const text = (en: string, de: string) => ({ en, de });
const step = (vector: Step): QuizStep => vector as QuizStep;

const CATALOG: CatalogView = {
  id: "ways",
  title: text("Ways catalog", "Wegekatalog"),
  introduction: { title: text("How the quizzes work", "So funktionieren die Quizze"), paragraphs: [text("Classify, sort and match.", "Klassifiziere, sortiere und ordne zu.")] },
  quizzes: fixture.quizzes.map((id) => ({ id, emoji: "🧲", title: text(`Quiz ${id}`, `Quiz ${id}`), description: text(`About ${id}.`, `Über ${id}.`), tasks: [] })),
  badges: [],
};

/** 🏃️ A run of `quiz` without tasks: open, submitted with a perfect result, or voided. */
function runOf(run: string, quiz: string, status: RunStatus): RunView {
  const sheet = { quiz, seed: 1, title: text(`Quiz ${quiz}`, `Quiz ${quiz}`), description: text(`About ${quiz}.`, `Über ${quiz}.`), tasks: [] };
  return { run, learner: LEARNER, quiz, status, sheet, answers: {}, startedAt: 1, ...(status === "submitted" ? { result: { quiz, score: 1, tasks: [] }, submittedAt: 2 } : {}) };
}

/** 🧭️ The state of an identified learner at `start` with the fixture's runs and a trail of its own. */
function stateAt(start: Step): QuizState {
  const runs = Object.fromEntries(Object.entries(fixture.runs).map(([run, { quiz, status }]) => [run, runOf(run, quiz, status as RunStatus)]));
  return { ...initialQuizState({ introduced: true, learner: { id: LEARNER, identity: IDENTITY }, catalog: CATALOG, learnerView: undefined, runs }), step: step(start) };
}

/** 👣️ The state after one move of a vector. */
function moved(state: QuizState, move: Move): QuizState {
  if (move.open !== undefined) return evolveQuizState(state, { type: "step-opened", step: step(move.open) });
  if (move.enter !== undefined) return evolveQuizState(state, { type: "step-opened", step: step(move.enter), instead: true });
  if (move.submit !== undefined) return evolveQuizState(state, { type: "run-submitted", run: move.submit, result: { quiz: fixture.runs[move.submit]!.quiz, score: 1, tasks: [] }, badges: [], at: 2 });
  if (move.close !== undefined) return evolveQuizState(state, { type: "run-loaded", view: runOf(move.close, fixture.runs[move.close]!.quiz, move.as as RunStatus) });
  if (move.void !== undefined) return evolveQuizState(state, { type: "run-voided", run: move.void });
  if (move.go === "back" || move.go === "forward") return evolveQuizState(state, { type: "step-retraced", to: move.go });
  const target = move.go === "up" ? stepAbove(state) : ({ screen: "home" } as const);
  return target === undefined ? state : evolveQuizState(state, { type: "step-opened", step: target });
}

/** 🔮️ A session history of `jsdom` in a window of its own, holding `start`: the oracle a trail without closed runs is
 * replayed on. A step is pushed, replaced or traversed to as a browser does it; what lies one entry behind and ahead
 * is read by going there and back. */
async function sessionHistory(start: QuizStep) {
  const { window } = new JSDOM("", { url: "https://quiz.example/" });
  let entries = 0;
  const entry = (shown: QuizStep) => ({ step: shown, entry: entries++ });
  const here = () => window.history.state as { readonly step: QuizStep; readonly entry: number };
  const settled = async (): Promise<void> => {
    for (let turn = 0; turn < 3; turn += 1) await new Promise((resolve) => window.setTimeout(resolve, 0));
  };
  const travel = async (delta: -1 | 1): Promise<boolean> => {
    const from = here().entry;
    window.history.go(delta);
    await settled();
    return here().entry !== from;
  };
  const beside = async (delta: -1 | 1): Promise<QuizStep | null> => {
    if (!(await travel(delta))) return null;
    const found = here().step;
    await travel(delta === 1 ? -1 : 1);
    return found;
  };
  window.history.replaceState(entry(start), "");
  return {
    open: (shown: QuizStep): void => void (sameStep(shown, here().step) || window.history.pushState(entry(shown), "")),
    enter: (shown: QuizStep): void => window.history.replaceState(entry(shown), ""),
    travel,
    read: async () => ({ step: here().step, back: await beside(-1), forward: await beside(1) }),
    close: (): void => window.close(),
  };
}
//#endregion 🧫️Vectors

//#region 🛂️Client
const RUN_OPEN = "1".repeat(32);
const RUN_DONE = "2".repeat(32);
const RUNS: Readonly<Record<string, RunView>> = { [RUN_OPEN]: runOf(RUN_OPEN, "heating", "open"), [RUN_DONE]: runOf(RUN_DONE, "cooling", "submitted") };
const LEARNER_VIEW: LearnerView = {
  learner: LEARNER,
  identity: IDENTITY,
  runs: [
    { run: RUN_OPEN, quiz: "heating", status: "open", startedAt: 1 },
    { run: RUN_DONE, quiz: "cooling", status: "submitted", score: 1, startedAt: 1, submittedAt: 2 },
  ],
  badges: [],
  best: { cooling: 1 },
  total: 100,
};
const TIMING = { minMs: 1, maxMs: 4 };
const QUIET_PRESENCE: PresenceConnect = () => ({ readyState: 0, onmessage: null, onclose: null, onerror: null, send: () => undefined, close: () => undefined });
const LOGO = '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1 1"><path d="M0 0h1v1z"/></svg>';
const encoder = new TextEncoder();
const decoder = new TextDecoder();

function reply(status: number, body: unknown): HttpResponse {
  const payload = JSON.stringify(body);
  return { status, text: async () => payload, bytes: async () => encoder.encode(payload) };
}

/** 🛂️ A proctor that answers every read from fixed views and takes no command: finding the way writes nothing. */
const PROCTOR: HttpTransport = {
  send: async (request) => {
    if (request.path !== "/queries") return reply(404, { kind: "notFound", message: request.path });
    const query = JSON.parse(decoder.decode(decodeQueryEnvelope(JSON.parse(typeof request.body === "string" ? request.body : decoder.decode(request.body)) as unknown).arguments)) as Query;
    const answer = query.type === "catalog" ? CATALOG : query.type === "learner" ? LEARNER_VIEW : query.type === "run" ? RUNS[query.run] : query.type === "leaderboard" ? { rows: [], learners: 0 } : query.type === "crowd" ? { quiz: query.quiz, runs: 0, tasks: [] } : undefined;
    return answer === undefined ? reply(404, { kind: "notFound", message: query.type }) : reply(200, encodeQueryResult({ kind: "snapshot", value: encoder.encode(JSON.stringify(answer)), frontier: null }));
  },
};

/** 🧑‍🎓️ The storage of a device whose learner is identified, or of one that visits for the first time. */
function device(identified = true): StorageArea {
  const area = memoryStorageOrigin().tab();
  if (!identified) return area;
  const store = localStore(area, CATALOG.id);
  store.write("introduced", true);
  store.write("learner", { id: LEARNER, identity: IDENTITY });
  return area;
}

function app(storage: StorageArea) {
  return <QuizApp proctor="" tenant={CATALOG.id} logo={LOGO} presence={QUIET_PRESENCE} transport={() => PROCTOR} storage={storage} languages={["en"]} timing={TIMING} />;
}

function navbar(): HTMLElement {
  return screen.getByRole("navigation", { name: "Main navigation" });
}

function way(name: (typeof NAVIGATION_WAYS)[number]): HTMLElement {
  return navbar().querySelector<HTMLElement>(`[data-quiz-nav="${name}"]`)!;
}

function leads(name: (typeof NAVIGATION_WAYS)[number]): readonly [string | null, boolean] {
  return [way(name).getAttribute("aria-label"), way(name).getAttribute("aria-disabled") === "false"];
}

function pane(page: string): HTMLElement {
  return document.querySelector<HTMLElement>(`[data-layered-pane="${page}"]`)!;
}

function openedPage(): string | null {
  return document.querySelector<HTMLElement>("[data-layered-pane][data-opened]")?.dataset.layeredPane ?? null;
}

async function arrived(): Promise<void> {
  await screen.findByRole("heading", { level: 1, name: "Quizzes" });
  await act(async () => undefined);
}
//#endregion 🛂️Client

afterEach(() => {
  window.history.replaceState(null, "", window.location.pathname);
});

describe("🧵️ the trail", () => {
  for (const trail of fixture.trails) {
    it(`leads through the trail ${trail.id}: the step in front, the steps a way back and forward lead to, the place above`, () => {
      let state = stateAt(trail.start);
      for (const [index, move] of trail.moves.entries()) {
        state = moved(state, move);
        const found = { step: state.step, back: stepBefore(state) ?? null, forward: stepAfter(state) ?? null, up: stepAbove(state) ?? null };
        expect(found, `${trail.id}, move ${index + 1}`).toEqual({ step: move.step, back: move.back, forward: move.forward, up: move.up });
        const overview = sameStep(step(move.step), { screen: "home" }) ? undefined : { screen: "home" };
        expect(navigationWays(state), `${trail.id}, move ${index + 1}`).toEqual({ overview, back: move.back ?? undefined, forward: move.forward ?? undefined, up: move.up ?? undefined });
      }
    });
  }

  for (const trail of fixture.trails.filter((candidate) => candidate.moves.every((move) => move.submit === undefined && move.close === undefined && move.void === undefined))) {
    it(`keeps and forgets the steps of the trail ${trail.id} as jsdom's session history does`, async () => {
      const history = await sessionHistory(step(trail.start));
      let state = stateAt(trail.start);
      for (const [index, move] of trail.moves.entries()) {
        state = moved(state, move);
        if (move.enter !== undefined) history.enter(step(move.enter));
        else if (move.go === "back" || move.go === "forward") await history.travel(move.go === "back" ? -1 : 1);
        else history.open(state.step);
        expect({ step: state.step, back: stepBefore(state) ?? null, forward: stepAfter(state) ?? null }, `${trail.id}, move ${index + 1}`).toEqual(await history.read());
      }
      history.close();
    });
  }

  it("keeps no more steps behind than its limit and forgets the oldest", () => {
    expect(TRAIL_LIMIT).toBe(fixture.limit);
    let state = stateAt({ screen: "home" });
    for (let opening = 0; opening < 2 * TRAIL_LIMIT; opening += 1) state = evolveQuizState(state, { type: "step-opened", step: { screen: "home", page: opening % 2 === 0 ? "board" : "badges" } });
    expect(state.trail.back).toHaveLength(TRAIL_LIMIT);
    let ways = 0;
    for (; stepBefore(state) !== undefined; ways += 1) state = evolveQuizState(state, { type: "step-retraced", to: "back" });
    expect(ways).toBe(TRAIL_LIMIT);
    expect(state.step, "the overview the learner started from is forgotten").toEqual({ screen: "home", page: "badges" });
  });

  it("starts a new trail with every new learner and after the introduction", () => {
    const walked = moved(moved(stateAt({ screen: "home" }), { open: { screen: "home", page: "board" } } as Move), { go: "back" } as Move);
    expect(walked.trail).toEqual({ back: [], forward: [{ screen: "home", page: "board" }] });
    for (const event of [{ type: "learner-forgotten" }, { type: "learner-identified", learner: "b".repeat(32) }, { type: "introduction-read" }] as const) expect(evolveQuizState(walked, event).trail, event.type).toEqual(EMPTY_TRAIL);
    expect(initialQuizState({ introduced: true, learner: undefined, catalog: undefined, learnerView: undefined, runs: {} }).trail).toEqual(EMPTY_TRAIL);
  });

  it("leaves a step behind while its run has not arrived yet, and forgets it once the run turns out to be no place", () => {
    const unknown: QuizStep = { screen: "results", run: "run-unknown" };
    const waiting = moved(moved(stateAt({ screen: "home" }), { open: unknown } as Move), { go: "overview" } as Move);
    expect(stepBefore(waiting)).toEqual(unknown);
    expect(stepAbove({ ...waiting, step: unknown })).toEqual({ screen: "home" });
    expect(stepBefore(evolveQuizState(waiting, { type: "run-loaded", view: runOf("run-unknown", "heating", "open") }))).toBeUndefined();
    expect(stepBefore(evolveQuizState(waiting, { type: "run-loaded", view: runOf("run-unknown", "heating", "submitted") }))).toEqual(unknown);
  });

  it("never leaves the introduction or the identity step behind", () => {
    const first = initialQuizState({ introduced: true, learner: undefined, catalog: CATALOG, learnerView: undefined, runs: {} });
    const again = evolveQuizState(evolveQuizState(first, { type: "step-opened", step: { screen: "introduction" } }), { type: "step-opened", step: { screen: "identity" } });
    expect(again.step).toEqual({ screen: "identity" });
    expect(again.trail).toEqual(EMPTY_TRAIL);
    expect([stepBefore(again), stepAfter(again), stepAbove(again)]).toEqual([undefined, undefined, undefined]);
  });
});

describe("🔗️ the address", () => {
  const pages = homePages(fixture.quizzes);

  for (const vector of fixture.addresses) {
    it(`names the step ${JSON.stringify(vector.step)} as "${vector.hash}"`, () => {
      expect(stepAddress(step(vector.step))).toBe(vector.hash);
    });
  }

  for (const vector of fixture.named) {
    it(`reads "${vector.hash}" as ${vector.page === null ? "no page" : `the page ${vector.page}`}`, () => {
      expect(addressPage(vector.hash, pages) ?? null).toBe(vector.page);
    });
  }

  it("round-trips every page of the overview", () => {
    for (const page of pages) expect(addressPage(stepAddress({ screen: "home", page }), pages)).toBe(page);
  });

  it("names every place a way can lead to, in both languages, and nothing that is no place", () => {
    const state = stateAt({ screen: "home" });
    const names = (locale: "en" | "de") => (shown: Step) => placeName(step(shown), state, locale, quizText(locale));
    expect([{ screen: "home" }, { screen: "home", page: "board" }, { screen: "home", page: "learner" }, { screen: "home", page: "heating" }, { screen: "run", run: "run-open" }, { screen: "results", run: "run-done" }].map(names("en"))).toEqual(["Overview", "Leaderboard", "Ada", "Quiz heating", "Quiz heating", "Results: Quiz cooling"]);
    expect([{ screen: "home" }, { screen: "home", page: "board" }, { screen: "results", run: "run-done" }].map(names("de"))).toEqual(["Übersicht", "Rangliste", "Ergebnisse: Quiz cooling"]);
    expect([{ screen: "home", page: "nowhere" }, { screen: "run", run: "run-unknown" }, { screen: "introduction" }, { screen: "identity" }].map(names("en"))).toEqual([undefined, undefined, undefined, undefined]);
  });
});

describe("🚏️ the navbar", () => {
  it("carries the overview, back, forward and up on its left, what the quizzes are about in its middle and the language on its right", async () => {
    render(app(device()));
    await arrived();
    const bar = navbar();
    const ways = within(bar).getByRole("group", { name: "Go to" });
    expect([...ways.querySelectorAll("button")].map((button) => button.dataset.quizNav)).toEqual([...NAVIGATION_WAYS]);
    expect(within(bar).getAllByRole("button")).toEqual(["Overview", "Back", "Forward", "Up", "English", "Deutsch"].map((name) => within(bar).getByRole("button", { name })));
    expect([...within(bar).getByRole("group", { name: "Language" }).querySelectorAll('button > [aria-hidden="true"]')].map((code) => [code.textContent, code.className])).toEqual([["EN", "md:hidden"], ["DE", "md:hidden"]]);
    expect(way("overview").textContent?.trim()).toBe("Overview");
    for (const name of NAVIGATION_WAYS.slice(1)) expect(way(name).textContent?.trim(), name).toBe("");
    for (const name of NAVIGATION_WAYS) {
      expect(way(name).getAttribute("aria-disabled"), name).toBe("true");
      expect((way(name) as HTMLButtonElement).disabled, name).toBe(false);
      expect(way(name).getAttribute("title"), name).toBe(way(name).getAttribute("aria-label"));
      expect(way(name).querySelector("svg"), name).not.toBeNull();
    }
    const brand = bar.querySelector<HTMLElement>("[data-quiz-brand]")!;
    expect(brand.textContent).toBe("Ways catalog");
    expect(brand.closest('[data-slot="navbar-centered"]')).not.toBeNull();
    expect(brand.closest("button, a")).toBeNull();
    expect(brand.querySelector("svg")?.closest('[aria-hidden="true"]')).not.toBeNull();
    expect(ways.closest('[data-slot="navbar-centered"]')).toBeNull();
    expect(ways.compareDocumentPosition(within(bar).getByRole("group", { name: "Language" })) & Node.DOCUMENT_POSITION_FOLLOWING).toBeTruthy();
    expect(bar.compareDocumentPosition(document.getElementById("quiz-main")!) & Node.DOCUMENT_POSITION_FOLLOWING).toBeTruthy();
    expect(document.querySelector("[data-layered-overview-button]")).toBeNull();
  });

  it("offers no way before a learner is identified, and still says what the quizzes are about", async () => {
    render(app(device(false)));
    await screen.findByRole("heading", { level: 1, name: "How the quizzes work" });
    expect(navbar().querySelector("[data-quiz-nav]")).toBeNull();
    expect(within(navbar()).queryByRole("group", { name: "Go to" })).toBeNull();
    expect(navbar().querySelector("[data-quiz-brand]")?.textContent).toBe("Ways catalog");
  });

  it("leads back, forward, up and to the overview, names where each way leads, and the address follows without a new history entry", { timeout: 30_000 }, async () => {
    const user = userEvent.setup();
    render(app(device()));
    await arrived();
    const entries = window.history.length;

    await user.click(within(screen.getByRole("region", { name: "Leaderboard" })).getByRole("button", { name: "Full leaderboard" }));
    expect(openedPage()).toBe("board");
    expect(pane("board").hasAttribute("inert")).toBe(false);
    expect(window.location.hash).toBe("#board");
    expect(document.title).toBe("Leaderboard · Ways catalog");
    expect([leads("overview"), leads("back"), leads("forward"), leads("up")]).toEqual([["Overview", true], ["Back: Overview", true], ["Forward", false], ["Up: Overview", true]]);
    expect(way("overview").getAttribute("aria-keyshortcuts")).toBe("Escape");

    await user.click(way("back"));
    expect(openedPage()).toBeNull();
    expect(window.location.hash).toBe("");
    expect([leads("overview"), leads("back"), leads("forward"), leads("up")]).toEqual([["Overview", false], ["Back", false], ["Forward: Leaderboard", true], ["Up", false]]);
    expect(way("overview").hasAttribute("aria-keyshortcuts")).toBe(false);

    await user.click(way("forward"));
    expect(openedPage()).toBe("board");
    expect(window.location.hash).toBe("#board");
    fireEvent.keyDown(window, { key: "Escape" });
    expect(openedPage()).toBeNull();
    expect(leads("back")).toEqual(["Back: Leaderboard", true]);
    await user.click(way("back"));
    await user.click(way("up"));
    expect(openedPage()).toBeNull();
    expect(leads("back")).toEqual(["Back: Leaderboard", true]);

    await user.click(within(screen.getByRole("region", { name: "Quiz heating" })).getByRole("button", { name: "Resume quiz" }));
    await screen.findByRole("heading", { level: 1, name: "Quiz heating" });
    expect(document.querySelector("[data-layered-overview]")).toBeNull();
    expect([leads("overview"), leads("back"), leads("up")]).toEqual([["Overview", true], ["Back: Overview", true], ["Up: Quiz heating", true]]);
    expect(way("overview").hasAttribute("aria-keyshortcuts")).toBe(false);
    expect(within(document.getElementById("quiz-main")!).queryByRole("button", { name: "Overview" })).toBeNull();

    await user.click(way("up"));
    expect(openedPage()).toBe("heating");
    expect(window.location.hash).toBe("#heating");
    expect(leads("back")).toEqual(["Back: Quiz heating", true]);
    await user.click(way("back"));
    await screen.findByRole("heading", { level: 1, name: "Quiz heating" });
    expect(window.location.hash).toBe("");
    expect(leads("forward")).toEqual(["Forward: Quiz heating", true]);

    await user.click(way("overview"));
    await screen.findByRole("heading", { level: 1, name: "Quizzes" });
    expect(openedPage()).toBeNull();
    expect(leads("forward")).toEqual(["Forward", false]);
    await user.click(within(screen.getByRole("region", { name: "Quiz cooling" })).getByRole("button", { name: "View last result" }));
    await screen.findByRole("heading", { level: 1, name: "Results: Quiz cooling" });
    expect(leads("up")).toEqual(["Up: Quiz cooling", true]);
    await user.click(way("up"));
    expect(openedPage()).toBe("cooling");
    expect(leads("back")).toEqual(["Back: Results: Quiz cooling", true]);
    expect(window.history.length).toBe(entries);
  });

  it("keeps a way that leads nowhere focusable and does nothing with it", async () => {
    const user = userEvent.setup();
    render(app(device()));
    await arrived();
    for (const name of NAVIGATION_WAYS) {
      act(() => way(name).focus());
      await user.keyboard("{Enter}");
      expect(document.activeElement, name).toBe(way(name));
      expect(openedPage(), name).toBeNull();
    }
    expect(screen.getAllByRole("region").length).toBeGreaterThan(8);
  });

  it("opens the page the address names on arrival instead of the overview, and a hash changed later like any opening", async () => {
    window.history.replaceState(null, "", "#board");
    render(app(device()));
    await waitFor(() => expect(openedPage()).toBe("board"));
    expect(window.location.hash).toBe("#board");
    expect([leads("back"), leads("forward"), leads("up")]).toEqual([["Back", false], ["Forward", false], ["Up: Overview", true]]);

    await userEvent.setup().click(way("up"));
    expect(openedPage()).toBeNull();
    expect(window.location.hash).toBe("");
    expect(leads("back")).toEqual(["Back: Leaderboard", true]);

    act(() => {
      window.location.hash = "#badges";
      window.dispatchEvent(new HashChangeEvent("hashchange"));
    });
    expect(openedPage()).toBe("badges");
    expect(leads("back")).toEqual(["Back: Overview", true]);
    act(() => {
      window.location.hash = "#quiz-main";
      window.dispatchEvent(new HashChangeEvent("hashchange"));
    });
    expect(openedPage()).toBe("badges");
    act(() => {
      window.history.replaceState(null, "", window.location.pathname);
      window.dispatchEvent(new HashChangeEvent("hashchange"));
    });
    expect(openedPage()).toBeNull();
    expect(leads("back")).toEqual(["Back: Badges", true]);
  });

  it("keeps the address through a first visit and opens the page it names once the learner is identified", { timeout: 30_000 }, async () => {
    window.history.replaceState(null, "", "#prefs");
    const origin = memoryStorageOrigin();
    render(app(origin.tab()));
    await screen.findByRole("heading", { level: 1, name: "How the quizzes work" });
    expect(window.location.hash).toBe("#prefs");
    const elsewhere = localStore(origin.tab(), CATALOG.id);
    act(() => {
      elsewhere.write("introduced", true);
      elsewhere.write("learner", { id: LEARNER, identity: IDENTITY });
    });
    await waitFor(() => expect(openedPage()).toBe("prefs"));
    expect(window.location.hash).toBe("#prefs");
    expect([leads("back"), leads("up")]).toEqual([["Back", false], ["Up: Overview", true]]);
  });
});
