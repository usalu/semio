/** 🧑‍🎓️ One chaotic learner of the proctor's chaos gate: a protocol-level client that registers, plays every quiz of the course, reads the boards and keeps a journal of every command and its verdict, while the network aborts its requests, drops its sockets and its event stream, and the proctor restarts beneath it.
 *
 * @see ./../🥒️.feature
 * @see ../../🏋️capacity/🎒️hall-learner/🟦️.ts — the answers and presence states it reuses
 */
import { ServerClient } from "../../../../../🧰️framework/🛍️products/🖥️server/📡️client/🟦️.ts";
import { decodeEventRecord } from "../../../../../🧰️framework/🛍️products/🖥️server/🔤️codec/🟦️.ts";
import type { ActorKey, CommandOutcome, EventRecord } from "../../../../../🧰️framework/🛍️products/🖥️server/🧬️wire-types/🟦️.ts";
import { CHALLENGES, challengeRules, learnerTag, normalizeHandle, rosterScope, type Command, type IdentityClaim, type PresenceState, type RunView } from "../../../../../🧰️framework/🛍️products/📚️course/📦️packages/🟦️typescript/🟦️.ts";
import {
  ProctorClient,
  ProctorThrottled,
  ProctorUnavailable,
  commandEnvelope,
  commandVerdict,
  isTransient,
  newId,
  retryWait,
  type CommandVerdict,
  type HttpRequest,
  type HttpResponse,
  type ProctorConnect,
} from "../../../../../🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/📦️packages/🟦️typescript/🔨️modules/🛂️proctor/🟦️.ts";
import { type Measurements } from "../../🏋️capacity/📊️settings-and-measurements/🟦️.ts";
import { answerOf, shuffled } from "../../🏋️capacity/🎒️hall-learner/🟦️.ts";
import { between, forwarded, pause, Room } from "../../🏋️capacity/🚚️transport/🟦️.ts";

//#region 📒️Journal
/** 📊️ What every learner of the gate counts together. */
export interface Tally {
  readonly latencies: Map<string, number[]>;
  readonly perceived: Map<string, number[]>;
  readonly statuses: Map<number, number>;
  readonly serverFaults: string[];
  readonly problems: string[];
  sent: number;
  noAnswer: number;
  aborted: number;
  throttled: number;
  resent: number;
  duplicatesSent: number;
  duplicatesAgreed: number;
  unknownLearnerRetries: number;
  streamReopened: number;
  roomsDropped: number;
  commandsSettled: number;
}

export const tally = (): Tally => ({ latencies: new Map(), perceived: new Map(), statuses: new Map(), serverFaults: [], problems: [], sent: 0, noAnswer: 0, aborted: 0, throttled: 0, resent: 0, duplicatesSent: 0, duplicatesAgreed: 0, unknownLearnerRetries: 0, streamReopened: 0, roomsDropped: 0, commandsSettled: 0 });

/** 📝️ One command of the client-side log: what was sent, what the proctor decided and the durable records it answered. */
export interface Entry {
  readonly command: Command;
  readonly verdict: CommandVerdict;
  readonly records: readonly EventRecord[];
  readonly at: number;
}

/** 📒️ Everything one learner knows about its own play, kept apart from the proctor. */
export interface Journal {
  readonly index: number;
  readonly learner: string;
  readonly tag: string;
  readonly address: string;
  readonly actor: ActorKey;
  readonly entries: Entry[];
  readonly runs: { readonly run: string; readonly quiz: string; readonly challenge: string }[];
  readonly handles: string[];
  state: PresenceState | undefined;
  roster: Room | undefined;
  registered: IdentityClaim | undefined;
  done: boolean;
}

/** 🎚️ How hard the network treats one learner. */
export interface Chaos {
  readonly base: string;
  readonly tenant: string;
  readonly quizzes: readonly string[];
  readonly thinkMs: number;
  readonly abortRate: number;
  readonly duplicateRate: number;
  readonly retryMs: { readonly min: number; readonly max: number };
  readonly giveUpMs: number;
  readonly tally: Tally;
  readonly rooms: Set<Room>;
  readonly taps: Set<Tap>;
  readonly roomErrors: Measurements;
  readonly submitted: { count: number };
}
//#endregion 📒️Journal

