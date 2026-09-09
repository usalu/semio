/** 🔗️ Shared references preserve independent artifact lifetimes and explicit history pins. */
import { parseArtifactRef, type ArtifactRef } from "../../../../../../🔨️modules/🚪️io/🧬️schema/🟦️.ts";
import { parseBlobRef, type BlobRef } from "../../📦️blob/🧬️schema/🟦️.ts";
export type { ArtifactRef } from "../../../../../../🔨️modules/🚪️io/🧬️schema/🟦️.ts";
export type { BlobRef } from "../../📦️blob/🧬️schema/🟦️.ts";
export type LinkPin = { kind: "head" } | { kind: "checkpoint"; id: string } | { kind: "snapshot"; blob: BlobRef };
export interface ArtifactLink { target: ArtifactRef; pin: LinkPin; role: string }

function record(value: unknown): Record<string, unknown> {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error("artifact link must be an object");
  return value as Record<string, unknown>;
}

/** 📌️ Parses the exact payload of one history pin variant. */
export function parseLinkPin(value: unknown): LinkPin {
  const row = record(value), count = Object.keys(row).length;
  if (row.kind === "head" && count === 1) return { kind: "head" };
  if (row.kind === "checkpoint" && count === 2 && typeof row.id === "string") return { kind: "checkpoint", id: row.id };
  if (row.kind === "snapshot" && count === 2 && Object.hasOwn(row, "blob")) return { kind: "snapshot", blob: parseBlobRef(row.blob) };
  throw new Error("artifact link pin has invalid fields or kind");
}

/** 🔗️ Parses a target, history pin and role through their shared owners. */
export function parseArtifactLink(value: unknown): ArtifactLink {
  const row = record(value);
  if (Object.keys(row).length !== 3 || !Object.hasOwn(row, "target") || !Object.hasOwn(row, "pin") || typeof row.role !== "string") throw new Error("artifact link requires target, pin and role");
  return { target: parseArtifactRef(row.target), pin: parseLinkPin(row.pin), role: row.role };
}
