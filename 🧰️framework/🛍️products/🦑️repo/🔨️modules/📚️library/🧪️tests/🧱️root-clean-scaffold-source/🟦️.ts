import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
import Ajv, { type AnySchema } from "ajv";
import ts from "typescript";
import { loadTaxonomy, semanticDirectoryKindId } from "../../📦️packages/🟦️typescript/🟦️.ts";

/** 🧱️ One extracted owner module: where it lives, the semantic context that names it, and the declarations it owns. */
interface CleanScaffoldOwner {
  readonly path: string;
  readonly directoryName: string;
  readonly parentKindId: string;
  readonly kindId: string;
  readonly declarations: readonly string[];
}

/** 🧭️ One directory whose semantic kind the taxonomy must resolve from its parent. */
interface CleanScaffoldContext {
  readonly directoryName: string;
  readonly parentKindId: string;
  readonly kindId: string;
}

/** 🔗️ One consumer file and the owner modules it must bind to. */
interface CleanScaffoldConsumer {
  readonly path: string;
  readonly owners: readonly string[];
}

/** 🚀️ The single Nx target and the launch row generated from it. */
interface CleanScaffoldRoute {
  readonly target: string;
  readonly command: string;
  readonly launchName: string;
  readonly launchCommand: string;
}

/** 🗃️ The zero-touch artifact directory contract `repoTestArtifactEnvironment` implements. */
interface CleanScaffoldArtifactEnvironment {
  readonly owner: string;
  readonly defaultRelativeRoot: string;
  readonly explicitRelativeRoot: string;
  readonly routes: readonly string[];
}

/** 🧹️ The whole cleanup-and-scaffold ownership fixture, mirroring `🧬️schema/🧱️root-clean-scaffold-source/🔣️.json`. */
interface CleanScaffoldSourceFixture {
  readonly schemaVersion: number;
  readonly owners: readonly CleanScaffoldOwner[];
  readonly contexts: readonly CleanScaffoldContext[];
  readonly consumers: readonly CleanScaffoldConsumer[];
  readonly artifactEnvironment: CleanScaffoldArtifactEnvironment;
  readonly route: CleanScaffoldRoute;
}

/** 🧾️ An Nx project manifest, read only for the one target this contract owns. */
interface NxProjectManifest {
  readonly targets: Readonly<Record<string, { readonly options?: { readonly command?: string } } | undefined>>;
}

/** 📦️ A npm manifest, read only for the script mirroring the Nx target. */
interface NpmPackageManifest {
  readonly scripts: Readonly<Record<string, string | undefined>>;
}

