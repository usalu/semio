import { MODULE_ROUTES } from "../../../../../🔌️plugin/📇️registry/📦️deployment/🟦️.ts";
import { FONT_ASSET } from "../../../../../♾️infinite/🖼️canvas/🔤️fonts/🟦️.ts";
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
/** @emoji 🧭️ The per-SERVER boot axes this serve bakes into the page, the wgpu twin of React's
 * `VITE_SEMIO_*` build-time env (`🧑‍💻dev/🟦️.ts`): the `?query=` is the per-navigation axis, a
 * `<meta name="semio-*">` is the per-server default. Names and precedence are owned by
 * `../🧭️boot-descriptor/🟦️.ts`'s `WGPU_BOOT_META_NAMES`, restated here because a Vite config cannot
 * import the browser bundle. An unset variable injects no tag at all, so the page never carries an
 * empty pin. */
function bootAxisMetaTags(variant: string | undefined): { tag: string; attrs: Record<string, string>; injectTo: "head" }[] {
  const axes: [string, string | undefined][] = [
    ["semio-plugin", variant],
    ["semio-app-id", process.env.SEMIO_APP_ID],
    ["semio-app-role", process.env.SEMIO_APP_ROLE],
    ["semio-brand", process.env.SEMIO_BRAND],
    ["semio-default-example", process.env.SEMIO_DEFAULT_EXAMPLE],
    ["semio-locked-example", process.env.SEMIO_LOCKED_EXAMPLE],
    ["semio-locked-locale", process.env.SEMIO_LOCKED_LOCALE],
    ["semio-locked-terminology", process.env.SEMIO_LOCKED_TERMINOLOGY],
    ["semio-locked-theme", process.env.SEMIO_LOCKED_THEME],
    ["semio-locked-appearance", process.env.SEMIO_LOCKED_APPEARANCE],
  ];
  return axes.filter((entry): entry is [string, string] => Boolean(entry[1])).map(([name, content]) => ({ tag: "meta", attrs: { name, content }, injectTo: "head" }));
}

export function createWgpuBrowserConfig(options: WgpuBrowserConfiguration): OwnedBuildConfig {
  if (!["dev", "release"].includes(options.profile)) throw new Error("Select a WGPU browser profile");
  for (const [root, file] of [[options.compilerRoot, "semio-framework-os-renderer-wgpu.js"], [options.compilerRoot, "semio-framework-os-renderer-wgpu_bg.wasm"], [options.bootRoot, "🟨️.js"], [options.workerRoot, "🟨️.js"], [join(options.moduleRoot, "🪞️vendor"), FONT_ASSET]]) if (!existsSync(join(root, file))) throw new Error("Missing prepared WGPU artifact: " + join(root, file));
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
      { name: "wgpu-browser-selection", transformIndexHtml: () => bootAxisMetaTags(options.variant) },
      { name: "wgpu-serve-only", config(_config, environment) { if (environment.command !== "serve") throw new Error("Build the finite WGPU wasm target through Nx"); } },
    ],
  };
}
