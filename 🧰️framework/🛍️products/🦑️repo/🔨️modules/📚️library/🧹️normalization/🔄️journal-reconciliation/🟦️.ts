import { type LoadedTaxonomy } from "../🔣️taxonomy/📚️loaded/🟦️.ts";
import { splitLeadingEmoji } from "../🛣️path/🟦️.ts";
import { lstatOrNull } from "../📁️input/🟦️.ts";
import { readFileSync, readdirSync } from "node:fs";
import { canonicalJson } from "../../🧾️serialization/🔣️json/🟦️.ts";
import { basename, join } from "node:path";
import { type TaxonomyPlan, requireTaxonomyValue } from "../📜️plan-contracts/🟦️.ts";
import { absolutePath } from "../🏷️file-kind-matching/🟦️.ts";
import { generatorPathCompare } from "../🧾️plan-parsing/🟦️.ts";
import { type MutableJournalRecord, assertJournalTransition, durableRemove, durableRename, journalSnapshot, readJournal } from "../💾️durable-journal/🟦️.ts";
import { assertJournalBackupAuthority, assertJournalPhaseMembership, assertJournalPlanMembership } from "../🩹️recovery-authorities/🟦️.ts";
import { isTransactionRepositoryAuthorityError } from "../🏛️repository-authority/🟦️.ts";
import { reconcileRollbackTuples } from "../⏪️rollback-reconciliation/🟦️.ts";
import { TRANSACTION_LEASE_TOKEN, transactionLeaseProcessIsAlive } from "../🔐️transaction-lease/🟦️.ts";
import { reconcileTransactionOwnedTuples } from "../🛟️recovery/🟦️.ts";
import { assertActiveTransactionEvidence } from "../🚿️transaction-cleanup/🟦️.ts";

function canonicalJsonFile(path: string): boolean {
  try {
    const bytes = readFileSync(path, "utf8");
    return bytes === `${canonicalJson(JSON.parse(bytes))}\n`;
  } catch (error) {
    if (isTransactionRepositoryAuthorityError(error)) throw error;
    return false;
  }
}

export function recoverCanonicalJsonCandidates(container: string, finalName: string, previousName: string, preparationName: (pid: number, token: string) => string, validate: (path: string) => void, serialized: boolean, validateOnly = false, finalPath?: string): string | undefined {
  const stat = lstatOrNull(container);
  if (!stat) return undefined;
  if (!stat.isDirectory() || stat.isSymbolicLink()) throw new Error(`Canonical JSON container must be a no-follow directory: ${container}`);
  const actions: { root: string; candidate?: string; previous?: string; exchange: boolean; publish: boolean }[] = [];
  const final = finalPath ?? join(container, finalName);
  let prospective = lstatOrNull(final) ? final : undefined;
  for (const name of readdirSync(container).sort(generatorPathCompare)) {
    if (name === finalName) continue;
    const match = /^write-([1-9][0-9]*)-([0-9a-f-]+)$/u.exec(splitLeadingEmoji(name).rest);
    if (!match || !TRANSACTION_LEASE_TOKEN.test(requireTaxonomyValue(match[2],"match[2]")) || name !== preparationName(Number.parseInt(requireTaxonomyValue(match[1],"match[1]"), 10), requireTaxonomyValue(match[2],"match[2]"))) throw new Error(`Canonical JSON write preparation name is invalid: ${name}`);
    const root = join(container, name), rootStat = lstatOrNull(root);
    if (!rootStat?.isDirectory() || rootStat.isSymbolicLink()) throw new Error(`Canonical JSON write preparation must be a no-follow directory: ${name}`);
    const children = readdirSync(root).sort(generatorPathCompare);
    if (children.length > 2 || children.some((child) => child !== finalName && child !== previousName)) throw new Error(`Canonical JSON write preparation contains unexpected evidence: ${name}`);
    const previous = children.includes(previousName) ? join(root, previousName) : undefined;
    if (previous) {
      const previousStat = lstatOrNull(previous);
      if (!previousStat?.isFile() || previousStat.isSymbolicLink() || !canonicalJsonFile(previous)) throw new Error(`Canonical JSON previous evidence is invalid: ${previous}`);
      validate(previous);
    }
    if (children.length === 0) {
      if (!serialized && transactionLeaseProcessIsAlive(Number.parseInt(requireTaxonomyValue(match[1],"match[1]"), 10))) throw new Error(`Canonical JSON write preparation is active for pid ${match[1]}`);
      actions.push({ root, exchange: false, publish: false });
      continue;
    }
    const leaf = children.includes(finalName) ? join(root, finalName) : undefined, leafStat = leaf ? lstatOrNull(leaf) : undefined;
    if (leaf && (!leafStat?.isFile() || leafStat.isSymbolicLink())) throw new Error(`Canonical JSON write candidate must be a regular no-follow file: ${leaf}`);
    if (leaf && !canonicalJsonFile(leaf)) {
      if (previous) throw new Error(`Canonical JSON exchanged candidate is invalid: ${leaf}`);
      if (!serialized && transactionLeaseProcessIsAlive(Number.parseInt(requireTaxonomyValue(match[1],"match[1]"), 10))) throw new Error(`Canonical JSON write preparation is active for pid ${match[1]}`);
      actions.push({ root, exchange: false, publish: false });
      continue;
    }
    if (leaf) validate(leaf);
    const finalStat = lstatOrNull(final);
    if (finalStat && (!finalStat.isFile() || finalStat.isSymbolicLink() || !canonicalJsonFile(final))) throw new Error(`Canonical JSON destination is invalid: ${final}`);
    if (leaf && previous && finalStat) throw new Error(`Canonical JSON exchange has simultaneous previous and durable destinations: ${root}`);
    if (!leaf && previous && !finalStat) throw new Error(`Canonical JSON previous-only state has no durable destination: ${root}`);
    if (!leaf && previous) validate(final);
    const equal = Boolean(leaf && finalStat && readFileSync(final).equals(readFileSync(leaf)));
    const exchange = Boolean(leaf && finalStat && !equal && !previous);
    const publish = Boolean(leaf && !finalStat);
    prospective = leaf && (exchange || publish) ? leaf : finalStat ? final : prospective;
    actions.push({ root, candidate: leaf, previous, exchange, publish });
  }
  if (actions.length > 1) throw new Error(`Canonical JSON container has duplicate write preparations: ${container}`);
  if (!validateOnly) for (const action of actions) {
    const previous = join(action.root, previousName);
    if (action.exchange) durableRename(final, previous);
    if (action.publish || action.exchange) durableRename(action.candidate!, final);
    if (lstatOrNull(previous)) durableRemove(previous);
    durableRemove(action.root, true);
  }
  return validateOnly ? prospective : lstatOrNull(final) ? final : undefined;
}

