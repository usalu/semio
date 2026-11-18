/** 🔍️ The audit of the proctor's chaos gate: everything the proctor holds about the learners is recomputed from the client-side journals alone, by the TypeScript quiz core, and compared with what the proctor's Rust deciders, streams and read models answer.
 *
 * @see ./../🥒️.feature
 * @see ../../../../../🧰️framework/🛍️products/📚️course/🔨️modules/🧾️lifecycle/🟦️.ts — the TypeScript deciders replayed here
 */
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { ServerClient } from "../../../../../🧰️framework/🛍️products/🖥️server/📡️client/🟦️.ts";
import type { EventRecord } from "../../../../../🧰️framework/🛍️products/🖥️server/🧬️wire-types/🟦️.ts";
import { PRESENCE_PROTOCOL, type PresenceEntry, type PresenceFrame } from "../../../../../🧰️framework/🛍️products/🖥️server/📦️packages/🟦️typescript/🧬️wire-types/🟦️.ts";
import { presenceSocketUrl } from "../../../../../🧰️framework/🛍️products/🖥️server/📦️packages/🟦️typescript/🛣️routes-and-sockets/🟦️.ts";
import { forwarded } from "../../🏋️capacity/🚚️transport/🟦️.ts";
import {
  DEFAULT_LIMITS,
  courseView,
  crowdView,
  decideLearner,
  emptyLearnerState,
  evolveLearner,
  leaderboard,
  learnerView,
  normalizeHandle,
  runView,
  transcript,
  type Course,
  type CourseView,
  type Event,
  type LeaderboardPeriod,
  type LearnerState,
  type LoadedQuiz,
  type Quiz,
  type RunResult,
} from "../../../../../🧰️framework/🛍️products/📚️course/📦️packages/🟦️typescript/🟦️.ts";
import { ProctorClient, type ProctorConnect } from "../../../../../🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/📦️packages/🟦️typescript/🔨️modules/🛂️proctor/🟦️.ts";
import { type Entry, type Journal } from "../🧑‍🎓️learner/🟦️.ts";

//#region 📚️Reference
/** 📚️ The course as the files hold it, with each quiz at the revision the proctor stamps: the SHA-256 of the file bytes. */
export interface Reference {
  readonly course: Course;
  readonly view: CourseView;
  readonly quizzes: Readonly<Record<string, LoadedQuiz>>;
  readonly ids: readonly string[];
}

export function reference(coursePath: string): Reference {
  const course = JSON.parse(readFileSync(coursePath, "utf8")) as Course & { quizzes: string[] };
  const loaded = course.quizzes.map((path) => {
    const bytes = readFileSync(resolve(dirname(coursePath), path));
    return { quiz: JSON.parse(bytes.toString("utf8")) as Quiz, revision: createHash("sha256").update(bytes).digest("hex") };
  });
  return { course, view: courseView(course, loaded.map(({ quiz }) => quiz)), quizzes: Object.fromEntries(loaded.map((entry) => [entry.quiz.id, entry])), ids: loaded.map(({ quiz }) => quiz.id) };
}

const sorted = (value: unknown, skip: string | undefined, digits?: number): unknown =>
  typeof value === "number" && digits !== undefined
    ? Number(value.toPrecision(digits))
    : Array.isArray(value)
    ? value.map((inner) => sorted(inner, skip, digits))
    : typeof value === "object" && value !== null
      ? Object.fromEntries(Object.entries(value).filter(([key, inner]) => key !== skip && inner !== undefined).sort(([left], [right]) => (left < right ? -1 : 1)).map(([key, inner]) => [key, sorted(inner, skip, digits)]))
      : value;
