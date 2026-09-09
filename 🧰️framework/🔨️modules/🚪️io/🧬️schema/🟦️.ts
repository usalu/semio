/** 🪪️ Artifact coordinates and references owned by the shared IO contract. */
export interface ArtifactDialect { artifactKind: string; standard: string; subset: string }
export interface ArtifactRef { artifactId: string; dialect: ArtifactDialect }

function record(value: unknown, keys: readonly string[]): Record<string, unknown> {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error("artifact address must be an object");
  const row = value as Record<string, unknown>;
  if (Object.keys(row).length !== keys.length || keys.some((key) => !Object.hasOwn(row, key))) throw new Error("artifact address fields do not match its contract");
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

/** 🧭️ Serializes the canonical dialect coordinate. */
export function dialectCoordinate(dialect: ArtifactDialect): string {
  return `${dialect.artifactKind}@${dialect.standard}/${dialect.subset}`;
}

/** 🧭️ Splits the first kind separator and final subset separator. */
export function parseDialectCoordinate(coordinate: string): ArtifactDialect {
  const at = coordinate.indexOf("@");
  if (at < 0) throw new Error("dialect coordinate is missing '@'");
  const slash = coordinate.lastIndexOf("/");
  if (slash <= at) throw new Error("dialect coordinate is missing '/'");
  const dialect = { artifactKind: coordinate.slice(0, at), standard: coordinate.slice(at + 1, slash), subset: coordinate.slice(slash + 1) };
  if (!dialect.artifactKind || !dialect.standard || !dialect.subset) throw new Error("dialect coordinate has an empty component");
  return dialect;
}

/** 🔗️ Serializes the canonical artifact URI. */
export function artifactRefUri(reference: ArtifactRef): string {
  return `${reference.artifactId}!${dialectCoordinate(reference.dialect)}`;
}

/** 🔗️ Decodes an artifact URI using the same dialect codec. */
export function parseArtifactRefUri(uri: string): ArtifactRef {
  const separator = uri.indexOf("!");
  if (separator <= 0) throw new Error("artifact URI requires an identifier followed by '!'");
  return { artifactId: uri.slice(0, separator), dialect: parseDialectCoordinate(uri.slice(separator + 1)) };
}
