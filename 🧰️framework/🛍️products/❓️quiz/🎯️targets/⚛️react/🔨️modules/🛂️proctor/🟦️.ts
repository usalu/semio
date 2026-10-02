/** 🛂️ The quiz client's side of the proctor: quiz commands and queries mapped onto the framework server contract
 * (design §9a) and sent through `@semio-tech/framework-server`'s typed {@link ServerClient}.
 *
 * Every command carries a client-minted id that doubles as the idempotency key, so a retry after a connection
 * shortage is applied exactly once. Failures split into transient ones (no answer, a 5xx, a busy actor, a rate limit),
 * which callers retry with jittered backoff — after the wait a rate limit's `Retry-After` asks for — and definitive
 * ones (a quiz {@link Rejection} or a refusal), which they surface.
 *
 * @see ../../../../../🖥️server/🟦️.ts — `ServerClient`, `CommandEnvelope`, `QueryEnvelope`
 * @see ../../../../🧬️schema/🔣️.json — `Command`, `Event`, `Query`, `Rejection` and the views
 * @see https://www.rfc-editor.org/rfc/rfc9110#section-10.2.3 — `Retry-After`
 */

import { fetchWithTimeout, retryWithJitteredBackoff } from "@semio-tech/framework";
import {
  ServerCallError,
  ServerClient,
  type ActorKey,
  type CommandEnvelope,
  type CommandOutcome,
  type EventRecord,
  type HttpRequest,
  type HttpResponse,
  type HttpTransport,
  type Notice,
  type Principal,
  type QueryEnvelope,
  type QueryResult,
  type Rejection as ServerRejection,
} from "@semio-tech/framework-server";
import { REJECTIONS, handleActorId, normalizeHandle, type CatalogView, type Command, type CrowdView, type Event, type HandleView, type Id, type Leaderboard, type LeaderboardPeriod, type LearnerView, type Query, type Rejection, type RunView, type Slug } from "@semio-tech/quiz";

export type { HttpRequest, HttpResponse, HttpTransport };

//#region 🆔️Ids
/** 🆔️ A fresh learner, run or command id: 128 random bits from `crypto.getRandomValues` as 32 lowercase hex chars. */
export function newId(): Id {
  return Array.from(crypto.getRandomValues(new Uint8Array(16)), (byte) => byte.toString(16).padStart(2, "0")).join("");
}
//#endregion 🆔️Ids

//#region 🔌️Transport
/** 📵️ The proctor could not be asked right now: no answer, a gateway failure or a busy actor. Always worth a retry. */
export class ProctorUnavailable extends Error {
  constructor(message: string) {
    super(message);
    this.name = "ProctorUnavailable";
  }
}

/** 🚦️ The proctor answered `429`: it is there, but asks this client to slow down — for `retryAfterMs` when it said how
 * long. When it names an `allowance`, the request was not too fast: that allowance of the client's address is spent. */
export class ProctorThrottled extends ProctorUnavailable {
  readonly retryAfterMs: number | undefined;
  readonly allowance: string | undefined;

  constructor(retryAfterMs?: number, allowance?: string) {
    super(allowance === undefined ? "the proctor asks to slow down" : `the ${allowance} allowance of this address is spent`);
    this.name = "ProctorThrottled";
    this.retryAfterMs = retryAfterMs;
    this.allowance = allowance;
  }
}

/** 🎟️ The allowance of a client address the proctor spends on registrations: so many learners at once, so many more per
 * hour. While it is spent, a sign-up from that address is refused; everything else — playing, recalling a pseudonym —
 * goes on. */
export const SIGN_UP_ALLOWANCE = "sign-up";

/** 🙅️ Whether `error` is the proctor saying that the sign-up allowance of this client's address is spent: an answer to
 * show the learner, not a shortage to wait out by sending the sign-up again. */
export function signUpsSpent(error: unknown): error is ProctorThrottled {
  return error instanceof ProctorThrottled && error.allowance === SIGN_UP_ALLOWANCE;
}

/** ⏳️ The longest wait a `Retry-After` may ask for; a longer one is cut, so a wrong header never parks the client. */
export const RETRY_AFTER_MAX_MS = 120_000;

