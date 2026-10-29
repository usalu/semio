import{graphMemoryAllocationV1,GRAPH_MEMORY_POLICY_V1}from"../../../../../🧠️memory/🟨️.mjs";
import {noticeLanguagesV1} from "../../../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts";
import { spawn } from "node:child_process";
import { closeSync, constants, mkdtempSync, openSync, readdirSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { serialize } from "node:v8";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import { runtimeHostIdV1 } from "../../../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🧭️selection/🟦️.ts";
import {refuseBuildV1} from "../../../../../📖️texts/🟦️.ts";

/** 🧊️ Freezes a declaration recursively. */
const freeze = value => { if (value !== null && typeof value === "object") { for (const child of Object.values(value)) freeze(child); Object.freeze(value); } return value; };

/** 🏊️ The declared limits of the graph inference farm: the schema file is the single source. */
export const GRAPH_FARM_POLICY_V1 = freeze(structuredClone(schema.$defs.Policy.const));
/** 🗣️ The bilingual texts of the farm, read from the schema file. */
export const GRAPH_FARM_MESSAGES_V1 = freeze(structuredClone(schema.$defs.Messages.const));
const P = GRAPH_FARM_POLICY_V1, MEGABYTE = 1048576;

export function selectGraphFarmLanguagesV1(environment) {
  return [...noticeLanguagesV1(P.languages, environment)];
}

/** 💬️ Renders one farm message as one line per language of the environment. */
export function renderGraphFarmMessagesV1(key, fields, environment) {
  return selectGraphFarmLanguagesV1(environment).map(language => {
    const template = GRAPH_FARM_MESSAGES_V1[language][key];
    if (template === undefined) throw Error("Closed graph farm message required: " + key);
    return template.replace(/\{([a-z]+)\}/gu, (_token, name) => String(fields[name]));
  });
}

/** 📐️ Plans how many worker processes the work can use: the explicit environment value wins, otherwise the processors left to the workers and the work there is to share bound the request; what the host capacity governor then grants decides how many run, so no constant limits the farm. */
export function planGraphFarmV1({ projects, environment = {}, cores, policy = P }) {
  const text = environment[policy.workersEnvironment]?.trim().toLowerCase();
  if (text === "0" || text === "off" || text === "false") return { workers: 0, reason: "disabled" };
  if (text !== undefined && text !== "" && text !== "auto") {
    const forced = Number(text);
    if (!Number.isSafeInteger(forced) || forced < 0 || forced > policy.maximumForcedWorkers) throw Error("Graph workers of 0 to " + policy.maximumForcedWorkers + " required: " + policy.workersEnvironment);
    return { workers: forced, reason: forced === 0 ? "disabled" : "forced" };
  }
  if (projects < policy.minimumProjects) return { workers: 0, reason: "small" };
  const bounds = [["work", Math.floor(projects / policy.projectsPerWorker)], ["cpu", cores - policy.cpuReserve]];
  const [reason, workers] = bounds.reduce((best, row) => row[1] < best[1] ? row : best);
  return workers < 2 ? { workers: 0, reason } : { workers, reason };
}

/** 🏠️ The worker pool the capacity grant this process already holds pays for: the declared number of workers (at most the jobs of the grant) with the megabytes of one job, or undefined when the process holds no grant (the capacity gate is off or this is no graph plan child). The host is still asked for more; this pool is what keeps a saturated host from leaving every project to the parent. */
export function ownGrantPoolV1(environment,policy=P){const allocation=graphMemoryAllocationV1(environment);if(!allocation.qualified||allocation.jobs<1)return undefined;const workers=Math.min(policy.ownWorkers,allocation.jobs),megabytes=Math.floor(allocation.workerMegabytes/workers);if(workers<1||megabytes<GRAPH_MEMORY_POLICY_V1.minimumBudgetMegabytes)return undefined;return{source:"own",workers,megabytes};}

/** 🪢️ Stops future ownership admission, drains accepted work and its adoption, then releases a shrunken pool. */
export async function retireOwnedGraphPoolV1(pool,next){if(next!==undefined&&next.workers>=pool.workers&&next.megabytes>=pool.megabytes)return false;pool.ready=false;const settled=await Promise.allSettled([...pool.work]);let failure;try{await pool.farm.close();}catch(error){failure=error;}finally{try{pool.release();}catch(error){failure??=error;}}const rejected=settled.find(row=>row.status==="rejected");if(rejected)throw rejected.reason;if(failure)throw failure;return true;}

/** 🎟️ The worker pool a host answer admits: any grant of at least the declared minimum starts a pool of the granted jobs; a smaller or missing answer starts none. */
export function grantedPoolV1(granted, policy = P) {
  if (granted === undefined || !Number.isSafeInteger(granted.jobs) || granted.jobs < policy.minimumGrantedWorkers) return undefined;
  return { source: "host", workers: granted.jobs, megabytes: granted.megabytes };
}

/** 🧭️ Chooses the pool that takes the next job among the pools that are ready and usable: a pool with an idle worker first (earliest), otherwise the one with the fewest jobs per worker (earliest on a tie); -1 when none can take it. */
export function choosePoolV1(pools) {
  let best = -1, ratio = Infinity;
  for (let index = 0; index < pools.length; index++) {
    const pool = pools[index];
    if (!pool.ready || !pool.usable) continue;
    if (pool.idle) return index;
    const load = pool.pending / pool.width;
    if (load < ratio) { ratio = load; best = index; }
  }
  return best;
}

/** ♻️ Whether a worker that just answered a job is replaced before the next one: `memory` when its resident set passed the declared share of its budget (a project that grew it is the last one it runs), `projects` when it served the declared number of jobs (what a long-lived process keeps for itself is bounded by its lifetime, not by the project); undefined keeps it. */
export function recycleReasonV1({ rssMegabytes, jobs, megabytes }, policy = P) {
  if (rssMegabytes > megabytes * policy.recycleMemoryRatio) return "memory";
  if (jobs >= policy.recycleAfterProjects) return "projects";
  return undefined;
}

/** 🧬️ The environment that bounds the heap of one worker process of the runtime in use; a runtime without such a switch gets none. */
export function graphFarmRuntimeEnvironmentV1(runtime, workerMegabytes, policy = P) {
  const row = policy.runtime[runtime];
  if (row === undefined) return {};
  const environment = { ...row.environment };
  if (row.heap !== undefined) { const megabytes = Math.floor(workerMegabytes * row.ramFactor); environment[row.heap.variable] = (row.heap.prefix ?? "") + String(row.heap.unit === "bytes" ? megabytes * MEGABYTE : megabytes); }
  return environment;
}

/** ⏳️ Awaits every promise of a list that may grow while it is awaited, so a loop started late is awaited as well; the first rejection rejects. */
export async function awaitGrowingV1(promises) {
  for (let seen = 0; seen < promises.length;) { const batch = promises.slice(seen); seen = promises.length; await Promise.all(batch); }
}

/** 🚦️ Names the runtime that runs this process, so its heap switches can be chosen without naming a tool in the callers. */
export function graphFarmRuntimeV1() { return runtimeHostIdV1(); }

/** 🧹️ Creates a private directory of this process for the farm (`kind`: `staging`, `image` or `session`) under the temporary directory, named with the id of this process; the first call of a process for a directory first removes the farm directories there whose process is gone, since a parent killed before its farm closed (out of memory, a budget kill, a terminated build) leaves its snapshot copy, staging and session files behind and nothing else removes them. A directory whose process still runs, whose process id answers anything but "no such process", or whose name is not one the farm writes is kept. */
export function createFarmTemporaryDirectoryV1(kind, { directory = realpathSync(tmpdir()), alive = farmProcessAliveV1, policy = P } = {}) {
  const prefix = policy.temporaryDirectories.prefixes[kind];
  if (typeof prefix !== "string") throw refuseBuildV1("invalid","graph.farm-directory-kind-required",{kind});
  if (!reclaimedDirectoriesV1.has(directory)) {
    reclaimedDirectoriesV1.add(directory);
    const pattern = new RegExp(policy.temporaryDirectories.pattern, "u");
    let names = [];
    try { names = readdirSync(directory); } catch {}
    for (const name of names) { const pid = Number(pattern.exec(name)?.groups?.pid); if (Number.isSafeInteger(pid) && pid > 0 && pid !== process.pid && !alive(pid)) { try { rmSync(join(directory, name), { recursive: true, force: true }); } catch {} } }
  }
  return mkdtempSync(join(directory, prefix + process.pid + "-"));
}

/** 🫀️ Whether a process id still names a process: only the answer "no such process" counts as gone, so a platform that cannot tell keeps the directory. */
export function farmProcessAliveV1(pid) { try { process.kill(pid, 0); return true; } catch (error) { return error?.code !== "ESRCH"; } }

const reclaimedDirectoriesV1 = new Set();

/**
 * 🧵️ A pool of worker processes, each running one service module (`open(session, where)` once, then `run(job)` per job, one at a time). Every job ends as `{ reply }`, `{ failure }` (the service threw) or `{ lost }` (the process died or timed out on every attempt), so a caller decides what a failed job means; nothing is thrown except the abort reason of the signal.
 * A lost job is sent again to a fresh process up to `maximumAttempts` times; more than `maximumRestarts` lost processes disable the pool. `accept(identity)` decides whether a worker that announced its identity may serve; a refusal disables the pool. A worker that grew past its memory budget is replaced after its job; `retired(slot, generation)` tells the caller that a process is gone, so it can reclaim what that process left.
 * With `auxiliaryFile` every worker reads that private regular file as descriptor 3 (opened read-only and without following links for each start, so no two workers share a file offset; a file is read in one pass, a pipe needs this process scheduled for every chunk) and the channel moves to descriptor 4.
 */
export function createFarmV1({ entry, service, session, workers, megabytes, environment = process.env, signal, policy = P, accept = () => true, report = () => {}, retired = () => {}, spawnProcess = spawn, runtime = graphFarmRuntimeV1(), command = process.execPath, commandArguments = [], auxiliaryFile = undefined }) {
  if (!Number.isSafeInteger(workers) || workers < 1) throw Error("Graph farm of at least one worker required");
  const slots = Array.from({ length: workers }, (_, index) => ({ index, generation: 0, child: undefined, state: "empty", job: undefined, timer: undefined })), queue = [];
  const stats = { workers, jobs: 0, replies: 0, failures: 0, lost: 0, recycled: 0, started: 0, peakRssMegabytes: 0, queuedMilliseconds: 0, workedMilliseconds: 0, bootMilliseconds: 0 };
  let nextId = 0, closed = false, disabled, restarts = 0, sessionDirectory, sessionFile, readyAccept;
  const ready = new Promise(done => { readyAccept = done; });
  const childEnvironment = { ...environment, ...graphFarmRuntimeEnvironmentV1(runtime, megabytes, policy), SEMIO_GRAPH_WORKER: "1" };
  delete childEnvironment[policy.workersEnvironment];

  const settle = (job, value) => { if (job.settled) return; job.settled = true; clearTimeout(job.timer); job.resolve(value); };
  const retire = (slot, graceful) => {
    const child = slot.child, generation = slot.generation;
    clearTimeout(slot.timer);
    slot.child = undefined; slot.state = "empty";
    if (!child) return;
    child.removeAllListeners("message");
    if (graceful && child.connected) { try { child.send({ type: "shutdown" }); } catch {} setTimeout(() => { try { child.kill("SIGKILL"); } catch {} }, policy.shutdownGraceMilliseconds).unref?.(); }
    else { try { child.kill("SIGKILL"); } catch {} }
    if (child.exitCode !== null || child.signalCode !== null) retired(slot.index, generation);
    else child.once("exit", () => retired(slot.index, generation));
  };
  const lose = (slot, why) => {
    const job = slot.job;
    slot.job = undefined;
    retire(slot, false);
    stats.lost++;
    report("lost", { reason: why });
    if (job) { clearTimeout(job.timer); if (job.attempts >= policy.maximumAttempts) settle(job, { lost: why }); else queue.unshift(job); }
    if (++restarts > policy.maximumRestarts) disable("restarts: " + why);
    pump();
  };
  const disable = why => {
    if (disabled !== undefined) return;
    disabled = why;
    report("disabled", { reason: why });
    readyAccept(false);
    for (const slot of slots) { const job = slot.job; slot.job = undefined; retire(slot, true); if (job) settle(job, { lost: "disabled: " + why }); }
    for (const job of queue.splice(0)) settle(job, { lost: "disabled: " + why });
  };
  const launch = slot => {
    slot.state = "starting"; slot.generation++; slot.served = 0; stats.started++; slot.spawned = performance.now();
    if (sessionFile === undefined) { sessionDirectory = createFarmTemporaryDirectoryV1("session", { policy }); sessionFile = join(sessionDirectory, "session.bin"); writeFileSync(sessionFile, serialize(session)); }
    let child, descriptor;
    try { descriptor = auxiliaryFile === undefined ? undefined : openSync(auxiliaryFile, constants.O_RDONLY | (constants.O_NOFOLLOW ?? 0)); child = spawnProcess(command, [...commandArguments, entry], { stdio: descriptor === undefined ? ["ignore", "inherit", "inherit", "ipc"] : ["ignore", "inherit", "inherit", descriptor, "ipc"], serialization: "json", env: childEnvironment }); }
    catch (error) { lose(slot, String(error?.message ?? error)); return; }
    finally { if (descriptor !== undefined) closeSync(descriptor); }
    slot.child = child;
    slot.timer = setTimeout(() => lose(slot, "init timeout"), policy.initTimeoutMilliseconds);
    slot.timer.unref?.();
    child.on("message", message => {
      if (slot.child !== child) return;
      if (message?.type === "ready") {
        clearTimeout(slot.timer);
        if (!accept(message.identity)) { disable("identity differs"); return; }
        slot.state = "idle"; stats.bootMilliseconds += performance.now() - slot.spawned; readyAccept(true); pump();
      } else if (message?.type === "fatal") lose(slot, message.message);
      else if (message?.type === "result" || message?.type === "failure") {
        const job = slot.job;
        if (!job || job.id !== message.id) return;
        slot.job = undefined; slot.state = "idle";
        clearTimeout(job.timer);
        if (message.type === "result") {
          stats.replies++; stats.workedMilliseconds += performance.now() - job.dispatched; stats.peakRssMegabytes = Math.max(stats.peakRssMegabytes, message.rssMegabytes);
          settle(job, { reply: message.reply, slot: slot.index, generation: slot.generation });
          slot.served++;
          const reason = recycleReasonV1({ rssMegabytes: message.rssMegabytes, jobs: slot.served, megabytes }, policy);
          if (reason !== undefined) { stats.recycled++; report("recycled", { worker: slot.index, megabytes: Math.round(message.rssMegabytes), reason }); retire(slot, true); }
        } else { stats.failures++; settle(job, { failure: { name: message.name, message: message.message, code: message.code ?? null } }); }
        pump();
      }
    });
    child.once("exit", () => { if (slot.child === child) lose(slot, "exited"); });
    child.once("error", error => { if (slot.child === child) lose(slot, String(error?.message ?? error)); });
    try { child.send({ type: "init", service, sessionFile, slot: slot.index, generation: slot.generation }); } catch (error) { lose(slot, String(error?.message ?? error)); }
  };
  const pump = () => {
    if (closed || disabled !== undefined) return;
    for (const slot of slots) {
      if (!queue.length) break;
      if (slot.state !== "idle") continue;
      const job = queue.shift();
      if (job.settled) continue;
      job.attempts++; job.dispatched = performance.now(); if (job.attempts === 1) stats.queuedMilliseconds += job.dispatched - job.queued;
      slot.state = "busy"; slot.job = job;
      job.timer = setTimeout(() => lose(slot, "job timeout"), policy.jobTimeoutMilliseconds);
      job.timer.unref?.();
      try { const jobFile = join(sessionDirectory, "job-" + job.id + ".json"); writeFileSync(jobFile, JSON.stringify(job.payload)); slot.child.send({ type: "run", id: job.id, jobFile }); } catch (error) { lose(slot, String(error?.message ?? error)); }
    }
    let waiting = queue.length - slots.filter(slot => slot.state === "starting").length;
    for (const slot of slots) if (waiting > 0 && slot.state === "empty") { launch(slot); waiting--; }
  };
  const close = async () => {
    if (closed) return;
    closed = true;
    signal?.removeEventListener("abort", abort);
    readyAccept(false);
    for (const job of queue.splice(0)) settle(job, { lost: "closed" });
    const exits = [];
    for (const slot of slots) {
      const child = slot.child, job = slot.job;
      slot.job = undefined;
      if (job) settle(job, { lost: "closed" });
      if (child) exits.push(new Promise(done => { child.once("exit", done); setTimeout(done, policy.shutdownGraceMilliseconds + 500).unref?.(); }));
      retire(slot, true);
    }
    await Promise.all(exits);
    if (sessionDirectory !== undefined) rmSync(sessionDirectory, { recursive: true, force: true });
  };
  const abort = () => { void close(); };
  if (signal) { if (signal.aborted) abort(); else signal.addEventListener("abort", abort, { once: true }); }
  for (const slot of slots) launch(slot);
  return Object.freeze({
    width: workers,
    ready,
    get idle() { return slots.some(slot => slot.state === "idle"); },
    get usable() { return !closed && disabled === undefined; },
    get pending() { return queue.length + slots.filter(slot => slot.state === "busy").length; },
    stats: () => ({ ...stats, queuedMilliseconds: Math.round(stats.queuedMilliseconds), workedMilliseconds: Math.round(stats.workedMilliseconds), bootMilliseconds: Math.round(stats.bootMilliseconds), restarts, ...(disabled === undefined ? {} : { disabled }) }),
    run(payload, options = {}) {
      if (closed || disabled !== undefined) return Promise.resolve({ lost: closed ? "closed" : "disabled: " + disabled });
      options.signal?.throwIfAborted();
      stats.jobs++;
      return new Promise(resolve => {
        const job = { id: nextId++, payload, attempts: 0, resolve, settled: false, timer: undefined, queued: performance.now(), dispatched: 0 };
        queue.push(job);
        pump();
      });
    },
    close,
  });
}
