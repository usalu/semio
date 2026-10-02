/** 🏋️ The proctor's capacity gate: a lecture hall against the real binary, then the same hall beside one abusive script.
 *
 * **What runs.** The release build of `proctor serve` in production mode (origin allowlist, trusted proxy forwarding,
 * the production limits stated explicitly: {@link LIMITS}) on a throw-away loopback port over a throw-away data
 * directory and the architecture catalog. The client is this file on Bun's own `fetch`, `WebSocket` and sockets — an
 * HTTP and WebSocket implementation that shares no code with the proctor — speaking the quiz client's real envelopes
 * ({@link ProctorClient}) and presence frames.
 *
 * **The hall.** `learners` simulated learners (300 by default) arrive from ONE address (the `X-Forwarded-For` the
 * terminating proxy would write): each enrols, joins the roster room and the home room (watching every page, quiz and
 * thinking room at 4 Hz), polls the leaderboard, then plays two quizzes — start a run, read its sheet, join the quiz
 * and thinking rooms, revise every task's answer a few times with cursor moves and drafts in between, submit, read the
 * result, the crowd and the leaderboard. Think times are real ones divided by `compression` (5 by default), so the
 * hall asks for `compression` times the requests per second a real lecture does. Once per phase every socket of every
 * learner drops and reconnects within two seconds (the Wi-Fi drop). A request that gets no answer is sent again, and a
 * socket join that gets none is made again, after the quiz client's own backoff, and the wait counts as its latency.
 *
 * **The script.** In the second phase a worker thread plays one abusive client from another address beside the hall:
 * it tries to fill the roster — registrations, anonymous and under handles, as fast as it can post them — while the
 * hall signs up, floods malformed commands and queries on connections it keeps open ({@link Wire}), sends oversized
 * bodies, pushes sockets into the hall's own roster room and sends oversized frames.
 *
 * **The load generator measures itself.** The hall is dealt to {@link HALL_THREADS} threads, and every thread probes
 * how late its own timers fire. A latency of the hall includes that lateness, so a latency over the budget in a phase
 * in which the generator ran later than {@link GENERATOR_LAG_MS} (p99) says nothing about the proctor and fails the
 * gate as *not measured*; so does any phase in which it ran later than {@link GENERATOR_STALL_MS}.
 *
 * **The verdict.** The hall must see no error, no refusal and no `429`/`503` in either phase — every one of its sign-ups
 * is registered, twice three hundred from one address within minutes —, send at most {@link RESENT_SHARE} of its
 * requests and make at most that share of its socket joins twice, and stay inside the latency budget ({@link BUDGET}); the script must be refused (`429`) and served no
 * more than its address's allowances over the time it ran — of commands, of queries and of sign-ups, the last named in
 * the refusal —, capped (sockets) and cut off (`413`, closed sockets); beside the script's sockets the hall must still
 * be sent {@link PRESENCE_SHARE} of the presence bytes it is sent alone; the roster, counted by the stopped proctor's own
 * `prune --dry-run`, must hold the hall and what the script's sign-up allowance let through and so end far below the
 * cap, with what the script left being exactly what a prune removes; the proctor must stay inside its memory budget
 * and answer `GET /instance` at the end. The proctor runs with its default cap of learners. The report lists
 * p50/p95/p99 per operation and phase, the script's answers by status, the roster and the proctor's memory.
 *
 * @see ../../README.md — the limits the proctor enforces and their variables
 * @see ../../../../🧰️framework/🛍️products/🖥️server/🔨️modules/🚦️throttle/🦀️.rs — the throttle
 * @see ../../../../🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🛂️proctor/🟦️.ts — the client mapping driven here
 */
