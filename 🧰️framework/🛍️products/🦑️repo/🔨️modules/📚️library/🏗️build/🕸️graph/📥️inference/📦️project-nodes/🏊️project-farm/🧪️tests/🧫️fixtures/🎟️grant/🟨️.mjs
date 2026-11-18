import { mkdirSync, writeFileSync } from "node:fs";
import { uptime } from "node:os";
import { join } from "node:path";

/** 🎟️ Writes the live capacity grant of one scripted process (one job, 2048 MB) under `root` and answers the environment that names its plan and ledger, so a farm under test owns the worker its grant pays for exactly as a graph child does. */
export function farmGrantEnvironmentV1(root, id, extra = {}) {
  const capacity = join(root, "capacity"), plan = JSON.stringify({ version: 1, id, jobs: 1, cpu: 1, memoryMegabytes: 2048, fixedMegabytes: 1536, floorMegabytes: 1981 });
  mkdirSync(join(capacity, "grants"), { recursive: true });
  writeFileSync(join(capacity, "grants", id + ".json"), JSON.stringify({ id, operation: "run", jobs: 1, cpu: 1, reservedMegabytes: 7168, admittedAtMs: Date.now(), observedMegabytes: null, pid: process.pid, bootAtMs: Math.round((Date.now() - uptime() * 1000) / 1000) * 1000 }));
  return { LANG: "en_US.UTF-8", SEMIO_SLICE_LEAN: "off", SEMIO_HOST_CAPACITY_DIRECTORY: capacity, SEMIO_HOST_CAPACITY_PLAN: plan, ...extra };
}
