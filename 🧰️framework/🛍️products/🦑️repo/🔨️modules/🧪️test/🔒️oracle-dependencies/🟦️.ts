/** 🔒️ Oracle dependencies
 *
 * Dependency ecosystems, classification and the ratchet of declared oracle dependencies.
 */

//#region 🔌️Adapters
import { testLevelBudgetMs } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { type Implementation } from "@semio-tech/framework/test/adapter";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join, relative, sep } from "node:path";
import { getRepoMetaDir, runProbe } from "../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { isExcludedTestPath, isTestOraclePath, TEST_DOMAIN_REL_PATH, testTaxonomy } from "../🗂️taxonomy/🟦️.ts";
import { SKIP_DIR_NAMES, walkDirectories } from "../🧭️case-discovery/🟦️.ts";
import { oraclePackages } from "../🗃️oracle-registry/🟦️.ts";
import { loadOracleRegistry, type OracleRegistry } from "../🤝️contributions/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🔒️Dependencies
/** 🔒️ Ecosystems the dependency classification covers — all five the repository actually ships. */
export const DEPENDENCY_ECOSYSTEMS = ["rust", "js", "go", "python", "dotnet"] as const;
export type DependencyEcosystem = (typeof DEPENDENCY_ECOSYSTEMS)[number];

/** 🔒️ Phase classification. Manifest placement is evidence, not proof — reachability decides. */
export const DEPENDENCY_CLASSES = ["production-runtime", "production-build", "repository-tooling", "test-runner", "test-oracle"] as const;
export type DependencyClass = (typeof DEPENDENCY_CLASSES)[number];

/** 🔒️ One classified external dependency in the shrink-only baseline. */
export type ClassifiedDependency = Readonly<{ ecosystem: DependencyEcosystem; name: string; version: string; kinds: readonly DependencyClass[]; users: readonly string[]; productionReachable: boolean; oracleIds?: readonly string[]; capabilities?: readonly string[] }>;

/** 🔒️ Maps the legacy four-value `kinds` vocabulary onto the five phase classes. */
export function classifyLegacyKind(kind: string, oracleIds: readonly string[]): DependencyClass {
  if (oracleIds.length > 0) return "test-oracle";
  switch (kind) {
    case "runtime":
      return "production-runtime";
    case "build":
      return "production-build";
    case "test":
      return "test-runner";
    default:
      return "repository-tooling";
  }
}

/** 🔒️ Whether a class can be reached from a production target — the only thing the gate cares about. */
export function isProductionClass(kind: DependencyClass): boolean {
  return kind === "production-runtime" || kind === "production-build";
}

/** 🔒️ The classification ecosystem an implementation's packages belong to. */
export function dependencyEcosystemOf(implementation: Implementation): DependencyEcosystem {
  return implementation === "typescript" ? "js" : implementation;
}

/** 🔒️ The classification ecosystem a registry `ecosystem` string names. */
export function dependencyEcosystemOfRegistryValue(ecosystem: string): DependencyEcosystem {
  return (ecosystem === "javascript" ? "js" : ecosystem) as DependencyEcosystem;
}

/**
 * 🔒️ Every EXTERNAL distribution the contributed test hosts must provision — a contributed host
 * package that carries no `path`. A Rust host reaches its reference libraries through a local crate
 * whose own manifest is already classified, but a Python or npm host names the distribution
 * directly, so without this the declaration would put a third-party library on a host's import path
 * while the ratchet never saw it. Only owner contributions are walked: the framework's own core
 * manifest is domain-neutral and contributes no host package at all.
 */
export function externalOracleHostPackages(registry: OracleRegistry): { ecosystem: DependencyEcosystem; name: string; version: string; users: string[] }[] {
  const byKey = new Map<string, { ecosystem: DependencyEcosystem; name: string; version: string; users: string[] }>();
  for (const contribution of registry.contributions) {
    for (const entry of contribution.oracleHostPackages) {
      if (entry.path !== undefined) continue;
      const ecosystem = dependencyEcosystemOf(entry.implementation);
      const existing = byKey.get(`${ecosystem}:${entry.package}`);
      if (existing === undefined) byKey.set(`${ecosystem}:${entry.package}`, { ecosystem, name: entry.package, version: entry.version ?? "*", users: [contribution.manifestPath] });
      else if (!existing.users.includes(contribution.manifestPath)) existing.users.push(contribution.manifestPath);
    }
  }
  return [...byKey.values()].sort((a, b) => a.ecosystem.localeCompare(b.ecosystem) || a.name.localeCompare(b.name));
}

/**
 * 🚫️ How one ecosystem spells "this source file reaches that package", and which files can spell it
 * at all. Kept as a table because a single regular expression pretending to be five languages at
 * once either misses a form — Python's `import x` was invisible to the purity gate — or matches a
 * stdlib module of an unrelated language and reports a breach that does not exist.
 */
