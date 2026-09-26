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
  if (!existsSync(current)) throw new Error(`no published trusted catalog at ${source}; publish one first (launch row 🛠️dev🗄️os-hub publishes the development catalog into .🧬semio/🌐hub/hub-dev)`);
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

//#region 🔖️ProbeClient
/** 🧑‍💻️ A hub probe client acting exactly as a signed-in human's browser does over HTTP and the document socket: credential
 * sign-in, the directory's `create-space` command, the server-owned artifact creation, the open plan, and chained edits
 * over one document socket. Shared by the hub's operational drills (backup/restore, residency). */
export type HubProbeAnswer = { readonly status: number; readonly text: string; readonly json: any; readonly bytes: Uint8Array };

/** 🌐️ One JSON call against `origin`, bounded to two minutes. */
export async function hubProbeCall(origin: string, method: string, path: string, token?: string, body?: string, accept?: string): Promise<HubProbeAnswer> {
  const response = await fetch(`${origin}${path}`, { method, headers: { ...(body === undefined ? {} : { "content-type": "application/json" }), ...(token ? { authorization: `Bearer ${token}` } : {}), ...(accept ? { accept } : {}) }, ...(body === undefined ? {} : { body }), signal: AbortSignal.timeout(120_000) });
  const bytes = new Uint8Array(await response.arrayBuffer());
  const text = new TextDecoder().decode(bytes);
  let json: any = null;
  try {
    json = JSON.parse(text);
  } catch {
    json = null;
  }
  return { status: response.status, text, json, bytes };
}

/** 🔑️ Credential sign-in as a browser client; answers the session token. */
export async function hubProbeSignIn(origin: string, email: string, password: string, device: string): Promise<string> {
  const answer = await hubProbeCall(origin, "POST", "/auth/sessions", undefined, JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email, password, deviceInstanceId: `${device}${randomBytes(10).toString("hex")}`, clientClass: "browser" }));
  if (answer.status !== 200 || typeof answer.json?.token !== "string") throw new Error(`sign-in ${answer.status} ${answer.text.slice(0, 200)}`);
  return answer.json.token;
}

/** 🏘️ Creates a private studio space through the directory command and answers its id. */
export async function hubProbeCreateSpace(origin: string, token: string, name: string): Promise<string> {
  const answer = await hubProbeCall(origin, "POST", "/directory/commands", token, directoryCommandRequestJson(sealDirectoryCommandRequestV1(randomBytes(16).toString("hex"), createSpaceCommandV1(name, "studio", "private"))));
  const spaceId = answer.json?.events?.find((event: any) => event?.body?.kind === "space.created")?.body?.spaceId;
  if (answer.status !== 202 || typeof spaceId !== "string") throw new Error(`create-space ${answer.status} ${answer.text.slice(0, 300)}`);
  return spaceId;
}

/** 🗂️ The space's creation catalog: its generation and creatable kinds. */
export async function hubProbeCreationCatalog(origin: string, token: string, spaceId: string): Promise<{ generationId: string; kinds: { kindId: string; schema: string }[] }> {
  const answer = await hubProbeCall(origin, "GET", `/spaces/${encodeURIComponent(spaceId)}/artifact-creations`, token);
  if (answer.status !== 200) throw new Error(`creation catalog ${answer.status} ${answer.text.slice(0, 300)}`);
  return { generationId: String(answer.json?.catalogGenerationId ?? ""), kinds: (answer.json?.kinds ?? []) as { kindId: string; schema: string }[] };
}

/** 🌱️ Runs the server-owned creation of one `kindId` document to `ready` and answers its artifact id and duration. */
export async function hubProbeCreateArtifact(origin: string, token: string, spaceId: string, generationId: string, kindId: string, name: string, budgetMs = 1_800_000): Promise<{ artifactId: string; ms: number }> {
  const route = `/spaces/${encodeURIComponent(spaceId)}/artifact-creations`;
  const requestId = randomBytes(16).toString("hex");
  const started = Date.now();
  const accepted = await hubProbeCall(origin, "POST", route, token, JSON.stringify(sealSpaceArtifactCreateV1({ requestId, expectedCatalogGenerationId: generationId, kindId, name })));
  if (accepted.status !== 202) throw new Error(`creation ${accepted.status} ${accepted.text.slice(0, 300)}`);
  let state = accepted.json;
  while (["accepted", "preparing", "indeterminate"].includes(state?.phase) && Date.now() - started < budgetMs) {
    await new Promise((resolveDelay) => setTimeout(resolveDelay, 500));
    state = (await hubProbeCall(origin, "GET", `${route}/${requestId}`, token)).json;
  }
  if (state?.phase !== "ready") throw new Error(`creation of ${kindId} ended ${JSON.stringify(state).slice(0, 300)}`);
  return { artifactId: String(state.ready.artifactId), ms: Date.now() - started };
}

