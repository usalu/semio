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
import { afterEach, beforeAll, describe, expect, it } from "vitest";
import { decodeQueryEnvelope, encodeQueryResult, type HttpResponse, type HttpTransport } from "@semio-tech/framework-server";
import type { CourseView, LearnerView, Query, RunStatus, RunView } from "@semio-tech/course";
import {
  EMPTY_TRAIL,
  NAVIGATION_WAYS,
  CourseApp,
  TRAIL_LIMIT,
  addressPage,
  addressPages,
  evolveCourseState,
  evolveCourseState,
  initialCourseState,
  localStore,
  memoryStorageOrigin,
  navigationWays,
  placeName,
  courseInstance,
  courseText,
  sameStep,
  stepAbove,
  stepAddress,
  stepAfter,
  stepBefore,
  type PresenceConnect,
  type CourseState,
  type CourseStep,
  type StorageArea,
} from "@semio-tech/course-react";
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
  readonly hierarchy: { readonly topics: NonNullable<CourseView["topics"]>; readonly parents: readonly { readonly step: Step; readonly up: Step | null }[] };
}

const fixture: Fixture = navigation;
const LEARNER = "a".repeat(32);
const IDENTITY = { kind: "pseudonym", handle: "Ada" } as const;
const text = (en: string, de: string) => ({ en, de });
const step = (vector: Step): CourseStep => vector as CourseStep;

const COURSE: CourseView = {
  id: "ways",
  title: text("Ways course", "Wegekatalog"),
  introduction: { title: text("How the quizzes work", "So funktionieren die Quizze"), paragraphs: [text("Classify, sort and match.", "Klassifiziere, sortiere und ordne zu.")] },
  quizzes: fixture.quizzes.map((id) => ({ id, emoji: "🧲", title: text(`Quiz ${id}`, `Quiz ${id}`), description: text(`About ${id}.`, `Über ${id}.`), tasks: [] })),
  badges: [],
};

/** 🏃️ A run of `quiz` without tasks: open, submitted with a perfect result, or voided. */
function runOf(run: string, quiz: string, status: RunStatus): RunView {
  const sheet = { quiz, seed: 1, challenge: "medium" as const, title: text(`Quiz ${quiz}`, `Quiz ${quiz}`), description: text(`About ${quiz}.`, `Über ${quiz}.`), tasks: [] };
  return { run, learner: LEARNER, quiz, status, sheet, answers: {}, startedAt: 1, ...(status === "submitted" ? { result: { quiz, challenge: "medium", score: 1, points: 200, tasks: [] }, submittedAt: 2 } : {}) };
}

/** 🧭️ The state of an identified learner at `start` with the fixture's runs and a trail of its own. */
function stateAt(start: Step): CourseState {
  const runs = Object.fromEntries(Object.entries(fixture.runs).map(([run, { quiz, status }]) => [run, runOf(run, quiz, status as RunStatus)]));
  return { ...initialCourseState({ introduced: true, learner: { id: LEARNER, identity: IDENTITY }, course: COURSE, learnerView: undefined, runs }), step: step(start) };
}

/** 👣️ The state after one move of a vector. */
function moved(state: CourseState, move: Move): CourseState {
  if (move.open !== undefined) return evolveCourseState(state, { type: "step-opened", step: step(move.open) });
  if (move.enter !== undefined) return evolveCourseState(state, { type: "step-opened", step: step(move.enter), instead: true });
  if (move.submit !== undefined) return evolveCourseState(state, { type: "run-submitted", run: move.submit, result: { quiz: fixture.runs[move.submit]!.quiz, challenge: "medium", score: 1, points: 200, tasks: [] }, badges: [], at: 2 });
  if (move.close !== undefined) return evolveCourseState(state, { type: "run-loaded", view: runOf(move.close, fixture.runs[move.close]!.quiz, move.as as RunStatus) });
  if (move.void !== undefined) return evolveCourseState(state, { type: "run-voided", run: move.void });
  if (move.go === "back" || move.go === "forward") return evolveCourseState(state, { type: "step-retraced", to: move.go });
  const target = move.go === "up" ? stepAbove(state) : ({ screen: "home" } as const);
  return target === undefined ? state : evolveCourseState(state, { type: "step-opened", step: target });
}

/** 🔮️ A session history of `jsdom` in a window of its own, holding `start`: the oracle a trail without closed runs is
 * replayed on. A step is pushed, replaced or traversed to as a browser does it; what lies one entry behind and ahead
 * is read by going there and back. */
