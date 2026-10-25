/** 🔶️ The closed until opened runs tests of the ⏱️ the clock of a timed task suite. */
import { act, fireEvent, render, screen, within } from "@testing-library/react";
import { computeAccessibleDescription } from "dom-accessibility-api";
import { expect, it, vi } from "vitest";
import { type SheetClassificationTask, type SheetMatchingTask } from "@semio-tech/course";
import { formatCountdown, formatDuration, courseText } from "@semio-tech/course-react";
import { clockStage } from "@semio-tech/course-react/modules/run/.tsx";
import { sheetAt } from "../../🪞️challenge-views-kit/🟦️.ts";
import { Run, clockSaid, runOf, start, stepClock, ticker } from "../🟠️harness/🟦️.tsx";

/** 🔷 Registers the closed until opened runs tests of the ⏱️ the clock of a timed task suite. */
export function registerClosedUntilOpenedRunsTests(): void {
  it("is closed until opened, runs, warns in its last 30 and 10 seconds and is up at zero", () => {
    expect([undefined, 90_000, 30_001, 30_000, 10_001, 10_000, 1, 0].map(clockStage)).toEqual(["closed", "running", "running", "thirty", "thirty", "ten", "ten", "up"]);
  });
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
    expect(document.querySelector(".course-task")).toBeNull();
    expect(screen.queryByText(first.prompt.en)).toBeNull();
    expect(stepClock(0)).toEqual(["closed", "⏸ Clock not started", undefined]);
    await start();
    expect(calls).toEqual([`open ${first.id} false`]);
    expect(clockSaid()).toBe(started);
    expect(document.activeElement?.tagName).toMatch(/^H[1-6]$/u);
    expect(document.activeElement?.textContent).toContain(first.title.en);
    const shown = document.querySelector<HTMLElement>(".course-clock")!;
    expect([shown.getAttribute("data-clock"), shown.getAttribute("aria-live"), shown.getAttribute("role")]).toEqual(["running", null, "timer"]);
    expect(shown.querySelector("[aria-hidden='true']:not(:first-child)")?.textContent).toBe(`Time left: 0:${first.seconds}`);
    expect(shown.querySelector(".sr-only")?.textContent).toBe(`Time left: ${first.seconds} seconds`);
    expect(stepClock(0)).toEqual(["running", `Time left: 0:${first.seconds}`, `Time left: ${first.seconds} seconds`]);
    expect(screen.getByText(first.prompt.en)).toBeTruthy();
    advance(1);
    expect(document.querySelector(".course-clock")!.textContent).toContain(`Time left: 0:${first.seconds! - 1}`);
    expect(document.querySelector(".course-clock .sr-only")!.textContent).toBe(`Time left: ${first.seconds! - 1} seconds`);
    expect(stepClock(0)[1]).toBe(`Time left: 0:${first.seconds! - 1}`);
    expect(clockSaid()).toBe(started);
    advance(first.seconds! - 32);
    expect(document.querySelector(".course-clock")!.textContent).toContain("Time left: 0:31");
    expect(clockSaid()).toBe(started);
    advance(1);
    expect(document.querySelector(".course-clock")!.getAttribute("data-clock")).toBe("thirty");
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
    expect(document.querySelector(".course-clock")!.getAttribute("data-clock")).toBe("up");
    expect(document.querySelector(".course-clock")!.textContent).toContain("Time is up");
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
    expect(document.querySelector(".course-clock")!.getAttribute("data-clock")).toBe("up");
    expect(clockSaid()).toBe("");
  });
  it("says every stage of the clock in German: closed, started, running, 30 s, 10 s and up, with the lock that says why", async () => {
    vi.useFakeTimers();
    const sheet = sheetAt("expert");
    const first = sheet.tasks[0] as SheetClassificationTask;
    const seconds = first.seconds!;
    const clock = { now: 2_000_000 };
    const advance = ticker(clock);
    render(<Run view={runOf(sheet)} clock={clock} calls={[]} text={courseText("de")} locale="de" />);
    expect(screen.getByText("Herausforderung: Experte")).toBeTruthy();
    expect(screen.getByText(`Verfügbare Zeit: 0:${seconds}`)).toBeTruthy();
    expect(screen.getByText("Diese Aufgabe läuft gegen die Uhr. Du siehst sie, sobald du die Uhr startest.")).toBeTruthy();
    expect(stepClock(0)).toEqual(["closed", "⏸ Uhr nicht gestartet", undefined]);
    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: "Uhr starten" }));
    });
    expect(clockSaid()).toBe(`Die Uhr läuft: noch ${formatDuration(seconds * 1000, "de")}`);
    const shown = document.querySelector<HTMLElement>(".course-clock")!;
    expect(shown.querySelector("[aria-hidden='true']:not(:first-child)")?.textContent).toBe(`Verbleibende Zeit: 0:${seconds}`);
    expect(shown.querySelector(".sr-only")?.textContent).toBe(`Verbleibende Zeit: ${formatDuration(seconds * 1000, "de")}`);
    expect(stepClock(0)).toEqual(["running", `Verbleibende Zeit: 0:${seconds}`, `Verbleibende Zeit: ${formatDuration(seconds * 1000, "de")}`]);
    advance(seconds - 30);
    expect(clockSaid()).toBe("Noch 30 Sekunden");
    advance(20);
    expect(clockSaid()).toBe("Noch 10 Sekunden");
    advance(10);
    expect(clockSaid()).toBe("Die Zeit ist um: Diese Aufgabe nimmt keine Antworten mehr an.");
    expect(document.querySelector(".course-clock")!.textContent).toContain("Die Zeit ist um");
    expect(stepClock(0)[0]).toBe("up");
    expect(computeAccessibleDescription(screen.getAllByRole("combobox")[0]!)).toBe("Die Zeit ist um: Diese Aufgabe nimmt keine Antworten mehr an.");
    expect(document.body.textContent).not.toMatch(/\{\{|course\.[a-z]+\.[a-zA-Z]+/u);
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
}