export function importProbe(ecosystem: string, packageName: string): { pattern: RegExp; files: RegExp } {
  const identifier = packageName.replace(/-/g, "_").replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const quoted = packageName.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  switch (dependencyEcosystemOfRegistryValue(ecosystem)) {
    case "rust":
      return { pattern: new RegExp(`(^|[^A-Za-z0-9_])(use\\s+${identifier}\\b|extern\\s+crate\\s+${identifier}\\b)`), files: /\.rs$/ };
    case "python":
      return { pattern: new RegExp(`^\\s*(import\\s+${identifier}\\b|from\\s+${identifier}[.\\s]|import\\s+[A-Za-z0-9_,\\s]*\\b${identifier}\\b\\s*$)`, "m"), files: /\.py$/ };
    case "go":
      return { pattern: new RegExp(`["'\`]${quoted}(/[^"'\`]*)?["'\`]`), files: /\.go$/ };
    case "dotnet":
      return { pattern: new RegExp(`^\\s*(global\\s+)?using\\s+(static\\s+)?${identifier}\\b`, "m"), files: /\.cs$/ };
    default:
      return { pattern: new RegExp(`(from\\s+["'\`]${quoted}["'\`]|require\\(["'\`]${quoted}["'\`]\\)|import\\(["'\`]${quoted}["'\`]\\))`), files: /\.(ts|tsx|js|jsx|mjs|cjs)$/ };
  }
}

/** 🔒️ Gate verdict for a candidate baseline against the committed one. */
export type DependencyRatchet = Readonly<{ ok: boolean; newProduction: readonly string[]; unregisteredTestDeps: readonly string[]; removed: readonly string[]; productionCount: number; baselineProductionCount: number }>;

/**
 * 🔒️ The shrink-only ratchet: a new production-reachable external dependency is always forbidden,
 * a new test dependency is only permitted when an oracle registry entry claims it, and removing
 * dependencies always passes.
 */
export function ratchetDependencies(baseline: readonly ClassifiedDependency[], candidate: readonly ClassifiedDependency[], registry: Pick<OracleRegistry, "oracles">): DependencyRatchet {
  const key = (entry: ClassifiedDependency): string => `${entry.ecosystem}:${entry.name}`;
  const baselineKeys = new Set(baseline.map(key));
  const candidateKeys = new Set(candidate.map(key));
  const registered = new Set(registry.oracles.flatMap((entry) => oraclePackages(entry)));
  const newProduction = candidate.filter((entry) => !baselineKeys.has(key(entry)) && entry.kinds.some(isProductionClass)).map(key);
  const unregisteredTestDeps = candidate.filter((entry) => !baselineKeys.has(key(entry)) && entry.kinds.includes("test-oracle") && !registered.has(entry.name)).map(key);
  const removed = [...baselineKeys].filter((entry) => !candidateKeys.has(entry));
  const productionCount = candidate.filter((entry) => entry.productionReachable).length;
  const baselineProductionCount = baseline.filter((entry) => entry.productionReachable).length;
  return { ok: newProduction.length === 0 && unregisteredTestDeps.length === 0 && productionCount <= baselineProductionCount, newProduction, unregisteredTestDeps, removed, productionCount, baselineProductionCount };
}

/**
 * 🔎️ Scans the LIVE tree for every externally-sourced dependency each ecosystem declares.
 *
 * The ratchet exists to refuse a NEW production-reachable external dependency, and it was being fed
 * the committed baseline as BOTH of its arguments — so `newProduction` and `unregisteredTestDeps` were
 * provably always empty whatever a developer added to a `package.json`, a `Cargo.toml`, a `go.mod` or
 * a `.csproj`. The function itself is sound; it simply had nothing to compare against. This produces
 * the missing half.
 *
 * A declaration is TEST-OWNED when the file declaring it lives in a test-owned location — a case
 * directory, an owner-declared oracle / probe / fixture-generator directory, the test domain itself, or the
 * repository's own meta directory. Everything else is production.
 */
