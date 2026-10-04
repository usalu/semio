/** 🚦️ A busy proctor (`429` with `Retry-After`) is a short shortage, never a freeze and never a reason to hammer: the
 * header is read as RFC 9110 defines it (shared vectors), every retry path waits what it asks plus jittered backoff —
 * interactive commands, the answer outbox even when woken, the leaderboard polling — a cancelled wait ends at once, the
 * indicator says "busy" without an alert, and a presence socket that keeps being dropped is rejoined ever more slowly.
 * A `429` that names the sign-up allowance as spent is no shortage but an answer: it carries the allowance and ends the
 * call instead of being sent again.
 *
 * @see ../../🧫️fixtures/🚦️rate-limits/🔣️.json
 * @see https://www.rfc-editor.org/rfc/rfc9110#section-10.2.3 — `Retry-After`
 * @see https://www.rfc-editor.org/rfc/rfc6585#section-4 — `429 Too Many Requests`
 */

import { act, render } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { ServerCallError, encodeQueryResult } from "@semio-tech/framework-server";
import type { CursorState, RecordAnswerCommand } from "@semio-tech/quiz";
import {
  Outbox,
  POLL_BACKOFF_MAX_MS,
  PRESENCE_STABLE_MS,
  PresenceRoom,
  ProctorClient,
  ProctorThrottled,
  ProctorUnavailable,
  RETRY_AFTER_MAX_MS,
  SIGN_UP_ALLOWANCE,
  connectionMessage,
  isThrottled,
  isTransient,
  localStore,
  memoryStorageOrigin,
  newId,
  pollDelay,
  proctorTransport,
  quizInstance,
  quizText,
  rejoinDelay,
  retryAfterMs,
  retryTransient,
  signUpsSpent,
  usePolling,
  waitingMessage,
  type CommandVerdict,
  type PresenceSocket,
  type QuizConnection,
  type RefreshOutcome,
} from "@semio-tech/quiz-react";
import limits from "../../🧫️fixtures/🚦️rate-limits/🔣️.json";

interface Fixture {
  readonly retryAfter: { readonly capMs: number; readonly now: string; readonly headers: readonly { readonly header: string | null; readonly ms: number | null }[] };
  readonly allowances: { readonly signUp: string; readonly answers: readonly { readonly id: string; readonly status: number; readonly header: string | null; readonly body: unknown; readonly retryAfterMs: number | null; readonly allowance: string | null; readonly ends: boolean }[] };
  readonly polling: { readonly intervalMs: number; readonly maxMs: number; readonly delays: readonly { readonly failures: number; readonly retryAfterMs: number | null; readonly random: number; readonly ms: number }[] };
  readonly rejoin: { readonly timing: { readonly minMs: number; readonly maxMs: number }; readonly stableMs: number; readonly delays: readonly { readonly flaps: number; readonly random: number; readonly ms: number }[] };
}

const fixture: Fixture = limits;
const TIMING = { minMs: 100, maxMs: 400 };
const LEARNER = "a".repeat(32);
const RUN = "b".repeat(32);

function answer(task: string): RecordAnswerCommand {
  return { type: "record-answer", id: newId(), learner: LEARNER, run: RUN, task, answer: { kind: "sorting", order: ["a", "b"] }, at: 1 };
}

function Poller(props: { readonly refresh: () => Promise<RefreshOutcome>; readonly intervalMs: number }): null {
  usePolling(props.refresh, props.intervalMs);
  return null;
}

afterEach(() => {
  vi.useRealTimers();
  vi.restoreAllMocks();
  vi.unstubAllGlobals();
});

