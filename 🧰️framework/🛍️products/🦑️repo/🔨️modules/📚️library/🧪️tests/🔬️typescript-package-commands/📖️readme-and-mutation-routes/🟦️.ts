/** 📖️ The README source, divergence callback, verification and mutation witness routes of the test script. */
import { join } from "node:path";
import { TEST_LEVEL_BUDGET_MS } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { runRepositoryTestCommand } from "../../../🧪️testing/🎛️repository-command/🟦️.ts";
import { repoTestArtifactEnvironment } from "../../../🏃️process/🌿️environment/🧪️test-output/🟦️.ts";
import { runTransactionV2 } from "../../../🔄️transactions/🧪️verification/📋️orchestration/🟦️.ts";
import type { TestRouteContext } from "../🧭️route-context/🟦️.ts";

/** 🧬️ Runs the README authority, divergence callback, source service, regeneration and mutation witness routes; true when one matched. */
export async function runReadmeAndMutationRoutes(context: TestRouteContext, segments: string[]): Promise<boolean> {
  if (segments[0] === "readme-move-source-authority") {
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🚚️readme-move-source-authority/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: context.repoRoot });
    return true;
  }
  if (segments[0] === "artifact-empty-facet-authority") {
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🫙️artifact-empty-facet-authority/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: context.repoRoot });
    return true;
  }
  if (segments[0] === "rust-divergence-callback-source") {
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/↪️rust-divergence-callback/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, "--test-name-pattern", "^(closed divergence|shared callback|candidate-only callback)", ...segments.slice(1)], { cwd: context.repoRoot });
    return true;
  }
  if (segments[0] === "rust-divergence-callback-native") {
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/↪️rust-divergence-callback/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, "--test-name-pattern", "^actual rustc", ...segments.slice(1)], { cwd: context.repoRoot });
    return true;
  }
  if (segments[0] === "rust-divergence-callback-syn") {
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/↪️rust-divergence-callback/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, "--test-name-pattern", "^independent syn callback", ...segments.slice(1)], { cwd: context.repoRoot, budgetMs: 120_000 });
    return true;
  }
  if (segments[0] === "readme-current-source-activation") {
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🟢️readme-current-source-activation/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: context.repoRoot, env: repoTestArtifactEnvironment(context.repoRoot, "readme-current-source-activation") });
    return true;
  }
  if (segments[0] === "artifact-empty-facet-authoring") {
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪶️artifact-empty-facet-authoring/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: context.repoRoot, env: repoTestArtifactEnvironment(context.repoRoot, "artifact-empty-facet-authoring"), budgetMs: 30_000 });
    return true;
  }
  if (segments[0] === "readme-reviewed-fixture-inputs") {
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/👀️readme-reviewed-fixture-inputs/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: context.repoRoot });
    return true;
  }
  if (segments[0] === "taxonomy-pattern-compiler-reuse") {
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/♻️taxonomy-pattern-compiler-reuse/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: context.repoRoot });
    return true;
  }
  if (segments[0] === "source-services") {
    if (segments.length !== 1) throw new Error("Expected test source-services");
    const tests = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests");
    await runRepositoryTestCommand(process.execPath, ["test", ...["🏗️source-services", "🚪️source-admission", "🚪️source-admission-io", "🔣️taxonomy-input"].map((facet) => join(tests, facet, "🟦️.ts"))], { cwd: context.repoRoot, env: repoTestArtifactEnvironment(context.repoRoot, "source-services") });
    return true;
  }
  if (segments[0] === "multi-scope-verification") {
    if (segments.length !== 1) throw new Error("Expected test multi-scope-verification");
    const budgetMs = TEST_LEVEL_BUDGET_MS[process.env.SEMIO_TEST_LEVEL === "exhaustive" ? "exhaustive" : "long"];
    await runRepositoryTestCommand(process.execPath, ["test", "--timeout", String(budgetMs), join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/🧪️multi-scope-verification/🟦️.ts")], { cwd: context.repoRoot, env: repoTestArtifactEnvironment(context.repoRoot, "multi-scope-verification"), budgetMs });
    return true;
  }
  if (segments[0] === "reference-coverage-selection") {
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎟️reference-coverage-selection/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: context.repoRoot });
    return true;
  }
  if (segments[0] === "taxonomy-leading-grapheme") {
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔤️taxonomy-leading-grapheme/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: context.repoRoot });
    return true;
  }
  if (segments[0] === "reference-coordinate-progress") {
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📈️reference-coordinate-progress/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: context.repoRoot });
    return true;
  }
  if (segments[0] === "markdown-inline-references") {
    if (segments.length !== 1) throw new Error("Markdown inline references accepts no extra arguments");
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔗️markdown-inline-references/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: context.repoRoot });
    return true;
  }
  if (segments[0] === "gherkin-description-inline-code") {
    if (segments.length !== 1) throw new Error("Gherkin description inline code accepts no extra arguments");
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🥒️gherkin-description-inline-code/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: context.repoRoot });
    return true;
  }
  if (segments[0] === "cargo-native-owner-discovery") {
    if(segments.length!==1)throw new Error("Expected test cargo-native-owner-discovery");
    const source=join(context.repoRoot,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🧪️tests/🧭️native-discovery/🟦️.ts");
    await runRepositoryTestCommand(process.execPath,["test",source],{cwd:context.repoRoot,env:repoTestArtifactEnvironment(context.repoRoot,"cargo-native-owner-discovery")});
    return true;
  }
  if(segments[0]==="cargo-nextest-configuration"){
    if(segments.length!==1)throw new Error("Expected test cargo-nextest-configuration");
    const source=join(context.repoRoot,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🧪️tests/🏎️nextest-configuration/🟦️.ts");
    await runRepositoryTestCommand(process.execPath,["test",source],{cwd:context.repoRoot,env:repoTestArtifactEnvironment(context.repoRoot,"cargo-nextest-configuration")});
    return true;
  }
  if (segments[0] === "composition-policy") {
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, "-t", "composition policy"], { cwd: context.repoRoot });
    return true;
  }
  if (segments[0] === "artifact-source-residue") {
    if (segments.length !== 1) throw new Error("Artifact source residue accepts no extra arguments");
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, "--timeout", "120000", "-t", "rejects ignored and unplanned residual children in a projected source owner without following links"], { cwd: context.repoRoot, budgetMs: 120000, env: { ...process.env, SEMIO_TEST_LEVEL: "long" } });
    return true;
  }
  if (segments[0] === "artifact-source-commit") {
    if (segments.length !== 1) throw new Error("Artifact source commit accepts no extra arguments");
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, "--timeout", "120000", "-t", "rolls back and atomically applies CAD and Draw projections to an empty second plan"], { cwd: context.repoRoot, budgetMs: 120000, env: { ...process.env, SEMIO_TEST_LEVEL: "long" } });
    return true;
  }
  if (segments[0] === "transaction-process-observer") {
    if (segments.length !== 1) throw new Error("Transaction process observer accepts no extra arguments");
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🧪️tests/⚙️transaction-process-ownership/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: context.repoRoot });
    return true;
  }
  if (segments[0] === "regeneration-commands") {
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏗️regeneration-commands/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: context.repoRoot });
    return true;
  }
  if (segments[0] === "transaction-v2") {
    await runTransactionV2(context.repoRoot, segments);
    return true;
  }
  if (segments[0] === "marker-only-folders") {
    if (segments.length !== 1) throw new Error("Expected test marker-only-folders");
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️marker-only-folders/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: context.repoRoot });
    return true;
  }
  if (segments[0] === "empty-folders") {
    if (segments.length !== 1) throw new Error("Expected test empty-folders");
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️empty-folders/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: context.repoRoot });
    return true;
  }
      if (segments[0] === "rust-incremental-targets") {
    if (segments.length !== 1) throw new Error("Expected test rust-incremental-targets");
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🦀️incremental-targets/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: context.repoRoot });
    return true;
  }
  if (segments[0] === "mutation-case-pair") {
    if (segments.length !== 1) throw new Error("Expected test mutation-case-pair");
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-case-pair/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, "--timeout", "240000"], { cwd: context.repoRoot, budgetMs: 480_000 });
    return true;
  }
  if (segments[0] === "mutation-wire-witness") {
    if (segments.length !== 1) throw new Error("Expected test mutation-wire-witness");
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️mutation-wire-witness/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, "--timeout", "240000"], { cwd: context.repoRoot, budgetMs: 480_000 });
    return true;
  }
  if (segments[0] === "mutation-leaf-identity") {
    if (segments.length !== 1) throw new Error("Expected test mutation-leaf-identity");
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️mutation-leaf-identity/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, "--timeout", "240000"], { cwd: context.repoRoot, budgetMs: 480_000 });
    return true;
  }
  return false;
}
