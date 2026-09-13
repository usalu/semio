import { MODULE_ROUTES } from "../../../../../🔌️plugin/📇️registry/📦️deployment/🟦️.ts";
import { repoCacheDirectory } from "../../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts";
import { existsSync, readFileSync, watch, type FSWatcher } from "node:fs";
import { basename, dirname, join } from "node:path";
import { playgroundAssetVitePlugins, resolveGisMapTileServeMode, semioEmojiIndexHtmlVitePlugin, staticDirVitePlugin, type PlaygroundAssetSpec } from "../../../../../../../../🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts";
import type { OwnedBuildConfig, OwnedBuildPlugin } from "../../../../../../../../🔨️modules/🖱️ui/🎯️targets/⚛️react/🛠️build-tooling/🟦️.ts";

export type WgpuBrowserConfiguration = {
  readonly workspace: string;
  readonly root: string;
  readonly profile: "dev" | "release";
  readonly variant?: string;
  readonly compilerRoot: string;
  readonly moduleRoot: string;
  readonly extensionRoot: string;
  readonly bootRoot: string;
  readonly workerRoot: string;
  readonly reloadFile: string;
  readonly assets: readonly PlaygroundAssetSpec[];
};

/** ♻️ Reloads browsers only after Nx activation publishes its completion marker. */
function completedArtifactReload(path: string): OwnedBuildPlugin {
  let watcher: FSWatcher | undefined;
  return {
    name: "wgpu-completed-artifact-reload",
    configureServer(server) {
      let previous = readFileSync(path, "utf8");
      watcher = watch(dirname(path), (_event, file) => {
        if (file !== null && file.toString() !== basename(path)) return;
        try {
          const next = readFileSync(path, "utf8");
          if (next === previous) return;
          previous = next;
          server.ws.send({ type: "full-reload", path: "*" });
        } catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOENT") console.error("WGPU activation observation failed", error); }
      });
      watcher.on("error", error => console.error("WGPU activation observation failed", error));
    },
    closeBundle() { watcher?.close(); watcher = undefined; },
  };
}

/** 🚏️ Projects each completed source to its one browser route. */
export function wgpuBrowserMounts(options: WgpuBrowserConfiguration): readonly (readonly [string, string])[] {
  return [
    ["/renderer-modules/wgpu", options.compilerRoot],
    ["/🚀️boot.js", options.bootRoot],
    ["/🎞️frame-worker.js", options.workerRoot],
    [MODULE_ROUTES.plugin, options.moduleRoot],
    [MODULE_ROUTES.extension, options.extensionRoot],
  ];
}

/** 🧊️ Mounts completed compiler/generator outputs and live modules without compiling or copying them. */
export function createWgpuBrowserConfig(options: WgpuBrowserConfiguration): OwnedBuildConfig {
  if (!["dev", "release"].includes(options.profile)) throw new Error("Select a WGPU browser profile");
  for (const [root, file] of [[options.compilerRoot, "semio-framework-os-renderer-wgpu.js"], [options.compilerRoot, "semio-framework-os-renderer-wgpu_bg.wasm"], [options.bootRoot, "🟨️.js"], [options.workerRoot, "🟨️.js"]]) if (!existsSync(join(root, file))) throw new Error("Missing prepared WGPU artifact: " + join(root, file));
  const mounts = wgpuBrowserMounts(options);
  return {
    root: options.root,
    publicDir: false,
    cacheDir: repoCacheDirectory(options.workspace, "vite", "wgpu", options.variant ?? "fixture", options.profile),
    optimizeDeps: { noDiscovery: true, include: [] },
    resolve: { alias: mounts.map(([find, replacement]) => ({ find, replacement })) },
    server: { watch: null, fs: { allow: [options.root, ...mounts.map(([, root]) => root)] } },
    plugins: [
      semioEmojiIndexHtmlVitePlugin(options.root),
      ...mounts.flatMap(([route, root]) => staticDirVitePlugin(options.workspace, { kind: "static-dir", route, root }).filter(plugin => plugin.apply !== "build")),
      ...playgroundAssetVitePlugins(options.workspace, options.assets, resolveGisMapTileServeMode(process.env.GIS_MAP_TILE_SERVE_MODE)),
      {
        name: "wgpu-artifact-route-boundary",
        enforce: "pre",
        configureServer(server) {
          server.middlewares.use((request, response, next) => {
            let path: string;
            try { path = decodeURIComponent((request.url ?? "").split("?")[0]); }
            catch { response.statusCode = 400; response.end(); return; }
            if (!mounts.some(([route]) => path === route || path.startsWith(route + "/"))) return next();
            response.statusCode = 404;
            response.end();
          });
        },
      },
      completedArtifactReload(options.reloadFile),
      { name: "wgpu-browser-selection", transformIndexHtml: () => options.variant ? [{ tag: "meta", attrs: { name: "semio-plugin", content: options.variant }, injectTo: "head" }] : [] },
      { name: "wgpu-serve-only", config(_config, environment) { if (environment.command !== "serve") throw new Error("Build the finite WGPU wasm target through Nx"); } },
    ],
  };
}
