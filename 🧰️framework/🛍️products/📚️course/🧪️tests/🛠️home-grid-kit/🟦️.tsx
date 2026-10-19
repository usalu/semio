/** 🛠️ Shared fixtures, stylesheet readers and the home harness of the layered-home suites (the topics sit in the sibling folders `🏠️home-grid`, `🎫️quiz-card-challenges`, `🥇️home-leaderboards` and `📱️home-text-and-viewport`). */
import { useState, type ReactElement } from "react";
import { afterEach, vi } from "vitest";
import { type LayeredMode } from "@semio-tech/ui-react/chrome";
import { LEADERBOARD_TOP, learnerTag, type CourseView, type Leaderboard, type LeaderboardRow, type LearnerView } from "@semio-tech/course";
import { HomeScreen, boardKey, courseText, type CoursePreferences, type CourseSession, type CourseState, type CourseStep, type HomeGrid } from "@semio-tech/course-react";
import grid from "../../🧫️fixtures/🏠️home-grid/🔣️.json";

interface Fixture {
  readonly hierarchy: { readonly topics: CourseView["topics"]; readonly scopes: readonly { readonly page: string | null; readonly pages: readonly string[] }[]; readonly parents: readonly { readonly page: string; readonly parent: string | null }[] };
  readonly breakpoints: { readonly mobileMaxPx: number; readonly tabletMaxPx: number };
  readonly layouts: readonly { readonly width: number; readonly grid: HomeGrid; readonly mode: LayeredMode; readonly columns: readonly number[]; readonly rows: readonly number[] }[];
  readonly heights: { readonly rowHeightPx: number; readonly chromeHeightPx: number; readonly minimum: { readonly desktop: number; readonly tablet: number }; readonly vectors: readonly { readonly width: number; readonly height: number; readonly scale: number; readonly grid: HomeGrid; readonly mode: LayeredMode }[] };
  readonly orders: readonly { readonly quizzes: readonly string[]; readonly pages: readonly string[] }[];
  readonly cells: readonly { readonly quizzes: number; readonly grid: HomeGrid; readonly cells: Readonly<Partial<Record<string, { readonly column: number; readonly row: number }>>> }[];
  readonly excerpts: readonly { readonly id: string; readonly ranks: number; readonly mine: number | null; readonly top: readonly number[]; readonly own: number | null }[];
  readonly board: { readonly sentRows: number; readonly page: readonly { readonly id: string; readonly ranks: number; readonly mine: number | null; readonly rows: number; readonly apart: number | null; readonly count: string; readonly shown: string | null }[] };
}

export const fixture = grid as Fixture;

export const SNOW_EMOJI = String.fromCodePoint(0x2744, 0xfe0f);

export const LEARNER = "a".repeat(32);

const text = (en: string, de: string) => ({ en, de });

export const PREFERENCES: CoursePreferences = { theme: "system", textSize: "normal", showCursors: true, others: "submitted", animateIcons: true, iconsChosen: false, pets: "calm", petsLiveliness: "calm", petsChosen: false, petsPlay: true, petsMischief: true, challenges: {} };

export const PAGES = ["learner", "quizzes", "board", "badges", "prefs"];

export function row(rank: number, tag: string, handle: string): LeaderboardRow {
  return { rank, tag, identity: { kind: "pseudonym", handle }, total: 1000 - rank * 10, reachedAt: rank, best: {}, badges: [], runs: 1, lastActivity: rank };
}

function rows(ranks: number, mine: number | null): readonly LeaderboardRow[] {
  return Array.from({ length: ranks }, (_, index) => (index + 1 === mine ? row(index + 1, learnerTag(LEARNER), "Ada") : row(index + 1, String(index + 1).padStart(8, "0"), `Learner ${index + 1}`)));
}

/** 🏆️ The all-time leaderboard as the proctor answers a learner of rank `mine` among `ranks` ranked learners: the top
 * rows, the counts, and the own row when ranked. */
export function boardOf(ranks: number, mine: number | null): Leaderboard {
  const all = rows(ranks, mine);
  const own = mine === null ? undefined : all[mine - 1];
  return { period: "all-time", rows: all.slice(0, LEADERBOARD_TOP), learners: ranks, submissions: ranks, ...(own === undefined ? {} : { own }) };
}

/** 🗃️ The leaderboards a client holds once the proctor answered `board`. */
export function held(board: Leaderboard, at: number): CourseState["leaderboards"] {
  return { [boardKey(board)]: { board, at } };
}

//#region 🏗️State
export const QUIZZES = ["physics", "heating", "cooling", "demand"] as const;

