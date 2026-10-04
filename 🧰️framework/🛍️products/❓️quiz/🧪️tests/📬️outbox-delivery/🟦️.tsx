/** 📬️ Answer outbox and proctor wire: coalescing per (run, task), retries through transient failures under one command
 * id, exactly-once application by a proctor that deduplicates idempotency keys, persistence across a reload (a start
 * only with its challenge, an opening and an answer only with their instant), an opening kept before the answers of its
 * task, definitive verdicts, per-run settling with progress and cancellation — and the §9a envelope mapping with the
 * rejections of the challenges.
 */

import { describe, expect, it } from "vitest";
import { ServerCallError, decodeCommandEnvelope, decodeQueryEnvelope, encodeCommandOutcome, type CommandEnvelope, type CommandOutcome, type HttpResponse, type HttpTransport } from "@semio-tech/framework-server";
import { WIRE_VERSION, handleActorId, type Command, type OpenTaskCommand, type RecordAnswerCommand } from "@semio-tech/quiz";
import {
  Outbox,
  ProctorClient,
  ProctorUnavailable,
  abortable,
  commandEnvelope,
  isTransient,
  localStore,
  memoryStorageOrigin,
  newId,
  queryEnvelope,
  quizInstance,
  quizRejection,
  retryTransient,
  type CommandVerdict,
  type OutboxActivity,
  type StorageArea,
} from "@semio-tech/quiz-react";

const TIMING = { minMs: 1, maxMs: 4 };
const LEARNER = "a".repeat(32);
const RUN = "b".repeat(32);
const OTHER_RUN = "c".repeat(32);
const encoder = new TextEncoder();

function answer(task: string, order: readonly string[], run = RUN): RecordAnswerCommand {
  return { type: "record-answer", id: newId(), learner: LEARNER, run, task, answer: { kind: "sorting", order }, at: 1 };
}

function opening(task: string, run = RUN): OpenTaskCommand {
  return { type: "open-task", id: newId(), learner: LEARNER, run, task, at: 1 };
}

function gate<T>(): { readonly promise: Promise<T>; readonly open: (value: T) => void } {
  let open: (value: T) => void = () => undefined;
  const promise = new Promise<T>((resolve) => {
    open = resolve;
  });
  return { promise, open };
}

function tick(): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, 10));
}

function reply(status: number, body: unknown): HttpResponse {
  const text = JSON.stringify(body);
  return { status, text: async () => text, bytes: async () => encoder.encode(text) };
}

/** 🛂️ A proctor double that applies every command id at most once, like the framework's idempotency law. */
function deduplicatingProctor(): { readonly transport: HttpTransport; readonly envelopes: CommandEnvelope[]; readonly applied: Map<string, CommandOutcome>; failNext(count: number): void; loseNextResponse(): void; rejectWith(detail: string): void } {
  const envelopes: CommandEnvelope[] = [];
  const applied = new Map<string, CommandOutcome>();
  let failures = 0;
  let lose = false;
  let rejection: string | undefined;
  const transport: HttpTransport = {
    async send(request) {
      if (request.method === "GET" && request.path === "/instance") return reply(200, quizInstance());
      const envelope = decodeCommandEnvelope(JSON.parse(String(request.body)));
      envelopes.push(envelope);
      if (failures > 0) {
        failures -= 1;
        return reply(503, { kind: "actorUnavailable", message: "busy" });
      }
      const command = JSON.parse(new TextDecoder().decode(envelope.payload)) as Command;
      const receipt = { commandId: envelope.commandId, actor: envelope.target, revision: applied.size + 1, acceptedAt: envelope.clientHlc };
      const outcome: CommandOutcome =
        applied.get(envelope.idempotencyKey ?? envelope.commandId) ??
        (rejection === undefined
          ? {
              status: "accepted",
              receipt,
              events: [{ stream: envelope.target, seq: applied.size + 1, hlc: envelope.clientHlc, kind: `quiz.answer-recorded`, payload: encoder.encode(JSON.stringify({ ...command, type: "answer-recorded", at: 1 })) }],
              frontier: null,
            }
          : { status: "rejected", receipt, reason: { kind: "invalid", detail: rejection }, notices: [] });
      applied.set(envelope.idempotencyKey ?? envelope.commandId, outcome);
      if (lose) {
        lose = false;
        throw new ProctorUnavailable("connection reset after the proctor applied the command");
      }
      return reply(200, encodeCommandOutcome(outcome));
    },
  };
  return {
    transport,
    envelopes,
    applied,
    failNext: (count) => {
      failures = count;
    },
    loseNextResponse: () => {
      lose = true;
    },
    rejectWith: (detail) => {
      rejection = detail;
    },
  };
}

