import { listToolSpawnFilesV1, pruneToolSpawnLedgerV1, scanRepositoryToolSpawnsV1, summarizeToolSpawnsV1, toolRegistryProblemsV1, toolSpawnFindingsV1, TOOL_SPAWN_LEDGER_V1, writeToolSpawnLedgerV1, type ToolSpawnLedgerV1 } from "../🟦️.ts";
import {writeResultLineV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/📤️output/🟦️.ts";

/** 🚪️ Runs `repo verify tool-spawns [--report] [--json] [--strict] [--prune [--accept-new]]`: scans the repository for tools started without the registry, compares the test spawns with the test ledger and answers the exit code. Production code has no ledger. `--prune` shrinks the test ledger to the truth; `--accept-new` also admits rows it does not have yet. */
export function runToolSpawnLawV1(root: string, args: readonly string[], write: (line: string) => void = writeResultLineV1, ledger: ToolSpawnLedgerV1 = TOOL_SPAWN_LEDGER_V1): number {
  const files = listToolSpawnFilesV1(root), scan = scanRepositoryToolSpawnsV1(root, files), actual = summarizeToolSpawnsV1(scan.sites);
  if (args.includes("--prune")) {
    const next = pruneToolSpawnLedgerV1(ledger, actual, args.includes("--accept-new"));
    writeToolSpawnLedgerV1(root, next);
    write(`[verify tool-spawns] ledger written: ${next.entries.length} test files (was ${ledger.entries.length}).`);
    return 0;
  }
  const { findings, stale } = toolSpawnFindingsV1(scan, ledger), problems = toolRegistryProblemsV1(root, new Set(files)), all = [...findings, ...problems];
  const totals = new Map<string, number>();
  for (const row of actual.entries) totals.set(`${row.tool} ${row.scope}`, (totals.get(`${row.tool} ${row.scope}`) ?? 0) + row.count);
  if (args.includes("--json")) { write(JSON.stringify({ scanned: scan.scanned, offenders: actual, findings: all, stale }, null, 1)); return all.length > 0 || args.includes("--strict") && stale.length > 0 ? 1 : 0; }
  write(`[verify tool-spawns] scanned ${scan.scanned} files; offenders ${actual.entries.reduce((sum, row) => sum + row.count, 0)} in ${actual.entries.length} files.`);
  for (const [key, count] of [...totals].sort((a, b) => b[1] - a[1])) write(`  ${String(count).padStart(5)} ${key}`);
  for (const row of actual.declared) write(`  ${String(row.count).padStart(5)} ${row.tool} ${row.scope} declared in manifests (reported only)`);
  if (args.includes("--report")) return 0;
  for (const finding of all) console.error(`[verify tool-spawns] ${finding.rule} ${finding.path} ${finding.tool}: ${finding.detail}`);
  for (const row of stale.slice(0, 20)) write(`[verify tool-spawns] stale ledger row ${row}`);
  if (stale.length > 0) write(`[verify tool-spawns] ${stale.length} stale ledger rows: run repo verify tool-spawns --prune.`);
  if (all.length > 0) { write(`[verify tool-spawns] ${all.length} finding(s).`); return 1; }
  if (args.includes("--strict") && stale.length > 0) return 1;
  write("[verify tool-spawns] passed.");
  return 0;
}
