import { randomBytes } from "node:crypto";
import { spawn, spawnSync, type ChildProcess } from "node:child_process";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { createServer } from "node:net";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import type { Duplex } from "node:stream";
import { cargoTargetDirectory } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🟦️.ts";
import { terminateOwnedChildTree } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts";
import { protectOwnerOnly } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🔐️owner-only/🟦️.ts";
import { getWorkspaceRoot } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts";
import { GIS_INFERENCE_CHECKPOINT_CONTROL_FRAME_MAX_BYTES } from "../../💡️inference/🧬️schema/🟦️.ts";
import { authenticatedFrame, LOCAL_BOOTSTRAP_SCHEMA, type LocalProfile, verifyAuthenticatedFrame } from "../🛂authentication/🟦️.ts";
import { LOCAL_BOOTSTRAP_DEADLINE_MS, LocalFrameReader, writeLocalFrame } from "../📡️framing/🟦️.ts";

/** ⏳ The span of NO OBSERVABLE PROGRESS that ends a readiness wait. It is not a total budget and
 * never was a good one: a total wall budget charges a booting hub for every millisecond the machine
 * spends on other work, so a launcher on a busy machine abandoned a hub that was still coming up
 * (measured 2026-09-21 by CE2 — two of three 7621 starts, while 17 rustc ran). The number is
 * unchanged from the total deadline it replaces: nothing here is a lengthened timeout. */
export const LOCAL_READINESS_STALL_BOUND_MS = 30_000;

/** ⏳ The no-progress span of a readiness wait over a hub that LOADS a trusted catalog at startup: the
 * hub's own startup loader declares its load stalled only after `TRUSTED_CATALOG_STARTUP_STALL_BOUND_MS`
 * (`🌎️hub/🏗️bootstrap/🦀️.rs`, 300 s) without a checkpoint, and a single unit (compiling or interpreting one
 * package's component) legitimately runs silent longer than {@link LOCAL_READINESS_STALL_BOUND_MS} on a busy
 * machine — measured 90 s for one unit (ticket 26/09/23 W1 §4.6). A waiter stricter than the hub's own bound
 * abandons a hub that is still loading. */
export const TRUSTED_CATALOG_READINESS_STALL_BOUND_MS = 300_000;

export type LocalHubRunAllocationOperations = Readonly<{
  makeTemporaryDirectory: (prefix: string) => string;
  protectDirectory: (path: string) => void;
  removeDirectory: (path: string) => void;
}>;

export type LocalHubStartOptions = Readonly<{
  port?: number;
  dataDir?: string;
  capture?: boolean;
  adminSubjects?: readonly string[];
  isolatedSecuritySmoke?: boolean;
  binaryPath?: string;
  /** 🔐️ When set, the child keeps `OS_HUB_ADMIN_TOKEN` for operator surfaces (collab e2e admin checks). */
  adminToken?: string;
  inferenceCheckpointControl?: boolean;
  runParent?: string;
  allocationOperations?: LocalHubRunAllocationOperations;
}>;

export type LocalHubRun = {
  readonly child: ChildProcess;
  readonly pipe: Duplex;
  readonly reader: LocalFrameReader;
  readonly channelKey: Buffer;
  readonly runId: string;
  readonly port: number;
  readonly runRoot: string;
  /** 🎫️ Whether this run asked the hub to also mint sessions from public credentials
   * (`OS_HUB_CREDENTIAL_SIGN_IN`). A development hub issues through the pipe alone by default; a run
   * that did not ask for public issuance must still see `/readyz` deny it, which is what
   * {@link localHubReadinessAdmitted} compares against. */
  readonly publicSessionIssuance: boolean;
  readonly output: () => string;
  readonly removeRunRoot: () => void;
  readonly inferenceCheckpointPipe?: Duplex;
  readonly inferenceCheckpointReader?: LocalFrameReader;
  inferenceCheckpointEnteredJobId?: string;
  inferenceCheckpointProgress?: Readonly<{ progressCursor: number; completed: number; total: number }>;
  inferenceCheckpointReleased?: boolean;
  finishPromise?: Promise<void>;
};

const nativeAllocationOperations: LocalHubRunAllocationOperations = {
  makeTemporaryDirectory: (prefix) => mkdtempSync(prefix),
  protectDirectory: (path) => protectOwnerOnly(path, "directory"),
  removeDirectory: (path) => rmSync(path, { recursive: true, force: true }),
};

/** 📁 Allocates one private local-Hub root through an injectable, owned filesystem boundary. */
export function allocateLocalHubRunRoot(parent = tmpdir(), operations: LocalHubRunAllocationOperations = nativeAllocationOperations): Readonly<{ path: string; remove: () => void }> {
  const path = operations.makeTemporaryDirectory(join(parent, "semio-hub-run-"));
  operations.protectDirectory(path);
  return { path, remove: () => operations.removeDirectory(path) };
}

