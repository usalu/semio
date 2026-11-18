import { writeFileSync } from "node:fs";
import { workerData } from "node:worker_threads";
import { OWNER_POOL_POLICY_V1 as P } from "../🟦️.ts";
import { ownerPoolAliveV1, ownerPoolChildFateV1, ownerPoolChildVerdictV1, readOwnerPoolLeaseV1 } from "../🪦️reaper/🟦️.ts";

/** 🐕️ The watchdog thread of a child of the pool: an owner is CPU bound JavaScript that can hold the event loop of the child for minutes, so a channel that closes or a timer on that loop never reaches it; this thread has a loop of its own and looks every `watchMilliseconds` at the parent and the lease (see {@link ownerPoolChildVerdictV1}). A dead or replaced parent ends the whole process after `graceMilliseconds`; a lease that is stale, names another pool or lacks the wave while the parent lives ends it only after `lostGraceMilliseconds` (the pool may be starved on a loaded machine: see {@link ownerPoolChildFateV1}). It writes its verdict to `exitFile` before it ends the process, so the pool can record why the child ended.
 * @see ../🧪️tests/🪦️reaper/🟦️.ts */
const { parentPid, wave, lease, exitFile, staleMilliseconds = P.leaseStaleMilliseconds, watchMilliseconds = P.childWatchMilliseconds, graceMilliseconds = P.orphanGraceMilliseconds, lostGraceMilliseconds = P.leaseLostGraceMilliseconds } = workerData;
let since;
setInterval(() => {
  const verdict = ownerPoolChildVerdictV1({ parentPid, currentParent: process.ppid, alive: ownerPoolAliveV1, lease: readOwnerPoolLeaseV1(lease), wave, now: Date.now(), staleMilliseconds });
  const fate = ownerPoolChildFateV1({ verdict, since, now: Date.now(), graceMilliseconds, lostGraceMilliseconds });
  since = fate.since;
  if (!fate.end) return;
  if (exitFile !== undefined) { try { writeFileSync(exitFile, JSON.stringify({ verdict, atMs: Date.now() })); } catch {} }
  try { process.kill(process.pid, "SIGKILL"); } catch {}
  process.exit(78);
}, watchMilliseconds);
