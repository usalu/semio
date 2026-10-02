#!/usr/bin/env bun
import { runOwnedCommand } from "../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
/** 🧭️ `@semio-tech/repo-lib` router: `bun ./📜️script.ts <typecheck|test [level]|workspaces <--write|--check>>`. */
import { join, resolve } from "node:path";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { dependencyDirectionEdges, dependencyDirectionSourceInventory, type DependencyDirectionGraphScope, type DependencyDirectionRule } from "../../🕸️dependencies/🧭️direction/🟦️.ts";
import { loadDependencyDirectionPolicy } from "../../🕸️dependencies/🧭️direction/🚀️bootstrap/🟦️.ts";
import { verifyRustSourceDirection } from "../../🕸️dependencies/🧭️direction/🦀️source/🏃️execution/🟦️.ts";
import { verifyCargoDependencyDirection } from "../../🕸️dependencies/🧭️direction/🦀️cargo/🏃️execution/🟦️.ts";
import { runRepositoryCommand } from "../../🏃️process/🎛️owned-execution/🟦️.ts";
import { TEST_LEVEL_BUDGET_MS, resolveTestLevel } from "../../../../../../🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { devToolingEnv, runBunx } from "./🟦️.ts";
import { repositoryProcessOwnerContextV1, repositoryVitestPolicyV1, runRepositoryTestCommand } from "../../🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { repoTestArtifactEnvironment } from "../../🏃️process/🌿️environment/🧪️test-output/🟦️.ts";
import { runTransactionV2 } from "../../🔄️transactions/🧪️verification/📋️orchestration/🟦️.ts";
import { GoTestScript } from "../../🧪️execution/🐹️go/🟦️.ts";
import { WorkspacePublicationScript } from "../../🗂️workspaces/🏃️execution/🟦️.ts";

/** 🪁️ Type-checks every `🦑️repo` product TypeScript source.
 *
 * Two programs, because the coordinator is a Next.js application: `next-env.d.ts` pulls in Next's global
 * augmentation of `NodeJS.ProcessEnv` (which makes `NODE_ENV` required), so compiling it together with the
 * repository tooling would reject every `env: { … }` literal the tooling passes to `spawnSync`. The
 * coordinator therefore keeps its own `tsconfig.json` next to its `next.config.ts`, and both are checked. */
class TypecheckScript extends BundleScript {
  run(segments: string[]): void {
    runBunx(["tsc", "--noEmit", "-p", "../../../../tsconfig.json", ...segments], this.root);
    runBunx(["tsc", "--noEmit", "-p", "../../../🖥️server/🎛️coordinator/📦️packages/🟦️typescript/tsconfig.json", ...segments], this.root);
  }
}


