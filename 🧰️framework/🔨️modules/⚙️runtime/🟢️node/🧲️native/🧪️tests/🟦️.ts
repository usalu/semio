import { spawn, spawnSync } from "node:child_process";
import { existsSync, mkdtempSync, readFileSync, readdirSync, rmSync, statfsSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, test } from "../../../🧪️test/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import fixture from "./🔣️.json" with { type: "json" };

const here = dirname(fileURLToPath(import.meta.url));
const runner = join(here, "🏃️corpus/🟦️.ts"), oracle = join(here, "🐍️.py"), fixturePath = join(here, "🔣️.json");
const hostRuntime = "bun" in process.versions ? "bun" : "node";
const posix = process.platform !== "win32";
const protocol = schema.definitions.Protocol.const;

const repositoryRoot = process.env.SEMIO_REPOSITORY_ROOT ?? resolve(here, "../../../../../..");
const pinnedBun = (): string | undefined => { try { return readdirSync(join(repositoryRoot, ".cache/b/t/m/bun")).map(entry => join(repositoryRoot, ".cache/b/t/m/bun", entry, "bun")).find(path => existsSync(path)); } catch { return undefined; } };
const modernBun = (path: string): boolean => { const run = spawnSync(path, ["--version"], { encoding: "utf8", cwd: tmpdir() }); const [major, minor] = run.stdout.trim().split(".").map(Number); return run.status === 0 && (major! > 1 || (major === 1 && minor! >= 4)); };
const executable = (name: string): string | undefined => {
  if (name === hostRuntime) return process.execPath;
  if (name === "node") return spawnSync("node", ["--version"], { encoding: "utf8" }).status === 0 ? "node" : undefined;
  return [pinnedBun(), "bun"].find(path => path !== undefined && modernBun(path));
};
const python = ["/usr/bin/python3", "python3"].find(candidate => spawnSync(candidate, ["-I", "-c", "print(1)"], { encoding: "utf8" }).status === 0);
const runtimes = Object.fromEntries(["bun", "node"].map(name => [name, executable(name)])) as Record<string, string | undefined>;

type Run = { runtime: string; system: string; platform: string; calls: Record<string, any>; memory: Record<string, any>; view: any; refusals: Record<string, string>; host: Record<string, any>; parentPid: number; stderr: string };

function runCorpus(runtimePath: string, system: string, scratch: string, environment: Record<string, string> = {}): Run {
  const run = spawnSync(runtimePath, [runner, `--system=${system}`, `--fixture=${fixturePath}`, `--scratch=${scratch}`], { encoding: "utf8", cwd: tmpdir(), env: { ...process.env, ...fixture.environment, ...environment }, maxBuffer: 1 << 24 });
  expect(run.status, run.stderr).toBe(0);
  return { ...JSON.parse(run.stdout), parentPid: process.pid, stderr: run.stderr };
}

const resolveContext = (expectation: unknown, context: { pid: number; ppid: number }): unknown => {
  const row = expectation as { context?: string } | null;
  if (row !== null && typeof row === "object" && "context" in row) return row.context === "pid" ? context.pid : row.context === "ppid" ? context.ppid : { bigint: String(context.pid) };
  return expectation;
};

const expected = (call: (typeof fixture.calls)[number], context: { pid: number; ppid: number }): Record<string, unknown> => {
  const spec = call as { expect: unknown; errno?: number; after?: Record<string, string> };
  return { value: resolveContext(spec.expect, context), ...(spec.errno === undefined ? {} : { errno: spec.errno }), ...(spec.after === undefined ? {} : { after: spec.after }) };
};

const available = posix && (process.platform === "darwin" || process.platform === "linux");
const scratchRoot = available ? mkdtempSync(join(tmpdir(), "nffi-corpus-")) : "";
const runs: Run[] = [];
if (available) {
  for (const [name, path] of Object.entries(runtimes)) {
    if (path === undefined) continue;
    for (const system of ["host", "core"]) {
      runs.push(runCorpus(path, system, join(scratchRoot, `${name}-${system}`)));
    }
  }
}