const DELAY_SECONDS = /^[0-9]+$/u;
const HTTP_DATE = /^[A-Za-z]{3}, [0-9]{2} [A-Za-z]{3} [0-9]{4} [0-9]{2}:[0-9]{2}:[0-9]{2} GMT$/u;

/** ⏳️ The wait a `Retry-After` header asks for at `now`: delay-seconds (a non-negative integer) or an HTTP-date, at most
 * {@link RETRY_AFTER_MAX_MS}; `undefined` for a missing header or anything else. */
export function retryAfterMs(header: string | null, now: number): number | undefined {
  const value = header?.trim() ?? "";
  if (DELAY_SECONDS.test(value)) return Math.min(RETRY_AFTER_MAX_MS, Number(value) * 1000);
  const at = HTTP_DATE.test(value) ? Date.parse(value) : Number.NaN;
  return Number.isNaN(at) ? undefined : Math.min(RETRY_AFTER_MAX_MS, Math.max(0, at - now));
}

/** 🔌️ Opens a transport whose requests abort with `signal`. */
export type ProctorConnect = (signal?: AbortSignal) => HttpTransport;

function jsonText(text: string): boolean {
  try {
    JSON.parse(text);
    return true;
  } catch {
    return false;
  }
}

/** 💬️ What an error body says besides its kind: the wait it asks for in its `retryAfterMs` member — what the proctor
 * repeats there for a browser that may not read the `Retry-After` header of another origin, capped like the header —
 * and the allowance it names as spent; each `undefined` when the body has none. */
function asked(body: string): { readonly waitMs: number | undefined; readonly allowance: string | undefined } {
  try {
    const { retryAfterMs: wait, allowance } = (JSON.parse(body) ?? {}) as { readonly retryAfterMs?: unknown; readonly allowance?: unknown };
    return { waitMs: typeof wait === "number" && Number.isFinite(wait) ? Math.min(RETRY_AFTER_MAX_MS, Math.max(0, wait)) : undefined, allowance: typeof allowance === "string" ? allowance : undefined };
  } catch {
    return { waitMs: undefined, allowance: undefined };
  }
}

/** 🌐️ `fetch` against the proctor at `baseUrl` (`""` is the site's own origin) with a per-request timeout; a request
 * that never answers becomes {@link ProctorUnavailable}; a `429` — and a `503` that says when to come back — becomes
 * {@link ProctorThrottled} with the wait its `Retry-After` header (else its body) asks for and, for a `429`, the
 * allowance its body names as spent; and a non-JSON error page becomes a typed gateway error. */
export function proctorTransport(baseUrl: string, timeoutMs = 10_000): ProctorConnect {
  const root = baseUrl.replace(/\/+$/u, "");
  return (signal) => ({
    async send(request: HttpRequest): Promise<HttpResponse> {
      const query = new URLSearchParams(request.query as Record<string, string>).toString();
      const body = request.body === undefined ? undefined : typeof request.body === "string" ? request.body : new Uint8Array(request.body);
      let status: number;
      let text: string;
      let wait: number | undefined;
      try {
        const response = await fetchWithTimeout(`${root}${request.path}${query === "" ? "" : `?${query}`}`, { method: request.method, headers: { ...request.headers }, body }, { timeoutMs, signal });
        status = response.status;
        wait = retryAfterMs(response.headers.get("retry-after"), Date.now());
        text = await response.text();
      } catch (error) {
        if (signal?.aborted) throw signal.reason ?? error;
        throw new ProctorUnavailable(error instanceof Error ? error.message : String(error));
      }
      const said = status === 429 || status === 503 ? asked(text) : undefined;
      if (said !== undefined && (status === 429 || (wait ?? said.waitMs) !== undefined)) throw new ProctorThrottled(wait ?? said.waitMs, status === 429 ? said.allowance : undefined);
      const answer = status >= 400 && !jsonText(text) ? JSON.stringify({ kind: "http", message: `HTTP ${status}` }) : text;
      return { status, text: async () => answer, bytes: async () => new TextEncoder().encode(answer) };
    },
  });
}

/** 🔍️ Whether `error` is the proctor saying the addressed view does not exist (yet). */
export function isNotFound(error: unknown): boolean {
  return error instanceof ServerCallError && error.status === 404;
}

/** ⏳️ Whether `error` is a connection shortage worth retrying rather than a definitive answer; a spent sign-up
 * allowance is an answer. */
