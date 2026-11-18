/** 🧪️ A stand-in for the project service of a farm worker whose jobs either spend processor time for a while or wait without spending any. */
export async function open(session) { return session.identity; }
export async function run(job) {
  if (job.kind === "spin") { const until = Date.now() + job.milliseconds; let sink = 0; while (Date.now() < until) sink += Math.sqrt(sink + 1); return { spun: job.milliseconds, sink: sink > 0 }; }
  if (job.kind === "idle") { await new Promise(done => setTimeout(done, job.milliseconds)); return { idled: job.milliseconds }; }
  return { n: job.n };
}