describe("🚦️ Retry-After", () => {
  const now = Date.parse(fixture.retryAfter.now);

  it("caps what a proctor may ask for", () => {
    expect(RETRY_AFTER_MAX_MS).toBe(fixture.retryAfter.capMs);
  });

  for (const vector of fixture.retryAfter.headers) {
    it(`reads ${JSON.stringify(vector.header)} as ${vector.ms === null ? "no hint" : `${vector.ms} ms`}`, () => {
      expect(retryAfterMs(vector.header, now) ?? null).toBe(vector.ms);
    });
  }

  it("turns a 429 of the proctor into a transient, throttled shortage that carries the wait", async () => {
    const answers = [new Response("slow down", { status: 429, headers: { "Retry-After": "7" } }), new Response("slow down", { status: 429 })];
    vi.stubGlobal(
      "fetch",
      vi.fn(async () => answers.shift()!),
    );
    const transport = proctorTransport("https://proctor.test")();
    const request = { method: "POST", path: "/queries", query: {}, headers: {}, body: "{}" } as const;
    const first = await transport.send(request).catch((error: unknown) => error);
    expect(first).toBeInstanceOf(ProctorThrottled);
    expect(first).toBeInstanceOf(ProctorUnavailable);
    expect((first as ProctorThrottled).retryAfterMs).toBe(7000);
    const second = await transport.send(request).catch((error: unknown) => error);
    expect((second as ProctorThrottled).retryAfterMs).toBeUndefined();
    answers.push(
      new Response(JSON.stringify({ kind: "throttled", message: "too many requests", retryAfterMs: 2500 }), { status: 429 }),
      new Response(JSON.stringify({ kind: "overloaded", message: "too many requests in flight" }), { status: 503, headers: { "Retry-After": "4" } }),
      new Response(JSON.stringify({ kind: "actorUnavailable", message: "busy" }), { status: 503 }),
    );
    expect(((await transport.send(request).catch((error: unknown) => error)) as ProctorThrottled).retryAfterMs).toBe(2500);
    const overloaded = await transport.send(request).catch((error: unknown) => error);
    expect(overloaded).toBeInstanceOf(ProctorThrottled);
    expect((overloaded as ProctorThrottled).retryAfterMs).toBe(4000);
    expect((await transport.send(request)).status).toBe(503);
    for (const error of [first, second, new ServerCallError(429, { kind: "http", message: "HTTP 429" })]) {
      expect(isTransient(error)).toBe(true);
      expect(isThrottled(error)).toBe(true);
    }
    expect(isThrottled(new ProctorUnavailable("gone"))).toBe(false);
    expect(isThrottled(new ServerCallError(503, { kind: "http", message: "HTTP 503" }))).toBe(false);
  });

  it("names the sign-up allowance by the word the proctor uses", () => {
    expect(SIGN_UP_ALLOWANCE).toBe(fixture.allowances.signUp);
  });

  for (const vector of fixture.allowances.answers) {
    it(`reads from a ${vector.status} answer the allowance that is spent: ${vector.id}`, async () => {
      vi.stubGlobal(
        "fetch",
        vi.fn(async () => new Response(JSON.stringify(vector.body), { status: vector.status, headers: vector.header === null ? {} : { "Retry-After": vector.header } })),
      );
      const error = await proctorTransport("https://proctor.test")()
        .send({ method: "POST", path: "/commands", query: {}, headers: {}, body: "{}" })
        .catch((thrown: unknown) => thrown);
      expect(error).toBeInstanceOf(ProctorThrottled);
      expect([(error as ProctorThrottled).retryAfterMs ?? null, (error as ProctorThrottled).allowance ?? null]).toEqual([vector.retryAfterMs, vector.allowance]);
      expect([signUpsSpent(error), isTransient(error), isThrottled(error)]).toEqual([vector.ends, !vector.ends, !vector.ends]);
      let tries = 0;
      const outcome = await retryTransient(
        async () => {
          tries += 1;
          if (tries === 1) throw error;
          return "answered";
        },
        { minMs: 0, maxMs: 0 },
        AbortSignal.timeout(50),
      ).catch((thrown: unknown) => thrown);
      if (vector.ends) expect([outcome, tries]).toEqual([error, 1]);
      else expect(outcome === "answered" || (outcome as Error).name === "TimeoutError").toBe(true);
    });
  }
});

