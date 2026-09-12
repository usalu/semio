import { describe, expect, test } from "bun:test";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { basename, dirname, resolve } from "node:path";
import Ajv from "ajv";
import * as TOML from "@iarna/toml";
import ts from "typescript";
import { semanticDirectoryKindId } from "../../🔍️discovery/🟦️.ts";

type Fixture = Readonly<{
  moves: readonly Readonly<{ source: string; owner: string; kind: string; anchor: string }>[];
  extractions: readonly Readonly<{ packageEntry: string; owner: string; maximumPackageLines: number; anchor: string }>[];
  nativeCrates: readonly Readonly<{ manifest: string; library: string; implementation: string; formerRoot: string }>[];
  testOwners: readonly Readonly<{ source: string; owner: string }>[];
  directoryContexts: readonly Readonly<{ name: string; parentKind: string; ancestorKinds?: readonly string[]; kind: string }>[];
  producerContracts: readonly Readonly<{ contract: string; inputs: readonly string[] }>[];
  scriptExtractions: readonly Readonly<{ command: string; owner: string; exports: readonly string[]; context: Readonly<{ parentKind: string; ancestorKinds: readonly string[]; kind: string }> }>[];
  packedFontCases: readonly Readonly<{ name: string; bytes: readonly number[]; count?: number; error?: string }>[];
  scanCoverage: Readonly<{ roots: readonly Readonly<{ former: string; current: string }>[]; hostileCases: readonly Readonly<{ kind: "px" | "color"; root: string; path: string; content: string; expected: string }>[]; overlappingRootCase: Readonly<{ root: string; nestedRoot: string; path: string; content: string; expectedPx: string; expectedColor: string }> }>;
  browserBoundary: Readonly<{ entry: string; builder: string; buildConfig: string; testSentinel: string; browserReachable: readonly string[]; forbiddenImports: readonly string[] }>;
}>;

const fixtureRoot = resolve(import.meta.dir, "../..");
const repoRoot = resolve(import.meta.dir, "../../../../../../..");
const fixture = JSON.parse(readFileSync(resolve(fixtureRoot, "🧫️fixtures/🧱️framework-source-topology/🔣️.json"), "utf8")) as Fixture;
const schema = JSON.parse(readFileSync(resolve(fixtureRoot, "🧬️schema/🧱️framework-source-topology/🔣️.json"), "utf8"));
const expectedBasename: Readonly<Record<string, string>> = { json: "🔣️.json", css: "🎨️.css", javascript: "🟨️.js", rust: "🦀️.rs", typescript: "🟦️.ts", "typescript-jsx": "🟦️.tsx" };

describe("framework source topology", () => {
  test("validates the portable owner projection", () => {
    const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    expect(validate({ ...fixture, extra: true })).toBe(false);
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

  test("registers the portable gate in both editor launch authorities", () => {
    const name = "🧹clean🧩️taxonomy🧪️framework-source-topology";
    const command = "bun nx run @semio-tech/repo-lib:test-framework-source-topology";
    for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
      const launch = Bun.JSONC.parse(readFileSync(resolve(repoRoot, path), "utf8")) as { readonly configurations: readonly { readonly name?: string; readonly command?: string }[] };
      expect(launch.configurations.filter((row) => row.name === name && row.command === command), path).toHaveLength(1);
    }
  });

  test("extracts font, styling, and asset APIs into their semantic owners", async () => {
    const exportedNames = (path: string): ReadonlySet<string> => {
      const source = ts.createSourceFile(path, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true);
      const names = new Set<string>();
      for (const statement of source.statements) {
        if (!statement.modifiers?.some(({ kind }) => kind === ts.SyntaxKind.ExportKeyword)) continue;
        if (ts.isVariableStatement(statement)) {
          for (const declaration of statement.declarationList.declarations) if (ts.isIdentifier(declaration.name)) names.add(declaration.name.text);
        } else if ("name" in statement && statement.name && ts.isIdentifier(statement.name)) {
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
    expect(verification.PX_SCAN_ROOTS).toEqual(fixture.scanCoverage.roots.map(({ current }) => current));
    expect(verification.COLOR_SCAN_ROOTS).toEqual(fixture.scanCoverage.roots.filter(({ former }) => former !== "framework/module/ui/styling/js").map(({ current }) => current).concat(".storybook"));
    for (const row of fixture.scanCoverage.roots) {
      expect(existsSync(resolve(repoRoot, row.former)), row.former).toBe(false);
      expect(existsSync(resolve(repoRoot, row.current)), row.current).toBe(true);
    }
    const sandbox = mkdtempSync(resolve(tmpdir(), "semio-style-scan-"));
    try {
      for (const row of fixture.scanCoverage.hostileCases) {
        const path = resolve(sandbox, row.root, row.path);
        mkdirSync(dirname(path), { recursive: true });
        writeFileSync(path, row.content);
        const violations = row.kind === "px" ? verification.collectPxViolations(sandbox, [row.root]) : verification.collectColorViolations(sandbox, [row.root]);
        expect(violations.map(({ kind }: { kind: string }) => kind), row.path).toContain(row.expected);
      }
      const overlap = fixture.scanCoverage.overlappingRootCase;
      const overlapPath = resolve(sandbox, overlap.root, overlap.nestedRoot, overlap.path);
      mkdirSync(dirname(overlapPath), { recursive: true });
      writeFileSync(overlapPath, overlap.content);
      const repeatedRoots = [overlap.root, resolve(overlap.root, overlap.nestedRoot), overlap.root];
      expect(verification.collectPxViolations(sandbox, repeatedRoots).map(({ kind }: { kind: string }) => kind)).toEqual([overlap.expectedPx]);
      expect(verification.collectColorViolations(sandbox, repeatedRoots).map(({ kind }: { kind: string }) => kind)).toEqual([overlap.expectedColor]);
    } finally {
      rmSync(sandbox, { recursive: true, force: true });
    }

    const logo = await import(resolve(repoRoot, fixture.scriptExtractions[7]!.owner));
    const logoRoot = resolve(repoRoot, "🧰️framework/🔨️modules/🖼️assets/🪧️logos");
    const keyframeRoot = resolve(logoRoot, "🎞️animation");
    const expectedFrames = readdirSync(keyframeRoot, { withFileTypes: true }).filter((entry) => entry.isDirectory() && /^[1-6]/.test(entry.name)).map((entry) => resolve(keyframeRoot, entry.name, "🖋️vector.svg")).sort();
    expect(logo.logoKeyframePaths(logoRoot)).toEqual(expectedFrames);
  });
});
