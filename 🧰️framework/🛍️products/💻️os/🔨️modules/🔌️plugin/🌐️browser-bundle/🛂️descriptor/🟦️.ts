import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { decodePackValue, encodePackValue, packValueToExactJson, type PackValue } from "@semio-tech/framework-os";

const DESCRIPTOR_ROOT = dirname(fileURLToPath(import.meta.url));
export const ACTOR_COMPONENT_EXPORTS = JSON.parse(readFileSync(join(DESCRIPTOR_ROOT, "../../🧫️fixtures/🛂️actor-exports/🔣️.json"), "utf8")) as Record<string, string[]>;

/** 🛂️ Both package roles must expose the complete actor world before publication. */
export function assertActorComponentExports(component: Record<string, unknown>, required: Record<string, string[]>): void {
  for (const [name, methods] of Object.entries(required)) {
    const api = component[name] as Record<string, unknown> | undefined;
    for (const method of methods) {
      if (typeof api?.[method] !== "function") throw new Error(`Missing actor export ${name}.${method}`);
    }
  }
}

export const PLUGIN_DESCRIPTOR_PROBE_SOURCE = `
import { pathToFileURL } from "node:url";
${assertActorComponentExports.toString()}
const component = await import(pathToFileURL(process.argv[1]).href);
assertActorComponentExports(component, ${JSON.stringify(ACTOR_COMPONENT_EXPORTS)});
const bytes = await component.describe.describe();
if (!(bytes instanceof Uint8Array) || bytes.length === 0 || bytes.length > 8 * 1024 * 1024) throw new Error("Invalid descriptor byte extent");
process.stdout.write(Buffer.from(bytes).toString("base64"));
`;

/** 🔏️ Uses the same native pack self-hash convention for the genuine guest descriptor. */
export function finalizePluginDescriptor(bytes: Uint8Array, pluginId: string, wasmSha256: string, coreWasmSha256: string): { pack: Uint8Array; json: string } {
  const descriptor = decodePackValue(bytes) as unknown as { manifest?: { pluginId?: string; label?: string }; hashes?: Record<string, string> };
  if (descriptor?.manifest?.pluginId === "assembly-failed") throw new Error(`Plugin descriptor assembly failed: ${descriptor.manifest.label ?? "no fault message"}`);
  if (descriptor?.manifest?.pluginId !== pluginId || !descriptor.hashes) throw new Error("Plugin descriptor identity mismatch");
  if (![wasmSha256, coreWasmSha256].every((hash) => /^[a-f0-9]{64}$/.test(hash))) throw new Error("Invalid plugin artifact digest");
  descriptor.hashes.wasmSha256 = wasmSha256;
  descriptor.hashes.coreWasmSha256 = coreWasmSha256;
  descriptor.hashes.descriptorSha256 = "";
  descriptor.hashes.descriptorSha256 = createHash("sha256").update(encodePackValue(descriptor as PackValue)).digest("hex");
  return { pack: encodePackValue(descriptor as PackValue), json: JSON.stringify(packValueToExactJson(descriptor as PackValue), null, 2) + "\n" };
}