async function sessionHistory(start: CourseStep) {
  const { window } = new JSDOM("", { url: "https://quiz.example/" });
  let entries = 0;
  const entry = (shown: CourseStep) => ({ step: shown, entry: entries++ });
  const here = () => window.history.state as { readonly step: CourseStep; readonly entry: number };
  const settled = async (): Promise<void> => {
    for (let turn = 0; turn < 3; turn += 1) await new Promise((resolve) => window.setTimeout(resolve, 0));
  };
  const travel = async (delta: -1 | 1): Promise<boolean> => {
    const from = here().entry;
    window.history.go(delta);
    await settled();
    return here().entry !== from;
  };
  const beside = async (delta: -1 | 1): Promise<CourseStep | null> => {
    if (!(await travel(delta))) return null;
    const found = here().step;
    await travel(delta === 1 ? -1 : 1);
    return found;
  };
  window.history.replaceState(entry(start), "");
  return {
    open: (shown: CourseStep): void => void (sameStep(shown, here().step) || window.history.pushState(entry(shown), "")),
    enter: (shown: CourseStep): void => window.history.replaceState(entry(shown), ""),
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
    { run: RUN_OPEN, quiz: "heating", challenge: "medium", status: "open", startedAt: 1 },
    { run: RUN_DONE, quiz: "cooling", challenge: "medium", status: "submitted", score: 1, points: 200, startedAt: 1, submittedAt: 2 },
  ],
  badges: [],
  best: { cooling: { challenge: "medium", score: 1, points: 200 } },
  total: 200,
};
const TIMING = { minMs: 1, maxMs: 4 };
const QUIET_PRESENCE: PresenceConnect = () => ({ readyState: 0, onopen: null, onmessage: null, onclose: null, onerror: null, send: () => undefined, close: () => undefined });
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
    if (request.method === "GET" && request.path === "/instance") return reply(200, courseInstance());
    if (request.path !== "/queries") return reply(404, { kind: "notFound", message: request.path });
    const query = JSON.parse(decoder.decode(decodeQueryEnvelope(JSON.parse(typeof request.body === "string" ? request.body : decoder.decode(request.body)) as unknown).arguments)) as Query;
    const answer = query.type === "course" ? COURSE : query.type === "learner" ? LEARNER_VIEW : query.type === "run" ? RUNS[query.run] : query.type === "leaderboard" ? { rows: [], learners: 0 } : query.type === "crowd" ? { quiz: query.quiz, runs: 0, tasks: [] } : undefined;
    return answer === undefined ? reply(404, { kind: "notFound", message: query.type }) : reply(200, encodeQueryResult({ kind: "snapshot", value: encoder.encode(JSON.stringify(answer)), frontier: null }));
  },
};

/** 🧑‍🎓️ The storage of a device whose learner is identified, or of one that visits for the first time. */
function device(identified = true): StorageArea {
  const area = memoryStorageOrigin().tab();
  if (!identified) return area;
  const store = localStore(area, COURSE.id);
  store.write("introduced", true);
  store.write("learner", { id: LEARNER, identity: IDENTITY });
  return area;
}

function app(storage: StorageArea) {
  return <CourseApp proctor="" tenant={COURSE.id} logo={LOGO} presence={QUIET_PRESENCE} transport={() => PROCTOR} storage={storage} languages={["en"]} timing={TIMING} />;
}

function navbar(): HTMLElement {
  return screen.getByRole("navigation", { name: "Main navigation" });
}

function way(name: (typeof NAVIGATION_WAYS)[number]): HTMLElement {
  return navbar().querySelector<HTMLElement>(`[data-course-nav="${name}"]`)!;
}

function leads(name: (typeof NAVIGATION_WAYS)[number]): readonly [string | null, boolean] {
  return [way(name).getAttribute("aria-label"), way(name).getAttribute("aria-disabled") === "false"];
}

function pane(page: string): HTMLElement {
  return document.querySelector<HTMLElement>(`[data-layered-pane="${page}"]`)!;
}

function openedPage(): string | null {
  return [...document.querySelectorAll<HTMLElement>("[data-layered-pane][data-opened]")].at(-1)?.dataset.layeredPane ?? null;
}

/** 🧭️ Opens a named page through the same address channel as a direct link. */
function openAddress(page: string): void {
  act(() => {
    window.history.replaceState(null, "", `#${page}`);
    window.dispatchEvent(new HashChangeEvent("hashchange"));
  });
}

