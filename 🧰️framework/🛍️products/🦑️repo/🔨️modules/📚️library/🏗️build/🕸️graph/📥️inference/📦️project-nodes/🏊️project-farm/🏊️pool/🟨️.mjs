import{graphMemoryAllocationV1,GRAPH_MEMORY_POLICY_V1,hostMemoryMegabytesV1}from"../../../../../🧠️memory/🟨️.mjs";
import { createBoundedCacheV1 } from "../../../../../🧊️bounded/🟨️.mjs";
import { readProcessFootprintBytesV1 } from "../../../../../🧠️memory/🦶️footprint/🟨️.mjs";
import { PLAN_STAGE_POLICY_V1, farmAloneCapV1 } from "../../../../../🧠️memory/🪜️plan-stage/🟨️.mjs";
import {noticeLanguagesV1} from "../../../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts";
import { spawn } from "node:child_process";
import { closeSync, mkdtempSync, readdirSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { getPriority as osPriorityV1, setPriority as setOsPriorityV1, tmpdir } from "node:os";
import { join } from "node:path";
import { serialize } from "node:v8";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import { runtimeHostIdV1 } from "../../../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🧭️selection/🟦️.ts";
import {refuseBuildV1} from "../../../../../📖️texts/🟦️.ts";
import { MACHINE_SPEED_POLICY_V1, localMachineSpeedFactorV1, readProcessCpuMillisecondsV1, scaledMillisecondsV1 } from "../../../../../🐢️speed/🟨️.mjs";

/** 🧊️ Freezes a declaration recursively. */
const freeze = value => { if (value !== null && typeof value === "object") { for (const child of Object.values(value)) freeze(child); Object.freeze(value); } return value; };

/** 🏊️ The declared limits of the graph inference farm: the schema file is the single source. */
export const GRAPH_FARM_POLICY_V1 = freeze(structuredClone(schema.$defs.Policy.const));
/** 🗣️ The bilingual texts of the farm, read from the schema file. */
export const GRAPH_FARM_MESSAGES_V1 = freeze(structuredClone(schema.$defs.Messages.const));
const P = GRAPH_FARM_POLICY_V1, MEGABYTE = 1048576;

/** ⏱️ What a job is expected to cost in a farm child, in milliseconds, from its kind and the cost table: a look of up to six owners, a verification, a warm settling, or an inference that grows with the megabytes the cost table knows for its owner (`costMegabytes`). The one source of the order of the queue.
 * @see ./🧪️tests/⏱️queue-order */
export function farmJobExpectedMillisecondsV1(payload, policy = P) {
  const q = policy.queue ?? P.queue, known = Number.isFinite(payload?.costMegabytes) && payload.costMegabytes > 0 ? payload.costMegabytes : 0;
  if (payload?.look === true) return q.lookMilliseconds * Math.max(1, Array.isArray(payload.owners) ? payload.owners.length : 1);
  if (payload?.warm === true) return q.warmMilliseconds;
  if (payload?.observe === true) return q.observeMilliseconds;
  return q.inferMilliseconds + q.perMegabyteMilliseconds * known;
}

/** 🥇 The queued jobs in the order the farm serves them: the shortest expected job first, every millisecond a job has waited counting `agingPerMille` thousandths of a millisecond against its expected cost, so no long owner waits longer than about its own cost; a job that was sent back (`front`: lost, or capped to run alone) stays in front. Equal scores keep their arrival order.
 * @see ./🧪️tests/⏱️queue-order */
export function farmQueueOrderV1(jobs, now, policy = P) {
  const q = policy.queue ?? P.queue, score = job => job.front === true ? -Infinity : job.expected - (now - job.queued) * q.agingPerMille / 1000;
  return jobs.map((job, index) => ({ job, index, score: score(job) })).sort((a, b) => a.score === b.score ? a.index - b.index : a.score < b.score ? -1 : 1).map(row => row.job);
}

/** ⏫️ Claims the priority class of the read farm (`priority.readFarm`, normal) for one child: a child that starts below it is set to it, which an unprivileged process may only do where the platform lets it raise priority (Windows); where the raise is refused (macOS and Linux, a niceness only the superuser lowers) the child keeps what it inherited from its starter and the result says so, so the farm can tell the person who started the read instead of staying silently slow.
 * @see ./🧬️schema/🔣️.json the declared classes */
export function claimFarmChildPriorityV1(pid, { get = osPriorityV1, set = setOsPriorityV1 } = {}, wanted = P.priority.readFarm) {
  const none = { before: undefined, after: undefined, raised: false, inherited: false };
  if (pid === undefined) return none;
  let before; try { before = get(pid); } catch { return none; }
  if (before <= wanted) return { before, after: before, raised: false, inherited: false };
  try { set(pid, wanted); } catch {}
  let after = before; try { after = get(pid); } catch {}
  return { before, after, raised: after < before, inherited: after > wanted };
}

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

/** 🚨️ The typed refusal of a project that grew a worker process past its footprint cap even when it ran alone: it names the owner and carries both measures in numbers, and its message is the bilingual line of the farm texts. */
export class GraphFarmOverrunErrorV1 extends Error {
  constructor({ project, megabytes, capMegabytes }, environment = process.env) {
    super(renderGraphFarmMessagesV1("overrun", { project, megabytes, cap: capMegabytes }, environment).join("\n"));
    this.name = "GraphFarmOverrunErrorV1"; this.code = "graph.farm-overrun"; this.project = project; this.megabytes = megabytes; this.capMegabytes = capMegabytes;
  }
}

/** 📐️ Plans how many worker processes the work can use: the explicit environment value wins, otherwise the processors left to the workers, the work there is to share and the memory the plan may add (`maximumWorkers`) bound the request; what the host capacity governor then grants decides how many run, so no constant limits the farm. */
export function planGraphFarmV1({ projects, environment = {}, cores, policy = P, maximumWorkers = Infinity, minimumWorkers = 2 }) {
  const text = environment[policy.workersEnvironment]?.trim().toLowerCase();
  if (text === "0" || text === "off" || text === "false") return { workers: 0, reason: "disabled" };
  if (text !== undefined && text !== "" && text !== "auto") {
    const forced = Number(text);
    if (!Number.isSafeInteger(forced) || forced < 0 || forced > policy.maximumForcedWorkers) throw Error("Graph workers of 0 to " + policy.maximumForcedWorkers + " required: " + policy.workersEnvironment);
    return { workers: forced, reason: forced === 0 ? "disabled" : "forced" };
  }
  if (projects < policy.minimumProjects) return { workers: 0, reason: "small" };
  const bounds = [["work", Math.floor(projects / policy.projectsPerWorker)], ["cpu", cores - policy.cpuReserve], ["memory", maximumWorkers]];
  const [reason, workers] = bounds.reduce((best, row) => row[1] < best[1] ? row : best);
  return workers < minimumWorkers ? { workers: 0, reason } : { workers, reason };
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

/** 🧭️ Chooses the pool that takes the next job among the pools that are ready and usable: a pool with a dispatchable idle worker first (earliest), otherwise the one with the fewest jobs per worker (earliest on a tie); -1 when none can take it. */
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

/** ♻️ Whether a worker that just answered a job is replaced before the next one: `memory` when the memory it holds (its physical footprint read by its parent, its resident set where no footprint can be read: the resident set also counts pages the worker freed and the host has not taken back, 1.5 to 2.1 GB for a worker that physically held 300 MB) passed the declared share of its budget (a project that grew it is the last one it runs), `projects` when it served the declared number of jobs (what a long-lived process keeps for itself is bounded by its lifetime, not by the project); undefined keeps it. */
export function recycleReasonV1({ heldMegabytes, baselineMegabytes, jobs, megabytes }, policy = P) {
  const ceiling = Math.max(megabytes, Number.isFinite(baselineMegabytes) && baselineMegabytes > 0 ? baselineMegabytes * policy.recycleCeilingFactor : 0);
  if (heldMegabytes > ceiling || heldMegabytes > megabytes * policy.recycleMemoryRatio && (!Number.isFinite(baselineMegabytes) || heldMegabytes > baselineMegabytes * (1 + policy.recycleGrowthRatio))) return "memory";
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
    reclaimedDirectoriesV1.set(directory, true);
    const pattern = new RegExp(policy.temporaryDirectories.pattern, "u");
    let names = [];
    try { names = readdirSync(directory); } catch {}
    for (const name of names) { const pid = Number(pattern.exec(name)?.groups?.pid); if (Number.isSafeInteger(pid) && pid > 0 && pid !== process.pid && !alive(pid)) { try { rmSync(join(directory, name), { recursive: true, force: true }); } catch {} } }
  }
  return mkdtempSync(join(directory, prefix + process.pid + "-"));
}

/** 🫀️ Whether a process id still names a process: only the answer "no such process" counts as gone, so a platform that cannot tell keeps the directory. */
export function farmProcessAliveV1(pid) { try { process.kill(pid, 0); return true; } catch (error) { return error?.code !== "ESRCH"; } }

const liveChildrenV1 = createBoundedCacheV1({ name: "inference:live-children", maxEntries: 4096, maxBytes: 4096 });
let exitHookedV1 = false;

/** 🪦️ Remembers a worker process until it is gone and, once per process, kills the ones still held when this process exits, so no worker outlives its parent. */
function holdForExitV1(child) {
  if (typeof child?.kill !== "function") return;
  liveChildrenV1.set(child, child);
  child.once?.("exit", () => liveChildrenV1.delete(child));
  if (exitHookedV1) return;
  exitHookedV1 = true;
  process.on("exit", () => { for (const each of liveChildrenV1.keys()) { try { each.kill("SIGKILL"); } catch {} } });
}

const reclaimedDirectoriesV1 = createBoundedCacheV1({ name: "inference:reclaimed-directories", maxEntries: 256, maxBytes: 256 });

/**
 * 🧵️ A pool of worker processes, each running one service module (`open(session, where)` once, then `run(job)` per job, one at a time). Every job ends as `{ reply }`, `{ failure }` (the service threw) or `{ lost }` (the process died or timed out on every attempt), so a caller decides what a failed job means; nothing is thrown except the abort reason of the signal.
 * A lost job is sent again to a fresh process up to `maximumAttempts` times; more than `maximumRestarts` lost processes disable the pool. `accept(identity)` decides whether a worker that announced its identity may serve; a refusal disables the pool. A worker that grew past its memory budget is replaced after its job; `retired(slot, generation)` tells the caller that a process is gone, so it can reclaim what that process left.
 * Every worker that runs a job is watched: its physical footprint (`footprintOf`, macOS `footprint`, Linux `smaps_rollup`) is read every `footprintMilliseconds`, and a worker above `capMegabytes` (default: `farmChildFootprintMegabytes` of the plan stage policy, the one source of the width and the caps of the farm) is stopped (SIGTERM, SIGKILL after `terminateGraceMilliseconds`; only this pool's own child) and its job goes back to the head of the queue to run ALONE: no job beside it and no further worker is started while it waits or runs. A job that passes `aloneCapMegabytes` even alone settles as `{ overrun }`, which the caller turns into a typed refusal; a capped worker is no lost worker (no attempt, no restart count).
 * With `auxiliaryFile` every worker opens that private regular file itself, named by `SEMIO_GRAPH_WORKER_SNAPSHOT`, so no two workers share a file offset and none needs this process scheduled to read it. A worker starts from its environment (`initEnvironment`, schema `Start`): it needs no message of this process, so a busy event loop of the parent never delays it. A process that ends takes every worker with it: they end when their channel closes, and this process kills the workers it still holds when it exits.
 */
export function createFarmV1({ entry, service, session, workers, megabytes, environment = process.env, signal, policy = P, accept = () => true, report = () => {}, retired = () => {}, priority = {}, spawnProcess = spawn, runtime = graphFarmRuntimeV1(), command = process.execPath, commandArguments = [], auxiliaryFile = undefined, hold = () => false, footprintOf = readProcessFootprintBytesV1, capMegabytes = PLAN_STAGE_POLICY_V1.farmChildFootprintMegabytes["project-farm"].capMegabytes, aloneCapMegabytes = farmAloneCapV1(hostMemoryMegabytesV1()), limitMegabytes = Infinity, speed = localMachineSpeedFactorV1(environment), cpuOf = readProcessCpuMillisecondsV1, liveness = MACHINE_SPEED_POLICY_V1 }) {
  if (!Number.isSafeInteger(workers) || workers < 1) throw Error("Graph farm of at least one worker required");
  let priorityReported = false;
  const retiring = new Set(), judging = new Set();
  const slots = Array.from({ length: workers }, (_, index) => ({ index, generation: 0, child: undefined, state: "empty", job: undefined, timer: undefined })), queue = [];
  const stats = { workers, jobs: 0, replies: 0, failures: 0, lost: 0, recycled: 0, capped: 0, overruns: 0, started: 0, peakRssMegabytes: 0, peakFootprintMegabytes: 0, queuedMilliseconds: 0, workedMilliseconds: 0, bootMilliseconds: 0 };
  let nextId = 0, watcher, closed = false, disabled, restarts = 0, sessionDirectory, sessionFile, readyAccept, closeWork, referenced = true, lastLoss, accepted = 0;
  const holdChild = child => { try { if (referenced) { child.ref?.(); child.channel?.ref?.(); } else { child.unref?.(); child.channel?.unref?.(); } } catch {} };
  const ready = new Promise(done => { readyAccept = done; });
  const childEnvironment = { ...environment, ...graphFarmRuntimeEnvironmentV1(runtime, megabytes, policy), SEMIO_GRAPH_WORKER: "1", ...(auxiliaryFile === undefined ? {} : { SEMIO_GRAPH_WORKER_SNAPSHOT: auxiliaryFile }) };
  delete childEnvironment[policy.workersEnvironment];

  const settle = (job, value) => { if (job.settled) return; job.settled = true; clearTimeout(job.timer); job.release?.(); job.resolve(value); };
  const retire = (slot, graceful) => {
    const child = slot.child, generation = slot.generation;
    clearTimeout(slot.timer);
    slot.measurement?.abort();
    slot.cpuMeasurement?.abort();
    slot.child = undefined;
    if (!child) { if (slot.state !== "retiring") slot.state = "empty"; return; }
    slot.state = "retiring";
    child.removeAllListeners("message");
    let finish, timer, finished = false;
    const exited = new Promise(done => { finish = done; });
    retiring.add(exited);
    const done = () => {
      if (finished) return;
      finished = true;
      clearTimeout(timer);
      child.removeListener("exit", done); child.removeListener("close", done);
      retiring.delete(exited);
      if (slot.generation === generation && slot.state === "retiring") slot.state = "empty";
      try { retired(slot.index, generation); } finally { finish(); pump(); }
    };
    if (child.exitCode !== null || child.signalCode !== null) { done(); return; }
    child.once("exit", done); child.once("close", done);
    if (graceful === "terminate") {
      timer = setTimeout(() => { try { child.kill("SIGKILL"); } catch {} }, policy.terminateGraceMilliseconds);
      timer.unref?.();
      try { child.kill("SIGTERM"); } catch {}
    } else if (graceful && child.connected) {
      timer = setTimeout(() => { try { child.kill("SIGKILL"); } catch {} }, policy.shutdownGraceMilliseconds);
      timer.unref?.();
      try { child.send({ type: "shutdown" }); } catch {}
    } else { try { child.kill("SIGKILL"); } catch {} }
  };
  const lose = (slot, why) => {
    lastLoss = why;
    const job = slot.job;
    slot.job = undefined;
    retire(slot, false);
    stats.lost++;
    report("lost", { reason: why });
    if (job) { clearTimeout(job.timer); if (job.attempts >= policy.maximumAttempts) settle(job, { lost: why }); else { job.front = true; queue.unshift(job); } }
    if (++restarts > policy.maximumRestarts) disable("restarts: " + why);
    pump();
  };
  const refuse = (slot, why) => {
    slot.refused = true;
    report("refused", { worker: slot.index, reason: why });
    retire(slot, false);
    if (slots.every(each => each.refused === true)) disable(why);
  };
  const disable = why => {
    if (disabled !== undefined) return;
    disabled = why;
    clearInterval(watcher); watcher = undefined;
    report("disabled", { reason: why });
    readyAccept(false);
    for (const slot of slots) { const job = slot.job; slot.job = undefined; retire(slot, true); if (job) settle(job, { lost: "disabled: " + why }); }
    for (const job of queue.splice(0)) settle(job, { lost: "disabled: " + why });
  };
  const capOf = job => job.alone === true ? aloneCapMegabytes : capMegabytes;
  const learned = new Map();
  const knownOf = job => { const value = job?.payload?.knownMegabytes, declared = Number.isFinite(value) && value > 0 ? value : 0; return Math.max(declared, learned.get(job?.payload?.name) ?? 0); };
  const runningKnown = () => slots.reduce((sum, slot) => sum + (slot.state === "busy" ? knownOf(slot.job) : 0), 0);
  const crowded = busy => busy > 0 && queue.length > 0 && runningKnown() + knownOf(queue[0]) > limitMegabytes;
  const overrun = (slot, megabytes) => {
    const job = slot.job;
    if (!job) return;
    const cap = capOf(job), alone = job.alone === true, project = job.payload?.name;
    slot.job = undefined;
    clearTimeout(job.timer);
    stats.capped++;
    report("capped", { worker: slot.index, project, megabytes: Math.round(megabytes), cap: Math.round(cap), alone });
    retire(slot, "terminate");
    if (alone) { stats.overruns++; settle(job, { overrun: { project, megabytes: Math.round(megabytes), capMegabytes: Math.round(cap) } }); }
    else { job.alone = true; job.front = true; job.attempts--; queue.unshift(job); }
    pump();
  };
  const armJob = (slot, job) => {
    clearTimeout(job.timer);
    job.timer = setTimeout(() => lose(slot, "job timeout"), scaledMillisecondsV1(policy.jobTimeoutMilliseconds, speed));
    job.timer.unref?.();
  };
  const breathe = (slot, job, child) => {
    const now = performance.now();
    if (slot.cpuMeasurement || now - (job.livenessAt ?? -Infinity) < liveness.livenessMilliseconds) return;
    job.livenessAt = now;
    const measurement = new AbortController(), { signal: sampling } = measurement;
    slot.cpuMeasurement = measurement;
    Promise.resolve().then(() => sampling.aborted ? undefined : cpuOf(child.pid, { signal: sampling })).then(milliseconds => {
      if (sampling.aborted || slot.job !== job || slot.child !== child || !Number.isFinite(milliseconds)) return;
      if (job.cpuMilliseconds !== undefined && milliseconds - job.cpuMilliseconds >= liveness.livenessCpuMilliseconds) armJob(slot, job);
      job.cpuMilliseconds = milliseconds;
    }, () => {}).finally(() => { slot.cpuMeasurement = undefined; });
  };
  const measure = () => {
    if (closed || disabled !== undefined || !slots.some(slot => slot.state === "busy")) { clearInterval(watcher); watcher = undefined; return; }
    for (const slot of slots) {
      const job = slot.job, child = slot.child;
      if (slot.state !== "busy" || !job || !child?.pid) continue;
      breathe(slot, job, child);
      if (slot.reading) continue;
      slot.reading = true;
      Promise.resolve().then(() => footprintOf(child.pid)).then(bytes => {
        slot.reading = false;
        if (slot.job !== job || slot.child !== child || !Number.isFinite(bytes) || bytes < 0) return;
        const megabytes = bytes / MEGABYTE;
        job.footprint = Math.max(job.footprint ?? 0, megabytes);
        if (job.payload?.name !== undefined) learned.set(job.payload.name, Math.max(learned.get(job.payload.name) ?? 0, megabytes));
        stats.peakFootprintMegabytes = Math.max(stats.peakFootprintMegabytes, megabytes);
        if (megabytes > capOf(job)) overrun(slot, megabytes);
      }, () => { slot.reading = false; });
    }
  };
  const watch = () => { if (watcher === undefined && !closed && disabled === undefined) { watcher = setInterval(measure, policy.footprintMilliseconds); watcher.unref?.(); } };
  const launch = slot => {
    slot.state = "starting"; slot.generation++; slot.served = 0; slot.baseline = undefined; stats.started++; slot.spawned = performance.now();
    if (sessionFile === undefined) { sessionDirectory = createFarmTemporaryDirectoryV1("session", { policy }); sessionFile = join(sessionDirectory, "session.bin"); writeFileSync(sessionFile, serialize(session)); }
    let child, descriptor;
    try { descriptor = undefined; child = spawnProcess(command, [...commandArguments, entry], { stdio: ["ignore", "inherit", "inherit", "ipc"], serialization: "json", windowsHide: true, env: { ...childEnvironment, [policy.initEnvironment]: JSON.stringify({ service, sessionFile, slot: slot.index, generation: slot.generation, ...(Number.isFinite(megabytes) && megabytes >= 1 ? { megabytes: Math.floor(megabytes) } : {}) }) } }); }
    catch (error) { lose(slot, String(error?.message ?? error)); return; }
    finally { if (descriptor !== undefined) closeSync(descriptor); }
    slot.child = child;
    const claimed = claimFarmChildPriorityV1(child.pid, priority);
    if (claimed.inherited && !priorityReported) { priorityReported = true; report("priority", { nice: claimed.after, wanted: P.priority.readFarm }); }
    holdChild(child);
    holdForExitV1(child);
    slot.timer = setTimeout(() => lose(slot, "init timeout"), scaledMillisecondsV1(policy.initTimeoutMilliseconds, speed));
    child.on("message", message => {
      if (slot.child !== child) return;
      if (message?.type === "ready") {
        clearTimeout(slot.timer);
        if (!accept(message.identity)) { if (accepted === 0) disable("identity differs"); else refuse(slot, "identity differs"); return; }
        accepted++;
        slot.state = "idle"; stats.bootMilliseconds += performance.now() - slot.spawned; readyAccept(true); pump();
      } else if (message?.type === "fatal") lose(slot, message.message);
      else if (message?.type === "result" || message?.type === "failure") {
        const job = slot.job;
        if (!job || job.id !== message.id) return;
        slot.cpuMeasurement?.abort();
        slot.job = undefined; slot.state = "settling";
        clearTimeout(job.timer);
        const result = message.type === "result", first = slot.served === 0 && result, weight = result && typeof message.reply?.weight === "number" ? message.reply.weight : 1;
        const conclude = held => {
          if (result) {
            stats.replies++; restarts = 0;
            settle(job, { reply: message.reply, slot: slot.index, generation: slot.generation, footprintMegabytes: Math.round(job.footprint ?? 0) });
          } else { stats.failures++; settle(job, { failure: { name: message.name, message: message.message, code: message.code ?? null } }); }
          stats.workedMilliseconds += performance.now() - job.dispatched;
          stats.peakRssMegabytes = Math.max(stats.peakRssMegabytes, message.rssMegabytes);
          if (slot.child !== child || slot.state !== "settling") return;
          slot.served += weight; slot.state = "idle";
          const megabytesHeld = held ?? message.rssMegabytes;
          if (first) slot.baseline = megabytesHeld;
          const reason = recycleReasonV1({ heldMegabytes: megabytesHeld, baselineMegabytes: slot.baseline, jobs: slot.served, megabytes }, policy);
          if (reason !== undefined) { stats.recycled++; report("recycled", { worker: slot.index, megabytes: Math.round(megabytesHeld), reason }); retire(slot, true); }
          pump();
        };
        const suspect = recycleReasonV1({ heldMegabytes: message.rssMegabytes, baselineMegabytes: slot.baseline, jobs: slot.served + weight, megabytes }, policy) === "memory";
        if (!child.pid || !(first || suspect || weight >= 1)) { conclude(undefined); return; }
        const controller = new AbortController();
        const { signal: sampling } = controller;
        slot.measurement = controller;
        const deadline = setTimeout(() => controller.abort(), policy.settleFootprintMilliseconds);
        deadline.unref?.();
        const measured = new Promise(resolve => {
          const finish = bytes => {
            sampling.removeEventListener("abort", cancel);
            resolve(Number.isFinite(bytes) && bytes >= 0 ? bytes / MEGABYTE : undefined);
          };
          const cancel = () => finish(undefined);
          sampling.addEventListener("abort", cancel, { once: true });
          Promise.resolve().then(() => sampling.aborted ? undefined : footprintOf(child.pid, { signal: sampling })).then(finish, cancel);
        });
        const judged = measured.then(conclude).finally(() => {
          clearTimeout(deadline);
          if (slot.measurement === controller) slot.measurement = undefined;
          judging.delete(judged);
        });
        judging.add(judged);
      }
    });
    child.once("exit", () => { if (slot.child === child) lose(slot, "exited"); });
    child.once("error", error => { if (slot.child === child) lose(slot, String(error?.message ?? error)); });
    
  };
  let recheck, lastHeld = false;
  const requiresFreshSlot = slot => queue[0]?.alone === true && slot.served > 0;
  const dispatchBlocked = (busy, soloRunning, held) => soloRunning || busy > 0 && (held || queue[0]?.alone === true || crowded(busy));
  const pump = () => {
    if (closed || disabled !== undefined) return;
    if (queue.length > 1) queue.splice(0, queue.length, ...farmQueueOrderV1(queue, performance.now(), policy));
    let held = false;
    try { held = hold() === true; } catch {}
    lastHeld = held;
    let busy = slots.filter(slot => slot.state === "busy").length, soloRunning = slots.some(slot => slot.state === "busy" && slot.job?.alone === true);
    for (const slot of slots) {
      if (!queue.length) break;
      if (slot.state !== "idle") continue;
      if (requiresFreshSlot(slot)) { retire(slot, true); continue; }
      if (dispatchBlocked(busy, soloRunning, held)) break;
      const job = queue.shift();
      if (job.settled) continue;
      busy++;
      job.attempts++; job.dispatched = performance.now(); if (!job.begun) { job.begun = true; stats.queuedMilliseconds += job.dispatched - job.queued; }
      slot.state = "busy"; slot.job = job;
      armJob(slot, job);
      try { const jobFile = join(sessionDirectory, "job-" + job.id + ".json"); writeFileSync(jobFile, JSON.stringify(job.payload)); slot.child.send({ type: "run", id: job.id, jobFile }); } catch (error) { lose(slot, String(error?.message ?? error)); continue; }
      watch();
      if (job.alone === true) { soloRunning = true; break; }
    }
    const solo = soloRunning || queue[0]?.alone === true || crowded(busy);
    let waiting = queue.length - slots.filter(slot => slot.state === "starting").length;
    for (const slot of slots) if (waiting > 0 && slot.state === "empty" && slot.refused !== true && !((held || solo) && slots.some(other => other !== slot && other.state !== "empty"))) { launch(slot); waiting--; }
    if (held && queue.length && recheck === undefined) { recheck = setTimeout(() => { recheck = undefined; pump(); }, policy.holdRecheckMilliseconds); recheck.unref?.(); }
  };
  const close = () => {
    if (closeWork !== undefined) return closeWork;
    closed = true;
    clearInterval(watcher); watcher = undefined;
    closeWork = (async () => {
      signal?.removeEventListener("abort", abort);
      readyAccept(false);
      for (const slot of slots) {
        slot.measurement?.abort();
        slot.cpuMeasurement?.abort();
      }
      await Promise.all([...judging]);
      for (const job of queue.splice(0)) settle(job, { lost: "closed" });
      for (const slot of slots) {
        const job = slot.job;
        slot.job = undefined;
        if (job) settle(job, { lost: "closed" });
        retire(slot, true);
      }
      await Promise.all([...retiring]);
      if (sessionDirectory !== undefined) rmSync(sessionDirectory, { recursive: true, force: true });
    })();
    return closeWork;
  };
  const abort = () => { void close(); };
  if (signal) { if (signal.aborted) abort(); else signal.addEventListener("abort", abort, { once: true }); }
  for (const slot of slots) launch(slot);
  if (disabled === undefined && slots.every(slot => slot.child === undefined)) disable("start: " + (lastLoss ?? "no process"));
  return Object.freeze({
    width: workers,
    ready,
    get idle() {
      if (!slots.some(slot => slot.state === "idle" && !requiresFreshSlot(slot))) return false;
      const busy = slots.filter(slot => slot.state === "busy");
      return !dispatchBlocked(busy.length, busy.some(slot => slot.job?.alone === true), lastHeld);
    },
    get usable() { return !closed && disabled === undefined; },
    get pending() { return queue.length + slots.filter(slot => slot.state === "busy" || slot.state === "settling").length; },
    /** 🔭️ The live slot counts, including final memory measurements and retiring workers, for the parked-owner report. */
    get snapshot() { const count = state => slots.filter(slot => slot.state === state).length; return { busy: count("busy"), idle: count("idle"), starting: count("starting"), empty: count("empty"), settling: count("settling"), retiring: count("retiring"), refused: slots.filter(slot => slot.refused === true).length, queued: queue.length, alone: slots.some(slot => slot.state === "busy" && slot.job?.alone === true), restarts }; },
    stats: () => ({ ...stats, queuedMilliseconds: Math.round(stats.queuedMilliseconds), workedMilliseconds: Math.round(stats.workedMilliseconds), bootMilliseconds: Math.round(stats.bootMilliseconds), restarts, ...(disabled === undefined ? {} : { disabled }) }),
    refer(on) { referenced = on === true; for (const slot of slots) if (slot.child) holdChild(slot.child); },
    run(payload, options = {}) {
      if (closed || disabled !== undefined) return Promise.resolve({ lost: closed ? "closed" : "disabled: " + disabled });
      options.signal?.throwIfAborted();
      stats.jobs++;
      return new Promise(resolve => {
        const job = { id: nextId++, payload, signal: options.signal, attempts: 0, alone: false, resolve, settled: false, timer: undefined, queued: performance.now(), dispatched: 0, expected: farmJobExpectedMillisecondsV1(payload, policy), front: false };
        queue.push(job);
        if (options.signal) {
          const cancel = () => {
            if (job.settled) return;
            let kept = 0;
            for (const queued of queue) {
              if (queued.signal === options.signal) settle(queued, { lost: "aborted" });
              else queue[kept++] = queued;
            }
            queue.length = kept;
            settle(job, { lost: "aborted" });
            const slot = slots.find(each => each.job === job);
            if (slot) { slot.job = undefined; retire(slot, false); }
            pump();
          };
          if (options.signal.aborted) cancel(); else { options.signal.addEventListener("abort", cancel, { once: true }); job.release = () => options.signal.removeEventListener("abort", cancel); }
        }
        pump();
      });
    },
    close,
  });
}
