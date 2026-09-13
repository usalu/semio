import { expect, test } from "bun:test";
import Ajv from "ajv";
import glob from "fast-glob";
import { exports as resolveExports } from "resolve.exports";
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { tmpdir } from "node:os";
import ts from "typescript";
import { loadTaxonomy, semanticDirectoryKindId } from "../../🔍️discovery/🟦️.ts";
import { computeWorkspaces } from "../../🗂️workspaces/🟦️.ts";
import { parseWorkspaceRootDocument } from "../../🗂️workspaces/📄️manifest-projection/🟦️.ts";
import { publishWorkspaceMembership } from "../../🗂️workspaces/📣️publication/🟦️.ts";
import { parseWorkspacePublicationArguments } from "../../🗂️workspaces/🏃️execution/🟦️.ts";

const repoRoot = resolve(import.meta.dir, "../../../../../../..");
const libraryRoot = resolve(import.meta.dir, "../..");
const schemaPath = join(libraryRoot, "🧬️schema/🧱️workspace-publication-source/🔣️.json");
const fixturePath = join(libraryRoot, "🧫️fixtures/🧱️workspace-publication-source/🔣️.json");
const fixtureSource = readFileSync(fixturePath, "utf8");
const fixture = JSON.parse(fixtureSource) as {
  schemaVersion: number;
  publicationConflicts: readonly { name: string; change: "source" | "file-kind"; expectedError: string }[];
  payloadCases: readonly {
    name: string;
    manifests: readonly { directory: string; document: Record<string, unknown> }[];
    files: readonly string[];
    externalFiles: readonly string[];
    links: readonly { path: string; target: string; external: boolean; kind: "file" | "directory" }[];
    expected: readonly string[];
    duplicate: boolean;
    resolutions: readonly { owner: string; entry: string; conditions: string[]; targets: string[]; payload: string }[];
  }[];
  owners: readonly { path: string; declarations: readonly string[]; imports: readonly string[]; contextChain: readonly string[] }[];
  contexts: readonly { directoryName: string; parentKindId: string; kindId: string }[];
  routes: Record<string, { target: string; command: string; launchName: string; launchCommand: string }>;
  inputs: { membership: readonly string[]; source: readonly string[] };
  arguments: readonly { segments: readonly string[]; accepted: boolean; mode?: "check" | "write" }[];
  workspaceCase: {
    rootDocument: Record<string, unknown>;
    packages: readonly { directory: string; name: string }[];
    ignoredPackages: readonly { directory: string; name: string }[];
    expectedWorkspaces: readonly string[];
  };
  retirement: {
    generatorId: string;
    commands: readonly string[];
    targets: readonly string[];
    launch: { name: string; command: string; order: number };
    historicalEvidence: readonly { path: string; bytes: number; sha256: string }[];
    absentPaths: readonly string[];
  };
};

function put(root: string, path: string, value: unknown): void {
  const absolute = join(root, path);
  mkdirSync(dirname(absolute), { recursive: true });
  writeFileSync(absolute, typeof value === "string" ? value : `${JSON.stringify(value, null, 2)}\n`);
}

function fixtureTree(): { root: string; outside: string } {
  const root = mkdtempSync(join(tmpdir(), "semio-workspace-publication-"));
  const outside = mkdtempSync(join(tmpdir(), "semio-workspace-outside-"));
  put(root, "package.json", fixture.workspaceCase.rootDocument);
  for (const row of [...fixture.workspaceCase.packages, ...fixture.workspaceCase.ignoredPackages]) put(root, `${row.directory}/package.json`, { name: row.name });
  put(outside, "foreign/package.json", { name: "@fixture/foreign" });
  symlinkSync(join(outside, "foreign"), join(root, "linked"), process.platform === "win32" ? "junction" : "dir");
  return { root, outside };
}

