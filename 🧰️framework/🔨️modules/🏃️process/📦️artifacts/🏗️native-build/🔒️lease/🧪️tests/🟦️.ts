import assert from "node:assert/strict";
import Ajv from "ajv";
import { existsSync, mkdtempSync, mkdirSync, readFileSync, readdirSync, realpathSync, writeFileSync, rmSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import corpus from "../🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import { admitCargoPrebuildPlanV1, cargoBuildLeaseIdentityV1, cargoPrebuildJobsV1, cargoPrebuildKeyV1, cargoPrebuildPassesV1, cargoPrebuildRequestV1, ensureCargoPrebuildV1, type CargoPrebuildPlanV1 } from "../🟦️.ts";

/** 🚦️ Verifies portable profile identities and actual cross-process ownership, cancellation and crash recovery. */
export async function proveCargoBuildLeasesV1(artifactRoot: string): Promise<void> {
  mkdirSync(artifactRoot, { recursive: true });
  const root = mkdtempSync(join(artifactRoot, "cargo-profile-lease-"));
  const implementation = fileURLToPath(new URL("../🟦️.ts", import.meta.url));
  const worker = join(root, "📜️script.ts");
  const oracle = new Ajv({ strict: true }).addSchema(schema);
  
  const validate = oracle.getSchema(schema.$id)!;
  const children: { process: Bun.Subprocess<"pipe", "pipe", "pipe">; output: string; error: string; done: Promise<number> }[] = [];
  writeFileSync(worker, `import { acquireCargoBuildLeaseV1 } from ${JSON.stringify(implementation)};
const [directory, buildDirectory, profile] = process.argv.slice(2), controller = new AbortController();
let finish; const finished = new Promise(resolve => { finish = resolve; });
process.stdin.on("data", data => { if (String(data).trim() === "cancel") controller.abort(); finish(); });
const timeout = setTimeout(() => controller.abort(), 15000);
try {
 const lease = await acquireCargoBuildLeaseV1({ directory, buildDirectory, args: ["--profile", profile], signal: controller.signal, onWait: () => console.log("waiting") });
 try { console.log("held"); await finished; } finally { lease.release(); console.log("released"); }
} catch (error) { if (controller.signal.aborted) console.log("cancelled"); else throw error; }
finally { clearTimeout(timeout); process.stdin.destroy(); }
`);
  const until = async (child: typeof children[number], event: string): Promise<void> => { const deadline = Date.now() + 10000; while (!child.output.split("\n").includes(event)) { assert.equal(child.process.exitCode, null, child.error || child.output); assert.ok(Date.now() < deadline, event + ": " + child.output); await Bun.sleep(20); } };
  const start = (profile: string, runtime: string) => {
    const child = { process: Bun.spawn([runtime, worker, join(root, "store"), join(root, "build"), profile], { stdin: "pipe", stdout: "pipe", stderr: "pipe" }), output: "", error: "", done: undefined as unknown as Promise<number> };
    const drain = async (stream: ReadableStream<Uint8Array>, key: "output" | "error") => { for await (const bytes of stream) child[key] += new TextDecoder().decode(bytes); };
    child.done = Promise.all([drain(child.process.stdout, "output"), drain(child.process.stderr, "error"), child.process.exited]).then(rows => rows[2]);
    children.push(child); return child;
  };
  const release = async (child: typeof children[number]) => { child.process.stdin.write("release\n"); child.process.stdin.end(); assert.equal(await child.done, 0, child.error); };
  try {
    for (const row of corpus.identities) {
      const identity = cargoBuildLeaseIdentityV1(join(root, "build"), row.args);
      assert.equal(identity.profile, row.profile);
      assert(validate(identity), JSON.stringify(validate.errors));
      assert.deepEqual(JSON.parse(JSON.stringify(identity)), identity);
    }
    for (const args of corpus.hostileArgs) assert.throws(() => cargoBuildLeaseIdentityV1(join(root, "build"), args));
    for (const row of corpus.cases) {
      const first = start(row.first, process.execPath); await until(first, "held");
      const second = start(row.second, "node");
      if (row.action === "parallel") { await until(second, "held"); await release(second); await release(first); }
      else {
        await until(second, "waiting"); assert.equal(second.output.includes("held"), false);
        if (row.action === "cancel-second") { second.process.stdin.write("cancel\n"); second.process.stdin.end(); assert.equal(await second.done, 0, second.error); assert.equal(second.output.includes("held"), false); await release(first); }
        else { if (row.action === "kill-first") { first.process.kill("SIGKILL"); await first.done; } else await release(first); await until(second, "held"); await release(second); }
      }
    }
    const { testResourceLeases } = await import("../../../../🔒️leases/🧪️tests/🔒️resource-leases/🟦️.ts");
    await testResourceLeases(root);
    await proveCargoPrebuildV1(root);
  } finally { for (const child of children) if (child.process.exitCode === null) child.process.kill("SIGKILL"); await Promise.all(children.map(child => child.done)); rmSync(root, { recursive: true, force: true }); }
}

/** 🔥️ Verifies the prebuild plan contract, job budget, request selection and the once-per-profile pass against an independent schema oracle and a stub compiler. */
export async function proveCargoPrebuildV1(root: string): Promise<void> {
  const fixture = corpus.prebuild;
  const oracle = new Ajv({ strict: true }).compile(schema.definitions.PrebuildPlan);
  assert(oracle(fixture.plan.valid), JSON.stringify(oracle.errors));
  assert.deepEqual(admitCargoPrebuildPlanV1(fixture.plan.valid), fixture.plan.valid);
  for (const invalid of fixture.plan.invalid) { assert.equal(oracle(invalid), false, JSON.stringify(invalid)); assert.throws(() => admitCargoPrebuildPlanV1(invalid), /Invalid Cargo prebuild plan/u); }
  for (const row of fixture.jobs) { assert.equal(cargoPrebuildPassesV1(row.memoryGiB * 1024 ** 3), row.passes); assert.equal(cargoPrebuildJobsV1(row.cpus, row.memoryGiB * 1024 ** 3), row.jobs); }
  assert.equal(cargoPrebuildJobsV1(10, 32 * 1024 ** 3, { SEMIO_CARGO_PREBUILD_JOBS: fixture.jobsOverride.value }), fixture.jobsOverride.jobs);
  for (const value of fixture.jobsOverride.invalid) assert.throws(() => cargoPrebuildJobsV1(10, 32 * 1024 ** 3, { SEMIO_CARGO_PREBUILD_JOBS: value }));
  const plan = fixture.plan.valid as CargoPrebuildPlanV1;
  for (const row of fixture.requests) {
    const request = cargoPrebuildRequestV1(plan, row.args);
    if (row.workspace === null) assert.equal(request, null, row.name);
    else { assert.deepEqual(request, { workspace: plan.workspaces[row.workspace], profile: row.profile, target: row.target }, row.name); }
  }
  const first = cargoPrebuildRequestV1(plan, fixture.requests[0]!.args)!;
  const shuffled = { ...first, workspace: { ...first.workspace, packages: [...first.workspace.packages].reverse() } };
  assert.equal(cargoPrebuildKeyV1("/build", first), cargoPrebuildKeyV1("/build", shuffled));
  assert.notEqual(cargoPrebuildKeyV1("/build", first), cargoPrebuildKeyV1("/build", { ...first, profile: "wasm-dev" }));
  assert.notEqual(cargoPrebuildKeyV1("/build", first), cargoPrebuildKeyV1("/other", first));

  const directory = mkdtempSync(join(root, "cargo-prebuild-")), calls = join(directory, "calls.jsonl"), stub = join(directory, "📜️script.ts");
  writeFileSync(stub, `import { appendFileSync } from "node:fs";
appendFileSync(${JSON.stringify(calls)}, JSON.stringify({ at: Date.now(), args: process.argv.slice(2), target: process.env.CARGO_TARGET_DIR, build: process.env.CARGO_BUILD_BUILD_DIR, unification: process.env.CARGO_RESOLVER_FEATURE_UNIFICATION, unstable: process.env.CARGO_UNSTABLE_FEATURE_UNIFICATION }) + "\\n");
const artifact = (name) => JSON.stringify({ reason: "compiler-artifact", package_id: "path+file:///w/" + name + "#" + name + "@0.1.0", target: { kind: ["cdylib", "rlib"] }, filenames: ["/x/" + name + ".rlib", "/x/" + name + ".wasm"] }) + "\\n";
console.error("   Compiling hub-a v0.1.0");
console.error("warning: stub");
process.stdout.write(JSON.stringify({ reason: "compiler-artifact", package_id: "path+file:///w/dep#dep@0.1.0", target: { kind: ["lib"] }, filenames: ["/x/dep.rlib"] }) + "\\n");
process.stdout.write(artifact("hub-a"));
await new Promise((accept) => setTimeout(accept, Number(process.env.STUB_DELAY ?? 0)));
process.stdout.write(artifact("hub-b"));
appendFileSync(${JSON.stringify(calls)}, JSON.stringify({ end: Date.now(), profile: process.argv[process.argv.indexOf("--profile") + 1] }) + "\\n");
if (process.env.STUB_EXIT) { console.error("error: stub failure"); process.exit(Number(process.env.STUB_EXIT)); }
`);
  const buildDirectory = join(directory, "build"), store = join(directory, "store"), prebuilds = join(store, "cargo-prebuild");
  mkdirSync(buildDirectory, { recursive: true });
  const run = (args: readonly string[], environment: Record<string, string> = {}, signal = new AbortController().signal, passes: 1 | 2 = 2) => ensureCargoPrebuildV1({ plan, args, cwd: directory, directory: store, buildDirectory, environment: { ...process.env, ...environment }, jobs: 4, passes, signal, commandPort: { command: process.execPath, args: [stub] } });
  const invocations = (): any[] => (existsSync(calls) ? readFileSync(calls, "utf8").trim().split("\n").filter(Boolean).map((line) => JSON.parse(line)).filter((row) => row.args) : []);
  const markers = (): number => readdirSync(prebuilds).filter((name) => name.endsWith(".json")).length;
  const release = fixture.requests[0]!.args, hubA = ["-p", "hub-a", ...release.slice(2)], development = [...release.slice(0, -1), "wasm-dev"];
  let runnerDone = false;
  const runner = run(release, { STUB_DELAY: "1500" }).then((outcome) => { runnerDone = true; return outcome; });
  while (invocations().length === 0) await Bun.sleep(20);
  assert.equal(await run(hubA), "ready", "a waiter is released by its own package");
  assert.equal(runnerDone, false, "the waiter did not wait for the whole pass");
  assert.equal(await runner, "ran");
  assert.equal(await run(release), "ready", "a finished profile is prebuilt once");
  assert.equal(await run(fixture.requests[2]!.args), "skipped", "an unplanned package never triggers a pass");
  assert.equal(invocations().length, 1);
  const call = invocations()[0];
  assert.deepEqual(call.args.slice(0, 3), ["build", "--locked", "--manifest-path"]);
  assert.equal(call.args[3], plan.workspaces[0]!.manifest);
  assert.deepEqual(call.args.filter((_: string, index: number) => call.args[index - 1] === "-p"), plan.workspaces[0]!.packages);
  for (const flag of ["--lib", "--keep-going"]) assert(call.args.includes(flag), flag);
  assert.equal(call.args[call.args.indexOf("--jobs") + 1], "4");
  assert.equal(call.args[call.args.indexOf("--profile") + 1], "wasm-release");
  assert(call.args.includes("--message-format=json-render-diagnostics"));
  assert.equal(call.unification, "package");
  assert.equal(call.unstable, "true");
  assert.equal(call.build, buildDirectory);
  assert.notEqual(call.target, buildDirectory);
  assert.equal(existsSync(call.target), false, "the temporary uplift directory is removed");
  assert.equal(markers(), 1);
  assert.deepEqual(readdirSync(join(prebuilds, readdirSync(prebuilds).find((name) => name.endsWith(".ready"))!)).sort(), plan.workspaces[0]!.packages, "only linked planned components are announced");
  assert.equal(await run(development, { STUB_EXIT: "1" }), "ran", "a failing pass never fails the build");
  assert.equal(invocations().length, 2);
  assert.equal(invocations()[1].args[invocations()[1].args.indexOf("--profile") + 1], "wasm-dev");
  assert.equal(await run(development), "ready", "a failed pass is recorded and not repeated");
  const stale = [...release.slice(0, -1), "wasm-stale"], staleRequest = cargoPrebuildRequestV1(plan, stale)!;
  writeFileSync(join(prebuilds, `${cargoPrebuildKeyV1(realpathSync(buildDirectory).replaceAll("\\", "/").normalize("NFC"), staleRequest)}.run`), "2147483646\n");
  assert.equal(await run(stale), "ran", "a claim whose process died is taken over");
  const events = (): any[] => readFileSync(calls, "utf8").trim().split("\n").filter(Boolean).map((line) => JSON.parse(line));
  const span = (profile: string): { start: number; end: number } => ({ start: events().find((row) => row.args?.includes(profile))!.at, end: events().find((row) => row.end && row.profile === profile)!.end });
  const withProfile = (profile: string) => [...release.slice(0, -1), profile];
  await Promise.all([run(withProfile("wasm-seq-a"), { STUB_DELAY: "600" }, undefined, 1), run(withProfile("wasm-seq-b"), {}, undefined, 1)]);
  const [serialFirst, serialSecond] = ["wasm-seq-a", "wasm-seq-b"].map(span).sort((left, right) => left.start - right.start);
  assert(serialSecond!.start >= serialFirst!.end, "one pass at a time where memory cannot hold two largest units");
  await Promise.all([run(withProfile("wasm-par-a"), { STUB_DELAY: "600" }), run(withProfile("wasm-par-b"))]);
  const [early, late] = ["wasm-par-a", "wasm-par-b"].map(span).sort((left, right) => left.start - right.start);
  assert(late!.start < early!.end, "passes of different profiles overlap where memory holds both");
  const aborted = new AbortController(), pending = run([...release.slice(0, -1), "wasm-test"], { STUB_DELAY: "5000" }, aborted.signal);
  while (invocations().length < 8) await Bun.sleep(20);
  aborted.abort();
  await assert.rejects(pending);
  assert.equal(markers(), 7, "a cancelled pass leaves no marker");
  assert.equal(readdirSync(prebuilds).filter((name) => name.endsWith(".run")).length, 0, "a cancelled pass releases its claim");
  rmSync(directory, { recursive: true, force: true });
}
