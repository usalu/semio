import { describe, expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { pathToFileURL } from "node:url";
import Ajv from "ajv";
import ts from "typescript";
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

function behavioralProjection(config: Record<string, any>, expectedRoot: string): Record<string, unknown> {
  return {
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
}

/** 🌍️ A quoted value that opens with a POSIX root or a Windows drive: the only way an environment-derived
 * path can survive `portable()` and pin `projectionSha256` to the machine that wrote the fixture. */
const ABSOLUTE_PATH_IN_PROJECTION = /"(?:\/|[A-Za-z]:[\\/])[^"]*"/u;

function projectionHash(config: Record<string, any>, expectedRoot: string): string {
  return createHash("sha256").update(canonical(behavioralProjection(config, expectedRoot))).digest("hex");
}

/** 🚫️ Directory names that hold neither a `runVitest` call site nor a collectable test file: installed
 * dependencies, generated and build output, VCS and repo-tool state. `.🧬semio` carries the shared cargo
 * build dir and every ticket folder, and `🗑️generated` the staged plugin bundles, so walking either
 * costs more than the rest of the repository together. */
const UNWALKED_DIRECTORIES: ReadonlySet<string> = new Set(["node_modules", "dist", "target", "storybook-static", "🗑️generated", "🤖️generated", "🤖️jco", ".git", ".🧬semio", "⚡️cache", "temp"]);

let repositoryTreeCache: string[] | undefined;

/** 🗂️ Every file in the repository, absolute. Walked once — three separate walks of this tree were the
 * largest single cost in this gate, and each one is the same traversal. */
function repositoryTree(): string[] {
  if (repositoryTreeCache !== undefined) return repositoryTreeCache;
  const files: string[] = [];
  const stack = [repoRoot];
  while (stack.length > 0) {
    const directory = stack.pop()!;
    let entries;
    try {
      entries = readdirSync(directory, { withFileTypes: true });
    } catch {
      continue;
    }
    for (const entry of entries) {
      if (entry.isSymbolicLink()) continue;
      const path = join(directory, entry.name);
      if (entry.isDirectory()) {
        if (!UNWALKED_DIRECTORIES.has(entry.name)) stack.push(path);
      } else files.push(path);
    }
  }
  repositoryTreeCache = files;
  return files;
}

const PRODUCT_TREE_ROOTS = ["♻️mit-bestand", "✏️s", "🌎️hub", "🧰️framework"].map((path) => `${resolve(repoRoot, path)}/`);

function walkProductFiles(visit: (path: string) => void): void {
  for (const path of repositoryTree()) if (PRODUCT_TREE_ROOTS.some((root) => path.startsWith(root))) visit(path);
}

/** 📥️ The user configuration an owner module declares, resolved the way Vitest resolves it.
 *
 * 🩸️ Loaded by plain `import()`, not `vite.loadConfigFromFile`. Both yield the same object — Vite's
 * loader esbuild-bundles the file and evaluates it, and `defineConfig` is the identity function, so the
 * module's default export *is* the user config — but the bundle step costs seconds per owner and,
 * across 46 owners under fleet load, pushed this gate past the 120 s `runTestBudgeted` ceiling while
 * proving nothing the import does not. An owner exporting a config *function* is called with the same
 * `test`-mode environment Vitest passes. */
async function ownerConfiguration(ownerPath: string): Promise<Record<string, any>> {
  const module = (await import(pathToFileURL(ownerPath).href)) as { default?: unknown };
  const exported = module.default;
  const resolved = typeof exported === "function" ? await (exported as (environment: unknown) => unknown)({ command: "serve", mode: "test", isSsrBuild: false, isPreview: false }) : exported;
  return resolved as Record<string, any>;
}

/** 🕳️ Owner patterns that select nothing on disk — the suite an owner believes it runs and does not.
 *
 * 🩸️ A `📜️script.ts test` target whose config collects nothing is the repo's most expensive silent
 * failure: `@semio-tech/framework` shipped a config whose `includeSource` reached zero files, and
 * `…/📅️33.projektetage` selected `"🧪️vitest.config.ts"`, a path that has never existed. Neither is
 * visible from a green `test` target, because a pattern that matches nothing is not an error to
 * vitest — it simply narrows the run. Patterns resolve against the owner's own `test.root`, and many
 * legitimately climb above it (`../../📥️cold-pair/🟦️.ts`), so they are matched absolute. */
function patternsSelectingNothing(config: Record<string, any>, ownerRoot: string): string[] {
  const patterns: string[] = [...(config.test?.include ?? []), ...(config.test?.includeSource ?? [])];
  const files = repositoryTree();
  return patterns.filter((pattern) => {
    const glob = new Bun.Glob(isAbsolute(pattern) ? pattern : resolve(ownerRoot, pattern).replaceAll("\\", "/"));
    return !files.some((file) => glob.match(file));
  });
}

/** 🧾️ Absolute file patterns one `📋️project.json` declares as cache inputs, from every `namedInputs`
 * entry and every target's own `inputs`.
 *
 * 🩸️ Read as JSON and expanded, not searched as text. A project may declare an owner literally
 * (`{workspaceRoot}/…/🧪️tests/🎚️config/🟦️.ts`) or by glob (`{workspaceRoot}/…/🧪️tests/**\/*`), and both
 * make Nx re-run the target when the config changes — a substring search sees only the first and
 * reported `@semio-tech/framework-replication` as unregistered the moment its explicit list became an
 * equivalent glob. Negations and non-path tokens (`default`, `^default`, `{ externalDependencies }`)
 * carry no file and are dropped. */
function nxInputPatterns(projectPath: string): string[] {
  let project: Record<string, any>;
  try {
    project = JSON.parse(readFileSync(projectPath, "utf8"));
  } catch {
    return [];
  }
  const projectRoot = dirname(projectPath);
  const declared: unknown[] = [...Object.values(project.namedInputs ?? {}).flatMap((entry) => (Array.isArray(entry) ? entry : [])), ...Object.values(project.targets ?? {}).flatMap((target: any) => (Array.isArray(target?.inputs) ? target.inputs : []))];
  return declared
    .filter((entry): entry is string => typeof entry === "string" && entry.includes("/") && !entry.startsWith("!"))
    .map((entry) => (entry.startsWith("{workspaceRoot}/") ? join(repoRoot, entry.slice("{workspaceRoot}/".length)) : entry.startsWith("{projectRoot}/") ? join(projectRoot, entry.slice("{projectRoot}/".length)) : join(projectRoot, entry)).replaceAll("\\", "/"));
}

/** 🧭️ The `this.root` a `BundleScript` in `path` runs with. A router lives in a `📜️script.ts`, but its
 * `*Script` classes are routinely extracted into sibling semantic owners (`…/🏃️execution/🟦️.ts`), which
 * have no `📜️script.ts` above them — the bundle that imports them is the `📦️packages/🟦️typescript` of the
 * nearest semantic ancestor. */
function routerRoot(path: string): string {
  let candidate = dirname(path);
  while (candidate !== repoRoot) {
    if (existsSync(join(candidate, "📜️script.ts"))) return candidate;
    const bundle = join(candidate, "📦️packages", "🟦️typescript");
    if (existsSync(join(bundle, "📜️script.ts"))) return bundle;
    candidate = dirname(candidate);
  }
  return repoRoot;
}

describe("Vitest configuration ownership", () => {
  test("validates the portable owner map and contextual directory kinds", () => {
    const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    expect(validate({ ...fixture, extra: true })).toBe(false);
    expect(fixture.owners).toHaveLength(46);
    expect(new Set(fixture.owners.map(({ ownerPath }) => ownerPath)).size).toBe(46);
    const taxonomy = loadCatalogTaxonomy();
    for (const row of fixture.directoryContexts) expect(semanticDirectoryKindId(row.name, taxonomy, { parentKindId: row.parentKind }), `${row.parentKind}/${row.name}`).toBe(row.kind);
  }, 30_000);

  test("requires anonymous semantic owners and removes every fixed predecessor", () => {
    for (const owner of fixture.owners) {
      expect(owner.ownerPath.includes("/📦️packages/"), owner.ownerPath).toBe(false);
      expect(owner.ownerPath.endsWith(fixture.selector.relativeOwnerSuffix), owner.ownerPath).toBe(true);
      expect(existsSync(resolve(repoRoot, owner.ownerPath)), owner.ownerPath).toBe(true);
      expect(existsSync(resolve(repoRoot, owner.previousPath)), owner.previousPath).toBe(false);
    }
  });

  test("loads every explicit owner and preserves its behavioral projection", async () => {
    await Promise.all(fixture.owners.map(async (owner) => {
      const ownerPath = resolve(repoRoot, owner.ownerPath);
      const expectedRoot = resolve(repoRoot, owner.configurationRoot);
      const config = await ownerConfiguration(ownerPath);
      expect(config, owner.ownerPath).not.toBeNull();
      expect(config.root, owner.ownerPath).toBe(expectedRoot);
      expect(config.test?.root, owner.ownerPath).toBe(expectedRoot);
      expect(config.test?.name ?? null, owner.ownerPath).toBe(owner.expectedName);
      const serialized = canonical(behavioralProjection(config, owner.configurationRoot));
      expect(ABSOLUTE_PATH_IN_PROJECTION.exec(serialized)?.[0] ?? null, owner.ownerPath).toBeNull();
      expect(projectionHash(config, owner.configurationRoot), owner.ownerPath).toBe(owner.projectionSha256);
      expect(patternsSelectingNothing(config, expectedRoot), owner.ownerPath).toEqual([]);
      const declared = [...(config.test?.include ?? []), ...(config.test?.includeSource ?? [])].length;
      expect(declared > 0 || owner.configurationRoot === ".", `${owner.ownerPath} collects nothing`).toBe(true);
    }));
  }, 180_000);

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
    const projectFiles = [resolve(repoRoot, "📋️project.json")];
    walkProductFiles((path) => { if (path.endsWith("/📋️project.json")) projectFiles.push(path); });
    const inputs = projectFiles.flatMap((path) => nxInputPatterns(path));
    for (const owner of fixture.owners.filter(({ configurationRoot }) => configurationRoot !== ".")) {
      const target = resolve(repoRoot, owner.ownerPath).replaceAll("\\", "/");
      expect(inputs.some((pattern) => pattern === target || new Bun.Glob(pattern).match(target)), owner.ownerPath).toBe(true);
    }
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