/** 🔌 Reserves and releases one loopback port before a bounded child starts. */
export async function freeLoopbackPort(): Promise<number> {
  const server = createServer();
  await new Promise<void>((resolveListen, rejectListen) => server.once("error", rejectListen).listen(0, "127.0.0.1", resolveListen));
  const address = server.address();
  if (!address || typeof address === "string") throw new Error("local bootstrap launcher could not allocate a loopback port");
  await new Promise<void>((resolveClose, rejectClose) => server.close((error) => (error ? rejectClose(error) : resolveClose())));
  return address.port;
}

/** 🗺️ Resolves the workspace-cached debug Hub executable for native proofs. */
export function hubBinaryPath(repoRoot: string): string {
  return join(cargoTargetDirectory(repoRoot), "debug", process.platform === "win32" ? "os-hub.exe" : "os-hub");
}

/** 🔏️ The one trusted-catalog package list every loopback DEVELOPMENT hub publishes into a data root that holds none —
 * the hub's own bootstrap closure (stdio, gis) plus the collaboration editors — shared by `os-hub:dev` and the `dev s`
 * local hub owner so whichever launch row reaches a clean `hub-dev` root first publishes the same catalog. */
export const LOCAL_HUB_DEVELOPMENT_CATALOG_PACKAGES = "stdio,gis,note,writer,draw,puzzle";

/** 👥️ The local-bootstrap profiles a development hub declares: the developer every single-user row signs in as, and the
 * two humans the two-user rows (`👤️1`, `👤️2`) sign in as — each a distinct hub user reached through the session broker. */
export const LOCAL_HUB_DEVELOPMENT_PROFILES: readonly LocalProfile[] = Object.freeze([
  Object.freeze({ profileId: "developer", subject: "local-developer-01", displayName: "Local Developer", allowedClientClasses: Object.freeze(["native", "mcp", "react-relay"] as const) }),
  Object.freeze({ profileId: "user-1", subject: "local-user-01", displayName: "Local User One", allowedClientClasses: Object.freeze(["react-relay"] as const) }),
  Object.freeze({ profileId: "user-2", subject: "local-user-02", displayName: "Local User Two", allowedClientClasses: Object.freeze(["react-relay"] as const) }),
]);

/** 🎯 The single Nx target that stages the development executable every launch route reads. */
export const HUB_DEV_BINARY_TARGET = "os-hub:build-dev";

/** 🐘️ The Nx target that stages the PostgreSQL-capable development executable — the SAME hub with
 * `db`'s `postgres` storage/directory drivers linked in, which the default build deliberately leaves
 * out so a zero-touch launch needs no database. */
export const HUB_DEV_POSTGRES_BINARY_TARGET = "os-hub:build-dev-postgres";

/** 🏗️ The injectable staging boundary: one attempt that returns the exit status of the Nx target. */
export type HubDevBinaryStaging = Readonly<{ stage: () => number }>;

const nativeHubDevBinaryStaging: HubDevBinaryStaging = {
  stage: () => spawnSync("bun", ["nx", "run", HUB_DEV_BINARY_TARGET], { cwd: getWorkspaceRoot(), stdio: "inherit", shell: false }).status ?? -1,
};

const nativeHubDevPostgresBinaryStaging: HubDevBinaryStaging = {
  stage: () => spawnSync("bun", ["nx", "run", HUB_DEV_POSTGRES_BINARY_TARGET], { cwd: getWorkspaceRoot(), stdio: "inherit", shell: false }).status ?? -1,
};

/** 📦 Stages the development executable through its own Nx target on EVERY launch and returns its path. The Nx
 * hash over the hub's native sources decides freshness (a cache hit restores or keeps the matching binary), so a
 * launch route never boots a binary left behind by an older tree; a failed staging names the target.
 * @see ../../📦️packages/🦀️rust/📋️project.json `build-dev` */
export function hubDevBinaryPath(root: string, staging: HubDevBinaryStaging = nativeHubDevBinaryStaging): string {
  const path = join(root, "dist", "build-dev", process.platform === "win32" ? "os-hub.exe" : "os-hub");
  const status = staging.stage();
  if (status !== 0 || !existsSync(path)) throw new Error(`Missing Nx-staged os-hub dev binary: ${path}; staging it through \`bun nx run ${HUB_DEV_BINARY_TARGET}\` exited with status ${status}`);
  return path;
}

/** 🐘️ The same staging for the PostgreSQL-capable executable, staged into its own directory so the two
 * feature sets never overwrite one another (and a running hub is never replaced in place). */
