import { createHash } from "node:crypto";
import { rmSync, writeFileSync } from "node:fs";
import { availableParallelism } from "node:os";
import { join } from "node:path";
import { serialize } from "node:v8";
import { fileURLToPath } from "node:url";
import { adoptStagedGraphLeavesV1 } from "../../../../../🏗️build/📥️inputs/🟨️.mjs";
import { physicalSourceOriginV1 } from "../../../../../🏗️build/📥️inputs/🔏️revision/🟨️.mjs";
import { inferenceAuthorityRowsV1, inferenceRecordIdentityV1 } from "../../../../../🏗️build/👁️observation/🟨️.mjs";
import { adoptedCapturedBuildLaunchV1 } from "../../../../../🏗️build/📥️source/🔏️closure/🚪️command/🟦️.ts";
import { GRAPH_FARM_WORKER_ROLE_V1, graphFarmIdentityDifferenceV1, graphFarmIdentityV1 } from "./🪆️captured/🟨️.mjs";
import { graphWaveMeterV1 } from "../../../📈️wave-row/🟨️.mjs";
import { FOOTPRINT_POLICY_V1, createFootprintMeterV1, footprintAtLeastManagedV1, footprintGateV1, readTreeFootprintBytesV1 } from "../../../../🧠️memory/🦶️footprint/🟨️.mjs";

let farmMeter;

/** 📏️ What the next farm child is booked at: the declared megabytes of a child until three children have reported their footprint, then the 75th percentile of the measured footprints plus 15 percent, never under 768 MB and never over the declared size, so the reservation follows what children really hold and the gate carries as many of them as the memory allows.
 * @see ./🧪️tests/🧮️measured-ask/🟦️.ts */
export const askWorkerMegabytesV1 = (declaredMegabytes, footprintsMegabytes) => {
  const sizes = footprintsMegabytes.filter(value => Number.isFinite(value) && value > 0).sort((left, right) => left - right);
  if (sizes.length < 3) return declaredMegabytes;
  return Math.max(768, Math.min(declaredMegabytes, Math.ceil(sizes[Math.min(sizes.length - 1, Math.ceil(sizes.length * 0.75) - 1)] * 1.15)));
};

/** 🛑️ Whether the farm holds its next job: admitting a job adds a child that peaks at `farmWorkerPeakMegabytes` of the plan policy (measured), so the farm holds while the tree footprint of this process (the worker and its farm children) plus that one child would pass the hard share of the ceiling of the footprint gate; with no gate it never holds. The reading is the farm's own meter of the same tree footprint the gate and the pool read, refreshed after one second: the meter of the gate answers from a reading up to five seconds old (`ttlMilliseconds`), and a farm child grew the tree to 9.4 and 10.0 GB within that time.
 * Before it answers true the parent releases the caches it keeps beside its work (`release` returns the bytes it let go): the parent is the largest process of the read and must give back what it can before a child is held back.
 * @see ./🏊️pool/🟨️.mjs `hold` */
export const farmHeldV1 = (gate = footprintGateV1(), allowanceMegabytes = PLAN_STAGE_POLICY_V1.farmWorkerPeakMegabytes["project-farm"], meter = (farmMeter ??= createFootprintMeterV1({ ttlMilliseconds: GRAPH_FARM_POLICY_V1.holdMeterMilliseconds, read: async () => footprintAtLeastManagedV1(await readTreeFootprintBytesV1(process.pid)) })), release = undefined) => {
  const grant = gate?.grantBytes();
  if (!(grant > 0)) return false;
  const limit = grant * FOOTPRINT_POLICY_V1.hardPercent / 100, over = freed => meter.bytes() - freed + allowanceMegabytes * 1048576 > limit;
  if (!over(0)) return false;
  let freed = 0;
  try { const bytes = release?.(); if (Number.isFinite(bytes) && bytes > 0) freed = bytes; } catch {}
  return over(freed);
};
import { requestBuildFarmCapacityV1 } from "../../../../../🏗️build/🌡️host-capacity/🟦️.ts";
import { PLAN_STAGE_POLICY_V1, leanSliceV1, reserveFarmV1 } from "../../../../🧠️memory/🪜️plan-stage/🟨️.mjs";
import { leanFarmPolicyV1, renderLeanSliceMessageV1 } from "../../../../🧠️memory/🪶️lean/🟨️.mjs";
import { warmThroughV1 } from "../🔱️warm/🟨️.mjs";
import { queuedJobTextV1 } from "../../../../../🏗️build/👁️observation/🚧️read-parent/🟨️.mjs";
import { GraphFarmOverrunErrorV1, choosePoolV1, createFarmTemporaryDirectoryV1, createFarmV1, grantedPoolV1, ownGrantPoolV1, retireOwnedGraphPoolV1, planGraphFarmV1, renderGraphFarmMessagesV1, GRAPH_FARM_POLICY_V1 } from "./🏊️pool/🟨️.mjs";

