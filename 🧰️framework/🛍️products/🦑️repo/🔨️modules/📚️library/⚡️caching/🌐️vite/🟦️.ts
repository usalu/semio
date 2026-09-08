import { mkdir, mkdtemp, rm } from "node:fs/promises";
import { createServer as createHttpServer } from "node:http";
import { createRequire } from "node:module";
import type { Socket } from "node:net";
import { dirname, join } from "node:path";
import { stageArtifacts } from "../📦️artifacts/🟦️.ts";
import { collectArtifactFiles } from "../📦️artifacts/🗂️files/🟦️.ts";
import { parseServiceSession, SERVICE_READY_ENDPOINT, type ServiceSession } from "./🧾️session/🟦️.ts";

export type ViteArtifactBuild = {
  readonly root: string;
  readonly workspace: string;
  readonly config: string;
  readonly output: string;
  readonly owner: string;
  readonly temporaryRoot?: string;
  readonly signal?: AbortSignal;
  readonly environment?: Readonly<Record<string, string | undefined>>;
};

/** 🌐️ Bundles one site and publishes its complete deliverable only after Vite succeeds. */
export async function buildViteArtifact(options: ViteArtifactBuild): Promise<void> {
  options.signal?.throwIfAborted();
  const temporaryRoot = options.temporaryRoot ?? join(dirname(options.output), ".staging");
  await mkdir(temporaryRoot, { recursive: true });
  const temporary = await mkdtemp(join(temporaryRoot, "vite-build-"));
  try {
    const cli = join(dirname(createRequire(join(options.workspace, "package.json")).resolve("vite/package.json")), "bin/vite.js");
    options.signal?.throwIfAborted();
    console.log(`Bundling ${options.owner}`);
    const child = Bun.spawn([process.execPath, cli, "build", "--config", options.config, "--configLoader", "bundle", "--outDir", temporary, "--emptyOutDir"], { cwd: options.root, env: { ...process.env, ...options.environment }, stdout: "inherit", stderr: "inherit", stdin: "ignore" });
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
    stageArtifacts(options.output, options.owner, files);
    console.log(`Published ${options.owner}: ${files.size} files at ${options.output}`);
  } finally { await rm(temporary, { recursive: true, force: true }); }
}

export type ViteService = {
  readonly root: string;
  readonly config: string;
  readonly host: string;
  readonly port: number;
  readonly signal: AbortSignal;
  readonly ready: (url: string) => void;
  readonly session?: ServiceSession;
};

/** 🖥️ Owns one Vite listener through readiness and cancellation without reusing ambient services. */
export async function serveVite(options: ViteService): Promise<void> {
  options.signal.throwIfAborted();
  if (!Number.isInteger(options.port) || options.port < 0 || options.port > 65535) throw new Error("Invalid Vite service port");
  const session = options.session ? parseServiceSession(options.session) : undefined;
  const { createServer } = await import("vite");
  const listener = createHttpServer(), sockets = new Set<Socket>();
  listener.on("connection", socket => { sockets.add(socket); socket.once("close", () => sockets.delete(socket)); });
  let server: Awaited<ReturnType<typeof createServer>> | undefined;
  try {
    server = await createServer({ root: options.root, configFile: options.config, configLoader: "bundle", server: { host: options.host, port: options.port, strictPort: true, open: false, middlewareMode: { server: listener }, hmr: { server: listener } }, plugins: session ? [{
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
    options.ready(`http://${host}:${address.port}/`);
    await new Promise<void>(resolve => {
      if (options.signal.aborted) resolve();
      else options.signal.addEventListener("abort", () => resolve(), { once: true });
    });
  } finally {
    const closed = new Promise<void>(resolve => listener.close(() => resolve()));
    for (const socket of sockets) socket.destroy();
    await Promise.all([closed, server?.close()]);
  }
}
