import assert from "node:assert/strict";
import Ajv from "ajv";
import { mkdtempSync, mkdirSync, writeFileSync, rmSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import corpus from "../🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import { cargoBuildLeaseIdentityV1 } from "../🟦️.ts";

/** 🚦️ Verifies portable profile identities and actual cross-process ownership, cancellation and crash recovery. */
export async function proveCargoBuildLeasesV1(artifactRoot: string): Promise<void> {
  mkdirSync(artifactRoot, { recursive: true });
  const root = mkdtempSync(join(artifactRoot, "cargo-profile-lease-"));
  const implementation = fileURLToPath(new URL("../🟦️.ts", import.meta.url));
  const worker = join(root, "📜️script.ts");
  const oracle = new Ajv({ strict: true }).addSchema(schema);
  assert(oracle.getSchema(schema.$id + "#/$defs/PortableCorpusV1")!(corpus));
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
  } finally { for (const child of children) if (child.process.exitCode === null) child.process.kill("SIGKILL"); await Promise.all(children.map(child => child.done)); rmSync(root, { recursive: true, force: true }); }
}
