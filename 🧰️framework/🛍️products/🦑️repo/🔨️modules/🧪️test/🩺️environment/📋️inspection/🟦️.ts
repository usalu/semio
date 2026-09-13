import { oracleHostPython } from "../../🖥️host/🏗️materialization/🟦️.ts";
import { type Implementation, discoverTestCases, testCacheDir, testTaxonomy } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { Script, runProbe } from "../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { join, relative, sep } from "node:path";

/** 🕸️ Emits the per-case Nx project graph the plugin generates, for inspection and for CI. */
export class NxScript extends Script {
  run(): void {
    console.log(
      JSON.stringify(
        discoverTestCases(this.repoRoot).map((entry) => ({ name: entry.projectName, root: entry.caseDir, owner: entry.owner, implementations: Object.keys(entry.adapters) })),
        null,
        2,
      ),
    );
  }
}

/** 🩺️ Reports which toolchains are present. A missing tool fails setup — it never becomes a silent skip. */
export class DoctorScript extends Script {
  run(): void {
    const taxonomy = testTaxonomy(this.repoRoot);
    const checks: [Implementation, string, string[]][] = [
      ["typescript", "bun", ["--version"]],
      ["rust", "cargo", ["--version"]],
      ["go", "go", ["version"]],
      ["python", oracleHostPython(this.repoRoot), ["--version"]],
      ["dotnet", "dotnet", ["--version"]],
    ];
    const claimed = new Set(discoverTestCases(this.repoRoot).flatMap((entry) => Object.keys(entry.adapters)));
    let missing = 0;
    for (const [implementation, command, args] of checks) {
      const probe = runProbe(command, args, { cwd: this.repoRoot, budgetMs: 30_000 });
      const ok = (probe.status ?? 1) === 0;
      const required = claimed.has(implementation);
      console.log(`[doctor] ${implementation}: ${ok ? probe.stdout.trim().split("\n")[0] : "MISSING"}${required ? " (required by a discovered case)" : ""}`);
      if (!ok && required) missing += 1;
    }
    console.log(`[doctor] cache root: ${relative(this.repoRoot, testCacheDir(this.repoRoot, taxonomy.testOutputChildDirs[0]!)).split(sep).join("/")}`);
    if (missing > 0) process.exit(1);
  }
}
