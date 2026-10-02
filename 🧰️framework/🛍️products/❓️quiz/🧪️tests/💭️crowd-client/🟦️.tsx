/** 💭️ What everyone answered in the web client (design §20): when it shows — by default once the learner has
 * submitted, before that only on request, always or never by preference, an ask ending with its run — and how: the
 * answers of every task as a figure with a row per item and a column per category, place or value, the others thinking
 * along as dots in their colours, the learner's own answer and the correct one marked, and the scores of all runs as a
 * histogram with the learner's own bin. Every figure is a table or list that says its numbers in words, never a live
 * region, and keeps its size whether the crowd is known or not, so the page stays still. The shared vectors are judged
 * a second time by `lodash` (`groupBy`, `orderBy`, `uniq`, `sum`, `max`, `round`) recomputing the columns, the thinkers
 * of every cell and the totals from the raw drafts and counts, by `d3-scale` (`scaleLinear`, clamped) recomputing
 * every share and column height, and by `colord`'s WCAG contrast judging the paint of the columns.
 *
 * @see https://www.w3.org/WAI/WCAG22/Understanding/non-text-contrast.html — 3 : 1 for the parts of a graphic
 * @see ../../🧫️fixtures/💭️crowd-client/🔣️.json
 * @see ../../🎯️targets/⚛️react/🔨️modules/🗳️crowd/🟦️.tsx
 * @see ../../🎯️targets/⚛️react/🔨️modules/📊️plot/🟦️.tsx
 */

import { fireEvent, render, screen, within } from "@testing-library/react";
import { colord, extend } from "colord";
import a11y from "colord/plugins/a11y";
import { scaleLinear } from "d3-scale";
import groupBy from "lodash/groupBy";
import max from "lodash/max";
import orderBy from "lodash/orderBy";
import round from "lodash/round";
import sortBy from "lodash/sortBy";
import sum from "lodash/sum";
import uniq from "lodash/uniq";
import type { ReactElement, ReactNode } from "react";
import { describe, expect, it, vi } from "vitest";
import { scoreRun, thinkingScope, type CatalogView, type CrowdView, type Quiz, type RunView, type Sheet, type SheetTask, type ThinkingState } from "@semio-tech/quiz";
import {
  AnswerFigure,
  CROWD_DOTS,
  CrowdDoor,
  EMPTY_PRESENCE_VIEW,
  OTHERS_CHOICES,
  PreferencesPanel,
  PresenceOverlay,
  PresenceProvider,
  QuizPage,
  ResultsScreen,
  RunScreen,
  ScoreFigure,
  answerFigure,
  columnShare,
  crowdGate,
  crowdPlace,
  crowdShown,
  evolveQuizState,
  initialQuizState,
  formatQuantity,
  formatScore,
  formatShare,
  livePlace,
  localStore,
  memoryStorageOrigin,
  peakOf,
  presenceDrafts,
  quizText,
  readPreferences,
  scoreFigure,
  type AnswerFigureInput,
  type CrowdGate,
  type CrowdPlace,
  type OthersChoice,
  type PeerCursor,
  type QuizLocale,
  type QuizSession,
  type QuizState,
} from "@semio-tech/quiz-react";
import palette from "../../../../🔨️modules/🖱️ui/🎨️styling/🎨️palette/🎨️.css?raw";
import stylesheet from "../../🎯️targets/⚛️react/🎨️.css?raw";
import vectors from "../../🧫️fixtures/💭️crowd-client/🔣️.json";

extend([a11y]);

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
const RESULT = scoreRun(QUIZ, SHEET, ANSWERS)!;

function run(status: "open" | "submitted"): RunView {
  const base = { run: RUN, learner: LEARNER, quiz: QUIZ.id, sheet: SHEET, answers: ANSWERS, startedAt: 10 };
  return status === "open" ? { ...base, status } : { ...base, status, result: RESULT, submittedAt: 20 };
}

function state(status: "open" | "submitted", crowds: QuizState["crowds"] = {}, asked: readonly string[] = []): QuizState {
  const held = initialQuizState({
    introduced: true,
    learner: { id: LEARNER, identity: { kind: "pseudonym", handle: "Ada" } },
    catalog: CATALOG,
    learnerView: { learner: LEARNER, identity: { kind: "pseudonym", handle: "Ada" }, runs: [{ run: RUN, quiz: QUIZ.id, status, startedAt: 10 }], badges: [], best: status === "submitted" ? { [QUIZ.id]: RESULT.score } : {}, total: 0 },
    runs: { [RUN]: run(status) },
  });
  return { ...held, step: status === "open" ? { screen: "run", run: RUN } : { screen: "results", run: RUN }, crowds, asked };
}

function stubSession() {
  return { open: vi.fn(), answer: vi.fn(), askCrowd: vi.fn(), unaskCrowd: vi.fn(), startRun: vi.fn(async () => undefined), resumeRun: vi.fn(async () => undefined), loadRun: vi.fn(async () => undefined), submitRun: vi.fn(async () => undefined) } as unknown as QuizSession & {
    readonly askCrowd: ReturnType<typeof vi.fn>;
    readonly unaskCrowd: ReturnType<typeof vi.fn>;
  };
}

function sheetTask(id: string): SheetTask {
  return SHEET.tasks.find((task) => task.id === id)!;
}

function thinkingAlong(others: readonly ThinkingState[], children: ReactNode): ReactElement {
  return (
    <PresenceProvider view={{ ...EMPTY_PRESENCE_VIEW, colours: COLOURS, thinking: new Map([[thinkingScope(CATALOG.id, QUIZ.id), others]]) }} setTask={() => undefined}>
      {children}
    </PresenceProvider>
  );
}

type FigureVector = (typeof vectors.figures)[number];

function inputOf(vector: Pick<FigureVector, "task" | "dimension" | "crowd" | "thinking" | "answer" | "result">, locale: QuizLocale = "en"): AnswerFigureInput {
  const task = sheetTask(vector.task);
  return {
    task,
    ...(vector.dimension === null ? {} : { dimension: vector.dimension }),
    ...(vector.crowd ? { crowd: VIEW } : {}),
    thinking: vector.thinking.map((index) => OTHERS[index]!),
    ...(vector.answer ? { answer: ANSWERS[task.id as keyof typeof ANSWERS] } : {}),
    ...(vector.result ? { result: RESULT.tasks.find((result) => result.task === task.id)! } : {}),
    locale,
  };
}

