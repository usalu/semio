/** @emoji 📜 `@semio-tech/graph-manifest` — compile-time graph manifest types and catalog projection. */
export * from "../generated/📦.ts";
//#region 🔖validate
/** @emoji 🛡️ Runtime validation helpers for graph manifest documents. */
import type { GraphManifestDocument } from "../generated/types.ts";
import manifestSchema from "../generated/manifest.schema.json";

function isRecord(value: unknown): value is Record<string, unknown> {
  return Boolean(value) && typeof value === "object" && !Array.isArray(value);
}

/** @emoji 🛡️ Validates unknown JSON against generated `manifest/v1` schema rules. */
export function validateGraphManifestDocument(raw: unknown): GraphManifestDocument {
  if (!isRecord(raw)) {
    throw new Error("graph manifest must be an object");
  }
  const requiredSchema = (manifestSchema as { properties?: { schema?: { const?: string } } }).properties?.schema?.const;
  if (raw.schema !== requiredSchema) {
    throw new Error(`graph manifest schema must be ${String(requiredSchema)}`);
  }
  if (typeof raw.id !== "string" || !raw.id.trim()) {
    throw new Error("graph manifest requires id");
  }
  return raw as GraphManifestDocument;
}
//#endregion 🔖validate
