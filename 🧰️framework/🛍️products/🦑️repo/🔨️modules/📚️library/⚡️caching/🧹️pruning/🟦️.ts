import { existsSync, lstatSync, readdirSync, rmdirSync, rmSync, unlinkSync } from "node:fs";
import { dirname, join, relative, sep } from "node:path";

export type CacheUnitKind = "cargo-build" | "cargo-incremental" | "cargo-target-file" | "directory";
/** 📦️ One independently deletable slice of a cache area: a compilation unit, an incremental crate dir, an uplifted file, or a whole scratch directory. */
export interface CacheUnit {
  readonly path: string;
  readonly bytes: number;
  readonly recencyMs: number;
  readonly lockHeld: boolean;
  readonly kind: CacheUnitKind;
}
export interface CacheAreaInput {
  readonly name: string;
  readonly budgetBytes: number | null;
  readonly unusedAgeMs: number;
  readonly units: readonly CacheUnit[];
}
export interface AreaPlan {
  readonly name: string;
  readonly totalBytes: number;
  readonly unitCount: number;
  readonly ageDeletions: readonly CacheUnit[];
  readonly budgetDeletions: readonly CacheUnit[];
  readonly retainedBytes: number;
  readonly guardedOverBudgetBytes: number;
}
export interface PrunePlan {
  readonly generatedAtMs: number;
  readonly areas: readonly AreaPlan[];
}

/**
 * 📐️ Pure age-then-budget eviction over an abstract cache tree; no filesystem access, so it is identically
 * testable in every language. Age deletions run first; if an area still exceeds its budget, the remaining
 * units are evicted oldest-first, but never past the guard age (`recencyMs` within `guardAgeMs` of `nowMs`)
 * and never while `lockHeld` — those bytes are reported as `guardedOverBudgetBytes` instead of deleted.
 */
export function planCachePrune(areas: readonly CacheAreaInput[], nowMs: number, guardAgeMs: number): PrunePlan {
  const areaPlans = areas.map((area): AreaPlan => {
    const totalBytes = area.units.reduce((sum, unit) => sum + unit.bytes, 0);
    const ageCutoff = nowMs - area.unusedAgeMs;
    const ageDeletions = area.units.filter((unit) => !unit.lockHeld && unit.recencyMs <= ageCutoff);
    const ageDeletedPaths = new Set(ageDeletions.map((unit) => unit.path));
    const remaining = area.units.filter((unit) => !ageDeletedPaths.has(unit.path));
    let remainingBytes = remaining.reduce((sum, unit) => sum + unit.bytes, 0);
    const budgetDeletions: CacheUnit[] = [];
    let guardedOverBudgetBytes = 0;
    if (area.budgetBytes !== null && remainingBytes > area.budgetBytes) {
      const guardCutoff = nowMs - guardAgeMs;
      for (const unit of [...remaining].sort((left, right) => left.recencyMs - right.recencyMs || (left.path < right.path ? -1 : left.path > right.path ? 1 : 0))) {
        if (remainingBytes <= area.budgetBytes) break;
        if (unit.lockHeld || unit.recencyMs > guardCutoff) { guardedOverBudgetBytes += unit.bytes; continue; }
        budgetDeletions.push(unit);
        remainingBytes -= unit.bytes;
      }
    }
    const deletedBytes = ageDeletions.reduce((sum, unit) => sum + unit.bytes, 0) + budgetDeletions.reduce((sum, unit) => sum + unit.bytes, 0);
    return { name: area.name, totalBytes, unitCount: area.units.length, ageDeletions, budgetDeletions, retainedBytes: totalBytes - deletedBytes, guardedOverBudgetBytes };
  });
  return { generatedAtMs: nowMs, areas: areaPlans };
}

/** 🔢️ Human-readable binary byte count for report text output. */
export function formatBytes(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  const units = ["KiB", "MiB", "GiB", "TiB"];
  let value = bytes, index = -1;
  do { value /= 1024; index++; } while (value >= 1024 && index < units.length - 1);
  return `${value.toFixed(2)} ${units[index]}`;
}

const safeReaddir = (path: string): string[] => { try { return readdirSync(path); } catch { return []; } };

/**
 * 📏️ Recursively measures one deletable unit's total bytes and most-recent access/modification time.
 * Recency comes only from file timestamps, never directory timestamps — a directory's own mtime changes
 * on unrelated churn (a sibling created or renamed) and would otherwise mask a genuinely stale unit.
 */
function measureUnit(root: string): { bytes: number; recencyMs: number } {
  let bytes = 0, recencyMs = 0;
  const walk = (path: string): void => {
    let stat;
    try { stat = lstatSync(path); } catch { return; }
    if (stat.isSymbolicLink()) return;
    if (stat.isDirectory()) { for (const child of safeReaddir(path)) walk(join(path, child)); return; }
    if (!stat.isFile()) return;
    recencyMs = Math.max(recencyMs, stat.atimeMs, stat.mtimeMs);
    bytes += stat.size;
  };
  walk(root);
  return { bytes, recencyMs };
}

const posix = (path: string): string => path.split(sep).join("/");