function figures(kind?: "answers" | "scores"): readonly HTMLElement[] {
  return [...document.querySelectorAll<HTMLElement>(kind === undefined ? "[data-crowd-figure]" : `[data-crowd-figure="${kind}"]`)];
}

function cellOf(item: string, key: string, within: ParentNode = document): HTMLElement {
  return within.querySelector<HTMLElement>(`[data-crowd-item="${item}"] > [data-key="${key}"]`)!;
}

function said(cell: HTMLElement): string | null | undefined {
  return cell.querySelector(".sr-only")?.textContent;
}

function outsideLiveRegions(): void {
  for (const element of document.querySelectorAll("[data-crowd-figure], [data-crowd-gate]")) expect(element.closest('[aria-live], [role="status"], [role="alert"], [role="log"]')).toBeNull();
}

function rule(selector: string): string {
  return new RegExp(`(?:^|\\n)${selector.replaceAll(/[.[\]*]/gu, "\\$&")} \\{([^}]*)\\}`, "u").exec(stylesheet)?.[1] ?? "";
}
//#endregion 🏗️Fixtures

//#region ⚖️Oracle
/** ⚖️ A share recomputed by `d3-scale`: `count` on a clamped linear scale from nothing to `full`; nothing where the
 * scale has no extent (a degenerate domain, which d3 maps to the middle of its range). */
function scaled(count: number, full: number): number {
  return full > 0 ? scaleLinear().domain([0, full]).range([0, 1]).clamp(true)(count) : 0;
}

/** ⚖️ The thinkers of one cell recomputed from the raw drafts by lodash: one draft per tag (the last given), grouped by
 * what it gives the item — a category, a value, or the place a sorting position rounds half up to. */
function draftedTags(vector: FigureVector, item: string, key: string, columns: number): readonly string[] {
  const latest = Object.values(groupBy(vector.thinking.map((index) => OTHERS[index]!), "tag")).map((drafts) => drafts.at(-1)!);
  const drafted = latest.flatMap((other) => {
    const answer = other.answers[vector.task];
    if (answer?.kind === "classification") return answer.assignments[item] === undefined ? [] : [{ tag: other.tag, key: answer.assignments[item] }];
    if (answer?.kind === "matching") return vector.dimension === null || answer.values[vector.dimension]?.[item] === undefined ? [] : [{ tag: other.tag, key: String(answer.values[vector.dimension]![item]) }];
    if (answer?.kind !== "sorting" || !answer.order.includes(item)) return [];
    return [{ tag: other.tag, key: String(round((answer.order.indexOf(item) / Math.max(1, answer.order.length - 1)) * (columns - 1) + 1e-9)) }];
  });
  return sortBy((groupBy(drafted, "key")[key] ?? []).map((entry) => entry.tag));
}
//#endregion ⚖️Oracle

describe("🚪️ when the others show", () => {
  for (const vector of vectors.gates) {
    it(`${vector.others} · ${vector.place} · ${vector.asked ? "asked" : "not asked"} · ${vector.submitted ? "submitted" : "not submitted"} → ${vector.expected}`, () => {
      const gate = crowdGate(vector.others as OthersChoice, vector.place as CrowdPlace, { asked: vector.asked, submitted: vector.submitted });
      expect(gate).toBe(vector.expected);
      expect(crowdShown(gate)).toBe(vector.expected === "open" || vector.expected === "asked");
    });
  }

  it("covers every choice, place and fact once", () => {
    expect(new Set(vectors.gates.map((vector) => JSON.stringify([vector.others, vector.place, vector.asked, vector.submitted]))).size).toBe(OTHERS_CHOICES.length * 3 * 2 * 2);
    expect(uniq(vectors.gates.map((vector) => vector.others))).toEqual([...OTHERS_CHOICES]);
  });

  it("keeps an ask until the learner takes it back, the run ends or the learner changes", () => {
    const open = state("open");
    const asked = evolveQuizState(open, { type: "crowd-asked", quiz: QUIZ.id });
    expect(asked.asked).toEqual([QUIZ.id]);
    expect(evolveQuizState(asked, { type: "crowd-asked", quiz: QUIZ.id })).toBe(asked);
    expect(evolveQuizState(open, { type: "crowd-unasked", quiz: QUIZ.id })).toBe(open);
    expect(evolveQuizState(asked, { type: "crowd-unasked", quiz: QUIZ.id }).asked).toEqual([]);
    expect(evolveQuizState(asked, { type: "answer-given", run: RUN, task: "heat", answer: ANSWERS.heat }).asked).toEqual([QUIZ.id]);
    expect(evolveQuizState(asked, { type: "run-submitted", run: RUN, result: RESULT, badges: [], at: 20 }).asked).toEqual([]);
    expect(evolveQuizState(asked, { type: "run-voided", run: RUN }).asked).toEqual([]);
    expect(evolveQuizState(asked, { type: "run-loaded", view: run("submitted") }).asked).toEqual([]);
    expect(evolveQuizState(asked, { type: "run-loaded", view: run("open") }).asked).toEqual([QUIZ.id]);
    expect(evolveQuizState(asked, { type: "learner-forgotten" }).asked).toEqual([]);
    expect(evolveQuizState(asked, { type: "learner-identified", learner: "b".repeat(32) }).asked).toEqual([]);
    expect(evolveQuizState(asked, { type: "learner-identified", learner: LEARNER }).asked).toEqual([QUIZ.id]);
    const two = evolveQuizState(asked, { type: "crowd-asked", quiz: "other" });
    expect(evolveQuizState(two, { type: "run-submitted", run: RUN, result: RESULT, badges: [], at: 20 }).asked).toEqual(["other"]);
  });

  it("offers the ask where the gate is locked and takes it back where it was asked, with one button that keeps the focus", () => {
    const [onAsk, onUnask] = [vi.fn(), vi.fn()];
    const door = (gate: CrowdGate) => <CrowdDoor gate={gate} onAsk={onAsk} onUnask={onUnask} text={quizText("en")} />;
    const { rerender } = render(door("locked"));
    const button = screen.getByRole("button", { name: "Show it now" });
    expect(document.getElementById(button.getAttribute("aria-describedby")!)?.textContent).toBe("You see what the others answered once you have submitted this quiz.");
    expect(button.parentElement!.firstElementChild).toBe(button);
    button.focus();
    fireEvent.click(button);
    expect([onAsk.mock.calls.length, onUnask.mock.calls.length]).toEqual([1, 0]);
    rerender(door("asked"));
    expect(screen.getByRole("button", { name: "Hide it again" })).toBe(button);
    expect(document.activeElement).toBe(button);
    expect(button.hasAttribute("aria-describedby")).toBe(false);
    expect(screen.queryByText("You see what the others answered once you have submitted this quiz.")).toBeNull();
    fireEvent.click(button);
    expect([onAsk.mock.calls.length, onUnask.mock.calls.length]).toEqual([1, 1]);
    for (const gate of ["open", "off"] as const) {
      rerender(door(gate));
      expect(document.querySelector("[data-crowd-gate], button")).toBeNull();
    }
    rerender(<CrowdDoor gate="locked" onAsk={onAsk} onUnask={onUnask} text={quizText("de")} />);
    expect(screen.getByRole("button", { name: "Jetzt schon anzeigen" })).toBeTruthy();
    outsideLiveRegions();
  });
});

