#!/usr/bin/env bun
import { resolveTestLevel } from "../../../../🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🧪️ Router of the repository testing domain:
//   bun ./📜️script.ts <discover|contract|oracle|subject|parity|run|report|clean|dependency|nx|doctor|acceptance> [args…]

//#endregion 🧲️Header

import { runMutationInventoryProviderChecksV1 } from "./🏭️inventory/🔌️providers/🧪️tests/🟦️.ts";
import { join } from "node:path";
import { orchestratorBudgetOpts, runCmd } from "../📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { Script, ScriptRouter } from "../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runRepoScriptMain } from "../📚️library/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { OracleScript, ParityScript, SubjectScript } from "./⚖️parity/📋️orchestration/🟦️.ts";
import { ContractScript, DiscoverScript, RunScript } from "./🧾️contracts/📋️orchestration/🟦️.ts";
import { FixtureScript } from "./🧾️provenance/📋️orchestration/🟦️.ts";
import { GapScript, ManifestScript } from "./🧾️provenance/🏗️authoring/🟦️.ts";
import { DependencyScript } from "./🕸️dependencies/📋️orchestration/🟦️.ts";
import { DoctorScript, NxScript } from "./🩺️environment/📋️inspection/🟦️.ts";
import { InventoryScript } from "./🏭️inventory/📋️orchestration/🟦️.ts";
import { CleanScript, GcScript } from "./🧹️retention/📋️orchestration/🟦️.ts";
import { SchemaScript } from "./🧬️schema/📋️orchestration/🟦️.ts";
import { MatrixScript, ProbeScript } from "./📊️coverage/📋️orchestration/🟦️.ts";
import { MetricsScript, ReportScript } from "./📊️reporting/📋️orchestration/🟦️.ts";
import { AcceptanceScript } from "./🎯️acceptance/📋️orchestration/🟦️.ts";

/** 🧭️ Explicit full-fleet DSL conformance; never part of focused kernel test routing. */
class DslScript extends Script {
  run(segments: string[]): void {
    const { level, rest } = resolveTestLevel(segments);
    runCmd("bun", ["nx", "run", "@semio-tech/s-fixture-sweep-rs:test", "--skip-nx-cache", "--", level, ...rest], { cwd: this.root, ...orchestratorBudgetOpts() });
  }
}

/** 🧪️ Selects the suite, schema gate, portable command-composition source contract, or the payload-parity, history-gate or fault-notice gate lint corpus. */
class TestScript extends Script {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "schema") return new SchemaScript(this.root, this.repoRoot).run(segments.slice(1));
    if (segments[0] === "artifact-root-source") {
      if (segments.length !== 1) throw new Error("Expected test artifact-root-source");
      runCmd(process.execPath, ["test", join(import.meta.dir, "🧪️tests", "🧾️artifact-root-source", "🟦️.ts")], { cwd: this.repoRoot, ...orchestratorBudgetOpts() });
      return;
    }
    if (segments[0] === "command-composition-source") {
      if (segments.length !== 1) throw new Error("Expected test command-composition-source");
      runCmd(process.execPath, ["test", join(import.meta.dir, "🧪️tests", "🧱️command-composition-source", "🟦️.ts")], { cwd: this.repoRoot, ...orchestratorBudgetOpts() });
      return;
    }
    if (segments[0] === "mutation-payload-parity" || segments[0] === "mutation-history-gates" || segments[0] === "fault-notices-gate") {
      if (segments.length !== 1) throw new Error(`Expected test ${segments[0]}`);
      runCmd(process.execPath, ["test", `./${join(import.meta.dir, "🧪️tests", `🧪️${segments[0]}`, "🟦️.ts").slice(this.repoRoot.length + 1)}`], { cwd: this.repoRoot, ...orchestratorBudgetOpts() });
      return;
    }
    return new RunScript(this.root, this.repoRoot).run(segments);
  }
}

export { policy } from "./⚖️policy/🧹️domain/🟦️.ts";
export const policyFile = "⚖️policy/🧹️domain/🟦️.ts";

const router = new ScriptRouter(import.meta.dir)
  .register("discover", DiscoverScript)
  .register("dsl", DslScript)
  .register("contract", ContractScript)
  .register("oracle", OracleScript)
  .register("subject", SubjectScript)
  .register("parity", ParityScript)
  .register("run", RunScript)
  .register("test", TestScript)
  .register("schema", SchemaScript)
  .register("report", ReportScript)
  .register("clean", CleanScript)
  .register("dependency", DependencyScript)
  .register("metrics", MetricsScript)
  .register("nx", NxScript)
  .register("doctor", DoctorScript)
  .register("inventory", InventoryScript)
  .register("inventory-provider-check", class extends Script { run(args: string[]): void { if (args.length) throw new Error("inventory-provider-check accepts no overrides"); console.log(`mutation inventory provider: ${runMutationInventoryProviderChecksV1()} laws`); } })
  .register("fixture", FixtureScript)
  .register("probe", ProbeScript)
  .register("matrix", MatrixScript)
  .register("gc", GcScript)
  .register("gap", GapScript)
  .register("manifest", ManifestScript)
  .register("acceptance", AcceptanceScript);

await runRepoScriptMain(router, import.meta.url);
