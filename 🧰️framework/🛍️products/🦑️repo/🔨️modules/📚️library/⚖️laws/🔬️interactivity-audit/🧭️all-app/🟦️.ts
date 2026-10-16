/** 🛰️ Interactivity audit: metadata-derived all-app discovery ledger.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🏁️run/🟦️.ts
 */
import { policyReadFileSafe } from "../../../🔍️discovery/📖️source-access/🟦️.ts";
import { policyWalkRelFiles } from "../../../🔍️discovery/🚶️file-walk/🟦️.ts";
import { dirname } from "node:path";
import type { ToolJobStaticRow } from "../../🎯️tool-job-coverage/📋️inventory/🟦️.ts";

//#region 🧭️AllAppDiscovery
const INTERACTIVITY_ALL_APP_DESCRIPTOR_CAPACITY = 256;
export const INTERACTIVITY_ALL_APP_APPS_PER_DESCRIPTOR_CAPACITY = 64;
export const INTERACTIVITY_ALL_APP_ACTIONS_PER_APP_CAPACITY = 512;
const INTERACTIVITY_ALL_APP_DESCRIPTOR_NAME = "🔣️.json";
/** 🧩️ The one child dir under a plugin root whose own `🔣️.json` is also a descriptor. */
const INTERACTIVITY_ALL_APP_EXTENSIONS_DIR = "🧩️extensions";
/** 🧫️ Taxonomy ownership dirs (`testOwnerKinds`) that sit BESIDE an owner's real children and are
 * therefore never themselves an extension — `🌊️flow/🧩️extensions/🧫️fixtures/🔣️.json` is the flow
 * extensions' shared fixture table, not a fourteenth extension. */
const INTERACTIVITY_ALL_APP_OWNERSHIP_DIRS: ReadonlySet<string> = new Set(["🧫️fixtures", "🧪️tests"]);
/** 🎮️ The generated playground catalog the dashboard registry lists as `playground:<variant>` commands. */
const INTERACTIVITY_ALL_APP_PLAYGROUND_FILE = "✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🚀️playgrounds.json";
/** 🎛️ The dashboard registry source that owns the renderer axis of every playground command. */
const INTERACTIVITY_ALL_APP_REGISTRY_FILE = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🎮️registry/🦀️.rs";
/** 🧬️ The schema every `metadata.semio.dashboard` declaration is written against. */
const INTERACTIVITY_ALL_APP_REGISTRY_SCHEMA_FILE = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🎮️registry/🔣️.json";
/** 🏠️ The workspace root project whose build tasks and dashboard declarations the gates are checked against. */
const INTERACTIVITY_ALL_APP_ROOT_MANIFEST_FILE = "📋️project.json";
/** 🔌️ Every tree whose depth-4 and extension `🔣️.json` files are plugin descriptors. */
const INTERACTIVITY_ALL_APP_DESCRIPTOR_ROOTS: readonly string[] = ["✏️s/🔌️plugins", "🌎️hub/🧩️compositions"];
/** ⚖️ Verification gates the dashboard must be able to start: each one is `workspace:verify` with these free arguments.
 *
 * The dashboard is the only control plane, so a verification verb that no declared command reaches is a
 * verb no dev can run. The workspace `verify` target forwards free arguments to `📜️script.ts verify`; the law
 * is that this target stays declared and launchable with a positional text parameter. */
export const INTERACTIVITY_ALL_APP_REQUIRED_GATES: readonly { readonly id: string; readonly check: readonly string[] }[] = [
  { id: "interactivity", check: ["interactivity"] },
  { id: "interactivity-tool-jobs", check: ["interactivity", "tool-jobs"] },
  { id: "interactivity-apps", check: ["interactivity", "apps"] },
  { id: "interactivity-apps-actions", check: ["interactivity", "apps", "--actions"] },
  { id: "dependencies", check: ["dependencies"] },
  { id: "dependencies-literal-external", check: ["dependencies", "literal-external"] },
  { id: "composed-child-refs", check: ["composed-child-refs"] },
];

