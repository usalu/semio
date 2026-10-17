/** 🔖️ The source freshness Vite plugins of the dev server: the unwatched segments, the react refresh coherence plugin, the freshness registry and the source watch and transform freshness plugins. */
import { readdirSync, realpathSync, statSync, watch } from "node:fs";
import { dirname, isAbsolute, join } from "node:path";

//#region 🔖️SourceFreshnessVitePlugins
/** 🚫️ Repository directory names no dev server may ever watch: version control metadata, the Nx
 * workspace store, the package store, the shared build/cache root, compiled output and generated
 * sources. Tools rewrite millions of files inside them while a dev session is open, and every such write
 * would otherwise be delivered into the dev server's event loop.
 *
 * `🤖️generated` and `.vscode` stay listed for the reason Vite's own `server.watch.ignored` once carried:
 * those files are config dependencies, so reacting to a registry or editor-settings rewrite restarts the
 * server in a loop.
 * @see https://github.com/paulmillr/chokidar/blob/3.6.0/lib/fsevents-handler.js */
export const UNWATCHED_REPOSITORY_SEGMENTS: readonly string[] = [".git", ".nx", ".vscode", ".🧬semio", "node_modules", "dist", "target", "🤖️generated", "🗑️generated"];

/** 👁️ The repository's top-level source directories — every place a dev server's module graph can
 * legitimately import from, with the unwatchable stores above removed. Read from disk rather than
 * hardcoded so a new top-level product directory is watched without touching this module. */
export function repositorySourceWatchRoots(repoRoot: string): readonly string[] {
  const excluded = new Set(UNWATCHED_REPOSITORY_SEGMENTS);
  return readdirSync(repoRoot, { withFileTypes: true }).filter((entry) => entry.isDirectory() && !excluded.has(entry.name)).map((entry) => join(repoRoot, entry.name)).sort();
}

/** 🧹️ Matches any relative path that crosses an unwatched store, on both `/` and `\` separators.
 * One precompiled test per filesystem event is the whole per-event budget this watcher may spend. */
export function unwatchedRepositoryPathMatcher(): RegExp {
  const alternatives = UNWATCHED_REPOSITORY_SEGMENTS.map((segment) => segment.replaceAll(".", "\\.")).join("|");
  return new RegExp(`(?:^|[\\\\/])(?:${alternatives})(?:[\\\\/]|$)`, "u");
}

/** 🛰️ Drives Vite's file-change pipeline from `node:fs` recursive watches over the repository's
 * source roots, so `⚙️vite.config.ts` can hand Vite `server.watch: null` and run no chokidar watcher of
 * its own.
 *
 * Vite watches its `root` plus — through `ensureWatchedFile` — every module-graph file outside it, which
 * in this repository is roughly a thousand individual paths spread across several top-level directories.
 * On macOS chokidar answers that by consolidating sibling FSEvents streams upward until ONE stream covers
 * the whole repository, then runs every watched path's prefix filter against every event that stream
 * delivers. A concurrent `cargo` build writing into the shared cache therefore costs the dev server
 * `events × watched paths` string comparisons — measured at 6 291 events per 2 s against 1 316 watched
 * paths, which blocks the event loop for ~2 s at a time, allocates ~600 MB per burst and, once resident
 * memory reaches the runtime's ceiling, wedges the server permanently. `server.watch.ignored` cannot undo
 * this: chokidar consults it only after those prefix filters have already run.
 *
 * Watching the source roots directly keeps the kernel from ever reporting cache, package-store or
 * generated-output writes, and reduces the per-event cost to one regular-expression test. Events are
 * replayed on Vite's own (no-op) watcher emitter, so module invalidation, HMR boundary computation and
 * config-dependency restarts behave exactly as they did with chokidar.
 *
 * 🛰️ macOS reports every write to an existing file — in place and atomic (temp + rename) alike — as `eventType: "rename"`,
 * while Vite invalidates a transformed module only from its `change` handler (`moduleGraph.onFileChange`). A file the module
 * graph already holds (tracked by the freshness registry) is therefore replayed as `change` ONLY: Vite's `add` handler runs
 * `handleHMRUpdate("create")`, which collects the file's modules too, so `add` + `change` sent every edit to the page as TWO
 * hot updates (measured, ticket 26/09/23 F2). A file the graph does not hold is replayed as `add` + `change` (a new inode
 * appeared; nothing imports it yet, so the `change` finds no module). Emitting only `add` served the pre-edit transform for
 * the life of the server, and `SEMIO_VITE_HMR=0` (`hmr: false`) removes the HMR pass that would otherwise have hidden it
 * (`📓️2026-09-13-wave-B53-nakagin-export-full-run.md` §4.2). The watched roots are the repository's REAL paths: Vite keys
 * its module graph by resolved (symlink-free) ids, so a repository reached through a symlink (`/tmp` → `/private/tmp`, a
 * devcontainer mount) otherwise named files no module matched and no edit ever reached the page (measured, same ticket). */