/** 🧱️ Enforces the complete current framework graph without baselines or source-form exemptions. */
async function verifyDependencyDirection(repoRoot: string, env: NodeJS.ProcessEnv, sourceRole?: "framework-modules"): Promise<void> {
  const { policy, taxonomy, workspacePackages } = loadDependencyDirectionPolicy(repoRoot, { onProgress: progress => console.log(`[canonical-architecture] policy ${progress.phase}; sources=${progress.sources}`) });
  const names = ["framework-no-implementation", "repo-no-implementation", "s-modules-no-plugins", ...Object.keys(taxonomy.dependencyDirections.rules)];
  const patterns = (value: string | readonly string[]): readonly string[] => typeof value === "string" ? [value] : value;
  const rules = policy.forbidden.filter((rule) => names.includes(rule.name) || rule.name.startsWith("plugin-no-extension-or-artifact-")).map((rule) => ({ ...rule, from: { path: patterns(rule.from.path!), ...(rule.from.pathNot ? { pathNot: patterns(rule.from.pathNot) } : {}) }, to: { path: patterns(rule.to.path!), ...(rule.to.pathNot ? { pathNot: patterns(rule.to.pathNot) } : {}) } }));
  if (names.some((name) => !rules.some((rule) => rule.name === name))) throw new Error("Canonical architecture is missing a declared strict rule");
  const artifactRoot = env.SEMIO_TEST_ARTIFACT_DIR!;
  mkdirSync(artifactRoot, { recursive: true });
  const output = mkdtempSync(join(artifactRoot, "canonical-direction-"));
  try {
    const configPath = join(output, "🔣️config.json"), reportPath = join(output, "🔣️graph.json");
    writeFileSync(configPath, JSON.stringify({ forbidden: rules, options: policy.options }));
    const areas = (sourceRole ? taxonomy.dependencyDirections.roles[sourceRole]!.ownerPaths : Object.keys(taxonomy.areaLayers)).filter((area) => existsSync(join(repoRoot, area)));
    const rootSources = sourceRole ? [] : readdirSync(repoRoot, { withFileTypes: true }).filter((entry) => (entry.isFile() || entry.isSymbolicLink()) && /\.(?:[cm]?[jt]s|[jt]sx)$/u.test(entry.name)).map((entry) => entry.name);
    if (!areas.some((area) => Object.entries(taxonomy.areaLayers).some(([root, layer]) => layer === "framework" && (area === root || area.startsWith(`${root}/`))))) throw new Error("Canonical architecture requires a taxonomy framework area");
    const scope: DependencyDirectionGraphScope = { workspaceRoots: Object.keys(taxonomy.areaLayers), workspacePackages, excludedPaths: policy.options.exclude.path, nonFollowedPaths: [policy.options.doNotFollow.path], expectedSources: [] };
    const expectedSources = dependencyDirectionSourceInventory(repoRoot, [...areas, ...rootSources], scope);
    if (!expectedSources.length) throw new Error("Canonical architecture has no followed TypeScript/JavaScript source inventory");
    const patternCount = rules.reduce((count, rule) => count + rule.from.path.length + (rule.from.pathNot?.length ?? 0) + rule.to.path.length + (rule.to.pathNot?.length ?? 0), 0);
    console.log(`[canonical-architecture] resolving present-owner TypeScript/JavaScript dependencies; scope=${sourceRole ?? "all"}; inventoriedSources=${expectedSources.length}; strictRules=${rules.length}; ownershipPatterns=${patternCount}`);
    await runRepositoryCommand(process.execPath, [join(repoRoot, "node_modules/dependency-cruiser/bin/dependency-cruise.mjs"), ...areas, ...rootSources, "--config", configPath, "--progress", "performance-log", "--output-type", "json", "--output-to", reportPath], repoRoot, "canonical-architecture", 240_000, { env });
    const edges = dependencyDirectionEdges(JSON.parse(readFileSync(reportPath, "utf8")), rules, { ...scope, expectedSources });
    if (edges.length) throw new Error(`Canonical ${sourceRole ?? "all"} TypeScript/JavaScript architecture found ${edges.length} forbidden semantic dependencies:\n${edges.map((edge) => `${edge.rule}: ${edge.from} → ${edge.to}`).join("\n")}`);
    console.log(`[canonical-architecture] scope=${sourceRole ?? "all"} present-owner TypeScript/JavaScript dependency direction passed`);
  } catch (error) {
    const reportPath = join(output, "🔣️graph.json");
    if (existsSync(reportPath)) writeFileSync(join(artifactRoot, "canonical-direction-failure.json"), readFileSync(reportPath));
    throw error;
  } finally { rmSync(output, { recursive: true, force: true }); }
}

class LintScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length === 1 && ["styling-pixels", "styling-colors"].includes(segments[0]!)) {
      const { loadWorkspaceStylingSourceV1 } = await import("../../🎨️styling/📇️source/🟦️.ts");
      const { collectStylingViolationsV1 } = await import("../../../../../../🔨️modules/🖱️ui/🎨️styling/🛡️verification/🟦️.ts");
      const kind = segments[0] === "styling-pixels" ? "px" : "color";
      const violations = collectStylingViolationsV1(loadWorkspaceStylingSourceV1(this.repoRoot), kind);
      if (violations.length) throw Error(`Workspace styling found ${violations.length} ${kind} violations:\n${violations.map(row => `${row.file}:${row.line} [${row.kind}] ${row.text}`).join("\n")}`);
      console.log(`[workspace-styling] ${kind}: current source ownership passed`);
      return;
    }
    if (segments.length === 1 && segments[0] === "framework-module-product-direction") { await verifyDependencyDirection(this.repoRoot, repoTestArtifactEnvironment(this.repoRoot, "framework-module-product-direction"), "framework-modules"); return; }
    if (segments.length === 1 && segments[0] === "rust-source-direction") { await verifyRustSourceDirection(this.repoRoot, join(repoTestArtifactEnvironment(this.repoRoot, "rust-source-direction").SEMIO_TEST_ARTIFACT_DIR!, "rust-source-direction.json")); return; }
    if (segments.length === 1 && segments[0] === "cargo-dependency-direction") { await verifyCargoDependencyDirection(this.repoRoot); return; }
    if (segments.length !== 1 || segments[0] !== "dependency-direction") throw new Error("Expected lint dependency-direction");
    await verifyDependencyDirection(this.repoRoot, repoTestArtifactEnvironment(this.repoRoot, "dependency-direction"));
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "nx-project-inference") {
      const revision = segments.length === 2 && segments[1] === "revision";
      const imports = segments.length === 2 && segments[1] === "imports";
      if (!revision && !imports && segments.length !== 1) throw Error("Expected test nx-project-inference [revision|imports]");
      if (!process.env.SEMIO_TEST_ARTIFACT_DIR) throw Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned ticket output");
      if (revision) {
        const output = repoTestArtifactEnvironment(this.repoRoot, "nx-project-inference-revision").SEMIO_TEST_ARTIFACT_DIR!;
        mkdirSync(output, { recursive: true });
        const { testGraphRevision } = await import("../../⚡️caching/🧪️tests/🔁️graph-revision/🟦️.ts");
        await testGraphRevision(this.repoRoot, output);
        console.log("[nx-project-inference] original Node/Bun graph revision laws passed");
        return;
      }
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📥️inference/🧪️tests", imports ? "🔍️imports/🟦️.ts" : "🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot, env: repoTestArtifactEnvironment(this.repoRoot, "nx-project-inference"), budgetMs: 30_000 });
      return;
    }
    if (segments[0] === "rust-family-ownership") {
      const native = segments.length === 2 && segments[1] === "native";
      if (!native && segments.length !== 1) throw Error("Expected test rust-family-ownership [native]");
      if (!process.env.SEMIO_TEST_ARTIFACT_DIR) throw Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned ticket output");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/📍️ownership/🧪️tests", native ? "🦀️native/🟦️.ts" : "🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot, env: repoTestArtifactEnvironment(this.repoRoot, "rust-family-ownership"), budgetMs: 45_000 });
      return;
    }
    if (segments[0] === "rust-binding") {
      const native = segments.length === 2 && segments[1] === "native";
      if (!native && segments.length !== 1) throw Error("Expected test rust-binding [native]");
      if (!process.env.SEMIO_TEST_ARTIFACT_DIR) throw Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned ticket output");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🔗️binding/🧪️tests", native ? "🦀️native/🟦️.ts" : "🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot, env: repoTestArtifactEnvironment(this.repoRoot, native ? "rust-binding-native" : "rust-binding"), budgetMs: 45_000 });
      return;
    }
    if (segments[0] === "dependency-policy-bootstrap") {
      if (segments.length !== 1) throw Error("Expected test dependency-policy-bootstrap");
      if (!process.env.SEMIO_TEST_ARTIFACT_DIR) throw Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned ticket output");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🚀️bootstrap/🧪️tests/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot, env: repoTestArtifactEnvironment(this.repoRoot, "dependency-policy-bootstrap"), budgetMs: 45_000 });
      return;
    }
    if (segments[0] === "native-owner-command-policy") {
      if (segments.length !== 1) throw Error("Expected test native-owner-command-policy");
      if (!process.env.SEMIO_TEST_ARTIFACT_DIR) throw Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned ticket output");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🧪️tests/📋️owner-command-policy/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot, env: repoTestArtifactEnvironment(this.repoRoot, "native-owner-command-policy") });
      return;
    }
    if (segments[0] === "locale-law-ownership") {
      if (segments.length !== 1) throw new Error("Expected test locale-law-ownership");
      await runRepositoryTestCommand(process.execPath, ["test", join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️locale-law-ownership/🟦️.ts")], { cwd: this.repoRoot, budgetMs: TEST_LEVEL_BUDGET_MS.fundamental });
      return;
    }
    if (segments[0] === "fixture-law-ownership") {
      if (segments.length !== 1) throw new Error("Expected test fixture-law-ownership");
      await runRepositoryTestCommand(process.execPath, ["test", join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️fixture-law-ownership/🟦️.ts")], { cwd: this.repoRoot, budgetMs: TEST_LEVEL_BUDGET_MS.fundamental });
      return;
    }
    if (segments[0] === "ui-router-ownership") {
      if (segments.length !== 1) throw new Error("Expected test ui-router-ownership");
      await runRepositoryTestCommand(process.execPath, ["test", join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🖱️ui/🧪️tests/🧭️router-ownership/🟦️.ts")], { cwd: this.repoRoot, budgetMs: TEST_LEVEL_BUDGET_MS.fundamental });
      return;
    }
    if (segments[0] === "native-source-ownership") {
      if (segments.length !== 1) throw new Error("Expected test native-source-ownership");
      await runRepositoryTestCommand(process.execPath, ["test", join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/⚙️native-source-ownership/🟦️.ts"), join(this.repoRoot, "🧰️framework/🔨️modules/🗺️surface/🧪️tests/🧩️suite/🟦️.ts")], { cwd: this.repoRoot, budgetMs: TEST_LEVEL_BUDGET_MS.fundamental });
      return;
    }
    if (segments.length === 1 && segments[0] === "owned-script-routes") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️owned-script-routes/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📂️registry-source-roots/🟦️.ts")], { cwd: this.repoRoot, env: repoTestArtifactEnvironment(this.repoRoot, "owned-script-routes"), budgetMs: 120_000 });
      return;
    }
    if (segments[0] === "native-dependencies") {
      if (segments.length !== 1) throw new Error("Expected test native-dependencies");
      const { testNativeDependencies } = await import("../../⚡️caching/🧪️tests/📦️native-dependencies/🟦️.ts");
      const env = repoTestArtifactEnvironment(this.repoRoot, "native-dependencies");
      mkdirSync(env.SEMIO_TEST_ARTIFACT_DIR!, { recursive: true });
      await testNativeDependencies(this.repoRoot, env.SEMIO_TEST_ARTIFACT_DIR!);
      console.log("[DEBUG] native-dependencies: neutral environment vectors and independent bundler/schema/Cargo/Nx oracles passed");
      return;
    }
    if (segments[0] === "canonical-json") {
      if (segments.length !== 1) throw new Error("Expected test canonical-json");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧾️canonical-json/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot, env: repoTestArtifactEnvironment(this.repoRoot, "canonical-json") });
      return;
    }
    if (segments[0] === "mutation-authority") {
      const patterns: Readonly<Record<string, string>> = {
        scaffolding: "prepares AST-safe direct mutation scaffolds before one guarded publication$",
        inventory: "resolves mutation consumers and schema-validated assignment evidence from a stable source index$",
        reachability: "proves direct leaf reachability through exact public canonical mounts and wrapped types$",
        "type-origin": "projects the actual wrapped mutation declaration origin through public aliases only$",
      };
      const pattern = segments.length === 2 ? patterns[segments[1]!] : undefined;
      if (!pattern) throw new Error("Expected test mutation-authority <scaffolding|inventory|reachability|type-origin>");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-authority/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, "--test-name-pattern", pattern], { cwd: this.repoRoot, env: repoTestArtifactEnvironment(this.repoRoot, "mutation-authority") });
      return;
    }
    if (segments[0] === "deferred-wake-ownership") {
      if (segments.length !== 1) throw Error("Expected test deferred-wake-ownership");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️rust-source-direction/🔔️deferred-wake-ownership/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot, env: repoTestArtifactEnvironment(this.repoRoot, "deferred-wake-ownership"), budgetMs: 15_000 });
      return;
    }
    if (segments[0] === "pool-use-ownership") {
      if (segments.length !== 1) throw Error("Expected test pool-use-ownership");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️rust-source-direction/🔐️pool-use-ownership/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot, env: repoTestArtifactEnvironment(this.repoRoot, "pool-use-ownership"), budgetMs: 15_000 });
      return;
    }
    if (segments[0] === "rust-fixture-ownership") {
      if (segments.length !== 1) throw Error("Expected test rust-fixture-ownership");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️rust-source-direction/🧫️fixture-ownership/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖌️Paint2dHost/✍️editing/🧪️tests/🟦️.ts")], { cwd: this.repoRoot, env: repoTestArtifactEnvironment(this.repoRoot, "rust-fixture-ownership"), budgetMs: 15_000 });
      return;
    }
    if (segments[0] === "rust-source-direction") {
      const roots = segments.length === 2 && segments[1] === "roots", attributes = segments.length === 2 && segments[1] === "attributes", scopes = segments.length === 2 && segments[1] === "scopes", graph = segments.length === 2 && segments[1] === "graph", participation = segments.length === 2 && segments[1] === "participation";
      if (segments.length !== 1 && !attributes && !scopes && !graph && !participation && !roots) throw new Error("Expected test rust-source-direction [attributes|scopes|graph|participation|roots]");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️rust-source-direction", roots ? "🪵️root/🟦️.ts" : participation ? "🔗️participation/🟦️.ts" : attributes ? "🧾️attributes/🟦️.ts" : "🟦️.ts");
      const pattern = scopes ? "^finite macro scopes require every live manifest and incoming module origin$" : graph ? "^module graph authority requires every exact physical source read$" : null;
      await runRepositoryTestCommand(process.execPath, ["test", source, ...(pattern ? ["--test-name-pattern", pattern] : attributes ? [] : ["--concurrent", "--max-concurrency", "4"])], { cwd: this.repoRoot, env: repoTestArtifactEnvironment(this.repoRoot, "rust-source-direction"), budgetMs: 45_000 });
      return;
    }
    if (segments[0] === "cargo-dependency-direction") {
      if (segments.length !== 1) throw new Error("Expected test cargo-dependency-direction");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️cargo-dependency-direction/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot, env: repoTestArtifactEnvironment(this.repoRoot, "cargo-dependency-direction"), budgetMs: 45_000 });
      return;
    }
    if (segments[0] === "dependency-direction") {
      if (segments.length !== 1) throw new Error("Expected test dependency-direction");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️dependency-direction/🟦️.ts");
      const env = repoTestArtifactEnvironment(this.repoRoot, "dependency-direction");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot, env, budgetMs: 45_000 });
      return;
    }
    if (segments[0] === "canonical-architecture") {
      if (segments.length !== 1) throw new Error("Expected test canonical-architecture");
      await this.run(["canonical-execution"]);
      await this.run(["dependency-direction"]);
      await this.run(["cargo-dependency-direction"]);
      await this.run(["rust-source-direction"]);
      await verifyDependencyDirection(this.repoRoot, repoTestArtifactEnvironment(this.repoRoot, "dependency-direction"));
      await verifyRustSourceDirection(this.repoRoot);
      await verifyCargoDependencyDirection(this.repoRoot);
      return;
    }
    if (segments[0] === "canonical-execution") {
      if (segments.length !== 1) throw new Error("Expected test canonical-execution");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏛️canonical-execution/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot, env: repoTestArtifactEnvironment(this.repoRoot, "canonical-execution"), budgetMs: 120_000 });
      return;
    }
    if (segments[0] === "process-budgets") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/⏱️process-budgets/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot, env: repoTestArtifactEnvironment(this.repoRoot, "process-budgets"), budgetMs: 30_000 });
      return;
    }
    if (segments[0] === "exact-cargo-laws") {
      if (segments.length !== 1) throw new Error("Expected test exact-cargo-laws");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🦀️exact-cargo-laws/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot, env: repoTestArtifactEnvironment(this.repoRoot, "exact-cargo-laws") });
      return;
    }
    if (segments[0] === "go-input-projection") {
      if (segments.length !== 1) throw new Error("Expected test go-input-projection");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🐹️canonical-go-discovery/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot, budgetMs: 30_000 });
      return;
    }
    if (segments[0] === "go-test-dispatch") {
      if (segments.length !== 1) throw new Error("Expected test go-test-dispatch");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🚦️test-dispatch/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot, budgetMs: 70_000 });
      return;
    }
    if (segments[0] === "generated-source-topology") {
      if (segments.length !== 1) throw new Error("Expected test generated-source-topology");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏭️generated-source-topology/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "package-body-policy") {
      if (segments.length !== 1) throw new Error("Expected test package-body-policy");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/📦️package-boundary-classification/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot, budgetMs: 240_000 });
      return;
    }
    if (segments[0] === "cross-platform-bootstrap") {
      if (segments.length !== 1) throw new Error("Expected test cross-platform-bootstrap");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🥾️cross-platform-bootstrap/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot, budgetMs: 120_000 });
      return;
    }
    if (segments[0] === "windows-command-paths") {
      if (segments.length !== 1) throw new Error("Expected test windows-command-paths");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪟️windows-command-paths/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot, budgetMs: 120_000 });
      return;
    }
    if (segments[0] === "kind-only-basename") {
      if (segments.length !== 1) throw new Error("Expected test kind-only-basename");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🌳️kind-only-basename/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot, budgetMs: 45_000 });
      return;
    }
    if (segments[0] === "framework-source-topology") {
      if (segments.length !== 1) throw new Error("Expected test framework-source-topology");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️framework-source-topology/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot, budgetMs: 30_000 });
      return;
    }
    if (segments[0] === "root-artifact-dependency-source") {
      if (segments.length !== 1) throw new Error("Expected test root-artifact-dependency-source");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️root-artifact-dependency-source/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot, budgetMs: 45_000 });
      return;
    }
    if (segments[0] === "root-taxonomy-workflow-source") {
      if (segments.length !== 1) throw new Error("Expected test root-taxonomy-workflow-source");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️root-taxonomy-workflow-source/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot, budgetMs: 45_000 });
      return;
    }
    if (segments[0] === "root-schema-field-source") {
      if (segments.length !== 1) throw new Error("Expected test root-schema-field-source");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️root-schema-field-source/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot, budgetMs: 45_000 });
      return;
    }
    if (segments[0] === "root-clean-scaffold-source") {
      if (segments.length !== 1) throw new Error("Expected test root-clean-scaffold-source");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️root-clean-scaffold-source/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot, budgetMs: 45_000 });
      return;
    }
    if (segments[0] === "root-artifact-schema-law-source") {
      if (segments.length !== 1) throw new Error("Expected test root-artifact-schema-law-source");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️root-artifact-schema-law-source/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot, env: repoTestArtifactEnvironment(this.repoRoot, "root-artifact-schema-law-source"), budgetMs: 45_000 });
      return;
    }
    if (segments[0] === "root-surface-abstraction-law-source") {
      if (segments.length !== 1) throw new Error("Expected test root-surface-abstraction-law-source");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️root-surface-abstraction-law-source/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot, budgetMs: 60_000 });
      return;
    }
    if (segments[0] === "root-inference-law-source") {
      if (segments.length !== 1) throw new Error("Expected test root-inference-law-source");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️root-inference-law-source/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot, env: repoTestArtifactEnvironment(this.repoRoot, "root-inference-law-source"), budgetMs: 45_000 });
      return;
    }
    if (segments[0] === "cargo-transaction-command-source") {
      if (segments.length !== 1) throw new Error("Expected test cargo-transaction-command-source");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️cargo-transaction-command-source/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot, env: repoTestArtifactEnvironment(this.repoRoot, "cargo-transaction-command-source"), budgetMs: 45_000 });
      return;
    }
    if (segments[0] === "workspace-publication-source") {
      if (segments.length !== 1) throw new Error("Expected test workspace-publication-source");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️workspace-publication-source/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot, budgetMs: 45_000 });
      return;
    }
    if (segments[0] === "wasm-package-wrappers") {
      if (segments.length !== 1) throw new Error("Expected test wasm-package-wrappers");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️wasm-package-wrappers/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot, env: repoTestArtifactEnvironment(this.repoRoot, "wasm-package-wrappers"), budgetMs: 45_000 });
      return;
    }
    if (segments[0] === "framework-root-source-topology") {
      if (segments.length !== 1) throw new Error("Expected test framework-root-source-topology");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧰️framework-root-source-topology/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot, budgetMs: 30_000 });
      return;
    }
    if (segments[0] === "manifestless-source-closure") {
      if (segments.length !== 1) throw new Error("Expected test manifestless-source-closure");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️manifestless-source-closure/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot, budgetMs: 30_000 });
      return;
    }
    if (segments[0] === "plugin-publication-source-ownership") {
      if (segments.length !== 1) throw new Error("Expected test plugin-publication-source-ownership");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📣️plugin-publication-source-ownership/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot, budgetMs: 30_000 });
      return;
    }
    if (segments[0] === "app-verification-source-ownership") {
      if (segments.length !== 1) throw new Error("Expected test app-verification-source-ownership");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📱️app-verification-source-ownership/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot, budgetMs: 30_000 });
      return;
    }
    if (segments[0] === "vitest-configuration-ownership") {
      if (segments.length !== 1) throw new Error("Expected test vitest-configuration-ownership");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎚️vitest-configuration-ownership/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot, budgetMs: TEST_LEVEL_BUDGET_MS.long });
      return;
    }
    if (segments[0] === "tool-configuration-ownership") {
      if (segments.length !== 1) throw new Error("Expected test tool-configuration-ownership");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎚️tool-configuration-ownership/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot, budgetMs: 120_000 });
      return;
    }
    if (segments[0] === "os-dev-composition-ownership") {
      if (segments.length !== 1) throw new Error("Expected test os-dev-composition-ownership");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧑‍💻os-dev-composition-ownership/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot, budgetMs: 30_000 });
      return;
    }
    if (segments[0] === "repo-source-ownership") {
      if (segments.length !== 1) throw new Error("Expected test repo-source-ownership");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🦑️repo-source-ownership/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot, budgetMs: 30_000 });
      return;
    }
    if (segments[0] === "storybook-discovery") {
      if (segments.length !== 1) throw new Error("Expected test storybook-discovery");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️storybook-discovery/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "os-source-topology") {
      if (segments.length !== 1) throw new Error("Expected test os-source-topology");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖥️os-source-topology/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "path-emoji-statutes") {
      if (segments.length !== 1) throw new Error("Expected test path-emoji-statutes");
      resolveTestLevel(["long"]);
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔏️path-emoji-statutes/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "mutation-ticket-role-routing") {
      if (segments.length !== 1) throw new Error("Expected test mutation-ticket-role-routing");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎫️ticket-role-routing/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "mutation-source-index-capture") {
      if (segments.length !== 1) throw new Error("Expected test mutation-source-index-capture");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📸️source-index-capture/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "mutation-source-roster-roles") {
      if (segments.length !== 1) throw new Error("Expected test mutation-source-roster-roles");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎭️source-roster-roles/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "mutation-source-file-facts") {
      if (segments.length > 2 || (segments[1] !== undefined && segments[1] !== "reference")) throw new Error("Expected test mutation-source-file-facts [reference]");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧾️source-file-facts/🟦️.ts");
      const selection = segments[1] === "reference" ? ["-t", "^mutation source-file facts (vectors|independent suffix reference|reference oracle)"] : [];
      await runRepositoryTestCommand(process.execPath, ["test", source, ...selection], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "typescript-declaration-facts") {
      if (segments.length > 2 || (segments[1] !== undefined && segments[1] !== "reference")) throw new Error("Expected test typescript-declaration-facts [reference]");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📣️typescript-declaration-facts/🟦️.ts");
      const selection = segments[1] === "reference" ? ["-t", "^TypeScript (?:(?:malformed|unsupported) )?declaration (?:reference:|(?:facts|cases) use the closed neutral schema)"] : [];
      await runRepositoryTestCommand(process.execPath, ["test", source, ...selection], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "artifact-support") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🍃️artifact-support-leaf-authority/🟦️.ts");
      const { rest } = resolveTestLevel(segments.slice(1));
      await runRepositoryTestCommand(process.execPath, ["test", source, ...rest], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "historical-package-owner-identity") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏺️historical-package-owner-identity/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "cargo-provider-binding-trace") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪢️cargo-provider-binding/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "metadata-source-provider") {
      const { rest } = resolveTestLevel(segments.slice(1));
      await runRepositoryTestCommand(process.execPath, ["test", "../../🧪️tests/🔬️workspace-contract/🟦️.ts", "-t", "mutation metadata source provider", ...rest], { cwd: this.root });
      return;
    }
    if (segments[0] === "rust-physical-reference-context") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧲️rust-physical-reference-context/🟦️.ts");
      const { rest } = resolveTestLevel(segments.slice(1));
      await runRepositoryTestCommand(process.execPath, ["test", source, ...rest], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "taxonomy-cli-cancellation") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🛑️taxonomy-cli-cancellation/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot, env: repoTestArtifactEnvironment(this.repoRoot, "taxonomy-cli-cancellation") });
      return;
    }
    if (segments[0] === "inventory-artifact-shards") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/💠️inventory-artifact-shards/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "root-script-compiler") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/⚙️root-script-compiler/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "json-reference-owner-lookup") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔎️json-reference-owner-lookup/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "nx-workspace-root-file-reference") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏠️nx-workspace-root-file-reference/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "bare-reference-sibling-precedence") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🥇️bare-reference-sibling-precedence/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "run-vitest-config-argument-tokens") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏃️run-vitest-config-argument-tokens/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "cargo-discovery-exclusions") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🚧️cargo-discovery-exclusions/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "nested-cargo-collision-authority") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/💥️nested-cargo-collision-authority/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "registry-import-language") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🌐️registry-import-language/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "preflight-reference-basis") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🛫️preflight-reference-basis/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "typescript-path-collection") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🛤️typescript-path-collection/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "frozen-markdown-coordinates") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/❄️frozen-markdown-coordinates/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "historical-document-evidence") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📜️historical-document-evidence/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "cargo-target-discovery-skip") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎯️cargo-target-discovery-skip/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "registry-catalog-gitlink-boundary") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧾️registry-catalog-gitlink-boundary/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "historical-json-source-encoding") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🕰️historical-json-source-encoding/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "transaction-recovery-authority") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🛟️transaction-recovery-authority/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "transaction-fixture-key-exactness") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🗝️transaction-fixture-key-exactness/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "testing-readme-coordinates") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🗺️testing-readme-coordinates/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "rust-finite-target-consumption") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🥤️rust-finite-target-consumption/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "readme-current-source-revision") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔖️readme-current-source-revision/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "rust-writable-path-authority") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/✍️rust-writable-path-authority/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "readme-move-source-authority") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🚚️readme-move-source-authority/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "artifact-empty-facet-authority") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🫙️artifact-empty-facet-authority/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "rust-divergence-callback-source") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/↪️rust-divergence-callback/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, "--test-name-pattern", "^(closed divergence|shared callback|candidate-only callback)", ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "rust-divergence-callback-native") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/↪️rust-divergence-callback/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, "--test-name-pattern", "^actual rustc", ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "rust-divergence-callback-syn") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/↪️rust-divergence-callback/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, "--test-name-pattern", "^independent syn callback", ...segments.slice(1)], { cwd: this.repoRoot, budgetMs: 120_000 });
      return;
    }
    if (segments[0] === "readme-current-source-activation") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🟢️readme-current-source-activation/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot, env: repoTestArtifactEnvironment(this.repoRoot, "readme-current-source-activation") });
      return;
    }
    if (segments[0] === "artifact-empty-facet-authoring") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪶️artifact-empty-facet-authoring/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot, env: repoTestArtifactEnvironment(this.repoRoot, "artifact-empty-facet-authoring"), budgetMs: 30_000 });
      return;
    }
    if (segments[0] === "readme-reviewed-fixture-inputs") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/👀️readme-reviewed-fixture-inputs/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "taxonomy-pattern-compiler-reuse") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/♻️taxonomy-pattern-compiler-reuse/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "source-services") {
      if (segments.length !== 1) throw new Error("Expected test source-services");
      const tests = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests");
      await runRepositoryTestCommand(process.execPath, ["test", ...["🏗️source-services", "🚪️source-admission", "🚪️source-admission-io", "🔣️taxonomy-input"].map((facet) => join(tests, facet, "🟦️.ts"))], { cwd: this.repoRoot, env: repoTestArtifactEnvironment(this.repoRoot, "source-services") });
      return;
    }
    if (segments[0] === "reference-coverage-selection") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎟️reference-coverage-selection/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "taxonomy-leading-grapheme") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔤️taxonomy-leading-grapheme/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "reference-coordinate-progress") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📈️reference-coordinate-progress/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "draw-destination-observation") {
      if (segments.length !== 1) throw new Error("Draw destination observation accepts no extra arguments");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📍️draw-destination-observation/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "markdown-inline-references") {
      if (segments.length !== 1) throw new Error("Markdown inline references accepts no extra arguments");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔗️markdown-inline-references/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "gherkin-description-inline-code") {
      if (segments.length !== 1) throw new Error("Gherkin description inline code accepts no extra arguments");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🥒️gherkin-description-inline-code/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "composition-policy") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, "-t", "composition policy"], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "artifact-source-residue") {
      if (segments.length !== 1) throw new Error("Artifact source residue accepts no extra arguments");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, "--timeout", "120000", "-t", "rejects ignored and unplanned residual children in a projected source owner without following links"], { cwd: this.repoRoot, budgetMs: 120000, env: { ...process.env, SEMIO_TEST_LEVEL: "long" } });
      return;
    }
    if (segments[0] === "artifact-source-commit") {
      if (segments.length !== 1) throw new Error("Artifact source commit accepts no extra arguments");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, "--timeout", "120000", "-t", "rolls back and atomically applies CAD and Draw projections to an empty second plan"], { cwd: this.repoRoot, budgetMs: 120000, env: { ...process.env, SEMIO_TEST_LEVEL: "long" } });
      return;
    }
    if (segments[0] === "transaction-process-observer") {
      if (segments.length !== 1) throw new Error("Transaction process observer accepts no extra arguments");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🧪️tests/⚙️transaction-process-ownership/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "transaction-v2") {
      await runTransactionV2(this.repoRoot, segments);
      return;
    }
    if (segments[0] === "marker-only-folders") {
      if (segments.length !== 1) throw new Error("Expected test marker-only-folders");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️marker-only-folders/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "empty-folders") {
      if (segments.length !== 1) throw new Error("Expected test empty-folders");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️empty-folders/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "mutation-case-pair") {
      if (segments.length !== 1) throw new Error("Expected test mutation-case-pair");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-case-pair/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, "--timeout", "240000"], { cwd: this.repoRoot, budgetMs: 480_000 });
      return;
    }
    if (segments[0] === "mutation-wire-witness") {
      if (segments.length !== 1) throw new Error("Expected test mutation-wire-witness");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️mutation-wire-witness/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, "--timeout", "240000"], { cwd: this.repoRoot, budgetMs: 480_000 });
      return;
    }
    if (segments[0] === "mutation-leaf-identity") {
      if (segments.length !== 1) throw new Error("Expected test mutation-leaf-identity");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️mutation-leaf-identity/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, "--timeout", "240000"], { cwd: this.repoRoot, budgetMs: 480_000 });
      return;
    }
    const { level, rest } = resolveTestLevel(segments);
    const lawTimeout = level === "long" || level === "exhaustive" ? ["--timeout", String(TEST_LEVEL_BUDGET_MS[level])] : [];
    await runRepositoryTestCommand(process.execPath, ["test", "../../🧪️tests/🔬️workspace-contract/🟦️.ts", ...lawTimeout, ...rest], { cwd: this.root, env: repoTestArtifactEnvironment(this.repoRoot, "workspace-contract") });
  }
}

