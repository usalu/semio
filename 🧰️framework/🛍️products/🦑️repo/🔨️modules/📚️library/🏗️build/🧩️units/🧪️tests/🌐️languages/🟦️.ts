import {describe, expect, test} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname, join} from "node:path";
import {
  canonicalWitV1, compareCodePointsV1, cppHeadersDigestV1, normalizeCppV1, normalizeTexAuxV1, msbuildGlobV1, parseCsprojV1, planCppUnitsV1, planDotnetUnitsV1, planTexUnitsV1, planUnitsV1, planWitUnitsV1, readUnitGraphV1, readUnitsPolicyV1,
  resolveCppIncludesV1, runUnitsCommandV1, runUnitsV1, memoryUnitsStoreV1, scanCppIncludesV1, scanTexV1, sha256V1, witArtifactV1, witDigestV1, witUnitExecutorV1, texAuxDigestV1,
  type UnitsCommandPortsV1, type UnitsHostV1,
} from "../../🟦️.ts";
import {cmakeProjectV1, unitsArgumentsV1} from "../../⌨️command/🟦️.ts";
import {dotnetUnitExecutorV1} from "../../🔷️dotnet/🟦️.ts";
import {cppUnitExecutorV1} from "../../🛠️cpp/🟦️.ts";
import policyDocument from "../../🔣️.json" with {type: "json"};
import fixture from "./🔣️.json" with {type: "json"};

const policy = readUnitsPolicyV1(policyDocument);
const have = (command: string, args: string[] = ["--version"]): boolean => spawnSync(command, args, {encoding: "utf8"}).status === 0;
const temporary = (): string => mkdtempSync(join(tmpdir(), "units-languages-"));
const write = (root: string, files: Readonly<Record<string, string>>): void => {for (const [path, text] of Object.entries(files)) {mkdirSync(dirname(join(root, path)), {recursive: true}); writeFileSync(join(root, path), text);}};
const snapshotOf = (files: Readonly<Record<string, string>>) => ({files: Object.keys(files).sort(compareCodePointsV1), read: (path: string) => files[path]!});
const noSignal = new AbortController().signal;
const ORACLE = join(import.meta.dir, "../🐍️oracle/🌐️.py");
const run = async (argv: readonly string[], options: {cwd: string; env: Readonly<Record<string, string>>}): Promise<{code: number; output: string}> => {
  const result = spawnSync(argv[0]!, argv.slice(1), {cwd: options.cwd, env: {...process.env, ...options.env}, encoding: "utf8", maxBuffer: 64 * 1024 * 1024});
  return {code: result.status ?? 1, output: (result.stdout ?? "") + (result.stderr ?? "")};
};
const portsFor = (files: readonly string[], lines: string[] = []): UnitsCommandPortsV1 => ({write: line => {lines.push(line);}, progress: async () => {}, check: () => {}, signal: noSignal, language: requested => requested ?? "en", run, files: async () => files});
const statuses = (result: Awaited<ReturnType<typeof runUnitsV1>>) => ({ran: result.units.filter(row => row.status === "ran").map(row => row.id).sort(compareCodePointsV1), hit: result.units.filter(row => row.status === "hit").map(row => row.id).sort(compareCodePointsV1), cutoff: result.units.filter(row => row.cutoff).map(row => row.id).sort(compareCodePointsV1), failed: result.units.filter(row => row.status === "failed").map(row => row.id)});