//#region 🚚️Transport
const latency = (into: Map<string, number[]>, operation: string, milliseconds: number): void => void (into.get(operation)?.push(milliseconds) ?? into.set(operation, [milliseconds]));

/** 🚚️ `fetch` as the proxy would forward the client at `address`, counting every status (a 5xx is a fault of the proctor), every request that got no answer and every one this learner aborted itself. */
export function connectFor(chaos: Chaos, address: string): ProctorConnect {
  return (signal) => ({
    async send(request: HttpRequest): Promise<HttpResponse> {
      const query = new URLSearchParams(request.query as Record<string, string>).toString();
      const started = performance.now();
      let status: number;
      let text: string;
      let wait: number | undefined;
      chaos.tally.sent += 1;
      try {
        const response = await fetch(`${chaos.base}${request.path}${query === "" ? "" : `?${query}`}`, {
          method: request.method,
          headers: { ...request.headers, ...forwarded(address) },
          body: request.body === undefined ? undefined : typeof request.body === "string" ? request.body : new Uint8Array(request.body),
          signal: signal === undefined ? AbortSignal.timeout(chaos.giveUpMs) : AbortSignal.any([signal, AbortSignal.timeout(chaos.giveUpMs)]),
        });
        status = response.status;
        const after = response.headers.get("retry-after");
        wait = after !== null && /^[0-9]+$/u.test(after.trim()) ? Number(after) * 1000 : undefined;
        text = await response.text();
      } catch (error) {
        if (signal?.aborted) throw signal.reason ?? error;
        chaos.tally.noAnswer += 1;
        throw new ProctorUnavailable(error instanceof Error ? `${(error as { code?: string }).code ?? error.name}: ${error.message}` : String(error));
      }
      chaos.tally.statuses.set(status, (chaos.tally.statuses.get(status) ?? 0) + 1);
      latency(chaos.tally.latencies, request.path.startsWith("/commands") ? "command" : request.path.startsWith("/queries") ? "query" : "other", performance.now() - started);
      if (status >= 500) chaos.tally.serverFaults.push(`${request.method} ${request.path} -> ${status}: ${text.slice(0, 200)}`);
      if (status === 429 || (status === 503 && wait !== undefined)) {
        chaos.tally.throttled += 1;
        let said: number | undefined = wait;
        try {
          const body = JSON.parse(text) as { retryAfterMs?: number };
          said ??= typeof body.retryAfterMs === "number" ? body.retryAfterMs : undefined;
        } catch {
          said ??= undefined;
        }
        throw new ProctorThrottled(said);
      }
      const answer = status >= 400 && !jsonText(text) ? JSON.stringify({ kind: "http", message: `HTTP ${status}` }) : text;
      return { status, text: async () => answer, bytes: async () => new TextEncoder().encode(answer) };
    },
  });
}

/** 🤝️ Whether two answers to one command id are the same verdict: the receipt of a command that was decided already answers `accepted` without its events. */
const agrees = (left: CommandVerdict, right: CommandVerdict): boolean =>
  left.kind === right.kind && (left.kind !== "accepted" || right.kind !== "accepted" || left.events.length === 0 || right.events.length === 0 || JSON.stringify(left.events) === JSON.stringify(right.events)) && (left.kind !== "rejected" || right.kind !== "rejected" || left.rejection === right.rejection);

function jsonText(text: string): boolean {
  try {
    JSON.parse(text);
    return true;
  } catch {
    return false;
  }
}
//#endregion 🚚️Transport

//#region 📡️EventTap
/** 📡️ One learner's durable event stream as a client holds it: records by `seq`, resumed after the last one received whenever the socket is dropped; a seq received twice is counted, never kept twice. */
export class Tap {
  readonly seen = new Map<number, EventRecord>();
  repeated = 0;
  last = 0;
  private socket: WebSocket | undefined;
  private gone = false;

  constructor(
    private readonly chaos: Chaos,
    private readonly journal: Journal,
  ) {}

  get learner(): string {
    return this.journal.learner;
  }