const REACT_REFRESH_RUNTIME = "/@react-refresh";

/** ⚛️ Preamble copied from `@vitejs/plugin-react` — semio-host-html replaces the whole document in
 * `transformIndexHtml` `order: "pre"`, so the react plugin's own preamble injection must be reinforced in
 * `order: "post"` or `@react-three/fiber` (and every other JSX dep) throws "can't detect preamble". */
function semioReactRefreshPreambleScript(base: string): string {
  const root = base.endsWith("/") ? base.slice(0, -1) : base;
  return `import { injectIntoGlobalHook } from "${root}${REACT_REFRESH_RUNTIME}";
injectIntoGlobalHook(window);
window.$RefreshReg$ = () => {};
window.$RefreshSig$ = () => (type) => type;`;
}

/** ⚛️ Aligns Vite 7 / Rolldown OXC JSX refresh with `server.hmr` — `SEMIO_VITE_HMR=0` must not emit
 * `$RefreshReg$` wrappers without the HTML preamble, and HMR-on serves must always ship that preamble even
 * after {@link semioHostHtmlVitePlugin} rebuilds `index.html`. */
export function semioPlaygroundReactRefreshCoherenceVitePlugin() {
  return {
    name: "semio-playground-react-refresh-coherence",
    enforce: "post" as const,
    config(userConfig: { readonly server?: { readonly hmr?: unknown } }, { command }: { readonly command: string }) {
      if (command !== "serve" || userConfig.server?.hmr !== false) return;
      return {
        esbuild: { jsxDev: false },
        oxc: { jsx: { refresh: false } },
        optimizeDeps: { esbuildOptions: { jsxDev: false } },
      };
    },
    transformIndexHtml: {
      order: "post" as const,
      handler(html: string, ctx: { readonly server?: { readonly config: { readonly base?: string; readonly server: { readonly hmr?: unknown } } } }) {
        if (ctx.server?.config.server.hmr === false) return;
        if (html.includes("injectIntoGlobalHook")) return;
        const base = ctx.server?.config.base ?? "/";
        return [{ tag: "script", attrs: { type: "module" }, children: semioReactRefreshPreambleScript(base) }];
      },
    },
  };
}

/** 🧾️ The `{mtimeMs, size}` pair a transformed module's file carried when the dev server last read
 * it. Two facts rather than one: a same-second rewrite of a different length moves `size` while `mtimeMs`
 * can still round to the same millisecond on some filesystems. */
export type SourceStamp = { readonly mtimeMs: number; readonly size: number };

/** 🔍️ Remembers what every transformed module's file looked like on disk when its transform was
 * produced, and answers which of them have moved since.
 *
 * ONLY files the dev server has actually transformed are tracked, so "moved" is exactly "the cached
 * transform is out of date": a file the server never read has no cached transform to be stale. The
 * per-directory index makes the answer to "did anything in THIS directory change" cost one stat per
 * tracked sibling, which is what turns a filesystem event naming an editor's temporary file into the
 * invalidation of the module that temporary file was renamed onto. */