import { execFile } from "node:child_process";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { connect, createServer, type Socket } from "node:net";
import { availableParallelism, tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { Worker, isMainThread, parentPort, workerData } from "node:worker_threads";
import { DEFAULT_LIMITS, learnerTag, rosterScope, thinkingScope, type Answer, type Command, type IdentityClaim, type PresenceState, type RunView, type SheetTask, type ThinkingAnswer } from "../../../../🧰️framework/🛍️products/❓️quiz/🟦️.ts";
import { ProctorClient, ProctorUnavailable, RETRY_TIMING, commandEnvelope, newId, retryTransient, type CommandVerdict } from "../../../../🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🛂️proctor/🟦️.ts";
import { PRESENCE_PROTOCOL, encodeCommandEnvelope, presenceSocketUrl, type HttpRequest, type HttpResponse, type HttpTransport } from "../../../../🧰️framework/🛍️products/🖥️server/🟦️.ts";
import { PROCTOR_DEV_CATALOG, buildProctor, launchProctor, type ProctorProcess } from "../../🏗️bootstrap/🟦️.ts";

//#region 🎚️Settings
/** 🎚️ What one capacity run plays. */
export interface CapacityOptions {
  /** 🧑‍🎓️ Learners in the hall. */
  readonly learners: number;
  /** ⏩️ How many times faster than a real lecture the hall thinks. */
  readonly compression: number;
  /** 📝️ Where the JSON report is written, when it is. */
  readonly report?: string;
  /** 📦️ A `proctor` binary to measure instead of building the release one: the binary of an image, an older build. */
  readonly executable?: string;
  /** 🏫️ Play the hall alone and skip the script: a quick look at the latencies, not the gate. */
  readonly hallOnly?: boolean;
}

/** 🏫️ The defaults: the class the limits are sized for, five times as fast as it is in life. */
export const CAPACITY_DEFAULTS: CapacityOptions = { learners: 300, compression: 5 };

/** 🌐️ The site origin the capacity proctor allows. */
const SITE = "https://quizzes.capacity.test";
/** 🏫️ The one address the whole hall arrives from. */
const HALL = "198.51.100.10";
/** 😈️ The address of the abusive script. */
const SCRIPT = "203.0.113.66";
/** 🔌️ Ports other agents and dev servers use; never picked. */
const RESERVED_PORTS = new Set([8791, 8891, 8892, 6061, 6161, 6162]);

/** ⏱️ The latency budget of the hall in milliseconds, per phase: the 95th and 99th percentile of every command and
 * query. The hall beside the script may be slower than the hall alone, and still has to feel instant. */
export const BUDGET = {
  alone: { p95: 250, p99: 500 },
  beside: { p95: 500, p99: 1500 },
} as const;

/** 🧠️ The most resident memory the proctor may reach, in MiB. */
export const MEMORY_BUDGET_MIB = 512;

/** 🔁️ The share of its requests the hall may have to send again, and of its socket joins it may have to make again,
 * because one got no answer. On one machine the hall, the script and the proctor share one pool of client ports, and a
 * script that opens connections as fast as it can empties it; the quiz client sends such a request and joins such a
 * room again after its backoff, and the wait counts as latency. */
export const RESENT_SHARE = 0.01;

/** ⌛️ How long a learner's request or join is tried again before it counts as failed. */
const GIVE_UP_MS = 30_000;

/** 🕳️ The close code of a socket that never opened because there was no connection or the connection was cut before
 * the proctor's answer; an upgrade the proctor answered with a status closes with 1002 instead. */
const NO_ANSWER = 1006;

/** 🚦️ The edge limits the capacity proctor is started with — its production defaults, stated to it explicitly so that
 * the verdict on the script is computed from the numbers the proctor was given. */
export const LIMITS = { commandsPerSecond: 300, commandsBurst: 900, queriesPerSecond: 600, queriesBurst: 1800, signUpsPerHour: 100, signUpsBurst: 1200, socketsPerAddress: 2048, bodyBytes: 16 * 1024 } as const;

/** 🎟️ The allowance a refused sign-up names. */
const SIGN_UP = "sign-up";

/** 🧱️ The cap of learners the capacity proctor runs with: the default, which it is not told. */
const ROSTER_CAP = DEFAULT_LIMITS.learners;

/** 🪶️ The share of the cap the roster may hold at the end for it to be far below the cap. */
export const ROSTER_SHARE = 0.05;

/** 🍰️ The least share of the presence bytes it is sent alone that the hall must still be sent while the script's sockets
 * sit in its rooms: the rooms' budget is dealt among addresses, so a flood of sockets from one address takes its own
 * share and leaves the hall an even split of the total — which is less than the busiest moments of a hall alone ask
 * for, and more than half of what it is sent over a lecture. */
export const PRESENCE_SHARE = 0.5;

/** 🐌️ How late (p99) the load generator's own timers may fire for a latency over the budget to say something about
 * the proctor: lateness of the generator is counted into every latency, so it can only make the hall look slower. A
 * hall inside its budget although the generator ran late is inside its budget. */
export const GENERATOR_LAG_MS = 100;

/** 🧊️ How late (p99) the load generator may run before the hall it plays is no longer the hall: its learners then act
 * so far off their schedule that the phase measures nothing, whatever the latencies. */
export const GENERATOR_STALL_MS = 1_000;

/** 🧮️ Real think times in milliseconds, before compression. */
const REAL = { arrival: 60_000, home: 15_000, revise: [4_000, 12_000] as const, read: 5_000, poll: 10_000, storm: 2_000 };
/** 🖱️ How often a learner who moves the pointer shares it, and how much of the time one does. */
const CURSOR = { intervalMs: 100, share: 0.3 };
/** 👀️ The interval a home page watches the other rooms at. */
const WATCH_INTERVAL_MS = 250;
/** 🐌️ How often a thread of the load generator probes its own lateness. */
const LAG_PROBE_MS = 20;
/** 🧵️ The threads the hall is played on, so that the load generator's own queueing stays out of the hall's latencies. */
const HALL_THREADS = Math.max(2, Math.min(8, Math.floor(availableParallelism() / 2)));
//#endregion 🎚️Settings

//#region 📊️Measurements
/** 📊️ What one phase measured of the hall. */
interface Measurements {
  readonly latencies: Map<string, number[]>;
  readonly errors: string[];
  throttled: number;
  /** 📮️ Commands and queries the hall made, and how many more times it had to send one that got no answer. */
  requests: number;
  resent: number;
  /** 🔌️ Presence sockets of the hall: joined (welcomed), lost, refused a frame, and how many more times a join that got
   * no answer had to be made. */
  sockets: { opened: number; dropped: number; refused: number; rejoined: number; frames: number; bytes: number };
}

function measurements(): Measurements {
  return { latencies: new Map(), errors: [], throttled: 0, requests: 0, resent: 0, sockets: { opened: 0, dropped: 0, refused: 0, rejoined: 0, frames: 0, bytes: 0 } };
}

function record(into: Measurements, operation: string, milliseconds: number): void {
  const samples = into.latencies.get(operation);
  if (samples === undefined) into.latencies.set(operation, [milliseconds]);
  else samples.push(milliseconds);
}

function fail(into: Measurements, message: string): void {
  if (into.errors.length < 200) into.errors.push(message);
  else if (into.errors.length === 200) into.errors.push("… further errors are not listed");
}

/** 📐️ The nearest-rank percentile of `samples` (milliseconds, rounded to a tenth). */
export function percentile(samples: readonly number[], rank: number): number {
  if (samples.length === 0) return 0;
  const sorted = [...samples].sort((left, right) => left - right);
  return Math.round(sorted[Math.min(sorted.length - 1, Math.max(0, Math.ceil((rank / 100) * sorted.length) - 1))] * 10) / 10;
}

/** 🧾️ One row of the report: an operation's count and percentiles. */
export interface LatencyRow {
  readonly operation: string;
  readonly count: number;
  readonly p50: number;
  readonly p95: number;
  readonly p99: number;
  readonly max: number;
}

function rows(of: Measurements): LatencyRow[] {
  return [...of.latencies.entries()].sort(([left], [right]) => left.localeCompare(right)).map(([operation, samples]) => ({ operation, count: samples.length, p50: percentile(samples, 50), p95: percentile(samples, 95), p99: percentile(samples, 99), max: percentile(samples, 100) }));
}
//#endregion 📊️Measurements

//#region 🚚️Transport
const pause = (milliseconds: number): Promise<void> => new Promise((done) => setTimeout(done, milliseconds));
const between = (low: number, high: number): number => low + Math.random() * (high - low);

/** 📨️ The headers the terminating proxy forwards a request of the site with, for the client at `address`. */
function forwarded(address: string): Record<string, string> {
  return { "x-forwarded-proto": "https", "x-forwarded-for": address, origin: SITE };
}

/** 🚚️ `fetch` against the proctor as the proxy would forward the client at `address`. A request that got no answer —
 * no connection, a connection cut — is {@link ProctorUnavailable}, as it is to the quiz client's own transport. */
function transport(base: string, address: string, seen?: (status: number) => void): HttpTransport {
  return {
    async send(request: HttpRequest): Promise<HttpResponse> {
      const query = new URLSearchParams(request.query as Record<string, string>).toString();
      let status: number;
      let text: string;
      try {
        const response = await fetch(`${base}${request.path}${query === "" ? "" : `?${query}`}`, { method: request.method, headers: { ...request.headers, ...forwarded(address) }, body: request.body === undefined ? undefined : typeof request.body === "string" ? request.body : new Uint8Array(request.body) });
        status = response.status;
        text = await response.text();
      } catch (error) {
        throw new ProctorUnavailable(error instanceof Error ? `${(error as { code?: string }).code ?? error.name}: ${error.message}` : String(error));
      }
      seen?.(status);
      return { status, text: async () => text, bytes: async () => new TextEncoder().encode(text) };
    },
  };
}

/** 🚪️ One presence socket of a learner: joins `scope`, counts what arrives, resends its latest state after a reconnect. */
class Room {
  private socket: WebSocket | undefined;
  private latest: unknown;
  private watching: readonly string[] | undefined;
  private gone = false;

  constructor(
    private readonly base: string,
    private readonly address: string,
    private readonly scope: string,
    private readonly surface: string,
    private readonly into: () => Measurements,
  ) {}

  /** 🔌️ Join the room as the quiz client's room does: a join that got no answer is made again after the client's own
   * backoff, and the time until the `welcome` — the waits included — is what the learner waited. */
  async join(): Promise<void> {
    const started = performance.now();
    let attempts = 0;
    try {
      await retryTransient(
        () => {
          attempts += 1;
          return this.gone ? Promise.resolve() : this.open(started);
        },
        RETRY_TIMING,
        AbortSignal.timeout(GIVE_UP_MS),
      );
    } catch (error) {
      fail(this.into(), `presence ${this.scope}: ${error instanceof Error ? `${error.name}: ${error.message}` : String(error)}`);
    } finally {
      this.into().sockets.rejoined += Math.max(0, attempts - 1);
    }
    if (this.latest !== undefined) this.send({ type: "state", state: this.latest });
    if (this.watching !== undefined) this.send({ type: "watch", scopes: this.watching, intervalMs: WATCH_INTERVAL_MS });
  }

  /** 🚪️ Open one socket and wait for its `welcome`. A socket that never opened because there was no connection or the
   * connection was cut ({@link NO_ANSWER}) is {@link ProctorUnavailable}, as a request without an answer is; a join the
   * proctor answered with a refusal, and a socket that closes after its welcome, are errors of the hall. */
  private open(started: number): Promise<void> {
    const socket = new WebSocket(presenceSocketUrl(this.base, this.scope, this.surface), { headers: forwarded(this.address), protocols: [PRESENCE_PROTOCOL] } as unknown as string[]);
    this.socket = socket;
    const into = this.into();
    return new Promise<void>((welcomed, unanswered) => {
      let greeted = false;
      socket.addEventListener("message", (event) => {
        const text = typeof event.data === "string" ? event.data : "";
        const now = this.into();
        now.sockets.frames += 1;
        now.sockets.bytes += text.length;
        if (!greeted) {
          greeted = true;
          record(into, "socket join", performance.now() - started);
          into.sockets.opened += 1;
          welcomed();
        } else if (text.startsWith('{"type":"refused"')) {
          now.sockets.refused += 1;
          fail(now, `presence ${this.scope} refused a frame: ${text.slice(0, 160)}`);
        }
      });
      socket.addEventListener("close", (event) => {
        if (this.socket === socket) {
          this.socket = undefined;
          if (!greeted && event.code === NO_ANSWER) {
            unanswered(new ProctorUnavailable(`no welcome from ${this.scope} (${event.reason || event.code})`));
            return;
          }
          const now = this.into();
          now.sockets.dropped += 1;
          fail(now, `presence ${this.scope} closed (${event.code}${greeted ? "" : ", before its welcome"})`);
        }
        welcomed();
      });
      socket.addEventListener("error", () => undefined);
    });
  }

  /** 📣️ Share `state` (the latest wins; it is sent again after a reconnect). */
  share(state: unknown): void {
    this.latest = state;
    this.send({ type: "state", state });
  }

  /** 👀️ Watch `scopes` read-only. */
  watch(scopes: readonly string[]): void {
    this.watching = scopes;
    this.send({ type: "watch", scopes, intervalMs: WATCH_INTERVAL_MS });
  }

  /** 📶️ Drop the socket as a network would and join again after `afterMs`, unless the learner left meanwhile. */
  async reconnect(afterMs: number): Promise<void> {
    this.drop();
    await pause(afterMs);
    if (!this.gone) await this.join();
  }

  /** 🚶️ Leave the room for good. */
  leave(): void {
    this.gone = true;
    this.drop();
  }

  private drop(): void {
    const socket = this.socket;
    this.socket = undefined;
    socket?.close();
  }

  private send(frame: unknown): void {
    if (this.socket?.readyState === WebSocket.OPEN) this.socket.send(JSON.stringify(frame));
  }
}
//#endregion 🚚️Transport

//#region 🧑‍🎓️Learner
/** 🏫️ What every learner of one phase shares. */
interface Hall {
  readonly base: string;
  readonly tenant: string;
  readonly quizzes: readonly string[];
  readonly scale: number;
  readonly measured: () => Measurements;
  readonly rooms: Set<Room>;
}

function shuffled<T>(items: readonly T[]): T[] {
  const order = [...items];
  for (let index = order.length - 1; index > 0; index -= 1) {
    const other = Math.floor(Math.random() * (index + 1));
    [order[index], order[other]] = [order[other], order[index]];
  }
  return order;
}

/** ✍️ A valid answer to one presented task and the draft a learner shares of it. */
function answerOf(task: SheetTask): { readonly answer: Answer; readonly draft: ThinkingAnswer } {
  const items = task.items.map((item) => item.id);
  if (task.kind === "sorting") {
    const answer = { kind: "sorting", order: shuffled(items) } as const;
    return { answer, draft: answer };
  }
  if (task.kind === "classification") {
    const answer = { kind: "classification", assignments: Object.fromEntries(items.map((item) => [item, task.categories[Math.floor(Math.random() * task.categories.length)].id])) } as const;
    return { answer, draft: answer };
  }
  const cards = task.dimensions.map((dimension) => ({ dimension, order: shuffled(dimension.cards.map((_, card) => card)) }));
  return {
    answer: { kind: "matching", assignments: Object.fromEntries(cards.map(({ dimension, order }) => [dimension.id, Object.fromEntries(items.map((item, index) => [item, order[index]]))])) },
    draft: { kind: "matching", values: Object.fromEntries(cards.map(({ dimension, order }) => [dimension.id, Object.fromEntries(items.map((item, index) => [item, dimension.cards[order[index]]]))])) },
  };
}

/** 🧑‍🎓️ One learner's lecture: enrol, look around, play `plays` quizzes, leave. */
async function attend(hall: Hall, index: number, plays: number): Promise<void> {
  const learner = newId();
  const tag = learnerTag(learner);
  const client = new ProctorClient(() => transport(hall.base, HALL, (status) => (status === 429 || status === 503 ? (hall.measured().throttled += 1) : undefined)), hall.tenant);
  /** ⏱️ One operation as the quiz client performs it: a call that got no answer is made again after the client's own
   * backoff, and the time until it is answered — the waits included — is what the learner waited. */
  const timed = async <T>(operation: string, call: () => Promise<T>): Promise<T | undefined> => {
    const started = performance.now();
    let attempts = 0;
    try {
      const value = await retryTransient(
        () => {
          attempts += 1;
          return call();
        },
        RETRY_TIMING,
        AbortSignal.timeout(GIVE_UP_MS),
      );
      record(hall.measured(), operation, performance.now() - started);
      return value;
    } catch (error) {
      fail(hall.measured(), `${operation}: ${error instanceof Error ? `${error.name}: ${error.message}` : String(error)}`);
      return undefined;
    } finally {
      hall.measured().requests += 1;
      hall.measured().resent += Math.max(0, attempts - 1);
    }
  };
  const command = async (sent: Command): Promise<boolean> => {
    const verdict: CommandVerdict | undefined = await timed(sent.type, () => client.command(sent));
    if (verdict !== undefined && verdict.kind !== "accepted") fail(hall.measured(), `${sent.type}: ${verdict.kind === "rejected" ? verdict.rejection : verdict.detail}`);
    return verdict?.kind === "accepted";
  };
  const room = (scope: string, surface: string): Room => {
    const joined = new Room(hall.base, HALL, scope, surface, hall.measured);
    hall.rooms.add(joined);
    return joined;
  };
  const leave = (joined: Room): void => {
    hall.rooms.delete(joined);
    joined.leave();
  };
  /** 🖱️ Spend `milliseconds` at `anchor`, moving the pointer some of the time. */
  const dwell = async (milliseconds: number, page: Room, anchor: string): Promise<void> => {
    const until = performance.now() + milliseconds;
    while (performance.now() < until) {
      if (Math.random() < CURSOR.share) page.share({ tag, cursor: { anchor, x: Math.round(Math.random() * 1000) / 1000, y: Math.round(Math.random() * 1000) / 1000 }, focus: anchor });
      await pause(Math.min(CURSOR.intervalMs, Math.max(0, until - performance.now())));
    }
  };

  await pause(Math.random() * REAL.arrival * hall.scale);
  const identity: IdentityClaim = index % 3 === 0 ? { kind: "anonymous" } : { kind: index % 3 === 1 ? "pseudonym" : "name", handle: `Hall ${index} ${learner.slice(0, 10)}` };
  if (!(await command({ type: "identify-learner", id: newId(), learner, identity }))) return;
  const here = (place: PresenceState["place"]): PresenceState => ({ tag, identity: identity.kind === "anonymous" ? identity : { kind: identity.kind, handle: identity.handle }, place, active: true });

  const roster = room(rosterScope(hall.tenant), "home");
  let page = room(`${hall.tenant}/home`, "home");
  await Promise.all([roster.join(), page.join()]);
  roster.share(here({ screen: "home" }));
  page.watch([`${hall.tenant}/introduction`, `${hall.tenant}/leaderboard`, `${hall.tenant}/badges`, ...hall.quizzes.flatMap((quiz) => [`${hall.tenant}/quiz/${quiz}`, thinkingScope(hall.tenant, quiz)])].slice(0, 16));
  await Promise.all([timed("query catalog", () => client.catalog(learner)), timed("query learner", () => client.learner(learner)), timed("query leaderboard", () => client.leaderboard({ period: "all-time" }, learner))]);

  let attending = true;
  const polling = (async () => {
    while (attending) {
      await pause(REAL.poll * hall.scale * between(0.8, 1.2));
      if (attending) await timed("query leaderboard", () => client.leaderboard({ period: "all-time" }, learner));
    }
  })();

  for (const quiz of [hall.quizzes[0], ...shuffled(hall.quizzes.slice(1))].slice(0, plays)) {
    await dwell(REAL.home * hall.scale * between(0.5, 1.5), page, `card:${quiz}`);
    const run = newId();
    if (!(await command({ type: "start-run", id: newId(), learner, run, quiz }))) break;
    const view: RunView | undefined = await timed("query run", () => client.run(run, learner));
    if (view === undefined) break;
    leave(page);
    page = room(`${hall.tenant}/quiz/${quiz}`, "run");
    const thinking = room(thinkingScope(hall.tenant, quiz), "run");
    await Promise.all([page.join(), thinking.join()]);
    const drafts: Record<string, ThinkingAnswer> = {};
    for (const task of view.sheet.tasks) {
      roster.share(here({ screen: "run", quiz, task: task.id }));
      for (let revision = 0, revisions = 2 + Math.floor(Math.random() * 3); revision < revisions; revision += 1) {
        await dwell(between(REAL.revise[0], REAL.revise[1]) * hall.scale, page, `task:${task.id}`);
        const { answer, draft } = answerOf(task);
        drafts[task.id] = draft;
        thinking.share({ tag, answers: { ...drafts } });
        await command({ type: "record-answer", id: newId(), learner, run, task: task.id, answer });
      }
    }
    await command({ type: "submit-run", id: newId(), learner, run });
    roster.share(here({ screen: "results", quiz }));
    await Promise.all([timed("query run", () => client.run(run, learner)), timed("query crowd", () => client.crowd(quiz, learner)), timed("query leaderboard", () => client.leaderboard({ period: "all-time" }, learner)), timed("query learner", () => client.learner(learner))]);
    await dwell(REAL.read * hall.scale, page, "card:results");
    leave(thinking);
    leave(page);
    page = room(`${hall.tenant}/home`, "home");
    await page.join();
    roster.share(here({ screen: "home" }));
  }
  attending = false;
  await polling;
  leave(page);
  leave(roster);
}

/** 📶️ The Wi-Fi drop: every open socket of the hall drops and rejoins within the (compressed) storm window. */
async function storm(hall: Hall): Promise<number> {
  const rooms = [...hall.rooms];
  await Promise.all(rooms.map((joined) => joined.reconnect(Math.random() * REAL.storm).then(() => undefined, () => undefined)));
  return rooms.length;
}

/** 🏫️ What one thread of the hall is told: the learners of one lecture it plays. */
interface HallOrders {
  readonly role: "hall";
  readonly base: string;
  readonly tenant: string;
  readonly quizzes: readonly string[];
  readonly scale: number;
  readonly learners: readonly number[];
}

/** 🧾️ What one thread of the hall measured. */
interface HallSlice {
  readonly latencies: Record<string, number[]>;
  readonly errors: string[];
  readonly throttled: number;
  readonly requests: number;
  readonly resent: number;
  readonly sockets: Measurements["sockets"];
  readonly reconnected: number;
  /** 🐌️ How late this thread's own timers fired, in milliseconds, probed every {@link LAG_PROBE_MS}. */
  readonly lag: number[];
}

/** 🏫️ One thread's part of a lecture: its learners attend, and when `dropped` resolves its sockets drop once. */
async function lecture(orders: HallOrders, dropped: Promise<void>): Promise<HallSlice> {
  const measured = measurements();
  const hall: Hall = { base: orders.base, tenant: orders.tenant, quizzes: orders.quizzes, scale: orders.scale, measured: () => measured, rooms: new Set() };
  const lag: number[] = [];
  let due = performance.now() + LAG_PROBE_MS;
  const probe = setInterval(() => {
    const now = performance.now();
    lag.push(Math.max(0, now - due));
    due = now + LAG_PROBE_MS;
  }, LAG_PROBE_MS);
  let attending = true;
  let reconnected = 0;
  let storming: Promise<void> = Promise.resolve();
  void dropped.then(() => {
    if (attending) storming = storm(hall).then((count) => void (reconnected = count));
  });
  await Promise.all(orders.learners.map((index) => attend(hall, index, 2)));
  attending = false;
  await storming;
  clearInterval(probe);
  return { latencies: Object.fromEntries(measured.latencies), errors: measured.errors, throttled: measured.throttled, requests: measured.requests, resent: measured.resent, sockets: measured.sockets, reconnected, lag };
}
//#endregion 🧑‍🎓️Learner

//#region 😈️Script
/** 😈️ What the abusive script is told. */
interface ScriptOrders {
  readonly role: "script";
  readonly base: string;
  readonly tenant: string;
  readonly quiz: string;
  readonly bodyLimit: number;
  readonly socketsPerAddress: number;
}

/** 🧾️ What the abusive script saw: answers by status per attack. */
export interface ScriptReport {
  /** ⏱️ How long the script ran: what its allowances are computed over. */
  readonly seconds: number;
  /** ✍️ Registrations of new learners, anonymous and under handles, as fast as the script can ask: answers by status. */
  readonly commands: Record<string, number>;
  /** 💰️ What became of those registrations: `registered`, `refused` (a `429` naming the sign-up allowance), `throttled`
   * (a `429` of the command allowance), `rejected <detail>` or `unanswered <status>`. */
  readonly signUps: Record<string, number>;
  /** 🧨️ Commands addressed to ids no actor can have. */
  readonly malformed: Record<string, number>;
  readonly queries: Record<string, number>;
  /** 🐘️ Bodies far past the limit: `413`, `429`, or `0` when the proctor cut the connection while the body was still on its way. */
  readonly oversizedBody: Record<string, number>;
  readonly sockets: { readonly attempted: number; readonly opened: number; readonly refused: Record<string, number> };
  readonly oversizedFrame: { readonly sent: number; readonly closed: number };
}

function tally(into: Record<string, number>, key: string | number): void {
  into[String(key)] = (into[String(key)] ?? 0) + 1;
}

/** 🧵️ One connection the script keeps open and posts on, one request after the other — how a script that wants the most
 * of its address sends, and how its requests arrive behind a proxy that keeps its connections to the proctor. A raw
 * socket rather than `fetch`, whose connection pool opens a new connection for every few requests once many run at
 * once: on one machine that empties the pool of client ports the hall connects from. */
class Wire {
  private socket: Socket | undefined;
  private received = Buffer.alloc(0);
  private answered: ((status: number, body: string) => void) | undefined;

  constructor(private readonly base: string) {}

  /** 📮️ Post `body` to `path` as the proxy would forward it for the script; the status, or `0` when the connection
   * was cut before the answer was whole. */
  async post(path: string, body: string): Promise<number> {
    return (await this.asked(path, body)).status;
  }

  /** 📬️ Post like {@link post} and answer the body beside the status. */
  asked(path: string, body: string): Promise<{ readonly status: number; readonly body: string }> {
    return new Promise((answered) => {
      this.answered = (status, body) => answered({ status, body });
      const { hostname, port, host } = new URL(this.base);
      const head = [`POST ${path} HTTP/1.1`, `Host: ${host}`, "Content-Type: application/json", `Content-Length: ${Buffer.byteLength(body)}`, ...Object.entries(forwarded(SCRIPT)).map(([name, value]) => `${name}: ${value}`), "", ""].join("\r\n");
      if (this.socket === undefined) {
        const socket = connect({ host: hostname, port: Number(port), noDelay: true });
        this.socket = socket;
        this.received = Buffer.alloc(0);
        socket.on("data", (chunk) => this.take(socket, chunk));
        socket.on("error", () => undefined);
        socket.on("close", () => this.settle(socket, 0, "", false));
      }
      this.socket.write(head + body);
    });
  }

  /** 🚪️ Close the connection. */
  close(): void {
    this.socket?.destroy();
    this.socket = undefined;
  }

  private take(socket: Socket, chunk: Buffer): void {
    if (socket !== this.socket) return;
    this.received = Buffer.concat([this.received, chunk]);
    const end = this.received.indexOf("\r\n\r\n");
    if (end < 0) return;
    const head = this.received.subarray(0, end).toString("latin1").toLowerCase();
    const length = Number(/\r\ncontent-length: (\d+)/u.exec(head)?.[1] ?? 0);
    if (this.received.length < end + 4 + length) return;
    const body = this.received.subarray(end + 4, end + 4 + length).toString("utf8");
    this.received = this.received.subarray(end + 4 + length);
    this.settle(socket, Number(head.slice(9, 12)), body, !head.includes("\r\nconnection: close"));
  }

  private settle(socket: Socket, status: number, body: string, keep: boolean): void {
    if (socket !== this.socket) return;
    if (!keep) {
      this.socket = undefined;
      socket.destroy();
    }
    const answered = this.answered;
    this.answered = undefined;
    answered?.(status, body);
  }
}

/** 🧧️ What became of one posted registration, from its answer: `registered` (accepted with its event), `refused` (a
 * `429` naming the sign-up allowance), `throttled` (any other `429`), `rejected <detail>` (decided against, or the
 * replay of one), `unanswered <status>` otherwise. */
export function signUpFate(status: number, body: string): string {
  let answer: { status?: string; events?: unknown[]; reason?: { detail?: string }; allowance?: string } = {};
  try {
    answer = JSON.parse(body) as typeof answer;
  } catch {}
  if (status === 429) return answer.allowance === SIGN_UP ? "refused" : "throttled";
  if (status !== 200) return `unanswered ${status}`;
  return answer.status === "accepted" && (answer.events?.length ?? 0) > 0 ? "registered" : `rejected ${answer.reason?.detail ?? answer.status ?? "?"}`;
}

/** 😈️ Run every attack beside the hall until told to stop; report what was answered. */
async function script(orders: ScriptOrders, stopped: () => boolean): Promise<ScriptReport> {
  const started = performance.now();
  const report: Omit<ScriptReport, "seconds"> = { commands: {}, signUps: {}, malformed: {}, queries: {}, oversizedBody: {}, sockets: { attempted: 0, opened: 0, refused: {} }, oversizedFrame: { sent: 0, closed: 0 } };
  const wires: Wire[] = [];
  const wire = (): Wire => {
    const opened = new Wire(orders.base);
    wires.push(opened);
    return opened;
  };
  const envelope = (command: Command): string => JSON.stringify(encodeCommandEnvelope(commandEnvelope(command, orders.tenant, Date.now())));
  const flood = async (claiming: boolean): Promise<void> => {
    const posting = wire();
    while (!stopped()) {
      const learner = newId();
      const answer = await posting.asked("/commands", envelope({ type: "identify-learner", id: newId(), learner, identity: claiming ? { kind: "pseudonym", handle: `Script ${learner.slice(0, 16)}` } : { kind: "anonymous" } }));
      tally(report.commands, answer.status);
      tally(report.signUps, signUpFate(answer.status, answer.body));
    }
  };
  const malformed = async (): Promise<void> => {
    const posting = wire();
    while (!stopped()) {
      const sent = JSON.parse(envelope({ type: "start-run", id: newId(), learner: newId(), run: newId(), quiz: orders.quiz })) as { target: { id: string } };
      sent.target.id = `no-such-learner-${Math.random()}`;
      tally(report.malformed, await posting.post("/commands", JSON.stringify(sent)));
    }
  };
  const queries = async (): Promise<void> => {
    const posting = wire();
    const query = JSON.stringify({ queryId: newId(), kind: "quiz.leaderboard", version: 1, scope: orders.tenant, principal: { kind: "anonymous" }, arguments: Array.from(new TextEncoder().encode(JSON.stringify({ type: "leaderboard", period: "all-time" }))), consistency: { kind: "authority" }, cursor: null });
    while (!stopped()) tally(report.queries, await posting.post("/queries", query));
  };
  const oversized = async (): Promise<void> => {
    const posting = wire();
    const body = JSON.stringify({ padding: "x".repeat(orders.bodyLimit * 64) });
    while (!stopped()) {
      tally(report.oversizedBody, await posting.post("/commands", body));
      await pause(50);
    }
  };
  const open = (scope: string): Promise<WebSocket | number> =>
    new Promise((settled) => {
      const socket = new WebSocket(presenceSocketUrl(orders.base, scope, "home"), { headers: forwarded(SCRIPT), protocols: [PRESENCE_PROTOCOL] } as unknown as string[]);
      socket.addEventListener("open", () => settled(socket));
      socket.addEventListener("close", (event) => settled(event.code));
      socket.addEventListener("error", () => settled(0));
    });
  const held: WebSocket[] = [];
  const sockets = async (): Promise<void> => {
    while (!stopped() && report.sockets.attempted < orders.socketsPerAddress + 512) {
      const wave = await Promise.all(Array.from({ length: 64 }, () => open(orders.tenant)));
      for (const opened of wave) {
        (report.sockets as { attempted: number }).attempted += 1;
        if (typeof opened === "number") tally(report.sockets.refused, opened);
        else {
          (report.sockets as { opened: number }).opened += 1;
          held.push(opened);
        }
      }
    }
  };
  const frames = async (): Promise<void> => {
    while (!stopped()) {
      const opened = await open(`${orders.tenant}/home`);
      if (typeof opened !== "number") {
        const closed = new Promise<void>((done) => opened.addEventListener("close", () => done()));
        opened.send(JSON.stringify({ type: "state", state: { padding: "x".repeat(64 * 1024) } }));
        (report.oversizedFrame as { sent: number }).sent += 1;
        if ((await Promise.race([closed.then(() => true), pause(5_000).then(() => false)])) === true) (report.oversizedFrame as { closed: number }).closed += 1;
        else opened.close();
      }
      await pause(250);
    }
  };
  await Promise.all([...Array.from({ length: 24 }, (_, wire) => flood(wire % 2 === 1)), ...Array.from({ length: 8 }, malformed), ...Array.from({ length: 64 }, queries), oversized(), sockets(), frames()]);
  const seconds = (performance.now() - started) / 1000;
  for (const socket of held) socket.close();
  for (const opened of wires) opened.close();
  return { ...report, seconds };
}

//#endregion 😈️Script

//#region 🖥️Proctor
/** 🔌️ A free loopback port no dev server of this repository uses. */
async function freePort(): Promise<number> {
  for (;;) {
    const port = await new Promise<number>((found, refused) => {
      const probe = createServer();
      probe.once("error", refused);
      probe.listen(0, "127.0.0.1", () => {
        const address = probe.address();
        probe.close(() => found(typeof address === "object" && address !== null ? address.port : 0));
      });
    });
    if (port !== 0 && !RESERVED_PORTS.has(port)) return port;
  }
}

/** 🖨️ What `command` prints, without holding this thread while it runs (a held thread would be counted as latency
 * of the hall); empty when it cannot run. */
function printed(command: string, args: readonly string[]): Promise<string> {
  return new Promise((done) => execFile(command, [...args], { encoding: "utf8", windowsHide: true }, (_error, stdout) => done(stdout ?? "")));
}

/** 🧠️ The resident memory of process `pid` in MiB, or `undefined` where this platform cannot say. */
export async function residentMebibytes(pid: number): Promise<number | undefined> {
  if (process.platform === "linux") {
    const status = existsSync(`/proc/${pid}/status`) ? readFileSync(`/proc/${pid}/status`, "utf8") : "";
    const kibibytes = /^VmRSS:\s+(\d+) kB$/mu.exec(status)?.[1];
    return kibibytes === undefined ? undefined : Number(kibibytes) / 1024;
  }
  if (process.platform === "win32") {
    const listed = await printed("tasklist", ["/FI", `PID eq ${pid}`, "/FO", "CSV", "/NH"]);
    const kibibytes = /"([\d.,'   ]+) K"\s*$/mu.exec(listed.trim())?.[1]?.replace(/\D/gu, "");
    return kibibytes === undefined || kibibytes === "" ? undefined : Number(kibibytes) / 1024;
  }
  const kibibytes = (await printed("ps", ["-o", "rss=", "-p", String(pid)])).trim();
  return kibibytes === "" ? undefined : Number(kibibytes) / 1024;
}

/** ⚙️ The processor time process `pid` has used so far in seconds, or `undefined` where this platform cannot say. */
export async function processorSeconds(pid: number): Promise<number | undefined> {
  if (process.platform === "win32") {
    const seconds = (await printed("powershell", ["-NoProfile", "-NonInteractive", "-Command", `(Get-Process -Id ${pid}).TotalProcessorTime.TotalMilliseconds`])).trim().replace(",", ".");
    return seconds === "" || Number.isNaN(Number(seconds)) ? undefined : Number(seconds) / 1000;
  }
  const time = (await printed("ps", ["-o", "time=", "-p", String(pid)])).trim();
  const parts = /^(?:(\d+)-)?(?:(\d+):)?(\d+):(\d+(?:\.\d+)?)$/u.exec(time);
  return parts === null ? undefined : Number(parts[1] ?? 0) * 86_400 + Number(parts[2] ?? 0) * 3_600 + Number(parts[3]) * 60 + Number(parts[4]);
}

/** 🪪️ The roster of a stopped proctor as an operator counts it: what `prune` would remove at once (every registration
 * nobody submitted a run under) and what would stay. */
export interface Roster {
  /** 🧹️ Registrations nobody played under: what a prune removes. */
  readonly idle: number;
  /** 🛟️ Learners that submitted a run, and the registrations they hold. */
  readonly played: number;
  readonly registrations: number;
}

/** 🪣️ Count the roster of the stopped proctor of `env` with its own `prune --older-than 0s --dry-run`, which changes
 * nothing; `undefined` when the count cannot be read. */
async function rosterOf(executable: string, repoRoot: string, env: NodeJS.ProcessEnv, log: string): Promise<Roster | undefined> {
  await launchProctor(executable, repoRoot, ["prune", "--older-than", "0s", "--dry-run"], env, log).exited;
  const counted = existsSync(log) ? readFileSync(log, "utf8") : "";
  const idle = /^would prune (\d+) registrations /mu.exec(counted)?.[1];
  const staying = /^ {2}staying: (\d+) learners \((\d+) of them submitted a run\), \d+ handles, (\d+) registrations$/mu.exec(counted);
  if (idle === undefined || staying === null || Number(staying[1]) !== Number(staying[2])) return undefined;
  return { idle: Number(idle), played: Number(staying[2]), registrations: Number(staying[3]) };
}

async function ready(base: string, timeoutMs: number): Promise<boolean> {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    try {
      const answer = await fetch(`${base}/instance`, { headers: { "x-forwarded-proto": "https" }, signal: AbortSignal.timeout(2_000) });
      if (answer.ok) return true;
    } catch {}
    await pause(200);
  }
  return false;
}
//#endregion 🖥️Proctor

