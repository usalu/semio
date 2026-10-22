import { repoCacheDirectory } from "../../🟦️.ts";
import { cargoDirectories } from "../../🦀️cargo/🗂️directories/🟦️.ts";
import { CACHE_POLICY } from "../../🔍️discovery/📂️source/🟦️.ts";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import { scanCargoBuildUnits, scanCargoIncrementalUnits, scanCargoTargetUnits, scanDirectoryUnits, type CacheAreaInput, type CacheUnit, type CargoIncrementalScan } from "../🟦️.ts";

export interface CacheAreaScanOperations {
  cargoDirectories(repoRoot: string): { readonly build: string; readonly target: string };
  cacheDirectory(repoRoot: string, area: string): string;
  scanCargoBuild(path: string, control: ResourceLeaseControl, onUnit?: (unit: CacheUnit) => void): Promise<CacheUnit[]>;
  scanCargoTarget(path: string, control: ResourceLeaseControl, onUnit?: (unit: CacheUnit) => void): Promise<CacheUnit[]>;
  scanCargoIncremental(path: string, control: ResourceLeaseControl, onUnit?: (unit: CacheUnit) => void): Promise<CargoIncrementalScan>;
  scanDirectories(path: string, control: ResourceLeaseControl, excluded?: ReadonlySet<string>, onUnit?: (unit: CacheUnit) => void): Promise<CacheUnit[]>;
}

/** 🧪️ Delegates test evidence retention to the test domain that owns pinning and active runs. */
export async function pruneTestEvidence(repoRoot: string, dry: boolean): Promise<string> {
  const [{ collectGarbage, formatGcReport }, { loadOracleRegistry }] = await Promise.all([
    import("../../../../🧪️test/🗑️retention/🟦️.ts"),
    import("../../../../🧪️test/🤝️contributions/🟦️.ts"),
  ]);
  return formatGcReport(collectGarbage(repoRoot, loadOracleRegistry(repoRoot), { dry, olderThanMs: CACHE_POLICY.storage.tests.unusedAgeMs }));
}

/** ⚡️ Projects the shared cache root into independently bounded areas from one cancellation signal. */
export async function scanCacheAreas(
  repoRoot: string,
  control: ResourceLeaseControl,
  onUnit?: (area: string, unit: CacheUnit) => void,
  operations: CacheAreaScanOperations = {
    cargoDirectories,
    cacheDirectory: repoCacheDirectory,
    scanCargoBuild: scanCargoBuildUnits,
    scanCargoTarget: scanCargoTargetUnits,
    scanCargoIncremental: scanCargoIncrementalUnits,
    scanDirectories: scanDirectoryUnits,
  },
): Promise<CacheAreaInput[]> {
  checkResourceLeaseControl(control);
  const storage = CACHE_POLICY.storage;
  const dirs = operations.cargoDirectories(repoRoot);
  const progress = (area: string) => (unit: CacheUnit) => onUnit?.(area, unit);
  const buildUnits = await operations.scanCargoBuild(dirs.build, control, progress("cargo"));
  const targetUnits = dirs.target === dirs.build ? [] : await operations.scanCargoTarget(dirs.target, control, progress("cargo"));
  const incremental = await operations.scanCargoIncremental(dirs.build, control, (unit) => onUnit?.(unit.kind === "cargo-incremental-session" ? "cargo-incremental-sessions" : "cargo-incremental", unit));
  const viteUnits = await operations.scanDirectories(operations.cacheDirectory(repoRoot, "vite"), control, new Set(), progress("vite"));
  const agentUnits = await operations.scanDirectories(operations.cacheDirectory(repoRoot, "agents"), control, new Set(["resource-leases"]), progress("agents"));
  checkResourceLeaseControl(control);
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
