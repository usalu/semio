import contract from "./🔣️.json";
import { validateJsonSchemaSubset } from "../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import { jsonDocumentDuplicateKeys } from "../../../../../🦑️repo/🔨️modules/📚️library/🧬️schema/🔣️json-document/🟦️.ts";

export const REGISTRY_DESCRIPTOR_MAX_BYTES = 4 * 1024 * 1024;

/** 📡️ The guest↔host app channel the host speaks, owned by the descriptor contract's `executionProtocol` const. */
export const REGISTRY_HOST_APP_CHANNEL_VERSION: number = contract.$defs.CatalogDescriptorV1.properties.executionProtocol.properties.appChannelVersion.const;

/** 🕰️ A descriptor compiled against another app channel: release gates refuse it, dev generation withholds its plugin. */
export class StaleChannelDescriptorError extends Error {
  constructor(readonly descriptorChannel: unknown, readonly hostChannel: number = REGISTRY_HOST_APP_CHANNEL_VERSION) {
    super(`Stale registry descriptor: app channel ${JSON.stringify(descriptorChannel)} differs from the host app channel ${hostChannel}`);
    this.name = "StaleChannelDescriptorError";
  }
}

/** 🧾️ Decodes the current catalog descriptor protocol without losing byte or member identity. */
export function decodeRegistryDescriptorV1(bytes: Uint8Array): Record<string, unknown> {
  if (bytes.byteLength === 0 || bytes.byteLength > REGISTRY_DESCRIPTOR_MAX_BYTES) throw new Error("Registry descriptor exceeds its byte boundary");
  const source = new TextDecoder("utf-8", { fatal: true }).decode(bytes), value: unknown = JSON.parse(source);
  if (jsonDocumentDuplicateKeys(source).length) throw new Error("Registry descriptor repeats an object member");
  const protocol = value !== null && typeof value === "object" && !Array.isArray(value) ? (value as { executionProtocol?: unknown }).executionProtocol : undefined;
  const channel = protocol !== null && typeof protocol === "object" && !Array.isArray(protocol) ? (protocol as { appChannelVersion?: unknown }).appChannelVersion : undefined;
  if (channel !== undefined && channel !== REGISTRY_HOST_APP_CHANNEL_VERSION) throw new StaleChannelDescriptorError(channel);
  const errors = validateJsonSchemaSubset(contract.$defs.CatalogDescriptorV1, value, contract);
  if (errors.length) throw new Error(`Invalid registry descriptor: ${errors.slice(0, 8).join("; ")}`);
  return value as Record<string, unknown>;
}