//#region 🏋️Gate
/** 🧾️ What one phase of the hall measured. */
export interface PhaseReport {
  readonly seconds: number;
  readonly operations: readonly LatencyRow[];
  readonly errors: readonly string[];
  readonly throttled: number;
  /** 📮️ Commands and queries the hall made. */
  readonly requests: number;
  /** 🔁️ How many more times the hall had to send a request that got no answer (no connection, a connection cut). */
  readonly resent: number;
  readonly sockets: Measurements["sockets"];
  readonly reconnected: number;
  /** ⚙️ Processor seconds the proctor spent in this phase; divided by `seconds` it is the cores it kept busy. */
  readonly proctorProcessorSeconds: number | undefined;
  /** 🧪️ Processor seconds this load generator's own thread pool spent: near `seconds` it, not the proctor, was the limit. */
  readonly clientProcessorSeconds: number;
  /** 🐌️ How late this load generator's own timers fired, in milliseconds: every latency of the hall includes it. */
  readonly generatorLag: { readonly p50: number; readonly p95: number; readonly p99: number; readonly max: number };
}

/** 🧾️ The whole report of one capacity run; `beside` and `script` are absent from a run of the hall alone. */
export interface CapacityReport {
  readonly options: CapacityOptions;
  readonly alone: PhaseReport;
  readonly beside?: PhaseReport;
  readonly script?: ScriptReport;
  /** 🧺️ The roster at the end and the cap it is held against. */
  readonly roster: (Roster & { readonly cap: number }) | undefined;
  readonly memoryMiB: { readonly idle: number | undefined; readonly peak: number | undefined; readonly end: number | undefined };
  readonly healthyAtEnd: boolean;
  readonly violations: readonly string[];
}

