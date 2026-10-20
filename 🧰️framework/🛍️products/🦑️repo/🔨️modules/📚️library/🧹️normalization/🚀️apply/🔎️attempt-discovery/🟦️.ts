/** 🔎️ The existing attempt discovery and validation of the taxonomy plan apply. */
import { normalizeRelative, splitLeadingEmoji } from "../../🛣️path/🟦️.ts";
import { lstatOrNull } from "../../📁️input/🟦️.ts";
import { lstatSync, mkdirSync, readdirSync } from "node:fs";
import { canonicalJson } from "../../../🧾️serialization/🔣️json/🟦️.ts";
import { basename, join } from "node:path";
import { requireTaxonomyValue } from "../../📜️plan-contracts/🟦️.ts";
import { absolutePath } from "../../🏷️file-kind-matching/🟦️.ts";
import { generatorPathCompare } from "../../🧾️plan-parsing/🟦️.ts";
import { type MutableJournalRecord, durableRemove, fsyncDirectory, readJournal } from "../../💾️durable-journal/🟦️.ts";
import { assertJournalBackupAuthority, assertJournalPhaseMembership, assertJournalPlanMembership } from "../../🩹️recovery-authorities/🟦️.ts";
import { canonicalDirectoryName } from "../../🧐️preimage-assertions/🟦️.ts";
import { transactionRepositoryFinally } from "../../🏛️repository-authority/🟦️.ts";
import { TRANSACTION_LEASE_TOKEN, type TransactionLeaseHandle, type TransactionLeaseRecord, parseTransactionLease, readTransactionLease, releaseTransactionLease, transactionLeaseProcessIsAlive } from "../../🔐️transaction-lease/🟦️.ts";
import { assertRecoveryRootNames, recoverReferenceEditPreparations, recoverRestorePreparations, recoverTransactionBackups } from "../../🛟️recovery/🟦️.ts";
import { cleanupCommittedTransaction } from "../../🚿️transaction-cleanup/🟦️.ts";
import { reconcileJournalWal, recoverCanonicalJsonCandidates } from "../../🔄️journal-reconciliation/🟦️.ts";
import { acquireTransactionLease } from "../../🔑️transaction-lease-acquisition/🟦️.ts";
import { actualAffectedDigest, validateLeasePreparationEvidence } from "../../🗃️affected-state/🟦️.ts";
import { cleanupRolledBackTransaction } from "../../↩️rollback-transaction/🟦️.ts";
import type { bindTransactionNames, checkPlanAuthority } from "../📜️plan-checks/🟦️.ts";