function exportedDeclarations(path: string): Set<string> {
  const source = ts.createSourceFile(path, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const names = new Set<string>();
  source.forEachChild((node) => {
    const declaration = ts.isFunctionDeclaration(node) || ts.isClassDeclaration(node) || ts.isInterfaceDeclaration(node) || ts.isTypeAliasDeclaration(node);
    if (declaration && node.name && node.modifiers?.some((modifier) => modifier.kind === ts.SyntaxKind.ExportKeyword)) names.add(node.name.text);
  });
  return names;
}

function internalOwnerImports(path: string, owners: ReadonlySet<string>): string[] {
  const absolute = join(repoRoot, path);
  const source = ts.createSourceFile(path, readFileSync(absolute, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  return source.statements
    .flatMap((node) => {
      if (!ts.isImportDeclaration(node) || !ts.isStringLiteral(node.moduleSpecifier) || !node.moduleSpecifier.text.startsWith(".")) return [];
      const imported = relative(repoRoot, resolve(dirname(absolute), node.moduleSpecifier.text)).replaceAll("\\", "/");
      return owners.has(imported) ? [imported] : [];
    })
    .sort();
}

test("workspace publication fixture is schema-first and independently parsed", async () => {
  const validate = new Ajv({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(schemaPath, "utf8")));
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  for (const invalid of [
    { ...fixture, schemaVersion: 2 },
    { ...fixture, extra: true },
    { ...fixture, owners: fixture.owners.slice(1) },
  ])
    expect(validate(invalid)).toBe(false);
  const jsonc = await import("jsonc-parser");
  const errors: import("jsonc-parser").ParseError[] = [];
  expect(jsonc.parse(fixtureSource, errors, { allowTrailingComma: false, disallowComments: true })).toEqual(fixture);
  expect(errors).toEqual([]);
});

test("workspace owners are anonymous, declared, context-bound, and avoid the package barrel", () => {
  const taxonomy = loadTaxonomy();
  const ownerPaths = new Set(fixture.owners.map((owner) => owner.path));
  expect(fixture.owners).toHaveLength(5);
  for (const owner of fixture.owners) {
    const absolute = join(repoRoot, owner.path);
    expect(absolute.endsWith("/🟦️.ts")).toBe(true);
    expect(existsSync(absolute)).toBe(true);
    const declarations = exportedDeclarations(absolute);
    for (const declaration of owner.declarations) expect(declarations.has(declaration), `${owner.path}:${declaration}`).toBe(true);
    const source = readFileSync(absolute, "utf8");
    expect(source).not.toMatch(/📦️packages\/🟦️typescript\/(?:🟦️|📜️script)\.ts/u);
    const segments = relative(libraryRoot, absolute).replaceAll("\\", "/").split("/").slice(0, -1);
    expect(segments.slice(-owner.contextChain.length)).toEqual(owner.contextChain.map((kindId) => fixture.contexts.find((row) => row.kindId === kindId)!.directoryName));
    expect(internalOwnerImports(owner.path, ownerPaths)).toEqual([...owner.imports].sort());
  }
  for (const context of fixture.contexts) expect(semanticDirectoryKindId(context.directoryName, taxonomy, { parentKindId: context.parentKindId })).toBe(context.kindId);
  const router = readFileSync(join(libraryRoot, "📦️packages/🟦️typescript/📜️script.ts"), "utf8");
  expect(router).toContain('import { WorkspacePublicationScript } from "../../🗂️workspaces/🏃️execution/🟦️.ts";');
  expect(router).toContain('.register("workspaces", WorkspacePublicationScript)');
  expect(router).not.toMatch(/class WorkspacesScript|function ticketImportantFemHandoffBytes|TicketImportantFemHandoff/u);
});

test("private publication preserves unrelated fields and matches fast-glob membership", async () => {
  const { root, outside } = fixtureTree();
  try {
    const expected = [...fixture.workspaceCase.expectedWorkspaces];
    const discovered = computeWorkspaces(root);
    expect(discovered).toEqual(expected);
    const oracle = (await glob("**/package.json", { cwd: root, onlyFiles: true, followSymbolicLinks: false, ignore: ["node_modules/**", ".*/**"] }))
      .map((path) => dirname(path).replaceAll("\\", "/"))
      .filter((path) => path !== ".")
      .sort((left, right) => left.localeCompare(right));
    expect(oracle).toEqual(expected);
    const before = parseWorkspaceRootDocument(readFileSync(join(root, "package.json"), "utf8"));
    const beforeSource = readFileSync(join(root, "package.json"), "utf8");
    let checkWrites = 0;
    expect(() => publishWorkspaceMembership(root, "check", { operations: { writeText: () => (checkWrites += 1) }, report: () => {} })).toThrow("Root workspace membership is stale");
    expect(checkWrites).toBe(0);
    expect(readFileSync(join(root, "package.json"), "utf8")).toBe(beforeSource);
    const progress: string[] = [];
    const result = publishWorkspaceMembership(root, "write", { discovery: { onProgress: (event) => progress.push(event.relativeDirectory) }, report: () => {} });
    const after = parseWorkspaceRootDocument(readFileSync(join(root, "package.json"), "utf8"));
    expect(result).toMatchObject({ fresh: false, written: true, expectedCount: expected.length, missing: expected, stale: ["obsolete"] });
    expect(after.workspaces).toEqual(expected);
    expect({ ...after, workspaces: before.workspaces }).toEqual(before);
    expect(progress.length).toBeGreaterThan(0);
    expect(publishWorkspaceMembership(root, "check", { report: () => {} })).toMatchObject({ fresh: true, written: false });
  } finally {
    rmSync(root, { recursive: true, force: true });
    rmSync(outside, { recursive: true, force: true });
  }
});

test("workspace discovery rejects duplicates, supports cancellation, and never follows directory links", () => {
  const { root, outside } = fixtureTree();
  try {
    put(root, "duplicate/package.json", { name: fixture.workspaceCase.packages[0]!.name });
    expect(() => computeWorkspaces(root)).toThrow(/duplicate package name/u);
    rmSync(join(root, "duplicate"), { recursive: true, force: true });
    const controller = new AbortController();
    expect(() => computeWorkspaces(root, { onProgress: () => controller.abort(), signal: controller.signal })).toThrow(/cancelled/u);
    expect(computeWorkspaces(root)).not.toContain("linked");
  } finally {
    rmSync(root, { recursive: true, force: true });
    rmSync(outside, { recursive: true, force: true });
  }
});

test("workspace payload ownership follows explicit physical exports independently of directory or compiler names", async () => {
  for (const row of fixture.payloadCases) {
    const root = mkdtempSync(join(tmpdir(), "semio-workspace-payload-"));
    const outside = mkdtempSync(join(tmpdir(), "semio-workspace-payload-outside-"));
    try {
      for (const manifest of row.manifests) put(root, `${manifest.directory}/package.json`, manifest.document);
      for (const path of row.files) put(root, path, "export const value = 1;\n");
      for (const path of row.externalFiles) put(outside, path, "export const value = 2;\n");
      for (const link of row.links) symlinkSync(join(link.external ? outside : root, link.target), join(root, link.path), link.kind === "file" ? "file" : process.platform === "win32" ? "junction" : "dir");
      const paths = await glob("**/package.json", { cwd: root, onlyFiles: true, followSymbolicLinks: false });
      const owned = new Set<string>();
      for (const resolution of row.resolutions) {
        const manifest = JSON.parse(readFileSync(join(root, resolution.owner, "package.json"), "utf8"));
        const targets = resolveExports(manifest, resolution.entry, { conditions: resolution.conditions });
        expect(targets, row.name).toEqual(resolution.targets);
        for (const target of targets!) {
          const absolute = resolve(root, resolution.owner, target);
          const files = await glob("**/*", { cwd: join(root, resolution.payload), onlyFiles: true, followSymbolicLinks: false });
          expect(files.map((path) => resolve(root, resolution.payload, path)), row.name).toContain(absolute);
        }
        if (resolution.payload) owned.add(resolution.payload);
      }
      if (row.duplicate) expect(() => computeWorkspaces(root), row.name).toThrow(/duplicate package name/u);
      else {
        const oracle = paths.map((path) => dirname(path)).filter((path) => !owned.has(path)).sort((a, b) => a.localeCompare(b));
        expect(oracle, row.name).toEqual(row.expected);
        expect(computeWorkspaces(root), row.name).toEqual(oracle);
      }
    } finally {
      rmSync(root, { recursive: true, force: true });
      rmSync(outside, { recursive: true, force: true });
    }
  }
});

test("workspace source admission surfaces unreadable and nonregular manifests", () => {
  const entries = new Map<string, readonly { kind: "directory" | "file" | "symlink" | "other"; name: string }[]>([
    ["/repo", [{ kind: "directory", name: "source" }]],
    ["/repo/source", []],
  ]);
  const state = (path: string): "directory" | "file" | "missing" | "symlink" | "other" => {
    if (path === "/repo" || path === "/repo/source") return "directory";
    if (path === "/repo/source/package.json") return "file";
    return "missing";
  };
  expect(() =>
    computeWorkspaces("/repo", {
      operations: {
        list: (path) => entries.get(path) ?? [],
        readText: () => {
          throw new Error("denied manifest");
        },
        state,
      },
    }),
  ).toThrow("denied manifest");
  expect(() =>
    computeWorkspaces("/repo", {
      operations: {
        list: (path) => {
          if (path === "/repo/source") throw new Error("denied directory");
          return entries.get(path) ?? [];
        },
        readText: () => "{}",
        state,
      },
    }),
  ).toThrow("denied directory");
  expect(() => computeWorkspaces("/repo", { operations: { list: (path) => entries.get(path) ?? [], readText: () => "{}", state: (path) => (path === "/repo/source/package.json" ? "symlink" : state(path)) } })).toThrow(/regular file/u);
  expect(() => publishWorkspaceMembership("/repo", "check", { operations: { state: () => "symlink" } })).toThrow(/physical regular file/u);
});

test("workspace publication preserves manifest changes observed during discovery", () => {
  for (const row of fixture.publicationConflicts) {
    const { root, outside } = fixtureTree();
    try {
      const path = join(root, "package.json");
      let changed = false;
      let writes = 0;
      const updated = { ...fixture.workspaceCase.rootDocument, concurrent: row.name };
      expect(() => publishWorkspaceMembership(root, "write", {
        discovery: { onProgress: () => {
          if (changed) return;
          changed = true;
          if (row.change === "source") put(root, "package.json", updated);
        } },
        operations: {
          state: () => changed && row.change === "file-kind" ? "symlink" : "file",
          writeText: () => { writes += 1; },
        },
        report: () => {},
      }), row.name).toThrow(row.expectedError);
      expect(writes, row.name).toBe(0);
      expect(JSON.parse(readFileSync(path, "utf8")), row.name).toEqual(row.change === "source" ? updated : fixture.workspaceCase.rootDocument);
    } finally {
      rmSync(root, { recursive: true, force: true });
      rmSync(outside, { recursive: true, force: true });
    }
  }
});

test("workspace command admits only one exact check or write argument", () => {
  for (const row of fixture.arguments) {
    if (row.accepted) expect(parseWorkspacePublicationArguments([...row.segments])).toBe(row.mode!);
    else expect(() => parseWorkspacePublicationArguments([...row.segments])).toThrow(/workspaces <--write\|--check>/u);
  }
});

test("historical FEM regeneration authority is retired while evidence bytes remain exact", () => {
  for (const row of fixture.retirement.historicalEvidence) {
    const bytes = readFileSync(join(repoRoot, row.path));
    expect(bytes.byteLength, row.path).toBe(row.bytes);
    expect(createHash("sha256").update(bytes).digest("hex"), row.path).toBe(row.sha256);
  }
  for (const path of fixture.retirement.absentPaths) expect(existsSync(join(repoRoot, path)), path).toBe(false);
  const authorities = [
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🏭️owned-generator-preview-inventory/🔣️.json",
    ".vscode/🧩️launch.seed.jsonc",
    ".vscode/launch.json",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🚀️launch/🟦️.ts",
  ];
  for (const path of authorities) expect(readFileSync(join(repoRoot, path), "utf8").includes(fixture.retirement.generatorId), path).toBe(false);
  const router = readFileSync(join(repoRoot, authorities[0]!), "utf8");
  for (const command of fixture.retirement.commands) expect(router, command).not.toContain(`.register("${command}"`);
  const project = JSON.parse(readFileSync(join(repoRoot, authorities[1]!), "utf8"));
  for (const target of fixture.retirement.targets) expect(project.targets[target]).toBeUndefined();
  const taxonomy = JSON.parse(readFileSync(join(repoRoot, authorities[2]!), "utf8"));
  expect(taxonomy.generatorContracts[fixture.retirement.generatorId]).toBeUndefined();
  const inventory = JSON.parse(readFileSync(join(repoRoot, authorities[3]!), "utf8")) as { ownedGeneratorIds: readonly string[] };
  const ownedGeneratorIds = Object.entries(taxonomy.generatorContracts)
    .filter(([, contract]) => (contract as { ownership?: unknown }).ownership === "owned")
    .map(([id]) => id)
    .sort();
  expect(inventory.ownedGeneratorIds).toEqual(ownedGeneratorIds);
  expect(ownedGeneratorIds).not.toContain(fixture.retirement.generatorId);
});

test("workspace source, check, and publication routes are exact across package, project, and launch catalogs", async () => {
  const packageRoot = join(libraryRoot, "📦️packages/🟦️typescript");
  const project = JSON.parse(readFileSync(join(packageRoot, "📋️project.json"), "utf8"));
  const manifest = JSON.parse(readFileSync(join(packageRoot, "package.json"), "utf8"));
  expect(project.namedInputs.workspaceMembershipSources).toEqual(fixture.inputs.membership);
  expect(project.namedInputs.workspacePublicationSources).toEqual(fixture.inputs.source);
  expect(project.targets[fixture.routes.source.target].inputs).toEqual(["workspacePublicationSources"]);
  expect(project.targets[fixture.routes.check.target].inputs).toEqual(["workspaceMembershipSources"]);
  expect(project.targets[fixture.routes.write.target].inputs).toEqual(["workspaceMembershipSources"]);
  expect(project.targets[fixture.routes.check.target].cache).toBe(false);
  expect(project.targets[fixture.routes.write.target].cache).toBe(false);
  const jsonc = await import("jsonc-parser");
  for (const route of Object.values(fixture.routes)) {
    expect(project.targets[route.target]?.options?.command).toBe(route.command);
    expect(
      Object.values(manifest.scripts).filter((command) => command === route.launchCommand.replace(/^bun /u, "")),
      route.target,
    ).toHaveLength(1);
    for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
      const launch = jsonc.parse(readFileSync(join(repoRoot, path), "utf8"));
      expect(launch.configurations.filter((row: { name?: string; command?: string }) => row.name === route.launchName && row.command === route.launchCommand)).toHaveLength(1);
    }
  }
});
