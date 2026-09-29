/** 💭️ What the others think in the web client (design §17): the crowd to show (live while anyone else thinks along,
 * else the submitted runs), its items keyed by item id with choices by count and then numerically, the places of a
 * sorting, the drafts a learner shares, and how the task views, the run, the results and a quiz's page show it — with a
 * sentence for assistive technology, never a live region — or hide it at the learner's wish. The shared vectors are
 * judged a second time by `lodash` (`groupBy`, `orderBy`, `mean`, `round`) recomputing the choices from the raw drafts
 * and counts, the mean positions and the places.
 *
 * @see ../../🧫️fixtures/💭️crowd-client/🔣️.json
 * @see ../../🎯️targets/⚛️react/🔨️modules/🗳️crowd/🟦️.tsx
 */

import { fireEvent, render, screen, within } from "@testing-library/react";
import groupBy from "lodash/groupBy";
import mean from "lodash/mean";
import orderBy from "lodash/orderBy";
import round from "lodash/round";
import sortBy from "lodash/sortBy";
import type { ReactElement, ReactNode } from "react";
import { describe, expect, it, vi } from "vitest";
import {
  scoreRun,
  thinkingCrowd,
  thinkingScope,
  type CatalogView,
  type CrowdView,
  type Quiz,
  type RunView,
  type Sheet,
  type SheetClassificationTask,
  type SheetMatchingTask,
  type SheetSortingTask,
  type SheetTask,
  type ThinkingState,
} from "@semio-tech/quiz";
import {
  ClassificationTaskView,
  CrowdProvider,
  EMPTY_PRESENCE_VIEW,
  MatchingTaskView,
  PreferencesPanel,
  PresenceProvider,
  QuizPage,
  ResultsScreen,
  RunScreen,
  SortingTaskView,
  chooseCrowd,
  crowdPlace,
  formatQuantity,
  liveItems,
  localStore,
  meanPosition,
  memoryStorageOrigin,
  presenceDrafts,
  quizText,
  readPreferences,
  submittedItems,
  type QuizLocale,
  type QuizSession,
  type QuizState,
} from "@semio-tech/quiz-react";
import vectors from "../../🧫️fixtures/💭️crowd-client/🔣️.json";

//#region 🏗️Fixtures
const QUIZ = vectors.quiz as unknown as Quiz;
const SHEET = vectors.sheet as unknown as Sheet;
const OTHERS = vectors.others as unknown as readonly ThinkingState[];
const VIEW = vectors.view as unknown as CrowdView;
const COLOURS = new Map([
  ["0000000a", 3],
  ["0000000b", 5],
  ["0000000c", 7],
]);
const LEARNER = "a".repeat(32);
const RUN = "1".repeat(32);
const text = (en: string, de: string) => ({ en, de });

const CATALOG: CatalogView = {
  id: "arch",
  title: text("Architecture", "Architektur"),
  introduction: { title: text("How it works", "So geht's"), paragraphs: [text("Classify, sort and match.", "Klassifiziere, sortiere und ordne zu.")] },
  quizzes: [{ id: QUIZ.id, emoji: QUIZ.emoji, title: QUIZ.title, description: QUIZ.description, tasks: QUIZ.tasks.map((task) => ({ id: task.id, kind: task.kind, title: task.title })) }],
  badges: [],
};

const ANSWERS = {
  heat: { kind: "classification", assignments: { kettle: "power", fridge: "power", candle: "power" } },
  masses: { kind: "sorting", order: ["mouse", "cat", "horse"] },
  lamps: { kind: "matching", assignments: { power: { led: 1, halogen: 0, floodlight: 2 } } },
} as const satisfies RunView["answers"];

function run(status: "open" | "submitted"): RunView {
  const base = { run: RUN, learner: LEARNER, quiz: QUIZ.id, sheet: SHEET, answers: ANSWERS, startedAt: 10 };
  return status === "open" ? { ...base, status } : { ...base, status, result: scoreRun(QUIZ, SHEET, ANSWERS)!, submittedAt: 20 };
}