/** 🔭 Reads the existing and unpublished attempt directories of the plan transaction. */
export function discoverTransactionAttempts({ attemptsStat, attemptsAbsolute, attemptPreparationName, unpublishedAttempts, taxonomy, attemptsRelative, repoRoot, journalFilename, stageDirectory, backupDirectory, leaseDirectory, journalWriteDirectory, jsonPreviousName, journalJsonWritePreparationName, digest, plan, existingAttempts }: Pick<ReturnType<typeof checkPlanAuthority>, "attemptsStat" | "attemptsAbsolute" | "unpublishedAttempts" | "existingAttempts"> & Pick<ReturnType<typeof bindTransactionNames>, "attemptPreparationName" | "taxonomy" | "attemptsRelative" | "repoRoot" | "journalFilename" | "stageDirectory" | "backupDirectory" | "leaseDirectory" | "journalWriteDirectory" | "jsonPreviousName" | "journalJsonWritePreparationName" | "digest" | "plan">) {
  if (attemptsStat) {
    if (!attemptsStat.isDirectory() || attemptsStat.isSymbolicLink()) throw new Error("Transaction attempts authority must be a no-follow directory");
    for (const name of readdirSync(attemptsAbsolute).sort(generatorPathCompare)) {
      const childSlug = splitLeadingEmoji(name).rest;
      const preparation = /^prepare-([0-9]{6})-([1-9][0-9]*)-([0-9a-f-]+)$/u.exec(childSlug);
      if (preparation) {
        const ordinal = requireTaxonomyValue(preparation[1],"preparation[1]"), pid = Number.parseInt(requireTaxonomyValue(preparation[2],"preparation[2]"), 10), token = requireTaxonomyValue(preparation[3],"preparation[3]");
        if (!Number.isSafeInteger(pid) || !TRANSACTION_LEASE_TOKEN.test(token) || name !== attemptPreparationName(ordinal, pid, token)) throw new Error(`Unexpected transaction attempt preparation: ${name}`);
        const path = join(attemptsAbsolute, name);
        const stat = lstatOrNull(path);
        if (!stat?.isDirectory() || stat.isSymbolicLink()) throw new Error(`Transaction attempt preparation must be a no-follow directory: ${name}`);
        unpublishedAttempts.push({ ordinal, pid, token, path });
        continue;
      }
      const ordinal = childSlug;
      if (!/^[0-9]{6}$/u.test(ordinal) || name !== canonicalDirectoryName(taxonomy, "transaction-attempt", ordinal, "transaction-attempts")) throw new Error(`Unexpected transaction attempt entry: ${name}`);
      const attemptRelative = normalizeRelative(`${attemptsRelative}/${name}`);
      const attemptAbsolute = absolutePath(repoRoot, attemptRelative);
      const attemptStat = lstatOrNull(attemptAbsolute);
      if (!attemptStat?.isDirectory() || attemptStat.isSymbolicLink()) throw new Error(`Transaction attempt must be a no-follow directory: ${attemptRelative}`);
      const childNames = readdirSync(attemptAbsolute).sort(generatorPathCompare);
      if (childNames.some((child) => child !== journalFilename && child !== stageDirectory && child !== backupDirectory && child !== leaseDirectory)) throw new Error(`Transaction attempt contains an unexpected artifact: ${attemptRelative}`);
      const attemptJournalRelative = normalizeRelative(`${attemptRelative}/${journalFilename}`);
      const attemptJournalAbsolute = absolutePath(repoRoot, attemptJournalRelative);
      const journalStat = lstatOrNull(attemptJournalAbsolute);
      const expectedStaging = normalizeRelative(`${attemptRelative}/${stageDirectory}`);
      const expectedBackup = normalizeRelative(`${attemptRelative}/${backupDirectory}`);
      if (journalStat && (!journalStat.isFile() || journalStat.isSymbolicLink())) throw new Error(`Transaction attempt journal must be a regular no-follow file: ${attemptJournalRelative}`);
      const walRoot = join(absolutePath(repoRoot, expectedStaging), journalWriteDirectory);
      const prospectiveJournal = lstatOrNull(walRoot) ? recoverCanonicalJsonCandidates(walRoot, journalFilename, jsonPreviousName, journalJsonWritePreparationName, (candidatePath) => {
        const candidate = readJournal(candidatePath, journalWriteDirectory, journalJsonWritePreparationName, jsonPreviousName);
        if (candidate.planDigest !== digest || candidate.attemptOrdinal !== ordinal || candidate.stagingRoot !== expectedStaging || candidate.backupRoot !== expectedBackup) throw new Error(`Transaction attempt WAL identity does not match its canonical path: ${attemptJournalRelative}`);
        assertJournalPlanMembership(plan, candidate);
        assertJournalPhaseMembership(plan, candidate);
        assertJournalBackupAuthority(plan, candidate);
      }, true, true, attemptJournalAbsolute) : journalStat ? attemptJournalAbsolute : undefined;
      if (!prospectiveJournal) throw new Error(`Transaction attempt has no recoverable durable journal: ${attemptRelative}`);
      const attemptJournal = readJournal(journalStat ? attemptJournalAbsolute : prospectiveJournal, journalWriteDirectory, journalJsonWritePreparationName, jsonPreviousName);
      if (attemptJournal.planDigest !== digest || attemptJournal.attemptOrdinal !== ordinal) throw new Error(`Transaction attempt identity does not match its canonical path: ${attemptJournalRelative}`);
      if (attemptJournal.stagingRoot !== expectedStaging || attemptJournal.backupRoot !== expectedBackup) throw new Error(`Transaction attempt roots do not match ordinal ${ordinal}`);
      const stageStat = lstatOrNull(absolutePath(repoRoot, expectedStaging));
      const backupStat = lstatOrNull(absolutePath(repoRoot, expectedBackup));
      const leaseStat = lstatOrNull(join(attemptAbsolute, leaseDirectory));
      if (stageStat && (!stageStat.isDirectory() || stageStat.isSymbolicLink()) || backupStat && (!backupStat.isDirectory() || backupStat.isSymbolicLink())) throw new Error(`Transaction attempt stage/backup must be direct no-follow directories: ${ordinal}`);
      if (leaseStat && (!leaseStat.isDirectory() || leaseStat.isSymbolicLink())) throw new Error(`Transaction attempt lease must be a direct no-follow directory: ${ordinal}`);
      if (attemptJournal.state !== "rolled-back" && attemptJournal.state !== "committed" && (!stageStat || !backupStat)) throw new Error(`Active transaction attempt is missing stage/backup roots: ${ordinal}`);
      assertJournalPhaseMembership(plan, attemptJournal);
      assertJournalBackupAuthority(plan, attemptJournal);
      existingAttempts.push({ ordinal, attemptRelative, journal: attemptJournal, journalRelative: attemptJournalRelative });
    }
  }
  for (let index = 0; index < existingAttempts.length; index++) {
    if (requireTaxonomyValue(existingAttempts[index],"existingAttempts[index]").ordinal !== String(index + 1).padStart(6, "0")) throw new Error("Transaction attempt ordinals are not contiguous");
    if (index < existingAttempts.length - 1 && requireTaxonomyValue(existingAttempts[index],"existingAttempts[index]").journal.state !== "rolled-back") throw new Error("Only rolled-back attempts may precede another transaction attempt");
  }
  const unpublishedOrdinals = new Set<string>();
  const publishedOrdinals = new Set(existingAttempts.map((entry) => entry.ordinal));
  const exactNextUnpublishedOrdinal = String(existingAttempts.length + 1).padStart(6, "0");
  for (const preparation of unpublishedAttempts) {
    if (publishedOrdinals.has(preparation.ordinal)) throw new Error(`Transaction attempt preparation collides with canonical ordinal ${preparation.ordinal}`);
    if (preparation.ordinal !== exactNextUnpublishedOrdinal) throw new Error(`Transaction attempt preparation ordinal ${preparation.ordinal} is not exact next ordinal ${exactNextUnpublishedOrdinal}`);
    if (unpublishedOrdinals.has(preparation.ordinal)) throw new Error(`Transaction attempt preparations duplicate ordinal ${preparation.ordinal}`);
    unpublishedOrdinals.add(preparation.ordinal);
  }
}

