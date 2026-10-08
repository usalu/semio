// #region Header
/**
 * 🌎️ `os-hub-ts` — a Bun integration-test harness for the real `os-hub` binary (ticket
 * 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS, lane 3-E). Boots the compiled
 * binary on a scanned free port against a temp `OS_HUB_DATA`, polls a real endpoint for
 * readiness, and hands back a handle `🤝️index.test.ts` drives with two independent
 * `@semio-tech/framework-os` `DirectoryClient`s plus raw document-WS wire frames. Never `cargo
 * run`s (a wrapper process would complicate teardown) — spawns the prebuilt debug binary
 * directly, so `stop()` is a plain signal to one process, no process tree to chase.
 */
// #endregion Header

import Ajv from "ajv";
import { type ChildProcessByStdio, spawn } from "node:child_process";
import { cpSync, existsSync, mkdirSync, readFileSync, readdirSync } from "node:fs";
import { createServer } from "node:net";
import { join } from "node:path";
import type { Readable } from "node:stream";
import { randomBytes } from "node:crypto";
import { cargoTargetDirectory } from "../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🟦️.ts";
import { decodeServerFrame, encodeClientFrame } from "../../🧰️framework/🔨️modules/📡️replication/🟦️.ts";
import { parseDocumentSocketGrantReceiptV1 } from "../../🧰️framework/🛍️products/💻️os/🟦️.ts";
import { createSpaceCommandV1 } from "../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🏘️spaces/🟦️.ts";
import { directoryCommandRequestJson, sealDirectoryCommandRequestV1 } from "../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts";
import { sealSpaceArtifactCreateV1 } from "../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🟦️.ts";
import { isDiscoverySkipDirectory } from "../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";

export { getWorkspaceRoot } from "../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

//#region 🧬️Scope-owned schema resolution

/** 📚️ The generated scope catalog: the only authority for scope id → module file and dependencies. */
const SCHEMA_CATALOG_PATH = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json";

type SchemaCatalogScope = { readonly path: string; readonly formats: Readonly<Record<string, string>>; readonly dependsOn: readonly string[] };

let schemaCatalogScopes: Readonly<Record<string, SchemaCatalogScope>> | undefined;

/** 🌎️ Every `🧬️schema` module directory the hub partition owns, as the catalog must spell it. */
function hubSchemaModuleDirectories(repoRoot: string): readonly string[] {
  const found: string[] = [];
  const pending = ["🌎️hub"];
  while (pending.length > 0) {
    const relative = pending.pop()!;
    for (const entry of readdirSync(join(repoRoot, relative), { withFileTypes: true })) {
      if (!entry.isDirectory() || isDiscoverySkipDirectory(entry.name)) continue;
      if (entry.name === "🧬️schema" && existsSync(join(repoRoot, relative, entry.name, "🔣️.json"))) found.push(`${relative}/${entry.name}`);
      else pending.push(`${relative}/${entry.name}`);
    }
  }
  return found.sort();
}

/** 📇️ Reads the generated catalog once and proves every hub module is registered in it. */
function schemaCatalog(repoRoot: string): Readonly<Record<string, SchemaCatalogScope>> {
  if (schemaCatalogScopes) return schemaCatalogScopes;
  const scopes = (JSON.parse(readFileSync(join(repoRoot, SCHEMA_CATALOG_PATH), "utf8")) as { readonly scopes: Record<string, SchemaCatalogScope> }).scopes;
  const catalogued = new Set(Object.values(scopes).map((scope) => scope.path));
  for (const module of hubSchemaModuleDirectories(repoRoot)) {
    if (!catalogued.has(module)) throw new Error(`hub schema module ${module} is absent from ${SCHEMA_CATALOG_PATH}; regenerate it with \`bun ./📜️script.ts schema generate\``);
  }
  schemaCatalogScopes = scopes;
  return scopes;
}