async function arrived(): Promise<void> {
  await screen.findByRole("heading", { level: 1, name: "Course" });
  await act(async () => undefined);
}
//#endregion 🛂️Client

afterEach(() => {
  window.history.replaceState(null, "", window.location.pathname);
});

beforeAll(async () => {
  await Promise.all([import("@semio-tech/course-react/modules/run/.tsx"), import("@semio-tech/course-react/modules/results/.tsx")]);
}, 180_000);

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
    for (let opening = 0; opening < 2 * TRAIL_LIMIT; opening += 1) state = evolveCourseState(state, { type: "step-opened", step: { screen: "home", page: opening % 2 === 0 ? "board" : "badges" } });
    expect(state.trail.back).toHaveLength(TRAIL_LIMIT);
    let ways = 0;
    for (; stepBefore(state) !== undefined; ways += 1) state = evolveCourseState(state, { type: "step-retraced", to: "back" });
    expect(ways).toBe(TRAIL_LIMIT);
    expect(state.step, "the overview the learner started from is forgotten").toEqual({ screen: "home", page: "badges" });
  });

  it("starts a new trail with every new learner and preserves it after the introduction", () => {
    const walked = moved(moved(stateAt({ screen: "home" }), { open: { screen: "home", page: "board" } } as Move), { go: "back" } as Move);
    expect(walked.trail).toEqual({ back: [], forward: [{ screen: "home", page: "board" }] });
    for (const event of [{ type: "learner-forgotten" }, { type: "learner-identified", learner: "b".repeat(32) }] as const) expect(evolveCourseState(walked, event).trail, event.type).toEqual(EMPTY_TRAIL);
    expect(evolveCourseState(walked, { type: "introduction-read" }).trail).toEqual(walked.trail);
    expect(initialCourseState({ introduced: true, learner: undefined, course: undefined, learnerView: undefined, runs: {} }).trail).toEqual(EMPTY_TRAIL);
  });

  it("leaves a step behind while its run has not arrived yet, and forgets it once the run turns out to be no place", () => {
    const unknown: CourseStep = { screen: "results", run: "run-unknown" };
    const waiting = moved(moved(stateAt({ screen: "home" }), { open: unknown } as Move), { go: "overview" } as Move);
    expect(stepBefore(waiting)).toEqual(unknown);
    expect(stepAbove({ ...waiting, step: unknown })).toEqual({ screen: "home" });
    expect(stepBefore(evolveCourseState(waiting, { type: "run-loaded", view: runOf("run-unknown", "heating", "open") }))).toBeUndefined();
    expect(stepBefore(evolveCourseState(waiting, { type: "run-loaded", view: runOf("run-unknown", "heating", "submitted") }))).toEqual(unknown);
  });

  it("never leaves the introduction or the identity step behind", () => {
    const first = initialCourseState({ introduced: true, learner: undefined, course: COURSE, learnerView: undefined, runs: {} });
    const again = evolveCourseState(evolveCourseState(first, { type: "step-opened", step: { screen: "introduction" } }), { type: "step-opened", step: { screen: "identity" } });
    expect(again.step).toEqual({ screen: "identity" });
    expect(again.trail).toEqual({ back: [{ screen: "home" }], forward: [] });
    expect([stepBefore(again), stepAfter(again), stepAbove(again)]).toEqual([{ screen: "home" }, undefined, undefined]);
  });
});