const plain = (value: unknown): unknown => sorted(value, "at");
const same = (left: unknown, right: unknown): boolean => JSON.stringify(plain(left)) === JSON.stringify(plain(right));
const exact = (left: unknown, right: unknown): boolean => JSON.stringify(sorted(left, undefined)) === JSON.stringify(sorted(right, undefined));
/** 🔬️ Equal to twelve significant digits: the proctor folds a crowd's mean positions incrementally and the quiz core all at once, so they may differ in the last bit. */
const close = (left: unknown, right: unknown): boolean => JSON.stringify(sorted(left, undefined, 12)) === JSON.stringify(sorted(right, undefined, 12));
const firstDifference = (left: unknown, right: unknown, path = ""): string | undefined => {
  const [a, b] = [sorted(left, undefined), sorted(right, undefined)];
  if (JSON.stringify(a) === JSON.stringify(b)) return undefined;
  if (typeof a === "object" && a !== null && typeof b === "object" && b !== null && Array.isArray(a) === Array.isArray(b)) {
    for (const key of new Set([...Object.keys(a), ...Object.keys(b)])) {
      const inner = firstDifference((a as Record<string, unknown>)[key], (b as Record<string, unknown>)[key], `${path}/${key}`);
      if (inner !== undefined) return inner;
    }
  }
  return `${path || "/"}: ${JSON.stringify(a)?.slice(0, 120)} vs ${JSON.stringify(b)?.slice(0, 120)}`;
};
const decoder = new TextDecoder();
const eventOf = (record: EventRecord): Event => JSON.parse(decoder.decode(record.payload)) as Event;
//#endregion 📚️Reference

//#region 🎞️Replay
/** 🎞️ What one journal replays to: the state the TypeScript deciders reach, the events a learner stream must hold, and every disagreement on the way. */
export interface Replayed {
  readonly state: LearnerState;
  readonly expected: readonly { readonly event: Event; readonly told: boolean }[];
  readonly disagreements: string[];
}

/** 🎞️ Decide every command of the journal again with the TypeScript core, in order: an accepted command must yield the events the proctor answered, a rejected one the rejection it answered. */
export function replay(journal: Journal, known: Reference): Replayed {
  let state = emptyLearnerState(journal.learner);
  const expected: { event: Event; told: boolean }[] = [];
  const disagreements: string[] = [];
  for (const entry of journal.entries) {
    const { command, verdict } = entry;
    const label = `${command.type} ${command.id}`;
    if (verdict.kind === "refused") {
      disagreements.push(`${label} was refused: ${verdict.detail}`);
      continue;
    }
    if (command.type === "identify-learner" && command.identity.kind !== "anonymous") {
      if (verdict.kind === "rejected") {
        if (verdict.rejection !== "handle-claimed") disagreements.push(`${label} was rejected as ${verdict.rejection}`);
      } else {
        const registered = verdict.events[0] as { type?: string; identity?: { kind: string; handle?: string } } | undefined;
        const display = normalizeHandle(command.identity.handle)?.display;
        if (verdict.events.length > 0 && verdict.events.length !== 1 || (verdict.events.length > 0 && (registered?.type !== "learner-registered" || registered.identity?.handle !== display || registered.identity?.kind !== command.identity.kind))) disagreements.push(`${label} registered ${JSON.stringify(verdict.events)} for ${JSON.stringify(command.identity)}`);
        const events = verdict.events.length > 0 ? verdict.events : [{ type: "learner-registered", learner: journal.learner, identity: { kind: command.identity.kind, handle: display }, at: entry.at } as Event];
        for (const event of events) state = evolveLearner(state, event);
        expected.push(...events.map((event) => ({ event, told: verdict.events.length > 0 })));
      }
      continue;
    }
    const decision = decideLearner(state, command, { now: entry.at, course: known.course, quizzes: known.quizzes, limits: DEFAULT_LIMITS });
    if (verdict.kind === "accepted") {
      if (!("events" in decision)) disagreements.push(`${label} was accepted by the proctor but the quiz core rejects it as ${decision.rejection}`);
      else if (verdict.events.length > 0 && !same(decision.events, verdict.events)) disagreements.push(`${label}: the quiz core decides ${JSON.stringify(plain(decision.events)).slice(0, 300)}, the proctor ${JSON.stringify(plain(verdict.events)).slice(0, 300)}`);
      const events = verdict.events.length > 0 ? verdict.events : "events" in decision ? decision.events : [];
      for (const event of events) state = evolveLearner(state, event);
      expected.push(...events.map((event) => ({ event, told: verdict.events.length > 0 })));
    } else if (!("rejection" in decision) || decision.rejection !== verdict.rejection) disagreements.push(`${label} was rejected as ${verdict.rejection} by the proctor, the quiz core decides ${JSON.stringify(decision).slice(0, 200)}`);
  }
  return { state, expected, disagreements };
}
//#endregion 🎞️Replay

