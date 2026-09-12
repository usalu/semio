import { existsSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { DEPENDENCY_BASELINE_REL_PATH, DEPENDENCY_ECOSYSTEMS, dependencyCollectGo, dependencyDiscoverScriptTsFiles, dependencyFreezeCheck, dependencyFreezeCurrentThirdParty, dependencyFreezeWriteBaseline, dependencyJsParity, dependencyReadFileSafe, type DependencyBaselineEntry, type DependencyDeclaration, type DependencyEcosystem, type DependencyJsParityReport, type DependencyKind } from "../📇️inventory/🟦️.ts";

export type DependencyTruthDisposition = "literal-external" | "first-party" | "composition-scoped" | "mandated-toolchain";
export type DependencyTruthEntry = DependencyBaselineEntry & { disposition: DependencyTruthDisposition; rationale: string; literalExternalUsers?: string[]; mandatedToolchainUsers?: string[] };
type DependencyTruthEcosystemSummary = { ecosystem: DependencyEcosystem; raw: number; thirdParty: number; firstParty: number; compositionScoped: number; mandatedToolchain: number; corrected: number; literalExternal: number; productionReachable: number; kinds: Record<DependencyKind, number> };
type DependencyToolchainRow = DependencyDeclaration & { name: string; lockVersion?: string; lockOwned: boolean };
export type DependencyTruthReport = {
  zeroTarget: 0;
  meetsTarget: boolean;
  ecosystems: DependencyTruthEcosystemSummary[];
  totals: Omit<DependencyTruthEcosystemSummary, "ecosystem" | "kinds">;
  auditedToolchain: { bun: { engine: string; packageManager: string; valid: boolean }; nxPackages: string[]; authorizedRows: DependencyToolchainRow[]; unauthorizedRows: DependencyToolchainRow[]; failures: string[] };
  oracleConflicts: { ecosystem: DependencyEcosystem; name: string; users: string[] }[];
  toolchainConflicts: DependencyToolchainRow[];
  entries: { raw: DependencyTruthEntry[]; literalExternal: DependencyTruthEntry[]; firstParty: DependencyTruthEntry[]; compositionScoped: DependencyTruthEntry[]; mandatedToolchain: DependencyTruthEntry[]; mandatedToolchainRows: DependencyToolchainRow[] };
};

const DEPENDENCY_MANDATED_NX_PACKAGES = new Set(["nx", "@nx/devkit", "@nx/js"]);
const DEPENDENCY_AUTHORIZED_TOOLCHAIN_MANIFESTS = new Set(["package.json"]);
export const DEPENDENCY_REPO_POLICY_ROOT = "🧰️framework/🛍️products/🦑️repo";
const DEPENDENCY_REPO_POLICY_LIBRARY = `${DEPENDENCY_REPO_POLICY_ROOT}/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts`;
export const DEPENDENCY_REPO_POLICY_ROUTERS = [`${DEPENDENCY_REPO_POLICY_ROOT}/📜️script.ts`, `${DEPENDENCY_REPO_POLICY_ROOT}/🔨️modules/💻️client/📜️script.ts`, `${DEPENDENCY_REPO_POLICY_ROOT}/🔨️modules/📚️library/📜️script.ts`] as const;

export function dependencyRepoPolicyLibrarySpecifier(repoRoot: string, script: string): string {
  const specifier = relative(dirname(join(repoRoot, script)), join(repoRoot, DEPENDENCY_REPO_POLICY_LIBRARY)).replaceAll("\\", "/");
  return specifier.startsWith(".") ? specifier : `./${specifier}`;
}

export function dependencyRepoPolicyImportBoundaryFailure(repoRoot: string, script: string, source = dependencyReadFileSafe(repoRoot, script), pathExists: (path: string) => boolean = existsSync): string | undefined {
  const expected = dependencyRepoPolicyLibrarySpecifier(repoRoot, script);
  const imports = [...source.matchAll(/\bfrom\s+["']([^"']+)["']/gu)].map((match) => match[1]!);
  if (!imports.includes(expected)) return `[verify dependencies import-boundary] ${script} must import its policy APIs from owned module ${expected}; found ${imports.join(",") || "no static import"}.`;
  const target = resolve(dirname(join(repoRoot, script)), expected);
  const targetRelative = relative(repoRoot, target);
  if (targetRelative.startsWith("..") || targetRelative === "") return `[verify dependencies import-boundary] ${script} resolves outside the repository: ${targetRelative || "."}.`;
  if (!pathExists(target)) return `[verify dependencies import-boundary] ${script} imports missing owned module ${targetRelative}; update the repo-native boundary without a shim.`;
  return undefined;
}

export function dependencyRepoPolicyRouterSetFailure(discovered: readonly string[]): string | undefined {
  const expected = new Set<string>(DEPENDENCY_REPO_POLICY_ROUTERS);
  const actual = new Set(discovered);
  const missing = [...expected].filter((script) => !actual.has(script));
  const unenumerated = [...actual].filter((script) => !expected.has(script));
  if (missing.length > 0 || unenumerated.length > 0) return `[verify dependencies import-boundary] repo policy router set drifted; missing=${missing.join(",") || "none"}; unenumerated=${unenumerated.join(",") || "none"}.`;
  return undefined;
}

function dependencyDiscoverRepoPolicyRouters(repoRoot: string): string[] {
  return dependencyDiscoverScriptTsFiles(join(repoRoot, DEPENDENCY_REPO_POLICY_ROOT))
    .map((script) => `${DEPENDENCY_REPO_POLICY_ROOT}/${script}`)
    .filter((script) => /\brunPolicyOnlyMain\s*\(/u.test(dependencyReadFileSafe(repoRoot, script)));
}

function dependencyAssertRepoPolicyImportBoundary(repoRoot: string): void {
  const setFailure = dependencyRepoPolicyRouterSetFailure(dependencyDiscoverRepoPolicyRouters(repoRoot));
  if (setFailure) throw new Error(setFailure);
  for (const script of DEPENDENCY_REPO_POLICY_ROUTERS) {
    const failure = dependencyRepoPolicyImportBoundaryFailure(repoRoot, script);
    if (failure) throw new Error(failure);
  }
}

function dependencyTruthBaselineEntry(entry: DependencyTruthEntry): DependencyBaselineEntry {
  return { ecosystem: entry.ecosystem, name: entry.name, version: entry.version, kinds: entry.kinds, users: entry.users, productionReachable: entry.productionReachable, ...(entry.oracleIds ? { oracleIds: entry.oracleIds } : {}), ...(entry.oracleConflictUsers ? { oracleConflictUsers: entry.oracleConflictUsers } : {}) };
}

function dependencyTruthRootToolchain(rootPackageContent: string): { bun: { engine: string; packageManager: string; valid: boolean }; nxPackages: Set<string>; failures: string[] } {
  let pkg: { devDependencies?: Record<string, string>; engines?: Record<string, string>; packageManager?: string } = {};
  try {
    pkg = JSON.parse(rootPackageContent) as typeof pkg;
  } catch {
    return { bun: { engine: "", packageManager: "", valid: false }, nxPackages: new Set(), failures: ["root package.json is unreadable"] };
  }
  const engine = pkg.engines?.bun ?? "";
  const packageManager = pkg.packageManager ?? "";
  const bunValid = engine !== "" && /^bun@[^\s]+$/u.test(packageManager);
  const nxPackages = new Set(Object.keys(pkg.devDependencies ?? {}).filter((name) => DEPENDENCY_MANDATED_NX_PACKAGES.has(name)));
  const failures: string[] = [];
  if (!bunValid) failures.push("root package.json must audit both engines.bun and packageManager bun@…");
  if (!nxPackages.has("nx")) failures.push("root package.json must directly declare the Nx runner");
  return { bun: { engine, packageManager, valid: bunValid }, nxPackages, failures };
}

function dependencyTruthDeclarations(entry: DependencyBaselineEntry): DependencyDeclaration[] {
  return entry.declarations ?? entry.users.map((user) => ({ user, version: entry.version, kind: entry.kinds[0] ?? "repository-tooling" }));
}

function dependencyToolchainLockVersions(content: string): Map<string, string> {
  const versions = new Map<string, string>();
  try {
    const workspaces = (Bun.JSONC.parse(content) as { workspaces?: Record<string, { devDependencies?: Record<string, string> }> }).workspaces ?? {};
    for (const [workspace, snapshot] of Object.entries(workspaces)) {
      const user = workspace === "" ? "package.json" : `${workspace}/package.json`;
      for (const [name, version] of Object.entries(snapshot.devDependencies ?? {})) versions.set(`${user}\0${name}`, version);
    }
  } catch {
    return versions;
  }
  return versions;
}

/** 🔒️Classifies only the narrow audited exceptions; every other third-party identity remains literal external inventory. */
export function dependencyTruthReportFromEntries(thirdParty: readonly DependencyBaselineEntry[], firstParty: readonly DependencyBaselineEntry[], rootPackageContent: string, lockContent = ""): DependencyTruthReport {
  const toolchain = dependencyTruthRootToolchain(rootPackageContent);
  const lockVersions = dependencyToolchainLockVersions(lockContent);
  const authorizedRows: DependencyToolchainRow[] = [];
  const unauthorizedRows: DependencyToolchainRow[] = [];
  const classifiedThirdParty: DependencyTruthEntry[] = thirdParty.map((entry) => {
    if (entry.ecosystem === "js" && DEPENDENCY_MANDATED_NX_PACKAGES.has(entry.name)) {
      const declarations = dependencyTruthDeclarations(entry);
      const exactAuthorized = declarations.filter((declaration) => toolchain.nxPackages.has(entry.name) && DEPENDENCY_AUTHORIZED_TOOLCHAIN_MANIFESTS.has(declaration.user) && declaration.kind === "repository-tooling");
      const external = declarations.filter((declaration) => !exactAuthorized.includes(declaration));
      const row = (declaration: DependencyDeclaration): DependencyToolchainRow => {
        const lockVersion = lockVersions.get(`${declaration.user}\0${entry.name}`);
        return { ...declaration, name: entry.name, ...(lockVersion ? { lockVersion } : {}), lockOwned: lockVersion === declaration.version };
      };
      authorizedRows.push(...exactAuthorized.map(row));
      unauthorizedRows.push(...external.map(row));
      if (exactAuthorized.length > 0 && external.length === 0) return { ...entry, disposition: "mandated-toolchain", rationale: "exact AGENTS-mandated Nx package declared only by an authorized orchestration manifest", mandatedToolchainUsers: exactAuthorized.map((declaration) => declaration.user) };
      return { ...entry, disposition: "literal-external", rationale: "Nx identity has a non-authorized owner or non-tooling declaration", literalExternalUsers: external.map((declaration) => declaration.user), ...(exactAuthorized.length > 0 ? { mandatedToolchainUsers: exactAuthorized.map((declaration) => declaration.user) } : {}) };
    }
    return { ...entry, disposition: "literal-external", rationale: "third-party source, build, test, runner, or tooling dependency" };
  });
  const classifiedFirstParty = firstParty.map<DependencyTruthEntry>((entry) => ({ ...entry, disposition: "first-party", rationale: "Go workspace module or locally replaced module" }));
  const raw = [...classifiedThirdParty, ...classifiedFirstParty].sort((left, right) => left.ecosystem.localeCompare(right.ecosystem) || left.name.localeCompare(right.name));
  const literalExternal = classifiedThirdParty.filter((entry) => entry.disposition === "literal-external");
  const compositionScoped = classifiedThirdParty.filter((entry) => entry.disposition === "composition-scoped");
  const mandatedToolchain = classifiedThirdParty.filter((entry) => entry.disposition === "mandated-toolchain");
  const kinds = (): Record<DependencyKind, number> => ({ "production-runtime": 0, "production-build": 0, "repository-tooling": 0, "test-runner": 0, "test-oracle": 0 });
  const ecosystems = DEPENDENCY_ECOSYSTEMS.map<DependencyTruthEcosystemSummary>((ecosystem) => {
    const rawEntries = raw.filter((entry) => entry.ecosystem === ecosystem);
    const thirdPartyEntries = classifiedThirdParty.filter((entry) => entry.ecosystem === ecosystem);
    const literalEntries = literalExternal.filter((entry) => entry.ecosystem === ecosystem);
    const kindCounts = kinds();
    for (const entry of thirdPartyEntries) for (const kind of entry.kinds) kindCounts[kind] += 1;
    return {
      ecosystem,
      raw: rawEntries.length,
      thirdParty: thirdPartyEntries.length,
      firstParty: rawEntries.filter((entry) => entry.disposition === "first-party").length,
      compositionScoped: thirdPartyEntries.filter((entry) => entry.disposition === "composition-scoped").length,
      mandatedToolchain: thirdPartyEntries.filter((entry) => entry.disposition === "mandated-toolchain").length,
      corrected: literalEntries.length,
      literalExternal: literalEntries.length,
      productionReachable: literalEntries.filter((entry) => entry.productionReachable).length,
      kinds: kindCounts,
    };
  });
  const sum = (field: keyof Omit<DependencyTruthEcosystemSummary, "ecosystem" | "kinds">): number => ecosystems.reduce((total, entry) => total + entry[field], 0);
  const totals = { raw: sum("raw"), thirdParty: sum("thirdParty"), firstParty: sum("firstParty"), compositionScoped: sum("compositionScoped"), mandatedToolchain: sum("mandatedToolchain"), corrected: sum("corrected"), literalExternal: sum("literalExternal"), productionReachable: sum("productionReachable") };
  const oracleConflicts = classifiedThirdParty.filter((entry) => (entry.oracleConflictUsers?.length ?? 0) > 0).map((entry) => ({ ecosystem: entry.ecosystem, name: entry.name, users: entry.oracleConflictUsers! }));
  authorizedRows.sort((left, right) => left.name.localeCompare(right.name) || left.user.localeCompare(right.user));
  unauthorizedRows.sort((left, right) => left.name.localeCompare(right.name) || left.user.localeCompare(right.user));
  if (lockContent !== "") for (const row of authorizedRows) if (!row.lockOwned) toolchain.failures.push(`authorized ${row.user} ${row.name}@${row.version} is not owned by the same bun.lock workspace snapshot`);
  return {
    zeroTarget: 0,
    meetsTarget: totals.literalExternal === 0 && oracleConflicts.length === 0 && unauthorizedRows.length === 0 && toolchain.failures.length === 0,
    ecosystems,
    totals,
    auditedToolchain: { bun: toolchain.bun, nxPackages: [...toolchain.nxPackages].sort(), authorizedRows, unauthorizedRows, failures: toolchain.failures },
    oracleConflicts,
    toolchainConflicts: unauthorizedRows,
    entries: { raw, literalExternal, firstParty: classifiedFirstParty, compositionScoped, mandatedToolchain, mandatedToolchainRows: authorizedRows },
  };
}

function dependencyTruthReport(repoRoot: string, thirdParty = dependencyFreezeCurrentThirdParty(repoRoot)): DependencyTruthReport {
  const firstPartyByKey = new Map<string, DependencyBaselineEntry>();
  dependencyCollectGo(repoRoot, () => {}, (entry) => {
    const key = `${entry.ecosystem}:${entry.name}`;
    const existing = firstPartyByKey.get(key);
    if (!existing) firstPartyByKey.set(key, entry);
    else {
      if (!existing.kinds.includes(entry.kinds[0]!)) existing.kinds.push(entry.kinds[0]!);
      if (!existing.users.includes(entry.users[0]!)) existing.users.push(entry.users[0]!);
      existing.productionReachable ||= entry.productionReachable;
    }
  });
  return dependencyTruthReportFromEntries(thirdParty, [...firstPartyByKey.values()], dependencyReadFileSafe(repoRoot, "package.json"), dependencyReadFileSafe(repoRoot, "bun.lock"));
}

function dependencyTruthSummaryText(report: DependencyTruthReport): string {
  const lines = ["ecosystem\traw\tthird-party\tfirst-party\tcomposition-scoped\tmandated-toolchain\tcorrected/literal-external\tproduction-reachable\tkind-census"];
  for (const row of report.ecosystems) {
    const kindCensus = Object.entries(row.kinds)
      .filter(([, count]) => count > 0)
      .map(([kind, count]) => `${kind}:${count}`)
      .join(",");
    lines.push(`${row.ecosystem}\t${row.raw}\t${row.thirdParty}\t${row.firstParty}\t${row.compositionScoped}\t${row.mandatedToolchain}\t${row.literalExternal}\t${row.productionReachable}\t${kindCensus || "none"}`);
  }
  lines.push(`total\t${report.totals.raw}\t${report.totals.thirdParty}\t${report.totals.firstParty}\t${report.totals.compositionScoped}\t${report.totals.mandatedToolchain}\t${report.totals.literalExternal}\t${report.totals.productionReachable}`);
  lines.push(`zero-target=${report.zeroTarget} literal-external=${report.totals.literalExternal} meets-target=${report.meetsTarget}`);
  const auditedRows = [...report.auditedToolchain.authorizedRows, ...report.auditedToolchain.unauthorizedRows];
  lines.push(`audited-toolchain bun=${report.auditedToolchain.bun.packageManager || "missing"} engines.bun=${report.auditedToolchain.bun.engine || "missing"} nx=${report.auditedToolchain.nxPackages.join(",") || "missing"} authorized-rows=${report.auditedToolchain.authorizedRows.length} unauthorized-rows=${report.auditedToolchain.unauthorizedRows.length} lock-owned=${auditedRows.filter((row) => row.lockOwned).length}/${auditedRows.length}`);
  lines.push(`oracle-conflicts=${report.oracleConflicts.length} toolchain-owner-conflicts=${report.toolchainConflicts.length}`);
  return lines.join("\n");
}




export interface DependencyVerificationSelfTests { readonly jsLockParity: () => number; readonly truth: () => number; }

/** ⚖️ Runs the dependency truth, parity, inventory, and ratchet command without owning task routing. */
export function runDependencyVerification(repoRoot: string, args: string[], selfTests: DependencyVerificationSelfTests): void {
  dependencyAssertRepoPolicyImportBoundary(repoRoot);
  if (args[0] === "parity") {
    if (args[1] !== "js") throw new Error("[verify dependencies] parity currently supports only the 'js' ecosystem.");
    const formatIndex = args.indexOf("--format");
    const format = formatIndex >= 0 ? args[formatIndex + 1] : "text";
    if (format !== "text" && format !== "json") throw new Error(`[verify dependencies parity js] unsupported format ${JSON.stringify(format)}.`);
    const report: DependencyJsParityReport = dependencyJsParity(repoRoot, selfTests.jsLockParity);
    if (format === "json") console.log(JSON.stringify(report, null, 2));
    else console.log(`[verify dependencies parity js] manifests=${report.manifests} external-rows=${report.externalRows} evidenced=${report.evidencedRows} unowned=${report.unownedRows.length} undeclared-imports=${report.undeclaredImports.length} lock-workspaces=${report.lockWorkspaces} lock-mismatches=${report.lockMismatches.length} lock-fixtures=${report.lockFixtureChecks}`);
    if (report.undeclaredImports.length > 0) {
      if (format === "text") for (const finding of report.undeclaredImports.slice(0, 100)) console.error(`  ${finding.file}:${finding.line}: ${finding.dependency} is not declared by ${finding.manifest}`);
      throw new Error(`[verify dependencies parity js] ${report.undeclaredImports.length} external import(s) have no declaration in their owning package.`);
    }
    if (report.lockMismatches.length > 0) {
      if (format === "text") for (const finding of report.lockMismatches.slice(0, 100)) console.error(`  ${finding.manifest}: ${finding.kind}${finding.dependency ? ` ${finding.section}:${finding.dependency}` : ""}${finding.manifestVersion !== undefined || finding.lockVersion !== undefined ? ` (manifest=${JSON.stringify(finding.manifestVersion)}, lock=${JSON.stringify(finding.lockVersion)})` : ""}`);
      throw new Error(`[verify dependencies parity js] ${report.lockMismatches.length} package manifest / bun.lock workspace snapshot mismatch(es).`);
    }
    if (args.includes("--no-unowned-rows") && report.unownedRows.length > 0) {
      if (format === "text") for (const finding of report.unownedRows.slice(0, 100)) console.error(`  ${finding.manifest}: ${finding.dependency} has no owned-scope source/config/script evidence`);
      throw new Error(`[verify dependencies parity js] ${report.unownedRows.length} direct external row(s) have no owned-scope evidence.`);
    }
    console.log("[verify dependencies parity js] clean.");
    return;
  }
  if (args[0] === "self-test") { console.log(`[verify dependencies self-test] hostile-mutations=${selfTests.truth()} clean.`); return; }
  if (args[0] === "summary" || args[0] === "literal-external") {
    const formatIndex = args.indexOf("--format");
    const format = formatIndex >= 0 ? args[formatIndex + 1] : "text";
    if (format !== "text" && format !== "json") throw new Error(`[verify dependencies ${args[0]}] unsupported format ${JSON.stringify(format)}.`);
    const report = dependencyTruthReport(repoRoot);
    console.log(format === "json" ? JSON.stringify(report, null, 2) : dependencyTruthSummaryText(report));
    if (args[0] === "literal-external" && !report.meetsTarget) {
      if (report.oracleConflicts.length > 0) for (const conflict of report.oracleConflicts) console.error(`  oracle-conflict ${conflict.ecosystem}:${conflict.name} declared by ${conflict.users.join(", ")}`);
      if (report.toolchainConflicts.length > 0) for (const conflict of report.toolchainConflicts) console.error(`  toolchain-owner-conflict js:${conflict.name}@${conflict.version} declared by ${conflict.user} (lock-owned=${conflict.lockOwned})`);
      if (report.auditedToolchain.failures.length > 0) for (const failure of report.auditedToolchain.failures) console.error(`  toolchain-audit ${failure}`);
      throw new Error(`[verify dependencies literal-external] target=0, current=${report.totals.literalExternal}, oracle-conflicts=${report.oracleConflicts.length}, toolchain-owner-conflicts=${report.toolchainConflicts.length}, toolchain-failures=${report.auditedToolchain.failures.length}.`);
    }
    return;
  }
  if (args[0] === "list") {
    const requested = args.slice(1).find((arg) => !arg.startsWith("--"));
    const ecosystem = requested === undefined || requested === "all" ? undefined : requested as DependencyEcosystem;
    if (ecosystem && !DEPENDENCY_ECOSYSTEMS.includes(ecosystem)) throw new Error(`[verify dependencies] list ecosystem must be 'all' or one of ${DEPENDENCY_ECOSYSTEMS.join(", ")}.`);
    const report = dependencyTruthReport(repoRoot);
    const entries = args.includes("--raw") ? report.entries.raw : args.includes("--literal-external") ? report.entries.literalExternal : report.entries.raw.filter((entry) => entry.disposition !== "first-party").map(dependencyTruthBaselineEntry);
    console.log(JSON.stringify(entries.filter((entry) => !ecosystem || entry.ecosystem === ecosystem), null, 2));
    return;
  }
  if (args[0] === "write-baseline") {
    const baseline = dependencyFreezeWriteBaseline(repoRoot);
    console.log(`[verify dependencies] wrote ${DEPENDENCY_BASELINE_REL_PATH}: ${baseline.entries.length} third-party dependenc(y/ies) at commit ${baseline.commit}.`);
    return;
  }
  const result = dependencyFreezeCheck(repoRoot);
  console.log(`[verify dependencies] baseline: ${result.baseline.entries.length} third-party dependenc(y/ies) (commit ${result.baseline.commit || "none — run write-baseline first"}); current: ${result.current.length}.`);
  console.log(dependencyTruthSummaryText(dependencyTruthReport(repoRoot, result.current)));
  if (result.removedDeps.length > 0) {
    console.log(`[verify dependencies] ${result.removedDeps.length} dependenc(y/ies) removed since baseline (always passes — ratchet only tightens):`);
    for (const dependency of result.removedDeps) console.log(`  ${dependency.ecosystem}:${dependency.name}`);
  }
  if (result.newDeps.length > 0) {
    console.error(`[verify dependencies] ${result.newDeps.length} NEW dependenc(y/ies) not in ${DEPENDENCY_BASELINE_REL_PATH}:`);
    for (const dependency of result.newDeps) console.error(`  ${dependency.ecosystem}:${dependency.name}@${dependency.version} (kinds: ${dependency.kinds.join(",")}; used by: ${dependency.users.slice(0, 3).join(", ")}${dependency.users.length > 3 ? `, +${dependency.users.length - 3} more` : ""})`);
    throw new Error(`[verify dependencies] ${result.newDeps.length} new third-party dependenc(y/ies) — approve deliberately with 'bun ./📜️script.ts verify dependencies write-baseline', or remove the dependency.`);
  }
  console.log("[verify dependencies] clean — no new third-party dependencies.");
}
