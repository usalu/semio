import { describe, expect, test } from "bun:test";
import { minimatch } from "minimatch";
import Ajv from "ajv";
import { sha256 } from "@noble/hashes/sha2.js";
import { basename, dirname } from "node:path";
import schema from "../../🛠️native-compiler/🧬️schema/🔣️.json" with { type: "json" };
import { findToolV1, resolveToolV1, routeRegisteredCommandV1, runToolSyncV1, type ToolHostV1 } from "../../🟦️.ts";
import vectors from "../🧫️fixtures/🛠️native-compiler/🔣️.json" with { type: "json" };
import { selectNativeCompilerV1, nativeCompilerEnvironmentV1, nativeCompilerArgumentsV1, nativeCompilerShimSourceV1, nativeCompilerSourceClosureV1, nativeCompilerElfSourcesV1, nativeCompilerSourcePlanV1, SYSTEM_NATIVE_COMPILER_SOURCE_PORT_V1 } from "../../🛠️native-compiler/🟦️.ts";

describe("native compiler selection", () => {
  for (const name of vectors.explicitRoots) test("explicit " + name + " declines SDK defaults", () => {
    const command = "/home/u/.local/share/flatpak/runtime/org.freedesktop.Sdk/x86_64/25.08/active/files/bin/gcc";
    expect(selectNativeCompilerV1({ platform: "linux", architecture: "x64", home: "/home/u", environment: { PATH: "/empty", [name]: "/chosen root" }, isExecutable: path => path === command, list: () => ["25.08"], canonical: path => path })).toBeNull();
  });
  for (const row of vectors.selections) test(row.name, () => {
    const present = new Set(row.present), machine = row.architecture === "x64" ? "x86_64" : "aarch64";
    const selected = selectNativeCompilerV1({ platform: row.platform as NodeJS.Platform, architecture: row.architecture, home: row.home, environment: { PATH: row.path }, isExecutable: path => present.has(path), list: path => path.startsWith(row.home + "/.local/") ? row.branches : [], canonical: path => path });
    expect(selected?.command ?? null).toBe(row.expected);
    if (row.expected) {
      expect(new Ajv({ strict: false }).compile({ ...schema, $ref: "#/definitions/Selection" })(selected)).toBe(true);
      const pattern = `${row.home}/.local/share/flatpak/runtime/org.freedesktop.Sdk/${machine}/*/active/files/bin/gcc`;
      const oracle = row.present.filter(path => minimatch(path, pattern)).sort((a, b) => b.localeCompare(a, "en", { numeric: true }));
      expect(selected!.command).toBe(oracle[0]);
    }
  });
  test("registry fallback appends defaults after caller arguments", () => {
    const command = "/home/u/.local/share/flatpak/runtime/org.freedesktop.Sdk/x86_64/25.08/active/files/bin/gcc";
    const host: ToolHostV1 = { platform: "linux", architecture: "x64", home: "/home/u", environment: { PATH: "/empty" }, root: "/repo", execPath: process.execPath, runtimeCommand: "bun", isExecutable: path => path === command, exists: path => path === command, list: path => path.startsWith("/home/u/") ? ["25.08"] : [], canonical: path => path, pins: { read: () => undefined } };
    const resolution = resolveToolV1(findToolV1("cc")!, host);
    expect(resolution.command).toBe(command);
    expect(resolution.prefix).toEqual([]);
    expect(["-L/custom libraries", ...resolution.suffix!].indexOf("-L/custom libraries")).toBeLessThan(["-L/custom libraries", ...resolution.suffix!].indexOf("-L/usr/lib"));
  });
  test("same-process compiler routing follows changed explicit environment", () => {
    const first = process.execPath, second = dirname(first) + "/./" + basename(first);
    expect(routeRegisteredCommandV1("cc", ["--version"], process.cwd(), { ...process.env, SEMIO_TOOL_CC: first }).command).toBe(first);
    expect(routeRegisteredCommandV1("cc", ["--version"], process.cwd(), { ...process.env, SEMIO_TOOL_CC: second }).command).toBe(second);
  });
});