/** ✅ Validates the unpublished and terminal attempts and closes the terminal ones. */
export function defineAttemptValidators({ unpublishedAttempts, stageDirectory, backupDirectory, leaseDirectory, journalFilename, taxonomy, attemptsRelative, digest, plan, journalWriteDirectory, journalJsonWritePreparationName, jsonPreviousName, leaseJsonWritePreparationName, existingAttempts, repoRoot, backupPreparationName, restorePreparationName, editPreparationName, leasePreparationName, backupWritePreparationName, backupWriteCandidateName, editWritePreparationName, editWriteCandidateName, ticketRoot, repositoryAuthority, transactionProbe }: Pick<ReturnType<typeof checkPlanAuthority>, "unpublishedAttempts" | "existingAttempts"> & Pick<ReturnType<typeof bindTransactionNames>, "stageDirectory" | "backupDirectory" | "leaseDirectory" | "journalFilename" | "taxonomy" | "attemptsRelative" | "digest" | "plan" | "journalWriteDirectory" | "journalJsonWritePreparationName" | "jsonPreviousName" | "leaseJsonWritePreparationName" | "repoRoot" | "backupPreparationName" | "restorePreparationName" | "editPreparationName" | "leasePreparationName" | "backupWritePreparationName" | "backupWriteCandidateName" | "editWritePreparationName" | "editWriteCandidateName" | "ticketRoot" | "repositoryAuthority" | "transactionProbe">) {
  const validateUnpublishedAttempt = (preparation: (typeof unpublishedAttempts)[number]): void => {
    if (transactionLeaseProcessIsAlive(preparation.pid)) throw new Error(`Transaction attempt preparation is active for pid ${preparation.pid}`);
    const allowed = new Set([stageDirectory, backupDirectory, leaseDirectory, journalFilename]);
    const children = readdirSync(preparation.path).sort(generatorPathCompare);
    if (children.some((name) => !allowed.has(name))) throw new Error(`Dead transaction attempt preparation contains unexpected evidence: ${basename(preparation.path)}`);
    const finalAttemptDirectory = canonicalDirectoryName(taxonomy, "transaction-attempt", preparation.ordinal, "transaction-attempts");
    const finalAttempt = normalizeRelative(`${attemptsRelative}/${finalAttemptDirectory}`);
    const assertPreparedIdentity = (prepared: MutableJournalRecord, path: string): void => {
      if (prepared.revision !== 0 || prepared.state !== "prepared" || prepared.planDigest !== digest || prepared.attemptOrdinal !== preparation.ordinal || prepared.stagingRoot !== normalizeRelative(`${finalAttempt}/${stageDirectory}`) || prepared.backupRoot !== normalizeRelative(`${finalAttempt}/${backupDirectory}`)) throw new Error(`Dead attempt preparation journal identity is invalid: ${path}`);
      assertJournalPlanMembership(plan, prepared);
      assertJournalPhaseMembership(plan, prepared);
      assertJournalBackupAuthority(plan, prepared);
    };
    for (const name of children) {
      const path = join(preparation.path, name);
      const stat = lstatOrNull(path);
      if (name === journalFilename) {
        if (!stat?.isFile() || stat.isSymbolicLink()) throw new Error(`Dead attempt preparation journal is not a no-follow file: ${path}`);
        assertPreparedIdentity(readJournal(path, journalWriteDirectory, journalJsonWritePreparationName, jsonPreviousName), path);
      } else {
        if (!stat?.isDirectory() || stat.isSymbolicLink()) throw new Error(`Dead attempt preparation child is not a no-follow directory: ${path}`);
        if (name === leaseDirectory) {
          recoverCanonicalJsonCandidates(path, journalFilename, jsonPreviousName, leaseJsonWritePreparationName, (candidate) => { const lease = parseTransactionLease(candidate, digest, preparation.ordinal); if (lease.pid !== preparation.pid) throw new Error(`Dead attempt preparation lease pid is invalid: ${path}`); }, false, true);
          if (lstatOrNull(join(path, journalFilename))) {
            const lease = parseTransactionLease(join(path, journalFilename), digest, preparation.ordinal);
            if (lease.pid !== preparation.pid) throw new Error(`Dead attempt preparation lease pid is invalid: ${path}`);
          }
        } else if (name === stageDirectory && readdirSync(path).length > 0) {
          const nested = readdirSync(path).sort(generatorPathCompare);
          if (canonicalJson(nested) !== canonicalJson([journalWriteDirectory])) throw new Error(`Dead attempt preparation stage contains unexpected evidence: ${path}`);
          const wal = join(path, journalWriteDirectory);
          const walStat = lstatOrNull(wal);
          if (!walStat?.isDirectory() || walStat.isSymbolicLink()) throw new Error(`Dead attempt preparation WAL is invalid: ${wal}`);
          recoverCanonicalJsonCandidates(wal, journalFilename, jsonPreviousName, journalJsonWritePreparationName, (candidate) => assertPreparedIdentity(readJournal(candidate, journalWriteDirectory, journalJsonWritePreparationName, jsonPreviousName), candidate), false, true);
          const walChildren = readdirSync(wal).sort(generatorPathCompare);
          if (walChildren.length > 0) {
            if (children.includes(journalFilename)) throw new Error(`Dead attempt preparation has both a durable journal and a pending initial WAL: ${wal}`);
            const walPath = join(wal, journalFilename);
            if (lstatOrNull(walPath)) {
              if (lstatSync(walPath).isSymbolicLink()) throw new Error(`Dead attempt preparation WAL snapshot is not a no-follow file: ${walPath}`);
              assertPreparedIdentity(readJournal(walPath, journalWriteDirectory, journalJsonWritePreparationName, jsonPreviousName), walPath);
            }
          }
        } else if (readdirSync(path).length > 0) throw new Error(`Dead attempt preparation executor root is not empty: ${path}`);
      }
    }
  };
  for (const preparation of unpublishedAttempts) validateUnpublishedAttempt(preparation);
  const validateTerminalAttempt = (attempt: (typeof existingAttempts)[number], ownedLease?: TransactionLeaseRecord): void => {
    if (attempt.journal.state !== "rolled-back" && attempt.journal.state !== "committed") return;
    const attemptAbsolute = absolutePath(repoRoot, attempt.attemptRelative);
    const stageAbsolute = absolutePath(repoRoot, attempt.journal.stagingRoot);
    const backupAbsolute = absolutePath(repoRoot, attempt.journal.backupRoot);
    const leaseAbsolute = join(attemptAbsolute, leaseDirectory);
    const leaseStat = lstatOrNull(leaseAbsolute);
    if (leaseStat) {
      const lease = readTransactionLease(leaseAbsolute, journalFilename, digest, attempt.ordinal);
      if (transactionLeaseProcessIsAlive(lease.pid) && (!ownedLease || canonicalJson(lease) !== canonicalJson(ownedLease))) throw new Error(`Terminal transaction attempt is leased by active pid ${lease.pid}`);
    }
    const stageStat = lstatOrNull(stageAbsolute), backupStat = lstatOrNull(backupAbsolute);
    if (!stageStat && !backupStat) {
      if (attempt.journal.state === "committed" && actualAffectedDigest(repoRoot, plan, taxonomy) !== plan.expectedPostStateDigest) throw new Error(`Committed attempt post-state changed before stale lease recovery: ${attempt.ordinal}`);
      return;
    }
    if (stageStat && (!stageStat.isDirectory() || stageStat.isSymbolicLink()) || backupStat && (!backupStat.isDirectory() || backupStat.isSymbolicLink()) || stageStat && !backupStat) throw new Error(`Terminal transaction attempt has an invalid executor tree: ${attempt.ordinal}`);
    assertRecoveryRootNames(repoRoot, plan, attempt.journal, backupPreparationName, restorePreparationName, editPreparationName, leasePreparationName);
    validateLeasePreparationEvidence(backupAbsolute, leasePreparationName, leaseJsonWritePreparationName, journalFilename, jsonPreviousName, digest, attempt.ordinal);
    recoverTransactionBackups(repoRoot, plan, attempt.journal, backupPreparationName, backupWritePreparationName, backupWriteCandidateName, true);
    recoverRestorePreparations(repoRoot, plan, attempt.journal, restorePreparationName, true);
    if (stageStat) {
      recoverReferenceEditPreparations(repoRoot, plan, attempt.journal, editPreparationName, editWritePreparationName, editWriteCandidateName, true);
      reconcileJournalWal(repoRoot, absolutePath(repoRoot, attempt.journalRelative), attempt.journal, plan, taxonomy, true);
    }
    const transient = [...(stageStat ? readdirSync(stageAbsolute) : []), ...readdirSync(backupAbsolute)].some((name) => splitLeadingEmoji(name).rest.startsWith("edit-") || splitLeadingEmoji(name).rest.startsWith("backup-") || splitLeadingEmoji(name).rest.startsWith("restore-") || splitLeadingEmoji(name).rest.startsWith("lease-") || name === journalWriteDirectory);
    if (!transient) {
      if (attempt.journal.state === "rolled-back") cleanupRolledBackTransaction(repoRoot, attempt.journal, plan, true);
      else {
        if (actualAffectedDigest(repoRoot, plan, taxonomy) !== plan.expectedPostStateDigest) throw new Error(`Committed attempt post-state changed: ${attempt.ordinal}`);
        cleanupCommittedTransaction(repoRoot, attempt.journal, plan, ticketRoot, true);
      }
    }
  };
  for (const attempt of existingAttempts) validateTerminalAttempt(attempt);
  const closeTerminalAttempt = (attempt: (typeof existingAttempts)[number]): void => {
    if (attempt.journal.state !== "rolled-back" && attempt.journal.state !== "committed") return;
    let terminalJournal = attempt.journal;
    const attemptAbsolute = absolutePath(repoRoot, attempt.attemptRelative);
    const stageAbsolute = absolutePath(repoRoot, attempt.journal.stagingRoot);
    const backupAbsolute = absolutePath(repoRoot, attempt.journal.backupRoot);
    const leaseAbsolute = join(attemptAbsolute, leaseDirectory);
    const hasResidue = Boolean(lstatOrNull(stageAbsolute) || lstatOrNull(backupAbsolute));
    let terminalLease: TransactionLeaseHandle | undefined;
    let createdLeaseBackup = false;
    if (hasResidue || lstatOrNull(leaseAbsolute)) {
      if (!hasResidue && attempt.journal.state === "committed" && actualAffectedDigest(repoRoot, plan, taxonomy) !== plan.expectedPostStateDigest) throw new Error(`Committed attempt post-state changed before stale lease recovery: ${attempt.ordinal}`);
      if (!lstatOrNull(backupAbsolute)) {
        mkdirSync(backupAbsolute);
        fsyncDirectory(attemptAbsolute);
        createdLeaseBackup = true;
      }
      terminalLease = acquireTransactionLease(repositoryAuthority, attempt.attemptRelative, attempt.journal.backupRoot, leaseDirectory, leasePreparationName, leaseJsonWritePreparationName, journalFilename, jsonPreviousName, digest, attempt.ordinal, (owned) => validateTerminalAttempt(attempt, owned), transactionProbe);
      if (!hasResidue && createdLeaseBackup) durableRemove(backupAbsolute, true);
      terminalJournal = reconcileJournalWal(repoRoot, absolutePath(repoRoot, attempt.journalRelative), terminalJournal, plan, taxonomy);
    }
    transactionRepositoryFinally(() => {
      if (terminalJournal.state === "rolled-back") cleanupRolledBackTransaction(repoRoot, terminalJournal, plan);
      else {
        if (actualAffectedDigest(repoRoot, plan, taxonomy) !== plan.expectedPostStateDigest) throw new Error(`Committed attempt post-state changed: ${attempt.ordinal}`);
        cleanupCommittedTransaction(repoRoot, terminalJournal, plan, ticketRoot);
      }
    }, () => {
      if (terminalLease) releaseTransactionLease(repositoryAuthority, `${attempt.attemptRelative}/${leaseDirectory}`, terminalLease);
    });
    if (canonicalJson(readdirSync(attemptAbsolute).sort(generatorPathCompare)) !== canonicalJson([journalFilename])) throw new Error(`Terminal transaction attempt is not closed: ${attempt.ordinal}`);
  };
  return { closeTerminalAttempt };
}
