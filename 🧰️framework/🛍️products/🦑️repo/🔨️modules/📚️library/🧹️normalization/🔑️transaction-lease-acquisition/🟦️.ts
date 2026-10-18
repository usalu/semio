import { normalizeRelative, splitLeadingEmoji } from "../🛣️path/🟦️.ts";
import { lstatOrNull } from "../📁️input/🟦️.ts";
import { randomUUID } from "node:crypto";
import { mkdirSync, readdirSync } from "node:fs";
import { canonicalJson } from "../../🧾️serialization/🔣️json/🟦️.ts";
import { basename, join, relative } from "node:path";
import { requireTaxonomyValue } from "../📜️plan-contracts/🟦️.ts";
import { absolutePath } from "../🏷️file-kind-matching/🟦️.ts";
import { generatorPathCompare } from "../🧾️plan-parsing/🟦️.ts";
import { durableRemove, durableRename, fsyncDirectory, publishCanonicalJsonCandidate } from "../💾️durable-journal/🟦️.ts";
import { TransactionRepositoryAuthority, isTransactionRepositoryAuthorityError } from "../🏛️repository-authority/🟦️.ts";
import { TRANSACTION_LEASE_TOKEN, type TransactionLeaseHandle, type TransactionLeaseRecord, assertTransactionLeaseRepositoryAuthority, parseTransactionLease, readTransactionLease, transactionLeaseProcessIsAlive } from "../🔐️transaction-lease/🟦️.ts";
import { recoverCanonicalJsonCandidates } from "../🔄️journal-reconciliation/🟦️.ts";

