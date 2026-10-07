/** 🪪️ Owned semantic artifact identity and dialect values. */
import contract from "./🔣️.json";

const artifactKindPattern = new RegExp(contract.$defs.ArtifactKindId.pattern, "u");

/** 🪪️ Admits exactly one canonical domain, plugin and artifact coordinate. */
export function isCanonicalArtifactKind(value: unknown): value is string {
  return typeof value === "string" && artifactKindPattern.test(value);
}

export interface ArtifactDialect { artifactKind: string; standard: string; subset: string }
export interface ArtifactRef { artifactId: string; dialect: ArtifactDialect }

function record(value: unknown, keys: readonly string[]): Record<string, unknown> {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error("Artifact reference value must be an object");
  const row = value as Record<string, unknown>;
  if (Object.keys(row).length !== keys.length || keys.some((key) => !Object.hasOwn(row, key))) throw new Error("Artifact reference fields do not match");
  return row;
}

/** 🪪️ Decodes the three canonical dialect components. */
export function parseArtifactDialect(value: unknown): ArtifactDialect {
  const row = record(value, ["artifactKind", "standard", "subset"]);
  if (typeof row.artifactKind !== "string" || typeof row.standard !== "string" || typeof row.subset !== "string") throw new Error("artifact dialect components must be strings");
  return { artifactKind: row.artifactKind, standard: row.standard, subset: row.subset };
}

/** 🔗️ Decodes an exact artifact identifier and its dialect. */
export function parseArtifactRef(value: unknown): ArtifactRef {
  const row = record(value, ["artifactId", "dialect"]);
  if (typeof row.artifactId !== "string") throw new Error("artifact identifier must be a string");
  return { artifactId: row.artifactId, dialect: parseArtifactDialect(row.dialect) };
}

