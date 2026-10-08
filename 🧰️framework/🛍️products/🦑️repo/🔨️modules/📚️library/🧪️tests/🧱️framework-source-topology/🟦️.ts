import { describe, expect, test } from "bun:test";
import { cpSync, existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { basename, dirname, posix, resolve } from "node:path";
import Ajv from "ajv";
import * as TOML from "@iarna/toml";
import ts from "typescript";
import { inventorySchemaScopes, semanticDirectoryKindId } from "../../🔍️discovery/🟦️.ts";

type Fixture = Readonly<{
  moves: readonly Readonly<{ source: string; owner: string; kind: string; anchor: string }>[];
  extractions: readonly Readonly<{ packageEntry: string; owner: string; maximumPackageLines: number; anchor: string }>[];
  nativeCrates: readonly Readonly<{ manifest: string; library: string; implementation: string; formerRoot: string }>[];
  testOwners: readonly Readonly<{ source: string; owner: string }>[];
  directoryContexts: readonly Readonly<{ name: string; parentKind: string; ancestorKinds?: readonly string[]; kind: string }>[];
  producerContracts: readonly Readonly<{ contract: string; inputs: readonly string[] }>[];
  scriptExtractions: readonly Readonly<{ command: string; owner: string; exports: readonly string[]; context: Readonly<{ parentKind: string; ancestorKinds: readonly string[]; kind: string }> }>[];
  packedFontCases: readonly Readonly<{ name: string; bytes: readonly number[]; count?: number; error?: string }>[];
  scanCoverage: Readonly<{ hostileCases: readonly Readonly<{ kind: "px" | "color"; root: string; path: string; content: string; expected: string }>[]; overlappingRootCase: Readonly<{ root: string; nestedRoot: string; path: string; content: string; expectedPx: string; expectedColor: string }> }>;
  browserBoundary: Readonly<{ entry: string; builder: string; buildConfig: string; testSentinel: string; browserReachable: readonly string[]; forbiddenImports: readonly string[] }>;
}>;

const fixtureRoot = resolve(import.meta.dir, "../..");
const repoRoot = resolve(import.meta.dir, "../../../../../../..");
const fixture = JSON.parse(readFileSync(resolve(fixtureRoot, "🧫️fixtures/🧱️framework-source-topology/🔣️.json"), "utf8")) as Fixture;

const expectedBasename: Readonly<Record<string, string>> = { json: "🔣️.json", css: "🎨️.css", javascript: "🟨️.js", rust: "🦀️.rs", typescript: "🟦️.ts", "typescript-jsx": "🟦️.tsx" };

describe("framework source topology", () => {
  test("validates the portable owner projection", () => {
    
    expect(fixture["version"]).toEqual(1);
    
    const taxonomy = JSON.parse(readFileSync(resolve(fixtureRoot, "🔣️taxonomy.json"), "utf8"));
    for (const row of fixture.producerContracts) {
      const inputs = taxonomy.generatorContracts[row.contract]?.inputPatterns as readonly string[] | undefined;
      for (const input of row.inputs) expect(inputs, `${row.contract}:${input}`).toContain(input);
    }
    for (const row of fixture.directoryContexts) expect(semanticDirectoryKindId(row.name, taxonomy, { parentKindId: row.parentKind, ancestorKindIds: row.ancestorKinds }), row.name).toBe(row.kind);
    const renderTestsOwner = dirname(fixture.testOwners[0]!.owner);
    expect(basename(renderTestsOwner)).toBe("🧪️tests");
    expect(semanticDirectoryKindId(basename(renderTestsOwner), taxonomy, { parentKindId: "ui-render" })).toBe("tests");
    expect(semanticDirectoryKindId(basename(dirname(renderTestsOwner)), taxonomy, { parentKindId: "members-of-modules" })).toBe("ui-render");
    for (const row of fixture.testOwners) {
      expect(dirname(row.owner), row.owner).toBe(renderTestsOwner);
      const context = fixture.directoryContexts.find(({ name, parentKind }) => name === basename(row.owner) && parentKind === "tests");
      expect(context?.kind, row.owner).toStartWith("ui-render-test-");
    }
    const builderOwner = dirname(fixture.browserBoundary.builder);
    expect(semanticDirectoryKindId(basename(dirname(dirname(builderOwner))), taxonomy, { parentKindId: "members-of-modules" })).toBe("styling");
    expect(semanticDirectoryKindId(basename(dirname(builderOwner)), taxonomy, { parentKindId: "styling" })).toBe("builder");
    const builderContext = fixture.directoryContexts.find(({ name, parentKind }) => name === basename(dirname(fixture.browserBoundary.builder)) && parentKind === "builder");
    expect(builderContext?.kind).toBe("ui-styling-builder-vite");
  });

  test("owns implementations as anonymous leaves and keeps packages as glue", () => {
    for (const row of fixture.moves) {
      expect(existsSync(resolve(repoRoot, row.source)), row.source).toBe(false);
      const owner = resolve(repoRoot, row.owner);
      expect(existsSync(owner), row.owner).toBe(true);
      expect(basename(owner)).toBe(expectedBasename[row.kind]);
      expect(readFileSync(owner, "utf8")).toContain(row.anchor);
    }
    for (const row of fixture.extractions) {
      const entry = readFileSync(resolve(repoRoot, row.packageEntry), "utf8");
      const owner = readFileSync(resolve(repoRoot, row.owner), "utf8");
      expect(entry.trimEnd().split("\n").length, row.packageEntry).toBeLessThanOrEqual(row.maximumPackageLines);
      expect(owner, row.owner).toContain(row.anchor);
    }
    for (const row of fixture.testOwners) {
      expect(existsSync(resolve(repoRoot, row.source)), row.source).toBe(false);
      expect(existsSync(resolve(repoRoot, row.owner, "🦀️.rs")), row.owner).toBe(true);
      expect(row.owner).not.toContain("packages-rust");
    }
  });

  test("binds native crates to owner roots and browser imports to a pure facade", async () => {
    for (const row of fixture.nativeCrates) {
      const manifest = TOML.parse(readFileSync(resolve(repoRoot, row.manifest), "utf8")) as { readonly lib?: { readonly path?: string } };
      expect(manifest.lib?.path, row.manifest).toBe(row.library);
      expect(existsSync(resolve(repoRoot, dirname(row.manifest), row.library)), row.library).toBe(true);
      expect(existsSync(resolve(repoRoot, row.formerRoot)), row.formerRoot).toBe(false);
      expect(basename(row.implementation), row.implementation).toBe("🦀️.rs");
      expect(readFileSync(resolve(repoRoot, row.implementation), "utf8").trimEnd().split("\n").length, row.implementation).toBeGreaterThan(10);
    }
    const entry = readFileSync(resolve(repoRoot, fixture.browserBoundary.entry), "utf8");
    for (const denied of fixture.browserBoundary.forbiddenImports) expect(entry, denied).not.toContain(denied);
    const build = await Bun.build({ entrypoints: fixture.browserBoundary.browserReachable.map((path) => resolve(repoRoot, path)), target: "browser", format: "esm", external: ["@semio-tech/framework"], throw: false });
    expect(build.success, build.logs.join("\n")).toBe(true);
    expect(existsSync(resolve(repoRoot, fixture.browserBoundary.builder))).toBe(true);
    expect(readFileSync(resolve(repoRoot, fixture.browserBoundary.buildConfig), "utf8")).toContain(fixture.browserBoundary.testSentinel);
  }, 30_000);

  test("declares the portable gate", () => {
    const project = JSON.parse(readFileSync(resolve(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json"), "utf8"));
    expect(project.targets["test-framework-source-topology"].options.command).toBe("bun ./📜️script.ts test framework-source-topology");
  });

  test("extracts font, styling, and asset APIs into their semantic owners", async () => {
    const exportedNames = (path: string): ReadonlySet<string> => {
      const source = ts.createSourceFile(path, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true);
      const names = new Set<string>();
      for (const statement of source.statements) {
        if (!(ts.canHaveModifiers(statement) && ts.getModifiers(statement)?.some((modifier) => modifier.kind === ts.SyntaxKind.ExportKeyword))) continue;
        if (ts.isVariableStatement(statement)) {
          for (const declaration of statement.declarationList.declarations) if (ts.isIdentifier(declaration.name)) names.add(declaration.name.text);
        } else if ((ts.isFunctionDeclaration(statement) || ts.isClassDeclaration(statement) || ts.isInterfaceDeclaration(statement) || ts.isTypeAliasDeclaration(statement) || ts.isEnumDeclaration(statement)) && statement.name && ts.isIdentifier(statement.name)) {
          names.add(statement.name.text);
        }
      }
      return names;
    };
    const commands = new Set<string>();
    for (const row of fixture.scriptExtractions) {
      const owner = resolve(repoRoot, row.owner);
      expect(basename(owner), row.owner).toBe("🟦️.ts");
      expect(existsSync(owner), row.owner).toBe(true);
      expect(semanticDirectoryKindId(basename(dirname(owner)), JSON.parse(readFileSync(resolve(fixtureRoot, "🔣️taxonomy.json"), "utf8")), { parentKindId: row.context.parentKind, ancestorKindIds: row.context.ancestorKinds }), row.owner).toBe(row.context.kind);
      const exported = exportedNames(owner);
      for (const name of row.exports) expect(exported.has(name), `${row.owner}:${name}`).toBe(true);
      commands.add(row.command);
    }
    for (const command of commands) expect(exportedNames(resolve(repoRoot, command)).size, command).toBe(0);

    const packed = await import(resolve(repoRoot, fixture.scriptExtractions[0]!.owner));
    const nativeCount = (values: readonly number[]): number => {
      const bytes = Buffer.from(values);
      if (bytes.length < 4) throw new Error("Missing packed font count");
      const count = bytes.readUInt32LE(0);
      if (count < 1 || count > 1024) throw new Error("Invalid packed font count");
      let offset = 4;
      for (let index = 0; index < count; index++) {
        if (offset + 4 > bytes.length) throw new Error("Missing packed font extent");
        const size = bytes.readUInt32LE(offset);
        offset += 4;
        if (size < 12 || offset + size > bytes.length) throw new Error("Invalid packed font extent");
        offset += size;
      }
      if (offset !== bytes.length) throw new Error("Trailing packed font bytes");
      return count;
    };
    for (const row of fixture.packedFontCases) {
      if (row.count !== undefined) {
        expect(nativeCount(row.bytes), row.name).toBe(row.count);
        expect(packed.validateFontAsset(Uint8Array.from(row.bytes)), row.name).toBe(row.count);
      } else {
        expect(() => nativeCount(row.bytes), row.name).toThrow(row.error);
        expect(() => packed.validateFontAsset(Uint8Array.from(row.bytes)), row.name).toThrow(row.error);
      }
    }

    const font = await import(resolve(repoRoot, fixture.scriptExtractions[1]!.owner));
    const catalogPath = resolve(repoRoot, "🧰️framework/🔨️modules/🖼️assets/🔤️fonts/📇️catalog.json");
    const catalog = JSON.parse(readFileSync(catalogPath, "utf8"));
    const fontSchema = JSON.parse(readFileSync(resolve(repoRoot, "🧰️framework/🔨️modules/🖼️assets/🔤️fonts/🧬️schema/🔣️.json"), "utf8"));
    expect(new Ajv({ strict: true }).compile(fontSchema)(catalog)).toBe(true);
    expect(font.parseFontCatalog(catalog)).toEqual(catalog);
    const expectedFontSources = catalog.families.reduce((total: number, family: { readonly weights: readonly string[]; readonly subsets: readonly unknown[] }) => total + family.weights.length * family.subsets.length * Object.keys(catalog.encodings).length, 0);
    expect(font.fontCatalogSources(catalog)).toHaveLength(expectedFontSources);

    const verification = await import(resolve(repoRoot, fixture.scriptExtractions[3]!.owner));
    expect(verification.admitStylingScanScopeV1({ roots: [] })).toEqual({ roots: [] });
    for (const row of fixture.scanCoverage.hostileCases) {
      const path = posix.join(row.root, row.path);
      const source = { roots: [row.root], files: [path], readText: () => row.content };
      expect(verification.collectStylingViolationsV1(source, row.kind).map(({ kind }: { kind: string }) => kind), row.path).toContain(row.expected);
    }
    const overlap = fixture.scanCoverage.overlappingRootCase;
    const overlapPath = posix.join(overlap.root, overlap.nestedRoot, overlap.path);
    const source = { roots: [overlap.root, posix.join(overlap.root, overlap.nestedRoot), overlap.root], files: [overlapPath], readText: () => overlap.content };
    expect(verification.collectStylingViolationsV1(source, "px").map(({ kind }: { kind: string }) => kind)).toEqual([overlap.expectedPx]);
    expect(verification.collectStylingViolationsV1(source, "color").map(({ kind }: { kind: string }) => kind)).toEqual([overlap.expectedColor]);

    const logo = await import(resolve(repoRoot, fixture.scriptExtractions[7]!.owner));
    const logoRoot = resolve(repoRoot, "🧰️framework/🔨️modules/🖼️assets/🪧️logos");
    const keyframeRoot = resolve(logoRoot, "🎞️animation");
    const expectedFrames = readdirSync(keyframeRoot, { withFileTypes: true }).filter((entry) => entry.isDirectory() && /^[1-6]/.test(entry.name)).map((entry) => resolve(keyframeRoot, entry.name, "🖋️vector.svg")).sort();
    expect(logo.logoKeyframePaths(logoRoot)).toEqual(expectedFrames);
  });
});

test("Hub WGPU command vectors retain source laws without corpus schema authority", async () => {
  const owner = "🌎️hub/🧪️tests/📺️renderer/🧊️wgpu", path = resolve(repoRoot, owner), source = readFileSync(resolve(path, "📜️script.ts"), "utf8"), vectors = JSON.parse(readFileSync(resolve(path, "🧫️fixtures/🔣️.json"), "utf8"));
  const parsed = ts.createSourceFile("📜️script.ts", source, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS), classes = new Map(parsed.statements.filter(ts.isClassDeclaration).map(node => [node.name?.text, node.getText(parsed)]));
  const routes = new Map<string, string>();
  const visit = (node: ts.Node): void => { if (ts.isCallExpression(node) && ts.isPropertyAccessExpression(node.expression) && node.expression.name.text === "register" && node.arguments.length === 2 && ts.isStringLiteral(node.arguments[0]!) && ts.isIdentifier(node.arguments[1]!)) routes.set(node.arguments[0]!.text, node.arguments[1]!.text); ts.forEachChild(node, visit); };
  visit(parsed);
  expect(new Set(vectors.commands.map(row => row.generalCommand)).size).toBe(vectors.commands.length);
  expect(vectors.commands).toHaveLength(5);
  for (const row of vectors.commands) {
    expect(routes.get(row.command)).toBe(row.className);
    expect(source).toContain(`.register("${row.command}", ${row.className})`);
    expect(classes.has(row.className)).toBe(true);
    for (const law of row.laws) expect(classes.get(row.className)).toContain(law.split("::").at(-1)!);
  }
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw Error("Explicit artifact directory required for bounded Hub WGPU inventory");
  mkdirSync(output, { recursive: true });
  const root = mkdtempSync(resolve(output, "hub-wgpu-corpus-"));
  try {
    cpSync(path, resolve(root, owner), { recursive: true });
    const inventory = inventorySchemaScopes(root, JSON.parse(readFileSync(resolve(fixtureRoot, "🔣️taxonomy.json"), "utf8"))), findings = inventory.diagnostics.filter(row => row.path.startsWith(owner)), scopes = inventory.modules.filter(row => row.modulePath.startsWith(owner));
    writeFileSync(resolve(output, "hub-wgpu-corpus-inventory.json"), JSON.stringify({ owner, scopes, diagnostics: findings, vectors: vectors.commands.length, routes: [...routes] }, null, 2) + "\n");
    expect(findings, "every nested collection schema authority must be absent independent of directory relocation").toEqual([]);
    expect(scopes).toEqual([]);
  } finally { rmSync(root, { recursive: true, force: true }); }
  await (await import(resolve(path, "🧪️tests/🟦️.ts"))).proveHubRendererCommandOwnershipV1(repoRoot);
});
