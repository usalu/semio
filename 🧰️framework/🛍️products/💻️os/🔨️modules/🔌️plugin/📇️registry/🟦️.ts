import type { PluginCatalog, PluginCatalogTarget, PluginHostConfig, PlaygroundCatalogTarget } from "@semio-tech/framework";
import schema from "./🧬️schema/🔣️.json";

export type PluginCatalogRowV1 = PluginCatalogTarget & { readonly moduleUrl: string; readonly dependsOn: readonly string[]; readonly activationEvents: readonly string[]; readonly capabilities: readonly string[] };
export type PluginCatalogRowsV1 = { readonly version: 1; readonly targets: readonly PluginCatalogRowV1[]; readonly hosts: readonly PluginHostConfig[]; readonly playgrounds: readonly PlaygroundCatalogTarget[] };
export type PluginCatalogAdmissionV1 = { readonly maxBytes: number; readonly maxRows: number; readonly maxEdges: number; readonly maxWork: number; readonly deadlineMs: number; readonly now: () => number; readonly cancelled: () => boolean; readonly progress: (event: { readonly completed: number; readonly total: number; readonly work: number }) => void };
export const PLUGIN_CATALOG_POLICY_V1 = Object.freeze(schema["x-semio-admission"]);
const idPattern = new RegExp(schema.$defs.PluginCatalogIdV1.pattern, "u");
const refused = (detail: string): never => { throw new Error(`plugin-catalog-invalid: ${detail}`); };

/** 🛂️ Admits finite serializable first-party inventory under the caller's explicit execution authority. */
export function admitPluginCatalogV1(input: unknown, control: PluginCatalogAdmissionV1): PluginCatalogRowsV1 {
  const policy = PLUGIN_CATALOG_POLICY_V1;
  for (const [value, maximum] of [[control.maxBytes, policy.maxJsonBytes], [control.maxRows, policy.maxRows], [control.maxEdges, policy.maxEdges], [control.maxWork, policy.maxWork]]) if (!Number.isSafeInteger(value) || value! <= 0 || value! > maximum!) return refused("finite authority");
  if (typeof control.now !== "function" || typeof control.cancelled !== "function" || typeof control.progress !== "function" || !Number.isFinite(control.deadlineMs)) return refused("execution authority");
  const start = control.now();
  if (!Number.isFinite(start) || control.deadlineMs - start > policy.maxDurationMs) return refused("deadline authority");
  let work = 0, edges = 0, completed = 0;
  const check = (units = 1) => {
    if (control.cancelled()) return refused("cancelled");
    const now = control.now();
    if (!Number.isFinite(now) || now >= control.deadlineMs) return refused("deadline");
    work += units;
    if (work > control.maxWork) return refused("work credits");
  };
  const object = (value: unknown, keys: readonly string[], required = keys): Record<string, unknown> => {
    check();
    if (!value || typeof value !== "object" || Array.isArray(value)) return refused("row shape");
    const row = value as Record<string, unknown>;
    const names = Object.keys(row);
    if (names.length > keys.length || names.some(key => !keys.includes(key)) || required.some(key => !Object.hasOwn(row, key))) return refused("row fields");
    return row;
  };
  const string = (value: unknown, identity = false, maximum = identity ? 256 : 2048): string => {
    check();
    if (typeof value !== "string" || value.length === 0 || value.length > maximum * 2) return refused("string length");
    check(value.length);
    if ([...value].length > maximum || (identity && !idPattern.test(value))) return refused("string fields");
    return value;
  };
  const list = (value: unknown, identity = false): readonly string[] => {
    check();
    if (!Array.isArray(value) || value.length > schema.$defs.PluginCatalogTextsV1.maxItems) return refused("edge count");
    edges += value.length;
    if (edges > control.maxEdges) return refused("edge credits");
    const result = value.map(item => string(item, identity));
    if (new Set(result).size !== result.length) return refused("duplicate edge");
    return Object.freeze(result);
  };
  check();
  const root = object(input, ["version", "targets", "hosts", "playgrounds"]);
  if (root.version !== 1) return refused("version");
  const groups = [root.targets, root.hosts, root.playgrounds];
  if (groups.some(group => !Array.isArray(group) || group.length > control.maxRows)) return refused("row count");
  const total = groups.reduce((count, group) => count + (group as unknown[]).length, 0);
  if (total > control.maxRows) return refused("row credits");
  const next = () => { check(); control.progress({ completed: ++completed, total, work }); check(); };
  const identities = new Set<string>(), urls = new Set<string>();
  const targets = (root.targets as unknown[]).map(value => {
    const row = object(value, ["pluginId", "wasmOut", "moduleUrl", "role", "dependsOn", "contributes", "consumes", "activationEvents", "capabilities", "extends"], ["pluginId", "wasmOut", "moduleUrl", "role", "dependsOn", "contributes", "consumes", "activationEvents", "capabilities"]);
    const pluginId = string(row.pluginId, true), moduleUrl = string(row.moduleUrl, false, 4096);
    if (identities.has(pluginId)) return refused("duplicate plugin identity");
    let url: URL;
    try { url = new URL(moduleUrl); } catch { return refused("module URL"); }
    if (!policy.schemes.includes(url.protocol) || url.username || url.password || !url.hostname || url.pathname.endsWith("/") || url.href !== moduleUrl || urls.has(moduleUrl)) return refused("module URL authority");
    if (row.role !== "plugin" && row.role !== "extension") return refused("role");
    identities.add(pluginId); urls.add(moduleUrl);
    const dependsOn = list(row.dependsOn, true), parent = row.role === "extension" ? string(row.extends, true) : undefined;
    if ((row.role === "plugin" && row.extends !== undefined) || (parent !== undefined && dependsOn[0] !== parent)) return refused("extension parent identity");
    const target: PluginCatalogRowV1 = Object.freeze({ pluginId, moduleUrl, wasmOut: string(row.wasmOut), role: row.role, dependsOn, capabilities: list(row.capabilities), ...(parent === undefined ? {} : { extends: parent }), contributes: list(row.contributes), consumes: list(row.consumes), activationEvents: list(row.activationEvents) });
    next(); return target;
  });
  for (const row of targets) for (const dependency of row.dependsOn) { check(); if (!identities.has(dependency)) return refused("missing dependency identity"); }
  const hostIds = new Set<string>();
  const hosts = (root.hosts as unknown[]).map(value => {
    const row = object(value, ["pluginId", "landingAppId", "hostAppId"]), pluginId = string(row.pluginId, true);
    if (!identities.has(pluginId) || hostIds.has(pluginId)) return refused("host owner identity");
    hostIds.add(pluginId);
    const result = Object.freeze({ pluginId, landingAppId: string(row.landingAppId, true), hostAppId: string(row.hostAppId, true) });
    next(); return result;
  });
  const names = new Set<string>();
  const playgrounds = (root.playgrounds as unknown[]).map(value => {
    const row = object(value, ["variant", "pluginId", "aliases", "app"], ["variant", "pluginId", "aliases"]);
    const pluginId = string(row.pluginId, true), variant = string(row.variant, true), aliases = list(row.aliases);
    if (!identities.has(pluginId)) return refused("playground owner identity");
    for (const name of [variant, ...aliases]) { check(); if (names.has(name) || (identities.has(name) && name !== pluginId)) return refused("ambiguous playground identity"); names.add(name); }
    const result = Object.freeze({ pluginId, variant, aliases, ...(row.app === undefined ? {} : { app: string(row.app) }) });
    next(); return result;
  });
  const result: PluginCatalogRowsV1 = Object.freeze({ version: 1, targets: Object.freeze(targets), hosts: Object.freeze(hosts), playgrounds: Object.freeze(playgrounds) });
  check();
  if (new TextEncoder().encode(JSON.stringify(result)).byteLength > control.maxBytes) return refused("byte credits");
  check();
  if (total === 0) { control.progress({ completed: 0, total: 0, work }); check(); }
  return result;
}

