// #region 🧲️Header

import { cpSync, existsSync, mkdirSync, readdirSync, readFileSync, rmSync, statSync } from "node:fs";
import { isAbsolute, relative, resolve } from "node:path";


import {
  type OwnedBuildConfig,
  type OwnedBuildMiddleware,
  type OwnedBuildPlugin,
} from "../../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🛠️build-tooling/🟦️.ts";
import { parseTileProxyAssetSpecV1 } from "../../../../🖼️assets/📦️packages/🟦️typescript/🗺️tile-proxy/🟦️.ts";
import type { AssetDeliveryDeclarationV1, AssetDeliveryProviderV1 } from "../../../../🖼️assets/📦️packages/🟦️typescript/🔍️resolver/🧭️dispatch/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import providerSchema from "./🧬️schema/🔣️.json" with { type: "json" };
import { contentTypeForStaticDirAsset, serveFileWithValidatorsV1 } from "./📤️file-serving/🟦️.ts";
import { createTileProxyMiddleware, tileProxyVitePlugin } from "./🛰️tile-proxy/🟦️.ts";
import { createMeshCollectionMiddleware, meshCollectionVitePlugin, type MeshCollectionAssetSpecV1} from "./🗺️mesh-collection/🟦️.ts";

// #endregion 🔌️Adapters

export { playgroundIframeEmbedHeadersPlugin } from "../../🌐️iframe/🟦️.ts";

/** 🥽️ Supplies one owner-authored mesh catalog to its build provider. */
export type { MeshCollectionAssetSpecV1 };

/** 🗂️ Supplies one owner-authored directory to its build provider. */
export type StaticDirectoryAssetSpecV1 = { readonly kind: "static-dir"; readonly route: string; readonly root: string };

function admitBuildAssetSpecV1<K extends keyof typeof providerSchema.definitions>(value: AssetDeliveryDeclarationV1, kind: K): K extends "MeshCollectionAssetSpecV1" ? MeshCollectionAssetSpecV1 : StaticDirectoryAssetSpecV1 {
  const failures = validateJsonSchemaSubset(providerSchema.definitions[kind], value);
  if (failures.length) throw Error("Invalid " + kind + ": " + failures.join("; "));
  return value as K extends "MeshCollectionAssetSpecV1" ? MeshCollectionAssetSpecV1 : StaticDirectoryAssetSpecV1;
}

//#region 🔖️ViteElementsAssets
/** @emoji 📦️ Relative-base Vite build defaults for playground static sites (iframe + subdomain safe). */
export function playgroundStaticSiteBuildOptions(overrides?: OwnedBuildConfig["build"]): NonNullable<OwnedBuildConfig["build"]> {
  return {
    target: "esnext",
    outDir: "dist",
    emptyOutDir: true,
    ...overrides,
  };
}

export { semioViteBuild } from "./🎚️mode/🟦️.ts";

//#endregion 🔖️TileProxyAssetPlugin

/** @emoji 🦀️ Vite `optimizeDeps.exclude` entries for wasm-bindgen flow modules (must not be prebundled). */
export const FLOW_WASM_MODULE_OPTIMIZE_DEPS_EXCLUDE = [
  "@semio-tech/flow-module-core",
  "@semio-tech/flow-module-math",
  "@semio-tech/flow-module-text",
  "@semio-tech/flow-module-logic",
  "@semio-tech/flow-module-dictionary",
  "@semio-tech/flow-module-list",
  "@semio-tech/flow-module-draw",
] as const;

/** @emoji 🧭️ Workspace Vite resolve preset: dedupe, fs.allow, optimizeDeps.exclude, scene-host aliases. */
export function createWorkspaceViteResolveConfig(repoRoot: string, extraAliases: ReadonlyArray<{ readonly find: string | RegExp; readonly replacement: string }> = []): Pick<OwnedBuildConfig, "resolve" | "server" | "optimizeDeps"> {
  return {
    resolve: {
      alias: [...extraAliases],
      dedupe: ["react", "react-dom", "three", "@react-three/fiber", "@react-three/drei"],
    },
    server: {
      fs: { allow: [repoRoot] },
    },
    optimizeDeps: {
      exclude: [...findWorkspacePackages(repoRoot), ...FLOW_WASM_MODULE_OPTIMIZE_DEPS_EXCLUDE],
    },
  };
}

