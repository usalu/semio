/** 🔸️ The tells once time task tests of the ⏱️ the clock of a timed task suite. */
import { act, fireEvent, render, screen, within } from "@testing-library/react";
import { expect, it, vi } from "vitest";
import { type SheetClassificationTask } from "@semio-tech/course";
import { type CourseText } from "@semio-tech/course-react";
import { en, said, sheetAt } from "../../🪞️challenge-views-kit/🟦️.ts";
import { Run, clockSaid, runOf, start, stepClock, ticker } from "../🟠️harness/🟦️.tsx";

/** 🔹 Registers the tells once time task tests of the ⏱️ the clock of a timed task suite. */
export function registerTellsOnceTimeTaskTests(): void {
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
    expect(document.querySelector(".course-clock")!.getAttribute("data-clock")).toBe("thirty");
    expect(said().filter((message) => message !== "")).toEqual([]);
    ticker(clock)(5);
    expect(said().filter((message) => message !== "")).toEqual([]);
    fireEvent.click(screen.getByRole("button", { name: "Next task" }));
    expect(document.querySelector(".course-clock")!.getAttribute("data-clock")).toBe("up");
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
    expect(document.querySelector(".course-clock")!.getAttribute("data-clock")).toBe("running");
    act(() => {
      document.dispatchEvent(new Event("visibilitychange"));
    });
    expect(document.querySelector(".course-clock")!.getAttribute("data-clock")).toBe("up");
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
    expect(document.querySelector(".course-clock")!.getAttribute("data-clock")).toBe("running");
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
    const counting: CourseText = (key, values) => {
      if (key === "course.classification.categories") renders += 1;
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
  it("offers an untimed run's submission with every incomplete task named before confirming", () => {
    render(<Run view={runOf(sheetAt("medium"))} clock={{ now: 2_000_000 }} calls={[]} />);
    expect(screen.getByText("Challenge: Medium")).toBeTruthy();
    expect(document.querySelector(".course-clock")).toBeNull();
    expect(screen.queryByRole("button", { name: "Start the clock" })).toBeNull();
    const submit = screen.getByRole("button", { name: "Submit quiz" });
    expect(submit.getAttribute("aria-disabled")).toBe("false");
    fireEvent.click(submit);
    const dialog = screen.getByRole("alertdialog", { name: "Submit this quiz?" });
    expect(within(dialog).getByText("Not answered or incomplete, scored as missed:")).toBeTruthy();
    expect(within(dialog).getAllByRole("listitem")).toHaveLength(sheetAt("medium").tasks.length);
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
}