describe("🔗️ the address", () => {
  const pages = addressPages(COURSE);

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

  it("opens each nested address and returns to its containing grid", () => {
    const course = { ...COURSE, topics: fixture.hierarchy.topics };
    const pages = addressPages(course);
    for (const vector of fixture.hierarchy.parents) {
      const shown = step(vector.step);
      expect(stepAbove({ ...stateAt(vector.step), course }) ?? null).toEqual(vector.up);
      if (shown.screen === "home" && shown.page !== undefined) expect(addressPage(stepAddress(shown), pages)).toBe(shown.page);
    }
    expect(placeName({ screen: "home", page: "energy" }, { ...stateAt({ screen: "home" }), course }, "de", courseText("de"))).toBe("Energie");
  });

  it("names every place a way can lead to, in both languages, and nothing that is no place", () => {
    const state = stateAt({ screen: "home" });
    const names = (locale: "en" | "de") => (shown: Step) => placeName(step(shown), state, locale, courseText(locale));
    expect([{ screen: "home" }, { screen: "home", page: "board" }, { screen: "home", page: "learner" }, { screen: "home", page: "heating" }, { screen: "run", run: "run-open" }, { screen: "results", run: "run-done" }].map(names("en"))).toEqual(["Course overview", "Leaderboard", "Ada", "Quiz heating", "Quiz heating (Medium)", "Results: Quiz cooling"]);
    expect([{ screen: "home" }, { screen: "home", page: "board" }, { screen: "run", run: "run-open" }, { screen: "results", run: "run-done" }].map(names("de"))).toEqual(["Kursübersicht", "Rangliste", "Quiz heating (Mittel)", "Ergebnisse: Quiz cooling"]);
    const hard = { ...state, runs: Object.fromEntries(Object.entries(state.runs).map(([run, view]) => [run, { ...view, sheet: { ...view.sheet, challenge: "expert" as const } }])) };
    expect(placeName(step({ screen: "run", run: "run-open" }), hard, "en", courseText("en"))).toBe("Quiz heating (Expert)");
    expect([{ screen: "home", page: "nowhere" }, { screen: "run", run: "run-unknown" }, { screen: "introduction" }, { screen: "identity" }].map(names("en"))).toEqual([undefined, undefined, undefined, undefined]);
  });
});