type InteractivityAllAppAction = { appId: string; windowId: string; actionId: string; disposition?: string };
type InteractivityAllAppDescriptor = { file: string; kind: "app" | "extension"; pluginId: string; parentPluginId?: string; appIds: string[]; actions: InteractivityAllAppAction[] };
type InteractivityAllAppPlayground = { variant: string; pluginId: string; react: number; wgpu: number; appId?: string };
type InteractivityAllAppDiscovery = {
  descriptors: InteractivityAllAppDescriptor[];
  appCount: number;
  surfaceCount: number;
  actionCount: number;
  migratedActionCount: number;
  missingActionCount: number;
  surfaceCoveredAppCount: number;
  surfaceMissingAppCount: number;
  duplicateAppIds: { id: string; files: string[] }[];
  playgrounds: number;
  renderers: string[];
  gates: number;
  failures: string[];
};

/** 🌐️Returns whether a descriptor label exposes equivalent English and German text without a default-language field. */
function interactivityAllAppLocalizedLabelExact(value: unknown): boolean {
  if (!value || typeof value !== "object") return false;
  const label = value as Record<string, unknown>;
  if (Object.hasOwn(label, "default")) return false;
  const localized = [label.native, label.reuse].filter((entry): entry is Record<string, unknown> => Boolean(entry) && typeof entry === "object");
  return localized.length > 0 && localized.every((entry) => typeof entry.en === "string" && entry.en.length > 0 && typeof entry.de === "string" && entry.de.length > 0 && !Object.hasOwn(entry, "default"));
}

/** 🪢️ The plugin id an extension registers — the FIRST argument of `ExtensionBundle::new(…)`, either a
 * string literal or a `const … : &str` resolved in the same source.
 *
 * Not `EXTENSION_ID`. That constant is the extension's short TOPIC id (`"bim"`, `"draw"`), passed to
 * `flow_extension_topic_contribution`; the bundle registers `"flow-extension-bim"` /
 * `"flow-extension-draw"` — the spelling `🌊️flow/🧩️extensions/🧫️fixtures/🔣️.json` maps each slug to.
 * Reading `EXTENSION_ID` as the plugin id made `🖍️draw`'s extension collide with the real `🖍️draw`
 * plugin under this verb's own ambiguity law. */
