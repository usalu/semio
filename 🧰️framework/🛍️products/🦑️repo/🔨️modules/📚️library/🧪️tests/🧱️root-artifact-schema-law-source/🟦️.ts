import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
import Ajv from "ajv";
import glob from "fast-glob";
import ts from "typescript";
import { loadTaxonomy, semanticDirectoryKindId } from "../../📦️packages/🟦️typescript/🟦️.ts";

const repoRoot = resolve(import.meta.dir, "../../../../../../../");
const libraryRoot = resolve(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const fixture = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🧫️fixtures/🧱️root-artifact-schema-law-source/🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🧬️schema/🧱️root-artifact-schema-law-source/🔣️.json"), "utf8"));

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

test("validates the portable artifact-schema law ownership contract", () => {
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  expect(JSON.parse(JSON.stringify(fixture))).toEqual(fixture);
  expect(fixture.owners).toHaveLength(11);
  expect(new Set(fixture.owners.map((owner: { path: string }) => owner.path)).size).toBe(fixture.owners.length);
  expect(ts.parseJsonText("fixture.json", JSON.stringify(fixture)).parseDiagnostics).toEqual([]);
});

test("resolves every artifact-schema owner through its exact semantic context", () => {
  const taxonomy = loadTaxonomy();
  for (const context of fixture.contexts) expect(semanticDirectoryKindId(context.directoryName, taxonomy, { parentKindId: context.parentKindId }), JSON.stringify(context)).toBe(context.kindId);
  for (const owner of fixture.owners) {
    expect(semanticDirectoryKindId(owner.directoryName, taxonomy, { parentKindId: owner.parentKindId }), owner.path).toBe(owner.kindId);
    expect(owner.path.split("/").at(-1)).toBe("🟦️.ts");
    expect(namedDeclarations(resolve(repoRoot, owner.path))).toEqual([...owner.declarations].sort());
  }
});

test("typechecks every artifact-schema law owner with the installed TypeScript compiler", { timeout: 30_000 }, () => {
  const paths = fixture.owners.map((owner: { path: string }) => resolve(repoRoot, owner.path));
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
  const diagnostics = paths.flatMap((path: string) => {
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

test("removes root implementations and binds the actual command and oracle consumers", () => {
  const moved = new Set(fixture.owners.flatMap((owner: { declarations: string[] }) => owner.declarations));
  expect(namedDeclarations(resolve(repoRoot, "📜️script.ts")).filter((name) => moved.has(name))).toEqual([]);
  for (const consumer of fixture.consumers) {
    const source = readFileSync(resolve(repoRoot, consumer.path), "utf8");
    for (const owner of consumer.owners) expect(source.includes(relativeSpecifier(consumer.path, owner)) || source.includes(owner), `${consumer.path} -> ${owner}`).toBe(true);
  }
});

test("keeps the artifact-schema owner graph acyclic and free of root back imports", () => {
  const owners = new Set(fixture.owners.map((owner: { path: string }) => resolve(repoRoot, owner.path)));
  const edges = new Map<string, string[]>([...owners].map((owner) => [owner, []]));
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
    if (visiting.has(owner)) throw new Error(`artifact-schema owner cycle at ${owner}`);
    if (visited.has(owner)) return;
    visiting.add(owner);
    for (const target of edges.get(owner)!) visit(target);
    visiting.delete(owner);
    visited.add(owner);
  };
  for (const owner of owners) visit(owner);
  expect(visited.size).toBe(owners.size);
});

test("distinguishes missing unreadable non-file and linked source without following directories", async () => {
  const { policySourceText } = await import("../../🔍️discovery/📖️source-access/🟦️.ts");
  for (const row of fixture.sourceStates) {
    const operations = {
      lstat: () => {
        if (row.stat === "missing") throw Object.assign(new Error(row.name), { code: "ENOENT" });
        return { isFile: row.stat === "file", isDirectory: row.stat === "directory", isSymbolicLink: row.stat === "symlink" };
      },
      readFile: () => {
        if (row.read === "unreadable") throw Object.assign(new Error(row.name), { code: "EACCES" });
        return row.name;
      },
      readdir: () => [],
    };
    expect(policySourceText("/repo", row.name, operations).state, row.name).toBe(row.expected);
  }

  const { policyDiscoverArtifactSchemaOwners } = await import("../../🧬️schema/🗿️artifact/🔍️owner-discovery/🟦️.ts");
  const directories = new Set<string>(["✏️s/🔌️plugins", "🧰️framework"]);
  for (const path of fixture.discovery.owner.split("/").map((_: string, index: number, parts: string[]) => parts.slice(0, index + 1).join("/"))) directories.add(path);
  const entries = (parent: string) =>
    [...directories]
      .filter((path) => dirname(path) === parent)
      .map((path) => ({ name: path.slice(parent.length + 1), isFile: false, isDirectory: true, isSymbolicLink: false }))
      .concat(parent === "✏️s/🔌️plugins" ? [{ name: fixture.discovery.symlink.split("/").at(-1)!, isFile: false, isDirectory: false, isSymbolicLink: true }] : []);
  const operations = {
    lstat: (path: string) => {
      const rel = relative("/repo", path).replaceAll("\\", "/");
      if (directories.has(rel)) return { isFile: false, isDirectory: true, isSymbolicLink: false };
      if (rel === fixture.discovery.symlink) return { isFile: false, isDirectory: false, isSymbolicLink: true };
      throw Object.assign(new Error(rel), { code: "ENOENT" });
    },
    readFile: () => "",
    readdir: (path: string) => entries(relative("/repo", path).replaceAll("\\", "/")),
  };
  expect(policyDiscoverArtifactSchemaOwners("/repo", operations)).toEqual([fixture.discovery.owner]);
});

test("reports unreadable schema evidence separately from absent leaves", async () => {
  const { policyArtifactSchemaFacetCompletenessBreaches } = await import("../../🧬️schema/🗿️artifact/⚖️laws/🧩️facet-completeness/🟦️.ts");
  const owner = fixture.discovery.owner,
    facets = new Set([`${owner}/🧬️schema`, `${owner}/🧬️schema/📸️snapshot`, `${owner}/🧬️schema/🔺️diff`]),
    unreadable = `${owner}/🧬️schema/🟦️.ts`;
  const operations = {
    lstat: (path: string) => {
      const rel = relative("/repo", path).replaceAll("\\", "/");
      if (facets.has(rel)) return { isFile: false, isDirectory: true, isSymbolicLink: false };
      if (rel === unreadable) return { isFile: true, isDirectory: false, isSymbolicLink: false };
      throw Object.assign(new Error(rel), { code: "ENOENT" });
    },
    readFile: () => {
      throw Object.assign(new Error(unreadable), { code: "EACCES" });
    },
    readdir: () => [],
  };
  const breaches = policyArtifactSchemaFacetCompletenessBreaches("/repo", [owner], operations);
  expect(breaches.some((breach) => breach.kind === "artifact-schema/source-unreadable" && breach.id.endsWith(unreadable))).toBe(true);
  expect(breaches.some((breach) => breach.kind === "artifact-schema/facet-completeness")).toBe(true);
});

test("retains real artifact law and field-oracle behavior without fixed diagnostic counts", { timeout: 30_000 }, async () => {
  const [{ policyDiscoverArtifactSchemaOwners }, { policyArtifactSchemaBreaches }, { policyArtifactOwnershipFieldParity }] = await Promise.all([
    import("../../🧬️schema/🗿️artifact/🔍️owner-discovery/🟦️.ts"),
    import("../../🧬️schema/🗿️artifact/⚖️laws/📋️aggregate/🟦️.ts"),
    import("../../🧬️schema/🗿️artifact/⚖️laws/🪪️ownership-field-parity/🟦️.ts"),
  ]);
  const taxonomyOwners = policyDiscoverArtifactSchemaOwners(repoRoot);
  const independentOwners = glob.sync("{✏️s/🔌️plugins,🧰️framework}/**/🗿️artifacts/*/🏅️standards/*/🪆️subsets/*", { cwd: repoRoot, onlyDirectories: true, ignore: ["**/node_modules/**", "**/target/**", "**/🗑️generated/**"] }).sort();
  expect(taxonomyOwners).toEqual(independentOwners);
  const breaches = policyArtifactSchemaBreaches(repoRoot);
  expect(breaches.length).toBeGreaterThan(0);
  expect([...new Set(breaches.map((breach) => breach.kind))].every((kind) => fixture.lawKinds.includes(kind))).toBe(true);
  for (const breach of breaches) expect(taxonomyOwners).toContain(breach.scope);
  expect(policyArtifactOwnershipFieldParity(repoRoot).filter((breach) => breach.path.endsWith("/🟦️.ts") && breach.missing.some((field) => field.startsWith("declaration:")))).toEqual([]);
});

test("registers one Bun Nx and seed-derived launch route", () => {
  const project = JSON.parse(readFileSync(resolve(libraryRoot, "📦️packages/🟦️typescript/📋️project.json"), "utf8"));
  const packageJson = JSON.parse(readFileSync(resolve(libraryRoot, "📦️packages/🟦️typescript/package.json"), "utf8"));
  expect(project.targets[fixture.route.target]?.options.command).toBe(fixture.route.command);
  expect(packageJson.scripts[fixture.route.target]).toBe(`nx run @semio-tech/repo-lib:${fixture.route.target}`);
  for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
    const source = readFileSync(resolve(repoRoot, path), "utf8");
    expect(source.split(fixture.route.launchName).length - 1).toBe(1);
    expect(source).toContain(fixture.route.launchCommand);
  }
});
