import { existsSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { moduleDirectoryName } from "../../📦️deployment/🟦️.ts";
import type { PlaygroundEntry } from "../../🎮️playground/🔎️discovery/🟦️.ts";

const VARIATION_SELECTOR = "\uFE0F";

/** @emoji 🔤️ Returns the ASCII slug after an emoji identity's variation selector in a taxonomy folder name. */
export function taxonomyFolderSlug(folderName: string): string {
  const index = folderName.lastIndexOf(VARIATION_SELECTOR);
  return index === -1 ? folderName : folderName.slice(index + 1);
}

/** @emoji 📂 Resolves the owning plugin directory (`✏️s/🔌️plugins/…`) from a crate path. */
export function pluginRootFromCratePath(cratePath: string): string {
  const parts = cratePath.split("/");
  const pluginsIndex = parts.indexOf("🔌️plugins");
  if (pluginsIndex === -1 || pluginsIndex + 2 > parts.length) throw new Error(`🚀️launch/🏷️name-prefix/🟦️.ts: cratePath ${JSON.stringify(cratePath)} is not under ✏️s/🔌️plugins`);
  return parts.slice(0, pluginsIndex + 2).join("/");
}

function listArtifactFolderNames(pluginRoot: string, repoRoot: string): readonly string[] {
  const directory = join(repoRoot, pluginRoot, "🗿️artifacts");
  if (!existsSync(directory)) return [];
  return readdirSync(directory).filter((name) => name.length > 0 && !name.startsWith("."));
}

function findArtifactFolder(folders: readonly string[], slug: string): string | undefined {
  const target = slug.toLowerCase();
  return folders.find((folder) => {
    const folderSlug = taxonomyFolderSlug(folder).toLowerCase();
    return folderSlug === target || folderSlug.endsWith(target);
  });
}

function appArtifactSlug(app: string | undefined): string | undefined {
  if (!app) return undefined;
  const match = app.match(/^s\.[^.]+\.([^@]+)@/);
  return match?.[1];
}

function combinePluginAndArtifact(pluginDirectoryName: string, artifactFolder: string | undefined): string {
  if (!artifactFolder) return pluginDirectoryName;
  if (artifactFolder === pluginDirectoryName) return pluginDirectoryName;
  if (taxonomyFolderSlug(artifactFolder) === taxonomyFolderSlug(pluginDirectoryName)) return pluginDirectoryName;
  return `${pluginDirectoryName}${artifactFolder}`;
}

function prefixForHostedApp(app: string, playgrounds: readonly PlaygroundEntry[], repoRoot: string): string | undefined {
  const donor = playgrounds.find((row) => row.app === app && row.pluginId !== "demonstrator");
  return donor ? playgroundLaunchNamePrefix(donor, repoRoot, playgrounds) : undefined;
}

/** @emoji 🏷️ Builds the `🛠️dev…` middle segment from plugin deployment folders and artifact taxonomy paths. */
export function playgroundLaunchNamePrefix(playground: PlaygroundEntry, repoRoot: string, playgrounds: readonly PlaygroundEntry[]): string {
  if (playground.brand?.startsWith("entwerfen-mit-bestand-")) {
    const hosted = playground.app ? prefixForHostedApp(playground.app, playgrounds, repoRoot) : undefined;
    return hosted ? `♻️mit-bestand${hosted}` : `♻️mit-bestand🎪️demonstrator`;
  }
  if (playground.pluginId === "demonstrator" && playground.variant === "demonstrator") return "♻️mit-bestand🧺️demonstrator";

  const pluginDirectoryName = moduleDirectoryName(playground.pluginId);
  const pluginRoot = pluginRootFromCratePath(playground.cratePath);
  const artifactFolders = listArtifactFolderNames(pluginRoot, repoRoot);

  const appSlug = appArtifactSlug(playground.app);
  if (appSlug) {
    const artifactFolder = findArtifactFolder(artifactFolders, appSlug);
    if (artifactFolder) return combinePluginAndArtifact(pluginDirectoryName, artifactFolder);
  }

  const variantFolder = findArtifactFolder(artifactFolders, playground.variant);
  if (variantFolder) return combinePluginAndArtifact(pluginDirectoryName, variantFolder);

  const hyphenTail = playground.variant.includes("-") ? playground.variant.split("-").pop()! : undefined;
  if (hyphenTail) {
    const tailFolder = findArtifactFolder(artifactFolders, hyphenTail);
    if (tailFolder) return combinePluginAndArtifact(pluginDirectoryName, tailFolder);
  }

  if (playground.variant.startsWith(playground.pluginId)) {
    const suffix = playground.variant.slice(playground.pluginId.length);
    const suffixFolder = findArtifactFolder(artifactFolders, suffix);
    if (suffixFolder) return combinePluginAndArtifact(pluginDirectoryName, suffixFolder);
  }

  if (playground.variant === playground.pluginId) return pluginDirectoryName;
  return combinePluginAndArtifact(pluginDirectoryName, findArtifactFolder(artifactFolders, playground.variant));
}

/** @emoji ✂️ Keeps fixture/concrete suffixes after the playground-specific prefix in a `3_dev` launch name. */
export function devLaunchNameSuffix(name: string): string | undefined {
  const body = name.startsWith("🛠️dev") ? name.slice("🛠️dev".length) : name;
  for (const marker of ["⚛️react", "🧊️wgpu🌐️wasm", "🧊️wgpu🖥️native"]) {
    const index = body.indexOf(marker);
    if (index !== -1) return body.slice(index);
  }
  return undefined;
}

/** @emoji 🎯️ Resolves a playground variant id from standard dev/native launch commands. */
export function devLaunchVariantFromCommand(command: string | undefined, playgrounds: readonly PlaygroundEntry[]): string | undefined {
  if (!command) return undefined;
  const variants = new Set(playgrounds.map((row) => row.variant));
  const workspace = command.match(/workspace:dev -- ([A-Za-z0-9-]+)/);
  if (workspace?.[1] && variants.has(workspace[1])) return workspace[1];
  const native = command.match(/framework-renderer-wgpu:native -- ([A-Za-z0-9-]+)/);
  if (native?.[1] && variants.has(native[1])) return native[1];
  const mitBestand = command.match(/dev:mit-bestand:([A-Za-z0-9-]+)/);
  if (mitBestand?.[1] && variants.has(mitBestand[1])) return mitBestand[1];
  return undefined;
}

/** @emoji 🔄 Rewrites `3_dev` launch names whose command maps to a playground row so emojis match taxonomy folders. */
export function normalizeDevLaunchConfigurationNames(configurations: readonly object[], playgrounds: readonly PlaygroundEntry[], repoRoot: string): object[] {
  const byVariant = new Map(playgrounds.map((row) => [row.variant, row]));
  return configurations.map((entry) => {
    const config = entry as { readonly name?: string; readonly command?: string; readonly presentation?: { readonly group?: string } };
    if (config.presentation?.group !== "3_dev" || !config.name?.startsWith("🛠️dev")) return entry;
    const variant = devLaunchVariantFromCommand(config.command, playgrounds);
    if (!variant) return entry;
    const playground = byVariant.get(variant);
    if (!playground) return entry;
    const suffix = devLaunchNameSuffix(config.name);
    if (!suffix) return entry;
    const prefix = playgroundLaunchNamePrefix(playground, repoRoot, playgrounds);
    const name = `🛠️dev${prefix}${suffix}`;
    if (name === config.name) return entry;
    return { ...(entry as Record<string, unknown>), name };
  });
}