describe("🚦️ retrying a busy proctor", () => {
  it("waits what the proctor asked plus jittered backoff before the next attempt, and ends at once when cancelled", async () => {
    vi.useFakeTimers();
    vi.spyOn(Math, "random").mockReturnValue(0.5);
    const attempts: number[] = [];
    const call = async (): Promise<string> => {
      attempts.push(Date.now());
      if (attempts.length < 3) throw new ProctorThrottled(3000);
      return "answered";
    };
    const started = Date.now();
    const answered = retryTransient(call, TIMING);
    await vi.advanceTimersByTimeAsync(3000 + TIMING.minMs - 1);
    expect(attempts).toHaveLength(1);
    await vi.advanceTimersByTimeAsync(20_000);
    await expect(answered).resolves.toBe("answered");
    expect(attempts).toHaveLength(3);
    expect(attempts[1]! - started).toBeGreaterThanOrEqual(3000 + TIMING.minMs);
    expect(attempts[2]! - attempts[1]!).toBeGreaterThanOrEqual(3000 + TIMING.minMs);
    expect(attempts[2]! - attempts[1]!).toBeLessThanOrEqual(3000 + TIMING.maxMs);

    const controller = new AbortController();
    let tries = 0;
    const cancelled = retryTransient(
      async () => {
        tries += 1;
        throw new ProctorThrottled(60_000);
      },
      TIMING,
      controller.signal,
    );
    const outcome = cancelled.catch((error: unknown) => error);
    await vi.advanceTimersByTimeAsync(1000);
    controller.abort(new Error("cancelled by the learner"));
    await vi.advanceTimersByTimeAsync(0);
    expect(((await outcome) as Error).message).toBe("cancelled by the learner");
    expect(tries).toBe(1);
  });

  it("keeps a throttled answer in the outbox, does not resend it before the proctor's wait is over even when woken, then delivers it", async () => {
    vi.useFakeTimers();
    vi.spyOn(Math, "random").mockReturnValue(0);
    const sends: number[] = [];
    const settled: CommandVerdict[] = [];
    const outbox = new Outbox({
      send: async () => {
        sends.push(Date.now());
        if (sends.length === 1) throw new ProctorThrottled(5000);
        return { kind: "accepted", events: [] };
      },
      store: localStore(memoryStorageOrigin().tab(), "limits"),
      timing: TIMING,
      onSettled: (_, verdict) => settled.push(verdict),
    });
    outbox.start();
    const started = Date.now();
    outbox.enqueue(answer("masses"));
    await vi.advanceTimersByTimeAsync(10);
    expect(sends).toHaveLength(1);
    expect(outbox.status()).toMatchObject({ pending: 1, activity: "retrying" });
    for (const at of [1000, 2000, 4000]) {
      await vi.advanceTimersByTimeAsync(at - (Date.now() - started));
      outbox.wake();
      await vi.advanceTimersByTimeAsync(1);
      expect(sends, `woken after ${at} ms`).toHaveLength(1);
    }
    await vi.advanceTimersByTimeAsync(5000 + TIMING.maxMs);
    expect(sends).toHaveLength(2);
    expect(sends[1]! - started).toBeGreaterThanOrEqual(5000);
    expect(settled).toEqual([{ kind: "accepted", events: [] }]);
    expect(outbox.status()).toMatchObject({ pending: 0, activity: "idle" });
    outbox.stop();
  });

  it("shows a busy proctor as busy — reachable, no alert — and as unreachable only when it does not answer at all", async () => {
    const encoder = new TextEncoder();
    const replies: (() => never | { readonly status: number; readonly text: () => Promise<string>; readonly bytes: () => Promise<Uint8Array> })[] = [];
    const agreeing = JSON.stringify(quizInstance());
    const client = new ProctorClient(() => ({ send: async (request) => (request.path === "/instance" ? { status: 200, text: async () => agreeing, bytes: async () => encoder.encode(agreeing) } : replies.shift()!()) }), "limits");
    const ok = () => {
      const payload = JSON.stringify(encodeQueryResult({ kind: "snapshot", value: encoder.encode(JSON.stringify({ rows: [], learners: 0 })), frontier: null }));
      return { status: 200, text: async () => payload, bytes: async () => encoder.encode(payload) };
    };
    const seen: string[] = [];
    client.subscribe(() => seen.push(client.reachability()));
    replies.push(() => {
      throw new ProctorThrottled(1000);
    }, ok, () => {
      throw new ProctorUnavailable("connection refused");
    });
    await client.leaderboard({ period: "all-time" }, undefined).catch(() => undefined);
    await client.leaderboard({ period: "all-time" }, undefined);
    await client.leaderboard({ period: "all-time" }, undefined).catch(() => undefined);
    expect(seen).toEqual(["throttled", "reachable", "unreachable"]);

    const connection = (reachability: QuizConnection["reachability"], pending: number): QuizConnection => ({ online: true, reachability, pending, activity: pending > 0 ? "retrying" : "idle", deputy: false });
    for (const [locale, busy, waiting] of [
      ["en", "Server busy – retrying shortly", "The quiz server is busy right now – trying again shortly."],
      ["de", "Server ausgelastet – neuer Versuch in Kürze", "Der Quiz-Server ist gerade ausgelastet – neuer Versuch in Kürze."],
    ] as const) {
      expect(connectionMessage(connection("throttled", 2), quizText(locale))).toEqual({ message: busy, tone: "busy" });
      expect(connectionMessage(connection("throttled", 0), quizText(locale))).toEqual({ message: busy, tone: "busy" });
      expect(waitingMessage(connection("throttled", 0), undefined, quizText(locale))).toEqual({ message: waiting, retry: false });
    }
    expect(connectionMessage(connection("unreachable", 0), quizText("en")).tone).toBe("alert");
  });
});