  open(): void {
    if (this.gone) return;
    const url = new ServerClient({ send: async () => { throw new Error("unused"); } }).eventStreamUrl(this.chaos.base, this.journal.actor, this.last);
    const socket = new WebSocket(url, { headers: forwarded(this.journal.address) } as unknown as string[]);
    this.socket = socket;
    socket.addEventListener("message", (event) => {
      try {
        const record = decodeEventRecord(JSON.parse(String(event.data)));
        if (this.seen.has(record.seq)) this.repeated += 1;
        else this.seen.set(record.seq, record);
        this.last = Math.max(this.last, record.seq);
      } catch (error) {
        this.chaos.tally.problems.push(`event stream of ${this.journal.learner}: unreadable frame ${String(event.data).slice(0, 120)} (${String(error)})`);
      }
    });
    socket.addEventListener("close", () => {
      if (this.socket === socket && !this.gone) {
        this.socket = undefined;
        setTimeout(() => this.open(), between(20, 400));
      }
    });
    socket.addEventListener("error", () => undefined);
  }

  drop(): void {
    const socket = this.socket;
    this.socket = undefined;
    socket?.close();
    if (socket !== undefined) setTimeout(() => this.open(), between(10, 300));
    this.chaos.tally.streamReopened += 1;
  }

  close(): void {
    this.gone = true;
    this.socket?.close();
  }
}
//#endregion 📡️EventTap

//#region 🧑‍🎓️Learner
const POOL = ["Ada Lovelace", "Alan Turing", "Grace Hopper", "Emmy Noether", "Konrad Zuse", "Hedy Lamarr", "Carl Gauss", "Mary Jackson"];
const spelled = (handle: string, variant: number): string => [handle, handle.toUpperCase(), `  ${handle.replace(" ", "   ")} `, handle.toLowerCase()][variant % 4];

/** 🆕️ A learner's plan: a quarter register anonymously, a quarter under a pseudonym of their own, a quarter under a name of their own and a quarter fight over a few shared handles spelled four ways. */
export function claimOf(index: number, learner: string): IdentityClaim {
  switch (index % 4) {
    case 0:
      return { kind: "anonymous" };
    case 1:
      return { kind: "pseudonym", handle: `Chaos ${index} ${learner.slice(0, 8)}` };
    case 2:
      return { kind: "name", handle: `Learner ${index} ${learner.slice(8, 16)}` };
    default:
      return { kind: index % 8 === 3 ? "pseudonym" : "name", handle: spelled(POOL[Math.floor(index / 4) % POOL.length], Math.floor(index / 32)) };
  }
}

/** 🆕️ A fresh journal for the learner at `index`. */
export function journalOf(chaos: Chaos, index: number, addresses: number): Journal {
  const learner = newId();
  return { index, learner, tag: learnerTag(learner), address: `198.51.100.${(index % addresses) + 1}`, actor: { tenant: chaos.tenant, kind: "course-learner", id: learner }, entries: [], runs: [], handles: [], state: undefined, roster: undefined, registered: undefined, done: false };
}

