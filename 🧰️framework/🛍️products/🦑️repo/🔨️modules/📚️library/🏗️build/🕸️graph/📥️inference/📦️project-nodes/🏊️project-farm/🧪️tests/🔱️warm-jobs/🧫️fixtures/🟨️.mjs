export async function open(session) { return session.identity; }
import { existsSync, writeFileSync } from "node:fs";
export async function run(job) {
  if (job.dieFile !== undefined && !existsSync(job.dieFile)) { writeFileSync(job.dieFile, "x"); process.exit(7); } return { names: [], n: job.n, pid: process.pid, ...(job.warm ? { weight: 0.01 } : {}) }; }
