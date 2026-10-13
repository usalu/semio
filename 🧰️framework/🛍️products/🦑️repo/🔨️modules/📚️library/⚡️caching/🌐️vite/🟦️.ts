import { buildModeEnvironment, readBuildMode, type BuildMode } from "../../../../../../🔨️modules/🏃️process/🏗️build/🎚️mode/🟦️.ts";
import { compilerStateDirectory, withCompilerState } from "../🏗️state/🟦️.ts";
import { repoCacheDirectory } from "../🟦️.ts";
import { readFileSync, existsSync } from "node:fs";
import { getWorkspaceRoot } from "../../🗂️workspaces/🟦️.ts";
import { mkdir, mkdtemp, rm, cp } from "node:fs/promises";
import { createServer as createHttpServer } from "node:http";
import { createRequire } from "node:module";
import type { Socket } from "node:net";
import { dirname, join } from "node:path";
import { stageRepositoryArtifacts } from "../📦️artifacts/🟦️.ts";
import { collectArtifactFiles } from "../../../../../../🔨️modules/🏃️process/📦️artifacts/🗂️files/🟦️.ts";
import { parseServiceSession, SERVICE_READY_ENDPOINT, type ServiceSession } from "./🧾️session/🟦️.ts";

/** 🗂️ Admits an owner- and intent-specific Vite cache, including toolchain and config identity. */
export function viteCacheDirectory(fallback: string, environment: Readonly<Record<string, string | undefined>> = process.env, config = fallback): string {
  if (environment.SEMIO_VITE_CACHE_DIRECTORY) return environment.SEMIO_VITE_CACHE_DIRECTORY;
  const require = createRequire(import.meta.url);
  return compilerStateDirectory({ root: fallback, owner: config, compiler: `vite:${require("vite/package.json").version}:bun:${process.versions.bun}`, configuration: config, mode: readBuildMode(environment.SEMIO_BUILD_MODE) });
}

export type ViteArtifactBuild = {
  readonly root: string;
  readonly workspace: string;
  readonly config: string;
  readonly output: string;
  readonly owner: string;
  readonly temporaryRoot?: string;
  readonly mode?: BuildMode;
  readonly signal?: AbortSignal;
  readonly environment?: Readonly<Record<string, string | undefined>>;
};

/** 🌐️ Bundles one site and publishes its complete deliverable only after Vite succeeds. */
export async function buildViteArtifact(options: ViteArtifactBuild): Promise<void> {
  options.signal?.throwIfAborted();
  const environment = { ...process.env, ...options.environment }, mode = options.mode ?? readBuildMode(environment.SEMIO_BUILD_MODE), require = createRequire(join(options.workspace, "package.json"));
  const state = { root: repoCacheDirectory(options.workspace), owner: options.owner, compiler: `vite:${require("vite/package.json").version}:bun:${process.versions.bun}`, configuration: JSON.stringify([options.config, readFileSync(options.config, "utf8")]), mode };
  const temporaryRoot = options.temporaryRoot ?? join(dirname(options.output), ".staging");
  await mkdir(temporaryRoot, { recursive: true });
  const temporary = await mkdtemp(join(temporaryRoot, "vite-build-"));
  try {
    await withCompilerState({ ...state, signal: options.signal ?? new AbortController().signal }, async directory => {
      const cli = join(dirname(require.resolve("vite/package.json")), "bin/vite.js");
      options.signal?.throwIfAborted();
      console.log(`Bundling ${options.owner}`);
      const child = Bun.spawn([process.execPath, cli, "build", "--mode", mode, "--minify", mode === "release" ? "esbuild" : "false", ...(mode === "prototype" ? ["--sourcemap"] : []), "--config", options.config, "--configLoader", "bundle", "--outDir", temporary, "--emptyOutDir"], { cwd: options.root, env: { ...buildModeEnvironment(mode, environment), SEMIO_VITE_CACHE_DIRECTORY: directory }, stdout: "inherit", stderr: "inherit", stdin: "ignore" });
      let force: ReturnType<typeof setTimeout> | undefined;
      const cancel = (): void => { child.kill("SIGTERM"); force ??= setTimeout(() => child.kill("SIGKILL"), 2000); };
      options.signal?.addEventListener("abort", cancel, { once: true });
      if (options.signal?.aborted) cancel();
      let status: number;
      try { status = await child.exited; }
      finally { options.signal?.removeEventListener("abort", cancel); clearTimeout(force); }
      options.signal?.throwIfAborted();
      if (status !== 0) throw new Error(`Vite build failed for ${options.owner} (${status})`);
      const files = await collectArtifactFiles(temporary, options.signal);
      options.signal?.throwIfAborted();
      await stageRepositoryArtifacts(options.output, options.owner, files, { signal: options.signal });
      console.log(`Published ${options.owner}: ${files.size} files at ${options.output}`);
    });
  } finally { await rm(temporary, { recursive: true, force: true }); }
}

