//#region 🔌️Adapters
import Ajv from "ajv/dist/2020";
import { describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import ts from "typescript";
import { cloneDeep } from "lodash";
import { resolve, relative, isAbsolute, join, sep, parse } from "node:path";
import { verifyNoFollowDirectoryChain } from "../../📁️input/🟦️.ts";
import { TICKET_GENERATED_OUTPUT_DIRECTORY } from "../../🟦️.ts";
//#endregion 🔌️Adapters

//#region 🧬️Fixture

const vectors = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🚪️source-admission/🧪️io/🔣️.json", import.meta.url), "utf8")) as { readonly cases: readonly { readonly id: string; readonly law: string; readonly input: Record<string, unknown>; readonly expected: Record<string, unknown> }[] };
const cases = vectors.cases;
const sourcePath = fileURLToPath(new URL("../../🟦️.ts", import.meta.url));
const source = readFileSync(sourcePath, "utf8");
const syntax = ts.createSourceFile(sourcePath, source, ts.ScriptTarget.Latest, true);
const providers = [syntax, ...["🛣️path", "📁️input", "🏃️operation", "🔣️taxonomy", "🚪️source-admission", "🚪️source-admission/📁️io"].map((owner) => {
  const path = fileURLToPath(new URL(`../../${owner}/🟦️.ts`, import.meta.url));
  return ts.createSourceFile(path, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true);
})];
const declaration = (name: string): string => {
  const rows = providers.flatMap((provider) => provider.statements.filter((row) => ((ts.isFunctionDeclaration(row) || ts.isClassDeclaration(row)) && row.name?.text === name) || (ts.isVariableStatement(row) && row.declarationList.declarations.some((item) => ts.isIdentifier(item.name) && item.name.text === name))).map((node) => ({ provider, node })));
  if (rows.length !== 1) throw new Error(`Current helper ${name} must have one canonical owner`);
  return rows[0]!.node.getText(rows[0]!.provider);
};
const invoke = (name: string, dependencies: readonly string[]): Function => new Function(...dependencies, ts.transpileModule(`${declaration(name).replace(/^export /u, "")}\nreturn ${name};`, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText);
const unsafe = invoke("UnsafeDirectoryAncestorError", [])() as new (message: string) => Error;
const records = invoke("sourceAdmissionGitRecords", ["TextDecoder"])(TextDecoder) as (bytes: Uint8Array, label: string) => readonly string[];
const safePath = invoke("sourceAdmissionSafePath", [])() as (path: string) => boolean;
const byteCompare = (left: string, right: string): number => Buffer.compare(Buffer.from(left), Buffer.from(right));
const containingRepository = invoke("sourceAdmissionContainingRepository", [])() as (path: string, fences: readonly string[], includeRoot: boolean) => string | null;
const assertRepositoryPath = invoke("sourceAdmissionAssertRepositoryPath", ["sourceAdmissionContainingRepository"])(containingRepository) as (path: string, fences: readonly string[], label: string, allowRoot: boolean) => void;
//#endregion 🧬️Fixture

//#region 🧪️IO
describe("taxonomy source admission IO", () => {
  test("neutral cases satisfy independent Ajv schema", () => {   expect(new Set(cases.map((row) => row.id)).size).toBe(cases.length); expect(cases).toHaveLength(10);  });
  for (const row of cases) test(row.id, () => {
    let handled = false;
    if (row.id === "strict-git-framing") { handled = true; let errors = 0; for (const hex of row.input.recordsHex as readonly string[]) try { records(Buffer.from(hex, "hex"), "mock"); } catch { errors++; } expect(errors).toBe<typeof row.expected.errors>(row.expected.errors); }
    if (row.id === "raw-git-spelling") { handled = true;
      const path = row.input.path as string, rows = invoke("sourceAdmissionGitRows", ["execFileSync", "sourceAdmissionGitExclusions", "sourceAdmissionGitRecords", "sourceAdmissionSafePath", "canonicalJson", "sourceAdmissionIndexObservation", "sourceAdmissionIndexObservations"])(() => Buffer.from(`100644 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa 0\t${path}\0`), () => [], records, safePath, JSON.stringify, () => undefined, new Map())("/fixture", { positivePathspec: ".", exclusionPathspecs: [] });
      expect(rows[0].path).toBe<typeof row.expected.path>(row.expected.path);
    }
    if (row.id === "opaque-untracked-pruning") { handled = true;
      let argumentsSeen: readonly string[] = [];
      invoke("sourceAdmissionUntrackedRows", ["execFileSync", "sourceAdmissionGitRecords", "sourceAdmissionSafePath", "sourceAdmissionByteCompare", "sourceAdmissionGitExclusions"])((_git: string, args: readonly string[]) => { argumentsSeen = args; return Buffer.from("safe.rs\0"); }, records, safePath, () => 0, () => [])("/fixture", { positivePathspec: ".", exclusionPathspecs: [] }, { exclusions: [{ path: row.input.opaque }] }, []);
      const boundary = argumentsSeen.indexOf("--"); for (const expected of row.expected.beforeBoundary as readonly string[]) expect(argumentsSeen.slice(0, boundary)).toContain(expected);
      expect(() => invoke("sourceAdmissionUntrackedRows", ["execFileSync", "sourceAdmissionGitRecords", "sourceAdmissionSafePath", "sourceAdmissionByteCompare", "sourceAdmissionGitExclusions"])(() => Buffer.from("unsafe\\path\0"), records, safePath, () => 0, () => [])("/fixture", { positivePathspec: ".", exclusionPathspecs: [] }, { exclusions: [] }, [])).toThrow();
    }
    if (row.id === "root-and-candidate-nofollow") { handled = true;
      const chain = invoke("noFollowDirectoryChain", ["parse", "sep", "join", "lstatSync", "UnsafeDirectoryAncestorError"])(() => ({ root: "/" }), "/", (...parts: string[]) => parts.join("/"), (path: string) => path.endsWith("fixture") ? { isSymbolicLink: () => true, isDirectory: () => true } : { isSymbolicLink: () => false, isDirectory: () => true }, unsafe);
      let rootError: unknown; try { chain(`${row.input.root}/child`); } catch (error) { rootError = error; } expect(rootError).toBeInstanceOf(unsafe); expect((rootError as Error).constructor.name).toBe<typeof row.expected.error>(row.expected.error);
      let descendant = 0;
      const candidate = invoke("sourceAdmissionLstat", ["sourceAdmissionAssertLexical", "noFollowDirectoryChain", "lstatOrNull", "join", "UnsafeDirectoryAncestorError", "verifyNoFollowDirectoryChain"])(() => {}, () => [], (path: string) => path.endsWith("/link") ? { isSymbolicLink: () => true, isDirectory: () => false } : (() => { descendant++; throw new Error("descendant lstat reached"); })(), (...parts: string[]) => parts.join("/"), unsafe, verifyNoFollowDirectoryChain);
      let candidateError: unknown; try { candidate(row.input.root, row.input.candidate); } catch (error) { candidateError = error; } expect(candidateError).toBeInstanceOf(unsafe); expect((candidateError as Error).constructor.name).toBe<typeof row.expected.error>(row.expected.error); expect(descendant).toBe<typeof row.expected.descendantLstat>(row.expected.descendantLstat);
    }
    if (row.id === "fifo-and-permission") { handled = true;
      const observe = invoke("sourceAdmissionObservation", ["sourceAdmissionLstat", "UnsafeDirectoryAncestorError"])(() => ({ isSymbolicLink: () => false, isDirectory: () => false, isFile: () => false, mode: row.input.mode }), unsafe);
      expect(observe("/fixture", "fifo", [], []).observedKind).toBe<typeof row.expected.kind>(row.expected.kind);
      const permission = invoke("sourceAdmissionObservation", ["sourceAdmissionLstat", "UnsafeDirectoryAncestorError"])(() => { const error = new Error(row.input.errno as string); (error as NodeJS.ErrnoException).code = row.input.errno as string; throw error; }, unsafe);
      expect(() => permission("/fixture", "denied", [], [])).toThrow(row.expected.error as string);
    }
    if (row.id === "nested-git-terminal") { handled = true;
      let reads = 0;
      const walk = invoke("sourceAdmissionWalk", ["sourceAdmissionSafePath", "sourceAdmissionOpaque", "inScope", "sourceAdmissionAssertRepositoryPath", "sourceAdmissionCheckCancellation", "sourceAdmissionLstat", "UnsafeDirectoryAncestorError", "sourceAdmissionContainingRepository", "readdirSync", "join", "TextDecoder", "sourceAdmissionByteCompare", "basename"])(safePath, () => false, () => true, assertRepositoryPath, () => {}, () => ({ isDirectory: () => true, isSymbolicLink: () => false, dev: 1, ino: 1, mode: 0o040000, mtimeMs: 1, ctimeMs: 1 }), unsafe, containingRepository, () => { reads++; return []; }, (...parts: string[]) => parts.join("/"), TextDecoder, () => 0, (path: string) => path.split("/").at(-1));
      expect(walk("/fixture", row.input.path, { exclusions: [], schema: { fixedDirectoryContracts: { "nested-git-metadata": { pathPattern: "**" } } }, pathMatcher: { matches: () => true } }, undefined, undefined, [])).toEqual<typeof row.expected.rows>(row.expected.rows); expect(reads).toBe<typeof row.expected.readdir>(row.expected.readdir);
    }
    if (row.id === "directory-identity-drift") { handled = true;
      let reads = 0;
      const walk = invoke("sourceAdmissionWalk", ["sourceAdmissionSafePath", "sourceAdmissionOpaque", "inScope", "sourceAdmissionAssertRepositoryPath", "sourceAdmissionCheckCancellation", "sourceAdmissionLstat", "UnsafeDirectoryAncestorError", "sourceAdmissionContainingRepository", "readdirSync", "join", "TextDecoder", "sourceAdmissionByteCompare", "basename"])(safePath, () => false, () => true, assertRepositoryPath, () => {}, () => ({ isDirectory: () => true, isSymbolicLink: () => false, dev: 1, ino: 1, mode: 0o040000, mtimeMs: 1, ctimeMs: ++reads === 1 ? row.input.before : row.input.after }), unsafe, containingRepository, () => [], (...parts: string[]) => parts.join("/"), TextDecoder, () => 0, (path: string) => path.split("/").at(-1));
      expect(() => walk("/fixture", row.input.path, { exclusions: [], schema: { fixedDirectoryContracts: {} }, pathMatcher: { matches: () => false } }, undefined, undefined, [])).toThrow(row.expected.error as string);
    }
    if (row.id === "loaded-hash-and-opaque-setup") { handled = true;
      const collect = invoke("collectTaxonomySourceAdmission", ["sourceAdmissionSafePath", "sourceAdmissionOpaque", "inScope", "sourceAdmissionAssertRepositoryPath", "taxonomyScopedGitPathspec", "sourceAdmissionCheckCancellation", "report", "sourceAdmissionUntrackedRows", "sourceAdmissionWalk", "SOURCE_ADMISSION_ORIGINS", "sourceAdmissionObservation", "projectTaxonomySourceAdmission", "relative", "sep", "sha256", "canonicalJson"])(safePath, () => false, () => true, assertRepositoryPath, () => ({ positivePathspec: ".", exclusionPathspecs: [] }), () => {}, () => {}, () => [], () => [], ["tracked", "nonignored-untracked", "ignored-generator", "explicit-ticket"], () => { throw new Error("unexpected observation"); }, () => ({ schemaVersion: 1, scope: null, status: "complete", observations: [], diagnostics: [] }), () => "taxonomy", "/", () => "digest", JSON.stringify);
      const publicSource = invoke("inventoryTaxonomySources", ["sourceAdmissionPrepareOptions", "loadTaxonomy", "collectTaxonomySourceAdmission"])(() => ({ repoRoot: "/fixture", scope: undefined, taxonomyPath: "/fixture/taxonomy", ticketDir: undefined, cancelFile: undefined, indexRows: [], repositoryFences: [] }), () => ({ path: "/fixture/taxonomy", input: { contentHash: row.input.contentHash }, exclusions: [], schema: { generatorContracts: {} } }), collect);
      const result = publicSource({ repoRoot: "/fixture" });
      expect(result.taxonomyContentHash).toBe<typeof row.expected.contentHash>(row.expected.contentHash); expect(Object.hasOwn(result, "inputText")).toBe(false); expect(Object.hasOwn(result, "inventory")).toBe(false);
      let filesystemCalls = 0;
      const lexical = invoke("sourceAdmissionAssertLexical", ["Buffer", "isAbsolute", "parse", "sep", "sourceAdmissionSafePath"])(Buffer, () => true, () => ({ root: "/" }), "/", safePath);
      const prepare = invoke("sourceAdmissionPrepareOptions", ["sourceAdmissionAssertLexical", "TAXONOMY_RELATIVE_PATH", "resolve", "relative", "isAbsolute", "join", "sep", "noFollowDirectoryChain", "sourceAdmissionLstat"])(lexical, "taxonomy", (value: string) => value, () => "", () => true, (...parts: string[]) => parts.join("/"), "/", () => { filesystemCalls++; return []; }, () => { filesystemCalls++; return null; });
      const full = invoke("inventoryTaxonomyWithSourceParentPruning", ["sourceAdmissionPrepareOptions", "report", "loadTaxonomy"])(prepare, () => {}, () => { filesystemCalls++; throw new Error("filesystem"); });
      expect(() => full({ repoRoot: "/fixture", scope: row.input.scope }, new Set())).toThrow(row.expected.error as string); expect(filesystemCalls).toBe<typeof row.expected.filesystem>(row.expected.filesystem);
    }
    if (row.id === "ticket-generated-output-exclusion") { handled = true;
      const ticketDir = row.input.ticketDir as string;
      const stats: Record<string, { isSymbolicLink: () => boolean; isDirectory: () => boolean; mode: number }> = {
        "repo/ticket": { isSymbolicLink: () => false, isDirectory: () => true, mode: 0o040000 },
        "repo/ticket/keep": { isSymbolicLink: () => false, isDirectory: () => true, mode: 0o040000 },
        "repo/ticket/keep/file.txt": { isSymbolicLink: () => false, isDirectory: () => false, mode: 0o100644 },
      };
      const listings: Record<string, readonly string[]> = { "repo/ticket": ["keep", TICKET_GENERATED_OUTPUT_DIRECTORY], "repo/ticket/keep": ["file.txt"] };
      const readdirCalls: string[] = [];
      const explicitTicketRows = invoke("explicitTicketRows", ["sourceRelative", "isAbsolute", "relative", "resolve", "isExcluded", "inScope", "absolutePath", "existsSync", "checkCancellation", "lstatSync", "readdirSync", "basename", "TICKET_GENERATED_OUTPUT_DIRECTORY"])(
        (path: string) => path,
        () => false,
        () => "",
        () => "",
        () => false,
        () => true,
        (root: string, rel: string) => `${root}/${rel}`,
        () => true,
        () => {},
        (path: string) => { if (!stats[path]) throw new Error(`unexpected lstat ${path}`); return stats[path]; },
        (path: string) => { readdirCalls.push(path); if (!listings[path]) throw new Error(`unexpected readdir ${path}`); return listings[path]; },
        (path: string) => path.split("/").at(-1),
        TICKET_GENERATED_OUTPUT_DIRECTORY,
      ) as (repoRoot: string, ticketDir: string | undefined, taxonomy: unknown, scope: string | undefined, cancelFile: string | undefined) => readonly unknown[];
      const rows = explicitTicketRows("repo", ticketDir, { schema: { fixedDirectoryContracts: {} } }, undefined, undefined);
      expect(rows).toEqual<typeof row.expected.rows>(row.expected.rows);
      expect(readdirCalls).toEqual<typeof row.expected.readdirCalls>(row.expected.readdirCalls);
    }
    if (row.id === "callback-request-snapshot") { handled = true;
      const expectedInput = cloneDeep(row.input.options) as { repoRoot: string; scope: string; taxonomyPath: string; structuralDirectoryNames: string[] };
      const mutation = row.input.mutation as { scope: string; structuralDirectoryNames: string[] };
      const expectedRows = cloneDeep(row.input.indexRows) as { path: string; entry: { mode: string; objectId: string; stage: number } }[];
      const indexRows = cloneDeep(expectedRows), repositoryFences = cloneDeep(row.input.repositoryFences) as string[];
      const options: typeof expectedInput & { progress?: (event: any) => void } = cloneDeep(expectedInput);
      const events: any[] = [];
      let replacementCallbacks = 0;
      options.progress = (event) => {
        events.push(event);
        if (event.phase === "tracked-enumeration" && event.current === 0) {
          options.scope = mutation.scope;
          options.structuralDirectoryNames.splice(0, options.structuralDirectoryNames.length, ...mutation.structuralDirectoryNames);
          options.progress = () => { replacementCallbacks++; };
        }
      };
      const lexical = invoke("sourceAdmissionAssertLexical", ["Buffer", "isAbsolute", "parse", "sep", "sourceAdmissionSafePath"])(Buffer, isAbsolute, parse, sep, safePath);
      const emit = invoke("report", [])();
      const prepare = invoke("sourceAdmissionPrepareOptions", ["sourceAdmissionAssertLexical", "TAXONOMY_RELATIVE_PATH", "resolve", "relative", "isAbsolute", "join", "sep", "noFollowDirectoryChain", "report", "sourceAdmissionGitRows", "taxonomyScopedGitPathspec", "sourceAdmissionRepositoryFences", "sourceAdmissionAssertRepositoryPath", "sourceAdmissionLstat"])(lexical, "taxonomy", resolve, relative, isAbsolute, join, sep, () => [], emit, () => indexRows, () => ({}), () => repositoryFences, assertRepositoryPath, () => ({ isFile: () => true, isSymbolicLink: () => false }));
      const prepared = prepare(options);
      console.log(`[DEBUG] callback-request-snapshot scope=${prepared.scope} tracked-callbacks=${events.length} replacement-callbacks=${replacementCallbacks}`);
      expect(prepared.scope).toBe<typeof row.expected.scope>(row.expected.scope);
      expect(prepared.structuralDirectoryNames).toEqual(expectedInput.structuralDirectoryNames);
      expect(prepared.structuralDirectoryNames).toEqual<typeof row.expected.structuralDirectoryNames>(row.expected.structuralDirectoryNames);
      expect(Object.isFrozen(prepared)).toBe(true);
      expect(Reflect.set(prepared, "scope", mutation.scope)).toBe(false);
      expect(Object.isFrozen(prepared.structuralDirectoryNames)).toBe(true);
      indexRows[0]!.path = "mutated";
      indexRows[0]!.entry.mode = "120000";
      repositoryFences[0] = "mutated";
      expect(prepared.indexRows).toEqual(expectedRows);
      expect(prepared.repositoryFences).toEqual<typeof row.input.repositoryFences>(row.input.repositoryFences);
      for (const value of [prepared.indexRows, prepared.indexRows[0], prepared.indexRows[0].entry, prepared.repositoryFences]) expect(Object.isFrozen(value)).toBe<typeof row.expected.preparedFactsFrozen>(row.expected.preparedFactsFrozen);
      expect(Reflect.set(prepared.indexRows[0].entry, "mode", "120000")).toBe(false);
      expect(replacementCallbacks).toBe<typeof row.expected.replacementCallbacks>(row.expected.replacementCallbacks);
      expect(events).toHaveLength(2);
      for (const event of events) {
        expect(Object.isFrozen(event)).toBe<typeof row.expected.progressFrozen>(row.expected.progressFrozen);
        expect(Reflect.set(event, "phase", "changed")).toBe(false);
      }
      let structuralNames: readonly string[] | undefined;
      const collect = invoke("collectTaxonomySourceAdmission", ["sourceAdmissionSafePath", "sourceAdmissionOpaque", "inScope", "sourceAdmissionAssertRepositoryPath", "taxonomyScopedGitPathspec", "sourceAdmissionCheckCancellation", "report", "sourceAdmissionUntrackedRows", "sourceAdmissionStructuralDirectories", "sourceAdmissionWalk", "SOURCE_ADMISSION_ORIGINS", "sourceAdmissionObservation", "projectTaxonomySourceAdmission", "relative", "sep", "sha256", "canonicalJson"])(safePath, () => false, (path: string, scope: string) => path === scope || path.startsWith(scope + "/"), assertRepositoryPath, () => ({}), () => {}, emit, () => [], (_root: string, names: readonly string[]) => { structuralNames = names; return []; }, () => [], [], () => { throw Error("Unexpected observation"); }, (input: any) => ({ schemaVersion: 1, scope: input.scope, status: "complete", observations: [], diagnostics: [] }), relative, sep, () => "digest", JSON.stringify);
      collect({ path: prepared.taxonomyPath, input: { contentHash: "captured" }, exclusions: [], schema: { generatorContracts: {} } }, prepared);
      expect(structuralNames).toEqual(expectedInput.structuralDirectoryNames);
      expect(replacementCallbacks).toBe<typeof row.expected.replacementCallbacks>(row.expected.replacementCallbacks);
      const workerCases = row.input.inventoryWorkerCases as readonly { workers: number; mutation: number }[];
      const outcomes = row.expected.workerOutcomes as readonly string[];
      for (const [index, worker] of workerCases.entries()) {
        const inventoryOptions = { ...cloneDeep(expectedInput), workers: worker.workers, progress: (_event: unknown) => { inventoryOptions.workers = worker.mutation; } };
        const inventory = invoke("inventoryTaxonomyWithSourceParentPruning", ["sourceAdmissionPrepareOptions", "report", "loadTaxonomy"])((request: any) => { emit(request.progress, "inventory", "tracked-enumeration", 0, 1, request.scope); return { repoRoot: request.repoRoot, scope: request.scope, taxonomyPath: request.taxonomyPath }; }, emit, () => { throw Error("taxonomy-boundary"); });
        expect(() => inventory(inventoryOptions, new Set(), () => new Uint8Array())).toThrow(outcomes[index]!);
        console.log(`[DEBUG] inventory-worker-snapshot original=${worker.workers} attempted=${worker.mutation} outcome=${outcomes[index]}`);
      }
    }
    if (!handled) throw new Error(`Unknown neutral IO case: ${row.id}`);
  });
});
//#endregion 🧪️IO