export function isTransient(error: unknown): boolean {
  if (signUpsSpent(error)) return false;
  if (error instanceof ProctorUnavailable) return true;
  if (error instanceof ServerCallError) return error.status >= 500 || error.status === 408 || error.status === 429;
  return false;
}

/** 🚦️ Whether `error` is the proctor asking this client to slow down (a rate limit) — not a spent sign-up allowance,
 * which no slower sending cures. */
export function isThrottled(error: unknown): boolean {
  return !signUpsSpent(error) && (error instanceof ProctorThrottled || (error instanceof ServerCallError && error.status === 429));
}

/** ⏳️ How long the proctor asked to wait before the call that failed with `error` is tried again; 0 when it did not say. */
export function retryWait(error: unknown): number {
  return error instanceof ProctorThrottled ? (error.retryAfterMs ?? 0) : 0;
}

/** 💤️ Resolves after `ms`; rejects with the abort reason of `signal` as soon as it aborts. */
export function pause(ms: number, signal?: AbortSignal): Promise<void> {
  return new Promise<void>((resolve, reject) => {
    if (signal?.aborted) {
      reject(signal.reason);
      return;
    }
    const abort = (): void => {
      clearTimeout(timer);
      reject(signal?.reason);
    };
    const timer = setTimeout(() => {
      signal?.removeEventListener("abort", abort);
      resolve();
    }, ms);
    signal?.addEventListener("abort", abort, { once: true });
  });
}

/** ✋️ `promise`, or the abort reason of `signal` as soon as it aborts; a `promise` that loses the race is still observed, so
 * its own rejection (usually the same abort) never surfaces as an unhandled one. */
export function abortable<T>(promise: Promise<T>, signal?: AbortSignal): Promise<T> {
  if (signal === undefined) return promise;
  if (signal.aborted) {
    promise.catch(() => undefined);
    return Promise.reject(signal.reason);
  }
  return new Promise<T>((resolve, reject) => {
    const abort = (): void => reject(signal.reason);
    signal.addEventListener("abort", abort, { once: true });
    promise.then(
      (value) => {
        signal.removeEventListener("abort", abort);
        resolve(value);
      },
      (error: unknown) => {
        signal.removeEventListener("abort", abort);
        reject(error);
      },
    );
  });
}

/** 🔁️ Backoff bounds for retrying a transient failure. */
export interface RetryTiming {
  readonly minMs: number;
  readonly maxMs: number;
}

/** 🔁️ The production backoff: half a second, growing to fifteen seconds. */
export const RETRY_TIMING: RetryTiming = { minMs: 500, maxMs: 15_000 };

/** 🔁️ Runs `call` until it answers, retrying transient failures with jittered backoff — after a rate limit not before
 * the wait the proctor asked for; a definitive failure or an abort of `signal` ends it at once. */
export async function retryTransient<T>(call: () => Promise<T>, timing: RetryTiming, signal?: AbortSignal): Promise<T> {
  const outcome = await retryWithJitteredBackoff<{ readonly value: T } | { readonly error: unknown }>(
    async () => {
      try {
        return { value: await call() };
      } catch (error) {
        if (!isTransient(error)) return { error };
        if (retryWait(error) > 0) await pause(retryWait(error), signal);
        throw error;
      }
    },
    { ...timing, signal },
  );
  if ("error" in outcome) throw outcome.error;
  return outcome.value;
}
//#endregion 🔌️Transport

//#region 📨️Envelopes
const encoder = new TextEncoder();
const decoder = new TextDecoder();

/** 🔢️ The wire version of every quiz command and query kind. */
export const QUIZ_WIRE_VERSION = 1;

/** 🎭️ The actor a command is serialized through: the learner — except a registration under a pseudonym or name, which
 * goes through the actor of its handle key, so a handle is claimed once. */
export function commandTarget(command: Command, tenant: string): ActorKey {
  if (command.type !== "identify-learner" || command.identity.kind === "anonymous") return { tenant, kind: "quiz-learner", id: command.learner };
  return { tenant, kind: "quiz-handle", id: handleActorId(normalizeHandle(command.identity.handle)?.key ?? command.identity.handle.toLowerCase()) };
}