/** 🏫️ Play one lecture and measure it: the hall's latencies and what the proctor and this process spent on it. The
 * learners are dealt to {@link HALL_THREADS} threads; after twice the arrival window every thread drops its sockets. */
async function phase(hall: Omit<HallOrders, "role" | "learners">, options: CapacityOptions, proctor: number | undefined): Promise<PhaseReport> {
  const spentBefore = proctor === undefined ? undefined : await processorSeconds(proctor);
  const [started, usedBefore] = [performance.now(), process.cpuUsage()];
  const threads = Math.min(HALL_THREADS, options.learners);
  const workers = Array.from({ length: threads }, (_, thread) => {
    const orders: HallOrders = { ...hall, role: "hall", learners: Array.from({ length: options.learners }, (_, index) => index).filter((index) => index % threads === thread) };
    return new Worker(new URL(import.meta.url), { workerData: orders });
  });
  const played = workers.map(
    (worker) =>
      new Promise<HallSlice>((reported, failed) => {
        worker.once("message", reported);
        worker.once("error", failed);
      }),
  );
  let playing = true;
  const dropping = pause(REAL.arrival * hall.scale * 2).then(() => {
    if (playing) for (const worker of workers) worker.postMessage("drop");
  });
  const slices = await Promise.all(played).finally(() => (playing = false));
  await dropping;
  await Promise.all(workers.map((worker) => worker.terminate()));
  const seconds = (performance.now() - started) / 1000;
  const used = process.cpuUsage(usedBefore);
  const spentAfter = proctor === undefined ? undefined : await processorSeconds(proctor);
  const measured = measurements();
  const lag: number[] = [];
  let reconnected = 0;
  for (const slice of slices) {
    for (const [operation, samples] of Object.entries(slice.latencies)) measured.latencies.set(operation, [...(measured.latencies.get(operation) ?? []), ...samples]);
    measured.errors.push(...slice.errors);
    measured.throttled += slice.throttled;
    measured.requests += slice.requests;
    measured.resent += slice.resent;
    for (const key of Object.keys(measured.sockets) as (keyof Measurements["sockets"])[]) measured.sockets[key] += slice.sockets[key];
    for (const late of slice.lag) lag.push(late);
    reconnected += slice.reconnected;
  }
  return {
    seconds: Math.round(seconds * 10) / 10,
    operations: rows(measured),
    errors: measured.errors,
    throttled: measured.throttled,
    requests: measured.requests,
    resent: measured.resent,
    sockets: measured.sockets,
    reconnected,
    proctorProcessorSeconds: spentBefore === undefined || spentAfter === undefined ? undefined : Math.round((spentAfter - spentBefore) * 10) / 10,
    clientProcessorSeconds: Math.round((used.user + used.system) / 100_000) / 10,
    generatorLag: { p50: percentile(lag, 50), p95: percentile(lag, 95), p99: percentile(lag, 99), max: percentile(lag, 100) },
  };
}