export function hubDevPostgresBinaryPath(root: string, staging: HubDevBinaryStaging = nativeHubDevPostgresBinaryStaging): string {
  const path = join(root, "dist", "build-dev-postgres", process.platform === "win32" ? "os-hub.exe" : "os-hub");
  const status = staging.stage();
  if (status !== 0 || !existsSync(path)) throw new Error(`Missing Nx-staged PostgreSQL os-hub dev binary: ${path}; staging it through \`bun nx run ${HUB_DEV_POSTGRES_BINARY_TARGET}\` exited with status ${status}`);
  return path;
}

/** 🧾️ The `semio.cargo.binary-sources/v1` record every hub staging verb writes beside the `os-hub` executable. */
export const HUB_BINARY_SOURCES_FILE = "os-hub.sources.json";

/** 🔎️ The pid of the process listening on `port`. */
export function listeningProcessId(port: number): number {
  if (process.platform === "win32") {
    const answer = spawnSync("netstat", ["-ano", "-p", "TCP"], { encoding: "utf8" });
    const row = answer.stdout.split(/\r?\n/u).find((line) => new RegExp(`:${port}\\s+\\S+\\s+LISTENING\\s+\\d+`, "u").test(line));
    const pid = Number(row?.trim().split(/\s+/u).at(-1));
    if (!Number.isInteger(pid) || pid <= 0) throw new Error(`no process listens on port ${port}`);
    return pid;
  }
  const answer = spawnSync("lsof", ["-nP", `-iTCP:${port}`, "-sTCP:LISTEN", "-t"], { encoding: "utf8" });
  const pid = Number(answer.stdout.trim().split("\n")[0]);
  if (!Number.isInteger(pid) || pid <= 0) throw new Error(`no process listens on port ${port} (lsof status ${answer.status})`);
  return pid;
}

//#region 🔖️BackendServers
/** 🗄️ The external durable backends a hub runs on besides the built-in `fs`/`sqlite` pair. */
export type HubBackendName = "postgres" | "neo4j";

/** 🐳️ One backend server as its `🌎️hub/compose.yaml` service declares it: the container port, the readiness
 * probe and query client (the server's own tools, run inside the container), and the hub environment that
 * points BOTH durable halves (document store and directory) at a server reachable on `host:port`. */
export type HubBackendDefinition = Readonly<{
  service: HubBackendName;
  containerPort: number;
  ready: readonly string[];
  client: readonly string[];
  env: (host: string, port: number) => Readonly<Record<string, string>>;
}>;

/** 📜️ Every external backend, keyed by name. Credentials are the compose service's own development values. */
export const HUB_BACKENDS: Readonly<Record<HubBackendName, HubBackendDefinition>> = Object.freeze({
  postgres: Object.freeze({
    service: "postgres",
    containerPort: 5432,
    ready: Object.freeze(["pg_isready", "--username=semio", "--dbname=semio"]),
    client: Object.freeze(["psql", "--username=semio", "--dbname=semio", "--tuples-only", "--no-align", "--command"]),
    env: (host: string, port: number) => {
      const url = `postgres://semio:semio@${host}:${port}/semio`;
      return { OS_HUB_STORAGE_BACKEND: "postgres", OS_HUB_DATABASE_URL: url, OS_HUB_DIRECTORY_BACKEND: "postgres", OS_HUB_DIRECTORY_DATABASE_URL: url };
    },
  }),
  neo4j: Object.freeze({
    service: "neo4j",
    containerPort: 7687,
    ready: Object.freeze(["cypher-shell", "--username", "neo4j", "--password", "semio-hub", "RETURN 1"]),
    client: Object.freeze(["cypher-shell", "--username", "neo4j", "--password", "semio-hub", "--format", "plain"]),
    env: (host: string, port: number) => {
      const uri = `bolt://${host}:${port}`;
      return { OS_HUB_STORAGE_BACKEND: "neo4j", OS_HUB_NEO4J_URI: uri, OS_HUB_NEO4J_USER: "neo4j", OS_HUB_NEO4J_PASSWORD: "semio-hub", OS_HUB_DIRECTORY_BACKEND: "neo4j", OS_HUB_DIRECTORY_NEO4J_URI: uri, OS_HUB_DIRECTORY_NEO4J_USER: "neo4j", OS_HUB_DIRECTORY_NEO4J_PASSWORD: "semio-hub" };
    },
  }),
});

/** 🔎️ Narrows one argv word to a backend name, naming every accepted value when it is none. */
export function hubBackendName(value: string | undefined): HubBackendName {
  if (value === "postgres" || value === "neo4j") return value;
  throw new Error(`expected a hub backend (${Object.keys(HUB_BACKENDS).join(" | ")}), got ${JSON.stringify(value ?? "")}`);
}

/** 🏷️ Where the ONE shared development server of a backend lives: its compose project and container. Every
 * consumer — `os-hub-ts:backend-up`, `os-hub:dev-postgres`/`dev-neo4j` and the pg/neo4j gates — reuses it; a gate
 * isolates itself through {@link claimHubBackend}, never through a second server. */
