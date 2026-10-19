import type { ScriptInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolveTestLevel } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { join } from "node:path";
import { repoTestArtifactEnvironment } from "../../../📚️library/📦️packages/🟦️typescript/🏃️process/🟦️.ts";
import { runRepositoryTestCommand } from "../../../📚️library/📦️packages/🟦️typescript/🧪️testing/🟦️.ts";

const PLATFORM_TESTS: ReadonlyMap<string, string> = new Map([
  ["platform-discovery-contract", "🔍️discovery-contract"],
  ["platform-comparison-profiles", "⚖️comparison-profiles"],
  ["platform-oracle-governance", "🔮️oracle-governance"],
  ["platform-mutation-catalogs", "🦠️mutation-catalogs"],
]);

/** 🏃️ Runs the `TestScript` command under the caller's roots and invocation. */
export async function runTestScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  if (segments[0] === "fixture-isolation") {
    if (segments.length !== 1) throw new Error("Expected test fixture-isolation");
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧬️schema-invariants/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, "-t", "placement, ownership and fixture isolation|schema protocol|module-member carve-out|repository that declares no submodules|every generator case"], { cwd: context.repoRoot, env: repoTestArtifactEnvironment(context.repoRoot, "fixture-isolation"), budgetMs: 30_000 });
    return;
  }
  if (segments[0] === "discovery-boundaries") {
    if (segments.length !== 1) throw new Error("Expected test discovery-boundaries");
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🚷️discovery-boundaries/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: context.repoRoot, env: repoTestArtifactEnvironment(context.repoRoot, "discovery-boundaries"), budgetMs: 30_000 });
    return;
  }
  const platform = PLATFORM_TESTS.get(segments[0] ?? "");
  if (platform) {
    if (segments.length !== 1) throw new Error(`Expected test ${segments[0]}`);
    await runRepositoryTestCommand(process.execPath, ["test", `../../🧪️tests/${platform}/🟦️.ts`], { cwd: context.root });
    return;
  }
  const { rest } = resolveTestLevel(segments);
  await runRepositoryTestCommand(process.execPath, ["test", "../../🧪️tests/🚷️discovery-boundaries/🟦️.ts", "../../🧪️tests/🔍️discovery-contract/🟦️.ts", "../../🧪️tests/⚖️comparison-profiles/🟦️.ts", "../../🧪️tests/🔮️oracle-governance/🟦️.ts", "../../🧪️tests/🦠️mutation-catalogs/🟦️.ts", "../../🧪️tests/🧬️schema-invariants/🟦️.ts", "../../🧪️tests/📐️test-layout/🟦️.ts", "../../🧪️tests/🧭️fixture-resolution/🟦️.ts", "../../🧪️tests/🧬️mutation-fixtures/🟦️.ts", ...rest], { cwd: context.root });
}
