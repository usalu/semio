/** 🏠️ A child envelope's exact ownership stamp, shared by every composition consumer. */
import { parseArtifactRef, type ArtifactRef } from "../../../../../../../🔨️modules/🚪️io/🧬️schema/🟦️.ts";
import { parseSchemaRecord } from "../../../../../../../🔨️modules/🧬️schema/🧾️record/🟦️.ts";
export type { ArtifactRef } from "../../../../../../../🔨️modules/🚪️io/🧬️schema/🟦️.ts";
export interface OwnerRef { parent: ArtifactRef; slot: string; childId: string; }

/** 🪪️ Decodes declared ownership fields without evaluating foreign accessors. */
export function parseOwnerRef(value: unknown): OwnerRef {
  const row = parseSchemaRecord(value, ["parent", "slot", "childId"]);
  if (typeof row.slot !== "string" || typeof row.childId !== "string") throw new Error("owner slot and childId must be strings");
  return { parent: parseArtifactRef(row.parent), slot: row.slot, childId: row.childId };
}
