export async function open(session) { return session.identity; }
export async function run(job) { const start = Date.now(); if (job.delay) await new Promise(done => setTimeout(done, job.delay)); return { n: job.n, pid: process.pid, start, end: Date.now() }; }