export function scanDeclaredDependencies(repoRoot: string, registry: OracleRegistry = loadOracleRegistry(repoRoot)): ClassifiedDependency[] {
  const taxonomy = testTaxonomy(repoRoot);
  const testOwnedDirs = [taxonomy.testProbeDirName, taxonomy.testGeneratorDirName, taxonomy.testsDirName];
  const metaRoot = relative(repoRoot, getRepoMetaDir(repoRoot)).split(sep).join("/");
  const isTestOwned = (rel: string): boolean => rel === metaRoot || rel.startsWith(`${metaRoot}/`) || rel === TEST_DOMAIN_REL_PATH || rel.startsWith(`${TEST_DOMAIN_REL_PATH}/`) || isTestOraclePath(taxonomy, rel) || rel.split("/").some((segment) => testOwnedDirs.includes(segment));
  const registered = new Set(registry.oracles.flatMap((entry) => oraclePackages(entry)));
  const probePackages = new Set(registry.probes.map((probe) => probe.package));

  const found = new Map<string, { entry: ClassifiedDependency; production: boolean }>();
  const record = (ecosystem: DependencyEcosystem, name: string, version: string, user: string, production: boolean): void => {
    if (name.length === 0) return;
    const id = `${ecosystem}:${name}`;
    const previous = found.get(id);
    // 🔒️Production wins over test: one production declaration makes the package production-reachable
    // however many test-owned files also declare it.
    const isProduction = production || (previous?.production ?? false);
    const kinds: ClassifiedDependency["kinds"] = isProduction ? ["production-runtime"] : registered.has(name) || probePackages.has(name) ? ["test-oracle"] : ["test-runner"];
    found.set(id, {
      production: isProduction,
      entry: {
        ecosystem,
        name,
        version: previous?.entry.version ?? version,
        kinds,
        users: [...new Set([...(previous?.entry.users ?? []), user])].sort(),
        productionReachable: isProduction,
        oracleIds: registered.has(name) ? registry.oracles.filter((oracle) => oraclePackages(oracle).includes(name)).map((oracle) => oracle.id).sort() : undefined,
      },
    });
  };

  walkDirectories(repoRoot, (abs, rel) => {
    if (isExcludedTestPath(repoRoot, rel)) return "skip";
    const production = !isTestOwned(rel);
    for (const name of readdirSync(abs, { withFileTypes: true })) {
      if (!name.isFile()) continue;
      const filePath = `${rel}/${name.name}`;
      let source: string;
      try {
        source = readFileSync(join(repoRoot, filePath), "utf8");
      } catch {
        continue;
      }
      if (name.name === "package.json") {
        try {
          const parsed = JSON.parse(source) as { dependencies?: Record<string, string>; devDependencies?: Record<string, string> };
          for (const [dependency, version] of Object.entries(parsed.dependencies ?? {})) if (!version.startsWith("workspace:") && !version.startsWith("file:") && !version.startsWith("link:")) record("js", dependency, version, filePath, production);
          for (const [dependency, version] of Object.entries(parsed.devDependencies ?? {})) if (!version.startsWith("workspace:") && !version.startsWith("file:") && !version.startsWith("link:")) record("js", dependency, version, filePath, false);
        } catch {
          /* 🧭️A malformed package.json is somebody else's gate to report. */
        }
        continue;
      }
      if (name.name === "Cargo.toml") {
        // 🦀️Only registry crates count. THREE things are not external distributions and each was
        // being counted as one: a `path = "…"` dependency is in-repository source; a
        // `{ workspace = true }` dependency resolves through the root manifest, which declares the
        // repository's own crates by path; and the keys of a `[[bench]]`/`[[bin]]` table are not
        // dependencies at all — a `^\[([^\]]+)\]$` heading match cannot match `[[bench]]`, so the
        // section silently stayed on `dependencies` and `harness`, `name` and `path` were reported as
        // crates. Any line opening a bracket now resets the section.
        let section = "";
        for (const line of source.split(/\r?\n/)) {
          const trimmed = line.trim();
          if (trimmed.startsWith("[")) {
            section = trimmed.match(/^\[([^\]]+)\]$/)?.[1] ?? "";
            continue;
          }
          if (!/^(dependencies|build-dependencies|dev-dependencies|workspace\.dependencies|target\..+\.dependencies)$/.test(section)) continue;
          const declaration = line.match(/^\s*([A-Za-z0-9_-]+)\s*=\s*(.+)$/);
          if (declaration === null) continue;
          const value = declaration[2]!;
          if (/\bpath\s*=/.test(value) || /\bworkspace\s*=\s*true/.test(value)) continue;
          const version = value.match(/"([^"]+)"/)?.[1] ?? "*";
          record("rust", declaration[1]!, version, filePath, production && section !== "dev-dependencies");
        }
        continue;
      }
      if (name.name === "go.mod") {
        for (const line of source.split(/\r?\n/)) {
          const declaration = line.trim().match(/^([a-z0-9.\-]+\/[^\s]+)\s+(v[^\s]+)/);
          if (declaration !== null && declaration[1]!.includes(".")) record("go", declaration[1]!, declaration[2]!, filePath, production);
        }
        continue;
      }
      if (name.name.endsWith(".csproj")) {
        for (const declaration of source.matchAll(/<PackageReference\s+Include="([^"]+)"(?:\s+Version="([^"]+)")?/g)) {
          record("dotnet", declaration[1]!, declaration[2] ?? "*", filePath, production && !/test/i.test(name.name));
        }
        continue;
      }
      if (name.name === "requirements.txt") {
        for (const line of source.split(/\r?\n/)) {
          const trimmed = line.trim();
          if (trimmed.length === 0 || trimmed.startsWith("#") || trimmed.startsWith("-")) continue;
          const declaration = trimmed.match(/^([A-Za-z0-9_.\-]+)\s*(?:\[[^\]]*\])?\s*(?:[=><~!]=+\s*([^\s;#]+))?/);
          if (declaration !== null && declaration[1]!.length > 1) record("python", declaration[1]!, declaration[2] ?? "*", filePath, production);
        }
        continue;
      }
      if (name.name === "pyproject.toml") {
        // 🐍️ONLY the `dependencies = [ … ]` array. Reading every `key = value` line reported
        // `name`, `version`, `requires-python` and `package` as production dependencies — a parser
        // that finds four packages in a file declaring none is worse than no parser.
        let inArray = false;
        for (const line of source.split(/\r?\n/)) {
          const trimmed = line.trim();
          if (/^(dependencies|optional-dependencies\.[A-Za-z0-9_-]+)\s*=\s*\[/.test(trimmed)) {
            inArray = true;
            if (trimmed.endsWith("]")) inArray = false;
            for (const quoted of trimmed.matchAll(/"([^"]+)"/g)) {
              const declaration = quoted[1]!.match(/^([A-Za-z0-9_.\-]+)\s*(?:\[[^\]]*\])?\s*(?:[=><~!]=+\s*([^\s;]+))?/);
              if (declaration !== null) record("python", declaration[1]!, declaration[2] ?? "*", filePath, production);
            }
            continue;
          }
          if (!inArray) continue;
          if (trimmed.startsWith("]")) {
            inArray = false;
            continue;
          }
          for (const quoted of trimmed.matchAll(/"([^"]+)"/g)) {
            const declaration = quoted[1]!.match(/^([A-Za-z0-9_.\-]+)\s*(?:\[[^\]]*\])?\s*(?:[=><~!]=+\s*([^\s;]+))?/);
            if (declaration !== null) record("python", declaration[1]!, declaration[2] ?? "*", filePath, production);
          }
        }
      }
    }
    return "enter";
  });
  return [...found.values()].map((row) => row.entry).sort((a, b) => `${a.ecosystem}:${a.name}`.localeCompare(`${b.ecosystem}:${b.name}`));
}

