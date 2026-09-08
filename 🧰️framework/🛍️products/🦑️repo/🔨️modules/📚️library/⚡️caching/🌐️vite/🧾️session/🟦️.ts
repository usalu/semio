import { existsSync, lstatSync, mkdirSync, readFileSync, readdirSync, renameSync, rmSync, rmdirSync, writeFileSync } from "node:fs";
import { randomUUID } from "node:crypto";
import { dirname, join, resolve } from "node:path";
import { setTimeout as delay } from "node:timers/promises";
import schema from "./🧬️schema/🔣️.json";

export type ServiceSession = { readonly schema: "semio.nx.service/v1"; readonly id: string; readonly owner: string; readonly pid: number };
export const SERVICE_READY_ENDPOINT = "/__semio/nx-service";
const idPattern = new RegExp(schema.$defs.Session.properties.id.pattern), urlPattern = new RegExp(schema.$defs.Ready.properties.url.pattern);
const keys = (value: unknown, expected: string): value is Record<string, unknown> => value !== null && typeof value === "object" && !Array.isArray(value) && Object.keys(value).sort().join() === expected;

/** 🧾️ Validates the owned service generation independently of any process-ID reuse. */
export function parseServiceSession(value: unknown): ServiceSession {
  if (!keys(value, "id,owner,pid,schema") || value.schema !== "semio.nx.service/v1" || typeof value.id !== "string" || !idPattern.test(value.id) || typeof value.owner !== "string" || !value.owner || value.owner.length > 256 || !Number.isSafeInteger(value.pid) || Number(value.pid) < 1) throw new Error("Invalid service session or pid");
  return value as ServiceSession;
}

/** 🗂️ Rejects links before accessing a task-owned service directory. */
function serviceDirectory(root: string, pid: number): string {
  if (!Number.isSafeInteger(pid) || pid < 1) throw new Error("Invalid Nx invocation pid");
  const directory = join(root, String(pid));
  for (let path = resolve(directory); dirname(path) !== path; path = dirname(path)) if (existsSync(path) && lstatSync(path).isSymbolicLink()) throw new Error(`Symlink service directory: ${path}`);
  return directory;
}

/** 📖️ Reads a bounded regular service record while rejecting unowned filesystem entries. */
function readRecord(path: string): unknown {
  const metadata = lstatSync(path);
  if (!metadata.isFile() || metadata.size > 4096) throw new Error(`Invalid service record: ${path}`);
  return JSON.parse(readFileSync(path, "utf8"));
}

/** 📬️ Publishes a small service record without exposing a partially written generation. */
function publishRecord(directory: string, name: string, value: unknown): void {
  const temporary = join(directory, `.stage-${randomUUID()}`);
  try { writeFileSync(temporary, JSON.stringify(value) + "\n", { flag: "wx" }); renameSync(temporary, join(directory, name)); }
  finally { rmSync(temporary, { force: true }); }
}

/** 📋️ Reads the generation explicitly prepared by this Nx invocation. */
export function readServiceSession(root: string, owner: string, pid: number): ServiceSession {
  const session = parseServiceSession(readRecord(join(serviceDirectory(root, pid), "session.json")));
  if (session.owner !== owner || session.pid !== pid) throw new Error("Service session owner mismatch");
  return session;
}

/** 🆕️ Starts a fresh generation after an uncached Nx preparation prerequisite. */
export function openServiceSession(root: string, owner: string, pid: number): ServiceSession {
  const directory = serviceDirectory(root, pid), session = parseServiceSession({ schema: "semio.nx.service/v1", id: randomUUID(), owner, pid });
  if (existsSync(directory)) {
    if (existsSync(join(directory, "session.json"))) readServiceSession(root, owner, pid);
    else if (readdirSync(directory).length) throw new Error(`Unowned service directory: ${directory}`);
  }
  mkdirSync(directory, { recursive: true });
  publishRecord(directory, "session.json", session);
  return session;
}

/** 🛣️ Restricts readiness to an actual loopback TCP listener without credentials or redirects. */
function serviceUrl(value: unknown): string {
  if (typeof value !== "string" || !urlPattern.test(value)) throw new Error("Invalid service URL");
  let url: URL;
  try { url = new URL(value); } catch { throw new Error("Invalid service URL"); }
  if (!url.port || Number(url.port) > 65535) throw new Error("Invalid service URL port");
  return value;
}

/** 🧾️ Reads a complete readiness record with an independently validated generation. */
function parseReady(value: unknown): { session: ServiceSession; url: string } {
  if (!keys(value, "schema,session,url") || value.schema !== "semio.nx.service-ready/v1") throw new Error("Invalid service readiness record");
  return { session: parseServiceSession(value.session), url: serviceUrl(value.url) };
}

/** 📡️ Announces the listener only if its prepared generation remains current. */
export function publishServiceReady(root: string, session: ServiceSession, url: string): void {
  const current = readServiceSession(root, session.owner, session.pid), directory = serviceDirectory(root, session.pid);
  if (current.id !== session.id) throw new Error("Service generation changed before readiness");
  serviceUrl(url);
  if (existsSync(join(directory, "ready.json"))) {
    const previous = parseReady(readRecord(join(directory, "ready.json"))).session;
    if (previous.owner !== session.owner || previous.pid !== session.pid) throw new Error("Service readiness owner mismatch");
  }
  publishRecord(directory, "ready.json", { schema: "semio.nx.service-ready/v1", session, url });
}

/** ⏳️ Requires the current generation's HTTP identity before allowing its consumer to proceed. */
export async function waitForServiceReady(root: string, session: ServiceSession, signal: AbortSignal, timeoutMs = 60000): Promise<string> {
  signal.throwIfAborted();
  const path = join(serviceDirectory(root, session.pid), "ready.json"), deadline = performance.now() + timeoutMs;
  while (performance.now() < deadline) {
    signal.throwIfAborted();
    if (readServiceSession(root, session.owner, session.pid).id !== session.id) throw new Error("Service generation changed while waiting");
    if (existsSync(path)) {
      const ready = parseReady(readRecord(path));
      if (ready.session.id === session.id && ready.session.owner === session.owner && ready.session.pid === session.pid) {
        try {
          const response = await fetch(new URL(SERVICE_READY_ENDPOINT, ready.url), { signal: AbortSignal.any([signal, AbortSignal.timeout(Math.max(1, Math.min(1000, Math.ceil(deadline - performance.now()))))]), redirect: "error" });
          if (response.ok) {
            const actual = parseServiceSession(await response.json());
            if (actual.id === session.id && actual.owner === session.owner && actual.pid === session.pid) return ready.url;
          }
        } catch { signal.throwIfAborted(); }
      }
    }
    await delay(Math.max(1, Math.min(40, deadline - performance.now())), undefined, { signal });
  }
  throw new Error(`Service did not become ready with the expected identity: ${session.owner}`);
}

/** 🧹️ Removes only this completed generation's records and preserves unrelated or newer state. */
export function closeServiceSession(root: string, session: ServiceSession): void {
  const directory = serviceDirectory(root, session.pid), currentPath = join(directory, "session.json");
  if (!existsSync(currentPath) || readServiceSession(root, session.owner, session.pid).id !== session.id) return;
  const readyPath = join(directory, "ready.json");
  if (existsSync(readyPath) && parseReady(readRecord(readyPath)).session.id === session.id) rmSync(readyPath);
  rmSync(currentPath);
  try { rmdirSync(directory); } catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOTEMPTY" && (error as NodeJS.ErrnoException).code !== "EEXIST") throw error; }
}
