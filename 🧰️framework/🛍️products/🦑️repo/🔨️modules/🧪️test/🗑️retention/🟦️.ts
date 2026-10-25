/** 🗑️ Retention
 *
 * Cleaning of test outputs and garbage collection of unreferenced blobs.
 */

//#region 🔌️Adapters
import { type TestEvidence } from "@semio-tech/framework/test/adapter";
import { existsSync, lstatSync, readFileSync, readdirSync, realpathSync, rmSync, statSync } from "node:fs";
import { join, relative, resolve, sep } from "node:path";
import { repoCacheDirectory } from "../../📚️library/📦️packages/🟦️typescript/🏃️process/🟦️.ts";
import { repositoryTestArtifactPathV1, requireRepositoryTestArtifactRetirementV1 } from "../💾️storage/🟦️.ts";
import { testTaxonomy } from "../🗂️taxonomy/🟦️.ts";
import { readOutputMarker, testCacheRoot } from "../⚡️output-cache/🟦️.ts";
import { type OracleRegistry } from "../🤝️contributions/🟦️.ts";
import { agentId, leaseReclaimable, PROTECTED_RETENTION_CLASSES, readLease, type RetentionClass } from "../🔐️run-lease/🟦️.ts";
import { type RunManifest } from "../🗝️run-key/🟦️.ts";
import { fixtureBlobRoot, testEvidenceRoot } from "../🗄️blob-storage/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🧹️Clean
/** 🧹️ One removable generated test artifact, classified for the report. */
export type TestCleanRemoval = Readonly<{ category: string; path: string; files: number; bytes: number; reason: "all" | "stale" | "incomplete" | "oversized" }>;

/** 🧼️ What a clean run did (or, in dry mode, would do) — identical structure either way. */
export type TestCleanReport = Readonly<{ dry: boolean; removals: readonly TestCleanRemoval[]; protectedPaths: readonly string[]; skippedUnmarked: readonly string[]; retained: readonly { category: string; path: string; files: number; bytes: number }[] }>;

function countTree(abs: string): { files: number; bytes: number } {
  let files = 0;
  let bytes = 0;
  const walk = (dir: string): void => {
    for (const entry of readdirSync(dir, { withFileTypes: true })) {
      const full = join(dir, entry.name);
      if (entry.isSymbolicLink()) continue;
      if (entry.isDirectory()) walk(full);
      else {
        files += 1;
        try {
          bytes += statSync(full).size;
        } catch {}
      }
    }
  };
  try {
    walk(abs);
  } catch {}
  return { files, bytes };
}

/**
 * 🖨️ Removes generated test state and nothing else. Every candidate is resolved and proven to sit
 * beneath the canonical test-output root, must carry an ownership marker, and symlinks are never
 * followed — so no tracked fixture, no source file and no excluded path can be reached from here.
 */
export function cleanTestOutputs(repoRoot: string, opts: { dry?: boolean; stale?: boolean; over?: number; liveTestIds?: ReadonlySet<string> } = {}): TestCleanReport {
  if(!opts.dry)requireRepositoryTestArtifactRetirementV1(repoRoot);
  const dry = opts.dry ?? false;
  const taxonomy = testTaxonomy(repoRoot);
  const root = resolve(testCacheRoot(repoRoot));
  const removals: TestCleanRemoval[] = [];
  const retained: { category: string; path: string; files: number; bytes: number }[] = [];
  const skippedUnmarked: string[] = [];
  const protectedPaths = [relative(repoRoot, repoCacheDirectory(repoRoot)).split(sep).join("/")];
  if (!existsSync(root)) return { dry, removals, protectedPaths, skippedUnmarked, retained };

  const runs=readdirSync(root,{withFileTypes:true}).filter(entry=>entry.isDirectory()&&!entry.isSymbolicLink()&&/^[a-f0-9]{16}$/u.test(entry.name));
  for(const run of runs) for (const child of taxonomy.testOutputChildDirs) {
    const childRoot = repositoryTestArtifactPathV1(repoRoot,run.name,child);
    if (!existsSync(childRoot)) continue;
    for (const entry of readdirSync(childRoot, { withFileTypes: true })) {
      if (!entry.isDirectory() || entry.isSymbolicLink()) continue;
      const abs = join(childRoot, entry.name);
      let real = abs;
      try {
        real = realpathSync(abs);
      } catch {}
      const marker = readOutputMarker(repoRoot, abs);
      if (!resolve(real).startsWith(root + sep) || marker === null) {
        skippedUnmarked.push(relative(repoRoot, abs).split(sep).join("/"));
        continue;
      }
      const isStale = opts.liveTestIds !== undefined && !opts.liveTestIds.has(marker.testId);
      const isIncomplete = !existsSync(join(abs, "🏁️done"));
      const { files, bytes } = countTree(abs);
      const rel = relative(repoRoot, abs).split(sep).join("/");
      const isOversized = opts.over !== undefined && bytes > opts.over;
      const selective = opts.stale === true || opts.over !== undefined;
      if (selective && !isStale && !isIncomplete && !isOversized) {
        retained.push({ category: child, path: rel, files, bytes });
        continue;
      }
      removals.push({ category: child, path: rel, files, bytes, reason: isStale ? "stale" : isIncomplete ? "incomplete" : isOversized ? "oversized" : "all" });
      if (!dry) rmSync(abs, { recursive: true, force: true });
    }
  }
  return { dry, removals, protectedPaths, skippedUnmarked, retained };
}