export type ViteService = {
  readonly root: string;
  readonly config: string;
  readonly configLoader?: "bundle" | "runner" | "native";
  readonly host: string;
  readonly port: number;
  readonly signal: AbortSignal;
  readonly ready: (url: string) => void | Promise<void>;
  readonly session?: ServiceSession;
  readonly temporaryRoot?: string;
  readonly mode?: BuildMode;
};

/** 🖥️ Owns one Vite listener through readiness and cancellation without reusing ambient services. */
export async function serveVite(options: ViteService): Promise<void> {
  options.signal.throwIfAborted();
  if (!Number.isInteger(options.port) || options.port < 0 || options.port > 65535) throw new Error("Invalid Vite service port");
  const session = options.session ? parseServiceSession(options.session) : undefined;
  const { createServer } = await import("vite");
  const mode = readBuildMode(options.mode ?? "prototype"), workspace = getWorkspaceRoot(), require = createRequire(join(workspace, "package.json"));
  const state = { root: repoCacheDirectory(workspace), owner: `vite-service:${options.root}`, compiler: `vite:${require("vite/package.json").version}:bun:${process.versions.bun}`, configuration: JSON.stringify([options.config, readFileSync(options.config, "utf8")]), mode };
  const temporaryRoot = options.temporaryRoot ?? (process.env.SEMIO_TICKET_DIR ? join(process.env.SEMIO_TICKET_DIR, "🗑️generated") : join(state.root, "vite-services"));
  await mkdir(temporaryRoot, { recursive: true });
  const cache = await mkdtemp(join(temporaryRoot, "vite-service-"));
  try { await withCompilerState({ ...state, signal: options.signal }, async directory => { if (existsSync(join(directory, "snapshot"))) await cp(join(directory, "snapshot"), cache, { recursive: true }); await rm(join(cache, ".nx-artifact.json"), { force: true }); }); }
  catch (error) { await rm(cache, { recursive: true, force: true }); throw error; }
  const listener = createHttpServer(), sockets = new Set<Socket>();
  listener.on("connection", socket => { sockets.add(socket); socket.once("close", () => sockets.delete(socket)); });
  let server: Awaited<ReturnType<typeof createServer>> | undefined;
  try {
    server = await createServer({ mode, cacheDir: cache, root: options.root, configFile: options.config, configLoader: options.configLoader ?? "native", server: { host: options.host, port: options.port, strictPort: true, open: false, middlewareMode: { server: listener }, hmr: { server: listener } }, plugins: session ? [{
    name: "semio-service-readiness",
    configureServer(server) {
      server.middlewares.use((request, response, next) => {
        if (request.url !== SERVICE_READY_ENDPOINT || request.method !== "GET") return next();
        response.setHeader("content-type", "application/json");
        response.setHeader("cache-control", "no-store");
        response.end(JSON.stringify(session));
      });
    },
    }] : [] });
    options.signal.throwIfAborted();
    listener.on("request", (request, response) => server!.middlewares(request, response));
    await new Promise<void>((resolve, reject) => {
      listener.once("error", reject);
      listener.listen(options.port, options.host, () => { listener.removeListener("error", reject); resolve(); });
    });
    options.signal.throwIfAborted();
    const address = listener.address();
    if (!address || typeof address === "string") throw new Error("Vite did not bind a TCP listener");
    const host = options.host === "0.0.0.0" ? "127.0.0.1" : options.host.includes(":") ? `[${options.host}]` : options.host;
    await options.ready(`http://${host}:${address.port}/`);
    await new Promise<void>(resolve => {
      if (options.signal.aborted) resolve();
      else options.signal.addEventListener("abort", () => resolve(), { once: true });
    });
  } finally {
    const closed = new Promise<void>(resolve => listener.close(() => resolve()));
    for (const socket of sockets) socket.destroy();
    try {
      await Promise.all([closed, server?.close()]);
      if (server) await withCompilerState({ ...state, signal: AbortSignal.timeout(30_000) }, async directory => {
        const files = await collectArtifactFiles(cache);
        await stageRepositoryArtifacts(join(directory, "snapshot"), "vite-service:optimizer", files);
      });
    } finally { await rm(cache, { recursive: true, force: true }); }
  }
}
