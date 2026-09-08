import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { copyFileSync, linkSync, mkdtempSync, readFileSync, readdirSync, rmSync, statSync, symlinkSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, join } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

/** 🔒️ Compares resource ownership against independent Node, Bun and Python SQLite processes. */
export async function testResourceLeases(generated: string): Promise<void> {
  const moduleRoot = join(dirname(fileURLToPath(import.meta.url)), ".."), implementation = join(moduleRoot, "🟦️.ts");
  const fixture = JSON.parse(readFileSync(join(moduleRoot, "🧫️cases.json"), "utf8"));
  assert.ok(new (createRequire(import.meta.url)("ajv").default)().validate(JSON.parse(readFileSync(join(moduleRoot, "🧬️schema/🔣️.json"), "utf8")), fixture));
  const api = await import(pathToFileURL(implementation).href);
  const root = mkdtempSync(join(generated, "resource-leases-")), directory = join(root, "store");
  const worker = join(root, "📜️script.ts");
  writeFileSync(worker, `import { acquireResourceLease } from ${JSON.stringify(implementation)};
const [directory, resource, mode] = process.argv.slice(2), controller = new AbortController();
const timer = setTimeout(() => controller.abort(), 15000);
process.once("SIGTERM", () => controller.abort());
let stop; const stopped = new Promise(resolve => { stop = resolve; });
process.stdin.on("data", chunk => { if (String(chunk).trim() === "cancel") controller.abort(); stop(); });
try {
  const lease = await acquireResourceLease({ directory, resource, mode, signal: controller.signal, onWait: () => console.log("waiting") });
  try { console.log("held"); await stopped; }
  finally { lease.release(); console.log("released"); }
} catch (error) { if (controller.signal.aborted) console.log("cancelled"); else throw error; }
finally { clearTimeout(timer); process.stdin.destroy(); }
`);
  const resources = [...new Set(fixture.cases.flatMap(row => [row.first.resource, row.second.resource]))] as string[];
  for (const resource of new Set(fixture.cases.filter(row => row.first.runtime === "python").map(row => row.first.resource))) (await api.acquireResourceLease({ directory, resource, mode: "exclusive", signal: new AbortController().signal })).release();
  const python = `import sqlite3,sys
db=sqlite3.connect(sys.argv[1], timeout=0, isolation_level=None)
db.execute("BEGIN EXCLUSIVE" if sys.argv[2] == "exclusive" else "BEGIN")
db.execute("SELECT resource FROM semio_resource_lease").fetchone()
print("held", flush=True)
sys.stdin.readline()
db.rollback()
db.close()
print("released", flush=True)
`;
  const children: { process: ReturnType<typeof Bun.spawn>; output: string; errors: string; done: Promise<number> }[] = [];
  const start = (row: { runtime: string; resource: string; mode: string }) => {
    const database = join(directory, createHash("sha256").update(row.resource).digest("hex") + ".sqlite");
    const args = row.runtime === "python" ? [Bun.which("python3") ?? Bun.which("python") ?? "python3", "-u", "-c", python, database, row.mode] : [row.runtime === "bun" ? process.execPath : "node", worker, directory, row.resource, row.mode];
    const child = { process: Bun.spawn(args, { stdout: "pipe", stderr: "pipe", stdin: "pipe" }), output: "", errors: "", done: undefined as unknown as Promise<number> };
    const drain = async (stream, key: "output" | "errors") => { for await (const chunk of stream) child[key] += Buffer.from(chunk).toString(); };
    child.done = Promise.all([drain(child.process.stdout, "output"), drain(child.process.stderr, "errors"), child.process.exited]).then(rows => rows[2]);
    children.push(child); return child;
  };
  const until = async (child, phase: string) => { const deadline = Date.now() + 10000; while (!child.output.split("\n").includes(phase)) { assert.equal(child.process.exitCode, null, child.errors || child.output); assert.ok(Date.now() < deadline, `${phase}: ${child.output} ${child.errors}`); await Bun.sleep(20); } };
  const release = async child => { child.process.stdin.write("release\n"); child.process.stdin.end(); assert.equal(await child.done, 0, child.errors); };
  try {
    for (const row of fixture.cases) {
      const first = start(row.first); await until(first, "held");
      const second = start(row.second);
      if (row.action === "parallel") { await until(second, "held"); await release(second); await release(first); }
      else {
        await until(second, "waiting"); assert.equal(second.output.includes("held"), false);
        if (row.action === "cancel-second") {
          second.process.stdin.write("cancel\n"); second.process.stdin.end(); assert.equal(await second.done, 0, second.errors); assert.ok(second.output.includes("cancelled"));
          assert.equal(first.process.exitCode, null); await release(first); assert.equal(second.output.includes("held"), false);
        } else {
          if (row.action === "kill-first") { first.process.kill("SIGKILL"); await first.done; }
          else await release(first);
          await until(second, "held"); await release(second);
        }
      }
      console.log(`[DEBUG] Resource leases ${row.name}: ${row.first.runtime}/${row.second.runtime} ${row.action} PASS`);
    }
    const files = readdirSync(directory).sort(), before = files.map(file => statSync(join(directory, file)).size);
    for (let index = 0; index < fixture.repetitions; index++) (await api.acquireResourceLease({ directory, resource: resources[0], mode: "exclusive", signal: new AbortController().signal })).release();
    assert.deepEqual(readdirSync(directory).sort(), files);
    assert.deepEqual(files.map(file => statSync(join(directory, file)).size), before);
    const abort = new AbortController(); abort.abort();
    await assert.rejects(() => api.acquireResourceLease({ directory, resource: "cancelled-before-open", mode: "shared", signal: abort.signal }), /abort/i);
    assert.deepEqual(readdirSync(directory).sort(), files);
    console.log(`[DEBUG] Resource leases: ${fixture.repetitions} repeated acquisitions retain exactly ${files.length} constant-size resource databases PASS`);
    const heldController = new AbortController(), owner = await api.acquireResourceLease({ directory, resource: "deadline", mode: "exclusive", signal: heldController.signal });
    try {
      heldController.abort();
      const progress: unknown[] = [];
      await assert.rejects(() => api.acquireResourceLease({ directory, resource: "deadline", mode: "shared", signal: new AbortController().signal, timeoutMs: 60, onWait: row => progress.push(row) }), /timed out/i);
      assert.ok(progress.length > 0, "Contended acquisition must report waiting");
      await assert.rejects(() => api.acquireResourceLease({ directory, resource: "deadline", mode: "shared", signal: new AbortController().signal, onWait: () => { throw new Error("progress consumer failed"); } }), /progress consumer failed/);
    } finally { owner.release(); owner.release(); }
    (await api.acquireResourceLease({ directory, resource: "deadline", mode: "exclusive", signal: new AbortController().signal })).release();
    let active = 0, maximum = 0, completed = 0;
    const operation = async () => { active++; maximum = Math.max(maximum, active); await Bun.sleep(40); completed++; active--; };
    await Promise.all([false, true].map(reverse => api.withResourceLeases({ directory, signal: new AbortController().signal, onWait: () => {}, resources: (reverse ? ["b", "a"] : ["a", "b"]).map(resource => ({ resource, mode: "exclusive" })) }, operation)));
    assert.equal(maximum, 1); assert.equal(completed, 2);
    await assert.rejects(() => api.withResourceLeases({ directory, signal: new AbortController().signal, resources: [{ resource: "a", mode: "exclusive" }, { resource: "a", mode: "shared" }] }, async () => { throw new Error("consumer failed"); }), /consumer failed/);
    (await api.acquireResourceLease({ directory, resource: "a", mode: "exclusive", signal: new AbortController().signal })).release();
    const databasePath = (resource: string) => join(directory, createHash("sha256").update(resource).digest("hex") + ".sqlite");
    const expectRejected = (resource: string, pattern: RegExp) => assert.rejects(() => api.acquireResourceLease({ directory, resource, mode: "exclusive", signal: new AbortController().signal }), pattern);
    copyFileSync(databasePath(resources[0]), databasePath("wrong-identity"));
    await expectRejected("wrong-identity", /identity mismatch/);
    const { Database } = await import("bun:sqlite");
    const foreign = new Database(databasePath("foreign")); foreign.exec("CREATE TABLE foreign_owner (value)"); foreign.close();
    await expectRejected("foreign", /unowned/i);
    (await api.acquireResourceLease({ directory, resource: "wrong-journal", mode: "exclusive", signal: new AbortController().signal })).release();
    const wal = new Database(databasePath("wrong-journal")); wal.exec("PRAGMA journal_mode=WAL"); wal.close();
    await expectRejected("wrong-journal", /rollback journal/);
    linkSync(databasePath(resources[0]), databasePath("hardlink"));
    try { await expectRejected("hardlink", /invalid.*database/i); }
    finally { rmSync(databasePath("hardlink")); }
    symlinkSync(directory, join(root, "linked-store"), process.platform === "win32" ? "junction" : "dir");
    await assert.rejects(() => api.acquireResourceLease({ directory: join(root, "linked-store"), resource: "linked", mode: "exclusive", signal: new AbortController().signal }), /invalid.*directory/i);
    console.log("[DEBUG] Resource lease deadlines, cancellation while held, progress failure, ordered multi-resource access, consumer failure, identity/journal and path guards PASS");
  } finally {
    for (const child of children) if (child.process.exitCode === null) child.process.kill("SIGKILL");
    await Promise.all(children.map(child => child.done));
    rmSync(root, { recursive: true, force: true });
  }
}