/** @emoji 🗂️ Connect middleware: serve one `static-dir` spec's files at `{route}/…`. */
function createStaticDirMiddleware(repoRoot: string, spec: StaticDirectoryAssetSpecV1): OwnedBuildMiddleware {
  const fixtureRoot = resolve(repoRoot, spec.root);
  const route = spec.route.endsWith("/") ? spec.route : `${spec.route}/`;
  return (req, res, next) => {
    const rawUrl = req.url ?? "";
    const pathOnly = rawUrl.split(/[?#]/, 1)[0] ?? "";
    let decodedPath = pathOnly;
    try {
      decodedPath = decodeURIComponent(pathOnly);
    } catch {
      next();
      return;
    }
    if (!decodedPath.startsWith(route)) {
      next();
      return;
    }
    const rel = decodedPath.slice(route.length);
    const filePath = resolve(fixtureRoot, rel);
    const relToRoot = relative(fixtureRoot, filePath);
    if (relToRoot.startsWith("..") || isAbsolute(relToRoot)) {
      next();
      return;
    }
    if (!existsSync(filePath) || !statSync(filePath).isFile()) {
      res.statusCode = 404;
      res.end();
      return;
    }
    serveFileWithValidatorsV1(req, res, filePath, contentTypeForStaticDirAsset(filePath));
  };
}

/** @emoji 🖼️ Generic dev/build Vite plugin pair for one `static-dir` asset spec: serves and copies
 * `spec.root` at `spec.route` — replaces the previous `cadFixtureVitePlugin`/`infiniteFixtureVitePlugin`
 * pair (byte-identical serving logic, now route/root-driven instead of hardcoded per fixture tree). */
export function staticDirVitePlugin(repoRoot: string, spec: StaticDirectoryAssetSpecV1): OwnedBuildPlugin[] {
  const serveFixture = createStaticDirMiddleware(repoRoot, spec);
  const fixtureRoot = resolve(repoRoot, spec.root);
  const destName = spec.route.replace(/^\//, "");
  let outDir = resolve(process.cwd(), "dist");
  let writeOutput = true;
  return [
    {
      name: `static-dir-serve${spec.route}`,
      enforce: "pre",
      configureServer(server) {
        server.middlewares.use(serveFixture);
      },
      configurePreviewServer(server) {
        server.middlewares.use(serveFixture);
      },
    },
    {
      name: `static-dir-build${spec.route}`,
      apply: "build",
      enforce: "pre",
      configResolved(config) {
        outDir = resolve(config.root, config.build.outDir);
        writeOutput = config.build.write !== false;
      },
      closeBundle() {
        if (!writeOutput) return;
        if (!existsSync(fixtureRoot)) {
          return;
        }
        const dest = resolve(outDir, destName);
        mkdirSync(outDir, { recursive: true });
        if (existsSync(dest)) rmSync(dest, { recursive: true, force: true });
        cpSync(fixtureRoot, dest, { recursive: true });
      },
    },
  ];
}

/** @emoji 🪜️ Serves several `static-dir` roots as ONE route table. Each mount answers 404 for a file its
 * root lacks (never the SPA fallback), so two roots on one route shadow each other — the second becomes
 * unreachable — and a parent route registered before a nested one swallows the nested one's requests.
 * The table therefore refuses a route claimed twice and orders the halves by nesting: serving
 * most-specific route first, build copies parent first (a parent copy replaces its destination).
 * Fixture law: `🧪️tests/🧫️fixtures/🗂️static-dir-mounts/🔣️.json`. */
export function staticDirMountVitePlugins(repoRoot: string, specs: readonly StaticDirectoryAssetSpecV1[]): OwnedBuildPlugin[] {
  const routes = specs.map((spec) => spec.route.replace(/\/+$/, ""));
  const claimed = routes.find((route, index) => routes.indexOf(route) !== index);
  if (claimed !== undefined) throw new Error(`Two static-dir roots claim one route: ${claimed}`);
  const depth = (index: number) => routes[index]!.split("/").length;
  const halves = specs.map((spec, index) => ({ index, plugins: staticDirVitePlugin(repoRoot, spec) }));
  return [
    ...[...halves].sort((a, b) => depth(b.index) - depth(a.index)).flatMap((half) => half.plugins.filter((plugin) => plugin.apply !== "build")),
    ...[...halves].sort((a, b) => depth(a.index) - depth(b.index)).flatMap((half) => half.plugins.filter((plugin) => plugin.apply === "build")),
  ];
}


/** @emoji 🚫️ Directory names [[findWorkspacePackages]] never descends into: installed dependencies and
 * build output, none of which may contribute a workspace package name. */
const WORKSPACE_PACKAGE_SCAN_SKIP: ReadonlySet<string> = new Set(["node_modules", "dist", "target", "storybook-static", "🗑️generated", "🤖️generated"]);

/** @emoji 🧠️ Memo for [[findWorkspacePackages]], keyed by repo root.
 *
 * 🩸️ The scan costs 14–27 s on this tree and every vite config that calls
 * [[createWorkspaceViteResolveConfig]] runs it at module load, so a process loading several configs paid
 * it several times over. The workspace set cannot change under a running dev server — its vite config is
 * evaluated once at boot — so one walk per process is the whole truth. The declared `workspaces` array in
 * the root `package.json` is NOT a substitute: it is missing `@semio-tech/framework-graph-layout-run-rs`,
 * `@semio-tech/framework-tool-run-rs` and `@semio-tech/print-viz-inference`, which must stay out of
 * `optimizeDeps`. */
const workspacePackagesByRoot = new Map<string, string[]>();

/** @emoji 📦️ Every `@semio-tech/*` package name in the workspace tree, for `optimizeDeps.exclude`.
 *
 * 🩸️ Directory entries are classified from `readdirSync`'s own `Dirent`, which never follows a link.
 * The previous `statSync(full).isDirectory()` did follow, and a hub test leaves a **self-referential**
 * link behind (`🌎️hub/📦️packages/🦀️rust/🗑️generated/test-artifacts/linked-ancestor-publication-owner-*
 * → …/test-artifacts`), so the walk recursed into the same directory forever. Every caller of
 * [[createWorkspaceViteResolveConfig]] evaluates this at config-module load, so a single leftover link
 * hung the `📐️cad` and `🌍️world/🎨️r3f` vitest owners — and any dev server sharing that config — with
 * no error and no timeout. Build output carries no workspace package and is skipped outright. */
export function findWorkspacePackages(repoRoot: string): string[] {
  const memoized = workspacePackagesByRoot.get(repoRoot);
  if (memoized !== undefined) return memoized;
  const packages: string[] = [];
  const scan = (dir: string) => {
    let entries;
    try {
      entries = readdirSync(dir, { withFileTypes: true });
    } catch {
      return;
    }
    for (const entry of entries) {
      if (entry.isSymbolicLink()) continue;
      if (WORKSPACE_PACKAGE_SCAN_SKIP.has(entry.name) || entry.name.startsWith(".")) continue;
      const full = resolve(dir, entry.name);
      if (entry.isDirectory()) {
        scan(full);
        continue;
      }
      if (entry.name !== "package.json" || full === resolve(repoRoot, "package.json")) continue;
      try {
        const pkg = JSON.parse(readFileSync(full, "utf8"));
        if (pkg.name && typeof pkg.name === "string" && pkg.name.startsWith("@semio-tech/")) packages.push(pkg.name);
      } catch {}
    }
  };
  scan(repoRoot);
  workspacePackagesByRoot.set(repoRoot, packages);
  return packages;
}

/** 🗺️ Contributes tile transport only when the caller explicitly registers it. */
export const TILE_PROXY_ASSET_PROVIDER_V1: AssetDeliveryProviderV1 = Object.freeze<AssetDeliveryProviderV1>({
  kind: "tile-proxy",
  middleware: (context, declaration) => {
    const spec = parseTileProxyAssetSpecV1(declaration);
    return createTileProxyMiddleware(spec.route, resolve(context.root, spec.cache), spec.upstream, context.mode, spec.userAgent);
  },
  plugins: (context, declaration) => tileProxyVitePlugin(context.root, parseTileProxyAssetSpecV1(declaration), context.mode),
});

/** 🥽️ Contributes catalog transport only when the caller explicitly registers it. */
export const MESH_COLLECTION_ASSET_PROVIDER_V1: AssetDeliveryProviderV1 = Object.freeze<AssetDeliveryProviderV1>({
  kind: "mesh-collection",
  middleware: (context, declaration) => createMeshCollectionMiddleware(context.root, admitBuildAssetSpecV1(declaration, "MeshCollectionAssetSpecV1")),
  plugins: (context, declaration) => meshCollectionVitePlugin(context.root, admitBuildAssetSpecV1(declaration, "MeshCollectionAssetSpecV1")),
});

/** 🗂️ Contributes directory transport only when the caller explicitly registers it. */
export const STATIC_DIRECTORY_ASSET_PROVIDER_V1: AssetDeliveryProviderV1 = Object.freeze<AssetDeliveryProviderV1>({
  kind: "static-dir",
  middleware: (context, declaration) => createStaticDirMiddleware(context.root, admitBuildAssetSpecV1(declaration, "StaticDirectoryAssetSpecV1")),
  plugins: (context, declaration) => staticDirVitePlugin(context.root, admitBuildAssetSpecV1(declaration, "StaticDirectoryAssetSpecV1")),
});

/** @emoji 🧦️ The browser socket of one proxied upgrade. */
type ProxiedBrowserSocket = {
  destroyed: boolean;
  on(event: "close" | "end" | "error", listener: () => void): void;
  unpipe?(destination?: unknown): void;
};

/** @emoji 🧦️ The socket the proxy opened to the upstream. */
type ProxiedUpstreamSocket = {
  destroyed: boolean;
  destroy(): void;
  unpipe?(destination?: unknown): void;
};

/** @emoji 🔌️ The part of a dev proxy that sees a websocket upgrade start and the upstream it opened. */
export type ProxiedUpgradeProxy = {
  on(event: "proxyReqWs", listener: (proxyReq: { destroy(): void }, request: unknown, socket: ProxiedBrowserSocket) => void): void;
  on(event: "open", listener: (proxySocket: ProxiedUpstreamSocket) => void): void;
  on(event: "error", listener: (error: unknown, request: unknown, connection: unknown) => void): void;
};

/** 🕊️ Destroys the upstream of a proxied websocket when the browser socket ends or closes. The pipe keeps the browser
 * socket's writable open, so a destroyed page emits `end` and not `close` while the upstream still writes, and a presence
 * room would keep that learner until idle. Upgrades are paired in the order they were asked for. One that fails before
 * it opens is taken off the queue, and its request is destroyed if the browser is already gone.
 * @see https://github.com/http-party/node-http-proxy — `proxySocket.pipe(socket).pipe(proxySocket)` */
export function closeProxiedSocketWithBrowser(proxy: ProxiedUpgradeProxy): void {
  const pending: ProxiedBrowserSocket[] = [];
  const upgraded = new WeakSet<ProxiedBrowserSocket>();
  proxy.on("proxyReqWs", (proxyReq, _request, socket) => {
    pending.push(socket);
    const abandon = (): void => {
      if (!upgraded.has(socket)) proxyReq.destroy();
    };
    socket.on("end", abandon);
    socket.on("close", abandon);
  });
  proxy.on("open", (proxySocket) => {
    const socket = pending.shift();
    if (socket === undefined) return;
    upgraded.add(socket);
    const drop = (): void => {
      socket.unpipe?.(proxySocket);
      proxySocket.unpipe?.(socket);
      if (!proxySocket.destroyed) proxySocket.destroy();
    };
    if (socket.destroyed) drop();
    else {
      socket.on("end", drop);
      socket.on("close", drop);
    }
  });
  proxy.on("error", (_error, _request, connection) => {
    const index = pending.indexOf(connection as ProxiedBrowserSocket);
    if (index >= 0) pending.splice(index, 1);
  });
}