function state(status: "open" | "submitted", crowds: QuizState["crowds"] = {}): QuizState {
  return {
    step: status === "open" ? { screen: "run", run: RUN } : { screen: "results", run: RUN },
    introduced: true,
    learner: { id: LEARNER, identity: { kind: "pseudonym", handle: "Ada" } },
    catalog: CATALOG,
    learnerView: { learner: LEARNER, identity: { kind: "pseudonym", handle: "Ada" }, runs: [{ run: RUN, quiz: QUIZ.id, status, startedAt: 10 }], badges: [], best: {}, total: 0 },
    runs: { [RUN]: run(status) },
    awards: {},
    crowds,
  };
}

function stubSession() {
  return { open: vi.fn(), answer: vi.fn(), startRun: vi.fn(async () => undefined), resumeRun: vi.fn(async () => undefined), loadRun: vi.fn(async () => undefined), submitRun: vi.fn(async () => undefined) } as unknown as QuizSession;
}

function sheetTask<K extends SheetTask["kind"]>(id: string): Extract<SheetTask, { kind: K }> {
  return SHEET.tasks.find((task) => task.id === id) as Extract<SheetTask, { kind: K }>;
}

function thinkingAlong(others: readonly ThinkingState[], children: ReactNode): ReactElement {
  return (
    <PresenceProvider view={{ ...EMPTY_PRESENCE_VIEW, colours: COLOURS, thinking: new Map([[thinkingScope(CATALOG.id, QUIZ.id), others]]) }} setTask={() => undefined}>
      {children}
    </PresenceProvider>
  );
}

function sentence(item: string): string | null | undefined {
  return document.querySelector(`[data-crowd-item="${item}"] .sr-only`)?.textContent;
}

function chips(item: string): readonly string[] {
  return [...document.querySelectorAll(`[data-crowd-item="${item}"] > span[aria-hidden="true"].border`)].map((chip) => chip.textContent ?? "");
}

function outsideLiveRegions(): void {
  const shown = document.querySelectorAll("[data-crowd-item], [data-crowd-source]");
  for (const element of shown) expect(element.closest('[aria-live], [role="status"], [role="alert"], [role="log"]')).toBeNull();
}
//#endregion 🏗️Fixtures

//#region ⚖️Oracle
const numeric = (key: string): number => (Number.isFinite(Number(key)) ? Number(key) : 0);

/** ⚖️ The choices of an item recomputed from the raw drafts of the others by lodash: grouped by key, ordered by count
 * descending, then numerically, then by code point. */
function draftedChoices(task: SheetTask, item: string, dimension: string | undefined): readonly { readonly key: string; readonly count: number; readonly tags: readonly string[] }[] {
  const drafted = OTHERS.flatMap((other) => {
    const answer = other.answers[task.id];
    const key = answer?.kind === "classification" ? answer.assignments[item] : answer?.kind === "matching" && dimension !== undefined ? answer.values[dimension]?.[item] : undefined;
    return key === undefined ? [] : [{ tag: other.tag, key: String(key) }];
  });
  const grouped = Object.entries(groupBy(drafted, "key")).map(([key, entries]) => ({ key, count: entries.length, tags: sortBy(entries.map((entry) => entry.tag)) }));
  return orderBy(grouped, ["count", (choice) => numeric(choice.key), "key"], ["desc", "asc", "asc"]);
}
//#endregion ⚖️Oracle

