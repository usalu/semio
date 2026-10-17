import { parseArtifactChild, type ArtifactChild } from "@semio-tech/framework-os/modules/store/child/schema";
import { parseSchemaRecord } from "@semio-tech/framework/schema/record";
import { requireArtifactDialect } from "@semio-tech/framework/schema/artifact-reference";
export type { ArtifactChild } from "@semio-tech/framework-os/modules/store/child/schema";

/** 🪆️ Admits independent local and target identities with the exact Semio subset. */
export function parseSemioChild(value: unknown, subset: string, at = "$"): ArtifactChild {
  parseSchemaRecord(value, ["childId", "target"], at);
  const child = parseArtifactChild(value);
  requireArtifactDialect(child.target.dialect, { artifactKind: "s.stdio.semio", standard: "v1", subset }, at);
  return child;
}
