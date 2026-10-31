import "./📣️invocations/🪟️batch/🟦️.ts";
import {resolve,dirname} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import _ from "lodash";
import ts from "typescript";
import {readFileSync} from "node:fs";
import {runInNewContext} from "node:vm";
import invocationCases from "./📣️invocations/🔣️.json" with {type:"json"};
import invocationSchema from "./📣️invocations/🧬️schema/🔣️.json" with {type:"json"};
import { describe, expect, test } from "bun:test";
import { findToolV1, readToolDigestsV1, readToolSourceV1, resolveToolV1, toolDigestForHostV1, toolEnvironmentNameMatchesV1, toolEnvironmentV1, toolPointerV1, type ToolHostV1 } from "../../🟦️.ts";

const files = (table: Record<string, string>) => ({ read: (path: string) => table[path] });
const host = (overrides: Partial<Omit<ToolHostV1, "pins">> & { present?: string[]; pins?: Record<string, string> } = {}): ToolHostV1 => {
  const { present: names, pins: table, ...rest } = overrides, present = new Set(names ?? []);
  return { platform: "darwin", architecture: "arm64", environment: { PATH: "/bin:/usr/bin" }, root: "/repo", home: "/home/u", execPath: "/runtime/node", runtimeCommand: "node", isExecutable: path => present.has(path), exists: path => present.has(path), list: () => [], pins: files(table ?? {}), ...rest };
};
const BOOT = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🧬️schema/🔣️.json";
const bootstrap = JSON.stringify({ definitions: { Pins: { const: { assets: [
  { tool: "go", version: "1.26.9", platform: "darwin", arch: "arm64", url: "u", sha256: "a".repeat(64) },
  { tool: "go", version: "1.26.9", platform: "linux", arch: "x64", url: "u", sha256: "b".repeat(64) },
  { tool: "uv", version: "0.11.6", platform: "darwin", arch: "arm64", url: "u", sha256: "c".repeat(64) },
] } } } });

describe("tool pins", () => {
  test("follows JSON pointers with escapes and reads TOML keys, text files and filtered rows", () => {
    expect(toolPointerV1({ "a/b": { "c~d": [10, 20] } }, "/a~1b/c~0d/1")).toBe(20);
    const go = findToolV1("go")!, cargo = findToolV1("cargo")!;
    expect(readToolSourceV1(go.pin.version, "/repo", files({ ["/repo/" + BOOT]: bootstrap }))).toBe("1.26.9");
    expect(readToolSourceV1(cargo.pin.version, "/repo", files({ "/repo/rust-toolchain.toml": "[toolchain]\nprofile = \"minimal\"\nchannel = \"nightly-2026-07-20\"\n" }))).toBe("nightly-2026-07-20");
    expect(readToolSourceV1({ file: ".bazelversion", format: "text" }, "/repo", files({ "/repo/.bazelversion": "8.4.2\n" }))).toBe("8.4.2");
    expect(readToolSourceV1(go.pin.version, "/repo", files({}))).toBeNull();
  });
  test("joins a CMake version object and keeps only the rows of the filter", () => {
    expect(readToolSourceV1({ file: "p.json", format: "json", pointer: "/v" }, "/r", files({ "/r/p.json": JSON.stringify({ v: { major: 3, minor: 28, patch: 0 } }) }))).toBe("3.28.0");
    const rows = readToolDigestsV1(findToolV1("go")!.pin.digests, "/repo", files({ ["/repo/" + BOOT]: bootstrap }));
    expect(rows.map(row => row.platform + "-" + row.architecture)).toEqual(["darwin-arm64", "linux-x64"]);
    expect(toolDigestForHostV1({ state: "pinned", version: "1", minimum: null, digests: rows }, "linux", "x64")?.sha256).toBe("b".repeat(64));
    expect(toolDigestForHostV1({ state: "pinned", version: "1", minimum: null, digests: rows }, "win32", "x64")).toBeUndefined();
  });
  test("reads one host field such as darwin-arm64", () => {
    const rows = readToolDigestsV1({ file: "d.json", pointer: "/d", where: {}, host: "host", sha256: "sha", name: "archive" }, "/r", files({ "/r/d.json": JSON.stringify({ d: [{ host: "darwin-arm64", sha: "x", archive: "a" }, { host: "win32-x64", sha: "y", archive: "b" }] }) }));
    expect(rows.map(row => [row.platform, row.architecture])).toEqual([["darwin", "arm64"], ["win32", "x64"]]);
  });
});

