/** 📟️ `stdio` command family: ledger and structural gates.
 * @see 📜️script.ts
 */
import { orchestratorBudgetOpts, runCmd } from "../../🏃️process/🟦️.ts";
import { semioBuildParallelFlag } from "../../🏃️process/🌿️environment/🟦️.ts";
import { Script } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { stdioArtifactLedger, stdioRequireClosed, stdioRunStructuralGate } from "../../🗄️stdio-ledger/🟦️.ts";

//#region 🔖️StdioLedgerScript

export class StdioScript extends Script {
  run(segments: string[]): void {
    const gate = segments[0] ?? "quick";
    if (gate === "ledger") {
      process.stdout.write(`${JSON.stringify(stdioArtifactLedger(this.root), null, 2)}\n`);
      return;
    }
    if (gate === "quick" || gate === "schema-parity" || gate === "standards-coverage" || gate === "codec" || gate === "mutation-law" || gate === "inference") {
      const ledger = stdioRunStructuralGate(this.root, gate);
      console.log(`[stdio] ${gate} passed (${ledger.counts.artifacts} artifacts, ${ledger.counts.dialects} dialects, ${ledger.counts.codecs} codecs).`);
      return;
    }
    if (gate === "long") {
      for (const structuralGate of ["quick", "schema-parity", "standards-coverage", "codec", "mutation-law", "inference"] as const) stdioRunStructuralGate(this.root, structuralGate);
      runCmd("bun", ["repo", "run-many", "-t", "test", "-p", "@semio-tech/stdio-plugin", "@semio-tech/stdio-js", ...semioBuildParallelFlag(), "--", "long"], { cwd: this.root, ...orchestratorBudgetOpts() });
      return;
    }
    if (gate === "exhaustive") {
      this.run(["long"]);
      runCmd("bun", ["repo", "run-many", "-t", "test", "-p", "@semio-tech/stdio-plugin", "@semio-tech/stdio-js", ...semioBuildParallelFlag(), "--", "exhaustive"], { cwd: this.root, ...orchestratorBudgetOpts() });
      return;
    }
    if (gate === "runtime") {
      stdioRequireClosed(stdioRunStructuralGate(this.root, "quick"), "runtime");
      return;
    }
    if (gate === "fuzz") {
      stdioRequireClosed(stdioRunStructuralGate(this.root, "quick"), "fuzz");
      return;
    }
    if (gate === "cross-platform") {
      stdioRequireClosed(stdioRunStructuralGate(this.root, "quick"), "cross-platform");
      return;
    }
    throw new Error(`[stdio] expected ledger|quick|long|exhaustive|schema-parity|standards-coverage|codec|mutation-law|inference|runtime|fuzz|cross-platform, got ${JSON.stringify(gate)}.`);
  }
}
//#endregion 🔖️StdioLedgerScript