/** 🗂️ Module file of one scope id, taken from the catalog with no nearest-parent or glob search. */
function schemaModulePath(repoRoot: string, scope: string): string {
  const entry = schemaCatalog(repoRoot)[scope];
  if (entry === undefined) throw new Error(`scope ${scope} is absent from ${SCHEMA_CATALOG_PATH}`);
  const file = entry.formats["🔣️jsonschema"];
  if (file === undefined) throw new Error(`scope ${scope} provides no JSON Schema format`);
  return `${entry.path}/${file}`;
}

// 🏷️Execution contract §B: an export that is only ever validated declares the formats it truly
// exists in. It is an annotation, never a constraint, so the strict validator is taught the keyword
// rather than being loosened — an unknown keyword must still be an error everywhere else.
const schemaAjv = new Ajv({ strict: true, allErrors: true }).addKeyword({ keyword: "x-semio-formats", metaSchema: { type: "array", minItems: 1, items: { type: "string" } } });
const schemaModuleIds = new Map<string, string>();

const SCHEMA_ID_PREFIX = "https://json.schemas.assets.semio-tech.com/";

/** 🧬️ Scope id of a document `$id`, per the `<scope path>/<facet>.json` grammar of the contract. */
function schemaScopeOfId(id: string): string {
  const segments = id.slice(SCHEMA_ID_PREFIX.length).split("/");
  return segments.slice(0, -1).join(".");
}

/** 🔎️ Every foreign scope one module `$ref`s, so a cross-scope reference names its own dependency. */
function schemaReferencedScopes(document: unknown, own: string): readonly string[] {
  const scopes = new Set<string>();
  const walk = (node: unknown): void => {
    if (Array.isArray(node)) return node.forEach(walk);
    if (node === null || typeof node !== "object") return;
    for (const [key, value] of Object.entries(node as Record<string, unknown>)) {
      if (key === "$ref" && typeof value === "string" && value.startsWith(SCHEMA_ID_PREFIX)) {
        const scope = schemaScopeOfId(value.split("#", 1)[0]!);
        if (scope !== own) scopes.add(scope);
      } else walk(value);
    }
  };
  walk(document);
  return [...scopes].sort();
}

/** 🔗️ Loads one scope module and every scope it depends on into the one shared draft-07 validator,
 * so a `$ref` from one scope into another resolves instead of being restated. */
function schemaModuleId(repoRoot: string, scope: string): string {
  const cached = schemaModuleIds.get(scope);
  if (cached !== undefined) return cached;
  const document = JSON.parse(readFileSync(join(repoRoot, schemaModulePath(repoRoot, scope)), "utf8")) as { $schema?: string; $id?: string };
  if (document.$schema !== "http://json-schema.org/draft-07/schema#") throw new Error(`${scope} module must declare the draft-07 dialect`);
  if (typeof document.$id !== "string" || !document.$id.startsWith(SCHEMA_ID_PREFIX)) throw new Error(`${scope} module must declare a semio.tech $id`);
  if (scope.startsWith("hub.") && !document.$id.startsWith(`${SCHEMA_ID_PREFIX}hub/`)) throw new Error(`${scope} module must declare a hub $id`);
  schemaModuleIds.set(scope, document.$id);
  for (const dependency of new Set([...(schemaCatalog(repoRoot)[scope]?.dependsOn ?? []), ...schemaReferencedScopes(document, scope)])) schemaModuleId(repoRoot, dependency);
  schemaAjv.addSchema(document);
  return document.$id;
}

/** 🔗️ Resolves `schema://<scope id>/<ExportId>` to the owning module's compiled export validator. */
export function hubSchemaExport(repoRoot: string, uri: string): (value: unknown) => boolean {
  const match = /^schema:\/\/([a-z0-9.-]+)\/([A-Z][A-Za-z0-9]*)$/.exec(uri);
  if (!match) throw new Error(`malformed schema export uri ${uri}`);
  const [, scope, exportId] = match;
  const id = schemaModuleId(repoRoot, scope!);
  const validate = schemaAjv.getSchema(`${id}#/$defs/${exportId}`);
  if (!validate) throw new Error(`${scope} exports no ${exportId}`);
  return (value: unknown): boolean => validate(value) as boolean;
}
//#endregion 🧬️Scope-owned schema resolution

