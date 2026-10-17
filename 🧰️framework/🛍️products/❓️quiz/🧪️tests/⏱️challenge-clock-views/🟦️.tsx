/** ⏱️ The clock of a timed task: closed until opened, running, warning, up and locked, in English and German, with the submission and the timers it leaves behind. */

import { useMemo, useState, type ReactElement } from "react";
import { act, cleanup, fireEvent, render, screen, within } from "@testing-library/react";
import { computeAccessibleDescription } from "dom-accessibility-api";
import { afterEach, describe, expect, it, vi } from "vitest";
import { type Answer, type RunView, type Sheet, type SheetClassificationTask, type SheetMatchingTask } from "@semio-tech/quiz";
import { RunScreen, clockStage, formatCountdown, formatDuration, initialQuizState, quizText, type OthersChoice, type QuizSession, type QuizText } from "@semio-tech/quiz-react";
import { en, said, sheetAt } from "../🪞️challenge-views-kit/🟦️.ts";

afterEach(() => {
  cleanup();
  vi.useRealTimers();
  vi.unstubAllGlobals();
  vi.restoreAllMocks();
});

describe("⏱️ the clock of a timed task", () => {
  it("is closed until opened, runs, warns in its last 30 and 10 seconds and is up at zero", () => {
    expect([undefined, 90_000, 30_001, 30_000, 10_001, 10_000, 1, 0].map(clockStage)).toEqual(["closed", "running", "running", "thirty", "thirty", "ten", "ten", "up"]);
  });

  /** 🎭️ A run screen over a session double whose clock the test turns: opening a task stamps it with the clock. */
  function Run(props: { readonly view: RunView; readonly clock: { now: number }; readonly calls: string[]; readonly others?: OthersChoice; readonly text?: QuizText; readonly locale?: "en" | "de" }): ReactElement {
    const { clock, calls } = props;
    const [view, setView] = useState(props.view);
    const session = useMemo(
      () =>
        ({
          now: () => clock.now,
          openTask: (run: string, task: string, signal: AbortSignal) => {
            calls.push(`open ${task} ${signal.aborted}`);
            setView((present) => ({ ...present, opened: { ...present.opened, [task]: clock.now } }));
            return Promise.resolve(undefined);
          },
          answer: (run: string, task: string, answer: Answer) => {
            calls.push(`answer ${task}`);
            setView((present) => ({ ...present, answers: { ...present.answers, [task]: answer } }));
          },
          askCrowd: () => undefined,
          unaskCrowd: () => undefined,
          submit: () => Promise.resolve(undefined),
        }) as unknown as QuizSession,
      [clock, calls],
    );
    return <RunScreen session={session} state={initialQuizState({ introduced: true, runs: { [view.run]: view } })} run={view.run} others={props.others} text={props.text ?? en} locale={props.locale ?? "en"} />;
  }

  const runOf = (sheet: Sheet, opened: Readonly<Record<string, number>> = {}): RunView => ({ run: "run-1", learner: "learner-1", quiz: sheet.quiz, status: "open", sheet, answers: {}, startedAt: 1_000_000, ...(sheet.challenge === "expert" ? { opened } : {}) });

  /** 🔔️ What the task's polite status beside its clock last said. */
  const clockSaid = (): string => document.querySelector(".quiz-clock")?.parentElement?.querySelector('[role="status"]')?.textContent ?? "";

  /** 👣️ What the step of the task at `index` says of its clock: its stage, what is shown and what is read. */
  const stepClock = (index: number): readonly (string | null | undefined)[] => {
    const state = document.querySelectorAll(".quiz-step")[index]?.querySelector(".quiz-clock-state");
    return [state?.getAttribute("data-clock-state"), state?.querySelector(":scope > [aria-hidden='true']:not(:first-child)")?.textContent ?? state?.textContent, state?.querySelector(".sr-only")?.textContent];
  };

  /** ⏩️ Turns the session clock and the fake timers on by whole seconds, one act per second. */
  const ticker =
    (clock: { now: number }) =>
    (seconds: number): void => {
      for (let step = 0; step < seconds; step += 1)
        act(() => {
          clock.now += 1000;
          vi.advanceTimersByTime(1000);
        });
    };

  /** ▶️ Starts the clock of the task shown. */
  const start = async (): Promise<void> => {
    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: "Start the clock" }));
    });
  };

  it("starts on request, ticks as text that is no live region, speaks at the start, at 30 s, at 10 s and at time up, then locks without moving focus — on a device that asks for reduced motion too", async () => {
    vi.useFakeTimers();
    vi.stubGlobal("matchMedia", (query: string) => ({ matches: query.includes("reduce"), media: query, addEventListener: () => undefined, removeEventListener: () => undefined, addListener: () => undefined, removeListener: () => undefined, onchange: null, dispatchEvent: () => false }));
    const sheet = sheetAt("expert");
    const [first, second] = sheet.tasks as readonly [SheetClassificationTask, SheetMatchingTask];
    const clock = { now: 2_000_000 };
    const calls: string[] = [];
    const advance = ticker(clock);
    const started = `The clock runs: ${first.seconds} seconds left`;
    render(<Run view={runOf(sheet)} clock={clock} calls={calls} />);
    expect(screen.getByText("Challenge: Expert")).toBeTruthy();
    expect(screen.getByText(`Time allowed: 0:${first.seconds}`)).toBeTruthy();
    expect(document.querySelector(".quiz-task")).toBeNull();
    expect(screen.queryByText(first.prompt.en)).toBeNull();
    expect(stepClock(0)).toEqual(["closed", "⏸ Clock not started", undefined]);
    await start();
    expect(calls).toEqual([`open ${first.id} false`]);
    expect(clockSaid()).toBe(started);
    expect(document.activeElement?.tagName).toMatch(/^H[1-6]$/u);
    expect(document.activeElement?.textContent).toContain(first.title.en);
    const shown = document.querySelector<HTMLElement>(".quiz-clock")!;
    expect([shown.getAttribute("data-clock"), shown.getAttribute("aria-live"), shown.getAttribute("role")]).toEqual(["running", null, "timer"]);
    expect(shown.querySelector("[aria-hidden='true']:not(:first-child)")?.textContent).toBe(`Time left: 0:${first.seconds}`);
    expect(shown.querySelector(".sr-only")?.textContent).toBe(`Time left: ${first.seconds} seconds`);
    expect(stepClock(0)).toEqual(["running", `Time left: 0:${first.seconds}`, `Time left: ${first.seconds} seconds`]);
    expect(screen.getByText(first.prompt.en)).toBeTruthy();
    advance(1);
    expect(document.querySelector(".quiz-clock")!.textContent).toContain(`Time left: 0:${first.seconds! - 1}`);
    expect(document.querySelector(".quiz-clock .sr-only")!.textContent).toBe(`Time left: ${first.seconds! - 1} seconds`);
    expect(stepClock(0)[1]).toBe(`Time left: 0:${first.seconds! - 1}`);
    expect(clockSaid()).toBe(started);
    advance(first.seconds! - 32);
    expect(document.querySelector(".quiz-clock")!.textContent).toContain("Time left: 0:31");
    expect(clockSaid()).toBe(started);
    advance(1);
    expect(document.querySelector(".quiz-clock")!.getAttribute("data-clock")).toBe("thirty");
    expect(clockSaid()).toBe("30 seconds left");
    advance(20);
    expect(clockSaid()).toBe("10 seconds left");
    const select = screen.getAllByRole("combobox")[0]! as HTMLSelectElement;
    select.focus();
    act(() => {
      fireEvent.change(select, { target: { value: first.categories[0]!.id } });
    });
    expect(calls.filter((call) => call.startsWith("answer"))).toHaveLength(1);
    const focused = document.activeElement;
    expect(focused?.tagName).toBe("SELECT");
    advance(10);
    expect(clockSaid()).toBe("Time is up: this task takes no more answers.");
    expect(document.querySelector(".quiz-clock")!.getAttribute("data-clock")).toBe("up");
    expect(document.querySelector(".quiz-clock")!.textContent).toContain("Time is up");
    expect(document.activeElement).toBe(focused);
    expect(focused?.getAttribute("aria-disabled")).toBe("true");
    act(() => {
      fireEvent.change(screen.getAllByRole("combobox")[1]!, { target: { value: first.categories[0]!.id } });
    });
    expect(calls.filter((call) => call.startsWith("answer"))).toHaveLength(1);
    advance(5);
    expect(clockSaid()).toBe("Time is up: this task takes no more answers.");
    fireEvent.click(screen.getByRole("button", { name: "Next task" }));
    expect(screen.getByText(`Time allowed: ${Math.floor(second.seconds! / 60)}:${String(second.seconds! % 60).padStart(2, "0")}`)).toBeTruthy();
    fireEvent.click(screen.getByRole("button", { name: "Previous task" }));
    expect(document.querySelector(".quiz-clock")!.getAttribute("data-clock")).toBe("up");
    expect(clockSaid()).toBe("");
  });

  it("says every stage of the clock in German: closed, started, running, 30 s, 10 s and up, with the lock that says why", async () => {
    vi.useFakeTimers();
    const sheet = sheetAt("expert");
    const first = sheet.tasks[0] as SheetClassificationTask;
    const seconds = first.seconds!;
    const clock = { now: 2_000_000 };
    const advance = ticker(clock);
    render(<Run view={runOf(sheet)} clock={clock} calls={[]} text={quizText("de")} locale="de" />);
    expect(screen.getByText("Herausforderung: Experte")).toBeTruthy();
    expect(screen.getByText(`Verfügbare Zeit: 0:${seconds}`)).toBeTruthy();
    expect(screen.getByText("Diese Aufgabe läuft gegen die Uhr. Du siehst sie, sobald du die Uhr startest.")).toBeTruthy();
    expect(stepClock(0)).toEqual(["closed", "⏸ Uhr nicht gestartet", undefined]);
    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: "Uhr starten" }));
    });
    expect(clockSaid()).toBe(`Die Uhr läuft: noch ${formatDuration(seconds * 1000, "de")}`);
    const shown = document.querySelector<HTMLElement>(".quiz-clock")!;
    expect(shown.querySelector("[aria-hidden='true']:not(:first-child)")?.textContent).toBe(`Verbleibende Zeit: 0:${seconds}`);
    expect(shown.querySelector(".sr-only")?.textContent).toBe(`Verbleibende Zeit: ${formatDuration(seconds * 1000, "de")}`);
    expect(stepClock(0)).toEqual(["running", `Verbleibende Zeit: 0:${seconds}`, `Verbleibende Zeit: ${formatDuration(seconds * 1000, "de")}`]);
    advance(seconds - 30);
    expect(clockSaid()).toBe("Noch 30 Sekunden");
    advance(20);
    expect(clockSaid()).toBe("Noch 10 Sekunden");
    advance(10);
    expect(clockSaid()).toBe("Die Zeit ist um: Diese Aufgabe nimmt keine Antworten mehr an.");
    expect(document.querySelector(".quiz-clock")!.textContent).toContain("Die Zeit ist um");
    expect(stepClock(0)[0]).toBe("up");
    expect(computeAccessibleDescription(screen.getAllByRole("combobox")[0]!)).toBe("Die Zeit ist um: Diese Aufgabe nimmt keine Antworten mehr an.");
    expect(document.body.textContent).not.toMatch(/\{\{|quiz\.[a-z]+\.[a-zA-Z]+/u);
  });

  it("lets a timed run be submitted at any time and names the tasks still open in the confirmation with where their clocks stand", () => {
    const sheet = sheetAt("expert");
    const [first, second] = sheet.tasks;
    const clock = { now: 2_000_000 };
    render(<Run view={runOf(sheet, { [first!.id]: clock.now - first!.seconds! * 1000 - 1, [second!.id]: clock.now - 5_000 })} clock={clock} calls={[]} />);
    const submit = screen.getByRole("button", { name: "Submit quiz" });
    expect(submit.getAttribute("aria-disabled")).toBe("false");
    fireEvent.click(submit);
    const dialog = screen.getByRole("alertdialog");
    expect(within(dialog).getByText("Not answered or incomplete, scored as missed:")).toBeTruthy();
    const left = formatCountdown(second!.seconds! * 1000 - 5_000);
    const words = formatDuration(second!.seconds! * 1000 - 5_000, "en");
    expect(within(dialog).getAllByRole("listitem").map((item) => item.textContent)).toEqual([
      `○ 1. ${first!.title.en} · ⌛ Time is up`,
      `○ 2. ${second!.title.en} · ⏱ Time left: ${left}Time left: ${words}`,
      ...sheet.tasks.slice(2).map((task, index) => `○ ${index + 3}. ${task.title.en} · ⏸ Clock not started`),
    ]);
  });

  it("tells once when the time of a task runs out while another is shown, and shows it in that task's step", async () => {
    vi.useFakeTimers();
    const sheet = sheetAt("expert");
    const [first, second] = sheet.tasks;
    const clock = { now: 2_000_000 };
    const advance = ticker(clock);
    render(<Run view={runOf(sheet)} clock={clock} calls={[]} />);
    await start();
    fireEvent.click(screen.getByRole("button", { name: "Next task" }));
    expect(screen.getByRole("button", { name: "Start the clock" })).toBeTruthy();
    expect(stepClock(1)[0]).toBe("closed");
    advance(first!.seconds! - 1);
    expect(stepClock(0)).toEqual(["ten", "Time left: 0:01", "Time left: 1 second"]);
    expect(said().filter((message) => message !== "")).toEqual([]);
    advance(1);
    expect(stepClock(0)).toEqual(["up", "⌛ Time is up", undefined]);
    expect(said().filter((message) => message !== "")).toEqual(["Task 1: time is up"]);
    const told = screen.getAllByRole("status").find((region) => region.textContent === "Task 1: time is up")!.firstElementChild;
    advance(3);
    expect(screen.getAllByRole("status").find((region) => region.textContent === "Task 1: time is up")!.firstElementChild).toBe(told);
    await start();
    expect(stepClock(1)[0]).toBe("running");
    advance(second!.seconds!);
    expect(said().filter((message) => message !== "")).toEqual(["Task 1: time is up", "Time is up: this task takes no more answers."]);
  });

  it("speaks nothing when it shows a clock already in its last 30 seconds or already up", () => {
    vi.useFakeTimers();
    const sheet = sheetAt("expert");
    const [first, second] = sheet.tasks;
    const clock = { now: 2_000_000 };
    render(<Run view={runOf(sheet, { [first!.id]: clock.now - (first!.seconds! - 20) * 1000, [second!.id]: clock.now - second!.seconds! * 1000 })} clock={clock} calls={[]} />);
    expect(document.querySelector(".quiz-clock")!.getAttribute("data-clock")).toBe("thirty");
    expect(said().filter((message) => message !== "")).toEqual([]);
    ticker(clock)(5);
    expect(said().filter((message) => message !== "")).toEqual([]);
    fireEvent.click(screen.getByRole("button", { name: "Next task" }));
    expect(document.querySelector(".quiz-clock")!.getAttribute("data-clock")).toBe("up");
    expect(second!.kind).toBe("matching");
    expect(screen.getAllByRole("textbox").map((field) => (field as HTMLInputElement).readOnly)).not.toContain(false);
    expect(said().filter((message) => message !== "")).toEqual([]);
  });

  it("takes no answer between the deadline and the lock, and says the entry was lost", async () => {
    vi.useFakeTimers();
    const sheet = sheetAt("expert");
    const first = sheet.tasks[0] as SheetClassificationTask;
    const clock = { now: 2_000_000 };
    const calls: string[] = [];
    render(<Run view={runOf(sheet)} clock={clock} calls={calls} />);
    await start();
    ticker(clock)(first.seconds! - 1);
    clock.now += 1000;
    act(() => {
      fireEvent.change(screen.getAllByRole("combobox")[0]!, { target: { value: first.categories[0]!.id } });
    });
    expect(calls.filter((call) => call.startsWith("answer"))).toEqual([]);
    expect(screen.getByText("Your last entry was not saved.")).toBeTruthy();
    act(() => {
      vi.advanceTimersByTime(1000);
    });
    expect(clockSaid()).toBe("Time is up: this task takes no more answers. Your last entry was not saved.");
  });

  it("loses a guess typed but not committed when the time runs out, and says so", async () => {
    vi.useFakeTimers();
    const sheet = sheetAt("expert");
    const matching = sheet.tasks.findIndex((task) => task.kind === "matching");
    const task = sheet.tasks[matching]!;
    const clock = { now: 2_000_000 };
    const calls: string[] = [];
    render(<Run view={runOf(sheet, { [task.id]: clock.now })} clock={clock} calls={calls} />);
    for (let step = 0; step < matching; step += 1) fireEvent.click(screen.getByRole("button", { name: "Next task" }));
    fireEvent.change(screen.getAllByRole("textbox")[0]!, { target: { value: "12 W" } });
    ticker(clock)(task.seconds!);
    expect(calls).toEqual([]);
    expect(clockSaid()).toBe("Time is up: this task takes no more answers. Your last entry was not saved.");
    expect((screen.getAllByRole("textbox")[0] as HTMLInputElement).value).toBe("");
  });

  it("looks at the clock again when the page shows again, though no timer fired while it was hidden", async () => {
    vi.useFakeTimers();
    const sheet = sheetAt("expert");
    const first = sheet.tasks[0]!;
    const clock = { now: 2_000_000 };
    render(<Run view={runOf(sheet)} clock={clock} calls={[]} />);
    await start();
    clock.now += first.seconds! * 1000;
    expect(document.querySelector(".quiz-clock")!.getAttribute("data-clock")).toBe("running");
    act(() => {
      document.dispatchEvent(new Event("visibilitychange"));
    });
    expect(document.querySelector(".quiz-clock")!.getAttribute("data-clock")).toBe("up");
    expect(stepClock(0)[0]).toBe("up");
    expect(clockSaid()).toBe("Time is up: this task takes no more answers.");
  });

  it("ends every timer and listener of its clocks with the screen, also while a clock runs", async () => {
    vi.useFakeTimers();
    const added = vi.spyOn(document, "addEventListener");
    const removed = vi.spyOn(document, "removeEventListener");
    const visibility = (spy: typeof added): number => spy.mock.calls.filter(([type]) => type === "visibilitychange").length;
    const clock = { now: 2_000_000 };
    const { unmount } = render(<Run view={runOf(sheetAt("expert"))} clock={clock} calls={[]} />);
    await start();
    ticker(clock)(3);
    expect(document.querySelector(".quiz-clock")!.getAttribute("data-clock")).toBe("running");
    expect(visibility(added)).toBeGreaterThanOrEqual(4);
    unmount();
    expect(visibility(removed)).toBe(visibility(added));
    act(() => {
      vi.runOnlyPendingTimers();
    });
    expect(vi.getTimerCount()).toBe(0);
  });

  it("ticks only the clocks: the task renders again once, when its time is up", async () => {
    vi.useFakeTimers();
    let renders = 0;
    const counting: QuizText = (key, values) => {
      if (key === "quiz.classification.categories") renders += 1;
      return en(key, values);
    };
    const sheet = sheetAt("expert");
    const first = sheet.tasks[0]!;
    expect(first.kind).toBe("classification");
    const clock = { now: 2_000_000 };
    render(<Run view={runOf(sheet)} clock={clock} calls={[]} text={counting} />);
    await start();
    const before = renders;
    ticker(clock)(first.seconds! - 1);
    expect(renders).toBe(before);
    ticker(clock)(1);
    expect(renders).toBe(before + 1);
  });

  it("keeps an untimed run as it was: no clock, and no submission before every task is complete", () => {
    render(<Run view={runOf(sheetAt("medium"))} clock={{ now: 2_000_000 }} calls={[]} />);
    expect(screen.getByText("Challenge: Medium")).toBeTruthy();
    expect(document.querySelector(".quiz-clock")).toBeNull();
    expect(screen.queryByRole("button", { name: "Start the clock" })).toBeNull();
    expect(screen.getByRole("button", { name: "Submit quiz" }).getAttribute("aria-disabled")).toBe("true");
  });

  it("offers the others' answers during a run that shows the keys, never during one that hides them", () => {
    const { unmount } = render(<Run view={runOf(sheetAt("medium"))} clock={{ now: 2_000_000 }} calls={[]} others="submitted" />);
    expect(document.querySelector("[data-crowd-gate]")).not.toBeNull();
    unmount();
    for (const challenge of ["hard", "expert"] as const) {
      const shown = render(<Run view={runOf(sheetAt(challenge))} clock={{ now: 2_000_000 }} calls={[]} others="always" />);
      expect(document.querySelector("[data-crowd-gate]"), challenge).toBeNull();
      expect(document.querySelector("[data-crowd]"), challenge).toBeNull();
      shown.unmount();
    }
  });
});
