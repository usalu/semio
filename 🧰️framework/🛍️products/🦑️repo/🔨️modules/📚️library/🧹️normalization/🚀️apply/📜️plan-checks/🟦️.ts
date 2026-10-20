/** 📜️ The plan and transaction name checks of the taxonomy plan apply. */
import { report } from "../../🏃️operation/🟦️.ts";
import { loadTaxonomy } from "../../🔣️taxonomy/🟦️.ts";
import { normalizeRelative, splitLeadingEmoji } from "../../🛣️path/🟦️.ts";
import { assertLexicalInputOutsideOpaque, lstatOrNull, assertNoFollowAncestors } from "../../📁️input/🟦️.ts";
import { readFileSync, readdirSync } from "node:fs";
import { canonicalJson } from "../../../🧾️serialization/🔣️json/🟦️.ts";
import { join, relative, resolve } from "node:path";
import { PLAN_COMMIT_ID, PLAN_HASH, type TaxonomyApplyOptions, type TaxonomyPlan } from "../../📜️plan-contracts/🟦️.ts";
import { absolutePath, isExcluded } from "../../🏷️file-kind-matching/🟦️.ts";
import { generatorPathCompare } from "../../🧾️plan-parsing/🟦️.ts";
import { type MutableJournalRecord } from "../../💾️durable-journal/🟦️.ts";
import { assertPlanOutsideTransaction, canonicalDirectoryName, canonicalKindOnlyFilename, canonicalScopedKindOnlyFilename, pathsOverlap, removalAuthorityPaths } from "../../🧐️preimage-assertions/🟦️.ts";
import { assertTransactionRepositoryPath, captureTransactionRepositoryAuthority, transactionRepositoryBootstrapPaths } from "../../🏛️repository-authority/🟦️.ts";
import { taxonomyPlanDigest } from "../../🗃️affected-state/🟦️.ts";
import { parseTaxonomyPlan } from "../../📃️plan-document/🟦️.ts";

