import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { chmodSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, readlinkSync, renameSync, symlinkSync, writeFileSync } from "node:fs";
import { basename, dirname, join, parse as parsePath, relative, resolve, sep } from "node:path";
import fastGlob from "fast-glob";
import { parse, type ParseError } from "jsonc-parser";
import ts from "typescript";
import { authorArtifactScaffold, ArtifactScaffoldError, type ArtifactScaffoldProgress } from "../../🏗️builder/🟦️.ts";
import { canonicalFilenameForKind, loadCatalogTaxonomy, semanticArtifactEmptyFacetProjectionAuthority } from "../../🔍️discovery/🟦️.ts";

type Case = Readonly<{ id: string; producer: "surface" | "subset"; setup: string; role: string; expected: "created" | "preserved" | "preview" | "rejected" }>;
type Result = { created: string[]; skipped: string[] };
type Options = { progress?: (event: { phase: string; path?: string }) => void };
type Node = { kind: string; mode: number; device: number; inode: number; bytes?: number; sha256?: string; target?: string };
const library = resolve(import.meta.dir, "../.."),
  repoRoot = resolve(library, "../../../../..");
const inputBytes = readFileSync(join(import.meta.dir, "../../🧫️fixtures/🪶️artifact-empty-facet-authoring/🔣️.json"), "utf8"),
  vector = JSON.parse(inputBytes) as Readonly<{
    schemaVersion: number;
    contractId: string;
    sourceContractId: string;
    subsetSegments: string[];
    leaf: string;
    customContent: string;
    customMode: number;
    directoryMode: number;
    surfaceLayout: string[];
    subsetLayout: string[];
    cases: Case[];
  }>;
const taxonomy = loadCatalogTaxonomy();
const contract = taxonomy.semanticOwnedFileProjectionContracts[vector.sourceContractId];
if (contract?.contractKind !== "semantic-facet-primary-file") throw new Error("The exact authored empty-facet contract is required");
const registryPath = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🌳️surface-scaffold/🟦️.ts"),
  artifactOwnerPath = join(library, "🏗️authoring/🗿️artifact-tree/🟦️.ts");
const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
if (!artifactRoot) throw new Error("SEMIO_TEST_ARTIFACT_DIR is required for artifact authoring output.");
const report = join(artifactRoot, "artifact-empty-facet-authoring");
mkdirSync(report, { recursive: true });
const hash = (bytes: string | Uint8Array): string => createHash("sha256").update(bytes).digest("hex");
const sort = (left: string, right: string): number => Buffer.compare(Buffer.from(left), Buffer.from(right));

const identityPaths = [
  artifactOwnerPath,
  registryPath,
  ...["🏗️builder/🟦️.ts", "📦️packages/🟦️typescript/🟦️.ts", "🔍️discovery/🟦️.ts", "🧹️normalization/🟦️.ts", "🔣️taxonomy.json"].map((path) => join(library, path)),
  import.meta.filename,
  ...[
    "../../🧫️fixtures/🪶️artifact-empty-facet-authoring/🔣️.json",
    "../../🧫️fixtures/🪶️artifact-empty-facet-authoring/📨️request/🔣️.json",
    "../../🧫️fixtures/🪶️artifact-empty-facet-authoring/📋️registration/🔣️.json",
  ].map((path) => join(import.meta.dir, path)),
];
const identities = (): Record<string, string> => Object.fromEntries(identityPaths.map((path) => [relative(repoRoot, path).replaceAll("\\", "/"), hash(readFileSync(path))]));