export function createSourceFreshnessRegistry() {
  const stamps = new Map<string, SourceStamp>();
  const siblings = new Map<string, Set<string>>();
  const readStamp = (file: string): SourceStamp | null => {
    try {
      const status = statSync(file);
      return { mtimeMs: status.mtimeMs, size: status.size };
    } catch {
      return null;
    }
  };
  const moved = (file: string): boolean => {
    const previous = stamps.get(file);
    if (previous === undefined) return false;
    const current = readStamp(file);
    if (current === null) return false;
    if (current.mtimeMs === previous.mtimeMs && current.size === previous.size) return false;
    stamps.set(file, current);
    return true;
  };
  return {
    record(file: string): void {
      const stamp = readStamp(file);
      if (stamp === null) return;
      const directory = dirname(file);
      const tracked = siblings.get(directory) ?? new Set<string>();
      tracked.add(file);
      siblings.set(directory, tracked);
      stamps.set(file, stamp);
    },
    trackedCount: (): number => stamps.size,
    tracks: (file: string): boolean => stamps.has(file),
    movedFile: (file: string): boolean => moved(file),
    movedInDirectory: (directory: string): readonly string[] => [...(siblings.get(directory) ?? [])].filter(moved),
    movedEverywhere: (): readonly string[] => [...stamps.keys()].filter(moved),
  };
}

export type SourceFreshnessRegistry = ReturnType<typeof createSourceFreshnessRegistry>;

type FreshnessServer = {
  readonly watcher: { emit(event: string, path: string): boolean };
  readonly httpServer: { once(event: "close", listener: () => void): void } | null;
  readonly environments?: Record<string, { readonly moduleGraph: { onFileChange(file: string): void } }>;
  readonly middlewares?: { use(handler: (request: { url?: string; headers: Record<string, string | string[] | undefined> }, response: unknown, next: () => void) => void): void };
  readonly config?: { readonly root: string; readonly server: { readonly hmr?: unknown } };
};

/** ♻️ Retires every cached transform of one file, synchronously for the request in flight and then
 * through Vite's own file-change pipeline for everything downstream of it (plugin `watchChange`, HMR
 * boundaries, config-dependency restarts). `onFileChange` walks importers, so an importer that inlined
 * the edited module's output is retired with it. */
function retireStaleModule(server: FreshnessServer, file: string): void {
  for (const environment of Object.values(server.environments ?? {})) environment.moduleGraph.onFileChange(file);
  server.watcher.emit("change", file);
}

export function semioSourceWatchVitePlugin(options: { readonly repoRoot: string; readonly freshness?: Pick<SourceFreshnessRegistry, "movedInDirectory" | "tracks"> }) {
  return {
    name: "semio-source-watch",
    apply: "serve" as const,
    configureServer(server: FreshnessServer) {
      const unwatched = unwatchedRepositoryPathMatcher();
      const freshness = options.freshness;
      const handles = repositorySourceWatchRoots(realpathSync(options.repoRoot)).map((root) => watch(root, { recursive: true, persistent: false }, (eventType, name) => {
        if (name === null || unwatched.test(name)) return;
        const path = join(root, name);
        const directory = dirname(path);
        if (eventType !== "rename") {
          server.watcher.emit("change", path);
          for (const moved of freshness?.movedInDirectory(directory) ?? []) if (moved !== path) server.watcher.emit("change", moved);
          return;
        }
        const entry = statSync(path, { throwIfNoEntry: false });
        if (!entry) {
          server.watcher.emit("unlink", path);
          for (const moved of freshness?.movedInDirectory(directory) ?? []) server.watcher.emit("change", moved);
          return;
        }
        if (entry.isDirectory()) {
          server.watcher.emit("addDir", path);
          for (const moved of freshness?.movedInDirectory(path) ?? []) server.watcher.emit("change", moved);
          return;
        }
        if (freshness?.tracks(path) !== true) server.watcher.emit("add", path);
        server.watcher.emit("change", path);
        for (const moved of freshness?.movedInDirectory(directory) ?? []) if (moved !== path) server.watcher.emit("change", moved);
      }));
      server.httpServer?.once("close", () => {
        for (const handle of handles) handle.close();
      });
    },
  };
}

/** 🗺️ The absolute file a dev-server request would be transformed from, or `null` for a request no
 * module graph entry can back (virtual ids, client runtime, the index document). `/@fs/` carries the
 * absolute path the module graph is keyed by; everything else is relative to Vite's `root`. */
