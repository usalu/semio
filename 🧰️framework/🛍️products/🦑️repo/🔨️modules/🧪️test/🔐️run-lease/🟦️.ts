/** 🔐️ Run leases
 *
 * Retention classes and the lease files that protect a run directory.
 */

//#region 🔌️Adapters
import { existsSync, mkdirSync, readFileSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { currentAgentSessionV1 } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🪪️agent-sessions/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🔐️Lease
/** 🗄️ What a cached path IS, which decides when it may be removed. */
export const RETENTION_CLASSES = ["ephemeral-success", "last-success-proof", "failure-evidence", "fixture-generation", "toolchain", "downloaded-source", "pinned"] as const;
export type RetentionClass = (typeof RETENTION_CLASSES)[number];

/** 🗄️ Retention classes routine stale cleanup may NEVER remove. Failure evidence outlives the run that produced it. */
export const PROTECTED_RETENTION_CLASSES: readonly RetentionClass[] = ["failure-evidence", "last-success-proof", "pinned", "toolchain"];

export const LEASE_STATES = ["creating", "active", "complete", "failed", "abandoned"] as const;
export type LeaseState = (typeof LEASE_STATES)[number];

/** 🔐️ One agent's claim on one run directory. */
export type RunLease = Readonly<{
  schema: "semio.repository-test.lease/v2";
  runId: string;
  agentId: string;
  pid?: number;
  state: LeaseState;
  createdAt: string;
  heartbeatAt: string;
  retention: RetentionClass;
  runKey?: string;
  testId?: string;
}>;

/** 🏷️ This process's agent identity: the agent of its agent session. Never inferred from a username or a hostname. */
export function agentId(): string {
  return currentAgentSessionV1().agent.id.replace(/[^A-Za-z0-9._-]+/g, "_");
}

/** 🔐️ How long an `active` lease may go without a heartbeat before it is even a CANDIDATE for reclaim. */
export const LEASE_STALE_MS = 45 * 60 * 1000;

const LEASE_FILENAME = "🔐️lease.json";

/** 🔐️ Path of one run directory's lease file. */
export function leasePath(absRunDir: string): string {
  return join(absRunDir, LEASE_FILENAME);
}

/** 🔐️ Writes or refreshes a lease. A run directory without one is never treated as reclaimable state. */
export function writeLease(absRunDir: string, lease: Omit<RunLease, "schema">): RunLease {
  const record: RunLease = { schema: "semio.repository-test.lease/v2", ...lease };
  mkdirSync(absRunDir, { recursive: true });
  writeFileSync(leasePath(absRunDir), `${JSON.stringify(record, null, 2)}\n`);
  return record;
}

/** 🔐️ Reads a lease, or `null` when the directory carries none. */
export function readLease(absRunDir: string): RunLease | null {
  const path = leasePath(absRunDir);
  if (!existsSync(path)) return null;
  try {
    const parsed = JSON.parse(readFileSync(path, "utf8")) as RunLease;
    return parsed.schema === "semio.repository-test.lease/v2" ? parsed : null;
  } catch {
    return null;
  }
}

/** 🔐️ Refreshes a lease's heartbeat in place, so a long run is never mistaken for an abandoned one. */
export function heartbeatLease(absRunDir: string, nowIso = new Date().toISOString()): RunLease | null {
  const lease = readLease(absRunDir);
  if (lease === null) return null;
  return writeLease(absRunDir, { ...lease, heartbeatAt: nowIso });
}

/** 🔐️ Whether a process is still alive. A stale heartbeat alone is never enough to reclaim a lease. */
export function processAlive(pid: number | undefined): boolean {
  if (pid === undefined || !Number.isInteger(pid) || pid <= 0) return false;
  try {
    process.kill(pid, 0);
    return true;
  } catch (error) {
    return (error as NodeJS.ErrnoException).code === "EPERM";
  }
}

/**
 * 🔐️ Whether a lease may be reclaimed. An `active` lease is reclaimable only when BOTH its heartbeat
 * is older than the documented timeout AND its process is gone — a slow run and a dead one look
 * identical from the timestamp alone, and deleting the slow one destroys a peer agent's work.
 */
export function leaseReclaimable(lease: RunLease | null, nowMs = Date.now(), staleMs = LEASE_STALE_MS): boolean {
  if (lease === null) return false;
  if (lease.state === "complete" || lease.state === "abandoned") return true;
  if (lease.state === "failed") return false;
  const heartbeat = Date.parse(lease.heartbeatAt);
  const stale = Number.isFinite(heartbeat) && nowMs - heartbeat > staleMs;
  return stale && !processAlive(lease.pid) && lease.agentId !== agentId();
}

/**
 * 🔐️ Runs `body` inside a private temporary directory and publishes it with ONE atomic rename. A
 * reader therefore observes either the previous complete directory or the new complete directory, and
 * never a half-written one. An interrupted generation leaves only the temporary, which GC sweeps.
 */
export function withAtomicRunDir<T>(absFinalDir: string, retention: RetentionClass, body: (tempDir: string) => T): { result: T; lease: RunLease } {
  const runId = `${Date.now().toString(36)}-${agentId()}-${process.pid.toString(36)}`;
  const tempDir = `${absFinalDir}.${runId}.creating`;
  mkdirSync(dirname(absFinalDir), { recursive: true });
  mkdirSync(tempDir, { recursive: true });
  const now = new Date().toISOString();
  let lease = writeLease(tempDir, { runId, agentId: agentId(), pid: process.pid, state: "creating", createdAt: now, heartbeatAt: now, retention });
  try {
    const result = body(tempDir);
    lease = writeLease(tempDir, { ...lease, state: "complete", heartbeatAt: new Date().toISOString() });
    rmSync(absFinalDir, { recursive: true, force: true });
    renameSync(tempDir, absFinalDir);
    return { result, lease };
  } catch (error) {
    writeLease(tempDir, { ...lease, state: "failed", retention: "failure-evidence", heartbeatAt: new Date().toISOString() });
    throw error;
  }
}
//#endregion 🔐️Lease