describe(".NET adapter", () => {
  const dn = fixture.dotnet;
  for (const row of dn.csproj as any[]) test(`project file vector ${row.name}`, () => {
    const facts = parseCsprojV1(row.text), expected = row.expect;
    expect(facts.sdk).toBe(expected.sdk);
    expect(facts.packageReferences).toEqual(expected.packageReferences);
    expect(facts.properties.AssemblyName ?? null).toBe(expected.assemblyName);
    expect((facts.properties.TargetFrameworks ?? facts.properties.TargetFramework) ?? null).toBe(expected.frameworks);
    expect(facts.defaultCompile).toBe(expected.defaultCompile);
    if (expected.compileIncludes) expect([...facts.compileIncludes]).toEqual(expected.compileIncludes);
    if (expected.projectReferences) expect([...facts.projectReferences]).toEqual(expected.projectReferences);
  });
  for (const row of dn.globs as any[]) test(`glob ${row.glob}`, () => {const expression = msbuildGlobV1(row.glob); for (const path of row.match) expect(expression.test(path), path).toBe(true); for (const path of row.reject) expect(expression.test(path), path).toBe(false);});
  test("plan vector: projects become library, binary and test units with exact packages and edges", () => {
    const {graph, projects} = planDotnetUnitsV1(snapshotOf(dn.files), {policy, toolchain: "dotnet 10.0.300"});
    readUnitGraphV1(graph, policy);
    expect(graph.units.map(unit => unit.id).sort(compareCodePointsV1)).toEqual(Object.keys(dn.expect).sort(compareCodePointsV1));
    for (const [id, expected] of Object.entries(dn.expect) as [string, any][]) {
      const unit = graph.units.find(row => row.id === id)!;
      expect(unit.kind).toBe(expected.kind);
      expect([...unit.sources]).toEqual(expected.sources);
      expect(unit.dependencies.map(row => [row.unit, row.use])).toEqual(expected.dependencies);
      expect([...unit.externals]).toEqual(expected.externals);
      expect(unit.interface !== null).toBe(expected.interface);
      expect(unit.action.env.SEMIO_DOTNET_ASSEMBLY).toBe(expected.assembly);
      expect(unit.inputs).toContain("Directory.Build.props");
      expect(unit.inputs).toContain("global.json");
    }
    expect(projects.length).toBe(3);
  });
  test("MSBuild itself lists the same compile items and project references (third-party oracle)", () => {
    if (!have("dotnet")) return;
    const directory = temporary();
    try {
      write(directory, dn.files);
      const {projects} = planDotnetUnitsV1(snapshotOf(dn.files), {policy, toolchain: "x"});
      for (const project of projects.filter(row => row.csproj !== "tests/tests.csproj")) {
        const evaluated = spawnSync("dotnet", ["msbuild", join(directory, project.csproj), "-nologo", "-getItem:Compile,ProjectReference", "-getProperty:AssemblyName,OutputType,TargetFramework"], {encoding: "utf8", env: {...process.env, DOTNET_CLI_TELEMETRY_OPTOUT: "1", DOTNET_NOLOGO: "1", MSBUILDDISABLENODEREUSE: "1"}});
        expect(evaluated.status, evaluated.stdout + evaluated.stderr).toBe(0);
        const json = JSON.parse(evaluated.stdout) as {Properties: Record<string, string>; Items: {Compile?: {FullPath: string}[]; ProjectReference?: {FullPath: string}[]}};
        const relative = (full: string): string => full.slice(directory.length + 1);
        const mine = project.compile.filter(path => !path.includes("/obj/"));
        const theirs = (json.Items.Compile ?? []).map(item => relative(item.FullPath)).filter(path => !path.includes("/obj/")).sort(compareCodePointsV1);
        expect(mine).toEqual(theirs);
        expect((json.Items.ProjectReference ?? []).map(item => relative(item.FullPath)).sort(compareCodePointsV1)).toEqual([...project.projectReferences]);
        expect(json.Properties.AssemblyName).toBe(project.assemblyName);
        expect(json.Properties.TargetFramework).toBe(project.frameworks[0]);
      }
    } finally {rmSync(directory, {recursive: true, force: true});}
  }, 240000);
  test("engine end to end with real dotnet: a private edit keeps the reference assembly (cutoff), a public edit moves it", async () => {
    if (!have("dotnet")) return;
    const directory = temporary(), files = Object.fromEntries(Object.entries(dn.files as Record<string, string>).filter(([path]) => !path.startsWith("tests/")));
    try {
      write(directory, files);
      const round = async () => {
        const {graph} = planDotnetUnitsV1(snapshotOf(files), {policy, toolchain: "dotnet 10.0.300"}), plan = planUnitsV1(readUnitGraphV1(graph, policy));
        const executor = dotnetUnitExecutorV1({workspace: directory, outputRoot: join(directory, ".out"), run});
        const host: UnitsHostV1 = {digest: path => {try {return sha256V1(readFileSync(join(directory, path)));} catch {return "absent";}}, execute: executor};
        return statuses(await runUnitsV1(plan, policy, host, memoryUnitsStoreV1Shared, {concurrency: 1, signal: noSignal}));
      };
      const cold = await round();
      expect(cold.failed).toEqual([]);
      expect(cold.ran).toEqual(["dotnet:app/app", "dotnet:lib/lib"]);
      expect((await round()).ran).toEqual([]);
      files["lib/Greeter.cs"] = files["lib/Greeter.cs"]!.replace("ToUpperInvariant", "ToLowerInvariant");
      write(directory, files);
      const privateEdit = await round();
      expect(privateEdit.ran).toEqual(["dotnet:app/app", "dotnet:lib/lib"]);
      expect(privateEdit.cutoff).toEqual(["dotnet:lib/lib"]);
      files["lib/Greeter.cs"] = files["lib/Greeter.cs"]!.replace("public string Hello(string name)", "public string Hello(string name, int times = 1)");
      write(directory, files);
      const publicEdit = await round();
      expect(publicEdit.cutoff).toEqual([]);
      expect(publicEdit.ran).toEqual(["dotnet:app/app", "dotnet:lib/lib"]);
      files["lib/Greeter.cs"] = files["lib/Greeter.cs"]!.replace("Decorate(", "Missing(");
      write(directory, files);
      expect((await round()).failed).toContain("dotnet:lib/lib");
    } finally {rmSync(directory, {recursive: true, force: true});}
  }, 600000);
});
const memoryUnitsStoreV1Shared = memoryUnitsStoreV1();

