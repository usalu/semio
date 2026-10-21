import { describe, expect, test } from "bun:test";
import {testPythonV1,testRipgrepV1} from "../../🧰️toolchain/🟦️.ts";
import Ajv from "ajv";
import { spawn, spawnSync } from "node:child_process";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, realpathSync, rmSync, statSync, utimesSync, writeFileSync } from "node:fs";
import { createServer } from "node:net";
import { tmpdir } from "node:os";
import { join } from "node:path";
import protocol from "../🔣️.json" with { type: "json" };
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import corpus from "./🧫️fixtures/🔣️.json" with { type: "json" };
import ledger from "./🧫️fixtures/📒️ledger.json" with { type: "json" };
import {
  abandonedTestSandboxV1, adoptedTestSandboxRootV1, attachedDiskImagesV1, mayHoldDiskImageV1, guardArgumentsV1, testCheckoutRootV1, compileLawV1, createTestSandboxV1, decideListenV1, decideSpawnV1, decideWriteV1, liveTestSandboxesV1, readTestPortLeaseV1,
  reclaimAbandonedTestSandboxesV1, releaseTestPortV1, summarizeGuardReportV1, scanIsTestPathV1, scanLanguageV1, scanOwnerV1, scanSourceV1, testPathInsideV1, testSandboxEnvironmentV1, testScratchRootV1, withTestSandboxV1,
  type GuardContextV1, type GuardViolationKindV1, type ScanLanguageV1, type TestSandboxProtocolV1,
} from "../🟦️.ts";

const declared = protocol as unknown as TestSandboxProtocolV1, law = compileLawV1(declared), root = join(import.meta.dir, "..");
const names = declared.environment, python = spawnSync(testPythonV1()??"python3", ["--version"], { encoding: "utf8" }).status === 0, ripgrep = spawnSync(testRipgrepV1()??"rg", ["--version"], { encoding: "utf8" }).status === 0;

function context(row: (typeof corpus.writes)[number]["context"]): GuardContextV1 {
  return { mode: "enforce", checkout: row.checkout, cwd: row.cwd, platform: row.platform as NodeJS.Platform, allowed: row.allowed, sandboxRoots: row.sandboxRoots, protectedPrefixes: row.protectedPrefixes, allowedCheckoutPrefixes: row.allowedCheckoutPrefixes, portsDirectory: undefined, sandbox: undefined, declared };
}

