/** 🔗️ The path statute, mutation source, reference and authority routes of the test script. */
import { join } from "node:path";
import { TEST_LEVEL_BUDGET_MS, resolveTestLevel } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { runRepositoryTestCommand } from "../../../🧪️testing/🎛️repository-command/🟦️.ts";
import { repoTestArtifactEnvironment } from "../../../🏃️process/🌿️environment/🧪️test-output/🟦️.ts";
import type { TestRouteContext } from "../🧭️route-context/🟦️.ts";

/** 🪝️ Runs the path statute, mutation source, provider, reference and recovery authority routes; true when one matched. */
export async function runReferenceAuthorityRoutes(context: TestRouteContext, segments: string[]): Promise<boolean> {
  if (segments[0] === "path-emoji-statutes") {
    if (segments.length !== 1) throw new Error("Expected test path-emoji-statutes");
    resolveTestLevel(["long"]);
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔏️path-emoji-statutes/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: context.repoRoot });
    return true;
  }
  if (segments[0] === "mutation-ticket-role-routing") {
    if (segments.length !== 1) throw new Error("Expected test mutation-ticket-role-routing");
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎫️ticket-role-routing/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: context.repoRoot });
    return true;
  }
  if (segments[0] === "mutation-source-index-capture") {
    if (segments.length !== 1) throw new Error("Expected test mutation-source-index-capture");
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📸️source-index-capture/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: context.repoRoot });
    return true;
  }
  if (segments[0] === "mutation-source-roster-roles") {
    if (segments.length !== 1) throw new Error("Expected test mutation-source-roster-roles");
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎭️source-roster-roles/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: context.repoRoot });
    return true;
  }
  if (segments[0] === "mutation-source-file-facts") {
    if (segments.length > 2 || (segments[1] !== undefined && segments[1] !== "reference")) throw new Error("Expected test mutation-source-file-facts [reference]");
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧾️source-file-facts/🟦️.ts");
    const selection = segments[1] === "reference" ? ["-t", "^mutation source-file facts (vectors|independent suffix reference|reference oracle)"] : [];
    await runRepositoryTestCommand(process.execPath, ["test", source, ...selection], { cwd: context.repoRoot });
    return true;
  }
  if (segments[0] === "typescript-declaration-facts") {
    if (segments.length > 2 || (segments[1] !== undefined && segments[1] !== "reference")) throw new Error("Expected test typescript-declaration-facts [reference]");
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📣️typescript-declaration-facts/🟦️.ts");
    const selection = segments[1] === "reference" ? ["-t", "^TypeScript (?:(?:malformed|unsupported) )?declaration (?:reference:|(?:facts|cases) use the closed neutral schema)"] : [];
    await runRepositoryTestCommand(process.execPath, ["test", source, ...selection], { cwd: context.repoRoot });
    return true;
  }
  if (segments[0] === "artifact-support") {
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🍃️artifact-support-leaf-authority/🟦️.ts");
    const { rest } = resolveTestLevel(segments.slice(1));
    await runRepositoryTestCommand(process.execPath, ["test", source, ...rest], { cwd: context.repoRoot });
    return true;
  }
  if (segments[0] === "historical-package-owner-identity") {
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏺️historical-package-owner-identity/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: context.repoRoot });
    return true;
  }
  if (segments[0] === "cargo-provider-binding-trace") {
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪢️cargo-provider-binding/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: context.repoRoot });
    return true;
  }
  if (segments[0] === "metadata-source-provider") {
    const { rest } = resolveTestLevel(segments.slice(1));
    await runRepositoryTestCommand(process.execPath, ["test", "../../🧪️tests/🔬️workspace-contract/🟦️.ts", "-t", "mutation metadata source provider", ...rest], { cwd: context.root });
    return true;
  }
  if (segments[0] === "rust-physical-reference-context") {
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧲️rust-physical-reference-context/🟦️.ts");
    const { rest } = resolveTestLevel(segments.slice(1));
    await runRepositoryTestCommand(process.execPath, ["test", source, ...rest], { cwd: context.repoRoot, env: repoTestArtifactEnvironment(context.repoRoot, "rust-physical-reference-context") });
    return true;
  }
  if (segments[0] === "taxonomy-cli-cancellation") {
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🛑️taxonomy-cli-cancellation/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: context.repoRoot, env: repoTestArtifactEnvironment(context.repoRoot, "taxonomy-cli-cancellation") });
    return true;
  }
  if (segments[0] === "inventory-artifact-shards") {
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/💠️inventory-artifact-shards/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: context.repoRoot });
    return true;
  }
  if (segments[0] === "root-script-compiler") {
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/⚙️root-script-compiler/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: context.repoRoot });
    return true;
  }
  if (segments[0] === "json-reference-owner-lookup") {
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔎️json-reference-owner-lookup/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: context.repoRoot });
    return true;
  }
  if (segments[0] === "nx-workspace-root-file-reference") {
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏠️nx-workspace-root-file-reference/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: context.repoRoot });
    return true;
  }
  if (segments[0] === "bare-reference-sibling-precedence") {
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🥇️bare-reference-sibling-precedence/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: context.repoRoot });
    return true;
  }
  if (segments[0] === "run-vitest-config-argument-tokens") {
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏃️run-vitest-config-argument-tokens/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: context.repoRoot });
    return true;
  }
  if (segments[0] === "cargo-discovery-exclusions") {
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🚧️cargo-discovery-exclusions/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, join(context.repoRoot,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🧪️tests/🟦️.ts"), join(context.repoRoot,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🧪️tests/production/🟦️.ts"), join(context.repoRoot,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🧪️tests/🎛️preparation/🟦️.ts"), ...segments.slice(1)], { cwd: context.repoRoot, budgetMs: TEST_LEVEL_BUDGET_MS.quick });
    return true;
  }
  if (segments[0] === "nested-cargo-collision-authority") {
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/💥️nested-cargo-collision-authority/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: context.repoRoot });
    return true;
  }
  if (segments[0] === "registry-import-language") {
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🌐️registry-import-language/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: context.repoRoot });
    return true;
  }
  if (segments[0] === "preflight-reference-basis") {
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🛫️preflight-reference-basis/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: context.repoRoot, env: repoTestArtifactEnvironment(context.repoRoot, "preflight-reference-basis") });
    return true;
  }
  if (segments[0] === "typescript-path-collection") {
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🛤️typescript-path-collection/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: context.repoRoot });
    return true;
  }
  if (segments[0] === "frozen-markdown-coordinates") {
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/❄️frozen-markdown-coordinates/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: context.repoRoot });
    return true;
  }
  if (segments[0] === "historical-document-evidence") {
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📜️historical-document-evidence/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: context.repoRoot });
    return true;
  }
  if (segments[0] === "cargo-target-discovery-skip") {
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎯️cargo-target-discovery-skip/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: context.repoRoot });
    return true;
  }
  if (segments[0] === "registry-catalog-gitlink-boundary") {
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧾️registry-catalog-gitlink-boundary/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: context.repoRoot });
    return true;
  }
  if (segments[0] === "historical-json-source-encoding") {
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🕰️historical-json-source-encoding/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: context.repoRoot });
    return true;
  }
  if (segments[0] === "transaction-recovery-authority") {
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🛟️transaction-recovery-authority/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: context.repoRoot });
    return true;
  }
  if (segments[0] === "transaction-fixture-key-exactness") {
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🗝️transaction-fixture-key-exactness/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: context.repoRoot });
    return true;
  }
  if (segments[0] === "testing-readme-coordinates") {
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🗺️testing-readme-coordinates/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: context.repoRoot });
    return true;
  }
  if (segments[0] === "rust-finite-target-consumption") {
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🥤️rust-finite-target-consumption/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: context.repoRoot, env: repoTestArtifactEnvironment(context.repoRoot, "rust-finite-target-consumption") });
    return true;
  }
  if (segments[0] === "readme-current-source-revision") {
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔖️readme-current-source-revision/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: context.repoRoot });
    return true;
  }
  if (segments[0] === "rust-writable-path-authority") {
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/✍️rust-writable-path-authority/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: context.repoRoot });
    return true;
  }
  return false;
}
