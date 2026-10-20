import { readdirSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { decodePackValue, encodePackValue, packValueToExactJson, type PackValue } from "../../../../📦️packages/🟦️typescript/🟦️.ts";

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

/** 📜️ Reads the actor package's authored WIT files, in name order, as the one text the package declares. */
export function readActorComponentWit(directory: string): string {
  return readdirSync(directory).filter((name) => name.endsWith(".wit")).sort().map((name) => readFileSync(join(directory, name), "utf8")).join("\n");
}

export const ACTOR_COMPONENT_EXPORTS = parseActorComponentExports(readActorComponentWit(join(DESCRIPTOR_ROOT, "../../🧬️schema")));

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

/** 🛂️ Admits the genuine semantic guest descriptor of one materialized module: an assembly-failure stub, a foreign plugin and a
 * pack carrying any byte binding of its own are refused, since the module's `🧾️receipt.json` alone names its bytes. Answers the
 * canonical pack and its exact JSON form. */
export function finalizePluginDescriptor(bytes: Uint8Array, pluginId: string): { pack: Uint8Array; json: string } {
  const descriptor = decodePackValue(bytes) as unknown as Record<string, unknown> & { manifest?: { pluginId?: string; label?: string } };
  if (descriptor === null || typeof descriptor !== "object" || Array.isArray(descriptor)) throw new Error("Plugin descriptor is not an object");
  if (descriptor.manifest?.pluginId === "assembly-failed") throw new Error(`Plugin descriptor assembly failed: ${descriptor.manifest.label ?? "no fault message"}`);
  if (descriptor.manifest?.pluginId !== pluginId) throw new Error("Plugin descriptor identity mismatch");
  if ("hashes" in descriptor) throw new Error("Plugin descriptor carries a byte binding; a semantic descriptor names no bytes");
  const pack = encodePackValue(descriptor as unknown as PackValue);
  if (!Buffer.from(pack).equals(Buffer.from(bytes))) throw new Error("Plugin descriptor is not its canonical pack");
  return { pack, json: JSON.stringify(packValueToExactJson(descriptor as unknown as PackValue), null, 2) + "\n" };
}