export type HubBackendIdentity = Readonly<{ name: HubBackendName; project: string; container: string }>;

/** 🏷️ The identity of the shared development server of `name`. */
export function hubBackendIdentity(name: HubBackendName): HubBackendIdentity {
  const project = `semio-hub-backend-${name}`;
  return Object.freeze({ name, project, container: project });
}

/** 🐘️ A running, ready backend server: its identity, the loopback port its container port is published on, the
 * hub environment that selects it, and the in-container query client a gate counts live WAL writers with. */
export type HubBackendServer = Readonly<{ identity: HubBackendIdentity; host: string; port: number; env: Readonly<Record<string, string>>; client: readonly string[] }>;

/** 📈️ One observable step of a backend start (`engine` answered, `start` in progress, `ready`), with the seconds spent. */
export type HubBackendProgress = Readonly<{ name: HubBackendName; phase: "engine" | "start" | "ready"; elapsedSeconds: number }>;

/** 🎛️ Start options: cancellation (the half-started container is removed) and a progress sink. */
export type HubBackendStartOptions = Readonly<{ signal?: AbortSignal; onProgress?: (progress: HubBackendProgress) => void; readinessBoundMs?: number }>;

/** ⏳ How long a started container may stay unready before the start fails — the neo4j JVM measured ~25 s cold. */
export const HUB_BACKEND_READINESS_BOUND_MS = 180_000;

/** 🐳️ The container engine CLI. Cross-platform story: Docker Desktop on macOS and Windows, Docker Engine on
 * Linux, and docker-in-docker inside the devcontainer — in every case the engine publishes on THIS process's
 * loopback, so the server is always `127.0.0.1:<port>` and no host resolution differs by platform. */
export const HUB_BACKEND_ENGINE = "docker";

/** 🌐️ The loopback host every backend is published on and reached through. */
export const HUB_BACKEND_HOST = "127.0.0.1";

type EngineResult = Readonly<{ status: number; stdout: string; stderr: string }>;

function engine(repoRoot: string, args: readonly string[], options: Readonly<{ inherit?: boolean; signal?: AbortSignal }> = {}): Promise<EngineResult> {
  return new Promise((resolveRun, rejectRun) => {
    if (options.signal?.aborted) return rejectRun(new Error("hub backend start cancelled"));
    const child = spawn(HUB_BACKEND_ENGINE, [...args], { cwd: repoRoot, shell: false, stdio: ["ignore", options.inherit ? "inherit" : "pipe", options.inherit ? "inherit" : "pipe"] });
    const out: Buffer[] = [];
    const err: Buffer[] = [];
    child.stdout?.on("data", (chunk: Buffer) => out.push(chunk));
    child.stderr?.on("data", (chunk: Buffer) => err.push(chunk));
    const abort = (): void => void child.kill("SIGTERM");
    options.signal?.addEventListener("abort", abort, { once: true });
    child.once("error", (error) => {
      options.signal?.removeEventListener("abort", abort);
      rejectRun(new Error(`${HUB_BACKEND_ENGINE} is not runnable (${error.message}); install Docker Desktop (macOS, Windows), Docker Engine (Linux) or the devcontainer docker-in-docker feature`));
    });
    child.once("close", (status) => {
      options.signal?.removeEventListener("abort", abort);
      if (options.signal?.aborted) return rejectRun(new Error("hub backend start cancelled"));
      resolveRun({ status: status ?? -1, stdout: Buffer.concat(out).toString("utf8"), stderr: Buffer.concat(err).toString("utf8") });
    });
  });
}

function composeArgs(repoRoot: string, identity: HubBackendIdentity): string[] {
  return ["compose", "--file", join(repoRoot, "🌎️hub", "compose.yaml"), "--project-name", identity.project, "--profile", identity.name];
}

/** 🩺️ The engine's server version, or a thrown error that names how to install one on this platform. */
export async function hubBackendEngineVersion(repoRoot: string): Promise<string> {
  const probe = await engine(repoRoot, ["info", "--format", "{{.ServerVersion}}"]);
  if (probe.status !== 0) throw new Error(`the ${HUB_BACKEND_ENGINE} daemon does not answer (${probe.stderr.trim() || `status ${probe.status}`}); start Docker Desktop (macOS, Windows) or the Docker Engine service (Linux)`);
  return probe.stdout.trim();
}

/** 🔭️ The live server of `identity`: running, published and answering its own readiness probe — or `undefined`.
 * Docker is the only record of it: nothing is cached on disk that could disagree with the engine. */