/** ⚖️ Everything a phase of the hall violates: an error, a refusal or a percentile over `budget`. */
function hallViolations(name: string, phase: PhaseReport, budget: { readonly p95: number; readonly p99: number }): string[] {
  const violations: string[] = [];
  if (phase.errors.length > 0) violations.push(`the hall ${name} saw ${phase.errors.length} error(s), first: ${phase.errors[0]}`);
  if (phase.throttled > 0) violations.push(`the hall ${name} was refused ${phase.throttled} time(s) with 429/503`);
  if (phase.resent > phase.requests * RESENT_SHARE) violations.push(`the hall ${name} had to send ${phase.resent} of ${phase.requests} requests again, more than ${RESENT_SHARE * 100} %`);
  if (phase.sockets.rejoined > phase.sockets.opened * RESENT_SHARE) violations.push(`the hall ${name} had to make ${phase.sockets.rejoined} of ${phase.sockets.opened} socket joins again, more than ${RESENT_SHARE * 100} %`);
  const slow: string[] = [];
  for (const row of phase.operations.filter((row) => row.operation !== "socket join")) {
    if (row.p95 > budget.p95) slow.push(`${row.operation} ${name}: p95 ${row.p95} ms over the budget of ${budget.p95} ms`);
    if (row.p99 > budget.p99) slow.push(`${row.operation} ${name}: p99 ${row.p99} ms over the budget of ${budget.p99} ms`);
  }
  const late = `the load generator itself ran ${phase.generatorLag.p99} ms late (p99) ${name}`;
  if (phase.generatorLag.p99 > GENERATOR_STALL_MS) violations.push(`${late}, more than ${GENERATOR_STALL_MS} ms: this machine was too busy to play the hall at all, run the gate again when it is quiet`);
  else if (slow.length > 0 && phase.generatorLag.p99 > GENERATOR_LAG_MS) violations.push(`${slow.length} latencies ${name} are over the budget, but ${late}, more than ${GENERATOR_LAG_MS} ms: this machine was too busy to measure the hall, run the gate again when it is quiet (first: ${slow[0]})`);
  else violations.push(...slow);
  return violations;
}

