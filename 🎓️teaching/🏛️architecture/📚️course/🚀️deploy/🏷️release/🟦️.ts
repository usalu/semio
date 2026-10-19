import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import { validateJsonSchemaSubset } from "../../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

/** 🏷️ Names a rollback image by its source revision and complete Docker image-config identity. */
export function courseRollbackTagV1(value: unknown): string {
  if (validateJsonSchemaSubset(schema, value).length) throw Error("Complete course revision and Docker SHA-256 image identity required");
  const request = value as { readonly revision: string; readonly imageId: string };
  return `sha-${request.revision.slice(0, 12)}-${request.imageId.slice(7)}`;
}