/** 🧭️ Asks for the editor open plan of one document (the hub loads the kind's package to answer it). */
export async function hubProbeOpenPlan(origin: string, token: string, spaceId: string, documentId: string, client: string): Promise<HubProbeAnswer> {
  return hubProbeCall(origin, "POST", `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}/open-plan`, token, JSON.stringify({ schema: "semio.hub.document-open-intent/v1", version: 1, scope: { spaceId, documentId }, clientInstanceId: client }));
}

/** 📡️ One open document socket: its Welcome frame, chained opaque edits and close. */
export type HubProbeDocument = Readonly<{ welcome: any; edit: (index: number, previous: string) => Promise<string>; close: () => void }>;

/** 📡️ Opens one document over the plan → socket grant → socket hello path and answers once it is welcomed. */
export async function hubProbeOpenDocument(origin: string, token: string, spaceId: string, documentId: string, client: string): Promise<HubProbeDocument> {
  const scope = `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}`;
  const plan = await hubProbeOpenPlan(origin, token, spaceId, documentId, client);
  if (plan.status !== 200) throw new Error(`open-plan ${plan.status} ${plan.text.slice(0, 300)}`);
  const grant = await hubProbeCall(origin, "POST", `${scope}/socket-grants`, token, JSON.stringify({ schema: "semio.hub.document-plan-socket-grant-intent/v1", version: 1, planReceipt: plan.json.receipt }));
  if (grant.status !== 200) throw new Error(`socket-grant ${grant.status} ${grant.text.slice(0, 300)}`);
  const granted = parseDocumentSocketGrantReceiptV1(grant.json);
  const socket = new WebSocket(`${origin.replace(/^http/u, "ws")}/scopes/${encodeURIComponent(`${spaceId}/${documentId}`)}/document/ws?surface=${encodeURIComponent(plan.json.surface.surfaceId)}`, ["semio.session.v1", token]);
  socket.binaryType = "arraybuffer";
  const frames: any[] = [];
  const waiters: ((frame: any) => void)[] = [];
  socket.addEventListener("message", (event) => {
    if (!(event.data instanceof ArrayBuffer)) return;
    const frame = decodeServerFrame(new Uint8Array(event.data)).frame as any;
    if ("Commands" in frame || "Presence" in frame) return;
    frames.push(frame);
    for (const waiter of [...waiters]) waiter(frame);
  });
  const waitFrame = (matches: (frame: any) => boolean, label: string, budgetMs = 60_000): Promise<any> =>
    new Promise((resolveFrame, rejectFrame) => {
      const hit = frames.find(matches);
      if (hit) return resolveFrame(hit);
      const onFrame = (frame: any): void => {
        if (!matches(frame)) return;
        clearTimeout(timer);
        waiters.splice(waiters.indexOf(onFrame), 1);
        resolveFrame(frame);
      };
      waiters.push(onFrame);
      const timer = setTimeout(() => rejectFrame(new Error(`missing ${label}; last frames ${JSON.stringify(frames.slice(-3)).slice(0, 400)}`)), budgetMs);
    });
  for (let tick = 0; tick < 400 && socket.readyState === WebSocket.CONNECTING; tick += 1) await new Promise((resolveDelay) => setTimeout(resolveDelay, 50));
  if (socket.readyState !== WebSocket.OPEN) throw new Error("document socket did not open");
  const packSchemaHash = [...(String(plan.json.artifact.packSchemaHash).match(/../gu) ?? [])].map((pair) => Number.parseInt(pair, 16));
  socket.send(encodeClientFrame({ SocketHelloV1: { wire_version: 1, protocol_version: 1, schema: plan.json.artifact.schema, pack_schema_hash: packSchemaHash, resume_token: null, frontier: null } }, "command"));
  const welcome = await waitFrame((frame) => "Welcome" in frame || "Error" in frame, "Welcome");
  if ("Error" in welcome) throw new Error(`refused: ${JSON.stringify(welcome.Error)}`);
  const edit = async (index: number, previous: string): Promise<string> => {
    const mutationId = `probe-${documentId}-${index}`;
    socket.send(encodeClientFrame({ Commands: { batch_id: index + 1, envelopes: [{ mutation_id: mutationId, document_id: documentId, actor: granted.actorId, dependencies: previous ? [previous] : [], observed: null, target: [], diff: { schema: plan.json.artifact.schema, payload: Array.from(new TextEncoder().encode(`probe:${index}:${"b".repeat(512)}`)) }, inverse: { schema: plan.json.artifact.schema, payload: [] }, timestamp: { actor: 1, physical_ms: Date.now(), logical: 0 } }] } }, "command"));
    const acked = await waitFrame((frame) => "Ack" in frame && frame.Ack.batch_id === index + 1, `Ack ${index}`);
    if (!JSON.stringify(acked.Ack.stages).includes("Accepted")) throw new Error(`edit ${index} not accepted: ${JSON.stringify(acked.Ack)}`);
    return mutationId;
  };
  return { welcome: welcome.Welcome, edit, close: () => socket.close(1000, "probe") };
}
//#endregion 🔖️ProbeClient