/** 🛑️ The refusal of a job that no farm child could do after its tries: the parent does not infer it in their place. */
const jobFailedV1 = (project, reason, tries) => { const text = language => language === "de" ? `Farm-Kindprozesse konnten ${project} in ${tries} Versuchen nicht ableiten (${reason}); der Elternprozess leitet es nicht ab` : `farm children could not infer ${project} in ${tries} tries (${reason}); the parent does not infer it`; return Object.assign(new Error(text("en") + " / " + text("de")), { code: "graph.farm-job-failed", semioProject: project }); };
const EMPTY_ATTEMPTS_V1 = 5, QUEUE_TELL_MILLISECONDS_V1 = 60000;
const ORIGIN = physicalSourceOriginV1(import.meta.url, import.meta.semioPhysicalSourceURL), ENTRY = fileURLToPath(new URL("./👷️worker/🟨️.mjs", ORIGIN)), SERVICE = new URL("./👷️service/🟨️.mjs", ORIGIN).href, REPORTED_FAILURES = 8;

/** 🏘️ The worker pools of this process, one hub per session (what the parent keyed its projects with): the graph constructions of one process that share a session (the waves of one refresh) share the pools, so a worker starts once, keeps what it loaded for its first project and serves every wave. */
const HUBS = new Map(), REFUSED = new Set(), AGAIN = Symbol("again");

/** 🔑️ The digest of the session a hub serves: everything the parent hands its workers and the captured implementation they run from. */
function hubKeyV1({ workspaceRoot, closureStore, context, contracts, commandInputs, lateClosures, projectsByRoot, componentRoots }, implementation) {
  return createHash("sha256").update(serialize([workspaceRoot, closureStore ?? null, context.registry ?? null, contracts, commandInputs ?? null, lateClosures === true, [...projectsByRoot], componentRoots, implementation ?? null])).digest("hex");
}

/** 🏗️ Creates the shared state of one hub: the pools, the capacity request, the waiters for a pool and the listeners of the constructions that use it live here, not in one construction. */
function createHubV1(key) {
  return { key, pools: [], readyWaiters: new Set(), listeners: new Set(), state: "idle", capacity: undefined, waiting: undefined, opening: Promise.resolve(), image: undefined, refusal: undefined, ownedRefresh: undefined, unavailable: false, closing: false, users: 0, idleTimer: undefined, retired: undefined, say: () => {}, wake() { for (const resolve of [...this.readyWaiters]) resolve(); } };
}

const closePoolV1 = pool => pool.closing ??= (pool.farm.close().finally(() => pool.release()));

/** 🧹️ Ends one hub: stops its pools, gives its capacity back and removes what the workers staged; idempotent. */
function retireHubV1(hub) {
  if (hub.retired !== undefined) return hub.retired;
  hub.closing = true;
  hub.lease?.release();
  if (HUBS.get(hub.key) === hub) HUBS.delete(hub.key);
  clearTimeout(hub.idleTimer);
  hub.wake();
  hub.retired = (async () => {
    const failures = [];
    try { await hub.ownedRefresh; } catch (error) { failures.push(error); }
    hub.waiting?.release();
    if (hub.pools.every(pool => pool.source === "own")) hub.capacity?.release();
    await Promise.race([hub.opening, new Promise(done => setTimeout(done, GRAPH_FARM_POLICY_V1.shutdownGraceMilliseconds).unref?.())]);
    for (const pool of hub.pools) { try { await closePoolV1(pool); } catch (error) { failures.push(error); } finally { rmSync(pool.base, { recursive: true, force: true }); } }
    if (hub.image !== undefined) rmSync(hub.image.directory, { recursive: true, force: true });
    if (failures.length === 1) throw failures[0];
    if (failures.length > 1) throw new AggregateError(failures, "Closing the graph farm failed for " + failures.length + " pools");
  })();
  return hub.retired;
}