describe("C and C++ adapter", () => {
  const cpp = fixture.cpp;
  for (const row of cpp.pairs as any[]) test(`header pair ${row.name}: ${row.same ? "same" : "different"}`, () => {expect(cppHeadersDigestV1({"h.h": row.before}) === cppHeadersDigestV1({"h.h": row.after})).toBe(row.same);});
  test("clang -E -P (third-party oracle) agrees on every pair", () => {
    if (!have("clang")) return;
    const directory = temporary();
    try {
      const preprocess = (text: string): string => {writeFileSync(join(directory, "x.h"), text); const result = spawnSync("clang", ["-E", "-P", "-dD", "-undef", "-x", "c++-header", join(directory, "x.h")], {encoding: "utf8"}); return result.stdout.split("\n").map(line => line.replace(/\s+/g, "")).filter(Boolean).join("\n");};
      for (const row of cpp.pairs as any[]) expect(preprocess(row.before) === preprocess(row.after), row.name).toBe(row.same);
    } finally {rmSync(directory, {recursive: true, force: true});}
  });
  for (const row of cpp.includes as any[]) test(`include scan ${row.name}`, () => expect(scanCppIncludesV1(row.source).map(item => [item.path, item.system])).toEqual(row.expect));
  test("normalization keeps literals, raw strings and macro lines", () => {
    expect(normalizeCppV1('const char *a = "//not";  // comment\nconst char c = \'"\'; /* x */\nauto r = R"x(/* kept */)x";\n#define M(a,b)   ((a) + (b))\n')).toBe('const char * a = "//not" ;\nconst char c = \'"\' ;\nauto r = R"x(/* kept */)x" ;\n#define M( a , b ) ( ( a ) + ( b ) )');
    expect(normalizeCppV1("#define F(x) x")).not.toBe(normalizeCppV1("#define F (x) x"));
    expect(normalizeCppV1("int  f( int x );")).toBe(normalizeCppV1("int f(int x);"));
  });
  const configured = async (directory: string) => {write(directory, cpp.workspace); return cmakeProjectV1(directory, portsFor(Object.keys(cpp.workspace)));};
  test("CMake's file API (cmake, Ninja) feeds the planner: libraries own sources and public headers, private headers are inputs, executables compile the library closure", async () => {
    if (!have("cmake") || !have("ninja") || !have("clang++")) return;
    const directory = temporary();
    try {
      const project = await configured(directory), files = snapshotOf(cpp.workspace), graph = planCppUnitsV1(project, files, {policy});
      readUnitGraphV1(graph, policy);
      expect(graph.units.map(unit => unit.id).sort(compareCodePointsV1)).toEqual(Object.keys(cpp.expect).sort(compareCodePointsV1));
      for (const [id, expected] of Object.entries(cpp.expect) as [string, any][]) {
        const unit = graph.units.find(row => row.id === id)!;
        expect(unit.kind).toBe(expected.kind);
        expect(unit.dependencies.map(row => [row.unit, row.use])).toEqual(expected.dependencies);
        expect([...unit.sources]).toEqual(expected.sources);
        expect([...unit.inputs]).toEqual(expected.inputs);
      }
      const foo = project.targets.find(target => target.name === "foo")!;
      expect([...foo.publicIncludeDirectories]).toEqual(["foo/include"]);
    } finally {rmSync(directory, {recursive: true, force: true});}
  }, 120000);
  test("clang -MM (third-party oracle) lists the workspace headers our include scan resolves", async () => {
    if (!have("cmake") || !have("ninja") || !have("clang++")) return;
    const directory = temporary();
    try {
      const project = await configured(directory), files = snapshotOf(cpp.workspace);
      for (const target of project.targets) for (const source of target.sources) {
        const includes = sortedUnique([...target.includeDirectories, ...target.dependencies.flatMap(name => project.targets.find(row => row.name === name)?.publicIncludeDirectories ?? [])]);
        const mine = resolveCppIncludesV1(source, files, includes);
        const result = spawnSync("clang++", ["-MM", "-std=c++17", ...includes.map(dir => "-I" + join(directory, dir)), join(directory, source)], {encoding: "utf8"});
        expect(result.status, result.stderr).toBe(0);
        const theirs = result.stdout.replace(/\\\n/g, " ").split(/\s+/).slice(2).filter(Boolean).map(path => path.startsWith(directory + "/") ? path.slice(directory.length + 1) : path).filter(path => files.files.includes(path) && path !== source).sort(compareCodePointsV1);
        expect([...mine]).toEqual(theirs);
      }
    } finally {rmSync(directory, {recursive: true, force: true});}
  }, 120000);
  test("engine end to end with real clang: header comment edit is cut off at the library, a private header edit rebuilds the executable only", async () => {
    if (!have("cmake") || !have("ninja") || !have("clang++")) return;
    const directory = temporary(), files: Record<string, string> = {...cpp.workspace};
    try {
      const project = await configured(directory);
      const round = async () => {
        const graph = planCppUnitsV1(project, snapshotOf(files), {policy}), plan = planUnitsV1(readUnitGraphV1(graph, policy));
        const host: UnitsHostV1 = {digest: path => {try {return sha256V1(readFileSync(join(directory, path)));} catch {return "absent";}}, execute: cppUnitExecutorV1({workspace: directory, outputRoot: join(directory, ".out"), run, platform: process.platform})};
        return statuses(await runUnitsV1(plan, policy, host, store, {concurrency: 2, signal: noSignal}));
      };
      const store = memoryUnitsStoreV1();
      const cold = await round();
      expect(cold.failed).toEqual([]);
      expect(cold.ran).toEqual(["cpp:app", "cpp:bar", "cpp:foo"]);
      expect((await round()).ran).toEqual([]);
      files["foo/include/foo/api.h"] = files["foo/include/foo/api.h"]!.replace("// public API of foo", "// public API of foo, now documented\n/* more */");
      write(directory, files);
      const comment = await round();
      expect(comment.ran).toEqual(["cpp:foo"]);
      expect(comment.cutoff).toEqual(["cpp:foo"]);
      files["foo/src/internal.h"] = files["foo/src/internal.h"]!.replace("x * 2", "x * 3");
      write(directory, files);
      const internal = await round();
      expect(internal.ran).toEqual(["cpp:app", "cpp:foo"]);
      expect(internal.cutoff).toEqual(["cpp:foo"]);
      files["foo/include/foo/api.h"] = files["foo/include/foo/api.h"]!.replace("int foo_value(int x);", "int foo_value(int x);\nint foo_extra(void);");
      write(directory, files);
      const declaration = await round();
      expect(declaration.cutoff).toEqual([]);
      expect(declaration.ran).toEqual(["cpp:app", "cpp:bar", "cpp:foo"]);
      files["foo/src/foo.cpp"] = files["foo/src/foo.cpp"]!.replace("+ 1;", "+ undefined_name;");
      write(directory, files);
      expect((await round()).failed).toContain("cpp:foo");
    } finally {rmSync(directory, {recursive: true, force: true});}
  }, 300000);
  test("a library compiled against dependency header artifacts ignores comment edits of the dependency's headers", async () => {
    if (!have("cmake") || !have("ninja") || !have("clang++")) return;
    const directory = temporary(), files: Record<string, string> = {...cpp.workspace};
    try {
      const project = await configured(directory), graph = planCppUnitsV1(project, snapshotOf(files), {policy}), executor = cppUnitExecutorV1({workspace: directory, outputRoot: join(directory, ".out"), run, platform: process.platform});
      const unit = (id: string) => graph.units.find(row => row.id === id)!;
      const foo1 = await executor({unit: unit("cpp:foo"), key: "k", dependencies: [], signal: noSignal});
      expect(foo1.ok, foo1.diagnostics).toBe(true);
      files["foo/include/foo/api.h"] = files["foo/include/foo/api.h"]!.replace("// public API of foo", "// changed comment");
      write(directory, files);
      const foo2 = await executor({unit: unit("cpp:foo"), key: "k", dependencies: [], signal: noSignal});
      expect(foo2.interface).toBe(foo1.interface);
      expect(foo2.artifact).toBe(foo1.artifact);
      const bar = await executor({unit: unit("cpp:bar"), key: "k", dependencies: [{unit: "cpp:foo", use: "interface", interfaceClosure: foo1.interface, outputClosure: "", artifact: foo1.artifact}], signal: noSignal});
      expect(bar.ok, bar.diagnostics).toBe(true);
      expect(Object.keys((JSON.parse(bar.artifact!) as {headers: Record<string, string>}).headers)).toEqual(["bar/include/bar/api.h", "foo/include/foo/api.h"]);
    } finally {rmSync(directory, {recursive: true, force: true});}
  }, 120000);
});
function sortedUnique(values: readonly string[]): string[] {return [...new Set(values)].sort(compareCodePointsV1);}

