import { expect, test } from "bun:test";
import Ajv from "ajv";
import { existsSync, mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { setImmediate, setTimeout } from "node:timers/promises";
import corpus from "../🧫️fixtures/🔣️.json";
import policySchema from "../../📋️policy/🧬️schema/🔣️.json";
import { runExactCargoLaws, type ExactCargoLawPort } from "../../🟦️.ts";
import { type ScriptInvocation } from "../../../../../🧭️routing/📥️invocation/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../../../../🧬️schema/✅️validator/🟦️.ts";

const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
if (!artifactRoot) throw Error("Original ticket artifact owner required");
const oracleCode = 'const row=JSON.parse(process.argv[1]),policy=JSON.parse(process.argv[2]),directory=process.argv[3],fs=require("node:fs"),timers=require("node:timers/promises"),controller=new AbortController();if(row.mutation==="cancel-before")controller.abort();(async()=>{let probes=0,budget=null,remaining=row.remainingMilliseconds;try{controller.signal.throwIfAborted();if(row.mutation==="missing-policy"||remaining!==null&&remaining<=0||row.maximumElapsedMilliseconds>0&&policy.buildMilliseconds===0)throw Error("refused");await timers.setImmediate(undefined,{signal:controller.signal});if(row.mutation==="publication-refused")throw Error("publisher refused");await timers.setTimeout(5,undefined,{signal:controller.signal});if(row.mutation==="remaining-after-publication")remaining=row.expectedBudgetMilliseconds;controller.signal.throwIfAborted();budget=remaining===null?policy.buildMilliseconds:Math.min(policy.buildMilliseconds,remaining);fs.mkdirSync(directory);probes++;const child=require("node:child_process").spawnSync(process.execPath,["--eval","process.exitCode=101"]);if(child.status!==101)throw Error("oracle failed");}catch(error){if(probes&&String(error).includes("oracle failed"))throw error;}console.log(JSON.stringify({probes,budget,acquired:fs.existsSync(directory)}))})().catch(error=>{console.error(error);process.exitCode=1});';

test("exact Cargo original control corpus has strict policy admission and actual Cargo metadata oracle", async () => {
  expect(existsSync(new URL("../../📋️policy/🧬️schema/🔣️.json", import.meta.url))).toBe(true);
  const admits = new Ajv({ strict: true }).compile(policySchema);
  expect(admits(corpus.policy)).toBe(true);
  expect(validateJsonSchemaSubset(policySchema, corpus.policy)).toEqual([]);
  for (const field of ["version", "buildMilliseconds", "listMilliseconds", "lawMilliseconds", "transport"] as const) {
    const value = { ...corpus.policy };
    delete (value as Partial<typeof corpus.policy>)[field];
    expect(admits(value)).toBe(false);
    expect(validateJsonSchemaSubset(policySchema, value).length).toBeGreaterThan(0);
  }
  const root = mkdtempSync(join(artifactRoot, "exact-control-oracle-"));
  mkdirSync(join(root, "src"));
  writeFileSync(join(root, "Cargo.toml"), '[package]\nname="exact-control-oracle"\nversion="0.1.0"\nedition="2021"\n[workspace]\n');
  writeFileSync(join(root, "src/lib.rs"), "pub fn original_caller() {}\n");
  const controller = new AbortController(), timer = globalThis.setTimeout(() => controller.abort(), corpus.callerCeilingMilliseconds);
  const child = Bun.spawn(["cargo", "metadata", "--manifest-path", join(root, "Cargo.toml"), "--offline", "--no-deps", "--format-version", "1"], { stdout: "pipe", stderr: "pipe" });
  const abort = () => child.kill();
  controller.signal.addEventListener("abort", abort, { once: true });
  try {
    const [stdout, stderr, status] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
    expect(status, stderr).toBe(0);
    const metadata = JSON.parse(stdout);
    expect(metadata.packages).toHaveLength(1);
    expect(metadata.packages[0].name).toBe("exact-control-oracle");
    expect(metadata.packages[0].manifest_path).toBe(join(root, "Cargo.toml"));
    console.log("[DEBUG] exact caller schemas admitted with independent Ajv and actual offline Cargo metadata");
  } finally {
    clearTimeout(timer);
    controller.signal.removeEventListener("abort", abort);
    if (child.exitCode === null) { child.kill(); await child.exited; }
  }
}, corpus.callerCeilingMilliseconds);

for (const row of corpus.cases) test(row.id, async () => {
  const started = performance.now(), controller = new AbortController(), timer = globalThis.setTimeout(() => controller.abort(), corpus.callerCeilingMilliseconds);
  const root = mkdtempSync(join(artifactRoot, "exact-control-")), output = join(root, "actual"), oracleOutput = join(root, "oracle");
  let remaining = row.remainingMilliseconds, probes = 0, budget: number | null = null, originalControl = false;
  const ledger: string[] = [], pending: Promise<void>[] = [];
  if (row.mutation === "cancel-before") controller.abort();
  const invocation: ScriptInvocation = {
    policy: { version: 1, owner: row.id, maximumElapsedMilliseconds: row.maximumElapsedMilliseconds },
    capabilities: { artifactDirectory: output },
    control: {
      signal: controller.signal,
      remainingMilliseconds: () => remaining === null ? null : Math.min(remaining, row.maximumElapsedMilliseconds - (performance.now() - started)),
      async publish(event) { ledger.push("original:" + event.stage); await setImmediate(); if (row.mutation === "publication-refused") throw Error("Original publisher refused"); },
      async yieldContinuation() { ledger.push("yield"); await setImmediate(); }
    }
  };
  const policy = { ...corpus.policy, buildMilliseconds: ["finite-zero-build", "ordinary-zero-build"].includes(row.mutation) ? 0 : corpus.policy.buildMilliseconds };
  const progress = () => {
    const publication = (async () => { ledger.push("publication"); await setTimeout(5); if (row.mutation === "remaining-after-publication") remaining = row.expectedBudgetMilliseconds; if (row.mutation === "publication-refused") throw Error("Original publisher refused"); })();
    pending.push(publication);
    publication.catch(() => {});
    return publication;
  };
  const port: ExactCargoLawPort = {
    fingerprint() { throw Error("Refused compiler has no executable"); },
    async probe(_command, _args, capture) {
      ledger.push("probe"); probes++; budget = capture.budgetMs;
      originalControl = (capture as typeof capture & { invocation: ScriptInvocation }).invocation === invocation;
      return { status: 101, signal: null, stdout: "", stderr: "Original compiler refusal witness" };
    }
  };
  const oracle = Bun.spawn(["node", "--eval", oracleCode, JSON.stringify(row), JSON.stringify(policy), oracleOutput], { stdout: "pipe", stderr: "pipe" });
  try {
    try {
      await runExactCargoLaws({ cwd: root, artifactDir: output, cargoTargetDir: join(root, "compiler"), manifestPaths: { fixture: join(root, "Cargo.toml") }, groups: [{ package: "fixture", target: { kind: "lib" }, laws: ["original_law"] }], env: process.env, invocation, ...(row.mutation === "missing-policy" ? {} : { policy }), progress } as Parameters<typeof runExactCargoLaws>[0], port);
    } catch (error) { ledger.push("refused:" + String(error)); }
    const [stdout, stderr, status] = await Promise.all([new Response(oracle.stdout).text(), new Response(oracle.stderr).text(), oracle.exited]);
    expect(status, stderr).toBe(0);
    const actual = { probes, budget, acquired: existsSync(output), originalControl }, expected = { probes: row.expectedProbes, budget: row.expectedBudgetMilliseconds, acquired: row.expectedAcquisition, originalControl: row.expectedProbes > 0 };
    const observed = JSON.parse(stdout);
    console.log("[DEBUG] " + JSON.stringify({ id: row.id, actual, oracle: observed, ledger }));
    expect(observed).toEqual({ probes: expected.probes, budget: expected.budget, acquired: expected.acquired });
    expect<typeof expected>(actual).toEqual(expected);
    if (probes) expect(ledger.indexOf("probe")).toBeGreaterThan(ledger.indexOf("publication"));
  } finally {
    await Promise.allSettled(pending);
    clearTimeout(timer);
    if (oracle.exitCode === null) { oracle.kill(); await oracle.exited; }
  }
}, corpus.callerCeilingMilliseconds);
