import { describe, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, posix } from "node:path";
import ts from "typescript";
import fixture from "../🧫️fixtures/🧱️package-implementation-admission/🔣️.json";
import { loadCatalogTaxonomy } from "../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { declaredEntryDirectories, packageContractTreePatterns, packageDirectoryAdmitted, packageImplementationAdmitted, type PackageDirectoryAdmission, type PackageImplementationAdmission } from "../../🔍️discovery/🧱️package-implementation-admission/🟦️.ts";
import { clearDiscoveryCache, discoverPackageProblems } from "../../🔍️discovery/🔭️repository-scan/🟦️.ts";
import { classifyPackageSource } from "../../🔍️discovery/🏷️package-source-classification/🟦️.ts";

/** 🧷️ A source fixture row whose authored `expectedRole` is read as the closed role the classifier returns. */
type SourceCaseRow = Omit<(typeof fixture.sourceCases)[number], "expectedRole"> & { readonly expectedRole: ReturnType<typeof classifyPackageSource>["role"] };

const formatterParses = (language: string, source: string): boolean | null => {
  const run = language === "go" ? spawnSync("gofmt", ["-e"], { input: source, encoding: "utf8" }) : spawnSync("rustfmt", ["--edition", "2021", "--emit", "stdout"], { input: source, encoding: "utf8" });
  return run.error ? null : run.status === 0;
};

