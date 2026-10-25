/** 🔶️ The coalesces queued answers per tests of the 📬️ answer outbox suite. */
import { expect, it } from "vitest";
import { ProctorClient, ProctorUnavailable, memoryStorageOrigin, type CommandVerdict, type OutboxActivity } from "@semio-tech/course-react";
import { OTHER_RUN, RUN, answer, deduplicatingProctor, gate, outboxOver, tick } from "../🟢️harness/🟦️.tsx";

/** 🔷 Registers the coalesces queued answers per tests of the 📬️ answer outbox suite. */
export function registerCoalescesQueuedAnswersPerTests(): void {
  it("coalesces queued answers per run and task and never replaces the one on the wire", async () => {
    const sent: string[] = [];
    const wire = gate<CommandVerdict>();
    const outbox = outboxOver(async (command) => {
      sent.push(command.id);
      return sent.length === 1 ? wire.promise : { kind: "accepted", events: [] };
    });
    const first = answer("t1", ["x", "y"]);
    const second = answer("t2", ["x", "y"]);
    const third = answer("t1", ["y", "x"]);
    const fourth = answer("t2", ["y", "x"]);
    outbox.start();
    outbox.enqueue(first);
    await Promise.resolve();
    outbox.enqueue(second);
    outbox.enqueue(third);
    outbox.enqueue(fourth);
    expect(outbox.queued().map((command) => command.id)).toEqual([first.id, third.id, fourth.id]);
    wire.open({ kind: "accepted", events: [] });
    await outbox.settled(RUN);
    expect(sent).toEqual([first.id, third.id, fourth.id]);
    expect(outbox.status().pending).toBe(0);
    outbox.stop();
  });
  it("keeps only the latest answer of a task while nothing is sent", () => {
    const outbox = outboxOver(async () => ({ kind: "accepted", events: [] }));
    outbox.enqueue(answer("t1", ["a", "b"]));
    const latest = answer("t1", ["b", "a"]);
    outbox.enqueue(latest);
    outbox.enqueue(answer("t1", ["a", "b"], OTHER_RUN));
    expect(outbox.queued(RUN)).toEqual([latest]);
    expect(outbox.pendingAnswers(RUN)).toEqual({ t1: latest.answer });
    expect(outbox.status().pending).toBe(2);
  });
  it("retries transient failures with jittered backoff under the same command id and shows it", async () => {
    const proctor = deduplicatingProctor();
    const client = new ProctorClient(() => proctor.transport, "test");
    const outbox = outboxOver((command, signal) => client.command(command, signal));
    const activities: OutboxActivity[] = [];
    outbox.subscribe(() => activities.push(outbox.status().activity));
    const reachability: string[] = [];
    client.subscribe(() => reachability.push(client.reachability()));
    proctor.failNext(2);
    const command = answer("t1", ["a", "b"]);
    outbox.start();
    outbox.enqueue(command);
    await outbox.settled(RUN);
    expect(proctor.envelopes.map((envelope) => envelope.commandId)).toEqual([command.id, command.id, command.id]);
    expect(proctor.envelopes.every((envelope) => envelope.idempotencyKey === command.id)).toBe(true);
    expect(proctor.applied.size).toBe(1);
    expect(activities).toContain("retrying");
    expect(activities.at(-1)).toBe("idle");
    expect(reachability).toEqual(["unreachable", "reachable"]);
    outbox.stop();
  });
  it("applies a command exactly once when its response is lost and the retry replays it", async () => {
    const proctor = deduplicatingProctor();
    const client = new ProctorClient(() => proctor.transport, "test");
    const verdicts: CommandVerdict[] = [];
    const outbox = outboxOver(
      (command, signal) => client.command(command, signal),
      (_, verdict) => verdicts.push(verdict),
    );
    proctor.loseNextResponse();
    outbox.start();
    outbox.enqueue(answer("t1", ["a", "b"]));
    await outbox.settled(RUN);
    expect(proctor.envelopes).toHaveLength(2);
    expect(new Set(proctor.envelopes.map((envelope) => envelope.idempotencyKey)).size).toBe(1);
    expect(proctor.applied.size).toBe(1);
    expect(verdicts).toHaveLength(1);
    expect(verdicts[0]?.kind).toBe("accepted");
    outbox.stop();
  });
  it("persists the queue locally and delivers it after a reload", async () => {
    const origin = memoryStorageOrigin();
    const before = outboxOver(async () => ({ kind: "accepted", events: [] }), undefined, origin.tab());
    const one = answer("t1", ["a", "b"]);
    const two = answer("t2", ["a", "b"]);
    before.enqueue(one);
    before.enqueue(two);
    const sent: string[] = [];
    const after = outboxOver(
      async (command) => {
        sent.push(command.id);
        return { kind: "accepted", events: [] };
      },
      undefined,
      origin.tab(),
    );
    expect(after.queued()).toEqual([one, two]);
    after.start();
    await after.settled(RUN);
    expect(sent).toEqual([one.id, two.id]);
    expect(outboxOver(async () => ({ kind: "accepted", events: [] }), undefined, origin.tab()).queued()).toEqual([]);
    after.stop();
  });
  it("keeps every tab's queued answers, each in its own record, so tabs never overwrite each other", async () => {
    const origin = memoryStorageOrigin();
    const area = origin.tab();
    const offline = async (): Promise<CommandVerdict> => {
      throw new ProctorUnavailable("offline");
    };
    const left = outboxOver(offline, undefined, origin.tab());
    const right = outboxOver(offline, undefined, origin.tab());
    left.start();
    right.start();
    const mine = answer("t1", ["a", "b"]);
    const theirs = answer("t2", ["b", "a"]);
    left.enqueue(mine);
    right.enqueue(theirs);
    await tick();
    const ids = new Set([mine.id, theirs.id]);
    expect(new Set(left.queued().map((command) => command.id))).toEqual(ids);
    expect(new Set(right.queued().map((command) => command.id))).toEqual(ids);
    expect(new Set(area.keys())).toEqual(new Set([`semio.course.test.outbox/${mine.id}`, `semio.course.test.outbox/${theirs.id}`]));
    left.stop();
    right.stop();
    const sent: string[] = [];
    const reloaded = outboxOver(
      async (command) => {
        sent.push(command.id);
        return { kind: "accepted", events: [] };
      },
      undefined,
      origin.tab(),
    );
    expect(new Set(reloaded.queued().map((command) => command.id))).toEqual(ids);
    reloaded.start();
    await reloaded.settled(RUN);
    expect(new Set(sent)).toEqual(ids);
    expect(area.keys()).toEqual([]);
    reloaded.stop();
  });
}