/** 📇️ Constructs one local runtime catalog from admitted serializable rows without module-global inventory. */
export function pluginCatalogV1(rows: PluginCatalogRowsV1): PluginCatalog {
  const targets = new Map(rows.targets.map(row => [row.pluginId, row]));
  const moduleUrl = (id: string, role: "plugin" | "extension") => {
    const row = targets.get(id);
    return row?.role === role ? row.moduleUrl : refused(`module ${id} is absent`);
  };
  return Object.freeze({ plugins: Object.freeze(rows.targets.filter(row => row.role === "plugin")), extensions: Object.freeze(rows.targets.filter(row => row.role === "extension")), hosts: rows.hosts, playgrounds: rows.playgrounds, moduleUrl: (id: string) => moduleUrl(id, "plugin"), extensionModuleUrl: (id: string) => moduleUrl(id, "extension") });
}

/** 🏠️ Resolves host ownership from the caller's explicit variant and target rows. */
export function isHostPlaygroundFilter(
  pluginFilter: string | undefined,
  playgrounds: readonly { readonly variant: string; readonly pluginId: string; readonly app?: string; readonly aliases: readonly string[] }[],
  targets: readonly { readonly pluginId: string; readonly host?: unknown }[],
): boolean {
  if (!pluginFilter) return true;
  const variantRow = playgrounds.find((row) => row.variant === pluginFilter || row.aliases.includes(pluginFilter));
  if (variantRow?.app !== undefined) return false;
  const pluginId = variantRow?.pluginId ?? pluginFilter;
  return targets.some((target) => target.pluginId === pluginId && target.host !== undefined);
}
// #endregion 🏠️HostPlaygroundFilter