/** 🛑️ Ends every worker pool of this process (the end of a refresh, a test): nothing of a hub outlives it. */
export async function closeProjectFarmsV1() {
  REFUSED.clear();
  const results = await Promise.allSettled([...HUBS.values()].map(retireHubV1));
  const failed = results.find(row => row.status === "rejected");
  if (failed) throw failed.reason;
}

/**
 * 🏊️ Opens the project farm of one graph construction on the worker pools of its session: pools that an earlier construction of this process started for the same session (the earlier waves of a refresh) are used at once, otherwise the farm plans, asks and starts them, and answers undefined when the work is small or the farm is switched off. Nothing is asked of the host before the work proves to be cold: the farm engages once at least `minimumProjects` projects had no valid record and they are a quarter of the projects seen so far, so a warm run costs nothing. A generation that is already cold (`engage`) starts the worker its grant pays for at once, so those misses are not inferred in this process until that quarter is reached.
 * The pools belong to the session, not to the construction: `close` of a construction reports its wave and leaves the workers running (they are unreferenced while no construction uses them, so they never keep the process alive, and they end after `idleCloseMilliseconds` without a construction, on `closeProjectFarmsV1` and with the process); a changed session (the registry, the contracts, the command inputs or the project roots) retires the pools of the old one.
 * The parent never infers a project while a worker can be had: a project waits for the pools that are being started (at most `holdMilliseconds`) or for the host's answer to the capacity request (as long as the request is pending: the first worker of a farm is `essential` and the gate admits it whatever the free memory says, so the parent, the largest process of the read, never infers an owner a worker could run); only when a started pool stays silent for `holdMilliseconds`, the host granted none or the farm is off, the parent infers it, and a pool that comes later takes the projects after it. A worker starts from its own environment (it needs no message of this process), so a busy event loop of the parent never keeps it from starting.
 * A project that a worker fails goes back to the pool once, on any worker; this process infers it only when the second try fails as well or no worker is left. Cancelling the signal of a construction retires the pools of its session at once, and a process that exits takes its workers with it.
 * Once the graph cache declines the leaves a worker produced (its optional quota is full), no further project goes to a worker and every pool is closed at once: a declined project is inferred again by this process, so going on would infer each one twice and keep the workers' memory and grants.
 * A pool that is disabled (its workers refused for a differing identity, or degraded) gives its host grant back at once, so the memory and cores reserved for workers that never run are free for the other runs of the machine at once.
 * The farm starts the worker the capacity grant of this process already pays for and asks the host capacity governor for more workers in the fair machine-wide queue without waiting for the answer; any grant of at least one worker starts a pool of its own. Each worker runs the real project inference into a staging directory of its own; the parent adopts the finished leaves under its writer custody (`prepare`) and then reads the record exactly as it would read one a previous run left, so the farm changes how fast a record appears, never what it holds.
 * The session hands every worker what the parent keyed its projects with (the registry, the command inputs and the generator contracts it read once), so a peer edit between the parent's read and a worker's start can never put content of other contracts under the parent's key.
 * In a process that runs from a captured snapshot (`launch`, by default the snapshot this process adopted; `null` starts workers from the files) every worker is started from that same snapshot, written once to a private file its environment names, and serves only when it reports the captured implementation and the record identity of this process, under the sealed load authority this process sends it (a worker never repeats what this command read before its first project); a worker loaded from the files on disk, which peers may have changed since the capture, is refused.
 */
