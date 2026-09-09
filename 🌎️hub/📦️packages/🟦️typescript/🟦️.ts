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

import { type ChildProcessByStdio, spawn } from "node:child_process";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { createServer } from "node:net";
import { join } from "node:path";
import type { Readable } from "node:stream";
import Ajv from "ajv";

export { getWorkspaceRoot } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

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
      if (!entry.isDirectory() || entry.name === "node_modules" || entry.name === "target") continue;
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

const SCHEMA_ID_PREFIX = "https://semio.tech/schema/";

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
/** 📁️ `<repoRoot>/target/debug/os-hub[.exe]` — the plain `cargo build` (default features)
 * output; never the `--release` path, matching `📜️script.ts`'s own build step. */
export function resolveHubBinaryPath(repoRoot: string): string {
  const name = process.platform === "win32" ? "os-hub.exe" : "os-hub";
  return join(repoRoot, "target", "debug", name);
}

export type HubOptions = {
  readonly repoRoot: string;
  readonly dataDir: string;
  readonly adminToken: string;
  readonly port?: number;
  readonly readyTimeoutMs?: number;
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
  const bin = resolveHubBinaryPath(options.repoRoot);
  if (!existsSync(bin)) {
    throw new Error(`startHub: ${bin} does not exist — build it first: cargo build --manifest-path 🌎️hub/📦️packages/🦀️rust/Cargo.toml`);
  }
  const port = options.port ?? (await findFreePort());
  const env: NodeJS.ProcessEnv = { ...process.env, OS_HUB_PORT: String(port), OS_HUB_DATA: options.dataDir, OS_HUB_ADMIN_TOKEN: options.adminToken };
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
