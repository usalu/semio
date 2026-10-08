#!/usr/bin/env bun
/** 🗂️ Runs the complete native ownership law through the General command owner. */
import { join, resolve } from "node:path";
import { executeCommandV1 } from "../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🎛️command/🟦️.ts";

if (process.argv.length !== 3 || process.argv[2] !== "test") throw Error("Expected native ownership test");
const cwd = import.meta.dir, artifacts = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR ?? join(cwd, "🤖️generated"));
let cancelled = false;
const stop = () => { cancelled = true; };
process.once("SIGINT", stop);
process.once("SIGTERM", stop);
try {
  const result = await executeCommandV1({ version: 1, cwd, command: process.execPath, args: ["test", join(cwd, "🟦️.ts")], manifests: [], preparation: "none" }, {
    version: 1, context: { version: 1, cwd, cacheRoot: join(artifacts, "cache"), leaseDirectory: join(artifacts, "leases") },
    budgetMs: 300000, maximumOutputBytes: 16777216, artifactDirectory: artifacts, retainArtifacts: true,
    cargoPolicies: [], vitestPolicy: null, cargoArtifactPolicy: null,
  }, { environment: process.env, cancelled: () => cancelled, onProgress: event => process.stderr.write(`[DEBUG] S native ownership ${event.phase} elapsedMs=${event.elapsedMs}\n`) });
  process.stdout.write(result.stdout);
  process.stderr.write(result.stderr);
  process.stderr.write(`[DEBUG] S native ownership receipt=${result.receiptPath} reason=${result.reason} status=${result.status}\n`);
  process.exitCode = result.reason === "exit" ? result.status ?? 1 : 1;
} finally {
  process.off("SIGINT", stop);
  process.off("SIGTERM", stop);
}
