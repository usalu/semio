/**
 * 🔍️ How a hub document's plugin module is resolved: by the hub catalog generation that serves the document, never by
 * whatever this device has staged. A hub document runs on its own program, named by the plugin and the SHA-256 of the
 * generation's module bundle ({@link hubProgramIdV1}); the device's locally staged program keeps serving its local
 * documents beside it. The program's bytes come from the first of three sources that holds the bundle's exact content
 * ({@link resolvePluginModuleSourceV1}): this device's module store when it holds the generation's bundle complete,
 * the locally staged module when every file of the bundle is byte-identical there (length, SHA-256 and BLAKE3), and
 * the hub otherwise. A staged module that differs in one file, lacks one, or stages another entry is not the
 * catalog's module and never runs a hub document.
 * @see ../🧬️schema/🔣️.json
 * @see ../🧫️fixtures/🔍️resolution/🔣️.json
 */
import {
  HUB_PROGRAM_ID_MAX_CHARS,
  HUB_PROGRAM_ID_PATTERN,
  PLUGIN_MODULE_STORE_V1,
  verifyTrustedPluginModuleFileV1,
  type TrustedPluginModuleBundleV1,
  type TrustedPluginModuleFileV1,
  type TrustedPluginModuleIndexEntryV1,
  type TrustedPluginModuleIndexV1,
} from "../🧬️schema/🟦️.ts";

/** 🪪️ `PluginModuleSourceV1`: where a hub program's verified bytes came from. */
export type PluginModuleSourceV1 = "store" | "local" | "hub";

/** 🔍️ The resolved source, with the verified local bytes when the staged module is the catalog's own. */
export type PluginModuleSourceResolutionV1 =
  | Readonly<{ source: "store" }>
  | Readonly<{ source: "local"; files: readonly (readonly [TrustedPluginModuleFileV1, Uint8Array])[] }>
  | Readonly<{ source: "hub" }>;

/** 🪪️ The program a hub document of `pluginId` runs on in the generation whose module bundle is `bundleSha256`. */
export function hubProgramIdV1(pluginId: string, bundleSha256: string): string {
  const programId = `${pluginId}${PLUGIN_MODULE_STORE_V1.programSeparator}${bundleSha256}`;
  if (programId.length > HUB_PROGRAM_ID_MAX_CHARS || !HUB_PROGRAM_ID_PATTERN.test(programId)) throw new Error(`hub program id for ${pluginId} is invalid`);
  return programId;
}

/** 🪪️ The plugin and bundle a hub program id names, or `null` for every other program id. */
export function parseHubProgramIdV1(programId: string): Readonly<{ pluginId: string; bundleSha256: string }> | null {
  if (programId.length > HUB_PROGRAM_ID_MAX_CHARS || !HUB_PROGRAM_ID_PATTERN.test(programId)) return null;
  const separator = programId.lastIndexOf(PLUGIN_MODULE_STORE_V1.programSeparator);
  return Object.freeze({ pluginId: programId.slice(0, separator), bundleSha256: programId.slice(separator + 1) });
}

/** 📂️ The URL root a bundle's paths resolve against inside a locally staged module whose entry is `localModuleUrl`,
 * or `null` when the staged entry is not the bundle's entry (then the staged module is not comparable). */
export function localPluginModuleRootV1(localModuleUrl: string, bundleEntry: string): string | null {
  const base = localModuleUrl.split(/[?#]/u)[0]!;
  let path: string;
  try {
    path = decodeURI(base);
  } catch {
    return null;
  }
  if (!path.endsWith(`/${bundleEntry}`)) return null;
  return path.slice(0, path.length - bundleEntry.length);
}

/** 🔍️ Resolves where a hub program's bytes come from. `storedComplete` says this device's store holds the generation's
 * bundle with every file; `readLocal` reads one bundle path from the locally staged module (`null` when nothing is
 * staged for the plugin, or when its entry is not the bundle's). Local files are compared the module's own directory
 * first, stopping at the first one that is absent or differs; `onVerified` counts every compared byte. */
export async function resolvePluginModuleSourceV1(input: Readonly<{
  bundle: TrustedPluginModuleBundleV1;
  storedComplete: boolean;
  readLocal: ((file: TrustedPluginModuleFileV1) => Promise<Uint8Array | null>) | null;
  signal: AbortSignal;
  onVerified?: (bytes: number) => void;
}>): Promise<PluginModuleSourceResolutionV1> {
  if (input.storedComplete) return { source: "store" };
  if (input.readLocal === null) return { source: "hub" };
  const own = `${input.bundle.moduleDirectory}/`;
  const ordered = [...input.bundle.files.filter((file) => file.path.startsWith(own)), ...input.bundle.files.filter((file) => !file.path.startsWith(own))];
  const files: (readonly [TrustedPluginModuleFileV1, Uint8Array])[] = [];
  for (const file of ordered) {
    input.signal.throwIfAborted();
    const bytes = await input.readLocal(file);
    input.signal.throwIfAborted();
    if (bytes === null || !(await verifyTrustedPluginModuleFileV1(file, bytes))) return { source: "hub" };
    input.onVerified?.(bytes.byteLength);
    files.push([file, bytes]);
  }
  return { source: "local", files };
}

/** 🎭️ The one package of a generation whose surfaces open `artifactKind` (a dialect artifact kind), or `null` when none
 * or more than one does. */
export function hubCatalogOwnerOfDialectV1(index: TrustedPluginModuleIndexV1, artifactKind: string): TrustedPluginModuleIndexEntryV1 | null {
  const owners = index.modules.filter((entry) => entry.dialectArtifactKinds.includes(artifactKind));
  return owners.length === 1 ? owners[0]! : null;
}

/** 🧩️ Every module a hub program of `pluginId` runs with, from the SAME generation: its dependency closure, itself, and
 * every extension of a member (with the extension's own dependencies), each after everything it depends on. `null`
 * when the plugin, a dependency or an extension's dependency is missing from the generation, or a dependency cycles. */
export function hubCatalogClosureV1(index: TrustedPluginModuleIndexV1, pluginId: string): readonly TrustedPluginModuleIndexEntryV1[] | null {
  const byId = new Map(index.modules.map((entry) => [entry.pluginId, entry] as const));
  const members = new Set<string>();
  const visiting = new Set<string>();
  const order: TrustedPluginModuleIndexEntryV1[] = [];
  const visit = (id: string): boolean => {
    if (members.has(id)) return true;
    const entry = byId.get(id);
    if (entry === undefined || visiting.has(id)) return false;
    visiting.add(id);
    if (!entry.dependencies.every(visit)) return false;
    visiting.delete(id);
    members.add(id);
    order.push(entry);
    return true;
  };
  if (!visit(pluginId)) return null;
  for (let grew = true; grew; ) {
    grew = false;
    for (const entry of index.modules) {
      if (entry.extendsPluginId === null || !members.has(entry.extendsPluginId) || members.has(entry.pluginId)) continue;
      if (!visit(entry.pluginId)) return null;
      grew = true;
    }
  }
  return order;
}
