/** 🔸️ The coalesces one task across tests of the 📬️ answer outbox suite. */
import { expect, it } from "vitest";
import { ServerCallError } from "@semio-tech/framework-server";
import { type Command } from "@semio-tech/course";
import { Outbox, ProctorClient, ProctorUnavailable, localStore, memoryStorageOrigin, newId, type CommandVerdict } from "@semio-tech/course-react";
import { LEARNER, OTHER_RUN, RUN, TIMING, answer, deduplicatingProctor, gate, outboxOver, tick } from "../🟢️harness/🟦️.tsx";

/** 🔹 Registers the coalesces one task across tests of the 📬️ answer outbox suite. */
export function registerCoalescesOneTaskAcrossTests(): void {
  it("coalesces one task across tabs so the newest answer wins and a delivered command is never sent again", async () => {
    const origin = memoryStorageOrigin();
    const area = origin.tab();
    let online = false;
    const applied: Command[] = [];
    const attempts: string[] = [];
    const send = async (command: Command): Promise<CommandVerdict> => {
      if (!online) throw new ProctorUnavailable("offline");
      attempts.push(command.id);
      if (!applied.some((known) => known.id === command.id)) applied.push(command);
      return { kind: "accepted", events: [] };
    };
    const left = outboxOver(send, undefined, origin.tab());
    const right = outboxOver(send, undefined, origin.tab());
    left.start();
    right.start();
    const older = answer("t1", ["a", "b"]);
    left.enqueue(older);
    await tick();
    expect(right.queued()).toEqual([older]);
    const newer = answer("t1", ["b", "a"]);
    right.enqueue(newer);
    await tick();
    expect(left.queued()).toEqual([newer]);
    expect(right.queued()).toEqual([newer]);
    expect(area.keys()).toEqual([`semio.course.test.outbox/${newer.id}`]);
    online = true;
    left.wake();
    right.wake();
    await Promise.all([left.settled(RUN), right.settled(RUN)]);
    expect(applied).toEqual([newer]);
    expect(attempts.every((id) => id === newer.id)).toBe(true);
    expect(area.keys()).toEqual([]);
    left.stop();
    right.stop();
  });
  it("reports definitive verdicts once and drops the answer", async () => {
    const proctor = deduplicatingProctor();
    proctor.rejectWith("quiz-revised");
    const client = new ProctorClient(() => proctor.transport, "test");
    const verdicts: CommandVerdict[] = [];
    const refused: CommandVerdict[] = [];
    const outbox = outboxOver(
      (command, signal) => client.command(command, signal),
      (_, verdict) => verdicts.push(verdict),
    );
    outbox.start();
    outbox.enqueue(answer("t1", ["a", "b"]));
    await outbox.settled(RUN);
    expect(verdicts).toEqual([{ kind: "rejected", rejection: "quiz-revised" }]);
    const broken = outboxOver(
      async () => {
        throw new ServerCallError(400, { kind: "invalid", message: "malformed" });
      },
      (_, verdict) => refused.push(verdict),
    );
    broken.start();
    broken.enqueue(answer("t1", ["a", "b"]));
    await broken.settled(RUN);
    expect(refused).toEqual([{ kind: "refused", detail: "invalid: malformed" }]);
    outbox.stop();
    broken.stop();
  });
  it("drops a closed run's answers, retiring the one in flight so it is neither retried nor restored", async () => {
    const area = memoryStorageOrigin().tab();
    const store = localStore(area, "test");
    const wire = gate<void>();
    const sent: string[] = [];
    const outbox = outboxOver(
      async (command) => {
        sent.push(command.id);
        if (sent.length === 1) {
          await wire.promise;
          throw new ProctorUnavailable("outage");
        }
        return { kind: "accepted", events: [] };
      },
      undefined,
      area,
    );
    const flying = answer("t1", ["a", "b"]);
    const queued = answer("t2", ["a", "b"]);
    const other = answer("t1", ["a", "b"], OTHER_RUN);
    outbox.start();
    outbox.enqueue(flying);
    outbox.enqueue(queued);
    outbox.enqueue(other);
    await tick();
    expect(sent).toEqual([flying.id]);
    outbox.discard(RUN);
    expect(store.record("outbox", flying.id)).toBeUndefined();
    expect(store.record("outbox", queued.id)).toBeUndefined();
    expect(new Outbox({ send: async () => ({ kind: "accepted", events: [] }), store, timing: TIMING }).queued()).toEqual([other]);
    wire.open();
    await outbox.settled(OTHER_RUN);
    await outbox.settled(RUN);
    expect(sent).toEqual([flying.id, other.id]);
    expect(area.keys()).toEqual([]);
    outbox.stop();
  });
  it("settles one run with progress and stops waiting when cancelled", async () => {
    const outbox = outboxOver(async () => ({ kind: "accepted", events: [] }));
    outbox.enqueue(answer("t1", ["a"]));
    outbox.enqueue(answer("t2", ["a"]));
    outbox.enqueue(answer("t1", ["a"], OTHER_RUN));
    const progress: string[] = [];
    outbox.start();
    await outbox.settled(RUN, undefined, (done, total) => progress.push(`${done}/${total}`));
    expect(progress[0]).toBe("0/2");
    expect(progress.at(-1)).toBe("2/2");
    outbox.stop();

    const stuck = outboxOver(() => new Promise<CommandVerdict>(() => undefined));
    stuck.start();
    stuck.enqueue(answer("t1", ["a"]));
    const controller = new AbortController();
    const waiting = stuck.settled(RUN, controller.signal);
    controller.abort(new Error("cancelled by the learner"));
    await expect(waiting).rejects.toThrow("cancelled by the learner");
    expect(stuck.status().pending).toBe(1);
    stuck.stop();
  });
  it("holds every kind of command in the order it was queued, superseding answers only, and delivers a run from its start to its submission", async () => {
    const origin = memoryStorageOrigin();
    const sent: Command[] = [];
    const outbox = outboxOver(
      async (command) => {
        sent.push(command);
        return { kind: "accepted", events: [] };
      },
      undefined,
      origin.tab(),
    );
    const identify: Command = { type: "identify-learner", id: newId(), learner: LEARNER, identity: { kind: "pseudonym", handle: "Ada" } };
    const start: Command = { type: "start-run", id: newId(), learner: LEARNER, run: RUN, quiz: "physics", challenge: "medium", at: 1_000 };
    const again: Command = { type: "start-run", id: newId(), learner: LEARNER, run: OTHER_RUN, quiz: "physics", challenge: "medium", at: 1_000 };
    const first = answer("t1", ["a", "b"]);
    const second = answer("t1", ["b", "a"]);
    const submit: Command = { type: "submit-run", id: newId(), learner: LEARNER, run: RUN };
    for (const command of [identify, start, first, second, submit, again]) outbox.enqueue(command);
    expect(outbox.queued()).toEqual([identify, start, second, submit, again]);
    expect(outbox.queued(RUN)).toEqual([start, second, submit]);
    expect(outbox.pendingAnswers(RUN)).toEqual({ t1: second.answer });
    expect(outbox.waiting((command) => command.type === "submit-run")).toBe(true);
    expect(outbox.waiting((command) => command.type === "submit-run" && command.run === OTHER_RUN)).toBe(false);
    expect(outboxOver(async () => ({ kind: "accepted", events: [] }), undefined, origin.tab()).queued()).toEqual([identify, start, second, submit, again]);
    outbox.start();
    await outbox.settled(RUN);
    await outbox.settled(OTHER_RUN);
    expect(sent).toEqual([identify, start, second, submit, again]);
    outbox.stop();
  });
  it("drops a discarded run from its start to its submission and keeps the registration and the other runs", () => {
    const outbox = outboxOver(async () => ({ kind: "accepted", events: [] }));
    const identify: Command = { type: "identify-learner", id: newId(), learner: LEARNER, identity: { kind: "anonymous" } };
    const other: Command = { type: "start-run", id: newId(), learner: LEARNER, run: OTHER_RUN, quiz: "physics", challenge: "medium", at: 1_000 };
    for (const command of [identify, { type: "start-run", id: newId(), learner: LEARNER, run: RUN, quiz: "physics", challenge: "medium", at: 1_000 }, answer("t1", ["a"]), { type: "submit-run", id: newId(), learner: LEARNER, run: RUN }, other] satisfies Command[]) outbox.enqueue(command);
    outbox.discard(RUN);
    expect(outbox.queued()).toEqual([identify, other]);
    outbox.forget(LEARNER);
    expect(outbox.queued()).toEqual([]);
  });
}