//#region 🧮️Audit
/** 🧮️ What the audit counted, and what it found wrong. */
export interface Audit {
  readonly learners: number;
  readonly commands: { readonly settled: number; readonly accepted: number; readonly rejected: number };
  readonly events: number;
  readonly eventsByType: Record<string, number>;
  readonly runsStarted: number;
  readonly runsSubmitted: number;
  readonly answers: number;
  readonly streamRepeats: number;
  readonly handlesClaimed: number;
  readonly boardsChecked: number;
  readonly viewsChecked: number;
  readonly findings: string[];
}

async function pages(server: ServerClient, journal: Journal): Promise<EventRecord[]> {
  const all: EventRecord[] = [];
  for (let since = 0; ; ) {
    const page = await server.events(journal.actor, since);
    if (page.length === 0) return all;
    all.push(...page);
    since = page[page.length - 1].seq;
  }
}

/** 🧮️ Compare, learner by learner and board by board, the proctor with the journals: the event stream equals the accepted commands' events in order with no seq twice, every accepted submit scored exactly once as the quiz core scores it, every read view and board equals its recomputation, and every handle has one holder. */
export async function audit(connect: ProctorConnect, tenant: string, journals: readonly Journal[], known: Reference, taps?: ReadonlyMap<string, { seen: ReadonlyMap<number, EventRecord>; repeated: number }>): Promise<Audit> {
  const server = new ServerClient(connect());
  const client = new ProctorClient(connect, tenant);
  const findings: string[] = [];
  const found = (text: string): void => void (findings.length < 200 ? findings.push(text) : undefined);
  const eventsByType: Record<string, number> = {};
  const counts = { events: 0, accepted: 0, rejected: 0, settled: 0, started: 0, submitted: 0, answers: 0, repeats: 0, views: 0, boards: 0 };
  const states: LearnerState[] = [];
  const results: RunResult[] = [];

  for (const journal of journals) {
    const label = `learner ${journal.index} (${journal.learner})`;
    const { expected, disagreements } = replay(journal, known);
    for (const text of disagreements) found(`${label}: ${text}`);
    counts.settled += journal.entries.length;
    for (const entry of journal.entries) counts[entry.verdict.kind === "accepted" ? "accepted" : "rejected"] += 1;

    const records = await pages(server, journal);
    counts.events += records.length;
    records.forEach((record, position) => {
      if (record.seq !== position + 1) found(`${label}: stream position ${position + 1} holds seq ${record.seq}`);
    });
    const held = records.map(eventOf);
    const unequal = held.findIndex((event, at) => expected[at] === undefined || !(expected[at].told ? exact(event, expected[at].event) : same(event, expected[at].event)));
    if (held.length !== expected.length || unequal >= 0) found(`${label}: the stream holds ${held.length} events, the accepted commands made ${expected.length}; first difference at ${unequal}: ${JSON.stringify(held[unequal])?.slice(0, 200)} vs ${JSON.stringify(expected[unequal]?.event)?.slice(0, 200)}`);
    for (const event of held) eventsByType[event.type] = (eventsByType[event.type] ?? 0) + 1;
    const folded = held.reduce(evolveLearner, emptyLearnerState(journal.learner));
    states.push(folded);

    const accepted = journal.entries.filter((entry): entry is Entry => entry.verdict.kind === "accepted");
    const submits = accepted.filter((entry) => entry.command.type === "submit-run");
    const submittedRuns = held.filter((event) => event.type === "run-submitted") as (Event & { run?: string; result?: RunResult })[];
    counts.started += held.filter((event) => event.type === "run-started").length;
    counts.submitted += submittedRuns.length;
    counts.answers += held.filter((event) => event.type === "answer-recorded").length;
    if (submittedRuns.length !== submits.length) found(`${label}: ${submits.length} submit-run commands were accepted, the stream holds ${submittedRuns.length} run-submitted events`);
    const once = new Set(submittedRuns.map((event) => event.run));
    if (once.size !== submittedRuns.length) found(`${label}: a run was submitted twice`);
    for (const submit of submits) if (!once.has((submit.command as { run: string }).run)) found(`${label}: the accepted submit of run ${(submit.command as { run: string }).run} has no run-submitted event`);
    for (const event of submittedRuns) if (event.result !== undefined) results.push(event.result);

    const tap = taps?.get(journal.learner);
    if (taps !== undefined && tap === undefined) found(`${label}: no event stream was held`);
    else if (tap !== undefined) {
      counts.repeats += tap.repeated;
      if (tap.seen.size !== records.length || records.some((record) => !tap.seen.has(record.seq) || !exact(eventOf(tap.seen.get(record.seq)!), eventOf(record)))) found(`${label}: the event stream followed over the socket holds ${tap.seen.size} of ${records.length} events`);
    }

    const view = learnerView(folded, known.view);
    counts.views += 1;
    const asked = await client.learner(journal.learner).catch((error: unknown) => `${String(error)}`);
    if (!exact(view, asked)) found(`${label}: the learner view differs from its recomputation at ${firstDifference(asked, view)}`);
    for (const { run } of journal.runs) {
      const status = folded.runs.find((entry) => entry.run === run)?.status;
      if (status !== "submitted" && status !== "voided") found(`${label}: run ${run} ended ${status ?? "unknown"}, neither submitted nor voided`);
      counts.views += 1;
      const expectedRun = runView(folded, run, known.quizzes);
      const answered = await client.run(run, journal.learner).catch((error: unknown) => `${String(error)}`);
      if (expectedRun === undefined ? typeof answered !== "string" : !exact(expectedRun, answered)) found(`${label}: the view of run ${run} differs from its recomputation at ${firstDifference(answered, expectedRun)}`);
    }
  }

  const transcripts = states.map(transcript).filter((entry) => entry !== undefined);
  const periods: LeaderboardPeriod[] = ["all-time", "daily", "weekly", "monthly"];
  for (const period of periods) {
    for (const quiz of [undefined, ...known.ids]) {
      const at = Date.now();
      const answered = await client.leaderboard({ period, ...(quiz === undefined ? {} : { quiz }) }, undefined);
      counts.boards += 1;
      const computed = leaderboard(transcripts, known.view, { period, ...(quiz === undefined ? {} : { quiz }) }, at);
      if (!exact(computed, answered)) found(`leaderboard ${period}${quiz === undefined ? "" : ` of ${quiz}`} differs from its recomputation at ${firstDifference(answered, computed)}`);
    }
  }
  for (const [index, journal] of journals.entries()) {
    if (index % 5 !== 0) continue;
    const answered = await client.leaderboard({ period: "all-time" }, journal.learner);
    counts.boards += 1;
    if (!exact(leaderboard(transcripts, known.view, { period: "all-time" }, Date.now(), journal.learner), answered)) found(`the own row of learner ${journal.index} on the all-time board differs from its recomputation`);
  }
  for (const quiz of known.ids) {
    const answered = await client.crowd(quiz, undefined);
    counts.boards += 1;
    const computed = crowdView(known.quizzes[quiz].quiz, results);
    if (!close(computed, answered)) found(`the crowd of ${quiz} differs from its recomputation over ${results.filter((result) => result.quiz === quiz).length} results at ${firstDifference(answered, computed)}`);
  }

  const holders = new Map<string, string[]>();
  for (const journal of journals) {
    for (const entry of journal.entries) {
      if (entry.command.type !== "identify-learner" || entry.command.identity.kind === "anonymous") continue;
      const key = normalizeHandle(entry.command.identity.handle)?.key;
      if (key !== undefined && entry.verdict.kind === "accepted") holders.set(key, [...(holders.get(key) ?? []), journal.learner]);
    }
  }
  for (const [key, learners] of holders) if (learners.length !== 1) found(`handle ${key} was accepted for ${learners.length} learners`);
  for (const journal of journals) {
    for (const handle of journal.handles) {
      const key = normalizeHandle(handle)?.key ?? handle;
      const holder = await client.handle(handle);
      const winner = holders.get(key)?.[0];
      if (holder.holder?.learner !== winner) found(`handle ${key} is held by ${holder.holder?.learner ?? "nobody"}, the journals say ${winner ?? "nobody"}`);
    }
  }

  return {
    learners: journals.length,
    commands: { settled: counts.settled, accepted: counts.accepted, rejected: counts.rejected },
    events: counts.events,
    eventsByType,
    runsStarted: counts.started,
    runsSubmitted: counts.submitted,
    answers: counts.answers,
    streamRepeats: counts.repeats,
    handlesClaimed: holders.size,
    boardsChecked: counts.boards,
    viewsChecked: counts.views,
    findings,
  };
}
//#endregion 🧮️Audit

