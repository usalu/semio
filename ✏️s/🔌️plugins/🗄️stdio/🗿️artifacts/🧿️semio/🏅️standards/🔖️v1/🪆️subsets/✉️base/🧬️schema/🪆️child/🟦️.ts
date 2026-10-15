import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";
import { parseSchemaRecord } from "../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
import { requireArtifactDialect } from "../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🗿️artifact-reference/🟦️.ts";
export type { ArtifactChild } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";

/** 🪆️ Admits independent local and target identities with the exact Semio subset. */
export function parseSemioChild(value: unknown, subset: string, at = "$"): ArtifactChild {
  parseSchemaRecord(value, ["childId", "target"], at);
  const child = parseArtifactChild(value);
  requireArtifactDialect(child.target.dialect, { artifactKind: "s.stdio.semio", standard: "v1", subset }, at);
  return child;
}