describe("tool resolution", () => {
  const git = findToolV1("git")!, bun = findToolV1("bun")!, tsc = findToolV1("tsc")!, go = findToolV1("go")!, hdiutil = findToolV1("hdiutil")!;
  test("tries the environment override first, then PATH, then the extra directories", () => {
    expect(resolveToolV1(git, host({ present: ["/over/git"], environment: { PATH: "/bin", SEMIO_TOOL_GIT: "/over/git" } })).command).toBe("/over/git");
    expect(resolveToolV1(git, host({ present: ["/bin/git"] })).source).toBe("path");
    expect(resolveToolV1(git, host({ present: ["/opt/homebrew/bin/git"], environment: { PATH: "/bin" } })).command).toBe("/opt/homebrew/bin/git");
    const missing = resolveToolV1(git, host());
    expect(missing.command).toBeNull();
    expect(missing.source).toBe("missing");
    expect(missing.tried.length).toBeGreaterThan(1);
  });
  test("uses the running Bun as bun and the provisioned digest directory otherwise", () => {
    expect(resolveToolV1(bun, host({ runtimeCommand: "bun", execPath: "/x/bun" })).command).toBe("/x/bun");
    const digest = "d".repeat(64), pins = { ["/repo/" + bun.pin.digests!.file]: JSON.stringify({ definitions: { Release: { const: { version: "1.4.2", distributions: [{ platform: "darwin", arch: "arm64", musl: false, archive: "a", bytes: 1, sha256: digest }] } } } }) };
    const found = resolveToolV1(bun, host({ pins, execPath: "/runtime/node", present: [`/repo/.cache/b/t/m/bun/${digest}/bun`] }));
    expect(found.source).toBe("provisioned");
    expect(found.pin.version).toBe("1.4.2");
  });
  test("builds the host and 16-digit placeholders of a provisioned Go", () => {
    const sha = "a".repeat(64), found = resolveToolV1(go, host({ pins: { ["/repo/" + BOOT]: bootstrap }, present: [`/repo/.cache/b/t/m/native/go/1.26.9/darwin-arm64-${sha.slice(0, 16)}/bin/go`] }));
    expect(found.source).toBe("provisioned");
  });
  test("runs a package script through its interpreter and refuses a platform the tool lacks", () => {
    const found = resolveToolV1(tsc, host({ runtimeCommand: "bun", execPath: "/x/bun", present: ["/repo/node_modules/typescript/bin/tsc"] }));
    expect(found.command).toBe("/x/bun");
    expect(found.prefix).toEqual(["/repo/node_modules/typescript/bin/tsc"]);
    expect(resolveToolV1(hdiutil, host({ platform: "linux", present: ["/usr/bin/hdiutil"] })).command).toBeNull();
  });
});

describe("tool environment", () => {
  const context = { root: "/repo", platform: "linux" as const };
  test("always drops IDE and debugger injections and lets the caller win", () => {
    const env = toolEnvironmentV1(findToolV1("bun")!, { PATH: "/bin", NODE_OPTIONS: "--inspect", VSCODE_INSPECTOR_OPTIONS: "x", CURSOR_TRACE_ID: "t", BUN_INSPECT: "1", BUN_JSC_x: "1", KEEP: "1" }, { EXTRA: "2", PATH: undefined }, context);
    expect(env).toEqual({ BUN_JSC_x: "1", KEEP: "1", EXTRA: "2", SEMIO_TOOL_RUN: "bun" });
  });
  test("lets an allowlist tool keep only the base and its own variables", () => {
    const env = toolEnvironmentV1(findToolV1("hdiutil")!, { PATH: "/bin", HOME: "/h", SEMIO_X: "1", BAZEL_FOO: "1", SECRET: "s", LC_ALL: "C" }, {}, context);
    expect(Object.keys(env).sort()).toEqual(["HOME", "LC_ALL", "PATH", "SEMIO_TOOL_RUN", "SEMIO_X"]);
  });
  test("sets fixed values and cache directories without overriding what the environment already has", () => {
    expect(toolEnvironmentV1(findToolV1("uv")!, {}, {}, context).UV_PYTHON_DOWNLOADS).toBe("never");
    expect(toolEnvironmentV1(findToolV1("go")!, {}, {}, context).GOCACHE).toBe("/repo/.🧬semio/🦑️repo/⚡️cache/go");
    expect(toolEnvironmentV1(findToolV1("go")!, { GOCACHE: "/mine" }, {}, context).GOCACHE).toBe("/mine");
    expect(toolEnvironmentV1(findToolV1("go")!, {}, {}, { ...context, cacheDirectory: (...parts) => "/c/" + parts.join("/") }).GOCACHE).toBe("/c/go");
  });
  test("matches names with wildcards and ignores case on Windows only", () => {
    expect(toolEnvironmentNameMatchesV1("LC_*", "LC_ALL", "linux")).toBe(true);
    expect(toolEnvironmentNameMatchesV1("path", "PATH", "win32")).toBe(true);
    expect(toolEnvironmentNameMatchesV1("path", "PATH", "linux")).toBe(false);
  });
});

