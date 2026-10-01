import { mkdirSync } from "node:fs";
import { findRepoRoot } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🧭️routing/🟦️.ts";
import { repoTestArtifactEnvironment } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🧪️test-output/🟦️.ts";

/** 📁️ Places every service composition temporary input beneath its inherited test-output owner. */
export function serviceMcpTestArtifactRoot(): string {
  const root = repoTestArtifactEnvironment(findRepoRoot(import.meta.dir), "services-mcp").SEMIO_TEST_ARTIFACT_DIR!;
  mkdirSync(root, { recursive: true });
  return root;
}