/** 🙋️ Who acts: nobody before identification, the learner after it. */
export function learnerPrincipal(learner: Id | undefined): Principal {
  return learner === undefined ? { kind: "anonymous" } : { kind: "user", id: learner };
}

/** 📨️ The framework envelope of one quiz command. */
export function commandEnvelope(command: Command, tenant: string, now: number): CommandEnvelope {
  return {
    commandId: command.id,
    kind: `quiz.${command.type}`,
    version: QUIZ_WIRE_VERSION,
    target: commandTarget(command, tenant),
    scope: tenant,
    principal: learnerPrincipal(command.type === "identify-learner" ? undefined : command.learner),
    session: null,
    device: null,
    payload: encoder.encode(JSON.stringify(command)),
    causalFrontier: null,
    clientHlc: { millis: now, counter: 0 },
    expectedRevision: null,
    idempotencyKey: command.id,
    capabilityProof: null,
    trace: { traceId: newId(), spanId: newId().slice(0, 16) },
  };
}

/** ❓️ The framework envelope of one quiz query, answered by the authority's read models. */
export function queryEnvelope(query: Query, tenant: string, learner: Id | undefined): QueryEnvelope {
  return {
    queryId: newId(),
    kind: `quiz.${query.type}`,
    version: QUIZ_WIRE_VERSION,
    scope: tenant,
    principal: learnerPrincipal(learner),
    arguments: encoder.encode(JSON.stringify(query)),
    consistency: { kind: "authority" },
    cursor: null,
  };
}

/** ⚖️ What the proctor decided about one command. */
export type CommandVerdict = { readonly kind: "accepted"; readonly events: readonly Event[] } | { readonly kind: "rejected"; readonly rejection: Rejection } | { readonly kind: "refused"; readonly detail: string };

const REJECTION_TOKEN = /[a-z]+(?:-[a-z]+)+/gu;

function rejectionIn(texts: readonly string[]): Rejection | undefined {
  for (const text of texts) {
    for (const token of text.match(REJECTION_TOKEN) ?? []) if ((REJECTIONS as readonly string[]).includes(token)) return token as Rejection;
  }
  return undefined;
}

/** 🚫️ The quiz rejection a framework rejection carries in its notices or detail, if any. */
export function quizRejection(reason: ServerRejection, notices: readonly Notice[]): Rejection | undefined {
  return rejectionIn([...notices.flatMap((notice) => [notice.code, notice.message]), "detail" in reason ? reason.detail : ""]);
}

/** 🚫️ The quiz rejection a refused query names in its error, if any (a handle outside the policy, a malformed id). */
export function errorRejection(error: unknown): Rejection | undefined {
  return error instanceof ServerCallError ? rejectionIn([error.message]) : undefined;
}

function quizEvents(records: readonly EventRecord[]): Event[] {
  return records.filter((record) => record.kind.startsWith("quiz.")).map((record) => JSON.parse(decoder.decode(record.payload)) as Event);
}

/** ⚖️ The quiz verdict of a framework outcome; a busy actor or a revision race is {@link ProctorUnavailable}. */
export function commandVerdict(outcome: CommandOutcome): CommandVerdict {
  switch (outcome.status) {
    case "accepted":
      return { kind: "accepted", events: quizEvents(outcome.events) };
    case "transformed":
      return { kind: "accepted", events: quizEvents(outcome.canonicalEvents) };
    case "pending":
      return { kind: "accepted", events: [] };
    case "rejected": {
      const reason = outcome.reason;
      const rejection = quizRejection(reason, outcome.notices);
      if (rejection !== undefined) return { kind: "rejected", rejection };
      if (reason.kind === "actorUnavailable" || reason.kind === "revisionConflict") throw new ProctorUnavailable(reason.kind);
      return { kind: "refused", detail: reason.kind === "unknownCommandKind" ? `${reason.kind}: ${reason.commandKind}` : `${reason.kind}: ${reason.detail}` };
    }
  }
}

function viewOf<V>(result: QueryResult): V {
  const bytes = result.kind === "snapshot" ? result.value : result.kind === "subscription" ? result.initial : result.items[0];
  if (bytes === undefined) throw new ProctorUnavailable("empty query result");
  return JSON.parse(decoder.decode(bytes)) as V;
}
//#endregion 📨️Envelopes

