import { repoCacheDirectory } from "../../🟦️.ts";
import { cargoDirectories } from "../../🦀️cargo/🟦️.ts";
import { CACHE_POLICY } from "../../🔍️discovery/📂️source/🟦️.ts";
import { scanCargoBuildUnits, scanCargoIncrementalUnits, scanCargoTargetUnits, scanDirectoryUnits, type CacheAreaInput, type CacheUnit, type CargoIncrementalScan } from "../🟦️.ts";

export interface CacheAreaScanOperations {
  cargoDirectories(repoRoot: string): { readonly build: string; readonly target: string };
  cacheDirectory(repoRoot: string, area: string): string;
  scanCargoBuild(path: string, signal: AbortSignal, onUnit?: (unit: CacheUnit) => void): CacheUnit[];
  scanCargoTarget(path: string, signal: AbortSignal, onUnit?: (unit: CacheUnit) => void): CacheUnit[];
  scanCargoIncremental(path: string, signal: AbortSignal, onUnit?: (unit: CacheUnit) => void): CargoIncrementalScan;
  scanDirectories(path: string, signal: AbortSignal, excluded?: ReadonlySet<string>, onUnit?: (unit: CacheUnit) => void): CacheUnit[];
}

/** 🧪️ Delegates test evidence retention to the test domain that owns pinning and active runs. */
export async function pruneTestEvidence(repoRoot: string, dry: boolean): Promise<string> {
  const { collectGarbage, loadOracleRegistry, formatGcReport } = await import("../../../../🧪️test/📦️packages/🟦️typescript/🟦️.ts");
  return formatGcReport(collectGarbage(repoRoot, loadOracleRegistry(repoRoot), { dry, olderThanMs: CACHE_POLICY.storage.tests.unusedAgeMs }));
}

/** ⚡️ Projects the shared cache root into independently bounded areas from one cancellation signal. */
export function scanCacheAreas(
  repoRoot: string,
  signal: AbortSignal,
  onUnit?: (area: string, unit: CacheUnit) => void,
  operations: CacheAreaScanOperations = {
    cargoDirectories,
    cacheDirectory: repoCacheDirectory,
    scanCargoBuild: scanCargoBuildUnits,
    scanCargoTarget: scanCargoTargetUnits,
    scanCargoIncremental: scanCargoIncrementalUnits,
    scanDirectories: scanDirectoryUnits,
  },
): CacheAreaInput[] {
  signal.throwIfAborted();
  const storage = CACHE_POLICY.storage;
  const dirs = operations.cargoDirectories(repoRoot);
  const progress = (area: string) => (unit: CacheUnit) => onUnit?.(area, unit);
  const buildUnits = operations.scanCargoBuild(dirs.build, signal, progress("cargo"));
  const targetUnits = dirs.target === dirs.build ? [] : operations.scanCargoTarget(dirs.target, signal, progress("cargo"));
  const incremental = operations.scanCargoIncremental(dirs.build, signal, (unit) => onUnit?.(unit.kind === "cargo-incremental-session" ? "cargo-incremental-sessions" : "cargo-incremental", unit));
  const viteUnits = operations.scanDirectories(operations.cacheDirectory(repoRoot, "vite"), signal, new Set(), progress("vite"));
  const agentUnits = operations.scanDirectories(operations.cacheDirectory(repoRoot, "agents"), signal, new Set(["resource-leases"]), progress("agents"));
  return [
    { name: "cargo", budgetBytes: storage.cargo.budgetBytes, unusedAgeMs: storage.cargo.unusedAgeMs, units: [...buildUnits, ...targetUnits] },
    { name: "cargo-incremental", budgetBytes: storage.cargo.incremental.budgetBytes, unusedAgeMs: storage.cargo.incremental.unusedAgeMs, guardAgeMs: storage.cargo.incremental.guardAgeMs, units: incremental.units },
    { name: "cargo-incremental-sessions", budgetBytes: null, unusedAgeMs: 0, units: incremental.staleSessions },
    { name: "vite", budgetBytes: null, unusedAgeMs: storage.vite.unusedAgeMs, units: viteUnits },
    { name: "agents", budgetBytes: null, unusedAgeMs: storage.agents.unusedAgeMs, units: agentUnits },
  ];
}

/** 🗺️ Resolves the real directory a scanned unit's relative path was measured against. */
export function areaUnitRoot(repoRoot: string, area: string, unit: CacheUnit, operations: Pick<CacheAreaScanOperations, "cargoDirectories" | "cacheDirectory"> = { cargoDirectories, cacheDirectory: repoCacheDirectory }): string {
  if (["cargo", "cargo-incremental", "cargo-incremental-sessions"].includes(area)) {
    const dirs = operations.cargoDirectories(repoRoot);
    return unit.kind === "cargo-target-file" ? dirs.target : dirs.build;
  }
  return operations.cacheDirectory(repoRoot, area);
}
