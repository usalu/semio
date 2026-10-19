/** 🪜️ The challenges in the client. The shared sheets of one seed at every challenge show each task kind with its keys
 * (easy, medium) or ask for guesses (hard, expert); guesses complete an answer; a key is read with its place; an easy
 * run's hints are questions about two concrete items in every wording branch, in English and German with exact
 * strings — named by their short forms, counts in words up to a power of ten, a reversed pair by its order alone, the
 * quantity named where a matching has several — stand beside the item they doubt with a question mark in a circle, describe its controls and are spoken
 * once when they appear or their question changes — several at once by their count; a timed task runs behind its clock —
 * a timer read in words, it starts on request, speaks at the start, at 30 s, at 10 s and when the time is up (never
 * when shown already late), ticks whatever the device says about motion and without rendering the task again, looks
 * again when the page shows, turns read-only at the end without moving focus, says when an entry was lost, and ends its
 * timers with the screen; every timed task's step shows its clock, and a task whose time runs out while another is
 * shown is told once; a timed run can be submitted at any time naming the tasks still open and their clocks; locked
 * controls say why; during a run that hides the keys the others' answers are not offered; and the results name the
 * challenge and the points, show guesses with their miss mark and what was not answered. The tolerance factor the
 * results name is worked out independently of the core (square root of the spread's ratio, at most 1000) and cut down to two digits
 * by plain arithmetic; accessible descriptions are computed by `dom-accessibility-api`.
 *
 * @see ../../🧫️fixtures/🃏️sheet-assembly/🔣️.json — `icons-1` at every challenge
 * @see ../../🎯️targets/⚛️react/🔨️modules/▶️run/🟦️.tsx — the clock
 * @see ../💬️challenge-hint-views/🟦️.tsx — the hints, in English and German
 * @see ../⏱️challenge-clock-views/🟦️.tsx — the clock of a timed task
 * @see ../🪞️challenge-views-kit/🟦️.ts — the shared fixtures
 */