describe("package implementation admission (language-agnostic fixture)", () => {
  for (const row of fixture.admissionCases) {
    test(`${row.id}`, () => {
      const names = new Set(row.directoryFiles);
      expect(packageImplementationAdmitted(row.admission as PackageImplementationAdmission | null, { relativePath: row.relativePath, kindId: row.kindId, directoryHasFile: (name) => names.has(name), manifestFilename: (id) => (fixture.manifestFilenames as Record<string, string>)[id] ?? null })).toBe(row.expected);
    });
  }

  test("the live schema admits Go sub-packages and Cargo crates and nothing else", () => {
    const rules = loadCatalogTaxonomy().packageBoundaryRules;
    expect(rules["🐹️go"]!.implementationAdmission).toEqual({ form: "nested-source-unit", unitSourceFileKindId: "go-source" });
    expect(rules["🦀️rust"]!.implementationAdmission).toEqual({ form: "nested-manifest-unit", unitManifestContractId: "cargo-manifest" });
    for (const [id, rule] of Object.entries(rules)) if (id !== "🐹️go" && id !== "🦀️rust") expect(rule.implementationAdmission).toBeNull();
  });

  test("the taxonomy reader normalises an absent implementationAdmission to null and every rule answers the admission", () => {
    const rules = JSON.parse(readFileSync(new URL("../../🔣️taxonomy.json", import.meta.url), "utf8")).packageBoundaryRules as Record<string, { implementationAdmission?: PackageImplementationAdmission | null }>;
    const input = { relativePath: "🧩️unit/🦀️.rs", kindId: "rust-source", directoryHasFile: () => false, manifestFilename: () => null };
    for (const [id, rule] of Object.entries(loadCatalogTaxonomy().packageBoundaryRules)) {
      const declared = rules[id]!.implementationAdmission;
      expect(rule.implementationAdmission).toEqual(declared ?? null);
      expect(() => packageImplementationAdmitted(rule.implementationAdmission, input)).not.toThrow();
      if (declared === undefined) expect(rule.implementationAdmission).toBeNull();
      if (declared === undefined || declared === null) expect(packageImplementationAdmitted(rule.implementationAdmission, input)).toBe(false);
    }
  });

  for (const row of fixture.directoryCases) {
    test(`directory admission: ${row.id}`, () => {
      expect(packageDirectoryAdmitted(row.admission as PackageImplementationAdmission | null, row.directoryAdmissions as PackageDirectoryAdmission[], { relativePath: row.relativePath, unitBelow: row.unitBelow, declaredEntryDirectories: new Set(row.declaredEntryDirectories), contractTreeBelow: row.contractTreeBelow })).toBe(row.expected);
    });
  }

  for (const row of fixture.declaredEntryCases) {
    test(`declared entries: ${row.id}`, () => {
      expect([...declaredEntryDirectories(row.manifest, row.entryFields)].sort()).toEqual([...row.expectedDirectories].sort());
      if (!row.oracle) return;
      const root = mkdtempSync(join(tmpdir(), "semio-declared-entries-"));
      try {
        const packageRoot = join(root, "node_modules", "fixture");
        const manifest = row.manifest as { exports?: Record<string, unknown> | string };
        const targets: string[] = [];
        const collect = (value: unknown): void => {
          if (typeof value === "string") targets.push(value);
          else if (value !== null && typeof value === "object") Object.values(value).forEach(collect);
        };
        collect(manifest.exports);
        mkdirSync(packageRoot, { recursive: true });
        writeFileSync(join(packageRoot, "package.json"), JSON.stringify(row.manifest));
        for (const target of targets) {
          mkdirSync(join(packageRoot, dirname(target)), { recursive: true });
          writeFileSync(join(packageRoot, target), "export {};\n");
        }
        writeFileSync(join(root, "importer.ts"), "export {};\n");
        const specifiers = typeof manifest.exports === "string" ? ["fixture"] : Object.keys(manifest.exports ?? {}).map((key) => (key === "." ? "fixture" : `fixture/${key.slice(2)}`));
        const declared = declaredEntryDirectories(row.manifest, row.entryFields);
        for (const specifier of specifiers) {
          const resolved = ts.resolveModuleName(specifier, join(root, "importer.ts"), { moduleResolution: ts.ModuleResolutionKind.Bundler, allowImportingTsExtensions: true, module: ts.ModuleKind.ESNext }, ts.sys).resolvedModule;
          expect(resolved, specifier).toBeDefined();
          const inside = posix.dirname(resolved!.resolvedFileName.split(`${packageRoot}/`)[1] ?? "");
          if (inside !== ".") expect(declared.has(inside), `${specifier} -> ${inside}`).toBe(true);
        }
      } finally {
        rmSync(root, { recursive: true, force: true });
      }
    });
  }

  for (const row of fixture.contractTreePatterns) {
    test(`contract trees: ${row.id}`, () => {
      expect([...packageContractTreePatterns(row.patterns, row.packagePrefix)]).toEqual(row.expected);
    });
  }

  for (const row of fixture.gitVisibleCases) {
    test(`repository scan in a git work tree: ${row.id}`, () => {
      const root = mkdtempSync(join(tmpdir(), "semio-package-git-visible-"));
      const packageRoot = join(root, "🧰️framework/🔨️modules/🎫️ticket/📦️packages", row.language);
      try {
        for (const [path, content] of Object.entries(row.files)) {
          mkdirSync(dirname(join(packageRoot, path)), { recursive: true });
          writeFileSync(join(packageRoot, path), content);
        }
        writeFileSync(join(root, ".gitignore"), `${row.ignore.map((path) => `/🧰️framework/🔨️modules/🎫️ticket/📦️packages/${row.language}/${path}`).join("\n")}\n`);
        const git = spawnSync("git", ["init", "--quiet"], { cwd: root, encoding: "utf8" });
        if (git.error || git.status !== 0) return;
        clearDiscoveryCache();
        const prefix = `🧰️framework/🔨️modules/🎫️ticket/📦️packages/${row.language}/`;
        const violations = discoverPackageProblems(root, loadCatalogTaxonomy()).filter((problem) => problem.kind === "packaging-violation").map((problem) => problem.path.slice(prefix.length)).sort();
        expect(violations).toEqual([...row.expectedViolations].sort());
        const ignored = spawnSync("git", ["check-ignore", "--quiet", ...row.ignore.map((path) => `🧰️framework/🔨️modules/🎫️ticket/📦️packages/${row.language}/${path}`)], { cwd: root });
        expect(ignored.status).toBe(0);
      } finally {
        clearDiscoveryCache();
        rmSync(root, { recursive: true, force: true });
      }
    });
  }

  for (const row of fixture.bodyCases) {
    test(`repository scan body: ${row.id}`, () => {
      const root = mkdtempSync(join(tmpdir(), "semio-package-body-scan-"));
      const packageRoot = join(root, "🧰️framework/🔨️modules/🎫️ticket/📦️packages", row.language);
      try {
        for (const [path, content] of Object.entries(row.files)) {
          mkdirSync(dirname(join(packageRoot, path)), { recursive: true });
          writeFileSync(join(packageRoot, path), content);
        }
        clearDiscoveryCache();
        const prefix = `🧰️framework/🔨️modules/🎫️ticket/📦️packages/${row.language}/`;
        const found = discoverPackageProblems(root, loadCatalogTaxonomy()).filter((problem) => problem.kind === "package-implementation" || problem.kind === "package-role-unresolved").map((problem) => problem.path.slice(prefix.length)).sort();
        expect(found).toEqual([...row.expectedImplementations].sort());
      } finally {
        clearDiscoveryCache();
        rmSync(root, { recursive: true, force: true });
      }
    });
  }

  for (const row of fixture.scanCases) {
    test(`repository scan: ${row.id}`, () => {
      const root = mkdtempSync(join(tmpdir(), "semio-package-directory-scan-"));
      const packageRoot = join(root, "🧰️framework/🔨️modules/🎫️ticket/📦️packages", row.language);
      try {
        for (const [path, content] of Object.entries(row.files)) {
          mkdirSync(dirname(join(packageRoot, path)), { recursive: true });
          writeFileSync(join(packageRoot, path), content);
        }
        clearDiscoveryCache();
        const prefix = `🧰️framework/🔨️modules/🎫️ticket/📦️packages/${row.language}/`;
        const violations = discoverPackageProblems(root, loadCatalogTaxonomy()).filter((problem) => problem.kind === "packaging-violation").map((problem) => problem.path.slice(prefix.length)).sort();
        expect(violations).toEqual([...row.expectedViolations].sort());
        if (row.language === "🐹️go") {
          const listed = spawnSync("go", ["list", "-f", "{{.Dir}}", "./..."], { cwd: packageRoot, encoding: "utf8" });
          if (!listed.error && listed.status === 0) {
            const packages = listed.stdout.split("\n").filter(Boolean).map((dir) => dir.slice(packageRoot.length + 1)).filter(Boolean).sort();
            expect(packages).toEqual(["🌨️payloads", "🗄️store/📥️inner"]);
          }
        }
      } finally {
        clearDiscoveryCache();
        rmSync(root, { recursive: true, force: true });
      }
    });
  }

  for (const row of fixture.sourceCases as readonly SourceCaseRow[]) {
    const source = row.source.join("\n") + "\n";
    test(`${row.id}: role`, () => {
      const grammar = loadCatalogTaxonomy().packageGlueGrammar[row.language]!;
      expect(classifyPackageSource(source, grammar).role).toBe(row.expectedRole);
    });
    test(`${row.id}: the language's own formatter parses the source`, () => {
      const parses = formatterParses(row.language, source);
      if (parses !== null) expect(parses).toBe(true);
    });
  }
});
