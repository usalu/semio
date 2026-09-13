import { describe, expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { dirname, join, relative, resolve } from "node:path";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import Ajv from "ajv";
import ts from "typescript";
import { loadConfigFromFile } from "vite";
import { loadCatalogTaxonomy, semanticDirectoryKindId } from "../../🔍️discovery/🟦️.ts";

type Owner = Readonly<{ configurationRoot: string; previousPath: string; ownerPath: string; expectedName: string | null; projectionSha256: string }>;
type Fixture = Readonly<{
  schemaVersion: 1;
  owners: readonly Owner[];
  selector: Readonly<{ libraryDeclaration: string; relativeOwnerSuffix: string; editorSetting: string; editorPattern: string }>;
  directoryContexts: readonly Readonly<{ name: string; parentKind: string; kind: string }>[];
  registration: Readonly<{ name: string; command: string; target: string }>;
}>;

const libraryRoot = resolve(import.meta.dir, "../..");
const repoRoot = resolve(import.meta.dir, "../../../../../../..");
const fixture = JSON.parse(readFileSync(resolve(libraryRoot, "🧫️fixtures/🎚️vitest-configuration-ownership/🔣️.json"), "utf8")) as Fixture;
const schema = JSON.parse(readFileSync(resolve(libraryRoot, "🧬️schema/🎚️vitest-configuration-ownership/🔣️.json"), "utf8"));

function portable(value: unknown): unknown {
  if (typeof value === "string") return value === repoRoot ? "." : value.startsWith(`${repoRoot}/`) ? `./${relative(repoRoot, value).replaceAll("\\", "/")}` : value;
  if (value instanceof RegExp) return { flags: value.flags, regex: value.source };
  if (typeof value === "function") return { function: value.name || "anonymous" };
  if (Array.isArray(value)) return value.map(portable);
  if (value && typeof value === "object") return Object.fromEntries(Object.entries(value).filter(([key]) => key !== "plugins").sort(([left], [right]) => left.localeCompare(right)).map(([key, child]) => [key, portable(child)]));
  return value;
}

function canonical(value: unknown): string {
  return JSON.stringify(portable(value));
}

function projectionHash(config: Record<string, any>, expectedRoot: string): string {
  const projection = {
    aliases: config.resolve?.alias ?? null,
    cacheDir: config.cacheDir ?? null,
    coverageInclude: config.test?.coverage?.include ?? null,
    root: expectedRoot,
    exclude: config.test?.exclude ?? null,
    include: config.test?.include ?? null,
    includeSource: config.test?.includeSource ?? null,
    name: config.test?.name ?? null,
    passWithNoTests: config.test?.passWithNoTests ?? null,
    projects: config.test?.projects ?? null,
    setupFiles: config.test?.setupFiles ?? null,
  };
  return createHash("sha256").update(canonical(projection)).digest("hex");
}

function walkProductFiles(visit: (path: string) => void): void {
  const stack = ["♻️mit-bestand", "✏️s", "🌎️hub", "🧰️framework"].map((path) => resolve(repoRoot, path));
  while (stack.length > 0) {
    const directory = stack.pop()!;
    for (const entry of readdirSync(directory, { withFileTypes: true })) {
      const path = join(directory, entry.name);
      if (entry.isSymbolicLink()) continue;
      if (entry.isDirectory()) {
        if (!["node_modules", "🗑️generated", "dist", "target"].includes(entry.name)) stack.push(path);
      } else visit(path);
    }
  }
}

function routerRoot(path: string): string {
  let candidate = dirname(path);
  while (candidate !== repoRoot && !existsSync(join(candidate, "📜️script.ts"))) candidate = dirname(candidate);
  return candidate;
}

describe("Vitest configuration ownership", () => {
  test("validates the portable owner map and contextual directory kinds", () => {
    const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    expect(validate({ ...fixture, extra: true })).toBe(false);
    expect(fixture.owners).toHaveLength(43);
    expect(new Set(fixture.owners.map(({ ownerPath }) => ownerPath)).size).toBe(43);
    const taxonomy = loadCatalogTaxonomy();
    for (const row of fixture.directoryContexts) expect(semanticDirectoryKindId(row.name, taxonomy, { parentKindId: row.parentKind }), `${row.parentKind}/${row.name}`).toBe(row.kind);
  });

  test("requires anonymous semantic owners and removes every fixed predecessor", () => {
    for (const owner of fixture.owners) {
      expect(owner.ownerPath.includes("/📦️packages/"), owner.ownerPath).toBe(false);
      expect(owner.ownerPath.endsWith(fixture.selector.relativeOwnerSuffix), owner.ownerPath).toBe(true);
      expect(existsSync(resolve(repoRoot, owner.ownerPath)), owner.ownerPath).toBe(true);
      expect(existsSync(resolve(repoRoot, owner.previousPath)), owner.previousPath).toBe(false);
    }
  });

  test("loads every explicit owner through installed Vite and preserves its behavioral projection", async () => {
    await Promise.all(fixture.owners.map(async (owner) => {
      const ownerPath = resolve(repoRoot, owner.ownerPath);
      const expectedRoot = resolve(repoRoot, owner.configurationRoot);
      const loaded = await loadConfigFromFile({ command: "serve", mode: "test" }, ownerPath, expectedRoot, "silent", undefined, "native");
      expect(loaded, owner.ownerPath).not.toBeNull();
      const config = loaded!.config as Record<string, any>;
      expect(config.root, owner.ownerPath).toBe(expectedRoot);
      expect(config.test?.root, owner.ownerPath).toBe(expectedRoot);
      expect(config.test?.name ?? null, owner.ownerPath).toBe(owner.expectedName);
      expect(projectionHash(config, owner.configurationRoot), owner.ownerPath).toBe(owner.projectionSha256);
    }));
  }, 120_000);

  test("requires every runVitest call to select an anonymous semantic configuration", () => {
    const missing: string[] = [];
    const wrong: string[] = [];
    const ownerPaths = new Set(fixture.owners.map(({ ownerPath }) => ownerPath));
    for (const path of [resolve(repoRoot, "📜️script.ts")]) visitSource(path);
    function visitSource(path: string): void {
      if (!path.endsWith(".ts")) return;
      const content = readFileSync(path, "utf8");
      if (!content.includes("runVitest")) return;
      const source = ts.createSourceFile(path, content, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
      const roots = new Map<string, string>();
      const visit = (node: ts.Node) => {
        if (ts.isVariableDeclaration(node) && ts.isIdentifier(node.name) && node.initializer && ts.isCallExpression(node.initializer) && node.initializer.expression.getText(source) === "join" && node.initializer.arguments[0]?.getText(source) === "this.root" && ts.isStringLiteral(node.initializer.arguments[1])) roots.set(node.name.text, resolve(routerRoot(path), node.initializer.arguments[1].text));
        if (ts.isCallExpression(node) && ts.isIdentifier(node.expression) && node.expression.text === "runVitest") {
          const argument = node.arguments[2];
          if (!argument) missing.push(relative(repoRoot, path));
          else if (!ts.isStringLiteral(argument)) wrong.push(`${relative(repoRoot, path)}: ${argument.getText(source)}`);
          else {
            const bundle = node.arguments[0];
            let bundleRoot: string | undefined;
            if (bundle?.getText(source) === "this.root") bundleRoot = routerRoot(path);
            else if (bundle && ts.isCallExpression(bundle) && bundle.expression.getText(source) === "join" && bundle.arguments[0]?.getText(source) === "this.root" && ts.isStringLiteral(bundle.arguments[1])) bundleRoot = resolve(routerRoot(path), bundle.arguments[1].text);
            else if (bundle && ts.isIdentifier(bundle)) bundleRoot = roots.get(bundle.text);
            const selected = bundleRoot && relative(repoRoot, resolve(bundleRoot, argument.text)).replaceAll("\\", "/");
            if (!selected || !ownerPaths.has(selected)) wrong.push(`${relative(repoRoot, path)}: ${argument.text} -> ${selected ?? "unresolved"}`);
          }
        }
        ts.forEachChild(node, visit);
      };
      visit(source);
    }
    walkProductFiles(visitSource);
    expect(missing).toEqual([]);
    expect(wrong).toEqual([]);
    const library = readFileSync(resolve(libraryRoot, "🟦️.ts"), "utf8");
    expect(library).toContain(fixture.selector.libraryDeclaration);
  }, 60_000);

  test("registers exact Nx inputs and the real editor selector", () => {
    const projectSources = [readFileSync(resolve(repoRoot, "📋️project.json"), "utf8")];
    walkProductFiles((path) => { if (path.endsWith("/📋️project.json")) projectSources.push(readFileSync(path, "utf8")); });
    for (const owner of fixture.owners.filter(({ configurationRoot }) => configurationRoot !== ".")) expect(projectSources.some((source) => source.includes(owner.ownerPath)), owner.ownerPath).toBe(true);
    const settings = JSON.parse(readFileSync(resolve(repoRoot, ".vscode/settings.json"), "utf8"));
    expect(settings[fixture.selector.editorSetting]).toBe(fixture.selector.editorPattern);
    expect(settings["vitest.configSearchPattern"]).toBeUndefined();
  }, 30_000);

  test("preserves the deliberate root no-tests configuration", () => {
    const owner = fixture.owners.find(({ configurationRoot }) => configurationRoot === ".")!;
    const source = readFileSync(resolve(repoRoot, owner.ownerPath), "utf8");
    expect(source).toContain("include: []");
    expect(source).toContain("passWithNoTests: false");
    const syntax = ts.createSourceFile(owner.ownerPath, source, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
    const configuration = syntax.statements.find((node) => ts.isExportAssignment(node));
    expect(configuration && ts.isExportAssignment(configuration) && ts.isCallExpression(configuration.expression)).toBe(true);
    const call = (configuration as ts.ExportAssignment).expression as ts.CallExpression;
    const options = call.arguments[0];
    expect(options && ts.isObjectLiteralExpression(options) && options.properties.some((property) => property.name?.getText(syntax) === "projects")).toBe(false);
  });

  test("registers the gate through package, Nx and both launch projections", () => {
    const packageRoot = resolve(libraryRoot, "📦️packages/🟦️typescript");
    const project = JSON.parse(readFileSync(resolve(packageRoot, "📋️project.json"), "utf8"));
    expect(project.targets[fixture.registration.target]?.options?.command).toBe("bun ./📜️script.ts test vitest-configuration-ownership");
    const manifest = JSON.parse(readFileSync(resolve(packageRoot, "package.json"), "utf8"));
    expect(manifest.scripts[fixture.registration.target]).toBe(`nx run @semio-tech/repo-lib:${fixture.registration.target}`);
    for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
      const launch = Bun.JSONC.parse(readFileSync(resolve(repoRoot, path), "utf8")) as { configurations: { name?: string; command?: string }[] };
      expect(launch.configurations.filter(({ name, command }) => name === fixture.registration.name && command === fixture.registration.command), path).toHaveLength(1);
    }
  });
});