function interactivityAllAppExtensionBundleId(source: string): string {
  const argument = source.match(/ExtensionBundle::new\(\s*([A-Za-z_][A-Za-z0-9_]*|"[^"]*")/u)?.[1];
  if (argument === undefined) return "";
  if (argument.startsWith('"')) return argument.slice(1, -1).trim();
  return source.match(new RegExp(`const\\s+${argument}\\s*:\\s*&str\\s*=\\s*"([^"]+)"`, "u"))?.[1]?.trim() ?? "";
}

/** 🧩️Parses one schema-first descriptor through an owned primitive-only discovery boundary. */
export function interactivityAllAppDescriptorFromSource(file: string, source: string, extensionSource = ""): { row?: InteractivityAllAppDescriptor; failures: string[] } {
  const failures: string[] = [];
  let descriptor: Record<string, unknown>;
  try {
    descriptor = JSON.parse(source) as Record<string, unknown>;
  } catch {
    return { failures: [`${file}: descriptor is not valid JSON`] };
  }
  const isExtension = file.includes(`/${INTERACTIVITY_ALL_APP_EXTENSIONS_DIR}/`);
  const expectedRole = isExtension ? "extension" : "plugin";
  if (descriptor.descriptorVersion !== 1) failures.push(`${file}: descriptorVersion must be 1`);
  if (descriptor.role !== expectedRole) failures.push(`${file}: role must be ${expectedRole}`);
  const manifest = descriptor.manifest && typeof descriptor.manifest === "object" ? descriptor.manifest as Record<string, unknown> : undefined;
  if (!manifest) return { failures: [...failures, `${file}: manifest is missing`] };
  const apps = Array.isArray(manifest.apps) ? manifest.apps : undefined;
  if (!apps) return { failures: [...failures, `${file}: manifest.apps is missing`] };
  if (isExtension) {
    const pluginId = interactivityAllAppExtensionBundleId(extensionSource);
    const parentPluginId = extensionSource.match(/\.extends\(\s*"([^"]+)"\s*\)/)?.[1] ?? "";
    if (pluginId === "") failures.push(`${file}: extension source declares no resolvable ExtensionBundle::new(<pluginId>, …)`);
    if (parentPluginId === "") failures.push(`${file}: extension source has no parent .extends(...) activation`);
    if (apps.length !== 0) failures.push(`${file}: extension descriptor must delegate app activation to its parent instead of duplicating apps`);
    return { row: { file, kind: "extension", pluginId, ...(parentPluginId ? { parentPluginId } : {}), appIds: [], actions: [] }, failures };
  }
  const pluginId = typeof manifest.pluginId === "string" ? manifest.pluginId.trim() : "";
  if (pluginId === "" || pluginId === "empty") failures.push(`${file}: pluginId must be a non-placeholder owned identifier`);
  if (apps.length === 0) failures.push(`${file}: manifest.apps is empty and has no parent activation evidence`);
  if (apps.length > INTERACTIVITY_ALL_APP_APPS_PER_DESCRIPTOR_CAPACITY) failures.push(`${file}: ${apps.length} apps exceed fixed capacity ${INTERACTIVITY_ALL_APP_APPS_PER_DESCRIPTOR_CAPACITY}`);
  const appIds: string[] = [];
  const actions: InteractivityAllAppAction[] = [];
  for (const [index, appValue] of apps.slice(0, INTERACTIVITY_ALL_APP_APPS_PER_DESCRIPTOR_CAPACITY).entries()) {
    const app = appValue && typeof appValue === "object" ? appValue as Record<string, unknown> : undefined;
    if (!app) {
      failures.push(`${file}: app ${index} is not an object`);
      continue;
    }
    const id = typeof app.id === "string" ? app.id.trim() : "";
    if (id === "") failures.push(`${file}: app ${index} has no owned id`);
    else appIds.push(id);
    if (!interactivityAllAppLocalizedLabelExact(app.label)) failures.push(`${file}: app ${id || index} lacks equivalent en/de labels or declares a default language`);
    const windows = Array.isArray(app.windowKinds) ? app.windowKinds : [];
    let actionCount = 0;
    for (const [windowIndex, windowValue] of windows.entries()) {
      const window = windowValue && typeof windowValue === "object" ? windowValue as Record<string, unknown> : undefined;
      if (!window) {
        failures.push(`${file}: app ${id || index} window ${windowIndex} is not an object`);
        continue;
      }
      const windowId = typeof window.id === "string" ? window.id.trim() : "";
      const windowActions = Array.isArray(window.actions) ? window.actions : [];
      actionCount += windowActions.length;
      for (const [actionIndex, actionValue] of windowActions.slice(0, INTERACTIVITY_ALL_APP_ACTIONS_PER_APP_CAPACITY).entries()) {
        const action = actionValue && typeof actionValue === "object" ? actionValue as Record<string, unknown> : undefined;
        if (!action) {
          failures.push(`${file}: app ${id || index} window ${windowId || windowIndex} action ${actionIndex} is not an object`);
          continue;
        }
        const actionId = typeof action.id === "string" ? action.id.trim() : "";
        if (actionId === "") failures.push(`${file}: app ${id || index} window ${windowId || windowIndex} action ${actionIndex} has no owned id`);
        if (!interactivityAllAppLocalizedLabelExact(action.label)) failures.push(`${file}: action ${id || index}/${windowId || windowIndex}/${actionId || actionIndex} lacks equivalent en/de labels or declares a default language`);
        const semantics = action.semantics && typeof action.semantics === "object" ? action.semantics as Record<string, unknown> : undefined;
        const execution = semantics?.execution && typeof semantics.execution === "object" ? semantics.execution as Record<string, unknown> : undefined;
        const disposition = typeof execution?.interactiveJob === "string" ? execution.interactiveJob : undefined;
        actions.push({ appId: id, windowId, actionId, ...(disposition ? { disposition } : {}) });
      }
    }
    if (actionCount > INTERACTIVITY_ALL_APP_ACTIONS_PER_APP_CAPACITY) failures.push(`${file}: app ${id || index} has ${actionCount} action rows exceeding fixed capacity ${INTERACTIVITY_ALL_APP_ACTIONS_PER_APP_CAPACITY}`);
  }
  return { row: { file, kind: "app", pluginId, appIds, actions }, failures };
}

/** 🎛️Fails every UI-declared action that has not joined the bounded production job migration. */
export function interactivityAllAppActionDispositionFailures(descriptors: InteractivityAllAppDescriptor[]): string[] {
  return descriptors.flatMap((descriptor) => descriptor.actions
    .filter((action) => action.disposition !== "migrated")
    .map((action) => `${descriptor.file}: ${action.appId}/${action.windowId}/${action.actionId} has interactiveJob=${JSON.stringify(action.disposition ?? "missing")}`));
}

/** 🧷️Joins truthful descriptor dispositions to the exact owner-local production command census. */
export function interactivityAllAppActionProductionFailures(descriptors: InteractivityAllAppDescriptor[], acceptedCommands: readonly ToolJobStaticRow[], acceptedReservedRoutes: readonly string[]): string[] {
  const acceptedShared = new Set(acceptedReservedRoutes);
  const acceptedOwned = new Set(acceptedCommands.map((row) => `${row.file.split("/").slice(0, 3).join("/")}\0${row.id}`));
  return descriptors.flatMap((descriptor) => {
    const owner = descriptor.file.split("/").slice(0, 3).join("/");
    return descriptor.actions
      .filter((action) => action.disposition === "migrated" && !acceptedShared.has(action.actionId) && !acceptedOwned.has(`${owner}\0${action.actionId}`))
      .map((action) => `${descriptor.file}: ${action.appId}/${action.windowId}/${action.actionId} declares migrated without an accepted owner-local production command or shared reserved route`);
  });
}

/** 🎮️Reads the generated playground catalog the dashboard registry lists as `playground:<variant>` commands. */
export function interactivityAllAppPlaygroundsFromSource(source: string): { rows: InteractivityAllAppPlayground[]; failures: string[] } {
  const failures: string[] = [];
  let catalog: unknown;
  try {
    catalog = JSON.parse(source);
  } catch {
    return { rows: [], failures: [`${INTERACTIVITY_ALL_APP_PLAYGROUND_FILE}: invalid JSON`] };
  }
  if (!Array.isArray(catalog)) return { rows: [], failures: [`${INTERACTIVITY_ALL_APP_PLAYGROUND_FILE}: catalog is not a list`] };
  const rows: InteractivityAllAppPlayground[] = [];
  const identities = new Set<string>();
  for (const value of catalog) {
    const row = value && typeof value === "object" ? value as Record<string, unknown> : {};
    const ports = row.ports && typeof row.ports === "object" ? row.ports as Record<string, unknown> : {};
    const variant = typeof row.variant === "string" ? row.variant : "";
    const pluginId = typeof row.pluginId === "string" ? row.pluginId : "";
    const react = typeof ports.react === "number" ? ports.react : 0;
    const wgpu = typeof ports.wgpu === "number" ? ports.wgpu : 0;
    if (variant === "" || pluginId === "") {
      failures.push(`${INTERACTIVITY_ALL_APP_PLAYGROUND_FILE}: malformed playground row`);
      continue;
    }
    if (identities.has(variant)) failures.push(`${INTERACTIVITY_ALL_APP_PLAYGROUND_FILE}: duplicate variant ${JSON.stringify(variant)}`);
    identities.add(variant);
    rows.push({ variant, pluginId, react, wgpu, ...(typeof row.app === "string" && row.app !== "" ? { appId: row.app } : {}) });
  }
  if (rows.length === 0) failures.push(`${INTERACTIVITY_ALL_APP_PLAYGROUND_FILE}: no playground rows discovered`);
  return { rows, failures };
}

/** 🎛️Reads the renderer axis every dashboard playground command offers from the registry source that owns it. */
export function interactivityAllAppRenderersFromSource(source: string): { renderers: string[]; failures: string[] } {
  const list = source.match(/\bconst\s+RENDERERS\s*:\s*&\[&str\]\s*=\s*&\[([^\]]*)\]/u)?.[1];
  const renderers = list === undefined ? [] : [...list.matchAll(/"([^"]+)"/gu)].map((match) => match[1]!);
  if (renderers.length === 0) return { renderers, failures: [`${INTERACTIVITY_ALL_APP_REGISTRY_FILE}: the RENDERERS axis of the playground commands is not readable`] };
  const react = renderers.filter((renderer) => renderer === "react").length;
  const native = renderers.filter((renderer) => renderer.endsWith("native")).length;
  const wasm = renderers.filter((renderer) => renderer.endsWith("wasm")).length;
  return { renderers, failures: react === 1 && native === 1 && wasm === 1 && renderers.length === 3 ? [] : [`${INTERACTIVITY_ALL_APP_REGISTRY_FILE}: RENDERERS must offer exactly one React, one Wasm and one native surface, found ${renderers.join(", ")}`] };
}

/** 🚀️Returns the renderers a playground variant can be started with: React needs its React port, every other renderer the WGPU port. */
export function interactivityAllAppPlaygroundSurfaces(playground: InteractivityAllAppPlayground, renderers: readonly string[]): string[] {
  return renderers.filter((renderer) => (renderer === "react" ? playground.react : playground.wgpu) > 0);
}

/** 🚀️Proves every owner-qualified app context has a playground command that starts on every renderer (React, Wasm, native). */
export function interactivityAllAppSurfaceCoverageFailures(descriptors: readonly InteractivityAllAppDescriptor[], playgrounds: readonly InteractivityAllAppPlayground[], renderers: readonly string[]): string[] {
  const completeVariants = new Set(playgrounds.filter((playground) => interactivityAllAppPlaygroundSurfaces(playground, renderers).length === renderers.length).map((playground) => playground.variant));
  const roleNeutral = (id: string) => id.replace(/#(?:editor|viewer)$/u, "#surface");
  const failures: string[] = [];
  for (const descriptor of descriptors.filter((row) => row.kind === "app")) for (const appId of descriptor.appIds) {
    const candidates = playgrounds.filter((playground) => playground.pluginId === descriptor.pluginId && (!playground.appId || roleNeutral(playground.appId) === roleNeutral(appId)));
    if (!candidates.some((playground) => completeVariants.has(playground.variant))) failures.push(`${descriptor.file}: ${appId} has no owner-qualified playground command with React + WGPU Wasm + WGPU native surfaces`);
  }
  return failures;
}

/** ⚖️Proves every required verification gate is startable from the dashboard: the workspace `verify` build task forwards free arguments to `📜️script.ts verify` and declares one positional text parameter. */
export function interactivityAllAppGateFailures(manifestSource: string, schemaSource: string, gates: readonly { readonly id: string; readonly check: readonly string[] }[] = INTERACTIVITY_ALL_APP_REQUIRED_GATES): string[] {
  const failures: string[] = [];
  const record = (value: unknown): Record<string, unknown> => (value && typeof value === "object" && !Array.isArray(value) ? value as Record<string, unknown> : {});
  let manifest: Record<string, unknown>;
  let schema: Record<string, unknown>;
  try {
    manifest = record(JSON.parse(manifestSource));
  } catch {
    return [`${INTERACTIVITY_ALL_APP_ROOT_MANIFEST_FILE}: invalid JSON`];
  }
  try {
    schema = record(JSON.parse(schemaSource));
  } catch {
    return [`${INTERACTIVITY_ALL_APP_REGISTRY_SCHEMA_FILE}: invalid JSON`];
  }
  const kinds = record(record(record(record(schema.$defs).Parameter).properties).kind).enum;
  if (!Array.isArray(kinds) || !kinds.includes("text")) failures.push(`${INTERACTIVITY_ALL_APP_REGISTRY_SCHEMA_FILE}: the Parameter schema declares no text kind`);
  if (!Object.hasOwn(record(record(record(schema.$defs).Parameter).properties), "valuePositional")) failures.push(`${INTERACTIVITY_ALL_APP_REGISTRY_SCHEMA_FILE}: the Parameter schema declares no valuePositional`);
  if (manifest.name !== "workspace") failures.push(`${INTERACTIVITY_ALL_APP_ROOT_MANIFEST_FILE}: the root project must be named workspace`);
  const verify = record(record(manifest.targets).verify);
  if (Object.keys(verify).length === 0) failures.push(`${INTERACTIVITY_ALL_APP_ROOT_MANIFEST_FILE}: target workspace:verify is missing, so no verification gate is a dashboard command`);
  const options = record(verify.options);
  if (typeof options.command !== "string" || !options.command.endsWith("📜️script.ts verify") || options.forwardAllArgs !== true) failures.push(`${INTERACTIVITY_ALL_APP_ROOT_MANIFEST_FILE}: workspace:verify must forward free arguments to 📜️script.ts verify`);
  const parameters = record(record(record(verify.metadata).semio).dashboard).parameters;
  const positional = (Array.isArray(parameters) ? parameters : []).map(record).filter((parameter) => parameter.kind === "text" && parameter.valuePositional === true);
  if (positional.length !== 1) failures.push(`${INTERACTIVITY_ALL_APP_ROOT_MANIFEST_FILE}: workspace:verify must declare exactly one positional text parameter in metadata.semio.dashboard, found ${positional.length}`);
  const seen = new Set<string>();
  for (const gate of gates) {
    const key = gate.check.join(" ");
    if (gate.check.length === 0 || gate.check.some((word) => word === "" || /\s/u.test(word))) failures.push(`gate ${gate.id}: check must be a list of non-empty words`);
    if (seen.has(key)) failures.push(`gate ${gate.id}: duplicate check ${JSON.stringify(key)}`);
    seen.add(key);
  }
  return failures;
}

/** 🪪️ Whether a repo-relative path is a plugin-descriptor COORDINATE rather than merely a file that
 * shares the taxonomy's generic JSON leaf name.
 *
 * `🔣️.json` is the taxonomy's JSON component filename everywhere — schemas, fixtures, mutation
 * descriptors, oracle tables. Matching it by name alone claims **20 368** files under `✏️s/🔌️plugins`
 * as plugin descriptors, of which 46 are: one per plugin root and one per `🧩️extensions/<id>`. The
 * other 20 322 were then rejected one by one with `manifest is missing` / `role must be plugin`, which
 * is how this verb reached 776 failures while reporting `descriptors=3` — the fixed capacity truncated
 * the walk to the first 256 paths in sort order, so real descriptors never got read at all. */
function interactivityAllAppIsDescriptorCoordinate(relPath: string): boolean {
  const parts = relPath.split("/");
  if (parts.length === 4) return true;
  return parts.length === 6 && parts[3] === INTERACTIVITY_ALL_APP_EXTENSIONS_DIR && !INTERACTIVITY_ALL_APP_OWNERSHIP_DIRS.has(parts[4]!);
}

/** 🧭️Discovers every plugin descriptor and the dashboard dev surfaces of every playground without a maintained product allowlist. */
export function interactivityAllAppDiscovery(repoRoot: string): InteractivityAllAppDiscovery {
  const failures: string[] = [];
  const files = policyWalkRelFiles(repoRoot, [...INTERACTIVITY_ALL_APP_DESCRIPTOR_ROOTS], (path, name) => name === INTERACTIVITY_ALL_APP_DESCRIPTOR_NAME && interactivityAllAppIsDescriptorCoordinate(path));
  if (files.length === 0) failures.push("no plugin descriptors discovered");
  if (files.length > INTERACTIVITY_ALL_APP_DESCRIPTOR_CAPACITY) failures.push(`${files.length} descriptors exceed fixed capacity ${INTERACTIVITY_ALL_APP_DESCRIPTOR_CAPACITY}`);
  const descriptors: InteractivityAllAppDescriptor[] = [];
  const pluginOwners = new Map<string, string[]>();
  const appOwners = new Map<string, string[]>();
  for (const file of files.slice(0, INTERACTIVITY_ALL_APP_DESCRIPTOR_CAPACITY)) {
    const extensionSource = file.includes(`/${INTERACTIVITY_ALL_APP_EXTENSIONS_DIR}/`) ? policyReadFileSafe(repoRoot, `${dirname(file)}/🦀️.rs`) : "";
    const parsed = interactivityAllAppDescriptorFromSource(file, policyReadFileSafe(repoRoot, file), extensionSource);
    failures.push(...parsed.failures);
    if (!parsed.row) continue;
    descriptors.push(parsed.row);
    pluginOwners.set(parsed.row.pluginId, [...(pluginOwners.get(parsed.row.pluginId) ?? []), file]);
    for (const id of parsed.row.appIds) appOwners.set(id, [...(appOwners.get(id) ?? []), file]);
  }
  for (const [pluginId, owners] of pluginOwners) if (pluginId !== "" && owners.length > 1) failures.push(`pluginId ${JSON.stringify(pluginId)} is ambiguous across ${owners.join(", ")}`);
  const appPlugins = new Set(descriptors.filter((descriptor) => descriptor.kind === "app" && descriptor.appIds.length > 0).map((descriptor) => descriptor.pluginId));
  for (const extension of descriptors.filter((descriptor) => descriptor.kind === "extension")) if (!extension.parentPluginId || !appPlugins.has(extension.parentPluginId)) failures.push(`${extension.file}: parent ${JSON.stringify(extension.parentPluginId ?? "")} has no discovered app activation`);
  const duplicateAppIds = [...appOwners]
    .filter(([, owners]) => owners.length > 1)
    .map(([id, owners]) => ({ id, files: owners }))
    .sort((left, right) => left.id.localeCompare(right.id));
  const gateFailures = interactivityAllAppGateFailures(policyReadFileSafe(repoRoot, INTERACTIVITY_ALL_APP_ROOT_MANIFEST_FILE), policyReadFileSafe(repoRoot, INTERACTIVITY_ALL_APP_REGISTRY_SCHEMA_FILE));
  failures.push(...gateFailures);
  const axis = interactivityAllAppRenderersFromSource(policyReadFileSafe(repoRoot, INTERACTIVITY_ALL_APP_REGISTRY_FILE));
  failures.push(...axis.failures);
  const playgrounds = interactivityAllAppPlaygroundsFromSource(policyReadFileSafe(repoRoot, INTERACTIVITY_ALL_APP_PLAYGROUND_FILE));
  failures.push(...playgrounds.failures);
  const surfaceFailures = interactivityAllAppSurfaceCoverageFailures(descriptors, playgrounds.rows, axis.renderers);
  failures.push(...surfaceFailures);
  const descriptorAppCount = descriptors.reduce((count, descriptor) => count + descriptor.appIds.length, 0);
  return {
    descriptors: descriptors.sort((left, right) => left.file.localeCompare(right.file)),
    appCount: descriptorAppCount,
    surfaceCount: playgrounds.rows.reduce((count, playground) => count + interactivityAllAppPlaygroundSurfaces(playground, axis.renderers).length, 0),
    actionCount: descriptors.reduce((count, descriptor) => count + descriptor.actions.length, 0),
    migratedActionCount: descriptors.reduce((count, descriptor) => count + descriptor.actions.filter((action) => action.disposition === "migrated").length, 0),
    missingActionCount: descriptors.reduce((count, descriptor) => count + descriptor.actions.filter((action) => action.disposition !== "migrated").length, 0),
    surfaceCoveredAppCount: descriptorAppCount - surfaceFailures.length,
    surfaceMissingAppCount: surfaceFailures.length,
    duplicateAppIds,
    playgrounds: playgrounds.rows.length,
    renderers: axis.renderers,
    gates: gateFailures.length === 0 ? INTERACTIVITY_ALL_APP_REQUIRED_GATES.length : 0,
    failures: [...new Set(failures)].sort(),
  };
}

/** 🔮️Returns canonical JSON through the test-only TypeScript parser behind an owned string boundary. */
export async function interactivityAllAppOracleJson(source: string): Promise<string> {
  const typescript = await import("typescript");
  const parsed = typescript.parseConfigFileTextToJson("interactivity-all-app-oracle.json", source);
  if (parsed.error) throw new Error("[verify interactivity apps] TypeScript oracle rejected the fixture");
  return JSON.stringify(parsed.config);
}


//#endregion 🧭️AllAppDiscovery