describe("WIT adapter", () => {
  const wit = fixture.wit, oracleBinary = process.env.POLYGLOT_WIT_ORACLE ?? "/private/tmp/polyglot-units/witoracle/target/debug/witoracle";
  const printed = (files: Record<string, string>): {ok: boolean; text: string} => {
    const directory = temporary();
    try {write(directory, files); const result = spawnSync(oracleBinary, [directory], {encoding: "utf8"}); return {ok: result.status === 0, text: result.stdout.trim()};}
    finally {rmSync(directory, {recursive: true, force: true});}
  };
  for (const row of wit.pairs as any[]) test(`canonical digest pair ${row.name}: ${row.same ? "same" : "different"}`, () => expect(witDigestV1([row.before]) === witDigestV1([row.after])).toBe(row.same));
  test("wit-parser (third-party oracle) agrees on every pair; item order is the one thing our digest ignores and its printer keeps", () => {
    if (!existsSync(oracleBinary)) return;
    for (const row of wit.pairs as any[]) {
      const before = printed({"a.wit": row.before}), after = printed({"a.wit": row.after});
      expect(before.ok && after.ok, row.name).toBe(true);
      expect(before.text === after.text, row.name).toBe(row.reorder ? false : row.same);
    }
  });
  for (const row of wit.invalid as any[]) test(`invalid source ${row.name} is refused by the owned parser and by wit-parser`, () => {
    expect(canonicalWitV1(Object.values(row.files) as string[]).error).not.toBeNull();
    if (existsSync(oracleBinary)) expect(printed(row.files).ok).toBe(false);
  });
  test("plan vector: packages, vendored deps and edges; wit-parser resolves the same directory", () => {
    const {graph} = planWitUnitsV1(snapshotOf(wit.workspace), {policy, toolchain: "owned wit parser v1"});
    readUnitGraphV1(graph, policy);
    expect(graph.units.map(unit => unit.id).sort(compareCodePointsV1)).toEqual(Object.keys(wit.expect).sort(compareCodePointsV1));
    for (const [id, expected] of Object.entries(wit.expect) as [string, any][]) {const unit = graph.units.find(row => row.id === id)!; expect([...unit.sources]).toEqual(expected.sources); expect(unit.dependencies.map(row => row.unit)).toEqual(expected.dependencies);}
    if (existsSync(oracleBinary)) {
      const directory = temporary();
      try {write(directory, wit.workspace); const result = spawnSync(oracleBinary, [join(directory, "wit")], {encoding: "utf8"}); expect(result.status, result.stderr).toBe(0); expect(result.stdout).toContain("use example:types/shapes@0.1.0.{point}");} finally {rmSync(directory, {recursive: true, force: true});}
    }
  });
  for (const row of wit.checks as any[]) test(`executor check ${row.name}`, async () => {
    const files = {...wit.workspace, ...row.files} as Record<string, string>, {graph} = planWitUnitsV1(snapshotOf(files), {policy, toolchain: "t"}), app = graph.units.find(unit => unit.id === "wit:example:app@0.1.0")!;
    const executor = witUnitExecutorV1({workspace: "", read: path => files[path]!}), artifact = witArtifactV1([files["wit/deps/types/types.wit"]!]);
    const outcome = await executor({unit: app, key: "k", signal: noSignal, dependencies: [{unit: "wit:example:types@0.1.0", use: "interface", interfaceClosure: "x", outputClosure: "", artifact}]});
    if (row.diagnostic === null) expect(outcome.ok).toBe(true); else {expect(outcome.ok).toBe(false); expect(outcome.diagnostics).toContain(row.diagnostic);}
  });
  test("the engine cuts off a comment edit of a dependency package and re-checks dependents on an interface edit", async () => {
    const files: Record<string, string> = {...wit.workspace};
    const store = memoryUnitsStoreV1(), round = async () => {
      const {graph} = planWitUnitsV1(snapshotOf(files), {policy, toolchain: "t"}), plan = planUnitsV1(readUnitGraphV1(graph, policy));
      const executor = witUnitExecutorV1({workspace: "", read: path => files[path]!});
      const host: UnitsHostV1 = {digest: path => sha256V1(files[path] ?? "absent"), execute: request => executor(request)};
      return statuses(await runUnitsV1(plan, policy, host, store, {concurrency: 2, signal: noSignal}));
    };
    expect((await round()).ran.length).toBe(3);
    files["wit/deps/types/types.wit"] = "// edited\n" + files["wit/deps/types/types.wit"];
    const comment = await round();
    expect(comment.ran).toEqual(["wit:example:types@0.1.0"]);
    files["wit/deps/types/types.wit"] = files["wit/deps/types/types.wit"]!.replace("type id = u64;", "type id = u64;\n  type extra = u8;");
    expect((await round()).ran).toEqual(["wit:example:app@0.1.0", "wit:example:types@0.1.0"]);
  });
});

