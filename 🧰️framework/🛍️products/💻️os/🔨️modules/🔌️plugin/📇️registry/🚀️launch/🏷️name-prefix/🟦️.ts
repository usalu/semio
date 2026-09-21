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

/** @emoji 🔍️ An EXACT taxonomy slug beats a suffix one: `🗄️stdio` holds both the artifact folder
 * `🧾️json` and the unrelated config leaf `🔣️.json`, and a suffix-first lookup picked whichever
 * `readdir` happened to yield first. */
function findArtifactFolder(folders: readonly string[], slug: string): string | undefined {
  const target = slug.toLowerCase();
  return (
    folders.find((folder) => taxonomyFolderSlug(folder).toLowerCase() === target) ??
    folders.find((folder) => taxonomyFolderSlug(folder).toLowerCase().endsWith(target))
  );
}

/** @emoji 🧬️ Splits a pinned app id `s.<plugin>.<artifact>@<standard>/<subset>#<role>` into the
 * three taxonomy coordinates its launch name is built from. */
function appDialect(app: string | undefined): { readonly artifact: string; readonly standard: string; readonly subset: string } | undefined {
  const match = app?.match(/^s\.[^.]+\.([^@]+)@([^/]+)\/([^#]+)#/);
  return match ? { artifact: match[1]!, standard: match[2]!, subset: match[3]! } : undefined;
}

/** @emoji 🪆️ The `🏅️standards/🔖️<standard>/🪆️subsets/<subset>` folder a row pins, or `undefined` for the
 * wildcard subset every single-dialect row carries. Two rows on the SAME artifact differ only here
 * (`🗄️stdio`'s `json` vs its `i-json` profile, `xml` vs `valid`), so the segment is what keeps their
 * launch names apart. */
function findSubsetFolder(pluginRoot: string, artifactFolder: string, standard: string, subset: string, repoRoot: string): string | undefined {
  if (subset === "*") return undefined;
  const standards = join(repoRoot, pluginRoot, "🗿️artifacts", artifactFolder, "🏅️standards");
  if (!existsSync(standards)) return undefined;
  const standardFolder = readdirSync(standards).find((folder) => taxonomyFolderSlug(folder).toLowerCase() === standard.toLowerCase());
  if (!standardFolder) return undefined;
  const subsets = join(standards, standardFolder, "🪆️subsets");
  if (!existsSync(subsets)) return undefined;
  return readdirSync(subsets).find((folder) => taxonomyFolderSlug(folder).toLowerCase() === subset.toLowerCase());
}

/** @emoji 🧲️ Joins the plugin folder with the artifact folder it pins. A plugin named after its one
 * artifact reads as `🗒️note`, not `🗒️note🗒️note` — but that collapse may only fire when the plugin
 * publishes a SINGLE playground row, because the bare plugin folder is also what a plugin-level row
 * (one naming no `app`, e.g. `🪐️space`'s studio host `s`) resolves to; collapsing a sibling onto it
 * makes two variants indistinguishable and one of their launchers unbuildable. */
function combinePluginAndArtifact(pluginDirectoryName: string, artifactFolder: string | undefined, collapseEponymous: boolean): string {
  if (!artifactFolder) return pluginDirectoryName;
  if (!collapseEponymous) return `${pluginDirectoryName}${artifactFolder}`;
  if (artifactFolder === pluginDirectoryName) return pluginDirectoryName;
  if (taxonomyFolderSlug(artifactFolder) === taxonomyFolderSlug(pluginDirectoryName)) return pluginDirectoryName;
  return `${pluginDirectoryName}${artifactFolder}`;
}

function prefixForHostedApp(app: string, playgrounds: readonly PlaygroundEntry[], repoRoot: string): string | undefined {
  const donor = playgrounds.find((row) => row.app === app && row.pluginId !== "demonstrator");
  return donor ? playgroundLaunchNamePrefix(donor, repoRoot, playgrounds) : undefined;
}

/**
 * @emoji 🏷️ Builds the `🛠️dev…` middle segment from plugin deployment folders and artifact taxonomy
 * paths: `<plugin folder><artifact folder><subset folder>`, each segment a real taxonomy folder name.
 *
 * 🔒️ INJECTIVE over playground variants — `🚀️launch/🟦️.ts` names every launcher after this prefix and
 * only synthesizes the ones the seed has not placed yet, so two variants sharing a prefix silently
 * cost one of them both of its launchers. The three segments carry exactly what distinguishes two
 * rows of one crate: the artifact they pin (`🏠️home` vs `🪐️space`), the dialect subset they pin
 * (`🧾️json` vs its `🛜️i-json` profile), and — through {@link combinePluginAndArtifact} — whether the
 * row pins an app at all (the studio host `s` keeps the bare `🪐️space`).
 */
export function playgroundLaunchNamePrefix(playground: PlaygroundEntry, repoRoot: string, playgrounds: readonly PlaygroundEntry[]): string {
  if (playground.brand?.startsWith("entwerfen-mit-bestand-")) {
    const hosted = playground.app ? prefixForHostedApp(playground.app, playgrounds, repoRoot) : undefined;
    return hosted ? `♻️mit-bestand${hosted}` : `♻️mit-bestand🎪️demonstrator`;
  }
  if (playground.pluginId === "demonstrator" && playground.variant === "demonstrator") return "♻️mit-bestand🧺️demonstrator";

  const pluginDirectoryName = moduleDirectoryName(playground.pluginId);
  const pluginRoot = pluginRootFromCratePath(playground.cratePath);
  const artifactFolders = listArtifactFolderNames(pluginRoot, repoRoot);
  const collapseEponymous = playgrounds.filter((row) => row.pluginId === playground.pluginId).length === 1;
  const dialect = appDialect(playground.app);
  const withArtifact = (artifactFolder: string | undefined): string => {
    const combined = combinePluginAndArtifact(pluginDirectoryName, artifactFolder, collapseEponymous);
    if (!artifactFolder || !dialect) return combined;
    const subsetFolder = findSubsetFolder(pluginRoot, artifactFolder, dialect.standard, dialect.subset, repoRoot);
    return subsetFolder ? `${combined}${subsetFolder}` : combined;
  };

  if (dialect) {
    const artifactFolder = findArtifactFolder(artifactFolders, dialect.artifact);
    if (artifactFolder) return withArtifact(artifactFolder);
  }

  const variantFolder = findArtifactFolder(artifactFolders, playground.variant);
  if (variantFolder) return withArtifact(variantFolder);

  const hyphenTail = playground.variant.includes("-") ? playground.variant.split("-").pop()! : undefined;
  if (hyphenTail) {
    const tailFolder = findArtifactFolder(artifactFolders, hyphenTail);
    if (tailFolder) return withArtifact(tailFolder);
  }

  if (playground.variant.startsWith(playground.pluginId)) {
    const suffix = playground.variant.slice(playground.pluginId.length);
    const suffixFolder = findArtifactFolder(artifactFolders, suffix);
    if (suffixFolder) return withArtifact(suffixFolder);
  }

  if (playground.variant === playground.pluginId) return pluginDirectoryName;
  return withArtifact(findArtifactFolder(artifactFolders, playground.variant));
}

/** @emoji ✂️ Keeps the renderer marker and the `👤️<slot>` multi-user discriminator after the
 * playground-specific prefix in a `3_dev` launch name. The user slot is part of a row's identity
 * exactly like the renderer is — dropping it collapses the two-user collaboration rows onto the
 * single-user one. */
export function devLaunchNameSuffix(name: string): string | undefined {
  const body = name.startsWith("🛠️dev") ? name.slice("🛠️dev".length) : name;
  for (const marker of ["⚛️react", "🧊️wgpu🌐️wasm", "🧊️wgpu🖥️native"]) {
    const index = body.indexOf(marker);
    if (index === -1) continue;
    const slot = body.slice(0, index).match(/👤️\d+$/);
    return slot ? `${slot[0]}${body.slice(index)}` : body.slice(index);
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

/** @emoji 🔄 Rewrites `3_dev` launch names whose command maps to a playground row so emojis match
 * taxonomy folders. A rename that would land on a name another row already carries is skipped: a
 * fixture row (`…🧩️concrete🌲️forest⚛️react`) normalizes onto its own plain sibling, and a stale-emoji
 * name a dev can still tell apart beats two indistinguishable rows in the Run panel. */
export function normalizeDevLaunchConfigurationNames(configurations: readonly object[], playgrounds: readonly PlaygroundEntry[], repoRoot: string): object[] {
  const byVariant = new Map(playgrounds.map((row) => [row.variant, row]));
  const taken = new Set((configurations as readonly { readonly name?: string }[]).map((entry) => entry.name).filter((name): name is string => name !== undefined));
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
    if (name === config.name || taken.has(name)) return entry;
    taken.delete(config.name);
    taken.add(name);
    return { ...(entry as Record<string, unknown>), name };
  });
}
