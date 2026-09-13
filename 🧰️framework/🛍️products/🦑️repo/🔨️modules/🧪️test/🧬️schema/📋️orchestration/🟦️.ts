import { type SchemaDiagnostic, type SchemaFixtureReport, discoverSchemaFixtures, runSchemaFixture, schemaContractDiagnostics } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { Script } from "../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { join } from "node:path";

/**
 * 🧬️ `test schema` — the scope-owned schema contract gate.
 *
 * It answers three questions in one pass and keeps them apart in the output: does every contract sit
 * on an eligible owner in the one place it belongs (the invariants), does every `schema://` reference
 * resolve through the declared catalog (resolution), and does every bound fixture fail or pass at the
 * STAGE it declared (the fixtures). Nothing here searches the tree for a schema; an absent catalog is
 * reported as absent, because a gate that silently found a substitute would be measuring the
 * substitute.
 *
 *   bun 📜️script.ts test schema                     # the whole tree
 *   bun 📜️script.ts test schema --under <path>      # one subtree, while the rest is mid-migration
 *   bun 📜️script.ts test schema --json
 */
export class SchemaScript extends Script {
  run(segments: string[]): void {
    const under = segments[segments.indexOf("--under") + 1];
    const scope = segments.includes("--under") && under !== undefined ? under : "";
    const diagnostics: SchemaDiagnostic[] = schemaContractDiagnostics(this.repoRoot, scope);
    const reports: SchemaFixtureReport[] = [];
    for (const collection of discoverSchemaFixtures(this.repoRoot, scope)) for (const fixture of collection.fixtures) reports.push(runSchemaFixture(this.repoRoot, fixture, collection.caseDir));
    const failed = reports.filter((report) => report.outcome === "failed");
    if (segments.includes("--json")) {
      console.log(JSON.stringify({ diagnostics, fixtures: reports }, null, 2));
      process.exit(diagnostics.length === 0 && failed.length === 0 ? 0 : 1);
    }
    const byCode = new Map<string, number>();
    for (const entry of diagnostics) byCode.set(entry.code, (byCode.get(entry.code) ?? 0) + 1);
    console.log(`[test schema] ${diagnostics.length} invariant finding(s) over ${scope.length === 0 ? "the repository" : scope}`);
    for (const [code, count] of [...byCode].sort((a, b) => b[1] - a[1])) console.log(`[test schema]   ${String(count).padStart(5)} × ${code}`);
    for (const entry of diagnostics.slice(0, 40)) console.log(`[test schema]   ${entry.code} ${entry.path ?? entry.scope ?? ""} — ${entry.detail}`);
    if (diagnostics.length > 40) console.log(`[test schema]   … and ${diagnostics.length - 40} more`);
    console.log(`[test schema] ${reports.length - failed.length}/${reports.length} schema-bound fixture(s) reached their declared stage`);
    for (const report of reports) {
      const line = report.stages.map((entry) => `${entry.stage}=${entry.result}`).join(" ");
      console.log(`[test schema]   ${report.outcome === "passed" ? "✔" : "✘"} ${report.fixture} ${report.uri} — ${line}`);
      if (report.outcome === "failed") console.log(`[test schema]     ${report.detail}`);
    }
    process.exit(diagnostics.length === 0 && failed.length === 0 ? 0 : 1);
  }
}