const repoRoot = resolve(import.meta.dir, "../../../../../../../");
const libraryRoot = resolve(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const fixture: CleanScaffoldSourceFixture = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🧫️fixtures/🧱️root-clean-scaffold-source/🔣️.json"), "utf8"));
const schema: AnySchema = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🧬️schema/🧱️root-clean-scaffold-source/🔣️.json"), "utf8"));

function namedDeclarations(path: string): string[] {
  const source = ts.createSourceFile(path, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  return source.statements
    .flatMap((statement) => {
      if ((ts.isFunctionDeclaration(statement) || ts.isTypeAliasDeclaration(statement) || ts.isInterfaceDeclaration(statement) || ts.isClassDeclaration(statement)) && statement.name) return [statement.name.text];
      if (ts.isVariableStatement(statement)) return statement.declarationList.declarations.flatMap((declaration) => (ts.isIdentifier(declaration.name) ? [declaration.name.text] : []));
      return [];
    })
    .sort();
}

function relativeSpecifier(consumer: string, owner: string): string {
  const path = relative(resolve(repoRoot, dirname(consumer)), resolve(libraryRoot, owner)).replaceAll("\\", "/");
  return path.startsWith(".") ? path : `./${path}`;
}

test("validates the portable cleanup and scaffold ownership contract with independent parsers", () => {
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  expect(JSON.parse(JSON.stringify(fixture))).toEqual(fixture);
  expect(fixture.owners).toHaveLength(11);
  expect(new Set(fixture.owners.map((owner) => owner.path)).size).toBe(fixture.owners.length);
  const source = JSON.stringify(fixture);
  expect((ts.parseJsonText("fixture.json", source) as ts.JsonSourceFile & { readonly parseDiagnostics: readonly ts.Diagnostic[] }).parseDiagnostics).toEqual([]);
});

test("resolves every cleanup and scaffold owner through its exact semantic context", () => {
  const taxonomy = loadTaxonomy();
  for (const context of fixture.contexts) expect(semanticDirectoryKindId(context.directoryName, taxonomy, { parentKindId: context.parentKindId }), JSON.stringify(context)).toBe(context.kindId);
  for (const owner of fixture.owners) {
    expect(semanticDirectoryKindId(owner.directoryName, taxonomy, { parentKindId: owner.parentKindId }), owner.path).toBe(owner.kindId);
    expect(owner.path.split("/").at(-1)).toBe("🟦️.ts");
    expect(namedDeclarations(resolve(repoRoot, owner.path))).toEqual([...owner.declarations].sort());
  }
});

test("typechecks every cleanup and scaffold owner with the installed TypeScript compiler", { timeout: 30_000 }, () => {
  const paths = fixture.owners.map((owner) => resolve(repoRoot, owner.path));
  const program = ts.createProgram(paths, {
    target: ts.ScriptTarget.ESNext,
    module: ts.ModuleKind.ESNext,
    moduleResolution: ts.ModuleResolutionKind.Bundler,
    strict: true,
    noUncheckedIndexedAccess: true,
    allowImportingTsExtensions: true,
    skipLibCheck: true,
    noEmit: true,
    types: ["node"],
  });
  const diagnostics = paths.flatMap((path) => {
    const source = program.getSourceFile(path);
    expect(source).toBeDefined();
    return source ? [...program.getSyntacticDiagnostics(source), ...program.getSemanticDiagnostics(source)] : [];
  });
  expect(
    diagnostics.map((diagnostic) => {
      const position = diagnostic.file && diagnostic.start !== undefined ? diagnostic.file.getLineAndCharacterOfPosition(diagnostic.start) : undefined;
      return `${diagnostic.file?.fileName ?? "unknown"}${position ? `:${position.line + 1}:${position.character + 1}` : ""}: ${ts.flattenDiagnosticMessageText(diagnostic.messageText, "\n")}`;
    }),
  ).toEqual([]);
});

test("removes root implementations and binds every direct and source-text consumer", () => {
  const moved = new Set(fixture.owners.flatMap((owner) => owner.declarations));
  expect(namedDeclarations(resolve(repoRoot, "📜️script.ts")).filter((name) => moved.has(name))).toEqual([]);
  for (const consumer of fixture.consumers) {
    const source = readFileSync(resolve(repoRoot, consumer.path), "utf8");
    for (const owner of consumer.owners) expect(source.includes(relativeSpecifier(consumer.path, owner)) || source.includes(owner), `${consumer.path} -> ${owner}`).toBe(true);
  }
});

test("keeps the extracted owner graph acyclic and free of root back imports", () => {
  const owners = new Set(fixture.owners.map((owner) => resolve(repoRoot, owner.path)));
  const edges = new Map<string, string[]>([...owners].map((owner): [string, string[]] => [owner, []]));
  for (const owner of owners) {
    const syntax = ts.createSourceFile(owner, readFileSync(owner, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
    for (const statement of syntax.statements) {
      const specifier = (ts.isImportDeclaration(statement) || ts.isExportDeclaration(statement)) && statement.moduleSpecifier && ts.isStringLiteral(statement.moduleSpecifier) ? statement.moduleSpecifier.text : null;
      if (!specifier?.startsWith(".")) continue;
      const target = resolve(dirname(owner), specifier);
      expect(target).not.toBe(resolve(repoRoot, "📜️script.ts"));
      if (owners.has(target)) edges.get(owner)!.push(target);
    }
  }
  const visiting = new Set<string>(),
    visited = new Set<string>();
  const visit = (owner: string): void => {
    if (visiting.has(owner)) throw new Error(`cleanup/scaffold owner cycle at ${owner}`);
    if (visited.has(owner)) return;
    visiting.add(owner);
    for (const target of edges.get(owner)!) visit(target);
    visiting.delete(owner);
    visited.add(owner);
  };
  for (const owner of owners) visit(owner);
  expect(visited.size).toBe(owners.size);
});

test("allocates a zero-touch current-ticket artifact environment and preserves explicit overrides", () => {
  const scriptPath = resolve(libraryRoot, "📦️packages/🟦️typescript/📜️script.ts"),
    text = readFileSync(scriptPath, "utf8");
  const ownerPath = resolve(repoRoot, fixture.artifactEnvironment.owner),
    ownerText = readFileSync(ownerPath, "utf8");
  const syntax = ts.createSourceFile(ownerPath, ownerText, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const owner = syntax.statements.find((statement) => ts.isFunctionDeclaration(statement) && statement.name?.text === "repoTestArtifactEnvironment");
  expect(owner).toBeDefined();
  const implementationSource = owner!.getText(syntax).replace(/^export /u, "");
  const implementations = [new Bun.Transpiler({ loader: "ts" }).transformSync(implementationSource), ts.transpileModule(implementationSource, { compilerOptions: { target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext } }).outputText].map(
    (code) => new Function("resolve", "mkdirSync", `${code}\nreturn repoTestArtifactEnvironment;`),
  );
  for (const compile of implementations) {
    const created: string[] = [],
      implementation = compile(resolve, (path: string) => created.push(path));
    for (const route of fixture.artifactEnvironment.routes) {
      const actual = implementation(repoRoot, route, {});
      expect(actual.SEMIO_TEST_ARTIFACT_DIR).toBe(resolve(repoRoot, fixture.artifactEnvironment.defaultRelativeRoot, route));
    }
    const explicit = implementation(repoRoot, "taxonomy-cli-cancellation", { SEMIO_TEST_ARTIFACT_DIR: fixture.artifactEnvironment.explicitRelativeRoot, SENTINEL: "preserved" });
    expect(explicit).toEqual({ SEMIO_TEST_ARTIFACT_DIR: resolve(repoRoot, fixture.artifactEnvironment.explicitRelativeRoot), SENTINEL: "preserved" });
    expect(created).toHaveLength(fixture.artifactEnvironment.routes.length + 1);
  }
  for (const route of fixture.artifactEnvironment.routes) expect(text).toContain(`repoTestArtifactEnvironment(this.repoRoot, "${route}")`);
});

test("registers one Bun Nx and seed-derived launch route", () => {
  const project: NxProjectManifest = JSON.parse(readFileSync(resolve(libraryRoot, "📦️packages/🟦️typescript/📋️project.json"), "utf8"));
  const packageJson: NpmPackageManifest = JSON.parse(readFileSync(resolve(libraryRoot, "📦️packages/🟦️typescript/package.json"), "utf8"));
  expect(project.targets[fixture.route.target]?.options?.command).toBe(fixture.route.command);
  expect(packageJson.scripts[fixture.route.target]).toBe(`nx run @semio-tech/repo-lib:${fixture.route.target}`);
  for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
    const source = readFileSync(resolve(repoRoot, path), "utf8");
    expect(source.split(fixture.route.launchName).length - 1).toBe(1);
    expect(source).toContain(fixture.route.launchCommand);
  }
});
