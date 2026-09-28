import { afterEach, expect, it, vi } from "vitest";
import Ajv2020 from "ajv/dist/2020";
import fixture from "../../../../../../../🔨️modules/🖱️ui/🖌️render/🧫️fixtures/⏱️deadline/🔣️.json";
import schema from "../../../../../../../🔨️modules/🖱️ui/🖌️render/⏱️schedule/🧬️schema/🔣️.json";

afterEach(() => vi.useRealTimers());

for (const row of fixture.sources) it(`independent browser clocks preserve ${row.id}`, () => {
  vi.useFakeTimers();
  vi.setSystemTime(0);
  const fired: number[] = [];
  for (const dueUs of [row.uiDueUs, row.shellDueUs]) {
    if (dueUs !== null) setTimeout(() => fired.push(Date.now() * 1000), dueUs / 1000);
  }
  if (row.expectedDueUs === null) {
    expect(vi.getTimerCount()).toBe(0);
  } else {
    vi.advanceTimersToNextTimer();
    expect(fired[0]).toBe(row.expectedDueUs);
  }
});

it("validates the shared replaceable deadline contract", () => {
  const validate = new Ajv2020().compile(schema);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
});

for (const row of fixture.cases) it(`browser timers replay ${row.id}`, () => {
  vi.useFakeTimers();
  vi.setSystemTime(0);
  const timers = new Map<number, ReturnType<typeof setTimeout>>();
  let fired = false;
  for (const step of row.steps) {
    if ("key" in step) {
      clearTimeout(timers.get(step.key!));
      timers.delete(step.key!);
      if (step.dueMs !== null) timers.set(step.key!, setTimeout(() => {
        timers.delete(step.key!);
        fired = true;
      }, step.dueMs! - Date.now()));
    } else {
      vi.advanceTimersByTime(step.nowMs! - Date.now());
      expect(fired, `${row.id}: ${step.nowMs}`).toBe(step.fired);
      fired = false;
      expect(vi.getTimerCount() === 0).toBe(step.nextMs === null);
    }
  }
});

for (const row of fixture.bridges) it(`browser wake translates ${row.id}`, () => {
  vi.useFakeTimers();
  vi.setSystemTime(row.hostSeconds * 1000);
  let fired = false;
  if (row.dueUs === null) {
    expect(row.expectedSeconds).toBeNull();
    expect(vi.getTimerCount()).toBe(0);
    return;
  }
  const delayMs = Math.max(0, row.dueUs - row.nowUs) / 1000;
  setTimeout(() => { fired = true; }, delayMs);
  if (delayMs > 0) {
    vi.advanceTimersByTime(delayMs - 1);
    expect(fired).toBe(false);
    vi.advanceTimersByTime(1);
  } else vi.advanceTimersByTime(0);
  expect(fired).toBe(true);
  expect(Date.now() / 1000).toBeCloseTo(row.expectedSeconds!, 5);
});
