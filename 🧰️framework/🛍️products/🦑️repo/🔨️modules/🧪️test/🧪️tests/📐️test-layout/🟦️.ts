import { describe, expect, test } from "bun:test";
import Ajv from "ajv";
import { parse as parseToml } from "@iarna/toml";
import { minimatch } from "minimatch";
import { build } from "esbuild";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { tmpdir } from "node:os";
import { pathToFileURL } from "node:url";
import { spawnSync } from "node:child_process";
import ts from "typescript";
import { inspectTestLayoutSources, repoRootFromHere, scanTestLayout, TEST_LAYOUT_FINDING_CODES, testTaxonomy, validateCaseContract, type DiscoveredCase, type OracleRegistry, type TestLayoutFinding, type TestLayoutSource } from "../../📦️packages/🟦️typescript/🟦️.ts";
import protocol from "../../🧬️schema/🔣️.json";
import vectors from "../../🧫️fixtures/📐️test-layout/🔣️.json";

type Expected = Readonly<{ code: string; path: string; line: number | null }>;
type VectorCase = Readonly<{ id: string; sources: readonly TestLayoutSource[]; directories?: readonly string[]; expected: readonly Expected[] }>;

const identity = (finding: Expected | TestLayoutFinding): Expected => ({ code: finding.code, path: finding.path, line: finding.line });
const sort = (findings: readonly Expected[]): Expected[] => [...findings].sort((left, right) => left.path.localeCompare(right.path) || left.code.localeCompare(right.code) || (left.line ?? 0) - (right.line ?? 0));
const taxonomy = testTaxonomy(repoRootFromHere());