export async function hubBackendStatus(repoRoot: string, identity: HubBackendIdentity): Promise<HubBackendServer | undefined> {
  const definition = HUB_BACKENDS[identity.name];
  const running = await engine(repoRoot, ["inspect", "--format", "{{.State.Running}}", identity.container]);
  if (running.status !== 0 || running.stdout.trim() !== "true") return undefined;
  const published = await engine(repoRoot, ["port", identity.container, `${definition.containerPort}/tcp`]);
  const port = Number(/:(\d+)\s*$/m.exec(published.stdout.trim())?.[1]);
  if (published.status !== 0 || !Number.isInteger(port) || port <= 0) return undefined;
  if ((await engine(repoRoot, ["exec", identity.container, ...definition.ready])).status !== 0) return undefined;
  return Object.freeze({ identity, host: HUB_BACKEND_HOST, port, env: Object.freeze(definition.env(HUB_BACKEND_HOST, port)), client: Object.freeze([HUB_BACKEND_ENGINE, "exec", identity.container, ...definition.client]) });
}

/** 🧹️ Removes the container and the compose project's volumes of `identity`; a missing server is not an error.
 * Synchronous, so a gate's `process.once("exit")` teardown can call it. */
export function stopHubBackend(repoRoot: string, identity: HubBackendIdentity): void {
  spawnSync(HUB_BACKEND_ENGINE, ["rm", "--force", identity.container], { cwd: repoRoot, stdio: "ignore", shell: false });
  spawnSync(HUB_BACKEND_ENGINE, [...composeArgs(repoRoot, identity), "down", "--volumes", "--remove-orphans"], { cwd: repoRoot, stdio: "ignore", shell: false });
}

/** 🚀️ Returns the ready server of `identity`, starting it from its compose service when none runs. The image pull
 * streams the engine's own progress; the readiness wait reports every 5 s and is bounded by
 * {@link HUB_BACKEND_READINESS_BOUND_MS}. Cancellation or a failed start removes the half-started container. */
export async function ensureHubBackend(repoRoot: string, identity: HubBackendIdentity, options: HubBackendStartOptions = {}): Promise<HubBackendServer> {
  const started = Date.now();
  const report = (phase: HubBackendProgress["phase"]): void => options.onProgress?.({ name: identity.name, phase, elapsedSeconds: Math.round((Date.now() - started) / 1000) });
  report("engine");
  await hubBackendEngineVersion(repoRoot);
  const existing = await hubBackendStatus(repoRoot, identity);
  if (existing) return existing;
  const definition = HUB_BACKENDS[identity.name];
  try {
    await engine(repoRoot, ["rm", "--force", identity.container]);
    report("start");
    const port = await freeLoopbackPort();
    const run = await engine(repoRoot, [...composeArgs(repoRoot, identity), "run", "--detach", "--rm", "--name", identity.container, "--publish", `${HUB_BACKEND_HOST}:${port}:${definition.containerPort}`, definition.service], { inherit: true, signal: options.signal });
    if (run.status !== 0) throw new Error(`the ${identity.name} compose service did not start (status ${run.status})`);
    const bound = options.readinessBoundMs ?? HUB_BACKEND_READINESS_BOUND_MS;
    let reported = 0;
    for (;;) {
      if (options.signal?.aborted) throw new Error("hub backend start cancelled");
      const server = await hubBackendStatus(repoRoot, identity);
      if (server) {
        report("ready");
        return server;
      }
      const elapsed = Date.now() - started;
      if (elapsed > bound) throw new Error(`${identity.name} did not answer ${definition.ready[0]} within ${Math.round(bound / 1000)} s`);
      if (elapsed - reported >= 5_000) {
        reported = elapsed;
        report("start");
      }
      await new Promise<void>((resolveDelay) => setTimeout(resolveDelay, 1000));
    }
  } catch (error) {
    stopHubBackend(repoRoot, identity);
    throw error;
  }
}
/** 🗝️ One gate run's claim on the shared server: the hub environment and writer-probe client scoped to what the run
 * owns, and a synchronous `release` (safe inside a `process.once("exit")` handler). */
export type HubBackendClaim = Readonly<{ server: HubBackendServer; env: Readonly<Record<string, string>>; client: readonly string[]; release: () => void }>;

/** 🗝️ The file that serialises neo4j claims: Community edition serves exactly one database, so a claim owns the
 * whole server. It records the owning pid; a lease whose pid is gone is taken over. */
export function hubBackendLeasePath(repoRoot: string, name: HubBackendName): string {
  return join(repoRoot, ".🧬semio", "🌐hub", `backend-${name}.lease`);
}

function processAlive(pid: number): boolean {
  try {
    process.kill(pid, 0);
    return true;
  } catch (error) {
    return (error as NodeJS.ErrnoException).code === "EPERM";
  }
}

