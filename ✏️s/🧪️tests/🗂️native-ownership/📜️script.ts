#!/usr/bin/env repo
/** 🗂️ Runs the complete native ownership law through the General command owner. */
import { join, resolve } from "node:path";
import { executeCommandV1 } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️command/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

if (process.argv.length !== 3 || process.argv[2] !== "test") throw new Error("Expected native ownership test");
const cwd = import.meta.dir, artifacts = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR ?? join(cwd, "🤖️generated"));
await receiveScriptProcessInvocation(process.env, async (invocation) => {
  const result = await executeCommandV1({ version: 1, cwd, command: process.execPath, args: ["test", join(cwd, "🟦️.ts")], manifests: [], preparation: "none" }, {
    version: 1, context: { version: 1, cwd, cacheRoot: join(artifacts, "cache"), leaseDirectory: join(artifacts, "leases") },
    budgetMs: 300000, maximumOutputBytes: 16777216, artifactDirectory: artifacts, retainArtifacts: true,
    cargoPolicies: [], vitestPolicy: null, cargoArtifactPolicy: null,
  }, { invocation, environment: process.env, onProgress: () => {} });
  process.stdout.write(result.stdout);
  process.stderr.write(result.stderr);
  process.exitCode = result.reason === "exit" ? result.status ?? 1 : 1;
});
