import { expect, test } from "bun:test";
import { commandStalledMessageV1, commandStalledTextV1, createCommandStallWatchV1, isCommandStalledFaultV1 } from "../🟦️.ts";

const clock = () => {
  const timers: Array<() => void> = [];
  return { timers, now: () => 0, setTimer: (run: () => void) => { timers.push(run); return timers.length - 1; }, clearTimer: () => undefined };
};

test("a held lane names the operation and the stage the turn is waiting in", async () => {
  const fake = clock();
  const watch = createCommandStallWatchV1(fake, 5000);
  const warnings: string[] = [];
  const warn = console.warn;
  console.warn = (message: string) => { warnings.push(message); };
  let stage = "queued";
  let release: () => void = () => undefined;
  const held = watch.watch({ actorId: "stdio-pdf#2", programId: "stdio-pdf", commandId: null, operation: "media-in", stage: () => stage }, () => new Promise<void>((resolve) => { release = resolve; }));
  stage = "continuation 3/64 (guest status more-work, ingress backpressure, 0 acknowledgements pending)";
  fake.timers[0]!();
  console.warn = warn;
  expect(warnings).toHaveLength(1);
  expect(warnings[0]).toContain("held stdio-pdf#2's lane for 5000 ms");
  expect(warnings[0]).toContain("operation: media-in");
  expect(warnings[0]).toContain("stage: continuation 3/64");
  release();
  await held;
});

test("a declared command is named by its own id and an unreported stage says so", async () => {
  const fake = clock();
  const watch = createCommandStallWatchV1(fake, 5000);
  const warnings: string[] = [];
  const warn = console.warn;
  console.warn = (message: string) => { warnings.push(message); };
  let release: () => void = () => undefined;
  const held = watch.watch({ actorId: "a#1", programId: "p", commandId: "insert-text" }, () => new Promise<void>((resolve) => { release = resolve; }));
  fake.timers[0]!();
  console.warn = warn;
  expect(warnings[0]).toContain("operation: insert-text, stage: unreported");
  release();
  await held;
});

test("the stalled-command fault is named and reads in both languages", () => {
  const message = commandStalledMessageV1("stdio-pdf", "media-in");
  expect(isCommandStalledFaultV1(message)).toBe(true);
  expect(message).toContain("plugin.command-stalled");
  expect(commandStalledTextV1("stdio-pdf", "media-in", "en-US")).toContain("made no progress");
  expect(commandStalledTextV1("stdio-pdf", "media-in", "de-DE")).toContain("keinen Fortschritt");
  expect(isCommandStalledFaultV1("plugin.command-page-invalid")).toBe(false);
});
