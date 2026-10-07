/** 🚪️ Physical artifact routes and coordinate codecs. */
import { parseArtifactDialect, type ArtifactDialect, type ArtifactRef } from "../../🧬️schema/🗿️artifact-reference/🟦️.ts";

/** 🪶️ Standalone SQLite carrier for one exact native artifact snapshot. */
export const SQLITE_SNAPSHOT: Readonly<ArtifactDialect> = Object.freeze({ artifactKind: "s.framework.sqlite-snapshot", standard: "1", subset: "*" });

function record(value: unknown, keys: readonly string[]): Record<string, unknown> {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error("IO contract value must be an object");
  const row = value as Record<string, unknown>;
  if (Object.keys(row).length !== keys.length || keys.some((key) => !Object.hasOwn(row, key))) throw new Error("IO contract fields do not match");
  return row;
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

/** ⚖️ Strongest fidelity declared by one IO hop. */
export type IoFidelity = "Exact" | "Canonical" | "Semantic" | "Lossy";
/** 📇️ Owned metadata for a registered directed IO hop. */
export interface IoEntryDescriptor { readonly from: ArtifactDialect; readonly into: ArtifactDialect; readonly fidelity: IoFidelity; readonly sniffs: boolean }
/** 🗺️ Owned route metadata consumed by the renderer-independent IO mechanism. */
export interface IoRoute { readonly hops: readonly IoEntryDescriptor[]; readonly fidelity: IoFidelity }

/** 🎚️ Validates the canonical fidelity spelling. */
export function parseIoFidelity(value: unknown): IoFidelity {
  if (value !== "Exact" && value !== "Canonical" && value !== "Semantic" && value !== "Lossy") throw new Error("IO fidelity must be Exact, Canonical, Semantic, or Lossy");
  return value;
}

/** 🚪️ Decodes a registered IO entry's strict wire contract. */
export function parseIoEntryDescriptor(value: unknown): IoEntryDescriptor {
  const row = record(value, ["from", "into", "fidelity", "sniffs"]);
  if (typeof row.sniffs !== "boolean") throw new Error("IO entry sniffs must be a boolean");
  return { from: parseArtifactDialect(row.from), into: parseArtifactDialect(row.into), fidelity: parseIoFidelity(row.fidelity), sniffs: row.sniffs };
}

/** 🌉️ Decodes route metadata through the same descriptor owner. */
export function parseIoRoute(value: unknown): IoRoute {
  const row = record(value, ["hops", "fidelity"]);
  if (!Array.isArray(row.hops)) throw new Error("IO route hops must be an array");
  return { hops: row.hops.map(parseIoEntryDescriptor), fidelity: parseIoFidelity(row.fidelity) };
}

/** ⚠️ Exposes the single controlled IO cause owner through the neutral vocabulary. */
export {IoError,encodeIoErrorControlled,decodeIoErrorControlled} from "./⚠️refusal/🟦️.ts";
