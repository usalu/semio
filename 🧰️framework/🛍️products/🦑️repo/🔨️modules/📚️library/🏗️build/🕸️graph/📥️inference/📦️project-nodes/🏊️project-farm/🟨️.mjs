import { mkdtempSync, realpathSync, rmSync } from "node:fs";
import { availableParallelism, tmpdir } from "node:os";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { adoptStagedGraphLeavesV1 } from "../../../../../🏗️build/📥️inputs/🟨️.mjs";
import { physicalSourceOriginV1 } from "../../../../../🏗️build/📥️inputs/🔏️revision/🟨️.mjs";
import { inferenceRecordIdentityV1 } from "../../../../../🏗️build/👁️observation/🟨️.mjs";
import { requestBuildFarmCapacityV1 } from "../../../../../🏗️build/🌡️host-capacity/🟦️.ts";
import { createFarmV1, planGraphFarmV1, renderGraphFarmMessagesV1, GRAPH_FARM_POLICY_V1 } from "./🏊️pool/🟨️.mjs";

const ORIGIN = physicalSourceOriginV1(import.meta.url, import.meta.semioPhysicalSourceURL), ENTRY = fileURLToPath(new URL("./👷️worker/🟨️.mjs", ORIGIN)), SERVICE = new URL("./👷️service/🟨️.mjs", ORIGIN).href, REPORTED_FAILURES = 8;

/**
 * 🏊️ Plans the farm that pre-infers the projects of one graph construction in worker processes, or answers undefined when the work is small or the farm is switched off. Nothing is asked of the host before the work proves to be cold: the farm engages once at least `minimumProjects` projects had no valid record and they are a quarter of the projects seen so far, so a warm run costs nothing.
 * The farm then asks the host capacity governor for its workers in the fair machine-wide queue without waiting for the answer: the parent keeps inferring projects itself until the workers it was granted are ready. Each worker runs the real project inference into a staging directory of its own; the parent adopts the finished leaves under its writer custody (`prepare`) and then reads the record exactly as it would read one a previous run left, so the farm changes how fast a record appears, never what it holds.
 */
export function openProjectFarmV1({ workspaceRoot, context, projects, commandInputs, projectsByRoot, componentRoots, environment = process.env, signal, processed = () => 0, command, commandArguments, spawnProcess, request = requestBuildFarmCapacityV1, cores = availableParallelism(), write = line => process.stderr.write(line + "\n") }) {
  const policy = GRAPH_FARM_POLICY_V1, say = (key, fields) => { for (const line of renderGraphFarmMessagesV1(key, fields, environment)) write(line); };
  const plan = planGraphFarmV1({ projects, environment, cores });
  if (plan.workers === 0) { if (plan.reason === "work" || plan.reason === "cpu") say("serial", { projects, reason: plan.reason }); return undefined; }
  const listeners = [], counts = { adopted: 0, failed: 0, lost: 0, declined: 0, recycled: 0, leaves: 0, adoptMilliseconds: 0, misses: 0, lostJobs: 0, workerCpuMilliseconds: 0 };
  let state = "idle", farm, workers = 0, started, base, closing = false, capacity, opening = Promise.resolve();
  const stop = (key, fields) => { state = "off"; say(key, fields); };
  const start = granted => {
    base = mkdtempSync(join(realpathSync(tmpdir()), "semio-graph-farm-"));
    workers = granted.jobs;
    farm = createFarmV1({
      entry: ENTRY, service: SERVICE, workers, megabytes: granted.megabytes, environment, signal,
      session: { version: 2, workspaceRoot, registry: context.registry ?? null, commandInputs: commandInputs ?? null, projectsByRoot: [...projectsByRoot], componentRoots, stagingBase: base },
      accept: identity => identity === inferenceRecordIdentityV1(),
      report: (event, fields) => {
        if (event === "recycled") { counts.recycled++; say("recycled", fields); }
        else if (event === "lost") { counts.lost++; say("lost", fields); }
        else if (event === "disabled") stop(String(fields.reason).startsWith("identity") ? "refused" : "degraded", fields);
      },
      retired: (slot, generation) => { try { rmSync(join(base, "w" + slot + "-g" + generation), { recursive: true, force: true }); } catch {} },
      ...(command === undefined ? {} : { command }), ...(commandArguments === undefined ? {} : { commandArguments }), ...(spawnProcess === undefined ? {} : { spawnProcess }),
    });
    started = performance.now();
    state = "starting";
    say("starting", { workers, megabytes: granted.megabytes });
    farm.ready.then(usable => { if (usable && state === "starting") { state = "ready"; for (const listener of listeners) listener(workers * policy.loopsPerWorker); } });
  };
  const open = () => {
    state = "waiting";
    say("waiting", { workers: plan.workers, projects });
    capacity = request({ preferredJobs: plan.workers, environment, signal, write });
    opening = capacity.granted.then(granted => {
      if (granted === undefined) { state = "off"; return; }
      if (closing) { capacity.release(); return; }
      if (granted.jobs < 2) { capacity.release(); stop("serial", { projects, reason: "host capacity" }); return; }
      start(granted);
    }).catch(error => stop("serial", { projects, reason: String(error?.message ?? error) }));
  };
  return Object.freeze({
    onReady(listener) { listeners.push(listener); if (state === "ready") listener(workers * policy.loopsPerWorker); },
    async prepare(job) {
      if (state === "idle") { counts.misses++; if (counts.misses >= policy.minimumProjects && counts.misses * 10000 >= processed() * policy.missBasisPoints) open(); return false; }
      if (state !== "ready" || !farm.usable) return false;
      const outcome = await farm.run(job, { signal });
      if (outcome.failure) { if (++counts.failed <= REPORTED_FAILURES) say("failed", { project: job.name, reason: outcome.failure.message.slice(0, 200) }); return false; }
      if (outcome.lost !== undefined) { counts.lostJobs++; (counts.lostReasons ??= {})[outcome.lost] = (counts.lostReasons[outcome.lost] ?? 0) + 1; return false; }
      const adopting = performance.now(), adoption = adoptStagedGraphLeavesV1(workspaceRoot, join(base, "w" + outcome.slot + "-g" + outcome.generation), outcome.reply.names);
      counts.adoptMilliseconds += performance.now() - adopting;
      counts.leaves += outcome.reply.names.length; counts.workerCpuMilliseconds += outcome.reply.cpuMilliseconds;
      if (adoption.declined) { counts.declined++; say("declined", { project: job.name }); return false; }
      if (adoption.published + adoption.present === 0) return false;
      counts.adopted++;
      return true;
    },
    async close() {
      closing = true;
      if (farm === undefined) capacity?.release();
      await opening;
      if (farm === undefined) return;
      await farm.close();
      capacity.release();
      rmSync(base, { recursive: true, force: true });
      say("finished", { adopted: counts.adopted, projects, workers, seconds: Math.round((performance.now() - started) / 1000), failed: counts.failed, lost: counts.lost, recycled: counts.recycled });
      if (environment[policy.phasesEnvironment] === "1") write("[graph-farm] " + JSON.stringify({ ...farm.stats(), ...counts, adoptMilliseconds: Math.round(counts.adoptMilliseconds), projects, reason: plan.reason }));
    },
  });
}