//#region 🛂️Client
/** 📶️ What the latest call found: nothing asked yet, an answer, the answer "slow down" (a rate limit), or no answer. */
export type ProctorReachability = "unknown" | "reachable" | "throttled" | "unreachable";

/** 🛂️ Quiz commands and queries against one proctor tenant, observing whether the proctor is reachable. */
export class ProctorClient {
  readonly tenant: string;
  private readonly connect: ProctorConnect;
  private readonly now: () => number;
  private reachable: ProctorReachability = "unknown";
  private readonly listeners = new Set<() => void>();

  constructor(connect: ProctorConnect, tenant: string, now: () => number = Date.now) {
    this.connect = connect;
    this.tenant = tenant;
    this.now = now;
  }

  /** 📶️ The reachability observed by the latest call. */
  reachability(): ProctorReachability {
    return this.reachable;
  }

  /** 🔔️ Calls `listener` whenever {@link reachability} changes; returns the unsubscribe function. */
  subscribe(listener: () => void): () => void {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }

  /** 🔇️ Notes that a caller gave a call up because no answer came in the time it had: the proctor counts as
   * unreachable until its next answer. */
  silent(): void {
    this.mark("unreachable");
  }

  /** 📨️ Submits one command once (callers retry transient failures with the same command id). */
  command(command: Command, signal?: AbortSignal): Promise<CommandVerdict> {
    return this.observe(async () => commandVerdict(await new ServerClient(this.connect(signal)).submitCommand(commandEnvelope(command, this.tenant, this.now()))), signal);
  }

  /** ❓️ Answers one query from the proctor's read models. */
  query<V>(query: Query, learner: Id | undefined, signal?: AbortSignal): Promise<V> {
    return this.observe(async () => viewOf<V>(await new ServerClient(this.connect(signal)).query(queryEnvelope(query, this.tenant, learner))), signal);
  }

  /** 📚️ The solution-free catalog. */
  catalog(learner: Id | undefined, signal?: AbortSignal): Promise<CatalogView> {
    return this.query<CatalogView>({ type: "catalog" }, learner, signal);
  }

  /** 🧑‍🎓️ One learner's runs, badges and best scores. */
  learner(learner: Id, signal?: AbortSignal): Promise<LearnerView> {
    return this.query<LearnerView>({ type: "learner", learner }, learner, signal);
  }

  /** 🏃️ One run with its sheet, answers and result. */
  run(run: Id, learner: Id, signal?: AbortSignal): Promise<RunView> {
    return this.query<RunView>({ type: "run", run }, learner, signal);
  }

  /** 👥️ What the learners answered in the submitted runs of `quiz`, per task and item. */
  crowd(quiz: Slug, learner: Id | undefined, signal?: AbortSignal): Promise<CrowdView> {
    return this.query<CrowdView>({ type: "crowd", quiz }, learner, signal);
  }

  /** 🏆️ One leaderboard — a period, of every quiz or of the one `board` names — as it stands at the proctor's clock:
   * the top learners with a submitted run in it, how many are ranked, and the row of `learner` when it is ranked. */
  leaderboard(board: { readonly period: LeaderboardPeriod; readonly quiz?: Slug }, learner: Id | undefined, signal?: AbortSignal): Promise<Leaderboard> {
    return this.query<Leaderboard>({ type: "leaderboard", period: board.period, ...(board.quiz === undefined ? {} : { quiz: board.quiz }), ...(learner === undefined ? {} : { learner }) }, learner, signal);
  }

  /** 🔦️ Who holds `handle`, with its normalized display; recalling a learner is this read. */
  handle(handle: string, signal?: AbortSignal): Promise<HandleView> {
    return this.query<HandleView>({ type: "handle", handle }, undefined, signal);
  }

  private async observe<T>(call: () => Promise<T>, signal?: AbortSignal): Promise<T> {
    try {
      const value = await abortable(call(), signal);
      this.mark("reachable");
      return value;
    } catch (error) {
      if (!signal?.aborted) this.mark(isThrottled(error) ? "throttled" : isTransient(error) ? "unreachable" : "reachable");
      throw error;
    }
  }

  private mark(reachable: ProctorReachability): void {
    if (this.reachable === reachable) return;
    this.reachable = reachable;
    for (const listener of this.listeners) listener();
  }
}
//#endregion 🛂️Client
