import {existsSync,writeFileSync} from "node:fs";
import {setTimeout as delay} from "node:timers/promises";

export async function open(session) { return session.identity; }
export async function run(job) {
  const start=Date.now();
  if (job.started) writeFileSync(job.started,"started");
  if (job.release) while(!existsSync(job.release)) await delay(10);
  if (job.delay) await delay(job.delay);
  return {n:job.n,pid:process.pid,start,end:Date.now()};
}