export const COURSE: CourseView = {
  id: "grid",
  title: text("Grid course", "Rasterkatalog"),
  introduction: { title: text("How the quizzes work", "So funktionieren die Quizze"), paragraphs: [text("Classify, sort and match.", "Klassifiziere, sortiere und ordne zu.")] },
  quizzes: QUIZZES.map((id, index) => ({
    id,
    emoji: ["🧲", "🔥", SNOW_EMOJI, "📊"][index]!,
    title: text(`Quiz ${id}`, `Quiz ${id}`),
    description: text(`About ${id}.`, `Über ${id}.`),
    tasks: [{ id: `${id}-task`, kind: "sorting", title: text("Task", "Aufgabe") }],
  })),
  badges: [
    { id: "heating-expert", emoji: "🔥", label: text("Heating expert", "Heizprofi"), description: text("All right in heating.", "Alles richtig beim Heizen.") },
    { id: "all-done", emoji: "🏁", label: text("All done", "Alles erledigt"), description: text("Every quiz once.", "Jedes Quiz einmal.") },
  ],
};

export const LEARNER_VIEW: LearnerView = {
  learner: LEARNER,
  identity: { kind: "pseudonym", handle: "Ada" },
  runs: [
    { run: "1".repeat(32), quiz: "heating", challenge: "medium", status: "open", startedAt: 10 },
    { run: "2".repeat(32), quiz: "cooling", challenge: "hard", status: "submitted", score: 0.997, points: 299.1, startedAt: 20, submittedAt: 30 },
    { run: "3".repeat(32), quiz: "demand", challenge: "easy", status: "submitted", score: 0.5, points: 50, startedAt: 40, submittedAt: 50 },
    { run: "4".repeat(32), quiz: "demand", challenge: "expert", status: "open", startedAt: 60 },
  ],
  badges: [{ badge: "heating-expert", run: "2".repeat(32), at: 30 }],
  best: { cooling: { challenge: "hard", score: 0.997, points: 299.1 }, demand: { challenge: "easy", score: 0.5, points: 50 } },
  total: 349.1,
};

export const STATE: CourseState = {
  step: { screen: "home" },
  trail: { back: [], forward: [] },
  introduced: true,
  learner: { id: LEARNER, identity: { kind: "pseudonym", handle: "Ada" } },
  course: COURSE,
  learnerView: LEARNER_VIEW,
  runs: {},
  awards: {},
  crowds: {},
  asked: [],
  board: { period: "all-time" },
  leaderboards: held(boardOf(8, 7), Date.UTC(2026, 8, 29, 12, 0)),
  submissions: 8,
};

export function stubSession() {
  return {
    open: vi.fn(),
    chooseBoard: vi.fn(),
    startRun: vi.fn(async () => undefined),
    resumeRun: vi.fn(async () => undefined),
    forgetLearner: vi.fn(),
    refreshLeaderboard: vi.fn(async () => ({ answered: true as const })),
    refreshCrowd: vi.fn(async (quiz: string) => void quiz),
    loadRun: vi.fn(async () => undefined),
  };
}
//#endregion 🏗️State

/** 📱️ Emulates a viewport `width` px wide and `height` px high (by default high enough for every grid) for the size
 * media queries (`min-`/`max-` `width`/`height` in px, joined by `and`); every other query is false. */
export function atViewport(width: number, height = 1200): void {
  vi.stubGlobal("matchMedia", (query: string) => ({
    matches: query.split(" and ").every((part) => {
      const found = /^\((min|max)-(width|height):\s*(\d+)px\)$/u.exec(part.trim());
      const size = found?.[2] === "height" ? height : width;
      return found !== null && (found[1] === "min" ? size >= Number(found[3]) : size <= Number(found[3]));
    }),
    media: query,
    onchange: null,
    addEventListener: () => undefined,
    removeEventListener: () => undefined,
    addListener: () => undefined,
    removeListener: () => undefined,
    dispatchEvent: () => false,
  }));
}


//#region 🥞️Overview
/** 🥞️ Home over a session double whose `open` really moves the step, so the overview's open page follows it. */
export function Home(props: { readonly session: ReturnType<typeof stubSession>; readonly initial?: CourseStep; readonly preferences?: CoursePreferences; readonly course?: CourseView }): ReactElement {
  const [step, setStep] = useState<CourseStep>(props.initial ?? { screen: "home" });
  const [session] = useState(() => ({
    ...props.session,
    open: (next: CourseStep) => {
      props.session.open(next);
      setStep(next);
    },
  }));
  return <HomeScreen session={session as unknown as CourseSession} state={{ ...STATE, step, course: props.course ?? COURSE }} text={courseText("en")} locale="en" preferences={props.preferences ?? PREFERENCES} onPreferences={() => undefined} />;
}

export function pane(page: string): HTMLElement {
  return document.querySelector<HTMLElement>(`[data-layered-pane="${page}"]`)!;
}

export function cardHost(page: string): HTMLElement {
  return document.querySelector<HTMLElement>(`[data-layered-card="${page}"]`)!;
}
//#endregion 🥞️Overview

afterEach(() => {
  window.history.replaceState(null, "", window.location.pathname);
  vi.useRealTimers();
  vi.unstubAllGlobals();
  vi.restoreAllMocks();
});
