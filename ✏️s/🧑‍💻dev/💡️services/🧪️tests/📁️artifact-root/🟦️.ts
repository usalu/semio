import { mkdirSync } from "node:fs";
import { findWorkspaceRoot } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { repoTestArtifactEnvironment } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🧪️test-output/🟦️.ts";

/** 📁️ Places every service composition temporary input beneath its inherited test-output owner. */
export function serviceMcpTestArtifactRoot(): string {
  const root = repoTestArtifactEnvironment(findWorkspaceRoot(import.meta.dir), "services-mcp").SEMIO_TEST_ARTIFACT_DIR!;
  mkdirSync(root, { recursive: true });
  return root;
}