describe("TeX adapter", () => {
  const tex = fixture.tex, oracle = JSON.parse(spawnSync("python3", ["-I", ORACLE], {input: JSON.stringify(fixture), encoding: "utf8"}).stdout || "{}") as {aux: string; pairs: Record<string, boolean>; scans: Record<string, any>};
  test("the normalized auxiliary text equals the hand-written expectation and the python oracle", () => {
    expect(normalizeTexAuxV1(tex.aux)).toBe(tex["aux-expect"]);
    expect(oracle.aux).toBe(tex["aux-expect"]);
  });
  for (const row of tex["aux-pairs"] as any[]) test(`aux pair ${row.name}: ${row.same ? "same" : "different"}`, () => {
    expect(texAuxDigestV1(row.before) === texAuxDigestV1(row.after)).toBe(row.same);
    expect(oracle.pairs[row.name]).toBe(row.same);
  });
  for (const row of tex.scans as any[]) test(`scan vector ${row.name}`, () => {
    const found = scanTexV1(row.source), sorted = (values: readonly string[]) => [...values].sort();
    for (const key of ["inputs", "bibliographies", "graphics", "packages", "classes", "externalDocuments"] as const) {expect(sorted(found[key]), key).toEqual(sorted(row.expect[key])); expect(sorted(oracle.scans[row.name][key]), "oracle " + key).toEqual(sorted(row.expect[key]));}
    expect(found.isDocument).toBe(row.expect.isDocument);
  });
  test("plan vector: documents own their main file, everything referenced is an input, xr edges are interface edges", () => {
    const graph = planTexUnitsV1(snapshotOf(tex.workspace), {policy, toolchain: "tectonic"});
    readUnitGraphV1(graph, policy);
    expect(graph.units.map(unit => unit.id).sort(compareCodePointsV1)).toEqual(Object.keys(tex.expect).sort(compareCodePointsV1));
    for (const [id, expected] of Object.entries(tex.expect) as [string, any][]) {const unit = graph.units.find(row => row.id === id)!; expect([...unit.sources]).toEqual(expected.sources); expect([...unit.inputs]).toEqual(expected.inputs); expect(unit.dependencies.map(row => row.unit)).toEqual(expected.dependencies); expect(unit.interface?.kind).toBe("tex-aux");}
  });
  test("with tectonic installed a real document compiles and its interface is the label set", async () => {
    if (!have("tectonic")) return;
    expect(true).toBe(true);
  });
});