/** 📊️ Renders a clean report; dry-run and applied output differ only in the action verb. */
export function formatCleanReport(repoRoot: string, report: TestCleanReport): string {
  const verb = report.dry ? "would-remove" : "removed";
  const totals = report.removals.reduce((acc, row) => ({ files: acc.files + row.files, bytes: acc.bytes + row.bytes }), { files: 0, bytes: 0 });
  const lines = [`[clean test] ${report.dry ? "dry-run" : "applied"} removals=${report.removals.length} files=${totals.files} bytes=${totals.bytes}`];
  for (const category of testTaxonomy(repoRoot).testOutputChildDirs) {
    const rows = report.removals.filter((row) => row.category === category);
    lines.push(`[clean test] ${category}: ${rows.length} (files=${rows.reduce((n, r) => n + r.files, 0)} bytes=${rows.reduce((n, r) => n + r.bytes, 0)})`);
  }
  for (const row of report.removals) lines.push(`[clean test] ${verb} ${row.category} ${row.path} (${row.reason}, files=${row.files}, bytes=${row.bytes})`);
  const kept = report.retained.reduce((acc, row) => ({ files: acc.files + row.files, bytes: acc.bytes + row.bytes }), { files: 0, bytes: 0 });
  if (report.retained.length > 0) lines.push(`[clean test] retained ${report.retained.length} (files=${kept.files} bytes=${kept.bytes})`);
  for (const path of report.skippedUnmarked) lines.push(`[clean test] skipped-unmarked ${path}`);
  for (const path of report.protectedPaths) lines.push(`[clean test] protected ${path}`);
  return lines.join("\n");
}
//#endregion 🧹️Clean

//#region 🧹️GarbageCollection
/** ⚖️ One candidate the sweep considered, with everything a reader needs to judge the decision. */
export type GcCandidate = Readonly<{ path: string; bytes: number; files: number; retention: RetentionClass; runId: string | null; agentId: string | null; lastAccessMs: number; eligible: boolean; reason: string; heldBy: readonly string[] }>;

/** 🚩️ The result of one mark-and-sweep pass. `dry` is the default everywhere it is offered. */
export type GcReport = Readonly<{ dry: boolean; candidates: readonly GcCandidate[]; removed: readonly string[]; markedBlobs: number; sweptBlobs: number; reclaimedBytes: number }>;

function directorySize(abs: string): { bytes: number; files: number } {
  let bytes = 0;
  let files = 0;
  const stack = [abs];
  while (stack.length > 0) {
    const dir = stack.pop()!;
    let entries: import("node:fs").Dirent[];
    try {
      entries = readdirSync(dir, { withFileTypes: true });
    } catch {
      continue;
    }
    for (const entry of entries) {
      const full = join(dir, entry.name);
      if (entry.isSymbolicLink()) continue;
      if (entry.isDirectory()) {
        stack.push(full);
        continue;
      }
      try {
        bytes += statSync(full).size;
        files += 1;
      } catch {}
    }
  }
  return { bytes, files };
}

/**
 * 🧫️ Marks every blob any committed fixture manifest, retained run manifest or pinned evidence
 * references. Sweeping is the complement, so an unreachable blob is one nothing can reach — never one
 * that merely looks old.
 */