test("shared artifact authoring passes the independent strict TypeScript compiler", () => {
  const path = artifactOwnerPath;
  const program = ts.createProgram([path], {
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
  const source = program.getSourceFile(path);
  expect(source).toBeDefined();
  expect([...program.getSyntacticDiagnostics(source), ...program.getSemanticDiagnostics(source)].map((diagnostic) => ts.flattenDiagnosticMessageText(diagnostic.messageText, "\n"))).toEqual([]);
}, { timeout: 30_000 });

/** 🧭️ Allocates one exclusive semantic run after checking its complete no-follow ancestry. */
function allocate(): string {
  const parent = join(report, "🧪️runs");
  let current = parsePath(parent).root;
  for (const part of relative(current, parent).split(sep)) {
    current = join(current, part);
    let stat;
    try {
      stat = lstatSync(current);
    } catch (error) {
      if ((error as NodeJS.ErrnoException).code !== "ENOENT" || current !== parent) throw error;
      mkdirSync(current);
      stat = lstatSync(current);
    }
    if (!stat.isDirectory() || stat.isSymbolicLink()) throw new Error(`Authoring run ancestor is not a no-follow directory: ${current}`);
  }
  return mkdtempSync(join(parent, "🧪️s-test-empty-facet-authoring-"));
}

/** 🔬️ Captures native node kinds and bytes without following any fixture link. */
function snapshot(root: string): Record<string, Node> {
  const nodes: Record<string, Node> = {};
  const visit = (path: string): void => {
    const stat = lstatSync(path),
      key = relative(root, path).replaceAll("\\", "/");
    const identity = { mode: stat.mode & 0o7777, device: stat.dev, inode: stat.ino };
    if (stat.isSymbolicLink()) nodes[key] = { ...identity, kind: "symlink", target: readlinkSync(path) };
    else if (stat.isDirectory()) {
      if (key) nodes[key] = { ...identity, kind: "directory" };
      for (const name of readdirSync(path).sort(sort)) visit(join(path, name));
    } else if (stat.isFile()) {
      const bytes = readFileSync(path);
      nodes[key] = { ...identity, kind: "file", bytes: bytes.length, sha256: hash(bytes) };
    } else nodes[key] = { ...identity, kind: "nonregular" };
  };
  visit(root);
  expect(fastGlob.sync("**/*", { cwd: root, dot: true, onlyFiles: false, followSymbolicLinks: false }).sort(sort)).toEqual(Object.keys(nodes).sort(sort));
  return nodes;
}

/** 🧪️ Seeds only independently declared inputs and bounded hostile nodes inside this run. */
function fixture(row: Case): { root: string; workspace: string; subset: string; marker: string; ordered: string[]; expected: string[] } {
  const root = allocate(),
    workspace = join(root, "🧪️workspace"),
    external = join(root, "🧪️external");
  mkdirSync(workspace);
  mkdirSync(external);
  writeFileSync(join(external, vector.leaf), vector.customContent, { flag: "wx" });
  const parts = [...vector.subsetSegments];
  if (row.setup === "wrong-owner") parts[3] = "📚️examples";
  const subset = parts.join("/"),
    subsetAbs = join(workspace, subset);
  if (row.setup !== "missing-standard") mkdirSync(dirname(subsetAbs), { recursive: true });
  if (row.producer === "surface" && row.setup !== "missing-subset") mkdirSync(row.setup === "missing-schema" ? subsetAbs : join(subsetAbs, "🧬️schema"), { recursive: true });
  const surface = row.role === "editor" ? "✏️editor" : "👁️viewer",
    mode = row.role === "editor" ? "✏️edit" : "👁️view";
  const prefix = row.producer === "surface" ? `${subset}/${surface}` : subset;
  const marker = `${prefix}/${row.producer === "surface" ? "🎮️commands" : "📚️examples"}/${vector.leaf}`;
  const absolute = join(workspace, marker);
  if (["custom", "leaf-link", "dangling-leaf-link", "leaf-directory"].includes(row.setup)) mkdirSync(dirname(absolute), { recursive: true });
  if (row.setup === "custom") {
    writeFileSync(absolute, vector.customContent, { flag: "wx", mode: vector.customMode });
    if (process.platform !== "win32") {
      chmodSync(absolute, vector.customMode);
      chmodSync(dirname(absolute), vector.directoryMode);
    }
  } else if (row.setup === "leaf-link" || row.setup === "dangling-leaf-link") {
    const target = join(external, row.setup === "leaf-link" ? vector.leaf : "🧪️absent.md");
    symlinkSync(process.platform === "win32" ? target : relative(dirname(absolute), target), absolute, "file");
  } else if (row.setup === "leaf-directory") mkdirSync(absolute);
  else if (row.setup === "facet-link" || row.setup === "empty-facet-link") {
    mkdirSync(dirname(dirname(absolute)), { recursive: true });
    const destination = row.setup === "empty-facet-link" ? join(external, "🧪️absent-facet") : external;
    if (row.setup === "empty-facet-link") mkdirSync(destination);
    symlinkSync(process.platform === "win32" ? destination : relative(dirname(dirname(absolute)), destination), dirname(absolute), process.platform === "win32" ? "junction" : "dir");
  }
  const ordered = (row.producer === "surface" ? vector.surfaceLayout.map((path) => path.replaceAll("{mode}", mode)) : vector.subsetLayout).map((path) => `${prefix}/${path}`);
  return { root, workspace, subset, marker, ordered, expected: [...ordered].sort(sort) };
}

let surfaceModule: Promise<{ scaffoldSurfaceTree: (root: string, subset: string, role: string, dryRun: boolean, options?: Options) => Result }> | undefined;
let rootModule: Promise<{ newScaffoldSubsetTree: (root: string, subset: string, schema: typeof taxonomy, dryRun: boolean, options?: Options) => Result }> | undefined;

/** 🏗️ Invokes the actual authored entrypoint, never registry generation or batch CLI routing. */
async function author(row: Case, target: ReturnType<typeof fixture>, options?: Options): Promise<Result> {
  if (row.producer === "surface") return (await (surfaceModule ??= import(registryPath))).scaffoldSurfaceTree(target.workspace, target.subset, row.role, row.setup === "dry-run", options);
  return (await (rootModule ??= import(artifactOwnerPath))).newScaffoldSubsetTree(target.workspace, target.subset, taxonomy, row.setup === "dry-run", options);
}

test("empty-facet authoring has a closed independent input and existing authored disposition", () => {
  
  expect(vector["schemaVersion"]).toEqual(1);expect(vector["contractId"]).toEqual("artifact-empty-facet-authoring-v1");expect(vector["sourceContractId"]).toEqual("artifact-empty-facet-primary-markdown-v1");expect(vector["subsetSegments"]).toEqual(["✏️s","🔌️plugins","🔋️energy","🗿️artifacts","🔋️model","🏅️standards","🔖️1","🪆️subsets","✳️any"]);expect(vector["leaf"]).toEqual("📝️.md");expect(vector["customMode"]).toEqual(416);expect(vector["directoryMode"]).toEqual(488);
  
  const errors: ParseError[] = [];
  expect(parse(inputBytes, errors, { disallowComments: true, allowTrailingComma: false })).toEqual(vector);
  expect(errors).toEqual([]);
  expect(new Set(vector.cases.map((row) => row.id)).size).toBe(30);
  expect(canonicalFilenameForKind(taxonomy.windowEmptyFacetFileKindId, taxonomy)).toBe(vector.leaf);
  expect(contract.sourceDisposition).toBe("authored");
  expect(contract.authoringCommand.command).toEqual(["new", "surface"]);
  expect(contract.authoringCommand.writeDisposition).toBe("create-if-absent");
  const marker = `${vector.subsetSegments.join("/")}/👁️viewer/🎮️commands/${contract.sourceFilename}`;
  expect(semanticArtifactEmptyFacetProjectionAuthority({ sourcePath: marker, sourceFileKindId: "markdown" }, taxonomy).ownerForm).toBe("artifact-surface");
});

for (const row of vector.cases)
  test(`actual empty-facet authoring ${row.id}`, async () => {
    const target = fixture(row),
      started = new Date().toISOString(),
      sourcesBefore = identities(),
      before = snapshot(target.root);
    const late = row.setup.startsWith("late-") || row.setup === "changed-parent";
    let result: Result | null = null,
      second: Result | null = null,
      operationError: unknown = null,
      testError: unknown = null,
      passed = false,
      after = before,
      injected = false,
      injectedSnapshot = before;
    try {
      const progress = (event: { phase: string; path?: string }): void => {
        if (!late || injected || event.phase !== "before-create" || event.path !== target.marker) return;
        injected = true;
        const absolute = join(target.workspace, target.marker),
          external = join(target.root, "🧪️external");
        if (row.setup === "late-regular") {
          writeFileSync(absolute, vector.customContent, { flag: "wx", mode: vector.customMode });
          if (process.platform !== "win32") chmodSync(absolute, vector.customMode);
        } else if (row.setup === "late-directory") mkdirSync(absolute);
        else if (row.setup === "late-link") symlinkSync(process.platform === "win32" ? join(external, vector.leaf) : relative(dirname(absolute), join(external, vector.leaf)), absolute, "file");
        else {
          renameSync(dirname(absolute), join(external, "🧪️moved-parent"));
          const replacement = join(external, "🧪️replacement");
          mkdirSync(replacement);
          symlinkSync(process.platform === "win32" ? replacement : relative(dirname(dirname(absolute)), replacement), dirname(absolute), process.platform === "win32" ? "junction" : "dir");
        }
        injectedSnapshot = snapshot(target.root);
      };
      try {
        result = await author(row, target, { progress });
      } catch (error) {
        operationError = error;
      }
      after = snapshot(target.root);
      if (late) expect(injected).toBe(true);
      if (row.expected === "rejected") {
        expect(operationError).toBeInstanceOf(Error);
        expect(String(operationError)).toMatch(
          row.setup === "invalid-role"
            ? /Unknown artifact surface role/u
            : row.setup === "wrong-owner"
              ? /Wrong structural authoring owner|Unregistered authoring directory/u
              : row.setup.startsWith("missing-")
                ? /Missing governing authoring directory/u
                : row.setup.includes("facet-link") || row.setup === "changed-parent"
                  ? /Authoring ancestor/u
                  : /Authoring target is not a regular file/u,
        );
        expect(after).toEqual(late ? injectedSnapshot : before);
        if (late) {
          const partial = (operationError as { partial: { created: { path: string; device: number; inode: number; mode: number; bytes: number; sha256: string }[]; failedPath: string } }).partial;
          expect(partial.failedPath).toBe(target.marker);
          expect(partial.created.length).toBeGreaterThan(0);
          for (const file of partial.created) {
            const { path, ...identity } = file;
            expect(after[`🧪️workspace/${path}`]).toEqual<{ readonly kind: string } & typeof identity>({ kind: "file", ...identity });
          }
        }
      } else {
        expect(operationError).toBeNull();
        expect(result).not.toBeNull();
        expect([...result!.created, ...result!.skipped].sort(sort)).toEqual(target.expected);
        if (row.expected === "preview") expect(after).toEqual(before);
        else {
          const workspaceNodes = snapshot(target.workspace),
            paths = Object.entries(workspaceNodes)
              .filter(([, node]) => node.kind === "file")
              .map(([path]) => path)
              .sort(sort);
          expect(paths).toEqual(target.expected);
          second = await author(row, target);
          expect(second).toEqual({ created: [], skipped: target.ordered });
          const repeated = snapshot(target.root);
          expect(repeated).toEqual(after);
          if (row.expected === "preserved") {
            const key = `workspace${target.marker}`;
            expect(after[key]).toEqual((late ? injectedSnapshot : before)[key]);
            expect(readFileSync(join(target.workspace, target.marker), "utf8")).toBe(vector.customContent);
            expect(after[`workspace${dirname(target.marker)}`]).toEqual((late ? injectedSnapshot : before)[`workspace${dirname(target.marker)}`]);
          }
          for (const path of result!.created.filter((path) => basename(path) === vector.leaf)) {
            const text = readFileSync(join(target.workspace, path), "utf8");
            expect(text).toContain("Authored by");
            expect(text).not.toContain("Generated by");
            expect(
              Object.values(taxonomy.generatorContracts)
                .flatMap((entry) => entry.outputRoots)
                .some((entry) => path === entry.path || path.startsWith(`${entry.path}/`)),
            ).toBe(false);
          }
        }
      }
      expect(identities()).toEqual(sourcesBefore);
      passed = true;
    } catch (error) {
      testError = error;
      throw error;
    } finally {
      const record = {
        id: row.id,
        expected: row.expected,
        passed,
        started,
        finished: new Date().toISOString(),
        producer: row.producer,
        result,
        second,
        injected,
        partial: (operationError as { partial?: unknown } | null)?.partial ?? null,
        operationError: operationError ? String(operationError) : null,
        testError: testError ? String(testError) : null,
        beforeDigest: hash(JSON.stringify(before)),
        afterDigest: hash(JSON.stringify(after)),
        addedPaths: Object.keys(after).filter((path) => !before[path]),
        sourcesBefore,
        sourcesAfter: identities(),
      };
      writeFileSync(
        join(target.root, "📝️.md"),
        `# Retained Empty-Facet Authoring Run\n\n\`\`\`json\n${JSON.stringify(record, null, 2)}\n\`\`\`\n\nAll authored inputs and physical failure evidence remain retained for exact review. No prior owner was modified.\n`,
        { flag: "wx" },
      );
    }
  });

const requestInput = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🪶️artifact-empty-facet-authoring/📨️request/🔣️.json"), "utf8")) as {
  schemaVersion: number;
  contractId: string;
  readBytes: number;
  readChunkBytes: number;
  cases: { id: string; leaves: string[]; action: string; error: string; ownedFiles: number; dryRun: boolean }[];
};

test("public authoring requests have independent closed language-neutral authority", () => {
  
  expect(requestInput["schemaVersion"]).toEqual(1);expect(requestInput["contractId"]).toEqual("artifact-scaffold-request-safety-v1");expect(requestInput["readBytes"]).toEqual(196608);expect(requestInput["readChunkBytes"]).toEqual(65536);expect(requestInput["cases"]).toEqual([{"id":"empty-request","leaves":[],"action":"none","error":"request must contain leaves","ownedFiles":0,"dryRun":true},{"id":"duplicate-leaf","leaves":["🦀️.rs","🦀️.rs"],"action":"none","error":"Invalid authored leaf request","ownedFiles":0,"dryRun":true},{"id":"noncanonical-leaf","leaves":["🦀️.rs","📝️note.md"],"action":"none","error":"Invalid authored leaf request","ownedFiles":0,"dryRun":true},{"id":"outside-owner","leaves":["🦀️.rs","../🟦️.ts"],"action":"none","error":"Invalid authoring coordinate","ownedFiles":0,"dryRun":true},{"id":"unowned-markdown","leaves":["🦀️.rs","📝️.md"],"action":"none","error":"Unowned authored leaf","ownedFiles":0,"dryRun":true},{"id":"globally-recognized-wrong-child","leaves":["🦀️.rs","🖼️assets/🧬️schema/🦀️.rs"],"action":"none","error":"Unpermitted authoring child","ownedFiles":0,"dryRun":true},{"id":"cancel-before-preflight","leaves":["🦀️.rs","📚️examples/📝️.md"],"action":"cancel-before","error":"Authoring cancelled","ownedFiles":0,"dryRun":false},{"id":"cancel-after-first-leaf","leaves":["🦀️.rs","📚️examples/📝️.md"],"action":"cancel-after-first","error":"Authoring cancelled","ownedFiles":1,"dryRun":false},{"id":"same-size-read-write","leaves":["🦀️.rs","📚️examples/📝️.md"],"action":"write-during-read","error":"changed while reading","ownedFiles":0,"dryRun":false}]);
  
  
  const errors: ParseError[] = [];
  expect(parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🪶️artifact-empty-facet-authoring/📨️request/🔣️.json"), "utf8"), errors)).toEqual(requestInput);
  expect(errors).toEqual([]);
});

for (const row of requestInput.cases)
  test(`public artifact authoring request ${row.id}`, () => {
    const target = fixture({ id: `subset-${row.id}`, producer: "subset", setup: "valid", role: "viewer", expected: "rejected" });
    const sourcesBefore = identities(),
      records: unknown[] = [];
    try {
      for (const dryRun of row.dryRun ? [false, true] : [false]) {
        const marker = join(target.workspace, target.marker);
        if (row.action === "write-during-read") {
          mkdirSync(dirname(marker), { recursive: true });
          writeFileSync(marker, Buffer.alloc(requestInput.readBytes, 65), { flag: "wx" });
        }
        const before = snapshot(target.root),
          leaves = row.leaves.map((path) => ({ path: `${target.subset}/${path}`, content: vector.customContent }));
        let cancelled = row.action === "cancel-before",
          injected = false,
          injectionSnapshot = before,
          operationError: unknown = null,
          readMutation: unknown = null;
        const progress = (event: ArtifactScaffoldProgress): void => {
          if (row.action === "cancel-after-first" && event.phase === "created") cancelled = true;
          if (row.action !== "write-during-read" || injected || event.phase !== "reading" || event.path !== target.marker || event.bytesRead !== requestInput.readChunkBytes) return;
          injected = true;
          const old = lstatSync(marker, { bigint: true });
          writeFileSync(marker, Buffer.alloc(requestInput.readBytes, 67));
          const current = lstatSync(marker, { bigint: true });
          readMutation = {
            bytesRead: event.bytesRead,
            before: { inode: old.ino.toString(), bytes: old.size.toString(), modifiedNanoseconds: old.mtimeNs.toString(), changedNanoseconds: old.ctimeNs.toString() },
            after: { inode: current.ino.toString(), bytes: current.size.toString(), modifiedNanoseconds: current.mtimeNs.toString(), changedNanoseconds: current.ctimeNs.toString() },
          };
          expect(current.ino).toBe(old.ino);
          expect(current.size).toBe(old.size);
          expect([current.mtimeNs.toString(), current.ctimeNs.toString()]).not.toEqual([old.mtimeNs.toString(), old.ctimeNs.toString()]);
          injectionSnapshot = snapshot(target.root);
        };
        try {
          authorArtifactScaffold(target.workspace, { kind: "subset", subsetPath: target.subset }, leaves, taxonomy, { dryRun, cancelled: () => cancelled, progress });
        } catch (error) {
          operationError = error;
        }
        const after = snapshot(target.root),
          partial = operationError instanceof ArtifactScaffoldError ? operationError.partial : null;
        records.push({ id: row.id, dryRun, injected, readMutation, error: String(operationError), partial, beforeDigest: hash(JSON.stringify(before)), afterDigest: hash(JSON.stringify(after)) });
        expect(operationError).toBeInstanceOf(ArtifactScaffoldError);
        expect(String(operationError)).toContain(row.error);
        expect(partial!.created).toHaveLength(row.ownedFiles);
        if (row.action === "write-during-read") expect(injected).toBe(true);
        if (row.ownedFiles === 0) expect(after).toEqual(injectionSnapshot);
        else
          for (const file of partial!.created) {
            const { path, ...identity } = file;
            expect(after[`🧪️workspace/${path}`]).toEqual<{ readonly kind: string } & typeof identity>({ kind: "file", ...identity });
          }
      }
      expect(identities()).toEqual(sourcesBefore);
    } finally {
      writeFileSync(join(target.root, "📝️.md"), `# Retained Public Authoring Request\n\n\`\`\`json\n${JSON.stringify({ records, sourcesBefore, sourcesAfter: identities() }, null, 2)}\n\`\`\`\n`, { flag: "wx" });
    }
  });

test("registers empty-facet authoring through its closed canonical route", async () => {
  const directory = join(import.meta.dir, "../../🧫️fixtures/🪶️artifact-empty-facet-authoring/📋️registration"),
    bytes = readFileSync(join(directory, "🔣️.json"), "utf8"),
    registration = JSON.parse(bytes);
  expect(registration).toEqual({ schemaVersion: 1, contractId: "artifact-empty-facet-authoring-registration-v1", command: "artifact-empty-facet-authoring", target: "test-artifact-empty-facet-authoring", cache: false, source: "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪶️artifact-empty-facet-authoring/🟦️.ts", runner: "runTestBudgeted", budget: "default" });
  const errors: ParseError[] = [];
  expect(parse(bytes, errors, { disallowComments: true, allowTrailingComma: false })).toEqual(registration);
  expect(errors).toEqual([]);
  const packageRelative = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript",
    packageRoot = join(repoRoot, packageRelative);
  expect(join(repoRoot, registration.source)).toBe(import.meta.filename);
  const project = JSON.parse(readFileSync(join(packageRoot, "📋️project.json"), "utf8"));
  expect(project.targets[registration.target]).toBeDefined();
  expect(project.targets[registration.target]).toEqual({ executor: "nx:run-commands", cache: registration.cache, options: { cwd: packageRelative, command: `bun ./📜️script.ts test ${registration.command}` } });
  const manifest = JSON.parse(readFileSync(join(packageRoot, "package.json"), "utf8"));
  expect(manifest.scripts[registration.target]).toBe(`nx run @semio-tech/repo-lib:${registration.target}`);
  const path = join(packageRoot, "📜️script.ts"),
    source = readFileSync(path, "utf8"),
    syntax = ts.createSourceFile(path, source, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const declarations = syntax.statements.filter((node) => ts.isClassDeclaration(node) && node.name?.text === "TestScript");
  expect(declarations.length).toBe(1);
  const environmentPath = join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🧪️test-output/🟦️.ts");
  const environmentSyntax = ts.createSourceFile(environmentPath, readFileSync(environmentPath, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const environmentOwner = environmentSyntax.statements.find((node) => ts.isFunctionDeclaration(node) && node.name?.text === "repoTestArtifactEnvironment");
  expect(environmentOwner).toBeDefined();
  const code = `${environmentOwner!.getText(environmentSyntax).replace(/^export /u, "")}\n${declarations[0]!.getText(syntax)}\nreturn new TestScript();`;
  for (const javascript of [new Bun.Transpiler({ loader: "ts" }).transformSync(code), ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText]) {
    const invocations: { executable: string; args: string[]; options: { cwd: string; env: NodeJS.ProcessEnv } }[] = [];
    class FixtureBundle {
      root = packageRoot;
      repoRoot = repoRoot;
    }
    const router = new Function("BundleScript", "join", "resolve", "mkdirSync", "runRepositoryTestCommand", "resolveTestLevel", javascript)(
      FixtureBundle,
      join,
      resolve,
      mkdirSync,
      async (executable: string, args: string[], options: { cwd: string; env: NodeJS.ProcessEnv }) => {
        invocations.push({ executable, args, options });
      },
      () => {
        throw new Error("Empty-facet authoring fell through to generic routing");
      },
    );
    await router.run([registration.command]);
    expect(invocations).toHaveLength(1);
    expect(invocations[0]!.executable).toBe(process.execPath);
    expect(invocations[0]!.args).toEqual(["test", join(repoRoot, registration.source)]);
    expect(invocations[0]!.options.cwd).toBe(repoRoot);
    expect(invocations[0]!.options.env.SEMIO_TEST_ARTIFACT_DIR).toBe(resolve(artifactRoot));
  }
});
