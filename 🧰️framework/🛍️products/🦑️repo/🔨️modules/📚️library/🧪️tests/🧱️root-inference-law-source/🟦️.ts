import { expect, test } from "bun:test";
import { chmodSync, existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
import Ajv from "ajv";
import ts from "typescript";
import { canonicalPrimaryFilenameForKind, loadTaxonomy, schemaFacetFormatEntries, semanticDirectoryKindId } from "../../📦️packages/🟦️typescript/🟦️.ts";
import type { PolicySourceOperations } from "../../🔍️discovery/📖️source-access/🟦️.ts";

const repoRoot = resolve(import.meta.dir, "../../../../../../../");
const libraryRoot = resolve(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const fixture = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🧫️fixtures/🧱️root-inference-law-source/🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🧬️schema/🧱️root-inference-law-source/🔣️.json"), "utf8"));
const familyRel = "✏️s/🔌️plugins/🔱️trinity/🗟️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences";

type VirtualNode = { kind: "directory" | "file" | "symlink"; text?: string; unreadable?: boolean };

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

function virtualOperations(input: Record<string, VirtualNode>): PolicySourceOperations {
  const nodes = new Map<string, VirtualNode>([["", { kind: "directory" }]]);
  for (const [path, node] of Object.entries(input)) {
    const parts = path.split("/").filter(Boolean);
    for (let index = 1; index < parts.length; index++) if (!nodes.has(parts.slice(0, index).join("/"))) nodes.set(parts.slice(0, index).join("/"), { kind: "directory" });
    nodes.set(path, node);
  }
  const rel = (path: string) => relative("/repo", path).replaceAll("\\", "/").replace(/^\.$/u, "");
  const unavailable = (path: string, code: string) => Object.assign(new Error(path), { code });
  return {
    lstat: (path) => {
      const node = nodes.get(rel(path));
      if (!node) throw unavailable(rel(path), "ENOENT");
      return { isFile: node.kind === "file", isDirectory: node.kind === "directory", isSymbolicLink: node.kind === "symlink" };
    },
    readFile: (path) => {
      const node = nodes.get(rel(path));
      if (!node) throw unavailable(rel(path), "ENOENT");
      if (node.unreadable) throw unavailable(rel(path), "EACCES");
      return node.text ?? "";
    },
    readdir: (path) => {
      const parent = rel(path),
        node = nodes.get(parent);
      if (!node) throw unavailable(parent, "ENOENT");
      if (node.unreadable) throw unavailable(parent, "EACCES");
      const prefix = parent ? `${parent}/` : "";
      return [...nodes]
        .filter(([candidate]) => candidate.startsWith(prefix) && candidate !== parent && !candidate.slice(prefix.length).includes("/"))
        .map(([candidate, child]) => ({ name: candidate.slice(prefix.length), isFile: child.kind === "file", isDirectory: child.kind === "directory", isSymbolicLink: child.kind === "symlink" }));
    },
  };
}

function validFamily(extra: Record<string, VirtualNode> = {}): Record<string, VirtualNode> {
  const taxonomy = loadTaxonomy(),
    nodes: Record<string, VirtualNode> = {};
  for (const [, format] of schemaFacetFormatEntries(familyRel, taxonomy)) nodes[`${familyRel}/${canonicalPrimaryFilenameForKind(format.fileKindId, taxonomy)}`] = { kind: "file", text: "schema" };
  Object.assign(nodes, {
    [`${familyRel}/🦀️.rs`]: { kind: "file", text: "pub struct JackInference { pub flat_position: JackFlatPosition, pub topology: Topology }" },
    [`${familyRel}/🎛flat-position/🦀️.rs`]: { kind: "file", text: "impl InferredField<JackSnapshot> for JackFlatPosition {}" },
    [`${familyRel}/🎛flat-position/🟦️.ts`]: { kind: "file", text: "export const flatPosition = true;" },
    [`${familyRel}/🧭topology/🦀️.rs`]: { kind: "file", text: "pub fn compute_topology() {}" },
    [`${familyRel}/🧭topology/🟦️.ts`]: { kind: "file", text: "export const topology = true;" },
  });
  return { ...nodes, ...extra };
}

test("validates the portable inference-law ownership contract", () => {
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  expect(fixture.owners).toHaveLength(12);
  expect(new Set(fixture.owners.map((owner: { path: string }) => owner.path)).size).toBe(12);
});

test("resolves and typechecks every anonymous owner", { timeout: 30_000 }, () => {
  const taxonomy = loadTaxonomy();
  for (const context of fixture.contexts) expect(semanticDirectoryKindId(context.directoryName, taxonomy, { parentKindId: context.parentKindId }), JSON.stringify(context)).toBe(context.kindId);
  const paths = fixture.owners.map((owner: { path: string }) => resolve(repoRoot, owner.path));
  for (const [index, owner] of fixture.owners.entries()) {
    expect(owner.path.split("/").at(-1)).toBe("🟦️.ts");
    expect(namedDeclarations(paths[index])).toEqual([...owner.declarations].sort());
  }
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
  expect(
    paths.flatMap((path: string) => [...program.getSyntacticDiagnostics(program.getSourceFile(path)), ...program.getSemanticDiagnostics(program.getSourceFile(path))]).map((diagnostic) => ts.flattenDiagnosticMessageText(diagnostic.messageText, "\n")),
  ).toEqual([]);
});

test("removes root bodies, binds consumers, and keeps an acyclic owner graph", () => {
  const moved = new Set(fixture.owners.flatMap((owner: { declarations: string[] }) => owner.declarations));
  expect(namedDeclarations(resolve(repoRoot, "📜️script.ts")).filter((name) => moved.has(name))).toEqual([]);
  for (const consumer of fixture.consumers) {
    const source = readFileSync(resolve(repoRoot, consumer.path), "utf8");
    for (const owner of consumer.owners) expect(source.includes(relativeSpecifier(consumer.path, owner)) || source.includes(owner), `${consumer.path} -> ${owner}`).toBe(true);
  }
  const owners = new Set(fixture.owners.map((owner: { path: string }) => resolve(repoRoot, owner.path))),
    edges = new Map<string, string[]>([...owners].map((owner) => [owner, []]));
  for (const owner of owners) {
    const syntax = ts.createSourceFile(owner, readFileSync(owner, "utf8"), ts.ScriptTarget.Latest, true);
    for (const statement of syntax.statements) {
      const specifier = (ts.isImportDeclaration(statement) || ts.isExportDeclaration(statement)) && statement.moduleSpecifier && ts.isStringLiteral(statement.moduleSpecifier) ? statement.moduleSpecifier.text : null;
      if (!specifier?.startsWith(".")) continue;
      const target = resolve(dirname(owner), specifier);
      expect(target).not.toBe(resolve(repoRoot, "📜️script.ts"));
      if (owners.has(target)) edges.get(owner)!.push(target);
    }
  }
  const seen = new Set<string>(),
    active = new Set<string>();
  const visit = (owner: string): void => {
    if (active.has(owner)) throw new Error(`inference owner cycle at ${owner}`);
    if (seen.has(owner)) return;
    active.add(owner);
    for (const target of edges.get(owner)!) visit(target);
    active.delete(owner);
    seen.add(owner);
  };
  for (const owner of owners) visit(owner);
  expect(seen.size).toBe(12);
});

test("walks source without following links and preserves unavailable evidence", async () => {
  const [{ policyWalkRelFileSources, policyWalkRelFiles }, { policyLineOfIndex }] = await Promise.all([import("../../🔍️discovery/🚶️file-walk/🟦️.ts"), import("../../🔍️discovery/📍️source-coordinate/🟦️.ts")]);
  const operations = virtualOperations({ "✏️s/a/🦀️.rs": { kind: "file" }, "✏️s/b": { kind: "symlink" } });
  expect(policyWalkRelFileSources("/repo", ["✏️s"], (_path, name) => name === "🦀️.rs", undefined, operations)).toEqual({ files: ["✏️s/a/🦀️.rs"], issues: [{ path: "✏️s/b", state: "symlink" }] });
  expect(() => policyWalkRelFiles("/repo", ["✏️s"], () => true, undefined, operations)).toThrow("symlink");
  expect(policyWalkRelFileSources("/repo", ["missing"], () => true, undefined, operations)).toEqual({ files: [], issues: [] });
  const unreadable = virtualOperations({ "✏️s/blocked": { kind: "directory", unreadable: true } });
  expect(policyWalkRelFileSources("/repo", ["✏️s"], () => true, undefined, unreadable).issues).toEqual([{ path: "✏️s/blocked", state: "unreadable" }]);
  const linked = virtualOperations({ linked: { kind: "symlink" }, "linked/nested/🦀️.rs": { kind: "file" } });
  expect(policyWalkRelFileSources("/repo", ["linked/nested"], () => true, undefined, linked)).toEqual({ files: [], issues: [{ path: "linked/nested", state: "symlink" }] });
  expect(policyLineOfIndex("first\nsecond", 6)).toBe(2);
});

test("rejects linked ancestors and native unreadable directories", async () => {
  const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
  expect(artifactRoot).toBeTruthy();
  const { policyWalkRelFileSources } = await import("../../🔍️discovery/🚶️file-walk/🟦️.ts"),
    root = mkdtempSync(resolve(artifactRoot!, "inference-source-")),
    target = resolve(root, "target"),
    blocked = resolve(root, "blocked");
  try {
    mkdirSync(resolve(target, "nested"), { recursive: true });
    writeFileSync(resolve(target, "nested", "🦀️.rs"), "pub fn hidden() {}\n");
    try {
      symlinkSync(target, resolve(root, "linked"), process.platform === "win32" ? "junction" : "dir");
    } catch (error) {
      if (process.platform === "win32" && (error as NodeJS.ErrnoException).code === "EPERM") return;
      throw error;
    }
    expect(policyWalkRelFileSources(root, ["linked/nested"], () => true)).toEqual({ files: [], issues: [{ path: "linked/nested", state: "symlink" }] });
    mkdirSync(blocked);
    if (process.platform !== "win32") {
      chmodSync(blocked, 0);
      expect(policyWalkRelFileSources(root, ["blocked"], () => true)).toEqual({ files: [], issues: [{ path: "blocked", state: "unreadable" }] });
      chmodSync(blocked, 0o700);
    }
  } finally {
    if (process.platform !== "win32" && existsSync(blocked)) chmodSync(blocked, 0o700);
    rmSync(root, { recursive: true, force: true });
  }
});

test("preserves inference family assembly derivation emoji and state laws", { timeout: 30_000 }, async () => {
  const [{ policyInferenceFamilyBreaches }, { policyDiscoverInferenceFamilies }, { policyInferenceEmojiUniquenessBreaches }, { policyInferenceAssemblyCoverageBreaches, policyInferenceNormalizeToken }, { policyDerivedMarkerLeakBreaches }] =
    await Promise.all([
      import("../../🧬️schema/💡️inference/⚖️laws/📋️aggregate/🟦️.ts"),
      import("../../🧬️schema/💡️inference/🔍️family-discovery/🟦️.ts"),
      import("../../🧬️schema/💡️inference/⚖️laws/😀️emoji-uniqueness/🟦️.ts"),
      import("../../🧬️schema/💡️inference/⚖️laws/🧶️assembly-coverage/🟦️.ts"),
      import("../../🧬️schema/💡️inference/⚖️laws/💾️state-separation/🟦️.ts"),
    ]);
  expect(policyInferenceFamilyBreaches("/repo", virtualOperations({ "✏️s": { kind: "directory" } }))).toEqual([]);
  const operations = virtualOperations(validFamily()),
    discovery = policyDiscoverInferenceFamilies("/repo", operations),
    family = discovery.families[0]!;
  expect(policyInferenceFamilyBreaches("/repo", operations)).toEqual([]);
  expect(policyInferenceEmojiUniquenessBreaches([family, { ...family, artifactRel: "other", inferencesRel: "other" }]).filter((breach: { id: string }) => breach.id.includes("-dup-"))).toEqual([]);
  expect(policyInferenceEmojiUniquenessBreaches([{ ...family, slugs: ["🎛first", "🎛second", "plain", "⚙️variation"] }])).toEqual(expect.arrayContaining([expect.objectContaining({ priority: "medium" }), expect.objectContaining({ priority: "low" })]));
  expect(["flat-position", "flat_position", "flatPosition", "FlatPosition"].map(policyInferenceNormalizeToken)).toEqual(["flatposition", "flatposition", "flatposition", "flatposition"]);
  expect(policyInferenceAssemblyCoverageBreaches("/repo", [family], operations)).toEqual([]);
  expect(policyInferenceAssemblyCoverageBreaches("/repo", [{ ...family, slugs: ["🎛flat-position", "🛰orphan"] }], operations).map((breach: { id: string }) => breach.id)).toEqual(
    expect.arrayContaining([expect.stringContaining("orphan-slug"), expect.stringContaining("uncovered-field")]),
  );
  const state = virtualOperations({
    "✏️s/plugin/🧬️schema/📸️snapshot/🦀️.rs": { kind: "file", text: "pub struct Snapshot { #[derived] value: u8 }" },
    "✏️s/plugin/🧬️schema/💡️inferences/value/🦀️.rs": { kind: "file", text: "#[derived] pub fn value() {}" },
  });
  expect(policyDerivedMarkerLeakBreaches("/repo", state)).toEqual([expect.objectContaining({ kind: "inference-migration/state-leak", line: 1 })]);
});

test("reports invalid and unreadable inference source instead of false clean", { timeout: 30_000 }, async () => {
  const { policyInferenceFamilyBreaches } = await import("../../🧬️schema/💡️inference/⚖️laws/📋️aggregate/🟦️.ts");
  const missingNodes = validFamily(),
    missingRootLeaf = Object.keys(missingNodes).find((path) => path.startsWith(`${familyRel}/`) && !path.slice(familyRel.length + 1).includes("/") && !path.endsWith("🦀️.rs"))!;
  delete missingNodes[missingRootLeaf];
  expect(policyInferenceFamilyBreaches("/repo", virtualOperations(missingNodes))).toEqual(expect.arrayContaining([expect.objectContaining({ kind: "inference-migration/family-root-completeness" })]));
  const linked = virtualOperations(validFamily({ [`${familyRel}/🟦️.ts`]: { kind: "symlink" } }));
  expect(policyInferenceFamilyBreaches("/repo", linked)).toEqual(expect.arrayContaining([expect.objectContaining({ kind: "inference-migration/source-unreadable", scope: `${familyRel}/🟦️.ts` })]));
  const wrongKind = virtualOperations(validFamily({ [`${familyRel}/🟦️.ts`]: { kind: "directory" } }));
  expect(policyInferenceFamilyBreaches("/repo", wrongKind)).toEqual(expect.arrayContaining([expect.objectContaining({ kind: "inference-migration/source-unreadable", scope: `${familyRel}/🟦️.ts` })]));
  const invalid = virtualOperations(
    validFamily({ [`${familyRel}/🎛flat-position/🟦️.ts`]: { kind: "file", text: "" }, [`${familyRel}/🧭topology/🟦️.ts`]: { kind: "file", text: "export {};" }, [`${familyRel}/🧭topology/🦀️.rs`]: { kind: "file", text: "struct Missing;" } }),
  );
  expect(policyInferenceFamilyBreaches("/repo", invalid)).toEqual(expect.arrayContaining([expect.objectContaining({ kind: "inference-migration/slug-leaf-presence" }), expect.objectContaining({ kind: "inference-migration/impl-presence" })]));
  expect(policyInferenceFamilyBreaches("/repo", virtualOperations({ "✏️s/🔌️plugins": { kind: "directory", unreadable: true } }))).toEqual([expect.objectContaining({ kind: "inference-migration/source-unreadable", scope: "✏️s/🔌️plugins" })]);
});

test("retains native source data and registers one Bun Nx launch route", () => {
  const sources = fixture.sourceData.map((path: string) => readFileSync(resolve(repoRoot, path), "utf8"));
  expect(sources[0]).toContain("pub fn compute_flat_position");
  expect(sources[1]).toContain("flat_position_bfs_walks_from_root");
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
