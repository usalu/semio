/** 🏋️ The proctor's capacity gate: a lecture hall against the real binary, then the same hall beside one abusive script.
 *
 * **What runs.** The release build of `proctor serve` in production mode (origin allowlist, trusted proxy forwarding,
 * the production limits stated explicitly: {@link LIMITS}) on a throw-away loopback port over a throw-away data
 * directory and the architecture course. The client is this file on Bun's own `fetch`, `WebSocket` and sockets — an
 * HTTP and WebSocket implementation that shares no code with the proctor — speaking the quiz client's real envelopes
 * ({@link ProctorClient}) and presence frames.
 *
 * **The hall.** `learners` simulated learners (300 by default) arrive from ONE address (the `X-Forwarded-For` the
 * terminating proxy would write): each enrols, joins the roster room and the home room (watching every page, quiz and
 * thinking room at 4 Hz), polls the leaderboard, then plays two quizzes at two of the four challenges, the hall
 * spread evenly over all four — start a run, read its sheet, join the quiz and thinking rooms, revise every task's
 * answer a few times with cursor moves and drafts in between (cards on easy and medium, guesses on hard and expert; on
 * expert each task is opened first and answered only while its clock runs, on easy the run is read again after every
 * answer for its hints), submit, read the result, the crowd and the leaderboard. Think times are real ones divided by `compression` (5 by default), so the
 * hall asks for `compression` times the requests per second a real lecture does. Once per phase every socket of every
 * learner drops and reconnects within two seconds (the Wi-Fi drop). A request that gets no answer is sent again, and a
 * socket join that gets none is made again, after the quiz client's own backoff, and the wait counts as its latency.
 *
 * **The script.** In the second phase a worker thread plays one abusive client from another address beside the hall:
 * it tries to fill the roster — registrations, anonymous and under handles, as fast as it can post them — while the
 * hall signs up, floods malformed commands and queries on connections it keeps open ({@link CapacityWire}), sends oversized
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
 * @see ../../../../🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🔨️modules/🛂️proctor/🟦️.ts — the client mapping driven here
 */
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { Worker, isMainThread, parentPort, workerData } from "node:worker_threads";
import { PROCTOR_DEV_COURSE, launchProctor, type ProctorArtifact, type ProctorProcess } from "../../🏗️bootstrap/🟦️.ts";
import { acquireProctor } from "../../🏗️bootstrap/🧾️product/🟦️.ts";
import { receiveScriptProcessInvocation, type ScriptInvocation } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { ticketOutput } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/⚡️caching/🎫️output/🟦️.ts";

/** 🎫️ Locates disposable capacity work beneath the active ticket or development cache. */
export function capacityWorkDirectory(repoRoot: string, environment: NodeJS.ProcessEnv = process.env): string {
  return environment.SEMIO_TICKET_DIR ? join(ticketOutput(repoRoot, [], environment), "capacity") : join(repoRoot, ".🧬semio", "🎓️teaching", "proctor-capacity");
}

import { BUDGET, CAPACITY_DEFAULTS, capacityProgress, type CapacityOptions, GENERATOR_LAG_MS, GENERATOR_STALL_MS, HALL, HALL_THREADS, type LatencyRow, LIMITS, measurements, type Measurements, MEMORY_BUDGET_MIB, percentile, PRESENCE_SHARE, REAL, RESENT_SHARE, ROSTER_CAP, ROSTER_SHARE, rows, SITE } from "./📊️settings-and-measurements/🟦️.ts";
import { pause } from "./🚚️transport/🟦️.ts";
import { type HallOrders, type HallSlice, lecture } from "./🎒️hall-learner/🟦️.ts";
import { script, type ScriptOrders, type ScriptReport } from "./🎭️abusive-script/🟦️.ts";
import { freePort, processorSeconds, ready, residentMebibytes, type Roster, rosterOf } from "./🖥️proctor-process/🟦️.ts";

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