/** 💳️ The most registrations one address may make in `seconds`: its burst and what flowed in meanwhile. */
function signUpAllowance(seconds: number): number {
  return LIMITS.signUpsBurst + Math.ceil((LIMITS.signUpsPerHour * seconds) / 3600);
}

/** 🪤️ Everything about the roster at the end that says the cap of learners was within the script's reach: learners of
 * the hall that are missing or never played, registrations beside the hall's that the script's sign-up allowance does
 * not explain, a roster that is not far below the cap. */
function rosterViolations(roster: Roster | undefined, hall: number, script: ScriptReport | undefined): string[] {
  if (roster === undefined) return ["the roster of the stopped proctor could not be counted (prune --dry-run)"];
  const violations: string[] = [];
  if (roster.played !== hall || roster.registrations !== hall) violations.push(`the hall registered ${hall} learners who all submitted a run, the roster holds ${roster.played} learners who did, with ${roster.registrations} registrations`);
  const [registered, unanswered] = script === undefined ? [0, 0] : [script.signUps.registered ?? 0, Object.entries(script.signUps).filter(([name]) => name.startsWith("unanswered")).reduce((sum, [, count]) => sum + count, 0)];
  if (roster.idle < registered || roster.idle > registered + unanswered) violations.push(`a prune would remove ${roster.idle} registrations nobody played under, the script registered ${registered} (and ${unanswered} of its sign-ups got no answer)`);
  if (script !== undefined && roster.idle > signUpAllowance(script.seconds)) violations.push(`${roster.idle} registrations nobody played under are more than the sign-up allowance of one address over ${script.seconds.toFixed(1)} s, ${signUpAllowance(script.seconds)}`);
  if (roster.idle + roster.registrations > ROSTER_CAP * ROSTER_SHARE) violations.push(`the roster holds ${roster.idle + roster.registrations} registrations, more than ${ROSTER_SHARE * 100} % of the cap of ${ROSTER_CAP}`);
  return violations;
}