/**
 * 🗃️ Walks Cargo's shared build-dir for its two evidenced unit shapes: `.../build/<package>/<unit-hash>/`
 * and `.../incremental/<crate>-<hash>/`. https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#build-dir
 */
export function scanCargoBuildUnits(buildDir: string, signal: AbortSignal, onUnit?: (unit: CacheUnit) => void): CacheUnit[] {
  const units: CacheUnit[] = [];
  const descend = (dir: string, depth: number): void => {
    signal.throwIfAborted();
    if (depth > 8 || !existsSync(dir)) return;
    for (const name of safeReaddir(dir)) {
      const path = join(dir, name);
      let stat;
      try { stat = lstatSync(path); } catch { continue; }
      if (!stat.isDirectory() || stat.isSymbolicLink()) continue;
      if (name === "build") {
        for (const pkg of safeReaddir(path)) for (const hash of safeReaddir(join(path, pkg))) {
          signal.throwIfAborted();
          const unitPath = join(path, pkg, hash);
          let unitStat;
          try { unitStat = lstatSync(unitPath); } catch { continue; }
          if (!unitStat.isDirectory() || unitStat.isSymbolicLink()) continue;
          const measured = measureUnit(unitPath);
          const unit: CacheUnit = { path: posix(relative(buildDir, unitPath)), bytes: measured.bytes, recencyMs: measured.recencyMs, lockHeld: false, kind: "cargo-build" };
          units.push(unit);
          onUnit?.(unit);
        }
        continue;
      }
      if (name === "incremental") {
        for (const crate of safeReaddir(path)) {
          signal.throwIfAborted();
          const unitPath = join(path, crate);
          let unitStat;
          try { unitStat = lstatSync(unitPath); } catch { continue; }
          if (!unitStat.isDirectory() || unitStat.isSymbolicLink()) continue;
          const measured = measureUnit(unitPath);
          const unit: CacheUnit = { path: posix(relative(buildDir, unitPath)), bytes: measured.bytes, recencyMs: measured.recencyMs, lockHeld: false, kind: "cargo-incremental" };
          units.push(unit);
          onUnit?.(unit);
        }
        continue;
      }
      descend(path, depth + 1);
    }
  };
  descend(buildDir, 0);
  return units;
}

const CARGO_SENTINEL_NAMES = new Set(["CACHEDIR.TAG"]);
const isCargoSentinel = (name: string): boolean => CARGO_SENTINEL_NAMES.has(name) || name.startsWith(".cargo-");

/** 🗃️ Walks Cargo's uplifted target-dir; under the new build-dir layout it holds only final deliverable files, so each file is its own unit. */
export function scanCargoTargetUnits(targetDir: string, signal: AbortSignal, onUnit?: (unit: CacheUnit) => void): CacheUnit[] {
  const units: CacheUnit[] = [];
  const descend = (dir: string): void => {
    signal.throwIfAborted();
    if (!existsSync(dir)) return;
    for (const name of safeReaddir(dir)) {
      const path = join(dir, name);
      let stat;
      try { stat = lstatSync(path); } catch { continue; }
      if (stat.isSymbolicLink()) continue;
      if (stat.isDirectory()) { descend(path); continue; }
      if (!stat.isFile() || isCargoSentinel(name)) continue;
      const unit: CacheUnit = { path: posix(relative(targetDir, path)), bytes: stat.size, recencyMs: Math.max(stat.atimeMs, stat.mtimeMs), lockHeld: false, kind: "cargo-target-file" };
      units.push(unit);
      onUnit?.(unit);
    }
  };
  descend(targetDir);
  return units;
}

/** 📂️ Walks one cache-root area whose units are its immediate child directories (Vite consumer caches, stray agent scratch dirs). */
export function scanDirectoryUnits(areaRoot: string, signal: AbortSignal, exclude: ReadonlySet<string> = new Set(), onUnit?: (unit: CacheUnit) => void): CacheUnit[] {
  if (!existsSync(areaRoot)) return [];
  const units: CacheUnit[] = [];
  for (const name of safeReaddir(areaRoot)) {
    signal.throwIfAborted();
    if (exclude.has(name)) continue;
    const path = join(areaRoot, name);
    let stat;
    try { stat = lstatSync(path); } catch { continue; }
    if (!stat.isDirectory() || stat.isSymbolicLink()) continue;
    const measured = measureUnit(path);
    const unit: CacheUnit = { path: name, bytes: measured.bytes, recencyMs: measured.recencyMs, lockHeld: false, kind: "directory" };
    units.push(unit);
    onUnit?.(unit);
  }
  return units;
}

/** 🗑️ Deletes one previously scanned unit and, for a flat target-dir file, prunes now-empty parent directories back to the area root. */
export function deleteUnit(areaRoot: string, unit: CacheUnit): void {
  const absolute = join(areaRoot, ...unit.path.split("/"));
  if (unit.kind === "cargo-target-file") {
    try { unlinkSync(absolute); } catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error; }
    let parent = dirname(absolute);
    for (;;) {
      if (parent === areaRoot || relative(areaRoot, parent).startsWith("..")) break;
      if (safeReaddir(parent).length > 0) break;
      try { rmdirSync(parent); } catch { break; }
      parent = dirname(parent);
    }
    return;
  }
  rmSync(absolute, { recursive: true, force: true });
}
