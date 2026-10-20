/** 🗺️ The Go dispatch, source topology and root source law routes of the test script. */
import { join } from "node:path";
import { runRepositoryTestCommand } from "../../../🧪️testing/🎛️repository-command/🟦️.ts";
import type { TestRouteContext } from "../🧭️route-context/🟦️.ts";

/** 📍️ Runs the runtime corpus, Go dispatch, catalog, topology, platform and root source routes; true when one matched. */
export async function runSourceTopologyRoutes(context: TestRouteContext, segments: string[]): Promise<boolean> {
  if (segments[0] === "runtime-corpus-behavior") {
    if (segments.length !== 1) throw new Error("Expected test runtime-corpus-behavior");
    if (!process.env.SEMIO_TEST_ARTIFACT_DIR) throw new Error("Caller-owned test output required");
    const sources = [
      "🧰️framework/🔨️modules/🧵️job/⏱️budget/🧪️tests/🔮️corpus-oracles/🟦️.ts",
      "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🪟️visible-surfaces/🧪️tests/🟦️.ts",
      "🧰️framework/🔨️modules/🧬️schema/🔮️oracles/✅️validator/🧪️tests/🟦️.ts",
      "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️tests/🔮️corpus-oracles/🟦️.ts",
      "🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🧪️tests/🔮️corpus-oracles/🟦️.ts",
      "🧰️framework/🔨️modules/🛠️tool-machine/🧪️tests/🧪️gesture-drive-law/🟦️.ts",
      "🧰️framework/🔨️modules/🧬️schema/✅️validator/🧪️tests/🧩️shape/🟦️.ts",
      "🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🧪️tests/🧱️ownership/🟦️.ts",
      "🧰️framework/🔨️modules/📡️replication/🧪️tests/🔮️corpus-oracles/🟦️.ts",
      "🧰️framework/🔨️modules/🧵️job/🧪️tests/📦️physical-close/🔮️oracle/🟦️.ts",
      "🧰️framework/🔨️modules/🎒️pack/🌱️value/🧪️tests/🫳️preflight/🟦️.ts",
      "🧰️framework/🔨️modules/🧬️schema/🧪️tests/🏷️entity-kinds/🟦️.ts",
      "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📦️artifact-packages/🔮️oracle/🟦️.ts",
      "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧱️command-source/🟦️.ts",
      "🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🫀️liveness/🧪️tests/🟦️.ts",
      "🧰️framework/🔨️modules/◻️2d/🧮️compute/🧪️tests/🟦️.ts",
      "🧰️framework/🔨️modules/🎒️pack/🌱️value/🧪️tests/🚦️refusals/🟦️.ts",
      "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔗️backbone-detach/🔮️oracle/🟦️.ts",
      "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔗️remote/🧩️pair/🧪️tests/🧪️canonical-pair-oracle/🔮️oracle/🟦️.ts",
      "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🛑️coordinator/🔮️oracle/🟦️.ts",
      "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧵️canonical-edit/🔮️oracle/🟦️.ts",
      "🧰️framework/🔨️modules/🎒️pack/🌱️value/🧪️tests/🔬️schema-hash/💰️storage/🟦️.ts",
      "🧰️framework/🔨️modules/🎒️pack/🔤️json/🧪️tests/🧱️ownership/🟦️.ts",
      "🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🧪️tests/🟦️.ts",
      "🧰️framework/🔨️modules/🧪️test/🎮️mutation/🏭️inventory/🧪️tests/🟦️.ts",
      "🧰️framework/🔨️modules/🧬️schema/🧪️tests/🧱️neutrality/🟦️.ts",
      "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🚦️test-dispatch/🟦️.ts"
    ];
    await runRepositoryTestCommand(process.execPath, ["test", ...sources.map(source => join(context.repoRoot, source))], { cwd: context.repoRoot, env: process.env, budgetMs: 120_000 });
    return true;
  }
  if (segments[0] === "go-test-dispatch") {
    if (segments.length !== 1) throw new Error("Expected test go-test-dispatch");
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🚦️test-dispatch/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: context.repoRoot, budgetMs: 70_000 });
    return true;
  }
  if (segments[0] === "collection-catalog") {
    if (segments.length !== 1) throw new Error("Expected test collection-catalog");
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🗂️collection-catalog/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: context.repoRoot, budgetMs: 120_000 });
    return true;
  }
  if (segments[0] === "generated-source-topology") {
    if (segments.length !== 1) throw new Error("Expected test generated-source-topology");
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏭️generated-source-topology/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: context.repoRoot });
    return true;
  }
  if (segments[0] === "parser-projection") {
    if (segments.length !== 1) throw new Error("Expected test parser-projection");
    const suites = ["🎯️ajv-oracle", "🧩️split-leaves", "🧩️rendered-forms", "🫥️nullable-fields", "🧷️generic-types"].map((suite) => join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/🧪️tests", suite, "🟦️.ts"));
    await runRepositoryTestCommand(process.execPath, ["test", ...suites], { cwd: context.repoRoot, budgetMs: 240_000 });
    return true;
  }
  if (segments[0] === "package-body-policy") {
    if (segments.length !== 1) throw new Error("Expected test package-body-policy");
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/📦️package-boundary-classification/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: context.repoRoot, budgetMs: 240_000 });
    return true;
  }
  if (segments[0] === "cross-platform-bootstrap") {
    if (segments.length !== 1) throw new Error("Expected test cross-platform-bootstrap");
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🥾️cross-platform-bootstrap/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: context.repoRoot, budgetMs: 120_000 });
    return true;
  }
  if (segments.length === 1 && (segments[0] === "agent-clients" || segments[0] === "derived-config")) {
    const source = segments[0] === "agent-clients"
      ? "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🤖️agent-client-configuration/🟦️.ts"
      : "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📝️derived-config/🧪️tests/📝️derived-config/🟦️.ts";
    await runRepositoryTestCommand(process.execPath, ["test", join(context.repoRoot, source)], { cwd: context.repoRoot, budgetMs: 120_000, signal: context.invocation.control.signal });
    return true;
  }
  if (segments[0] === "windows-command-paths") {
    if (segments.length !== 1) throw new Error("Expected test windows-command-paths");
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪟️windows-command-paths/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: context.repoRoot, budgetMs: 120_000 });
    return true;
  }
  if (segments[0] === "kind-only-basename") {
    if (segments.length !== 1) throw new Error("Expected test kind-only-basename");
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🌳️kind-only-basename/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: context.repoRoot, budgetMs: 45_000 });
    return true;
  }
  if (segments[0] === "framework-source-topology") {
    if (segments.length !== 1) throw new Error("Expected test framework-source-topology");
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️framework-source-topology/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: context.repoRoot, budgetMs: 30_000 });
    return true;
  }
  if (segments[0] === "root-artifact-dependency-source") {
    if (segments.length !== 1) throw new Error("Expected test root-artifact-dependency-source");
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️root-artifact-dependency-source/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: context.repoRoot, budgetMs: 45_000 });
    return true;
  }
  if (segments[0] === "root-taxonomy-workflow-source") {
    if (segments.length !== 1) throw new Error("Expected test root-taxonomy-workflow-source");
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️root-taxonomy-workflow-source/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: context.repoRoot, budgetMs: 45_000 });
    return true;
  }
  if (segments[0] === "root-schema-field-source") {
    if (segments.length !== 1) throw new Error("Expected test root-schema-field-source");
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️root-schema-field-source/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: context.repoRoot, budgetMs: 45_000 });
    return true;
  }
  if (segments[0] === "root-clean-scaffold-source") {
    if (segments.length !== 1) throw new Error("Expected test root-clean-scaffold-source");
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️root-clean-scaffold-source/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: context.repoRoot, budgetMs: 45_000 });
    return true;
  }
  return false;
}
