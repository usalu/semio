import { join } from "node:path";

/** @emoji 📦️ Playground row fields that decide where a react release bundle is published. */
export type PlaygroundReactReleaseRow = { readonly variant: string; readonly distDir?: string; readonly pluginId?: string; readonly cratePath?: string };

/** @emoji 🧭️ Repo-root-relative plugin owner directory for a playground crate path. */
export function pluginOwnerRootFromCratePath(cratePath: string): string | undefined {
  const parts = cratePath.replaceAll("\\", "/").split("/");
  const pluginsIdx = parts.indexOf("🔌️plugins");
  if (pluginsIdx < 1 || parts[pluginsIdx - 1] !== "✏️s" || pluginsIdx + 1 >= parts.length) return undefined;
  return parts.slice(0, pluginsIdx + 2).join("/");
}

/** @emoji 📦️ CDN `dist/` directory for a playground variant — explicit `distDir` wins; otherwise `{owner}/dist` or `{owner}/dist/{variant}` when the plugin ships multiple sites. */
export function resolvePlaygroundDistDir(
  entry: Pick<PlaygroundReactReleaseRow, "variant" | "distDir" | "pluginId"> & { readonly cratePath: string },
  catalog: readonly Pick<PlaygroundReactReleaseRow, "variant" | "pluginId" | "distDir">[],
): string | undefined {
  if (entry.distDir) return entry.distDir;
  const owner = pluginOwnerRootFromCratePath(entry.cratePath);
  if (!owner || !entry.pluginId) return undefined;
  const variantsForPlugin = catalog.filter((row) => row.pluginId === entry.pluginId);
  return variantsForPlugin.length <= 1 ? `${owner}/dist` : `${owner}/dist/${entry.variant}`;
}

/** @emoji 📦️ Playground row with a resolved CDN output directory when the crate lives under `✏️s/🔌️plugins/`. */
export function withResolvedPlaygroundDistDir<T extends PlaygroundReactReleaseRow & { readonly cratePath: string; readonly pluginId: string }>(
  entry: T,
  catalog: readonly Pick<PlaygroundReactReleaseRow, "variant" | "pluginId" | "distDir">[],
): T {
  const distDir = resolvePlaygroundDistDir(entry, catalog);
  return distDir === undefined || distDir === entry.distDir ? entry : { ...entry, distDir };
}

/** @emoji 🧭️ Nx `outputs` path for `build-<variant>-react-release` — plugin `distDir` when set, else framework-os-dev staging. */
export function playgroundReactReleaseNxOutput(projectRoot: string, playground: PlaygroundReactReleaseRow): string {
  const name = `build-${playground.variant}-react-release`;
  return playground.distDir ? `{workspaceRoot}/${playground.distDir}` : `{projectRoot}/dist/${name}`;
}

/** @emoji 📂 Absolute filesystem path `buildViteArtifact` publishes into for one variant. */
export function playgroundReactReleaseOutputPath(workspaceRoot: string, osDevTypescriptPackageRoot: string, playground: PlaygroundReactReleaseRow): string {
  return playground.distDir ? join(workspaceRoot, playground.distDir) : join(osDevTypescriptPackageRoot, "dist", `build-${playground.variant}-react-release`);
}