/** 🧑‍🎓️ Play one learner's whole attendance against the chaos: register, hold the roster and page rooms and the event stream open, play every quiz of the course at a challenge that varies, read the boards, then stay connected until the gate has audited the presence. */
export async function attend(chaos: Chaos, journal: Journal, arrivalMs: number): Promise<void> {
  const { tally: counted } = chaos;
  const connect = connectFor(chaos, journal.address);
  const client = new ProctorClient(connect, chaos.tenant);
  const think = (): Promise<void> => pause(Math.random() * chaos.thinkMs);
  const problem = (text: string): number => counted.problems.push(`learner ${journal.index} (${journal.learner}): ${text}`);
  const submit = (command: Command, connection: ReturnType<ProctorConnect>): Promise<CommandOutcome> => new ServerClient(connection).submitCommand(commandEnvelope(command, chaos.tenant, Date.now()));

  const send = async (command: Command, rejects: readonly string[] = []): Promise<CommandVerdict> => {
    const started = performance.now();
    let attempts = 0;
    let verdict: CommandVerdict | undefined;
    let outcome: CommandOutcome | undefined;
    while (verdict === undefined) {
      attempts += 1;
      if (performance.now() - started > chaos.giveUpMs * 4) throw new Error(`${command.type} ${command.id} got no verdict in ${Math.round((performance.now() - started) / 1000)} s`);
      const abort = new AbortController();
      const timer = Math.random() < chaos.abortRate ? setTimeout(() => abort.abort(), between(0, 25)) : undefined;
      const twin = Math.random() < chaos.duplicateRate ? submit(command, connect()).catch(() => undefined) : undefined;
      if (twin !== undefined) counted.duplicatesSent += 1;
      try {
        outcome = await submit(command, connect(abort.signal));
        verdict = commandVerdict(outcome);
        const twinOutcome = await twin;
        if (twinOutcome !== undefined) {
          if (agrees(commandVerdict(twinOutcome), verdict)) counted.duplicatesAgreed += 1;
          else problem(`two concurrent sends of ${command.type} ${command.id} got different verdicts: ${JSON.stringify(commandVerdict(twinOutcome)).slice(0, 200)} vs ${JSON.stringify(verdict).slice(0, 200)}`);
        }
        if (verdict.kind === "rejected" && verdict.rejection === "unknown-learner" && command.type !== "identify-learner") {
          counted.unknownLearnerRetries += 1;
          verdict = undefined;
          await pause(between(20, 200));
        }
      } catch (error) {
        await twin;
        if (abort.signal.aborted) counted.aborted += 1;
        else if (isTransient(error)) await pause(Math.max(retryWait(error), between(chaos.retryMs.min, chaos.retryMs.max)));
        else throw error;
      } finally {
        clearTimeout(timer);
      }
    }
    counted.resent += attempts - 1;
    latency(counted.perceived, command.type, performance.now() - started);
    const records = outcome === undefined ? [] : outcome.status === "accepted" ? outcome.events : outcome.status === "transformed" ? outcome.canonicalEvents : [];
    journal.entries.push({ command, verdict, records, at: Date.now() });
    counted.commandsSettled += 1;
    if (verdict.kind === "refused") problem(`${command.type} refused: ${verdict.detail}`);
    if (verdict.kind === "rejected" && !rejects.includes(verdict.rejection)) problem(`${command.type} rejected: ${verdict.rejection}`);
    if (Math.random() < chaos.duplicateRate) {
      counted.duplicatesSent += 1;
      const again = await submit(command, connect()).then(commandVerdict, () => undefined);
      if (again !== undefined && !agrees(again, verdict)) problem(`a late duplicate of ${command.type} ${command.id} got another verdict: ${JSON.stringify(again).slice(0, 200)}`);
    }
    return verdict;
  };

  const ask = async <V>(operation: string, query: (signal: AbortSignal) => Promise<V>): Promise<V> => {
    const started = performance.now();
    for (;;) {
      const abort = new AbortController();
      const timer = Math.random() < chaos.abortRate ? setTimeout(() => abort.abort(), between(0, 25)) : undefined;
      try {
        const value = await query(abort.signal);
        latency(counted.perceived, operation, performance.now() - started);
        return value;
      } catch (error) {
        if (abort.signal.aborted) counted.aborted += 1;
        else if (isTransient(error)) await pause(Math.max(retryWait(error), between(chaos.retryMs.min, chaos.retryMs.max)));
        else throw error;
      } finally {
        clearTimeout(timer);
      }
    }
  };

  const room = (scope: string, surface: string): Room => {
    const joined = new Room(chaos.base, journal.address, scope, surface, () => chaos.roomErrors);
    chaos.rooms.add(joined);
    return joined;
  };

  await pause(Math.random() * arrivalMs);
  const learner = journal.learner;
  let claim = claimOf(journal.index, learner);
  let registration = await send({ type: "identify-learner", id: newId(), learner, identity: claim }, ["handle-claimed"]);
  if (claim.kind !== "anonymous") {
    journal.handles.push(claim.handle);
    if (registration.kind === "rejected") {
      const holder = await ask("handle", (signal) => client.handle(claim.kind === "anonymous" ? "" : claim.handle, signal));
      if (holder.holder?.learner === learner) problem(`its own handle was refused as claimed while it holds it`);
      claim = { kind: "anonymous" };
      registration = await send({ type: "identify-learner", id: newId(), learner, identity: claim });
    }
  }
  if (registration.kind !== "accepted") return void problem("never registered");
  journal.registered = claim;
  const shown: PresenceState["identity"] = (registration.events[0] as { identity?: PresenceState["identity"] } | undefined)?.identity ?? (claim.kind === "anonymous" ? claim : { kind: claim.kind, handle: normalizeHandle(claim.handle)?.display ?? claim.handle });
  const here = (place: PresenceState["place"]): PresenceState => (journal.state = { tag: journal.tag, identity: shown, place, active: true });

  const tap = new Tap(chaos, journal);
  chaos.taps.add(tap);
  tap.open();
  const roster = room(rosterScope(chaos.tenant), "home");
  journal.roster = roster;
  let page = room(`${chaos.tenant}/home`, "home");
  await Promise.all([roster.join(), page.join()]);
  roster.share(here({ screen: "home" }));
  await Promise.all([ask("course", (signal) => client.course(learner, signal)), ask("learner", (signal) => client.learner(learner, signal))]);

  const order = shuffled(chaos.quizzes);
  for (const [played, quiz] of order.entries()) {
    await think();
    const start = async (challenge: (typeof CHALLENGES)[number]): Promise<{ run: string; view: RunView } | undefined> => {
      const run = newId();
      const started = await send({ type: "start-run", id: newId(), learner, run, quiz, challenge, at: Date.now() });
      if (started.kind !== "accepted") return undefined;
      journal.runs.push({ run, quiz, challenge });
      return { run, view: await ask("run", (signal) => client.run(run, learner, signal)) };
    };
    let challenge = CHALLENGES[(journal.index + played) % CHALLENGES.length];
    let current = await start(challenge);
    if (current === undefined) continue;
    if (journal.index % 7 === 0 && played % 3 === 0) await send({ type: "start-run", id: newId(), learner, run: newId(), quiz, challenge, at: Date.now() }, ["run-open"]);
    if (journal.index % 3 === 0 && played % 4 === 0) {
      const [first] = current.view.sheet.tasks;
      const draft = answerOf(first).answer;
      await send({ type: "record-answer", id: newId(), learner, run: current.run, task: first.id, answer: draft, at: Date.now() }, ["task-unopened", "time-up"]);
      challenge = CHALLENGES[(journal.index + played + 1) % CHALLENGES.length];
      current = await start(challenge);
      if (current === undefined) continue;
    }
    page.leave();
    chaos.rooms.delete(page);
    page = room(`${chaos.tenant}/quiz/${quiz}`, "run");
    await page.join();
    const { run, view } = current;
    for (const task of view.sheet.tasks) {
      roster.share(here({ screen: "run", quiz, task: task.id }));
      const opened = Date.now();
      if (task.seconds !== undefined && (await send({ type: "open-task", id: newId(), learner, run, task: task.id, at: opened })).kind !== "accepted") continue;
      for (let revision = 0, revisions = 1 + Math.floor(Math.random() * 3); revision < revisions; revision += 1) {
        await think();
        page.share({ tag: journal.tag, cursor: { anchor: `task:${task.id}`, x: Math.random(), y: Math.random() }, focus: `task:${task.id}` });
        const verdict = await send({ type: "record-answer", id: newId(), learner, run, task: task.id, answer: answerOf(task).answer, at: Date.now() }, ["time-up"]);
        if (verdict.kind === "accepted" && challengeRules(challenge).hints) await ask("run", (signal) => client.run(run, learner, signal));
      }
    }
    const submitted = await send({ type: "submit-run", id: newId(), learner, run });
    if (submitted.kind === "accepted") chaos.submitted.count += 1;
    roster.share(here({ screen: "results", quiz }));
    await Promise.all([
      ask("run", (signal) => client.run(run, learner, signal)),
      ask("crowd", (signal) => client.crowd(quiz, learner, signal)),
      ask("leaderboard", (signal) => client.leaderboard({ period: "all-time" }, learner, signal)),
      ask("leaderboard", (signal) => client.leaderboard({ period: "daily", quiz }, learner, signal)),
      ask("learner", (signal) => client.learner(learner, signal)),
    ]);
    page.leave();
    chaos.rooms.delete(page);
    page = room(`${chaos.tenant}/home`, "home");
    await page.join();
    roster.share(here({ screen: "home" }));
  }
  journal.done = true;
}
//#endregion 🧑‍🎓️Learner