describe("native compiler environment precedence", () => {
  const context = (target: string, configuredLinker = false) => ({ target, host: target, configuredLinker, configuredHostLinker: configuredLinker, configuredEnvironment: [] as string[] });
  const selection = { version: 1 as const, command: "/home/Example User/sdk/bin/gcc", root: "/home/Example User/sdk", machine: "x86_64" as const };
  test("configured compiler and root environment remains Cargo's explicit choice", () => {
    const environment = { PATH: "/user/bin" };
    const compiler = nativeCompilerEnvironmentV1(environment, selection, { ...context("x86_64-unknown-linux-gnu"), configuredEnvironment: ["CC"] });
    expect(compiler.CC).toBeUndefined();
    expect(compiler.PATH).toBe("/home/Example User/sdk/bin:/user/bin");
    const roots = nativeCompilerEnvironmentV1(environment, selection, { ...context("x86_64-unknown-linux-gnu"), configuredEnvironment: ["LIBRARY_PATH"] });
    expect(roots).toBe(environment);
  });
  test("explicit empty roots preserve the exact original environment", () => {
    const environment = { LIBRARY_PATH: "" };
    expect(nativeCompilerEnvironmentV1(environment, selection, context("x86_64-unknown-linux-gnu"))).toBe(environment);
  });
  for (const row of vectors.precedence) test(row.name, () => {
    const environment: Record<string, string | undefined> = { ...row.environment, RUSTFLAGS: "--cfg example", CFLAGS: "-O2" };
    const actual = nativeCompilerEnvironmentV1(environment, selection, context("x86_64-unknown-linux-gnu", row.configuredLinker));
    for (const [name, value] of Object.entries(environment)) expect(actual[name]).toBe(value);
    expect(actual.CC).toBe(row.keepC ? environment.CC : selection.command);
    expect(actual.PATH).toBe(row.keepRust ? environment.PATH : "/home/Example User/sdk/bin");
    expect(actual.C_INCLUDE_PATH).toBe(row.environment.C_INCLUDE_PATH);
  });
  test("no selection and cross compilation preserve exact environment", () => {
    const environment = { CC: "chosen", RUSTFLAGS: "--cfg example" };
    expect(nativeCompilerEnvironmentV1(environment, null, context("x86_64-unknown-linux-gnu"))).toBe(environment);
    expect(nativeCompilerEnvironmentV1(environment, selection, context("aarch64-unknown-linux-gnu"))).toBe(environment);
  });
  test("HOST_CC and target compiler choices remain explicit", () => {
    for (const name of ["HOST_CC", "TARGET_CC", "CC_x86_64-unknown-linux-gnu", "CC_x86_64_unknown_linux_gnu"]) {
      const environment = { [name]: "/chosen compiler" };
      expect(nativeCompilerEnvironmentV1(environment, selection, context("x86_64-unknown-linux-gnu")).CC).toBeUndefined();
    }
  });
  test("explicit Rust linker flags preserve PATH and flags byte for byte", () => {
    for (const RUSTFLAGS of ["-Clinker=/chosen linker", "-C linker=/chosen linker", "--codegen=linker=/chosen linker"]) {
      const environment = { PATH: "/user/bin", RUSTFLAGS };
      expect(nativeCompilerEnvironmentV1(environment, selection, context("x86_64-unknown-linux-gnu"))).toMatchObject(environment);
    }
  });
  for (const row of vectors.crossCompilation) test(row.name, () => {
    const actual = nativeCompilerEnvironmentV1(row.environment, selection, { target: "wasm32-wasip2", host: "x86_64-unknown-linux-gnu", configuredLinker: row.configuredLinker, configuredHostLinker: row.configuredHostLinker, configuredEnvironment: [] });
    for (const [name, value] of Object.entries(row.environment)) if (name !== "PATH") expect(actual[name]).toBe(value);
    for (const [name, value] of Object.entries(row.expected)) expect(actual[name]).toBe(value === null ? undefined : value === "__NATIVE_CC__" ? selection.command : value === "__NATIVE_PATH__" ? "/home/Example User/sdk/bin:/user/bin" : value);
  });
  test("the shell driver preserves spaces and caller library precedence, independently parsed by Python", () => {
    const text = nativeCompilerShimSourceV1(selection), line = text.split("\n")[1]!;
    const oracle = runToolSyncV1("python", ["-c", "import json,shlex,sys;print(json.dumps(shlex.split(sys.argv[1])))", line], { root: process.cwd(), trusted: true, encoding: "utf8", admission: false });
    expect(JSON.parse(oracle.stdout)).toEqual(["exec", selection.command, "$@", ...nativeCompilerArgumentsV1(selection)]);
    expect(line.indexOf('"$@"')).toBeLessThan(line.indexOf("'-L/usr/lib'"));
  });
});

