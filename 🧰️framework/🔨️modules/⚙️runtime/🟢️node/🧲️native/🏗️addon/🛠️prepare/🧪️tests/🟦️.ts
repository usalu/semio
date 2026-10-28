import { spawnSync } from "node:child_process";
import { mkdtempSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, test } from "../../../../../🧪️test/🟦️.ts";
import messages from "../../../../../🧬️schema/🗣️messages.json" with { type: "json" };
import { defaultNativeToolchainPortsV1, nativeToolchainHostV1, probeNativeToolchainV1, type NativeToolProbeV1 } from "../../../../../🧲️native/🧰️toolchain/🟦️.ts";
import schema from "../../../🧬️schema/🔣️.json" with { type: "json" };
import { defaultNodeNativePortsV1, planNodeNativeAddonV1, type NodeNativePortsV1 } from "../../🟦️.ts";
import { nodeNativeCoreStateV1, prepareNodeNativeCoreV1, runNodeNativePrepareV1 } from "../🟦️.ts";

const protocol = schema.definitions.Protocol.const, script = join(import.meta.dirname, "..", "📜️script.ts");
const source = Buffer.from("fn main() {}");
const probes = (missing: readonly string[]): NativeToolProbeV1[] => ["cc", "rust"].map(id => ({ id, found: !missing.includes(id), location: missing.includes(id) ? null : `/usr/bin/${id}`, how: missing.includes(id) ? "none" : "path" }));

function table(env: Record<string, string | undefined> = {}) {
  const files = new Set<string>(), log: string[] = [];
  const value: NodeNativePortsV1 = {
    host: { platform: "linux", arch: "x64", env, home: "/home/dev", source: "/repo/native.rs" },
    read: () => source, exists: path => files.has(path), mkdir: path => { files.add(path); }, lock: path => { files.add(path); return true; }, age: () => undefined,
    remove: path => { for (const file of [...files]) if (file === path || file.startsWith(`${path}/`)) files.delete(file); },
    rename: (from, to) => { log.push("rename"); files.delete(from); files.add(to); files.add(join(to, protocol.addon.file)); },
    compiler: () => "/cargo/bin/rustc",
    run: (_program, arguments_) => { log.push("compile"); files.add(arguments_[arguments_.indexOf("-o") + 1]!); return { status: 0, stderr: "" }; },
    sleep: () => undefined, now: () => 0, staging: () => "t1", notice: () => undefined,
  };
  return { value, files, log, plan: () => planNodeNativeAddonV1(value.host, source) };
}

