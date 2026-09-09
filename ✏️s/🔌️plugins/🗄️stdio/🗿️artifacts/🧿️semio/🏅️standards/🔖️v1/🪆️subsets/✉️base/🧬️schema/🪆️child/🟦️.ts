import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";
import { parseSchemaRecord } from "../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
export type { ArtifactChild } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";

/** 🪆️ Admits the exact persisted child identity for one Semio subset. */
export function parseSemioChild(value: unknown, subset: string, at = "$"): ArtifactChild {
  parseSchemaRecord(value, ["childId", "target"], at);
  const child = parseArtifactChild(value);
  const dialect = child.target.dialect;
  if (child.childId !== child.target.artifactId || dialect.artifactKind !== "s.stdio.semio" || dialect.standard !== "v1" || dialect.subset !== subset) throw new Error(at + ": Semio child identity or dialect mismatch");
  return child;
}
