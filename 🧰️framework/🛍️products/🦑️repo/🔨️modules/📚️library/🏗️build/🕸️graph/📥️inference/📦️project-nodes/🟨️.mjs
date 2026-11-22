import{machinePathV1}from"../../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import {ROSTER_RULES_V1} from "../../🧭️roster/🟨️.mjs";
import {isOpaqueSourcePathV1} from "../../../../⚡️caching/🔍️discovery/📂️source/🟦️.ts";
import{applyBuildGeneratedTaskDependenciesV1,buildInputSourceRoleV1,buildNativeSourceObservationV1}from"../../🚪️admission/🟦️.ts";
import{ownerPoolNoProducerTextV1,ownerPoolProducersV1,ownerPoolReadFleetV1}from"./🌐️owner-pool/🤝️fleet/🟦️.ts";
import managersV1 from "../../../../⚡️caching/🚀️bootstrap/📦️dependencies/🔌️managers/🟨️.cjs";
const javascriptManagerV1 = managersV1.selectedManagerV1("javascript");
/** 📦️ Infers every project node of one workspace snapshot with bounded concurrency and settlement. */
import { advanceScriptInvocation } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { availableParallelism, totalmem } from "node:os";
import { AsyncResource } from "node:async_hooks";
import { createHash } from "node:crypto";
import { serialize } from "node:v8";
import { dirname, join, relative } from "node:path";
import { inferenceInventoryReferencesHoldV1, inferenceInventoryPresenceStatsV1 } from "../../../../🏗️build/👁️observation/📥️reuse/🟨️.mjs";
import { detachedFailureV1, memoizedObservedAsyncComputationV1, observeInferenceSnapshotV1, observeProjectInferenceV1, verifyStoredProjectRecordV1, observedFs, observedInferenceRowsV1, forgetObservedInferenceRowsV1, projectInferenceObservationsV1, releaseProjectInferenceRetentionV1, observationLookupClockV1, useProjectFleetV1, settleClosureRowsV1 } from "../../../../🏗️build/👁️observation/🟨️.mjs";
import { readStoredRecordV1, warmDigestV1, warmDirectoryV1, warmLineV1, warmOwnerV1, warmStampV1, warmWaveV1 } from "./🔱️warm/🟨️.mjs";
import { statSync as rawStatSync, readFileSync as rawReadFileSync, writeFileSync as rawWriteFileSync, renameSync as rawRenameSync, rmSync as rawRmSync } from "node:fs";
import { brotliDecompressSync as refsBrotliV1 } from "node:zlib";
import { INFERENCE_LIMITS_V1 } from "../🧬️schema/🟨️.mjs";
const { existsSync, readFileSync } = observedFs;

/** 🏷️ Names this component as its roster entry does. */
export const component = "project-nodes";

import { inferenceRecordIdentityV1, observationScopeV1, observationsHold, observeGlobalFactV1, registerGlobalFactV1, useSourceAnalysisStoreV1, withObservationPassRoundV1 } from "../../../../🏗️build/👁️observation/🟨️.mjs";
import { createGraphPhaseClockV1, writeGraphPhaseRowV1, writeGraphSettleRowV1 } from "../../⏱️phases/🟨️.mjs";
import { createGraphOwnerPeaksFileV1, createGraphParentMemoryGovernorV1, graphHostAskV1, createMemoryPortsV1, createParentCollectorV1, graphMemoryAllocationV1, graphParentManagedBytesV1, hostMemoryMegabytesV1, selectGraphMemoryLanguagesV1 } from "../../../🧠️memory/🟨️.mjs";
import { ownerTooHeavyOfV1, runInOwnerScopeV1 } from "../../../🧠️memory/🎯️owner-scope/🟨️.mjs";
import { footprintGateV1, useEnvironmentGateV1, noteSlowV1, createOwnerClockV1 } from "../../../🧠️memory/🦶️footprint/🟨️.mjs";
import { farmAloneCapV1, planFarmWorkersV1, planStageLimitBytesV1 } from "../../../🧠️memory/🪜️plan-stage/🟨️.mjs";
import { graphCutConcurrencyV1 } from "../../../🧠️memory/✂️cut/🟨️.mjs";
import { describeCollectionV1, registerSliceCollectionsV1, registerSliceReportSourceV1, sliceReadStepV1 } from "../../../🧠️memory/📋️slice-report/🟨️.mjs";
import { startSliceSamplerV1 } from "../../../🧠️memory/🔬️sampler/🟨️.mjs";
import { createErrorCollectorFromEnvV1 } from "../../🧾️collect-errors/🟨️.mjs";
import { openProjectFarmV1 } from "./🏊️project-farm/🟨️.mjs";
import { ownerRecordKeyV1, ownerRecordsPresentTextV1, ownerRecordsPresentV1 } from "./📋️present/🟨️.mjs";
import observationSchemaV1 from "../../../👁️observation/🧬️schema/🔣️.json" with { type: "json" };
import { TAKE_POLICY_V1, createOwnerClaimsV1, takeClaimedV1, takeFetchFirstV1, createAdoptCountersV1, recordTakeLookFailureV1, createHotNamesV1, takeBeforeInferV1, fleetPublishedNamesV1, createProgressWatchV1, createTailReporterV1, takeProgressSignV1, takeMissingRowsV1, takeStallKindV1, takeTailTextV1, createCpuProgressV1, takeChildrenCpuV1, createTakeLedgerV1, takeNoProgressTextV1, takeStrongMachineV1, takeAdmittedV1, joinTakeLookV1, takeLookConcurrencyV1, takeLanesV1, takePassV1, takePollV1, runTakeBesideInferV1, takeSplitV1 } from "./📥️take/🟨️.mjs";
import { graphWaveMeterV1 } from "../../📈️wave-row/🟨️.mjs";
import { GENERATOR_CONTRACTS_FACT_V1, generatorContractReadHoldsV1, generatorContractsInScopeV1, ownerRecordHeadV1, trackGeneratorContractsV1, withGeneratorContractsScopeV1 } from "../📜️contract-reads/🟨️.mjs";
import { awaitGrowingV1, GRAPH_FARM_POLICY_V1 } from "./🏊️project-farm/🏊️pool/🟨️.mjs";
import {refuseBuildV1} from "../../../📖️texts/🟦️.ts";
import {isForeignFailureV1,isOwnerFailureV1,ownerFailureOfReplyV1,ownerFailureReplyV1,ownerFailureV1,ownerIsolationPolicyV1,ownerRetryPlanV1} from "../../📥️load/🛟️owner-isolation/🟦️.ts";
import { assertNotReadParentV1, readParentObservedCountsV1, withReadParentScopeV1, withTakeLookScopeV1 } from "../../../../🏗️build/👁️observation/🚧️read-parent/🟨️.mjs";
import { createLookQueueV1 } from "./🔭️look-queue/🟨️.mjs";
import { ownerSourceFailureV1, replicaUnsettledV1 } from "./🚧️source-failure/🟨️.mjs";
let closureVerdictScopeV1, runClosureTakeV1, releaseAfterTakeLookV1, withClosurePassV1, primeSharedClosuresInChildV1, releaseOwnerStateV1, primeSharedClosuresV1, askHeavyQuestionV1, isHeavyQuestionV1, closureStoreDirectoryV1, bindClosureStoreV1, closureApiV1, closureHandleOfV1, closureHandleFromRecipeV1, deriveClosureFactV1, resolveClosureRefsV1, resolveClosureRefsPlanV1, adoptClosureRefsV1, inferenceCandidates,readBuildProducedCompileOutputV1,buildPath, yieldInferenceV1, catalogBuildContributionsV1, createScriptInputCache, POLICY, nativeCommandInputs, readTaxonomyProjectionV1, collectPlaygroundCatalog, PROJECT_BASENAME, playgroundSessionTargets, playgroundPreparationTargets, playgroundSiteTargetsForAdmissionV1, readToml, cargoTargets, componentBuildTargets, componentCrateRoots, projectAssemblyAuthorityV1, workspaceDataDirectory, projectWithDefaults, withNativePreparation, withReleasePipelinePreparation, loaded, readBunLockGraph, projectStampPathsV1;

/** 🔌️ Binds this component's dependencies from the revision registry once its needs are wired. */
export function wire(registry) {
  ({inferenceCandidates,readBuildProducedCompileOutputV1}=registry.ownerDiscovery);
  ({ buildPath, POLICY, PROJECT_BASENAME, workspaceDataDirectory } = registry.coordinates);
  ({ closureVerdictScopeV1, runClosureTakeV1, releaseAfterTakeLookV1, withClosurePassV1, primeSharedClosuresInChildV1, releaseOwnerStateV1, primeSharedClosuresV1, yieldInferenceV1, createScriptInputCache, nativeCommandInputs, closureApiV1, closureHandleOfV1, closureHandleFromRecipeV1, deriveClosureFactV1, closureStoreDirectoryV1, bindClosureStoreV1, askHeavyQuestionV1, isHeavyQuestionV1 } = registry.scriptInputs);
  ({ catalogBuildContributionsV1, projectAssemblyAuthorityV1 } = registry.ownerDiscovery);
  ({ readTaxonomyProjectionV1 } = registry.sourceAnalyses);
  ({ collectPlaygroundCatalog, playgroundSessionTargets, playgroundPreparationTargets, playgroundSiteTargetsForAdmissionV1 } = registry.playgroundTargets);
  ({ readToml, withNativePreparation } = registry.nativeManifests);
  ({ cargoTargets, componentBuildTargets, componentCrateRoots, withReleasePipelinePreparation } = registry.componentTargets);
  ({ projectWithDefaults, resolveClosureRefsV1, resolveClosureRefsPlanV1, adoptClosureRefsV1 } = registry.projectAssembly);
  ({ loaded } = registry.bootstrap);
  ({ readBunLockGraph } = registry.bunLock);
  ({ projectStampPathsV1 } = registry.stamps);
}

/** 🐘️ The truthful progress line of the wait for owners this machine can never infer: the owners that arrived, which are still wanted and what they need, in both languages. It changes with every arrival and every producing machine, never with the clock. */
const HEAVY_WAIT_V1 = Object.freeze({ en: "heavy owners that need more memory than this machine can give one inference ({megabytes} MB at least) are inferred by a machine of the fleet that can hold them: {arrived} of {wanted} arrived, {produced} machines delivered records so far, still wanted: {names}", de: "Schwere Projekte, die mehr Speicher brauchen, als diese Maschine einer Ableitung geben kann (mindestens {megabytes} MB), leitet eine Maschine der Flotte ab, die sie tragen kann: {arrived} von {wanted} angekommen, {produced} Maschinen lieferten bisher Einträge, noch gewünscht: {names}" });
const HEAVY_RETRY_V1 = Object.freeze({ en: "the fleet delivered nothing for {count} heavy owners ({names}); this machine runs them once itself, one at a time, before it refuses", de: "Die Flotte lieferte für {count} schwere Projekte nichts ({names}); diese Maschine leitet sie vor der Ablehnung einmal selbst ab, eines nach dem anderen" });
const HEAVY_ENDED_V1 = Object.freeze({ en: "the fleet has shown no progress on the {count} heavy owners ({cause}); this machine does not infer them and says so", de: "Die Flotte zeigte bei den {count} schweren Projekten keinen Fortschritt ({cause}); diese Maschine leitet sie nicht ab und sagt das" });

const OVERRUN_ASIDE_V1 = Object.freeze({ en: "owner {name} is set aside on this machine: it needed {megabytes} MB in a worker process even when it ran alone, above the {cap} MB this machine allows; a machine of the fleet with more memory infers it, this read goes on with every other owner and waits for it at the end", de: "Das Projekt {name} wird auf dieser Maschine zurückgestellt: es brauchte {megabytes} MB in einem Arbeitsprozess, auch als es allein lief, mehr als die {cap} MB, die diese Maschine erlaubt; eine Maschine der Flotte mit mehr Speicher leitet es ab, dieser Lauf macht mit allen anderen Projekten weiter und wartet am Ende darauf" });

/** 🪼️ What the reader of the owners does with an owner whose inference met a store that changed under it (a layer or inventory record added, replaced or removed by a pool child, a peer read or a landing, after the bounded looks of the lower levels): `requeue` the first time (its look starts again against the state that is there then), `aside` the second time (set aside with its reason, the read goes on and ends naming it), `undefined` for every other failure. `seen` holds the owners that were requeued.
 * @see ./🧪️tests/🔁️store-changed/🟦️.ts */
export function ownerStoreChangeV1(error,name,seen){
  if(error?.storeChanged!==true&&error?.code!=="BUILD_STORE_CHANGED")return undefined;
  if(seen.has(name))return"aside";
  seen.add(name);return"requeue";
}
/** 🧾️ Says the ledger line of the owner stage every `milliseconds` for as long as the poll for the owners the fleet has not stored yet runs: the reporter of the first pass ends with that pass, so before this the line stayed at the numbers of the first minutes (`0 fetched from the fleet`, `128 of 4775 done`) for the rest of an eight-hour read while the poll went on taking records. Answers the function that stops it; nothing is told after `polling` aborted.
 * @see ./🧪️tests/🧾️poll-ledger/🟦️.ts */
export function reportWhilePollingV1({ tell, milliseconds, polling }) {
  const timer = setInterval(() => { if (!polling.aborted) tell(); }, milliseconds);
  timer.unref?.();
  return () => clearInterval(timer);
}
/** 🧮️ Counts the owners a read set aside by reason and names the largest groups first in both languages, so a fault of the tooling that sets aside hundreds of owners shows as one group with its number, never as a line per owner only. */
export function asideCensusV1(reasons, top = 3) {
  const counts = new Map(); for (const reason of reasons) { const key = String(reason).slice(0, 160); counts.set(key, (counts.get(key) ?? 0) + 1); }
  const rows = [...counts].sort((a, b) => b[1] - a[1] || (a[0] < b[0] ? -1 : 1)).slice(0, top).map(([text, count]) => count + " x " + text).join(" | ");
  return { en: reasons.length + " owners are set aside by this read, by reason: " + rows, de: reasons.length + " Projekte werden von diesem Lauf zurückgestellt, nach Grund: " + rows };
}

/** 🪂️ Sets one owner aside that outgrew the alone cap of a worker process: it joins the owners this machine never infers (`never`, `alone`), the cost it showed is recorded under its name (`recorder`, scaled back by `factor` so the cost table reads the measured megabytes) so the offer to the fleet and the next read know what it needs, and one line in each language says why; answers whether `error` was such an overrun, any other failure is left to the caller. @param {(text: (language: string) => string) => void} say */
export function setAsideOverrunV1({ error, never, alone, recorder, factor, say }) {
  if (error?.code !== "graph.farm-overrun") return false;
  never.add(error.project); alone.add(error.project);
  try { recorder?.record({ key: error.project, kind: "project", takenBytes: Math.round(error.megabytes * 1048576 / factor), seconds: 0, stopped: true }); } catch {}
  say(language => OVERRUN_ASIDE_V1[language === "de" ? "de" : "en"].replace("{name}", error.project).replace("{megabytes}", error.megabytes).replace("{cap}", error.capMegabytes));
  return true;
}

/** 🕸️ How often the projects being inferred are named again while none finishes, so a heavy project stays visible with its running seconds. */
const RUNNING_TICK_MS = INFERENCE_LIMITS_V1.runningTickMilliseconds;

/** 🧊️ The first moment from which a recorded STOP of an owner in a farm child is its own: before it an alone job could land on a child that kept the memory of earlier jobs, so such a stop proves nothing about the owner and never refuses it (2026-11-18 00:30 UTC, `graph.farm-overrun` now runs alone only in a fresh child). */
export const STOPS_ATTRIBUTABLE_FROM_MS_V1 = 1794961800000;

/** 🚧️ Withholds one project whose consumed source does not parse, naming the exact bytes the snapshot observed; a freshly modified source rethrows so settlement waits for it. */
function unavailableProjectV1(root, project, workspaceRoot, error) {
  const unsettled = replicaUnsettledV1(error, { modified: path => rawStatSync(path).mtimeMs });
  if (unsettled !== undefined) throw unsettled;
  return { root, ...(project ? { project } : {}), source: buildPath(relative(workspaceRoot, error.source)), sha256: error.sha256, reason: String(error.message).slice(0, 2048) };
}

/** 🚦️ Whether a failure is never an owner's own: a cancelled invocation, anything another layer follows (restart codes, withheld sources, membership, concurrent edits, a named failure) and system errors. */
export const ownerFailureIsForeignV1 = (raised, aborted) => aborted === true || raised === null || typeof raised !== "object" || isForeignFailureV1(raised) || /^(E[A-Z0-9]+|ERR_.*)$/u.test(String(raised.code ?? ""));

/** 🛟️ Turns the failure of one owner's own inference (a catalogued refusal or a logic error; never a system error, a cancellation or a failure another layer follows) into the withheld-source failure of that owner: the named EN/DE failure is its reason (`rounds` observations, `stable` when it reproduced on settled inputs), the declaration of the owner is its source, and everything the failed inference read (the route tables included, or the rows a farm child sent with the named failure) stays among its observations, so the owner is inferred again exactly when one of them changes. A failure that is already the named failure of this owner (from a farm child or the late references) is used as it is. Answers `undefined` for every other failure.
 * @see ../../📥️load/🛟️owner-isolation/🟦️.ts */
