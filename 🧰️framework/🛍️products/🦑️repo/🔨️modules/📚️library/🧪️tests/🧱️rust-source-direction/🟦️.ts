import { inspectRustCompileReferences, type RustCompileReference } from "../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
import { expect, test } from "bun:test";
import { writeRustLayerOracle } from "./🔮️oracle/🟦️.ts";
import { lstatSync, mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
import Ajv from "ajv/dist/2020.js";
import { parse as parseToml } from "@iarna/toml";
import glob from "fast-glob";
import { normalize as oracleNormalize, join as oracleJoin, dirname as oracleDirname, isAbsolute as oracleAbsolute } from "pathe";
import { rustSourceDirectionEdges, rustSourceTargets, rustSourceTargetProblem, type RustSourceOwnership, type RustSourceDirectionEdge, type RustSourceInputNode, type RustSourceInputProblem } from "../../🕸️dependencies/🧭️direction/🦀️source/🟦️.ts";
import type { DependencyDirectionRule } from "../../🕸️dependencies/🧭️direction/🟦️.ts";
import { inspectRustModuleGraph, inspectRustModuleGraphFacts, rustModuleScopeProof } from "../../🔍️discovery/🟦️.ts";
import { inspectRustSourceDirection, inspectRustSourceInputs, type RustSourceInputFailure } from "../../🕸️dependencies/🧭️direction/🦀️source/🏃️execution/🟦️.ts";

const library = resolve(import.meta.dir, "../.."), read = (path: string): any => JSON.parse(readFileSync(join(library, path), "utf8"));
const fixture = read("🧫️fixtures/🧱️rust-source-direction/🔣️.json") as { cases: readonly { id: string; source: string; references: readonly RustCompileReference[] }[]; rules: readonly DependencyDirectionRule[]; directions: readonly { id: string; from: string; source: string; edges: readonly RustSourceDirectionEdge[] }[]; mounts: readonly { id: string; files: Readonly<Record<string, string>>; sourcePath: string; targets: readonly string[] }[]; unsupported: readonly { id: string; source: string }[] };
const targets = read("🧫️fixtures/🧱️rust-source-direction/🔣️.json").targets as readonly { id: string; target: string; directory?: true; inventory: readonly RustSourceInputNode[]; sources: readonly string[]; problem: RustSourceInputProblem | null }[];
function participationProjection(graph: ReturnType<typeof inspectRustModuleGraph>) {
  return graph.participations.map((row) => !("context" in row) ? { target: row.target, state: row.state, reason: row.reason } : { target: row.target, crateRoot: row.context.crateRoot, manifestPath: row.context.manifestPath, modulePath: row.context.modulePath, sourceScope: row.context.sourceScope, mount: row.context.mount.kind, state: row.state, reason: row.state === "denied" ? row.reason : null }).sort((a, b) => Buffer.from(JSON.stringify(a)).compare(Buffer.from(JSON.stringify(b))));
}
type ParticipationProjection = ReturnType<typeof participationProjection>[number];

let activeCompilers = 0;
const compilerWaiters: (() => void)[] = [];

/** 🚦️ Runs each independent native oracle through one process-wide four-compiler permit pool. */
async function nativeCommand(args: readonly string[], cwd: string, env?: Record<string, string | undefined>): Promise<{ stdout: string; stderr: string; status: number }> {
  const compiler = args[0] === "rustc";
  if (compiler) {
    if (activeCompilers === 4) await new Promise<void>((accept) => compilerWaiters.push(accept));
    else activeCompilers++;
  }
  try {
    const child = Bun.spawn([...args], { cwd, ...(env ? { env } : {}), stdout: "pipe", stderr: "pipe" });
    const [stdout, stderr, status] = await Promise.allSettled([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
    if (stdout.status === "rejected") throw stdout.reason;
    if (stderr.status === "rejected") throw stderr.reason;
    if (status.status === "rejected") throw status.reason;
    return { stdout: stdout.value, stderr: stderr.value, status: status.value };
  } finally {
    if (compiler) {
      const next = compilerWaiters.shift();
      if (next) next();
      else activeCompilers--;
    }
  }
}

test("every present native owner follows the actual layer policy and deleted owners disappear", async () => {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw new Error("Rust layer oracle requires a ticket-owned test artifact directory");
  mkdirSync(output, { recursive: true });
  const root = realpathSync(mkdtempSync(join(output, "rust-layer-owners-")));
  const corpus = read("🧫️fixtures/🧱️rust-source-direction/🔣️.json");
  const cases = corpus.ownerCases as readonly { id: string; files: Readonly<Record<string, string>>; source: string; sources: readonly string[]; violations: readonly RustSourceDirectionEdge[]; remove?: readonly string[] }[];
  const schema = read("🧬️schema/🧱️rust-source-direction/🔣️.json");
  expect(read("🧫️fixtures/🧱️rust-source-direction/🔣️.json")["schemaVersion"]).toEqual(1);expect(read("🧫️fixtures/🧱️rust-source-direction/🔣️.json")["attributeOrigins"]["provider"]).toEqual("extern crate proc_macro;\nuse proc_macro::TokenStream;\n#[proc_macro_attribute]\npub fn test(_: TokenStream, _: TokenStream) -> TokenStream { TokenStream::new() }\n#[proc_macro_attribute]\npub fn rewrite(_: TokenStream, _: TokenStream) -> TokenStream { TokenStream::new() }\n#[proc_macro_attribute]\npub fn doc(_: TokenStream, _: TokenStream) -> TokenStream { TokenStream::new() }\n");
  expect(new Set(cases.map((row) => row.id)).size).toBe(cases.length);
  const validateReport = new Ajv({ strict: true }).compile({ $schema: schema.$schema, $defs: schema.$defs, $ref: "#/$defs/report" });
  try {
    let next = 0;
    const workers = await Promise.allSettled(Array.from({ length: 4 }, async () => {
      while (next < cases.length) {
        const row = cases[next++]!, cwd = join(root, row.id);
        writeRustLayerOracle(cwd, row.files);
        const members = corpus.contribution.members as string[];
        expect(glob.sync(members, { cwd, onlyDirectories: true, followSymbolicLinks: false })).toEqual(members);
        for (const path of row.remove ?? []) rmSync(join(cwd, path), { recursive: true });
        if (row.remove?.length) expect(glob.sync(members, { cwd, onlyDirectories: true, followSymbolicLinks: false })).toEqual([]);
        const { stdout, stderr, status } = await nativeCommand(["rustc", "--crate-name", "layer_oracle", "--crate-type", "lib", "--emit=dep-info=dependencies.d", row.source], cwd, { ...process.env, CARGO_MANIFEST_DIR: cwd });
        expect(status, `${row.id}: ${stdout}${stderr}`).toBe(0);
        const nativeInputs = dependencies(cwd).map(oracleNormalize);
        expect(nativeInputs).toContain(row.source);
        expect(row.violations.every((edge) => nativeInputs.includes(edge.to)), row.id).toBe(true);
        expect(Object.keys(row.files).filter((path) => path.endsWith(".rs")).sort(), row.id).toEqual([...row.sources].sort());
        const report = await inspectRustSourceDirection(cwd);
        expect(validateReport(report), JSON.stringify(validateReport.errors)).toBe(true);
        expect(report.files, row.id).toBe(row.sources.length);
        expect(report.violations, row.id).toEqual(row.violations);
        expect(report.problems, row.id).toEqual([]);
      }
    }));
    for (const worker of workers) if (worker.status === "rejected") throw worker.reason;
  } finally { rmSync(root, { recursive: true, force: true }); }
}, 45_000);

for (const row of read("🧫️fixtures/🧱️rust-source-direction/🔣️.json").ownerRejections as readonly { id: string; files: Readonly<Record<string, string>>; link?: { path: string; target: string }; error?: string; problem?: { code: "unresolved-target"; detail: string } }[]) test(row.id, async () => {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw new Error("Rust owner rejection requires caller-owned output");
  mkdirSync(output, { recursive: true });
  const root = realpathSync(mkdtempSync(join(output, "rust-layer-rejection-")));
  try {
    writeRustLayerOracle(root, row.files);
    if (row.link) {
      symlinkSync(join(root, row.link.target), join(root, row.link.path), "file");
      expect(lstatSync(join(root, row.link.path)).isSymbolicLink()).toBe(true);
    }
    if (row.error) await expect(inspectRustSourceDirection(root)).rejects.toThrow(row.error);
    else {
      const report = await inspectRustSourceDirection(root);
      expect(report.problems).toHaveLength(1);
      expect(report.problems[0]!.code).toBe(row.problem!.code);
      expect(report.problems[0]!.detail).toContain(row.problem!.detail);
    }
  } finally { rmSync(root, { recursive: true, force: true }); }
}, 45_000);

test("physical input census refuses linked, missing, unscanned and mistyped authored targets", () => {
  for (const row of targets) expect(rustSourceTargetProblem(row.target, row.directory === true, new Map(row.inventory.map((node) => [node.path, node.kind])), new Set(row.sources)), row.id).toBe(row.problem);
});

test("portable native path forms cannot escape the authored workspace", () => {
  for (const row of read("🧫️fixtures/🧱️rust-source-direction/🔣️.json").escapes as readonly { id: string; from: string; source: string }[]) {
    const refs = inspectRustCompileReferences(row.source), path = refs[0]!.path;
    expect(oracleAbsolute(path) || oracleNormalize(oracleJoin(oracleDirname(row.from), path)).startsWith("../"), row.id).toBe(true);
    expect(() => rustSourceDirectionEdges(row.from, refs, fixture.rules), row.id).toThrow("escapes");
  }
});

test("live input census closes linked data reads confirmed independently by rustc", async () => {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw new Error("Rust source oracle requires a ticket-owned test artifact directory");
  mkdirSync(output, { recursive: true });
  const root = realpathSync(mkdtempSync(join(output, "rust-source-links-")));
  try {
    mkdirSync(join(root, "general")); mkdirSync(join(root, "specific"));
    writeFileSync(join(root, "specific/fixture.json"), "specific-owner");
    writeFileSync(join(root, "general/leaf.rs"), "const LEAF: usize = 1;");
    symlinkSync(join(root, "specific"), join(root, "general/mount"), process.platform === "win32" ? "junction" : "dir");
    const source = 'const TEXT: &str = include_str!("general/mount/fixture.json");';
    writeFileSync(join(root, "source.rs"), source);
    const { stdout, stderr, status } = await nativeCommand(["rustc", "--crate-name", "boundary_oracle", "--crate-type", "lib", "--emit=dep-info=dependencies.d", "source.rs"], root);
    expect(status, stdout + stderr).toBe(0);
    expect(dependencies(root)).toContain("general/mount/fixture.json");
    const reference = { kind: "include_str", path: "general/mount/fixture.json", line: 1 } as const;
    expect(await inspectRustSourceInputs(root, [{ to: reference.path, reference, directories: [] }], new Set())).toEqual([{ code: "linked-input", to: reference.path, kind: "include_str", line: 1 }]);
    const refs = [{ to: "general/leaf.rs", reference: { kind: "include", path: "general/leaf.rs", line: 1 }, directories: [] }, { to: "general/missing.rs", reference: { kind: "path", path: "general/missing.rs", line: 2 }, directories: [] }] as const;
    expect(await inspectRustSourceInputs(root, refs, new Set())).toEqual([{ code: "uncensused-source", to: "general/leaf.rs", kind: "include", line: 1 }, { code: "missing-input", to: "general/missing.rs", kind: "path", line: 2 }]);
    expect(await inspectRustSourceInputs(root, [refs[0]], new Set(["general/leaf.rs"]))).toEqual([]);
    const schema = read("🧬️schema/🧱️rust-source-direction/🔣️.json"), validate = new Ajv({ strict: true }).compile({ $schema: schema.$schema, $defs: schema.$defs, $ref: "#/$defs/report" });
    const report = { schemaVersion: 1, files: 1, references: 1, violations: [], problems: [{ code: "linked-input", from: "source.rs", to: reference.path, kind: reference.kind, line: 1, detail: "Authored input traverses a linked product owner" }] };
    expect(validate(report), JSON.stringify(validate.errors)).toBe(true);
    expect(validate({ ...report, problems: [{ ...report.problems[0], code: "allowed-linked-input" }] })).toBe(false);
    expect(validate({ ...report, waiver: true })).toBe(false);
  } finally { rmSync(root, { recursive: true, force: true }); }
}, 45_000);

const dependencies = (cwd: string): readonly string[] => {
  const line = readFileSync(join(cwd, "dependencies.d"), "utf8").split("\n")[0]!, marker = line.indexOf(": ");
  if (marker < 0) throw new Error("rustc did not emit dependency provenance");
  return (line.slice(marker + 2).match(/(?:\\[ #\\]|[^\s])+/gu) ?? []).map((value) => {
    const path = value.replace(/\\([ #\\])/gu, "$1");
    return (isAbsolute(path) ? relative(cwd, path) : path).replaceAll("\\", "/");
  });
};

test("portable compile-time paths preserve exact literal identity and ignore lexical decoys", () => {
  const validate = new Ajv({ strict: true }).compile(read("🧬️schema/🧱️rust-source-direction/🔣️.json"));
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  for (const row of fixture.cases) expect(inspectRustCompileReferences(row.source), row.id).toEqual(row.references);
});

test("inline reference metadata preserves its closed portable scope contract", () => {
  const corpus = read("🧫️fixtures/🧱️rust-source-direction/🔣️.json"), validate = new Ajv({ strict: true }).compile(read("🧬️schema/🧱️rust-source-direction/🔣️.json"));
  const rows = corpus.referenceRejections as readonly { id: string; reference: RustCompileReference; invalidSchema: boolean; ownership?: RustSourceOwnership }[];
  expect(new Set(rows.map((row) => row.id)).size).toBe(rows.length);
  for (const row of rows) {
    expect(validate({ ...corpus, cases: [{ id: row.id, source: "source", references: [row.reference] }] }), row.id).toBe(!row.invalidSchema);
    expect(() => rustSourceTargets("general/source.rs", [row.reference], row.ownership), row.id).toThrow("inline base");
  }
});

test("explicit inline anchors prove physical bases without granting orphan Cargo authority", async () => {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw new Error("Rust inline oracle requires ticket-owned output");
  mkdirSync(output, { recursive: true });
  const root = realpathSync(mkdtempSync(join(output, "rust-inline-bases-"))), corpus = read("🧫️fixtures/🧱️rust-source-direction/🔣️.json");
  const rows = corpus.inlineScopes as readonly { id: string; root: string; source: string; files: Readonly<Record<string, string>>; targets: readonly string[] }[];
  const rules: DependencyDirectionRule[] = [{ name: "every-input", severity: "error", from: { path: [".*"] }, to: { path: [".*"] } }];
  expect(new Set(rows.map((row) => row.id)).size).toBe(rows.length);
  try {
    for (const row of rows) {
      const cwd = join(root, row.id);
      for (const [path, source] of Object.entries(row.files)) { mkdirSync(dirname(join(cwd, path)), { recursive: true }); writeFileSync(join(cwd, path), source); }
      const { stdout, stderr, status } = await nativeCommand(["rustc", "--crate-name", "inline_oracle", "--crate-type", "lib", "--emit=dep-info=dependencies.d", row.root], cwd);
      expect(status, `${row.id}: ${stdout}${stderr}`).toBe(0);
      const nativeInputs = dependencies(cwd).map(oracleNormalize), refs = inspectRustCompileReferences(row.files[row.source]!);
      const targets = rustSourceDirectionEdges(row.source, refs, rules).map((edge) => edge.to);
      expect([...new Set(targets)].sort(), row.id).toEqual([...row.targets].sort());
      expect(row.targets.filter((path) => path.endsWith(".rs") || path.endsWith(".txt")).every((path) => nativeInputs.includes(path)), row.id).toBe(true);
      expect(await inspectRustSourceInputs(cwd, rustSourceTargets(row.source, refs), new Set(Object.keys(row.files)))).toEqual([]);
      expect(() => rustSourceTargets(row.source, inspectRustCompileReferences('include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/fixture.txt"));'))).toThrow("manifest provenance");
    }
    for (const row of corpus.inlineRejections as readonly { id: string; from: string; source: string; error: string }[]) expect(() => rustSourceTargets(row.from, inspectRustCompileReferences(row.source)), row.id).toThrow(row.error);
  } finally { rmSync(root, { recursive: true, force: true }); }
}, 45_000);

test("authored traversal visits physical prefixes before resolving parent navigation", async () => {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw new Error("Rust traversal oracle requires ticket-owned output");
  mkdirSync(output, { recursive: true });
  const root = realpathSync(mkdtempSync(join(output, "rust-prefix-traversal-")));
  const rows = read("🧫️fixtures/🧱️rust-source-direction/🔣️.json").traversals as readonly { id: string; root: string; files: Readonly<Record<string, string>>; link?: { path: string; target: string }; native: boolean; stdout?: string; failures: readonly RustSourceInputFailure[] }[];
  expect(new Set(rows.map((row) => row.id)).size).toBe(rows.length);
  try {
    for (const row of rows) {
      const cwd = join(root, row.id), executable = process.platform === "win32" ? "oracle.exe" : "oracle";
      for (const [path, source] of Object.entries(row.files)) { mkdirSync(dirname(join(cwd, path)), { recursive: true }); writeFileSync(join(cwd, path), source); }
      if (row.link) symlinkSync(join(cwd, row.link.target), join(cwd, row.link.path), process.platform === "win32" ? "junction" : "dir");
      const { stdout, stderr, status } = await nativeCommand(["rustc", "--crate-name", "traversal_oracle", row.root, "-o", executable], cwd);
      expect(status === 0, `${row.id}: ${stdout}${stderr}`).toBe(row.native);
      if (row.native) {
        const runtime = Bun.spawn([join(cwd, executable)], { cwd, stdout: "pipe", stderr: "pipe" });
        const [actual, errors, code] = await Promise.all([new Response(runtime.stdout).text(), new Response(runtime.stderr).text(), runtime.exited]);
        expect(code, errors).toBe(0);
        expect(actual.replaceAll("\r\n", "\n"), row.id).toBe(row.stdout!);
      }
      const targets = rustSourceTargets(row.root, inspectRustCompileReferences(row.files[row.root]!));
      const failures = await inspectRustSourceInputs(cwd, targets, new Set(Object.keys(row.files)));
      expect([...new Map(failures.map((failure) => [JSON.stringify(failure), failure])).values()], row.id).toEqual([...row.failures]);
    }
  } finally { rmSync(root, { recursive: true, force: true }); }
}, 45_000);

test("rustc independently confirms literal inputs, nested module mounts and environment inputs", async () => {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw new Error("Rust source oracle requires a ticket-owned test artifact directory");
  mkdirSync(output, { recursive: true });
  const root = realpathSync(mkdtempSync(join(output, "rust-source-direction-")));
  try {
    let next = 0;
    const workers = await Promise.allSettled(Array.from({ length: 4 }, async () => {
      while (next < fixture.cases.length) {
        const row = fixture.cases[next++]!, cwd = join(root, row.id);
        mkdirSync(cwd);
        writeFileSync(join(cwd, "source.rs"), row.source);
        writeFileSync(join(cwd, "fixture.txt"), "neutral");
        writeFileSync(join(cwd, "leaf.rs"), "const LEAF: usize = 1;\n");
        writeFileSync(join(cwd, "module.rs"), "pub const MODULE: usize = 2;\n");
        const { stdout, stderr, status } = await nativeCommand(["rustc", "--crate-name", "boundary_oracle", "--crate-type", "lib", "--test", "--emit=dep-info=dependencies.d", "source.rs"], cwd, { ...process.env, CARGO_MANIFEST_DIR: cwd, OUT_DIR: cwd });
        expect(status, `${row.id}: ${stdout}${stderr}`).toBe(0);
        const actual = dependencies(cwd).filter((path) => path !== "source.rs").sort();
        const expected = row.id === "unused-macro-authored-input" ? [] : [...new Set(row.references.filter((ref) => !ref.directory).map((ref) => ref.base ? ref.path.slice(1) : ref.path))].sort();
        expect(actual, row.id).toEqual(expected);
      }
    }));
    for (const worker of workers) if (worker.status === "rejected") throw worker.reason;
    for (const row of fixture.mounts) {
      const cwd = join(root, row.id);
      for (const [path, source] of Object.entries(row.files)) { mkdirSync(join(cwd, oracleDirname(path)), { recursive: true }); writeFileSync(join(cwd, path), source); }
      const { stdout, stderr, status } = await nativeCommand(["rustc", "--crate-name", "boundary_oracle", "--crate-type", "lib", "--test", "--emit=dep-info=dependencies.d", "source.rs"], cwd, { ...process.env, CARGO_MANIFEST_DIR: cwd });
      expect(status, `${row.id}: ${stdout}${stderr}`).toBe(0);
      const actual = dependencies(cwd);
      const compileReferences = new Map(Object.entries(row.files).filter(([path]) => path.endsWith(".rs")).map(([path, source]) => [path, inspectRustCompileReferences(source)]));
      const graph = inspectRustModuleGraph(Object.keys(row.files), (path) => row.files[path], { compileReferences });
      const refs = inspectRustCompileReferences(row.files[row.sourcePath]!);
      const rules: DependencyDirectionRule[] = [{ name: "all-inputs", severity: "error", from: { path: [".*"] }, to: { path: [".*"] } }];
      const edges = rustSourceDirectionEdges(row.sourcePath, refs, rules, { contexts: graph.contexts.get(row.sourcePath) });
      const targets = edges.filter((edge) => row.targets.includes(edge.to)).map((edge) => edge.to);
      expect([...new Set(targets)].sort(), row.id).toEqual([...row.targets].sort());
      expect(targets.every((target) => actual.includes(target)), row.id).toBe(true);
      expect([...graph.targets.values()].filter((target) => row.targets.includes(target)).sort(), row.id).toEqual(row.targets.filter((target) => target.endsWith(".rs")).sort());
    }
  } finally { rmSync(root, { recursive: true, force: true }); }
}, 45_000);

test("finite local macro inputs retain native expansion provenance and runtime bytes", async () => {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw new Error("Rust macro oracle requires caller-owned output");
  mkdirSync(output, { recursive: true });
  const root = realpathSync(mkdtempSync(join(output, "rust-macro-inputs-"))), corpus = read("🧫️fixtures/🧱️rust-source-direction/🔣️.json");
  const rows = corpus.macroScopes as readonly { id: string; root: string; files: Readonly<Record<string, string>>; references: readonly RustCompileReference[]; stdout: string; nativeTests?: readonly string[] }[];
  const schema = read("🧬️schema/🧱️rust-source-direction/🔣️.json");
  expect(corpus["schemaVersion"]).toEqual(1);expect(corpus["attributeOrigins"]["provider"]).toEqual("extern crate proc_macro;\nuse proc_macro::TokenStream;\n#[proc_macro_attribute]\npub fn test(_: TokenStream, _: TokenStream) -> TokenStream { TokenStream::new() }\n#[proc_macro_attribute]\npub fn rewrite(_: TokenStream, _: TokenStream) -> TokenStream { TokenStream::new() }\n#[proc_macro_attribute]\npub fn doc(_: TokenStream, _: TokenStream) -> TokenStream { TokenStream::new() }\n");
  expect(new Set(rows.map((row) => row.id)).size).toBe(rows.length);
  try {
    const modules = rows.map((row, index) => "#[path=" + JSON.stringify(row.id + "/" + row.root) + "] mod case_" + index + ";").join("\n");
    const calls = rows.map((row, index) => 'println!("[DEBUG] macro-case:' + row.id + '"); case_' + index + "::main();").join("\n");
    for (const row of rows) for (const [path, source] of Object.entries(row.files)) {
      mkdirSync(dirname(join(root, row.id, path)), { recursive: true }); writeFileSync(join(root, row.id, path), source);
    }
    writeFileSync(join(root, "macro-host.rs"), modules + "\nfn main() {\n" + calls + "\n}\n");
    const binary = join(root, process.platform === "win32" ? "oracle.exe" : "oracle");
    const build = await nativeCommand(["rustc", "--crate-name", "macro_oracle", "--emit=dep-info=dependencies.d,link", "-o", binary, "macro-host.rs"], root);
    expect(build.status, build.stdout + build.stderr).toBe(0);
    const runtime = await nativeCommand([binary], root);
    expect(runtime.status, runtime.stderr).toBe(0);
    expect(runtime.stdout.replaceAll("\r\n", "\n")).toBe(rows.map((row) => "[DEBUG] macro-case:" + row.id + "\n" + row.stdout).join(""));
    const testBinary = join(root, process.platform === "win32" ? "laws.exe" : "laws");
    const testBuild = await nativeCommand(["rustc", "--crate-name", "macro_laws", "--test", "--emit=dep-info=dependencies.d,link", "-o", testBinary, "macro-host.rs"], root);
    expect(testBuild.status, testBuild.stdout + testBuild.stderr).toBe(0);
    const nativeTests = rows.flatMap((row, index) => (row.nativeTests ?? []).map((name) => "case_" + index + "::" + name));
    expect(nativeTests).toHaveLength(1);
    const lawRun = await nativeCommand([testBinary], root);
    expect(lawRun.status, lawRun.stderr).toBe(0);
    expect(lawRun.stdout).toContain(nativeTests.length + " passed; 0 failed; 0 ignored");
    for (const name of nativeTests) expect(lawRun.stdout).toContain("test " + name + " ... ok");
    const nativeInputs = dependencies(root).map(oracleNormalize), expectedInputs: string[] = ["macro-host.rs"];
    for (const row of rows) {
      const references = inspectRustCompileReferences(row.files[row.root]!);
      expect(references, row.id).toEqual(row.references);
      const resolved = rustSourceTargets(row.root, references), expected = references.map((ref) => oracleNormalize(oracleJoin(oracleDirname(row.root), ref.path)));
      expect(resolved.map((target) => target.to), row.id).toEqual(expected);
      expectedInputs.push(row.id + "/" + row.root, ...expected.map((path) => row.id + "/" + path));
      expect(expected.every((path) => nativeInputs.includes(row.id + "/" + path)), row.id).toBe(true);
      expect(await inspectRustSourceInputs(join(root, row.id), resolved, new Set(Object.keys(row.files))), row.id).toEqual([]);
      expect(rustSourceDirectionEdges(row.root, references, fixture.rules), row.id).toEqual(expected.map((to, index) => ({ rule: "framework-no-implementation", from: row.root, to, kind: references[index]!.kind, line: references[index]!.line, expansion: references[index]!.expansion })));
    }
    expect(nativeInputs.sort()).toEqual([...new Set(expectedInputs)].sort());
    const rejections = corpus.macroRejections as readonly { id: string; source: string; native: boolean; files?: Readonly<Record<string, string>> }[];
    expect(new Set(rejections.map((row) => row.id)).size).toBe(rejections.length);
    let next = 0;
    const workers = await Promise.allSettled(Array.from({ length: 4 }, async () => {
      while (next < rejections.length) {
        const row = rejections[next++]!, cwd = join(root, row.id);
        mkdirSync(join(cwd, "general"), { recursive: true }); mkdirSync(join(cwd, "specific"));
        writeFileSync(join(cwd, "general/source.rs"), row.source); writeFileSync(join(cwd, "specific/input.txt"), "specific-input"); writeFileSync(join(cwd, "specific/payload.bin"), "probe");
        for (const [path, source] of Object.entries(row.files ?? {})) { mkdirSync(dirname(join(cwd, path)), { recursive: true }); writeFileSync(join(cwd, path), source); }
        const native = await nativeCommand(["rustc", "--crate-name", "macro_refusal", "--emit=dep-info=dependencies.d", "general/source.rs"], cwd);
        expect(native.status === 0, row.id + ": " + native.stdout + native.stderr).toBe(row.native);
        expect(() => inspectRustCompileReferences(row.source), row.id).toThrow("Unsupported Rust compile");
      }
    }));
    for (const worker of workers) if (worker.status === "rejected") throw worker.reason;
  } finally { rmSync(root, { recursive: true, force: true }); }
}, 45_000);



test("compiler marker origins require captured aliases, wildcard exports and included producers", async () => {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw new Error("Rust attribute origins require caller-owned output");
  mkdirSync(output, { recursive: true });
  const root = realpathSync(mkdtempSync(join(output, "rust-attribute-origins-"))), corpus = read("🧫️fixtures/🧱️rust-source-direction/🔣️.json");
  const validate = new Ajv({ strict: true }).compile(read("🧬️schema/🧱️rust-source-direction/🔣️.json"));
  expect(validate(corpus), JSON.stringify(validate.errors)).toBe(true);
  const rows = corpus.attributeOrigins.cases as readonly { id: string; source: string; files: Readonly<Record<string, string>>; problem: "unresolved-template-scope" | null }[];
  expect(new Set(rows.map(row => row.id)).size).toBe(10);
  try {
    for (const row of rows) {
      const cwd = join(root, row.id);
      writeLayerOracle(cwd, row.files);
      const refs = inspectRustCompileReferences(row.files[row.source]!);
      expect(refs.filter(ref => ref.expansion)).toHaveLength(2);
      const report = await inspectRustSourceDirection(cwd);
      expect(report.violations, row.id).toEqual([]);
      expect(report.problems.map(problem => ({ from: problem.from, code: problem.code })), row.id).toEqual(row.problem ? [{ from: row.source, code: row.problem }] : []);
    }
  } finally { rmSync(root, { recursive: true, force: true }); }
}, 45_000);

test("independent compiler attributes preserve or erase the exact authored fixture inputs", async () => {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw new Error("Rust attribute reference requires caller-owned output");
  mkdirSync(output, { recursive: true });
  const root = realpathSync(mkdtempSync(join(output, "rust-attribute-reference-"))), corpus = read("🧫️fixtures/🧱️rust-source-direction/🔣️.json");
  const rows = corpus.attributeOrigins.cases as readonly { id: string; root: string; source: string; files: Readonly<Record<string, string>>; native: { build: boolean; tests: number | null; input: boolean | null; error?: string } }[];
  const provider = join(root, process.platform === "win32" ? "attribute_provider.dll" : process.platform === "darwin" ? "libattribute_provider.dylib" : "libattribute_provider.so");
  try {
    writeFileSync(join(root, "provider.rs"), corpus.attributeOrigins.provider);
    const build = await nativeCommand(["rustc", "--edition=2021", "--crate-name", "attribute_provider", "--crate-type", "proc-macro", "provider.rs", "-o", provider], root);
    expect(build.status, build.stdout + build.stderr).toBe(0);
    for (const row of rows) {
      const cwd = join(root, row.id);
      writeLayerOracle(cwd, row.files);
      const binary = join(cwd, process.platform === "win32" ? "laws.exe" : "laws");
      const build = await nativeCommand(["rustc", "--edition=2021", "--crate-name", "attribute_case", "--test", "--extern", "attribute_provider=" + provider, "--emit=dep-info=dependencies.d,link", "-o", binary, row.root], cwd);
      expect(build.status === 0, row.id + ": " + build.stdout + build.stderr).toBe(row.native.build);
      if (!row.native.build) { expect(build.stderr, row.id).toContain(row.native.error!); continue; }
      const runtime = await nativeCommand([binary, "--nocapture"], cwd);
      expect(runtime.status, runtime.stderr).toBe(0);
      expect(runtime.stdout, row.id).toContain(row.native.tests + " passed; 0 failed; 0 ignored");
      expect(dependencies(cwd).some(path => oracleNormalize(path).endsWith("/input.txt")), row.id).toBe(row.native.input!);
    }
  } finally { rmSync(root, { recursive: true, force: true }); }
}, 45_000);

test("finite macro scopes require every live manifest and incoming module origin", async () => {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw new Error("Rust macro ownership oracle requires caller-owned output");
  mkdirSync(output, { recursive: true });
  const root = realpathSync(mkdtempSync(join(output, "rust-macro-owners-"))), corpus = read("🧫️fixtures/🧱️rust-source-direction/🔣️.json");
  const rows = corpus.macroOwners as readonly { id: string; root: string; source: string; files: Readonly<Record<string, string>>; native: boolean; nativeHarness?: true; scope: unknown; mounts: readonly ("root" | "module" | "include")[]; problem?: "unresolved-template-scope" | "unsupported-expression"; remove?: readonly string[]; replace?: Readonly<Record<string, string>>; contextProof?: Readonly<{ sourceScope: readonly string[]; mounted: boolean }> }[];
  const schema = read("🧬️schema/🧱️rust-source-direction/🔣️.json");
  const reportValid = new Ajv({ strict: true }).compile({ $schema: schema.$schema, $defs: schema.$defs, $ref: "#/$defs/report" });
  const scopeValid = new Ajv({ strict: true }).compile({ $schema: schema.$schema, $defs: schema.$defs, $ref: "#/$defs/moduleScopeFact" });
  const proofValid = new Ajv({ strict: true }).compile({ $schema: schema.$schema, $defs: schema.$defs, $ref: "#/$defs/moduleScopeProof" });
  expect(corpus["schemaVersion"]).toEqual(1);expect(corpus["attributeOrigins"]["provider"]).toEqual("extern crate proc_macro;\nuse proc_macro::TokenStream;\n#[proc_macro_attribute]\npub fn test(_: TokenStream, _: TokenStream) -> TokenStream { TokenStream::new() }\n#[proc_macro_attribute]\npub fn rewrite(_: TokenStream, _: TokenStream) -> TokenStream { TokenStream::new() }\n#[proc_macro_attribute]\npub fn doc(_: TokenStream, _: TokenStream) -> TokenStream { TokenStream::new() }\n");
  expect(new Set(rows.map((row) => row.id)).size).toBe(rows.length);
  try {
    let next = 0;
    const failures: unknown[] = [];
    const workers = await Promise.allSettled(Array.from({ length: 4 }, async () => {
      while (next < rows.length) {
        const row = rows[next++]!, cwd = join(root, row.id);
        try {
        writeLayerOracle(cwd, row.files);
        for (const [path, source] of Object.entries(row.replace ?? {})) {
          expect(readFileSync(join(cwd, path), "utf8"), row.id).toContain('mod sealed;');
          writeFileSync(join(cwd, path), source);
          expect(readFileSync(join(cwd, path), "utf8"), row.id).toBe(source);
        }
        for (const path of row.remove ?? []) { rmSync(join(cwd, path)); expect(glob.sync(path, { cwd, followSymbolicLinks: false }), row.id).toEqual([]); }
        const binary = join(cwd, process.platform === "win32" ? "oracle.exe" : "oracle");
        const { stdout, stderr, status } = await nativeCommand(["rustc", "--edition=2021", "--crate-name", "scope_oracle", ...(row.nativeHarness ? ["--test"] : []), "--emit=dep-info=dependencies.d,link", "-o", binary, row.root], cwd);
        expect(status === 0, row.id + ": " + stdout + stderr).toBe(row.native);
        const runtime = Bun.spawn([binary, ...(row.nativeHarness ? ["--nocapture"] : [])], { cwd, stdout: "pipe", stderr: "pipe" });
        const [runtimeOut, runtimeErr, runtimeStatus] = await Promise.all([new Response(runtime.stdout).text(), new Response(runtime.stderr).text(), runtime.exited]);
        expect(runtimeStatus, runtimeErr).toBe(0);
        if (row.nativeHarness) expect(runtimeOut, row.id).toContain("1 passed; 0 failed; 0 ignored");
        if (row.id !== "physically-delete-parent-mount") expect(runtimeOut).toContain("sealed-input sealed-input");
        const live = Object.keys(row.files).filter((path) => !row.remove?.includes(path));
        const sources = new Map(live.map((path) => [path, readFileSync(join(cwd, path), "utf8")]));
        const refs = new Map(live.filter((path) => path.endsWith(".rs")).map((path) => { try { return [path, inspectRustCompileReferences(sources.get(path)!)] as const; } catch { return [path, []] as const; } }));
        const graph = inspectRustModuleGraph(live, (path) => sources.get(path), { strictManifests: true, compileReferences: refs });
        const facts = inspectRustModuleGraphFacts(sources.get(row.source)!);
        expect(facts.scopes.length, row.id).toBeGreaterThan(0);
        for (const fact of facts.scopes) expect(scopeValid(fact), JSON.stringify(scopeValid.errors)).toBe(true);
        const proof = rustModuleScopeProof(facts, row.contextProof?.sourceScope ?? []);
        expect(proofValid(proof), JSON.stringify(proofValid.errors)).toBe(true);
        if (row.id.startsWith("inner-")) expect(proof.state, row.id).toBe("unresolved");
        if (row.id === "function-local-in-inline-module") expect(proof.state, row.id).toBe("resolved");
        expect([...new Set((graph.contexts.get(row.source) ?? []).filter((context) => context.sourceScope.length === 0).map((context) => context.mount.kind))].sort(), row.id).toEqual([...row.mounts].sort());
        if (row.contextProof) expect((graph.contexts.get(row.source) ?? []).some((context) => context.sourceScope.join("::") === row.contextProof!.sourceScope.join("::")), row.id).toBe(row.contextProof.mounted);
        if (row.problem !== "unsupported-expression") {
          const references = refs.get(row.source)!;
          expect(references, row.id).toHaveLength(2);
          expect(new Set(references.map((ref) => ref.expansion!.templateOffset)).size, row.id).toBe(2);
          expect(references.every((ref) => JSON.stringify(ref.expansion?.scope) === JSON.stringify(row.scope)), row.id).toBe(true);
          if (row.id !== "physically-delete-parent-mount") expect(dependencies(cwd).map(oracleNormalize), row.id).toContain(row.source);
        }
        const report = await inspectRustSourceDirection(cwd);
        expect(reportValid(report), JSON.stringify(reportValid.errors)).toBe(true);
        expect(report.violations, row.id).toEqual([]);
        expect(report.problems.map((problem) => ({ from: problem.from, code: problem.code })), row.id).toEqual(row.problem ? [{ from: row.source, code: row.problem }] : []);
        if (row.problem === "unresolved-template-scope") expect(report.references, row.id).toBeGreaterThanOrEqual(2);
        console.log("[DEBUG] macro scope oracle; id=" + row.id + "; native=" + status + "; problems=" + report.problems.length);
        } catch (error) {
          console.error("[DEBUG] macro scope failure; id=" + row.id + "; " + String(error));
          failures.push(error);
        }
      }
    }));
    for (const worker of workers) if (worker.status === "rejected") failures.push(worker.reason);
    if (failures.length) throw new AggregateError(failures, "Rust macro scope owner laws failed");
  } finally { rmSync(root, { recursive: true, force: true }); }
}, 45_000);

test("module graph authority requires every exact physical source read", async () => {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw new Error("Rust graph authority requires caller-owned output");
  mkdirSync(output, { recursive: true });
  const root = realpathSync(mkdtempSync(join(output, "rust-graph-authority-"))), corpus = read("🧫️fixtures/🧱️rust-source-direction/🔣️.json");
  const schema = read("🧬️schema/🧱️rust-source-direction/🔣️.json");
  expect(corpus["schemaVersion"]).toEqual(1);expect(corpus["attributeOrigins"]["provider"]).toEqual("extern crate proc_macro;\nuse proc_macro::TokenStream;\n#[proc_macro_attribute]\npub fn test(_: TokenStream, _: TokenStream) -> TokenStream { TokenStream::new() }\n#[proc_macro_attribute]\npub fn rewrite(_: TokenStream, _: TokenStream) -> TokenStream { TokenStream::new() }\n#[proc_macro_attribute]\npub fn doc(_: TokenStream, _: TokenStream) -> TokenStream { TokenStream::new() }\n");
  const rows = corpus.graphAuthority as readonly { id: string; files: Readonly<Record<string, string>>; unavailable: string | null; nativeInputs: readonly string[]; contexts: readonly string[]; targets: readonly (readonly [string, string])[]; invalidManifests: readonly string[]; participations: readonly ParticipationProjection[] }[];
  expect(new Set(rows.map((row) => row.id)).size).toBe(rows.length);
  const failures: unknown[] = [];
  try {
    for (const row of rows) {
      try {
        const cwd = join(root, row.id);
        mkdirSync(cwd);
        for (const [path, source] of Object.entries(row.files)) writeFileSync(join(cwd, path), source);
        const manifest = parseToml(readFileSync(join(cwd, "Cargo.toml"), "utf8"));
        expect((manifest.lib as { path: string }).path).toBe("root.rs");
        const { stdout, stderr, status } = await nativeCommand(["rustc", "--edition=2021", "--crate-type=lib", "--crate-name", "graph_probe", "--emit=dep-info=dependencies.d,link", "-o", "libgraph_probe.rlib", "root.rs"], cwd);
        expect(status, stdout + stderr).toBe(0);
        const depInfo = readFileSync(join(cwd, "dependencies.d"), "utf8"), inputs = /^libgraph_probe\.rlib: ([^\r\n]+)$/mu.exec(depInfo);
        expect(inputs, row.id).not.toBeNull();
        expect([...new Set(inputs![1]!.split(" ").filter(Boolean))].sort(), row.id).toEqual([...row.nativeInputs].sort());
        if (row.unavailable) { rmSync(join(cwd, row.unavailable)); expect(glob.sync(row.unavailable, { cwd, followSymbolicLinks: false }), row.id).toEqual([]); }
        const graph = inspectRustModuleGraph(Object.keys(row.files), (path) => {
          try { return readFileSync(join(cwd, path), "utf8"); } catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") return undefined; throw error; }
        }, { strictManifests: true });
        expect([...graph.contexts].flatMap(([path, rows]) => rows.map(() => path)).sort(), row.id).toEqual([...row.contexts]);
        expect([...graph.targets].sort((left, right) => left[0].localeCompare(right[0])), row.id).toEqual(row.targets.map(([key, target]): [string, string] => [key, target]));
        expect([...graph.invalidManifests].sort(), row.id).toEqual([...row.invalidManifests]);
        const participations = participationProjection(graph);
        expect<readonly ParticipationProjection[]>(participations, row.id).toEqual(row.participations);
        console.log("[DEBUG] graph authority oracle; id=" + row.id + "; native=" + status + "; contexts=" + [...graph.contexts.values()].flat().length);
      } catch (error) { console.error("[DEBUG] graph authority failure; id=" + row.id + "; " + String(error)); failures.push(error); }
    }
    if (failures.length) throw new AggregateError(failures, "Rust graph authority laws failed");
  } finally { rmSync(root, { recursive: true, force: true }); }
}, 45_000);


test("framework source and test paths cannot depend on implementation files", () => {
  for (const row of fixture.directions) {
    const refs = inspectRustCompileReferences(row.source);
    expect(rustSourceDirectionEdges(row.from, refs, fixture.rules), row.id).toEqual(row.edges);
    const targets = refs.map((ref) => oracleNormalize(oracleJoin(oracleDirname(row.from), ref.path)));
    expect(row.edges.every((edge) => targets.includes(edge.to)), row.id).toBe(true);
  }
  expect(() => rustSourceDirectionEdges("general/tests/source.rs", inspectRustCompileReferences('include!("../../../outside.rs");'), fixture.rules)).toThrow("escapes");
});

test("unsupported compile expressions fail closed", () => {
  for (const row of fixture.unsupported) expect(() => inspectRustCompileReferences(row.source), row.id).toThrow("Unsupported Rust compile");
  const refs = inspectRustCompileReferences('include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../specific/fixture.txt"));');
  expect(() => rustSourceDirectionEdges("general/source.rs", refs, fixture.rules)).toThrow("manifest provenance");
  expect(rustSourceDirectionEdges("general/source.rs", refs, fixture.rules, { manifestPaths: ["general/Cargo.toml"] })).toEqual([{ rule: "framework-no-implementation", from: "general/source.rs", to: "specific/fixture.txt", kind: "include_str", line: 1 }]);
  expect(rustSourceDirectionEdges("general/source.rs", inspectRustCompileReferences('include!(concat!(env!("OUT_DIR"), "/generated.rs"));'), fixture.rules)).toEqual([]);
});

import "../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/📤️generation/🧪️tests/🟦️.ts";