describe("protocol and corpus", () => {
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addSchema(schema);
  const validate = (name: string, value: unknown): void => {
    const check = ajv.getSchema(`${schema.$id}#/definitions/${name}`)!;
    expect(check(value) ? [] : check.errors).toEqual([]);
  };
  test("the closed schema admits the protocol, the corpus and every rule, and rejects stray fields", () => {
    validate("Protocol", protocol);
    validate("Corpus", corpus);
    validate("Ledger", ledger);
    expect(new Set(ledger.entries.map(entry => `${entry.file}\0${entry.rule}`)).size).toBe(ledger.entries.length);
    for (const entry of ledger.entries) expect(protocol.rules.some(rule => rule.id === entry.rule && rule.severity === "error"), entry.rule).toBe(true);
    for (const rule of protocol.rules) validate("Rule", rule);
    expect(ajv.getSchema(`${schema.$id}#/definitions/Protocol`)!({ ...protocol, extra: 1 })).toBe(false);
    expect(ajv.getSchema(`${schema.$id}#/definitions/Rule`)!({ ...protocol.rules[0], pattern: undefined, scope: "nowhere" })).toBe(false);
    expect(new Set(protocol.rules.map(rule => rule.id)).size).toBe(protocol.rules.length);
  });
  test("the scanner replays every language-agnostic scan, owner and path case", () => {
    for (const row of corpus.scans) {
      const got = scanSourceV1(law, row.path, row.language as ScanLanguageV1, row.text).map(finding => `${finding.rule}:${finding.line}`).sort();
      expect(got, row.id).toEqual(row.expected.map(finding => `${finding.rule}:${finding.line}`).sort());
    }
    for (const row of corpus.owners) expect(scanOwnerV1(law, "x", row.sources as { language: ScanLanguageV1; text: string }[]).map(finding => finding.rule).sort(), row.id).toEqual(row.expected.map(finding => finding.rule).sort());
    for (const row of corpus.paths) expect(scanLanguageV1(law, row.path) !== undefined && scanIsTestPathV1(law, row.path, new Set(row.declared ?? [])), row.path).toBe(row.test);
  });
  test("the guard decisions agree with the corpus", () => {
    for (const row of corpus.writes) expect(decideWriteV1(context(row.context), row.function, row.target)?.kind ?? null, row.id).toBe(row.expected as GuardViolationKindV1 | null);
    for (const row of corpus.spawns) expect(decideSpawnV1(context(row.context), row.command, row.args)?.kind ?? null, row.id).toBe(row.expected as GuardViolationKindV1 | null);
    for (const row of corpus.listens) {
      const directory = mkdtempSync(join(tmpdir(), "sb-l-"));
      try {
        for (const port of row.leased) mkdirSync(join(directory, declared.ports.directory, String(port)), { recursive: true });
        expect(decideListenV1({ ...context(row.context), portsDirectory: join(directory, declared.ports.directory), sandbox: "x" }, row.port)?.kind ?? null, row.id).toBe(row.expected as GuardViolationKindV1 | null);
      } finally { rmSync(directory, { recursive: true, force: true }); }
    }
  });
  test.skipIf(!python)("the independent Python oracle replays the same corpus with the same outcome", () => {
    const run = spawnSync(testPythonV1()??"python3", ["-I", join(import.meta.dir, "🐍️oracle.py"), join(root, "🔣️.json"), join(import.meta.dir, "🧫️fixtures/🔣️.json")], { encoding: "utf8" });
    expect(run.stdout + run.stderr).toContain("mismatches 0");
    expect(run.status).toBe(0);
  });
  test.skipIf(!ripgrep)("ripgrep, a third-party regex engine, marks the same lines as the line rules", () => {
    const directory = mkdtempSync(join(tmpdir(), "sb-rg-"));
    try {
      let checked = 0;
      for (const compiled of law.rules) {
        const rule = compiled.rule as typeof compiled.rule & { pattern?: string; flags?: string; also?: string[]; exempt?: string[] };
        if (rule.scope !== "line" || rule.also?.length || rule.exempt?.length) continue;
        const lines = corpus.scans.filter(row => (rule.languages as readonly string[]).includes(row.language)).flatMap(row => row.text.split("\n").map(line => law.comment[row.language]!.test(line) ? "" : line)), file = join(directory, "case.txt");
        writeFileSync(file, lines.join("\n"));
        const run = spawnSync(testRipgrepV1()??"rg", ["--no-config", "--line-number", "--no-heading", "--no-filename", ...(rule.flags === "i" ? ["-i"] : []), "-e", rule.pattern!, file], { encoding: "utf8" });
        expect(run.status === 0 || run.status === 1, `${rule.id}: ${run.stderr}`).toBe(true);
        expect(run.stdout.split("\n").filter(Boolean).map(line => Number(line.split(":")[0])), rule.id).toEqual(lines.flatMap((line, index) => compiled.pattern!.test(line) ? [index + 1] : []));
        checked += lines.length;
      }
      expect(checked).toBeGreaterThan(300);
    } finally { rmSync(directory, { recursive: true, force: true }); }
  }, 60000);
});