export function ownerFailureAsWithheldV1(raised, { workspaceRoot, root, name, aborted, rounds = 1, stable = true }) {
  const named = isOwnerFailureV1(raised) && raised.owners.length === 1 && raised.owners[0].name === name;
  if (!named && ownerFailureIsForeignV1(raised, aborted)) return undefined;
  const source = join(workspaceRoot, root, PROJECT_BASENAME);
  let bytes;
  try { bytes = readFileSync(source); } catch { return undefined; }
  const failure = named ? raised : ownerFailureV1(name, raised, rounds, stable);
  return Object.assign(Error(failure.message, { cause: raised }), { code: "BUILD_SOURCE_UNPARSEABLE", source, sha256: createHash("sha256").update(bytes).digest("hex"), ...named && Array.isArray(raised.observed) ? { observedRows: raised.observed } : {} });
}

/** 🧺️ The rows the running owner observation has read so far (inside the computation of an owner, where a failure is caught before its observation ends). */
function observedRowsSoFarV1() {
  const scope = observationScopeV1();
  if (scope === undefined || scope === null) return [];
  if (scope.layers?.size) { const rows = new Set(); for (const item of scope.order) { if (Array.isArray(item)) for (const row of item) rows.add(row); else rows.add(item); } return [...rows]; }
  return [...scope.values()];
}

/** 🔁️ The code of the failure an owner inference throws when its failure was on inputs that moved (or were written within the racy window): the attempt loop {@link observeOwnerAttemptsV1} observes the owner again after the pause it names. */
export const OWNER_RETRY_CODE_V1 = "BUILD_OWNER_RETRY";

/** 🧭️ The decision of the shared table (see `ownerFailureDecisionV1`) for one failure of an owner inferred in this process: the rows it consumed up to the failure are taken from the failure and verified outside the validation round of the inference, so a file that moved after it was read is seen as moved. */
export async function ownerRetryPlanOfV1(raised, { aborted, round, started, scopeRows = false, hold = rows => withObservationPassRoundV1(undefined, () => observationsHold(rows)), now }) {
  const failure = detachedFailureV1(raised), rows = failure ? [...failure.rows, ...failure.layers.flat()] : scopeRows ? observedRowsSoFarV1() : [];
  return { ...await ownerRetryPlanV1({ rows, hold, foreign: ownerFailureIsForeignV1(raised, aborted), started, round, now }), rows };
}

/** 🛟️ What the inference of one owner makes of its failure, inside its observation: its own source failure, else (by the shared decision table) a retry (the failure that makes {@link observeOwnerAttemptsV1} observe the owner again after the pause), or the withheld-source failure with the named EN/DE failure of the owner, or `undefined` when the failure is another layer's; a caller that never retries passes the last round.
 * @see ../../📥️load/🛟️owner-isolation/🟦️.ts */
export async function ownerInferenceFailureV1(raised, { workspaceRoot, root, name, aborted, round, started }) {
  const own = ownerSourceFailureV1(raised, { workspaceRoot, fallback: join(workspaceRoot, root, PROJECT_BASENAME) });
  if (own !== undefined) return own;
  const plan = await ownerRetryPlanOfV1(raised, { aborted, round, started, scopeRows: true });
  if (plan.decision === "retry") { const failure = ownerFailureV1(name, raised, round, true); throw Object.assign(new Error(failure.message, { cause: raised }), { code: OWNER_RETRY_CODE_V1, ownerRetry: { owner: name, round, milliseconds: plan.milliseconds, text: failure.text } }); }
  return plan.decision === "foreign" ? undefined : ownerFailureAsWithheldV1(raised, { workspaceRoot, root, name, aborted, rounds: round, stable: plan.decision === "stable" });
}

/** 🔁️ Runs the observation of one owner again while its failure said the inputs moved: `run(round)` throws {@link OWNER_RETRY_CODE_V1} with the pause to wait (and the bilingual line `[graph-owner-retry]` says why); any other outcome is the outcome. The rounds are bounded by the table, which never asks for a retry in the last one.
 * @see ../../📥️load/🛟️owner-isolation/🧪️tests/🟦️.ts */
export async function observeOwnerAttemptsV1(run, { pause = milliseconds => new Promise(done => setTimeout(done, milliseconds)), say = line => process.stderr.write(line + "\n") } = {}) {
  for (let round = 1; ; round++) {
    try { return await run(round); }
    catch (raised) {
      if (raised?.code !== OWNER_RETRY_CODE_V1 || raised.ownerRetry === undefined || round > ownerIsolationPolicyV1.retryRounds) throw raised?.code === OWNER_RETRY_CODE_V1 && raised.cause !== undefined ? raised.cause : raised;
      try { say("[graph-owner-retry] " + JSON.stringify({ version: 1, ...raised.ownerRetry })); } catch {}
      await pause(raised.ownerRetry.milliseconds);
    }
  }
}

/** 🔁️ Resolves the closure references of one owner with the same isolation: a failure that is the owner's own is observed again while the rows it consumed (`rows()`, with what the failure carries) moved, and a failure that reproduces on settled inputs (or kept moving) is thrown as the named failure of the owner with those rows; source failures and failures another layer follows pass through unchanged for their own handling.
 * @param {() => Promise<unknown>} body @param {{ owner: string, scope: { workspaceRoot: string, fallback: string }, aborted?: () => boolean, rows?: () => readonly unknown[], pause?: (milliseconds: number) => Promise<void>, say?: (line: string) => void, hold?: (rows: readonly unknown[]) => Promise<boolean> }} port */
export async function observeRefsWithRetriesV1(body, { owner, scope, aborted = () => false, rows = () => [], pause = milliseconds => new Promise(done => setTimeout(done, milliseconds)), say = line => process.stderr.write(line + "\n"), hold }) {
  for (let round = 1; ; round++) {
    const started = Date.now();
    try { return await body(); }
    catch (raised) {
      if (["graph.farm-job-failed", "BUILD_OWNER_DEFERRED", "BUILD_STORE_CHANGED"].includes(raised?.code) || raised?.storeChanged === true) throw raised;
      let own; try { own = ownerSourceFailureV1(raised, scope); } catch { own = undefined; }
      if (own !== undefined) throw raised;
      const plan = await ownerRetryPlanOfV1(raised, { aborted: aborted(), round, started, ...hold === undefined ? {} : { hold } });
      if (plan.decision === "foreign") throw raised;
      if (plan.decision === "retry") {
        try { const failure = ownerFailureV1(owner, raised, round, true); say("[graph-owner-retry] " + JSON.stringify({ version: 1, owner, round, milliseconds: plan.milliseconds, text: { en: failure.text.en, de: failure.text.de } })); } catch {}
        await pause(plan.milliseconds);
        continue;
      }
      throw ownerFailureV1(owner, raised, round, plan.decision === "stable", [...plan.rows, ...rows()].slice(0, ownerIsolationPolicyV1.replyRows * 8));
    }
  }
}

/** 🛟️ The failure of one owner's closure references as a fact about that owner: the named failure of this owner (see {@link observeRefsWithRetriesV1}, or a farm child's answer), its own source (see `ownerSourceFailureV1`), else the named failure of its inference as `ownerFailureAsWithheldV1` withholds it; the failures another layer follows (a farm job that failed, a deferred owner, a store that changed) answer `undefined`. */
export function ownerRefsFailureV1(raised, scope) {
  if (isOwnerFailureV1(raised) && raised.owners.length === 1 && raised.owners[0].name === scope.name) return ownerFailureAsWithheldV1(raised, scope);
  const own = ownerSourceFailureV1(raised, { workspaceRoot: scope.workspaceRoot, fallback: join(scope.workspaceRoot, scope.root, PROJECT_BASENAME) });
  if (own !== undefined) return own;
  if (["graph.farm-job-failed", "BUILD_OWNER_DEFERRED", "BUILD_STORE_CHANGED"].includes(raised?.code) || raised?.storeChanged === true) return undefined;
  return ownerFailureAsWithheldV1(raised, scope);
}

/** 🧷️ Reads the withholding source inside the observation of the project it withholds and refuses a withholding that rests on other bytes than the ones the project now holds: the project then stays stale until exactly that file changes, and a read that disagrees with the failure is a half-written source that settlement retries. */
function observeWithheldSourceV1(error) {
  let digest;
  try { digest = createHash("sha256").update(readFileSync(error.source)).digest("hex"); } catch { throw error; }
  if (digest !== error.sha256) throw error;
}

/** 🧾️ The facts that withhold one project outside its own observation: what the failed computation read, then the bytes of the source it names. */
function withheldRowsV1(error) {
  const failure = detachedFailureV1(error), rows = failure ? [...failure.rows, ...failure.layers.flat()] : [];
  if (Array.isArray(error.observedRows)) rows.push(...error.observedRows);
  const stat = rawStatSync(error.source), bytes = readFileSync(error.source);
  if (createHash("sha256").update(bytes).digest("hex") !== error.sha256) throw error;
  rows.push({ kind: "file", path: error.source, fact: { ctimeMs: stat.ctimeMs, ino: stat.ino, mtimeMs: stat.mtimeMs, sha256: error.sha256, size: stat.size } });
  return rows;
}

/** 📛️ Names the project a failing inference belongs to without replacing the error, so a repeated settlement round says which project stops the build. */
export async function tagProjectFailureV1(root, run) {
  try { return await run(); }
  catch (error) {
    if (error === null || typeof error !== "object") throw Object.assign(new Error(String(error)), { semioProject: root });
    if (Object.hasOwn(error, "semioProject")) throw error;
    if (!Object.isExtensible(error)) throw Object.assign(new Error(String(error.message ?? error), { cause: error }), { ...(typeof error.code === "string" ? { code: error.code } : {}), semioProject: root });
    Object.defineProperty(error, "semioProject", { value: root, configurable: true });
    throw error;
  }
}

/** 🗝️ The key of the project record of one owner root. */
function ownerCacheKeyV1(root) { return ownerRecordKeyV1(root); }

/** 🗂️ The project record path of one owner root, where its observations are recorded while it runs. */
function ownerCachePathV1(workspaceRoot, root) { return join(workspaceDataDirectory(workspaceRoot), "np", ownerCacheKeyV1(root) + ".json"); }

/** 🏷️ The owner kind whose memory peak is learned: the language tag of the project, else `language:cargo` for a project with a Cargo manifest at its native root (read outside the observation), else `project`; the root project of the repository (`.`) is a kind of its own (`root:` before the kind), because the closures of all its targets make it far larger than any crate and its cost must not decide for them. */
function ownerKindV1(json, workspaceRoot) { const kind = json.tags?.find(tag => tag.startsWith("language:")) ?? (rawStatSync(join(workspaceRoot, json.metadata?.nativeRoot ?? json.root, "Cargo.toml"), { throwIfNoEntry: false }) !== undefined ? "language:cargo" : "project"); return json.root === "." ? "root:" + kind : kind; }

/** 🧵️ Bounds concurrently inferred projects; one worker is the serial reference order. */
export function projectConcurrencyV1(options, environment = process.env) {
  const value = Math.min(options?.projectConcurrency ?? Math.max(1, Math.min(8, availableParallelism())), graphCutConcurrencyV1(environment));
  if (!Number.isSafeInteger(value) || value < 1 || value > 64) throw Error("Build project inference concurrency must be an integer from 1 to 64");
  return value;
}

/** 🛣️ The projects inferred at once by one read: `explicit` when the caller names it, else the cores (at most eight), the memory the parent may grow by divided by what one owner is measured to hold (`parentMegabytes` / `ownerMegabytes`), and never fewer than the cut of the load allows (`cut`, Infinity when uncut): the cut ladder halves the lanes after a plan overrun, but the governor admits every owner by its measured footprint, so a cut load keeps the lanes its memory fits (the worker that read with 1 lane held 1.3 to 3.4 GB of a 6.5 GB plan).
 * @see ./🧪️tests/🛣️lanes/🟦️.ts */
export function inferenceLanesV1({ cores, cut, parentMegabytes, ownerMegabytes, explicit }) {
  if (explicit !== undefined) return Math.max(1, Math.min(explicit, cut));
  const byCores = Math.max(1, Math.min(8, cores)), byMemory = Math.max(1, Math.floor(parentMegabytes / Math.max(1, ownerMegabytes || 800)));
  return Math.max(1, Math.min(byCores, Math.max(Number.isFinite(cut) ? cut : 8, byMemory)));
}

/** 🎞️ How many times this process entered the owner loop: a read that is repaired, retried or settled again enters it again and its `taken N of M` starts from the records already stored, so the slice report of a worker says how often that happened.
 * @see ../../../🧪️tests/🕸️graph/🩹️repair-rounds */
let ownerPassesV1=0;

/** 🪨️ Reports and cancels inference settlement rounds through the original invocation. */
export function settlementOptionsV1(context) {
  return { signal: context.phaseControl?.signal, onRound: async round => { writeGraphSettleRowV1(round); await yieldInferenceV1(context,"build:settle",0,true); } };
}

