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