export function openProjectFarmV1({ workspaceRoot, context, contracts, closureStore, projects, commandInputs, lateClosures = false, projectsByRoot, componentRoots, environment = process.env, signal = undefined, processed = () => 0, launch = adoptedCapturedBuildLaunchV1(), command = undefined, commandArguments = undefined, spawnProcess = undefined, request = requestBuildFarmCapacityV1, adopt = adoptStagedGraphLeavesV1, cores = availableParallelism(), maximumWorkers = Infinity, engage = false, immediate = false, releaseCaches = undefined, policy: declaredPolicy = GRAPH_FARM_POLICY_V1, write = line => { process.stderr.write(line + "\n"); } }) {
  const lean = leanSliceV1({ environment, cores, residentMegabytes: process.memoryUsage().rss / 1048576 }), policy = leanFarmPolicyV1(declaredPolicy, lean);
  if (lean.lean) engage = true;
  const say = (key, fields) => { for (const line of renderGraphFarmMessagesV1(key, fields, environment)) write(line); };
  const captured = launch ?? undefined, key = hubKeyV1({ workspaceRoot, closureStore, context, contracts, commandInputs, lateClosures, projectsByRoot, componentRoots }, captured?.implementation);
  let hub = HUBS.get(key);
  for (const [other, old] of HUBS) if (other !== key && old.users === 0 && old.pools.every(pool => !pool.farm.pending)) void retireHubV1(old).catch(() => {});
  if (hub !== undefined && (hub.state === "off" || hub.closing)) { void retireHubV1(hub).catch(() => {}); hub = undefined; }
  const plan = planGraphFarmV1({ projects, environment, cores, policy, maximumWorkers, minimumWorkers: lean.lean ? 1 : 2 });
  if (hub === undefined && REFUSED.has(key)) { say("serial", { projects, reason: "identity" }); return undefined; }
  if (hub === undefined) {
    if (plan.workers === 0) { if (plan.reason === "work" || plan.reason === "cpu" || plan.reason === "memory") say("serial", { projects, reason: plan.reason }); return undefined; }
    const lease = reserveFarmV1({ kind: "project-farm", workerMegabytes: policy.workerMegabytes, wanted: plan.workers, environment });
    if (lease.workers < (lean.lean ? 1 : 2)) { lease.release(); if (lean.lean) for (const line of renderLeanSliceMessageV1("tooSmall", { child: lean.childMegabytes, parent: lean.parentMegabytes, ceiling: Math.round(lean.limitMegabytes ?? 0) }, environment)) write(line); say("serial", { projects, reason: "memory" }); return undefined; }
    plan.workers = Math.min(plan.workers, lease.workers);
    hub = createHubV1(key);
    hub.lease = lease;
    HUBS.set(key, hub);
  }
  const { pools, readyWaiters, listeners } = hub, own = [], call = { closed: false };
  let warmGaveUp = false;
  const warmWhy = {}, noteWarm = why => { warmWhy[why] = (warmWhy[why] ?? 0) + 1; };
  const counts = { adopted: 0, failed: 0, lost: 0, declined: 0, recycled: 0, capped: 0, overruns: 0, leaves: 0, adoptMilliseconds: 0, misses: 0, lostJobs: 0, workerCpuMilliseconds: 0, holding: false };
  const started = performance.now();
  hub.say = say;
  hub.users++;
  const abort = () => { void retireHubV1(hub).catch(() => {}); };
  if (signal !== undefined) { if (signal.aborted) abort(); else signal.addEventListener("abort", abort, { once: true }); }
  clearTimeout(hub.idleTimer);
  for (const pool of pools) pool.farm.refer(true);
  const sayHub = (name, fields) => hub.say(name, fields);
  const readyWithin = milliseconds => new Promise(resolve => { const done = () => { clearTimeout(timer); readyWaiters.delete(done); resolve(); }, timer = setTimeout(done, milliseconds); timer.unref?.(); readyWaiters.add(done); });
  const imageFile = () => { if (hub.image !== undefined) return hub.image.file; const directory = createFarmTemporaryDirectoryV1("image"), file = join(directory, "snapshot.json"); try { writeFileSync(file, captured.text(), { flag: "wx", mode: 0o600 }); } catch (error) { rmSync(directory, { recursive: true, force: true }); throw error; } hub.image = { directory, file }; return file; };
  const stop = (name, fields) => { hub.state = "off"; sayHub(name, fields); hub.wake(); };
  const loops = () => pools.reduce((total, pool) => total + (pool.ready ? pool.workers * policy.loopsPerWorker : 0), 0);
  const start = ({ source, workers, megabytes }, release) => {
    if (hub.state === "off" || hub.closing) { release(); return undefined; }
    let auxiliaryFile;
    try { auxiliaryFile = captured === undefined ? undefined : imageFile(); } catch (error) { release(); stop("degraded", { reason: String(error?.message ?? error) }); return undefined; }
    const base = createFarmTemporaryDirectoryV1("staging"), pool = { source, workers, megabytes, base, released: false, release: () => { if (!pool.released) { pool.released = true; release(); } }, ready: false, farm: undefined, work: new Set(), closing: undefined };
    pool.farm = createFarmV1({
      entry: captured === undefined ? ENTRY : GRAPH_FARM_WORKER_ROLE_V1, service: SERVICE, workers, megabytes, policy, limitMegabytes: lean.limitMegabytes ?? Infinity, environment: captured === undefined ? environment : { ...environment, ...captured.environment },
      session: { version: 3, workspaceRoot, closureStore, registry: context.registry ?? null, contracts, commandInputs: commandInputs ?? null, lateClosures, projectsByRoot: [...projectsByRoot], componentRoots, stagingBase: base, authority: inferenceAuthorityRowsV1() },
      hold: () => farmHeldV1(undefined, undefined, undefined, releaseCaches),
      accept: identity => { const difference = graphFarmIdentityDifferenceV1(graphFarmIdentityV1(inferenceRecordIdentityV1(), captured?.implementation), identity); hub.refusal = difference === "" ? undefined : difference + (Array.isArray(identity?.conflicts) && identity.conflicts.length ? " (" + identity.conflicts.join(", ") + ")" : ""); return difference === ""; },
      report: (event, fields) => {
        if (event === "disabled") {
          pool.ready = false; if (pool.farm === undefined) pool.disabledEarly = true; else void closePoolV1(pool).catch(() => {});
          if (String(fields.reason).startsWith("identity")) { REFUSED.add(key); hub.state = "off"; hub.waiting?.release(); }
          else if (!pools.some(each => !each.closing && each.farm.usable)) hub.unavailable = true;
          hub.wake();
        }
        if (event === "recycled") { counts.recycled++; sayHub("recycled", fields); }
        else if (event === "lost") { counts.lost++; sayHub("lost", fields); }
        else if (event === "priority") sayHub("priority", fields);
        else if (event === "refused") sayHub("workerRefused", fields);
        else if (event === "capped") { counts.capped++; sayHub(fields.alone ? "cappedAlone" : "capped", fields); }
        else if (event === "disabled") sayHub(String(fields.reason).startsWith("identity") ? "refused" : "degraded", hub.refusal === undefined ? fields : { ...fields, reason: fields.reason + ": " + hub.refusal });
      },
      retired: (slot, generation) => { try { rmSync(join(base, "w" + slot + "-g" + generation), { recursive: true, force: true }); } catch {} },
      ...(captured === undefined ? { ...(command === undefined ? {} : { command }), ...(commandArguments === undefined ? {} : { commandArguments }) } : { command: captured.command, commandArguments: captured.arguments, auxiliaryFile }), ...(spawnProcess === undefined ? {} : { spawnProcess }),
    });
    pools.push(pool);
    hub.unavailable = pool.disabledEarly === true && !pools.some(each => each !== pool && !each.closing && each.farm.usable);
    if (pool.disabledEarly === true) void closePoolV1(pool).catch(() => {});
    sayHub(source === "own" ? "own" : "starting", { workers, megabytes });
    pool.farm.ready.then(usable => { if (usable && hub.state !== "off" && !hub.closing) { pool.ready = true; hub.unavailable = false; for (const listener of [...listeners]) listener(loops()); } hub.wake(); });
    return pool;
  };
  const open = () => {
    if (hub.state !== "idle") return;
    hub.state = "waiting";
    const first = ownGrantPoolV1(environment, policy);
    if (first !== undefined) start(first, () => {});
    say("waiting", { workers: plan.workers, projects });
    const ask = (want, essential = false) => {
      const raw = request({ preferredJobs: want, essential, megabytes: askWorkerMegabytesV1(policy.workerMegabytes, [...peaks.values()].map(bytes => bytes / 1048576)), environment, signal, write });
      let given = false;
      const asked = { granted: raw.granted, release: () => { if (!given) { given = true; raw.release(); } } };
      hub.capacity = asked; hub.waiting = asked;
      return asked.granted.then(granted => {
        if (hub.waiting === asked) hub.waiting = undefined;
        if (granted === undefined) { hub.wake(); return; }
        if (hub.closing || hub.state === "off") { asked.release(); return; }
        const hosted = grantedPoolV1(granted, policy);
        if (hosted === undefined) { asked.release(); if (pools.length === 0) stop("serial", { projects, reason: "host capacity" }); hub.wake(); return; }
        start(hosted, asked.release);
        const have = pools.reduce((total, pool) => total + pool.workers, 0);
        if (have < plan.workers && !hub.closing && hub.state !== "off") return ask(plan.workers - have);
      });
    };
    hub.opening = ask(plan.workers, pools.length === 0).catch(error => { if (pools.length === 0) stop("serial", { projects, reason: String(error?.message ?? error) }); hub.wake(); });
  };
  const refreshOwned = () => { if (hub.ownedRefresh) return hub.ownedRefresh; const pool = pools.find(each => each.source === "own" && each.ready && each.farm.usable); if (!pool) return Promise.resolve(); const next = ownGrantPoolV1(environment, policy); hub.ownedRefresh = retireOwnedGraphPoolV1(pool, next).then(retired => { if (retired && !hub.closing && hub.state !== "off") { const current = ownGrantPoolV1(environment, policy); if (current) start(current, () => {}); } }).finally(() => { hub.ownedRefresh = undefined; }); return hub.ownedRefresh; };
  const peaks = new Map();
  const dispatch = async (pool, job, note = {}) => {
    const outcome = await pool.farm.run(job, { signal });
    if (outcome.failure) { if (++counts.failed <= REPORTED_FAILURES) say("failed", { project: job.name, reason: outcome.failure.message.slice(0, 200) }); note.reason = outcome.failure.message.slice(0, 200); return AGAIN; }
    if (outcome.overrun !== undefined) { counts.overruns++; throw new GraphFarmOverrunErrorV1({ ...outcome.overrun, project: job.name }, environment); }
    if (outcome.lost !== undefined) { counts.lostJobs++; (counts.lostReasons ??= {})[outcome.lost] = (counts.lostReasons[outcome.lost] ?? 0) + 1; note.reason = "lost: " + outcome.lost; return AGAIN; }
    const adopting = performance.now(), adoption = adopt(workspaceRoot, join(pool.base, "w" + outcome.slot + "-g" + outcome.generation), outcome.reply.names);
    counts.adoptMilliseconds += performance.now() - adopting;
    counts.leaves += outcome.reply.names.length; counts.workerCpuMilliseconds += outcome.reply.cpuMilliseconds; if (typeof outcome.reply.peakMegabytes === "number") peaks.set(job.name, outcome.reply.peakMegabytes * 1048576);
    if (adoption.declined) { counts.declined++; say("declined", { project: job.name }); if (hub.state !== "off") { stop("degraded", { reason: "the graph cache declined the leaves of " + job.name }); for (const each of pools) { each.ready = false; void closePoolV1(each).catch(() => {}); } } return false; }
    if (adoption.published + adoption.present === 0) { note.reason = "nothing adopted"; return AGAIN; }
    counts.adopted++;
    return true;
  };
  if (hub.state === "idle" && (engage === true || immediate === true)) open();
  else if (hub.state !== "idle" && pools.some(pool => pool.ready)) say("reused", { projects });
  const askChild = job => warmThroughV1({
    pick: () => { const at = choosePoolV1(pools.map(pool => ({ ready: pool.ready, usable: pool.farm.usable, idle: pool.farm.idle, pending: pool.farm.pending, width: pool.farm.width }))); return at < 0 ? undefined : pools[at]; },
    run: async pool => { const work = pool.farm.run(job, { signal }); pool.work.add(work); try { const outcome = await work; if (outcome.overrun === undefined) return outcome; counts.overruns++; const error = new GraphFarmOverrunErrorV1({ ...outcome.overrun, project: job.name }, environment); return { failure: { name: error.name, message: error.message, code: error.code } }; } finally { pool.work.delete(work); } },
    hopeless: () => warmGaveUp ? "no child came before" : hub.state === "off" || hub.closing ? "farm-" + (hub.closing ? "closing" : "off") : signal?.aborted ? "aborted" : pools.some(pool => !pool.closing && pool.farm.usable) || hub.waiting !== undefined || hub.state === "waiting" && pools.length === 0 ? undefined : "no-usable-child",
    wait: milliseconds => readyWithin(milliseconds),
  });
  const exists = () => hub.state !== "off" && !hub.closing && !call.closed && signal?.aborted !== true && !(hub.unavailable && hub.waiting === undefined && !pools.some(pool => !pool.ready && !pool.closing && pool.farm.usable));
  const waitForChild = async name => {
    let told = -Infinity;
    for (;;) {
      if (hub.state === "off" || hub.closing || call.closed || signal?.aborted) return false;
      if (pools.some(pool => pool.ready && pool.farm.usable)) return true;
      const granting = hub.waiting !== undefined, booting = pools.some(pool => !pool.ready && !pool.closing && pool.farm.usable);
      if (!granting && !booting && hub.state !== "idle") { hub.unavailable = true; return false; }
      if (!counts.holding) { counts.holding = true; say("holding", { seconds: Math.round((booting ? policy.holdMilliseconds : policy.grantWaitMilliseconds) / 1000) }); }
      if (performance.now() - told >= QUEUE_TELL_MILLISECONDS_V1) { told = performance.now(); write(queuedJobTextV1(name, granting ? "the host capacity governor holds the first child" : booting ? "the first child is starting" : "no child is ready")); }
      await readyWithin(1000);
    }
  };
  return Object.freeze({
    /** 🔔️ Calls the listener with the loop count of the pools once a pool is ready and again whenever the farm grows. @param {(loops: number) => void} listener */
    /** 📏️ The memory the child that inferred `name` grew by, in bytes, or undefined when no child did. @param {string} name */
    peakOf(name) { return peaks.get(name); },
    onReady(listener) { listeners.add(listener); own.push(listener); if (pools.some(pool => pool.ready)) listener(loops()); },
    /** 🔱️ Starts the pools now instead of at the first miss (the warm pass settles owners before any is known to miss). */
    engage() { if (hub.state === "idle") open(); },
    /** 📐️ The children that are ready: the lanes the warm pass can run at once. */
    width() { return pools.reduce((total, pool) => total + (pool.ready ? pool.workers : 0), 0); },
    /** 🧮️ The memory budget of one child, as the farm plans it. */
    workerMegabytes() { return policy.workerMegabytes; },
    /** ⏳️ Waits at most `milliseconds` for the first child to be ready and answers whether one is. */
    async whenReady(milliseconds) {
      const until = performance.now() + milliseconds;
      while (!pools.some(pool => pool.ready) && hub.state !== "off" && !hub.closing && !hub.unavailable && !signal?.aborted && performance.now() < until) await readyWithin(Math.max(1, Math.min(1000, until - performance.now())));
      return pools.some(pool => pool.ready);
    },
    /** 🔱️ Runs one warm job (an owner to settle, see `warmProjectRecordV1`) on a ready child and answers its reply, or undefined when no child could (the parent then takes the owner itself); nothing is adopted, the child wrote its durable row itself. */
    warmReasons() { return { ...warmWhy }; },
    exists() { return exists(); },
    /** 🔍️ What the children are doing right now, for the line of a parked owner: busy, idle, starting and empty slots, the queue, whether a job runs alone (every other child waits for it) and the restarts. */
    describe() { return pools.filter(pool => !pool.closing && pool.farm?.snapshot !== undefined).map(pool => { const s = pool.farm.snapshot; return "children busy " + s.busy + ", idle " + s.idle + ", starting " + s.starting + ", not started " + s.empty + (s.refused ? ", refused " + s.refused : "") + ", queued jobs " + s.queued + (s.alone ? ", a job runs alone and the others wait" : "") + ", restarts " + s.restarts; }).join("; ") || (hub.state === "off" ? "the farm is off" + (hub.refusal ? ": " + hub.refusal : "") : "no children"); },
    async observe(job) {
      let failures = 0, lost = 0, told = -Infinity;
      for (;;) {
        warmGaveUp = false;
        const result = await askChild(job);
        if (result.reply !== undefined) { counts.workerCpuMilliseconds += result.reply.cpuMilliseconds ?? 0; return result.reply; }
        if (!exists()) return undefined;
        const why = String(result.why);
        if (why.startsWith("failure:") && ++failures > policy.maximumAttempts) throw jobFailedV1(job.name, why, failures);
        if (why.startsWith("lost:") && ++lost > EMPTY_ATTEMPTS_V1) throw jobFailedV1(job.name, why, lost);
        if (performance.now() - told >= QUEUE_TELL_MILLISECONDS_V1) { told = performance.now(); write(queuedJobTextV1(job.name, why)); }
        await readyWithin(1000);
      }
    },
    async warm(job) {
      const result = await askChild(job);
      if (result.reply === undefined) { if (result.why.startsWith("no child within")) warmGaveUp = true; noteWarm(result.why); return undefined; }
      counts.workerCpuMilliseconds += result.reply.cpuMilliseconds ?? 0;
      return result.reply;
    },
    async prepare(job) {
      if (hub.state === "idle") { counts.misses++; if (immediate === true || counts.misses >= policy.minimumProjects && counts.misses * 10000 >= processed() * policy.missBasisPoints) open(); if (immediate !== true) return false; }
      if (hub.state === "off" || hub.closing) return false;
      await refreshOwned();
      const note = {};
      for (let attempt = 0; ; attempt++) {
        if (!(await waitForChild(job.name))) return false;
        const at = choosePoolV1(pools.map(pool => ({ ready: pool.ready, usable: pool.farm.usable, idle: pool.farm.idle, pending: pool.farm.pending, width: pool.farm.width })));
        if (at < 0) continue;
        const pool = pools[at], work = dispatch(pool, job, note);
        pool.work.add(work);
        let answer;
        try { answer = await work; } finally { pool.work.delete(work); }
        if (answer !== AGAIN) return answer;
        if (hub.state === "off" || hub.closing) return false;
        const empty = note.reason === "nothing adopted";
        if (empty ? attempt >= EMPTY_ATTEMPTS_V1 : attempt >= policy.maximumAttempts) throw jobFailedV1(job.name, note.reason ?? "no reason", attempt + 1);
        if (empty) await readyWithin(2500);
      }
    },
    async close() {
      if (call.closed) return;
      call.closed = true;
      signal?.removeEventListener("abort", abort);
      for (const listener of own) listeners.delete(listener);
      hub.users--;
      graphWaveMeterV1.farm({ workers: pools.reduce((total, pool) => total + pool.workers, 0), projects, adopted: counts.adopted, failed: counts.failed, lost: counts.lostJobs, declined: counts.declined });
      if (pools.length > 0) {
        say("finished", { adopted: counts.adopted, projects, workers: pools.reduce((total, pool) => total + pool.workers, 0), seconds: Math.round((performance.now() - started) / 1000), failed: counts.failed, lost: counts.lost, recycled: counts.recycled });
        if (environment[policy.phasesEnvironment] === "1") write("[graph-farm] " + JSON.stringify({ pools: pools.map(pool => pool.farm.stats()), ...counts, adoptMilliseconds: Math.round(counts.adoptMilliseconds), projects, reason: plan.reason }));
      }
      if (hub.closing) return retireHubV1(hub);
      if (hub.state === "off" || pools.length === 0 && hub.waiting === undefined) { if (hub.users === 0) return retireHubV1(hub); return; }
      if (hub.users === 0) {
        for (const pool of pools) pool.farm.refer(false);
        hub.idleTimer = setTimeout(() => void retireHubV1(hub).catch(() => {}), policy.idleCloseMilliseconds);
        hub.idleTimer.unref?.();
      }
    },
  });
}