//#region 🔖️Port
/** 🔌️ Binds an ephemeral port (`:0`), reads back what the OS assigned, then releases it — the
 * standard "scan, don't hardcode" free-port trick (contract-freeze §C0's own e2e-port section
 * describes the same idea for a sibling lane's fixed pool; this harness has no fixed pool to
 * share, so it scans instead). A small bind-then-release race is possible (another process could
 * grab the port before the hub does); acceptable for a local dev/CI integration test. */
export async function findFreePort(host = "127.0.0.1"): Promise<number> {
  return new Promise((resolvePort, rejectPort) => {
    const server = createServer();
    server.unref();
    server.once("error", rejectPort);
    server.listen(0, host, () => {
      const address = server.address();
      if (address === null || typeof address === "string") {
        server.close(() => rejectPort(new Error("findFreePort: no ephemeral port assigned")));
        return;
      }
      const { port } = address;
      server.close(() => resolvePort(port));
    });
  });
}
//#endregion 🔖️Port

//#region 🔖️Readiness
/** ⏳️ Polls `url` with `fetch` until it answers (any HTTP status counts — the point is proving
 * the listener is actually accepting connections and axum is routing, not that this particular
 * route accepts us), or throws once `timeoutMs` elapses. */
export async function waitForHttpReady(url: string, headers: Record<string, string> = {}, timeoutMs = 60_000): Promise<void> {
  const deadline = Date.now() + timeoutMs;
  let lastError: unknown = new Error("waitForHttpReady: never attempted");
  while (Date.now() < deadline) {
    try {
      await fetch(url, { headers });
      return;
    } catch (error) {
      lastError = error;
      await new Promise((resolveDelay) => setTimeout(resolveDelay, 150));
    }
  }
  throw new Error(`waitForHttpReady: ${url} unreachable after ${timeoutMs}ms: ${String(lastError)}`);
}
//#endregion 🔖️Readiness

//#region 🔖️Hub
/** 📁️ `<cargo target-dir>/debug/os-hub[.exe]` — the plain `cargo build` (default features)
 * output; never the `--release` path, matching `📜️script.ts`'s own build step.
 * https://doc.rust-lang.org/cargo/reference/config.html#buildtarget-dir */
export function resolveHubBinaryPath(repoRoot: string): string {
  const name = process.platform === "win32" ? "os-hub.exe" : "os-hub";
  return join(cargoTargetDirectory(repoRoot), "debug", name);
}

/** 🗂️ Copies a published trusted catalog (`trusted-catalog/current.json` + its generation) into a fresh data root. */
export function hubSeedTrustedCatalog(catalogRoot: string, dataRoot: string): string {
  const source = join(catalogRoot, "trusted-catalog");
  const current = join(source, "current.json");
  if (!existsSync(current)) throw new Error(`no published trusted catalog at ${source}; publish one first (dashboard command os-hub:dev publishes the development catalog into .🧬semio/🌐hub/hub-dev)`);
  const generation = String((JSON.parse(readFileSync(current, "utf8")) as { generationId?: unknown }).generationId ?? "");
  if (!/^[0-9a-f]{64}$/u.test(generation)) throw new Error(`${current} names no generation`);
  const target = join(dataRoot, "trusted-catalog");
  mkdirSync(join(target, "generations"), { recursive: true, mode: 0o700 });
  cpSync(join(source, "generations", generation), join(target, "generations", generation), { recursive: true });
  cpSync(current, join(target, "current.json"));
  return generation;
}

export type HubOptions = {
  readonly repoRoot: string;
  readonly dataDir: string;
  readonly adminToken: string;
  readonly port?: number;
  readonly readyTimeoutMs?: number;
  readonly binaryPath?: string;
  readonly env?: Readonly<Record<string, string>>;
};

export type HubHandle = {
  readonly port: number;
  readonly baseUrl: string;
  readonly wsBaseUrl: string;
  readonly stdout: () => string;
  readonly stderr: () => string;
  stop(): Promise<void>;
};

