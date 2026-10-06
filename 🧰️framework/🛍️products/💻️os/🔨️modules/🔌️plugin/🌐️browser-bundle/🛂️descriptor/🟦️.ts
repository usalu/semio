import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { decodePackValue, encodePackValue, packValueToExactJson, type PackValue } from "@semio-tech/framework-os";

const DESCRIPTOR_ROOT = dirname(fileURLToPath(import.meta.url));

/** 📜️ Projects the actor world's exported functions from its authored WIT contract. */
export function parseActorComponentExports(wit: string): Record<string, string[]> {
  const source = wit.replace(/\/\*[\s\S]*?\*\//g, "").replace(/\/\/[^\n]*/g, "");
  const body = (kind: string, name: string) => {
    const matches = [...source.matchAll(new RegExp(`\\b${kind}\\s+${name}\\s*\\{`, "g"))];
    if (matches.length !== 1) throw new Error(`Expected one WIT ${kind} ${name}`);
    let depth = 1, text = "";
    for (let index = matches[0]!.index! + matches[0]![0].length; index < source.length; index++) {
      const char = source[index]!;
      if (char === "{") depth++;
      else if (char === "}" && --depth === 0) return text;
      else if (depth === 1) text += char;
    }
    throw new Error(`Unclosed WIT ${kind} ${name}`);
  };
  const result: Record<string, string[]> = {};
  for (const match of body("world", "actor").matchAll(/\bexport\s+([a-z][a-z0-9-]*)\s*;/g)) {
    const name = match[1]!;
    if (Object.hasOwn(result, name)) throw new Error(`Duplicate actor export ${name}`);
    result[name] = [...body("interface", name).matchAll(/\b([a-z][a-z0-9-]*)\s*:\s*(?:async\s+)?func\s*\(/g)].map((method) => method[1]!.replace(/-([a-z])/g, (_match, char: string) => char.toUpperCase()));
    if (!result[name]!.length) throw new Error(`Actor export ${name} has no functions`);
  }
  if (!Object.keys(result).length) throw new Error("Actor world has no exports");
  return result;
}

export const ACTOR_COMPONENT_EXPORTS = parseActorComponentExports(readFileSync(join(DESCRIPTOR_ROOT, "../../🧬️schema/📜️.wit"), "utf8"));

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