//#region 👥️Presence
/** 👥️ A silent member of the course-wide roster room that applies every frame as a client does: `welcome` replaces, `batch` removes `left` and upserts `entries`. */
export class Observer {
  readonly roster = new Map<string, PresenceEntry>();
  private socket: WebSocket | undefined;
  private own = "";

  async join(base: string, address: string, scope: string): Promise<void> {
    const socket = new WebSocket(presenceSocketUrl(base, scope, "observer"), { headers: forwarded(address), protocols: [PRESENCE_PROTOCOL] } as unknown as string[]);
    this.socket = socket;
    await new Promise<void>((welcomed, failed) => {
      socket.addEventListener("message", (event) => {
        const frame = JSON.parse(String(event.data)) as PresenceFrame;
        if (frame.type === "welcome") {
          this.own = frame.session;
          this.roster.clear();
          for (const entry of frame.roster) this.roster.set(entry.session, entry);
          welcomed();
        } else if (frame.type === "batch") {
          for (const session of frame.left) this.roster.delete(session);
          for (const entry of frame.entries) this.roster.set(entry.session, entry);
        }
      });
      socket.addEventListener("close", () => failed(new Error("the observer's socket closed")));
    });
  }

  members(): PresenceEntry[] {
    return [...this.roster.values()].filter((entry) => entry.session !== this.own);
  }

