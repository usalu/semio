#!/usr/bin/env bun
/** 🧭️ `@semio-tech/repo-lib` router: `bun ./📜️script.ts <lint|test [level]|workspaces <--write|--check>>`. */
import { join } from "node:path";
import { BundleScript, ScriptRouter, runBundleScriptMain, runBunx, resolveTestLevel, runTestBudgeted } from "./🟦️.ts";
import { repoTestArtifactEnvironment } from "../../🏃️process/🌿️environment/🧪️test-output/🟦️.ts";
import { runTransactionV2 } from "../../🔄️transactions/🧪️verification/📋️orchestration/🟦️.ts";
import { GoTestScript } from "../../🧪️execution/🐹️go/🟦️.ts";
import { WorkspacePublicationScript } from "../../🗂️workspaces/🏃️execution/🟦️.ts";

class LintScript extends BundleScript {
  run(): void {
    runBunx(["tsc", "-p", "tsconfig.json", "--noEmit"], this.root);
  }
}


class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "process-budgets") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/⏱️process-budgets/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot, env: repoTestArtifactEnvironment(this.repoRoot, "process-budgets"), budgetMs: 30_000 });
      return;
    }
    if (segments[0] === "exact-cargo-laws") {
      if (segments.length !== 1) throw new Error("Expected test exact-cargo-laws");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🦀️exact-cargo-laws/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source], { cwd: this.repoRoot, env: repoTestArtifactEnvironment(this.repoRoot, "exact-cargo-laws") });
      return;
    }
    if (segments[0] === "go-input-projection") {
      if (segments.length !== 1) throw new Error("Expected test go-input-projection");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🐹️canonical-go-discovery/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source], { cwd: this.repoRoot, budgetMs: 30_000 });
      return;
    }
    if (segments[0] === "go-test-dispatch") {
      if (segments.length !== 1) throw new Error("Expected test go-test-dispatch");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🚦️test-dispatch/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source], { cwd: this.repoRoot, budgetMs: 70_000 });
      return;
    }
    if (segments[0] === "generated-source-topology") {
      if (segments.length !== 1) throw new Error("Expected test generated-source-topology");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏭️generated-source-topology/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "package-body-policy") {
      if (segments.length !== 1) throw new Error("Expected test package-body-policy");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/📦️package-boundary-classification/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source], { cwd: this.repoRoot, budgetMs: 240_000 });
      return;
    }
    if (segments[0] === "kind-only-basename") {
      if (segments.length !== 1) throw new Error("Expected test kind-only-basename");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🌳️kind-only-basename/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source], { cwd: this.repoRoot, budgetMs: 45_000 });
      return;
    }
    if (segments[0] === "framework-source-topology") {
      if (segments.length !== 1) throw new Error("Expected test framework-source-topology");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️framework-source-topology/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source], { cwd: this.repoRoot, budgetMs: 30_000 });
      return;
    }
    if (segments[0] === "root-artifact-dependency-source") {
      if (segments.length !== 1) throw new Error("Expected test root-artifact-dependency-source");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️root-artifact-dependency-source/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source], { cwd: this.repoRoot, budgetMs: 45_000 });
      return;
    }
    if (segments[0] === "root-taxonomy-workflow-source") {
      if (segments.length !== 1) throw new Error("Expected test root-taxonomy-workflow-source");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️root-taxonomy-workflow-source/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source], { cwd: this.repoRoot, budgetMs: 45_000 });
      return;
    }
    if (segments[0] === "root-schema-field-source") {
      if (segments.length !== 1) throw new Error("Expected test root-schema-field-source");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️root-schema-field-source/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source], { cwd: this.repoRoot, budgetMs: 45_000 });
      return;
    }
    if (segments[0] === "root-clean-scaffold-source") {
      if (segments.length !== 1) throw new Error("Expected test root-clean-scaffold-source");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️root-clean-scaffold-source/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source], { cwd: this.repoRoot, budgetMs: 45_000 });
      return;
    }
    if (segments[0] === "root-artifact-schema-law-source") {
      if (segments.length !== 1) throw new Error("Expected test root-artifact-schema-law-source");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️root-artifact-schema-law-source/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source], { cwd: this.repoRoot, env: repoTestArtifactEnvironment(this.repoRoot, "root-artifact-schema-law-source"), budgetMs: 45_000 });
      return;
    }
    if (segments[0] === "root-surface-abstraction-law-source") {
      if (segments.length !== 1) throw new Error("Expected test root-surface-abstraction-law-source");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️root-surface-abstraction-law-source/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source], { cwd: this.repoRoot, budgetMs: 60_000 });
      return;
    }
    if (segments[0] === "root-inference-law-source") {
      if (segments.length !== 1) throw new Error("Expected test root-inference-law-source");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️root-inference-law-source/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source], { cwd: this.repoRoot, env: repoTestArtifactEnvironment(this.repoRoot, "root-inference-law-source"), budgetMs: 45_000 });
      return;
    }
    if (segments[0] === "cargo-transaction-command-source") {
      if (segments.length !== 1) throw new Error("Expected test cargo-transaction-command-source");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️cargo-transaction-command-source/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source], { cwd: this.repoRoot, env: repoTestArtifactEnvironment(this.repoRoot, "cargo-transaction-command-source"), budgetMs: 45_000 });
      return;
    }
    if (segments[0] === "workspace-publication-source") {
      if (segments.length !== 1) throw new Error("Expected test workspace-publication-source");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️workspace-publication-source/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source], { cwd: this.repoRoot, budgetMs: 45_000 });
      return;
    }
    if (segments[0] === "wasm-package-wrappers") {
      if (segments.length !== 1) throw new Error("Expected test wasm-package-wrappers");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️wasm-package-wrappers/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source], { cwd: this.repoRoot, env: repoTestArtifactEnvironment(this.repoRoot, "wasm-package-wrappers"), budgetMs: 45_000 });
      return;
    }
    if (segments[0] === "framework-root-source-topology") {
      if (segments.length !== 1) throw new Error("Expected test framework-root-source-topology");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧰️framework-root-source-topology/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source], { cwd: this.repoRoot, budgetMs: 30_000 });
      return;
    }
    if (segments[0] === "manifestless-source-closure") {
      if (segments.length !== 1) throw new Error("Expected test manifestless-source-closure");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️manifestless-source-closure/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source], { cwd: this.repoRoot, budgetMs: 30_000 });
      return;
    }
    if (segments[0] === "plugin-publication-source-ownership") {
      if (segments.length !== 1) throw new Error("Expected test plugin-publication-source-ownership");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📣️plugin-publication-source-ownership/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source], { cwd: this.repoRoot, budgetMs: 30_000 });
      return;
    }
    if (segments[0] === "app-verification-source-ownership") {
      if (segments.length !== 1) throw new Error("Expected test app-verification-source-ownership");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📱️app-verification-source-ownership/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source], { cwd: this.repoRoot, budgetMs: 30_000 });
      return;
    }
    if (segments[0] === "vitest-configuration-ownership") {
      if (segments.length !== 1) throw new Error("Expected test vitest-configuration-ownership");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎚️vitest-configuration-ownership/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source], { cwd: this.repoRoot, budgetMs: 120_000 });
      return;
    }
    if (segments[0] === "tool-configuration-ownership") {
      if (segments.length !== 1) throw new Error("Expected test tool-configuration-ownership");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎚️tool-configuration-ownership/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source], { cwd: this.repoRoot, budgetMs: 120_000 });
      return;
    }
    if (segments[0] === "os-dev-composition-ownership") {
      if (segments.length !== 1) throw new Error("Expected test os-dev-composition-ownership");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧑‍💻os-dev-composition-ownership/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source], { cwd: this.repoRoot, budgetMs: 30_000 });
      return;
    }
    if (segments[0] === "repo-source-ownership") {
      if (segments.length !== 1) throw new Error("Expected test repo-source-ownership");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🦑️repo-source-ownership/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source], { cwd: this.repoRoot, budgetMs: 30_000 });
      return;
    }
    if (segments[0] === "storybook-discovery") {
      if (segments.length !== 1) throw new Error("Expected test storybook-discovery");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️storybook-discovery/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "os-source-topology") {
      if (segments.length !== 1) throw new Error("Expected test os-source-topology");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖥️os-source-topology/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "path-emoji-statutes") {
      if (segments.length !== 1) throw new Error("Expected test path-emoji-statutes");
      resolveTestLevel(["long"]);
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔏️path-emoji-statutes/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "mutation-ticket-role-routing") {
      if (segments.length !== 1) throw new Error("Expected test mutation-ticket-role-routing");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎫️ticket-role-routing/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "mutation-source-index-capture") {
      if (segments.length !== 1) throw new Error("Expected test mutation-source-index-capture");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📸️source-index-capture/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "mutation-source-roster-roles") {
      if (segments.length !== 1) throw new Error("Expected test mutation-source-roster-roles");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎭️source-roster-roles/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "mutation-source-file-facts") {
      if (segments.length > 2 || (segments[1] !== undefined && segments[1] !== "reference")) throw new Error("Expected test mutation-source-file-facts [reference]");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧾️source-file-facts/🟦️.ts");
      const selection = segments[1] === "reference" ? ["-t", "^mutation source-file facts (vectors|independent suffix reference|reference oracle)"] : [];
      await runTestBudgeted(process.execPath, ["test", source, ...selection], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "typescript-declaration-facts") {
      if (segments.length > 2 || (segments[1] !== undefined && segments[1] !== "reference")) throw new Error("Expected test typescript-declaration-facts [reference]");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📣️typescript-declaration-facts/🟦️.ts");
      const selection = segments[1] === "reference" ? ["-t", "^TypeScript (?:(?:malformed|unsupported) )?declaration (?:reference:|(?:facts|cases) use the closed neutral schema)"] : [];
      await runTestBudgeted(process.execPath, ["test", source, ...selection], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "artifact-support") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🍃️artifact-support-leaf-authority/🟦️.ts");
      const { rest } = resolveTestLevel(segments.slice(1));
      await runTestBudgeted(process.execPath, ["test", source, ...rest], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "historical-package-owner-identity") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏺️historical-package-owner-identity/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "cargo-provider-binding-trace") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪢️cargo-provider-binding/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "metadata-source-provider") {
      const { rest } = resolveTestLevel(segments.slice(1));
      await runTestBudgeted(process.execPath, ["test", "../../🧪️tests/🔬️workspace-contract/🟦️.ts", "-t", "mutation metadata source provider", ...rest], { cwd: this.root });
      return;
    }
    if (segments[0] === "rust-physical-reference-context") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧲️rust-physical-reference-context/🟦️.ts");
      const { rest } = resolveTestLevel(segments.slice(1));
      await runTestBudgeted(process.execPath, ["test", source, ...rest], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "taxonomy-cli-cancellation") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🛑️taxonomy-cli-cancellation/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot, env: repoTestArtifactEnvironment(this.repoRoot, "taxonomy-cli-cancellation") });
      return;
    }
    if (segments[0] === "inventory-artifact-shards") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/💠️inventory-artifact-shards/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "root-script-compiler") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/⚙️root-script-compiler/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "json-reference-owner-lookup") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔎️json-reference-owner-lookup/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "nx-workspace-root-file-reference") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏠️nx-workspace-root-file-reference/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "bare-reference-sibling-precedence") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🥇️bare-reference-sibling-precedence/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "run-vitest-config-argument-tokens") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏃️run-vitest-config-argument-tokens/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "cargo-discovery-exclusions") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🚧️cargo-discovery-exclusions/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "nested-cargo-collision-authority") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/💥️nested-cargo-collision-authority/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "registry-import-language") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🌐️registry-import-language/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "preflight-reference-basis") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🛫️preflight-reference-basis/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "typescript-path-collection") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🛤️typescript-path-collection/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "frozen-markdown-coordinates") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/❄️frozen-markdown-coordinates/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "historical-document-evidence") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📜️historical-document-evidence/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "cargo-target-discovery-skip") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎯️cargo-target-discovery-skip/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "registry-catalog-gitlink-boundary") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧾️registry-catalog-gitlink-boundary/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "historical-json-source-encoding") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🕰️historical-json-source-encoding/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "transaction-recovery-authority") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🛟️transaction-recovery-authority/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "transaction-fixture-key-exactness") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🗝️transaction-fixture-key-exactness/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "testing-readme-coordinates") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🗺️testing-readme-coordinates/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "rust-finite-target-consumption") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🥤️rust-finite-target-consumption/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "readme-current-source-revision") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔖️readme-current-source-revision/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "rust-writable-path-authority") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/✍️rust-writable-path-authority/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "readme-move-source-authority") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🚚️readme-move-source-authority/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "artifact-empty-facet-authority") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🫙️artifact-empty-facet-authority/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "rust-divergence-callback-source") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/↪️rust-divergence-callback/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source, "--test-name-pattern", "^(closed divergence|shared callback|candidate-only callback)", ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "rust-divergence-callback-native") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/↪️rust-divergence-callback/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source, "--test-name-pattern", "^actual rustc", ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "rust-divergence-callback-syn") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/↪️rust-divergence-callback/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source, "--test-name-pattern", "^independent syn callback", ...segments.slice(1)], { cwd: this.repoRoot, budgetMs: 120_000 });
      return;
    }
    if (segments[0] === "readme-current-source-activation") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🟢️readme-current-source-activation/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "artifact-empty-facet-authoring") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪶️artifact-empty-facet-authoring/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot, env: repoTestArtifactEnvironment(this.repoRoot, "artifact-empty-facet-authoring"), budgetMs: 30_000 });
      return;
    }
    if (segments[0] === "readme-reviewed-fixture-inputs") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/👀️readme-reviewed-fixture-inputs/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "taxonomy-pattern-compiler-reuse") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/♻️taxonomy-pattern-compiler-reuse/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "reference-coverage-selection") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎟️reference-coverage-selection/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "taxonomy-leading-grapheme") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔤️taxonomy-leading-grapheme/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "reference-coordinate-progress") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📈️reference-coordinate-progress/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "draw-destination-observation") {
      if (segments.length !== 1) throw new Error("Draw destination observation accepts no extra arguments");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📍️draw-destination-observation/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "markdown-inline-references") {
      if (segments.length !== 1) throw new Error("Markdown inline references accepts no extra arguments");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔗️markdown-inline-references/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "gherkin-description-inline-code") {
      if (segments.length !== 1) throw new Error("Gherkin description inline code accepts no extra arguments");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🥒️gherkin-description-inline-code/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "composition-policy") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source, "-t", "composition policy"], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "artifact-source-residue") {
      if (segments.length !== 1) throw new Error("Artifact source residue accepts no extra arguments");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source, "--timeout", "120000", "-t", "rejects ignored and unplanned residual children in a projected source owner without following links"], { cwd: this.repoRoot, budgetMs: 120000 });
      return;
    }
    if (segments[0] === "artifact-source-commit") {
      if (segments.length !== 1) throw new Error("Artifact source commit accepts no extra arguments");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source, "--timeout", "120000", "-t", "rolls back and atomically applies CAD and Draw projections to an empty second plan"], { cwd: this.repoRoot, budgetMs: 120000 });
      return;
    }
    if (segments[0] === "transaction-process-observer") {
      if (segments.length !== 1) throw new Error("Transaction process observer accepts no extra arguments");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🧪️tests/⚙️transaction-process-ownership/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "transaction-v2") {
      await runTransactionV2(this.repoRoot, segments);
      return;
    }
    const { rest } = resolveTestLevel(segments);
    await runTestBudgeted(process.execPath, ["test", "../../🧪️tests/🔬️workspace-contract/🟦️.ts", ...rest], { cwd: this.root, env: repoTestArtifactEnvironment(this.repoRoot, "workspace-contract") });
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("lint", LintScript)
  .register("test", TestScript)
  .register("go-test", GoTestScript)
  .register("workspaces", WorkspacePublicationScript);

await runBundleScriptMain(router, import.meta.url);
