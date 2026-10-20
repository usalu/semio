/** 🟠️ The re addresses queued commands tests of the 📬️ answer outbox suite. */
import { expect, it } from "vitest";
import { type Command } from "@semio-tech/course";
import { ProctorUnavailable, localStore, memoryStorageOrigin, newId, type CommandVerdict } from "@semio-tech/course-react";
import { LEARNER, OTHER_RUN, RUN, answer, gate, opening, outboxOver, tick } from "../🟢️shared/🟦️.tsx";

/** 🟡 Registers the re addresses queued commands tests of the 📬️ answer outbox suite. */
export function registerReAddressesQueuedCommandsTests(): void {
  it("re-addresses the queued commands of a learner under their own ids, in every tab, and never the one on the wire", async () => {
    const origin = memoryStorageOrigin();
    const holder = "d".repeat(32);
    const wire = gate<CommandVerdict>();
    const sent: Command[] = [];
    const left = outboxOver(
      async (command) => {
        sent.push(command);
        return sent.length === 1 ? wire.promise : { kind: "accepted", events: [] };
      },
      undefined,
      origin.tab(),
    );
    const right = outboxOver(async () => Promise.reject(new ProctorUnavailable("offline")), undefined, origin.tab());
    const identify: Command = { type: "identify-learner", id: newId(), learner: LEARNER, identity: { kind: "pseudonym", handle: "Ada" } };
    const start: Command = { type: "start-run", id: newId(), learner: LEARNER, run: RUN, quiz: "physics", challenge: "medium", at: 1_000 };
    const given = answer("t1", ["a", "b"]);
    const stranger: Command = { type: "start-run", id: newId(), learner: "e".repeat(32), run: OTHER_RUN, quiz: "physics", challenge: "medium", at: 1_000 };
    right.start();
    left.start();
    for (const command of [identify, start, given, stranger]) left.enqueue(command);
    await tick();
    expect(right.queued()).toEqual([identify, start, given, stranger]);
    left.reassign(LEARNER, holder);
    expect(left.queued()).toEqual([identify, { ...start, learner: holder, origin: LEARNER }, { ...given, learner: holder }, stranger]);
    await tick();
    expect(right.queued().map((command) => command.learner)).toEqual([LEARNER, holder, holder, stranger.learner]);
    right.stop();
    wire.open({ kind: "rejected", rejection: "handle-claimed" });
    await left.settled(RUN);
    await left.settled(OTHER_RUN);
    expect(sent).toEqual([identify, { ...start, learner: holder, origin: LEARNER }, { ...given, learner: holder }, stranger]);
    left.stop();
  });
  it("restores only commands it can read", () => {
    const area = memoryStorageOrigin().tab();
    const store = localStore(area, "test");
    const start: Command = { type: "start-run", id: newId(), learner: LEARNER, run: RUN, quiz: "physics", challenge: "medium", at: 1_000 };
    store.put("outbox", start.id, { command: start, queuedAt: 2 });
    store.put("outbox", "no-run", { command: { type: "submit-run", id: "no-run", learner: LEARNER }, queuedAt: 1 });
    store.put("outbox", "no-identity", { command: { type: "identify-learner", id: "no-identity", learner: LEARNER }, queuedAt: 1 });
    store.put("outbox", "unknown", { command: { type: "erase-learner", id: "unknown", learner: LEARNER }, queuedAt: 1 });
    store.put("outbox", "other-key", { command: { ...start, id: newId() }, queuedAt: 1 });
    expect(outboxOver(async () => ({ kind: "accepted", events: [] }), undefined, area).queued()).toEqual([start]);
  });
  it("restores a start only with its challenge and the instant the learner acted, an opening or an answer only with that instant", () => {
    const area = memoryStorageOrigin().tab();
    const store = localStore(area, "test");
    const { challenge: _, ...unchallenged } = { type: "start-run", id: "unchallenged", learner: LEARNER, run: RUN, quiz: "physics", challenge: "hard", at: 1_000 } as const;
    const whole = [{ type: "start-run", id: newId(), learner: LEARNER, run: RUN, quiz: "physics", challenge: "expert", at: 1_000 } satisfies Command, opening("t1"), answer("t1", ["a", "b"])];
    whole.forEach((command, index) => store.put("outbox", command.id, { command, queuedAt: 10 + index }));
    const broken: readonly Record<string, unknown>[] = [
      unchallenged,
      { type: "start-run", id: "lenient", learner: LEARNER, run: RUN, quiz: "physics", challenge: "lenient", at: 1_000 },
      { type: "start-run", id: "undated", learner: LEARNER, run: RUN, quiz: "physics", challenge: "hard" },
      { type: "start-run", id: "beyond", learner: LEARNER, run: RUN, quiz: "physics", challenge: "hard", at: 2 ** 53 },
      { type: "open-task", id: "untimed", learner: LEARNER, run: RUN, task: "t1" },
      { type: "open-task", id: "fractional", learner: LEARNER, run: RUN, task: "t1", at: 1.5 },
      { type: "open-task", id: "negative", learner: LEARNER, run: RUN, task: "t1", at: -1 },
      { type: "open-task", id: "untasked", learner: LEARNER, run: RUN, at: 1 },
      { type: "record-answer", id: "unstamped", learner: LEARNER, run: RUN, task: "t1", answer: { kind: "sorting", order: ["a"] } },
      { type: "record-answer", id: "textual", learner: LEARNER, run: RUN, task: "t1", answer: { kind: "sorting", order: ["a"] }, at: "1" },
    ];
    for (const command of broken) store.put("outbox", String(command.id), { command, queuedAt: 1 });
    expect(outboxOver(async () => ({ kind: "accepted", events: [] }), undefined, area).queued()).toEqual(whole);
  });
  it("keeps an opening before the answers of its task, supersedes only those answers and delivers the opening first, also after a reload", async () => {
    const origin = memoryStorageOrigin();
    const before = outboxOver(async () => ({ kind: "accepted", events: [] }), undefined, origin.tab());
    const start: Command = { type: "start-run", id: newId(), learner: LEARNER, run: RUN, quiz: "physics", challenge: "expert", at: 1_000 };
    const open = opening("t1");
    const first = answer("t1", ["a", "b"]);
    const other = opening("t2");
    const second = answer("t1", ["b", "a"]);
    const third = answer("t2", ["a", "b"]);
    for (const command of [start, open, first, other, second, third]) before.enqueue(command);
    expect(before.queued()).toEqual([start, open, other, second, third]);
    expect(before.pendingAnswers(RUN)).toEqual({ t1: second.answer, t2: third.answer });
    const sent: Command[] = [];
    const after = outboxOver(
      async (command) => {
        sent.push(command);
        return { kind: "accepted", events: [] };
      },
      undefined,
      origin.tab(),
    );
    expect(after.queued()).toEqual([start, open, other, second, third]);
    after.start();
    await after.settled(RUN);
    expect(sent).toEqual([start, open, other, second, third]);
    expect(sent.findIndex((command) => command.id === open.id)).toBeLessThan(sent.findIndex((command) => command.type === "record-answer" && command.task === "t1"));
    after.stop();
  });
  it("re-addresses an opening like every command and drops it with the rest of a discarded run", () => {
    const outbox = outboxOver(async () => ({ kind: "accepted", events: [] }));
    const open = opening("t1");
    const elsewhere = opening("t1", OTHER_RUN);
    for (const command of [open, answer("t1", ["a"]), elsewhere]) outbox.enqueue(command);
    outbox.reassign(LEARNER, "d".repeat(32));
    expect(outbox.queued(OTHER_RUN)).toEqual([{ ...elsewhere, learner: "d".repeat(32) }]);
    outbox.discard(RUN);
    expect(outbox.queued().map((command) => command.id)).toEqual([elsewhere.id]);
  });
}
