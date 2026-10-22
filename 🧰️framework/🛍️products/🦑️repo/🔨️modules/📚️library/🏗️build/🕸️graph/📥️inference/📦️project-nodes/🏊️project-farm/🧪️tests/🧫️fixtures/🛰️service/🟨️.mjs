import { existsSync, writeFileSync } from "node:fs";

/** 🧪️ A deterministic stand-in for the project service of a farm worker: every job answers a pure function of its number, or misbehaves the way the job says. */
let where;
export async function open(session, place) { where = place; return session.identity; }
export async function run(job) {
  if (job.delay) await new Promise(done => setTimeout(done, job.delay));
  if (job.kind === "throw") throw Object.assign(Error("boom " + job.n), { code: "BOOM" });
  if (job.kind === "crash" && !existsSync(job.marker)) { writeFileSync(job.marker, "crashed"); process.exit(3); }
  if (job.kind === "crash-always") process.exit(3);
  if (job.kind === "hang") await new Promise(() => {});
  return { n: job.n, value: (job.n * job.n + 7) % 1013, slot: where.slot, generation: where.generation, pid: process.pid };
}
