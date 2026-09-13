import { type RetentionClass, cleanTestOutputs, collectGarbage, discoverTestCases, formatCleanReport, formatGcReport, isExcludedTestPath, loadOracleRegistry } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { Script } from "../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";

/** 🧹️ Marker-guarded removal of generated test state. Never descends into an excluded area. */
export class CleanScript extends Script {
  run(segments: string[]): void {
    const dry = segments.includes("--dry");
    const stale = segments.includes("--stale");
    const overIndex = segments.indexOf("--over");
    const over = overIndex === -1 ? undefined : Number(segments[overIndex + 1]);
    if (over !== undefined && !Number.isFinite(over)) {
      console.error("[clean test] --over needs a byte count, for example `--over 104857600`");
      process.exit(1);
    }
    const liveTestIds = new Set(discoverTestCases(this.repoRoot).map((entry) => `${entry.owner}::${entry.case}`));
    const report = cleanTestOutputs(this.repoRoot, { dry, stale, over, liveTestIds: stale ? liveTestIds : undefined });
    console.log(formatCleanReport(this.repoRoot, report));
    // 🚫️Which areas may never be touched is taxonomy vocabulary; this router names none of them.
    const leaked = report.removals.filter((row) => isExcludedTestPath(this.repoRoot, row.path));
    if (leaked.length > 0) {
      console.error(`[clean test] refusing: ${leaked.length} candidate(s) resolved inside an excluded area`);
      process.exit(1);
    }
  }
}

/** 🧹️ Mark-and-sweep over the fixture store and the run directories. Dry by default, everywhere. */
export class GcScript extends Script {
  run(segments: string[]): void {
    const registry = loadOracleRegistry(this.repoRoot);
    const value = (flag: string): string | null => {
      const index = segments.indexOf(flag);
      return index === -1 ? null : (segments[index + 1] ?? null);
    };
    const olderThan = value("--older-than");
    const overSize = value("--over-size");
    const report = collectGarbage(this.repoRoot, registry, {
      dry: !segments.includes("--apply"),
      olderThanMs: olderThan === null ? undefined : Number(olderThan) * 1000,
      overBytes: overSize === null ? undefined : Number(overSize),
      agent: value("--agent") ?? undefined,
      retention: value("--retention") === null ? undefined : [value("--retention") as RetentionClass],
    });
    console.log(formatGcReport(report));
  }
}
