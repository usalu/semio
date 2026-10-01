import schema from "../../../../🧬️schema/🧩️plugin-modules/🔣️.json";

export type WgpuPluginRegistrySelection = "variant" | "all";
export type WgpuPluginModule = { readonly pluginId: string; readonly moduleUrl: string };
const policy = schema["x-semio-admission"];
const idPattern = new RegExp(schema.items.properties.pluginId.pattern, "u");
const urlPattern = new RegExp(schema.items.properties.moduleUrl.pattern, "u");
const scalarLength = (value: string) => [...value].length;
const refused = (detail: string): never => { throw new Error(`plugin-registry-invalid: ${detail}`); };

/** 🧩️ Admits a bounded explicit registry and resolves each module against the owning page. */
export function admitWgpuPluginModules(input: unknown, baseUrl: string): readonly WgpuPluginModule[] {
  if (!Array.isArray(input) || input.length > schema.maxItems) return refused("module count");
  const ids = new Set<string>();
  const urls = new Set<string>();
  const modules = input.map(row => {
    if (!row || typeof row !== "object" || Object.keys(row).length !== 2 || typeof row.pluginId !== "string" || typeof row.moduleUrl !== "string") return refused("module shape");
    if (row.pluginId.length > schema.items.properties.pluginId.maxLength * 2 || row.moduleUrl.length > schema.items.properties.moduleUrl.maxLength * 2) return refused("module field length");
    if (!idPattern.test(row.pluginId) || scalarLength(row.pluginId) > schema.items.properties.pluginId.maxLength || !urlPattern.test(row.moduleUrl) || scalarLength(row.moduleUrl) > schema.items.properties.moduleUrl.maxLength) return refused("module fields");
    let url: URL;
    try { url = new URL(row.moduleUrl, baseUrl); } catch { return refused("module URL"); }
    if (scalarLength(url.href) > schema.items.properties.moduleUrl.maxLength) return refused("resolved module URL length");
    if (!policy.schemes.includes(url.protocol) || url.username || url.password || !url.pathname || url.pathname.endsWith("/")) return refused("module URL authority");
    if (ids.has(row.pluginId) || urls.has(url.href)) return refused("duplicate module owner or URL");
    ids.add(row.pluginId);
    urls.add(url.href);
    return { pluginId: row.pluginId, moduleUrl: url.href };
  });
  if (new TextEncoder().encode(JSON.stringify(modules)).byteLength > policy.maxJsonBytes) return refused("resolved module bytes");
  return modules;
}


/** 🧭️ Admits the declared supplied-registry selection policy. */
export function admitWgpuPluginRegistrySelection(value: unknown = "variant"): WgpuPluginRegistrySelection {
  if (typeof value !== "string" || !policy.selectionModes.includes(value)) return refused("registry selection");
  return value as WgpuPluginRegistrySelection;
}