describe("per-run sandbox", () => {
  test("a sandbox owns a short private root with the declared layout, environment and owner row", () => {
    const sandbox = createTestSandboxV1({ route: "unit", checkout: root, reclaim: false });
    try {
      expect(sandbox.root.length).toBeLessThanOrEqual(declared.layout.maximumRootLength);
      for (const name of declared.layout.directories) expect(statSync(join(sandbox.root, name)).isDirectory()).toBe(true);
      expect(JSON.parse(readFileSync(join(sandbox.root, declared.layout.owner), "utf8")).pid).toBe(process.pid);
      expect(sandbox.environment[names.sandbox]).toBe(sandbox.root);
      expect(sandbox.environment[names.artifacts]).toBe(join(sandbox.root, "artifacts", "unit"));
      expect(sandbox.environment.TMPDIR).toBe(sandbox.tmp);
      expect(sandbox.environment.TMP).toBe(sandbox.tmp);
      expect(sandbox.environment.TEMP).toBe(sandbox.tmp);
      expect(sandbox.environment[names.checkout]).toBe(root);
      expect(testPathInsideV1(sandbox.shared, sandbox.root)).toBe(true);
      const other = createTestSandboxV1({ route: "unit", reclaim: false });
      try { expect(other.root).not.toBe(sandbox.root); expect(liveTestSandboxesV1().length).toBeGreaterThanOrEqual(2); } finally { other.dispose(); }
    } finally {
      sandbox.dispose();
      sandbox.dispose();
    }
    expect(existsSync(sandbox.root)).toBe(false);
    expect(() => createTestSandboxV1({ route: "../escape" })).toThrow("Invalid test sandbox route");
  });
  test("a sandbox lies below a scratch root the boot-sweep reclaims leaked images from, and the two scratch policies agree", () => {
    const sandbox = createTestSandboxV1({ route: "boot-sweep", reclaim: false });
    try {
      const real = (path: string): string => { try { return realpathSync(path); } catch { return path; } };
      const roots = [...declared.guard.scratchRoots.map(real), real(tmpdir())];
      expect(roots.some(scratch => testPathInsideV1(scratch, real(sandbox.root))), sandbox.root).toBe(true);
      expect(testPathInsideV1(real(sandbox.root), real(sandbox.environment.TMPDIR!))).toBe(true);
      expect(testPathInsideV1(real(sandbox.root), real(sandbox.environment[names.artifacts]!))).toBe(true);
      const schemaPath = join(root, "../../📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🥾️boot/🩹️recovery/🧬️schema/🔣️.json");
      if (!existsSync(schemaPath)) return;
      const leaked = JSON.parse(readFileSync(schemaPath, "utf8")).definitions.Protocol.const.leakedImages as { scratchRoots: string[]; environmentRoots: string[] };
      expect(leaked.scratchRoots).toEqual(declared.guard.scratchRoots);
      expect(leaked.environmentRoots).toContain(names.artifacts);
      expect(leaked.environmentRoots).toContain("TMPDIR");
    } finally { sandbox.dispose(); }
  });
  test.skipIf(process.platform !== "darwin")("disposal detaches an attached image of the sandbox and skips the kernel roster when no image file exists", () => {
    const quiet = createTestSandboxV1({ route: "no-image", reclaim: false }), started = performance.now();
    expect(mayHoldDiskImageV1(quiet.root)).toBe(false);
    quiet.dispose();
    expect(performance.now() - started).toBeLessThan(1000);
    const sandbox = createTestSandboxV1({ route: "image", reclaim: false }), image = join(sandbox.images, "probe.dmg"), mount = join(sandbox.images, "mnt");
    const hdiutil = (...args: string[]) => spawnSync("/usr/bin/hdiutil", args, { encoding: "utf8" });
    expect(hdiutil("create", "-size", "4m", "-fs", "APFS", "-volname", "sbprobe", image).status).toBe(0);
    mkdirSync(mount);
    expect(hdiutil("attach", "-nobrowse", "-mountpoint", mount, image).status).toBe(0);
    const real = realpathSync(image);
    expect(attachedDiskImagesV1().some(row => row.image === real)).toBe(true);
    expect(mayHoldDiskImageV1(sandbox.root)).toBe(true);
    sandbox.dispose();
    expect(attachedDiskImagesV1().some(row => row.image === real)).toBe(false);
    expect(existsSync(sandbox.root)).toBe(false);
  }, 60000);
  test.skipIf(process.platform !== "darwin")("a killed run leaves an attached image behind and the next reclaim detaches it and removes the sandbox", async () => {
    const parent = mkdtempSync(join(tmpdir(), "sb-kill-")), modulePath = join(root, "🟦️.ts");
    const program = `import {createTestSandboxV1} from ${JSON.stringify(modulePath)};import {spawnSync} from "node:child_process";import {mkdirSync} from "node:fs";import {join} from "node:path";const s=createTestSandboxV1({route:"killed",parent:${JSON.stringify(parent)},reclaim:false});const image=join(s.images,"leak.dmg"),mount=join(s.images,"mnt");spawnSync("/usr/bin/hdiutil",["create","-size","4m","-fs","APFS","-volname","sbleak",image]);mkdirSync(mount);spawnSync("/usr/bin/hdiutil",["attach","-nobrowse","-mountpoint",mount,image]);console.log(JSON.stringify({root:s.root,image}));setInterval(()=>{},1000);`;
    const child = spawn(process.execPath, ["-e", program], { stdio: ["ignore", "pipe", "inherit"] });
    try {
      const facts = await new Promise<{ root: string; image: string }>((done, fail) => { let out = ""; child.stdout.on("data", chunk => { out += chunk; if (out.includes("\n")) done(JSON.parse(out.trim())); }); child.once("error", fail); child.once("close", () => fail(Error("child ended early"))); });
      const real = realpathSync(facts.image);
      expect(attachedDiskImagesV1().some(row => row.image === real)).toBe(true);
      child.kill("SIGKILL");
      await new Promise<void>(done => child.once("close", () => done()));
      expect(existsSync(facts.root)).toBe(true);
      const run = reclaimAbandonedTestSandboxesV1({ parent, force: true });
      expect(run.reclaimed.map(path => realpathSync(join(path, "..")))).toEqual([realpathSync(join(facts.root, ".."))]);
      expect(attachedDiskImagesV1().some(row => row.image === real)).toBe(false);
      expect(existsSync(facts.root)).toBe(false);
    } finally { child.kill("SIGKILL"); rmSync(parent, { recursive: true, force: true }); }
  }, 60000);
  test("nested runs adopt the sandbox of their parent and a scratch root follows the caller-owned artifact directory", async () => {
    await withTestSandboxV1({ route: "outer", reclaim: false }, sandbox => {
      expect(adoptedTestSandboxRootV1(sandbox.environment)).toBe(sandbox.root);
      const nested = testSandboxEnvironmentV1({ ...process.env, ...sandbox.environment }, { route: "inner" });
      expect(nested.environment[names.sandbox]).toBe(sandbox.root);
      nested.release();
      expect(existsSync(sandbox.root)).toBe(true);
      const fresh = testSandboxEnvironmentV1({}, { route: "fresh", reclaim: false });
      expect(fresh.environment[names.sandbox]).not.toBe(sandbox.root);
      const freshRoot = fresh.environment[names.sandbox]!;
      fresh.release();
      expect(existsSync(freshRoot)).toBe(false);
      expect(testScratchRootV1("x", { [names.artifacts]: join(sandbox.root, "artifacts", "given") })).toBe(join(sandbox.root, "artifacts", "given"));
    });
  });
  test("test commands receive the guard preload only for bun test with an active guard, and the checkout root is found", () => {
    const preload = join(root, "🛡️guard/🔌️preload/🟦️.ts");
    expect(guardArgumentsV1("bun", ["test", "./a.ts"], {})).toEqual(["test", "./a.ts"]);
    expect(guardArgumentsV1("bun", ["test", "./a.ts"], { [names.guard]: "report" })).toEqual(["test", "--preload", preload, "./a.ts"]);
    expect(guardArgumentsV1(process.execPath, ["test", "--timeout", "5"], { [names.guard]: "enforce" })).toEqual(["test", "--preload", preload, "--timeout", "5"]);
    expect(guardArgumentsV1("bun", ["test", "./a.ts"], { [names.guard]: "off" })).toEqual(["test", "./a.ts"]);
    expect(guardArgumentsV1("bun", ["run", "x"], { [names.guard]: "report" })).toEqual(["run", "x"]);
    expect(declared.guard.defaultMode).toBe("off");
    expect(guardArgumentsV1("node", ["test"], { [names.guard]: "report" })).toEqual(["test"]);
    expect(guardArgumentsV1("bun", ["test", "--preload", "p", "./a.ts"], { [names.guard]: "report" })).toEqual(["test", "--preload", "p", "./a.ts"]);
    const directory = mkdtempSync(join(tmpdir(), "sb-co-"));
    try {
      mkdirSync(join(directory, "a", "b"), { recursive: true });
      expect(testCheckoutRootV1(join(directory, "a", "b"), { [names.checkout]: "/given/root" })).toBe("/given/root");
      mkdirSync(join(directory, ".git"));
      expect(testCheckoutRootV1(join(directory, "a", "b"), {})).toBe(directory);
    } finally { rmSync(directory, { recursive: true, force: true }); }
  });
  test("ports are leased atomically across sandboxes and processes and released on disposal", async () => {
    const first = createTestSandboxV1({ route: "ports", reclaim: false }), second = createTestSandboxV1({ route: "ports", reclaim: false });
    try {
      const all = await Promise.all([...Array.from({ length: 6 }, () => first.reservePort()), ...Array.from({ length: 6 }, () => second.reservePort())]);
      expect(new Set(all).size).toBe(12);
      for (const port of all) { expect(port).toBeGreaterThanOrEqual(declared.ports.minimum); expect(port).toBeLessThanOrEqual(declared.ports.maximum); expect(readTestPortLeaseV1(first.shared, port)?.pid).toBe(process.pid); }
      const modulePath = join(root, "🟦️.ts"), program = `import {createTestSandboxV1} from ${JSON.stringify(modulePath)};const s=createTestSandboxV1({route:"child",reclaim:false});const ports=await Promise.all([1,2,3,4,5].map(()=>s.reservePort()));console.log(JSON.stringify(ports));await new Promise(r=>setTimeout(r,150));s.dispose();`;
      const children = await Promise.all(Array.from({ length: 3 }, () => new Promise<number[]>((done, fail) => {
        const child = spawn(process.execPath, ["-e", program], { stdio: ["ignore", "pipe", "inherit"] });
        let out = "";
        child.stdout.on("data", chunk => { out += chunk; });
        child.once("error", fail);
        child.once("close", code => code === 0 ? done(JSON.parse(out.trim().split("\n").pop()!)) : fail(Error(`child exited ${code}`)));
      })));
      const everything = [...all, ...children.flat()];
      expect(new Set(everything).size).toBe(everything.length);
      const taken = all[0]!, probe = createServer();
      await new Promise<void>((done, fail) => { probe.once("error", fail); probe.listen(taken, "127.0.0.1", done); });
      probe.close();
      first.dispose();
      for (const port of all.slice(0, 6)) expect(readTestPortLeaseV1(second.shared, port)).toBeUndefined();
      expect(readTestPortLeaseV1(second.shared, all[6]!)?.sandbox).toBe(second.id);
      releaseTestPortV1(second.shared, all[6]!, "someone-else");
      expect(readTestPortLeaseV1(second.shared, all[6]!)?.sandbox).toBe(second.id);
    } finally { first.dispose(); second.dispose(); }
  });
  test("abandoned sandboxes, images and port leases of dead owners are reclaimed and live ones are kept", () => {
    const parent = mkdtempSync(join(tmpdir(), "sb-rc-")), base = join(parent, declared.layout.parent), dead = spawnSync(process.execPath, ["-e", "0"]).pid!;
    const make = (id: string, row: object | undefined, age = 0): string => {
      const directory = join(base, id);
      mkdirSync(join(directory, "images"), { recursive: true });
      if (row) writeFileSync(join(directory, declared.layout.owner), JSON.stringify(row));
      if (age) { const stamp = (Date.now() - age) / 1000; if (row) utimesSync(join(directory, declared.layout.owner), stamp, stamp); utimesSync(directory, stamp, stamp); }
      return directory;
    };
    try {
      const gone = make("aaaaaaaa", { pid: dead }), mine = make("bbbbbbbb", { pid: process.pid }), stale = make("cccccccc", { pid: process.pid }, declared.layout.staleHeartbeatMilliseconds + 60000), torn = make("dddddddd", undefined), old = make("eeeeeeee", undefined, declared.layout.graceMilliseconds + 60000);
      mkdirSync(join(base, declared.ports.directory, "51234"), { recursive: true });
      writeFileSync(join(base, declared.ports.directory, "51234", declared.layout.owner), JSON.stringify({ port: 51234, pid: dead, sandbox: "aaaaaaaa" }));
      mkdirSync(join(base, declared.ports.directory, "51235"), { recursive: true });
      writeFileSync(join(base, declared.ports.directory, "51235", declared.layout.owner), JSON.stringify({ port: 51235, pid: process.pid, sandbox: "bbbbbbbb" }));
      const detached: string[] = [];
      const run = reclaimAbandonedTestSandboxesV1({ parent, force: true, detach: directory => { detached.push(directory); expect(existsSync(directory)).toBe(true); return []; } });
      expect(run.reclaimed.map(path => path.slice(-8)).sort()).toEqual(["aaaaaaaa", "cccccccc", "eeeeeeee"]);
      expect(detached.length).toBe(3);
      expect(existsSync(mine) && existsSync(torn)).toBe(true);
      expect([existsSync(gone), existsSync(stale), existsSync(old)]).toEqual([false, false, false]);
      expect(existsSync(join(base, declared.ports.directory, "51234"))).toBe(false);
      expect(existsSync(join(base, declared.ports.directory, "51235"))).toBe(true);
      expect(reclaimAbandonedTestSandboxesV1({ parent }).throttled).toBe(true);
    } finally { rmSync(parent, { recursive: true, force: true }); }
    const facts = (pid: number | undefined, heartbeat: number, directory = 0) => ({ pid, heartbeatAgeMilliseconds: heartbeat, directoryAgeMilliseconds: directory });
    expect(abandonedTestSandboxV1(facts(1, 0), () => false)).toBe(true);
    expect(abandonedTestSandboxV1(facts(1, 0), () => true)).toBe(false);
    expect(abandonedTestSandboxV1(facts(1, declared.layout.staleHeartbeatMilliseconds + 1), () => true)).toBe(true);
    expect(abandonedTestSandboxV1(facts(undefined, 0, declared.layout.graceMilliseconds - 1))).toBe(false);
    expect(abandonedTestSandboxV1(facts(undefined, 0, declared.layout.graceMilliseconds + 1))).toBe(true);
  });
});

