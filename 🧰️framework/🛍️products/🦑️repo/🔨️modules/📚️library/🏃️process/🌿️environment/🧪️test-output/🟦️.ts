import { mkdirSync } from "node:fs";
import { resolve } from "node:path";
import { testSandboxEnvironmentV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";

/** 🧾️ Resolves caller-owned test output, or the artifact directory of the per-run test sandbox (private temp root, image root, port leases, reclaimed when its owner dies), without naming a ticket or a shared cache route. */
export function repoTestArtifactEnvironment(repoRoot: string, route: string, env: NodeJS.ProcessEnv = process.env): NodeJS.ProcessEnv {
  if (!/^[a-z0-9][a-z0-9-]{0,127}$/u.test(route)) throw new Error("Invalid test artifact route");
  const configured = env.SEMIO_TEST_ARTIFACT_DIR?.trim();
  if (!configured) return testSandboxEnvironmentV1(env, { route, checkout: repoRoot }).environment;
  const artifactRoot = resolve(repoRoot, configured);
  mkdirSync(artifactRoot, { recursive: true });
  return { ...env, SEMIO_TEST_ARTIFACT_DIR: artifactRoot };
}
