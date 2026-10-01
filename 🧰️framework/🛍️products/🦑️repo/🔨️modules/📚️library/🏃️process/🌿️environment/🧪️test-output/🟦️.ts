import { mkdirSync } from "node:fs";
import { resolve } from "node:path";

/** 🧾️ Resolves caller-owned test output or a neutral cache route without naming a ticket. */
export function repoTestArtifactEnvironment(repoRoot: string, route: string, env: NodeJS.ProcessEnv = process.env): NodeJS.ProcessEnv {
  if (!/^[a-z0-9][a-z0-9-]{0,127}$/u.test(route)) throw new Error("Invalid test artifact route");
  const configured = env.SEMIO_TEST_ARTIFACT_DIR?.trim();
  const artifactRoot = resolve(repoRoot, configured || `.🧬semio/🦑️repo/⚡️cache/🧪️tests/${route}`);
  mkdirSync(artifactRoot, { recursive: true });
  return { ...env, SEMIO_TEST_ARTIFACT_DIR: artifactRoot };
}
