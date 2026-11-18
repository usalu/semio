/** 🌪️ The proctor's chaos gate: about a hundred protocol-level learners play every quiz of the architecture course against the real binary while sockets drop, requests abort, commands are sent twice and the proctor process restarts once; everything the proctor holds is then audited against a recomputation from the learners' own journals.
 *
 * Run from the repository root: `bun <this file> [--learners 100] [--think-ms 60] [--addresses 100] [--abort-rate 0.05] [--duplicate-rate 0.05] [--restart kill|term|none] [--port 8792] [--executable proctor] [--report file]`.
 *
 * @see ./🥒️.feature
 * @see ../🏋️capacity/🟦️.ts — the lecture-hall gate this one complements
 */
import { spawn, type ChildProcess } from "node:child_process";
import { closeSync, mkdirSync, openSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { PROCTOR_DEV_COURSE } from "../../🏗️bootstrap/🟦️.ts";
import { rosterScope } from "../../../../🧰️framework/🛍️products/📚️course/📦️packages/🟦️typescript/🟦️.ts";
import { measurements, percentile, SITE } from "../🏋️capacity/📊️settings-and-measurements/🟦️.ts";
import { ready } from "../🏋️capacity/🖥️proctor-process/🟦️.ts";
import { between, pause } from "../🏋️capacity/🚚️transport/🟦️.ts";
import { attend, connectFor, journalOf, tally, type Chaos, type Journal } from "./🧑‍🎓️learner/🟦️.ts";
import { audit, converged, Observer, reference, type Audit } from "./🔍️audit/🟦️.ts";

//#region ⚙️Options
/** ⚙️ What one run of the gate is told. */
export interface ChaosOptions {
  readonly learners: number;
  readonly thinkMs: number;
  readonly arrivalMs: number;
  readonly addresses: number;
  readonly abortRate: number;
  readonly duplicateRate: number;
  readonly restart: "kill" | "term" | "none";
  readonly port: number;
  readonly executable: string;
  readonly report: string | undefined;
}

const DEFAULTS = { learners: 100, thinkMs: 60, arrivalMs: 3_000, addresses: 100, abortRate: 0.05, duplicateRate: 0.05, restart: "kill" as const, port: 8792 };
//#endregion ⚙️Options

//#region 🖥️Proctor
/** 🖥️ The proctor child process on a fixed port over one data directory; started again after a restart on the same data. */
class Proctor {
  private child: ChildProcess | undefined;
  private exited: Promise<void> | undefined;

  constructor(
    private readonly executable: string,
    private readonly env: NodeJS.ProcessEnv,
    private readonly log: string,
    readonly base: string,
  ) {}

  async start(): Promise<void> {
    const sink = openSync(this.log, "a");
    const child = spawn(this.executable, ["serve"], { env: this.env, stdio: ["ignore", sink, sink], windowsHide: true });
    this.child = child;
    this.exited = new Promise((done) => child.once("close", () => (closeSync(sink), done())));
    if (!(await ready(this.base, 60_000))) throw new Error(`the proctor did not come up on ${this.base}:\n${readFileSync(this.log, "utf8").slice(-1500)}`);
  }

  async stop(signal: NodeJS.Signals): Promise<void> {
    this.child?.kill(signal);
    await this.exited;
    this.child = undefined;
  }
}
//#endregion 🖥️Proctor

//#region 🌪️Gate
/** 📑️ The report of one run. */
export interface ChaosReport {
  readonly options: ChaosOptions;
  readonly seconds: number;
  readonly audit: Audit;
  readonly auditAfterRestart: Audit;
  readonly chaos: Record<string, number>;
  readonly statuses: Record<string, number>;
  readonly latency: Record<string, { readonly count: number; readonly p50: number; readonly p95: number; readonly p99: number; readonly max: number }>;
  readonly perceived: Record<string, { readonly count: number; readonly p50: number; readonly p95: number; readonly p99: number; readonly max: number }>;
  readonly presence: readonly string[];
  readonly violations: readonly string[];
}

const row = (samples: readonly number[]) => ({ count: samples.length, p50: percentile(samples, 50), p95: percentile(samples, 95), p99: percentile(samples, 99), max: percentile(samples, 100) });

/** 🌪️ Play the learners against the proctor under chaos, audit the result, restart the proctor once more and audit again; `violations` is empty exactly when the gate holds. */
export async function runChaos(repoRoot: string, options: ChaosOptions): Promise<ChaosReport> {
  const coursePath = resolve(repoRoot, ...PROCTOR_DEV_COURSE);
  const known = reference(coursePath);
  const work = join(repoRoot, ".🧬semio", "🎓️teaching", "proctor-chaos", `run-${Date.now()}`);
  mkdirSync(work, { recursive: true });
  const base = `http://127.0.0.1:${options.port}`;
  const env: NodeJS.ProcessEnv = { ...process.env, PROCTOR_PORT: String(options.port), PROCTOR_BIND: "127.0.0.1", PROCTOR_MODE: "production", PROCTOR_ALLOWED_ORIGINS: SITE, PROCTOR_TRUSTED_FORWARDING: "proxy", PROCTOR_DATA: join(work, "data"), PROCTOR_COURSE: coursePath };
  mkdirSync(join(work, "data"), { recursive: true });
  const proctor = new Proctor(resolve(options.executable), env, join(work, "proctor.log"), base);
  const violations: string[] = [];
  const count = tally();
  const chaos: Chaos = {
    base,
    tenant: known.course.id,
    quizzes: known.ids,
    thinkMs: options.thinkMs,
    abortRate: options.abortRate,
    duplicateRate: options.duplicateRate,
    retryMs: { min: 20, max: 400 },
    giveUpMs: 20_000,
    tally: count,
    rooms: new Set(),
    taps: new Set(),
    roomErrors: measurements(),
    submitted: { count: 0 },
  };
  const counters = { restarts: 0, roomsDropped: 0, streamsDropped: 0, wifiStorms: 0 };
  const started = performance.now();
  try {
    await proctor.start();
    const journals: Journal[] = Array.from({ length: options.learners }, (_, index) => journalOf(chaos, index, options.addresses));
    const attending = journals.map((journal) =>
      attend(chaos, journal, options.arrivalMs).catch((error: unknown) => void violations.push(`learner ${journal.index} (${journal.learner}) failed: ${error instanceof Error ? error.stack ?? error.message : String(error)}`)),
    );
    let playing = true;
    const everyone = Promise.all(attending).finally(() => (playing = false));
    const expectedRuns = options.learners * known.ids.length;
    let restarted = options.restart === "none";
    const progress = setInterval(() => console.log(`[INFO] chaos: ${Math.round((performance.now() - started) / 1000)} s, ${chaos.submitted.count}/${expectedRuns} runs submitted, ${count.sent} requests sent, ${count.aborted} aborted, ${count.resent} commands sent again, ${count.throttled} throttled, ${count.noAnswer} without answer, ${count.problems.length} problems`), 10_000);
    void everyone.finally(() => clearInterval(progress));
    const reconnecting = new Set<unknown>();
    const reconnect = (room: Chaos["rooms"] extends Set<infer R> ? R : never, afterMs: number): void => {
      if (reconnecting.has(room)) return;
      reconnecting.add(room);
      void room.reconnect(afterMs).catch(() => undefined).finally(() => reconnecting.delete(room));
    };
    const mischief = (async () => {
      while (playing) {
        await pause(between(40, 160));
        if (!playing) break;
        const roll = Math.random();
        const rooms = [...chaos.rooms];
        const streams = [...chaos.taps];
        if (!restarted && chaos.submitted.count >= expectedRuns * 0.35) {
          restarted = true;
          const marked = chaos.roomErrors.errors.length;
          await proctor.stop(options.restart === "kill" ? "SIGKILL" : "SIGTERM");
          await pause(between(300, 1_200));
          await proctor.start();
          counters.restarts += 1;
          chaos.roomErrors.errors.splice(marked);
          for (const room of chaos.rooms) reconnect(room, between(0, 500));
        } else if (roll < 0.5 && rooms.length > 0) {
          counters.roomsDropped += 1;
          reconnect(rooms[Math.floor(Math.random() * rooms.length)], between(0, 400));
        } else if (roll < 0.85 && streams.length > 0) {
          counters.streamsDropped += 1;
          streams[Math.floor(Math.random() * streams.length)].drop();
        } else if (rooms.length > 0) {
          counters.wifiStorms += 1;
          for (const room of rooms.filter(() => Math.random() < 0.15)) reconnect(room, between(0, 800));
        }
      }
    })();
    await everyone;
    await mischief;
    const taps = new Map([...chaos.taps].map((tap) => [tap.learner, tap] as const));

    if (!restarted) violations.push("the proctor was never restarted: the gate ended before it reached the point of the restart");
    for (const journal of journals) journal.roster?.share(journal.state);
    await pause(1_500);
    const observer = new Observer();
    await observer.join(base, "198.51.100.250", rosterScope(chaos.tenant));
    const expected = new Map(journals.filter((journal) => journal.state !== undefined).map((journal) => [journal.tag, journal.state]));
    const presence = await converged(observer, expected, 15_000);
    await pause(1_000);
    const connect = connectFor(chaos, "198.51.100.251");
    const settled = await audit(connect, chaos.tenant, journals, known, taps);
    for (const room of chaos.rooms) room.leave();
    for (const tap of chaos.taps) tap.close();
    const emptied = await converged(observer, new Map(), 15_000);
    observer.leave();
    presence.push(...emptied.map((problem) => `after every learner left: ${problem}`));

    await proctor.stop("SIGTERM");
    await proctor.start();
    const again = await audit(connect, chaos.tenant, journals, known);
    const seconds = Math.round((performance.now() - started) / 100) / 10;

    const asked = (name: string) => Object.fromEntries([...(name === "raw" ? count.latencies : count.perceived)].map(([operation, samples]) => [operation, row(samples)]));
    const report: ChaosReport = {
      options,
      seconds,
      audit: settled,
      auditAfterRestart: again,
      chaos: {
        restarts: counters.restarts,
        roomsReconnected: counters.roomsDropped,
        wifiStorms: counters.wifiStorms,
        eventStreamsDropped: counters.streamsDropped,
        requestsSent: count.sent,
        requestsAborted: count.aborted,
        requestsWithoutAnswer: count.noAnswer,
        commandsSentAgain: count.resent,
        duplicatesSent: count.duplicatesSent,
        duplicatesAgreed: count.duplicatesAgreed,
        throttled: count.throttled,
        unknownLearnerRetries: count.unknownLearnerRetries,
        submittedCommandsAccepted: chaos.submitted.count,
      },
      statuses: Object.fromEntries([...count.statuses].sort(([left], [right]) => left - right).map(([status, times]) => [String(status), times])),
      latency: asked("raw"),
      perceived: asked("perceived"),
      presence,
      violations,
    };
    violations.push(...count.serverFaults.slice(0, 20).map((fault) => `server fault: ${fault}`));
    violations.push(...count.problems.slice(0, 40));
    violations.push(...chaos.roomErrors.errors.slice(0, 20).map((error) => `presence: ${error}`));
    violations.push(...presence.map((problem) => `presence did not converge: ${problem}`));
    violations.push(...settled.findings.map((finding) => `audit: ${finding}`), ...again.findings.map((finding) => `audit after the restart: ${finding}`));
    if (settled.runsSubmitted !== chaos.submitted.count) violations.push(`${chaos.submitted.count} submissions were accepted, the streams hold ${settled.runsSubmitted} run-submitted events`);
    if (settled.eventsByType["run-submitted"] !== again.eventsByType["run-submitted"]) violations.push("the events differ across the final restart");
    if (settled.events !== again.events) violations.push(`the streams held ${settled.events} events before the final restart and ${again.events} after`);
    for (const [name, minimum] of [["requestsAborted", 1], ["roomsReconnected", 1], ["eventStreamsDropped", 1], ["duplicatesSent", 1], ["restarts", options.restart === "none" ? 0 : 1]] as const) if (report.chaos[name] < minimum) violations.push(`the chaos never happened: ${name} is ${report.chaos[name]}`);
    if (options.report !== undefined) {
      mkdirSync(dirname(resolve(options.report)), { recursive: true });
      writeFileSync(resolve(options.report), `${JSON.stringify(report, null, 2)}\n`);
    }
    return report;
  } finally {
    await proctor.stop("SIGKILL").catch(() => undefined);
    for (const tap of chaos.taps) tap.close();
    for (const room of chaos.rooms) room.leave();
    if (process.env.PROCTOR_CHAOS_KEEP === undefined) rmSync(work, { recursive: true, force: true, maxRetries: 20, retryDelay: 100 });
  }
}

/** ⌨️ The command line of the gate. */
export async function chaosMain(repoRoot: string, args: readonly string[]): Promise<void> {
  const value = (name: string): string | undefined => (args.includes(name) ? args[args.indexOf(name) + 1] : undefined);
  const number = (name: string, fallback: number): number => {
    const stated = value(name);
    if (stated === undefined) return fallback;
    if (!Number.isFinite(Number(stated)) || Number(stated) < 0) throw new Error(`${name} must be a non-negative number, got ${JSON.stringify(stated)}`);
    return Number(stated);
  };
  const executable = value("--executable");
  if (executable === undefined) throw new Error("--executable <path to the proctor binary> is required");
  const restart = (value("--restart") ?? DEFAULTS.restart) as ChaosOptions["restart"];
  if (!["kill", "term", "none"].includes(restart)) throw new Error("--restart must be kill, term or none");
  const report = await runChaos(repoRoot, {
    learners: number("--learners", DEFAULTS.learners),
    thinkMs: number("--think-ms", DEFAULTS.thinkMs),
    arrivalMs: number("--arrival-ms", DEFAULTS.arrivalMs),
    addresses: Math.max(1, number("--addresses", DEFAULTS.addresses)),
    abortRate: number("--abort-rate", DEFAULTS.abortRate),
    duplicateRate: number("--duplicate-rate", DEFAULTS.duplicateRate),
    restart,
    port: number("--port", DEFAULTS.port),
    executable,
    report: value("--report"),
  });
  const { audit: settled } = report;
  console.log(`[INFO] chaos: ${settled.learners} learners, ${report.seconds} s, ${settled.commands.settled} commands (${settled.commands.accepted} accepted, ${settled.commands.rejected} rejected), ${settled.events} events, ${settled.runsStarted} runs started, ${settled.runsSubmitted} submitted, ${settled.answers} answers`);
  console.log(`[INFO] chaos: ${JSON.stringify(report.chaos)}`);
  console.log(`[INFO] chaos: statuses ${JSON.stringify(report.statuses)}`);
  for (const [operation, measured] of Object.entries(report.latency)) console.log(`[INFO] chaos: request ${operation.padEnd(8)} n ${measured.count} p50 ${measured.p50} p95 ${measured.p95} p99 ${measured.p99} max ${measured.max} ms`);
  for (const [operation, measured] of Object.entries(report.perceived)) console.log(`[INFO] chaos: learner  ${operation.padEnd(16)} n ${measured.count} p50 ${measured.p50} p95 ${measured.p95} p99 ${measured.p99} ms`);
  console.log(`[INFO] chaos: audited ${settled.viewsChecked} views and ${settled.boardsChecked} boards, ${settled.handlesClaimed} handles held; event-stream repeats absorbed: ${settled.streamRepeats}`);
  if (report.violations.length > 0) {
    for (const violation of report.violations) console.error(`[ERROR] chaos: ${violation}`);
    process.exitCode = 1;
    return;
  }
  console.log("[INFO] chaos: the gate holds");
}

if (import.meta.main) await chaosMain(process.cwd(), process.argv.slice(2));
//#endregion 🌪️Gate