describe("🚦️ polling", () => {
  it("never backs off beyond its cap", () => {
    expect(POLL_BACKOFF_MAX_MS).toBe(fixture.polling.maxMs);
  });

  for (const vector of fixture.polling.delays) {
    it(`waits ${vector.ms} ms after ${vector.failures} failures${vector.retryAfterMs === null ? "" : ` and a Retry-After of ${vector.retryAfterMs} ms`} (draw ${vector.random})`, () => {
      expect(pollDelay(fixture.polling.intervalMs, vector.failures, vector.retryAfterMs ?? undefined, vector.random)).toBe(vector.ms);
    });
  }

  it("polls at its interval while the proctor answers, backs off while it is busy and returns to the interval once it answers", async () => {
    vi.useFakeTimers();
    vi.spyOn(Math, "random").mockReturnValue(0.5);
    const outcomes: RefreshOutcome[] = [{ answered: true }, { answered: false, retryAfterMs: 30_000 }, { answered: false }, { answered: true }, { answered: true }];
    const calls: number[] = [];
    const started = Date.now();
    const refresh = async (): Promise<RefreshOutcome> => {
      calls.push(Date.now() - started);
      return outcomes.shift() ?? { answered: true };
    };
    const { unmount } = render(<Poller refresh={refresh} intervalMs={10_000} />);
    await act(async () => {
      await vi.advanceTimersByTimeAsync(200_000);
    });
    expect(calls.slice(0, 5)).toEqual([0, 10_000, 10_000 + 35_000, 10_000 + 35_000 + 25_000, 10_000 + 35_000 + 25_000 + 10_000]);
    unmount();
    const count = calls.length;
    await vi.advanceTimersByTimeAsync(100_000);
    expect(calls).toHaveLength(count);
  });
});

describe("🚦️ presence sockets", () => {
  it("counts a session as stable after ten seconds", () => {
    expect(PRESENCE_STABLE_MS).toBe(fixture.rejoin.stableMs);
  });

  for (const vector of fixture.rejoin.delays) {
    it(`rejoins ${vector.ms} ms after ${vector.flaps} short-lived sessions in a row (draw ${vector.random})`, () => {
      expect(rejoinDelay(vector.flaps, fixture.rejoin.timing, vector.random)).toBe(vector.ms);
    });
  }

  it("rejoins a socket that is welcomed and dropped again and again after growing pauses, and quickly again after a stable session", async () => {
    vi.useFakeTimers();
    const sockets: (PresenceSocket & { readonly openedAt: number })[] = [];
    const started = Date.now();
    const room = new PresenceRoom<CursorState>({
      url: "ws://proctor.test/x",
      connect: () => {
        const socket = { readyState: 1, onopen: null, onmessage: null, onclose: null, onerror: null, send: () => undefined, close: () => undefined, openedAt: Date.now() - started } as PresenceSocket & { readonly openedAt: number };
        sockets.push(socket);
        return socket;
      },
      parse: (state) => state as CursorState,
      timing: fixture.rejoin.timing,
      random: () => 1,
    });
    const flap = async (stayMs = 0): Promise<void> => {
      const socket = sockets.at(-1)!;
      socket.onmessage?.(new MessageEvent("message", { data: JSON.stringify({ type: "welcome", session: `s-${sockets.length}`, colour: 0, roster: [] }) }));
      await vi.advanceTimersByTimeAsync(stayMs);
      socket.onclose?.(new CloseEvent("close"));
      await vi.advanceTimersByTimeAsync(60_000);
    };
    room.start();
    for (let index = 0; index < 6; index += 1) await flap();
    const waited = sockets.slice(1).map((socket, index) => socket.openedAt - index * 60_000);
    expect(waited).toEqual([1, 2, 3, 4, 5, 6].map((flaps) => rejoinDelay(flaps, fixture.rejoin.timing, 1)));
    await flap(fixture.rejoin.stableMs);
    expect(sockets.at(-1)!.openedAt - (6 * 60_000 + fixture.rejoin.stableMs)).toBe(rejoinDelay(0, fixture.rejoin.timing, 1));
    room.stop();
  });
});
