import { expect, test } from "bun:test";
import Ajv from "ajv";
import { existsSync, mkdtempSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { setImmediate, setTimeout } from "node:timers/promises";
import corpus from "../🧫️fixtures/🔣️.json";
import policySchema from "../../📋️policy/🧬️schema/🔣️.json";
import { captureOwnedProcess, type OwnedProcessCaptureResult } from "../../🟦️.ts";
import { type ScriptInvocation } from "../../../🧭️routing/📥️invocation/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../../🧬️schema/✅️validator/🟦️.ts";

const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
if (!artifactRoot) throw Error("Original ticket artifact owner required");
test("capture policy admits exact configured output authority and refuses unsafe extents", () => {
  const oracle = new Ajv({ strict: true }).compile(policySchema);
  for (const row of corpus.policyCases) {
    const value = { budgetMs: row.budgetMs, maxOutputBytes: row.maxOutputBytes };
    expect(oracle(value)).toBe(row.accepted);
    expect(validateJsonSchemaSubset(policySchema, value).length === 0).toBe(row.accepted);
  }
  console.log("[DEBUG] Original capture policy safe-integer extents agree with Ajv");
});
test("a finite original capture stays finite with an explicit zero child ceiling", async () => {
  const row = corpus.finiteWithoutChildCeiling, started = performance.now(), root = mkdtempSync(join(artifactRoot, "capture-zero-child-")), controller = new AbortController();
  const invocation: ScriptInvocation = { policy: { version: 1, owner: "finite-zero-child", maximumElapsedMilliseconds: row.originalMilliseconds }, capabilities: { artifactDirectory: root }, control: { signal: controller.signal, remainingMilliseconds: () => row.originalMilliseconds - (performance.now() - started), publish() {}, async yieldContinuation() { await setImmediate(); } } };
  const result = await captureOwnedProcess(process.execPath, ["--eval", "process.stdout.write('ready');setInterval(()=>{},1000)"], { invocation, cwd: root, env: process.env, budgetMs: row.childMilliseconds, maxOutputBytes: row.maximumOutputBytes, stdoutPath: join(root, "stdout"), stderrPath: join(root, "stderr") });
  expect(result.reason).toBe(row.reason);
  expect(result.stdout).toBe("ready");
  expect(performance.now() - started).toBeLessThan(row.maximumObservedMilliseconds);
  console.log("[DEBUG] Original finite capture expired through same remaining port with zero extra child ceiling");
});
const oracleCode = String.raw`const c=JSON.parse(process.argv[1]),row=JSON.parse(process.argv[2]),controller=new AbortController(),timers=require("node:timers/promises"),spawn=require("node:child_process").spawn;let published=false,stdout="";(async()=>{if(row.mode==="pre-cancel"){controller.abort();console.log(JSON.stringify({reason:"cancelled",status:null,stdout:"",acquired:false,published:false}));return}const timer=setTimeout(()=>controller.abort(),c.operationMilliseconds);try{await timers.setTimeout(c.publicationMilliseconds,undefined,{signal:controller.signal});published=true;const child=spawn(process.execPath,["--eval",row.mode==="live-cancel"?"process.stdout.write(\"ready\");setInterval(()=>{},1000)":"process.stdout.write(\"ready\")"],{signal:controller.signal});child.stdout.on("data",bytes=>{stdout+=bytes;if(row.mode==="live-cancel"&&stdout==="ready")controller.abort()});child.stderr.resume();child.on("error",error=>{if(error.code!=="ABORT_ERR")throw error});const status=await new Promise(accept=>child.once("close",accept));if(!controller.signal.aborted)await timers.setTimeout(c.publicationMilliseconds);console.log(JSON.stringify({reason:controller.signal.aborted?"cancelled":"exit",status,stdout,acquired:true,published}));}finally{clearTimeout(timer)}})().catch(error=>{console.error(error);process.exitCode=1});`;

for (const row of corpus.cases) test(row.id, async () => {
  const started = performance.now(), controller = new AbortController(), root = mkdtempSync(join(artifactRoot, "capture-control-"));
  const stdoutPath = join(root, "stdout"), stderrPath = join(root, "stderr"), pending: Promise<void>[] = [], ledger: string[] = [];
  let published = false;
  const timer = globalThis.setTimeout(() => controller.abort(), corpus.operationMilliseconds);
  const monitor = row.mode === "live-cancel" ? setInterval(() => { if (existsSync(stdoutPath) && readFileSync(stdoutPath, "utf8") === "ready") controller.abort(); }, 5) : undefined;
  if (row.mode === "pre-cancel") controller.abort();
  const invocation: ScriptInvocation = {
    policy: { version: 1, owner: row.id, maximumElapsedMilliseconds: corpus.operationMilliseconds },
    capabilities: { artifactDirectory: root },
    control: {
      signal: controller.signal,
      remainingMilliseconds: () => corpus.operationMilliseconds - (performance.now() - started),
      publish(event) { const publication = (async () => { ledger.push(event.stage); await setTimeout(corpus.publicationMilliseconds); published = true; })(); pending.push(publication); return publication; },
      async yieldContinuation() { ledger.push("yield"); await setImmediate(); }
    }
  };
  const oracle = Bun.spawn(["node", "--eval", oracleCode, JSON.stringify(corpus), JSON.stringify(row)], { stdout: "pipe", stderr: "pipe" });
  try {
    const policy = { budgetMs: corpus.operationMilliseconds, maxOutputBytes: corpus.maximumOutputBytes };
    expect(new Ajv({ strict: true }).compile(policySchema)(policy)).toBe(true);
    expect(validateJsonSchemaSubset(policySchema, policy)).toEqual([]);
    let result: OwnedProcessCaptureResult | undefined, failure = "";
    try {
      result = await captureOwnedProcess(process.execPath, ["--eval", row.mode === "live-cancel" ? 'process.stdout.write("ready");setInterval(()=>{},1000)' : 'process.stdout.write("ready")'], { cwd: root, env: process.env, budgetMs: corpus.operationMilliseconds, maxOutputBytes: corpus.maximumOutputBytes, stdoutPath, stderrPath, invocation } as Parameters<typeof captureOwnedProcess>[2]);
    } catch (error) { failure = String(error); }
    const actual = { reason: result?.reason ?? "thrown", status: result?.status ?? null, stdout: result?.stdout ?? "", acquired: existsSync(stdoutPath) || existsSync(stderrPath), published };
    const [stdout, stderr, status] = await Promise.all([new Response(oracle.stdout).text(), new Response(oracle.stderr).text(), oracle.exited]);
    expect(status, stderr).toBe(0);
    const observed = JSON.parse(stdout), expected = { reason: row.reason, status: row.status, stdout: row.stdout, acquired: row.acquired, published: row.published };
    console.log("[DEBUG] " + JSON.stringify({ id: row.id, actual, oracle: observed, ledger, failure }));
    expect(observed).toEqual(expected);
    expect(actual).toEqual(expected);
  } finally {
    clearTimeout(timer);
    if (monitor !== undefined) clearInterval(monitor);
    await Promise.allSettled(pending);
    if (oracle.exitCode === null) { oracle.kill(); await oracle.exited; }
  }
}, corpus.callerCeilingMilliseconds);