/** 📑️ The whole report of one capacity run; `beside` and `script` are absent from a run of the hall alone. */
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
  capacityProgress(hall.progressFile, "phase started");
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
  capacityProgress(hall.progressFile, "phase worker reports complete");
  await dropping;
  await Promise.all(workers.map((worker) => worker.terminate()));
  capacityProgress(hall.progressFile, "phase workers terminated");
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
export function hallViolations(name: string, phase: PhaseReport, budget: { readonly p95: number; readonly p99: number }): string[] {
  const violations: string[] = [];
  if (phase.reconnected === 0) violations.push(`the hall ${name} skipped the connection drop: no presence socket reconnected`);
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

/** 🚨️ Everything the script got away with: a flood that was never refused or was served more than its address's
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

/** 🚀️ Build the release proctor, serve it on a throw-away port and data directory with its default cap of learners, play
 * the hall alone and beside the script, stop the proctor, count its roster and return the report; `violations` is empty
 * exactly when the gate holds. */
export async function runCapacity(repoRoot: string, invocation: ScriptInvocation, options: CapacityOptions = CAPACITY_DEFAULTS): Promise<CapacityReport> {
  const artifact = options.executable === undefined ? await acquireProctor(repoRoot, invocation, process.env, undefined, "release") : undefined;
  const launcher = artifact ?? { launch: (args: readonly string[], env?: NodeJS.ProcessEnv, log?: string) => launchProctor(resolve(options.executable!), repoRoot, args, env, log) };
  try { return await measureCapacity(repoRoot, launcher, options); } finally { await artifact?.close(); }
}

/** 📊️ Retains the native executable owner through load measurement and the final roster command. */
async function measureCapacity(repoRoot: string, artifact: Pick<ProctorArtifact, "launch">, options: CapacityOptions): Promise<CapacityReport> {
  const work = capacityWorkDirectory(repoRoot);
  mkdirSync(work, { recursive: true });
  const data = mkdtempSync(join(work, "run-"));
  const progressFile = options.report ? `${resolve(options.report)}.progress.jsonl` : join(data, "progress.jsonl");
  mkdirSync(dirname(progressFile), { recursive: true });
  writeFileSync(progressFile, "");
  const port = await freePort();
  const base = `http://127.0.0.1:${port}`;
  const coursePath = resolve(repoRoot, ...PROCTOR_DEV_COURSE);
  const course = JSON.parse(readFileSync(coursePath, "utf8")) as { id: string; quizzes: string[] };
  const quizzes = course.quizzes.map((path) => (JSON.parse(readFileSync(resolve(dirname(coursePath), path), "utf8")) as { id: string }).id);
  const { PROCTOR_MAX_LEARNERS: _default, ...inherited } = process.env;
  const env: NodeJS.ProcessEnv = { ...inherited, PROCTOR_PORT: String(port), PROCTOR_BIND: "127.0.0.1", PROCTOR_MODE: "production", PROCTOR_ALLOWED_ORIGINS: SITE, PROCTOR_TRUSTED_FORWARDING: "proxy", PROCTOR_DATA: data, PROCTOR_COURSE: coursePath, PROCTOR_LIMIT_COMMANDS_PER_SECOND: String(LIMITS.commandsPerSecond), PROCTOR_LIMIT_COMMANDS_BURST: String(LIMITS.commandsBurst), PROCTOR_LIMIT_QUERIES_PER_SECOND: String(LIMITS.queriesPerSecond), PROCTOR_LIMIT_QUERIES_BURST: String(LIMITS.queriesBurst), PROCTOR_LIMIT_SIGNUPS_PER_HOUR: String(LIMITS.signUpsPerHour), PROCTOR_LIMIT_SIGNUPS_BURST: String(LIMITS.signUpsBurst), PROCTOR_LIMIT_SOCKETS_PER_ADDRESS: String(LIMITS.socketsPerAddress), PROCTOR_LIMIT_BODY_BYTES: String(LIMITS.bodyBytes) };
  let proctor: ProctorProcess | undefined;
  let peak: number | undefined;
  const sample = async (): Promise<number | undefined> => {
    const now = proctor?.pid === undefined ? undefined : await residentMebibytes(proctor.pid);
    if (now !== undefined) peak = Math.max(peak ?? 0, now);
    return now;
  };
  try {
    proctor = artifact.launch(["serve"], env, join(data, "proctor.log"));
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
    const hall = { base, tenant: course.id, quizzes, scale: 1 / options.compression, progressFile };

    const alone = await phase(hall, options, proctor.pid);
    console.log(table("the hall alone", alone));
    const violations = hallViolations("alone", alone, BUDGET.alone);

    let beside: PhaseReport | undefined;
    let scriptReport: ScriptReport | undefined;
    if (options.hallOnly !== true) {
      const orders: ScriptOrders = { role: "script", base, tenant: course.id, quiz: quizzes[0], bodyLimit: LIMITS.bodyBytes, socketsPerAddress: LIMITS.socketsPerAddress, progressFile };
      const worker = new Worker(new URL(import.meta.url), { workerData: orders });
      const scripted = new Promise<ScriptReport>((reported, failed) => {
        worker.once("message", reported);
        worker.once("error", failed);
      });
      await pause(1_000);
      beside = await phase(hall, options, proctor.pid);
      worker.postMessage("stop");
      capacityProgress(progressFile, "script stop requested");
      scriptReport = await scripted;
      capacityProgress(progressFile, "script report complete");
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
    const roster = await rosterOf(artifact, env, join(data, "prune.log"));
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
export async function capacityMain(repoRoot: string, invocation: ScriptInvocation, args: readonly string[]): Promise<void> {
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
  const report = await runCapacity(repoRoot, invocation, { learners: whole("--learners", CAPACITY_DEFAULTS.learners), compression: whole("--compression", CAPACITY_DEFAULTS.compression), report: value("--report"), executable: value("--executable"), hallOnly: args.includes("--hall-only") });
  if (report.violations.length > 0) {
    for (const violation of report.violations) console.error(`[ERROR] capacity: ${violation}`);
    process.exitCode = 1;
    return;
  }
  console.log(report.beside === undefined ? "[INFO] capacity: the hall alone saw no error and no refusal and stayed inside its latency budget (the script was not played: this is not the gate)" : "[INFO] capacity: the hall saw no error and no refusal and stayed inside its latency budget alone and beside the script; the script was throttled, capped and cut off, and its sign-ups left the roster far below the cap");
}

if (isMainThread && import.meta.main) await receiveScriptProcessInvocation(process.env, invocation => capacityMain(process.cwd(), invocation, process.argv.slice(2)));
if (!isMainThread && (workerData as HallOrders | ScriptOrders).role === "hall") {
  void lecture(workerData as HallOrders, new Promise((dropped) => parentPort!.on("message", (message) => (message === "drop" ? dropped() : undefined)))).then(report => parentPort!.postMessage(report), error => queueMicrotask(() => { throw error; }));
}
if (!isMainThread && (workerData as HallOrders | ScriptOrders).role === "script") {
  let stop = false;
  parentPort!.on("message", (message) => {
    if (message === "stop") { stop = true; capacityProgress((workerData as ScriptOrders).progressFile, "script received stop"); }
  });
  void script(workerData as ScriptOrders, () => stop).then(report => parentPort!.postMessage(report), error => queueMicrotask(() => { throw error; }));
}
//#endregion 🏋️Gate
