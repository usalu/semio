import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, test } from "../../../../🧪️test/🟦️.ts";
import { RuntimeCapabilityMissingV1 } from "../../../../🗣️messages/🟦️.ts";
import messages from "../../../../🧬️schema/🗣️messages.json" with { type: "json" };
import schema from "../../🧬️schema/🔣️.json" with { type: "json" };
import { ensureNodeNativeAddonV1, loadNodeNativeCoreV1, nodeNativeCacheRootV1, nodeNativeCompilerArgumentsV1, nodeNativeCompilerV1, nodeNativeTargetV1, planNodeNativeAddonV1, type NodeNativeHostV1, type NodeNativePortsV1, type NodeNativeRunV1 } from "../🟦️.ts";

const protocol = schema.definitions.Protocol.const;
const source = Buffer.from("fn main() {}");
const host = (overrides: Partial<NodeNativeHostV1> = {}): NodeNativeHostV1 => ({ platform: "linux", arch: "x64", env: {}, home: "/home/dev", source: "/repo/native/rs", ...overrides });

type Script = { runs?: NodeNativeRunV1[]; lockHeld?: number; lockAge?: number; renameFails?: boolean; compiler?: string | undefined; appearsAfterSleeps?: number; clock?: number };
function ports(overrides: Partial<NodeNativeHostV1> = {}, script: Script = {}) {
  const files = new Set<string>(), log: string[] = [], env: Record<string, string | undefined>[] = [];
  let sleeps = 0, held = script.lockHeld ?? 0, time = 0;
  const runs = [...(script.runs ?? [{ status: 0, stderr: "" }])];
  const value: NodeNativePortsV1 = {
    host: host(overrides),
    read: () => source,
    exists: path => files.has(path),
    mkdir: path => { log.push(`mkdir ${path}`); files.add(path); },
    lock: path => { if (held > 0) { held--; return false; } log.push(`lock ${path}`); files.add(path); return true; },
    age: () => script.lockAge,
    remove: path => { log.push(`remove ${path}`); for (const file of [...files]) if (file === path || file.startsWith(`${path}/`)) files.delete(file); },
    rename: (from, to) => { if (script.renameFails) { files.add(join(to, protocol.addon.file)); throw new Error("EEXIST"); } log.push(`rename ${from} ${to}`); files.delete(from); files.add(to); files.add(join(to, protocol.addon.file)); },
    compiler: () => "compiler" in script ? script.compiler : "/cargo/bin/rustc",
    run: (program, arguments_, options) => { log.push(`run ${program}`); env.push({ ...options.env }); const output = arguments_[arguments_.indexOf("-o") + 1]!; const result = runs.shift() ?? { status: 0, stderr: "" }; if (result.status === 0) files.add(output); return result; },
    sleep: () => { sleeps++; if (script.appearsAfterSleeps !== undefined && sleeps >= script.appearsAfterSleeps) files.add(plan().file); },
    now: () => (time += script.clock ?? 1),
    staging: () => "t1",
    notice: text => { log.push(`notice ${text}`); },
  };
  const plan = () => planNodeNativeAddonV1(value.host, source);
  return { value, files, log, env, plan, sleeps: () => sleeps };
}