describe("🗳️ the crowd to show", () => {
  for (const vector of vectors.live) {
    it(`gathers the live crowd: ${vector.id}`, () => {
      const task = sheetTask(vector.task);
      const items = [...liveItems(thinkingCrowd(OTHERS, task), vector.dimension ?? undefined).values()];
      expect(items).toEqual(vector.expected);
      for (const item of items) {
        if (task.kind !== "sorting") expect(item.choices).toEqual(draftedChoices(task, item.item, vector.dimension ?? undefined));
        else expect(meanPosition(item)).toBeCloseTo(mean(item.positions.map((entry) => entry.position)), 12);
      }
    });
  }

  for (const vector of vectors.submitted) {
    it(`reads the submitted crowd: ${vector.id}`, () => {
      const items = [...submittedItems(VIEW, vector.task, vector.dimension ?? undefined).values()];
      expect(items).toEqual(vector.expected);
      const counted = VIEW.tasks.find((task) => task.task === vector.task && task.dimension === (vector.dimension ?? undefined))?.items ?? [];
      for (const item of items) {
        const counts = counted.find((entry) => entry.item === item.item)?.counts ?? [];
        expect(item.choices.map(({ key, count }) => ({ key, count }))).toEqual(orderBy(counts, ["count", (count) => numeric(count.key), "key"], ["desc", "asc", "asc"]));
      }
    });
  }

  for (const vector of vectors.choices) {
    it(`chooses the crowd: ${vector.id}`, () => {
      const crowd = chooseCrowd(
        vector.others.map((index) => OTHERS[index]!),
        vector.runs === null ? undefined : { ...VIEW, runs: vector.runs },
      );
      expect(crowd === undefined ? null : { source: crowd.source, count: crowd.count }).toEqual(vector.expected);
    });
  }

  for (const vector of vectors.places) {
    it(`places position ${vector.position} of ${vector.total} items at ${vector.place}`, () => {
      expect(crowdPlace(vector.position, vector.total)).toBe(vector.place);
      expect(round(1 + vector.position * Math.max(0, vector.total - 1), 1)).toBe(vector.place);
    });
  }

  for (const vector of vectors.drafts) {
    it(`shares the drafts of the run on screen: ${vector.id}`, () => {
      const view = { ...run("open"), status: vector.status, answers: vector.answers } as unknown as RunView;
      expect(presenceDrafts(view) ?? null).toEqual(vector.expected);
    });
  }
});

describe("🧩️ what the others think in the task views", () => {
  const live = chooseCrowd(OTHERS, VIEW)!;

  it("shows where the others put every item of a classification, most given first, in their colours, with a sentence", () => {
    render(
      <CrowdProvider crowd={live} colours={COLOURS}>
        <ClassificationTaskView task={sheetTask<"classification">("heat") as SheetClassificationTask} answer={undefined} onAnswer={() => undefined} text={quizText("en")} locale="en" />
      </CrowdProvider>,
    );
    expect(sentence("kettle")).toBe("The others on Kettle: Power 2×, Energy 1×");
    expect(sentence("fridge")).toBe("The others on Fridge: Energy 1×, Power 1×");
    expect(sentence("candle")).toBe("The others on Tea light: Power 1×");
    expect(chips("kettle")).toEqual(["Power 2×", "Energy 1×"]);
    const dots = [...document.querySelectorAll<HTMLElement>('[data-crowd-item="kettle"] > span.border')[0]!.querySelectorAll<HTMLElement>(".quiz-online")].map((dot) => dot.style.getPropertyValue("--quiz-peer"));
    expect(dots).toEqual(["var(--presence-3)", "var(--presence-5)"]);
    for (const item of ["kettle", "fridge", "candle"]) expect(document.querySelector(`[data-quiz-item="${item}"]`)?.getAttribute("data-presence-anchor")).toBe(`item:${item}`);
    expect(document.querySelector('[data-presence-anchor="category:power"]')).not.toBeNull();
    outsideLiveRegions();
  });

  it("says it in German too", () => {
    render(
      <CrowdProvider crowd={live} colours={COLOURS}>
        <ClassificationTaskView task={sheetTask<"classification">("heat") as SheetClassificationTask} answer={undefined} onAnswer={() => undefined} text={quizText("de")} locale="de" />
      </CrowdProvider>,
    );
    expect(sentence("kettle")).toBe("Die anderen bei Wasserkocher: Leistung 2×, Energie 1×");
  });

  it("marks where the others place every item of a sorting and says the average place", () => {
    render(
      <CrowdProvider crowd={live} colours={COLOURS}>
        <SortingTaskView task={sheetTask<"sorting">("masses") as SheetSortingTask} answer={undefined} onAnswer={() => undefined} text={quizText("en")} locale="en" />
      </CrowdProvider>,
    );
    expect(sentence("horse")).toBe("On average the others put Horse at place 3 of 3");
    expect(sentence("mouse")).toBe("On average the others put Mouse at place 1.5 of 3");
    const markers = [...document.querySelectorAll<HTMLElement>('[data-crowd-item="mouse"] .quiz-crowd-marker')].map((marker) => [marker.style.getPropertyValue("--quiz-crowd-at"), marker.style.getPropertyValue("--quiz-peer")]);
    expect(markers).toEqual([
      ["0", "var(--presence-3)"],
      ["0.5", "var(--presence-5)"],
    ]);
    expect(document.querySelector('[data-crowd-item="mouse"] .quiz-crowd-track')?.getAttribute("aria-hidden")).toBe("true");
    outsideLiveRegions();
  });

  it("names the values the others match with every item, smallest first among equals", () => {
    const task = sheetTask<"matching">("lamps") as SheetMatchingTask;
    render(
      <CrowdProvider crowd={live} colours={COLOURS}>
        <MatchingTaskView task={task} answer={undefined} onAnswer={() => undefined} text={quizText("en")} locale="en" />
      </CrowdProvider>,
    );
    const watt = (value: number): string => formatQuantity(value, task.dimensions[0]!.quantity, "en");
    expect(sentence("led")).toBe(`The others on LED bulb: ${watt(8)} 1×, ${watt(120)} 1×, ${watt(2000)} 1×`);
    expect(chips("halogen")).toEqual([`${watt(120)} 2×`]);
    expect(document.querySelector('[data-presence-anchor="item:floodlight"]')).not.toBeNull();
    outsideLiveRegions();
  });

  it("shows nothing of the others without a crowd", () => {
    render(<ClassificationTaskView task={sheetTask<"classification">("heat") as SheetClassificationTask} answer={undefined} onAnswer={() => undefined} text={quizText("en")} locale="en" />);
    expect(document.querySelector("[data-crowd-item]")).toBeNull();
  });
});

