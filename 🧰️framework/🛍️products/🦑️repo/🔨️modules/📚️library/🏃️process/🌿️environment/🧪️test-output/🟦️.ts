import { mkdirSync } from "node:fs";
import { resolve } from "node:path";

/** 🧾️ Gives filesystem-mutating tests one current-ticket output owner while preserving an explicit caller override. */
export function repoTestArtifactEnvironment(repoRoot: string, route: string, env: NodeJS.ProcessEnv = process.env): NodeJS.ProcessEnv {
  const configured = env.SEMIO_TEST_ARTIFACT_DIR?.trim();
  const artifactRoot = resolve(repoRoot, configured || `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/KIND-ONLY-BASENAMES-ACROSS-THE-TAXONOMY-TREE/🗑️generated/sol-root-clean-scaffold-extraction/repo-lib-test-artifacts/${route}`);
  mkdirSync(artifactRoot, { recursive: true });
  return { ...env, SEMIO_TEST_ARTIFACT_DIR: artifactRoot };
}