function takeLease(path: string): boolean {
  mkdirSync(dirname(path), { recursive: true });
  try {
    writeFileSync(path, String(process.pid), { flag: "wx" });
    return true;
  } catch (error) {
    if ((error as NodeJS.ErrnoException).code !== "EEXIST") throw error;
    const owner = Number(readFileSync(path, "utf8").trim());
    if (Number.isInteger(owner) && owner > 0 && processAlive(owner)) return false;
    rmSync(path, { force: true });
    return takeLease(path);
  }
}

/** 🗝️ Claims the shared server of `name` for one gate run `owner` (letters, digits, `_`). postgres: a fresh
 * database `semio_run_<owner>` on the shared server, dropped on release. neo4j: the exclusive lease
 * ({@link hubBackendLeasePath}, waited for with progress and cancellation), then every node is deleted so the run
 * starts on an empty graph — a neo4j gate resets the shared development graph. */
export async function claimHubBackend(repoRoot: string, name: HubBackendName, owner: string, options: HubBackendStartOptions = {}): Promise<HubBackendClaim> {
  if (!/^[a-z0-9_]{1,40}$/.test(owner)) throw new Error(`hub backend claim owner must match [a-z0-9_]{1,40}, got ${JSON.stringify(owner)}`);
  const server = await ensureHubBackend(repoRoot, hubBackendIdentity(name), options);
  const exec = [HUB_BACKEND_ENGINE, "exec", server.identity.container];
  const run = (args: readonly string[]): number => spawnSync(args[0]!, args.slice(1), { cwd: repoRoot, stdio: "ignore", shell: false }).status ?? -1;
  if (name === "postgres") {
    const database = `semio_run_${owner}`;
    const admin = [...exec, "psql", "--username=semio", "--dbname=semio", "--command"];
    run([...admin, `DROP DATABASE IF EXISTS ${database} WITH (FORCE)`]);
    if (run([...admin, `CREATE DATABASE ${database}`]) !== 0) throw new Error(`could not create the run database ${database} on ${server.identity.container}`);
    const url = `postgres://semio:semio@${server.host}:${server.port}/${database}`;
    return Object.freeze({
      server,
      env: Object.freeze({ ...server.env, OS_HUB_DATABASE_URL: url, OS_HUB_DIRECTORY_DATABASE_URL: url }),
      client: Object.freeze([...exec, "psql", "--username=semio", `--dbname=${database}`, "--tuples-only", "--no-align", "--command"]),
      release: () => void run([...admin, `DROP DATABASE IF EXISTS ${database} WITH (FORCE)`]),
    });
  }
  const lease = hubBackendLeasePath(repoRoot, name);
  const started = Date.now();
  let reported = 0;
  while (!takeLease(lease)) {
    if (options.signal?.aborted) throw new Error("hub backend claim cancelled");
    if (Date.now() - reported >= 10_000) {
      reported = Date.now();
      options.onProgress?.({ name, phase: "start", elapsedSeconds: Math.round((reported - started) / 1000) });
    }
    await new Promise<void>((resolveDelay) => setTimeout(resolveDelay, 1000));
  }
  const release = (): void => {
    if (existsSync(lease) && readFileSync(lease, "utf8").trim() === String(process.pid)) rmSync(lease, { force: true });
  };
  if (run([...exec, ...HUB_BACKENDS.neo4j.client, "MATCH (n) CALL { WITH n DETACH DELETE n } IN TRANSACTIONS OF 10000 ROWS"]) !== 0) {
    release();
    throw new Error(`could not reset the shared neo4j graph on ${server.identity.container}`);
  }
  return Object.freeze({ server, env: server.env, client: server.client, release });
}
//#endregion 🔖️BackendServers