export function reconcileJournalWal(repoRoot: string, path: string, current: MutableJournalRecord, plan: TaxonomyPlan, taxonomy: LoadedTaxonomy, validateOnly = false): MutableJournalRecord {
  const walRoot = join(absolutePath(repoRoot, current.stagingRoot), current.journalWriteDirectory);
  const stat = lstatOrNull(walRoot);
  if (!stat) return current;
  if (!stat.isDirectory() || stat.isSymbolicLink()) throw new Error(`Taxonomy journal WAL must be a direct no-follow directory: ${walRoot}`);
  if (readdirSync(walRoot).length === 0) {
    if (!validateOnly) durableRemove(walRoot, true);
    return current;
  }
  const validateCandidate = (candidatePath: string): void => {
    const candidate = readJournal(candidatePath, current.journalWriteDirectory, current.jsonWritePreparationName, current.jsonPreviousName);
    if (canonicalJson(journalSnapshot(candidate)) === canonicalJson(journalSnapshot(current))) return;
    if (candidate.revision + 1 === current.revision && candidate.planDigest === current.planDigest && candidate.attemptOrdinal === current.attemptOrdinal && candidate.stagingRoot === current.stagingRoot && candidate.backupRoot === current.backupRoot) {
      assertJournalTransition(candidate, current);
      assertJournalPlanMembership(plan, candidate);
      assertJournalPhaseMembership(plan, candidate);
      assertJournalBackupAuthority(plan, candidate);
      return;
    }
    if (candidate.revision !== current.revision + 1 || candidate.planDigest !== current.planDigest || candidate.attemptOrdinal !== current.attemptOrdinal || candidate.stagingRoot !== current.stagingRoot || candidate.backupRoot !== current.backupRoot) throw new Error("Taxonomy journal WAL candidate identity or revision differs from its durable attempt");
    assertJournalTransition(current, candidate);
    assertJournalPlanMembership(plan, candidate);
    assertJournalPhaseMembership(plan, candidate);
    assertJournalBackupAuthority(plan, candidate);
    assertActiveTransactionEvidence(repoRoot, plan, candidate, false, true);
    const changed = candidate.state === "rolling-back" || candidate.state === "rolled-back" ? reconcileRollbackTuples(repoRoot, plan, candidate, taxonomy) : reconcileTransactionOwnedTuples(repoRoot, plan, candidate, taxonomy);
    if (changed) throw new Error("Taxonomy journal WAL candidate does not exactly match its durable filesystem tuples");
  };
  const prospectiveWal = recoverCanonicalJsonCandidates(walRoot, basename(path), current.jsonPreviousName, current.jsonWritePreparationName, validateCandidate, true, validateOnly, path);
  if (!prospectiveWal) {
    if (!validateOnly) durableRemove(walRoot, true);
    return current;
  }
  const walPath = validateOnly ? prospectiveWal : path;
  const walStat = lstatOrNull(walPath);
  if (!walStat?.isFile() || walStat.isSymbolicLink()) throw new Error(`Taxonomy journal WAL snapshot is not a regular no-follow file: ${walPath}`);
  const next = readJournal(walPath, current.journalWriteDirectory, current.jsonWritePreparationName, current.jsonPreviousName);
  if (canonicalJson(journalSnapshot(next)) === canonicalJson(journalSnapshot(current))) {
    if (!validateOnly) durableRemove(walRoot, true);
    return current;
  }
  if (next.revision !== current.revision + 1 || next.planDigest !== current.planDigest || next.attemptOrdinal !== current.attemptOrdinal || next.stagingRoot !== current.stagingRoot || next.backupRoot !== current.backupRoot) throw new Error("Taxonomy journal WAL identity or revision differs from its durable attempt");
  assertJournalTransition(current, next);
  assertJournalPlanMembership(plan, next);
  assertJournalPhaseMembership(plan, next);
  assertJournalBackupAuthority(plan, next);
  assertActiveTransactionEvidence(repoRoot, plan, next, false, true);
  const tupleChanged = next.state === "rolling-back" || next.state === "rolled-back" ? reconcileRollbackTuples(repoRoot, plan, next, taxonomy) : reconcileTransactionOwnedTuples(repoRoot, plan, next, taxonomy);
  if (tupleChanged) throw new Error("Taxonomy journal WAL does not exactly match its durable filesystem tuples");
  if (!validateOnly) {
    durableRemove(walRoot, true);
  }
  return next;
}