/** 🚀️ Spawns the prebuilt `os-hub` binary directly (no `cargo run` wrapper) on a scanned free
 * port with a temp `OS_HUB_DATA`/`OS_HUB_ADMIN_TOKEN`, waits for a real HTTP response before
 * returning, and hands back a handle whose `stop()` reliably tears the process down (`SIGTERM`,
 * escalating to `SIGKILL` after a grace period) even if the caller never reads the rest of the
 * handle. */
export async function startHub(options: HubOptions): Promise<HubHandle> {
  const bin = options.binaryPath ?? resolveHubBinaryPath(options.repoRoot);
  if (!existsSync(bin)) {
    throw new Error(`startHub: ${bin} does not exist — build it first: cargo build --manifest-path 🌎️hub/📦️packages/🦀️rust/Cargo.toml`);
  }
  const port = options.port ?? (await findFreePort());
  const env: NodeJS.ProcessEnv = { ...process.env, ...(options.env ?? {}), OS_HUB_PORT: String(port), OS_HUB_DATA: options.dataDir, OS_HUB_ADMIN_TOKEN: options.adminToken };
  const child: ChildProcessByStdio<null, Readable, Readable> = spawn(bin, [], { env, stdio: ["ignore", "pipe", "pipe"] });
  let stdout = "";
  let stderr = "";
  child.stdout.on("data", (chunk: Buffer) => {
    stdout += chunk.toString("utf8");
  });
  child.stderr.on("data", (chunk: Buffer) => {
    stderr += chunk.toString("utf8");
  });
  let exited = false;
  child.once("exit", () => {
    exited = true;
  });
  const baseUrl = `http://127.0.0.1:${port}`;
  const wsBaseUrl = `ws://127.0.0.1:${port}`;

  const stop = async (): Promise<void> => {
    if (exited) return;
    await new Promise<void>((resolveStop) => {
      child.once("exit", () => resolveStop());
      child.kill("SIGTERM");
      setTimeout(() => {
        if (!exited) child.kill("SIGKILL");
      }, 5_000);
    });
  };

  try {
    await waitForHttpReady(`${baseUrl}/admin/api/overview`, { authorization: `Bearer ${options.adminToken}` }, options.readyTimeoutMs ?? 60_000);
  } catch (error) {
    await stop();
    throw new Error(`startHub: hub never became ready — stderr:\n${stderr}\n${String(error)}`);
  }
  if (exited) {
    throw new Error(`startHub: hub process exited before becoming ready — stderr:\n${stderr}`);
  }

  return { port, baseUrl, wsBaseUrl, stdout: () => stdout, stderr: () => stderr, stop };
}
//#endregion 🔖️Hub



//#region 🔖️ForwardingProxy
/** 🔀️ A loopback stand-in for the TLS-terminating reverse proxy a production hub on a network interface sits behind
 * (`OS_HUB_TRUSTED_FORWARDING=proxy`): every HTTP request and document/directory WebSocket reaches `upstream` stamped
 * `X-Forwarded-Proto: https` + `X-Forwarded-Host`, exactly the statement the hub's transport layer requires, so probe
 * clients drive a production-posture hub (a container, a network bind) unchanged. Hop-by-hop headers are dropped both
 * ways; WebSocket subprotocols are negotiated with the upstream first and its choice is handed to the client.
 * https://developer.mozilla.org/docs/Web/HTTP/Headers/X-Forwarded-Proto */
export type HubForwardingProxy = Readonly<{ origin: string; requests: () => number; stop: () => void }>;

const HUB_PROXY_HOP_HEADERS = ["connection", "keep-alive", "upgrade", "proxy-connection", "transfer-encoding", "te", "trailer", "host", "content-length", "accept-encoding", "sec-websocket-key", "sec-websocket-version", "sec-websocket-extensions", "sec-websocket-protocol", "sec-websocket-accept"];

