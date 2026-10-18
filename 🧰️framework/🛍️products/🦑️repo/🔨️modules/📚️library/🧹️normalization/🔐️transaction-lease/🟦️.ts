import { lstatOrNull } from "../📁️input/🟦️.ts";
import { sourceAdmissionAssertLexical } from "../🚪️source-admission/📁️io/🟦️.ts";
import { readFileSync, readdirSync } from "node:fs";
import { canonicalJson } from "../../🧾️serialization/🔣️json/🟦️.ts";
import { isAbsolute, join, posix, relative, resolve, sep } from "node:path";
import { absolutePath } from "../🏷️file-kind-matching/🟦️.ts";
import { generatorPathCompare, planRecord } from "../🧾️plan-parsing/🟦️.ts";
import { durableRemove } from "../💾️durable-journal/🟦️.ts";
import { TransactionRepositoryAuthority, TransactionRepositoryAuthorityError, assertTransactionRepositoryPath, assertTransactionRepositoryWitness, captureTransactionRepositoryAuthority, isTransactionRepositoryAuthorityError } from "../🏛️repository-authority/🟦️.ts";

export interface TransactionLeaseRecord {
  readonly schemaVersion: 1;
  readonly planDigest: string;
  readonly attemptOrdinal: string;
  readonly token: string;
  readonly pid: number;
}

export interface TransactionLeaseHandle {
  readonly root: string;
  readonly filename: string;
  readonly record: TransactionLeaseRecord;
}

export const TRANSACTION_LEASE_TOKEN = /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/u;

function transactionLeaseSnapshot(record: TransactionLeaseRecord): TransactionLeaseRecord {
  return { schemaVersion: 1, planDigest: record.planDigest, attemptOrdinal: record.attemptOrdinal, token: record.token, pid: record.pid };
}

export function parseTransactionLease(path: string, planDigest: string, attemptOrdinal: string, expectedToken?: string): TransactionLeaseRecord {
  const file = lstatOrNull(path);
  if (!file?.isFile() || file.isSymbolicLink()) throw new Error(`Transaction lease metadata must be a regular no-follow file: ${path}`);
  const bytes = readFileSync(path, "utf8");
  const value = planRecord(JSON.parse(bytes), "transaction lease", ["schemaVersion", "planDigest", "attemptOrdinal", "token", "pid"]);
  if (value.schemaVersion !== 1 || value.planDigest !== planDigest || value.attemptOrdinal !== attemptOrdinal || typeof value.token !== "string" || !TRANSACTION_LEASE_TOKEN.test(value.token) || expectedToken !== undefined && value.token !== expectedToken || !Number.isSafeInteger(value.pid) || (value.pid as number) < 1) throw new Error(`Transaction lease identity is invalid: ${path}`);
  const record = transactionLeaseSnapshot(value as unknown as TransactionLeaseRecord);
  if (bytes !== `${canonicalJson(record)}\n`) throw new Error(`Transaction lease is not canonical JSON: ${path}`);
  return record;
}

export function readTransactionLease(root: string, filename: string, planDigest: string, attemptOrdinal: string, expectedToken?: string): TransactionLeaseRecord {
  const stat = lstatOrNull(root);
  if (!stat?.isDirectory() || stat.isSymbolicLink()) throw new Error(`Transaction lease must be a direct no-follow directory: ${root}`);
  if (canonicalJson(readdirSync(root).sort(generatorPathCompare)) !== canonicalJson([filename])) throw new Error(`Transaction lease contains unexpected evidence: ${root}`);
  return parseTransactionLease(join(root, filename), planDigest, attemptOrdinal, expectedToken);
}

export function transactionLeaseProcessIsAlive(pid: number): boolean {
  try {
    process.kill(pid, 0);
    return true;
  } catch (error) {
    const code = String((error as NodeJS.ErrnoException).code);
    if (code === "EPERM") return true;
    if (code === "ESRCH") return false;
    throw error;
  }
}

export function assertTransactionLeaseRepositoryAuthority(authority: TransactionRepositoryAuthority, request: { kind: "acquire"; attemptRelative: string; backupRelative: string; leaseDirectory: string; filename: string; previousName: string } | { kind: "release"; expectedLogicalLeasePath: string; handle: TransactionLeaseHandle }): void {
  if (!TransactionRepositoryAuthority.owns(authority)) throw new TransactionRepositoryAuthorityError("missing-authority", new Error("Missing captured transaction repository authority"));
  let attemptRelative: string, leaseRelative: string, physicalRelative: string, backupRelative: string | undefined;
  try {
    const component = (value: string, label: string): void => { sourceAdmissionAssertLexical(value, label, false); if (value.includes("/")) throw new Error(label + " is not a single component"); };
    if (request.kind === "acquire") {
      attemptRelative = request.attemptRelative; backupRelative = request.backupRelative;
      sourceAdmissionAssertLexical(attemptRelative, "Transaction lease attempt", false);
      sourceAdmissionAssertLexical(backupRelative, "Transaction lease backup", false);
      component(request.leaseDirectory, "Transaction lease directory"); component(request.filename, "Transaction lease filename"); component(request.previousName, "Transaction lease previous filename");
      leaseRelative = attemptRelative + "/" + request.leaseDirectory; physicalRelative = leaseRelative;
    } else if (request.kind === "release") {
      leaseRelative = request.expectedLogicalLeasePath;
      sourceAdmissionAssertLexical(leaseRelative, "Transaction lease logical root", false);
      attemptRelative = posix.dirname(leaseRelative);
      sourceAdmissionAssertLexical(attemptRelative, "Transaction lease attempt", false);
      component(request.handle.filename, "Transaction lease filename");
      const physicalRoot = request.handle.root;
      sourceAdmissionAssertLexical(physicalRoot, "Transaction lease physical root", true);
      if (!isAbsolute(physicalRoot) || resolve(physicalRoot) !== physicalRoot || physicalRoot !== absolutePath(authority.repoRoot, leaseRelative)) throw new Error("Transaction lease physical root does not match its logical authority");
      physicalRelative = relative(authority.repoRoot, physicalRoot).split(sep).join("/");
      sourceAdmissionAssertLexical(physicalRelative, "Transaction lease physical coordinate", false);
    } else throw new Error("Transaction lease access role is invalid");
  } catch (cause) {
    if (isTransactionRepositoryAuthorityError(cause)) throw cause;
    throw new TransactionRepositoryAuthorityError("invalid-access", cause);
  }
  assertTransactionRepositoryPath(authority, attemptRelative, "point", "Transaction lease attempt metadata");
  assertTransactionRepositoryPath(authority, leaseRelative, "subtree", "Transaction lease logical root");
  assertTransactionRepositoryPath(authority, physicalRelative, "subtree", "Transaction lease physical root");
  if (backupRelative !== undefined) assertTransactionRepositoryPath(authority, backupRelative, "subtree", "Transaction lease backup root");
  assertTransactionRepositoryWitness(authority, captureTransactionRepositoryAuthority(authority.repoRoot).indexRows);
}

export function releaseTransactionLease(authority: TransactionRepositoryAuthority, expectedLogicalLeasePath: string, handle: TransactionLeaseHandle): void {
  assertTransactionLeaseRepositoryAuthority(authority, { kind: "release", expectedLogicalLeasePath, handle });
  const current = readTransactionLease(handle.root, handle.filename, handle.record.planDigest, handle.record.attemptOrdinal, handle.record.token);
  if (current.pid !== handle.record.pid) throw new Error("Transaction lease ownership changed before release");
  durableRemove(handle.root, true);
}
