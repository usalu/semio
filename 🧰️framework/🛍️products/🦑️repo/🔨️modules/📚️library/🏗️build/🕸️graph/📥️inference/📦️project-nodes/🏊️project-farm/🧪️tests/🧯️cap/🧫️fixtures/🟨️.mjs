import { randomFillSync } from "node:crypto";
import { readFileSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";

export async function open(session) { return session.identity; }
export async function run(job) {
  const start = Date.now(), counter = join(job.dir, "run-" + job.n);
  let run = 0;
  try { run = Number(readFileSync(counter, "utf8")); } catch {}
  writeFileSync(counter, String(run + 1));
  const megabytes = job.megabytes[Math.min(run, job.megabytes.length - 1)];
  writeFileSync(join(job.dir, "pid-" + process.pid), String(megabytes));
  const held = job.real ? randomFillSync(Buffer.allocUnsafe(megabytes * 1048576)) : undefined;
  if (held !== undefined) held[held.length - 1] = 73;
  await new Promise(done => setTimeout(done, job.delay));
  if (!job.real) rmSync(join(job.dir, "pid-" + process.pid), { force: true });
  return { n: job.n, run, pid: process.pid, start, end: Date.now(), held: held?.length ?? 0, sentinel: held?.at(-1) };
}
