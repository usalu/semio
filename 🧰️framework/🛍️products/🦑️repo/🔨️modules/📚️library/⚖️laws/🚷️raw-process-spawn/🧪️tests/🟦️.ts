import { afterAll, beforeAll, describe, expect, test } from "bun:test";
import Ajv from "ajv";
import ts from "typescript";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { runToolSyncV1, toolGlobMatchesV1, ToolErrorV1, TOOL_REGISTRY_V1, type ToolRegistryV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";
import { gitSyncV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";
import allowlistDocument from "../📒️allowlist/🔣️.json" with { type: "json" };
import allowlistSchema from "../🧬️schema/🔣️.json" with { type: "json" };
import fixtures from "./🧫️fixtures/🔣️.json" with { type: "json" };
import { runProcessSpawnLawV1, runToolSpawnLawsV1 } from "../⌨️command/🟦️.ts";
import { judgeProcessStartsV1, listProcessSpawnFilesV1, processAllowlistProblemsV1, processCommandNameV1, processInterfaceGlobsV1, readProcessAllowlistV1, scanProcessStartsV1, PROCESS_ALLOWLIST_V1, PROCESS_SPAWN_LAW_DIRECTORY_V1, type ProcessAllowlistV1, type ProcessSiteV1 } from "../🟦️.ts";

const here = dirname(fileURLToPath(import.meta.url)), repository = resolve(here, ...Array(PROCESS_SPAWN_LAW_DIRECTORY_V1.split("/").length + 1).fill(".."));
const oraclePath = join(here, "🐍️oracle", "🐍️.py");
const registry = fixtures.registry as unknown as ToolRegistryV1, law = fixtures.law as unknown as ProcessAllowlistV1;
const row = (site: ProcessSiteV1): unknown[] => [site.line, site.callee, site.form, site.program, site.scope];

describe("raw process spawn law", () => {
  test("keeps its allowlist valid against the schema with a third-party validator and refuses malformed documents", () => {
    const ajv = new (Ajv as any)({ strict: false, allErrors: true });
    expect(ajv.validate(allowlistSchema, allowlistDocument)).toBe(true);
    expect(() => readProcessAllowlistV1({ ...allowlistDocument, exceptions: [{ path: "a", program: "b", count: 0, class: "os-primitive", reason: { en: "x", de: "y" } }] })).toThrow("invalid");
    expect(() => readProcessAllowlistV1({ ...allowlistDocument, exceptions: [{ path: "a", program: "b", count: 1, class: "whatever", reason: { en: "x", de: "y" } }] })).toThrow("invalid");
    expect(() => readProcessAllowlistV1({ ...allowlistDocument, interfaces: [{ glob: "a/**", kind: "runtime-port", reason: { en: "only English" } }] })).toThrow("invalid");
    expect(processAllowlistProblemsV1(new Set(), PROCESS_ALLOWLIST_V1).filter(problem => !problem.detail.includes("matches no file"))).toEqual([]);
  });

  test("lists every exception once per file and program, in order", () => {
    const keys = PROCESS_ALLOWLIST_V1.exceptions.map(entry => `${entry.path}\0${entry.program}`);
    expect(new Set(keys).size).toBe(keys.length);
    expect([...keys].sort()).toEqual(keys);
  });

  test("scans every fixture case to the same sites as the language-agnostic expectation", () => {
    for (const entry of fixtures.scanCases) expect({ name: entry.name, sites: scanProcessStartsV1(entry.path, entry.text, registry, law).map(row) }).toEqual({ name: entry.name, sites: entry.expect });
  });

  test("matches globs like the registry does and like the Python oracle expects", () => {
    for (const entry of fixtures.globCases) expect({ ...entry, got: toolGlobMatchesV1(entry.pattern, entry.path) }).toEqual({ ...entry, got: entry.matches });
  });

  test("judges every law case to the same dispositions, findings and stale rows", () => {
    for (const entry of fixtures.lawCases) {
      const sites = entry.files.flatMap(file => scanProcessStartsV1(file.path, file.text, registry, law));
      const verdict = judgeProcessStartsV1(sites, registry, { ...law, exceptions: ((entry as { exceptions?: ProcessAllowlistV1["exceptions"] }).exceptions ?? []), roots: (entry as { roots?: string[] }).roots ?? law.roots });
      expect({ name: entry.name, dispositions: verdict.rows.map(site => site.disposition), findings: verdict.findings.map(finding => `${finding.rule} ${finding.path} ${finding.program}`).sort(), stale: verdict.stale }).toEqual({ name: entry.name, ...entry.expect });
    }
  });

  test("counts the calls of the TypeScript compiler's syntax tree, the independent oracle for comments, strings and regular expressions", () => {
    const names = new Set(["spawn", "spawnSync", "execFile", "execFileSync", "exec", "execSync", "fork"]);
    for (const entry of fixtures.scanCases.filter(item => item.path.endsWith(".ts") && item.name.startsWith("ts ") && !item.name.includes("runtime port") && !item.name.includes("funnels") && !item.name.includes("bun shell") && !item.name.includes("without process import"))) {
      const source = ts.createSourceFile(entry.path, entry.text, ts.ScriptTarget.Latest, true), locals = new Set<string>(), spaces = new Set<string>(), lines = new Set<number>();
      source.forEachChild(function visit(node) {
        if (ts.isImportDeclaration(node) && ts.isStringLiteral(node.moduleSpecifier) && /^(?:node:)?child_process$/u.test(node.moduleSpecifier.text)) {
          const bindings = node.importClause?.namedBindings;
          if (bindings !== undefined && ts.isNamespaceImport(bindings)) spaces.add(bindings.name.text);
          if (bindings !== undefined && ts.isNamedImports(bindings)) for (const element of bindings.elements) if (names.has((element.propertyName ?? element.name).text)) locals.add(element.name.text);
        }
        if (ts.isCallExpression(node)) {
          const callee = node.expression, line = source.getLineAndCharacterOfPosition(node.getStart()).line + 1;
          if (ts.isIdentifier(callee) && locals.has(callee.text)) lines.add(line);
          if (ts.isPropertyAccessExpression(callee) && ts.isIdentifier(callee.expression) && spaces.has(callee.expression.text) && names.has(callee.name.text)) lines.add(line);
        }
        ts.forEachChild(node, visit);
      });
      expect({ name: entry.name, lines: [...new Set(scanProcessStartsV1(entry.path, entry.text, registry, law).filter(site => site.form !== "funnel" && site.callee !== "$" && !site.callee.startsWith("runtime")).map(site => site.line))].sort((a, b) => a - b) }).toEqual({ name: entry.name, lines: [...lines].sort((a, b) => a - b) });
    }
  });

  test("agrees with the Python oracle on real production files of every language", () => {
    const wanted = [
      "🧰️framework/🔨️modules/🏃️process/🪓️termination/🟦️.ts", "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts", "🧰️framework/🔨️modules/🏃️process/📥️capture/🟦️.ts",
      "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧵️executor/🐹️.go", "🧰️framework/🛍️products/🦑️repo/🔨️modules/🏠️workspace/📦️packages/🐹️go/🌿️git/🐹️.go", "🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/📦️packages/🐹️go/🐚️unix-host/🐹️.go",
      "🧰️framework/🔨️modules/🖱️ui/⌨️tui/🔌️backend/📋️clipboard-and-osc52/🦀️.rs", "🧰️framework/🔨️modules/🖱️ui/⌨️tui/🚇️pty/🦀️.rs", "🧰️framework/🛍️products/🦑️repo/🔨️modules/🏠️workspace/🦀️.rs",
      "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🚀️apply/🏁️transaction-run/🔧️transaction-steps/🟦️.ts", "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🌐️network/🔌️ports/🟦️.ts",
    ].filter(path => existsSync(join(repository, path)));
    expect(wanted.length).toBeGreaterThan(3);
    const listing = join(mkdtempSync(join(tmpdir(), "semio-process-law-")), "files.txt");
    writeFileSync(listing, wanted.join("\n") + "\n");
    let output: string;
    try { output = runToolSyncV1("python", [oraclePath, "sites", "--root", repository, "--files", listing], { encoding: "utf8", check: true, maxBuffer: 64 * 1024 * 1024 }).stdout; } catch (error) { if (error instanceof ToolErrorV1 && error.code === "tool-missing") return; throw error; }
    const oracle = (JSON.parse(output) as ProcessSiteV1[]).map(site => [site.path, site.line, site.callee, site.form, site.program, site.scope]);
    const mine = wanted.flatMap(path => scanProcessStartsV1(path, readFileSync(join(repository, path), "utf8"))).map(site => [site.path, site.line, site.callee, site.form, site.program, site.scope]);
    expect(mine.length).toBeGreaterThan(5);
    expect(mine).toEqual(oracle);
  });

  describe("in a temporary git repository", () => {
    let root = "";
    const write = (path: string, text: string): void => { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), text); };
    beforeAll(() => {
      root = mkdtempSync(join(tmpdir(), "semio-process-law-repo-"));
      for (const args of [["init", "-q"], ["config", "user.email", "t@example.org"], ["config", "user.name", "t"]]) gitSyncV1(args, { cwd: root });
    });
    afterAll(() => { rmSync(root, { recursive: true, force: true }); });
    const allowlist = (exceptions: ProcessAllowlistV1["exceptions"] = []): ProcessAllowlistV1 => ({ ...PROCESS_ALLOWLIST_V1, roots: ["src"], interfaces: [], exceptions });
    test("passes a repository whose only spawn is documented, fails on a new raw spawn and on a stale-free strict run", () => {
      write("src/a/🟦️.ts", 'import { spawnSync } from "node:child_process";\nspawnSync("kill", ["-0", "1"]);\n');
      gitSyncV1(["add", "-A"], { cwd: root });
      const lines: string[] = [];
      const documented = allowlist([{ path: "src/a/🟦️.ts", program: "kill", count: 1, class: "os-primitive", reason: { en: "Signals a pid.", de: "Signalisiert eine Pid." } }]);
      expect(listProcessSpawnFilesV1(root, documented)).toEqual(["src/a/🟦️.ts"]);
      expect(runProcessSpawnLawV1(root, ["--json"], line => lines.push(line), documented)).toBe(0);
      expect(JSON.parse(lines.join("\n"))).toMatchObject({ scanned: 1, sites: 1, findings: [], stale: [] });
      expect(runProcessSpawnLawV1(root, ["--json"], () => {}, allowlist())).toBe(1);
      write("src/b/🟦️.ts", 'import { spawn } from "node:child_process";\nspawn(program, []);\n');
      gitSyncV1(["add", "-A"], { cwd: root });
      const second: string[] = [];
      expect(runProcessSpawnLawV1(root, ["--json"], line => second.push(line), documented)).toBe(1);
      expect(JSON.parse(second.join("\n")).findings.map((finding: { rule: string; path: string }) => `${finding.rule} ${finding.path}`)).toEqual(["raw-spawn src/b/🟦️.ts"]);
      const strict = { ...documented, exceptions: [...documented.exceptions, { path: "src/gone/🟦️.ts", program: "ps", count: 1, class: "os-primitive" as const, reason: { en: "x", de: "y" } }] };
      write("src/b/🟦️.ts", "export {};\n");
      gitSyncV1(["add", "-A"], { cwd: root });
      expect(runProcessSpawnLawV1(root, [], () => {}, strict)).toBe(0);
      expect(runProcessSpawnLawV1(root, ["--strict"], () => {}, strict)).toBe(1);
    });
    test("runs behind repo verify tool-spawns as one document with both laws", () => {
      const lines: string[] = [];
      const code = runToolSpawnLawsV1(root, ["--json"], line => lines.push(line));
      const document = JSON.parse(lines.join("\n"));
      expect(typeof code).toBe("number");
      expect(Object.keys(document)).toEqual(expect.arrayContaining(["scanned", "offenders", "findings", "rawProcessSpawn"]));
      expect(document.rawProcessSpawn.sites).toBeGreaterThanOrEqual(0);
    });
  });

  test("keeps the committed allowlist consistent with the registry: interfaces include every adapter scope and every tool command is routed", () => {
    const globs = processInterfaceGlobsV1(TOOL_REGISTRY_V1, PROCESS_ALLOWLIST_V1);
    for (const tool of TOOL_REGISTRY_V1.tools) { expect(globs).toContain(tool.adapter); for (const scope of tool.adapterScope) expect(globs).toContain(scope); }
    expect(processCommandNameV1("C:\\Windows\\System32\\tar.EXE")).toBe("tar");
    expect(processCommandNameV1("/usr/bin/hdiutil")).toBe("hdiutil");
    for (const command of ["tar", "ffmpeg", "wasm-bindgen"]) expect(TOOL_REGISTRY_V1.tools.some(tool => tool.commands.includes(command))).toBe(true);
  });
});
