import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";
import { parseSchemaRecord } from "../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
export type { ArtifactChild } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";

/** 🪆️ Admits independent local and target identities with the exact Semio subset. */
export function parseSemioChild(value: unknown, subset: string, at = "$"): ArtifactChild {
  parseSchemaRecord(value, ["childId", "target"], at);
  const child = parseArtifactChild(value);
  const dialect = child.target.dialect;
  if (dialect.artifactKind !== "s.stdio.semio" || dialect.standard !== "v1" || dialect.subset !== subset) throw new Error(at + ": Semio child dialect mismatch");
  return child;
}