describe("native compiler source closure", () => {
  const context = { check: () => {}, yield: async () => {} };
  test("the declared executed sources include the shim interpreter and native tools", () => {
    const plan = nativeCompilerSourcePlanV1({ version: 1, command: "/sdk/bin/gcc", root: "/sdk", machine: "x86_64" }, {}, [], { ...SYSTEM_NATIVE_COMPILER_SOURCE_PORT_V1, list: () => [] });
    for (const path of vectors.executedTools) expect(plan.roots).toContain(path);
  });
  const snapshot = (table: Record<string, string>) => nativeCompilerSourceClosureV1(["/root"], context, {
    inspect: path => ({ kind: path in table ? "file" : "directory", bytes: path in table ? Buffer.byteLength(table[path]!) : 0 }),
    list: path => Object.keys(table).filter(name => name.startsWith(path + "/")).map(name => name.slice(path.length + 1)),
    read: path => Buffer.from(table[path]!), target: () => "", canonical: path => path,
  });
  test("directory and byte identity match an independent SHA256 implementation", async () => {
    const actual = await snapshot({ "/root/header.h": "abc" });
    const hex = (value: string) => Buffer.from(sha256(Buffer.from(value))).toString("hex");
    const manifest = JSON.stringify({ path: "/root", kind: "directory", names: ["header.h"] }) + "\n" + JSON.stringify({ path: "/root/header.h", kind: "file", bytes: 3, digest: hex("abc") }) + "\n";
    expect(actual.digest).toBe(hex(manifest));
    expect(actual.files).toBe(1);
    expect(actual.bytes).toBe(3);
    expect((await snapshot({ "/root/header.h": "abd" })).digest).not.toBe(actual.digest);
    expect((await snapshot({ "/root/header.h": "abc", "/root/new.h": "" })).digest).not.toBe(actual.digest);
  });
  test("oversized source extents fail before allocating their bytes", async () => {
    let read = false;
    await expect(nativeCompilerSourceClosureV1(["/large"], context, { inspect: () => ({ kind: "file", bytes: schema.definitions.Protocol.const.maximumFileBytes + 1 }), read: () => { read = true; return Buffer.alloc(0); }, list: () => [], target: () => "", canonical: path => path })).rejects.toThrow("bound");
    expect(read).toBe(false);
  });
  test("source traversal honors cancellation before reading", async () => {
    await expect(nativeCompilerSourceClosureV1(["/root"], { check: () => { throw Error("cancelled"); }, yield: async () => {} })).rejects.toThrow("cancelled");
  });
  test("a changed symlink target changes the actual source identity", async () => {
    const capture = (target: string) => nativeCompilerSourceClosureV1(["/link"], context, { inspect: path => ({ kind: path === "/link" ? "symlink" : "file", bytes: 3 }), list: () => [], read: () => Buffer.from("abc"), target: () => target, canonical: () => target });
    expect((await capture("/first.h")).digest).not.toBe((await capture("/second.h")).digest);
  });
  test("ELF dependencies include the loader and honor user, RUNPATH, then system lookup", () => {
    const bytes = Buffer.alloc(512), strings = "\0libexample.so\0$ORIGIN/private\0";
    bytes.writeUInt32BE(0x7f454c46, 0); bytes[4] = 2; bytes[5] = 1; bytes.writeUInt16LE(3, 16); bytes.writeBigUInt64LE(64n, 32); bytes.writeUInt16LE(56, 54); bytes.writeUInt16LE(3, 56);
    bytes.writeUInt32LE(1, 64); bytes.writeBigUInt64LE(512n, 96);
    bytes.writeUInt32LE(2, 120); bytes.writeBigUInt64LE(256n, 128); bytes.writeBigUInt64LE(80n, 152);
    bytes.writeUInt32LE(3, 176); bytes.writeBigUInt64LE(400n, 184); bytes.writeBigUInt64LE(16n, 208);
    for (const [index, tag, value] of [[0, 5, 352], [1, 10, strings.length], [2, 1, 1], [3, 29, strings.indexOf("$ORIGIN")]]) { bytes.writeBigInt64LE(BigInt(tag!), 256 + index! * 16); bytes.writeBigUInt64LE(BigInt(value!), 264 + index! * 16); }
    bytes.write(strings, 352); bytes.write("/lib/loader.so\0", 400);
    const all = new Set(["/user/lib/libexample.so", "/sdk/bin/private/libexample.so", "/system/lib/libexample.so"]);
    expect(nativeCompilerElfSourcesV1(bytes, "/sdk/bin/gcc", ["/user/lib"], path => all.has(path), ["/system/lib"])).toEqual(["/lib/loader.so", "/user/lib/libexample.so"]);
    all.delete("/user/lib/libexample.so");
    expect(nativeCompilerElfSourcesV1(bytes, "/sdk/bin/gcc", ["/user/lib"], path => all.has(path), ["/system/lib"])).toEqual(["/lib/loader.so", "/sdk/bin/private/libexample.so"]);
    all.clear();
    expect(() => nativeCompilerElfSourcesV1(bytes, "/sdk/bin/gcc", [], path => all.has(path), [])).toThrow("unavailable");
  });
});
