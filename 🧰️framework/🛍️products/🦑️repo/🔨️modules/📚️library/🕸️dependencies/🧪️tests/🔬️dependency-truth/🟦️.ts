import { getWorkspaceRoot } from "../../../🗂️workspaces/🟦️.ts";
import { dependencyClassifyOracleEntry, dependencyIsCompositionManifest, dependencyParseGoModule, type DependencyBaselineEntry, type DependencyEcosystem, type DependencyKind } from "../../📇️inventory/🟦️.ts";
import { DEPENDENCY_REPO_POLICY_ROOT, DEPENDENCY_REPO_POLICY_ROUTERS, dependencyRepoPolicyImportBoundaryFailure, dependencyRepoPolicyLibrarySpecifier, dependencyRepoPolicyRouterSetFailure, dependencyTruthReportFromEntries } from "../../⚖️truth/🟦️.ts";

const WORKSPACE_ROOT = getWorkspaceRoot();

/** 🧪️Hostile source-only fixtures for every correction and exception in the literal-external census. */
export function dependencyTruthSelfTests(): number {
  if (!dependencyIsCompositionManifest("compose/client/Cargo.toml") || !dependencyIsCompositionManifest("temp/compose/client/Cargo.toml") || dependencyIsCompositionManifest("temp/owned/Cargo.toml"))
    throw new Error("[verify dependencies self-test] canonical or generated composition manifests escaped the declared out-of-scope boundary.");
  const rootRouter = DEPENDENCY_REPO_POLICY_ROUTERS[0];
  const boundarySource = `import { defineLint, runPolicyOnlyMain, type TechnologyLinter } from "${dependencyRepoPolicyLibrarySpecifier(WORKSPACE_ROOT, rootRouter)}";`;
  const missingBoundary = dependencyRepoPolicyImportBoundaryFailure(WORKSPACE_ROOT, rootRouter, boundarySource, () => false);
  if (!missingBoundary?.includes("imports missing owned module")) throw new Error("[verify dependencies self-test] a moved repo-owned policy module lacked the owned import-boundary diagnostic.");
  const staleBoundary = dependencyRepoPolicyImportBoundaryFailure(WORKSPACE_ROOT, rootRouter, 'import { defineLint } from "../../removed/index.ts";', () => true);
  if (!staleBoundary?.includes("must import its policy APIs from owned module")) throw new Error("[verify dependencies self-test] a stale repo policy import escaped the stable owned-boundary diagnostic.");
  if (!dependencyRepoPolicyRouterSetFailure(DEPENDENCY_REPO_POLICY_ROUTERS.filter((script) => !script.includes("/💻️client/")) as string[])?.includes("missing=")) throw new Error("[verify dependencies self-test] a missing enumerated repo policy router escaped the owned-boundary diagnostic.");
  if (!dependencyRepoPolicyRouterSetFailure([...DEPENDENCY_REPO_POLICY_ROUTERS, `${DEPENDENCY_REPO_POLICY_ROOT}/unowned/📜️script.ts`])?.includes("unenumerated=")) throw new Error("[verify dependencies self-test] a newly discovered repo policy router escaped enumeration.");
  const firstModule = dependencyParseGoModule("module github.com/example/first\n");
  const consumer = dependencyParseGoModule("module github.com/example/consumer\nrequire (\n github.com/example/first v0.0.0\n example.net/external v1.2.3 // indirect\n)\nreplace github.com/example/replaced => ../replaced\nrequire github.com/example/replaced v0.0.0\n");
  const internalGo = new Set([firstModule.module!, consumer.module!, ...consumer.localReplaces]);
  const goExternal = consumer.requirements.filter((entry) => !internalGo.has(entry.name));
  if (goExternal.some((entry) => entry.name === "github.com/example/first") || goExternal.some((entry) => entry.name === "github.com/example/replaced")) throw new Error("[verify dependencies self-test] first-party Go module or local replace was retained as external.");
  if (!goExternal.some((entry) => entry.name === "example.net/external" && entry.kind === "production-build")) throw new Error("[verify dependencies self-test] external indirect Go requirement was not retained as production build input.");
  const oracleRuntime: DependencyBaselineEntry = { ecosystem: "rust", name: "runtime-oracle-name", version: "1", kinds: ["production-runtime"], users: ["product/Cargo.toml"], productionReachable: true };
  dependencyClassifyOracleEntry(oracleRuntime, ["claimed-oracle"], {}, "🧪️oracle");
  if (!oracleRuntime.kinds.includes("production-runtime") || oracleRuntime.oracleConflictUsers?.[0] !== "product/Cargo.toml") throw new Error("[verify dependencies self-test] direct runtime manifest hid behind an oracle registry name.");
  const oracleOnly: DependencyBaselineEntry = { ecosystem: "rust", name: "isolated-oracle", version: "1", kinds: ["test-runner"], users: ["unit/oracleCargo.toml"], productionReachable: false };
  dependencyClassifyOracleEntry(oracleOnly, ["isolated"], {}, "🧪️oracle");
  if (oracleOnly.kinds.join() !== "test-oracle" || oracleOnly.oracleConflictUsers) throw new Error("[verify dependencies self-test] isolated test-only oracle was not classified as an oracle.");
  const entry = (ecosystem: DependencyEcosystem, name: string, kind: DependencyKind, user: string, version = "1"): DependencyBaselineEntry => ({ ecosystem, name, version, kinds: [kind], users: [user], productionReachable: kind === "production-runtime" || kind === "production-build", declarations: [{ user, version, kind }] });
  const mixedEntry = (name: string): DependencyBaselineEntry => ({ ecosystem: "js", name, version: "1", kinds: ["repository-tooling"], users: ["package.json", "product/package.json"], productionReachable: false, declarations: [{ user: "package.json", version: "1", kind: "repository-tooling" }, { user: "product/package.json", version: "2", kind: "repository-tooling" }] });
  const rootPackage = JSON.stringify({ packageManager: "bun@1.2.5", engines: { bun: ">=1.2.0" }, devDependencies: { nx: "1", "@nx/devkit": "1", "@nx/js": "1", eslint: "1" } });
  const lock = JSON.stringify({ workspaces: { "": { devDependencies: { nx: "1", "@nx/devkit": "1", "@nx/js": "1", eslint: "1" } }, product: { devDependencies: { nx: "2", "@nx/devkit": "2" } } } });
  const rootOwnerReport = dependencyTruthReportFromEntries([entry("js", "nx", "repository-tooling", "package.json")], [], rootPackage, lock);
  const nonRootOwnerReport = dependencyTruthReportFromEntries([entry("js", "nx", "repository-tooling", "product/package.json", "2")], [], rootPackage, lock);
  if (!rootOwnerReport.entries.mandatedToolchain.some((item) => item.name === "nx") || rootOwnerReport.toolchainConflicts.length !== 0) throw new Error("[verify dependencies self-test] authorized root Nx runner row was not precisely excepted.");
  if (!nonRootOwnerReport.entries.literalExternal.some((item) => item.name === "nx") || nonRootOwnerReport.toolchainConflicts[0]?.user !== "product/package.json") throw new Error("[verify dependencies self-test] unauthorized non-root Nx runner row escaped literal-external inventory.");
  const report = dependencyTruthReportFromEntries(
    [
      entry("python", "composition-runner", "test-runner", "pyproject.toml"),
      entry("python", "product-python", "production-runtime", "product/pyproject.toml"),
      mixedEntry("@nx/devkit"),
      entry("js", "@nx/js", "repository-tooling", "package.json"),
      entry("js", "eslint", "repository-tooling", "package.json"),
      entry("js", "@nx/undeclared", "repository-tooling", "package.json"),
      oracleRuntime,
    ],
    [entry("go", "github.com/example/first", "production-runtime", "consumer/go.mod")],
    rootPackage,
    lock,
  );
  if (!report.entries.literalExternal.some((item) => item.name === "composition-runner")) throw new Error("[verify dependencies self-test] root Python dependency escaped literal-external inventory.");
  if (!report.entries.literalExternal.some((item) => item.name === "product-python")) throw new Error("[verify dependencies self-test] external product Python dependency was hidden.");
  if (!report.entries.firstParty.some((item) => item.name === "github.com/example/first") || report.entries.literalExternal.some((item) => item.name === "github.com/example/first")) throw new Error("[verify dependencies self-test] synthetic first-party Go identity reached literal-external inventory.");
  if (!report.entries.mandatedToolchain.some((item) => item.name === "@nx/js") || report.entries.mandatedToolchain.some((item) => item.name === "@nx/devkit")) throw new Error("[verify dependencies self-test] mixed Nx ownership was incorrectly classified as an identity-wide exception.");
  const mixedDevkit = report.entries.literalExternal.find((item) => item.name === "@nx/devkit");
  if (mixedDevkit?.literalExternalUsers?.join() !== "product/package.json" || mixedDevkit.mandatedToolchainUsers?.join() !== "package.json") throw new Error("[verify dependencies self-test] mixed Nx identity did not split authorized and literal-external owner rows.");
  if (!report.auditedToolchain.authorizedRows.every((row) => row.lockOwned) || !report.auditedToolchain.unauthorizedRows.every((row) => row.lockOwned)) throw new Error("[verify dependencies self-test] Bun lock ownership evidence was not attached to every audited Nx owner row.");
  if (!report.auditedToolchain.bun.valid || report.entries.mandatedToolchain.some((item) => item.name === "eslint") || !report.entries.literalExternal.some((item) => item.name === "eslint") || !report.entries.literalExternal.some((item) => item.name === "@nx/undeclared")) throw new Error("[verify dependencies self-test] illicit root/non-declared Nx tooling escaped literal-external inventory.");
  return 18;
}