export function markReferencedBlobs(repoRoot: string, registry: OracleRegistry): Set<string> {
  const marked = new Set<string>();
  for (const contribution of registry.contributions) for (const fixture of contribution.testEvidence) for (const file of fixture.files) marked.add(file.sha256);
  const manifestRoot = testEvidenceRoot(repoRoot);
  if (existsSync(manifestRoot)) {
    for (const name of readdirSync(manifestRoot)) {
      if (!name.endsWith(".json")) continue;
      try {
        const fixture = JSON.parse(readFileSync(join(manifestRoot, name), "utf8")) as TestEvidence;
        for (const file of fixture.files ?? []) marked.add(file.sha256);
      } catch (cause) { throw new Error(`Cannot determine fixture reachability from ${join(manifestRoot, name)}`, { cause }); }
    }
  }
  for (const resultsRoot of [join(testCacheRoot(repoRoot), "results"), join(testCacheRoot(repoRoot), "tasks")]) {
    if (!existsSync(resultsRoot)) continue;
    const stack = [resultsRoot];
    while (stack.length > 0) {
      const dir = stack.pop()!;
      let entries: import("node:fs").Dirent[];
      try {
        entries = readdirSync(dir, { withFileTypes: true });
      } catch {
        continue;
      }
      for (const entry of entries) {
        const full = join(dir, entry.name);
        if (entry.isSymbolicLink()) continue;
        if (entry.isDirectory()) {
          stack.push(full);
          continue;
        }
        if (entry.name !== "🗝️run.json") continue;
        try {
          const manifest = JSON.parse(readFileSync(full, "utf8")) as RunManifest;
          for (const artifact of manifest.artifacts ?? []) marked.add(artifact.sha256);
        } catch (cause) { throw new Error(`Cannot determine retained run reachability from ${full}`, { cause }); }
      }
    }
  }
  return marked;
}

/**
 * 🗃️ Mark-and-sweep over the fixture blob store plus a retention-aware scan of run directories.
 * Refuses to act at all unless the cache root looks like this repository's cache, and never follows a
 * symlink out of it — the two failure modes that turn a cleaner into a data-loss incident.
 */