/** 🐹️ Production dependency closure of a Go module — `go list` deps of the non-test build. */
export function goProductionClosure(repoRoot: string, moduleDir: string): string[] {
  const probe = runProbe("go", ["list", "-deps", "..."], { cwd: join(repoRoot, moduleDir), env: { ...process.env, GOWORK: join(repoRoot, "go.work") }, budgetMs: testLevelBudgetMs("long") });
  if ((probe.status ?? 1) !== 0) return [];
  return probe.stdout
    .split("\n")
    .map((line) => line.trim())
    .filter((line) => line.includes(".") && line.includes("/"));
}

/** 🐍️ Runtime imports of a Python package — source-level import analysis of non-test modules. */
export function pythonRuntimeImports(repoRoot: string, packageDir: string): string[] {
  const abs = join(repoRoot, packageDir);
  if (!existsSync(abs)) return [];
  const imports = new Set<string>();
  const walk = (dir: string): void => {
    for (const entry of readdirSync(dir, { withFileTypes: true })) {
      const full = join(dir, entry.name);
      if (entry.isDirectory()) {
        if (!SKIP_DIR_NAMES.has(entry.name)) walk(full);
        continue;
      }
      if (!entry.name.endsWith(".py") || entry.name.startsWith("test_") || entry.name.endsWith("_test.py")) continue;
      for (const line of readFileSync(full, "utf8").split(/\r?\n/)) {
        const match = line.match(/^\s*(?:from\s+([A-Za-z0-9_.]+)\s+import|import\s+([A-Za-z0-9_.]+))/);
        const module = (match?.[1] ?? match?.[2])?.split(".")[0];
        if (module && !module.startsWith("_")) imports.add(module);
      }
    }
  };
  walk(abs);
  return [...imports].sort();
}

/** 🔷️ Package references of a .NET project, split by whether the project is a test asset. */
export function dotnetPackageReferences(repoRoot: string, csprojRel: string): { packages: string[]; isTestProject: boolean } {
  const abs = join(repoRoot, csprojRel);
  if (!existsSync(abs)) return { packages: [], isTestProject: false };
  const content = readFileSync(abs, "utf8");
  const packages = [...content.matchAll(/<PackageReference\s+Include="([^"]+)"/g)].map((match) => match[1]!);
  const isTestProject = /<IsTestProject>\s*true\s*<\/IsTestProject>/i.test(content) || packages.some((name) => name.startsWith("xunit") || name.startsWith("Microsoft.NET.Test"));
  return { packages, isTestProject };
}
//#endregion 🔒️Dependencies