import { useState, type ReactElement } from "react";
import { act, cleanup, fireEvent, render, screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { computeAccessibleDescription } from "dom-accessibility-api";
import { afterEach, describe, expect, it, vi } from "vitest";
import { answerComplete, type Answer, type RunResult, type RunView, type Sheet, type SheetClassificationTask, type SheetMatchingTask, type SheetSortingTask } from "@semio-tech/quiz";
import { ClassificationTaskView, SortingTaskView, formatQuantity, formatScore, initialQuizState, quizText, type QuizSession } from "@semio-tech/quiz-react";
import { TaskView } from "@semio-tech/quiz-react/modules/run/.tsx";
import { ResultsScreen } from "@semio-tech/quiz-react/modules/results/.tsx";
import { en, factorOf, LAMPS, MASSES, NBSP, PLACES, sheetAt, TEMPERATURES, text } from "../🪞️challenge-views-kit/🟦️.ts";

afterEach(() => {
  cleanup();
  vi.useRealTimers();
  vi.unstubAllGlobals();
  vi.restoreAllMocks();
});

describe("🪜️ every task kind at every challenge", () => {
  for (const challenge of ["easy", "medium", "hard", "expert"] as const) {
    it(`shows the keys or asks for guesses on ${challenge}`, () => {
      const shown = challenge === "easy" || challenge === "medium";
      for (const task of sheetAt(challenge).tasks) {
        const { unmount } = render(<TaskView task={task} answer={undefined} onAnswer={() => undefined} text={en} locale="en" />);
        if (task.kind === "sorting") {
          expect(task.keys !== undefined, task.id).toBe(shown);
          expect([...document.querySelectorAll(".quiz-sort-key")].map((key) => key.textContent)).toEqual(shown ? task.keys!.map((key) => formatQuantity(key, task.quantity, "en")) : []);
          expect(screen.queryAllByRole("textbox")).toHaveLength(shown ? 0 : task.items.length);
          expect(screen.queryAllByRole("button", { name: /^Move / })).toHaveLength(shown ? 2 * task.items.length : 0);
          if (!shown) expect(screen.getByText(`Guessed: 0 of ${task.items.length}`)).toBeTruthy();
        }
        if (task.kind === "matching") {
          expect(screen.queryAllByRole("list", { name: /^Value cards: / })).toHaveLength(shown ? task.dimensions.length : 0);
          expect(screen.queryAllByRole("combobox")).toHaveLength(shown ? task.dimensions.length * task.items.length : 0);
          expect(screen.queryAllByRole("textbox")).toHaveLength(shown ? 0 : task.dimensions.length * task.items.length);
          expect(document.querySelectorAll('.quiz-match[data-keys="hidden"]')).toHaveLength(shown ? 0 : task.dimensions.length);
        }
        if (task.kind === "classification") expect(screen.getAllByRole("combobox")).toHaveLength(task.items.length);
        unmount();
      }
    });
  }

  it("completes a sorting and a matching whose keys are hidden once every item has a guess", async () => {
    const user = userEvent.setup();
    const hard = sheetAt("hard");
    for (const task of hard.tasks.filter((candidate) => candidate.kind !== "classification")) {
      const answers: Answer[] = [];
      function Harness(): ReactElement {
        const [answer, setAnswer] = useState<Answer | undefined>(undefined);
        return (
          <TaskView
            task={task}
            answer={answer}
            onAnswer={(next) => {
              answers.push(next);
              setAnswer(next);
            }}
            text={en}
            locale="en"
          />
        );
      }
      const { unmount } = render(<Harness />);
      const fields = screen.getAllByRole("textbox");
      for (const [index, field] of fields.entries()) {
        expect(answerComplete(task, answers.at(-1))).toBe(false);
        await user.type(screen.getAllByRole("textbox")[index] ?? field, `${index + 2}{Enter}`);
      }
      expect(answerComplete(task, answers.at(-1)), task.id).toBe(true);
      expect(Object.keys(answers.at(-1)!)).toEqual(["kind", task.kind === "sorting" ? "order" : "guesses", ...(task.kind === "sorting" ? ["guesses"] : [])]);
      unmount();
    }
  });
});

describe("🔒️ a task whose time is up", () => {
  it("keeps its guesses read-only and takes none", async () => {
    const answers: Answer[] = [];
    const { keys: _keys, ...hidden } = MASSES;
    render(<SortingTaskView task={hidden} answer={{ kind: "sorting", order: ["horse", "mouse", "cat"], guesses: { horse: 500_000 } }} locked onAnswer={(answer) => answers.push(answer)} text={en} locale="en" />);
    const horse = screen.getByRole("textbox", { name: "Guess for Horse" }) as HTMLInputElement;
    expect(horse.readOnly).toBe(true);
    const user = userEvent.setup();
    await user.type(horse, "9{Enter}");
    await user.type(screen.getByRole("textbox", { name: "Guess for Cat" }), "4 kg{Enter}");
    expect(horse.value).toBe(`500${NBSP}kg`);
    expect(answers).toEqual([]);
  });

  it("refuses to move by its buttons, saying why, and keeps focus on the button", async () => {
    const answers: Answer[] = [];
    render(<SortingTaskView task={MASSES} answer={undefined} locked onAnswer={(answer) => answers.push(answer)} text={en} locale="en" />);
    expect(screen.queryByRole("button", { name: "Keep this order" })).toBeNull();
    const down = screen.getByRole("button", { name: "Move Horse down" });
    expect(down.getAttribute("aria-disabled")).toBe("true");
    down.focus();
    await userEvent.setup().keyboard("{Enter}");
    expect(answers).toEqual([]);
    expect(screen.getAllByRole("status")[0]!.textContent).toBe("Time is up: this task takes no more answers.");
    expect(document.activeElement).toBe(screen.getByRole("button", { name: "Move Horse down" }));
    expect(computeAccessibleDescription(screen.getByRole("button", { name: "Move Horse down" }))).toBe("Time is up: this task takes no more answers.");
  });

  it("describes its locked selects by why, and says it again when the arrow keys try to change one", () => {
    vi.useFakeTimers();
    const answers: Answer[] = [];
    render(<ClassificationTaskView task={PLACES} answer={{ kind: "classification", assignments: { desert: "dry" } }} locked onAnswer={(answer) => answers.push(answer)} text={en} locale="en" />);
    const select = screen.getByRole("combobox", { name: "Category for Desert" }) as HTMLSelectElement;
    expect(select.getAttribute("aria-disabled")).toBe("true");
    expect(computeAccessibleDescription(select)).toBe("Time is up: this task takes no more answers.");
    act(() => {
      fireEvent.change(select, { target: { value: "wet" } });
      vi.advanceTimersByTime(400);
    });
    expect(answers).toEqual([]);
    expect(select.value).toBe("dry");
    expect(screen.getAllByRole("status")[0]!.textContent).toBe("Time is up: this task takes no more answers.");
  });

  it("hands on a guess typed but not committed when the lock comes, and loses one that does not read as a guess", async () => {
    const { keys: _keys, ...hidden } = MASSES;
    const answers: Answer[] = [];
    let dropped = 0;
    const view = (locked: boolean): ReactElement => <SortingTaskView task={hidden} answer={undefined} locked={locked} onAnswer={(answer) => answers.push(answer)} onDropped={() => (dropped += 1)} text={en} locale="en" />;
    const shown = render(view(false));
    const user = userEvent.setup();
    await user.type(screen.getByRole("textbox", { name: "Guess for Horse" }), "500 kg");
    shown.rerender(view(true));
    expect(answers).toEqual([{ kind: "sorting", order: ["horse", "mouse", "cat"], guesses: { horse: 500_000 } }]);
    expect(dropped).toBe(0);
    expect(screen.getAllByRole("status")[0]!.textContent).toBe("");
    shown.unmount();
    const again = render(view(false));
    await user.type(screen.getByRole("textbox", { name: "Guess for Cat" }), "heavy");
    again.rerender(view(true));
    expect(answers).toHaveLength(1);
    expect(dropped).toBe(1);
    expect((screen.getByRole("textbox", { name: "Guess for Cat" }) as HTMLInputElement).value).toBe("");
  });
});

describe("🏁️ results by challenge", () => {
  const submitted = (sheet: Sheet, result: RunResult): RunView => ({ run: "run-1", learner: "learner-1", quiz: sheet.quiz, status: "submitted", sheet, answers: {}, result, startedAt: 1_000_000, submittedAt: 1_100_000 });
  const results = (view: RunView): void => {
    render(<ResultsScreen session={{ open: () => undefined } as unknown as QuizSession} state={initialQuizState({ introduced: true, runs: { [view.run]: view } })} run={view.run} others="never" text={en} locale="en" />);
  };
  const columns = (table: HTMLElement): readonly string[] => within(table).getAllByRole("columnheader").map((head) => head.textContent ?? "");
  const cellsOf = (table: HTMLElement, label: string): readonly string[] => [...table.querySelectorAll(`td[data-label="${label}"]`)].map((cell) => cell.textContent ?? "");
  const verdictsOf = (table: HTMLElement, label: string): readonly string[] =>
    [...table.querySelectorAll(`td[data-label="${label}"]`)].map((cell) => {
      const copy = cell.cloneNode(true) as HTMLElement;
      copy.querySelector("[data-off]")?.remove();
      return copy.textContent ?? "";
    });
  const offsOf = (table: HTMLElement): readonly string[] => [...table.querySelectorAll("[data-off]")].map((off) => off.textContent ?? "");

  it("names the challenge and the points, shows guesses with a miss mark where the keys were hidden and what was left unanswered", () => {
    const sheet = sheetAt("expert");
    const [sources, appliances, masses] = sheet.tasks as readonly [SheetClassificationTask, SheetMatchingTask, SheetSortingTask];
    const category = sources.categories[0]!.id;
    const result: RunResult = {
      quiz: sheet.quiz,
      challenge: "expert",
      score: 0.652,
      points: 260.8,
      tasks: [
        { kind: "classification", task: sources.id, score: 2 / 3, items: sources.items.map((item, index) => (index === 0 ? { item: item.id, correct: category, credit: 0 } : { item: item.id, assigned: category, correct: category, credit: 1 })) },
        {
          kind: "matching",
          task: appliances.id,
          score: 1 / 3,
          dimensions: appliances.dimensions.map((dimension) => ({
            dimension: dimension.id,
            score: 1 / 3,
            items: appliances.items.map((item, index) => (index === 0 ? { item: item.id, assigned: 12, correct: 10, miss: false } : index === 1 ? { item: item.id, assigned: 50_000, correct: 10, miss: true } : { item: item.id, correct: 10, miss: true })),
          })),
        },
        {
          kind: "sorting",
          task: masses.id,
          score: 0,
          items: masses.items.map((item, index) => ({ item: item.id, value: 10 ** index, position: index, rank: index, miss: index !== 0, ...(index === 2 ? {} : { guess: index === 0 ? 1 : 1e6 }) })),
        },
      ],
    };
    results(submitted(sheet, result));
    expect(screen.getByText(`Your score: ${formatScore(0.652, "en")}`)).toBeTruthy();
    expect(screen.getByText("Expert · Points: 260.8 of 400")).toBeTruthy();
    const tables = screen.getAllByRole("table");
    const [classification, ...rest] = tables;
    expect(cellsOf(classification!, "Your answer")[0]).toBe("Not answered");
    const sorting = rest.at(-1)!;
    expect(columns(sorting)).toContain("Your guess");
    expect(verdictsOf(sorting, "Your guess")).toEqual([`${formatQuantity(1, masses.quantity, "en")} ≈ Not far off`, `${formatQuantity(1e6, masses.quantity, "en")} ≉ Far off`, "Not answered"]);
    expect(offsOf(sorting)).toEqual(["×1 from the true value", "×100,000 from the true value"]);
    for (const [index, dimension] of appliances.dimensions.entries()) {
      const table = rest[index]!;
      expect(columns(table)).toEqual(["Item", "Your guess", "Solution", "Explanation"]);
      expect(verdictsOf(table, "Your guess")).toEqual([`${formatQuantity(12, dimension.quantity, "en")} ≈ Not far off`, `${formatQuantity(50_000, dimension.quantity, "en")} ≉ Far off`, "Not answered"]);
      expect(offsOf(table)).toHaveLength(2);
    }
    expect([...document.querySelectorAll(".quiz-miss")].map((mark) => mark.textContent)).toEqual([...appliances.dimensions.map(() => "≉ Far off"), "≉ Far off"]);
    for (const mark of document.querySelectorAll(".quiz-miss [aria-hidden]")) expect(mark.textContent).toBe("≉ ");
  });

  it("says where the keys were hidden how far a guess may lie off — ×N on a logarithmic scale, ±d on a linear one — and how far each guess lay, in English and German", () => {
    const { keys: _masses, ...masses } = MASSES;
    const { keys: _temperatures, ...temperatures } = TEMPERATURES;
    const lamps: SheetMatchingTask = { ...LAMPS, dimensions: LAMPS.dimensions.map(({ cards: _cards, ...dimension }) => dimension) };
    const sheet = { quiz: "guesses", seed: 7, challenge: "hard", title: text("Guesses", "Schätzungen"), description: text("Guess.", "Schätze."), tasks: [masses, temperatures, lamps] } as Sheet;
    const result: RunResult = {
      quiz: "guesses",
      challenge: "hard",
      score: 0.5,
      points: 150,
      tasks: [
        { kind: "sorting", task: "masses", score: 0.5, items: [{ item: "mouse", value: 20, position: 0, rank: 0, guess: 30, miss: false }, { item: "cat", value: 4000, position: 1, rank: 1, guess: 4000, miss: false }, { item: "horse", value: 500_000, position: 2, rank: 2, guess: 5e9, miss: true }] },
        { kind: "sorting", task: "temperatures", score: 0.5, items: [{ item: "ice", value: 270, position: 0, rank: 0, guess: 280, miss: false }, { item: "room", value: 293, position: 1, rank: 1, miss: true }, { item: "tea", value: 350, position: 2, rank: 2, guess: 600, miss: true }] },
        { kind: "matching", task: "lamps", score: 0.5, dimensions: [{ dimension: "power", score: 0.5, items: [{ item: "led", assigned: 10, correct: 8, miss: false }, { item: "halogen", assigned: 5000, correct: 50, miss: true }, { item: "floodlight", correct: 2000, miss: true }] }] },
      ],
    };
    const kelvin = (amount: number, locale: "en" | "de"): string => formatQuantity(amount, TEMPERATURES.quantity, locale);
    const cases = [
      { locale: "en", within: (bound: string) => `A guess counts within ${bound} of the true value.`, off: (amount: string) => `${amount} from the true value`, miss: "≉ Far off", unanswered: "Not answered" },
      { locale: "de", within: (bound: string) => `Ein Schätzwert zählt, wenn er höchstens ${bound} vom wahren Wert abweicht.`, off: (amount: string) => `${amount} vom wahren Wert entfernt`, miss: "≉ Weit daneben", unanswered: "Nicht beantwortet" },
    ] as const;
    for (const { locale, within: line, off, miss, unanswered } of cases) {
      const shown = render(<ResultsScreen session={{ open: () => undefined } as unknown as QuizSession} state={initialQuizState({ introduced: true, runs: { "run-1": submitted(sheet, result) } })} run="run-1" others="never" text={quizText(locale)} locale={locale} />);
      const lines = [`×${factorOf([20, 4000, 500_000], locale)}`, `±${kelvin((350 - 270) / 2, locale)}`, `×${factorOf([8, 50, 2000], locale)}`].map(line);
      expect([...document.querySelectorAll("[data-tolerance]")].map((note) => note.textContent), locale).toEqual(lines);
      expect(screen.getAllByRole("table").map((table) => computeAccessibleDescription(table)), locale).toEqual(lines);
      expect(offsOf(document.body), locale).toEqual([`×${(30 / 20).toLocaleString(locale)}`, "×1", `×${(10_000).toLocaleString(locale)}`, kelvin(280 - 270, locale), kelvin(600 - 350, locale), `×${(1.2).toLocaleString(locale)}`, `×${(5000 / 50).toLocaleString(locale)}`].map(off));
      expect([...document.querySelectorAll(".quiz-miss")].map((mark) => mark.textContent), locale).toEqual([miss, miss, miss]);
      expect(screen.getAllByText(unanswered), locale).toHaveLength(2);
      expect(document.body.textContent, locale).not.toMatch(/\{\{|quiz\.[a-z]+\.[a-zA-Z]+/u);
      shown.unmount();
    }
  });

  it("reads every position of a timed sorting nobody ordered as not answered, with no verdict on it", () => {
    const sheet = sheetAt("expert");
    const masses = sheet.tasks.find((task): task is SheetSortingTask => task.kind === "sorting")!;
    const result: RunResult = { quiz: sheet.quiz, challenge: "expert", score: 0, points: 0, tasks: [{ kind: "sorting", task: masses.id, score: 0, items: masses.items.map((item, index) => ({ item: item.id, value: 10 ** index, position: index, rank: masses.items.length - 1 - index, miss: true })) }] };
    results(submitted(sheet, result));
    const table = screen.getByRole("table");
    expect(cellsOf(table, "Your position")).toEqual(masses.items.map(() => "Not answered"));
    expect(cellsOf(table, "Correct position")).toEqual(masses.items.map((_, index) => String(masses.items.length - index)));
  });

  it("names the credit of an item and the points of a run by different German words", () => {
    const de = quizText("de");
    expect(de("quiz.results.credit")).toBe("Beurteilung");
    expect(de("quiz.challenge.scored", { challenge: "Schwer", points: 1, par: 300 })).toContain("Punkte");
    expect(de("quiz.results.credit")).not.toMatch(/Punkt/u);
  });

  it("shows no guess where the keys were shown", () => {
    const sheet = sheetAt("medium");
    const masses = sheet.tasks.find((task): task is SheetSortingTask => task.kind === "sorting")!;
    const result: RunResult = { quiz: sheet.quiz, challenge: "medium", score: 1, points: 200, tasks: [{ kind: "sorting", task: masses.id, score: 1, items: masses.items.map((item, index) => ({ item: item.id, value: masses.keys![index]!, position: index, rank: index })) }] };
    results(submitted(sheet, result));
    expect(screen.getByText("Medium · Points: 200 of 200")).toBeTruthy();
    expect(columns(screen.getByRole("table"))).not.toContain("Your guess");
    expect(document.querySelector(".quiz-miss")).toBeNull();
  });
});
