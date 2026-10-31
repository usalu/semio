import { rmSync, writeFileSync } from "node:fs";
import { availableParallelism } from "node:os";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { adoptStagedGraphLeavesV1 } from "../../../../../🏗️build/📥️inputs/🟨️.mjs";
import { physicalSourceOriginV1 } from "../../../../../🏗️build/📥️inputs/🔏️revision/🟨️.mjs";
import { inferenceAuthorityRowsV1, inferenceRecordIdentityV1 } from "../../../../../🏗️build/👁️observation/🟨️.mjs";
import { adoptedCapturedBuildLaunchV1 } from "../../../../../🏗️build/📥️source/🔏️closure/🚪️command/🟦️.ts";
import { GRAPH_FARM_WORKER_ROLE_V1, graphFarmIdentityDifferenceV1, graphFarmIdentityV1 } from "./🪆️captured/🟨️.mjs";
import { requestBuildFarmCapacityV1 } from "../../../../../🏗️build/🌡️host-capacity/🟦️.ts";
import { choosePoolV1, createFarmTemporaryDirectoryV1, createFarmV1, grantedPoolV1, ownGrantPoolV1, retireOwnedGraphPoolV1, planGraphFarmV1, renderGraphFarmMessagesV1, GRAPH_FARM_POLICY_V1 } from "./🏊️pool/🟨️.mjs";

const ORIGIN = physicalSourceOriginV1(import.meta.url, import.meta.semioPhysicalSourceURL), ENTRY = fileURLToPath(new URL("./👷️worker/🟨️.mjs", ORIGIN)), SERVICE = new URL("./👷️service/🟨️.mjs", ORIGIN).href, REPORTED_FAILURES = 8;

/**
 * 🏊️ Plans the farm that pre-infers the projects of one graph construction in worker processes, or answers undefined when the work is small or the farm is switched off. Nothing is asked of the host before the work proves to be cold: the farm engages once at least `minimumProjects` projects had no valid record and they are a quarter of the projects seen so far, so a warm run costs nothing.
 * Once the graph cache declines the leaves a worker produced (its optional quota is full), no further project goes to a worker and every pool is closed at once: a declined project is inferred again by this process, so going on would infer each one twice and keep the workers' memory and grants.
 * A pool that is disabled (its workers refused for a differing identity, or degraded) gives its host grant back at once, so the memory and cores reserved for workers that never run are free for the other runs of the machine at once, not when the graph construction ends.
 * The farm then starts the worker the capacity grant of this process already pays for (so the parent, whose heap only grows over hundreds of projects, never infers the whole delta alone) and asks the host capacity governor for more workers in the fair machine-wide queue without waiting for the answer; any grant of at least one worker starts a pool of its own. Each worker runs the real project inference into a staging directory of its own; the parent adopts the finished leaves under its writer custody (`prepare`) and then reads the record exactly as it would read one a previous run left, so the farm changes how fast a record appears, never what it holds.
 * The session hands every worker what the parent keyed its projects with (the registry, the command inputs and the generator contracts it read once), so a peer edit between the parent's read and a worker's start can never put content of other contracts under the parent's key.
 * In a process that runs from a captured snapshot (`launch`, by default the snapshot this process adopted; `null` starts workers from the files) every worker is started from that same snapshot, written once to a private file it reads as descriptor 3, and serves only when it reports the captured implementation and the record identity of this process, under the sealed load authority this process sends it (a worker never repeats what this command read before its first project); a worker loaded from the files on disk, which peers may have changed since the capture, is refused.
 */
