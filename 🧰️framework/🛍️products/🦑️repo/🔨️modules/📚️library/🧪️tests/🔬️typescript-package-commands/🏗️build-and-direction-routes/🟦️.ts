/** 🏗️ The build surface, dependency direction and canonical execution routes of the test script. */
import { createIssuedResourceControlV1 } from "@semio-tech/process/routing";
import { scriptInvocationBudget } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { join } from "node:path";
import { mkdirSync } from "node:fs";
import { verifyRustSourceDirection } from "../../../🕸️dependencies/🧭️direction/🦀️source/🏃️execution/🟦️.ts";
import { verifyCargoDependencyDirection } from "../../../🕸️dependencies/🧭️direction/🦀️cargo/🏃️execution/🟦️.ts";
import { runRepositoryCommand } from "../../../🏃️process/🎛️owned-execution/🟦️.ts";
import { runRepositoryTestCommand } from "../../../🧪️testing/🎛️repository-command/🟦️.ts";
import { repoTestArtifactEnvironment } from "../../../🏃️process/🌿️environment/🧪️test-output/🟦️.ts";
import { verifyDependencyDirection } from "../../../🎛️commands/🕸️dependency-lint/🟦️.ts";
import { resolveTypescriptCompilerV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import type { TestRouteContext } from "../🧭️route-context/🟦️.ts";

/** 🧱️ Runs the build surface, native dependency, canonical JSON, mutation authority, direction and corpus routes; true when one matched. */
export async function runBuildAndDirectionRoutes(context: TestRouteContext, segments: string[]): Promise<boolean> {
  if (segments.length === 1 && segments[0] === "build-surfaces") {
    await runRepositoryTestCommand(process.execPath, ["test", join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔧️build-surfaces/🟦️.ts")], { cwd: context.repoRoot, env: repoTestArtifactEnvironment(context.repoRoot, "build-surfaces"), budgetMs: 120_000 });
    return true;
  }
  if (segments.length === 1 && segments[0] === "owned-script-routes") {
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️owned-script-routes/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📂️registry-source-roots/🟦️.ts")], { cwd: context.repoRoot, env: repoTestArtifactEnvironment(context.repoRoot, "owned-script-routes"), budgetMs: 120_000 });
    return true;
  }
  if (segments[0] === "native-dependencies") {
    if (segments.length !== 1) throw new Error("Expected test native-dependencies");
    const env = repoTestArtifactEnvironment(context.repoRoot, "native-dependencies");
    mkdirSync(env.SEMIO_TEST_ARTIFACT_DIR!, { recursive: true });
    await runRepositoryTestCommand(process.execPath, [join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts"), "test", "native-dependencies"], { cwd: context.repoRoot, env });
    await runRepositoryTestCommand(process.execPath, ["test", join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🏗️native/🟦️.ts")], { cwd: context.repoRoot, env, budgetMs: scriptInvocationBudget(context.invocation, 120_000), signal: context.invocation.control.signal });
    return true;
  }
  if (segments[0] === "canonical-json") {
    if (segments.length !== 1) throw new Error("Expected test canonical-json");
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧾️canonical-json/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: context.repoRoot, env: repoTestArtifactEnvironment(context.repoRoot, "canonical-json") });
    return true;
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
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-authority/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, "--test-name-pattern", pattern], { cwd: context.repoRoot, env: repoTestArtifactEnvironment(context.repoRoot, "mutation-authority") });
    return true;
  }
  if (segments[0] === "deferred-wake-ownership") {
    if (segments.length !== 1) throw new Error("Expected test deferred-wake-ownership");
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️rust-source-direction/🔔️deferred-wake-ownership/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: context.repoRoot, env: repoTestArtifactEnvironment(context.repoRoot, "deferred-wake-ownership"), budgetMs: 15_000 });
    return true;
  }
  if (segments[0] === "pool-use-ownership") {
    if (segments.length !== 1) throw new Error("Expected test pool-use-ownership");
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️rust-source-direction/🔐️pool-use-ownership/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: context.repoRoot, env: repoTestArtifactEnvironment(context.repoRoot, "pool-use-ownership"), budgetMs: 15_000 });
    return true;
  }
  if (segments[0] === "rust-fixture-ownership") {
    if (segments.length !== 1) throw new Error("Expected test rust-fixture-ownership");
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️rust-source-direction/🧫️fixture-ownership/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, join(context.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖌️Paint2dHost/✍️editing/🧪️tests/🟦️.ts")], { cwd: context.repoRoot, env: repoTestArtifactEnvironment(context.repoRoot, "rust-fixture-ownership"), budgetMs: 15_000 });
    return true;
  }
  if (segments[0] === "rust-source-direction") {
    const roots = segments.length === 2 && segments[1] === "roots", attributes = segments.length === 2 && segments[1] === "attributes", scopes = segments.length === 2 && segments[1] === "scopes", graph = segments.length === 2 && segments[1] === "graph", participation = segments.length === 2 && segments[1] === "participation";
    if (segments.length !== 1 && !attributes && !scopes && !graph && !participation && !roots) throw new Error("Expected test rust-source-direction [attributes|scopes|graph|participation|roots]");
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️rust-source-direction", roots ? "🪵️root/🟦️.ts" : participation ? "🔗️participation/🟦️.ts" : attributes ? "🧾️attributes/🟦️.ts" : "🟦️.ts");
    const pattern = scopes ? "^finite macro scopes require every live manifest and incoming module origin$" : graph ? "^module graph authority requires every exact physical source read$" : null;
    const runtimeSources=segments.length===1?["🟦️.ts","🏘️execution/🟦️.ts"].map(path=>join(context.repoRoot,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/📁️runtime/🧪️tests",path)):[];
    await runRepositoryTestCommand(process.execPath, ["test", source, ...runtimeSources, ...(pattern ? ["--test-name-pattern", pattern] : attributes ? [] : ["--concurrent", "--max-concurrency", "4"])], { cwd: context.repoRoot, env: repoTestArtifactEnvironment(context.repoRoot, "rust-source-direction"), budgetMs: 45_000 });
    return true;
  }
  if (segments[0] === "cargo-dependency-direction") {
    if (segments.length !== 1) throw new Error("Expected test cargo-dependency-direction");
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️cargo-dependency-direction/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: context.repoRoot, env: repoTestArtifactEnvironment(context.repoRoot, "cargo-dependency-direction"), budgetMs: 45_000 });
    return true;
  }
  if (segments[0] === "compute-constructor-owner") {
    if (segments.length !== 1) throw new Error("Expected test compute-constructor-owner");
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🧩️compute-constructor/🧪️tests/🟦️.ts");
    const env = repoTestArtifactEnvironment(context.repoRoot, "compute-constructor-owner");
    await runRepositoryTestCommand(process.execPath,[resolveTypescriptCompilerV1(context.repoRoot),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--esModuleInterop","--resolveJsonModule","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],{cwd:context.repoRoot,env,budgetMs:45_000});
    await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: context.repoRoot, env, budgetMs: 45_000 });
    return true;
  }
  if (segments[0] === "dependency-direction") {
    if (segments.length !== 1) throw new Error("Expected test dependency-direction");
    const sources = ["🧪️tests/🧱️dependency-direction/🟦️.ts", "🕸️dependencies/🧭️direction/🏗️construction/🧪️tests/🟦️.ts", "🕸️dependencies/🧭️direction/🧪️tests/🧾️selection/🟦️.ts"].map(path=>join(context.repoRoot,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library",path));
    const env = repoTestArtifactEnvironment(context.repoRoot, "dependency-direction");
    await runRepositoryTestCommand(process.execPath,[resolveTypescriptCompilerV1(context.repoRoot),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--esModuleInterop","--resolveJsonModule","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",...sources],{cwd:context.repoRoot,env,budgetMs:45_000});
    await runRepositoryTestCommand(process.execPath, ["test", ...sources], { cwd: context.repoRoot, env, budgetMs: 45_000 });
    return true;
  }
  if (segments[0] === "canonical-architecture") {
    if (segments.length !== 1) throw new Error("Expected test canonical-architecture");
    await context.run(["canonical-execution"]);
    await context.run(["dependency-direction"]);
    await context.run(["cargo-dependency-direction"]);
    await context.run(["rust-source-direction"]);
    await verifyDependencyDirection(context.invocation,context.repoRoot, repoTestArtifactEnvironment(context.repoRoot, "dependency-direction"));
    await verifyRustSourceDirection(context.repoRoot);
    await verifyCargoDependencyDirection(context.repoRoot,context.invocation,createIssuedResourceControlV1(context.invocation,"cargo:direction-entry",0));
    return true;
  }
  if (segments[0] === "canonical-execution") {
    if (segments.length !== 1) throw new Error("Expected test canonical-execution");
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏛️canonical-execution/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: context.repoRoot, env: repoTestArtifactEnvironment(context.repoRoot, "canonical-execution"), budgetMs: 120_000 });
    return true;
  }
  if (segments[0] === "process-budgets") {
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/⏱️process-budgets/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: context.repoRoot, env: repoTestArtifactEnvironment(context.repoRoot, "process-budgets"), budgetMs: 30_000 });
    return true;
  }
  if (segments[0] === "exact-cargo-laws") {
    if (segments.length !== 1) throw new Error("Expected test exact-cargo-laws");
    await runRepositoryCommand(context.invocation, process.execPath, [join(context.repoRoot, "🧰️framework/🔨️modules/🏃️process/📜️script.ts"), "test", "exact-cargo-laws"], context.repoRoot, "process:exact-cargo-laws", { version: 1, maximumElapsedMilliseconds: context.invocation.policy.maximumElapsedMilliseconds, output: context.invocation.policy.maximumElapsedMilliseconds === 0 ? null : { maximumBytes: 1048576, maximumLines: 16384 } }, { env: repoTestArtifactEnvironment(context.repoRoot, "exact-cargo-laws") });
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🦀️exact-cargo-laws/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: context.repoRoot, env: repoTestArtifactEnvironment(context.repoRoot, "exact-cargo-laws") });
    return true;
  }
  if (segments[0] === "go-input-projection") {
    if (segments.length !== 1) throw new Error("Expected test go-input-projection");
    const source = join(context.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🐹️canonical-go-discovery/🟦️.ts");
    await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: context.repoRoot, budgetMs: 30_000 });
    return true;
  }
  return false;
}