describe("Node native core parity", () => {
  test("the corpus fixture is schema-valid and has unique ids", () => {
    expect(validateJsonSchemaSubset(schema.definitions.Corpus, fixture, schema)).toEqual([]);
    const ids = fixture.calls.map(call => call.id);
    expect(new Set(ids).size).toBe(ids.length);
    expect(ids.length).toBeGreaterThanOrEqual(40);
  });

  test("the Rust core exports exactly the declared primitives, targets Node-API 8 and never panics on purpose", () => {
    const text = readFileSync(join(here, "../🦀️.rs"), "utf8");
    const exported = [...text.matchAll(/export\(env, exports, b"(\w+)\\0"/gu)].map(match => match[1]).sort();
    expect(exported).toEqual([...protocol.primitives].sort());
    expect(text).toContain("napi_register_module_v1");
    expect(text).toContain("node_api_module_get_api_version_v1");
    expect(text).not.toMatch(/\.unwrap\(|panic!\(|\.expect\(/u);
  });

  test("the Rust core generates code for every declared target whose standard library is installed, and always for the host's", () => {
    const rustc = spawnSync("rustc", ["-vV"], { encoding: "utf8", cwd: here });
    if (rustc.status !== 0) { expect(process.env.CI ?? "").toBe(""); return; }
    const hostTriple = /^host: (.+)$/mu.exec(rustc.stdout)![1]!, directory = mkdtempSync(join(tmpdir(), "nffi-xt-"));
    try {
      const checked: string[] = [];
      for (const target of protocol.targets) {
        const libdir = spawnSync("rustc", ["--print", "target-libdir", "--target", target.triple], { encoding: "utf8", cwd: here }).stdout.trim();
        if (target.triple !== hostTriple && !existsSync(libdir)) continue;
        const run = spawnSync("rustc", [...protocol.compiler.arguments.slice(0, 4), "--crate-name", protocol.compiler.crateName, "--target", target.triple, "--emit=obj", "-C", "panic=abort", "-o", join(directory, `${target.triple}.o`), join(here, "../🦀️.rs")], { encoding: "utf8", cwd: here });
        expect(run.status, `${target.triple}: ${run.stderr}`).toBe(0);
        expect(run.stderr, target.triple).toBe("");
        checked.push(target.triple);
      }
      if (protocol.targets.some(target => target.triple === hostTriple)) expect(checked).toContain(hostTriple);
    } finally { rmSync(directory, { recursive: true, force: true }); }
  });

  test.skipIf(!available)("at least one runtime ran the corpus in both system flavours", () => {
    expect(runs.length).toBeGreaterThanOrEqual(2);
    for (const [name, path] of Object.entries(runtimes)) if (path !== undefined) for (const system of ["host", "core"]) expect(runs.some(run => run.runtime === name && run.system === system), `${name}/${system}`).toBe(true);
    expect(runs.some(run => run.runtime === "node" && run.system === "core")).toBe(true);
  });

  test.skipIf(!available)("every run matches the fixture's literal expectations on every call, memory read, live view and refusal", () => {
    for (const run of runs) {
      const label = `${run.runtime}/${run.system}`, context = { pid: run.host["process-tree"].pid, ppid: run.host["process-tree"].ppid };
      for (const call of fixture.calls) expect(run.calls[call.id], `${label} ${call.id}`).toEqual(expected(call, context));
      for (const memory of fixture.memory) expect(run.memory[memory.id], `${label} ${memory.id}`).toEqual(memory.reads.map(row => row[2]));
      expect(run.view, label).toEqual({ before: fixture.view.expectBefore, after: fixture.view.expectAfter });
      for (const refusal of fixture.refusals) expect(run.refusals[refusal.id], `${label} ${refusal.id}`).toBe("throws");
    }
  });

  test.skipIf(!available)("Bun's own binding, Node's core and Bun over the core agree on every host fact", () => {
    const normalise = (run: Run) => JSON.stringify({ ...run.host, "process-tree": { uid: run.host["process-tree"].uid } });
    const reference = normalise(runs[0]!);
    for (const run of runs) expect(normalise(run), `${run.runtime}/${run.system}`).toBe(reference);
  });

  test.skipIf(!available)("native pids, parents and groups equal the operating system's process table", () => {
    for (const run of runs) {
      const row = run.host["process-tree"];
      expect(row.ppid, `${run.runtime}/${run.system}`).toBe(run.parentPid);
      expect(row.pid).not.toBe(run.parentPid);
      expect(row.uid).toBe(process.getuid!());
      const group = spawnSync(python ?? ((): never => { throw Error("Independent Python oracle required"); })(), ["-I", "-c", "import os,sys;print(os.getpgid(int(sys.argv[1])))", String(process.pid)], { encoding: "utf8" });
      expect(row.pgid).toBe(Number(group.stdout.trim()));
    }
  });

  test.skipIf(!available)("native statfs, case folding and the directory exchange agree with Node's own file system calls", () => {
    for (const run of runs) {
      const native = statfsSync(join(scratchRoot, `${run.runtime}-${run.system}`));
      expect(run.host.statfs.bsize).toBe(native.bsize);
      expect(run.host.statfs.blocks).toBe(native.blocks);
      expect(run.host.statfs.bavailPositive).toBe(native.bavail > 0);
      const folded = existsSync(join(scratchRoot, `${run.runtime}-${run.system}`, "case", "probe-case.txt")) ? 0 : -1;
      expect(run.host["case-sensitivity"]).toEqual({ exact: 0, folded });
      expect(run.host["rename-swap"]).toEqual({ swapped: 0, namesA: ["y"], namesB: ["x"], missing: { status: -1, errno: 2 } });
    }
  });

  test.skipIf(!available || python === undefined)("an independent Python ctypes implementation reproduces the fixture, the memory reads and the host facts", () => {
    const scratch = join(scratchRoot, "python");
    const run = spawnSync(python!, ["-I", oracle, fixturePath, scratch], { encoding: "utf8", env: { ...process.env, ...fixture.environment } });
    expect(run.status, run.stderr).toBe(0);
    const model = JSON.parse(run.stdout);
    const context = { pid: model.host["process-tree"].pid, ppid: model.host["process-tree"].ppid };
    for (const call of fixture.calls) expect(model.calls[call.id], call.id).toEqual(expected(call, context));
    for (const memory of fixture.memory) expect(model.memory[memory.id], memory.id).toEqual(memory.reads.map(row => row[2]));
    const reference = runs[0]!.host;
    expect(model.host["rename-swap"]).toEqual(reference["rename-swap"]);
    expect(model.host["case-sensitivity"]).toEqual(reference["case-sensitivity"]);
    expect(model.host.statfs.totalBytes).toBe(reference.statfs.bsize * reference.statfs.blocks);
  });

  test.skipIf(!available || runtimes.node === undefined)("Node needs no setting: a fresh cache builds the core once, racing processes build it exactly once, the next start is silent", async () => {
    const cache = mkdtempSync(join(tmpdir(), "nffi-cache-"));
    try {
      const environment = { ...process.env, ...fixture.environment, SEMIO_RUNTIME_CACHE: cache, SEMIO_RUNTIME_NATIVE_PROVIDER: "", SEMIO_RUNTIME_NATIVE_ADDON: "", SEMIO_LANGUAGE: "en" };
      const start = (index: number): Promise<{ status: number | null; stderr: string }> => new Promise(accept => {
        const child = spawn(runtimes.node!, [runner, "--system=host", `--fixture=${fixturePath}`, `--scratch=${join(cache, `scratch-${index}`)}`], { env: environment, stdio: ["ignore", "ignore", "pipe"] });
        let stderr = "";
        child.stderr!.on("data", chunk => { stderr += String(chunk); });
        child.on("close", status => accept({ status, stderr }));
      });
      const racing = await Promise.all([start(1), start(2), start(3)]);
      for (const row of racing) expect(row.status, row.stderr).toBe(0);
      expect(racing.filter(row => row.stderr.includes("Building the Node native core once")).length).toBe(1);
      const again = await start(4);
      expect(again.status, again.stderr).toBe(0);
      expect(again.stderr).toBe("");
    } finally { rmSync(cache, { recursive: true, force: true }); }
  });

  test.skipIf(process.platform !== "win32" || runtimes.node === undefined)("on Windows the Node core builds once for the msvc target, loads under node.exe and exports every primitive; it never skips silently where Node and rustc exist", () => {
    const rustc = spawnSync("rustc", ["-vV"], { encoding: "utf8", cwd: here });
    if (rustc.status !== 0) { expect(process.env.CI ?? "", "rustc is required on a CI Windows host").toBe(""); return; }
    const cache = mkdtempSync(join(tmpdir(), "nffi-win-"));
    try {
      const script = `const { loadNodeNativeCoreV1 } = require(${JSON.stringify(join(here, "../🏗️addon/🟦️.ts"))}); const core = loadNodeNativeCoreV1(); process.stdout.write(JSON.stringify(${JSON.stringify(protocol.primitives)}.map(name => typeof core[name])));`;
      const run = spawnSync(runtimes.node!, ["-e", script], { encoding: "utf8", env: { ...process.env, SEMIO_RUNTIME_CACHE: cache, SEMIO_RUNTIME_NATIVE_PROVIDER: "", SEMIO_RUNTIME_NATIVE_ADDON: "", SEMIO_LANGUAGE: "en" } });
      expect(run.status, run.stderr).toBe(0);
      expect(JSON.parse(run.stdout)).toEqual(protocol.primitives.map(() => "function"));
      expect(run.stderr).toContain("Building the Node native core once");
    } finally { rmSync(cache, { recursive: true, force: true }); }
  });

  test.skipIf(!available || runtimes.node === undefined)("a designated provider still wins over the built-in core", () => {
    const provider = join(here, "../../../🧪️tests/🧫️fixtures/🧲️native-provider.cjs");
    const script = `const { nativeSystemFromProviderV1 } = require(${JSON.stringify(join(here, "../🟦️.ts"))}); process.stdout.write(String(nativeSystemFromProviderV1().ptr(Buffer.alloc(1))));`;
    const run = spawnSync(runtimes.node!, ["-e", script], { encoding: "utf8", env: { ...process.env, SEMIO_RUNTIME_NATIVE_PROVIDER: provider } });
    expect(run.status, run.stderr).toBe(0);
    expect(run.stdout).toBe("1");
  });
});

process.on("exit", () => { if (scratchRoot !== "") rmSync(scratchRoot, { recursive: true, force: true }); });