export function openProjectFarmV1({ workspaceRoot, context, contracts, projects, commandInputs, lateClosures = false, projectsByRoot, componentRoots, environment = process.env, signal = undefined, processed = () => 0, launch = adoptedCapturedBuildLaunchV1(), command = undefined, commandArguments = undefined, spawnProcess = undefined, request = requestBuildFarmCapacityV1, adopt = adoptStagedGraphLeavesV1, cores = availableParallelism(), write = line => { process.stderr.write(line + "\n"); } }) {
  const policy = GRAPH_FARM_POLICY_V1, say = (key, fields) => { for (const line of renderGraphFarmMessagesV1(key, fields, environment)) write(line); };
  const plan = planGraphFarmV1({ projects, environment, cores });
  if (plan.workers === 0) { if (plan.reason === "work" || plan.reason === "cpu") say("serial", { projects, reason: plan.reason }); return undefined; }
  const listeners = [], pools = [], counts = { adopted: 0, failed: 0, lost: 0, declined: 0, recycled: 0, leaves: 0, adoptMilliseconds: 0, misses: 0, lostJobs: 0, workerCpuMilliseconds: 0 };
  let state = "idle", started, closing = false, capacity, opening = Promise.resolve(), image, refusal, ownedRefresh;
  const captured = launch ?? undefined, imageFile = () => { if (image !== undefined) return image.file; const directory = createFarmTemporaryDirectoryV1("image"), file = join(directory, "snapshot.json"); try { writeFileSync(file, captured.text(), { flag: "wx", mode: 0o600 }); } catch (error) { rmSync(directory, { recursive: true, force: true }); throw error; } image = { directory, file }; return file; };
  const stop = (key, fields) => { state = "off"; say(key, fields); };
  const closePool=pool=>pool.closing??=(pool.farm.close().finally(()=>pool.release()));
  const loops = () => pools.reduce((total, pool) => total + (pool.ready ? pool.workers * policy.loopsPerWorker : 0), 0);
  const start = ({ source, workers, megabytes }, release) => {
    if (state === "off") { release(); return undefined; }
    let auxiliaryFile;
    try { auxiliaryFile = captured === undefined ? undefined : imageFile(); } catch (error) { release(); stop("degraded", { reason: String(error?.message ?? error) }); return undefined; }
    const base = createFarmTemporaryDirectoryV1("staging"), pool = { source, workers, megabytes, base, released:false,release:()=>{if(!pool.released){pool.released=true;release();}}, ready: false, farm: undefined, work:new Set(),closing:undefined };
    pool.farm = createFarmV1({
      entry: captured === undefined ? ENTRY : GRAPH_FARM_WORKER_ROLE_V1, service: SERVICE, workers, megabytes, environment: captured === undefined ? environment : { ...environment, ...captured.environment }, signal,
      session: { version: 3, workspaceRoot, registry: context.registry ?? null, contracts, commandInputs: commandInputs ?? null, lateClosures, projectsByRoot: [...projectsByRoot], componentRoots, stagingBase: base, authority: inferenceAuthorityRowsV1() },
      accept: identity => { const difference = graphFarmIdentityDifferenceV1(graphFarmIdentityV1(inferenceRecordIdentityV1(), captured?.implementation), identity); refusal = difference === "" ? undefined : difference + (Array.isArray(identity?.conflicts) && identity.conflicts.length ? " (" + identity.conflicts.join(", ") + ")" : ""); return difference === ""; },
      report: (event, fields) => {
        if (event === "disabled") { pool.ready=false;void closePool(pool).catch(()=>{}); }
        if (event === "recycled") { counts.recycled++; say("recycled", fields); }
        else if (event === "lost") { counts.lost++; say("lost", fields); }
        else if (event === "disabled") say(String(fields.reason).startsWith("identity") ? "refused" : "degraded", refusal === undefined ? fields : { ...fields, reason: fields.reason + ": " + refusal });
      },
      retired: (slot, generation) => { try { rmSync(join(base, "w" + slot + "-g" + generation), { recursive: true, force: true }); } catch {} },
      ...(captured === undefined ? { ...(command === undefined ? {} : { command }), ...(commandArguments === undefined ? {} : { commandArguments }) } : { command: captured.command, commandArguments: captured.arguments, auxiliaryFile }), ...(spawnProcess === undefined ? {} : { spawnProcess }),
    });
    pools.push(pool);
    started ??= performance.now();
    say(source === "own" ? "own" : "starting", { workers, megabytes });
    pool.farm.ready.then(usable => { if (usable && state !== "off" && !closing) { pool.ready = true; for (const listener of listeners) listener(loops()); } });
    return pool;
  };
  const open = () => {
    state = "waiting";
    const own = ownGrantPoolV1(environment, policy);
    if (own !== undefined) start(own, () => {});
    say("waiting", { workers: plan.workers, projects });
    capacity = request({ preferredJobs: plan.workers, environment, signal, write });
    opening = capacity.granted.then(granted => {
      if (granted === undefined) return;
      if (closing) { capacity.release(); return; }
      const hosted = grantedPoolV1(granted, policy);
      if (hosted === undefined) { capacity.release(); if (pools.length === 0) stop("serial", { projects, reason: "host capacity" }); return; }
      start(hosted, capacity.release);
    }).catch(error => { if (pools.length === 0) stop("serial", { projects, reason: String(error?.message ?? error) }); });
  };
  const refreshOwned=()=>{if(ownedRefresh)return ownedRefresh;const pool=pools.find(each=>each.source==="own"&&each.ready&&each.farm.usable);if(!pool)return Promise.resolve();const next=ownGrantPoolV1(environment,policy);ownedRefresh=retireOwnedGraphPoolV1(pool,next).then(retired=>{if(retired&&!closing&&state!=="off"){const current=ownGrantPoolV1(environment,policy);if(current)start(current,()=>{});}}).finally(()=>{ownedRefresh=undefined;});return ownedRefresh;};
  return Object.freeze({
    /** 🔔️ Calls the listener with the loop count of the pools once a pool is ready and again whenever the farm grows. @param {(loops: number) => void} listener */
    onReady(listener) { listeners.push(listener); if (pools.some(pool => pool.ready)) listener(loops()); },
    async prepare(job) {
      if (state === "idle") { counts.misses++; if (counts.misses >= policy.minimumProjects && counts.misses * 10000 >= processed() * policy.missBasisPoints) open(); return false; }
      if (state === "off") return false;
      await refreshOwned();
      const at = choosePoolV1(pools.map(pool => ({ ready: pool.ready, usable: pool.farm.usable, idle: pool.farm.idle, pending: pool.farm.pending, width: pool.farm.width })));
      if (at < 0) return false;
      const pool = pools[at], work=(async()=>{const outcome = await pool.farm.run(job, { signal });
      if (outcome.failure) { if (++counts.failed <= REPORTED_FAILURES) say("failed", { project: job.name, reason: outcome.failure.message.slice(0, 200) }); return false; }
      if (outcome.lost !== undefined) { counts.lostJobs++; (counts.lostReasons ??= {})[outcome.lost] = (counts.lostReasons[outcome.lost] ?? 0) + 1; return false; }
      const adopting = performance.now(), adoption = adopt(workspaceRoot, join(pool.base, "w" + outcome.slot + "-g" + outcome.generation), outcome.reply.names);
      counts.adoptMilliseconds += performance.now() - adopting;
      counts.leaves += outcome.reply.names.length; counts.workerCpuMilliseconds += outcome.reply.cpuMilliseconds;
      if (adoption.declined) { counts.declined++; say("declined", { project: job.name }); if (state !== "off") { stop("degraded", { reason: "the graph cache declined the leaves of " + job.name }); for (const each of pools) { each.ready = false; void closePool(each).catch(()=>{}); } } return false; }
      if (adoption.published + adoption.present === 0) return false;
      counts.adopted++;
      return true;})();pool.work.add(work);try{return await work;}finally{pool.work.delete(work);}
    },
    async close() {
      closing = true;
      let refreshFailure;try{await ownedRefresh;}catch(error){refreshFailure=error;}
      if (pools.every(pool => pool.source === "own")) capacity?.release();
      await opening;
      if (pools.length === 0) return;
      const stats = pools.map(pool => pool.farm.stats());
      const failures = refreshFailure===undefined?[]:[refreshFailure];
      for (const pool of pools) { try { await closePool(pool); } catch (error) { failures.push(error); } finally { rmSync(pool.base, { recursive: true, force: true }); } }
      if (image !== undefined) rmSync(image.directory, { recursive: true, force: true });
      say("finished", { adopted: counts.adopted, projects, workers: pools.reduce((total, pool) => total + pool.workers, 0), seconds: Math.round((performance.now() - started) / 1000), failed: counts.failed, lost: counts.lost, recycled: counts.recycled });
      if (environment[policy.phasesEnvironment] === "1") write("[graph-farm] " + JSON.stringify({ pools: stats, ...counts, adoptMilliseconds: Math.round(counts.adoptMilliseconds), projects, reason: plan.reason }));
      if (failures.length === 1) throw failures[0];
      if (failures.length > 1) throw new AggregateError(failures, "Closing the graph farm failed for " + failures.length + " pools");
    },
  });
}