/** ⚖️ Everything the script got away with: a flood that was never refused or was served more than its address's
 * allowance over the time it ran — of commands, of queries, of sign-ups —, a body or frame past its limit that was
 * taken, more sockets than its address may hold. */
function scriptViolations(script: ScriptReport): string[] {
  const violations: string[] = [];
  const sockets = LIMITS.socketsPerAddress;
  const answered = (answers: Record<string, number>, status: number): number => answers[String(status)] ?? 0;
  const allowance = (perSecond: number, burst: number): number => Math.ceil(perSecond * script.seconds + burst);
  const fate = (name: string): number => script.signUps[name] ?? 0;
  const [commands, queries] = [answered(script.commands, 200) + fate("refused") + answered(script.malformed, 200), answered(script.queries, 200)];
  if (commands > allowance(LIMITS.commandsPerSecond, LIMITS.commandsBurst)) violations.push(`the script's commands passed its address's command allowance ${commands} times in ${script.seconds.toFixed(1)} s, more than the allowance of ${allowance(LIMITS.commandsPerSecond, LIMITS.commandsBurst)}`);
  if (queries > allowance(LIMITS.queriesPerSecond, LIMITS.queriesBurst)) violations.push(`the script was served ${queries} queries in ${script.seconds.toFixed(1)} s, more than its address's allowance of ${allowance(LIMITS.queriesPerSecond, LIMITS.queriesBurst)}`);
  const signUps = signUpAllowance(script.seconds);
  if (fate("registered") > signUps) violations.push(`the script registered ${fate("registered")} learners in ${script.seconds.toFixed(1)} s, more than its address's sign-up allowance of ${signUps}`);
  if (fate("registered") < LIMITS.signUpsBurst) violations.push(`the script registered ${fate("registered")} learners, fewer than the ${LIMITS.signUpsBurst} its address may register at once: something other than the sign-up allowance stopped it (${JSON.stringify(script.signUps)})`);
  if (fate("refused") === 0) violations.push("the script's sign-ups were never refused in the name of the sign-up allowance");
  const filled = Object.keys(script.signUps).filter((name) => name.includes("roster-full"));
  if (filled.length > 0) violations.push(`the script reached the cap of learners: ${JSON.stringify(script.signUps)}`);
  if (answered(script.commands, 429) === 0) violations.push("the command flood was never answered 429");
  if (answered(script.queries, 429) === 0) violations.push("the query flood was never answered 429");
  if (answered(script.malformed, 429) === 0) violations.push("the malformed-command flood was never answered 429");
  if (Object.entries(script.oversizedBody).some(([status]) => status !== "413" && status !== "429" && status !== "0")) violations.push(`an oversized body was answered ${JSON.stringify(script.oversizedBody)}, expected 413, 429 or the connection cut while it was still being sent`);
  if (answered(script.oversizedBody, 413) === 0) violations.push("no oversized body was answered 413");
  if (script.sockets.opened > sockets) violations.push(`the script holds ${script.sockets.opened} sockets, over the cap of ${sockets} per address`);
  if (script.sockets.attempted > sockets && script.sockets.opened === script.sockets.attempted) violations.push("the socket flood was never refused");
  if (script.oversizedFrame.sent === 0 || script.oversizedFrame.closed !== script.oversizedFrame.sent) violations.push(`an oversized frame did not close its socket (${script.oversizedFrame.closed} of ${script.oversizedFrame.sent})`);
  return violations;
}

function table(title: string, phase: PhaseReport): string {
  const head = `${title} — ${phase.seconds} s, ${phase.sockets.opened} socket joins (${phase.reconnected} reconnected in the drop, ${phase.sockets.rejoined} made again), ${phase.sockets.frames} frames / ${(phase.sockets.bytes / 1_048_576).toFixed(1)} MiB received, ${phase.errors.length} errors, ${phase.throttled} refusals, ${phase.resent} of ${phase.requests} requests sent again; processor seconds: proctor ${phase.proctorProcessorSeconds ?? "?"}, load generator ${phase.clientProcessorSeconds}; load generator late by p50 ${phase.generatorLag.p50} / p95 ${phase.generatorLag.p95} / p99 ${phase.generatorLag.p99} / max ${phase.generatorLag.max} ms`;
  const lines = phase.operations.map((row) => `  ${row.operation.padEnd(18)} ${String(row.count).padStart(6)}  p50 ${String(row.p50).padStart(7)}  p95 ${String(row.p95).padStart(7)}  p99 ${String(row.p99).padStart(7)}  max ${String(row.max).padStart(7)}  ms`);
  return [head, ...lines].join("\n");
}

/** 🏋️ Build the release proctor, serve it on a throw-away port and data directory with its default cap of learners, play
 * the hall alone and beside the script, stop the proctor, count its roster and return the report; `violations` is empty
 * exactly when the gate holds. */