/** 🚀 Starts one loopback Hub and completes its authenticated local-bootstrap handshake. */
export async function startLocalHub(repoRoot: string, root: string, profiles: readonly LocalProfile[], options: LocalHubStartOptions = {}): Promise<LocalHubRun> {
  if (profiles.length === 0 || profiles.length > 8) throw new Error("local bootstrap profiles must contain 1..=8 entries");
  const runId = randomBytes(16).toString("hex");
  const channelKey = randomBytes(32);
  const allocation = allocateLocalHubRunRoot(options.runParent, options.allocationOperations);
  const runRoot = allocation.path;
  const port = options.port ?? (await freeLoopbackPort());
  const captured: Buffer[] = [];
  let capturedBytes = 0;
  const capture = (chunk: Buffer): void => {
    const remaining = 1024 * 1024 - capturedBytes;
    if (remaining <= 0) return;
    const retained = Buffer.from(chunk.subarray(0, remaining));
    captured.push(retained);
    capturedBytes += retained.length;
  };
  const env: NodeJS.ProcessEnv = {
    ...process.env,
    OS_HUB_MODE: "development",
    OS_HUB_BIND: "127.0.0.1",
    OS_HUB_PORT: String(port),
    OS_HUB_DATA: options.dataDir ?? join(runRoot, "data"),
  };
  if (options.inferenceCheckpointControl) env.OS_HUB_TEST_INFERENCE_CHECKPOINT_FD = "4";
  else delete env.OS_HUB_TEST_INFERENCE_CHECKPOINT_FD;
  delete env.OS_HUB_TRUSTED_CATALOG_BUNDLE;
  delete env.OS_HUB_TRUSTED_CATALOG_PROFILE;
  if (options.adminToken) env.OS_HUB_ADMIN_TOKEN = options.adminToken;
  else delete env.OS_HUB_ADMIN_TOKEN;
  delete env.S_USER;
  for (const name of Object.keys(env)) if (/^S_.*TOKEN$/.test(name)) delete env[name];
  if (options.isolatedSecuritySmoke) {
    env.OS_HUB_STORAGE_BACKEND = "fs";
    env.OS_HUB_DIRECTORY_BACKEND = "sqlite";
    delete env.OS_HUB_ADMIN_DIR;
  }
  if (options.adminSubjects?.length) env.OS_HUB_ADMIN_SUBJECTS = options.adminSubjects.join(",");
  else delete env.OS_HUB_ADMIN_SUBJECTS;
  const publicSessionIssuance = env.OS_HUB_CREDENTIAL_SIGN_IN === "true" || env.OS_HUB_CREDENTIAL_SIGN_IN === "1";
  const outputMode: "pipe" | "inherit" = options.capture ? "pipe" : "inherit";
  const child = spawn(options.binaryPath ?? hubBinaryPath(repoRoot), [], {
    cwd: root,
    env,
    shell: false,
    stdio: ["ignore", outputMode, outputMode, "pipe", options.inferenceCheckpointControl ? "pipe" : "ignore"],
  });
  if (options.capture) {
    child.stdout?.on("data", capture);
    child.stderr?.on("data", capture);
  }
  const pipe = child.stdio[3] as Duplex;
  const inferenceCheckpointPipe = options.inferenceCheckpointControl ? (child.stdio[4] as Duplex) : undefined;
  if (!pipe || (options.inferenceCheckpointControl && !inferenceCheckpointPipe)) {
    channelKey.fill(0);
    terminateOwnedChildTree(child);
    await waitForChildExit(child, 2_000).catch(() => undefined);
    allocation.remove();
    throw new Error("local bootstrap inherited endpoint was not created");
  }
  const reader = new LocalFrameReader(pipe);
  const run: LocalHubRun = {
    child,
    pipe,
    reader,
    channelKey,
    runId,
    port,
    runRoot,
    publicSessionIssuance,
    output: () => Buffer.concat(captured).toString("utf8"),
    removeRunRoot: allocation.remove,
    inferenceCheckpointPipe,
    inferenceCheckpointReader: inferenceCheckpointPipe ? new LocalFrameReader(inferenceCheckpointPipe, GIS_INFERENCE_CHECKPOINT_CONTROL_FRAME_MAX_BYTES + 4, "GIS inference checkpoint control") : undefined,
    inferenceCheckpointReleased: inferenceCheckpointPipe ? false : undefined,
  };
  try {
    const initialize = { schema: LOCAL_BOOTSTRAP_SCHEMA, kind: "initialize", runId, channelKey: channelKey.toString("hex"), profiles };
    await writeLocalFrame(pipe, initialize);
    initialize.channelKey = "";
    const now = Date.now();
    const helloExchange = randomBytes(16).toString("hex");
    const hello = authenticatedFrame(channelKey, {
      schema: LOCAL_BOOTSTRAP_SCHEMA,
      kind: "hello",
      runId,
      sequence: 1,
      exchangeId: helloExchange,
      issuedAt: now,
      expiresAt: now + LOCAL_BOOTSTRAP_DEADLINE_MS,
      launcherNonce: randomBytes(32).toString("hex"),
    });
    await writeLocalFrame(pipe, hello);
    const accepted = await reader.read();
    verifyAuthenticatedFrame(channelKey, accepted);
    if (accepted.schema !== LOCAL_BOOTSTRAP_SCHEMA || accepted.kind !== "hello-accepted" || accepted.runId !== runId || accepted.exchangeId !== helloExchange || accepted.sequence !== 1)
      throw new Error("local bootstrap mutual hello binding mismatch");
    return run;
  } catch (error) {
    let diagnostics = run.output().slice(-2_048).replaceAll(channelKey.toString("hex"), "<channel-key-redacted>");
    for (const profile of profiles) diagnostics = diagnostics.replaceAll(profile.subject, "<profile-subject-redacted>");
    const status = child.exitCode;
    await finishLocalHub(run);
    throw new Error(`local bootstrap handshake failed (child status ${status ?? "running"}): ${error instanceof Error ? error.message : "unknown"}\n${diagnostics}`);
  }
}