function outboxOver(send: (command: Command, signal: AbortSignal) => Promise<CommandVerdict>, onSettled?: (command: Command, verdict: CommandVerdict) => void, area: StorageArea = memoryStorageOrigin().tab()): Outbox {
  return new Outbox({ send, store: localStore(area, "test"), timing: TIMING, onSettled });
}

describe("📬️ answer outbox", () => {
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
    expect(new Set(area.keys())).toEqual(new Set([`semio.quiz.test.outbox/${mine.id}`, `semio.quiz.test.outbox/${theirs.id}`]));
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
    expect(area.keys()).toEqual([`semio.quiz.test.outbox/${newer.id}`]);
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
    expect(left.queued()).toEqual([identify, { ...start, learner: holder }, { ...given, learner: holder }, stranger]);
    await tick();
    expect(right.queued().map((command) => command.learner)).toEqual([LEARNER, holder, holder, stranger.learner]);
    right.stop();
    wire.open({ kind: "rejected", rejection: "handle-claimed" });
    await left.settled(RUN);
    await left.settled(OTHER_RUN);
    expect(sent).toEqual([identify, { ...start, learner: holder }, { ...given, learner: holder }, stranger]);
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
});

describe("🛂️ proctor wire (design §9a)", () => {
  it("maps a registration under a pseudonym onto the actor of its handle key, an anonymous one onto its own learner, with an anonymous principal", () => {
    const command: Command = { type: "identify-learner", id: newId(), learner: newId(), identity: { kind: "pseudonym", handle: "  Ada   L. " } };
    const envelope = commandEnvelope(command, "architecture", 1234);
    expect(envelope).toMatchObject({
      commandId: command.id,
      idempotencyKey: command.id,
      kind: "quiz.identify-learner",
      version: WIRE_VERSION,
      scope: "architecture",
      target: { tenant: "architecture", kind: "quiz-handle", id: handleActorId("ada l.") },
      principal: { kind: "anonymous" },
      clientHlc: { millis: 1234, counter: 0 },
    });
    expect(envelope.target.id).toBe([..."ada l."].map((character) => character.charCodeAt(0).toString(16)).join(""));
    expect(JSON.parse(new TextDecoder().decode(envelope.payload))).toEqual(command);
    const anonymous: Command = { type: "identify-learner", id: newId(), learner: newId(), identity: { kind: "anonymous" } };
    expect(commandEnvelope(anonymous, "architecture", 1)).toMatchObject({ target: { tenant: "architecture", kind: "quiz-learner", id: anonymous.learner }, principal: { kind: "anonymous" } });
  });

  it("names the period, the one quiz and the asking learner in the leaderboard query and asks for a handle as typed", async () => {
    const queries: unknown[] = [];
    const transport: HttpTransport = {
      send: async (request) => {
        if (request.method === "GET" && request.path === "/instance") return reply(200, quizInstance());
        queries.push(JSON.parse(new TextDecoder().decode(decodeQueryEnvelope(JSON.parse(String(request.body))).arguments)));
        return reply(404, { kind: "notFound", message: "none" });
      },
    };
    const client = new ProctorClient(() => transport, "architecture");
    await client.leaderboard({ period: "all-time" }, LEARNER).catch(() => undefined);
    await client.leaderboard({ period: "daily" }, undefined).catch(() => undefined);
    await client.leaderboard({ period: "weekly", quiz: "heating" }, LEARNER).catch(() => undefined);
    await client.leaderboard({ period: "monthly", quiz: "cooling" }, undefined).catch(() => undefined);
    await client.handle("  Ada ").catch(() => undefined);
    expect(queries).toEqual([
      { type: "leaderboard", period: "all-time", learner: LEARNER },
      { type: "leaderboard", period: "daily" },
      { type: "leaderboard", period: "weekly", quiz: "heating", learner: LEARNER },
      { type: "leaderboard", period: "monthly", quiz: "cooling" },
      { type: "handle", handle: "  Ada " },
    ]);
  });

  it("maps learner commands onto the learner actor with the learner as principal", () => {
    const command: Command = { type: "start-run", id: newId(), learner: LEARNER, run: RUN, quiz: "physics", challenge: "medium", at: 1_000 };
    const envelope = commandEnvelope(command, "architecture", 1);
    expect(envelope.target).toEqual({ tenant: "architecture", kind: "quiz-learner", id: LEARNER });
    expect(envelope.principal).toEqual({ kind: "user", id: LEARNER });
    const open = opening("t1");
    expect(commandEnvelope(open, "architecture", 1)).toMatchObject({ kind: "quiz.open-task", target: { tenant: "architecture", kind: "quiz-learner", id: LEARNER }, principal: { kind: "user", id: LEARNER }, idempotencyKey: open.id });
    expect(JSON.parse(new TextDecoder().decode(commandEnvelope(open, "architecture", 1).payload))).toEqual(open);
    expect(JSON.parse(new TextDecoder().decode(commandEnvelope(command, "architecture", 1).payload))).toEqual(command);
    const query = queryEnvelope({ type: "run", run: RUN }, "architecture", LEARNER);
    expect(query).toMatchObject({ kind: "quiz.run", version: WIRE_VERSION, scope: "architecture", principal: { kind: "user", id: LEARNER }, consistency: { kind: "authority" }, cursor: null });
    expect(JSON.parse(new TextDecoder().decode(query.arguments))).toEqual({ type: "run", run: RUN });
  });

  it("reads the quiz rejection from notices or detail and treats a busy actor as transient", () => {
    expect(quizRejection({ kind: "invalid", detail: "rejected: run-open" }, [])).toBe("run-open");
    expect(quizRejection({ kind: "unauthorized", detail: "nope" }, [{ code: "handle-invalid", message: "" }])).toBe("handle-invalid");
    expect(quizRejection({ kind: "invalid", detail: "garbled-input" }, [])).toBeUndefined();
    expect(quizRejection({ kind: "invalid", detail: "rejected: run-untimed" }, [])).toBe("run-untimed");
    expect(quizRejection({ kind: "invalid", detail: "nope" }, [{ code: "task-unopened", message: "" }])).toBe("task-unopened");
    expect(quizRejection({ kind: "invalid", detail: "quiz rejected the command: time-up" }, [])).toBe("time-up");
    expect(isTransient(new ProctorUnavailable("x"))).toBe(true);
    expect(isTransient(new ServerCallError(503, { kind: "actorUnavailable", message: "" }))).toBe(true);
    expect(isTransient(new ServerCallError(404, { kind: "notFound", message: "" }))).toBe(false);
  });

  it("mints 128-bit lowercase hex ids and stops retrying on abort", async () => {
    const ids = new Set(Array.from({ length: 64 }, newId));
    expect(ids.size).toBe(64);
    for (const id of ids) expect(id).toMatch(/^[0-9a-f]{32}$/u);
    const controller = new AbortController();
    const retried = retryTransient(async () => Promise.reject(new ProctorUnavailable("down")), TIMING, controller.signal);
    controller.abort(new Error("stop"));
    await expect(retried).rejects.toThrow("stop");
  });

  it("never leaves the losing call unhandled when a query starts on an already stopped session", async () => {
    const unhandled: unknown[] = [];
    const record = (reason: unknown): void => {
      unhandled.push(reason);
    };
    process.on("unhandledRejection", record);
    try {
      const stopped = new AbortController();
      stopped.abort();
      const client = new ProctorClient((signal) => ({ send: async () => Promise.reject(signal?.reason ?? new Error("sent")) }), "test");
      await expect(client.leaderboard({ period: "all-time" }, undefined, stopped.signal)).rejects.toBe(stopped.signal.reason);
      await expect(abortable(Promise.reject(new Error("late")), stopped.signal)).rejects.toBe(stopped.signal.reason);
      await new Promise((resolve) => setTimeout(resolve, 20));
      expect(unhandled).toEqual([]);
    } finally {
      process.off("unhandledRejection", record);
    }
  });
});