export function requestedTransformFile(url: string, root: string): string | null {
  const pathname = url.split("?",1)[0]??"";
  if (pathname === "" || pathname === "/" || pathname.endsWith("/")) return null;
  let decoded: string;
  try {
    decoded = decodeURIComponent(pathname);
  } catch {
    return null;
  }
  if (decoded.startsWith("/@fs/")) return decoded.slice("/@fs".length);
  if (decoded.startsWith("/@") || decoded.startsWith("/\0") || decoded.includes("\0")) return null;
  return join(root, decoded);
}

/** 🛡️ Proves, at request time, that every transform this dev server is about to serve was produced
 * from the bytes currently on disk — the guarantee the filesystem watcher alone cannot give.
 *
 * macOS reports a recursive `fs.watch` event by the path whose directory entry changed, and an atomic
 * save (write a temporary file, `rename` it onto the target) changes the entry of the TEMPORARY file. The
 * edited module's own path is then never named by any event — measured 0 times in 5 at every module depth,
 * for `sed -i ''`, for a rename-into-place and for every editor that saves atomically, which is all of
 * them. `SEMIO_VITE_HMR=0` removes the HMR pass that would otherwise have papered over it, so the dev
 * server keeps serving the pre-edit transform until the process is recycled: a developer moves a slider
 * and the preview runs yesterday's module.
 *
 * {@link semioSourceWatchVitePlugin} now answers such an event by re-stating its whole directory, which
 * repairs the common case at edit time. This plugin is the guarantee underneath it, and it does not
 * depend on any event arriving at all: every module request re-stats the one file behind it, and every
 * document request re-stats the whole transformed set, so the very next request after any edit — by any
 * tool, through any write style, with the watcher armed or not — serves the current file.
 *
 * @see 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧹️config/🟦️.ts */
export function semioTransformFreshnessVitePlugin(options: { readonly freshness: SourceFreshnessRegistry }) {
  let documentSweep = { verified: 0, retired: 0 };
  return {
    name: "semio-transform-freshness",
    apply: "serve" as const,
    enforce: "pre" as const,
    transform(_code: string, id: string) {
      const file = id.split("?",1)[0]??"";
      if (isAbsolute(file)) options.freshness.record(file);
      return null;
    },
    configureServer(server: FreshnessServer) {
      const configured = server.config?.root ?? "";
      const root = configured === "" ? "" : realpathSync(configured);
      server.middlewares?.use((request, _response, next) => {
        const url = request.url ?? "";
        const accept = request.headers.accept;
        if (typeof accept === "string" && accept.includes("text/html")) {
          const stale = options.freshness.movedEverywhere();
          for (const file of stale) retireStaleModule(server, file);
          documentSweep = { verified: options.freshness.trackedCount(), retired: stale.length };
          next();
          return;
        }
        const file = requestedTransformFile(url, root);
        if (file !== null && options.freshness.movedFile(file)) retireStaleModule(server, file);
        next();
      });
    },
    transformIndexHtml: {
      order: "post" as const,
      handler(_html: string, context: { server?: { config: { server: { hmr: unknown } } } }) {
        const hmr = context.server?.config.server.hmr === false ? "off" : "on";
        const banner = `semio dev · transform freshness: stat-guard (every module request + whole graph per document) · hmr ${hmr} · ${documentSweep.verified} modules verified, ${documentSweep.retired} stale transforms retired · serve pid ${process.pid} · document ${new Date().toISOString()}`;
        return [{ tag: "script", attrs: { type: "module" }, children: `console.info(${JSON.stringify(banner)});` }];
      },
    },
  };
}

/** 🛰️ The dev server's complete "never serve a stale module" contract: the source watcher that
 * pushes edits into Vite's module graph, and the request-time stat guard that verifies what the watcher
 * delivered. They share one {@link createSourceFreshnessRegistry}, so the watcher can resolve a
 * temporary-file event into the module it was renamed onto. Mount both or neither. */
export function semioSourceFreshnessVitePlugins(options: { readonly repoRoot: string }) {
  const freshness = createSourceFreshnessRegistry();
  return [semioTransformFreshnessVitePlugin({ freshness }), semioSourceWatchVitePlugin({ repoRoot: options.repoRoot, freshness })];
}
//#endregion 🔖️SourceFreshnessVitePlugins