/** ✅ Classifies a readiness response against the exact local run and component boundary. */
export function localHubReadinessAdmitted(body: Record<string, any>, status: number, runId: string, bootstrapSecuritySmoke = false, publicSessionIssuance = false): boolean {
  if (body.schema !== "semio.hub.readiness/v1" || body.runId !== runId || body.mode !== "development" || body.bindScope !== "loopback" || body.authentication?.kind !== "local-bootstrap-pipe-v1" || body.authentication?.publicSessionIssuance !== publicSessionIssuance)
    throw new Error("hub readiness binding mismatch");
  const componentsReady = body.directory?.ready === true && body.storage?.ready === true && body.adminAssets?.ready === true;
  const fullyReady = status === 200 && body.status === "ready" && body.authentication.bootstrapReady === true && componentsReady && body.artifactAuthority?.ready === true;
  const bootstrapReadyOnly = status === 503 && body.status === "not-ready" && body.authentication.bootstrapReady === true && componentsReady && body.artifactAuthority?.ready === false;
  return fullyReady || (bootstrapSecuritySmoke && bootstrapReadyOnly);
}

/** 🔭 Everything one poll can observe about a booting hub, flattened into the exact string whose
 * CHANGE means the hub advanced: whether `/readyz` answered at all, its HTTP status, the readiness
 * status it declared, its closed gates, and how many bytes the hub has written (captured runs only —
 * the hub rate-limits an in-flight `server.catalog.publication` record so a pre-bind catalog load is
 * visible here too). Anything new in this string restarts {@link LOCAL_READINESS_STALL_BOUND_MS}. */
function localHubReadinessObservation(run: LocalHubRun, answer: string): string {
  return `${answer} bytes=${run.output().length}`;
}

/** ⏱️ Polls the bounded readiness endpoint until the exact local run is admitted, bounded by how long
 * the hub goes without showing any sign of advancing rather than by how long the boot takes. */
export async function waitForReadiness(run: LocalHubRun, bootstrapSecuritySmoke = false, stallBoundMs = LOCAL_READINESS_STALL_BOUND_MS): Promise<Record<string, any>> {
  let closedGates = "no /readyz answer was ever received";
  let observation = "";
  let observedAt = Date.now();
  for (;;) {
    if (run.child.exitCode !== null) throw new Error("hub exited before readiness");
    let answer = "no-answer";
    try {
      const response = await fetch(`http://127.0.0.1:${run.port}/readyz`, { signal: AbortSignal.timeout(1000) });
      const body = (await response.json()) as Record<string, any>;
      if (localHubReadinessAdmitted(body, response.status, run.runId, bootstrapSecuritySmoke, run.publicSessionIssuance)) return body;
      const blocked = Array.isArray(body.blockedBy) ? (body.blockedBy as { gate: string; reason: string }[]) : [];
      closedGates = blocked.length ? blocked.map((closed) => `${closed.gate}=${closed.reason}`).join(" ") : `status=${body.status} with no closed gate declared`;
      answer = `http=${response.status} ${closedGates}`;
    } catch (error) {
      if (error instanceof Error && error.message === "hub readiness binding mismatch") throw error;
    }
    const next = localHubReadinessObservation(run, answer);
    const now = Date.now();
    if (next !== observation) {
      observation = next;
      observedAt = now;
    } else if (now - observedAt >= stallBoundMs) {
      throw new Error(`hub readiness stalled — nothing about the hub changed for ${now - observedAt} ms; last observation: ${next}; closed gates: ${closedGates}`);
    }
    await new Promise<void>((resolveDelay) => setTimeout(resolveDelay, 50));
  }
}

/** ⏳ Waits for one already-owned child without taking over its termination policy. */
export async function waitForChildExit(child: ChildProcess, deadlineMs = LOCAL_BOOTSTRAP_DEADLINE_MS + 2_000): Promise<void> {
  if (child.exitCode !== null) return;
  await Promise.race([new Promise<void>((resolveExit) => child.once("exit", () => resolveExit())), new Promise<void>((_, reject) => setTimeout(() => reject(new Error("hub child exit deadline exceeded")), deadlineMs))]);
}

/** 🧹 Finishes one local Hub exactly once, clears secrets, and removes only its allocated root. */
export async function finishLocalHub(run: LocalHubRun): Promise<void> {
  if (run.finishPromise) return run.finishPromise;
  run.finishPromise = (async () => {
    run.pipe.end();
    run.inferenceCheckpointPipe?.end();
    run.channelKey.fill(0);
    if (run.child.exitCode === null) {
      try {
        await waitForChildExit(run.child, 2_000);
      } catch {
        run.child.kill();
        await waitForChildExit(run.child, 2_000).catch(() => terminateOwnedChildTree(run.child));
      }
    }
    run.removeRunRoot();
  })();
  return run.finishPromise;
}
