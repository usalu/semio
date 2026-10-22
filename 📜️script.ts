#!/usr/bin/env repo
/**
 * 🧭️ Monorepo command router: `repo ./📜️script.ts <command> [segments…]` (e.g. `📜️script.ts dev storybook`, `📜️script.ts dev mcp stdio os`).
 * The commands, what each needs before it starts and its help live in the registry `🎛️commands/🗂️registry/🔣️.json`; a command that needs the locked dependencies gets them installed through `repo dependencies ensure` before its module is loaded, and `<command> --help` never loads a command.
 */
import type { TechnologyLinter } from "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🚨️policy/🧩️linters/🟦️.ts";
import { type BreachRecord } from "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🚨️policy/🔖️breach/🟦️.ts";
import { ScriptRouter } from "./🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { registerRootCommandsV1, rootCommandHelpV1, rootCommandRegistryV1 } from "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🗂️registry/🟦️.ts";
import { dependencyGateV1 } from "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🗂️registry/🥾️provision/🟦️.ts";
import { dirname } from "node:path";
import { fileURLToPath } from "node:url";

const WORKSPACE_ROOT = dirname(fileURLToPath(import.meta.url));

//#region 🔖️Dispatch
const registry = rootCommandRegistryV1();
const router = registerRootCommandsV1(new ScriptRouter(WORKSPACE_ROOT, WORKSPACE_ROOT), registry, {
  repo: async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts")).RepoScript,
  os: async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧩️os/🟦️.ts")).OsScript,
  components: async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🚀️release/🟦️.ts")).ComponentsScript,
  semio: async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧩️os/🟦️.ts")).SemioScript,
  examples: async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📚️examples/🟦️.ts")).ExamplesScript,
  setup: async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🔧️setup/🟦️.ts")).SetupScript,
  agents: async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🤖️agent-clients/🎮️command/🟦️.ts")).AgentsScript,
  dev: async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🛠️dev/🟦️.ts")).DevScript,
  generate: async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🏭️generate/🟦️.ts")).GenerateScript,
  "scale-fixture": async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧱️scale-fixture/🟦️.ts")).ScaleFixtureScript,
  new: async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/🎮️command/🟦️.ts")).CleanMechanismNewScript,
  schema: async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🟦️.ts")).SchemaScript,
  lint: async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧹️code-style/🟦️.ts")).LintScript,
  verify: async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/🌳️root/🟦️.ts")).RootVerifyScript,
  format: async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧹️code-style/🟦️.ts")).FormatScript,
  test: async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧪️test/🚦️root/🟦️.ts")).RootTestScript,
  bench: async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧪️test/🟦️.ts")).BenchScript,
  stdio: async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🗄️stdio/🟦️.ts")).StdioScript,
  build: async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts")).BuildScript,
  release: async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🚀️release/🟦️.ts")).ReleaseScript,
  cpp: async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🛠️cmake/🟦️.ts")).CppScript,
  publish: async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🚀️release/🟦️.ts")).PublishScript,
  clean: async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧼️workspace-cleanup/🎮️command/🟦️.ts")).CleanScript,
  "micro-commit": async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📝️commit/🟦️.ts")).MicroCommitScript,
  commit: async () => (await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📝️commit/🟦️.ts")).CommitScript,
}, dependencyGateV1(WORKSPACE_ROOT, registry));
//#endregion 🔖️Dispatch

/** ⚖️ The Wave 4 app-plugin consistency lint the policy runner loads from this script (`repo ./📜️script.ts policy`). */
export const policy = async (_l: TechnologyLinter): Promise<BreachRecord[]> => {
  const { policyBreaches } = await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🧭️breaches/🟦️.ts");
  return policyBreaches();
};

if (import.meta.main) {
  const args = process.argv.slice(2);
  const help = args[0] === "repo" || args[0] === "policy" ? undefined : rootCommandHelpV1(registry, args, process.env);
  if (help !== undefined) console.log(help);
  else if (args[0] === "repo") {
    const { runBuildCliV1 } = await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts");
    await runBuildCliV1(WORKSPACE_ROOT, args);
  }
  else {
    const { dispatchPolicyArgv, runWorkspaceScriptMain } = await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🚨️policy/🧭️entrypoints/🟦️.ts");
    if (!(await dispatchPolicyArgv(args, import.meta.url))) await runWorkspaceScriptMain(router);
  }
}
