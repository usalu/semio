/** 📬️ Answer outbox and proctor wire: coalescing per (run, task), retries through transient failures under one command
 * id, exactly-once application by a proctor that deduplicates idempotency keys, persistence across a reload (a start
 * only with its challenge, an opening and an answer only with their instant), an opening kept before the answers of its
 * task, definitive verdicts, per-run settling with progress and cancellation — and the §9a envelope mapping with the
 * rejections of the challenges.
 */
import { describe, expect, it } from "vitest";
import { ServerCallError, decodeQueryEnvelope, type HttpTransport } from "@semio-tech/framework-server";
import { WIRE_VERSION, handleActorId, type Command } from "@semio-tech/course";
import { ProctorClient, ProctorUnavailable, abortable, commandEnvelope, isTransient, newId, queryEnvelope, courseInstance, courseRejection, retryTransient } from "@semio-tech/course-react";
import { TIMING, LEARNER, RUN, opening, reply } from "./🟢️harness/🟦️.tsx";
import { registerCoalescesQueuedAnswersPerTests } from "./🔶️coalesces-queued-answers-per/🟦️.tsx";
import { registerCoalescesOneTaskAcrossTests } from "./🔸️coalesces-one-task-across/🟦️.tsx";
import { registerReAddressesQueuedCommandsTests } from "./🟠️re-addresses-queued-commands/🟦️.tsx";

describe("📬️ answer outbox", () => {
  registerCoalescesQueuedAnswersPerTests();
  registerCoalescesOneTaskAcrossTests();
  registerReAddressesQueuedCommandsTests();
});

describe("🛂️ proctor wire (design §9a)", () => {
  it("maps a registration under a pseudonym onto the actor of its handle key, an anonymous one onto its own learner, with an anonymous principal", () => {
    const command: Command = { type: "identify-learner", id: newId(), learner: newId(), identity: { kind: "pseudonym", handle: "  Ada   L. " } };
    const envelope = commandEnvelope(command, "architecture", 1234);
    expect(envelope).toMatchObject({
      commandId: command.id,
      idempotencyKey: command.id,
      kind: "course.identify-learner",
      version: WIRE_VERSION,
      scope: "architecture",
      target: { tenant: "architecture", kind: "course-handle", id: handleActorId("ada l.") },
      principal: { kind: "anonymous" },
      clientHlc: { millis: 1234, counter: 0 },
    });
    expect(envelope.target.id).toBe([..."ada l."].map((character) => character.charCodeAt(0).toString(16)).join(""));
    expect(JSON.parse(new TextDecoder().decode(envelope.payload))).toEqual(command);
    const anonymous: Command = { type: "identify-learner", id: newId(), learner: newId(), identity: { kind: "anonymous" } };
    expect(commandEnvelope(anonymous, "architecture", 1)).toMatchObject({ target: { tenant: "architecture", kind: "course-learner", id: anonymous.learner }, principal: { kind: "anonymous" } });
  });

  it("names the period, the one quiz and the asking learner in the leaderboard query and asks for a handle as typed", async () => {
    const queries: unknown[] = [];
    const transport: HttpTransport = {
      send: async (request) => {
        if (request.method === "GET" && request.path === "/instance") return reply(200, courseInstance());
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
    expect(envelope.target).toEqual({ tenant: "architecture", kind: "course-learner", id: LEARNER });
    expect(envelope.principal).toEqual({ kind: "user", id: LEARNER });
    const open = opening("t1");
    expect(commandEnvelope(open, "architecture", 1)).toMatchObject({ kind: "course.open-task", target: { tenant: "architecture", kind: "course-learner", id: LEARNER }, principal: { kind: "user", id: LEARNER }, idempotencyKey: open.id });
    expect(JSON.parse(new TextDecoder().decode(commandEnvelope(open, "architecture", 1).payload))).toEqual(open);
    expect(JSON.parse(new TextDecoder().decode(commandEnvelope(command, "architecture", 1).payload))).toEqual(command);
    const query = queryEnvelope({ type: "run", run: RUN }, "architecture", LEARNER);
    expect(query).toMatchObject({ kind: "course.run", version: WIRE_VERSION, scope: "architecture", principal: { kind: "user", id: LEARNER }, consistency: { kind: "authority" }, cursor: null });
    expect(JSON.parse(new TextDecoder().decode(query.arguments))).toEqual({ type: "run", run: RUN });
  });

  it("reads the quiz rejection from notices or detail and treats a busy actor as transient", () => {
    expect(courseRejection({ kind: "invalid", detail: "rejected: run-open" }, [])).toBe("run-open");
    expect(courseRejection({ kind: "unauthorized", detail: "nope" }, [{ code: "handle-invalid", message: "" }])).toBe("handle-invalid");
    expect(courseRejection({ kind: "invalid", detail: "garbled-input" }, [])).toBeUndefined();
    expect(courseRejection({ kind: "invalid", detail: "rejected: run-untimed" }, [])).toBe("run-untimed");
    expect(courseRejection({ kind: "invalid", detail: "nope" }, [{ code: "task-unopened", message: "" }])).toBe("task-unopened");
    expect(courseRejection({ kind: "invalid", detail: "quiz rejected the command: time-up" }, [])).toBe("time-up");
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