describe("Node native core prebuild", () => {
  test("a missing prerequisite is named before any compiler starts, a present one builds once, a built core is left alone", () => {
    const missing = table();
    expect(() => prepareNodeNativeCoreV1(missing.value, () => probes(["cc", "rust"]))).toThrow(/missing cc, rust/u);
    expect(() => prepareNodeNativeCoreV1(missing.value, () => probes(["cc"]))).toThrow(/Haben|missing cc|fehlt cc/u);
    expect(missing.log).toEqual([]);
    const machine = table();
    expect(nodeNativeCoreStateV1(machine.value)).toEqual({ built: false, file: machine.plan().file, designated: false });
    expect(prepareNodeNativeCoreV1(machine.value, () => probes([]))).toEqual({ built: true, file: machine.plan().file, designated: false });
    expect(machine.log).toEqual(["compile", "rename"]);
    let asked = 0;
    expect(prepareNodeNativeCoreV1(machine.value, () => { asked++; return probes(["cc"]); }).built).toBe(true);
    expect(asked, "a built core needs no prerequisite").toBe(0);
    expect(machine.log).toEqual(["compile", "rename"]);
  });

  test("a designated prebuilt core replaces the build and the exit statuses follow the state", () => {
    const named = table({ [protocol.addon.environment.addon]: "/prebuilt/core.node" });
    const out: string[] = [];
    expect(runNodeNativePrepareV1(["verify"], named.value, () => probes([]), line => out.push(line))).toBe(1);
    named.files.add("/prebuilt/core.node");
    expect(runNodeNativePrepareV1(["verify"], named.value, () => probes([]), line => out.push(line))).toBe(0);
    expect(runNodeNativePrepareV1(["prepare"], named.value, () => probes(["cc"]), line => out.push(line))).toBe(0);
    expect(out[0]).toContain("/prebuilt/core.node");
    expect(named.log).toEqual([]);
    const plain = table();
    expect(runNodeNativePrepareV1(["verify"], plain.value, () => probes([]), line => out.push(line))).toBe(1);
    expect(runNodeNativePrepareV1(["prepare"], plain.value, () => probes(["rust"]), line => out.push(line))).toBe(1);
    expect(runNodeNativePrepareV1(["prepare"], plain.value, () => probes([]), line => out.push(line))).toBe(0);
    expect(runNodeNativePrepareV1(["verify"], plain.value, () => probes(["cc"]), line => out.push(line))).toBe(0);
    for (const bad of [[], ["build"], ["verify", "extra"], ["prepare", "--force"]]) expect(runNodeNativePrepareV1(bad, plain.value, () => probes([]), line => out.push(line)), JSON.stringify(bad)).toBe(2);
  });

  test("every message exists in both languages with the same placeholders", () => {
    const catalog = messages as Record<string, { en: string; de: string }>;
    for (const id of ["nativePrebuildUsage", "nativePrebuildReady", "nativePrebuildDesignated", "nativePrebuildMissing", "nativePrebuildToolchain"]) {
      const row = catalog[id]!, holes = (text: string) => [...text.matchAll(/\{(\w+)\}/gu)].map(match => match[1]).sort();
      expect(row.en.length).toBeGreaterThan(20);
      expect(row.de).not.toBe(row.en);
      expect(holes(row.de), id).toEqual(holes(row.en));
    }
  });

  test("on this machine the script builds the real core once into a private cache, and Node's own loader accepts the result", () => {
    const toolchain = probeNativeToolchainV1(nativeToolchainHostV1(), defaultNativeToolchainPortsV1());
    if (toolchain.some(probe => !probe.found)) { console.warn(`skipped: missing ${toolchain.filter(probe => !probe.found).map(probe => probe.id).join(", ")}`); return; }
    const cache = mkdtempSync(join(tmpdir(), "semio-prebuild-")), env = { ...process.env, SEMIO_RUNTIME_CACHE: cache, SEMIO_LANGUAGE: "en", SEMIO_RUNTIME_NATIVE_ADDON: "" };
    const run = (command: string): { status: number | null; stdout: string; seconds: number } => { const started = Date.now(), result = spawnSync(process.execPath, [script, command], { env, encoding: "utf8", timeout: 600000 }); return { status: result.status, stdout: result.stdout, seconds: (Date.now() - started) / 1000 }; };
    try {
      const expected = planNodeNativeAddonV1(defaultNodeNativePortsV1(env).host, readFileSync(defaultNodeNativePortsV1(env).host.source));
      expect(run("verify")).toMatchObject({ status: 1 });
      const built = run("prepare");
      expect(built.status).toBe(0);
      expect(built.stdout.trim()).toBe(`The Node native core is ready: ${expected.file}`);
      expect(run("verify")).toMatchObject({ status: 0, stdout: `The Node native core is ready: ${expected.file}\n` });
      const again = run("prepare");
      expect(again.status).toBe(0);
      expect(again.seconds, "a built core costs a start, not a build").toBeLessThan(Math.max(5, built.seconds));
      const probe = spawnSync(process.versions.node === undefined ? "node" : process.execPath, ["-e", `const m={exports:{}};process.dlopen(m,${JSON.stringify(expected.file)});console.log(Object.keys(m.exports).sort().join())`], { encoding: "utf8" });
      expect(probe.stdout.trim()).toBe([...protocol.primitives].sort().join());
    } finally { rmSync(cache, { recursive: true, force: true }); }
  }, 300000);
});