/** 🏷 Verifies the plan against the options and binds the canonical transaction, attempt, lease and preparation names. */
export function bindTransactionNames({ rawPlan, options }: { readonly rawPlan: TaxonomyPlan; readonly options: TaxonomyApplyOptions }) {
  const plan = parseTaxonomyPlan(rawPlan);
  const repoRoot = resolve(options.repoRoot);
  if (!PLAN_COMMIT_ID.test(options.expectedBaselineCommit) || options.expectedBaselineCommit !== plan.baselineCommit) throw new Error("Plan baselineCommit does not match expectedBaselineCommit authority");
  if (options.workers !== undefined && (!Number.isSafeInteger(options.workers) || options.workers < 1)) throw new Error("workers must be a positive integer");
  const digest = taxonomyPlanDigest(plan);
  if (plan.planDigest !== digest) throw new Error("Plan digest does not match canonical plan bytes");
  if (options.expectedPlanDigest !== undefined && options.expectedPlanDigest !== digest) throw new Error("Plan digest does not match expectedPlanDigest");
  if (plan.unresolved.some((entry) => entry.severity === "error")) throw new Error("Plan has unresolved blocking violations");
  const bootstrap = transactionRepositoryBootstrapPaths(plan, options, repoRoot);
  const repositoryAuthority = captureTransactionRepositoryAuthority(repoRoot);
  for (const access of bootstrap.accesses) assertTransactionRepositoryPath(repositoryAuthority, access.path, access.role, access.label);
  const taxonomy = loadTaxonomy({ repoRoot, taxonomyPath: absolutePath(repoRoot, bootstrap.taxonomyPath) });
  const transactionProbe = (phase: string, path?: string): void => report(options.progress, "apply", phase, 0, 0, path);
  const ticketRelative = bootstrap.ticketDir;
  if (isExcluded(ticketRelative, taxonomy)) throw new Error(`Ticket directory is opaque: ${ticketRelative}`);
  const ticketRoot = absolutePath(repoRoot, ticketRelative);
  const transactionDirectory = canonicalDirectoryName(taxonomy, "taxonomy-transaction", "taxonomy-transaction");
  const digestDirectory = canonicalDirectoryName(taxonomy, "transaction-digest", digest, "taxonomy-transaction");
  const transactionRootRelative = normalizeRelative(`${ticketRelative}/${transactionDirectory}`);
  assertTransactionRepositoryPath(repositoryAuthority, transactionRootRelative, "subtree", "Taxonomy transaction executor");
  const transactionRelative = normalizeRelative(`${transactionRootRelative}/${digestDirectory}`);
  const attemptsDirectory = canonicalDirectoryName(taxonomy, "transaction-attempts", "attempts", "transaction-digest");
  const stageDirectory = canonicalDirectoryName(taxonomy, "transaction-stage", "stage", "transaction-attempt");
  const backupDirectory = canonicalDirectoryName(taxonomy, "transaction-backup", "backup", "transaction-attempt");
  const leaseDirectory = canonicalDirectoryName(taxonomy, "transaction-lease", "lease", "transaction-attempt");
  const journalWriteDirectory = canonicalDirectoryName(taxonomy, "transaction-journal-write", "journal", "transaction-stage");
  const attemptPreparationName = (ordinal: string, pid: number, token: string): string => canonicalDirectoryName(taxonomy, "transaction-attempt-preparation", `prepare-${ordinal}-${pid}-${token}`, "transaction-attempts");
  const leasePreparationName = (pid: number, token: string, state: "preparing" | "stale"): string => canonicalDirectoryName(taxonomy, "transaction-lease-preparation", `lease-${pid}-${token}-${state}`, "transaction-backup");
  const journalJsonWritePreparationName = (pid: number, token: string): string => canonicalDirectoryName(taxonomy, "transaction-json-write-preparation", `write-${pid}-${token}`, "transaction-journal-write");
  const leaseJsonWritePreparationName = (pid: number, token: string): string => canonicalDirectoryName(taxonomy, "transaction-json-write-preparation", `write-${pid}-${token}`, "transaction-lease-preparation");
  const backupPreparationName = (identity: string, pid: number, token: string): string => canonicalDirectoryName(taxonomy, "transaction-backup-preparation", `backup-${identity}-${pid}-${token}`, "transaction-backup");
  const backupWritePreparationName = (pid: number, token: string): string => canonicalDirectoryName(taxonomy, "transaction-backup-write-preparation", `write-${pid}-${token}`, "transaction-backup-preparation");
  const restorePreparationName = (identity: string, pid: number, token: string): string => canonicalDirectoryName(taxonomy, "transaction-restore-preparation", `restore-${identity}-${pid}-${token}`, "transaction-backup");
  const editPreparationName = (identity: string, pid: number, token: string): string => canonicalDirectoryName(taxonomy, "transaction-edit-preparation", `edit-${identity}-${pid}-${token}`, "transaction-stage");
  const editWritePreparationName = (pid: number, token: string): string => canonicalDirectoryName(taxonomy, "transaction-edit-write-preparation", `write-${pid}-${token}`, "transaction-edit-preparation");
  const attemptsRelative = normalizeRelative(`${transactionRelative}/${attemptsDirectory}`);
  const journalFilename = canonicalKindOnlyFilename(taxonomy, "json", ".json");
  const jsonPreviousName = canonicalScopedKindOnlyFilename(taxonomy, "transaction-json-previous", "transaction-json-write-preparation", ".json");
  const backupWriteCandidateName = canonicalScopedKindOnlyFilename(taxonomy, "transaction-backup-write-candidate", "transaction-backup-write-preparation", ".backup");
  const editWriteCandidateName = canonicalScopedKindOnlyFilename(taxonomy, "transaction-edit-write-candidate", "transaction-edit-write-preparation", ".edit");
  const planBytes = Buffer.from(`${canonicalJson(plan)}\n`);
  return { bootstrap, repoRoot, planBytes, plan, transactionRootRelative, taxonomy, attemptsRelative, transactionRelative, attemptsDirectory, attemptPreparationName, journalFilename, stageDirectory, backupDirectory, leaseDirectory, journalWriteDirectory, jsonPreviousName, journalJsonWritePreparationName, digest, leaseJsonWritePreparationName, backupPreparationName, restorePreparationName, editPreparationName, leasePreparationName, backupWritePreparationName, backupWriteCandidateName, editWritePreparationName, editWriteCandidateName, ticketRoot, repositoryAuthority, transactionProbe };
}

