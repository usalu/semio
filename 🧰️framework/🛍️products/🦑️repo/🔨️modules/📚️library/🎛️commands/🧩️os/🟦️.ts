/** 💻️ `os` and `semio` commands: headless OS studio run and the `.semio` file processor.
 * @see 📜️script.ts
 */
import { buildBudgetMs } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { daemonBudgetOpts, runCmd } from "../../🏃️process/🟦️.ts";
import { getWorkspaceRoot } from "../../🗂️workspaces/🟦️.ts";
import { Script } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { COMPONENT_PROFILE_POLICIES, SHIPPED_COMPONENT_PROFILE } from "../../../../../💻️os/🔨️modules/🔌️plugin/🖨️describe/🏷️component-receipt/🟦️.ts";
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";

//#region 🔖️OsScript
/** 🧩️One `framework/os/dev` plugin registry row ([[🔌️plugins.json]]) — only the fields `os run`'s preflight needs. */
type OsPluginArtifact = { pluginId: string; cratePath: string; wasmOut: string };

/** 🔍️ Whether one registry row's shipped component exists under its crate's `dist/components` — the same path as
 * `resolve_plugin_paths` in `semio-framework-os-run`. */
function pluginWasmArtifactExists(repoRoot: string, entry: OsPluginArtifact): boolean {
  return existsSync(join(repoRoot, entry.cratePath, COMPONENT_PROFILE_POLICIES[SHIPPED_COMPONENT_PROFILE].directory, entry.wasmOut));
}

function missingPluginWasmArtifacts(repoRoot: string): string[] {
  const registryPath = join(repoRoot, "✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🔌️plugins.json");
  if (!existsSync(registryPath)) return [];
  const entries = JSON.parse(readFileSync(registryPath, "utf8")) as OsPluginArtifact[];
  return entries.filter((entry) => !pluginWasmArtifactExists(repoRoot, entry)).map((entry) => entry.pluginId);
}

/** 🕸️Headless OS studio commands — computes a workflow without a UI (`os run <bundle>.studio`). */
export class OsScript extends Script {
  run(segments: string[]): void {
    const sub = segments[0];
    if (sub === "run") {
      const rest = segments.slice(1);
      const bundle = rest.find((segment) => !segment.startsWith("--"));
      if (!bundle) {
        console.error("[os.run] usage: bun ./📜️script.ts os run <bundle>.studio [--node <id>] [--watch] [--dry]");
        process.exit(1);
      }
      const repoRoot = getWorkspaceRoot();
      if (!rest.includes("--dry")) {
        const missing = missingPluginWasmArtifacts(repoRoot);
        if (missing.length > 0) {
          console.error(`[os.run] missing built plugin artifact(s): ${missing.join(", ")}`);
          for (const pluginId of missing) console.error(`[os.run]   build it: bun repo run workspace:components describe ${pluginId}`);
          process.exit(1);
        }
      }
      const watch = rest.includes("--watch");
      runCmd("cargo", ["run", "-p", "semio-framework-os-run", "--", ...rest], {
        cwd: this.root,
        env: { ...process.env, SEMIO_REPO_ROOT: repoRoot },
        ...(watch ? daemonBudgetOpts() : { budgetMs: buildBudgetMs() }),
      });
      return;
    }
    console.error(`[os] unknown subcommand ${JSON.stringify(sub)}`);
    process.exit(1);
  }
}
//#endregion 🔖️OsScript

//#region 🔖️SemioScript
/** 🧬 Universal `.semio` file processor (`inspect`, `verify`, `open`, `convert`). */
export class SemioScript extends Script {
  run(segments: string[]): void {
    if (segments.length === 0) {
      console.error("[semio] usage: bun ./📜️script.ts semio <inspect|verify|open|convert> <path>");
      process.exit(1);
    }
    runCmd("cargo", ["run", "-p", "semio-framework-os-kernel", "--features", "native-bin", "--bin", "semio", "--", ...segments], {
      cwd: this.root,
      budgetMs: buildBudgetMs(),
    });
  }
}
//#endregion 🔖️SemioScript
