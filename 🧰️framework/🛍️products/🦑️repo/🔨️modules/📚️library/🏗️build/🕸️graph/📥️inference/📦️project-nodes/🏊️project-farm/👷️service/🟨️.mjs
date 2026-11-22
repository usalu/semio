import { lstatSync, mkdirSync, readdirSync } from "node:fs";
import { join, relative, sep } from "node:path";
import { libraryBootstrap } from "../../../🟨️.mjs";
import { adoptInferenceAuthorityV1, inferenceRecordIdentityV1, observeInferenceSnapshotV1, releaseInferenceMemoryV1 } from "../../../../../../🏗️build/👁️observation/🟨️.mjs";
import { shrinkBoundedCachesV1 } from "../../../../../../🏗️build/🧊️bounded/🟨️.mjs";
import { withImplementationPinV1 } from "../../../../../../🏗️build/📥️inputs/🔏️revision/🟨️.mjs";
import { withBuildGraphStagingV1 } from "../../../../../../🏗️build/📥️inputs/💾️storage/🟨️.mjs";
import { adoptProjectInferenceRegistryV1, createProjectInferenceEnvironmentV1, lookOwnersV1, observeProjectRecordV1, produceProjectRecordV1, warmProjectRecordV1 } from "../../🟨️.mjs";
import { graphFarmIdentityV1 } from "../🪆️captured/🟨️.mjs";
import poolSchema from "../🏊️pool/🧬️schema/🔣️.json" with { type: "json" };

let environment, staging, retainedBudgetBytes = Infinity;
const RETAINED_POLICY_V1 = poolSchema.$defs.Policy.const;

/** 🔌️ Opens one worker session: the same inference implementation as the parent, memos of its own, and a staging directory of its own that nothing else writes to and that only the parent removes, once it saw this process exit (a worker recycled right after a job must leave the leaves of that job for the parent to adopt); serves under the sealed load authority its parent sent unless one of its own load rows conflicts with it, and answers the farm identity it serves under: the captured implementation it runs from, the identity its records are written under and the first conflicting rows. */
export async function open(session, { slot, generation, megabytes }) {
  await libraryBootstrap;
  retainedBudgetBytes = retainedBudgetBytesV1(megabytes);
  environment = await observeInferenceSnapshotV1(() => createProjectInferenceEnvironmentV1(session));
  staging = join(session.stagingBase, "w" + slot + "-g" + generation);
  mkdirSync(staging, { recursive: true });
  const conflicts = Array.isArray(session.authority) ? adoptInferenceAuthorityV1(session.authority) : [];
  return { ...graphFarmIdentityV1(inferenceRecordIdentityV1()), conflicts: conflicts.slice(0, 3) };
}

/** 🗃️ Runs `body` with the private staging directory `directory` as the graph data directory: a farm child holds no custody of the graph store of its parent, so what a job writes beside its answer (the optional program receipts of the closure resolution) goes to a private directory that the parent removes and recomputes when it needs it, never to the store. */
export function stagedWithinV1(directory, body) {
  mkdirSync(directory, { recursive: true });
  return withBuildGraphStagingV1({ root: directory }, body);
}

/** 🧭️ The inference environment and the staging directory of the open session, for a puller that infers owners of the fleet pool in this same service. */
export const sessionEnvironmentV1 = () => environment, stagingDirectoryV1 = () => staging;

/** 🧾️ Names a staged leaf relative to the staging root with forward slashes, or undefined when the path is no leaf of this staging. */
function leafNameV1(path) {
  const name = relative(staging, path).split(sep).join("/");
  if (name === "" || name.startsWith("../")) return undefined;
  try { return lstatSync(path).isFile() && !name.endsWith(".tmp") ? name : undefined; } catch { return undefined; }
}

/** ▶️ Infers one project into the staging directory under the key and scope its parent uses, and answers the names of the leaves it published (the record, the blocks and receipts it wrote and the pack files of its record store), its cpu time and `peakMegabytes`, the memory THIS process grew by while it inferred (its peak over its resident size at the start of the job: the child's own need, never the parent's growth). */
export async function run(job) {
  adoptProjectInferenceRegistryV1(environment, job.registryDigest);
  if (job.warm === true) return warmProjectRecordV1(environment, job);
  if (job.look === true) return stagedWithinV1(join(staging, "look"), () => lookOwnersV1(environment, job));
  if (job.observe === true) return stagedWithinV1(join(staging, "observe"), () => observeProjectRecordV1(environment, job));
  const cpu = process.cpuUsage(), startRss = process.memoryUsage().rss, startPeak = process.resourceUsage().maxRSS * 1024, names = new Set(), note = path => { const name = leafNameV1(path); if (name !== undefined) names.add(name); };
  await withImplementationPinV1(() => withBuildGraphStagingV1({ root: staging, published: note }, () => produceProjectRecordV1(environment, job)));
  const records = join(staging, "inference", "np", job.cacheKey + ".json.records");
  try { for (const entry of readdirSync(records)) note(join(records, entry)); } catch (error) { if (error?.code !== "ENOENT") throw error; }
  const used = process.cpuUsage(cpu);
  const endPeak = process.resourceUsage().maxRSS * 1024, top = Math.max(process.memoryUsage().rss, endPeak > startPeak ? endPeak : 0);
  return { names: [...names].sort(), cpuMilliseconds: Math.round((used.user + used.system) / 1000), peakMegabytes: Math.max(0, Math.round((top - startRss) / 1048576)) };
}

/** 🧺️ The bytes of managed memory (heap in use plus external buffers) a farm child may keep after an owner: `retainedBasisPoints` of its reservation but at least `retainedMinimumMegabytes`; unbounded without a reservation. */
export function retainedBudgetBytesV1(megabytes, policy = RETAINED_POLICY_V1) { return Number.isFinite(megabytes) && megabytes > 0 ? Math.max(policy.retainedMinimumMegabytes, Math.floor(megabytes * policy.retainedBasisPoints / 10000)) * 1048576 : Infinity; }

/** 🧽️ Drops what the child kept from the owners it finished once the managed memory it holds after a collection passed its budget (the interned rows, verified blocks, images and publications of {@link releaseInferenceMemoryV1} and every bounded cache): they only save recomputation. Answers whether it dropped, so the caller collects once more. Safe between jobs only. */
export function trimRetainedV1(budgetBytes = retainedBudgetBytes) {
  const held = process.memoryUsage();
  if (held.heapUsed + held.external <= budgetBytes) return false;
  releaseInferenceMemoryV1();
  shrinkBoundedCachesV1(0);
  return true;
}