describe("🚏️ the navbar", () => {
  it("carries the overview, back, forward and up on its left, what the quizzes are about in its middle and the language on its right", async () => {
    render(app(device()));
    await arrived();
    const bar = navbar();
    const ways = within(bar).getByRole("group", { name: "Go to" });
    expect([...ways.querySelectorAll("button")].map((button) => button.dataset.courseNav)).toEqual([...NAVIGATION_WAYS]);
    expect(within(bar).getAllByRole("button")).toEqual(["Course overview", "Back", "Forward", "Up", "Introduction", "English", "Deutsch"].map((name) => within(bar).getByRole("button", { name })));
    const languageButtons = [...within(bar).getByRole("group", { name: "Language" }).querySelectorAll("button")];
    expect(languageButtons.map((button) => button.querySelector("[data-icon]")?.getAttribute("data-icon"))).toEqual(["globe", "globe"]);
    expect(languageButtons.map((button) => button.querySelector('span[class~="md:hidden"]')?.textContent)).toEqual(["EN", "DE"]);
    expect(NAVIGATION_WAYS.map((name) => way(name).textContent?.replace(/\s+/g, " ").trim())).toEqual(["Course overview", "Back", "Forward", "Up"]);
    expect(NAVIGATION_WAYS.map((name) => way(name).querySelector("[data-icon]")?.getAttribute("data-icon"))).toEqual(["layout-grid", "arrow-left", "arrow-right", "arrow-up"]);
    for (const name of NAVIGATION_WAYS) {
      expect(way(name).getAttribute("aria-disabled"), name).toBe("true");
      expect((way(name) as HTMLButtonElement).disabled, name).toBe(false);
      expect(way(name).getAttribute("title"), name).toBe(way(name).getAttribute("aria-label"));
      expect(way(name).querySelector("svg"), name).not.toBeNull();
    }
    const brand = bar.querySelector<HTMLElement>("[data-course-brand]")!;
    expect(brand.textContent).toBe("Ways course");
    expect(brand.closest('[data-slot="navbar-centered"]')).not.toBeNull();
    expect(brand.closest("button, a")).toBeNull();
    expect(brand.querySelector("svg")?.closest('[aria-hidden="true"]')).not.toBeNull();
    expect(ways.closest('[data-slot="navbar-centered"]')).toBeNull();
    expect(ways.compareDocumentPosition(within(bar).getByRole("group", { name: "Language" })) & Node.DOCUMENT_POSITION_FOLLOWING).toBeTruthy();
    expect(bar.compareDocumentPosition(document.getElementById("course-main")!) & Node.DOCUMENT_POSITION_FOLLOWING).toBeTruthy();
    expect(document.querySelector("[data-layered-overview-button]")).toBeNull();
  });

  it("offers navigation before a learner is identified, and still says what the quizzes are about", async () => {
    render(app(device(false)));
    await screen.findByRole("heading", { level: 1, name: "Course" });
    expect(navbar().querySelectorAll("[data-course-nav]")).toHaveLength(4);
    expect(within(navbar()).getByRole("group", { name: "Go to" })).toBeTruthy();
    expect(navbar().querySelector("[data-course-brand]")?.textContent).toBe("Ways course");
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
    expect(document.title).toBe("Leaderboard · Ways course");
    expect([leads("overview"), leads("back"), leads("forward"), leads("up")]).toEqual([["Course overview", true], ["Back: Course overview", true], ["Forward", false], ["Up: Course overview", true]]);
    expect(way("up").getAttribute("aria-keyshortcuts")).toBe("Escape");

    await user.click(way("back"));
    expect(openedPage()).toBeNull();
    expect(window.location.hash).toBe("");
    expect([leads("overview"), leads("back"), leads("forward"), leads("up")]).toEqual([["Course overview", false], ["Back", false], ["Forward: Leaderboard", true], ["Up", false]]);
    expect(way("up").hasAttribute("aria-keyshortcuts")).toBe(false);

    await user.click(way("forward"));
    expect(openedPage()).toBe("board");
    expect(window.location.hash).toBe("#board");
    fireEvent.keyDown(window, { key: "Escape" });
    expect(openedPage()).toBeNull();
    expect(document.activeElement).toBe(document.querySelector('[data-layered-card="board"] a[href="#board"]'));
    expect(leads("back")).toEqual(["Back: Leaderboard", true]);
    await user.click(way("back"));
    await user.click(way("up"));
    expect(openedPage()).toBeNull();
    expect(leads("back")).toEqual(["Back: Leaderboard", true]);

    openAddress("quizzes");
    await user.click(within(screen.getByRole("region", { name: "Quiz heating" })).getByRole("button", { name: "Resume (Medium)" }));
    await screen.findByRole("heading", { level: 1, name: "Quiz heating" });
    expect(document.querySelector("[data-layered-overview]")).toBeNull();
    expect([leads("overview"), leads("back"), leads("up")]).toEqual([["Course overview", true], ["Back: Quizzes", true], ["Up: Quiz heating", true]]);
    expect(way("overview").hasAttribute("aria-keyshortcuts")).toBe(false);
    expect(within(document.getElementById("course-main")!).queryByRole("button", { name: "Course overview" })).toBeNull();

    await user.click(way("up"));
    expect(openedPage()).toBe("heating");
    expect(window.location.hash).toBe("#heating");
    expect(leads("back")).toEqual(["Back: Quiz heating (Medium)", true]);
    await user.click(way("back"));
    await screen.findByRole("heading", { level: 1, name: "Quiz heating" });
    expect(window.location.hash).toBe("");
    expect(leads("forward")).toEqual(["Forward: Quiz heating", true]);

    await user.click(way("overview"));
    await screen.findByRole("heading", { level: 1, name: "Course" });
    expect(openedPage()).toBeNull();
    expect(leads("forward")).toEqual(["Forward", false]);
    openAddress("quizzes");
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
    expect(within(document.querySelector<HTMLElement>('[data-course-grid="root"] > [data-layered-overview] > [data-layered-overlay]')!).getAllByRole("region")).toHaveLength(5);
  });

  it("opens the page the address names on arrival instead of the overview, and a hash changed later like any opening", async () => {
    window.history.replaceState(null, "", "#board");
    render(app(device()));
    await waitFor(() => expect(openedPage()).toBe("board"));
    expect(window.location.hash).toBe("#board");
    expect([leads("back"), leads("forward"), leads("up")]).toEqual([["Back", false], ["Forward", false], ["Up: Course overview", true]]);

    await userEvent.setup().click(way("up"));
    expect(openedPage()).toBeNull();
    expect(window.location.hash).toBe("");
    expect(leads("back")).toEqual(["Back: Leaderboard", true]);

    act(() => {
      window.location.hash = "#badges";
      window.dispatchEvent(new HashChangeEvent("hashchange"));
    });
    expect(openedPage()).toBe("badges");
    expect(leads("back")).toEqual(["Back: Course overview", true]);
    act(() => {
      window.location.hash = "#course-main";
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

  it("opens an addressed page on the first visit before a learner is identified", { timeout: 30_000 }, async () => {
    window.history.replaceState(null, "", "#prefs");
    const origin = memoryStorageOrigin();
    render(app(origin.tab()));
    await waitFor(() => expect(openedPage()).toBe("prefs"));
    expect(window.location.hash).toBe("#prefs");
    expect([leads("back"), leads("up")]).toEqual([["Back", false], ["Up: Course overview", true]]);
  });
});