export function acquireTransactionLease(authority: TransactionRepositoryAuthority, attemptRelative: string, backupRelative: string, leaseDirectory: string, leasePreparationName: (pid: number, token: string, state: "preparing" | "stale") => string, jsonWritePreparationName: (pid: number, token: string) => string, filename: string, previousName: string, planDigest: string, attemptOrdinal: string, beforePublish?: (owned?: TransactionLeaseRecord) => void, probe?: (phase: string, path?: string) => void): TransactionLeaseHandle {
  assertTransactionLeaseRepositoryAuthority(authority, { kind: "acquire", attemptRelative, backupRelative, leaseDirectory, filename, previousName });
  const repoRoot = authority.repoRoot;
  const invokeLeaseCallback = (invoke: () => void): void => {
    try { invoke(); } catch (error) {
      if (isTransactionRepositoryAuthorityError(error)) throw error;
      assertTransactionLeaseRepositoryAuthority(authority, { kind: "acquire", attemptRelative, backupRelative, leaseDirectory, filename, previousName });
      throw error;
    }
    assertTransactionLeaseRepositoryAuthority(authority, { kind: "acquire", attemptRelative, backupRelative, leaseDirectory, filename, previousName });
  };
  const attemptRoot = absolutePath(repoRoot, attemptRelative);
  const backupRoot = absolutePath(repoRoot, backupRelative);
  const leaseRoot = join(attemptRoot, leaseDirectory);
  const backup = lstatOrNull(backupRoot);
  if (!backup?.isDirectory() || backup.isSymbolicLink()) throw new Error(`Transaction backup authority is unavailable for lease acquisition: ${backupRelative}`);
  const leasePreparations = (rejectLive = true): readonly { root: string; pid: number; token: string; state: "preparing" | "stale"; record?: TransactionLeaseRecord }[] => {
    const rows: { root: string; pid: number; token: string; state: "preparing" | "stale"; record?: TransactionLeaseRecord }[] = [];
    for (const name of readdirSync(backupRoot).sort(generatorPathCompare)) {
      const match = /^lease-([1-9][0-9]*)-([0-9a-f-]+)-(preparing|stale)$/u.exec(splitLeadingEmoji(name).rest);
      if (!match) continue;
      const root = join(backupRoot, name), pid = Number.parseInt(requireTaxonomyValue(match[1],"match[1]"), 10), token = requireTaxonomyValue(match[2],"match[2]"), state = match[3] as "preparing" | "stale";
      if (!Number.isSafeInteger(pid) || !TRANSACTION_LEASE_TOKEN.test(token) || name !== leasePreparationName(pid, token, state)) throw new Error(`Transaction lease preparation name is invalid: ${name}`);
      const stat = lstatOrNull(root);
      if (!stat?.isDirectory() || stat.isSymbolicLink()) throw new Error(`Transaction lease preparation must be a no-follow directory: ${name}`);
      recoverCanonicalJsonCandidates(root, filename, previousName, jsonWritePreparationName, (path) => { const candidate = parseTransactionLease(path, planDigest, attemptOrdinal, token); if (candidate.pid !== pid) throw new Error(`Transaction lease preparation pid is invalid: ${name}`); }, false, true);
      const canonical = join(root, filename);
      const record = lstatOrNull(canonical) ? parseTransactionLease(canonical, planDigest, attemptOrdinal, token) : undefined;
      if (record && record.pid !== pid) throw new Error(`Transaction lease preparation pid is invalid: ${name}`);
      if (rejectLive && transactionLeaseProcessIsAlive(pid)) throw new Error(`Transaction attempt lease preparation is active for pid ${pid}`);
      rows.push({ root, pid, token, state, record });
    }
    return rows;
  };
  leasePreparations();
  if (beforePublish != null) invokeLeaseCallback(() => beforePublish());
  if (probe != null) invokeLeaseCallback(() => probe("transaction-lease-scanned", attemptRelative));
  let quarantinedLease: string | undefined;
  const record: TransactionLeaseRecord = { schemaVersion: 1, planDigest, attemptOrdinal, token: randomUUID(), pid: process.pid };
  const preparing = join(backupRoot, leasePreparationName(record.pid, record.token, "preparing"));
  try {
    const current = lstatOrNull(leaseRoot);
    if (current) {
      const staleRecord = readTransactionLease(leaseRoot, filename, planDigest, attemptOrdinal);
      if (transactionLeaseProcessIsAlive(staleRecord.pid)) throw new Error(`Transaction attempt is leased by active pid ${staleRecord.pid}`);
      const stale = join(backupRoot, leasePreparationName(staleRecord.pid, staleRecord.token, "stale"));
      if (lstatOrNull(stale)) throw new Error(`Transaction attempt contains duplicate stale lease evidence: ${staleRecord.token}`);
      durableRename(leaseRoot, stale);
      quarantinedLease = stale;
      if (probe != null) invokeLeaseCallback(() => probe("transaction-lease-stale-quarantined", normalizeRelative(relative(repoRoot, stale).replaceAll("\\", "/"))));
    }
    if (lstatOrNull(preparing)) throw new Error(`Transaction lease token collision: ${record.token}`);
    mkdirSync(preparing);
    fsyncDirectory(backupRoot);
    if (probe != null) invokeLeaseCallback(() => probe("transaction-lease-preparation-mkdir", normalizeRelative(relative(repoRoot, preparing).replaceAll("\\", "/"))));
    if (beforePublish != null) invokeLeaseCallback(() => beforePublish(record));
    publishCanonicalJsonCandidate(preparing, filename, previousName, record, jsonWritePreparationName, undefined, probe, "transaction-lease-json");
    if (probe != null) invokeLeaseCallback(() => probe("transaction-lease-prepared", normalizeRelative(relative(repoRoot, preparing).replaceAll("\\", "/"))));
    if (lstatOrNull(leaseRoot)) throw new Error("Transaction lease acquisition fence found concurrent canonical lease evidence");
    readTransactionLease(preparing, filename, planDigest, attemptOrdinal, record.token);
    if (beforePublish != null) invokeLeaseCallback(() => beforePublish(record));
    durableRename(preparing, leaseRoot);
    if (probe != null) invokeLeaseCallback(() => probe("transaction-lease-canonical-published", normalizeRelative(relative(repoRoot, leaseRoot).replaceAll("\\", "/"))));
    if (beforePublish != null) invokeLeaseCallback(() => beforePublish(record));
    for (let pass = 0; pass < 100; pass++) {
      const preparations = leasePreparations(false);
      const live = preparations.filter((preparation) => transactionLeaseProcessIsAlive(preparation.pid));
      for (const preparation of preparations.filter((entry) => !live.includes(entry))) {
        recoverCanonicalJsonCandidates(preparation.root, filename, previousName, jsonWritePreparationName, (path) => { const candidate = parseTransactionLease(path, planDigest, attemptOrdinal, preparation.token); if (candidate.pid !== preparation.pid) throw new Error(`Transaction lease preparation pid is invalid: ${basename(preparation.root)}`); }, true);
        const canonical = join(preparation.root, filename);
        if (lstatOrNull(canonical)) {
          const stale = parseTransactionLease(canonical, planDigest, attemptOrdinal, preparation.token);
          if (stale.pid !== preparation.pid) throw new Error(`Transaction lease preparation pid is invalid: ${basename(preparation.root)}`);
        }
        durableRemove(preparation.root, true);
      }
      if (live.length === 0) break;
      if (pass === 99) throw new Error("Transaction lease acquisition timed out waiting for a live contender to retire its preparation");
      Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, 2);
    }
  } catch (error) {
    if (isTransactionRepositoryAuthorityError(error)) throw error;
    if (lstatOrNull(leaseRoot)) {
      const currentLease = readTransactionLease(leaseRoot, filename, planDigest, attemptOrdinal);
      if (canonicalJson(currentLease) === canonicalJson(record)) durableRemove(leaseRoot, true);
    }
    if (lstatOrNull(preparing)) {
      const stat = lstatOrNull(preparing);
      if (!stat?.isDirectory() || stat.isSymbolicLink()) throw new Error("Transaction lease preparation ownership changed during failed acquisition");
      recoverCanonicalJsonCandidates(preparing, filename, previousName, jsonWritePreparationName, (path) => {
        const owned = parseTransactionLease(path, planDigest, attemptOrdinal, record.token);
        if (owned.pid !== record.pid) throw new Error("Transaction lease preparation ownership changed during failed acquisition");
      }, true, true);
      const canonical = join(preparing, filename);
      if (lstatOrNull(canonical)) {
        const owned = parseTransactionLease(canonical, planDigest, attemptOrdinal, record.token);
        if (owned.pid !== record.pid) throw new Error("Transaction lease preparation ownership changed during failed acquisition");
      }
      durableRemove(preparing, true);
    }
    if (quarantinedLease && lstatOrNull(quarantinedLease)) {
      if (lstatOrNull(leaseRoot)) throw new Error("Transaction stale lease destination changed during failed acquisition");
      durableRename(quarantinedLease, leaseRoot);
    }
    throw new Error(`Transaction attempt lease acquisition failed: ${error instanceof Error ? error.message : String(error)}`);
  }
  return { root: leaseRoot, filename, record };
}