/** 📣️ Owned secondary invocations use the qualified primary; genuine sibling programs keep their own lookup. */
describe("declared primary invocations",()=>{
 test("closed neutral invocation declarations agree with Ajv",()=>{const validate=new Ajv({strict:true}).compile(invocationSchema);expect(validate(invocationCases)).toBe(true);const row=new Ajv({strict:true}).compile(invocationSchema.definitions.Invocation);expect(row(invocationCases.invocation)).toBe(true);for(const invalid of invocationCases.invalid)expect(row(invalid)).toBe(false);});
 for(const row of invocationCases.resolutions)test(`actual resolver retains declared invocation ${row.id}`,()=>{
  const base=findToolV1(row.tool)!,command=row.command??undefined,invocations=row.tool==="bun"?[invocationCases.invocation]:row.tool==="tsc"?[{command:"tscx",prefix:["x"]}]:[],entry={...base,commands:row.tool==="tsc"?[...base.commands,"tscx"]:base.commands,invocations},digest="d".repeat(64),pins=row.source==="provisioned"?{["/repo/"+base.pin.digests!.file]:JSON.stringify({definitions:{Release:{const:{version:"1.4.2",distributions:[{platform:"darwin",arch:"arm64",musl:false,archive:"a",bytes:1,sha256:digest}]}}}})}:{},environment={PATH:"/bin",PATHEXT:".EXE;.CMD;.BAT",...(row.source==="environment"?{SEMIO_TOOL_BUN:"/owned/bun"}:{}),...(row.tool==="cargo"?{SEMIO_TOOL_CARGO:"/owned/cargo"}:{})},actual=resolveToolV1(entry,host({platform:row.platform as "darwin"|"win32",environment,present:row.present,pins,runtimeCommand:row.source==="runtime"||row.source==="package-script"?"bun":"node",execPath:row.source==="runtime"||row.source==="package-script"?"/runtime/bun":"/runtime/node"}),undefined,0,command),declared=_.find(invocations,{command}),oraclePrefix=_.concat(row.tool==="tsc"?row.expected.prefix.slice(0,1):[],declared?.prefix??[]);
  expect(oraclePrefix).toEqual(row.expected.prefix);expect({command:actual.command,prefix:actual.prefix,suffix:actual.suffix??[]}).toEqual(row.expected);expect(actual.source).toBe(row.source);
 });
 test("existing execution expression preserves prefix caller suffix and originally requested capacity",()=>{
  const source=ts.createSourceFile("run.ts",readFileSync(new URL("../../🏃️run/🟦️.ts",import.meta.url),"utf8"),ts.ScriptTarget.Latest,true),nodes:ts.VariableDeclaration[]=[];const visit=(node:ts.Node)=>{if(ts.isVariableDeclaration(node))nodes.push(node);ts.forEachChild(node,visit);};visit(source);const expression=(name:string)=>nodes.find(node=>node.name.getText(source)===name)!.initializer!.getText(source);
  for(const row of invocationCases.composition){const actual=runInNewContext(expression("fullArguments"),{resolution:{prefix:row.prefix,suffix:row.suffix},args:row.args}),oracle=_.concat(row.prefix,row.args,row.suffix);expect(JSON.parse(JSON.stringify(actual))).toEqual(row.expected);expect(oracle).toEqual(row.expected);}
  const row=invocationCases.capacity,entry={commands:[row.primary,row.requested],capacity:{operation:"primary",rules:[{command:row.requested,argumentsPrefix:["test"],operation:row.expected,kind:"test"}]}},actual=runInNewContext(ts.transpileModule("(()=>{const requested="+expression("requested")+",capacity="+expression("capacity")+";return capacity.operation;})()",{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText,{options:{command:row.requested},entry,args:row.args});expect(actual).toBe(row.expected);
 });
});

/** 🧭️ The ordinary explicit root discovers the unchanged real CLI execution laws. */
test("ordinary library discovery imports the real owned-command suite",()=>{
 const root=fileURLToPath(new URL("../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts",import.meta.url)),source=ts.createSourceFile(root,readFileSync(root,"utf8"),ts.ScriptTarget.Latest,true),imports=source.statements.filter(ts.isImportDeclaration).flatMap(node=>ts.isStringLiteral(node.moduleSpecifier)?[resolve(dirname(root),node.moduleSpecifier.text)]:[]),expected=resolve(root.slice(0,-invocationCases.discovery.root.length),invocationCases.discovery.suite);
 expect(imports.filter(value=>value===expected)).toHaveLength(1);
 const actual=readFileSync(expected,"utf8");expect(actual).toContain('["exec", "--light", "bunx", "--", "--version"]');expect(actual).toContain("signal outcomes agree with the neutral corpus");
});

/** 🪟️ Existing registered suffix normalization applies before invocation lookup. */
for(const row of invocationCases.projections)test(`bare-command suffix projection ${row.id}`,()=>{
 const base=findToolV1(row.commands[0])!,entry={...base,commands:row.commands,invocations:row.invocations},name=row.requested.replace(/\.(?:exe|cmd|bat)$/iu,""),declared=_.find(row.invocations,{command:name}),oracle={selected:declared?row.commands[0]:name,prefix:declared?.prefix??[]},actual=resolveToolV1(entry,host({platform:"win32",environment:{PATH:"/bin",PATHEXT:".EXE;.CMD;.BAT"},present:[`/bin/${row.selected}.exe`]}),undefined,0,row.requested);
 expect(oracle).toEqual({selected:row.selected,prefix:row.prefix});expect(actual.command).toBe(`/bin/${row.selected}.exe`);expect(actual.prefix).toEqual(row.prefix);
});
