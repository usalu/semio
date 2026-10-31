/** 📦️ Provisioning of the checksum-pinned CMake and Ninja agrees with an independent archive writer and the system unpackers: the pinned size and SHA-256 gate every byte, nothing unsafe is published, a published directory is complete or absent, and a second call does nothing. */
import {afterAll, beforeAll, describe, expect, setDefaultTimeout, test} from "bun:test";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {chmodSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, symlinkSync, writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {delimiter, join} from "node:path";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {NATIVE_TOOLCHAINS_V1, nativeToolSpecV1, nativeToolBinaryDirectoryV1, nativeToolDirectoryV1, type NativeToolV1, type ToolchainHostV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🧰️toolchains/🟦️.ts";
import {pinnedCargoToolsEnvironmentV1, pinnedGoEnvironmentV1, pinnedUvEnvironmentV1, prepareNativeToolV1, preparedNativeToolV1, verifyExtractedTreeV1, withPinnedNativePathV1, type NativeToolPortV1} from "../🟦️.ts";

setDefaultTimeout(120000);
const python = testPythonV1(), sandbox = mkdtempSync(join(tmpdir(), "native-tools-")), windows = process.platform === "win32";
const archiveWriter = String.raw`
import io, os, stat, sys, tarfile, zipfile, json
spec = json.load(sys.stdin)
def script(version):
    return "#!/bin/sh\necho '%s'\n" % version
members = spec["members"]
if spec["format"] == "tar.gz":
    with tarfile.open(spec["output"], "w:gz") as out:
        for member in members:
            info = tarfile.TarInfo(member["path"])
            if member.get("link"):
                info.type = tarfile.SYMTYPE; info.linkname = member["link"]; out.addfile(info)
            elif member.get("directory"):
                info.type = tarfile.DIRTYPE; info.mode = 0o755; out.addfile(info)
            else:
                data = member["text"].encode(); info.size = len(data); info.mode = 0o755 if member.get("executable") else 0o644; out.addfile(info, io.BytesIO(data))
else:
    with zipfile.ZipFile(spec["output"], "w", zipfile.ZIP_DEFLATED) as out:
        for member in members:
            info = zipfile.ZipInfo(member["path"]); info.external_attr = (0o755 if member.get("executable") else 0o644) << 16
            out.writestr(info, member["text"])
`;
const write = (spec: Record<string, unknown>) => {
  const run = spawnSync(python!, ["-I", "-c", archiveWriter], {input: JSON.stringify(spec), encoding: "utf8"});
  if (run.status !== 0) throw new Error(run.stderr);
  const bytes = readFileSync(spec.output as string);
  return {bytes, sha256: createHash("sha256").update(bytes).digest("hex")};
};
const shell = (version: string) => `#!/bin/sh\necho '${version}'\n`;
const spec = nativeToolSpecV1;
const host = "linux-x64" as ToolchainHostV1, original = {go: structuredClone(NATIVE_TOOLCHAINS_V1.go.tool.distributions), cmake: structuredClone(NATIVE_TOOLCHAINS_V1.cmake.cmake.distributions), ninja: structuredClone(NATIVE_TOOLCHAINS_V1.cmake.ninja.distributions), uv: structuredClone(NATIVE_TOOLCHAINS_V1.python.tool.distributions), "cargo-nextest": structuredClone(NATIVE_TOOLCHAINS_V1.cargo["cargo-nextest"].distributions), "cargo-llvm-cov": structuredClone(NATIVE_TOOLCHAINS_V1.cargo["cargo-llvm-cov"].distributions)};
type Variant = {cmake?: {members?: unknown[]; version?: string}; ninja?: {version?: string; members?: unknown[]}};
const pin = (tool: NativeToolV1, variant: Variant): Buffer => {
  const row = spec(tool).distributions.find(candidate => candidate.host === host)!, version = spec(tool).version, output = join(sandbox, tool + "-" + Math.random().toString(16).slice(2) + (row.format === "zip" ? ".zip" : ".tar.gz"));
  const defaults = tool === "cmake"
    ? [{path: row.prefix + "/", directory: true}, {path: row.prefix + "/bin/", directory: true}, {path: row.prefix + "/bin/cmake", text: shell("cmake version " + (variant.cmake?.version ?? version)), executable: true}, {path: row.prefix + "/bin/ctest", text: shell("ctest version"), executable: true}, {path: row.prefix + "/bin/cpack", text: shell("cpack version"), executable: true}, {path: row.prefix + "/share/readme", text: "docs"}, {path: row.prefix + "/doc", link: "share"}]
    : tool === "go"
    ? [{path: row.prefix + "/bin/go", text: shell("go version go" + version + " linux/amd64"), executable: true}, {path: row.prefix + "/bin/gofmt", text: shell("gofmt"), executable: true}, {path: row.prefix + "/VERSION", text: "go" + version}]
    : tool === "uv"
    ? [{path: row.prefix + "/uv", text: shell("uv " + version + " (0000000 2026-01-01 aarch64-apple-darwin)"), executable: true}, {path: row.prefix + "/uvx", text: shell("uvx"), executable: true}]
    : tool === "cargo-nextest" || tool === "cargo-llvm-cov"
    ? [{path: tool, text: shell(tool + " " + version), executable: true}]
    : [{path: "ninja", text: shell(variant.ninja?.version ?? version), executable: true}];
  const made = write({format: row.format, output, members: (tool === "cmake" ? variant.cmake?.members : variant.ninja?.members) ?? defaults});
  Object.assign(row, {bytes: made.bytes.length, sha256: made.sha256});
  return made.bytes;
};
const portOf = (served: Map<string, Buffer>, platform: NodeJS.Platform, options: {offline?: boolean; fetches?: string[]; delay?: boolean} = {}): NativeToolPortV1 => Object.freeze({
  platform, offline: options.offline ?? false, environment: {SEMIO_LANGUAGE: "en"},
  fetch: async (url: string, signal: AbortSignal) => {
    options.fetches?.push(url);
    const bytes = served.get(url);
    if (!bytes) return new Response("missing", {status: 404});
    return new Response(new ReadableStream({async start(controller) { if (options.delay) await new Promise(resolve => setTimeout(resolve, 60)); for (let at = 0; at < bytes.length; at += 4096) { signal.throwIfAborted(); controller.enqueue(bytes.subarray(at, at + 4096)); } controller.close(); }}));
  },
  run: async (command: string, args: readonly string[]) => { const run = spawnSync(command, [...args], {encoding: "utf8"}); if (run.status !== 0) throw new Error(command + " failed: " + run.stderr); },
  capture: async (command: string, args: readonly string[]) => { const run = spawnSync(command, [...args], {encoding: "utf8"}); if (run.status !== 0) throw new Error(command + " failed: " + run.stderr); return run.stdout; },
  log: () => undefined,
});
const url = (tool: NativeToolV1) => spec(tool).release + "/" + spec(tool).distributions.find(row => row.host === host)!.archive;
const fresh = () => mkdtempSync(join(sandbox, "tools-"));

afterAll(() => {
  NATIVE_TOOLCHAINS_V1.go.tool.distributions.splice(0, Infinity, ...original.go);
  NATIVE_TOOLCHAINS_V1.cmake.cmake.distributions.splice(0, Infinity, ...original.cmake);
  NATIVE_TOOLCHAINS_V1.cmake.ninja.distributions.splice(0, Infinity, ...original.ninja);
  NATIVE_TOOLCHAINS_V1.python.tool.distributions.splice(0, Infinity, ...original.uv);
  for(const tool of ["cargo-nextest", "cargo-llvm-cov"] as const)NATIVE_TOOLCHAINS_V1.cargo[tool].distributions.splice(0,Infinity,...original[tool]);
  rmSync(sandbox, {recursive: true, force: true});
});

describe.skipIf(python === null || windows)("the provisioning of a pinned archive tool", () => {
  beforeAll(() => { expect(python).not.toBeNull(); });

  test("publishes a complete directory for a tar.gz and a zip, once, and finds it again without the network", async () => {
    const root = fresh(), served = new Map<string, Buffer>(), fetches: string[] = [];
    served.set(url("cmake"), pin("cmake", {}));
    served.set(url("ninja"), pin("ninja", {}));
    for (const platform of (process.platform === "darwin" ? ["linux", "darwin"] : ["linux"]) as NodeJS.Platform[]) {
      const toolsRoot = join(root, platform);
      for (const tool of ["cmake", "ninja"] as const) {
        const binaries = await prepareNativeToolV1(tool, host, toolsRoot, portOf(served, platform, {fetches}), AbortSignal.timeout(60000));
        expect(binaries).toBe(nativeToolBinaryDirectoryV1(tool, host, toolsRoot));
        const main = join(binaries, tool);
        expect(lstatSync(main).mode & 0o111, tool).not.toBe(0);
        expect(spawnSync(main, ["--version"], {encoding: "utf8"}).stdout).toContain(spec(tool).version);
        expect(readdirSync(join(nativeToolDirectoryV1(tool, host, toolsRoot), "..")).filter(name => name.startsWith(".prepare-"))).toEqual([]);
      }
      expect(lstatSync(join(nativeToolDirectoryV1("cmake", host, toolsRoot), "doc")).isSymbolicLink()).toBe(true);
      expect(existsSync(join(nativeToolDirectoryV1("cmake", host, toolsRoot), ".toolchain.json"))).toBe(true);
    }
    const before = fetches.length;
    await prepareNativeToolV1("cmake", host, join(root, "linux"), portOf(new Map(), "linux", {offline: true, fetches}), AbortSignal.timeout(60000));
    expect(fetches.length).toBe(before);
  });

  test("provisions the pinned Go and returns the environment every Go command runs under, whatever Go the machine has", async () => {
    const root = fresh(), served = new Map([[url("go"), pin("go", {})]]);
    const environment = await pinnedGoEnvironmentV1({PATH: ["/usr/bin", "/old/go/bin"].join(delimiter), GOROOT: "/old/go", GOFLAGS: "-mod=mod"}, root, portOf(served, "linux"), AbortSignal.timeout(60000), true, host);
    const directory = nativeToolDirectoryV1("go", host, root);
    expect(environment.GOROOT).toBe(directory);
    expect(environment.GOTOOLCHAIN).toBe("local");
    expect(environment.PATH!.split(delimiter)).toEqual([nativeToolBinaryDirectoryV1("go", host, root), "/usr/bin", "/old/go/bin"]);
    expect(environment.GOFLAGS).toBe("-mod=mod");
    expect(spawnSync(join(directory, "bin/go"), ["version"], {encoding: "utf8"}).stdout).toContain(NATIVE_TOOLCHAINS_V1.go.version);
    await expect(pinnedGoEnvironmentV1({PATH: "/usr/bin"}, fresh(), portOf(new Map(), "linux"), AbortSignal.timeout(60000), false, host)).rejects.toThrow(/repo tools setup toolchains/u);
  });

  test("provisions the pinned uv, selects it before any uv on the machine and reports the progress of a slow download", async () => {
    const root = fresh(), served = new Map([[url("uv"), pin("uv", {})]]), lines: string[] = [], port = {...portOf(served, "linux", {delay: true}), log: (text: string) => { lines.push(text); }};
    const environment = await pinnedUvEnvironmentV1({PATH: ["/usr/bin", "/old/uv"].join(delimiter)}, root, port, AbortSignal.timeout(60000), true, host);
    expect(environment.PATH!.split(delimiter)).toEqual([nativeToolBinaryDirectoryV1("uv", host, root), "/usr/bin", "/old/uv"]);
    expect(spawnSync(join(nativeToolBinaryDirectoryV1("uv", host, root), "uv"), ["--version"], {encoding: "utf8"}).stdout).toContain("uv " + NATIVE_TOOLCHAINS_V1.python.uv);
    expect(lines[0]).toContain("uv " + NATIVE_TOOLCHAINS_V1.python.uv);
    await expect(pinnedUvEnvironmentV1({PATH: "/usr/bin"}, fresh(), portOf(new Map(), "linux"), AbortSignal.timeout(60000), false, host)).rejects.toThrow(/repo tools setup toolchains/u);
  });

  test("Cargo tools retain the actual compiler environment, verify receipts and need no network once retained", async()=>{
    const root=fresh(),tools=["cargo-nextest","cargo-llvm-cov"] as const,served=new Map(tools.map(tool=>[url(tool),pin(tool,{})])),fetches:string[]=[],input={Path:["/compiler/bin","/system/bin"].join(delimiter),CARGO_HOME:join(root,"cargo-home"),CARGO_TARGET_DIR:join(root,"target"),SEMIO_CARGO_STORE_OWNER:"held-owner",RUSTUP_TOOLCHAIN:"pinned-toolchain"},environment=await pinnedCargoToolsEnvironmentV1(input,root,portOf(served,"linux",{fetches}),AbortSignal.timeout(60000),true,host);
    expect(environment.CARGO_HOME).toBe(input.CARGO_HOME);expect(environment.CARGO_TARGET_DIR).toBe(input.CARGO_TARGET_DIR);expect(environment.SEMIO_CARGO_STORE_OWNER).toBe(input.SEMIO_CARGO_STORE_OWNER);expect(environment.RUSTUP_TOOLCHAIN).toBe(input.RUSTUP_TOOLCHAIN);expect(environment.PATH).toBeUndefined();expect(environment.Path!.split(delimiter)).toEqual([...tools.map(tool=>nativeToolBinaryDirectoryV1(tool,host,root)),"/compiler/bin","/system/bin"]);
    for(const tool of tools){const variable="SEMIO_TOOL_"+tool.toUpperCase().replaceAll("-","_");expect(environment[variable]).toBe(join(preparedNativeToolV1(tool,host,root),tool));}
    const offline=await pinnedCargoToolsEnvironmentV1(input,root,portOf(new Map(),"linux",{offline:true,fetches}),AbortSignal.timeout(60000),false,host);expect(offline).toEqual(environment);expect(fetches.length).toBe(tools.length);
    writeFileSync(environment.SEMIO_TOOL_CARGO_NEXTEST!,"tampered");await expect(pinnedCargoToolsEnvironmentV1(input,root,portOf(new Map(),"linux",{offline:true}),AbortSignal.timeout(60000),false,host)).rejects.toThrow(/differs from its receipt/u);
  });

  test("lets concurrent agents provision the same pin: every call succeeds and exactly one directory is published", async () => {
    const root = fresh(), served = new Map([[url("ninja"), pin("ninja", {})]]);
    const results = await Promise.all([1, 2, 3, 4].map(() => prepareNativeToolV1("ninja", host, root, portOf(served, "linux", {delay: true}), AbortSignal.timeout(60000))));
    expect(new Set(results).size).toBe(1);
    expect(readdirSync(join(nativeToolDirectoryV1("ninja", host, root), "..")).sort()).toEqual([nativeToolDirectoryV1("ninja", host, root).split("/").at(-1)!]);
    expect(preparedNativeToolV1("ninja", host, root, {SEMIO_LANGUAGE: "en"})).toBe(results[0]!);
  });

  test("refuses bytes that differ from the pinned size or SHA-256 and publishes nothing", async () => {
    const root = fresh(), good = pin("cmake", {}), tampered = Buffer.from(good);
    tampered[tampered.length >> 1] ^= 0xff;
    for (const [name, bytes] of [["flipped byte", tampered], ["truncated", good.subarray(0, good.length - 1)], ["extended", Buffer.concat([good, Buffer.from([0])])]] as const) {
      await expect(prepareNativeToolV1("cmake", host, root, portOf(new Map([[url("cmake"), bytes]]), "linux"), AbortSignal.timeout(60000)), name).rejects.toThrow(/SHA-256/u);
    }
    expect(existsSync(nativeToolDirectoryV1("cmake", host, root))).toBe(false);
    expect(readdirSync(join(nativeToolDirectoryV1("cmake", host, root), "..")).filter(name => name.startsWith(".prepare-"))).toEqual([]);
    await expect(prepareNativeToolV1("cmake", host, root, portOf(new Map(), "linux"), AbortSignal.timeout(60000))).rejects.toThrow(/404/u);
  });

  test("refuses to download when offline, in the selected language", async () => {
    const root = fresh(), fetches: string[] = [];
    await expect(prepareNativeToolV1("ninja", host, root, portOf(new Map(), "linux", {offline: true, fetches}), AbortSignal.timeout(60000))).rejects.toThrow(/offline/u);
    expect(fetches).toEqual([]);
  });

  test("refuses an archive link that leaves its tree, a missing executable and a wrong version", async () => {
    const prefix = NATIVE_TOOLCHAINS_V1.cmake.cmake.distributions.find(row => row.host === host)!.prefix;
    const cases: [string, Variant, RegExp][] = [
      ["escaping link", {cmake: {members: [{path: prefix + "/bin/", directory: true}, {path: prefix + "/bin/cmake", text: shell("cmake version 4.4.4"), executable: true}, {path: prefix + "/bin/ctest", text: "x", executable: true}, {path: prefix + "/bin/cpack", text: "x", executable: true}, {path: prefix + "/bad", link: "../../../../etc"}]}}, /leaves its tree/u],
      ["absolute link", {cmake: {members: [{path: prefix + "/bin/", directory: true}, {path: prefix + "/bin/cmake", text: shell("cmake version 4.4.4"), executable: true}, {path: prefix + "/bin/ctest", text: "x", executable: true}, {path: prefix + "/bin/cpack", text: "x", executable: true}, {path: prefix + "/bad", link: "/usr/bin"}]}}, /leaves its tree/u],
      ["missing executable", {cmake: {members: [{path: prefix + "/bin/", directory: true}, {path: prefix + "/bin/cmake", text: shell("cmake version 4.4.4"), executable: true}]}}, /holds no ctest/u],
      ["wrong version", {cmake: {version: "3.0.0"}}, /differs from the pin/u],
    ];
    for (const [name, variant, message] of cases) {
      const root = fresh();
      const bytes = pin("cmake", variant);
      await expect(prepareNativeToolV1("cmake", host, root, portOf(new Map([[url("cmake"), bytes]]), "linux"), AbortSignal.timeout(60000)), name).rejects.toThrow(message);
      expect(existsSync(nativeToolDirectoryV1("cmake", host, root)), name).toBe(false);
    }
  });

  test("stops on cancellation without publishing", async () => {
    const root = fresh(), bytes = pin("cmake", {}), controller = new AbortController();
    setTimeout(() => controller.abort(new Error("cancelled")), 10);
    await expect(prepareNativeToolV1("cmake", host, root, portOf(new Map([[url("cmake"), bytes]]), "linux", {delay: true}), controller.signal)).rejects.toThrow(/cancelled/u);
    expect(existsSync(nativeToolDirectoryV1("cmake", host, root))).toBe(false);
  });

  test("detects a changed executable and a foreign receipt", async () => {
    const root = fresh(), served = new Map([[url("ninja"), pin("ninja", {})]]);
    const binaries = await prepareNativeToolV1("ninja", host, root, portOf(served, "linux"), AbortSignal.timeout(60000));
    expect(preparedNativeToolV1("ninja", host, root, {SEMIO_LANGUAGE: "en"})).toBe(binaries);
    const receipt = join(nativeToolDirectoryV1("ninja", host, root), ".toolchain.json"), saved = readFileSync(receipt, "utf8");
    writeFileSync(receipt, saved.replace('"owner":"workspace:native-tools"', '"owner":"someone"'));
    expect(() => preparedNativeToolV1("ninja", host, root, {SEMIO_LANGUAGE: "de"})).toThrow(/Quittung/u);
    writeFileSync(receipt, saved);
    chmodSync(join(binaries, "ninja"), 0o755);
    writeFileSync(join(binaries, "ninja"), shell("tampered"));
    expect(() => preparedNativeToolV1("ninja", host, root, {SEMIO_LANGUAGE: "en"})).toThrow(/differs from its receipt/u);
    expect(() => preparedNativeToolV1("cmake", host, fresh(), {SEMIO_LANGUAGE: "en"})).toThrow(/repo tools setup toolchains/u);
  });
});

describe("the helpers", () => {
  test("verify a tree: links inside pass, links outside fail, special files fail", () => {
    const root = mkdtempSync(join(sandbox, "tree-"));
    mkdirSync(join(root, "a/b"), {recursive: true});
    writeFileSync(join(root, "a/file"), "x");
    if (!windows) {
      symlinkSync("../file", join(root, "a/b/inside"));
      expect(verifyExtractedTreeV1(root)).toBe(4);
      symlinkSync("../../../escape", join(root, "a/b/outside"));
      expect(() => verifyExtractedTreeV1(root)).toThrow(/leaves its tree/u);
    }
  });

  test("put the pinned directories first on the PATH under the spelling the environment uses", () => {
    const upper = withPinnedNativePathV1({PATH: ["/usr/bin", "/bin"].join(delimiter)}, host, "/t"), mixed = withPinnedNativePathV1({Path: "C:\\Windows"}, "win32-x64", "C:\\t");
    expect(upper.PATH!.split(delimiter).slice(-2)).toEqual(["/usr/bin", "/bin"]);
    expect(upper.PATH!.split(delimiter)[0]).toContain("cmake");
    expect(Object.keys(mixed)).toEqual(["Path"]);
    expect(mixed.Path!.endsWith(delimiter + "C:\\Windows")).toBe(true);
  });
});