describe("🧮️ the figures of what everyone answered", () => {
  for (const vector of vectors.figures) {
    it(`builds the answers of a task: ${vector.id}`, () => {
      const figure = answerFigure(inputOf(vector));
      expect({ runs: figure.runs, thinkers: figure.thinkers, columns: figure.columns.map((column) => column.key), rows: figure.rows.map(({ label: _label, ...row }) => row) }).toEqual(vector.expected);
      expect(figure.kind).toBe(sheetTask(vector.task).kind);
      expect(figure.thinkers).toBe(uniq(vector.thinking.map((index) => OTHERS[index]!.tag)).length);
      for (const row of figure.rows) {
        expect(row.cells.map((cell) => cell.key)).toEqual(figure.columns.map((column) => column.key));
        expect(sum(row.cells.map((cell) => cell.count))).toBe(row.answers);
        expect(row.cells.filter((cell) => cell.own).length).toBeLessThanOrEqual(1);
        expect(row.cells.filter((cell) => cell.correct).length).toBe(vector.result ? 1 : 0);
        for (const cell of row.cells) {
          expect(cell.share, `${row.item}/${cell.key}`).toBeCloseTo(scaled(cell.count, row.answers), 12);
          expect(cell.tags, `${row.item}/${cell.key}`).toEqual(draftedTags(vector, row.item, cell.key, figure.columns.length));
        }
      }
    });
  }

  it("names the columns of a classification by its categories in sheet order, in the learner's language", () => {
    const vector = vectors.figures.find((candidate) => candidate.task === "heat")!;
    expect(answerFigure(inputOf(vector, "en")).columns.map((column) => column.label)).toEqual(["Power", "Energy"]);
    expect(answerFigure(inputOf(vector, "de")).columns.map((column) => column.label)).toEqual(["Leistung", "Energie"]);
    expect(answerFigure(inputOf(vector, "de")).rows.map((row) => row.label)).toEqual(["Wasserkocher", "Kühlschrank", "Teelicht"]);
  });

  it("numbers the columns of a sorting by place, one per item of the sheet until the crowd says how many are presented", () => {
    const [known, unknown] = [vectors.figures.find((candidate) => candidate.id === "sorting-in-sheet-order-without-an-answer")!, vectors.figures.find((candidate) => candidate.id === "sorting-before-the-crowd-is-known")!];
    for (const vector of [known, unknown]) expect(answerFigure(inputOf(vector)).columns).toEqual(["1", "2", "3"].map((label, place) => ({ key: String(place), label })));
  });

  it("orders the columns of a matching by value — the sheet's cards and every value anyone gave — and writes them as quantities", () => {
    const vector = vectors.figures.find((candidate) => candidate.id === "matching-values-ascending-with-a-value-of-another-sheet")!;
    const task = sheetTask("lamps");
    if (task.kind !== "matching") throw new Error("lamps is a matching");
    const quantity = task.dimensions[0]!.quantity;
    const given = [...task.dimensions[0]!.cards.map(String), ...VIEW.tasks.find((crowd) => crowd.task === "lamps")!.items.flatMap((item) => (item.counts ?? []).map((count) => count.key)), ...OTHERS.flatMap((other) => (other.answers.lamps?.kind === "matching" ? Object.values(other.answers.lamps.values.power ?? {}).map(String) : []))];
    for (const locale of ["en", "de"] as const) {
      expect(answerFigure(inputOf(vector, locale)).columns).toEqual(orderBy(uniq(given), Number).map((key) => ({ key, label: formatQuantity(Number(key), quantity, locale) })));
    }
    expect(answerFigure(inputOf(vector)).columns.map((column) => column.key)).toEqual(["8", "45", "120", "2000"]);
  });

  for (const vector of vectors.scores) {
    it(`builds the scores of all runs: ${vector.id}`, () => {
      const figure = scoreFigure(vector.bins ?? undefined, vector.own ?? undefined);
      expect({ bins: figure.bins, runs: figure.runs, peak: figure.peak, own: figure.own ?? null }).toEqual(vector.expected);
      expect(figure.runs).toBe(sum(figure.bins));
      expect(figure.peak).toBe(max(figure.bins) ?? 0);
      expect(peakOf(figure.bins)).toBe(figure.peak);
      for (const count of figure.bins) expect(columnShare(count, figure.peak)).toBeCloseTo(scaled(count, figure.peak), 12);
    });
  }

  it("keeps a column inside its plot whatever it is given", () => {
    for (const [count, full] of [
      [5, 4],
      [-1, 4],
      [3, 0],
      [0, 0],
    ] as const)
      expect(columnShare(count, full)).toBe(scaled(count, full));
  });

  for (const vector of vectors.shares) {
    it(`writes the share ${vector.share} as ${vector.en} and ${vector.de}`, () => {
      expect(formatShare(vector.share, "en")).toBe(vector.en);
      expect(formatShare(vector.share, "de")).toBe(vector.de);
    });
  }

  for (const vector of vectors.livePlaces) {
    it(`puts a thinker at position ${vector.position} on place ${vector.place} of ${vector.places}`, () => {
      expect(livePlace(vector.position, vector.places)).toBe(vector.place);
    });
  }

  for (const vector of vectors.places) {
    it(`reads the mean position ${vector.position} of ${vector.total} items as place ${vector.place}`, () => {
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

describe("🖼️ a figure on the page", () => {
  const live = vectors.figures.find((candidate) => candidate.id === "classification-thinkers-and-the-learners-own-answer")!;
  const figure = (vector: FigureVector, locale: QuizLocale = "en", title?: string) => thinkingAlong([], <AnswerFigure {...inputOf(vector, locale)} name="What everyone answered: Heat" title={title} text={quizText(locale)} />);

  it("draws the answers of a classification as a named table that says every cell in words", () => {
    render(figure(live));
    const table = screen.getByRole("table", { name: "What everyone answered: Heat" });
    expect(
      within(table)
        .getAllByRole("columnheader")
        .map((head) => head.textContent),
    ).toEqual(["Item", "Power", "Energy"]);
    expect(
      within(table)
        .getAllByRole("rowheader")
        .map((head) => head.textContent),
    ).toEqual(["Kettle", "Fridge", "Tea light"]);
    expect(said(cellOf("kettle", "power"))).toBe("75% (3 of 4), your answer, thinking this now: 2");
    expect(said(cellOf("kettle", "energy"))).toBe("25% (1 of 4), thinking this now: 1");
    expect(said(cellOf("fridge", "power"))).toBe("0% (0 of 0), your answer, thinking this now: 1");
    expect(said(cellOf("candle", "energy"))).toBe("0% (0 of 1)");
    expect(cellOf("kettle", "power").title).toBe(said(cellOf("kettle", "power")));
    expect(figures()[0]!.querySelector("figcaption")?.textContent).toBe("👥What everyone answered· Submitted runs: 4· Thinking along now: 3");
    outsideLiveRegions();
  });

  it("draws every share as a column of that height with its percentage on the cap, the learner's own in the accent", () => {
    render(figure(live));
    const column = (item: string, key: string): HTMLElement => cellOf(item, key).querySelector<HTMLElement>(".quiz-column")!;
    expect(column("kettle", "power").style.getPropertyValue("--quiz-column")).toBe("0.75");
    expect(column("kettle", "power").querySelector(".quiz-column-cap")?.textContent).toBe("75%");
    expect(column("kettle", "power").hasAttribute("data-emphasis")).toBe(true);
    expect(column("kettle", "energy").style.getPropertyValue("--quiz-column")).toBe("0.25");
    expect(column("kettle", "energy").hasAttribute("data-emphasis")).toBe(false);
    expect(column("fridge", "energy").style.getPropertyValue("--quiz-column")).toBe("0");
    expect(column("fridge", "energy").querySelector(".quiz-column-cap")).toBeNull();
    expect(column("fridge", "energy").hasAttribute("data-filled")).toBe(false);
    for (const drawn of document.querySelectorAll(".quiz-column, .quiz-plot-marks")) expect(drawn.getAttribute("aria-hidden")).toBe("true");
    expect(rule(".quiz-column-bar")).toMatch(/block-size: calc\(var\(--quiz-column\) \* \(100% - 1\.25em\)\);/u);
    expect(rule(".quiz-column[data-filled] > .quiz-column-bar")).toMatch(/min-block-size: 2px;/u);
    expect(rule(".quiz-column[data-emphasis] > .quiz-column-bar")).toMatch(/background: var\(--active-base\);/u);
  });

  it("marks the learner's own answer, shows who thinks what right now as dots in their colours, and explains the marks", () => {
    render(figure(live));
    const marks = (item: string, key: string): HTMLElement => cellOf(item, key).querySelector<HTMLElement>(".quiz-plot-marks")!;
    expect(marks("kettle", "power").querySelector(".quiz-plot-own")?.textContent).toBe("●");
    expect([...marks("kettle", "power").querySelectorAll<HTMLElement>("[data-crowd-dot]")].map((dot) => [dot.dataset.crowdDot, dot.style.getPropertyValue("--quiz-peer")])).toEqual([
      ["0000000a", "var(--presence-3)"],
      ["0000000b", "var(--presence-5)"],
    ]);
    expect([...marks("kettle", "energy").querySelectorAll<HTMLElement>("[data-crowd-dot]")].map((dot) => dot.style.getPropertyValue("--quiz-peer"))).toEqual(["var(--presence-7)"]);
    expect(marks("candle", "energy").textContent).toBe("");
    expect([...document.querySelectorAll("[data-legend]")].map((entry) => [entry.getAttribute("data-legend"), entry.textContent])).toEqual([
      ["own", "●your answer"],
      ["live", "thinking this now"],
    ]);
    expect(document.querySelector("[data-legend]")!.closest("ul")!.getAttribute("aria-hidden")).toBe("true");
  });

  it("counts the thinkers of a cell beyond the dots it shows", () => {
    const crowd = Array.from({ length: CROWD_DOTS + 3 }, (_, index): ThinkingState => ({ tag: `0000001${index}`, answers: { heat: { kind: "classification", assignments: { kettle: "power" } } } }));
    render(thinkingAlong([], <AnswerFigure task={sheetTask("heat")} thinking={crowd} locale="en" name="Heat" text={quizText("en")} />));
    const marks = cellOf("kettle", "power").querySelector<HTMLElement>(".quiz-plot-marks")!;
    expect(marks.querySelectorAll("[data-crowd-dot]")).toHaveLength(CROWD_DOTS);
    expect(marks.textContent).toBe("+3");
    expect(said(cellOf("kettle", "power"))).toBe(`0% (0 of 0), thinking this now: ${CROWD_DOTS + 3}`);
  });

  it("marks the correct answer once there is a result", () => {
    render(figure(vectors.figures.find((candidate) => candidate.id === "classification-result-marks-the-correct-category")!));
    expect(said(cellOf("kettle", "power"))).toBe("75% (3 of 4), your answer, correct");
    expect(said(cellOf("fridge", "energy"))).toBe("0% (0 of 0), correct");
    expect(cellOf("fridge", "energy").querySelector(".quiz-plot-marks")?.textContent).toBe("✓");
    expect(cellOf("fridge", "power").querySelector(".quiz-plot-marks")?.textContent).toBe("●");
    expect([...document.querySelectorAll("[data-legend]")].map((entry) => entry.textContent)).toEqual(["●your answer", "✓correct"]);
  });

  it("says it in German too", () => {
    render(figure(live, "de"));
    expect(said(cellOf("kettle", "power"))).toBe("75 % (3 von 4), deine Antwort, denken das gerade: 2");
    expect(figures()[0]!.querySelector("figcaption")?.textContent).toBe("👥Was alle geantwortet haben· Abgegebene Durchgänge: 4· Denken gerade mit: 3");
  });

  it("draws a sorting place by place in the learner's own order and ends every row with the average place", () => {
    render(figure(vectors.figures.find((candidate) => candidate.id === "sorting-in-the-learners-own-order-with-places-and-thinkers")!));
    const table = screen.getByRole("table");
    expect(
      within(table)
        .getAllByRole("columnheader")
        .map((head) => [head.querySelector(".sr-only")?.textContent ?? head.textContent, head.querySelector('[aria-hidden="true"]')?.textContent ?? null]),
    ).toEqual([
      ["Item", null],
      ["Place 1", "1"],
      ["Place 2", "2"],
      ["Place 3", "3"],
      ["Avg. place", null],
    ]);
    expect(
      within(table)
        .getAllByRole("row")
        .slice(1)
        .map((row) => [row.querySelector("th")?.textContent, row.querySelector(".quiz-plot-mean")?.textContent, [...row.querySelectorAll("[data-own]")].map((cell) => cell.getAttribute("data-key"))]),
    ).toEqual([
      ["Mouse", "1.5", ["0"]],
      ["Cat", "1.8", ["1"]],
      ["Horse", "2.8", ["2"]],
    ]);
    expect(said(cellOf("horse", "2"))).toBe("75% (3 of 4), your answer, thinking this now: 2");
    expect(table.style.getPropertyValue("--quiz-plot-columns")).toBe("4");
  });

  it("says that nobody placed an item where no average place is known", () => {
    render(figure(vectors.figures.find((candidate) => candidate.id === "sorting-before-the-crowd-is-known")!));
    expect([...document.querySelectorAll(".quiz-plot-mean")].map((cell) => cell.textContent)).toEqual(Array.from({ length: 3 }, () => "–Nobody has answered this yet."));
  });

  it("heads the figure of a matching's dimension with what the caller names it", () => {
    render(figure(vectors.figures.find((candidate) => candidate.id === "matching-result-marks-the-correct-value")!, "en", "Power"));
    expect(figures()[0]!.querySelector("figcaption")?.textContent).toBe("👥Power· Submitted runs: 4");
    expect(figures()[0]!.dataset.dimension).toBe("power");
    expect(said(cellOf("halogen", "120"))).toBe("67% (2 of 3), your answer, correct");
  });

  it("names the unit of a matching once, in the caption, when every value is in it, and with every value when its prefix varies", () => {
    const task = sheetTask("lamps");
    if (task.kind !== "matching") throw new Error("lamps is a matching");
    const plain = { ...task, dimensions: task.dimensions.map((dimension) => ({ ...dimension, quantity: { ...dimension.quantity, prefixed: false } })) };
    const model = answerFigure({ task: plain, dimension: "power", crowd: VIEW, locale: "en" });
    expect([model.unit, model.columns.map((column) => column.label)]).toEqual(["W", ["8", "45", "120", "2,000"]]);
    expect(answerFigure({ task, dimension: "power", crowd: VIEW, locale: "en" }).unit).toBeUndefined();
    render(thinkingAlong([], <AnswerFigure task={plain} dimension="power" crowd={VIEW} locale="en" name="Lamps" title="Power" text={quizText("en")} />));
    expect(figures()[0]!.querySelector("figcaption")?.textContent).toBe("👥Power (W)· Submitted runs: 4");
    const heads = within(screen.getByRole("table", { name: "Lamps" })).getAllByRole("columnheader").slice(1);
    expect(heads.map((head) => [head.firstChild?.textContent, head.querySelector(".sr-only")?.textContent])).toEqual(["8", "45", "120", "2,000"].map((value) => [value, " W"]));
  });

  it("draws the scores of all runs as a named list of bins over a percent axis, the learner's own bin in the accent", () => {
    render(<ScoreFigure name="How everyone scored: Household physics" bins={VIEW.scores} own={0.5555} text={quizText("en")} locale="en" />);
    const list = screen.getByRole("list", { name: "How everyone scored: Household physics" });
    const bins = within(list).getAllByRole("listitem");
    expect(bins.map((bin) => bin.dataset.count)).toEqual(VIEW.scores.map(String));
    expect(bins.map((bin) => bin.querySelector(".sr-only")?.textContent).slice(3, 6)).toEqual(["30% to 40%: 1", "40% to 50%: 0", `50% to 60%: 1, You: ${formatScore(0.5555, "en")}`]);
    expect(bins.map((bin) => bin.hasAttribute("data-own"))).toEqual(VIEW.scores.map((_, bin) => bin === 5));
    expect(bins.map((bin) => bin.querySelector<HTMLElement>(".quiz-column")!.style.getPropertyValue("--quiz-column"))).toEqual(VIEW.scores.map(String));
    expect(bins.map((bin) => bin.querySelector(".quiz-column")!.hasAttribute("data-emphasis"))).toEqual(VIEW.scores.map((_, bin) => bin === 5));
    expect(bins.map((bin) => bin.querySelector(".quiz-column-cap")?.textContent ?? null)).toEqual(VIEW.scores.map((count) => (count === 0 ? null : String(count))));
    expect(figures("scores")[0]!.querySelector("figcaption")?.textContent).toBe(`👥How everyone scored· Submitted runs: 4· ● You: ${formatScore(0.5555, "en")}`);
    expect([...document.querySelector(".quiz-histogram-axis")!.children].map((tick) => tick.textContent)).toEqual(["0%", "50%", "100%"]);
    expect(document.querySelector(".quiz-histogram-axis")!.getAttribute("aria-hidden")).toBe("true");
    outsideLiveRegions();
  });

  it("draws the scores before the crowd is known as ten empty bins, and without the learner's own while there is none", () => {
    render(<ScoreFigure name="Scores" bins={undefined} text={quizText("de")} locale="de" />);
    const bins = within(screen.getByRole("list", { name: "Scores" })).getAllByRole("listitem");
    expect(bins).toHaveLength(10);
    expect(bins.map((bin) => bin.querySelector(".quiz-column-cap"))).toEqual(Array.from({ length: 10 }, () => null));
    expect(bins[0]!.querySelector(".sr-only")?.textContent).toBe("0 % bis 10 %: 0");
    expect(document.querySelector("[data-own], [data-own-score]")).toBeNull();
    expect(figures("scores")[0]!.querySelector("figcaption")?.textContent).toBe("👥Wie alle abgeschnitten haben· Abgegebene Durchgänge: 0");
  });
});

describe("🧘️ a page that stays still while the crowd arrives and the others come and go", () => {
  /** 📐️ What decides the size of every figure: its kind, its rows and how many cells each holds. */
  function shapes(): readonly string[] {
    return figures().map((figure) => `${figure.dataset.crowdFigure}:${figure.dataset.task ?? ""}:${[...figure.querySelectorAll("tbody tr, ol > li")].map((row) => `${(row as HTMLElement).dataset.crowdItem ?? (row as HTMLElement).dataset.bin}×${row.children.length}`).join(",")}`);
  }

  it("paints the columns of the others and of the learner at least 3 : 1 against the surface in both appearances", () => {
    const token = (name: string): string => new RegExp(`--color-${name}: (#[0-9a-f]{6});`, "u").exec(palette)![1]!;
    const share = Number(/background: color-mix\(in srgb, var\(--foreground\) (\d+)%, var\(--base\)\);/u.exec(rule(".quiz-column-bar"))![1]) / 100;
    const [dark, light, accent] = [colord(token("dark")).toRgb(), colord(token("light")).toRgb(), colord(token("primary"))];
    for (const [ink, surface] of [
      [dark, light],
      [light, dark],
    ] as const) {
      const quiet = colord({ r: ink.r * share + surface.r * (1 - share), g: ink.g * share + surface.g * (1 - share), b: ink.b * share + surface.b * (1 - share) });
      expect(quiet.contrast(colord(surface)), `the others on ${colord(surface).toHex()}`).toBeGreaterThanOrEqual(3);
      expect(accent.contrast(colord(surface)), `the learner on ${colord(surface).toHex()}`).toBeGreaterThanOrEqual(3);
    }
  });

  it("gives every plot one height and every table columns of one width, whatever stands in them", () => {
    expect(rule(".quiz-column")).toMatch(/block-size: 3\.5em;/u);
    expect(rule(".quiz-plot")).toMatch(/table-layout: fixed;/u);
    expect(rule(".quiz-figure")).toMatch(/container-type: inline-size;/u);
    expect(rule(".quiz-plot")).toMatch(/inline-size: min\(100%, calc\(20em \+ var\(--quiz-plot-columns, 1\) \* 9em\)\);/u);
    expect(rule(".quiz-plot")).toMatch(/min-inline-size: calc\(9em \+ var\(--quiz-plot-columns, 1\) \* 3\.5em\);/u);
    expect(rule(".quiz-plot .quiz-plot-item")).toMatch(/inline-size: clamp\(9em, 36cqi, 20em\);/u);
    expect(rule(".quiz-plot-marks")).toMatch(/min-block-size: 1\.25em;/u);
    expect(rule(".quiz-histogram")).toMatch(/grid-auto-columns: minmax\(0, 1fr\);/u);
    expect(stylesheet).toMatch(/@media \(prefers-reduced-motion: no-preference\) \{\s*\.quiz-column-bar \{\s*animation: quiz-column-rise/u);
  });

  it("keeps every figure of the results the size it has before the crowd is known", () => {
    const results = (crowds: QuizState["crowds"]) => thinkingAlong([], <ResultsScreen session={stubSession()} state={state("submitted", crowds)} run={RUN} text={quizText("en")} locale="en" />);
    const { rerender } = render(results({}));
    const unknown = shapes();
    expect(unknown).toHaveLength(1 + 2 * SHEET.tasks.length);
    expect(figures().map((figure) => figure.dataset.runs)).toEqual(unknown.map(() => "0"));
    rerender(results({ [QUIZ.id]: VIEW }));
    const known = shapes();
    expect(known.filter((shape) => !shape.includes(":lamps:"))).toEqual(unknown.filter((shape) => !shape.includes(":lamps:")));
    expect(known.map((shape) => shape.split(",").length)).toEqual(unknown.map((shape) => shape.split(",").length));
    expect(figures().map((figure) => figure.dataset.runs)).toEqual(known.map(() => "4"));
  });

  it("keeps the figure of a run's task the size it has whoever thinks along", () => {
    const runScreen = (others: readonly ThinkingState[], crowds: QuizState["crowds"]) => thinkingAlong(others, <RunScreen session={stubSession()} state={state("open", crowds, [QUIZ.id])} run={RUN} text={quizText("en")} locale="en" />);
    const { rerender } = render(runScreen([], {}));
    const alone = shapes();
    expect(alone).toHaveLength(1);
    for (const [others, crowds] of [
      [OTHERS.slice(0, 1), {}],
      [OTHERS, { [QUIZ.id]: VIEW }],
      [[], { [QUIZ.id]: VIEW }],
    ] as const) {
      rerender(runScreen(others, crowds));
      expect(shapes()).toEqual(alone);
    }
  });

  it("keeps the place of how many are learning a quiz now, invisible and unspoken while nobody is", () => {
    const quiz = CATALOG.quizzes[0]!;
    const page = (learning: number) => (
      <PresenceProvider view={{ ...EMPTY_PRESENCE_VIEW, roster: { online: learning, active: learning, learners: [], quizzes: { [QUIZ.id]: learning } } }} setTask={() => undefined}>
        <QuizPage quiz={quiz} session={stubSession()} state={{ ...state("submitted"), step: { screen: "home", page: QUIZ.id } }} text={quizText("en")} locale="en" busy={false} act={() => undefined} view={{ opened: true, revealed: false }} />
      </PresenceProvider>
    );
    const fact = (): HTMLElement => document.querySelector<HTMLElement>('[data-card="quiz"] [data-learning]')!;
    const facts = (): number => fact().closest("ul")!.children.length;
    const { rerender } = render(page(0));
    const reserved = facts();
    expect([fact().textContent, fact().getAttribute("aria-hidden"), fact().classList.contains("invisible")]).toEqual(["Learning now: 0", "true", true]);
    rerender(page(2));
    expect([fact().textContent, fact().getAttribute("aria-hidden"), fact().classList.contains("invisible")]).toEqual(["Learning now: 2", null, false]);
    expect(facts()).toBe(reserved);
  });

  it("draws the others' pointers and names over the page, never in it", () => {
    const peers: readonly PeerCursor[] = [{ session: "s-1", tag: "0000000a", label: "Ada", colour: 3, cursor: { anchor: "run", x: 0.5, y: 0.5 }, focus: "run", drag: "kettle" }];
    render(<PresenceOverlay view={{ ...EMPTY_PRESENCE_VIEW, peers }} show />);
    const layer = document.querySelector<HTMLElement>("[data-presence-layer]")!;
    for (const name of ["fixed", "inset-0", "overflow-hidden", "pointer-events-none"]) expect(layer.classList.contains(name), name).toBe(true);
    expect(layer.getAttribute("aria-hidden")).toBe("true");
    expect([...layer.querySelectorAll("[data-peer]")].map((mark) => mark.className)).toEqual(["quiz-peer-focus", "quiz-peer"]);
    expect(stylesheet).toMatch(/\.quiz-peer,\s*\.quiz-peer-focus \{\s*position: absolute;/u);
    expect(stylesheet).toMatch(/\.quiz-peer-label \{\s*position: absolute;/u);
  });
});

describe("🏃️ the run, the results and a quiz's page", () => {
  const runScreen = (session: QuizSession, asked: readonly string[], others?: OthersChoice, thinking: readonly ThinkingState[] = OTHERS) =>
    thinkingAlong(thinking, <RunScreen session={session} state={state("open", { [QUIZ.id]: VIEW }, asked)} run={RUN} text={quizText("en")} locale="en" others={others} />);

  it("keeps the others out of a run until it is submitted, and shows them at once when the learner asks", () => {
    const session = stubSession();
    const { rerender } = render(runScreen(session, []));
    const task = (): HTMLElement => document.querySelector<HTMLElement>('[data-card="task"]')!;
    expect(figures()).toEqual([]);
    expect(task().querySelector("[data-crowd-gate]")?.getAttribute("data-crowd-gate")).toBe("locked");
    expect(within(task()).getByText("You see what the others answered once you have submitted this quiz.")).toBeTruthy();
    expect(task().querySelector("[data-crowd-dot], .quiz-column")).toBeNull();
    fireEvent.click(within(task()).getByRole("button", { name: "Show it now" }));
    expect(session.askCrowd.mock.calls).toEqual([[QUIZ.id]]);
    rerender(runScreen(session, [QUIZ.id]));
    expect(task().querySelector("[data-crowd-gate]")?.getAttribute("data-crowd-gate")).toBe("asked");
    expect(figures().map((figure) => [figure.dataset.crowdFigure, figure.dataset.task, figure.dataset.runs, figure.dataset.thinkers])).toEqual([["answers", SHEET.tasks[0]!.id, "4", "3"]]);
    expect(said(cellOf("kettle", "power", task()))).toBe("75% (3 of 4), your answer, thinking this now: 2");
    expect(task().querySelector("[data-correct]")).toBeNull();
    const [interaction, door, figure] = [task().querySelector("select")!, task().querySelector("[data-crowd-gate]")!, figures()[0]!];
    expect(interaction.compareDocumentPosition(door) & Node.DOCUMENT_POSITION_FOLLOWING).toBeTruthy();
    expect(door.compareDocumentPosition(figure) & Node.DOCUMENT_POSITION_FOLLOWING).toBeTruthy();
    fireEvent.click(within(task()).getByRole("button", { name: "Hide it again" }));
    expect(session.unaskCrowd.mock.calls).toEqual([[QUIZ.id]]);
    outsideLiveRegions();
  });

  it("shows the others in a run without being asked to a learner who always wants them, and never to one who never does", () => {
    const { rerender } = render(runScreen(stubSession(), [], "always"));
    expect(figures("answers")).toHaveLength(1);
    expect(document.querySelector("[data-crowd-gate]")).toBeNull();
    rerender(runScreen(stubSession(), [QUIZ.id], "never"));
    expect(document.querySelector("[data-crowd-figure], [data-crowd-gate], [data-crowd-dot]")).toBeNull();
  });

  it("shows on the results how all runs scored and, below every task's table, what all runs answered there", () => {
    const results = (others?: OthersChoice) => thinkingAlong(OTHERS, <ResultsScreen session={stubSession()} state={state("submitted", { [QUIZ.id]: VIEW })} run={RUN} text={quizText("en")} locale="en" others={others} />);
    const { rerender } = render(results());
    const summary = document.querySelector<HTMLElement>('[data-card="results"]')!;
    expect(within(summary).getByRole("list", { name: "How everyone scored: Household physics" })).toBeTruthy();
    expect(summary.querySelector("[data-own-score]")?.textContent).toBe(`● You: ${formatScore(RESULT.score, "en")}`);
    const cards = [...document.querySelectorAll<HTMLElement>('[data-card="task-result"]')];
    expect(cards.map((card) => [...card.querySelectorAll<HTMLElement>("[data-crowd-figure]")].map((figure) => figure.dataset.crowdFigure))).toEqual(SHEET.tasks.map(() => ["answers", "scores"]));
    for (const card of cards) {
      const [table, plot] = within(card).getAllByRole("table");
      expect(table!.compareDocumentPosition(plot!) & Node.DOCUMENT_POSITION_FOLLOWING).toBeTruthy();
      expect(
        within(table!)
          .getAllByRole("columnheader")
          .map((head) => head.textContent),
      ).not.toContain("Everyone");
    }
    expect(screen.getByRole("table", { name: "What everyone answered: Heat" })).toBeTruthy();
    expect(screen.getByRole("table", { name: "What everyone answered: Lamps – Power" })).toBeTruthy();
    expect(screen.getByRole("list", { name: "How everyone scored: Lamps – Power" })).toBeTruthy();
    expect(said(cellOf("kettle", "power"))).toBe("75% (3 of 4), your answer, correct");
    expect(said(cellOf("fridge", "energy"))).toBe("0% (0 of 0), correct");
    expect(document.querySelector("[data-crowd-dot], [data-crowd-gate]")).toBeNull();
    const heat = RESULT.tasks.find((task) => task.task === "heat")!;
    expect(cards[0]!.querySelector("[data-own-score]")?.textContent).toBe(`● You: ${formatScore(heat.score, "en")}`);
    rerender(results("never"));
    expect(figures()).toEqual([]);
    expect(screen.getAllByRole("table")).toHaveLength(SHEET.tasks.length);
    rerender(results("always"));
    expect(figures()).toHaveLength(1 + 2 * SHEET.tasks.length);
    outsideLiveRegions();
  });

  it("shows on a quiz's page what everyone answered once the learner has submitted the quiz, and before that only when asked", () => {
    const quiz = CATALOG.quizzes[0]!;
    const page = (session: QuizSession, status: "open" | "submitted", crowds: QuizState["crowds"], asked: readonly string[] = [], others?: OthersChoice, thinking: readonly ThinkingState[] = []) =>
      thinkingAlong(thinking, <QuizPage quiz={quiz} session={session} state={{ ...state(status, crowds, asked), step: { screen: "home", page: QUIZ.id } }} text={quizText("en")} locale="en" busy={false} act={() => undefined} view={{ opened: true, revealed: false }} others={others} />);
    const card = () => document.querySelector<HTMLElement>('[data-card="quiz-crowd"]');
    const session = stubSession();
    const { rerender } = render(page(session, "open", { [QUIZ.id]: VIEW }));
    expect(within(card()!).getByRole("heading", { level: 2, name: "What everyone answered" })).toBeTruthy();
    expect(card()!.querySelector("[data-crowd-gate]")?.getAttribute("data-crowd-gate")).toBe("locked");
    expect(figures()).toEqual([]);
    fireEvent.click(within(card()!).getByRole("button", { name: "Show it now" }));
    expect(session.askCrowd.mock.calls).toEqual([[QUIZ.id]]);
    rerender(page(session, "open", { [QUIZ.id]: VIEW }, [QUIZ.id]));
    expect(within(card()!).getByRole("button", { name: "Hide it again" })).toBeTruthy();
    expect(figures().map((figure) => figure.dataset.crowdFigure)).toEqual(["scores", "answers", "answers", "answers"]);
    expect(card()!.querySelector("[data-own-score]")).toBeNull();
    rerender(page(session, "submitted", { [QUIZ.id]: VIEW }));
    expect(card()!.querySelector("[data-crowd-gate]")).toBeNull();
    expect(
      within(card()!)
        .getAllByRole("heading", { level: 3 })
        .map((heading) => heading.textContent),
    ).toEqual(["Heat", "Masses", "Lamps"]);
    expect(within(card()!).getByRole("list", { name: "How everyone scored: Household physics" })).toBeTruthy();
    expect(card()!.querySelector("[data-own-score]")?.textContent).toBe(`● You: ${formatScore(RESULT.score, "en")}`);
    expect(said(cellOf("kettle", "power"))).toBe("75% (3 of 4), your answer");
    expect(card()!.querySelector("[data-correct]")).toBeNull();
    rerender(page(session, "submitted", { [QUIZ.id]: VIEW }, [], undefined, OTHERS));
    expect(said(cellOf("kettle", "power"))).toBe("75% (3 of 4), your answer, thinking this now: 2");
    rerender(page(session, "submitted", {}));
    expect(figures()).toEqual([]);
    expect(within(card()!).getByText("Nobody has answered this yet.")).toBeTruthy();
    rerender(page(session, "open", { [QUIZ.id]: VIEW }, [], "always"));
    expect(figures()).toHaveLength(4);
    expect(card()!.querySelector("[data-crowd-gate]")).toBeNull();
    rerender(page(session, "submitted", { [QUIZ.id]: VIEW }, [QUIZ.id], "never"));
    expect(card()).toBeNull();
    outsideLiveRegions();
  });

  it("keeps on the device when the others' answers show: after the learner submitted, unless chosen otherwise", () => {
    const store = localStore(memoryStorageOrigin().tab(), "arch");
    expect(readPreferences(store).others).toBe("submitted");
    store.write("preferences", { theme: "system", textSize: "normal", showCursors: true, showAnswers: true });
    expect(readPreferences(store).others).toBe("submitted");
    expect("showAnswers" in readPreferences(store)).toBe(false);
    store.write("preferences", { theme: "system", textSize: "normal", showCursors: true, others: "sometimes" });
    expect(readPreferences(store).others).toBe("submitted");
    store.write("preferences", { theme: "system", textSize: "normal", showCursors: true, others: "always" });
    const preferences = readPreferences(store);
    expect(preferences.others).toBe("always");
    const onChange = vi.fn();
    render(<PreferencesPanel preferences={preferences} locale="en" text={quizText("en")} onChange={onChange} />);
    const choice = screen.getByRole("group", { name: "Others' answers" });
    expect(
      within(choice)
        .getAllByRole("button")
        .map((button) => [button.textContent, button.getAttribute("aria-pressed")]),
    ).toEqual([
      ["Never", "false"],
      ["After I submit", "false"],
      ["Always", "true"],
    ]);
    fireEvent.click(within(choice).getByRole("button", { name: "After I submit" }));
    expect(onChange).toHaveBeenLastCalledWith(expect.objectContaining({ others: "submitted", showCursors: true }));
    expect(screen.queryByRole("checkbox", { name: "Show what others think" })).toBeNull();
  });
});