describe("runtime guard", () => {
  const preload = join(root, "🛡️guard/🔌️preload/🟦️.ts");
  const run = async (mode: string, body: string, expectFail: boolean): Promise<{ output: string; report: string; checkout: string }> => {
    const sandbox = createTestSandboxV1({ route: "guard", reclaim: false, guard: mode as "report" | "enforce" }), checkout = mkdtempSync(join(tmpdir(), "sb-co-"));
    try {
      mkdirSync(join(checkout, ".cache"), { recursive: true });
      const file = join(sandbox.tmp, "guard.test.ts");
      writeFileSync(file, `import {test} from "bun:test";import {writeFileSync,mkdirSync,rmSync} from "node:fs";import {writeFile} from "node:fs/promises";import {spawnSync} from "node:child_process";import {createServer} from "node:net";import {join} from "node:path";import {Database} from "bun:sqlite";import Ajv from ${JSON.stringify(Bun.resolveSync("ajv", import.meta.dir))};\nconst checkout=${JSON.stringify(checkout)},tmp=${JSON.stringify(sandbox.tmp)};\n${body}\n`);
      const env: NodeJS.ProcessEnv = { ...process.env, ...sandbox.environment, [names.checkout]: checkout };
      const child = spawnSync(process.execPath, ["test", "--preload", preload, file], { cwd: sandbox.root, env, encoding: "utf8" });
      expect(child.status === 0, child.stdout + child.stderr).toBe(!expectFail);
      return { output: child.stdout + child.stderr, report: existsSync(sandbox.environment[names.report]!) ? readFileSync(sandbox.environment[names.report]!, "utf8") : "", checkout: [...(existsSync(checkout) ? readdirSync(checkout) : []), ...(existsSync(join(checkout, ".cache")) ? readdirSync(join(checkout, ".cache")).map(name => ".cache/" + name) : [])].join(",") };
    } finally { sandbox.dispose(); rmSync(checkout, { recursive: true, force: true }); }
  };
  test("enforce refuses writes into the checkout, kernel image tools outside the sandbox and name-based kills before they act", async () => {
    const write = await run("enforce", `test("w",()=>{writeFileSync(join(tmp,"ok.txt"),"ok");writeFileSync(join(checkout,"leak.txt"),"leak");});`, true);
    expect(write.output).toContain("[sandbox-guard] checkout-write refused: writeFileSync");
    expect(write.checkout).not.toContain("leak.txt");
    const protectedWrite = await run("enforce", `test("p",async()=>{await writeFile(join(checkout,".cache","x.json"),"{}");});`, true);
    expect(protectedWrite.output).toContain("protected-write refused: writeFile");
    expect(protectedWrite.checkout).not.toContain("x.json");
    const kernel = await run("enforce", `test("k",()=>{spawnSync("hdiutil",["attach",join(checkout,".cache","x.dmg")]);});`, true);
    expect(kernel.output).toContain("kernel-image refused: hdiutil attach");
    const kill = await run("enforce", `test("x",()=>{spawnSync("pkill",["-f","semio-test-never-matches"]);});`, true);
    expect(kill.output).toContain("process-kill refused: pkill");
    const database = await run("enforce", `test("d",()=>{new Database(":memory:").close();const ok=new Database(join(tmp,"ok.sqlite"));ok.close();new Database(join(checkout,".cache","lease.sqlite")).close();});`, true);
    expect(database.output).toContain("protected-write refused: Database");
    expect(database.checkout).not.toContain("lease.sqlite");
    const fine = await run("enforce", `test("f",async()=>{if(!new Ajv().compile({type:"object"})({}))throw Error("CommonJS dependency broke");new Database(":memory:").close();new Database(join(tmp,"fine.sqlite"),{readonly:false,create:true}).close();mkdirSync(join(tmp,"d"));await writeFile(join(tmp,"d","f"),"x");rmSync(join(tmp,"d"),{recursive:true});const s=createServer();await new Promise<void>(r=>s.listen(0,"127.0.0.1",r));s.close();});`, false);
    expect(fine.report).toBe("");
  });
  test("report records every violation as a schema-valid row, lets the run finish and flags fixed ports", async () => {
    const ajv = new Ajv({ strict: true });
    ajv.addSchema(schema);
    const sandbox = createTestSandboxV1({ route: "guard-port", reclaim: false }), taken = await sandbox.reservePort(), free = await sandbox.reservePort();
    releaseTestPortV1(sandbox.shared, free, sandbox.id);
    sandbox.dispose();
    const result = await run("report", `test("r",async()=>{writeFileSync(join(checkout,"leak.txt"),"leak");mkdirSync(join(checkout,".cache","b"));const s=createServer();await new Promise<void>(r=>s.listen(${free},"127.0.0.1",r));s.close();});`, false);
    const rows = result.report.split("\n").filter(Boolean).map(line => JSON.parse(line));
    expect(rows.map(row => row.kind).sort()).toEqual(["checkout-write", "fixed-port", "protected-write"]);
    for (const row of rows) expect(ajv.getSchema(`${schema.$id}#/definitions/GuardReportRow`)!(row)).toBe(true);
    expect(result.checkout).toContain("leak.txt");
    expect(taken).toBeGreaterThan(0);
    const summary = summarizeGuardReportV1(result.report);
    expect(summary[0]).toBe("[sandbox-guard] 3 violation(s): checkout-write=1 protected-write=1 fixed-port=1");
    expect(summary.length).toBe(4);
    expect(summarizeGuardReportV1("")).toEqual([]);
  });
  test("every declared write function has a guarded export in the shims", () => {
    const fs = readFileSync(join(root, "🛡️guard/🩻️fs/🟦️.ts"), "utf8"), fsp = readFileSync(join(root, "🛡️guard/🩻️fs-promises/🟦️.ts"), "utf8"), child = readFileSync(join(root, "🛡️guard/🩻️child-process/🟦️.ts"), "utf8");
    for (const name of Object.keys(declared.guard.writeTargets)) expect(fs, name).toContain(`export const ${name} = writes("${name}"`);
    for (const name of ["writeFile", "appendFile", "mkdir", "rm", "rmdir", "unlink", "rename", "copyFile", "cp", "truncate", "symlink", "link"]) expect(fsp, name).toContain(`export const ${name} = writes("${name}"`);
    for (const name of declared.guard.openFunctions) expect(fs, name).toContain(`export const ${name} = opens("${name}"`);
    for (const name of ["spawn", "spawnSync", "execFile", "execFileSync", "exec", "execSync"]) expect(child, name).toContain(`export const ${name} =`);
    for (const name of Object.keys(declared.guard.sqliteTargets)) expect(readFileSync(join(root, "🛡️guard/🩻️sqlite/🟦️.ts"), "utf8"), name).toContain(`class ${name} extends`);
  });
});
