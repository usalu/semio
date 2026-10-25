/** 🟠️ The shared declarations of the ⏱️ the clock of a timed task suite tests. */
import { useMemo, useState, type ReactElement } from "react";
import { act, fireEvent, screen } from "@testing-library/react";
import { vi } from "vitest";
import { type Answer, type RunView, type Sheet } from "@semio-tech/course";
import { initialCourseState, type OthersChoice, type CourseSession, type CourseText } from "@semio-tech/course-react";
import { RunScreen } from "@semio-tech/course-react/modules/run/.tsx";
import { en } from "../../🪞️challenge-views-kit/🟦️.ts";

/** 🎭️ A run screen over a session double whose clock the test turns: opening a task stamps it with the clock. */
export function Run(props: { readonly view: RunView; readonly clock: { now: number }; readonly calls: string[]; readonly others?: OthersChoice; readonly text?: CourseText; readonly locale?: "en" | "de" }): ReactElement {
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
      }) as unknown as CourseSession,
    [clock, calls],
  );
  return <RunScreen session={session} state={initialCourseState({ introduced: true, runs: { [view.run]: view } })} run={view.run} others={props.others} text={props.text ?? en} locale={props.locale ?? "en"} />;
}

export const runOf = (sheet: Sheet, opened: Readonly<Record<string, number>> = {}): RunView => ({ run: "run-1", learner: "learner-1", quiz: sheet.quiz, status: "open", sheet, answers: {}, startedAt: 1_000_000, ...(sheet.challenge === "expert" ? { opened } : {}) });

/** 🔔️ What the task's polite status beside its clock last said. */
export const clockSaid = (): string => document.querySelector(".course-clock")?.parentElement?.querySelector('[role="status"]')?.textContent ?? "";

/** 👣️ What the step of the task at `index` says of its clock: its stage, what is shown and what is read. */
export const stepClock = (index: number): readonly (string | null | undefined)[] => {
  const state = document.querySelectorAll(".course-step")[index]?.querySelector(".course-clock-state");
  return [state?.getAttribute("data-clock-state"), state?.querySelector(":scope > [aria-hidden='true']:not(:first-child)")?.textContent ?? state?.textContent, state?.querySelector(".sr-only")?.textContent];
};

/** ⏩️ Turns the session clock and the fake timers on by whole seconds, one act per second. */
export const ticker =
  (clock: { now: number }) =>
  (seconds: number): void => {
    for (let step = 0; step < seconds; step += 1)
      act(() => {
        clock.now += 1000;
        vi.advanceTimersByTime(1000);
      });
  };

/** ▶️ Starts the clock of the task shown. */
export const start = async (): Promise<void> => {
  await act(async () => {
    fireEvent.click(screen.getByRole("button", { name: "Start the clock" }));
  });
};
