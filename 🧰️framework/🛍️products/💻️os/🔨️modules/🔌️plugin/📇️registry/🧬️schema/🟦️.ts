import contract from "./🔣️.json";
import { validateJsonSchemaSubset } from "../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import { jsonDocumentDuplicateKeys } from "../../../../../🦑️repo/🔨️modules/📚️library/🧬️schema/🔣️json-document/🟦️.ts";

export const REGISTRY_DESCRIPTOR_MAX_BYTES = 4 * 1024 * 1024;

/** 🧾️ Decodes the current catalog descriptor protocol without losing byte or member identity. */
export function decodeRegistryDescriptorV1(bytes: Uint8Array): Record<string, unknown> {
  if (bytes.byteLength === 0 || bytes.byteLength > REGISTRY_DESCRIPTOR_MAX_BYTES) throw new Error("Registry descriptor exceeds its byte boundary");
  const source = new TextDecoder("utf-8", { fatal: true }).decode(bytes), value: unknown = JSON.parse(source);
  if (jsonDocumentDuplicateKeys(source).length) throw new Error("Registry descriptor repeats an object member");
  const errors = validateJsonSchemaSubset(contract.$defs.CatalogDescriptorV1, value, contract);
  if (errors.length) throw new Error(`Invalid registry descriptor: ${errors.slice(0, 8).join("; ")}`);
  return value as Record<string, unknown>;
}
