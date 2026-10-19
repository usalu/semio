import { mkdirSync } from "node:fs";
import { findWorkspaceRoot } from "@semio-tech/process/routing";
import { repoTestArtifactEnvironment } from "@semio-tech/process/execution";

/** 📁️ Places every service composition temporary input beneath its inherited test-output owner. */
export function serviceMcpTestArtifactRoot(): string {
  const root = repoTestArtifactEnvironment(findWorkspaceRoot(import.meta.dirname), "services-mcp").SEMIO_TEST_ARTIFACT_DIR!;
  mkdirSync(root, { recursive: true });
  return root;
}
