import { graphFarmIdentityV1 } from "../../../🪆️captured/🟨️.mjs";

/** 🔌️ Opens a stand-in session: announces the farm identity of this process for the record identity the session names. */
export async function open(session) { return graphFarmIdentityV1(session.records); }

/** ▶️ Answers a job with its number, the process that ran it and the implementation that process runs from. */
export async function run(job) { return { n: job.n, pid: process.pid, implementation: import.meta.semioCapturedImplementationIdentity ?? null }; }