/** 📜️ The generator contracts of the workspace taxonomy, read through the observed source analysis in the scope of the caller: a graph construction reads them once and keys every project record with their digest. */
export function projectGeneratorContractsV1(workspaceRoot) {
  const contractPath = join(workspaceRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json");
  return existsSync(contractPath) ? readTaxonomyProjectionV1(contractPath).generatorContracts : {};
}



/** 🧱️ Builds, from plain data, what the inference of one project shares across the projects of a graph construction; a worker process opens the session its parent describes, the generator contracts included: they are no fact of a project record but part of its key, so a worker that read them again after a peer edit would write content of other contracts under the key of its parent. */
export function createProjectInferenceEnvironmentV1(session) {
  const { workspaceRoot, registry, projectsByRoot, componentRoots, contracts } = session;
  if (typeof session.closureStore === "string") bindClosureStoreV1(workspaceRoot, session.closureStore);
  if (contracts !== null && typeof contracts === "object") useGeneratorContractsV1(workspaceRoot, contracts);
  useAnalysisStoreV1(workspaceRoot, session.analysisStore ?? analysisStoreDirectoryOfV1(session.closureStore));
  const context = { workspaceRoot, registry, lateClosures: session.lateClosures === true }, scripts = createScriptInputCache(context), commandInputs = session.commandInputs?.closureRecipe ? closureHandleFromRecipeV1(session.commandInputs.closureRecipe) : session.commandInputs;
  if (contracts === null || typeof contracts !== "object" || Array.isArray(contracts)) throw refuseBuildV1("invalid","graph.generator-contracts-required");
  scripts.generatedOutput = (path, origin) => readBuildProducedCompileOutputV1(path, workspaceRoot, generatorContractsInScopeV1(contracts), registry, origin);
  return { workspaceRoot, contracts, commandInputs, scripts, projectsByRoot: new Map(projectsByRoot), componentRoots, nativeCache: new Map(), nativeClosures: new Map(), context };
}

/** 📜️ The generator contracts a verification of a stored record compares its reads with, by workspace root: the contracts of the read that runs in this process, else the contracts of the taxonomy file as it is now (kept while its size and modification time stay). A process that holds neither answers false, so the record is inferred again.
 * @see ../📜️contract-reads/🧬️schema/🔣️.json */
const readContractsV1 = new Map(), taxonomyContractsV1 = new Map();
/** 🗄️ The directory of the source analyses of a dataset of the graph store: `sa` beside the closure store `ce` of the same dataset, so it lies inside the graph partition image (the declared area `graph-inference`: budgeted, reclaimed, not logged by the volume), is shared by the parent, the farm children and the pool children that read the dataset, and is reached by a child through the closure store path its session carries (a child holds no custody and cannot ask the store for it). Undefined where the closure store is.
 * @see ../../../👁️observation/🗄️analysis-store/🧬️schema/🔣️.json */
export const analysisStoreDirectoryOfV1 = closureStore => typeof closureStore === "string" && closureStore !== "" ? join(dirname(closureStore), "sa") : undefined;

/** 🗄️ Makes this process use the source analyses of the dataset (`analysisStore`, a directory) unless `SEMIO_ANALYSIS_STORE` is `off`; none without a directory. */
export function useAnalysisStoreV1(workspaceRoot, analysisStore, environment = process.env) {
  useSourceAnalysisStoreV1(environment.SEMIO_ANALYSIS_STORE === "off" || typeof analysisStore !== "string" ? undefined : { root: workspaceRoot, directory: analysisStore });
}
export function useGeneratorContractsV1(workspaceRoot, contracts) { if (contracts === undefined) readContractsV1.delete(workspaceRoot); else readContractsV1.set(workspaceRoot, contracts); }
registerGlobalFactV1(GENERATOR_CONTRACTS_FACT_V1, (workspaceRoot, segments, fact) => {
  let contracts = readContractsV1.get(workspaceRoot);
  if (contracts === undefined) {
    const path = join(workspaceRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), status = statSync(path), stamp = status.size + ":" + status.mtimeMs, held = taxonomyContractsV1.get(workspaceRoot);
    if (held?.stamp !== stamp) taxonomyContractsV1.set(workspaceRoot, { stamp, contracts: projectGeneratorContractsV1(workspaceRoot) });
    contracts = taxonomyContractsV1.get(workspaceRoot).contracts;
  }
  return generatorContractReadHoldsV1(contracts, segments, fact);
});

/** 🌐️ Infers the node of one project: its defaults, its native or release preparation and its build modes; a source that does not parse withholds the project instead of failing the graph. What the inference reads of the generator contracts is recorded as one `global` row of the record, so the record key holds the owner's inputs only.
 * @see ../📜️contract-reads/🟨️.mjs */
export async function inferProjectBodyV1(environment, job, admission) {
  assertNotReadParentV1("infer", job.name);
  if (environment.contracts === null || typeof environment.contracts !== "object") return inferProjectBodyTrackedV1(environment, job, admission);
  const told = new Map();
  const tracked = trackGeneratorContractsV1(environment.contracts, (segments, fact) => {
    const scope = observationScopeV1(), key = segments.join("\u0001");
    let scopes = told.get(key);
    if (scopes === undefined) told.set(key, scopes = new WeakSet());
    if (scope !== undefined) { if (scopes.has(scope)) return; scopes.add(scope); }
    observeGlobalFactV1(GENERATOR_CONTRACTS_FACT_V1, environment.workspaceRoot, segments, fact);
  });
  return withGeneratorContractsScopeV1(tracked.view, () => inferProjectBodyTrackedV1({ ...environment, contracts: tracked.view }, job, admission));
}

async function inferProjectBodyTrackedV1(environment, job, admission) {
  const { workspaceRoot, contracts, commandInputs, scripts, projectsByRoot, componentRoots, nativeCache, nativeClosures, context } = environment, { name, root, json, native, pipeline, selectedTargets, selectedInputs } = job, began = performance.now(), began0 = Date.now();
  assertNotReadParentV1("infer", name);
  try {
    await yieldInferenceV1(context, "build:project-assembly", 0);
    const project = await projectWithDefaults(json, root, join(workspaceRoot, root), workspaceRoot, contracts, new Map(), commandInputs, scripts, undefined, "hash", admission ? selectedTargets ?? [] : undefined, selectedInputs);
    const prepare = (apply, predicate) => { const preparing = admission ? { ...project, targets: Object.fromEntries(Object.entries(project.targets).filter(([, target]) => predicate(target))) } : project; apply(preparing); project.targets = { ...project.targets, ...preparing.targets }; if (preparing.metadata !== undefined) project.metadata = preparing.metadata; };
    if (native) prepare(project => withNativePreparation(project, workspaceRoot, contracts, nativeCache, projectsByRoot, nativeClosures), target => target.metadata?.semio?.execution?.kind !== "source");
    if (pipeline) prepare(project => withReleasePipelinePreparation(project, workspaceRoot, contracts, componentRoots, nativeCache, nativeClosures), target => Boolean(target.metadata?.semio?.release));
    for (const [targetName, target] of Object.entries(project.targets)) project.targets[targetName] = loaded.targetBuildModes(targetName, target);
    return project;
  } catch (raised) {
    const aborted = context?.invocation?.control?.signal?.aborted === true || context?.phaseControl?.signal?.aborted === true, error = await ownerInferenceFailureV1(raised, { workspaceRoot, root, name, aborted, round: job.ownerRound ?? ownerIsolationPolicyV1.retryRounds, started: began0 });
    if (error === undefined) throw raised; const unavailable = unavailableProjectV1(root, name, workspaceRoot, error); observeWithheldSourceV1(error); return { unavailable }; }
  finally { environment.clock?.add(name, "inline", performance.now() - began); }
}

/** ♻️ Whether a stored project record may be reused: a failure is never reused, because a record that withholds its project rests on a source the failure may not have named completely, and the new inference costs one cached walk. */
export const projectRecordReusableV1 = value => value?.unavailable === undefined;

/** 🔑️ The key a project record is stored under: what the graph shares across projects, then everything the project itself is made of. */
export const projectRecordKeyV1 = (head, root, json, selectedTargets, selectedInputs) => [...head, root, json, selectedTargets ?? null, selectedInputs];

/** 🔑️ The key text a project record is filed under in the fleet cache by every machine that reads a graph: a graph read runs its owners inside a settlement round, where `observeInferenceComputationV1` files a nested project by the record stamp and not by the revision of the observation module, so a machine that pulls owners for the fleet must claim and publish under this text too: under the text of the revision its records were filed where no reader looked (the cache held a record of 366 of 424 owners under the revision key and the readers asked for the stamp key). @see ./📥️take/🧪️tests */
export const ownerRecordKeyTextV1 = key => JSON.stringify([observationSchemaV1.$defs.record.const.stamp, null, key]);

/** ✍️ Writes the persisted record of one project into the graph directories the running body is staged to, under the key and the observation scope the parent uses, so the parent finds it valid once it adopted the leaves. What the job leaves in the cache shared by the owners of this child (the module graph, the closure walks, the source reads) is released once it grew by `lookReleaseGrowthBytes` beyond its lowest reading, like after a look or a warm job: the child serves many owners one after the other and kept the state of every one of them until its memory made the pool recycle it, so a heavy owner then landed on top of everything the earlier ones left. The project is inferred inside one closure pass, as the parent does for the whole of its discovery: a shared closure entry is verified once for the project, so a peer that changes a provider file while the project is inferred cannot make the entry hold when it is admitted and not hold when it is expanded (`closure.provider-entry-stale` of a worker); the parent verifies the record it adopts. */
export async function produceProjectRecordV1(environment, job) {
  const cachePath = join(workspaceDataDirectory(environment.workspaceRoot), "np", job.cacheKey + ".json");
  try { await observeOwnerAttemptsV1(round => withClosurePassV1(() => observeInferenceSnapshotV1(() => observeProjectInferenceV1({ cachePath, key: projectRecordKeyV1(job.head, job.root, job.json, job.selectedTargets, job.selectedInputs), owner: job.root, stamps: projectStampPathsV1({ native: job.native }), reusable: projectRecordReusableV1 }, () => inferProjectBodyV1(environment, { ...job, ownerRound: round }, job.admission === true))), { verdicts: job.verdicts })); } finally { releaseAfterTakeLookV1(environment.scripts, { shrink: false }); }
}

/** 🔱️ Settles one owner for the warm pass in a farm child: reads its stored record from the real graph store (the staging directory of a child holds nothing of the parent's), brings the closure entries it names up to date inside one closure pass (the stale ones are derived here, in the child's capped process, and saved to the closure store, which this child binds writable) and appends one durable row. The reply carries a tiny `weight`, so a child that serves hundreds of such jobs is recycled by its memory and not as if each were a derivation, and `peakMegabytes`, the memory this process grew by.
 * @see ./🔱️warm/🟨️.mjs */
export async function warmProjectRecordV1(environment, job) {
  const startRss = process.memoryUsage().rss, startPeak = process.resourceUsage().maxRSS * 1024, owner = ownerJobV1(environment, job.globals, { name: job.name, json: job.json });
  const ports = { keyText: each => ownerRecordKeyTextV1(projectRecordKeyV1(each.head, each.root, each.json, undefined, each.selectedInputs)), read: path => readStoredRecordV1(path, environment.workspaceRoot), settle: (observations, value) => settleClosureRowsV1(observations, value) };
  let done;
  if (typeof job.closureStore === "string") bindClosureStoreV1(environment.workspaceRoot, job.closureStore, true);
  closureApiV1(environment.scripts, environment.workspaceRoot);
  try { done = await withClosurePassV1(() => observeInferenceSnapshotV1(() => warmOwnerV1({ ...owner, warmKey: job.warmKey }, { directory: job.directory, cachePath: job.cachePath, ports }))); } finally { releaseAfterTakeLookV1(environment.scripts, { shrink: false }); }
  const endPeak = process.resourceUsage().maxRSS * 1024, top = Math.max(process.memoryUsage().rss, endPeak > startPeak ? endPeak : 0);
  return { names: [], outcome: done.outcome, cpuMilliseconds: done.cpuMilliseconds, peakMegabytes: Math.max(0, Math.round((top - startRss) / 1048576)), weight: 0.01 };
}

/** 🔭️ Looks at the owners of one batch in a farm child, the only place a look runs while a farm exists: for each owner the child derives what the parent used to derive (native or release preparation, the assembly authority, the head of the record key) from the global parts the parent sent, verifies the stored record and resolves the closure references, and answers per owner `{ name, native, pipeline, head, verdict, headerDigest, refs }`; an owner it could not look at is answered `{ name, failed }` and does not fail the others of its batch. The reply carries a small `weight` per owner.
 * @see ./🔭️look-queue/🧪️tests/🟦️.ts */
export async function lookOwnersV1(environment, job) {
  const cpu = process.cpuUsage(), startRss = process.memoryUsage().rss, startPeak = process.resourceUsage().maxRSS * 1024, looks = [];
  for (const owner of job.owners) {
    try {
      const prepared = await observeInferenceSnapshotV1(() => ownerJobV1(environment, job.globals, owner));
      const { names, cpuMilliseconds, peakMegabytes, weight, ...said } = await observeProjectRecordV1(environment, { ...prepared, cachePath: owner.cachePath, closureStore: job.closureStore, verdicts: job.verdicts, refs: job.refs });
      looks.push({ name: owner.name, native: prepared.native, pipeline: prepared.pipeline, head: prepared.head, ...said });
    } catch (error) { looks.push({ name: owner.name, failed: String(error?.message ?? error).replace(/\s+/gu, " ").slice(0, 300) }); }
  }
  const used = process.cpuUsage(cpu), endPeak = process.resourceUsage().maxRSS * 1024, top = Math.max(process.memoryUsage().rss, endPeak > startPeak ? endPeak : 0);
  return { names: [], looks, cpuMilliseconds: Math.round((used.user + used.system) / 1000), peakMegabytes: Math.max(0, Math.round((top - startRss) / 1048576)), weight: 0.01 * job.owners.length };
}

/** ⏳️ How long a reader waits for the verdict of a farm child before it verifies the record itself: a child that derives stale closure entries or is lost must never hold a take lane for ever. */
export const OWNER_VERDICT_WAIT_MILLISECONDS_V1 = 180_000;

/** ⏳️ The answer of `work` within `milliseconds`, undefined when it is late or fails (the late answer is dropped). */
export const verdictWithinV1 = (work, milliseconds) => new Promise(resolve => { const timer = setTimeout(() => resolve(undefined), milliseconds); Promise.resolve(work).then(resolve, () => resolve(undefined)).finally(() => clearTimeout(timer)); });

/** 🤝️ Verifies one stored owner record in a farm child, the only verification it gets in a take: the child reads the record plainly from the real graph store, settles its closure entries (the stale ones are derived here and saved to the closure store, which this child binds writable) and verifies every observed row, then answers the verdict of `verifyStoredProjectRecordV1` (`holds`, `stale`, `other`, `missing`) with the digest of the record header it judged, so the reader that asked adopts the record without settling, verifying or probing anything itself. The reply carries a tiny `weight` like a warm job.
 * @see ../../../👁️observation/🧪️tests/🤝️adopted-verdict */
export async function observeProjectRecordV1(environment, job) {
  const cpu = process.cpuUsage(), startRss = process.memoryUsage().rss, startPeak = process.resourceUsage().maxRSS * 1024;
  if (typeof job.closureStore === "string") bindClosureStoreV1(environment.workspaceRoot, job.closureStore, true);
  closureApiV1(environment.scripts, environment.workspaceRoot);
  let verdict;
  let refs;
  try { verdict = await withClosurePassV1(async () => { const said = await observeInferenceSnapshotV1(() => verifyStoredProjectRecordV1({ cachePath: job.cachePath, key: projectRecordKeyV1(job.head, job.root, job.json, undefined, job.selectedInputs) })); if (job.refs === true && typeof job.verdicts === "string" && said.verdict === "holds") { try { refs = await observeRefsWithRetriesV1(() => observeInferenceSnapshotV1(() => resolveOwnerRefsV1(environment, job, said.headerDigest)), { owner: job.name, scope: { workspaceRoot: environment.workspaceRoot, fallback: join(environment.workspaceRoot, job.root, PROJECT_BASENAME) } }); } catch (error) { refs = refsFailureV1(error, { workspaceRoot: environment.workspaceRoot, fallback: join(environment.workspaceRoot, job.root, PROJECT_BASENAME) }); } } return said; }, { verdicts: job.verdicts }); } finally { releaseAfterTakeLookV1(environment.scripts, { shrink: false }); }
  const used = process.cpuUsage(cpu), endPeak = process.resourceUsage().maxRSS * 1024, top = Math.max(process.memoryUsage().rss, endPeak > startPeak ? endPeak : 0);
  return { names: [], ...verdict, ...(refs === undefined ? {} : { refs }), cpuMilliseconds: Math.round((used.user + used.system) / 1000), peakMegabytes: Math.max(0, Math.round((top - startRss) / 1048576)), weight: 0.01 };
}

/** 🪘️ What a farm child answers instead of a reference when it could not resolve the closure references of an owner: a source that does not parse (the parent withholds the owner from it) or any other failure (named, the parent asks again). */
export function recordHeaderDigestV1(cachePath) {
  try { const text = refsBrotliV1(rawReadFileSync(cachePath)).toString("utf8"), cut = text.indexOf("\n"); return cut < 0 ? undefined : createHash("sha256").update(text.slice(0, cut)).digest("hex"); } catch { return undefined; }
}

/** 🪘️ What a farm child answers instead of a reference (second form). */
export function refsFailureV1(raised, scope) {
  if (isOwnerFailureV1(raised)) return { ownerFailed: ownerFailureReplyV1(raised) };
  const error = scope === undefined ? raised : ownerSourceFailureV1(raised, scope) ?? raised;
  if (error?.code === "BUILD_SOURCE_UNPARSEABLE") return { unparseable: { source: String(error.source), sha256: String(error.sha256), message: String(error.message).slice(0, 2048) } };
  return { failed: String(error?.message ?? error).replace(/\s+/gu, " ").slice(0, 300) };
}

/** 🪘️ Asks farm children for the closure references of one owner until one answers a reference or a withheld source, up to `attempts` times; undefined when no farm exists any more (the parent then resolves them, as it always did without a farm), a refusal naming the owner and the last failure when every try failed. @param {{ observe: (job: object) => Promise<object | undefined>, exists: () => boolean }} farm */
export async function askOwnerRefsV1({ farm, job, headerDigest, attempts = 3 }) {
  let last = "no answer";
  for (let attempt = 0; attempt < attempts; attempt++) {
    if (!farm.exists()) return undefined;
    const reply = await farm.observe({ ...job, observe: true, refs: true });
    if (reply === undefined) return undefined;
    const named = reply.refs?.ownerFailed === undefined ? undefined : ownerFailureOfReplyV1(job.name, reply.refs.ownerFailed);
    if (named !== undefined) throw named;
    if (reply.verdict === "holds" && reply.headerDigest === headerDigest && reply.refs !== undefined && reply.refs.failed === undefined) return { refs: reply.refs, headerDigest };
    last = reply.refs?.failed ?? "verdict " + reply.verdict;
    if (reply.verdict === "stale" || reply.verdict === "other" || reply.verdict === "missing") break;
  }
  throw Object.assign(new Error("farm children could not resolve the closure references of " + job.name + " (" + last + "); the parent does not resolve them / Farm-Kindprozesse konnten die Closure-Referenzen von " + job.name + " nicht auflösen (" + last + "); der Elternprozess löst sie nicht auf"), { code: "graph.farm-job-failed", semioProject: job.root });
}

/** 🪘️ What the parent of a read did with the closure references of its owners: `adopted` from a farm child (the child resolved them and filed the result), `resolved` by the parent itself (no child, no result or a result that did not hold). */
export const ownerRefsCountsV1 = { adopted: 0, resolved: 0 };

/** 🪘️ The file a farm child files the resolved result of one owner in: inside the verdict directory of the read, named by the digest of the header of the record it resolved. */
export const ownerRefsFileV1 = (directory, headerDigest) => join(directory, "refs-" + headerDigest);

/** 🪘️ Resolves the closure references of the stored record whose header digest is `headerDigest` in this process (a farm child that just verified it) and files the resolved project and the plan of its rows in the verdict directory of the read; answers the reference the parent keeps (path, digest of the file, size, header digest, the process that resolved it) or undefined when the record changed, is withheld, has a layer that is not stored or there is no read directory. The parent streams the file only when it assembles the owner.
 * @see ../🧪️tests/🪘️refs-file */
export async function resolveOwnerRefsV1(environment, job, headerDigest) {
  const directory = closureVerdictScopeV1();
  if (directory === undefined || typeof headerDigest !== "string") return undefined;
  let text; try { text = refsBrotliV1(rawReadFileSync(job.cachePath)).toString("utf8"); } catch { return undefined; }
  const cut = text.indexOf("\n");
  if (cut < 0 || createHash("sha256").update(text.slice(0, cut)).digest("hex") !== headerDigest) return undefined;
  let value; try { value = JSON.parse(text.slice(cut + 1)); } catch { return undefined; }
  if (value === null || typeof value !== "object" || value.unavailable !== undefined) return undefined;
  const resolved = await resolveClosureRefsPlanV1(value, environment.workspaceRoot, environment.scripts, "hash");
  if (resolved === undefined) return undefined;
  const body = JSON.stringify({ v: 1, headerDigest, resolver: process.pid, project: resolved.project, plan: resolved.plan }), path = ownerRefsFileV1(directory, headerDigest), temporary = path + "." + process.pid + ".tmp";
  rawWriteFileSync(temporary, body); rawRenameSync(temporary, path);
  return { path, digest: createHash("sha256").update(body).digest("hex"), size: Buffer.byteLength(body), headerDigest, resolver: process.pid };
}

/** 🪘️ The resolved result a farm child filed for the record the parent adopted, as the closure resolution of the parent would have answered it: the project and rows and derived facts listed in `log` from the closure store. Undefined (the parent then resolves itself) when the file is gone, torn or not the one the reference names, when the record it was made from is not the one the parent holds, or when a layer of the store is lost; the file is removed once read.
 * @see ../🧪️tests/🪘️refs-file */
export function adoptOwnerRefsV1(refs, headerDigest, workspaceRoot, scripts, log) {
  if (log === undefined || refs === undefined || refs.headerDigest !== headerDigest) return undefined;
  let body; try { body = rawReadFileSync(refs.path); } catch { return undefined; }
  try { rawRmSync(refs.path, { force: true }); } catch {}
  if (body.length !== refs.size || createHash("sha256").update(body).digest("hex") !== refs.digest) return undefined;
  try { const parsed = JSON.parse(body.toString("utf8")); if (parsed.headerDigest !== headerDigest) return undefined; const adopted = adoptClosureRefsV1(parsed.plan, workspaceRoot, scripts, log); return { project: parsed.project, rows: adopted.rows, derived: adopted.derived, resolver: parsed.resolver }; } catch { return undefined; }
}

/** 🧷️ What an owner admission selects of one project: the selected targets (`undefined` when the admission names none for it), the sorted selected inputs and the project JSON reduced to the selected targets. The one function the reading machine and a pool child share, so both file the record under the same key.
 * @see ./🧪️tests/🤝️pool-admission/🟦️.ts */
export function ownerAdmissionSelectionV1(admission, json) {
  const selectedTargets = admission?.targets[json.name], selectedInputs = [...new Set(admission?.inputs?.[json.name] ?? [])].sort(), activeJson = admission ? { ...json, targets: Object.fromEntries(Object.entries(json.targets ?? {}).filter(([name]) => selectedTargets?.includes(name))) } : json;
  return { selectedTargets, selectedInputs, activeJson };
}

/** 🧷️ Whether the selected part of a project is compiled (not only read as source): the decision of an admission read for `native`, shared with the pool child.
 * @see ./🧪️tests/🤝️pool-admission/🟦️.ts */
export const ownerAdmissionCompilesV1 = (activeJson, selectedInputs, nativeCandidate, nativeTargets) => buildNativeSourceObservationV1({ selectedSource: Object.values(activeJson.targets ?? {}).some(target => target.metadata?.semio?.execution?.kind === "source"), selectedNative: nativeCandidate && nativeTargets.length > 0, requestedInputs: selectedInputs.length > 0, dependencyInputs: false, inputRole: buildInputSourceRoleV1(Object.keys(activeJson.targets ?? {}).length > 0, false, Object.values(activeJson.targets ?? {}).map(target => target.metadata?.semio?.sourceInputObservation)) }) === "compile";

/** 📦️ What a reader of an admission hands a pool wave about one owner so a machine that pulls it files the same record: the targets and inputs the admission selected for it and the generated task dependencies it applies. */
export const ownerAdmissionOfV1 = (admission, job) => ({ ...admission.targets[job.json.name] === undefined ? {} : { targets: admission.targets[job.json.name] }, inputs: [...new Set(admission.inputs?.[job.json.name] ?? [])].sort(), ...admission.generatedDependencies?.[job.name] ? { generated: admission.generatedDependencies[job.name] } : {} });

/** 🏷️ The job of one owner of a wave that is inferred without an admission selection, from its project JSON and the head parts the wave shares: the same fields the wave computes for the owner it infers itself (native or release preparation, the assembly authority, the head of the record key and the record file name), so a machine that pulls the owner from the fleet pool files its record under the key the machine that runs the wave looks up. `environment` is the one of {@link createProjectInferenceEnvironmentV1}. */
export function ownerJobV1(environment, globals, { name, json: raw, admission }) {
  const { workspaceRoot, projectsByRoot, componentRoots, nativeCache, nativeClosures } = environment, json = admission?.generated ? applyBuildGeneratedTaskDependenciesV1(raw, admission.generated) : raw, root = json.root, nativeRoot = json.metadata?.nativeRoot ?? root;
  const { selectedTargets, selectedInputs, activeJson } = ownerAdmissionSelectionV1(admission === undefined ? undefined : { targets: { [json.name]: admission.targets }, inputs: { [json.name]: admission.inputs } }, json);
  const nativeTargets = Object.entries(activeJson.targets ?? {}).filter(([, target]) => target.metadata?.semio?.execution?.kind !== "source");
  const nativeCandidate = existsSync(join(workspaceRoot, nativeRoot, "Cargo.toml")) || nativeTargets.some(([, target]) => target.options?.command?.includes("⚡️caching/🦀️cargo/📜️script.ts")), native = admission === undefined ? nativeCandidate : ownerAdmissionCompilesV1(activeJson, selectedInputs, nativeCandidate, nativeTargets), pipeline = Object.values(activeJson.targets ?? {}).some(target => target.metadata?.semio?.release);
  const assemblyAuthority = native || pipeline ? projectAssemblyAuthorityV1(workspaceRoot, native ? nativeRoot : undefined, projectsByRoot, pipeline ? componentRoots : undefined, nativeCache, nativeClosures) : null;
  const head = ownerRecordHeadV1(globals, { native, pipeline, assemblyAuthority });
  return { name, root, json, native, pipeline, nativeRoot, selectedTargets, selectedInputs, head, cacheKey: ownerCacheKeyV1(root), ...admission === undefined ? {} : { admission: true } };
}

/** 🔁️ Opens the answer of derived facts for a generation: the same inference environment as a build, so asking a command walk again reads the live files exactly as the inference that recorded it did; every question runs in its own observation snapshot. */
export async function openGenerationDeriverV1({ workspaceRoot, registry }) {
  const environment = await observeInferenceSnapshotV1(() => createProjectInferenceEnvironmentV1({ workspaceRoot, registry, contracts: projectGeneratorContractsV1(workspaceRoot), projectsByRoot: [], componentRoots: [], commandInputs: null, lateClosures: false }));
  const derive = (row, signal, pass) => { signal?.throwIfAborted(); return isHeavyQuestionV1(row) ? askHeavyQuestionV1(row, { workspaceRoot, signal }) : observeInferenceSnapshotV1(() => deriveClosureFactV1(row, environment.scripts, pass)); };
  return Object.defineProperty(derive, "scripts", { value: environment.scripts });
}

/** 🧺️ Runs the whole-workspace setup of an inference once per share: a later call of the same command (the build after the prefetch) reuses it while every observation it was made from still holds, replays those observations into its own capture, and counts `computed` and `reused` on the share. */
export async function sharedSetupV1(share, key, compute, checkpoint) {
  const before = share.computed;
  const value = await memoizedObservedAsyncComputationV1(share.cache, key, async () => { share.computed++; return compute(); }, checkpoint);
  if (share.computed === before) share.reused++;
  return value;
}

/** 🐌️ The recorded seconds from which an owner counts as slow: it runs after the owners that take seconds, so the minutes of its synchronous stretches never hold back the light owners and their workers. */
export const OWNER_SLOW_SECONDS_V1 = 120;

/** ⏲️ The farm with its wait charged to the owner clock under `farm` (the same farm when there is none). */
export function timedFarmV1(farm, clock) {
  if (farm === undefined) return undefined;
  return Object.freeze({ ...farm, prepare: async job => { const at = performance.now(); try { return await farm.prepare(job); } finally { clock.add(job.name, "farm", performance.now() - at); } } });
}

const PARKED_SECONDS_V1 = 120, PARKED_REPEAT_SECONDS_V1 = 60;
const STAGE_INFER_V1 = { en: "inferring in this process (one thread: closure walks and hashing hold it, a busy machine slows it)", de: "Inferenz in diesem Prozess (ein Thread: Closure-Läufe und Hashes halten ihn, eine ausgelastete Maschine bremst ihn)" }, STAGE_FARM_V1 = { en: "waiting for a farm child to infer the owner, or for one to start (the host capacity governor holds a child while the machine lacks free memory)", de: "wartet auf einen Farm-Kindprozess, der den Owner ableitet, oder auf dessen Start (die Kapazitätssteuerung hält einen Kindprozess, solange der Maschine freier Speicher fehlt)" }, STAGE_RESUME_V1 = { en: "the farm child is done; waiting for the memory governor to let the owner back into the process", de: "der Farm-Kindprozess ist fertig; wartet darauf, dass die Speichersteuerung den Owner wieder in den Prozess lässt" }, STAGE_REFS_V1 = { en: "resolving closure references in the parent (a first derivation of a shared closure runs on the one thread)", de: "löst Closure-Referenzen im Elternprozess auf (die erste Ableitung einer geteilten Closure läuft im einen Thread)" }, STAGE_ROWS_V1 = { en: "recording the observations of the owner", de: "zeichnet die Beobachtungen des Owners auf" };
const STAGE_VERDICT_V1 = { en: "waiting for a farm child to verify the stored record (the parent verifies nothing)", de: "wartet auf einen Farm-Kindprozess, der den gespeicherten Eintrag prüft (der Elternprozess prüft nichts)" }, STAGE_REFS_FARM_V1 = { en: "waiting for a farm child to resolve the closure references (the parent resolves nothing)", de: "wartet auf einen Farm-Kindprozess, der die Closure-Referenzen auflöst (der Elternprozess löst nichts auf)" }, STAGE_LOOK_V1 = { en: "waiting for a farm child to look at the owner: assembly authority, stored record and closure references (the parent computes nothing)", de: "wartet auf einen Farm-Kindprozess, der das Projekt prüft: Zusammenbau-Autorität, gespeicherter Eintrag und Closure-Referenzen (der Elternprozess rechnet nichts)" };
/** 🛑️ The refusal of an owner job that no farm child could do: the owner is set aside with this reason and the read goes on (see `setAside`). */
export function farmJobRefusedV1(name, root, what) {
  const text = language => language === "de" ? "Farm-Kindprozesse konnten " + what + " von " + name + " nicht liefern; der Elternprozess rechnet nicht, das Projekt wird zurückgestellt" : "farm children could not deliver " + what + " of " + name + "; the parent computes nothing, the owner is set aside";
  return Object.assign(new Error(text("en") + " / " + text("de")), { code: "graph.farm-job-failed", semioProject: root });
}
const stagesV1 = new Map();
const projectSizesV1 = { count: 0, bytes: 0, top: [] };
/** 📏️ Notes the real JSON size of one resolved project for the slice memory report: the total and the five largest (a resolved project is what a cold part keeps per owner). */
const noteProjectSizeV1 = (name, value) => { let bytes = 0; try { bytes = JSON.stringify(value).length; } catch {} projectSizesV1.count++; projectSizesV1.bytes += bytes; projectSizesV1.top.push({ name: String(name).slice(0, 80), bytes }); projectSizesV1.top.sort((a, b) => b.bytes - a.bytes); if (projectSizesV1.top.length > 5) projectSizesV1.top.length = 5; };
const stageV1 = (name, text) => { stagesV1.set(name, text); };

/** 🧭️ Runs the farm preparation of one owner through `away` and names each wait of it for the progress line, in both languages: first the farm child (or its start), then the memory governor that lets the owner back into the process, then the process itself. */
export async function stagedPrepareV1(name, away, work) {
  stageV1(name, STAGE_FARM_V1);
  try { return await away(async () => { try { return await work(); } finally { stageV1(name, STAGE_RESUME_V1); } }); }
  finally { stageV1(name, STAGE_INFER_V1); }
}

/** 🅿️ The owners of a wave that have run for `PARKED_SECONDS_V1` with no observation row and why: the stage they are in (a farm child, the memory governor, the closure derivation of the parent, the recording of rows), one bilingual line per owner at most every `PARKED_REPEAT_SECONDS_V1`. `say` writes a line; `told` remembers when a name was last named. */
export function reportParkedOwnersV1(list, told, say, now = performance.now(), detail = undefined) {
  for (const row of list) {
    if (!(row.seconds >= PARKED_SECONDS_V1) || (row.rows ?? 0) > 0 || now - (told.get(row.name) ?? -Infinity) < PARKED_REPEAT_SECONDS_V1 * 1000) continue;
    told.set(row.name, now);
    const stage = stagesV1.get(row.name) ?? STAGE_INFER_V1;
    say(`graph: ${row.name} has run ${Math.round(row.seconds)} s with 0 rows and is parked: ${stage.en}${detail ? " [" + detail + "]" : ""} / Graph: ${row.name} läuft seit ${Math.round(row.seconds)} s ohne Zeile und ist geparkt: ${stage.de}${detail ? " [" + detail + "]" : ""}`);
  }
}

/** 🧭️ The order owners start in: the owners the cost table says need a solo grant (`alone`) last, before them the ones recorded as slow (`slow`), the held requests first (`rank`) within each class, then the owners whose stored record is present (`present`, names) before those without one, so the cheap ones finish and publish first; the order inside a class is kept. */
export function orderOwnerJobsV1(jobs, { alone = new Set(), slow = new Set(), rank, present } = {}) {
  const tier = job => alone.has(job.name) ? 2 : slow.has(job.name) ? 1 : 0, cold = job => present === undefined || present.has(job.name) ? 0 : 1;
  return jobs.sort((a, b) => tier(a) - tier(b) || (rank === undefined ? 0 : rank(a.name) - rank(b.name)) || cold(a) - cold(b));
}

/** 🏠️ The cost table with only the samples that this machine or a smaller one measured: what a bigger host measured (a lax heap that shares its warm-up) informs an owner nobody here ran, but it is no evidence that the owner does not fit this machine, so it never bars an owner here; a stop that this machine records after a real overrun is its own evidence.
 * @see ./🧪️tests/🌍️bigger-host/🟦️.ts */
export function ownHostTableV1(table, hostMegabytes) {
  return { ...table, owners: Object.fromEntries(Object.entries(table.owners ?? {}).map(([name, entry]) => [name, { ...entry, samples: entry.samples.filter(sample => sample.hostMegabytes <= hostMegabytes) }])) };
}

/** 🔁️ The owners a read that waited for the fleet in vain runs here once, alone, before it refuses: all the owners it waited for the first time, none the second time. A pure function.
 * @see ./🧪️tests/🔁️retry-alone/🟦️.ts */
export function heavyRetryNamesV1(left, retried) { return retried ? [] : [...left]; }

/** 🧱️ The most one owner may take while it runs alone on a machine that has a plan limit: the solo grant, but never more than the share `planShareBasisPoints` of what the plan guard allows the whole plan, and never less than the grant of an owner beside others, so the owner's own limit always stops it before the plan guard blames the plan. An unlimited plan leaves the solo grant. */
export function hereLimitBytesV1({ limitBytes, soloBytes, planBytes, planShareBasisPoints = 9000 }) {
  const capped = Number.isFinite(planBytes) ? Math.min(soloBytes, Math.floor(planBytes * planShareBasisPoints / 10000)) : soloBytes;
  return Math.max(limitBytes, capped);
}

/** 📏️ The peak a farm child reports (its physical footprint in bytes) in the unit of the cost table, the managed bytes: divided by the footprint factor, so `known` multiplies it back to the footprint once and not twice; undefined stays undefined, no factor leaves the bytes. */
export function ownerMeasuredManagedBytesV1(bytes, factor) { return bytes === undefined ? undefined : factor > 0 ? bytes / factor : bytes; }

/** 🌊️ The free memory of the machine in megabytes, `undefined` where it cannot be read: the farm of a read is widened by it, so a rebuild reservation shrunk by a busy moment does not leave the whole read to one thread while the machine is idle.
 * @see ../../../🧠️memory/🪜️plan-stage/🟨️.mjs */
async function hostAvailableMegabytesV1() { try { const sampling = await import("../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts"), bytes = sampling.hostAvailableBytesV1(); return Number.isFinite(bytes) ? Math.floor(bytes / 1048576) : undefined; } catch { return undefined; } }

/** 🪙️ What one project may take on this machine and where the cost of every owner is recorded: the limit is the grant of the machine for one project (`ownerGrantV1` of half its memory, never below the growth budget of the parent), none on a serving machine or with `SEMIO_GRAPH_OWNER_LIMIT=off`; a cost module that cannot be loaded enforces nothing and records nothing (a read never depends on it). A limited machine can read alone: `heavy(names)` names the owners whose recorded cost does not fit that grant, which the inference runs one at a time after the others, and `soloLimitBytes()` is the grant of one owner while nothing else runs (`soloGrantV1` of the memory available at that moment, never below the limit), taken when the solo owner starts. @param {(() => number | undefined) | undefined} availableBytes the available memory of the host in bytes */
export async function graphOwnerEnforcementV1(workspaceRoot, environment, budgetBytes, rowsOf, availableBytes, hostMegabytes) {
  if (environment.SEMIO_GRAPH_OWNER_LIMIT === "off") return {};
  try {
    const [cost, wanted, sampling] = await Promise.all([import("../../../🪪️graph-state/🙋️wanted/💰️cost/🟦️.ts"), import("../../../🪪️graph-state/🙋️wanted/🟦️.ts"), import("../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts")]);
    const host = (hostMegabytes ?? hostMemoryMegabytesV1)(), grant = cost.ownerGrantV1(Math.floor(host / 2), wanted.wantedPolicyV1), context = cost.ownerCostContextV1(), recorder = cost.createOwnerCostRecorderV1({ path: cost.ownerCostPathV1(workspaceRoot), rowsOf, ceilingMegabytes: context.ceilingMegabytes });
    const slow = names => { const owners = recorder.table().owners ?? {}; return names.filter(name => (owners[name]?.seconds ?? 0) >= OWNER_SLOW_SECONDS_V1); };
    if (!Number.isFinite(grant.managedMegabytes)) return { cost, recorder, limitBytes: undefined, slow };
    const limitBytes = Math.max(grant.managedMegabytes * 1048576, budgetBytes), available = availableBytes ?? sampling.hostAvailableBytesV1;
    const soloBytes = () => { const bytes = available(); return cost.soloGrantV1({ hostMegabytes: host, ...(bytes === undefined ? {} : { availableMegabytes: Math.floor(bytes / 1048576) }) }, wanted.wantedPolicyV1).managedMegabytes * 1048576; };
    const stoppedOf = name => { const entry = recorder.table().owners[name], stopped = entry === undefined ? [] : entry.samples.filter(sample => sample.stopped && sample.atMs >= STOPS_ATTRIBUTABLE_FROM_MS_V1); return stopped.length === 0 ? undefined : Math.round(cost.estimateOwnerCostV1({ ...recorder.table(), owners: { [name]: { ...entry, samples: stopped } } }, name, undefined, undefined, { ceilingMegabytes: context.ceilingMegabytes, nowMs: Date.now() }).megabytes * cost.ownerCostPolicyV1.footprintFactor); };
    return {
      cost, recorder, limitBytes,
      slow,
      recordedBytes: name => { const row = cost.estimateOwnerCostV1(recorder.table(), name, undefined, undefined, { ceilingMegabytes: context.ceilingMegabytes, nowMs: Date.now() }); return row.source === "recorded" ? row.megabytes * 1048576 : undefined; },
      stopped: stoppedOf,
      known: name => { const row = cost.estimateOwnerCostV1(recorder.table(), name, undefined, undefined, { ceilingMegabytes: context.ceilingMegabytes, nowMs: Date.now() }); return cost.ownerCostPolicyV1.heavySources.includes(row.source) ? Math.round(row.megabytes * cost.ownerCostPolicyV1.footprintFactor) : undefined; },
      neverHere: (names, childFootprintMegabytes) => { const here = hereLimitBytesV1({ limitBytes, soloBytes: soloBytes(), planBytes: planStageLimitBytesV1(environment) }), table = ownHostTableV1(recorder.table(), context.hostMegabytes ?? host), estimate = (source, name) => cost.estimateOwnerCostV1(source, name, undefined, undefined, { ceilingMegabytes: context.ceilingMegabytes, nowMs: Date.now() }); return names.filter(name => { if (childFootprintMegabytes !== undefined) { const footprint = stoppedOf(name); return footprint !== undefined && footprint > childFootprintMegabytes; } const row = estimate(table, name); if (row.source !== "recorded") return false; return row.saturated === true || row.megabytes * 1048576 > here; }); },
      heavy: names => cost.classifyOwnersV1(names, name => cost.estimateOwnerCostV1(recorder.table(), name, undefined, undefined, { ceilingMegabytes: context.ceilingMegabytes, nowMs: Date.now() }), grant).heavy.map(row => row.name),
      soloLimitBytes: () => hereLimitBytesV1({ limitBytes, soloBytes: soloBytes(), planBytes: planStageLimitBytesV1(environment) }),
    };
  } catch { return {}; }
}


/** 🧾️ The project configuration candidates of a read: the listed files and the sibling `📋️project.json` of every listed Cargo manifest, in an admission read as in any other, so the owner of a manifest is declared at its root and no project is built under the package name there.
 * @see ./🧪️tests/🦀️cargo-stand-in/🟦️.ts */
export function projectConfigCandidatesV1(configFiles, workspaceRoot) {
  return inferenceCandidates([...configFiles, ...configFiles.filter(file => file.endsWith("Cargo.toml")).map(file => buildPath(join(dirname(file), PROJECT_BASENAME)))], workspaceRoot);
}

/** 🛡️ Infers the project nodes of every authored emoji-named project config. Nx's native walker sometimes hands back lossy paths with U+FFFD where a multi-byte emoji used to be: those are unopenable duplicates of a real file and are skipped. */
export function emojiProjectJsonNodes(configFiles, options, context) { return withReadParentScopeV1(readScope=>emojiProjectJsonNodesBodyV1(configFiles, options, context, readScope)); }

/** 🛡️ The read of {@link emojiProjectJsonNodes} inside the scope of its parent: the computations of a farm child are refused only in what this read started, and only while its farm exists. */
async function emojiProjectJsonNodesBodyV1(configFiles, options, parentContext, readScope) {
  const farmStop = new AbortController();
  const waveSignal = AbortSignal.any([farmStop.signal, parentContext.ownerSignal, parentContext.invocation?.control.signal, parentContext.phaseControl?.signal].filter(Boolean));
  waveSignal.throwIfAborted();
  const context = { ...parentContext, ownerSignal: waveSignal };
  const { workspaceRoot } = context, unavailable = new Map(), wantedRoots = options?.ownerRoots, wanted = root => !wantedRoots || wantedRoots.has(root), enteredMs = performance.now();
  const collector = createErrorCollectorFromEnvV1({ workspaceRoot, signal: context.invocation?.control.signal }), clock = createOwnerClockV1();
  const isAuthoredProjectConfigV1 = configFile => ROSTER_RULES_V1.isAuthoredProjectConfig(configFile);
  const sharedSetup=!options?.ownerAdmission&&context.setupShare?.cache instanceof Map?context.setupShare:undefined;
  const setupPass=async()=>{
    const catalog=await catalogBuildContributionsV1(workspaceRoot,options?.catalogProviders??[]);
    const found={files:projectConfigCandidatesV1(configFiles,workspaceRoot),nativeCache:new Map(),nativeClosures:new Map()},texts=new Map();
    for(const file of found.files.filter(isAuthoredProjectConfigV1)){const abs=join(workspaceRoot,file);texts.set(file,existsSync(abs)?readFileSync(abs):null);}
    const catalogFiles=options?.catalogFiles?.length?[...found.files,...inferenceCandidates(options.catalogFiles,workspaceRoot)]:found.files;return{catalog,files:found.files,catalogFiles,nativeCache:found.nativeCache,nativeClosures:found.nativeClosures,playground:collectPlaygroundCatalog(catalogFiles,workspaceRoot,catalog),texts};
  };
  let setupSteps=0;
  const setup=sharedSetup?await sharedSetupV1(sharedSetup,JSON.stringify([workspaceRoot,options?.catalogProviders??[],createHash("sha256").update(configFiles.join("\0")).digest("hex")]),setupPass,(force=false)=>yieldInferenceV1(context,"build:cargo-owners",++setupSteps,force)):await setupPass();
  const catalogContributions=setup.catalog,catalogFiles=setup.catalogFiles,nativeCache=new Map(setup.nativeCache),nativeClosures=new Map(setup.nativeClosures);configFiles=setup.files;
  const late = !options?.ownerAdmission && options?.snapshot === true && typeof options?.onOwnerObservations === "function", rootsByName = new Map(), scripts = createScriptInputCache({ ...context, lateClosures: late });
  const contracts = projectGeneratorContractsV1(workspaceRoot);
  scripts.generatedOutput=(path,origin)=>readBuildProducedCompileOutputV1(path,workspaceRoot,generatorContractsInScopeV1(contracts),context.registry,origin);
  useGeneratorContractsV1(workspaceRoot,contracts);useAnalysisStoreV1(workspaceRoot,analysisStoreDirectoryOfV1(closureStoreDirectoryV1(workspaceRoot)));
  if(context.invocation)await advanceScriptInvocation(context.invocation,"build:commands","running");
  const commandInputs = !options?.ownerAdmission&&configFiles.some((path) => !isOpaqueSourcePathV1(path) && path.endsWith("Cargo.toml") && !path.includes(".🧬semio") && !POLICY.generatedDirectories.some((name) => path.split("/").includes(name))) ? await closureApiV1(scripts, workspaceRoot).nativeCommandInputs() : undefined;
  if(context.invocation)await advanceScriptInvocation(context.invocation,"build:commands","complete");
  const commandRevision=createHash("sha256").update(JSON.stringify(closureHandleOfV1(commandInputs)?.key??commandInputs??null)).digest("hex"),registryDigest=createHash("sha256").update(JSON.stringify(context.registry?.catalogProviders?.map(({ownerRoot,sourcePath})=>[ownerRoot,sourcePath])??null)).digest("hex");
  const playgroundCatalog = setup.playground;

  const results = (await Promise.all(configFiles
    .filter(isAuthoredProjectConfigV1)
    .map(async (configFile) => {
      const abs = join(workspaceRoot, configFile), bytes = setup.texts.get(configFile);
      if (!bytes) return null;
      let json;
      try {
        json = JSON.parse(bytes.toString("utf8"));
      } catch (error) {
        const root = buildPath(relative(workspaceRoot, dirname(abs))) || ".";
        unavailable.set(root, unavailableProjectV1(root, undefined, workspaceRoot, Object.assign(Error(`Invalid Build project metadata ${configFile}: ${error.message}`), { code: "BUILD_SOURCE_UNPARSEABLE", source: abs, sha256: createHash("sha256").update(bytes).digest("hex") })));
        return null;
      }
      const name = json.name;
      if (!name) throw new Error(`Build project metadata ${configFile} requires a name`);
      const projectDir = dirname(abs);
      const root = buildPath(relative(workspaceRoot, projectDir)) || ".";
      const prior = rootsByName.get(name);
      if (prior !== undefined && prior !== root) throw new Error(`Duplicate Build project ${name}: ${prior} and ${root}`);
      if (prior === undefined) rootsByName.set(name, root);
      if (options?.ownerAdmission && !wanted(root)) return [configFile, { projects: { [name]: {...json,root,targets:{}} } }];
      if (name === "@semio-tech/plugin-registry") json.targets = { ...json.targets, ...playgroundSessionTargets(catalogFiles, workspaceRoot,catalogContributions) };
      if (name === "@semio-tech/framework-os-dev") json.targets = { ...json.targets, ...await playgroundPreparationTargets(catalogFiles, workspaceRoot, root, contracts,catalogContributions,context) };
      if (playgroundCatalog.some(row => row.cratePath === root)) {
        json.targets = { ...playgroundSiteTargetsForAdmissionV1(root,playgroundCatalog,workspaceRoot,context.registry,options?.ownerAdmission?options.ownerAdmission.targets[name]??[]:undefined), ...json.targets };
      }
      const canonicalConfig = buildPath(relative(workspaceRoot, join(projectDir, PROJECT_BASENAME)));
      return [canonicalConfig, { projects: { [name]: {...json,root} } }];
    })))
    .filter(Boolean);
  const declaredRoots = new Set([...rootsByName.values()]);let cargoIndex=0;
  for (const configFile of configFiles.filter((file) => file.endsWith("Cargo.toml"))) {
    await yieldInferenceV1(context,"build:cargo-targets",++cargoIndex);
    if (!ROSTER_RULES_V1.admitsManifest(configFile)) continue;
    if (!existsSync(join(workspaceRoot, configFile))) continue;
    const projectDir = dirname(join(workspaceRoot, configFile));
    const root = buildPath(relative(workspaceRoot, projectDir)) || ".";
    if (declaredRoots.has(root)) continue;
    let manifest;
    try { manifest = readToml(join(projectDir, "Cargo.toml")); }
    catch (error) { if (error?.code !== "BUILD_SOURCE_UNPARSEABLE") throw error; unavailable.set(root, unavailableProjectV1(root, undefined, workspaceRoot, error)); continue; }
    if (!manifest.package?.name) continue;
    const name = manifest.package.name;
    if (rootsByName.has(name)) throw new Error(`Duplicate Build project ${name}: ${rootsByName.get(name)} and ${root}`);
    rootsByName.set(name, root);
    if (!wanted(root)) { results.push([configFile, { projects: { [name]: { name, root, projectType: "library", targets: {} } } }]); continue; }
    const build = async () => {
    const targets = await cargoTargets(root, workspaceRoot, commandInputs,scripts);
    const project = { name, root, projectType: "library", tags: ["language:cargo", "discovery:manifest"], targets: { ...targets, ...componentBuildTargets(root, workspaceRoot), ...playgroundSiteTargetsForAdmissionV1(root,playgroundCatalog,workspaceRoot,context.registry,options?.ownerAdmission?options.ownerAdmission.targets[name]??[]:undefined) } };
    results.push([configFile, { projects: { [name]: project } }]);
    };
    if (collector) await collector.guard({ project: name, root, phase: "cargo-targets" }, build); else await build();
  }
  const projects = results.flatMap(([, result]) => Object.values(result.projects));
  const projectsByRoot = new Map(projects.flatMap(project=>[[project.root,project.name],...(project.metadata?.nativeRoot?[[project.metadata.nativeRoot,project.name]]:[])]));
  const componentRoots = componentCrateRoots(catalogFiles, workspaceRoot);
  const projectEnvironment = { workspaceRoot, contracts, commandInputs, scripts, projectsByRoot, componentRoots, nativeCache, nativeClosures, context, clock };

  const jobs=results.flatMap(([,result])=>Object.entries(result.projects).map(([name,json])=>({result,name,json}))).filter(job=>wanted(job.json.root));let projectIndex=0,cursor=0,stopped=false,stalled=false,takeScope,released;const deferred=new Set();const missed=new Set();const taken=new Set(),inflight=new Map(),ledger=createTakeLedgerV1(jobs.length),adopt=createAdoptCountersV1(),listedNames=createHotNamesV1({read:()=>fleetPublishedNamesV1(process.env.SEMIO_FLEET_STORE_ROOT,Date.now(),TAKE_POLICY_V1,{registryDigest,commandRevision,late}),counters:adopt}),resolved=new Promise(resolve=>{released=resolve;});
  const lookPhases={authority:0,observe:0,refs:0,rows:0},lap=(key,milliseconds)=>{lookPhases[key]+=milliseconds;};
  const takenMemo=late?undefined:context.takenMemo,memoContext=takenMemo===undefined?"":createHash("sha256").update(serialize([registryDigest,commandRevision,late,[...projectsByRoot],componentRoots,contracts,context.registry??null])).digest("hex"),memoKeyOf=job=>createHash("sha256").update(memoContext+"\0"+JSON.stringify(job.json)+"\0"+JSON.stringify(options?.ownerAdmission?ownerAdmissionOfV1(options.ownerAdmission,job):null)).digest("hex");
  const inferProject=async({result,name,json},away=work=>work(),withdraw=waveSignal)=>{
    const memoKey=takenMemo===undefined?undefined:memoKeyOf({name,json});
    if(options?.ownerAdmission?.generatedDependencies?.[name])json=applyBuildGeneratedTaskDependenciesV1(json,options.ownerAdmission.generatedDependencies[name]);
    let looked;if(lookQueue!==undefined){stageV1(name,STAGE_LOOK_V1);try{looked=await lookQueue.ask(name,withdraw);withdraw.throwIfAborted();}finally{stageV1(name,STAGE_INFER_V1);}}if(looked?.verdict==="stale")adopt.staleRow(looked.staleRow?.kind);if(looked?.verdict==="stale"&&looked.staleRow!==undefined&&staleRowTold++<400){const row=looked.staleRow;takeSay(language=>ledger.staleRowLine(language,name,row));}
    const started=performance.now();
    await yieldInferenceV1(context,"build:projects",++projectIndex);
    const inactive=options?.ownerAdmission?Object.fromEntries(Object.entries(json.targets??{}).filter(([name])=>!options.ownerAdmission.targets[json.name]?.includes(name))):{};
    const{selectedTargets,selectedInputs,activeJson}=ownerAdmissionSelectionV1(options?.ownerAdmission,json);
    const root=json.root,nativeRoot=json.metadata?.nativeRoot??root,nativeTargets=Object.entries(activeJson.targets??{}).filter(([,target])=>target.metadata?.semio?.execution?.kind!=="source"),nativeCandidate=looked!==undefined?looked.native:existsSync(join(workspaceRoot,nativeRoot,"Cargo.toml"))||nativeTargets.some(([,target])=>target.options?.command?.includes("⚡️caching/🦀️cargo/📜️script.ts")),native=options?.ownerAdmission?ownerAdmissionCompilesV1(activeJson,selectedInputs,nativeCandidate,nativeTargets):nativeCandidate,pipeline=looked!==undefined?looked.pipeline:Object.values(activeJson.targets??{}).some(target=>target.metadata?.semio?.release);
    const assemblyAuthority=looked!==undefined?undefined:native||pipeline?projectAssemblyAuthorityV1(workspaceRoot,native?nativeRoot:undefined,projectsByRoot,pipeline?componentRoots:undefined,nativeCache,nativeClosures):null;
    const authorityMilliseconds=performance.now()-started;lap("authority",authorityMilliseconds);clock.add(name,"authority",authorityMilliseconds);
    const cacheKey=ownerCacheKeyV1(root),cachePath=ownerCachePathV1(workspaceRoot,root),head=looked!==undefined?looked.head:ownerRecordHeadV1({registryDigest,commandRevision,late},{native,pipeline,assemblyAuthority});
    let offered,seenHeader;const observeJob=farm===undefined?undefined:{name,root,json,native,pipeline,nativeRoot,selectedInputs,head,cacheKey,cachePath,observe:true,refs:late&&options.rowLog!==undefined&&closureVerdictScopeV1()!==undefined,closureStore:closureStoreDirectoryV1(workspaceRoot),verdicts:closureVerdictScopeV1()};const inferStartedAt=Date.now();stageV1(name,STAGE_INFER_V1);result.projects[name]=await tagProjectFailureV1(root,()=>observeOwnerAttemptsV1(ownerRound=>observeProjectInferenceV1({cachePath,fleet:root,key:projectRecordKeyV1(head,root,json,selectedTargets,selectedInputs),owner:root,stamps:projectStampPathsV1({native}),reusable:value=>projectRecordReusableV1(value)&&inferenceInventoryReferencesHoldV1(workspaceRoot,value,context.phaseControl,withdraw),prepare:farm?()=>{missed.add(name);return stagedPrepareV1(name,away,()=>farm.prepare({name,root,json,native,pipeline,nativeRoot,selectedInputs,head,cacheKey,verdicts:closureVerdictScopeV1(),costMegabytes:Math.round(enforcement.known?.(name)??0)}));}:away.take===true?()=>away():undefined,deferOffer:away.take===true&&planStageLimitBytesV1()!==Infinity,verdict:observeJob===undefined?undefined:({headerDigest})=>{seenHeader=headerDigest;if(looked?.verdict!==undefined&&looked.headerDigest===headerDigest){if(looked.verdict==="holds"&&looked.refs!==undefined&&looked.refs.failed===undefined&&looked.refs.ownerFailed===undefined)offered={refs:looked.refs,headerDigest};return Promise.resolve(looked);}stageV1(name,STAGE_VERDICT_V1);return farm.observe(observeJob).finally(()=>stageV1(name,STAGE_INFER_V1)).then(reply=>{if(reply?.verdict!==undefined&&reply.verdict!=="holds"&&staleRowTold++<400){const row=reply.staleRow??Object.freeze({kind:"verdict",path:String(reply.verdict),fact:"judged by pid "+reply.verifier});takeSay(language=>ledger.staleRowLine(language,name,row));}if(reply?.verdict==="holds"&&reply.headerDigest===headerDigest&&reply.refs!==undefined&&reply.refs.failed===undefined&&reply.refs.ownerFailed===undefined)offered={refs:reply.refs,headerDigest};return reply;});},verdictFinal:farm!==undefined,prepareAttempts:farm!==undefined?3:1},()=>{if(registryFailure!==undefined)throw registryFailure;missed.add(name);if(farm?.exists?.())throw farmJobRefusedV1(name,root,"the owner (the farm gave no record)");return inferProjectBodyV1(projectEnvironment,{name,root,json,native,pipeline,nativeRoot,selectedTargets,selectedInputs,ownerRound},options?.ownerAdmission);})));
    lap("observe",Date.now()-inferStartedAt);clock.add(name,"observe",Math.max(0,Date.now()-inferStartedAt-clock.get(name,"farm")-clock.get(name,"inline")));noteSlowV1("owner-inference "+name,inferStartedAt,10000);stageV1(name,STAGE_REFS_V1);let rows=projectInferenceObservationsV1(cachePath)??[],derived=[];
    if(late&&!result.projects[name].unavailable){
      try{const resolved=await tagProjectFailureV1(root,()=>observeRefsWithRetriesV1(async()=>{const startedAt=Date.now();try{let adopted,last="no answer";const viaFarm=()=>farm!==undefined&&farm.exists();for(let round=0;round<5&&adopted===undefined&&viaFarm();round++){let from=round===0?offered:undefined;if(from===undefined){if(observeJob?.refs!==true){last="the read has no row log or verdict directory for the references";break;}const headerDigest=round===0&&seenHeader!==undefined?seenHeader:recordHeaderDigestV1(cachePath);if(headerDigest===undefined){last="no stored record (a source changed a moment ago)";await new Promise(done=>setTimeout(done,2500));continue;}stageV1(name,STAGE_REFS_FARM_V1);try{from=await askOwnerRefsV1({farm,job:observeJob,headerDigest});}finally{stageV1(name,STAGE_REFS_V1);}if(from===undefined)continue;}if(from.refs.unparseable!==undefined){const detail=from.refs.unparseable;throw Object.assign(Error(detail.message),{code:"BUILD_SOURCE_UNPARSEABLE",source:detail.source,sha256:detail.sha256});}adopted=adoptOwnerRefsV1(from.refs,from.headerDigest,workspaceRoot,scripts,options.rowLog);if(adopted===undefined){last="the reference file did not hold";seenHeader=undefined;}}if(adopted!==undefined){ownerRefsCountsV1.adopted++;return adopted;}if(viaFarm())throw farmJobRefusedV1(name,root,"the closure references ("+last+")");ownerRefsCountsV1.resolved++;return await resolveClosureRefsV1(result.projects[name],workspaceRoot,scripts,"hash",options.rowLog);}finally{lap("refs",Date.now()-startedAt);clock.add(name,"refs",Date.now()-startedAt);noteSlowV1("resolve-closure-refs "+name,startedAt,5000);}},{owner:name,scope:{workspaceRoot,fallback:join(workspaceRoot,root,PROJECT_BASENAME)},aborted:()=>context?.invocation?.control?.signal?.aborted===true||context?.phaseControl?.signal?.aborted===true,rows:()=>projectInferenceObservationsV1(cachePath)??[]}));result.projects[name]=resolved.project;const extra=[...rows,...resolved.derived.map(fact=>fact.row)];rows=options.rowLog?resolved.rows.concat(extra):[...extra.slice(0,rows.length),...resolved.rows,...extra.slice(rows.length)];derived=resolved.derived;}
      catch(raised){const error=ownerRefsFailureV1(raised,{workspaceRoot,root,name,aborted:context?.invocation?.control?.signal?.aborted===true||context?.phaseControl?.signal?.aborted===true});if(error===undefined)throw raised;const unavailable=unavailableProjectV1(root,name,workspaceRoot,error);rows=[...rows,...withheldRowsV1(error)];result.projects[name]={unavailable};}
    }
    stageV1(name,STAGE_ROWS_V1);{const at=performance.now();options?.onOwnerObservations?.(name,rows,derived);lap("rows",performance.now()-at);clock.add(name,"observations",performance.now()-at);}if(governor===undefined||governor.pressure()===true)releaseProjectInferenceRetentionV1(cachePath);
    if(result.projects[name].unavailable){unavailable.set(root,result.projects[name].unavailable);delete result.projects[name];return;}
    result.projects[name].targets={...inactive,...result.projects[name].targets};noteProjectSizeV1(name,result.projects[name]);
    if(memoKey!==undefined){const stored=projectInferenceObservationsV1(cachePath);if(stored!==undefined)takenMemo.remember(name,{key:memoKey,project:result.projects[name],rows:stored});}
  };
  const phases=context.invocation?createGraphPhaseClockV1({publish:async resource=>{context.invocation.control.signal.throwIfAborted();await context.invocation.control.publish({owner:context.invocation.policy.owner,command:"build:projects",stage:"running",activity:{resource,mode:"shared",elapsedMs:0}});},write:writeGraphPhaseRowV1}):undefined;let finished=0;
  await phases?.begin("inference",jobs.length);
  let fleetPort;{try{const records=await import("../../../🪪️graph-state/📦️project-records/🟦️.ts");fleetPort=records.createProjectFleetPortV1({root:workspaceRoot,environment:process.env,busy:()=>jobs.slice(cursor).some(job=>!taken.has(job.name)&&!never.has(job.name)&&!deferred.has(job.name)),strong:takeStrongMachineV1({memoryMegabytes:Math.floor(totalmem()/1048576),cores:availableParallelism()})});if(fleetPort!==undefined)takeScope={run:records.runTakeLookV1};}catch{}useProjectFleetV1(fleetPort);}
  const warmStop=new AbortController(),passStop=new AbortController(),polling=new AbortController();
  let registryFailure;
  const stopRegistryWave=error=>{
    if(registryFailure!==undefined)return;
    registryFailure=error;
    stopped=true;
    warmStop.abort();
    passStop.abort();
    polling.abort();
    farmStop.abort(error);
  };
  const farm=timedFarmV1(options?.ownerAdmission?undefined:openProjectFarmV1({workspaceRoot,context,contracts,closureStore:closureStoreDirectoryV1(workspaceRoot),projects:jobs.length,lateClosures:late,commandInputs:closureHandleOfV1(commandInputs)?{closureRecipe:closureHandleOfV1(commandInputs).recipe}:commandInputs,projectsByRoot,componentRoots,signal:waveSignal,onOwnerRegistryFailure:stopRegistryWave,processed:()=>projectIndex,maximumWorkers:planFarmWorkersV1({kind:"project-farm",workerMegabytes:GRAPH_FARM_POLICY_V1.workerMegabytes,availableMegabytes:await hostAvailableMegabytesV1()}),engage:options?.coldInference===true||wantedRoots!==undefined,immediate:true,releaseCaches:()=>{const held=scripts.sharedBytes??0;releaseAfterTakeLookV1(scripts,{shrink:true});return held;}}),clock);
  useEnvironmentGateV1();
  const memoryPorts=createMemoryPortsV1(),parentBytes=graphParentManagedBytesV1(process.env),parentBaseline=memoryPorts.sample(),parentCollector=createParentCollectorV1({ports:memoryPorts,budgetBytes:parentBytes,baselineBytes:parentBaseline});
  const rowCounts=new Map(),tooHeavy=new Map(),enforcement=await graphOwnerEnforcementV1(workspaceRoot,process.env,parentBytes,key=>rowCounts.get(key)??0);
  const alone=new Set(enforcement.heavy?.(jobs.map(job=>job.name))??[]);const rank=options?.ownerRank;const census=ownerRecordsPresentV1(join(workspaceDataDirectory(workspaceRoot),"np"),jobs.map(job=>job.json.root)),present=new Set(jobs.filter(job=>census.present.has(job.json.root)).map(job=>job.name));try{for(const language of selectGraphMemoryLanguagesV1(process.env))process.stderr.write("[owners] "+ownerRecordsPresentTextV1(language,census)+"\n");}catch{}const slowNames=new Set(enforcement.slow?.(jobs.map(job=>job.name))??[]);orderOwnerJobsV1(jobs,{alone,slow:slowNames,rank,present});const never=new Set(enforcement.neverHere?.(jobs.filter(job=>alone.has(job.name)).map(job=>job.name),farm===undefined?undefined:farmAloneCapV1(hostMemoryMegabytesV1()))??[]);const lightEnd=jobs.length-jobs.filter(job=>alone.has(job.name)).length;
  let ownerPool;{try{ownerPool=(await import("./🌐️owner-pool/📣️publisher/🟦️.ts")).createOwnerPoolPublisherV1({workspaceRoot,environment:process.env,identity:inferenceRecordIdentityV1(),globals:{registryDigest,commandRevision,late},session:()=>({version:3,workspaceRoot,closureStore:closureStoreDirectoryV1(workspaceRoot),registry:context.registry??null,contracts,commandInputs:closureHandleOfV1(commandInputs)?{closureRecipe:closureHandleOfV1(commandInputs).recipe}:commandInputs??null,lateClosures:late,projectsByRoot:[...projectsByRoot],componentRoots}),units:()=>{const slow=new Set(enforcement.slow?.(jobs.map(job=>job.name))??[]);return jobs.filter(job=>!taken.has(job.name)).map(job=>({name:job.name,kind:ownerKindV1(job.json,workspaceRoot),expectedMilliseconds:slow.has(job.name)?OWNER_SLOW_SECONDS_V1*2000:60000,...enforcement.known?.(job.name)===undefined?{}:{megabytes:enforcement.known(job.name)},...options?.ownerAdmission?{admission:ownerAdmissionOfV1(options.ownerAdmission,job)}:{},json:job.json}));},say:line=>process.stderr.write("[fleet] "+line+"\n"),store:process.env.SEMIO_FLEET_STORE_ROOT,origin:(()=>{try{return JSON.parse(rawReadFileSync(machinePathV1(workspaceRoot,".cache","f","machine.json"),"utf8")).originId;}catch{return undefined;}})()});}catch{}}
  const ownerSizes=jobs.map(job=>enforcement.known?.(job.name)).filter(value=>Number.isFinite(value)).sort((a,b)=>a-b),limit=Math.min(inferenceLanesV1({cores:availableParallelism(),cut:graphCutConcurrencyV1(process.env),parentMegabytes:parentBytes/1048576,ownerMegabytes:ownerSizes.length===0?undefined:ownerSizes[Math.floor(ownerSizes.length*0.75)],explicit:options?.projectConcurrency}),jobs.length),governor=limit>0?createGraphParentMemoryGovernorV1({workers:limit,ports:memoryPorts,...farm===undefined?{budgetBytes:parentBytes}:{},baselineBytes:parentBaseline,peaks:createGraphOwnerPeaksFileV1(join(workspaceDataDirectory(workspaceRoot),"np","owner-peaks.json")),...enforcement.limitBytes===undefined?{}:{ownerLimitBytes:enforcement.limitBytes},...enforcement.recorder===undefined?{}:{costs:enforcement.recorder},...enforcement.recordedBytes===undefined?{}:{estimate:enforcement.recordedBytes},ask:graphHostAskV1(process.env),measured:name=>ownerMeasuredManagedBytesV1(farm?.peakOf?.(name),enforcement.cost?.ownerCostPolicyV1.footprintFactor),signal:waveSignal,decide:event=>{decision=event;},report:line=>process.stderr.write(line+"\n"),environment:process.env}):undefined;
  const collectResident=()=>{parentCollector.afterProject();};
  let decision;const parkedTold=new Map(),active=new Map(),publishRunning=()=>{ownerPool?.miss(missed.size);graphWaveMeterV1.observe(memoryPorts.sample(),active.keys());const at=performance.now(),list=[...active].sort((a,b)=>a[1].since-b[1].since).map(([name,row])=>({name,seconds:(at-row.since)/1000,...(row.kind===undefined?{}:{kind:row.kind,rows:observedInferenceRowsV1(row.cachePath)})}));reportParkedOwnersV1(list,parkedTold,line=>process.stderr.write(line+"\n"),performance.now(),farm?.describe?.());options?.onOwnerRunning?.(list,decision);return phases?.running("inference",list,decision);};
    const asideOwner=(owner,reason)=>{const root=owner.json.root;unavailable.set(root,{root,project:owner.name,source:buildPath(join(root,PROJECT_BASENAME)),sha256:createHash("sha256").update(JSON.stringify(owner.json)).digest("hex"),reason});delete owner.result.projects[owner.name];const count=unavailable.size;takeSay(()=>"owner "+owner.name+" is set aside ("+count+" so far), the read goes on and ends naming it: "+reason+" / Projekt "+owner.name+" wird zurückgestellt ("+count+" bisher), der Lauf geht weiter und nennt es am Ende: "+reason);if(count===1||unavailable.size%50===0)takeSay(()=>{const census=asideCensusV1([...unavailable.values()].map(row=>row.reason));return census.en+" / "+census.de;});return true;};
    const failedOwner=error=>{if(error?.code!=="graph.farm-job-failed")return false;const owner=jobs.find(job=>job.json.root===error.semioProject||job.name===error.semioProject);if(owner===undefined)return false;return asideOwner(owner,String(error.message).slice(0,2048));};
    const changedOnce=new Set(),changedOwner=(error,job)=>{const owner=job??jobs.find(item=>item.json.root===error?.semioProject);if(owner===undefined)return false;const verdict=ownerStoreChangeV1(error,owner.name,changedOnce);if(verdict===undefined)return false;const reason=String(error.message).slice(0,2048);if(verdict==="aside")return asideOwner(owner,reason);deferredNow.add(owner.name);jobs.push(owner);takeSay(()=>"owner "+owner.name+" saw the store change under it ("+reason+"), it is requeued once / Projekt "+owner.name+" sah den Speicher sich ändern ("+reason+"), es wird einmal neu eingereiht");return true;};
    const setAside=(error,job)=>deferOwner(error)||failedOwner(error)||changedOwner(error,job)||enforcement.soloLimitBytes!==undefined&&enforcement.cost!==undefined&&setAsideOverrunV1({error,never,alone,recorder:enforcement.recorder,factor:enforcement.cost.ownerCostPolicyV1.footprintFactor,say:takeSay});
  const extras=[],deferredNow=new Set();let helped=false;
  const deferOwner=error=>{if(error?.code!=="BUILD_OWNER_DEFERRED")return false;const owner=jobs.find(job=>job.json.root===error.semioProject);if(owner===undefined)return false;deferred.add(owner.name);deferredNow.add(owner.name);jobs.push(owner);return true;};
  const ownerClaims=createOwnerClaimsV1({directory:machinePathV1(workspaceRoot,".cache","f","owner-claims")});
  const loop=async end=>{await resolved;waveSignal.throwIfAborted();if(end===Infinity&&!helped&&alone.size===0&&deferred.size>0){helped=true;for(let extra=1;extra<limit;extra++)extras.push(startLoop(Infinity));}while(!stopped&&cursor<Math.min(end,jobs.length)){waveSignal.throwIfAborted();{const job=jobs[cursor++],solo=alone.has(job.name);let again=false;if(taken.has(job.name)||never.has(job.name))continue;if(inflight.has(job.name)){await inflight.get(job.name).catch(()=>undefined);if(taken.has(job.name))continue;}if(options?.skipOwner?.(job.name)===true){finished++;continue;}{const adopted=await takeBeforeInferV1({name:job.name,listed:name=>listedNames().has(name),taken,look:()=>look(job,"infer"),counters:adopt});if(adopted!==undefined){ledger.mark(job.name,adopted);counted(job.name);continue;}}if(stopped)return;let held;{const claimed=await takeClaimedV1({name:job.name,stem:ownerCacheKeyV1(job.json.root),claims:ownerClaims,look:()=>look(job,"infer"),signal:waveSignal});if(claimed.outcome!==undefined){ledger.mark(job.name,claimed.outcome);counted(job.name);continue;}held=claimed;}clock.begin(job.name);const requested=performance.now();ledger.run(1);try{await governor.run(async({away,scope})=>{clock.add(job.name,"queue",performance.now()-requested);decision=undefined;active.set(job.name,{since:performance.now(),kind:ownerKindV1(job.json,workspaceRoot),cachePath:ownerCachePathV1(workspaceRoot,job.json.root)});{const at=performance.now();await publishRunning();clock.add(job.name,"publish",performance.now()-at);}try{try{await runInOwnerScopeV1(scope,()=>collector?collector.guard({project:job.name,root:job.json.root,phase:"project"},()=>inferProject(job,away)):inferProject(job,away)).catch(error=>{if(!setAside(error,job))throw error;});}catch(error){if(ownerTooHeavyOfV1(error)===undefined)throw error;}if(scope.exceeded!==undefined){if(!solo&&enforcement.soloLimitBytes!==undefined&&enforcement.soloLimitBytes()>scope.exceeded.limitBytes){alone.add(job.name);jobs.push(job);again=true;}else tooHeavy.set(job.name,{...scope.exceeded,name:job.name});}}finally{rowCounts.set(job.name,observedInferenceRowsV1(ownerCachePathV1(workspaceRoot,job.json.root)));forgetObservedInferenceRowsV1(ownerCachePathV1(workspaceRoot,job.json.root));active.delete(job.name);decision=undefined;{const at=performance.now();releaseOwnerStateV1(scripts,{shrink:governor.pressure()});clock.add(job.name,"release",performance.now()-at);}}},{kind:ownerKindV1(job.json,workspaceRoot),key:job.name,...solo?{limitBytes:enforcement.soloLimitBytes()}:{}});}finally{ledger.run(-1);held.release();}clock.finish(job.name,{running:active.size});if(!again&&!deferredNow.delete(job.name)){finished++;countedNames.add(job.name);ledger.mark(job.name,"inferred");}}await publishRunning();reportProgress();collectResident();await new Promise(resolve=>setTimeout(resolve,0));}};
  const startLoop=end=>{const running=loop(end);void running.catch(()=>undefined);return running;};
  try{waveSignal.throwIfAborted();if(late){const since=performance.now(),storeDirectory=closureStoreDirectoryV1(workspaceRoot);let shown;try{if(storeDirectory!==undefined&&farm!==undefined){shown="shared closures in a child";active.set(shown,{since});await publishRunning();const primed=await primeSharedClosuresInChildV1({workspaceRoot,storeDirectory,signal:waveSignal});try{const seconds=Math.round((performance.now()-since)/1000);for(const language of selectGraphMemoryLanguagesV1(process.env))process.stderr.write("[owners] "+(language==="de"?"gemeinsame Closures vorbereitet in "+seconds+" s: "+(primed?.recipes??"?")+" Rezepte, "+(primed?.derived??"?")+" abgeleitet, "+(primed?.loaded??"?")+" geladen, "+(primed?.stale??"?")+" veraltet":"shared closures primed in "+seconds+" s: "+(primed?.recipes??"?")+" recipes, "+(primed?.derived??"?")+" derived, "+(primed?.loaded??"?")+" loaded, "+(primed?.stale??"?")+" stale")+"\n");}catch{}}else await primeSharedClosuresV1(workspaceRoot,scripts,{signal:waveSignal,progress:async({done,total})=>{active.delete(shown);shown="shared closures "+done+"/"+total;active.set(shown,{since});try{await options?.onOwnerProgress?.(done,total,"prime");}catch{}await publishRunning();}});}finally{active.delete(shown);}}waveSignal.throwIfAborted();}
  catch(error){governor?.dispose();await farm?.close();if(fleetPort)useProjectFleetV1(undefined);throw error;}
  const ticker=phases?setInterval(()=>{Promise.resolve(publishRunning()).catch(()=>{});},RUNNING_TICK_MS):undefined;ticker?.unref?.();
  const takeSay=text=>{try{for(const language of selectGraphMemoryLanguagesV1(process.env))process.stderr.write("[owners] "+text(language)+"\n");}catch{}};
  const byName=new Map(jobs.map(job=>[job.name,job])),lookProject=inferProject,takeAway=Object.assign(()=>{throw Object.assign(Error("not stored here or in the fleet cache"),{code:"BUILD_TAKE_MISS"});},{take:true});let poll=Promise.resolve();
  const lookQueue=farm===undefined||options?.ownerAdmission?undefined:createLookQueueV1({farm,jobOf:name=>{const job=byName.get(name);return{name,root:job.json.root,json:job.json,cacheKey:ownerCacheKeyV1(job.json.root),cachePath:ownerCachePathV1(workspaceRoot,job.json.root)};},shared:()=>({globals:{registryDigest,commandRevision,late},closureStore:closureStoreDirectoryV1(workspaceRoot),verdicts:closureVerdictScopeV1(),refs:late&&options?.rowLog!==undefined&&closureVerdictScopeV1()!==undefined}),refuse:(name,reason)=>farmJobRefusedV1(name,byName.get(name)?.json.root,"the look ("+reason+")"),say:text=>takeSay(text)});
  farm?.onReady(()=>lookQueue?.kick());
  const looks={count:0,milliseconds:0,maximum:0,buckets:[0,0,0,0]},lookActive=new Map(),lookSlowest=[];
  const look=(job,lane="pass",signal=waveSignal)=>{
    if(taken.has(job.name))return Promise.resolve("present");
    const running=inflight.get(job.name);if(running!==undefined)return joinTakeLookV1(running,signal);
    adopt.look(lane);const lookedAt=performance.now();lookActive.set(job.name,lookedAt);
    const work=(async()=>{
      try{signal.throwIfAborted();const outcome=await takeScope.run(async looked=>{try{await withTakeLookScopeV1(()=>runClosureTakeV1(()=>lookProject(job,takeAway,signal),signal));}catch(error){if(error?.code==="BUILD_TAKE_MISS"){missed.delete(job.name);error.outcome=looked.outcome;error.lookupFault=looked.fault;}throw error;}return looked.outcome==="fetched"?"fetched":"present";},signal);signal.throwIfAborted();taken.add(job.name);adopt.outcome(outcome);return outcome;}
      catch(error){const result=recordTakeLookFailureV1(error,adopt,signal.aborted||waveSignal.aborted);if(waveSignal.aborted)throw error;return result;}
      finally{const took=performance.now()-lookedAt;lookActive.delete(job.name);lookSlowest.push({name:String(job.name).slice(0,80),milliseconds:Math.round(took)});lookSlowest.sort((a,b)=>b.milliseconds-a.milliseconds);lookSlowest.length=Math.min(lookSlowest.length,5);looks.count++;looks.milliseconds+=took;looks.maximum=Math.max(looks.maximum,took);looks.buckets[took<=50?0:took<=500?1:took<=5000?2:3]++;inflight.delete(job.name);releaseAfterTakeLookV1(scripts,{shrink:governor?.pressure?.()===true});}
    })();
    inflight.set(job.name,work);return work;
  };
  let staleTold=0,staleRowTold=0;const countedNames=new Set(),counted=name=>{if(!countedNames.has(name)){countedNames.add(name);finished++;}};const flight={flying:0};
  const resolveOwners=async()=>{
    ownerPool?.announce(jobs.filter(job=>!present.has(job.name)&&options?.skipOwner?.(job.name)!==true).length);
    if(takeScope===undefined||jobs.length===0||process.env.SEMIO_GRAPH_TAKE==="off")return;
    const stopper=new AbortController(),bound=setTimeout(()=>stopper.abort(),TAKE_POLICY_V1.passBoundMilliseconds),signal=AbortSignal.any([waveSignal,stopper.signal,passStop.signal]),reporter=setInterval(()=>{takeSay(language=>ledger.line(language));reportProgress();ownerPool?.learn(jobs.length-taken.size);},TAKE_POLICY_V1.progressMilliseconds);
    bound.unref?.();reporter.unref?.();reportWhilePollingV1({tell:()=>takeSay(language=>adopt.line(language)),milliseconds:TAKE_POLICY_V1.adoptReportMilliseconds,polling:polling.signal});
    try{
      const names=jobs.filter(job=>options?.skipOwner?.(job.name)!==true).map(job=>job.name);
      const split=takeSplitV1(names,name=>alone.has(name)||slowNames.has(name)),countedGate=()=>{const gate=footprintGateV1();return gate===undefined?undefined:{admit:async abort=>{const at=performance.now();try{return await gate.admit(abort);}finally{const waited=performance.now()-at;if(waited>=50)adopt.gateWait(waited);}}};},attempt=takeAdmittedV1((name,active)=>look(byName.get(name),"pass",active),countedGate,signal,flight),adoptAttempt=takeAdmittedV1((name,active)=>look(byName.get(name),"adopt",active),()=>undefined,signal,{flying:0}),mark=(name,outcome)=>{ledger.mark(name,outcome);if(outcome==="stale"&&staleTold++<200)takeSay(language=>ledger.staleLine(language,name));if(outcome==="present"||outcome==="fetched")counted(name);};
      const published=listedNames(),lanes=takeLanesV1(split.light,name=>present.has(name),name=>published.has(name));lookQueue?.start([...lanes,split.heavy],name=>{const owner=byName.get(name);return owner!==undefined&&(existsSync(join(workspaceRoot,owner.json.metadata?.nativeRoot??owner.json.root,"Cargo.toml"))||Object.values(owner.json.targets??{}).some(target=>target.metadata?.semio?.release));},signal);
      await takeFetchFirstV1({lanes,heavy:split.heavy,concurrency:takeLookConcurrencyV1(graphCutConcurrencyV1(process.env)),attempt,adoptAttempt,signal,mark,hot:listedNames,queue:()=>jobs.filter(job=>{if(taken.has(job.name)||inflight.has(job.name)||options?.skipOwner?.(job.name)===true)return false;if(active.has(job.name)){if(listedNames().has(job.name))adopt.skipInferring(job.name);return false;}return true;}).map(job=>job.name),arrived:(name,outcome)=>{ledger.mark(name,outcome==="fetched"?"fetched":"present");counted(name);},stop:()=>stopped});
      reportProgress();
      {const seen=readParentObservedCountsV1();if(seen.computations+seen.records>0)takeSay(()=>"the parent ran "+seen.computations+" observed computations and "+seen.records+" observed records inside looks, a farm child should / der Elternprozess führte "+seen.computations+" beobachtete Berechnungen und "+seen.records+" beobachtete Einträge in Prüfungen aus, ein Farm-Kindprozess sollte");}
      const missing=jobs.filter(job=>!taken.has(job.name)&&options?.skipOwner?.(job.name)!==true);
      if(ownerPool!==undefined&&missing.length>0){ledger.want(missing.map(job=>job.name));ownerPool.settle(missing.length);takeSay(language=>ledger.waveLine(language));}
      takeSay(language=>ledger.line(language));
      if(missing.length>0){const stopTelling=reportWhilePollingV1({tell:()=>takeSay(language=>ledger.line(language)),milliseconds:TAKE_POLICY_V1.progressMilliseconds,polling:polling.signal});poll=takePollV1({hot:listedNames,concurrency:takeLookConcurrencyV1(graphCutConcurrencyV1(process.env)),queue:()=>jobs.slice(cursor).filter(job=>!taken.has(job.name)&&!inflight.has(job.name)).map(job=>job.name),attempt:takeAdmittedV1((name,active)=>look(byName.get(name),"adopt",active),()=>undefined,polling.signal,{flying:0}),arrived:name=>{ledger.mark(name,"fetched");counted(name);},stop:()=>stopped||polling.signal.aborted,signal:polling.signal}).catch(()=>undefined).finally(stopTelling);}
    }catch(error){if(waveSignal.aborted)throw error;takeSay(()=>"the take pass failed and every owner takes the normal path / der Durchgang ist gescheitert und jedes Projekt nimmt den normalen Weg: "+String(error?.message??error).replace(/\s+/gu," ").slice(0,160));}
    finally{stopper.abort();clearTimeout(bound);clearInterval(reporter);}
  };
  let warmState=null,reportedProgress=0;const reportProgress=()=>{const count=countedNames.size,advanced=count>reportedProgress;reportedProgress=Math.max(reportedProgress,count);if(advanced)options?.onOwnerProgress?.(reportedProgress,jobs.length);Promise.resolve(phases?.advance("inference",reportedProgress)).catch(()=>{});};
  const warmOwners=async()=>{
    if(options?.ownerAdmission||takeScope===undefined||farm===undefined||process.env.SEMIO_GRAPH_TAKE==="off")return;
    const globals={registryDigest,commandRevision,late},globalsText=JSON.stringify(globals),directory=warmDirectoryV1(workspaceRoot),closureStore=closureStoreDirectoryV1(workspaceRoot);
    if(closureStore===undefined)return;
    const describe=name=>{const job=byName.get(name),key=warmDigestV1(globalsText+"\0"+JSON.stringify(job.json));return{key,stamp:warmStampV1(ownerCachePathV1(workspaceRoot,job.json.root)),cost:enforcement.known?.(name)??1,job:{name,json:job.json,globals,warmKey:key,warm:true,knownMegabytes:Math.round(enforcement.known?.(name)??0),directory,closureStore,cachePath:ownerCachePathV1(workspaceRoot,job.json.root)}};};
    try{
      const names=jobs.filter(job=>present.has(job.name)&&options?.skipOwner?.(job.name)!==true).map(job=>job.name);
      const warmStartedMs=performance.now();const counters=await warmWaveV1({farm,names,describe,directory,cores:availableParallelism(),limitMegabytes:planStageLimitBytesV1()/1048576,signal:AbortSignal.any([waveSignal,warmStop.signal]),tell:fields=>{warmState=fields;takeSay(language=>warmLineV1(language,fields));try{phases?.running("inference",[{name:"warm",kind:"warm",seconds:(performance.now()-warmStartedMs)/1000,rows:fields.settled+fields.durable}]);}catch{}}});try{phases?.running("inference",[]);}catch{}
      if(counters!==undefined)warmState=counters;
    }catch(error){if(waveSignal.aborted)throw error;takeSay(()=>"the warm pass failed and the parent takes every owner itself / der Abgleich ist gescheitert und der Elternprozess übernimmt jedes Projekt selbst: "+String(error?.message??error).replace(/\s+/gu," ").slice(0,160));}
  };
  const loopStartedMs=performance.now();
  const named=left=>enforcement.cost.describeHeavyV1(left.map(name=>({name,megabytes:Math.round((enforcement.stopped?.(name)??enforcement.known?.(name)??0)/enforcement.cost.ownerCostPolicyV1.footprintFactor),source:"recorded"})),selectGraphMemoryLanguagesV1(process.env)[0]);
  const refuseHeavy=(left,extra)=>enforcement.cost.refuseHeavyOwnerV1({named:named(left)+(extra?" ("+extra+")":""),limit:Math.round(enforcement.soloLimitBytes()/1048576*enforcement.cost.ownerCostPolicyV1.footprintFactor),producers:Object.keys(fleetPort?.producers?.()??{}).length});
  let retriedAlone=false;
  const waitHeavy=async()=>{
    const pending=jobs.filter(job=>never.has(job.name)&&!taken.has(job.name)&&options?.skipOwner?.(job.name)!==true);
    if(pending.length===0||stopped)return;
    if(takeScope===undefined||ownerPool===undefined)throw refuseHeavy(pending.map(job=>job.name));
    ownerPool.miss(Number.MAX_SAFE_INTEGER);
    const fleetStore=process.env.SEMIO_FLEET_STORE_ROOT,askUnits=pending.map(job=>({name:job.name,kind:ownerKindV1(job.json,workspaceRoot),expectedMilliseconds:OWNER_SLOW_SECONDS_V1*2000,megabytes:Math.max(1,enforcement.known?.(job.name)??Math.round(enforcement.soloLimitBytes()/1048576*enforcement.cost.ownerCostPolicyV1.footprintFactor)+1),...options?.ownerAdmission?{admission:ownerAdmissionOfV1(options.ownerAdmission,job)}:{},json:job.json})),need=Math.max(0,...askUnits.map(unit=>unit.megabytes)),seeFleet=()=>ownerPoolReadFleetV1(fleetStore),seeProducers=()=>ownerPoolProducersV1(seeFleet(),need,Date.now());
    if(seeFleet().length>0&&seeProducers().length===0){const why=ownerPoolNoProducerTextV1(seeFleet(),need,Date.now());takeSay(()=>why);throw refuseHeavy(pending.map(job=>job.name),why);}
    await ownerPool.ask(askUnits).catch(()=>false);
    const wantedModule=await import("../../../🪪️graph-state/🙋️wanted/🟦️.ts"),names=pending.map(job=>job.name);let ended;
    const sign=()=>Object.keys(fleetPort?.producers?.()??{}).length+names.filter(name=>taken.has(name)).length;
    const left=await sliceReadStepV1("heavy-wait",()=>wantedModule.awaitWantedV1({projects:names,signal:waveSignal,
      take:async name=>{const outcome=await look(byName.get(name));if(outcome==="present"||outcome==="fetched"){ledger.mark(name,outcome==="fetched"?"fetched":"present");finished++;countedNames.add(name);return true;}return false;},
      alive:()=>sign()+Math.max(0,...seeProducers().map(row=>row.updatedMs)),progressSign:sign,renew:async()=>{ownerPool.miss(Number.MAX_SAFE_INTEGER);await ownerPool.ask(askUnits,true).catch(()=>false);},end:cause=>{ended=cause;},
      progress:({arrived,wanted})=>takeSay(language=>HEAVY_WAIT_V1[language==="de"?"de":"en"].replace("{arrived}",arrived).replace("{wanted}",wanted).replace("{names}",names.filter(name=>!taken.has(name)).slice(0,3).join(", ")).replace("{megabytes}",Math.max(0,...names.map(name=>enforcement.known?.(name)??0))).replace("{produced}",Object.keys(fleetPort?.producers?.()??{}).length))}),{names:names.slice(0,3).join(", ")});
    const again=heavyRetryNamesV1(left,retriedAlone);
    if(again.length>0){retriedAlone=true;takeSay(language=>HEAVY_RETRY_V1[language==="de"?"de":"en"].replace("{count}",again.length).replace("{names}",again.slice(0,3).join(", ")));for(const name of again){never.delete(name);const job=byName.get(name);if(job!==undefined)jobs.push(job);}await loop(Infinity);const rest=again.filter(name=>never.has(name)&&!["present","fetched","inferred"].includes(ledger.state(name)));if(rest.length>0){takeSay(language=>HEAVY_ENDED_V1[language==="de"?"de":"en"].replace("{count}",rest.length).replace("{cause}",String(ended)));throw refuseHeavy(rest);}return;}
    if(left.length>0){takeSay(language=>HEAVY_ENDED_V1[language==="de"?"de":"en"].replace("{count}",left.length).replace("{cause}",String(ended)));throw refuseHeavy(left);}
  };
  const largestOf=holder=>Object.entries(holder??{}).flatMap(([key,value])=>value instanceof Map||value instanceof Set?[[key,value.size]]:value!==null&&typeof value==="object"&&!Array.isArray(value)?Object.entries(value).filter(([,inner])=>inner instanceof Map||inner instanceof Set).map(([inner,collection])=>[key+"."+inner,collection.size]):[]).sort((a,b)=>b[1]-a[1]).slice(0,10);
  let jobsBytes;const collectionsOff=registerSliceCollectionsV1("owners",()=>{const all=results.flatMap(([,row])=>Object.values(row.projects));jobsBytes??=jobs.reduce((sum,job)=>sum+JSON.stringify(job.json).length,0);return[{name:"owners:results.projects",entries:all.length,sample:()=>all.slice(0,16)},{name:"owners:jobs (real JSON)",entries:jobs.length,bytes:jobsBytes},{name:"owners:resolvedProjects (real JSON of "+projectSizesV1.count+" finished: "+projectSizesV1.top.map(row=>row.name.split("/").pop()+" "+row.bytes).join(", ")+")",entries:projectSizesV1.count,bytes:projectSizesV1.bytes},describeCollectionV1("owners:jobs",jobs),describeCollectionV1("owners:taken",taken),describeCollectionV1("owners:inflight",inflight),describeCollectionV1("owners:missed",missed),describeCollectionV1("owners:rowCounts",rowCounts),describeCollectionV1("owners:never",never),describeCollectionV1("owners:alone",alone),describeCollectionV1("owners:stages",stagesV1)];});
  ownerPassesV1++;const samplerOff=startSliceSamplerV1(),reportOff=registerSliceReportSourceV1("owners",()=>({pass:ownerPassesV1,cpuSeconds:(usage=>({user:Math.round(usage.user/1e5)/10,system:Math.round(usage.system/1e5)/10}))(process.cpuUsage()),jobs:jobs.length,ledger:ledger.counts(),taken:taken.size,reuse:takenMemo?.stats()??null,warm:warmState,inflight:inflight.size,missed:missed.size,finished,never:[...never].slice(0,8),neverCount:never.size,alone:alone.size,cursor,stopped,limit,active:[...active].slice(0,12).map(([name,row])=>({name:String(name).slice(0,80),seconds:Math.round((performance.now()-row.since)/1000),kind:row.kind})),activeCount:active.size,looks:{count:looks.count,averageMilliseconds:looks.count?Math.round(looks.milliseconds/looks.count):0,maximumMilliseconds:Math.round(looks.maximum),upTo50:looks.buckets[0],upTo500:looks.buckets[1],upTo5000:looks.buckets[2],over5000:looks.buckets[3],active:[...lookActive].slice(0,6).map(([name,at])=>({name:String(name).slice(0,80),seconds:Math.round((performance.now()-at)/1000)})),slowest:lookSlowest,outcomes:adopt.lookups(),phases:Object.fromEntries([...Object.entries(lookPhases).map(([key,value])=>[key,Math.round(value)]),...Object.entries(observationLookupClockV1()).map(([key,value])=>["lookup."+key,value])])},presence:inferenceInventoryPresenceStatsV1(workspaceRoot,context.phaseControl),scriptCollections:largestOf(scripts),farm:farm?.stats?.()??null,fleet:fleetPort?.stats?.()??null}));
  let failStalled;const stall=new Promise((_resolve,reject)=>{failStalled=error=>{reject(error);farmStop.abort(error);warmStop.abort(error);};});stall.catch(()=>undefined);
  const ownPoolStatus=async()=>{try{const status=await import("./🌐️owner-pool/📊️status/🟦️.ts");return status.readOwnerPoolStatusV1({store:process.env.SEMIO_FLEET_STORE_ROOT,origin:(()=>{try{return JSON.parse(rawReadFileSync(machinePathV1(workspaceRoot,".cache","f","machine.json"),"utf8")).originId;}catch{return undefined;}})()});}catch{return undefined;}};
  const watch=createProgressWatchV1(),tail=createTailReporterV1(),cpuProgress=createCpuProgressV1({sample:()=>takeChildrenCpuV1()}),missingRows=()=>{const at=performance.now();return takeMissingRowsV1(jobs.filter(job=>!["present","fetched","inferred"].includes(ledger.state(job.name))&&options?.skipOwner?.(job.name)!==true).map(job=>job.name),{never,running:new Map([...active].map(([name,row])=>[name,(at-row.since)/1000])),deferred,megabytes:name=>enforcement.known?.(name),limitMegabytes:enforcement.soloLimitBytes===undefined||enforcement.cost===undefined?undefined:Math.round(enforcement.soloLimitBytes()/1048576*enforcement.cost.ownerCostPolicyV1.footprintFactor)});},watcher=setInterval(async()=>{
    const cpu=await cpuProgress.tick(),rows=[...active.values()].reduce((sum,row)=>sum+(observedInferenceRowsV1(row.cachePath)??0),0),sign=takeProgressSignV1({finished,taken:taken.size,cursor,projectIndex,deferred:deferred.size,rows,done:ledger.counts().done,producers:Object.values(fleetPort?.producers?.()??{}).reduce((sum,each)=>sum+each,0),cpu:active.size>0?cpu:0}),hit=watch.check(sign),quiet=tail.check(sign);
    if(quiet!==undefined&&hit===undefined&&!stalled){const pool=await ownPoolStatus();takeSay(language=>takeTailTextV1(language,{minutes:Math.max(1,Math.round(quiet.sinceMilliseconds/60000)),done:ledger.counts().done,total:jobs.length,rows:missingRows(),pool}));}
    if(hit===undefined||stalled)return;
    stalled=true;stopped=true;passStop.abort();polling.abort();
    let peers=[];try{peers=(await import("../../../🪪️graph-state/🛰️exchange/🟦️.ts")).graphFleetWaitingV1();}catch{}
    const at=performance.now(),running=[...active].sort((a,b)=>a[1].since-b[1].since).slice(0,6).map(([name,row])=>name+" ("+Math.round((at-row.since)/1000)+" s, "+(observedInferenceRowsV1(row.cachePath)??0)+" rows)").join(", ")||"nothing",reasons=[],left=missingRows();
    if(peers.length>0)reasons.push(peers.length+" owners claimed by live peers ("+[...new Set(peers.map(row=>row.by.split("|")[0]))].slice(0,4).join(", ")+") / "+peers.length+" Projekte, die aktive Peers beanspruchen");
    if(decision!==undefined)reasons.push("the memory governor ("+decision.key+": "+decision.used+" of "+decision.budget+" MB) / die Speichersteuerung ("+decision.key+": "+decision.used+" von "+decision.budget+" MB)");
    if(reasons.length===0)reasons.push("the running owners themselves, which show no row / die laufenden Projekte selbst, die keine Zeile zeigen");
    const fields={minutes:Math.round(hit.sinceMilliseconds/60000),done:ledger.counts().done,total:jobs.length,running,waiting:reasons.join("; ")};
    {const pool=await ownPoolStatus();takeSay(language=>takeTailTextV1(language,{minutes:fields.minutes,done:fields.done,total:fields.total,rows:left,pool}));}
    if(takeStallKindV1(left)==="heavy"&&enforcement.cost!==undefined&&enforcement.soloLimitBytes!==undefined){failStalled(refuseHeavy(left.map(row=>row.name)));return;}
    try{for(const language of selectGraphMemoryLanguagesV1(process.env))process.stderr.write("[owners] "+takeNoProgressTextV1(language,fields)+"\n");}catch{}
    failStalled(refuseBuildV1("unserved","graph.no-progress",fields));
  },TAKE_POLICY_V1.watchMilliseconds);watcher.unref?.();
  const preloadTaken=async()=>{
    if(takenMemo===undefined)return;
    const reused=await takenMemo.replay(jobs.filter(job=>options?.skipOwner?.(job.name)!==true).map(job=>({name:job.name,key:memoKeyOf(job)})),{signal:waveSignal});
    for(const row of reused){byName.get(row.name).result.projects[row.name]=row.project;taken.add(row.name);ledger.mark(row.name,"present");counted(row.name);}
    if(reused.length>0){reportProgress();const memo=takenMemo.stats();takeSay(()=>reused.length+" of "+jobs.length+" owners answered from the previous pass of this read without a look ("+memo.evictedChanged+" changed rows, "+memo.evictedKey+" changed jobs, "+memo.evictedNamed+" named paths forgotten so far) / "+reused.length+" von "+jobs.length+" Projekten aus dem vorigen Durchgang dieses Lesens ohne Prüfung beantwortet ("+memo.evictedChanged+" geänderte Zeilen, "+memo.evictedKey+" geänderte Aufträge, "+memo.evictedNamed+" genannte Pfade bisher vergessen)");}
  };
  const unmarkParent=farm===undefined?undefined:readScope.arm(()=>farm.exists());
  const workers=Array.from({length:limit},()=>startLoop(lightEnd)),grow=AsyncResource.bind(()=>startLoop(lightEnd));
  farm?.onReady(loops=>{for(let extra=loops-workers.length;extra>0&&!stopped&&cursor<lightEnd;extra--)workers.push(grow());});
  const sweepOrphans=async()=>{
    if(stopped)return;
    const orphans=[...new Map(jobs.filter(job=>!never.has(job.name)&&!taken.has(job.name)&&!active.has(job.name)&&!inflight.has(job.name)&&!tooHeavy.has(job.name)&&!unavailable.has(job.json.root)&&!["present","fetched","inferred"].includes(ledger.state(job.name))&&options?.skipOwner?.(job.name)!==true).map(job=>[job.name,job])).values()];
    if(orphans.length===0)return;
    const names=orphans.slice(0,3).map(job=>job.name).join(", ");
    takeSay(()=>orphans.length+" owners are neither done nor running ("+names+"), they are read now / "+orphans.length+" Projekte sind weder fertig noch in Arbeit ("+names+"), sie werden jetzt gelesen");
    for(const job of orphans)jobs.push(job);
    await sliceReadStepV1("orphan-sweep",()=>loop(Infinity),{names});
  };
  const inferAll=async()=>{await awaitGrowingV1(workers);if(!stopped&&cursor<jobs.length){memoryPorts.collect();await loop(Infinity);}await Promise.allSettled(extras);if(registryFailure!==undefined)throw registryFailure;await sweepOrphans();await waitHeavy();};
  try{await preloadTaken();await runTakeBesideInferV1({take:resolveOwners,warm:warmOwners,stopWarm:()=>warmStop.abort(),release:released,end:()=>passStop.abort(),infer:()=>Promise.race([stall,inferAll()])});}catch(error){stopped=true;released();if(!stalled)await Promise.allSettled([...workers,...extras]);throw registryFailure??error;}finally{clearInterval(watcher);reportOff();samplerOff();collectionsOff();clearInterval(ticker);polling.abort();await poll;takeSay(language=>ledger.line(language));governor?.dispose();unmarkParent?.();await farm?.close();if(fleetPort){useProjectFleetV1(undefined);try{process.stderr.write("[fleet] "+await fleetPort.flush()+"\n");}catch{}try{ownerPool?.finish({owners:jobs.length,seconds:(performance.now()-loopStartedMs)/1000,consumedByHost:fleetPort.producers?.()??{}});}catch{}}}
  options?.onPhases?.({setup:Math.round(loopStartedMs-enteredMs),projects:Math.round(performance.now()-loopStartedMs)});options?.onInferred?.(missed.size);
  graphWaveMeterV1.parent(parentCollector.finish());
  enforcement.recorder?.flush();
  reportProgress();
  await phases?.end("inference",{projects:jobs.length,...(governor?{memory:governor.stats()}:{})});
  if(tooHeavy.size>0){const heavy=[...tooHeavy.values()].sort((a,b)=>b.takenBytes-a.takenBytes).map(row=>({name:row.name,megabytes:Math.round(row.takenBytes/1048576),source:"recorded"}));throw enforcement.cost.refuseHeavyOwnerV1({named:enforcement.cost.describeHeavyV1(heavy,selectGraphMemoryLanguagesV1(process.env)[0]),limit:Math.round(Math.max(...[...tooHeavy.values()].map(row=>row.limitBytes))/1048576*enforcement.cost.ownerCostPolicyV1.footprintFactor),producers:Object.keys(fleetPort?.producers?.()??{}).length});}
  if (collector) await collector.finish({ projects: jobs.length });
  if(wantedRoots)for(const entry of results)entry[1].projects=Object.fromEntries(Object.entries(entry[1].projects??{}).filter(([,project])=>wanted(project.root)));
  for(const row of [...unavailable.values()].sort((a,b)=>a.root<b.root?-1:a.root>b.root?1:0))results.push([row.source,{projects:{},unavailable:[row]}]);
  if (options?.analyzeLockfile && existsSync(join(workspaceRoot, javascriptManagerV1.primaryLockName))) results.push([javascriptManagerV1.primaryLockName, { externalNodes: readBunLockGraph(workspaceRoot).externalNodes }]);
  return results;
}