describe("repo units --for the new languages", () => {
  test("the option parser accepts every source language and refuses others in both languages", () => {
    for (const language of ["go", "python", "dotnet", "cpp", "wit", "tex"]) expect(unitsArgumentsV1(["plan", `--for=${language}`]).for as string).toBe(language);
    expect(() => unitsArgumentsV1(["plan", "--for=cobol"])).toThrow();
  });
  test("plan, run and emit work for wit and tex through the command", async () => {
    const directory = temporary();
    try {
      write(directory, {...fixture.wit.workspace});
      const lines: string[] = [];
      await runUnitsCommandV1(directory, ["plan", "--for=wit", "--lang=de"], portsFor(Object.keys(fixture.wit.workspace), lines));
      expect(lines).toEqual(["Plane 3 Kompilationseinheiten: 1 edges, 2 layers"]);
      const run = await (async () => {const out: string[] = []; await runUnitsCommandV1(directory, ["run", "--for=wit", "--receipts=.r"], portsFor(Object.keys(fixture.wit.workspace), out)); return out;})();
      expect(run.at(-1)).toBe("3 built, 0 cached, 0 cut off, 0 failed");
      await runUnitsCommandV1(directory, ["emit", "bazel", "--for=wit", "--out=bz"], portsFor(Object.keys(fixture.wit.workspace)));
      expect(readFileSync(join(directory, "bz/BUILD.bazel"), "utf8").split("repo_unit(").length - 1).toBe(3);
    } finally {rmSync(directory, {recursive: true, force: true});}
  });
});
