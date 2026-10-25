import { createIssuedResourceControlV1 } from "@semio-tech/process/routing";
import { join } from "node:path";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { artifactIoArchitectureBreaches } from "../../🚪️io/🏛️architecture/🟦️.ts";
import { dependencyDirectionEdges, dependencyDirectionResolverRules, dependencyDirectionSourceInventory, dependencyDirectionStrictRules, type DependencyDirectionGraphScope } from "../../🕸️dependencies/🧭️direction/🟦️.ts";
import { loadDependencyDirectionPolicy } from "../../🕸️dependencies/🧭️direction/🚀️bootstrap/🟦️.ts";
import { observeDependencyDirectionV1 } from "../../🕸️dependencies/🧭️direction/🔁️observation/🟦️.ts";
import { verifyRustSourceDirection } from "../../🕸️dependencies/🧭️direction/🦀️source/🏃️execution/🟦️.ts";
import { verifyCargoDependencyDirection } from "../../🕸️dependencies/🧭️direction/🦀️cargo/🏃️execution/🟦️.ts";
import { runRepositoryCommand } from "../../🏃️process/🎛️owned-execution/🟦️.ts";
import { BundleScript, checkScriptInvocation, type ScriptInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { repoTestArtifactEnvironment } from "../../🏃️process/🌿️environment/🧪️test-output/🟦️.ts";

/** ⏱️ The wall-clock budget of the resolver child: it visits every followed source (about 19000), analyses cycles and applies every strict rule (about 1650) on a loaded shared host. */
const DEPENDENCY_RESOLUTION_BUDGET_MS = 1_200_000;

/** 🗺️ One observation of the dependency direction scope: the policy, its strict rules, the resolver roots and the independent source inventory. */
function observeDependencyDirectionScope(repoRoot: string, sourceRole?: "framework-neutral") {
  const snapshot = loadDependencyDirectionPolicy(repoRoot, { onProgress: progress => console.log(`[canonical-architecture] policy ${progress.phase}; sources=${progress.sources}`) });
  const { policy, taxonomy, workspacePackages } = snapshot, rules = dependencyDirectionStrictRules(snapshot);
  const areas = (sourceRole ? taxonomy.dependencyDirections.roles[sourceRole]!.ownerPaths : Object.keys(taxonomy.areaLayers)).filter((area) => existsSync(join(repoRoot, area)));
  const rootSources = sourceRole ? [] : readdirSync(repoRoot, { withFileTypes: true }).filter((entry) => (entry.isFile() || entry.isSymbolicLink()) && /\.(?:[cm]?[jt]s|[jt]sx)$/u.test(entry.name)).map((entry) => entry.name);
  if (!areas.some((area) => Object.entries(taxonomy.areaLayers).some(([root, layer]) => layer === "framework" && (area === root || area.startsWith(`${root}/`))))) throw new Error("Canonical architecture requires a taxonomy framework area");
  const scope: DependencyDirectionGraphScope = { workspaceRoots: Object.keys(taxonomy.areaLayers), workspacePackages, excludedPaths: policy.options.exclude.path, nonFollowedPaths: [policy.options.doNotFollow.path], expectedSources: [] };
  const expectedSources = dependencyDirectionSourceInventory(repoRoot, [...areas, ...rootSources], scope);
  if (!expectedSources.length) throw new Error("Canonical architecture has no followed TypeScript/JavaScript source inventory");
  return { policy, rules, areas, rootSources, scope, expectedSources };
}

/** 🧱️ Enforces the complete current framework graph without baselines or source-form exemptions; a source that a peer moves while the scope is walked or resolved takes a new observation (bounded), never a verdict. */
export async function verifyDependencyDirection(invocation: ScriptInvocation, repoRoot: string, env: NodeJS.ProcessEnv, sourceRole?: "framework-neutral"): Promise<void> {
  checkScriptInvocation(invocation);
  const artifactRoot = env.SEMIO_TEST_ARTIFACT_DIR!;
  mkdirSync(artifactRoot, { recursive: true });
  await observeDependencyDirectionV1(() => observeDependencyDirectionScope(repoRoot, sourceRole), async ({ policy, rules, areas, rootSources, scope, expectedSources }) => {
    const output = mkdtempSync(join(artifactRoot, "canonical-direction-"));
    try {
      const configPath = join(output, "🔣️config.json"), reportPath = join(output, "🔣️graph.json");
      writeFileSync(configPath, JSON.stringify({ forbidden: dependencyDirectionResolverRules(rules), options: policy.options }));
      const patternCount = rules.reduce((count, rule) => count + rule.from.path.length + (rule.from.pathNot?.length ?? 0) + rule.to.path.length + (rule.to.pathNot?.length ?? 0), 0);
      console.log(`[canonical-architecture] resolving present-owner TypeScript/JavaScript dependencies; scope=${sourceRole ?? "all"}; inventoriedSources=${expectedSources.length}; strictRules=${rules.length}; ownershipPatterns=${patternCount}`);
      await runRepositoryCommand(invocation,process.execPath, [join(repoRoot, "node_modules/dependency-cruiser/bin/dependency-cruise.mjs"), ...areas, ...rootSources, "--config", configPath, "--progress", "performance-log", "--output-type", "json", "--output-to", reportPath], repoRoot, "canonical-architecture", {version:1,maximumElapsedMilliseconds:DEPENDENCY_RESOLUTION_BUDGET_MS,output:{maximumBytes:134217728,maximumLines:65536}}, { env });
      const edges = dependencyDirectionEdges(JSON.parse(readFileSync(reportPath, "utf8")), rules, { ...scope, expectedSources });
      if (edges.length) throw new Error(`Canonical ${sourceRole ?? "all"} TypeScript/JavaScript architecture found ${edges.length} forbidden semantic dependencies:\n${edges.map((edge) => `${edge.rule}: ${edge.from} → ${edge.to}`).join("\n")}`);
      console.log(`[canonical-architecture] scope=${sourceRole ?? "all"} present-owner TypeScript/JavaScript dependency direction passed`);
    } catch (error) {
      const reportPath = join(output, "🔣️graph.json");
      if (existsSync(reportPath)) writeFileSync(join(artifactRoot, "canonical-direction-failure.json"), readFileSync(reportPath));
      throw error;
    } finally { rmSync(output, { recursive: true, force: true }); }
  }, { signal: invocation.control.signal });
}

export class LintScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length === 1 && segments[0] === "artifact-io-ownership") {
      let checked = 0;
      const breaches = artifactIoArchitectureBreaches(this.repoRoot, undefined, undefined, { progress: event => { if (++checked % 1000 === 0 || event.phase === "complete") console.log(`[artifact-io-ownership] ${event.phase}; checked=${checked}; ${event.path}`); } });
      if (breaches.length) throw Error(`Artifact I/O ownership found ${breaches.length} breaches:\n${breaches.map(row => `${row.kind}: ${row.scope}`).join("\n")}`);
      console.log("[artifact-io-ownership] all artifact schema and native codec ownership boundaries passed");
      return;
    }
    if (segments.length === 1 && ["styling-pixels", "styling-colors"].includes(segments[0]!)) {
      const { loadWorkspaceStylingSourceV1 } = await import("../../🎨️styling/📇️source/🟦️.ts");
      const { collectStylingViolationsV1 } = await import("@semio-tech/ui-styling/verification");
      const kind = segments[0] === "styling-pixels" ? "px" : "color";
      const violations = collectStylingViolationsV1(loadWorkspaceStylingSourceV1(this.repoRoot), kind);
      if (violations.length) throw Error(`Workspace styling found ${violations.length} ${kind} violations:\n${violations.map(row => `${row.file}:${row.line} [${row.kind}] ${row.text}`).join("\n")}`);
      console.log(`[workspace-styling] ${kind}: current source ownership passed`);
      return;
    }
    if (segments.length === 1 && segments[0] === "framework-neutral-product-direction") { await verifyDependencyDirection(this.invocation,this.repoRoot, repoTestArtifactEnvironment(this.repoRoot, "framework-neutral-product-direction"), "framework-neutral"); return; }
    if (segments.length === 1 && segments[0] === "rust-source-direction") { await verifyRustSourceDirection(this.repoRoot, join(repoTestArtifactEnvironment(this.repoRoot, "rust-source-direction").SEMIO_TEST_ARTIFACT_DIR!, "rust-source-direction.json")); return; }
    if (segments.length === 1 && segments[0] === "cargo-dependency-direction") { await verifyCargoDependencyDirection(this.repoRoot,this.invocation,createIssuedResourceControlV1(this.invocation,"cargo:direction-entry",0)); return; }
    if (segments.length !== 1 || segments[0] !== "dependency-direction") throw new Error("Expected lint dependency-direction");
    await verifyDependencyDirection(this.invocation,this.repoRoot, repoTestArtifactEnvironment(this.repoRoot, "dependency-direction"));
  }
}
