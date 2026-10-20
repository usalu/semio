import { readdirSync } from "node:fs";
import { canonicalJson } from "../../🧾️serialization/🔣️json/🟦️.ts";
import { type TaxonomyApplyOptions, type TaxonomyApplyResult, type TaxonomyPlan } from "../📜️plan-contracts/🟦️.ts";
import { absolutePath } from "../🏷️file-kind-matching/🟦️.ts";
import { generatorPathCompare } from "../🧾️plan-parsing/🟦️.ts";
import { actualAffectedDigest } from "../🗃️affected-state/🟦️.ts";
import { runTransactionAttempt } from "./🏁️transaction-run/🟦️.ts";
import { bindTransactionNames, checkPlanAuthority } from "./📜️plan-checks/🟦️.ts";
import { defineAttemptValidators, discoverTransactionAttempts } from "./🔎️attempt-discovery/🟦️.ts";
import { defineLeaseAndPreChecks, selectAttempt } from "./🎯️attempt-selection/🟦️.ts";
import { capturePreflight, lockResumeSnapshot } from "./🛫️preflight-and-lock/🟦️.ts";
import { prepareAttemptJournal } from "./🪪️journal-preparation/🟦️.ts";

/** 🛠️ Applies a digest-verified plan through two-phase staging, journaled backups, verification, cancellation, resume and rollback. */
export function applyTaxonomyPlan(rawPlan: TaxonomyPlan, options: TaxonomyApplyOptions): TaxonomyApplyResult {
  const { bootstrap, repoRoot, planBytes, plan, transactionRootRelative, taxonomy, attemptsRelative, transactionRelative, attemptsDirectory, attemptPreparationName, journalFilename, stageDirectory, backupDirectory, leaseDirectory, journalWriteDirectory, jsonPreviousName, journalJsonWritePreparationName, digest, leaseJsonWritePreparationName, backupPreparationName, restorePreparationName, editPreparationName, leasePreparationName, backupWritePreparationName, backupWriteCandidateName, editWritePreparationName, editWriteCandidateName, ticketRoot, repositoryAuthority, transactionProbe } = bindTransactionNames({ rawPlan, options });
  const { attemptsStat, attemptsAbsolute, unpublishedAttempts, existingAttempts, resumeRelative, planAuthority, transactionRootAbsolute } = checkPlanAuthority({ bootstrap, options, repoRoot, planBytes, plan, transactionRootRelative, taxonomy, attemptsRelative, transactionRelative, attemptsDirectory });
  discoverTransactionAttempts({ attemptsStat, attemptsAbsolute, attemptPreparationName, unpublishedAttempts, taxonomy, attemptsRelative, repoRoot, journalFilename, stageDirectory, backupDirectory, leaseDirectory, journalWriteDirectory, jsonPreviousName, journalJsonWritePreparationName, digest, plan, existingAttempts });
  const { closeTerminalAttempt } = defineAttemptValidators({ unpublishedAttempts, stageDirectory, backupDirectory, leaseDirectory, journalFilename, taxonomy, attemptsRelative, digest, plan, journalWriteDirectory, journalJsonWritePreparationName, jsonPreviousName, leaseJsonWritePreparationName, existingAttempts, repoRoot, backupPreparationName, restorePreparationName, editPreparationName, leasePreparationName, backupWritePreparationName, backupWriteCandidateName, editWritePreparationName, editWriteCandidateName, ticketRoot, repositoryAuthority, transactionProbe });
  const { txn, attemptRelative, journalPath } = selectAttempt({ existingAttempts, resumeRelative, digest, taxonomy, attemptsRelative, journalFilename, repoRoot });
  const { acquireLease, releaseLease } = defineLeaseAndPreChecks({ txn, repositoryAuthority, attemptRelative, leaseDirectory, leasePreparationName, leaseJsonWritePreparationName, journalFilename, jsonPreviousName, digest, transactionProbe, plan, repoRoot, taxonomy, options });
  capturePreflight({ options, repositoryAuthority, repoRoot, bootstrap, transactionRootRelative, plan, taxonomy, planAuthority, txn });
  lockResumeSnapshot({ txn, repoRoot, leaseDirectory, journalFilename, digest, journalWriteDirectory, jsonPreviousName, journalJsonWritePreparationName, plan, taxonomy, backupPreparationName, restorePreparationName, editPreparationName, leasePreparationName, leaseJsonWritePreparationName, backupWritePreparationName, backupWriteCandidateName, editWritePreparationName, editWriteCandidateName, options, planAuthority, transactionRootRelative, acquireLease, unpublishedAttempts, existingAttempts, closeTerminalAttempt, releaseLease });
  if (options.resumeJournal && txn.selectedAttempt?.journal.state === "committed") {
    if (actualAffectedDigest(repoRoot, plan, taxonomy) !== plan.expectedPostStateDigest) throw new Error("Committed resume post-state digest changed");
    const terminalRoot = absolutePath(repoRoot, txn.selectedAttempt.attemptRelative);
    if (canonicalJson(readdirSync(terminalRoot).sort(generatorPathCompare)) !== canonicalJson([journalFilename])) throw new Error("Committed resume attempt is not terminal journal-only evidence");
    return { planDigest: digest, journalPath, state: "committed", appliedMoves: plan.moves.length, appliedEmbeddedTicketRootRelocations: plan.embeddedTicketRootRelocations.length, appliedSymlinkTargetEdits: plan.symlinkTargetEdits.length, appliedEvidenceRemovals: plan.evidenceRemovals.length, appliedEdits: plan.edits.length, appliedRegenerations: plan.regenerations.length };
  }
  const prepared = prepareAttemptJournal({ options, txn, repoRoot, journalPath, plan, taxonomy, transactionProbe, backupPreparationName, backupWritePreparationName, backupWriteCandidateName, restorePreparationName, editPreparationName, editWritePreparationName, editWriteCandidateName, attemptRelative, stageDirectory, backupDirectory, digest, ticketRoot, releaseLease, planAuthority, transactionRootRelative, journalWriteDirectory, journalJsonWritePreparationName, jsonPreviousName, attemptsRelative, attemptsAbsolute, transactionRelative, transactionRootAbsolute, attemptPreparationName, leaseDirectory, journalFilename, leaseJsonWritePreparationName });
  if (!("journal" in prepared)) return prepared;
  const { journal, sourceSet } = prepared;
  return runTransactionAttempt({ options, transactionProbe, attemptRelative, repoRoot, plan, journal, sourceSet, txn, journalPath, backupPreparationName, backupWritePreparationName, backupWriteCandidateName, editPreparationName, editWritePreparationName, editWriteCandidateName, taxonomy, ticketRoot, planAuthority, transactionRootRelative, releaseLease, digest, journalWriteDirectory, journalJsonWritePreparationName, jsonPreviousName, journalFilename, restorePreparationName, leasePreparationName, leaseJsonWritePreparationName });
}
