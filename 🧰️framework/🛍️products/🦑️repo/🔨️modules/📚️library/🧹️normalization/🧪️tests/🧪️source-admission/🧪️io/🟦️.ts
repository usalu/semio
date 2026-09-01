//#region 🔌️Adapters
import Ajv from "ajv/dist/2020";
import { describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import ts from "typescript";
import { TICKET_GENERATED_OUTPUT_DIRECTORY } from "../../../🟦️.ts";
//#endregion 🔌️Adapters

//#region 🧬️Fixture
const schema = JSON.parse(readFileSync(new URL("./🧬️schema/🔣️.json", import.meta.url), "utf8"));
const vectors = JSON.parse(readFileSync(new URL("./🔣️.json", import.meta.url), "utf8")) as { readonly cases: readonly { readonly id: string; readonly law: string; readonly input: Record<string, unknown>; readonly expected: Record<string, unknown> }[] };
const cases = vectors.cases;
const sourcePath = fileURLToPath(new URL("../../../🟦️.ts", import.meta.url));
const source = readFileSync(sourcePath, "utf8");
const syntax = ts.createSourceFile(sourcePath, source, ts.ScriptTarget.Latest, true);
const declaration = (name: string): string => {
  const node = syntax.statements.find((row) => ((ts.isFunctionDeclaration(row) || ts.isClassDeclaration(row)) && row.name?.text === name) || (ts.isVariableStatement(row) && row.declarationList.declarations.some((item) => ts.isIdentifier(item.name) && item.name.text === name)));
  if (!node) throw new Error(`Missing current helper ${name}`);
  return node.getText(syntax);
};
const invoke = (name: string, dependencies: readonly string[]): Function => new Function(...dependencies, ts.transpileModule(`${declaration(name).replace(/^export /u, "")}\nreturn ${name};`, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText);
const unsafe = invoke("SourceAdmissionUnsafeAncestorError", [])() as new (message: string) => Error;
const records = invoke("sourceAdmissionGitRecords", ["TextDecoder"])(TextDecoder) as (bytes: Uint8Array, label: string) => readonly string[];
const safePath = invoke("sourceAdmissionSafePath", [])() as (path: string) => boolean;
const byteCompare = (left: string, right: string): number => Buffer.compare(Buffer.from(left), Buffer.from(right));
const containingRepository = invoke("sourceAdmissionContainingRepository", [])() as (path: string, fences: readonly string[], includeRoot: boolean) => string | null;
const assertRepositoryPath = invoke("sourceAdmissionAssertRepositoryPath", ["sourceAdmissionContainingRepository"])(containingRepository) as (path: string, fences: readonly string[], label: string, allowRoot: boolean) => void;
//#endregion 🧬️Fixture

//#region 🧪️IO
describe("taxonomy source admission IO", () => {
  test("neutral cases satisfy independent Ajv schema", () => { const validate = new Ajv({ strict: true }).compile(schema); expect(validate(vectors)).toBe(true); expect(new Set(cases.map((row) => row.id)).size).toBe(cases.length); expect(cases).toHaveLength(9); for (const candidate of [{ cases: cases.map((row) => row.id === "raw-git-spelling" ? { ...row, input: {} } : row) }, { cases: cases.map((row) => row.id === "raw-git-spelling" ? { ...row, extra: true } : row) }, { cases: cases.map((row) => row.id === "raw-git-spelling" ? { ...row, id: "unknown" } : row) }]) expect(validate(candidate)).toBe(false); });
  for (const row of cases) test(row.id, () => {
    let handled = false;
    if (row.id === "strict-git-framing") { handled = true; let errors = 0; for (const hex of row.input.recordsHex as readonly string[]) try { records(Buffer.from(hex, "hex"), "mock"); } catch { errors++; } expect(errors).toBe(row.expected.errors); }
    if (row.id === "raw-git-spelling") { handled = true;
      const path = row.input.path as string, rows = invoke("sourceAdmissionGitRows", ["execFileSync", "sourceAdmissionGitExclusions", "sourceAdmissionGitRecords", "sourceAdmissionSafePath"])(() => Buffer.from(`100644 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa 0\t${path}\0`), () => [], records, safePath)("/fixture", {}, { positivePathspec: ".", exclusionPathspecs: [] });
      expect(rows[0].path).toBe(row.expected.path);
    }
    if (row.id === "opaque-untracked-pruning") { handled = true;
      let argumentsSeen: readonly string[] = [];
      invoke("sourceAdmissionUntrackedRows", ["execFileSync", "sourceAdmissionGitRecords", "sourceAdmissionSafePath", "sourceAdmissionByteCompare", "sourceAdmissionGitExclusions"])((_git: string, args: readonly string[]) => { argumentsSeen = args; return Buffer.from("safe.rs\0"); }, records, safePath, () => 0, () => [])("/fixture", { positivePathspec: ".", exclusionPathspecs: [] }, { exclusions: [{ path: row.input.opaque }] }, []);
      const boundary = argumentsSeen.indexOf("--"); for (const expected of row.expected.beforeBoundary as readonly string[]) expect(argumentsSeen.slice(0, boundary)).toContain(expected);
      expect(() => invoke("sourceAdmissionUntrackedRows", ["execFileSync", "sourceAdmissionGitRecords", "sourceAdmissionSafePath", "sourceAdmissionByteCompare", "sourceAdmissionGitExclusions"])(() => Buffer.from("unsafe\\path\0"), records, safePath, () => 0, () => [])("/fixture", { positivePathspec: ".", exclusionPathspecs: [] }, { exclusions: [] }, [])).toThrow();
    }
    if (row.id === "root-and-candidate-nofollow") { handled = true;
      const chain = invoke("sourceAdmissionDirectoryChain", ["parse", "sep", "join", "lstatSync", "SourceAdmissionUnsafeAncestorError"])(() => ({ root: "/" }), "/", (...parts: string[]) => parts.join("/"), (path: string) => path.endsWith("fixture") ? { isSymbolicLink: () => true, isDirectory: () => true } : { isSymbolicLink: () => false, isDirectory: () => true }, unsafe);
      let rootError: unknown; try { chain(`${row.input.root}/child`); } catch (error) { rootError = error; } expect(rootError).toBeInstanceOf(unsafe); expect((rootError as Error).constructor.name).toBe(row.expected.error);
      let descendant = 0;
      const candidate = invoke("sourceAdmissionLstat", ["sourceAdmissionAssertLexical", "sourceAdmissionDirectoryChain", "lstatOrNull", "join", "SourceAdmissionUnsafeAncestorError"])(() => {}, () => [], (path: string) => path.endsWith("/link") ? { isSymbolicLink: () => true, isDirectory: () => false } : (() => { descendant++; throw new Error("descendant lstat reached"); })(), (...parts: string[]) => parts.join("/"), unsafe);
      let candidateError: unknown; try { candidate(row.input.root, row.input.candidate); } catch (error) { candidateError = error; } expect(candidateError).toBeInstanceOf(unsafe); expect((candidateError as Error).constructor.name).toBe(row.expected.error); expect(descendant).toBe(row.expected.descendantLstat);
    }
    if (row.id === "fifo-and-permission") { handled = true;
      const observe = invoke("sourceAdmissionObservation", ["sourceAdmissionLstat", "SourceAdmissionUnsafeAncestorError"])(() => ({ isSymbolicLink: () => false, isDirectory: () => false, isFile: () => false, mode: row.input.mode }), unsafe);
      expect(observe("/fixture", "fifo", [], []).observedKind).toBe(row.expected.kind);
      const permission = invoke("sourceAdmissionObservation", ["sourceAdmissionLstat", "SourceAdmissionUnsafeAncestorError"])(() => { const error = new Error(row.input.errno as string); (error as NodeJS.ErrnoException).code = row.input.errno as string; throw error; }, unsafe);
      expect(() => permission("/fixture", "denied", [], [])).toThrow(row.expected.error as string);
    }
    if (row.id === "nested-git-terminal") { handled = true;
      let reads = 0;
      const walk = invoke("sourceAdmissionWalk", ["sourceAdmissionSafePath", "sourceAdmissionOpaque", "inScope", "sourceAdmissionAssertRepositoryPath", "sourceAdmissionCheckCancellation", "sourceAdmissionLstat", "SourceAdmissionUnsafeAncestorError", "sourceAdmissionContainingRepository", "readdirSync", "join", "TextDecoder", "sourceAdmissionByteCompare", "basename"])(safePath, () => false, () => true, assertRepositoryPath, () => {}, () => ({ isDirectory: () => true, isSymbolicLink: () => false, dev: 1, ino: 1, mode: 0o040000, mtimeMs: 1, ctimeMs: 1 }), unsafe, containingRepository, () => { reads++; return []; }, (...parts: string[]) => parts.join("/"), TextDecoder, () => 0, (path: string) => path.split("/").at(-1));
      expect(walk("/fixture", row.input.path, { exclusions: [], schema: { fixedDirectoryContracts: { "nested-git-metadata": { pathPattern: "**" } } }, pathMatcher: { matches: () => true } }, undefined, undefined, [])).toEqual(row.expected.rows); expect(reads).toBe(row.expected.readdir);
    }
    if (row.id === "directory-identity-drift") { handled = true;
      let reads = 0;
      const walk = invoke("sourceAdmissionWalk", ["sourceAdmissionSafePath", "sourceAdmissionOpaque", "inScope", "sourceAdmissionAssertRepositoryPath", "sourceAdmissionCheckCancellation", "sourceAdmissionLstat", "SourceAdmissionUnsafeAncestorError", "sourceAdmissionContainingRepository", "readdirSync", "join", "TextDecoder", "sourceAdmissionByteCompare", "basename"])(safePath, () => false, () => true, assertRepositoryPath, () => {}, () => ({ isDirectory: () => true, isSymbolicLink: () => false, dev: 1, ino: 1, mode: 0o040000, mtimeMs: 1, ctimeMs: ++reads === 1 ? row.input.before : row.input.after }), unsafe, containingRepository, () => [], (...parts: string[]) => parts.join("/"), TextDecoder, () => 0, (path: string) => path.split("/").at(-1));
      expect(() => walk("/fixture", row.input.path, { exclusions: [], schema: { fixedDirectoryContracts: {} }, pathMatcher: { matches: () => false } }, undefined, undefined, [])).toThrow(row.expected.error as string);
    }
    if (row.id === "loaded-hash-and-opaque-setup") { handled = true;
      const collect = invoke("collectTaxonomySourceAdmission", ["sourceAdmissionSafePath", "sourceAdmissionOpaque", "inScope", "sourceAdmissionAssertRepositoryPath", "taxonomyScopedGitPathspec", "sourceAdmissionCheckCancellation", "report", "sourceAdmissionUntrackedRows", "sourceAdmissionWalk", "SOURCE_ADMISSION_ORIGINS", "sourceAdmissionObservation", "projectTaxonomySourceAdmission", "relative", "sep", "sha256", "canonicalJson"])(safePath, () => false, () => true, assertRepositoryPath, () => ({ positivePathspec: ".", exclusionPathspecs: [] }), () => {}, () => {}, () => [], () => [], ["tracked", "nonignored-untracked", "ignored-generator", "explicit-ticket"], () => { throw new Error("unexpected observation"); }, () => ({ schemaVersion: 1, scope: null, status: "complete", observations: [], diagnostics: [] }), () => "taxonomy", "/", () => "digest", JSON.stringify);
      const publicSource = invoke("inventoryTaxonomySources", ["sourceAdmissionPrepareOptions", "loadTaxonomy", "collectTaxonomySourceAdmission"])(() => ({ repoRoot: "/fixture", scope: undefined, taxonomyPath: "/fixture/taxonomy", ticketDir: undefined, cancelFile: undefined, indexRows: [], repositoryFences: [] }), () => ({ path: "/fixture/taxonomy", input: { contentHash: row.input.contentHash }, exclusions: [], schema: { generatorContracts: {} } }), collect);
      const result = publicSource({ repoRoot: "/fixture" });
      expect(result.taxonomyContentHash).toBe(row.expected.contentHash); expect(Object.hasOwn(result, "inputText")).toBe(false); expect(Object.hasOwn(result, "inventory")).toBe(false);
      let filesystemCalls = 0;
      const lexical = invoke("sourceAdmissionAssertLexical", ["Buffer", "isAbsolute", "parse", "sep", "sourceAdmissionSafePath"])(Buffer, () => true, () => ({ root: "/" }), "/", safePath);
      const prepare = invoke("sourceAdmissionPrepareOptions", ["sourceAdmissionAssertLexical", "TAXONOMY_RELATIVE_PATH", "resolve", "relative", "isAbsolute", "join", "sep", "sourceAdmissionDirectoryChain", "sourceAdmissionLstat"])(lexical, "taxonomy", (value: string) => value, () => "", () => true, (...parts: string[]) => parts.join("/"), "/", () => { filesystemCalls++; return []; }, () => { filesystemCalls++; return null; });
      const full = invoke("inventoryTaxonomyWithSourceParentPruning", ["sourceAdmissionPrepareOptions", "report", "loadTaxonomy"])(prepare, () => {}, () => { filesystemCalls++; throw new Error("filesystem"); });
      expect(() => full({ repoRoot: "/fixture", scope: row.input.scope }, new Set())).toThrow(row.expected.error as string); expect(filesystemCalls).toBe(row.expected.filesystem);
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
      expect(rows).toEqual(row.expected.rows);
      expect(readdirCalls).toEqual(row.expected.readdirCalls);
    }
    if (!handled) throw new Error(`Unknown neutral IO case: ${row.id}`);
  });
});
//#endregion 🧪️IO