describe("🏃️ the run, the results and a quiz's page", () => {
  const runScreen = (others: readonly ThinkingState[], crowds: QuizState["crowds"], showAnswers: boolean, locale: QuizLocale = "en") =>
    thinkingAlong(others, <RunScreen session={stubSession()} state={state("open", crowds)} run={RUN} text={quizText(locale)} locale={locale} showAnswers={showAnswers} />);

  it("shows what the others think now while anyone thinks along, else what they answered, else nothing", () => {
    const { rerender } = render(runScreen(OTHERS.slice(0, 2), { [QUIZ.id]: VIEW }, true));
    expect(document.querySelector("[data-crowd-source]")?.getAttribute("data-crowd-source")).toBe("live");
    expect(document.querySelector("[data-crowd-source]")?.textContent).toBe("👥What others think now· Thinking along: 2");
    expect(sentence("kettle")).toBe("The others on Kettle: Power 2×");
    rerender(runScreen([], { [QUIZ.id]: VIEW }, true));
    expect(document.querySelector("[data-crowd-source]")?.textContent).toBe("👥What everyone answered· Submitted runs: 3");
    expect(sentence("kettle")).toBe("All runs on Kettle: Power 2×, Energy 1×");
    rerender(runScreen([], {}, true));
    expect(document.querySelector("[data-crowd-source], [data-crowd-item]")).toBeNull();
    outsideLiveRegions();
  });

  it("hides what the others think when the learner prefers", () => {
    render(runScreen(OTHERS, { [QUIZ.id]: VIEW }, false));
    expect(document.querySelector("[data-crowd-source], [data-crowd-item]")).toBeNull();
  });

  it("puts the learner's answers beside the others' in every result table", () => {
    const results = (showAnswers: boolean) => thinkingAlong(OTHERS, <ResultsScreen session={stubSession()} state={state("submitted", { [QUIZ.id]: VIEW })} run={RUN} text={quizText("en")} locale="en" showAnswers={showAnswers} />);
    const { rerender } = render(results(true));
    const tables = screen.getAllByRole("table");
    expect(tables).toHaveLength(3);
    for (const table of tables)
      expect(
        within(table)
          .getAllByRole("columnheader")
          .map((head) => head.textContent),
      ).toContain("Everyone");
    const kettle = within(tables[0]!).getByRole("rowheader", { name: "Kettle" }).closest("tr")!;
    expect(within(kettle).getByText("All runs on Kettle: Power 2×, Energy 1×")).toBeTruthy();
    const fridge = within(tables[0]!).getByRole("rowheader", { name: "Fridge" }).closest("tr")!;
    expect(
      within(fridge)
        .getAllByRole("cell")
        .map((cell) => cell.textContent),
    ).toContain("–Nobody has answered this yet.");
    expect(document.querySelector("[data-crowd-source]")?.getAttribute("data-crowd-source")).toBe("submitted");
    rerender(results(false));
    for (const table of screen.getAllByRole("table"))
      expect(
        within(table)
          .getAllByRole("columnheader")
          .map((head) => head.textContent),
      ).not.toContain("Everyone");
  });

  it("tells on a quiz's page what the others answered, item by item from the learner's sheet, or that nobody has", () => {
    const quiz = CATALOG.quizzes[0]!;
    const page = (crowds: QuizState["crowds"], others: readonly ThinkingState[], showAnswers = true) =>
      thinkingAlong(
        others,
        <QuizPage
          quiz={quiz}
          session={stubSession()}
          state={{ ...state("submitted", crowds), step: { screen: "home", page: QUIZ.id } }}
          text={quizText("en")}
          locale="en"
          busy={false}
          act={() => undefined}
          view={{ opened: true, revealed: false }}
          showAnswers={showAnswers}
        />,
      );
    const { rerender } = render(page({}, []));
    const card = () => document.querySelector<HTMLElement>('[data-card="quiz-crowd"]');
    expect(within(card()!).getByText("What everyone answered")).toBeTruthy();
    expect(within(card()!).getByText("Nobody has answered this yet.")).toBeTruthy();
    rerender(page({ [QUIZ.id]: VIEW }, []));
    expect(
      within(card()!)
        .getAllByRole("heading", { level: 3 })
        .map((heading) => heading.textContent),
    ).toEqual(["Heat", "Masses", "Lamps"]);
    expect(sentence("kettle")).toBe("All runs on Kettle: Power 2×, Energy 1×");
    expect(sentence("horse")).toBe("On average all runs put Horse at place 3 of 3");
    rerender(page({ [QUIZ.id]: VIEW }, OTHERS));
    expect(within(card()!).getByText("What others think now", { selector: "h2 *, h2" })).toBeTruthy();
    rerender(page({ [QUIZ.id]: VIEW }, OTHERS, false));
    expect(card()).toBeNull();
    outsideLiveRegions();
  });

  it("keeps the wish to see what the others think on the device, shown unless switched off", () => {
    const store = localStore(memoryStorageOrigin().tab(), "arch");
    expect(readPreferences(store).showAnswers).toBe(true);
    store.write("preferences", { theme: "system", textSize: "normal", showCursors: true, showAnswers: false });
    const preferences = readPreferences(store);
    expect(preferences.showAnswers).toBe(false);
    const onChange = vi.fn();
    render(<PreferencesPanel preferences={preferences} locale="en" text={quizText("en")} onChange={onChange} />);
    const box = screen.getByRole("checkbox", { name: "Show what others think" }) as HTMLInputElement;
    expect(box.checked).toBe(false);
    fireEvent.click(box);
    expect(onChange).toHaveBeenLastCalledWith(expect.objectContaining({ showAnswers: true, showCursors: true }));
    expect(screen.getAllByRole("checkbox").map((checkbox) => (checkbox as HTMLInputElement).labels?.[0]?.textContent)).toEqual(expect.arrayContaining(["Show what others think"]));
  });
});