describe("📐️ canonical test layout", () => {
  test("language-neutral vectors satisfy their schema", () => {
    const compiler = new Ajv({ allErrors: true, strict: false });
    compiler.addSchema(protocol);
    const validate = compiler.getSchema(`${protocol.$id}#/$defs/TestLayoutCases`)!;
    expect(validate(vectors)).toBe(true);
    expect(validate.errors).toBeNull();
    expect(protocol.$defs.TestLayoutFindingCode.enum).toEqual([...TEST_LAYOUT_FINDING_CODES]);
  });

  for (const vector of vectors.cases as readonly VectorCase[]) test(vector.id, () => {
    expect(sort(inspectTestLayoutSources(taxonomy, vector.sources, vector.directories).map(identity))).toEqual(sort(vector.expected));
  });

  test("negative Vitest boot gates agree with TypeScript unary-expression polarity", () => {
    const source = (vectors.cases as readonly VectorCase[]).find(({ id }) => id === "production-negative-vitest-gate")!.sources[0]!;
    const root = ts.createSourceFile(source.path, source.source, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
    let negativeGate = false;
    const visit = (node: ts.Node): void => {
      if (ts.isPropertyAccessExpression(node) && node.name.text === "vitest" && ts.isMetaProperty(node.expression)) negativeGate = ts.isPrefixUnaryExpression(node.parent) && node.parent.operator === ts.SyntaxKind.ExclamationToken;
      ts.forEachChild(node, visit);
    };
    visit(root);
    expect(negativeGate).toBe(true);
    expect(inspectTestLayoutSources(taxonomy, [source]).some(({ code }) => code === "inline-test-body")).toBe(false);
  });

  test("legacy JavaScript suffix classification matches minimatch", () => {
    const sources = (vectors.cases as readonly VectorCase[]).flatMap((vector) => vector.sources);
    const findings = inspectTestLayoutSources(taxonomy, sources);
    for (const path of vectors.filenameOracle.paths) expect(findings.some((finding) => finding.path === path && finding.code === "legacy-test-filename")).toBe(minimatch(path, vectors.filenameOracle.pattern));
  });

  test("legacy fixture directories agree with minimatch", () => {
    const patterns = taxonomy.testFixtureLegacyDirectoryNames.map(name => `**/${name}/**`);
    for (const vector of vectors.cases.filter(row => row.id.startsWith("fixture-directory-"))) for (const source of vector.sources) {
      const opaque = minimatch(source.path, `**/${taxonomy.testFixturesDirName}/**`, { dot: true });
      const implementation = minimatch(source.path, `**/${taxonomy.testsDirName}/*/{🟦️.ts,🦀️.rs,🐹️.go,🐍️.py}`, { dot: true });
      const observed = !opaque && !implementation && patterns.some(pattern => minimatch(source.path, pattern, { dot: true }));
      expect(inspectTestLayoutSources(taxonomy, vector.sources).some(finding => finding.path === source.path && finding.code === "legacy-fixture-directory")).toBe(observed);
    }
  });

  test("obsolete testing categories agree with independent segment matching", () => {
    const stemPattern = `{${taxonomy.testObsoleteCategoryStems.join(",")}}`, testEmojiStemPattern = `{${taxonomy.testObsoleteTestEmojiCategoryStems.join(",")}}`;
    const testEmoji = taxonomy.testsDirName.slice(0, taxonomy.testsDirName.length - "tests".length), canonical = new Map([["tests", taxonomy.testsDirName], ["fixtures", taxonomy.testFixturesDirName], ["examples", taxonomy.testExamplesDirName], ["oracles", taxonomy.testOraclesDirName]]);
    const segmenter = new Intl.Segmenter("und", { granularity: "grapheme" });
    const withoutEmoji = (segment: string): string => {
      const first = segmenter.segment(segment)[Symbol.iterator]().next().value?.segment ?? "";
      return /\p{Extended_Pictographic}|\p{Emoji_Presentation}/u.test(first) ? segment.slice(first.length) : segment;
    };
    for (const vector of (vectors.cases as readonly VectorCase[]).filter(row => row.id.includes("testing-categor"))) for (const candidate of [...vector.sources.map(({ path }) => ({ path, directory: false })), ...(vector.directories ?? []).map((path) => ({ path, directory: true }))]) {
      const pathDirectories = candidate.directory ? candidate.path.split("/") : candidate.path.split("/").slice(0, -1), opaque = pathDirectories.includes(taxonomy.testFixturesDirName) || pathDirectories.includes(taxonomy.exampleAssetsDirName);
      const observed = !opaque && pathDirectories.some((segment, index) => {
        if (index > 0 && pathDirectories[index - 1] === taxonomy.testsDirName) return false;
        if (taxonomy.testLegacyDirectoryNames.includes(segment) || taxonomy.testFixtureLegacyDirectoryNames.includes(segment)) return false;
        const stem = withoutEmoji(segment), expected = canonical.get(stem);
        return minimatch(stem, stemPattern) || (segment.startsWith(testEmoji) && minimatch(stem, testEmojiStemPattern)) || (expected !== undefined && segment !== expected);
      });
      const findings = inspectTestLayoutSources(taxonomy, candidate.directory ? [] : [{ path: candidate.path, source: "" }], candidate.directory ? [candidate.path] : []);
      expect(findings.some(finding => finding.code === "obsolete-testing-category"), candidate.path).toBe(observed);
    }
  });

  test("empty legacy test and fixture directories agree with minimatch", () => {
    const vector = (vectors.cases as readonly VectorCase[]).find(({ id }) => id === "empty-obsolete-testing-category-directories")!;
    const testPatterns = taxonomy.testLegacyDirectoryNames.map((name) => `**/${name}`), fixturePatterns = taxonomy.testFixtureLegacyDirectoryNames.map((name) => `**/${name}`);
    for (const path of vector.directories ?? []) {
      const findings = inspectTestLayoutSources(taxonomy, [], [path]);
      expect(findings.some(({ code }) => code === "legacy-test-directory"), path).toBe(testPatterns.some((pattern) => minimatch(path, pattern, { dot: true })));
      const opaque = minimatch(path, `**/${taxonomy.testFixturesDirName}/**`, { dot: true });
      expect(findings.some(({ code }) => code === "legacy-fixture-directory"), path).toBe(!opaque && fixturePatterns.some((pattern) => minimatch(path, pattern, { dot: true })));
    }
  });

  test("the filesystem scan reports empty obsolete testing directories", async () => {
    const vector = (vectors.cases as readonly VectorCase[]).find(({ id }) => id === "empty-obsolete-testing-category-directories")!;
    const root = mkdtempSync(join(tmpdir(), "test-layout-empty-directories-")), taxonomyPath = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json";
    try {
      mkdirSync(join(root, dirname(taxonomyPath)), { recursive: true });
      writeFileSync(join(root, taxonomyPath), JSON.stringify(taxonomy));
      for (const path of vector.directories ?? []) mkdirSync(join(root, path), { recursive: true });
      const paths = new Set(vector.directories), observed = (await scanTestLayout(root)).filter(({ path }) => paths.has(path)).map(identity);
      expect(sort(observed)).toEqual(sort(vector.expected));
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("fixture manifests agree with independent TOML parsing and Cargo package discovery", () => {
    for (const vector of vectors.cases.filter(row => row.id.startsWith("fixture-manifest-") && row.sources[0]!.path.endsWith("Cargo.toml"))) expect(Bun.TOML.parse(vector.sources[0]!.source)).toEqual(parseToml(vector.sources[0]!.source));
    const vector = vectors.cases.find(row => row.id === "fixture-manifest-workspace-fixture")!;
    const root = mkdtempSync(join(tmpdir(), "semio-fixture-manifest-"));
    try {
      for (const source of [...vector.sources, { path: "owner/🧫️fixtures/guest/src/lib.rs", source: "pub const VALUE: i32 = 1;" }]) {
        const path = join(root, source.path);
        mkdirSync(dirname(path), { recursive: true });
        writeFileSync(path, source.source);
      }
      const observed = spawnSync("cargo", ["metadata", "--offline", "--no-deps", "--format-version", "1", "--manifest-path", join(root, "Cargo.toml")], { cwd: root, encoding: "utf8", timeout: 15000 });
      expect(observed.status, observed.stderr).toBe(0);
      const packages = JSON.parse(observed.stdout).packages as { manifest_path: string }[];
      expect(packages).toHaveLength(1);
      expect(packages[0]!.manifest_path.replaceAll("\\", "/").split("/")).toContain(taxonomy.testFixturesDirName);
      expect(inspectTestLayoutSources(taxonomy, vector.sources).some(finding => finding.code === "production-fixture-dependency")).toBe(false);
    } finally { rmSync(root, { recursive: true, force: true }); }
  });

  test("fixture file reads agree with Node filesystem and esbuild module execution", async () => {
    const directory = mkdtempSync(join(tmpdir(), "semio-fixture-read-"));
    try {
      for (const vector of vectors.cases.filter(row => /^fixture-read-(?:node-url|node-alias|node-namespace|asset-url)$/u.test(row.id))) {
        const root = join(directory, vector.id);
        for (const source of vector.sources) {
          const path = join(root, source.path);
          mkdirSync(dirname(path), { recursive: true });
          writeFileSync(path, source.source);
        }
        const entry = join(root, vector.sources[0]!.path);
        const output = join(dirname(entry), "📜️script.mjs");
        await build({ entryPoints: [entry], outfile: output, bundle: true, platform: "node", format: "esm", logLevel: "silent" });
        const observed = spawnSync(process.execPath.includes("bun") ? "node" : process.execPath, ["--input-type=module", "--eval", `import { value } from ${JSON.stringify(pathToFileURL(output).href)}; process.stdout.write(value);`], { encoding: "utf8" });
        expect(observed.status, observed.stderr).toBe(0);
        expect(observed.stdout).toBe("{}");
        expect(inspectTestLayoutSources(taxonomy, vector.sources).some(finding => finding.code === "production-fixture-dependency")).toBe(vector.sources[1]!.path.split("/").includes(taxonomy.testFixturesDirName));
      }
    } finally { rmSync(directory, { recursive: true, force: true }); }
  });

  test("fixture import boundaries agree with esbuild's resolved dependency graph", async () => {
    const directory = mkdtempSync(join(tmpdir(), "semio-fixture-import-"));
    try {
      for (const vector of vectors.cases.filter(row => /^production-typescript-(?:fixture-import|fixture-reexport|asset-import)$/u.test(row.id))) {
        const root = join(directory, vector.id);
        for (const source of vector.sources) {
          const path = join(root, source.path);
          mkdirSync(dirname(path), { recursive: true });
          writeFileSync(path, source.source);
        }
        const result = await build({ entryPoints: [vector.sources[0]!.path], absWorkingDir: root, bundle: true, write: false, metafile: true, platform: "node", format: "esm", logLevel: "silent" });
        const fixture = Object.keys(result.metafile!.inputs).some(path => path.split("/").includes(taxonomy.testFixturesDirName));
        expect(inspectTestLayoutSources(taxonomy, vector.sources).some(finding => finding.code === "production-fixture-dependency")).toBe(fixture);
      }
    } finally { rmSync(directory, { recursive: true, force: true }); }
  });

  test("Rust module path bases agree with the compiler", () => {
    const directory = mkdtempSync(join(tmpdir(), "semio-test-layout-"));
    try {
      for (const vector of (vectors.cases as readonly VectorCase[]).filter(vector => vector.id.startsWith("rust-module-base-"))) {
        const root = join(directory, vector.id);
        for (const source of vector.sources) {
          const path = join(root, source.path);
          mkdirSync(dirname(path), { recursive: true });
          writeFileSync(path, source.source);
        }
        const binary = join(root, process.platform === "win32" ? "oracle.exe" : "oracle");
        const compiled = spawnSync("rustc", ["--crate-name", "test_layout_path_oracle", "--edition=2024", "--test", join(root, vector.sources[0]!.path), "-o", binary], { cwd: repoRootFromHere(), encoding: "utf8" });
        const valid = !vector.expected.some(finding => finding.code === "invalid-test-module-wiring");
        expect(compiled.status === 0, compiled.stderr).toBe(valid);
        if (valid) {
          const executed = spawnSync(binary, ["--test-threads=1"], { encoding: "utf8" });
          expect(executed.status, executed.stderr).toBe(0);
          expect(executed.stdout).toContain("1 passed");
        }
      }
    } finally { rmSync(directory, { recursive: true, force: true }); }
  }, 60_000);

  test("lexical binding classification matches the TypeScript checker", () => {
    const vector = vectors.cases.find(vector => vector.id === "javascript-lexical-bindings")!;
    for (const source of vector.sources.filter(source => !/namespace-shadow|modified-register|require-alias|expect-shadow/u.test(source.path))) {
      const file = ts.createSourceFile(source.path, source.source, ts.ScriptTarget.Latest, true, ts.ScriptKind.JS);
      const options: ts.CompilerOptions = { allowJs: true, noLib: true, noResolve: true };
      const host = ts.createCompilerHost(options);
      host.getSourceFile = name => name === source.path ? file : undefined;
      host.fileExists = name => name === source.path;
      host.readFile = name => name === source.path ? source.source : undefined;
      const checker = ts.createProgram([source.path], options, host).getTypeChecker(), registrars = new Set<ts.Declaration>();
      for (const statement of file.statements) if (ts.isImportDeclaration(statement) && ts.isStringLiteralLike(statement.moduleSpecifier) && taxonomy.testJavaScriptFrameworkModules.includes(statement.moduleSpecifier.text)) {
        const bindings = statement.importClause?.namedBindings;
        if (bindings && ts.isNamedImports(bindings)) for (const element of bindings.elements) if ((element.propertyName ?? element.name).text === "test") registrars.add(element);
      }
      const lines: number[] = [];
      const visit = (node: ts.Node): void => {
        if (ts.isCallExpression(node) && ts.isIdentifier(node.expression)) {
          const declarations = checker.getSymbolAtLocation(node.expression)?.declarations;
          if (declarations?.some(declaration => registrars.has(declaration)) || !declarations && node.expression.text === "test") lines.push(file.getLineAndCharacterOfPosition(node.getStart(file)).line + 1);
        }
        ts.forEachChild(node, visit);
      };
      visit(file);
      const expected = vector.expected.find(finding => finding.path === source.path)?.line;
      expect(lines[0]).toBe(expected);
      expect(inspectTestLayoutSources(taxonomy, [source]).find(finding => finding.code === "inline-test-body")?.line).toBe(lines[0]);
    }
  });

  test("feature contracts and implementation paths share the canonical case names", () => {
    const owner = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test", caseDir = `${owner}/🧪️tests/🖥️host-protocol-parity`;
    const registry: OracleRegistry = { schemaVersion: 1, oracles: [], probes: [], noOracleDecisions: [], comparisonProfiles: [], comparisonPipelines: [], toleranceProfiles: [], oracleHostPackages: [], mutationCatalogs: [], mutationManifests: [], fixtureManifests: [], contributions: [] };
    const discovered: DiscoveredCase = { owner, ownerName: "test", case: "", caseDir, featurePath: `${caseDir}/🥒️.feature`, adapters: {}, sharedFixtureDir: null, projectName: "layout-fixture" };
    const names = new Map<string, boolean>();
    for (const vector of vectors.cases as readonly VectorCase[]) for (const source of vector.sources) {
      const name = /\/🧪️tests\/([^/]+)\/🟦️\.ts$/u.exec(source.path)?.[1];
      if (name) names.set(name, !vector.expected.some(finding => finding.path === source.path && finding.code === "test-case-name"));
    }
    for (const [name, accepted] of names) {
      const findings = validateCaseContract(repoRootFromHere(), { ...discovered, case: name }, registry);
      expect(findings.every(finding => finding.id !== "case-slug")).toBe(accepted);
    }
    for (const vector of vectors.cases as readonly VectorCase[]) for (const finding of vector.expected.filter(finding => finding.code === "test-owner-delivery-scope")) {
      const sourceOwner = finding.path.slice(0, finding.path.indexOf("/🧪️tests/"));
      const findings = validateCaseContract(repoRootFromHere(), { ...discovered, owner: sourceOwner, case: "🧪️case" }, registry);
      expect(findings.some(finding => finding.id === "case-in-delivery-scope")).toBe(true);
    }
  });

  test("Nx hashes semantic-owner cases while production excludes them", async () => {
    const repoRoot = repoRootFromHere(), { cacheInternals } = await import("../../../📚️library/🟨️.mjs");
    const targetRoot = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust";
    const owner = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine";
    const project = JSON.parse(readFileSync(join(repoRoot, targetRoot, "📋️project.json"), "utf8"));
    const inputs = cacheInternals.projectInputs(project, targetRoot, repoRoot, new Map());
    expect(inputs.default.some((input: unknown) => typeof input === "string" && input.startsWith(`{workspaceRoot}/${owner}/`))).toBe(true);
    expect(inputs.production).toContain(`!{workspaceRoot}/${owner}/**/🧪️tests/**/*`);
    const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library", libraryRoot = `${library}/📦️packages/🟦️typescript`;
    const libraryProject = JSON.parse(readFileSync(join(repoRoot, libraryRoot, "📋️project.json"), "utf8"));
    const libraryInputs = cacheInternals.projectInputs(libraryProject, libraryRoot, repoRoot, new Map());
    expect(libraryInputs.default).not.toContain(`!{workspaceRoot}/${library}/**/🧪️tests/**/*`);
    expect(libraryInputs.production).toContain(`!{workspaceRoot}/${library}/**/🧪️tests/**/*`);
  });

  test("Nx discovers the same canonical names and semantic owners as the layout vectors", async () => {
    const { default: plugin } = await import("../../🟨️.mjs");
    const cases = new Map<string, boolean>();
    for (const vector of vectors.cases as readonly VectorCase[]) for (const source of vector.sources) {
      if (!/^.+\/🧪️tests\/[^/]+\/🟦️\.ts$/u.test(source.path)) continue;
      const accepted = !vector.expected.some(finding => finding.path === source.path && ["test-case-name", "test-owner-delivery-scope", "test-layout-depth"].includes(finding.code));
      cases.set(source.path.replace(/🟦️\.ts$/u, "🥒️.feature"), accepted);
    }
    for (const [path, accepted] of cases) {
      const discovered = await plugin.createNodesV2[1]([path], {}, { workspaceRoot: repoRootFromHere() });
      expect(discovered.length > 0, path).toBe(accepted);
    }
  });
});