export async function runCapacity(repoRoot: string, options: CapacityOptions = CAPACITY_DEFAULTS): Promise<CapacityReport> {
  const executable = options.executable === undefined ? await buildProctor(repoRoot, process.env, undefined, "release") : resolve(options.executable);
  const data = mkdtempSync(join(tmpdir(), "proctor-capacity-"));
  const port = await freePort();
  const base = `http://127.0.0.1:${port}`;
  const catalogPath = resolve(repoRoot, ...PROCTOR_DEV_CATALOG);
  const catalog = JSON.parse(readFileSync(catalogPath, "utf8")) as { id: string; quizzes: string[] };
  const quizzes = catalog.quizzes.map((path) => (JSON.parse(readFileSync(resolve(dirname(catalogPath), path), "utf8")) as { id: string }).id);
  const { PROCTOR_MAX_LEARNERS: _default, ...inherited } = process.env;
  const env: NodeJS.ProcessEnv = { ...inherited, PROCTOR_PORT: String(port), PROCTOR_BIND: "127.0.0.1", PROCTOR_MODE: "production", PROCTOR_ALLOWED_ORIGINS: SITE, PROCTOR_TRUSTED_FORWARDING: "proxy", PROCTOR_DATA: data, PROCTOR_CATALOG: catalogPath, PROCTOR_LIMIT_COMMANDS_PER_SECOND: String(LIMITS.commandsPerSecond), PROCTOR_LIMIT_COMMANDS_BURST: String(LIMITS.commandsBurst), PROCTOR_LIMIT_QUERIES_PER_SECOND: String(LIMITS.queriesPerSecond), PROCTOR_LIMIT_QUERIES_BURST: String(LIMITS.queriesBurst), PROCTOR_LIMIT_SIGNUPS_PER_HOUR: String(LIMITS.signUpsPerHour), PROCTOR_LIMIT_SIGNUPS_BURST: String(LIMITS.signUpsBurst), PROCTOR_LIMIT_SOCKETS_PER_ADDRESS: String(LIMITS.socketsPerAddress), PROCTOR_LIMIT_BODY_BYTES: String(LIMITS.bodyBytes) };
  let proctor: ProctorProcess | undefined;
  let peak: number | undefined;
  const sample = async (): Promise<number | undefined> => {
    const now = proctor?.pid === undefined ? undefined : await residentMebibytes(proctor.pid);
    if (now !== undefined) peak = Math.max(peak ?? 0, now);
    return now;
  };
  try {
    proctor = launchProctor(executable, repoRoot, ["serve"], env, join(data, "proctor.log"));
    if (!(await ready(base, 120_000))) throw new Error(`the capacity proctor did not come up on ${base}:\n${readFileSync(join(data, "proctor.log"), "utf8").slice(-2000)}`);
    const instance = (await (await fetch(`${base}/instance`, { headers: { "x-forwarded-proto": "https" } })).json()) as { id: string };
    console.log(`[INFO] capacity: ${instance.id} on ${base}, ${options.learners} learners from ${HALL}, think time ÷ ${options.compression}`);
    const idle = await sample();
    let sampling = true;
    const sampler = (async () => {
      while (sampling) {
        await pause(2_000);
        await sample();
      }
    })();
    const hall = { base, tenant: catalog.id, quizzes, scale: 1 / options.compression };

    const alone = await phase(hall, options, proctor.pid);
    console.log(table("the hall alone", alone));
    const violations = hallViolations("alone", alone, BUDGET.alone);

    let beside: PhaseReport | undefined;
    let scriptReport: ScriptReport | undefined;
    if (options.hallOnly !== true) {
      const orders: ScriptOrders = { role: "script", base, tenant: catalog.id, quiz: quizzes[0], bodyLimit: LIMITS.bodyBytes, socketsPerAddress: LIMITS.socketsPerAddress };
      const worker = new Worker(new URL(import.meta.url), { workerData: orders });
      const scripted = new Promise<ScriptReport>((reported, failed) => {
        worker.once("message", reported);
        worker.once("error", failed);
      });
      await pause(1_000);
      beside = await phase(hall, options, proctor.pid);
      worker.postMessage("stop");
      scriptReport = await scripted;
      await worker.terminate();
      console.log(table("the hall beside the script", beside));
      console.log(`the script — ${scriptReport.seconds.toFixed(1)} s, sign-ups ${JSON.stringify(scriptReport.signUps)} (allowance ${signUpAllowance(scriptReport.seconds)}), commands ${JSON.stringify(scriptReport.commands)}, malformed ${JSON.stringify(scriptReport.malformed)}, queries ${JSON.stringify(scriptReport.queries)}, oversized bodies ${JSON.stringify(scriptReport.oversizedBody)}, sockets ${JSON.stringify(scriptReport.sockets)}, oversized frames ${JSON.stringify(scriptReport.oversizedFrame)}`);
      violations.push(...hallViolations("beside the script", beside, BUDGET.beside), ...scriptViolations(scriptReport));
      const kept = beside.sockets.bytes / Math.max(1, alone.sockets.bytes);
      console.log(`presence — beside the script's ${scriptReport.sockets.opened} sockets the hall was sent ${(kept * 100).toFixed(0)} % of the presence bytes it was sent alone`);
      if (kept < PRESENCE_SHARE) violations.push(`beside the script the hall was sent ${(kept * 100).toFixed(0)} % of the presence bytes it was sent alone, less than ${PRESENCE_SHARE * 100} %: the flood of sockets took the hall's share of the rooms' budget`);
    }

    sampling = false;
    await sampler;
    const end = await sample();
    const healthyAtEnd = await ready(base, 10_000);
    if (!healthyAtEnd) violations.push("the proctor no longer answers GET /instance");
    if (peak !== undefined && peak > MEMORY_BUDGET_MIB) violations.push(`the proctor reached ${Math.round(peak)} MiB, over the budget of ${MEMORY_BUDGET_MIB} MiB`);
    console.log(`the proctor — memory idle ${idle?.toFixed(0) ?? "?"} MiB, peak ${peak?.toFixed(0) ?? "?"} MiB, end ${end?.toFixed(0) ?? "?"} MiB; healthy at the end: ${healthyAtEnd}`);
    await proctor.stop();
    proctor = undefined;
    const roster = await rosterOf(executable, repoRoot, env, join(data, "prune.log"));
    violations.push(...rosterViolations(roster, options.learners * (beside === undefined ? 1 : 2), scriptReport));
    console.log(roster === undefined ? "the roster — not counted" : `the roster — ${roster.idle + roster.registrations} registrations of a cap of ${ROSTER_CAP} (${(((roster.idle + roster.registrations) / ROSTER_CAP) * 100).toFixed(1)} %): ${roster.registrations} of the ${roster.played} learners who played, ${roster.idle} nobody played under, which a prune removes`);
    const report: CapacityReport = { options, alone, beside, script: scriptReport, roster: roster === undefined ? undefined : { ...roster, cap: ROSTER_CAP }, memoryMiB: { idle, peak, end }, healthyAtEnd, violations };
    if (options.report !== undefined) {
      mkdirSync(dirname(resolve(options.report)), { recursive: true });
      writeFileSync(resolve(options.report), `${JSON.stringify(report, null, 2)}\n`);
    }
    return report;
  } finally {
    await proctor?.stop();
    rmSync(data, { recursive: true, force: true, maxRetries: 20, retryDelay: 100 });
  }
}

/** ⌨️ `bun <this file> [--learners n] [--compression n] [--report file] [--executable proctor] [--hall-only]`, run from the repository root. */
export async function capacityMain(repoRoot: string, args: readonly string[]): Promise<void> {
  const value = (name: string): string | undefined => {
    const index = args.indexOf(name);
    return index < 0 ? undefined : args[index + 1];
  };
  const whole = (name: string, fallback: number): number => {
    const stated = value(name);
    if (stated === undefined) return fallback;
    if (!/^[1-9][0-9]*$/u.test(stated)) throw new Error(`${name} must be a positive whole number, got ${JSON.stringify(stated)}`);
    return Number(stated);
  };
  const report = await runCapacity(repoRoot, { learners: whole("--learners", CAPACITY_DEFAULTS.learners), compression: whole("--compression", CAPACITY_DEFAULTS.compression), report: value("--report"), executable: value("--executable"), hallOnly: args.includes("--hall-only") });
  if (report.violations.length > 0) {
    for (const violation of report.violations) console.error(`[ERROR] capacity: ${violation}`);
    process.exitCode = 1;
    return;
  }
  console.log(report.beside === undefined ? "[INFO] capacity: the hall alone saw no error and no refusal and stayed inside its latency budget (the script was not played: this is not the gate)" : "[INFO] capacity: the hall saw no error and no refusal and stayed inside its latency budget alone and beside the script; the script was throttled, capped and cut off, and its sign-ups left the roster far below the cap");
}

if (isMainThread && import.meta.main) await capacityMain(process.cwd(), process.argv.slice(2));
if (!isMainThread && (workerData as HallOrders | ScriptOrders).role === "hall") {
  parentPort!.postMessage(await lecture(workerData as HallOrders, new Promise((dropped) => parentPort!.on("message", (message) => (message === "drop" ? dropped() : undefined)))));
}
if (!isMainThread && (workerData as HallOrders | ScriptOrders).role === "script") {
  let stop = false;
  parentPort!.on("message", (message) => {
    if (message === "stop") stop = true;
  });
  parentPort!.postMessage(await script(workerData as ScriptOrders, () => stop));
}
//#endregion 🏋️Gate