/** 🧪️ Supplies repository-owned tool and cache policy to a selected owner's test command. */
class OwnerCommandScript extends BundleScript{
  async run(args:string[]):Promise<void>{
    if(args[0]!=="--cwd"||!args[1]||args[2]!=="--"||!args[3])throw Error("owner-command --cwd <directory> -- <command> <args>");
    const cwd=resolve(this.repoRoot,args[1]),policy=repositoryVitestPolicyV1(cwd),env=devToolingEnv({SEMIO_VITEST_POLICY:JSON.stringify(policy),SEMIO_PROCESS_OWNER_CONTEXT:JSON.stringify(repositoryProcessOwnerContextV1(cwd))});
    await runOwnedCommand(args[3],args.slice(4),cwd,"process:owner-command",0,{env});
  }
}

/** 🖱️ Dispatches the actual repository UI owner. */
class RepositoryUiScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { runOwnedCommand } = await import("../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts");
    await runOwnedCommand(process.execPath, [join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🖱️ui/📜️script.ts"), ...segments], this.repoRoot, "repo-ui", 0, { env: process.env });
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("owner-command", OwnerCommandScript)
  .register("typecheck", TypecheckScript)
  .register("lint", LintScript)
  .register("test", TestScript)
  .register("go-test", GoTestScript)
  .register("workspaces", WorkspacePublicationScript)
  .register("ui", RepositoryUiScript);

await runScriptMain(router);