/** 🔀️ The request headers the proxy forwards: the client's own minus hop-by-hop, plus the TLS-termination statement. */
export function hubForwardedHeaders(incoming: Headers, forwardedHost: string): Headers {
  const headers = new Headers(incoming);
  for (const name of HUB_PROXY_HOP_HEADERS) headers.delete(name);
  headers.set("x-forwarded-proto", "https");
  headers.set("x-forwarded-host", forwardedHost);
  return headers;
}

/** 🔀️ Starts the forwarding proxy on a free loopback port in front of `upstream` (`http://127.0.0.1:<port>`). */
export function hubForwardingProxy(upstream: string, forwardedHost?: string): HubForwardingProxy {
  const target = new URL(upstream);
  const upstreamSocketOrigin = `${target.protocol === "https:" ? "wss" : "ws"}://${target.host}`;
  let requests = 0;
  type Relay = { upstream: WebSocket; backlog: (string | ArrayBuffer)[]; client?: { send(message: string | ArrayBuffer): unknown; close(code?: number, reason?: string): void } };
  const closeCode = (code: number): number => (code === 1000 || code === 1001 || (code >= 1007 && code <= 1014 && code !== 1010) || (code >= 3000 && code <= 4999) ? code : 1011);
  const server = Bun.serve<Relay>({
    hostname: "127.0.0.1",
    port: 0,
    async fetch(request, proxy) {
      requests += 1;
      const url = new URL(request.url);
      const headers = hubForwardedHeaders(request.headers, forwardedHost ?? url.host);
      if (request.headers.get("upgrade")?.toLowerCase() === "websocket") {
        const protocols = (request.headers.get("sec-websocket-protocol") ?? "").split(",").map((value) => value.trim()).filter(Boolean);
        const socket = new WebSocket(`${upstreamSocketOrigin}${url.pathname}${url.search}`, { headers: Object.fromEntries(headers), protocols } as unknown as string[]);
        socket.binaryType = "arraybuffer";
        const relay: Relay = { upstream: socket, backlog: [] };
        socket.addEventListener("message", (event) => (relay.client ? relay.client.send(event.data as string | ArrayBuffer) : relay.backlog.push(event.data as string | ArrayBuffer)));
        socket.addEventListener("close", (event) => relay.client?.close(closeCode(event.code), event.reason));
        const opened = await new Promise<boolean>((resolveOpen) => {
          socket.addEventListener("open", () => resolveOpen(true), { once: true });
          socket.addEventListener("error", () => resolveOpen(false), { once: true });
          socket.addEventListener("close", () => resolveOpen(false), { once: true });
        });
        if (!opened) return new Response(null, { status: 502 });
        if (proxy.upgrade(request, { data: relay, headers: socket.protocol ? { "Sec-WebSocket-Protocol": socket.protocol } : {} })) return undefined;
        socket.close(1000, "client upgrade refused");
        return new Response(null, { status: 400 });
      }
      const answer = await fetch(`${target.origin}${url.pathname}${url.search}`, { method: request.method, headers, body: request.method === "GET" || request.method === "HEAD" ? undefined : await request.arrayBuffer(), redirect: "manual" });
      const answered = new Headers(answer.headers);
      for (const name of [...HUB_PROXY_HOP_HEADERS, "content-encoding"]) answered.delete(name);
      return new Response(request.method === "HEAD" ? null : await answer.arrayBuffer(), { status: answer.status, statusText: answer.statusText, headers: answered });
    },
    websocket: {
      open(client) {
        client.data.client = client;
        for (const message of client.data.backlog.splice(0)) client.send(message);
        if (client.data.upstream.readyState >= WebSocket.CLOSING) client.close(1011, "upstream closed");
      },
      message(client, message) {
        client.data.upstream.send(message);
      },
      close(client, code, reason) {
        if (client.data.upstream.readyState < WebSocket.CLOSING) client.data.upstream.close(closeCode(code), reason);
      },
    },
  });
  return { origin: `http://127.0.0.1:${server.port}`, requests: () => requests, stop: () => void server.stop(true) };
}
//#endregion 🔖️ForwardingProxy

