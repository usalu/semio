/** 🧰️ A task of the Python, .NET or CMake class is keyed by the pin of its toolchain exactly as the neutral contract says, and only such a task is: the adapter's action key moves with the digest for members and never for any other task. */
import {expect, test} from "bun:test";
import {createHash, randomUUID} from "node:crypto";
import {readFile} from "node:fs/promises";
import {join} from "node:path";
import Ajv from "ajv";
import ts from "typescript";
import {NATIVE_TOOLCHAINS_V1, toolchainIdentityV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🧰️toolchains/🟦️.ts";
import * as native from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type: "json"};
import table from "../🔣️.json" with {type: "json"};
import corpus from "./🔣️.json" with {type: "json"};

test("the class table is closed and every class names a toolchain of the contract", () => {
  const validate = new Ajv({strict: false}).compile(schema);
  expect(validate(table), JSON.stringify(validate.errors)).toBe(true);
  expect(validate({...table, classes: [{kind: "go", family: "go"}]})).toBe(false);
  expect(validate({...table, classes: [{kind: "python"}]})).toBe(false);
  expect(new Set(table.classes.map(row => row.kind))).toEqual(new Set(["python", "dotnet", "cmake"]));
});

test("a task is a member exactly when the family rules or the driver command say so", () => {
  for (const row of corpus.classes) expect<unknown>(native.bazelNativeToolchainClassV1({project: row.project, target: row.target, root: row.root, definition: {execution: {kind: row.kind, ...("command" in row ? {command: row.command} : {})}}}), row.name).toBe(row.expected ?? undefined);
});

test("the digest is the identity of the contract, keyed by the project interpreter and by the declared variables only", () => {
  const styling = NATIVE_TOOLCHAINS_V1.python.projects[1]!, oracle = NATIVE_TOOLCHAINS_V1.python.projects[2]!;
  const digest = (root: string, environment: Record<string, string> = {}) => native.bazelNativeToolchainDigestV1("python", environment, root, "linux", "x64");
  expect(digest(styling.root)).toBe(toolchainIdentityV1("python", {host: "linux-x64", project: styling.id}).digest);
  expect(digest(oracle.root)).toBe(toolchainIdentityV1("python", {host: "linux-x64", project: oracle.id}).digest);
  expect(digest(oracle.root)).not.toBe(digest(styling.root));
  expect(digest(styling.root, {PATH: "/anywhere", HOME: "/home/x"})).toBe(digest(styling.root));
  expect(digest(styling.root, {PYTHONHASHSEED: "7"})).not.toBe(digest(styling.root));
  expect(digest("elsewhere")).toBe(toolchainIdentityV1("python", {host: "linux-x64"}).digest);
  for (const kind of ["dotnet", "cmake"] as const) {
    expect(native.bazelNativeToolchainDigestV1(kind, {}, ".", "darwin", "arm64")).toBe(toolchainIdentityV1(kind, {host: "darwin-arm64"}).digest);
    expect(native.bazelNativeToolchainDigestV1(kind, {}, ".", "darwin", "arm64")).not.toBe(native.bazelNativeToolchainDigestV1(kind, {}, ".", "linux", "x64"));
  }
  expect(native.bazelNativeToolchainDigestV1("cmake", {CXX: "clang++"}, ".", "linux", "x64")).not.toBe(native.bazelNativeToolchainDigestV1("cmake", {}, ".", "linux", "x64"));
});

test("the adapter's action key takes the native toolchain digest for members only", async () => {
  const source = await readFile(new URL("../../📜️script.ts", import.meta.url), "utf8"), tree = ts.createSourceFile("bazel.ts", source, ts.ScriptTarget.Latest, true), node = tree.statements.find(row => ts.isFunctionDeclaration(row) && row.name?.text === "taskFingerprint");
  if (!node) throw Error("Bazel fingerprint implementation required");
  const code = ts.transpileModule(node.getText(tree).replace(/^export /u, "") + ";return taskFingerprint;", {compilerOptions: {target: ts.ScriptTarget.ES2022}}).outputText;
  const fingerprint = async (row: (typeof corpus.classes)[number], digest: string) => {
    const bindings = {createHash, randomUUID, join, bazelCargoClassV1: () => undefined, bazelCargoToolchainDigestV1: async () => "cargo", bazelGoClassV1: () => false, bazelGoToolchainDigestV1: async () => "go", bazelNativeToolchainClassV1: native.bazelNativeToolchainClassV1, bazelNativeToolchainDigestV1: () => digest, process: {...process, env: {}}, bazelTaskInputsV1: () => [], issueBazelTaskEnvironmentV1: () => ({SEMIO_BUILD_MODE: "prototype"}), bazelTaskBaseEnvironmentV1: () => ({}), checkScriptInvocation: () => undefined, bazelSourceIdentityV1: async () => "source", captureOwnedCommandV1: async () => ""};
    const run = Function(...Object.keys(bindings), code)(...Object.values(bindings)), definition = {execution: {kind: row.kind, ...("command" in row ? {command: row.command} : {})}};
    return run("/root", {}, {id: "p:" + row.target, project: row.project, target: row.target, root: row.root, args: [], definition}, ["a.txt"], new Map(), {control: {signal: new AbortController().signal}}, {}, true, "implementation", undefined) as Promise<string>;
  };
  for (const row of corpus.classes.filter(candidate => candidate.kind === "command")) {
    const moved = (await fingerprint(row, "one")) !== (await fingerprint(row, "two"));
    expect({name: row.name, moved}).toEqual({name: row.name, moved: row.expected !== null});
    expect(await fingerprint(row, "one")).toBe(await fingerprint(row, "one"));
  }
});