  leave(): void {
    this.socket?.close();
  }
}

/** ⏳️ Wait until the roster the observer holds is exactly `expected` (public tag → latest shared state, one session per tag), or say how it differs after `timeoutMs`. */
export async function converged(observer: Observer, expected: ReadonlyMap<string, unknown>, timeoutMs: number): Promise<string[]> {
  const deadline = Date.now() + timeoutMs;
  for (;;) {
    const problems: string[] = [];
    const held = new Map<string, PresenceEntry[]>();
    for (const entry of observer.members()) {
      const tag = (entry.state as { tag?: string } | null)?.tag ?? `(no state: ${entry.session})`;
      held.set(tag, [...(held.get(tag) ?? []), entry]);
    }
    for (const [tag, entries] of held) {
      if (!expected.has(tag)) problems.push(`a session of ${tag} is in the room that nobody holds open`);
      else if (entries.length !== 1) problems.push(`${tag} is in the room ${entries.length} times`);
      else if (!exact(entries[0].state, expected.get(tag))) problems.push(`${tag} shows ${JSON.stringify(entries[0].state).slice(0, 120)}, not its latest ${JSON.stringify(expected.get(tag)).slice(0, 120)}`);
    }
    for (const tag of expected.keys()) if (!held.has(tag)) problems.push(`${tag} is missing from the room`);
    if (problems.length === 0 || Date.now() > deadline) return problems.slice(0, 20);
    await new Promise((later) => setTimeout(later, 100));
  }
}
//#endregion 👥️Presence