/** 🛡 Captures the plan bytes, checks the plan and cancellation file stay outside the transaction and reads the transaction ancestors. */
export function checkPlanAuthority({ bootstrap, options, repoRoot, planBytes, plan, transactionRootRelative, taxonomy, attemptsRelative, transactionRelative, attemptsDirectory }: Pick<ReturnType<typeof bindTransactionNames>, "bootstrap" | "repoRoot" | "planBytes" | "plan" | "transactionRootRelative" | "taxonomy" | "attemptsRelative" | "transactionRelative" | "attemptsDirectory"> & { readonly options: TaxonomyApplyOptions }) {
  const planAuthority = (() => {
    const candidateRelative = bootstrap.planArtifactPath;
    if (options.planArtifactPath) assertLexicalInputOutsideOpaque(repoRoot, options.planArtifactPath, "planArtifactPath", true);
    const candidate = absolutePath(repoRoot, candidateRelative);
    if (!options.planArtifactPath) assertNoFollowAncestors(repoRoot, candidate, "canonical plan artifact", true);
    const stat = lstatOrNull(candidate);
    if (options.planArtifactPath && (!stat?.isFile() || stat.isSymbolicLink() || !readFileSync(candidate).equals(planBytes))) throw new Error("planArtifactPath must be a regular no-follow file containing the exact canonical plan bytes");
    return stat?.isFile() && !stat.isSymbolicLink() && readFileSync(candidate).equals(planBytes) ? { path: candidateRelative, bytes: planBytes } : undefined;
  })();
  const resumeRelative = bootstrap.resumeJournal;
  assertPlanOutsideTransaction(plan, transactionRootRelative, taxonomy, repoRoot);
  if (options.cancelFile) {
    const cancelAbsolute = assertLexicalInputOutsideOpaque(repoRoot, options.cancelFile, "cancelFile", true);
    const cancelRelative = normalizeRelative(relative(repoRoot, cancelAbsolute).replaceAll("\\", "/"));
    const mutationPaths = [
      ...plan.moves.flatMap((entry) => [entry.sourcePath, entry.destinationPath]),
      ...plan.embeddedTicketRoots.flatMap((entry) => [entry.sourceMetadataRoot, entry.sourceTicketRoot, entry.canonicalTicketRoot]),
      ...plan.embeddedTicketRootRelocations.flatMap((entry) => [entry.sourcePath, entry.destinationPath]),
      ...plan.evidenceRemovals.flatMap((entry) => [entry.sourcePath, ...removalAuthorityPaths(entry.authority)]),
      ...plan.symlinkTargetEdits.flatMap((entry) => [entry.sourcePath, entry.finalPath, entry.logicalTargetSourcePath, entry.logicalTargetFinalPath]),
      ...plan.edits.map((entry) => entry.path),
      ...plan.regenerations.flatMap((entry) => [entry.cwd, ...entry.outputRoots, ...entry.inputs.map((input) => input.path), ...entry.preOutputs.map((output) => output.path), ...entry.outputs.map((output) => output.path), ...entry.staleRemovals]),
    ];
    if (pathsOverlap(cancelRelative, transactionRootRelative) || mutationPaths.some((path) => pathsOverlap(cancelRelative, path))) throw new Error(`cancelFile overlaps transaction or mutation authority: ${cancelRelative}`);
  }
  const existingAttempts: { readonly ordinal: string; readonly attemptRelative: string; readonly journal: MutableJournalRecord; readonly journalRelative: string }[] = [];
  const unpublishedAttempts: { readonly ordinal: string; readonly pid: number; readonly token: string; readonly path: string }[] = [];
  const attemptsAbsolute = absolutePath(repoRoot, attemptsRelative);
  assertNoFollowAncestors(repoRoot, attemptsAbsolute, "transaction attempts root", true);
  const transactionRootAbsolute = absolutePath(repoRoot, transactionRootRelative);
  const transactionRootStat = lstatOrNull(transactionRootAbsolute);
  if (transactionRootStat) {
    if (!transactionRootStat.isDirectory() || transactionRootStat.isSymbolicLink()) throw new Error("Taxonomy transaction root must be a no-follow directory");
    for (const name of readdirSync(transactionRootAbsolute).sort(generatorPathCompare)) {
      const childDigest = splitLeadingEmoji(name).rest;
      if (!PLAN_HASH.test(childDigest) || name !== canonicalDirectoryName(taxonomy, "transaction-digest", childDigest, "taxonomy-transaction")) throw new Error(`Unexpected taxonomy transaction-root entry: ${name}`);
      const stat = lstatOrNull(join(transactionRootAbsolute, name));
      if (!stat?.isDirectory() || stat.isSymbolicLink()) throw new Error(`Transaction digest must be a no-follow directory: ${name}`);
    }
  }
  const digestAbsolute = absolutePath(repoRoot, transactionRelative);
  const digestStat = lstatOrNull(digestAbsolute);
  if (digestStat) {
    if (!digestStat.isDirectory() || digestStat.isSymbolicLink()) throw new Error("Selected transaction digest must be a no-follow directory");
    const digestChildren = readdirSync(digestAbsolute).sort(generatorPathCompare);
    if (digestChildren.some((name) => name !== attemptsDirectory)) throw new Error("Selected transaction digest contains an unexpected artifact");
  }
  const attemptsStat = lstatOrNull(attemptsAbsolute);
  return { attemptsStat, attemptsAbsolute, unpublishedAttempts, existingAttempts, resumeRelative, planAuthority, transactionRootAbsolute };
}
