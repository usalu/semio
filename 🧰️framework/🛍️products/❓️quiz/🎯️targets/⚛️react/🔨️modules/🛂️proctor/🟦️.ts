/** 🛂️ The quiz client's side of the proctor: quiz commands and queries mapped onto the framework server contract
 * (design §9a) and sent through `@semio-tech/framework-server`'s typed {@link ServerClient}.
 *
 * Every command carries a client-minted id that doubles as the idempotency key, so a retry after a connection
 * shortage is applied exactly once. Failures split into transient ones (no answer, a 5xx, a busy actor), which callers
 * retry with jittered backoff, and definitive ones (a quiz {@link Rejection} or a refusal), which they surface.
 *
 * @see ../../../../../🖥️server/🟦️.ts — `ServerClient`, `CommandEnvelope`, `QueryEnvelope`
 * @see ../../../../🧬️schema/🔣️.json — `Command`, `Event`, `Query`, `Rejection` and the views
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
import { REJECTIONS, type CatalogView, type Command, type Event, type Id, type Leaderboard, type LearnerView, type Query, type Rejection, type RunView } from "@semio-tech/quiz";

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

/** 🌐️ `fetch` against the proctor at `baseUrl` (`""` is the site's own origin) with a per-request timeout; a request
 * that never answers becomes {@link ProctorUnavailable}, and a non-JSON error page becomes a typed gateway error. */
export function proctorTransport(baseUrl: string, timeoutMs = 10_000): ProctorConnect {
  const root = baseUrl.replace(/\/+$/u, "");
  return (signal) => ({
    async send(request: HttpRequest): Promise<HttpResponse> {
      const query = new URLSearchParams(request.query as Record<string, string>).toString();
      const body = request.body === undefined ? undefined : typeof request.body === "string" ? request.body : new Uint8Array(request.body);
      let status: number;
      let text: string;
      try {
        const response = await fetchWithTimeout(`${root}${request.path}${query === "" ? "" : `?${query}`}`, { method: request.method, headers: { ...request.headers }, body }, { timeoutMs, signal });
        status = response.status;
        text = await response.text();
      } catch (error) {
        if (signal?.aborted) throw signal.reason ?? error;
        throw new ProctorUnavailable(error instanceof Error ? error.message : String(error));
      }
      const answer = status >= 400 && !jsonText(text) ? JSON.stringify({ kind: "http", message: `HTTP ${status}` }) : text;
      return { status, text: async () => answer, bytes: async () => new TextEncoder().encode(answer) };
    },
  });
}

/** 🔍️ Whether `error` is the proctor saying the addressed view does not exist (yet). */
export function isNotFound(error: unknown): boolean {
  return error instanceof ServerCallError && error.status === 404;
}

/** ⏳️ Whether `error` is a connection shortage worth retrying rather than a definitive answer. */
export function isTransient(error: unknown): boolean {
  if (error instanceof ProctorUnavailable) return true;
  if (error instanceof ServerCallError) return error.status >= 500 || error.status === 408 || error.status === 429;
  return false;
}

/** ✋️ `promise`, or the abort reason of `signal` as soon as it aborts. */
export function abortable<T>(promise: Promise<T>, signal?: AbortSignal): Promise<T> {
  if (signal === undefined) return promise;
  if (signal.aborted) return Promise.reject(signal.reason);
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

/** 🔁️ Runs `call` until it answers, retrying transient failures with jittered backoff; a definitive failure or an
 * abort of `signal` ends it. */
export async function retryTransient<T>(call: () => Promise<T>, timing: RetryTiming, signal?: AbortSignal): Promise<T> {
  const outcome = await retryWithJitteredBackoff<{ readonly value: T } | { readonly error: unknown }>(
    async () => {
      try {
        return { value: await call() };
      } catch (error) {
        if (isTransient(error)) throw error;
        return { error };
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

/** 🎭️ The actor a command is serialized through: the roster for identification, else the learner. */
export function commandTarget(command: Command, tenant: string): ActorKey {
  return command.type === "identify-learner" ? { tenant, kind: "quiz-roster", id: "roster" } : { tenant, kind: "quiz-learner", id: command.learner };
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

/** 🚫️ The quiz rejection a framework rejection carries in its notices or detail, if any. */
export function quizRejection(reason: ServerRejection, notices: readonly Notice[]): Rejection | undefined {
  const texts = [...notices.flatMap((notice) => [notice.code, notice.message]), "detail" in reason ? reason.detail : ""];
  for (const text of texts) {
    for (const token of text.match(REJECTION_TOKEN) ?? []) if ((REJECTIONS as readonly string[]).includes(token)) return token as Rejection;
  }
  return undefined;
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
/** 📶️ Whether the proctor answered the latest call. */
export type ProctorReachability = "unknown" | "reachable" | "unreachable";

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

  /** 🏆️ Every learner with a submitted run. */
  leaderboard(learner: Id | undefined, signal?: AbortSignal): Promise<Leaderboard> {
    return this.query<Leaderboard>({ type: "leaderboard" }, learner, signal);
  }

  private async observe<T>(call: () => Promise<T>, signal?: AbortSignal): Promise<T> {
    try {
      const value = await abortable(call(), signal);
      this.mark("reachable");
      return value;
    } catch (error) {
      if (!signal?.aborted) this.mark(isTransient(error) ? "unreachable" : "reachable");
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