export function collectGarbage(repoRoot: string, registry: OracleRegistry, opts: { dry?: boolean; olderThanMs?: number; overBytes?: number; agent?: string; retention?: readonly RetentionClass[]; nowMs?: number } = {}): GcReport {
  if(opts.dry===false)requireRepositoryTestArtifactRetirementV1(repoRoot);
  const dry = opts.dry !== false;
  const nowMs = opts.nowMs ?? Date.now();
  const cacheRoot = resolve(testCacheRoot(repoRoot));
  if (!existsSync(cacheRoot)) return { dry, candidates: [], removed: [], markedBlobs: 0, sweptBlobs: 0, reclaimedBytes: 0 };
  const resolvedCacheRoot = realpathSync(cacheRoot);
  const resolvedMetaRoot = realpathSync(testCacheRoot(repoRoot));
  if (resolvedCacheRoot !== resolvedMetaRoot && !resolvedCacheRoot.startsWith(resolvedMetaRoot + sep)) {
    throw new Error(`refusing to collect: ${cacheRoot} resolves to ${resolvedCacheRoot}, outside the repository cache at ${resolvedMetaRoot}`);
  }

  const candidates: GcCandidate[] = [];
  const removed: string[] = [];
  let reclaimedBytes = 0;

  for (const child of ["work", "hosts", "oracles", "results", "diffs", "tasks"]) {
    const root = join(cacheRoot, child);
    if (!existsSync(root)) continue;
    const stack: string[] = [root];
    while (stack.length > 0) {
      const dir = stack.pop()!;
      let entries: import("node:fs").Dirent[];
      try {
        entries = readdirSync(dir, { withFileTypes: true });
      } catch {
        continue;
      }
      for (const entry of entries) {
        if (!entry.isDirectory() || entry.isSymbolicLink()) continue;
        const abs = join(dir, entry.name);
        const lease = readLease(abs);
        if (lease === null) {
          stack.push(abs);
          continue;
        }
        const { bytes, files } = directorySize(abs);
        const held: string[] = [];
        if (lease.state === "active" || lease.state === "creating") held.push(`lease ${lease.state} (agent ${lease.agentId})`);
        if (PROTECTED_RETENTION_CLASSES.includes(lease.retention)) held.push(`retention ${lease.retention}`);
        if (lease.agentId === agentId() && lease.state !== "complete") held.push("owned by this agent");
        const age = nowMs - Date.parse(lease.heartbeatAt);
        const oldEnough = opts.olderThanMs === undefined || (Number.isFinite(age) && age > opts.olderThanMs);
        const bigEnough = opts.overBytes === undefined || bytes > opts.overBytes;
        const agentMatches = opts.agent === undefined || lease.agentId === opts.agent;
        const retentionMatches = opts.retention === undefined || opts.retention.includes(lease.retention);
        const reclaimable = leaseReclaimable(lease, nowMs);
        const eligible = held.length === 0 && oldEnough && bigEnough && agentMatches && retentionMatches && (lease.state === "complete" || reclaimable);
        const reason = held.length > 0 ? `held: ${held.join("; ")}` : !oldEnough ? "newer than --older-than" : !bigEnough ? "smaller than --over-size" : !agentMatches ? "another agent's run" : !retentionMatches ? "retention class not selected" : eligible ? `${lease.state} run, reclaimable` : "lease is not reclaimable";
        candidates.push({ path: relative(repoRoot, abs).split(sep).join("/"), bytes, files, retention: lease.retention, runId: lease.runId, agentId: lease.agentId, lastAccessMs: age, eligible, reason, heldBy: held });
        if (eligible && !dry) {
          rmSync(abs, { recursive: true, force: true });
          removed.push(relative(repoRoot, abs).split(sep).join("/"));
          reclaimedBytes += bytes;
        }
      }
    }
  }

  const marked = markReferencedBlobs(repoRoot, registry);
  const activeRun = candidates.some((candidate) => candidate.heldBy.some((reason) => reason.startsWith("lease active") || reason.startsWith("lease creating")));
  let sweptBlobs = 0;
  const blobRoot = fixtureBlobRoot(repoRoot);
  if (existsSync(blobRoot)) {
    for (const shard of readdirSync(blobRoot)) {
      const shardDir = join(blobRoot, shard);
      if (!existsSync(shardDir) || !lstatSync(shardDir).isDirectory()) continue;
      for (const name of readdirSync(shardDir)) {
        if (name.endsWith(".tmp")) continue;
        const sha256 = `sha256:${name}`;
        if (marked.has(sha256)) continue;
        const abs = join(shardDir, name);
        let bytes = 0;
        try {
          bytes = statSync(abs).size;
        } catch {
          continue;
        }
        candidates.push({ path: relative(repoRoot, abs).split(sep).join("/"), bytes, files: 1, retention: "ephemeral-success", runId: null, agentId: null, lastAccessMs: 0, eligible: !activeRun, reason: activeRun ? "active run may still be publishing blob references" : "blob is referenced by no fixture manifest, retained run manifest or pinned evidence", heldBy: activeRun ? ["active publication"] : [] });
        if (!dry && !activeRun) {
          rmSync(abs, { force: true });
          sweptBlobs += 1;
          reclaimedBytes += bytes;
        }
      }
    }
  }
  return { dry, candidates, removed, markedBlobs: marked.size, sweptBlobs, reclaimedBytes };
}

/** 🎟️ Renders a GC pass so every decision is legible: what, how big, whose, and why it was or was not eligible. */
export function formatGcReport(report: GcReport): string {
  const lines = [`[gc] ${report.dry ? "DRY RUN — nothing removed" : "SWEEPING"} — ${report.candidates.length} candidate(s), ${report.markedBlobs} blob(s) marked reachable`];
  for (const candidate of [...report.candidates].sort((a, b) => b.bytes - a.bytes)) {
    lines.push(`[gc] ${candidate.eligible ? "eligible" : "held    "} ${(candidate.bytes / 1024 / 1024).toFixed(2)} MiB ${candidate.retention.padEnd(19)} ${candidate.agentId ?? "-"} ${candidate.path} — ${candidate.reason}`);
  }
  lines.push(`[gc] ${report.removed.length} run director(ies) removed, ${report.sweptBlobs} blob(s) swept, ${(report.reclaimedBytes / 1024 / 1024).toFixed(2)} MiB reclaimed`);
  return lines.join("\n");
}
//#endregion 🧹️GarbageCollection