describe("Node native core preparation", () => {
  test("every platform and architecture the POSIX consumers qualify has a declared target with a distinct triple", () => {
    for (const platform of ["darwin", "linux"]) for (const arch of ["arm64", "x64"]) expect(nodeNativeTargetV1(platform, arch).triple).toContain(arch === "x64" ? "x86_64" : "aarch64");
    expect(nodeNativeTargetV1("win32", "x64").triple).toBe("x86_64-pc-windows-msvc");
    expect(new Set(protocol.targets.map(row => row.triple)).size).toBe(protocol.targets.length);
    expect(() => nodeNativeTargetV1("freebsd", "x64")).toThrow(RuntimeCapabilityMissingV1);
    expect(() => nodeNativeTargetV1("linux", "riscv64")).toThrow("riscv64");
  });

  test("the cache root follows the operating system and the documented overrides", () => {
    const rows: [Partial<NodeNativeHostV1>, string][] = [
      [{ platform: "darwin", arch: "arm64" }, "/home/dev/Library/Caches/semio/native/node"],
      [{ platform: "linux" }, "/home/dev/.cache/semio/native/node"],
      [{ platform: "linux", env: { XDG_CACHE_HOME: "/xdg" } }, "/xdg/semio/native/node"],
      [{ platform: "win32", env: { LOCALAPPDATA: "C:\\Local" } }, join("C:\\Local", "semio", "native", "node")],
      [{ platform: "win32" }, join("/home/dev", "AppData", "Local", "semio", "native", "node")],
      [{ env: { SEMIO_RUNTIME_CACHE: "/shared" } }, "/shared/native/node"],
      [{ platform: "darwin", env: { SEMIO_RUNTIME_CACHE: "/shared", XDG_CACHE_HOME: "/xdg" } }, "/shared/native/node"],
      [{ platform: "linux", env: { XDG_CACHE_HOME: "" } }, "/home/dev/.cache/semio/native/node"],
    ];
    for (const [overrides, expected] of rows) expect(nodeNativeCacheRootV1(host(overrides)), JSON.stringify(overrides)).toBe(expected);
  });

  test("the key changes with the source, the platform and the architecture and the path stays inside the limit", () => {
    const base = planNodeNativeAddonV1(host(), source);
    expect(base.key).toMatch(/^linux-x64-[0-9a-f]{16}$/u);
    expect(planNodeNativeAddonV1(host(), Buffer.from("fn main() { }")).key).not.toBe(base.key);
    expect(planNodeNativeAddonV1(host({ arch: "arm64" }), source).key).not.toBe(base.key);
    expect(planNodeNativeAddonV1(host({ platform: "darwin" }), source).key).not.toBe(base.key);
    expect(planNodeNativeAddonV1(host(), source)).toEqual(base);
    expect(base.file.length).toBeLessThanOrEqual(protocol.addon.maximumPathCharacters);
    expect(() => planNodeNativeAddonV1(host({ env: { SEMIO_RUNTIME_CACHE: `/${"x".repeat(300)}` } }), source)).toThrow("SEMIO_RUNTIME_CACHE");
  });

  test("the compiler call names the crate, the target's link arguments, the output and the source in a fixed order", () => {
    const darwin = planNodeNativeAddonV1(host({ platform: "darwin", arch: "arm64" }), source), linux = planNodeNativeAddonV1(host(), source), windows = planNodeNativeAddonV1(host({ platform: "win32" }), source);
    const arguments_ = (plan: typeof darwin) => nodeNativeCompilerArgumentsV1(plan, "/src.rs", "/out.node");
    expect(arguments_(darwin)).toContain("link-arg=-Wl,-undefined,dynamic_lookup");
    expect(arguments_(linux)).toContain("target-feature=-crt-static");
    expect(arguments_(windows).some(argument => argument.startsWith("link-arg") || argument.startsWith("target-feature"))).toBe(false);
    for (const plan of [darwin, linux, windows]) {
      const list = arguments_(plan);
      expect(list.slice(-5)).toEqual(["--crate-name", protocol.compiler.crateName, "-o", "/out.node", "/src.rs"]);
      expect(list).toContain("cdylib");
      expect(list).toContain("panic=abort");
    }
  });

  test("the compiler is found by SEMIO_RUSTC, then PATH, then CARGO_HOME or ~/.cargo, with the .exe suffix on Windows", () => {
    const present = (...paths: string[]) => (path: string) => paths.includes(path);
    expect(nodeNativeCompilerV1(host({ env: { SEMIO_RUSTC: "/custom/rustc" } }), present("/custom/rustc"))).toBe("/custom/rustc");
    expect(nodeNativeCompilerV1(host({ env: { SEMIO_RUSTC: "/custom/rustc", PATH: "/usr/bin" } }), present("/usr/bin/rustc"))).toBeUndefined();
    expect(nodeNativeCompilerV1(host({ env: { PATH: "/a:/b" } }), present("/b/rustc"))).toBe("/b/rustc");
    expect(nodeNativeCompilerV1(host(), present("/home/dev/.cargo/bin/rustc"))).toBe("/home/dev/.cargo/bin/rustc");
    expect(nodeNativeCompilerV1(host({ env: { CARGO_HOME: "/cargo" } }), present("/cargo/bin/rustc"))).toBe("/cargo/bin/rustc");
    expect(nodeNativeCompilerV1(host({ platform: "win32", env: { CARGO_HOME: "/cargo" } }), present(join("/cargo", "bin", "rustc.exe")))).toBe(join("/cargo", "bin", "rustc.exe"));
    expect(nodeNativeCompilerV1(host(), present())).toBeUndefined();
  });

  test("a cache hit builds nothing", () => {
    const fake = ports();
    fake.files.add(fake.plan().file);
    expect(ensureNodeNativeAddonV1(fake.value)).toBe(fake.plan().file);
    expect(fake.log.filter(line => line.startsWith("run") || line.startsWith("lock"))).toEqual([]);
  });

  test("a miss takes the lock, compiles once into staging, renames atomically and releases the lock", () => {
    const fake = ports();
    const file = ensureNodeNativeAddonV1(fake.value), plan = fake.plan();
    expect(file).toBe(plan.file);
    expect(fake.log.filter(line => line.startsWith("run"))).toEqual(["run /cargo/bin/rustc"]);
    expect(fake.log.find(line => line.startsWith("rename"))).toBe(`rename ${plan.directory}.staging-t1 ${plan.directory}`);
    expect(fake.log.findIndex(line => line.startsWith("lock"))).toBeLessThan(fake.log.findIndex(line => line.startsWith("run")));
    expect(fake.log.at(-1)).toBe(`remove ${plan.directory}.lock`);
    expect(fake.files.has(`${plan.directory}.lock`)).toBe(false);
    expect(fake.log.find(line => line.startsWith("notice"))).toContain("linux x64");
  });

  test("a process that loses the race waits for the holder's result instead of compiling", () => {
    const fake = ports({}, { lockHeld: 1000, lockAge: 10, appearsAfterSleeps: 3 });
    expect(ensureNodeNativeAddonV1(fake.value)).toBe(fake.plan().file);
    expect(fake.sleeps()).toBe(3);
    expect(fake.log.some(line => line.startsWith("run"))).toBe(false);
  });

  test("a stale lock is removed and the build proceeds; a lock that never clears times out with a named refusal", () => {
    const stale = ports({}, { lockHeld: 1, lockAge: (protocol.lock.staleSeconds + 1) * 1000 });
    expect(ensureNodeNativeAddonV1(stale.value)).toBe(stale.plan().file);
    expect(stale.log.some(line => line.startsWith("run"))).toBe(true);
    const stuck = ports({}, { lockHeld: 1_000_000, lockAge: 1, clock: protocol.lock.waitSeconds * 400 });
    expect(() => ensureNodeNativeAddonV1(stuck.value)).toThrow(String(protocol.lock.waitSeconds));
  });

  test("a missing compiler and a failing compile are named refusals that leave no staging directory and no lock", () => {
    const missing = ports({}, { compiler: undefined });
    expect(() => ensureNodeNativeAddonV1(missing.value)).toThrow(RuntimeCapabilityMissingV1);
    expect(() => ensureNodeNativeAddonV1(ports({}, { compiler: undefined }).value)).toThrow("SEMIO_RUSTC");
    const failing = ports({}, { runs: [{ status: 1, stderr: "line1\nerror[E0432]: unresolved import\n" }] });
    expect(() => ensureNodeNativeAddonV1(failing.value)).toThrow("unresolved import");
    expect([...failing.files].filter(path => path.includes(".staging-") || path.endsWith(".lock"))).toEqual([]);
  });

  test("a Windows host plans, compiles and publishes the core like the others: the .exe compiler, no link arguments, the .node file, both architectures, a long account name inside the path limit", () => {
    const account = "C:\\Users\\" + "a-very-long-account-name-of-forty-chars".padEnd(40, "x"), env = { LOCALAPPDATA: `${account}\\AppData\\Local` };
    const x64 = ports({ platform: "win32", arch: "x64", home: account, env }), arm = ports({ platform: "win32", arch: "arm64", home: account, env }), compiler = `${account}\\.cargo\\bin\\rustc.exe`;
    for (const fake of [x64, arm]) {
      const plan = fake.plan();
      expect(plan.key).toMatch(/^win32-(x64|arm64)-[0-9a-f]{16}$/u);
      expect(plan.file.length).toBeLessThanOrEqual(protocol.addon.maximumPathCharacters);
      expect(plan.file.endsWith(protocol.addon.file)).toBe(true);
      expect(plan.directory.startsWith(env.LOCALAPPDATA)).toBe(true);
    }
    expect(x64.plan().key).not.toBe(arm.plan().key);
    expect(nodeNativeTargetV1("win32", "arm64").triple).toBe("aarch64-pc-windows-msvc");
    const built = ports({ platform: "win32", arch: "x64", home: account, env }, { compiler });
    expect(ensureNodeNativeAddonV1(built.value)).toBe(built.plan().file);
    expect(built.log.filter(line => line.startsWith("run"))).toEqual([`run ${compiler}`]);
    expect(built.env.length).toBe(1);
    expect(built.env[0]!.DEVELOPER_DIR).toBeUndefined();
    expect(nodeNativeCompilerArgumentsV1(built.plan(), "/src.rs", "/out.node").filter(argument => argument.startsWith("link-arg") || argument.startsWith("target-feature"))).toEqual([]);
    expect(built.log.at(-1)).toBe(`remove ${built.plan().directory}.lock`);
    const tooLong = ports({ platform: "win32", arch: "x64", env: { LOCALAPPDATA: `C:\\${"x".repeat(240)}` } });
    expect(() => tooLong.plan()).toThrow(RuntimeCapabilityMissingV1);
  });

  test("a rename that loses to a concurrent publisher is accepted and a real rename failure is raised", () => {
    const lost = ports({}, { renameFails: true });
    expect(ensureNodeNativeAddonV1(lost.value)).toBe(lost.plan().file);
  });

  test("macOS retries a license refusal once with the command line tools; other platforms and other failures do not", () => {
    const retry = ports({ platform: "darwin", arch: "arm64" }, { runs: [{ status: 69, stderr: "You have not agreed to the Xcode license agreements" }, { status: 0, stderr: "" }] });
    retry.files.add(protocol.compiler.licenseRetry.developerDirectory);
    ensureNodeNativeAddonV1(retry.value);
    expect(retry.env.map(row => row.DEVELOPER_DIR)).toEqual([undefined, protocol.compiler.licenseRetry.developerDirectory]);
    const linux = ports({}, { runs: [{ status: 69, stderr: "Xcode license" }] });
    linux.files.add(protocol.compiler.licenseRetry.developerDirectory);
    expect(() => ensureNodeNativeAddonV1(linux.value)).toThrow("compiler status 69");
    expect(linux.env.length).toBe(1);
    const other = ports({ platform: "darwin", arch: "arm64" }, { runs: [{ status: 1, stderr: "syntax error" }] });
    other.files.add(protocol.compiler.licenseRetry.developerDirectory);
    expect(() => ensureNodeNativeAddonV1(other.value)).toThrow("syntax error");
    expect(other.env.length).toBe(1);
  });

  test("a prebuilt core named by SEMIO_RUNTIME_NATIVE_ADDON must load and export every primitive", () => {
    const directory = mkdtempSync(join(tmpdir(), "nffi-addon-"));
    try {
      const complete = join(directory, "complete.cjs"), partial = join(directory, "partial.cjs"), broken = join(directory, "broken.cjs");
      writeFileSync(complete, `module.exports = {${protocol.primitives.map(name => `${name}() {}`).join(",")}};`);
      writeFileSync(partial, "module.exports = { open() {} };");
      writeFileSync(broken, "throw new Error('boom');");
      const load = (file: string) => loadNodeNativeCoreV1(ports({ env: { SEMIO_RUNTIME_NATIVE_ADDON: file } }).value);
      expect(typeof load(complete).call).toBe("function");
      expect(() => load(partial)).toThrow("symbol, close, call");
      expect(() => load(broken)).toThrow("boom");
    } finally { rmSync(directory, { recursive: true, force: true }); }
  });

  test("every refusal message exists in English and German with the same placeholders", () => {
    const catalog = messages as Record<string, { en: string; de: string }>;
    for (const id of ["nativeTargetUnsupported", "nativePathLength", "nativeCompilerMissing", "nativeBuilding", "nativeCompileFailed", "nativeLockTimeout", "nativeLoadFailed", "nativeCoreIncomplete"]) {
      const row = catalog[id]!;
      expect(row.en.length).toBeGreaterThan(10);
      expect(row.de.length).toBeGreaterThan(10);
      expect([...row.en.matchAll(/\{(\w+)\}/gu)].map(m => m[1]).sort()).toEqual([...row.de.matchAll(/\{(\w+)\}/gu)].map(m => m[1]).sort());
    }
  });
});
